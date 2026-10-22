//! Clause-family lowering: the shared V1 clause form, oracle, subject ABI and slots, with
//! admitted V2 clause lowering in [`v2`].

pub(super) mod v2;

use std::collections::BTreeMap;

use quire_contract_model::{
    BoundClause, BoundPackage, ClauseKind, ClauseRef, DependencyIdentity, DependencyKind,
    ExecutionPoint, StateObservation,
};

use crate::{
    core::diagnostic::{GenerationDiagnostic, GenerationErrorCode},
    core::naming::{oracle_symbol, reference_identifier},
    kani::abi::{
        i64_literal, readable_component, KaniBindingRole, KaniIntegerBounds, KaniPrimitiveType,
    },
    kani::generate::outcome::UnsupportedObligation,
    kani::identity::{ObligationBinding, ObligationKind},
    oracle::boolean_v1::{
        generate_boolean_oracle_shaped, measure_named_boolean_oracle, typed_dependency_parameters,
        DependencyParameter, OracleRequest, OracleShape, RustValueType,
    },
};

pub(super) struct LoweredClause<'a> {
    pub(super) package: &'a BoundPackage,
    pub(super) clause: &'a BoundClause,
    pub(super) kind: ObligationKind,
    pub(super) oracle: ClauseOracle,
    pub(super) symbols: Symbols,
    /// Preconditions sharing the anchor, with their oracles, filled by `resolve_assumptions`.
    pub(super) assumed: Vec<ClauseOracle>,
    /// For a contract obligation, the clause contexts its subject signature is built from: its
    /// own and those of every other supported contract obligation on the same operation, so all
    /// harnesses for one subject call it with one argument list. Filled by
    /// `unify_subject_signatures`.
    pub(super) signature: Vec<(ClauseOracle, SlotContext)>,
}

#[derive(Clone)]
pub(super) struct ClauseOracle {
    pub(super) clause: ClauseRef,
    pub(super) kind: ObligationKind,
    pub(super) anchor: ExecutionPoint,
    pub(super) symbol: String,
    pub(super) source: String,
    parameters: Vec<Parameter>,
}

#[derive(Clone)]
struct Parameter {
    dependency: DependencyIdentity,
    value_type: RustValueType,
}

pub(super) struct Symbols {
    pub(super) module: String,
    pub(super) harness: String,
    pub(super) contract: String,
}

pub(super) const fn obligation_kind(kind: ClauseKind) -> Option<ObligationKind> {
    match kind {
        ClauseKind::Precondition => Some(ObligationKind::Precondition),
        ClauseKind::Postcondition => Some(ObligationKind::Postcondition),
        ClauseKind::Invariant => Some(ObligationKind::Invariant),
        ClauseKind::Assertion | ClauseKind::Case | ClauseKind::Information => None,
    }
}

pub(super) fn lower_clause(
    clause: &BoundClause,
    kind: ObligationKind,
) -> Result<ClauseOracle, UnsupportedObligation> {
    let identity = clause.identity();
    let oracle_request = OracleRequest {
        requirement: identity.requirement(),
        clause: identity.clause(),
        expression: clause.expression(),
    };
    let plan = prepare_clause(clause, kind, &oracle_request)?;
    let _measured_oracle_bytes = plan.oracle_source_bytes;
    let parameters = plan.parameters;
    let bundle = generate_boolean_oracle_shaped(&oracle_request, OracleShape::PlainBool)
        .map_err(clause_diagnostic)?;
    let symbol =
        oracle_function_symbol(&bundle.rust.contents).ok_or(UnsupportedObligation::RenderFailed)?;
    Ok(ClauseOracle {
        clause: identity.clone(),
        kind,
        anchor: clause.anchor().clone(),
        symbol,
        source: bundle.rust.contents,
        parameters: parameters
            .into_iter()
            .map(
                |DependencyParameter {
                     dependency,
                     value_type,
                     ..
                 }| Parameter {
                    dependency,
                    value_type,
                },
            )
            .collect(),
    })
}

/// Artifact-free FR-015 lowering preflight shared by settlement and generation.
pub(crate) fn classify_bound_clause(clause: &BoundClause) -> Result<(), UnsupportedObligation> {
    let identity = clause.identity();
    let request = OracleRequest {
        requirement: identity.requirement(),
        clause: identity.clause(),
        expression: clause.expression(),
    };
    let Some(kind) = obligation_kind(clause.kind()) else {
        return Err(UnsupportedObligation::ClauseKindNotObligation {
            kind: clause.kind(),
        });
    };
    prepare_clause(clause, kind, &request).map(|_| ())
}

/// The shared, artifact-free FR-015 lowering plan for one admitted clause. It validates the
/// typed oracle, its Kani ABI, and its exact source-size ceiling; generation consumes the same
/// parameter plan and only materializes source after settlement.
fn prepare_clause(
    clause: &BoundClause,
    kind: ObligationKind,
    request: &OracleRequest<'_>,
) -> Result<PreparedClause, UnsupportedObligation> {
    let parameters = typed_clause_parameters(clause, request)?;
    let identity = clause.identity();
    let symbol = oracle_symbol(
        identity.requirement().requirement().as_str(),
        identity.requirement().revision().get(),
        identity.clause().as_str(),
    );
    let oracle_source_bytes =
        measure_named_boolean_oracle(request, &symbol, OracleShape::PlainBool)
            .map_err(clause_diagnostic)?;
    validate_clause_abi(clause, kind, &parameters)?;
    Ok(PreparedClause {
        parameters,
        oracle_source_bytes,
    })
}

/// Pure typed lowering facts consumed by both settlement and generation.
struct PreparedClause {
    parameters: Vec<DependencyParameter>,
    oracle_source_bytes: usize,
}

fn validate_clause_abi(
    clause: &BoundClause,
    kind: ObligationKind,
    parameters: &[DependencyParameter],
) -> Result<(), UnsupportedObligation> {
    let identity = clause.identity();
    let oracle = ClauseOracle {
        clause: identity.clone(),
        kind,
        anchor: clause.anchor().clone(),
        symbol: String::new(),
        source: String::new(),
        parameters: parameters
            .iter()
            .map(|parameter| Parameter {
                dependency: parameter.dependency.clone(),
                value_type: parameter.value_type.clone(),
            })
            .collect(),
    };
    match kind {
        ObligationKind::Precondition => abi(&[(&oracle, SlotContext::Precondition)]).map(|_| ()),
        ObligationKind::Postcondition => abi(&[(&oracle, SlotContext::Postcondition)]).map(|_| ()),
        ObligationKind::Invariant => abi(&[
            (&oracle, SlotContext::InvariantBefore),
            (&oracle, SlotContext::InvariantAfter),
        ])
        .map(|_| ()),
        ObligationKind::Frame => Ok(()),
    }
}

fn typed_clause_parameters(
    clause: &BoundClause,
    request: &OracleRequest<'_>,
) -> Result<Vec<DependencyParameter>, UnsupportedObligation> {
    let obligations = clause.expression().obligations().len();
    if obligations > 0 {
        return Err(UnsupportedObligation::DefinednessNotEncoded { obligations });
    }
    // The clause oracle is embedded where a plain `bool` is required, so a clause holding integer
    // arithmetic is refused rather than read as one (FR-031).
    typed_dependency_parameters(request, OracleShape::PlainBool).map_err(clause_diagnostic)
}

fn clause_diagnostic(diagnostics: Vec<GenerationDiagnostic>) -> UnsupportedObligation {
    UnsupportedObligation::ClauseLowering {
        generation_code: diagnostics
            .first()
            .map_or(GenerationErrorCode::UnsupportedExpression, |diagnostic| {
                diagnostic.code
            }),
    }
}

/// The one `pub fn oracle_…` the oracle generator emits.
fn oracle_function_symbol(source: &str) -> Option<String> {
    source.lines().find_map(|line| {
        line.strip_prefix("pub fn ")
            .and_then(|tail| tail.split('(').next())
            .map(str::to_owned)
    })
}

/// The readable stem of a clause obligation's names: its requirement, revision and clause. Kani
/// derives object-file names from these symbols, so the readable components are bounded.
pub(super) fn clause_stem(clause: &ClauseRef) -> String {
    let requirement = clause.requirement();
    format!(
        "kob_{}_{}_{}",
        readable_component(requirement.requirement().as_str()),
        requirement.revision().get(),
        readable_component(clause.clause().as_str()),
    )
}

/// The module, harness and contract names built on one settled name.
pub(super) fn symbols(base: &str) -> Symbols {
    Symbols {
        module: format!("{base}_module"),
        harness: format!("{base}_proof"),
        contract: format!("{base}_contract"),
    }
}

pub(super) fn contract_contexts<'l>(
    lowered: &'l LoweredClause<'_>,
) -> Vec<(&'l ClauseOracle, SlotContext)> {
    let mut contexts = lowered
        .assumed
        .iter()
        .map(|oracle| (oracle, SlotContext::Precondition))
        .collect::<Vec<_>>();
    match lowered.kind {
        ObligationKind::Precondition => contexts.push((&lowered.oracle, SlotContext::Precondition)),
        ObligationKind::Postcondition => {
            contexts.push((&lowered.oracle, SlotContext::Postcondition));
        }
        ObligationKind::Invariant => {
            contexts.push((&lowered.oracle, SlotContext::InvariantBefore));
            contexts.push((&lowered.oracle, SlotContext::InvariantAfter));
        }
        ObligationKind::Frame => {}
    }
    contexts
}

// ---------------------------------------------------------------------------
// Subject ABI
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Eq, PartialEq)]
pub(super) enum SlotContext {
    /// Evaluated on the arguments before the call.
    Precondition,
    /// Evaluated after the call, over pre-state arguments and post-state results.
    Postcondition,
    /// An invariant evaluated on the arguments before the call.
    InvariantBefore,
    /// An invariant evaluated on the results after the call.
    InvariantAfter,
}

pub(super) struct Abi {
    pub(super) arguments: Vec<ObligationBinding>,
    pub(super) results: Vec<ObligationBinding>,
}

impl Abi {
    pub(super) fn access(&self, identifier: &str) -> Option<String> {
        if self
            .arguments
            .iter()
            .any(|binding| binding.identifier == identifier)
        {
            return Some(identifier.to_owned());
        }
        let index = self
            .results
            .iter()
            .position(|binding| binding.identifier == identifier)?;
        Some(if self.results.len() == 1 {
            "*post_state".to_owned()
        } else {
            format!("post_state.{index}")
        })
    }
}

/// The slot a dependency occupies at one evaluation point.
fn slot(
    dependency: &DependencyIdentity,
    context: SlotContext,
) -> Option<(String, KaniBindingRole)> {
    let name = dependency.path().first()?.as_str();
    let observation = dependency.observation();
    let slot = |observation, role| Some((reference_identifier(name, Some(observation)), role));
    match (dependency.kind(), observation, context) {
        (DependencyKind::Input, None | Some(StateObservation::Current), _) => {
            slot(StateObservation::Current, KaniBindingRole::Argument)
        }
        (
            DependencyKind::State,
            Some(StateObservation::Current),
            SlotContext::Precondition | SlotContext::InvariantBefore,
        )
        | (
            DependencyKind::State,
            Some(StateObservation::Pre),
            SlotContext::Precondition | SlotContext::Postcondition,
        ) => slot(StateObservation::Pre, KaniBindingRole::Argument),
        (DependencyKind::State, Some(StateObservation::Current), SlotContext::InvariantAfter)
        | (DependencyKind::State, Some(StateObservation::Post), SlotContext::Postcondition) => {
            slot(StateObservation::Post, KaniBindingRole::Result)
        }
        _ => None,
    }
}

pub(super) fn abi(contexts: &[(&ClauseOracle, SlotContext)]) -> Result<Abi, UnsupportedObligation> {
    let mut bindings: BTreeMap<String, ObligationBinding> = BTreeMap::new();
    for (oracle, context) in contexts {
        for parameter in &oracle.parameters {
            let (identifier, role) = slot(&parameter.dependency, *context).ok_or_else(|| {
                UnsupportedObligation::ObservationNotBindable {
                    dependency: parameter.dependency.clone(),
                }
            })?;
            let (primitive_type, integer_bounds) = match &parameter.value_type {
                RustValueType::Boolean => (KaniPrimitiveType::Boolean, None),
                RustValueType::Integer(value) => (
                    KaniPrimitiveType::I64,
                    Some(KaniIntegerBounds::from_model(value).ok_or(
                        UnsupportedObligation::ClauseLowering {
                            generation_code: GenerationErrorCode::UnsupportedDependency,
                        },
                    )?),
                ),
            };
            match bindings.get_mut(&identifier) {
                Some(existing) => {
                    if existing.role != role
                        || existing.primitive_type != primitive_type
                        || existing.integer_bounds != integer_bounds
                        || existing.dependencies.first().map(DependencyIdentity::kind)
                            != Some(parameter.dependency.kind())
                    {
                        return Err(UnsupportedObligation::AbiConflict { identifier });
                    }
                    if !existing.dependencies.contains(&parameter.dependency) {
                        existing.dependencies.push(parameter.dependency.clone());
                        existing.dependencies.sort();
                    }
                }
                None => {
                    bindings.insert(
                        identifier.clone(),
                        ObligationBinding {
                            identifier,
                            role,
                            primitive_type,
                            integer_bounds,
                            dependencies: vec![parameter.dependency.clone()],
                        },
                    );
                }
            }
        }
    }
    let (arguments, results) = bindings
        .into_values()
        .partition(|binding| binding.role == KaniBindingRole::Argument);
    Ok(Abi { arguments, results })
}

pub(super) fn call(oracle: &ClauseOracle, context: SlotContext, abi: &Abi) -> Option<String> {
    let arguments = oracle
        .parameters
        .iter()
        .map(|parameter| {
            let (identifier, _) = slot(&parameter.dependency, context)?;
            abi.access(&identifier)
        })
        .collect::<Option<Vec<_>>>()?;
    Some(format!("{}({})", oracle.symbol, arguments.join(", ")))
}

/// `kani::any()` for every argument, constrained only by its IR integer bounds.
pub(super) fn symbolic_arguments(arguments: &[ObligationBinding]) -> String {
    arguments
        .iter()
        .map(|binding| {
            let declaration = format!(
                "        let {}: {} = kani::any();\n",
                binding.identifier,
                binding.primitive_type.source_name()
            );
            match &binding.integer_bounds {
                Some(bounds) => format!(
                    "{declaration}        kani::assume({id} >= {} && {id} <= {});\n",
                    i64_literal(bounds.minimum),
                    i64_literal(bounds.maximum),
                    id = binding.identifier,
                ),
                None => declaration,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ix_trace_rs::trace;
    use quire_contract_model::{
        AnchorName, ClauseId, DependencyName, IntegerDomain, IntegerType, OverflowPolicy,
        RequirementRef,
    };

    #[trace("FR-015-AC-1", "TC-025")]
    #[test]
    fn tc_025_every_clause_kind_maps_to_at_most_one_obligation_kind() {
        assert_eq!(
            obligation_kind(ClauseKind::Precondition),
            Some(ObligationKind::Precondition)
        );
        assert_eq!(
            obligation_kind(ClauseKind::Postcondition),
            Some(ObligationKind::Postcondition)
        );
        assert_eq!(
            obligation_kind(ClauseKind::Invariant),
            Some(ObligationKind::Invariant)
        );
        for kind in [
            ClauseKind::Assertion,
            ClauseKind::Case,
            ClauseKind::Information,
        ] {
            assert_eq!(obligation_kind(kind), None);
        }
    }

    #[trace("FR-015-CON-1", "TC-025")]
    #[test]
    fn tc_025_clause_abi_refuses_both_outside_i64_endpoints() {
        let requirement = RequirementRef::parse("test/kani-abi", "FR-015", 1).unwrap();
        let dependency = DependencyIdentity::new(
            requirement.clone(),
            DependencyKind::Input,
            vec![DependencyName::new("x").unwrap()],
        )
        .unwrap();
        for (minimum, maximum) in [
            (i128::from(i64::MIN) - 1, 0),
            (i128::MIN, 0),
            (0, i128::from(i64::MAX) + 1),
            (0, i128::MAX),
        ] {
            let value = IntegerType::new(
                IntegerDomain::Signed,
                minimum,
                maximum,
                OverflowPolicy::Reject,
            )
            .unwrap();
            let oracle = ClauseOracle {
                clause: ClauseRef::new(requirement.clone(), ClauseId::new("wide").unwrap()),
                kind: ObligationKind::Precondition,
                anchor: ExecutionPoint::Pre {
                    operation: AnchorName::new("run").unwrap(),
                },
                symbol: "oracle_wide".to_owned(),
                source: String::new(),
                parameters: vec![Parameter {
                    dependency: dependency.clone(),
                    value_type: RustValueType::Integer(value),
                }],
            };
            assert!(matches!(
                abi(&[(&oracle, SlotContext::Precondition)]),
                Err(UnsupportedObligation::ClauseLowering {
                    generation_code: GenerationErrorCode::UnsupportedDependency
                })
            ));
        }
    }
}
