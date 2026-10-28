//! Exact complete-V1 function-application oracle generation from one admitted
//! ContractPackage V1 (FR-021, FR-039).
//!
//! A requested item names one declared function and one admitted `call`
//! expression applying it. The generator classifies each function body from
//! Contract IR, resolves its QSL callable identity from the same checked
//! graph, and emits a Rust body factory. Each generated oracle reacquires the
//! checked closure through `qsl-replay`, selects the function by its owner and
//! wire node identity, registers the generated body, and applies it through
//! `quire-eval`. The QSL application session owns the call charge and
//! refusal ordering; this module does not implement a second evaluator.
//!
//! ## Scope of the function-body vocabulary (V1)
//!
//! FR-021's Inputs describe a declared function's body as one of: a `binary`
//! scalar expression (FR-014's forms), a `binary` equality expression
//! (FR-018's forms), or a nested `call` expression (this requirement's own
//! form). This generator's nested-`call` support is complete: any declared
//! function may call another declared function in the same request through a
//! QSL-selected call site and the `quire-eval` child scheduler. Its scalar
//! and composite-equality support is a deliberately scoped subset, not the
//! full FR-014/FR-018 operator matrix:
//!
//! - Scalar bodies: unbounded (`IntegerDomain::Mathematical`) integer
//!   arithmetic only -- [`crate::oracle::operator_catalog::IntegerOperator::Add`],
//!   `Subtract` and `Multiply` (binary; `Negate` is unary and out of scope,
//!   since FR-021's Inputs names only *binary* scalar expressions).
//! - Composite-equality bodies: [`crate::oracle::equality::EqualityOperatorKind`]
//!   (both `Equal` and `NotEqual`, fully supported) over `Boolean` or
//!   unbounded `Integer` operands only -- not the full record/tuple/
//!   collection declaration closure FR-018 itself reconstructs.
//!
//! The classifier has a bounded operator vocabulary. Its claim and location
//! maps remain deterministic over the admitted request. A function whose body
//! needs an operator outside this vocabulary is refused as
//! [`ExactFunctionRefusal::UnsupportedOperator`], never silently
//! miscompiled -- and nor is a function whose declared signature (parameter
//! count, or a parameter/result operand kind) disagrees with what its own
//! body kind requires: that is refused as
//! [`ExactFunctionRefusal::SignatureMismatch`], checked once every
//! parameter and result type is resolved, before the function can ever
//! reach generated application. The location map retains a static V1
//! description; the QSL session supplies dynamic application locations.
//!
//! Reused types, not reimplemented: this module imports
//! [`crate::oracle::operator_catalog::IntegerOperator`] and
//! [`crate::oracle::equality::EqualityOperatorKind`] directly rather than
//! declaring parallel enums, so a caller's scalar or equality descriptor is
//! the same type FR-014/FR-018 already validate elsewhere in this crate.
//!
//! ## Package assembly (Behavior, and AC-10/AC-11/AC-12)
//!
//! Every declared function in one `generate_exact_function_oracles` call is
//! classified independently first (a fixed-point pass resolves nested `call`
//! bodies against their callee's own classification). A function-scoped
//! refusal -- an unlowerable body, an unsupported operator, a signature
//! (declared parameter count, or a parameter/result operand kind)
//! disagreeing with its own body kind
//! ([`ExactFunctionRefusal::SignatureMismatch`]), a name shared with
//! another declared function in the same request
//! ([`ExactFunctionRefusal::AmbiguousFunctionName`], caught before
//! application), a `reference` composite operand
//! ([`UpstreamBlocker::QuireSpecLanguage120`]), a
//! model/relation/state/temporal/protocol node
//! ([`UpstreamBlocker::QuireSpecLanguage120`]/[`UpstreamBlocker::QuireSpecLanguage121`]),
//! or a nested `call` naming a callee absent from the request or itself
//! refused -- refuses only the items naming that function (AC-10, AC-11,
//! AC-12), never a sibling function's items. Two kinds of duplicate are
//! never seen by Stage 1's body classification at all, and never reach
//! Stage 2. Declarations sharing one declaring node id are refused first
//! ([`ExactFunctionRefusal::DuplicateDeclaringNode`], AC-22), including a
//! pair that also shares a name; the node-id check precedes the name check.
//! Declarations whose node ids are all distinct but whose name is shared are
//! refused as [`ExactFunctionRefusal::AmbiguousFunctionName`]. `resolved`
//! and `function_index` are keyed by each declaration's own node
//! id, which keeps two same-name declarations on distinct node ids apart,
//! but keying by node id cannot separate two declarations that share a node
//! id: they would collapse into one entry and cross claims and oracle
//! symbols. Refusing every such declaration up front is what prevents
//! that, so no node id reaching those maps is held by more than one
//! survivor. A surviving function is rendered from its admitted callable
//! identity. If that identity cannot be resolved uniquely, its requested
//! items receive a typed refusal rather than a generated oracle.
//!
//! ## Location tagging
//!
//! The static V1 location map records the admitted body's source order and one
//! root call point per classified body. Its path is empty for this one-node
//! body vocabulary. Generated oracles return the QSL application's full
//! `quire-eval::Evaluation` without reading its dynamic `location` or
//! `losses` fields and make no claim that either is populated.
//!
//! TC-031 checks parity with a separately assembled FR-485 session application
//! of the same admitted source, as AC-2 requires. It makes no QSL value-authority
//! agreement claim; quire-integration IR-842 owns that cross-repository boundary.

use crate::core::artifact::{Artifact, MAX_ARTIFACT_BYTES, MAX_GENERATED_SOURCE_BYTES};
mod owner_path;
mod owner_render;
use crate::core::contract_package::{
    routed_contract_lowering_profile, ContractDriverResultSelection, ContractRequestPosition,
    PackageItem, PackageItems, PackageRequestError,
};
use crate::core::ir::checked_term::aggregate_members;
use crate::core::ir::frame::application_identity;
use crate::core::naming::{bounded_readable_component, unique_names};
use crate::core::profile::function_oracle_crate_manifest;
use crate::oracle::checked_graph::{CheckedGraph, GraphBody};
use crate::oracle::claim::{ClaimDisposition, ClaimMap, OracleGenerationError, UpstreamBlocker};
use crate::oracle::equality::EqualityOperatorKind;
use crate::oracle::operator_catalog::IntegerOperator;
use crate::oracle::{classify_lowering_failure, direct_unsupported_form, LoweringFailure};
pub use owner_path::{
    apply_checked_function, checked_function_wire_id, checked_package_digest,
    select_checked_callable, selected_nested_call_site, CheckedFunctionError,
    CheckedFunctionSource,
};
use quire_contract_model::{
    ApplicationOperator, CheckedNodeId, CheckedNodeTag, CheckedPackageIncomplete,
    CheckedPackageReadLimits, CheckedPackageRefusal, CheckedPackageV2, CheckedSemanticId,
    CheckedSemanticNodeV2, CheckedSourceMapEntry, CheckedTerm, CompleteLoweringProfileV2,
    CompleteLoweringRecordV2, ContractDependencies, ContractFeatureRecordV1,
    ContractPackageReadResult, ContractPackageV1, ContractPackageWireV1, ContractReachedClosureV1,
    LiteralKind,
};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

// Target-neutral iterative SSA scheduling, introduced independently of the historical adapter.
pub mod schedule;

/// A function parameter's admitted V2 value node and declared type node.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct FunctionScheduleParameter {
    /// V2 `value` node with semantic form `parameter`.
    pub value_node_id: CheckedNodeId,
    /// V2 scalar type node declared for this parameter.
    pub type_node_id: CheckedNodeId,
}

/// Caller-supplied binding for one admitted V2 function declaration.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct FunctionScheduleDeclaration {
    /// Exact V2 `function` declaration node identity.
    pub function_node_id: CheckedNodeId,
    /// Admitted V2 `expression` node that is the function's body root.
    pub body_node_id: CheckedNodeId,
    /// Function name used by requested items.
    pub name: String,
    /// Parameter bindings in declaration order.
    pub parameters: Vec<FunctionScheduleParameter>,
    /// V2 scalar result type node.
    pub result_type_node_id: CheckedNodeId,
}

/// A requested call item whose checked source node is retained in its claim.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct FunctionScheduleItem {
    /// Admitted V2 call node identity.
    pub call_node_id: CheckedNodeId,
    /// Name of the function binding selected by the caller.
    pub function: String,
}

/// One item's schedule disposition.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum FunctionScheduleDisposition {
    /// The item binds to this function's body root slot.
    Scheduled {
        /// Exact V2 function declaration identity.
        function_node_id: CheckedNodeId,
        /// Root instruction slot in the target-neutral function body schedule.
        root: schedule::SsaSlot,
    },
    /// The item could not be scheduled.
    Refused {
        /// Exact function declaration, if one was resolved.
        function_node_id: Option<CheckedNodeId>,
        /// Typed graph or binding cause.
        refusal: schedule::ScheduleRefusal,
    },
}

/// Claim preserving one requested call node and its function schedule disposition.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct FunctionScheduleClaim {
    /// Requested call node identity.
    pub call_node_id: CheckedNodeId,
    /// Requested function name.
    pub function: String,
    /// Scheduled root or typed refusal.
    pub result: FunctionScheduleDisposition,
}

/// One function's target-neutral instruction and location tables.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ScheduledFunctionBody {
    /// Exact V2 function declaration identity.
    pub function_node_id: CheckedNodeId,
    /// Caller-supplied function name.
    pub name: String,
    /// Flat SSA schedule and compact parent-linked locations.
    pub schedule: schedule::Schedule,
}

/// One function whose body was refused before a schedule was produced.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RefusedFunctionBody {
    /// Exact V2 function declaration identity.
    pub function_node_id: CheckedNodeId,
    /// Caller-supplied function name.
    pub name: String,
    /// Typed refusal from graph projection or schedule analysis.
    pub refusal: schedule::ScheduleRefusal,
}

/// Complete function schedule result for one admitted package and request set.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct FunctionScheduleGeneration {
    /// Function bodies sorted by declaration node identity.
    pub functions: Vec<ScheduledFunctionBody>,
    /// Function bodies that failed independently of healthy siblings.
    pub refused_functions: Vec<RefusedFunctionBody>,
    /// One ordered disposition per requested call item.
    pub claims: Vec<FunctionScheduleClaim>,
    /// Compilable crate root source embedding the deterministic schedule metadata.
    pub source: String,
    /// Additional deterministic ordinal-range source modules for large schedules.
    pub modules: Vec<Artifact>,
    /// Total UTF-8 source bytes across the root and every module.
    pub source_bytes: usize,
}

/// Caller-selected work and source ceilings for function schedule generation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FunctionScheduleLimits {
    /// Maximum charged work per function body.
    pub generation_work_units: u64,
    /// Maximum total generated Rust source bytes across the crate and modules.
    pub source_bytes: usize,
}

impl Default for FunctionScheduleLimits {
    fn default() -> Self {
        Self {
            generation_work_units: schedule::DEFAULT_GENERATION_WORK_LIMIT,
            source_bytes: MAX_GENERATED_SOURCE_BYTES,
        }
    }
}

/// Builds target-neutral flat body schedules from one admitted V2 graph.
///
/// Every declaration and item is resolved against the package's retained node identities. The
/// work ceiling applies independently to each function body.
pub fn generate_function_schedules(
    package: &CheckedPackageV2,
    declarations: &[FunctionScheduleDeclaration],
    items: &[FunctionScheduleItem],
    work_limit: u64,
) -> Result<FunctionScheduleGeneration, schedule::ScheduleRefusal> {
    generate_function_schedules_with_limits(
        package,
        declarations,
        items,
        FunctionScheduleLimits {
            generation_work_units: work_limit,
            source_bytes: MAX_GENERATED_SOURCE_BYTES,
        },
    )
}

/// Builds target-neutral schedules with caller-selected work and source-byte limits.
///
/// The work ceiling applies independently to each function body. The default source ceiling is
/// 1,048,576 bytes; larger callers may raise it, while each emitted source artifact remains
/// bounded by the publication artifact limit and is split into deterministic ordinal modules.
pub fn generate_function_schedules_with_limits(
    package: &CheckedPackageV2,
    declarations: &[FunctionScheduleDeclaration],
    items: &[FunctionScheduleItem],
    limits: FunctionScheduleLimits,
) -> Result<FunctionScheduleGeneration, schedule::ScheduleRefusal> {
    // Reject the unrepresentable setting before package indexing or graph projection begins.
    let _ = schedule::WorkCounter::new(limits.generation_work_units)?;
    let source_nodes = Graph::from_checked(package);
    let mut type_kinds = BTreeMap::new();
    for node in &package.graph().nodes {
        if node.node_tag.as_ref() == "scalar_type" {
            let kind = match node.semantic_form.as_ref() {
                "boolean" => Some(schedule::ValueKind::Boolean),
                "integer" => Some(schedule::ValueKind::Integer),
                _ => None,
            };
            if let Some(kind) = kind {
                type_kinds.insert(node.node_id.clone(), kind);
            }
        }
    }

    // Scalar/Equality roots retain the checked Contract IR lowering gate. Call roots are
    // projected directly from the admitted graph so their transitive suffixes are not flattened
    // into one oversized lowering request (FR-021-AC-34).
    let mut scalar_roots = declarations
        .iter()
        .filter_map(|declaration| {
            source_nodes
                .node(&declaration.body_node_id)
                .filter(|node| {
                    source_nodes.operator(&node.node_id) == Some(ApplicationOperator::Binary)
                })
                .map(|_| declaration.body_node_id.clone())
        })
        .collect::<Vec<_>>();
    scalar_roots.sort();
    scalar_roots.dedup();
    let lowering_failures = if scalar_roots.is_empty() {
        BTreeMap::new()
    } else {
        let lowered_scalars = package.lower(&scalar_roots, &lowering_profile());
        scalar_roots
            .iter()
            .zip(&lowered_scalars.records)
            .filter_map(|(node_id, record)| {
                schedule_lowering_failure(record).map(|refusal| (node_id.clone(), refusal))
            })
            .collect::<BTreeMap<_, _>>()
    };

    // Requested call items have their own lowering batch. This intentionally does not include
    // function Call body roots, whose suffixes are projected per function above.
    let mut call_nodes = items
        .iter()
        .map(|item| item.call_node_id.clone())
        .collect::<Vec<_>>();
    call_nodes.sort();
    call_nodes.dedup();
    let call_lowering_failures = if call_nodes.is_empty() {
        BTreeMap::new()
    } else {
        let lowered_calls = package.lower(&call_nodes, &lowering_profile());
        call_nodes
            .iter()
            .zip(&lowered_calls.records)
            .filter_map(|(node_id, record)| {
                schedule_lowering_failure(record).map(|refusal| (node_id.clone(), refusal))
            })
            .collect::<BTreeMap<_, _>>()
    };

    let mut bindings = BTreeMap::new();
    let mut unsupported_functions: BTreeMap<CheckedNodeId, schedule::ScheduleRefusal> =
        BTreeMap::new();
    let mut ordered_declarations: Vec<&FunctionScheduleDeclaration> = declarations.iter().collect();
    ordered_declarations.sort_by(|left, right| left.function_node_id.cmp(&right.function_node_id));
    for declaration in &ordered_declarations {
        let function_node = source_nodes
            .node(&declaration.function_node_id)
            .ok_or_else(|| {
                schedule::ScheduleRefusal::MissingNode(declaration.function_node_id.clone())
            })?;
        if function_node.node_tag.as_ref() != "function" {
            return Err(schedule::ScheduleRefusal::MalformedTerm(
                declaration.function_node_id.clone(),
            ));
        }
        if decode_reference(
            source_nodes.term(&function_node.node_id).ok_or_else(|| {
                schedule::ScheduleRefusal::MalformedTerm(declaration.function_node_id.clone())
            })?,
            &declaration.function_node_id,
        )? != declaration.body_node_id
        {
            return Err(schedule::ScheduleRefusal::WrongBodyBinding {
                function: declaration.function_node_id.clone(),
                expected: declaration.body_node_id.clone(),
            });
        }
        if declaration
            .parameters
            .iter()
            .any(|parameter| !type_kinds.contains_key(&parameter.type_node_id))
            || !type_kinds.contains_key(&declaration.result_type_node_id)
        {
            unsupported_functions.insert(
                declaration.function_node_id.clone(),
                schedule::ScheduleRefusal::UnsupportedOperation(
                    declaration.function_node_id.clone(),
                ),
            );
            continue;
        }
        let parameter_kinds = declaration
            .parameters
            .iter()
            .map(|parameter| {
                type_kinds
                    .get(&parameter.type_node_id)
                    .copied()
                    .ok_or_else(|| {
                        schedule::ScheduleRefusal::MissingNode(parameter.type_node_id.clone())
                    })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let result_kind = type_kinds
            .get(&declaration.result_type_node_id)
            .copied()
            .ok_or_else(|| {
                schedule::ScheduleRefusal::MissingNode(declaration.result_type_node_id.clone())
            })?;
        let binding = schedule::FunctionBinding {
            declaration: declaration.function_node_id.clone(),
            parameter_kinds,
            result_kind,
        };
        bindings.insert(declaration.function_node_id.clone(), binding);
    }

    let mut function_bodies = Vec::with_capacity(ordered_declarations.len());
    let mut refused_functions = Vec::new();
    let mut schedules_by_node = BTreeMap::new();
    let mut declarations_by_name: BTreeMap<&str, Vec<CheckedNodeId>> = BTreeMap::new();
    for declaration in ordered_declarations {
        declarations_by_name
            .entry(declaration.name.as_str())
            .or_default()
            .push(declaration.function_node_id.clone());
        if let Some(refusal) = unsupported_functions.get(&declaration.function_node_id) {
            schedules_by_node.insert(declaration.function_node_id.clone(), Err(refusal.clone()));
            refused_functions.push(RefusedFunctionBody {
                function_node_id: declaration.function_node_id.clone(),
                name: declaration.name.clone(),
                refusal: refusal.clone(),
            });
            continue;
        }
        let binding = bindings.get(&declaration.function_node_id).ok_or_else(|| {
            schedule::ScheduleRefusal::MissingNode(declaration.function_node_id.clone())
        })?;
        let parameters = declaration
            .parameters
            .iter()
            .enumerate()
            .map(|(index, parameter)| {
                u32::try_from(index)
                    .map(|index| (parameter.value_node_id.clone(), index))
                    .map_err(|_| schedule::ScheduleRefusal::CounterOverflow)
            })
            .collect::<Result<BTreeMap<_, _>, _>>()?;
        let mut work = schedule::WorkCounter::new(limits.generation_work_units)?;
        let result = if let Some(refusal) = lowering_failures.get(&declaration.body_node_id) {
            Err(refusal.clone())
        } else {
            project_body_graph(
                &source_nodes,
                &type_kinds,
                &declaration.body_node_id,
                &mut work,
            )
            .and_then(|projected| {
                let schedule = schedule::build_schedule_with_work(
                    &declaration.body_node_id,
                    &projected,
                    &parameters,
                    binding,
                    &bindings,
                    &mut work,
                )?;
                let root = schedule
                    .instructions
                    .last()
                    .map(|instruction| instruction.output)
                    .ok_or_else(|| {
                        schedule::ScheduleRefusal::MissingNode(declaration.body_node_id.clone())
                    })?;
                Ok((schedule, root))
            })
        };
        match result {
            Ok((schedule, root)) => {
                schedules_by_node.insert(declaration.function_node_id.clone(), Ok(root));
                function_bodies.push(ScheduledFunctionBody {
                    function_node_id: declaration.function_node_id.clone(),
                    name: declaration.name.clone(),
                    schedule,
                });
            }
            Err(refusal) => {
                schedules_by_node
                    .insert(declaration.function_node_id.clone(), Err(refusal.clone()));
                refused_functions.push(RefusedFunctionBody {
                    function_node_id: declaration.function_node_id.clone(),
                    name: declaration.name.clone(),
                    refusal,
                });
            }
        }
    }

    let mut item_counts = BTreeMap::<(CheckedNodeId, String), usize>::new();
    for item in items {
        *item_counts
            .entry((item.call_node_id.clone(), item.function.clone()))
            .or_default() += 1;
    }
    let mut claims = Vec::with_capacity(item_counts.len());
    for ((call_node_id, function), count) in item_counts {
        let item = FunctionScheduleItem {
            call_node_id: call_node_id.clone(),
            function: function.clone(),
        };
        let matching = declarations_by_name.get(item.function.as_str());
        let result = if count > 1 {
            FunctionScheduleDisposition::Refused {
                function_node_id: matching.and_then(|nodes| nodes.first().cloned()),
                refusal: schedule::ScheduleRefusal::DuplicateRequest {
                    call_node_id: call_node_id.clone(),
                    function: function.clone(),
                },
            }
        } else {
            match matching.map(Vec::as_slice) {
                Some([function_node_id]) => {
                    if let Some(refusal) = call_lowering_failures.get(&item.call_node_id) {
                        claims.push(FunctionScheduleClaim {
                            call_node_id: item.call_node_id.clone(),
                            function: item.function.clone(),
                            result: FunctionScheduleDisposition::Refused {
                                function_node_id: Some(function_node_id.clone()),
                                refusal: refusal.clone(),
                            },
                        });
                        continue;
                    }
                    let binding = bindings.get(function_node_id);
                    let item_check = binding.map_or_else(
                        || {
                            Err(unsupported_functions
                                .get(function_node_id)
                                .cloned()
                                .unwrap_or_else(|| {
                                    schedule::ScheduleRefusal::MissingNode(function_node_id.clone())
                                }))
                        },
                        |binding| {
                            validate_schedule_item(&source_nodes, &type_kinds, &item, binding)
                        },
                    );
                    match item_check {
                        Err(refusal) => FunctionScheduleDisposition::Refused {
                            function_node_id: Some(function_node_id.clone()),
                            refusal,
                        },
                        Ok(()) => match schedules_by_node.get(function_node_id) {
                            Some(Ok(root)) => FunctionScheduleDisposition::Scheduled {
                                function_node_id: function_node_id.clone(),
                                root: *root,
                            },
                            Some(Err(refusal)) => FunctionScheduleDisposition::Refused {
                                function_node_id: Some(function_node_id.clone()),
                                refusal: refusal.clone(),
                            },
                            None => FunctionScheduleDisposition::Refused {
                                function_node_id: Some(function_node_id.clone()),
                                refusal: schedule::ScheduleRefusal::MissingNode(
                                    function_node_id.clone(),
                                ),
                            },
                        },
                    }
                }
                Some([]) | None => FunctionScheduleDisposition::Refused {
                    function_node_id: None,
                    refusal: schedule::ScheduleRefusal::UnknownFunction(item.function.clone()),
                },
                Some(nodes) => FunctionScheduleDisposition::Refused {
                    function_node_id: nodes.first().cloned(),
                    refusal: schedule::ScheduleRefusal::AmbiguousFunctionName(
                        item.function.clone(),
                    ),
                },
            }
        };
        claims.push(FunctionScheduleClaim {
            call_node_id,
            function,
            result,
        });
    }
    claims.sort_by(|left, right| {
        left.call_node_id
            .cmp(&right.call_node_id)
            .then_with(|| {
                let declaration_key = |claim: &FunctionScheduleClaim| {
                    declarations_by_name
                        .get(claim.function.as_str())
                        .and_then(|nodes| nodes.first())
                };
                declaration_key(left).cmp(&declaration_key(right))
            })
            .then_with(|| left.function.as_bytes().cmp(right.function.as_bytes()))
    });

    let (source, modules, source_bytes) = render_schedule_source(
        &function_bodies,
        &refused_functions,
        &claims,
        limits.source_bytes,
    )?;
    Ok(FunctionScheduleGeneration {
        functions: function_bodies,
        refused_functions,
        claims,
        source,
        modules,
        source_bytes,
    })
}

const SCHEDULE_INSTRUCTIONS_PER_MODULE: usize = 1024;

#[derive(Serialize)]
struct ScheduleSourcePart<'a> {
    function_node_id: &'a CheckedNodeId,
    function_name: &'a str,
    instruction_start: usize,
    instruction_end: usize,
    instructions: &'a [schedule::Instruction],
    locations: &'a [schedule::LocationRecord],
    control_flow: &'a [schedule::ControlBlock],
}

fn render_schedule_source(
    functions: &[ScheduledFunctionBody],
    refused_functions: &[RefusedFunctionBody],
    claims: &[FunctionScheduleClaim],
    source_limit: usize,
) -> Result<(String, Vec<Artifact>, usize), schedule::ScheduleRefusal> {
    let complete_json = serde_json::to_string(&(functions, refused_functions, claims))
        .map_err(|_| schedule::ScheduleRefusal::SerializationFailure)?;
    let flat_source = format!(
        "/// Deterministic target-neutral function schedule.\npub const FUNCTION_SCHEDULE_JSON: &str = {complete_json:?};\n"
    );
    let subjects = functions
        .iter()
        .map(|function| crate::core::provenance::node_subject(&function.function_node_id))
        .chain(
            claims
                .iter()
                .map(|claim| crate::core::provenance::node_subject(&claim.call_node_id)),
        )
        .collect::<Vec<_>>();
    let mut flat_subjects = subjects.clone();
    flat_subjects.extend(functions.iter().flat_map(|function| {
        function
            .schedule
            .instructions
            .iter()
            .map(|instruction| crate::core::provenance::node_subject(&instruction.node))
    }));
    let flat_source = crate::core::provenance::with_rust_header(
        flat_source,
        flat_subjects.iter().map(String::as_str),
    );
    if flat_source.len() <= MAX_ARTIFACT_BYTES {
        let source_bytes = flat_source.len();
        if source_bytes > source_limit {
            return Err(schedule::ScheduleRefusal::SourceTooLarge {
                bytes: source_bytes,
                limit: source_limit,
                setting: "cg.function.source_bytes",
            });
        }
        return Ok((flat_source, Vec::new(), source_bytes));
    }

    let metadata = functions
        .iter()
        .map(|function| {
            (
                &function.function_node_id,
                &function.name,
                function.schedule.entry,
                function.schedule.instructions.len(),
            )
        })
        .collect::<Vec<_>>();
    let metadata_json = serde_json::to_string(&(metadata, refused_functions, claims))
        .map_err(|_| schedule::ScheduleRefusal::SerializationFailure)?;
    let source = format!(
        "/// Deterministic target-neutral function schedule metadata.\npub const FUNCTION_SCHEDULE_METADATA_JSON: &str = {metadata_json:?};\n"
    );
    let mut source =
        crate::core::provenance::with_rust_header(source, subjects.iter().map(String::as_str));
    let mut modules = Vec::new();
    let mut declarations = String::new();
    let mut module_values = String::new();
    for (function_ordinal, function) in functions.iter().enumerate() {
        let instruction_count = function.schedule.instructions.len();
        let location_count = function.schedule.locations.len();
        if instruction_count != location_count {
            return Err(schedule::ScheduleRefusal::MalformedTerm(
                function.function_node_id.clone(),
            ));
        }
        let mut start = 0usize;
        while start < instruction_count {
            let end = start
                .saturating_add(SCHEDULE_INSTRUCTIONS_PER_MODULE)
                .min(instruction_count);
            let block_start = start
                .checked_mul(2)
                .ok_or(schedule::ScheduleRefusal::CounterOverflow)?;
            let mut block_end = end
                .checked_mul(2)
                .ok_or(schedule::ScheduleRefusal::CounterOverflow)?;
            if end == instruction_count {
                block_end = function.schedule.control_flow.len();
            }
            let part = ScheduleSourcePart {
                function_node_id: &function.function_node_id,
                function_name: &function.name,
                instruction_start: start,
                instruction_end: end,
                instructions: &function.schedule.instructions[start..end],
                locations: &function.schedule.locations[start..end],
                control_flow: function
                    .schedule
                    .control_flow
                    .get(block_start..block_end)
                    .ok_or_else(|| {
                        schedule::ScheduleRefusal::MalformedTerm(function.function_node_id.clone())
                    })?,
            };
            let part_json = serde_json::to_string(&part)
                .map_err(|_| schedule::ScheduleRefusal::SerializationFailure)?;
            let module_name = format!("schedule_{function_ordinal:06}_{start:06}_{end:06}");
            let module_source = format!(
                "/// Ordinal range {start}..{end} for function schedule {function_ordinal}.\npub const FUNCTION_SCHEDULE_PART_JSON: &str = {part_json:?};\n"
            );
            let part_subjects = std::iter::once(crate::core::provenance::node_subject(
                &function.function_node_id,
            ))
            .chain(
                part.instructions
                    .iter()
                    .map(|instruction| crate::core::provenance::node_subject(&instruction.node)),
            )
            .collect::<Vec<_>>();
            let module_source = crate::core::provenance::with_rust_header(
                module_source,
                part_subjects.iter().map(String::as_str),
            );
            if module_source.len() > MAX_ARTIFACT_BYTES {
                return Err(schedule::ScheduleRefusal::SourceArtifactTooLarge {
                    bytes: module_source.len(),
                    limit: MAX_ARTIFACT_BYTES,
                });
            }
            declarations.push_str(&format!("mod {module_name};\n"));
            module_values.push_str(&format!(
                "    {module_name}::FUNCTION_SCHEDULE_PART_JSON,\n"
            ));
            modules.push(Artifact::new(
                format!("src/{module_name}.rs"),
                module_source,
            ));
            start = end;
        }
    }
    let artifact_count = modules
        .len()
        .checked_add(1)
        .ok_or(schedule::ScheduleRefusal::CounterOverflow)?;
    if artifact_count > crate::core::artifact::MAX_ARTIFACTS {
        return Err(schedule::ScheduleRefusal::TooManySourceModules {
            count: artifact_count,
            limit: crate::core::artifact::MAX_ARTIFACTS,
        });
    }
    source.push_str(&declarations);
    source.push_str(
        "\n/// Serialized per-range schedule tables in function and instruction order.\n",
    );
    source.push_str("pub const FUNCTION_SCHEDULE_PARTS: &[&str] = &[\n");
    source.push_str(&module_values);
    source.push_str("];\n");
    if source.len() > MAX_ARTIFACT_BYTES {
        return Err(schedule::ScheduleRefusal::SourceArtifactTooLarge {
            bytes: source.len(),
            limit: MAX_ARTIFACT_BYTES,
        });
    }
    let source_bytes = modules.iter().try_fold(source.len(), |total, module| {
        total
            .checked_add(module.contents.len())
            .ok_or(schedule::ScheduleRefusal::CounterOverflow)
    })?;
    if source_bytes > crate::core::artifact::MAX_BUNDLE_BYTES {
        return Err(schedule::ScheduleRefusal::SourceBundleTooLarge {
            bytes: source_bytes,
            limit: crate::core::artifact::MAX_BUNDLE_BYTES,
        });
    }
    if source_bytes > source_limit {
        return Err(schedule::ScheduleRefusal::SourceTooLarge {
            bytes: source_bytes,
            limit: source_limit,
            setting: "cg.function.source_bytes",
        });
    }
    Ok((source, modules, source_bytes))
}

fn project_body_graph(
    source_nodes: &CheckedGraph<'_, CheckedTerm>,
    type_kinds: &BTreeMap<CheckedNodeId, schedule::ValueKind>,
    root: &CheckedNodeId,
    work: &mut schedule::WorkCounter,
) -> Result<BTreeMap<CheckedNodeId, schedule::GraphNode>, schedule::ScheduleRefusal> {
    enum ProjectionEvent {
        Enter(CheckedNodeId),
        Continue {
            children: Vec<schedule::GraphOperand>,
            next: usize,
        },
    }

    let mut projected = BTreeMap::new();
    let mut stack = vec![ProjectionEvent::Enter(root.clone())];
    while let Some(event) = stack.pop() {
        match event {
            ProjectionEvent::Enter(node_id) => {
                work.charge()?;
                if projected.contains_key(&node_id) {
                    continue;
                }
                let source = source_nodes
                    .node(&node_id)
                    .ok_or_else(|| schedule::ScheduleRefusal::MissingNode(node_id.clone()))?;
                if source_nodes.operator(&source.node_id) == Some(ApplicationOperator::Call) {
                    // The call's argument-zero declaration binding is a child edge and is
                    // charged before its identity is decoded or any value argument is visited.
                    work.charge()?;
                }
                let (value_kind, kind, children) =
                    project_function_node(source, source_nodes.term(&source.node_id), type_kinds)?
                        .ok_or_else(|| schedule::ScheduleRefusal::MalformedTerm(node_id.clone()))?;
                projected.insert(
                    node_id.clone(),
                    schedule::GraphNode {
                        id: node_id,
                        value_kind,
                        kind,
                    },
                );
                stack.push(ProjectionEvent::Continue { children, next: 0 });
            }
            ProjectionEvent::Continue { children, next } => {
                if let Some(child) = children.get(next).cloned() {
                    work.charge()?;
                    stack.push(ProjectionEvent::Continue {
                        children,
                        next: next + 1,
                    });
                    if let schedule::GraphOperand::Node(child) = child {
                        stack.push(ProjectionEvent::Enter(child));
                    }
                }
            }
        }
    }
    Ok(projected)
}

fn project_function_node<Body>(
    source: &CheckedSemanticNodeV2<Body>,
    body: Option<&CheckedTerm>,
    type_kinds: &BTreeMap<CheckedNodeId, schedule::ValueKind>,
) -> Result<
    Option<(
        schedule::ValueKind,
        schedule::NodeKind,
        Vec<schedule::GraphOperand>,
    )>,
    schedule::ScheduleRefusal,
> {
    if source.node_tag.as_ref() == "value" && source.semantic_form.as_ref() == "parameter" {
        let kind = type_kinds
            .get(&source.semantic_type)
            .copied()
            .ok_or_else(|| schedule::ScheduleRefusal::MissingNode(source.semantic_type.clone()))?;
        return Ok(Some((kind, schedule::NodeKind::Parameter, Vec::new())));
    }
    if !matches!(source.node_tag.as_ref(), "function" | "expression") {
        return Ok(None);
    }

    let Some(CheckedTerm::Application {
        result_type,
        arguments,
        operator,
        ..
    }) = body
    else {
        return Err(schedule::ScheduleRefusal::MalformedTerm(
            source.node_id.clone(),
        ));
    };
    let value_kind = type_kinds
        .get(result_type)
        .copied()
        .ok_or_else(|| schedule::ScheduleRefusal::UnsupportedOperation(source.node_id.clone()))?;
    let operation_identity = body
        .and_then(application_identity)
        .ok_or_else(|| schedule::ScheduleRefusal::MalformedTerm(source.node_id.clone()))?;

    if *operator == ApplicationOperator::Call {
        if operation_identity != "quire.op.function.call" {
            return Err(schedule::ScheduleRefusal::UnsupportedOperation(
                source.node_id.clone(),
            ));
        }
        let callee = arguments
            .first()
            .ok_or_else(|| schedule::ScheduleRefusal::MalformedTerm(source.node_id.clone()))?;
        let callee = decode_reference(callee, &source.node_id)?;
        let value_arguments = arguments[1..]
            .iter()
            .map(|argument| decode_schedule_operand(argument, &source.node_id, type_kinds))
            .collect::<Result<Vec<_>, _>>()?;
        let children = value_arguments.clone();
        return Ok(Some((
            value_kind,
            schedule::NodeKind::Call {
                callee,
                arguments: value_arguments,
            },
            children,
        )));
    }

    if *operator != ApplicationOperator::Binary || arguments.len() != 2 {
        return Err(schedule::ScheduleRefusal::MalformedTerm(
            source.node_id.clone(),
        ));
    }
    let left = decode_schedule_operand(&arguments[0], &source.node_id, type_kinds)?;
    let right = decode_schedule_operand(&arguments[1], &source.node_id, type_kinds)?;
    let children = vec![left.clone(), right.clone()];
    Ok(Some((
        value_kind,
        schedule::NodeKind::Binary {
            operation_identity: operation_identity.to_owned(),
            left,
            right,
        },
        children,
    )))
}

fn decode_schedule_operand(
    term: &CheckedTerm,
    owner: &CheckedNodeId,
    type_kinds: &BTreeMap<CheckedNodeId, schedule::ValueKind>,
) -> Result<schedule::GraphOperand, schedule::ScheduleRefusal> {
    match term {
        CheckedTerm::Reference { target } => Ok(schedule::GraphOperand::Node(target.clone())),
        CheckedTerm::Literal {
            value_kind,
            type_id,
            value,
        } => {
            let value_kind = match value_kind {
                LiteralKind::Boolean => schedule::ValueKind::Boolean,
                LiteralKind::Integer => schedule::ValueKind::Integer,
                _ => return Err(schedule::ScheduleRefusal::UnsupportedLiteral(owner.clone())),
            };
            if type_kinds.get(type_id).copied() != Some(value_kind) {
                return Err(schedule::ScheduleRefusal::WrongType(owner.clone()));
            }
            let value = value
                .as_str()
                .ok_or_else(|| schedule::ScheduleRefusal::MalformedTerm(owner.clone()))?;
            if value_kind == schedule::ValueKind::Boolean && !matches!(value, "true" | "false") {
                return Err(schedule::ScheduleRefusal::MalformedTerm(owner.clone()));
            }
            if value_kind == schedule::ValueKind::Integer
                && (value.is_empty()
                    || value.strip_prefix('-').unwrap_or(value).is_empty()
                    || !value
                        .strip_prefix('-')
                        .unwrap_or(value)
                        .bytes()
                        .all(|byte| byte.is_ascii_digit()))
            {
                return Err(schedule::ScheduleRefusal::MalformedTerm(owner.clone()));
            }
            Ok(schedule::GraphOperand::Literal {
                value_kind,
                value: value.to_owned(),
            })
        }
        _ => Err(schedule::ScheduleRefusal::MalformedTerm(owner.clone())),
    }
}

fn decode_reference(
    term: &CheckedTerm,
    owner: &CheckedNodeId,
) -> Result<CheckedNodeId, schedule::ScheduleRefusal> {
    match term {
        CheckedTerm::Reference { target } => Ok(target.clone()),
        CheckedTerm::Literal { .. } => {
            Err(schedule::ScheduleRefusal::UnsupportedLiteral(owner.clone()))
        }
        _ => Err(schedule::ScheduleRefusal::MalformedTerm(owner.clone())),
    }
}

fn validate_schedule_item(
    source_nodes: &CheckedGraph<'_, CheckedTerm>,
    type_kinds: &BTreeMap<CheckedNodeId, schedule::ValueKind>,
    item: &FunctionScheduleItem,
    binding: &schedule::FunctionBinding,
) -> Result<(), schedule::ScheduleRefusal> {
    let source = source_nodes
        .node(&item.call_node_id)
        .ok_or_else(|| schedule::ScheduleRefusal::MissingNode(item.call_node_id.clone()))?;
    if source.node_tag.as_ref() != "expression" {
        return Err(schedule::ScheduleRefusal::MalformedTerm(
            item.call_node_id.clone(),
        ));
    }
    let body = source_nodes.term(&item.call_node_id);
    let Some(CheckedTerm::Application {
        operator: ApplicationOperator::Call,
        arguments,
        result_type,
        ..
    }) = body
    else {
        return Err(schedule::ScheduleRefusal::MalformedTerm(
            item.call_node_id.clone(),
        ));
    };
    if body.and_then(application_identity) != Some("quire.op.function.call") {
        return Err(schedule::ScheduleRefusal::MalformedTerm(
            item.call_node_id.clone(),
        ));
    }
    let callee_term = arguments
        .first()
        .ok_or_else(|| schedule::ScheduleRefusal::MalformedTerm(item.call_node_id.clone()))?;
    let found_callee = decode_reference(callee_term, &item.call_node_id)?;
    if found_callee != binding.declaration {
        return Err(schedule::ScheduleRefusal::WrongCallBinding {
            expected: binding.declaration.clone(),
            found: found_callee,
        });
    }
    let found = arguments.len().saturating_sub(1);
    if found != binding.parameter_kinds.len() {
        return Err(schedule::ScheduleRefusal::WrongArity {
            function: binding.declaration.clone(),
            expected: binding.parameter_kinds.len(),
            found,
        });
    }
    for (index, term) in arguments.iter().skip(1).enumerate() {
        let value_kind =
            schedule_term_value_kind(term, source_nodes, type_kinds, &item.call_node_id)?;
        if value_kind != binding.parameter_kinds[index] {
            let node_id = match term {
                CheckedTerm::Reference { target } => target.clone(),
                _ => item.call_node_id.clone(),
            };
            return Err(schedule::ScheduleRefusal::WrongType(node_id));
        }
    }
    if type_kinds.get(result_type).copied() != Some(binding.result_kind) {
        return Err(schedule::ScheduleRefusal::WrongType(
            item.call_node_id.clone(),
        ));
    }
    Ok(())
}

fn schedule_term_value_kind(
    term: &CheckedTerm,
    source_nodes: &CheckedGraph<'_, CheckedTerm>,
    type_kinds: &BTreeMap<CheckedNodeId, schedule::ValueKind>,
    owner: &CheckedNodeId,
) -> Result<schedule::ValueKind, schedule::ScheduleRefusal> {
    let type_node_id = match term {
        CheckedTerm::Reference { target } => {
            let source = source_nodes
                .node(target)
                .ok_or_else(|| schedule::ScheduleRefusal::MissingNode(target.clone()))?;
            let body = source_nodes.term(target);
            if source.node_tag.as_ref() == "function"
                || (source.node_tag.as_ref() == "expression"
                    && matches!(body, Some(CheckedTerm::Application { .. })))
            {
                let Some(CheckedTerm::Application { result_type, .. }) = body else {
                    return Err(schedule::ScheduleRefusal::MalformedTerm(target.clone()));
                };
                result_type.clone()
            } else {
                source.semantic_type.clone()
            }
        }
        CheckedTerm::Literal { .. } => {
            return decode_schedule_operand(term, owner, type_kinds).and_then(
                |operand| match operand {
                    schedule::GraphOperand::Literal { value_kind, .. } => Ok(value_kind),
                    schedule::GraphOperand::Node(_) => {
                        Err(schedule::ScheduleRefusal::MalformedTerm(owner.clone()))
                    }
                },
            );
        }
        _ => return Err(schedule::ScheduleRefusal::MalformedTerm(owner.clone())),
    };
    type_kinds
        .get(&type_node_id)
        .copied()
        .ok_or(schedule::ScheduleRefusal::MissingNode(type_node_id))
}

/// Work budget for lowering one requested function body or call node.
pub const EXACT_FUNCTION_LOWERING_WORK_LIMIT: u64 = 65_536;

/// Name of the generated crate.
pub const EXACT_FUNCTION_CRATE_NAME: &str = "quire-exact-function-oracles";

// ---------------------------------------------------------------------------
// Request types
// ---------------------------------------------------------------------------

/// One declared function parameter: its name and its declared type, named as
/// a V2 type node id (`scalar_type` or `composite_type`).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FunctionParameter {
    /// Parameter name.
    pub name: String,
    /// V2 node id of the parameter's declared type.
    pub type_node_id: CheckedNodeId,
}

/// The classified shape of a declared function's body, scoped per the
/// module-level doc comment.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExactFunctionBody {
    /// A `binary` scalar expression body (FR-014's forms), scoped to
    /// unbounded integer arithmetic.
    Scalar {
        /// The binary integer operator the body applies.
        operator: IntegerOperator,
    },
    /// A `binary` equality expression body (FR-018's forms), scoped to
    /// `Boolean`/`Integer` operands.
    CompositeEquality {
        /// `=` or `!=`.
        operator: EqualityOperatorKind,
    },
    /// A nested `call` expression body naming another declared function in
    /// the same request.
    Call {
        /// The name of the applied function.
        callee: String,
    },
}

/// One declared function: its own checked body node, its declared
/// parameters and result type, and its classified body.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExactFunctionDeclaration {
    /// Checked expression node this function's body lowers from.
    pub node_id: CheckedNodeId,
    /// Function name, unique within the request.
    pub name: String,
    /// Declared parameters, in order.
    pub parameters: Vec<FunctionParameter>,
    /// V2 node id of the declared result type.
    pub result_type: CheckedNodeId,
    /// The body's classified shape.
    pub body: ExactFunctionBody,
    /// Named capabilities this function's operator requirements demand
    /// (V2's `IeeeItemRequirement`/`IntegerDivisionConsumer` lists, carried
    /// by name only in this V1). This generator registers no backend for
    /// any capability at all, so a non-empty list always negotiates
    /// `unsupported` (AC-6) -- mirroring FR-273-AC-4's pre-application
    /// negotiation, decided at generation time, before any item naming the
    /// function is applied and before any `Meter` is touched.
    ///
    /// Unlike `capability_requirements`, this request type carries no field
    /// at all for the underlying `IeeeItemRequirement`/`IntegerDivisionConsumer`
    /// lists themselves. This request refuses any non-empty named capability
    /// before application because no backend is registered. The checked QSL
    /// graph and application facade own the corresponding runtime
    /// requirements; this request cannot invent or discharge them.
    pub capability_requirements: Vec<String>,
}

/// One requested oracle: a checked `call` expression node applying one
/// declared function.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExactFunctionItem {
    /// Checked `call` expression node to generate.
    pub call_node_id: CheckedNodeId,
    /// Name of the applied function; must name a declaration in the same
    /// request.
    pub function: String,
    /// Source node ids of the argument operands, in parameter order.
    pub argument_node_ids: Vec<CheckedNodeId>,
}

// ---------------------------------------------------------------------------
// Upstream blockers and refusals
// ---------------------------------------------------------------------------

/// Why one declared function or requested item generated no code.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum ExactFunctionRefusal {
    /// Exact refusal and source occurrences from admitted FR-330 accounting.
    PackageAccounting {
        /// Reader-provided disposition.
        record: Box<ContractFeatureRecordV1>,
        /// Exact source occurrences at the requested position.
        occurrences: Vec<CheckedSourceMapEntry>,
    },
    /// Selected external targets require a separate resolver.
    SelectedDependencyUnsupported {
        /// Ordered `(package_id, target_node_id)` pairs from the admitted closure.
        references: Vec<(CheckedSemanticId, CheckedNodeId)>,
    },
    /// The item's `(call node id, function, argument node ids)` triple was
    /// requested more than once. The claim map records exactly one
    /// `Refused` entry for that key, not one per copy requested -- every
    /// copy shares one identity and collapses to it
    /// (`tc_031_ac1_duplicate_request_refuses_every_copy` asserts
    /// `items.len() == 1`).
    DuplicateRequest,
    /// The requested node is not in the admitted graph.
    InvalidInput,
    /// The node is not an expression.
    NotExpression {
        /// Its family.
        node_tag: &'static str,
    },
    /// The expression form is not the one this body kind expects.
    FormMismatch {
        /// Form found.
        found: String,
    },
    /// The body is not a two-argument `binary` application (scalar/equality
    /// bodies).
    BodyMismatch,
    /// The declared operator is outside this generator's V1 scalar/equality
    /// vocabulary (see module doc).
    UnsupportedOperator {
        /// What was requested.
        detail: &'static str,
    },
    /// The declared parameter count, or a resolved parameter/result operand
    /// kind, disagrees with what the function's own body kind requires (a
    /// `Scalar` body needs exactly two `Integer` parameters and an
    /// `Integer` result; a `CompositeEquality` body needs exactly two
    /// parameters sharing one operand kind and a `Boolean` result) --
    /// caught once every parameter and result type is resolved, before the
    /// function can enter the assembled package with a signature the
    /// rendered body can never actually satisfy.
    SignatureMismatch {
        /// Which part of the signature disagreed, and why.
        detail: &'static str,
    },
    /// The item, or a declared function's own nested `call` body, names a
    /// function this request declares more than once. `call` could not
    /// tell which declaration is meant -- the same ambiguity the runtime's
    /// own `CheckCause::AmbiguousName` refuses at package admission, caught
    /// here instead, before Stage 1 classification even runs, so two
    /// declarations sharing one name never both enter the assembled
    /// package.
    AmbiguousFunctionName {
        /// The ambiguous name.
        name: String,
    },
    /// The declaration, or the item naming one, shares its declaring node id
    /// with another declaration in the request (FR-021-AC-22). Every
    /// declaration holding that node id is refused before Stage 1
    /// classification, so none enters the assembled package, and an item
    /// naming any of them gets this refusal rather than
    /// [`ExactFunctionRefusal::UnknownFunction`] or
    /// [`ExactFunctionRefusal::AmbiguousFunctionName`]: this check takes
    /// precedence over the name check. A nested `call` naming such a
    /// declaration is refused as [`ExactFunctionRefusal::UnknownCallee`].
    DuplicateDeclaringNode {
        /// The shared declaring node id. When an item's name is held by
        /// duplicate groups on more than one node id, the smallest.
        node_id: CheckedNodeId,
    },
    /// The function's declared operator requirements name a capability no
    /// registered backend can discharge (AC-6). Decided at generation time,
    /// before any item naming the function is applied; no `Meter` is
    /// touched to reach this disposition.
    UnsupportedCapability {
        /// The first named, undischargeable capability.
        capability: String,
    },
    /// A declared parameter or result type is not one of this generator's
    /// supported operand kinds (`Boolean`, unbounded `Integer`).
    UnsupportedOperandType {
        /// The unsupported type node.
        type_node_id: CheckedNodeId,
    },
    /// A parameter/result type node, or a node reached while classifying a
    /// function, is not in the admitted graph.
    UnknownTypeNode {
        /// The unresolved node id.
        type_node_id: CheckedNodeId,
    },
    /// A reachable node's family awaits upstream semantics.
    BlockedOnUpstream {
        /// The blocked node.
        unsupported_node_id: CheckedNodeId,
        /// Its family or form.
        node_tag: &'static str,
        /// The upstream issue.
        issue: UpstreamBlocker,
    },
    /// The item names a function absent from the request's own declarations.
    UnknownFunction {
        /// The requested function name.
        name: String,
    },
    /// A nested `call` body names a callee absent from the request's own
    /// declarations, or one that itself failed to classify.
    UnknownCallee {
        /// The requested callee name.
        callee: String,
    },
    /// The item's argument count disagrees with the applied function's
    /// declared parameter count.
    ArityMismatch {
        /// Declared parameter count.
        expected: usize,
        /// Requested argument count.
        found: usize,
    },
    /// Lowering exceeded [`EXACT_FUNCTION_LOWERING_WORK_LIMIT`]; the refusal of the work ceiling
    /// and of nothing else.
    LoweringWorkExhausted {
        /// The ceiling.
        limit: u64,
        /// Counter at the failed charge.
        consumed: u64,
    },
    /// The package's byte ceiling failed the lowering (Contract IR FR-038-AC-95).
    LoweringByteLimitExceeded {
        /// The ceiling the package was read under.
        limit: u64,
        /// The canonical byte count the encoder needed.
        consumed: u64,
    },
    /// A `failed` lowering record names a limit kind that is neither `work` nor `bytes`.
    LoweringLimitUnrecognised {
        /// The snake_case name of the `CheckedPackageLimit` variant.
        limit_kind: &'static str,
        /// The record's ceiling.
        limit: u64,
        /// The record's counter.
        consumed: u64,
    },
    /// A reachable node's family has no finite exact encoding this
    /// generator reads.
    Unsupported {
        /// First unsupported reachable node.
        unsupported_node_id: CheckedNodeId,
        /// Its family or form.
        node_tag: &'static str,
    },
    /// The retained checked closure or selected callable disagrees with the IR-admitted identity.
    OwnerIdentityMismatch,
    /// QSL did not issue a callable selection.
    FacadeLookup {
        /// Stable CG cause code, mapped exhaustively from QSL's typed refusal.
        cause_code: &'static str,
        /// The original typed refusal rendered for diagnostics.
        detail: String,
    },
    /// QSL did not issue a checked nested-call location for the selected body.
    NestedCallSite {
        /// Stable CG cause code, preserving the typed source or facade refusal.
        cause_code: &'static str,
        /// The original typed refusal rendered for diagnostics.
        detail: String,
    },
    /// The leaf refused generated-body registration.
    RegistrationNotAccepted {
        /// Stable CG cause code, mapped exhaustively from the leaf admission result.
        cause_code: &'static str,
        /// The original typed result rendered for diagnostics.
        detail: String,
    },
    /// The selection did not belong to the registry's acquisition.
    SelectionRefused {
        /// Stable CG cause code, mapped exhaustively from QSL's typed refusal.
        cause_code: &'static str,
        /// The original typed refusal rendered for diagnostics.
        detail: String,
    },
    /// Begin refused before any generated body or application charge.
    ApplicationBeginRefused {
        /// Stable CG cause code, mapped exhaustively from QSL's typed refusal.
        cause_code: &'static str,
        /// The original typed refusal rendered for diagnostics.
        detail: String,
    },
}

// ---------------------------------------------------------------------------
// Claim map and location map
// ---------------------------------------------------------------------------

/// Static V1 body origin. Claim records use request order; location records
/// use admitted body source order. QSL owns dynamic application locations.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum RecordedOrigin {
    /// The `index`-th declared function's own body.
    Body {
        /// The function's name.
        function: String,
        /// The function's index in its assembled package.
        index: usize,
    },
}

/// Static V1 location recorded for a classified function body.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RecordedLocation {
    /// The origin.
    pub origin: RecordedOrigin,
    /// The child-index path from the origin. Always empty in this
    /// generator's V1: every emitted function body reaches exactly one
    /// classified call point, at its own root (see module doc).
    pub path: Vec<usize>,
}

/// Which runtime surface a location map entry's call point reaches.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CallPointKind {
    /// A scalar operator (`evaluate_integer_arithmetic`).
    ScalarOperator,
    /// An equality evaluation (`CheckedEquality::evaluate`).
    EqualityEvaluation,
    /// A nested `Frame::call`.
    NestedCall,
}

/// One location map entry: the runtime call point one generated function
/// body reaches, and the `Location` it is tagged with.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct LocationMapEntry {
    /// The function whose body reaches this call point.
    pub function: String,
    /// What kind of call point it is.
    pub call_point: CallPointKind,
    /// The tagged location.
    pub location: RecordedLocation,
}

/// Traceability for one generated oracle.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct GeneratedExactFunctionClaim {
    /// Generated oracle function name.
    pub oracle_symbol: String,
    /// Identity of the one Contract IR package that admitted this claim.
    pub contract_package_identity: CheckedSemanticId,
    /// Exact requested node key.
    pub node_id: CheckedNodeId,
    /// Actual declaring V2 function node, distinct from the body-root key.
    pub function_node_id: CheckedNodeId,
    /// Package id.
    pub package_id: CheckedSemanticId,
    /// Semantic type key of the `call` expression node.
    pub semantic_type: CheckedNodeId,
    /// Exact source correspondence.
    pub source_map: Vec<CheckedSourceMapEntry>,
    /// Reachable claim keys.
    pub claims: Vec<CheckedNodeId>,
    /// Reachable bounded-domain keys.
    pub bounds: Vec<CheckedNodeId>,
    /// Every reached key except the root.
    pub dependencies: Vec<CheckedNodeId>,
    /// The applied function's name.
    pub function: String,
    /// The applied function's `Origin::Body { function, index }`, indexed in
    /// canonical body-root node order among surviving declarations (AC-3).
    pub function_origin: RecordedOrigin,
    /// Reconstructed declaration closure keys. Always empty in this V1: no
    /// requested item's supported operand types reach a composite
    /// declaration.
    pub declaration_keys: Vec<CheckedNodeId>,
}

/// One claim-map entry per distinct requested item.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ExactFunctionClaim {
    /// Requested `call` expression node.
    pub node_id: CheckedNodeId,
    /// Outcome.
    pub result: ClaimDisposition<GeneratedExactFunctionClaim, ExactFunctionRefusal>,
}

/// Generation output: the crate files, the claim map, and the static
/// location map.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExactFunctionOracles {
    /// `Cargo.toml`, `src/lib.rs`, `claim-map.json` and `location-map.json`,
    /// in that order.
    pub artifacts: Vec<Artifact>,
    /// Typed claim map, identical to `claim-map.json`. Items are ordered by
    /// the item key (FR-021-AC-13). `blocked` is always empty: every blocker
    /// is recorded on its own entry's refusal. A `Generated` disposition
    /// means one oracle function was emitted. The blockers this generator
    /// records are `QuireSpecLanguage120` and `QuireSpecLanguage121`.
    pub claim_map: ClaimMap<ExactFunctionClaim>,
    /// Typed location map, identical to `location-map.json`.
    pub location_map: Vec<LocationMapEntry>,
}

// ---------------------------------------------------------------------------
// Generation
// ---------------------------------------------------------------------------

type Graph<'a> = CheckedGraph<'a>;

/// Ordered key whose comparisons expose actual indexed lookup work to tests.
#[derive(Eq, PartialEq)]
struct IndexedNodeId(CheckedNodeId);

impl Ord for IndexedNodeId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        #[cfg(test)]
        FUNCTION_PREFLIGHT_KEY_COMPARISONS.with(|count| count.set(count.get() + 1));
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for IndexedNodeId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Keep source-call matching identical for borrowed generation graphs and preflight indexes.
trait NodeLookup<'a> {
    type Body: 'a;

    fn get_node(&self, node_id: &CheckedNodeId) -> Option<&'a CheckedSemanticNodeV2<Self::Body>>;
    fn get_term(&self, node_id: &CheckedNodeId) -> Option<&'a CheckedTerm>;
}

impl<'a, Body: GraphBody + 'a> NodeLookup<'a> for CheckedGraph<'a, Body> {
    type Body = Body;

    fn get_node(&self, node_id: &CheckedNodeId) -> Option<&'a CheckedSemanticNodeV2<Self::Body>> {
        self.node(node_id)
    }
    fn get_term(&self, node_id: &CheckedNodeId) -> Option<&'a CheckedTerm> {
        self.term(node_id)
    }
}

/// One checked-graph lookup view shared by function preflight items in a request.
pub(crate) struct FunctionPreflightGraph<'a> {
    // Metadata-only private fixtures have no admitted owner; production new() always sets it.
    checked: Option<&'a CheckedPackageV2>,
    package_id: CheckedSemanticId,
    nodes: BTreeMap<IndexedNodeId, &'a CheckedSemanticNodeV2>,
}

impl<'a> FunctionPreflightGraph<'a> {
    /// Index the admitted graph once, before classifying its function items.
    pub(crate) fn new(checked: &'a CheckedPackageV2) -> Self {
        let mut graph = Self::from_nodes(&checked.graph().nodes, checked.package_id().clone());
        graph.checked = Some(checked);
        graph
    }

    fn from_nodes(nodes: &'a [CheckedSemanticNodeV2], package_id: CheckedSemanticId) -> Self {
        let nodes = nodes
            .iter()
            .map(|node| {
                #[cfg(test)]
                FUNCTION_PREFLIGHT_GRAPH_VISITS.with(|count| count.set(count.get() + 1));
                (IndexedNodeId(node.node_id.clone()), node)
            })
            .collect();
        Self {
            checked: None,
            package_id,
            nodes,
        }
    }

    /// Resolve one node from the view's admitted package graph.
    pub(crate) fn get(&self, node_id: &CheckedNodeId) -> Option<&'a CheckedSemanticNodeV2> {
        #[cfg(test)]
        let before = FUNCTION_PREFLIGHT_KEY_COMPARISONS.with(std::cell::Cell::get);
        let found = self.nodes.get(&IndexedNodeId(node_id.clone())).copied();
        #[cfg(test)]
        FUNCTION_PREFLIGHT_GRAPH_LOOKUP_WORK.with(|work| {
            work.borrow_mut()
                .push(FUNCTION_PREFLIGHT_KEY_COMPARISONS.with(std::cell::Cell::get) - before);
        });
        found
    }
}

impl<'a> NodeLookup<'a> for FunctionPreflightGraph<'a> {
    type Body = CheckedTerm;

    fn get_node(&self, node_id: &CheckedNodeId) -> Option<&'a CheckedSemanticNodeV2> {
        self.get(node_id)
    }
    fn get_term(&self, node_id: &CheckedNodeId) -> Option<&'a CheckedTerm> {
        self.checked?.term(node_id)
    }
}

fn reference_in_graph<'a>(
    term: &CheckedTerm,
    graph: &impl NodeLookup<'a>,
) -> Option<&'a CheckedNodeId> {
    let CheckedTerm::Reference { target } = term else {
        return None;
    };
    graph.get_node(target).map(|node| &node.node_id)
}

/// Check that the admitted application names this exact function and ordered argument list.
fn source_call_matches<'a>(
    body: &CheckedTerm,
    graph: &impl NodeLookup<'a>,
    function_node_id: &CheckedNodeId,
    argument_node_ids: &[CheckedNodeId],
) -> bool {
    if !graph.get_node(function_node_id).is_some_and(|node| {
        &*node.node_tag == "function" && &*node.semantic_form == "pure_function"
    }) {
        return false;
    }
    let CheckedTerm::Application {
        operator: ApplicationOperator::Call,
        arguments,
        ..
    } = body
    else {
        return false;
    };
    if application_identity(body) != Some("quire.op.function.call") {
        return false;
    }
    if arguments.len() != argument_node_ids.len() + 1 {
        return false;
    }
    let mut expected = std::iter::once(function_node_id).chain(argument_node_ids.iter());
    arguments.iter().all(|term| {
        reference_in_graph(term, graph).is_some_and(|actual_id| {
            expected
                .next()
                .is_some_and(|expected_id| actual_id == expected_id)
        })
    }) && expected.next().is_none()
}

/// Resolve the source function's parameter nodes in declared order. The
/// generated body takes positional arguments, so every body reference must
/// use this exact sequence before it can be rendered.
fn source_parameter_ids<'a>(
    graph: &impl NodeLookup<'a>,
    function_node_id: &CheckedNodeId,
) -> Option<Vec<CheckedNodeId>> {
    let parameters = graph
        .get_term(function_node_id)
        .and_then(|term| binding_value(term, "parameters"))
        .and_then(aggregate_members)?;
    parameters
        .iter()
        .map(|term| reference_in_graph(term, graph).cloned())
        .collect()
}

fn source_argument_refs_match<'a>(
    graph: &impl NodeLookup<'a>,
    function_node_id: &CheckedNodeId,
    arguments: &[CheckedTerm],
) -> bool {
    let Some(parameters) = source_parameter_ids(graph, function_node_id) else {
        return false;
    };
    arguments.len() == parameters.len()
        && arguments
            .iter()
            .zip(&parameters)
            .all(|(argument, parameter)| reference_in_graph(argument, graph) == Some(parameter))
}

fn source_nested_body_matches<Body: GraphBody>(
    index: &FunctionSourceIndex<'_, Body>,
    caller_name: &str,
    caller_body: &CheckedNodeId,
    callee_name: &str,
    callee_body: &CheckedNodeId,
) -> bool {
    let Some(caller) = index.source_function_node_id(caller_name, caller_body) else {
        return false;
    };
    let Some(callee) = index.source_function_node_id(callee_name, callee_body) else {
        return false;
    };
    let Some(body) = index.graph.term(caller_body) else {
        return false;
    };
    let CheckedTerm::Application {
        operator: ApplicationOperator::Call,
        arguments,
        ..
    } = body
    else {
        return false;
    };
    application_identity(body) == Some("quire.op.function.call")
        && arguments
            .first()
            .and_then(|term| reference_in_graph(term, &index.graph))
            == Some(callee)
        && arguments
            .get(1..)
            .is_some_and(|arguments| source_argument_refs_match(&index.graph, caller, arguments))
}

struct FunctionSourceIndex<'a, Body: GraphBody = serde_json::Value> {
    graph: CheckedGraph<'a, Body>,
    by_name_body: BTreeMap<(&'a str, &'a CheckedNodeId), Option<&'a CheckedNodeId>>,
}

impl<'a, Body: GraphBody> FunctionSourceIndex<'a, Body> {
    fn new(graph: CheckedGraph<'a, Body>) -> Self {
        let mut by_name_body = BTreeMap::new();
        for node in graph.nodes() {
            #[cfg(test)]
            IR988_SOURCE_GRAPH_VISITS.with(|visits| visits.set(visits.get() + 1));
            if &*node.node_tag != "function" || &*node.semantic_form != "pure_function" {
                continue;
            }
            let Some(name) = node.declaration.as_ref().and_then(|declaration| {
                (declaration.qualified_name.len() == 1)
                    .then(|| declaration.qualified_name[0].as_ref())
            }) else {
                continue;
            };
            let Some(body) = graph
                .term(&node.node_id)
                .and_then(|body| binding_value(body, "body"))
                .and_then(|term| reference_in_graph(term, &graph))
            else {
                continue;
            };
            by_name_body
                .entry((name, body))
                .and_modify(|matching| *matching = None)
                .or_insert(Some(&node.node_id));
        }
        Self {
            graph,
            by_name_body,
        }
    }

    fn source_function_node_id(
        &self,
        name: &str,
        body_node_id: &CheckedNodeId,
    ) -> Option<&'a CheckedNodeId> {
        self.by_name_body
            .get(&(name, body_node_id))
            .copied()
            .flatten()
    }
}

#[cfg(test)]
thread_local! {
    static IR988_SOURCE_GRAPH_VISITS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

#[cfg(test)]
mod ir988_source_function_tests {
    use super::*;
    use ix_trace_rs::trace;
    use quire_contract_model::CheckedDeclaration;
    use serde_json::json;

    fn id(value: u64) -> CheckedNodeId {
        serde_json::from_value(json!({
            "domain":"quire.checked-semantic-node/v1",
            "digest":format!("{value:064x}")
        }))
        .expect("synthetic node key")
    }

    fn node(
        value: u64,
        name: Option<String>,
        body: &CheckedNodeId,
    ) -> crate::oracle::checked_graph::fixture::Node {
        let metadata = CheckedSemanticNodeV2 {
            node_id: id(value),
            owner: None,
            schema_version: "quire.checked-semantic-graph/v2".into(),
            node_tag: if name.is_some() {
                "function"
            } else {
                "expression"
            }
            .into(),
            semantic_form: if name.is_some() {
                "pure_function"
            } else {
                "binary"
            }
            .into(),
            semantic_type: body.clone(),
            dependencies: Vec::new(),
            occurrences: Vec::new(),
            recursion_group: None,
            nominal_identity_preimage: None,
            declaration: name.map(|name| CheckedDeclaration {
                qualified_name: vec![name.into()],
            }),
            body: CheckedTerm::aggregate(Vec::new()),
        };
        crate::oracle::checked_graph::fixture::Node::new(
            metadata,
            Some(CheckedTerm::Aggregate {
                members: vec![CheckedTerm::Binding {
                    name: "body".into(),
                    value: Box::new(CheckedTerm::Reference {
                        target: body.clone(),
                    }),
                }],
            }),
        )
    }

    #[trace("TC-031")]
    #[test]
    fn ir_988_source_function_lookup_builds_once_for_repeated_items() {
        let mut excessive = Vec::new();
        for requests in [2, 4, 8] {
            for functions in [4, 8, 16] {
                let body = id(10_000);
                let mut nodes = vec![node(10_000, None, &body)];
                nodes.extend(
                    (0..functions)
                        .map(|index| node(index as u64 + 1, Some(format!("f{index}")), &body)),
                );
                let graph: CheckedGraph<'_, CheckedTerm> =
                    nodes.iter().map(|node| (&node.node_id, node)).collect();
                IR988_SOURCE_GRAPH_VISITS.with(|visits| visits.set(0));
                let index = FunctionSourceIndex::new(graph);
                for _ in 0..requests {
                    let found =
                        index.source_function_node_id(&format!("f{}", functions - 1), &body);
                    assert_eq!(found, Some(&id(functions as u64)));
                }
                let visits = IR988_SOURCE_GRAPH_VISITS.with(std::cell::Cell::get);
                println!("K={requests} R={functions} source_graph_visits={visits}");
                if visits > functions + requests + 1 {
                    excessive.push((requests, functions, visits));
                }
            }
        }
        assert!(excessive.is_empty(), "per-item graph scans: {excessive:?}");
    }
}

#[cfg(test)]
#[allow(clippy::duplicate_mod)] // Reuse the admitted fixture in this test binary.
#[path = "../../../tests/exact_function_support/package.rs"]
mod ir988_function_fixture;

#[cfg(test)]
mod ir988_admitted_generation_tests {
    use super::*;
    use ix_trace_rs::trace;
    use serde_json::json;

    use super::ir988_function_fixture as fixture;

    fn source_reference(code: u32) -> serde_json::Value {
        let id = fixture::code_id(code);
        json!({"term":"reference","target":{"domain":id.domain,"digest":id.digest}})
    }

    struct AdmittedCase {
        contract: ContractPackageV1,
        positions: Vec<ContractRequestPosition>,
        function_positions: Vec<usize>,
        declarations: Vec<ExactFunctionDeclaration>,
        item_positions: Vec<usize>,
        items: Vec<ExactFunctionItem>,
    }

    fn admitted_case(requests: usize, function_count: usize) -> AdmittedCase {
        use fixture::*;

        let mut builder = source_bound_call_package();
        for offset in 0..(function_count - 2) {
            builder.code(
                40_000 + offset as u32,
                "function",
                "pure_function",
                T_INTEGER,
                aggregate(vec![
                    json!({"term":"binding","name":"parameters","value":aggregate(vec![
                        source_reference(SOURCE_PARAM_A), source_reference(SOURCE_PARAM_B)])}),
                    json!({"term":"binding","name":"body","value":source_reference(SOURCE_BODY)}),
                ]),
            );
        }
        for offset in 0..2 {
            let parameter = 41_000 + offset;
            builder.code(
                parameter,
                "value",
                "parameter",
                T_INTEGER,
                aggregate(vec![
                    bound_member("name", literal("text", &format!("p{offset}"))),
                    bound_member("level", integer_literal(0)),
                ]),
            );
        }
        let arguments = [
            (41_000, SOURCE_PARAM_B),
            (41_001, SOURCE_PARAM_B),
            (SOURCE_PARAM_A, SOURCE_PARAM_A),
            (SOURCE_PARAM_B, SOURCE_PARAM_A),
            (SOURCE_PARAM_B, SOURCE_PARAM_B),
            (SOURCE_PARAM_B, 41_000),
            (SOURCE_PARAM_B, 41_001),
            (41_000, SOURCE_PARAM_A),
        ];
        for (offset, &(left, right)) in arguments.iter().take(requests).enumerate() {
            builder.application_node(
                42_000 + offset as u32,
                "call",
                &key(T_INTEGER),
                json!({
                    "term":"application","operator":"call",
                    "operation":{"identity":"quire.op.function.call","laws":[],"mode":null,
                        "member":null,"leaves":[]},
                    "result_type":{"domain":NODE_DOMAIN,"digest":key(T_INTEGER)},
                    "arguments":[source_reference(SOURCE_FUNCTION),
                        source_reference(left),source_reference(right)]
                }),
            );
        }
        let (checked, ids) = builder.admit_resolved();
        let name = format!("n{}", key(SOURCE_FUNCTION));
        let other_name = format!("n{}", key(SOURCE_OTHER_FUNCTION));
        let functions = vec![
            ids.resolve_function(source_function(&name, SOURCE_BODY)),
            ids.resolve_function(source_function(&other_name, SOURCE_OTHER_BODY)),
        ];
        let mut requested = functions
            .iter()
            .map(|function| function.node_id.clone())
            .collect::<Vec<_>>();
        requested.push(ids.resolve(&code_id(SOURCE_OTHER_FUNCTION)));
        requested.extend(
            (0..(function_count - 2)).map(|offset| ids.resolve(&code_id(40_000 + offset as u32))),
        );
        let function_positions = [0, 1].to_vec();
        let mut items = Vec::new();
        let mut item_positions = Vec::new();
        for (offset, &(left, right)) in arguments.iter().take(requests).enumerate() {
            let item = ExactFunctionItem {
                call_node_id: code_id(42_000 + offset as u32),
                function: name.clone(),
                argument_node_ids: vec![code_id(left), code_id(right)],
            };
            item_positions.push(requested.len());
            requested.push(ids.resolve(&item.call_node_id));
            items.push(ids.resolve_call(item));
        }
        let mut refused = items[0].clone();
        refused.function = other_name;
        item_positions.push(requested.len());
        requested.push(refused.call_node_id.clone());
        items.push(refused);
        let contract = admitted_function_contract(&checked, &requested);
        let positions = driver_positions(&requested);
        AdmittedCase {
            contract,
            positions,
            function_positions,
            declarations: functions,
            item_positions,
            items,
        }
    }

    #[trace("TC-031")]
    #[test]
    fn ir_988_public_admitted_generation_indexes_source_once_and_preserves_outputs() {
        for requests in [2, 4, 8] {
            for functions in [4, 8, 16] {
                let AdmittedCase {
                    contract,
                    positions,
                    function_positions,
                    declarations,
                    item_positions,
                    items,
                } = admitted_case(requests, functions);
                IR988_SOURCE_GRAPH_VISITS.with(|visits| visits.set(0));
                let output = generate_exact_function_oracles(
                    &contract,
                    &positions,
                    &function_positions,
                    &declarations,
                    &item_positions,
                    &items,
                )
                .expect("admitted function generation");
                let visits = IR988_SOURCE_GRAPH_VISITS.with(std::cell::Cell::get);
                let graph_nodes = contract.wire().semantic_graph.nodes.len();
                assert_eq!(
                    contract
                        .wire()
                        .semantic_graph
                        .nodes
                        .iter()
                        .filter(|node| node.node_tag.as_ref() == "function")
                        .count(),
                    functions,
                );
                println!("K={requests} R={functions} G={graph_nodes} visits={visits}");
                assert_eq!(
                    visits, graph_nodes,
                    "one source index build per admitted batch"
                );
                let repeated = generate_exact_function_oracles(
                    &contract,
                    &positions,
                    &function_positions,
                    &declarations,
                    &item_positions,
                    &items,
                )
                .expect("repeat generation from the same admitted package");
                assert_eq!(
                    output, repeated,
                    "complete output changed across repeat runs"
                );
                if let Some(directory) = std::env::var_os("IR988_OUTPUT_DIR") {
                    let directory = std::path::PathBuf::from(directory);
                    std::fs::create_dir_all(&directory).expect("evidence directory");
                    std::fs::write(
                        directory.join(format!("k{requests}-r{functions}.txt")),
                        format!("{output:#?}"),
                    )
                    .expect("full output evidence");
                }
                assert_eq!(output.claim_map.items.len(), requests + 1);
                assert_eq!(
                    output
                        .claim_map
                        .items
                        .iter()
                        .filter(|item| matches!(item.result, ClaimDisposition::Generated(_)))
                        .count(),
                    requests
                );
                assert_eq!(
                    output
                        .claim_map
                        .items
                        .iter()
                        .filter(|item| matches!(
                            item.result,
                            ClaimDisposition::Refused {
                                refusal: ExactFunctionRefusal::BodyMismatch
                            }
                        ))
                        .count(),
                    1
                );
            }
        }
    }
}

pub(crate) fn binding_value<'a>(body: &'a CheckedTerm, name: &str) -> Option<&'a CheckedTerm> {
    aggregate_members(body)?
        .iter()
        .find_map(|member| match member {
            CheckedTerm::Binding { name: found, value } if found.as_ref() == name => {
                Some(value.as_ref())
            }
            _ => None,
        })
}

#[cfg(test)]
pub(crate) fn reference_node_id<'a>(
    term: &CheckedTerm,
    mut nodes: impl Iterator<Item = &'a CheckedSemanticNodeV2>,
) -> Option<&'a CheckedNodeId> {
    let CheckedTerm::Reference { target } = term else {
        return None;
    };
    nodes
        .find(|node| &node.node_id == target)
        .map(|node| &node.node_id)
}

#[cfg(test)]
thread_local! {
    static FUNCTION_PREFLIGHT_READS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static FUNCTION_PREFLIGHT_INDICES: std::cell::RefCell<Vec<usize>> = const { std::cell::RefCell::new(Vec::new()) };
    static FUNCTION_PREFLIGHT_GRAPH_VISITS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static FUNCTION_PREFLIGHT_KEY_COMPARISONS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static FUNCTION_PREFLIGHT_GRAPH_LOOKUP_WORK: std::cell::RefCell<Vec<usize>> = const { std::cell::RefCell::new(Vec::new()) };
}

#[cfg(test)]
pub(crate) fn take_function_preflight_reads() -> usize {
    FUNCTION_PREFLIGHT_READS.with(|count| count.replace(0))
}

#[cfg(test)]
pub(crate) fn take_function_preflight_indices() -> Vec<usize> {
    FUNCTION_PREFLIGHT_INDICES.with(|indices| std::mem::take(&mut *indices.borrow_mut()))
}

#[cfg(test)]
pub(crate) fn take_function_preflight_graph_visits() -> usize {
    FUNCTION_PREFLIGHT_GRAPH_VISITS.with(|count| count.replace(0))
}

#[cfg(test)]
pub(crate) fn take_function_preflight_graph_lookup_work() -> Vec<usize> {
    FUNCTION_PREFLIGHT_GRAPH_LOOKUP_WORK.with(|work| std::mem::take(&mut *work.borrow_mut()))
}

/// Non-admitted result of the internal function-package reader.
pub(crate) enum FunctionPackageReadFailure {
    Refused(Box<CheckedPackageRefusal>),
    Incomplete(CheckedPackageIncomplete),
}

/// Classify a function item using the request's shared admitted-graph view.
pub(crate) fn checked_integer_function_application_is_supported_with_graph(
    checked: &CheckedPackageV2,
    graph: &FunctionPreflightGraph<'_>,
    function_node_id: &CheckedNodeId,
    call_node_id: &CheckedNodeId,
    argument_node_ids: &[CheckedNodeId],
    admitted: Option<(&ContractPackageV1, &[ContractRequestPosition], usize)>,
) -> Result<bool, FunctionPackageReadFailure> {
    if &graph.package_id != checked.package_id() {
        return Ok(false);
    }
    #[cfg(test)]
    if let Some((_, _, request_index)) = admitted {
        FUNCTION_PREFLIGHT_INDICES.with(|indices| indices.borrow_mut().push(request_index));
    }
    let Some(function_node) = graph.get(function_node_id) else {
        return Ok(false);
    };
    if &*function_node.node_tag != "function" || &*function_node.semantic_form != "pure_function" {
        return Ok(false);
    }
    let Some(name_segments) = function_node
        .declaration
        .as_ref()
        .map(|declaration| declaration.qualified_name.as_slice())
    else {
        return Ok(false);
    };
    if name_segments.len() != 1 {
        return Ok(false);
    }
    let Some(parameter_terms) = graph
        .get_term(function_node_id)
        .and_then(|body| binding_value(body, "parameters"))
        .and_then(aggregate_members)
    else {
        return Ok(false);
    };
    let mut parameters = Vec::with_capacity(parameter_terms.len());
    for term in parameter_terms {
        let Some(parameter_id) = reference_in_graph(term, graph) else {
            return Ok(false);
        };
        let Some(parameter_node) = graph.get(parameter_id) else {
            return Ok(false);
        };
        let Some(name) = graph
            .get_term(parameter_id)
            .and_then(|body| binding_value(body, "name"))
            .and_then(|term| match term {
                CheckedTerm::Literal { value, .. } => value.as_str(),
                _ => None,
            })
        else {
            return Ok(false);
        };
        parameters.push(FunctionParameter {
            name: name.to_owned(),
            type_node_id: parameter_node.semantic_type.clone(),
        });
    }
    let Some(body_id) = graph
        .get_term(function_node_id)
        .and_then(|body| binding_value(body, "body"))
        .and_then(|term| reference_in_graph(term, graph))
    else {
        return Ok(false);
    };
    let Some(_body_node) = graph.get(body_id) else {
        return Ok(false);
    };
    let Some(operation_identity) = graph.get_term(body_id).and_then(application_identity) else {
        return Ok(false);
    };
    let operator = match operation_identity {
        "quire.op.integer.add" => IntegerOperator::Add,
        "quire.op.integer.sub" => IntegerOperator::Subtract,
        "quire.op.integer.mul" => IntegerOperator::Multiply,
        _ => return Ok(false),
    };
    let Some(_call_node) = graph.get(call_node_id) else {
        return Ok(false);
    };
    let Some(call_body) = graph.get_term(call_node_id) else {
        return Ok(false);
    };
    if !source_call_matches(call_body, graph, function_node_id, argument_node_ids) {
        return Ok(false);
    }
    let declaration = ExactFunctionDeclaration {
        node_id: body_id.clone(),
        name: name_segments[0].to_string(),
        parameters,
        result_type: function_node.semantic_type.clone(),
        body: ExactFunctionBody::Scalar { operator },
        capability_requirements: Vec::new(),
    };
    let item = ExactFunctionItem {
        call_node_id: call_node_id.clone(),
        function: name_segments[0].to_string(),
        argument_node_ids: argument_node_ids.to_vec(),
    };
    let requested_ids = [body_id.clone(), call_node_id.clone()];
    let owned_contract;
    let owned_positions;
    let (contract, positions, declaration_position, item_position) =
        if let Some((package, positions, request_index)) = admitted {
            let Some(body_index) = positions
                .iter()
                .filter(|position| !matches!(position, ContractRequestPosition::ContextRoot { .. }))
                .enumerate()
                .find_map(|(index, position)| match position {
                    ContractRequestPosition::DriverItem { node_id, .. } if node_id == body_id => {
                        Some(index)
                    }
                    ContractRequestPosition::DriverItem { .. }
                    | ContractRequestPosition::Excluded { .. }
                    | ContractRequestPosition::ContextRoot { .. } => None,
                })
            else {
                return Ok(false);
            };
            (package, positions, [body_index], [request_index])
        } else {
            let lowered = checked.lower(&requested_ids, &routed_contract_lowering_profile());
            let Some(bytes) = lowered.package.canonical_bytes() else {
                return Ok(false);
            };
            #[cfg(test)]
            FUNCTION_PREFLIGHT_READS.with(|count| count.set(count.get() + 1));
            owned_contract = match ContractPackageWireV1::read(
                bytes,
                CheckedPackageReadLimits::bounded(),
                &ContractDependencies::new(),
            ) {
                ContractPackageReadResult::Admitted(package) => *package,
                ContractPackageReadResult::Refused(refusal) => {
                    return Err(FunctionPackageReadFailure::Refused(Box::new(refusal)))
                }
                ContractPackageReadResult::Incomplete(incomplete) => {
                    return Err(FunctionPackageReadFailure::Incomplete(incomplete))
                }
            };
            owned_positions = requested_ids.map(|node_id| ContractRequestPosition::DriverItem {
                node_id,
                result_selection: ContractDriverResultSelection::NotApplicable,
            });
            (&owned_contract, owned_positions.as_slice(), [0], [1])
        };
    let declarations = [declaration];
    let items = [item];
    let Ok(oracles) = generate_exact_function_oracles(
        contract,
        positions,
        &declaration_position,
        &declarations,
        &item_position,
        &items,
    ) else {
        return Ok(false);
    };
    Ok(matches!(
        oracles.claim_map.items.first().map(|claim| &claim.result),
        Some(ClaimDisposition::Generated(_))
    ))
}

/// One supported operand kind: this generator's V1 scalar/equality operand
/// vocabulary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum OperandKind {
    Boolean,
    Integer,
}

/// A declared function that survived Stage 1 classification, ready to enter
/// the checked-closure renderer with the body shape that Stage 1 validated.
struct ClassifiedFunction<'r> {
    declaration: &'r ExactFunctionDeclaration,
    body: ClassifiedBody<'r>,
    body_node_id: CheckedNodeId,
}

/// What Stage 1 resolved for one function that passed its own checks: the classified body and
/// the operand kinds it resolved, carried forward so that Stage 2 reads them and never resolves
/// them a second time.
struct Stage1Shape<'r> {
    body: ClassifiedBody<'r>,
    body_node_id: CheckedNodeId,
}

/// The binary integer operators a scalar function body can apply: the
/// operators of [`IntegerOperator`] that have a two-operand runtime form.
/// `Negate` is unary and has no variant here, so a body that survives
/// classification cannot carry it.
#[derive(Clone, Copy)]
enum BinaryIntegerOperator {
    Add,
    Subtract,
    Multiply,
}

impl BinaryIntegerOperator {
    /// The binary form of `operator`, or `None` for the unary `Negate`.
    fn of(operator: IntegerOperator) -> Option<Self> {
        match operator {
            IntegerOperator::Add => Some(Self::Add),
            IntegerOperator::Subtract => Some(Self::Subtract),
            IntegerOperator::Multiply => Some(Self::Multiply),
            IntegerOperator::Negate => None,
        }
    }

    /// The `exact::IntegerArithmetic` variant this operator renders as.
    fn variant(self) -> &'static str {
        match self {
            Self::Add => "Add",
            Self::Subtract => "Subtract",
            Self::Multiply => "Multiply",
        }
    }
}

/// A survivor's body with its operator resolved, ready to render.
enum ClassifiedBody<'r> {
    Scalar(BinaryIntegerOperator),
    Equality(EqualityOperatorKind),
    Call(&'r str),
}

fn check_position(
    requested: &PackageItems<'_>,
    request_index: usize,
    node_id: &CheckedNodeId,
) -> Result<(), OracleGenerationError> {
    let accounted = requested
        .bound_driver_node(request_index)
        .map_err(OracleGenerationError::PackageRequest)?;
    if accounted != node_id {
        return Err(OracleGenerationError::PackageRequest(
            PackageRequestError::Identity {
                request_index,
                requested: node_id.clone(),
                accounted: accounted.clone(),
            },
        ));
    }
    Ok(())
}

fn admitted_binary_body<'a>(
    item: PackageItem<'a>,
    graph: &Graph<'_>,
) -> Result<(ContractReachedClosureV1<'a>, &'a [CheckedSourceMapEntry]), ExactFunctionRefusal> {
    let (closure, occurrences) = match item {
        PackageItem::Refused {
            record,
            occurrences,
        } => {
            if let ContractFeatureRecordV1::Unsupported {
                unsupported_node_id,
                node_tag,
                ..
            } = record
            {
                if let Some(
                    tag @ (CheckedNodeTag::Model
                    | CheckedNodeTag::Relation
                    | CheckedNodeTag::State
                    | CheckedNodeTag::Temporal
                    | CheckedNodeTag::Protocol),
                ) = CheckedNodeTag::from_wire(node_tag.as_ref())
                {
                    return Err(unsupported_family(unsupported_node_id, tag));
                }
                return Err(ExactFunctionRefusal::Unsupported {
                    unsupported_node_id: unsupported_node_id.clone(),
                    node_tag: CheckedNodeTag::from_wire(node_tag.as_ref())
                        .map_or("unknown", CheckedNodeTag::as_wire),
                });
            }
            return Err(ExactFunctionRefusal::PackageAccounting {
                record: Box::new(record.clone()),
                occurrences: occurrences.to_vec(),
            });
        }
        PackageItem::Lowered {
            closure,
            occurrences,
            ..
        } => (closure, occurrences),
    };
    if !closure.selected_references.is_empty() {
        return Err(ExactFunctionRefusal::SelectedDependencyUnsupported {
            references: closure
                .selected_references
                .iter()
                .map(|reference| (reference.package.clone(), reference.node.clone()))
                .collect(),
        });
    }
    if let Some(node_tag) = direct_unsupported_form(closure.root, graph.term(&closure.root.node_id))
    {
        return Err(ExactFunctionRefusal::Unsupported {
            unsupported_node_id: closure.root.node_id.clone(),
            node_tag,
        });
    }
    let tag = CheckedNodeTag::from_wire(&closure.root.node_tag);
    if tag != Some(CheckedNodeTag::Expression) {
        return Err(tag.map_or_else(
            || ExactFunctionRefusal::NotExpression {
                node_tag: "unknown",
            },
            |tag| unsupported_family(&closure.root.node_id, tag),
        ));
    }
    Ok((closure, occurrences))
}

fn lowering_profile() -> CompleteLoweringProfileV2 {
    CompleteLoweringProfileV2 {
        reference: quire_contract_model::LoweringProfileRef {
            authority: "agent-ix".into(),
            identity: "quire.codegen.exact-function/v1".into(),
        },
        supported_tags: BTreeSet::from([
            CheckedNodeTag::ScalarType,
            CheckedNodeTag::CompositeType,
            CheckedNodeTag::BoundedDomain,
            CheckedNodeTag::Value,
            CheckedNodeTag::Expression,
            CheckedNodeTag::Claim,
            CheckedNodeTag::Correspondence,
        ]),
        require_bounds: false,
        work_limit: EXACT_FUNCTION_LOWERING_WORK_LIMIT,
    }
}

fn schedule_lowering_failure(
    record: &CompleteLoweringRecordV2,
) -> Option<schedule::ScheduleRefusal> {
    classify_lowering_failure(record).map(|failure| match failure {
        LoweringFailure::WorkExhausted { limit, consumed } => {
            schedule::ScheduleRefusal::LoweringWorkExhausted { limit, consumed }
        }
        LoweringFailure::ByteLimitExceeded { limit, consumed } => {
            schedule::ScheduleRefusal::LoweringByteLimitExceeded { limit, consumed }
        }
        LoweringFailure::LimitUnrecognised {
            limit_kind,
            limit,
            consumed,
        } => schedule::ScheduleRefusal::LoweringLimitUnrecognised {
            limit_kind,
            limit,
            consumed,
        },
    })
}

/// Resolve a V2 type node id to this generator's supported operand kind, or
/// a typed refusal -- including `reference`/family blockers (AC-10, AC-11).
fn resolve_operand_type(
    graph: &Graph<'_>,
    type_node_id: &CheckedNodeId,
) -> Result<OperandKind, ExactFunctionRefusal> {
    let node = graph
        .node(type_node_id)
        .ok_or_else(|| ExactFunctionRefusal::UnknownTypeNode {
            type_node_id: type_node_id.clone(),
        })?;
    match CheckedNodeTag::from_wire(&node.node_tag) {
        Some(CheckedNodeTag::ScalarType) => match &*node.semantic_form {
            "boolean" => Ok(OperandKind::Boolean),
            "integer" => Ok(OperandKind::Integer),
            _ => Err(ExactFunctionRefusal::UnsupportedOperandType {
                type_node_id: type_node_id.clone(),
            }),
        },
        Some(CheckedNodeTag::CompositeType) if &*node.semantic_form == "reference" => {
            Err(ExactFunctionRefusal::BlockedOnUpstream {
                unsupported_node_id: type_node_id.clone(),
                node_tag: "composite_type.reference",
                issue: UpstreamBlocker::QuireSpecLanguage120,
            })
        }
        Some(tag @ (CheckedNodeTag::Model | CheckedNodeTag::Relation)) => {
            Err(ExactFunctionRefusal::BlockedOnUpstream {
                unsupported_node_id: type_node_id.clone(),
                node_tag: tag.as_wire(),
                issue: UpstreamBlocker::QuireSpecLanguage120,
            })
        }
        Some(
            tag @ (CheckedNodeTag::State | CheckedNodeTag::Temporal | CheckedNodeTag::Protocol),
        ) => Err(ExactFunctionRefusal::BlockedOnUpstream {
            unsupported_node_id: type_node_id.clone(),
            node_tag: tag.as_wire(),
            issue: UpstreamBlocker::QuireSpecLanguage121,
        }),
        _ => Err(ExactFunctionRefusal::UnsupportedOperandType {
            type_node_id: type_node_id.clone(),
        }),
    }
}

fn unsupported_family(node_id: &CheckedNodeId, tag: CheckedNodeTag) -> ExactFunctionRefusal {
    let blocked = |issue| ExactFunctionRefusal::BlockedOnUpstream {
        unsupported_node_id: node_id.clone(),
        node_tag: tag.as_wire(),
        issue,
    };
    match tag {
        CheckedNodeTag::Model | CheckedNodeTag::Relation => {
            blocked(UpstreamBlocker::QuireSpecLanguage120)
        }
        CheckedNodeTag::State | CheckedNodeTag::Temporal | CheckedNodeTag::Protocol => {
            blocked(UpstreamBlocker::QuireSpecLanguage121)
        }
        _ => ExactFunctionRefusal::Unsupported {
            unsupported_node_id: node_id.clone(),
            node_tag: tag.as_wire(),
        },
    }
}

fn application_arguments(body: &CheckedTerm) -> Option<&[CheckedTerm]> {
    match body {
        CheckedTerm::Application {
            operator: ApplicationOperator::Binary,
            arguments,
            ..
        } => Some(arguments),
        _ => None,
    }
}

/// Classify one declared function's own body shape against its lowered
/// node, independent of any other function (Stage 1, non-`Call` bodies), and
/// return the body with its operator resolved: the one place a scalar operator
/// is checked, so the rendered body cannot disagree with the refusal.
fn classify_body_shape<'r, Body>(
    node: &CheckedSemanticNodeV2<Body>,
    body: Option<&CheckedTerm>,
    declaration: &'r ExactFunctionDeclaration,
    parameter_kinds: &[OperandKind],
) -> Result<ClassifiedBody<'r>, ExactFunctionRefusal> {
    match &declaration.body {
        ExactFunctionBody::Scalar { operator } => {
            let operator = BinaryIntegerOperator::of(*operator).ok_or(
                ExactFunctionRefusal::UnsupportedOperator {
                    detail: "only binary IntegerOperator::{Add,Subtract,Multiply} are supported",
                },
            )?;
            if &*node.semantic_form != "binary" {
                return Err(ExactFunctionRefusal::FormMismatch {
                    found: node.semantic_form.to_string(),
                });
            }
            let arguments = body
                .and_then(application_arguments)
                .filter(|arguments| arguments.len() == 2)
                .ok_or(ExactFunctionRefusal::BodyMismatch)?;
            let operation_identity = body.and_then(application_identity);
            let expected_identity = match operator {
                BinaryIntegerOperator::Add => "quire.op.integer.add",
                BinaryIntegerOperator::Subtract => "quire.op.integer.sub",
                BinaryIntegerOperator::Multiply => "quire.op.integer.mul",
            };
            if operation_identity != Some(expected_identity) {
                return Err(ExactFunctionRefusal::BodyMismatch);
            }
            let _ = arguments;
            Ok(ClassifiedBody::Scalar(operator))
        }
        ExactFunctionBody::CompositeEquality { operator } => {
            if &*node.semantic_form != "binary" {
                return Err(ExactFunctionRefusal::FormMismatch {
                    found: node.semantic_form.to_string(),
                });
            }
            let arguments = body
                .and_then(application_arguments)
                .filter(|arguments| arguments.len() == 2)
                .ok_or(ExactFunctionRefusal::BodyMismatch)?;
            let expected_identity = match (parameter_kinds, operator) {
                ([OperandKind::Integer, OperandKind::Integer], EqualityOperatorKind::Equal) => {
                    "quire.op.integer.eq"
                }
                ([OperandKind::Integer, OperandKind::Integer], EqualityOperatorKind::NotEqual) => {
                    "quire.op.integer.ne"
                }
                ([OperandKind::Boolean, OperandKind::Boolean], EqualityOperatorKind::Equal) => {
                    "quire.op.boolean.eq"
                }
                ([OperandKind::Boolean, OperandKind::Boolean], EqualityOperatorKind::NotEqual) => {
                    "quire.op.boolean.ne"
                }
                _ => return Err(ExactFunctionRefusal::BodyMismatch),
            };
            if body.and_then(application_identity) != Some(expected_identity) {
                return Err(ExactFunctionRefusal::BodyMismatch);
            }
            let _ = arguments;
            Ok(ClassifiedBody::Equality(*operator))
        }
        ExactFunctionBody::Call { callee } => {
            if &*node.semantic_form != "call" {
                return Err(ExactFunctionRefusal::FormMismatch {
                    found: node.semantic_form.to_string(),
                });
            }
            Ok(ClassifiedBody::Call(callee))
        }
    }
}

/// Validate that the declared parameter count and the resolved
/// parameter/result operand kinds actually agree with `declaration`'s own
/// body kind -- the check `classify_body_shape` never made, since it only
/// ever read the *lowered node's* own argument count, not the declared
/// signature. Called once every parameter and result type is resolved
/// (Stage 1), so a `Scalar`/`CompositeEquality` function whose signature
/// disagrees with its body is refused before it can ever enter the
/// assembled package -- rather than assembling with a `FunctionDeclaration`
/// the rendered `Body` can never actually satisfy, which would only ever
/// return `Outcome::Refused(CheckedInvariant)` at call time: a silent
/// miscompile this module's own doc rules out.
fn validate_signature(
    declaration: &ExactFunctionDeclaration,
    parameter_kinds: &[OperandKind],
    result_kind: OperandKind,
) -> Result<(), ExactFunctionRefusal> {
    match &declaration.body {
        ExactFunctionBody::Scalar { .. } => {
            if declaration.parameters.len() != 2 {
                return Err(ExactFunctionRefusal::SignatureMismatch {
                    detail: "a Scalar body needs exactly two declared parameters",
                });
            }
            if !matches!(
                parameter_kinds,
                [OperandKind::Integer, OperandKind::Integer]
            ) {
                return Err(ExactFunctionRefusal::SignatureMismatch {
                    detail: "a Scalar body's two declared parameters must both be Integer",
                });
            }
            if result_kind != OperandKind::Integer {
                return Err(ExactFunctionRefusal::SignatureMismatch {
                    detail: "a Scalar body's declared result must be Integer",
                });
            }
            Ok(())
        }
        ExactFunctionBody::CompositeEquality { .. } => {
            if declaration.parameters.len() != 2 {
                return Err(ExactFunctionRefusal::SignatureMismatch {
                    detail: "a CompositeEquality body needs exactly two declared parameters",
                });
            }
            if parameter_kinds[0] != parameter_kinds[1] {
                return Err(ExactFunctionRefusal::SignatureMismatch {
                    detail: "a CompositeEquality body's two declared parameters must share one operand kind",
                });
            }
            if result_kind != OperandKind::Boolean {
                return Err(ExactFunctionRefusal::SignatureMismatch {
                    detail: "a CompositeEquality body's declared result must be Boolean",
                });
            }
            Ok(())
        }
        ExactFunctionBody::Call { .. } => Ok(()),
    }
}

/// Generate function-application oracles for `items`, over the declared
/// functions in `functions`, from an admitted package.
///
/// Fails as a whole only with `SourceTooLarge`, `ClaimMapSerialization` or
/// `LocationMapSerialization`; every per-item problem is a refusal in the
/// claim map.
pub fn generate_exact_function_oracles(
    package: &ContractPackageV1,
    requested: &[ContractRequestPosition],
    function_positions: &[usize],
    functions: &[ExactFunctionDeclaration],
    item_positions: &[usize],
    items: &[ExactFunctionItem],
) -> Result<ExactFunctionOracles, OracleGenerationError> {
    let accounting =
        PackageItems::new(package, requested).map_err(OracleGenerationError::PackageRequest)?;
    for (positions, count) in [
        (function_positions, functions.len()),
        (item_positions, items.len()),
    ] {
        if positions.len() != count {
            return Err(OracleGenerationError::PackageRequest(
                PackageRequestError::ItemPositions {
                    items: count,
                    positions: positions.len(),
                },
            ));
        }
    }
    let mut function_position_by_id = BTreeMap::new();
    for (function, &position) in functions.iter().zip(function_positions) {
        check_position(&accounting, position, &function.node_id)?;
        function_position_by_id
            .entry(&function.node_id)
            .or_insert(position);
    }
    for (item, &position) in items.iter().zip(item_positions) {
        check_position(&accounting, position, &item.call_node_id)?;
    }
    let source_index = FunctionSourceIndex::new(Graph::new(package));
    let graph = &source_index.graph;

    // Order declared functions by declaring node id (digest domain, then
    // digest), the same total order FR-014 already uses.
    let mut ordered_functions: Vec<&ExactFunctionDeclaration> = functions.iter().collect();
    ordered_functions.sort_by(|a, b| a.node_id.cmp(&b.node_id));

    // `resolved` and `function_index` below are keyed by each
    // declaration's own node id, which keeps same-name declarations on
    // distinct node ids apart but cannot separate declarations that share a
    // node id. Both kinds of duplicate are therefore refused outright,
    // before Stage 1 classification even runs (see module doc, "Package
    // assembly"), and `FunctionNames` holds only the names Stage 1 will
    // actually attempt to classify as resolvable.
    let names = FunctionNames::of(&ordered_functions);

    // Stage 1: per-function classification, with a bounded fixed-point pass
    // for `Call` bodies (a nested call's own validity depends on its
    // callee's classification, per AC-12).
    // `shapes` holds each declaration's own Stage 1 result by position, so
    // the resolution loop below reads a declaration's own result and no
    // lookup can miss.
    let mut shapes: Vec<Result<Stage1Shape<'_>, ExactFunctionRefusal>> =
        Vec::with_capacity(ordered_functions.len());
    for declaration in &ordered_functions {
        let position = function_position_by_id[&declaration.node_id];
        let admitted = accounting
            .at(position, &declaration.node_id)
            .map_err(OracleGenerationError::PackageRequest)?;
        let result = (|| -> Result<Stage1Shape<'_>, ExactFunctionRefusal> {
            if names.shares_node_id(&declaration.node_id) {
                return Err(ExactFunctionRefusal::DuplicateDeclaringNode {
                    node_id: declaration.node_id.clone(),
                });
            }
            if names.is_ambiguous(&declaration.name) {
                return Err(ExactFunctionRefusal::AmbiguousFunctionName {
                    name: declaration.name.clone(),
                });
            }
            // AC-6: capability negotiation happens first, before lowering
            // the body and before any item naming this function is
            // applied. This generator registers no backend for any named
            // capability, so any non-empty requirement list is
            // undischargeable by construction.
            if let Some(capability) = declaration.capability_requirements.first() {
                return Err(ExactFunctionRefusal::UnsupportedCapability {
                    capability: capability.clone(),
                });
            }
            let (closure, _) = admitted_binary_body(admitted, graph)?;
            let mut parameter_kinds = Vec::with_capacity(declaration.parameters.len());
            for parameter in &declaration.parameters {
                parameter_kinds.push(resolve_operand_type(graph, &parameter.type_node_id)?);
            }
            let result_kind = resolve_operand_type(graph, &declaration.result_type)?;
            validate_signature(declaration, &parameter_kinds, result_kind)?;
            let body = classify_body_shape(
                closure.root,
                graph.term(&closure.root.node_id),
                declaration,
                &parameter_kinds,
            )?;
            // A classified expression root is not authority to register a
            // function. Retain only bodies bound by one admitted source
            // function declaration; otherwise no concrete claim or location
            // entry may be emitted for it.
            if !matches!(declaration.body, ExactFunctionBody::Call { .. }) {
                let function_node_id = source_index
                    .source_function_node_id(&declaration.name, &declaration.node_id)
                    .ok_or(ExactFunctionRefusal::BodyMismatch)?;
                if !graph
                    .term(&closure.root.node_id)
                    .and_then(application_arguments)
                    .is_some_and(|arguments| {
                        source_argument_refs_match(graph, function_node_id, arguments)
                    })
                {
                    return Err(ExactFunctionRefusal::BodyMismatch);
                }
            }
            Ok(Stage1Shape {
                body,
                body_node_id: closure.root.node_id.clone(),
            })
        })();
        shapes.push(result);
    }

    let mut resolved: BTreeMap<&CheckedNodeId, Result<(), ExactFunctionRefusal>> = BTreeMap::new();
    for _ in 0..=ordered_functions.len() {
        let mut changed = false;
        for (declaration, own) in ordered_functions.iter().zip(&shapes) {
            if resolved.contains_key(&declaration.node_id) {
                continue;
            }
            let outcome = match (&declaration.body, own) {
                (_, Err(refusal)) => Some(Err(refusal.clone())),
                (ExactFunctionBody::Call { callee }, Ok(_)) => match names.unique_node_id(callee) {
                    None => Some(Err(ExactFunctionRefusal::UnknownCallee {
                        callee: callee.clone(),
                    })),
                    Some(callee_node_id) => match resolved.get(callee_node_id) {
                        Some(Ok(())) => Some(
                            source_nested_body_matches(
                                &source_index,
                                &declaration.name,
                                &declaration.node_id,
                                callee,
                                callee_node_id,
                            )
                            .then_some(())
                            .ok_or(ExactFunctionRefusal::BodyMismatch),
                        ),
                        Some(Err(_)) => Some(Err(ExactFunctionRefusal::UnknownCallee {
                            callee: callee.clone(),
                        })),
                        None => None,
                    },
                },
                (_, Ok(_)) => Some(Ok(())),
            };
            if let Some(outcome) = outcome {
                resolved.insert(&declaration.node_id, outcome);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    // Anything still unresolved is a nested-call dependency this bounded
    // pass could not settle (a cycle among `Call` bodies): refuse.
    for declaration in &ordered_functions {
        resolved.entry(&declaration.node_id).or_insert_with(|| {
            Err(match &declaration.body {
                ExactFunctionBody::Call { callee } => ExactFunctionRefusal::UnknownCallee {
                    callee: callee.clone(),
                },
                _ => ExactFunctionRefusal::BodyMismatch,
            })
        });
    }

    // Stage 2: assemble one package from every surviving function, in
    // order, and admit it once.
    let mut classified: Vec<ClassifiedFunction<'_>> = Vec::with_capacity(ordered_functions.len());
    for (declaration, shape) in ordered_functions.iter().zip(shapes) {
        // A refused function (its own Stage 1 refusal, or a callee's) is left out.
        let Ok(shape) = shape else {
            continue;
        };
        if !matches!(resolved.get(&declaration.node_id), Some(Ok(()))) {
            continue;
        }
        classified.push(ClassifiedFunction {
            declaration,
            body: shape.body,
            body_node_id: shape.body_node_id,
        });
    }
    let survivors: Vec<&ExactFunctionDeclaration> = classified
        .iter()
        .map(|function| function.declaration)
        .collect();

    // A location origin follows the admitted source's function order, which
    // is what the checked QSL call site records. Omitting or refusing another
    // function in the request cannot renumber this source location.
    let source_order = |node_id: &CheckedNodeId| {
        package
            .wire()
            .source_map
            .nodes
            .iter()
            .find(|entry| &entry.node_id == node_id)
            .and_then(|entry| {
                entry
                    .occurrences
                    .iter()
                    .flat_map(|occurrence| &occurrence.regions)
                    .map(|region| (&region.source, region.start))
                    .min()
            })
    };
    let mut bodies_by_source = source_index
        .graph
        .nodes()
        .filter(|node| &*node.node_tag == "function" && &*node.semantic_form == "pure_function")
        .filter_map(|node| {
            let body = source_index
                .graph
                .term(&node.node_id)
                .and_then(|term| binding_value(term, "body"))
                .and_then(|term| reference_in_graph(term, &source_index.graph))?;
            Some((
                source_order(&node.node_id).or_else(|| source_order(body)),
                &node.node_id,
                body,
            ))
        })
        .collect::<Vec<_>>();
    bodies_by_source.sort();
    let location_index: BTreeMap<&CheckedNodeId, usize> = bodies_by_source
        .into_iter()
        .enumerate()
        .map(|(index, (_, _, body))| (body, index))
        .collect();
    let survivor_index: BTreeMap<&CheckedNodeId, (usize, &ExactFunctionDeclaration)> = classified
        .iter()
        .enumerate()
        .map(|(index, function)| {
            let declaration = function.declaration;
            (&declaration.node_id, (index, declaration))
        })
        .collect();
    let classified_by_id: BTreeMap<&CheckedNodeId, &ClassifiedFunction<'_>> = classified
        .iter()
        .map(|function| (&function.declaration.node_id, function))
        .collect();

    // Static location records describe the admitted IR graph, independently of a later QSL run.
    let mut location_map = Vec::with_capacity(survivors.len());
    for declaration in &survivors {
        let index = location_index[&declaration.node_id];
        let call_point = match &declaration.body {
            ExactFunctionBody::Scalar { .. } => CallPointKind::ScalarOperator,
            ExactFunctionBody::CompositeEquality { .. } => CallPointKind::EqualityEvaluation,
            ExactFunctionBody::Call { .. } => CallPointKind::NestedCall,
        };
        location_map.push(LocationMapEntry {
            function: declaration.name.clone(),
            call_point,
            location: RecordedLocation {
                origin: RecordedOrigin::Body {
                    function: declaration.name.clone(),
                    index,
                },
                path: Vec::new(),
            },
        });
    }

    // Stage 3: per requested item.
    let mut counts: BTreeMap<ItemKey, u32> = BTreeMap::new();
    let mut by_key: BTreeMap<ItemKey, (usize, &ExactFunctionItem)> = BTreeMap::new();
    // Built from `ordered_functions` (sorted by node id), not the
    // caller-supplied `functions` order, so a name shared by more than one
    // declaration always resolves to the same (highest-sorted) node id
    // regardless of what order the caller passed its declarations in --
    // keeping AC-13's determinism-across-permutations guarantee even for an
    // item naming an ambiguous function. (A refused duplicate-node name keys
    // by its own name as well, as does an unknown name, see `ItemKey::name`.)
    let function_node_id_by_name: BTreeMap<&str, &CheckedNodeId> = ordered_functions
        .iter()
        .map(|declaration| (declaration.name.as_str(), &declaration.node_id))
        .collect();
    for (item, &position) in items.iter().zip(item_positions) {
        let key = ItemKey::of(item, &function_node_id_by_name, &names);
        *counts.entry(key.clone()).or_insert(0) += 1;
        by_key.entry(key).or_insert((position, item));
    }

    let mut source = SourceBuilder::default();
    let mut claims = Vec::with_capacity(by_key.len());
    // Generated claims as (claim position, readable stem, key, declaration), rendered once
    // every generated function is named.
    let mut pending = Vec::new();
    for (key, (position, item)) in by_key {
        let duplicate = counts.get(&key).copied().unwrap_or(0) > 1;
        // An item naming a refused duplicate-node function carries that
        // refusal even when it was also requested more than once (AC-22).
        let result = if let Some(refusal) = names.duplicate_node_refusal(&item.function) {
            ClaimDisposition::Refused { refusal }
        } else if duplicate {
            ClaimDisposition::Refused {
                refusal: ExactFunctionRefusal::DuplicateRequest,
            }
        } else if let Some(node_tag) = graph
            .node(&item.call_node_id)
            .and_then(|node| direct_unsupported_form(node, graph.term(&item.call_node_id)))
        {
            ClaimDisposition::Refused {
                refusal: ExactFunctionRefusal::Unsupported {
                    unsupported_node_id: item.call_node_id.clone(),
                    node_tag,
                },
            }
        } else {
            let admitted = accounting
                .at(position, &item.call_node_id)
                .map_err(OracleGenerationError::PackageRequest)?;
            match item_disposition(
                &names,
                &resolved,
                &survivor_index,
                (package, &source_index),
                item,
                admitted,
            ) {
                Ok((stem, claim, declaration)) => {
                    pending.push((claims.len(), stem, key.clone(), declaration));
                    ClaimDisposition::Generated(Box::new(claim))
                }
                Err(refusal) => ClaimDisposition::Refused { refusal },
            }
        };
        claims.push(ExactFunctionClaim {
            node_id: item.call_node_id.clone(),
            result,
        });
    }
    // Each function is named by the function it applies; items applying one function are
    // numbered in key order, so a refused or differently requested sibling never renames one.
    let symbols = unique_names(
        pending
            .iter()
            .map(|(_, stem, key, _)| (stem.clone(), key.clone()))
            .collect(),
    );
    for ((claim, _, _, declaration), symbol) in pending.into_iter().zip(symbols) {
        let rendered = classified_by_id
            .get(&declaration.node_id)
            .copied()
            .ok_or(ExactFunctionRefusal::BodyMismatch)
            .and_then(|function| {
                owner_render::classified_item(&source_index, &names, &symbol, function)
            });
        match rendered {
            Ok(item_source) => {
                source.item(&item_source);
                if let ClaimDisposition::Generated(generated) = &mut claims[claim].result {
                    generated.oracle_symbol = format!("oracle_{symbol}");
                }
            }
            Err(refusal) => claims[claim].result = ClaimDisposition::Refused { refusal },
        }
    }

    let claim_map = ClaimMap {
        package_id: package.wire().package.source.package_id.clone(),
        blocked: Vec::new(),
        items: claims,
    };
    let subjects = classified
        .iter()
        .map(|function| crate::core::provenance::node_subject(&function.declaration.node_id))
        .chain(
            claim_map
                .items
                .iter()
                .filter(|claim| matches!(&claim.result, ClaimDisposition::Generated(_)))
                .map(|claim| crate::core::provenance::node_subject(&claim.node_id)),
        )
        .collect::<Vec<_>>();
    let lib = crate::core::provenance::with_rust_header(
        source.finish(),
        subjects.iter().map(String::as_str),
    );
    if lib.len() > MAX_GENERATED_SOURCE_BYTES {
        return Err(OracleGenerationError::SourceTooLarge {
            bytes: lib.len(),
            limit: MAX_GENERATED_SOURCE_BYTES,
            setting: "cg.function.source_bytes",
        });
    }
    let mut map_bytes = serde_json::to_vec_pretty(&claim_map)
        .map_err(|_| OracleGenerationError::ClaimMapSerialization)?;
    map_bytes.push(b'\n');
    let map_text =
        String::from_utf8(map_bytes).map_err(|_| OracleGenerationError::ClaimMapSerialization)?;

    let mut location_bytes = serde_json::to_vec_pretty(&location_map)
        .map_err(|_| OracleGenerationError::LocationMapSerialization)?;
    location_bytes.push(b'\n');
    let location_text = String::from_utf8(location_bytes)
        .map_err(|_| OracleGenerationError::LocationMapSerialization)?;

    Ok(ExactFunctionOracles {
        artifacts: vec![
            Artifact::new(
                "Cargo.toml",
                function_oracle_crate_manifest(EXACT_FUNCTION_CRATE_NAME),
            ),
            Artifact::new("src/lib.rs", lib),
            Artifact::new("claim-map.json", map_text),
            Artifact::new("location-map.json", location_text),
        ],
        claim_map,
        location_map,
    })
}

/// How the request's declarations share node ids and names, computed once
/// from every declaration (FR-021-AC-22 and `AmbiguousFunctionName`).
struct FunctionNames<'a> {
    /// Declarations per declaring node id.
    node_id_counts: BTreeMap<&'a CheckedNodeId, usize>,
    /// Declarations per name, counting every declaration whatever its node id.
    name_counts: BTreeMap<&'a str, usize>,
    /// For each name held by a declaration whose node id is shared, that node
    /// id: the smallest when several duplicate groups hold the name.
    duplicate_node_by_name: BTreeMap<&'a str, &'a CheckedNodeId>,
    /// The declaring node id of each name held by exactly one declaration
    /// whose node id is not shared: the only names a call can resolve.
    unique_node_id: BTreeMap<&'a str, &'a CheckedNodeId>,
}

impl<'a> FunctionNames<'a> {
    /// Build the tables from declarations sorted by declaring node id, so the
    /// smallest duplicate node id wins `duplicate_node_by_name` whatever the
    /// request order.
    fn of(ordered: &[&'a ExactFunctionDeclaration]) -> Self {
        let mut node_id_counts: BTreeMap<&CheckedNodeId, usize> = BTreeMap::new();
        let mut name_counts: BTreeMap<&str, usize> = BTreeMap::new();
        for declaration in ordered {
            *node_id_counts.entry(&declaration.node_id).or_insert(0) += 1;
            *name_counts.entry(declaration.name.as_str()).or_insert(0) += 1;
        }
        let mut duplicate_node_by_name = BTreeMap::new();
        let mut unique_node_id = BTreeMap::new();
        for declaration in ordered {
            let name = declaration.name.as_str();
            if node_id_counts
                .get(&declaration.node_id)
                .copied()
                .unwrap_or(0)
                > 1
            {
                duplicate_node_by_name
                    .entry(name)
                    .or_insert(&declaration.node_id);
            } else if name_counts.get(name).copied().unwrap_or(0) == 1 {
                unique_node_id.insert(name, &declaration.node_id);
            }
        }
        Self {
            node_id_counts,
            name_counts,
            duplicate_node_by_name,
            unique_node_id,
        }
    }

    /// Whether more than one declaration holds `node_id`.
    fn shares_node_id(&self, node_id: &CheckedNodeId) -> bool {
        self.node_id_counts.get(node_id).copied().unwrap_or(0) > 1
    }

    /// Whether more than one declaration holds `name`.
    fn is_ambiguous(&self, name: &str) -> bool {
        self.name_counts.get(name).copied().unwrap_or(0) > 1
    }

    /// The declaring node id of the one resolvable declaration named `name`.
    fn unique_node_id(&self, name: &str) -> Option<&'a CheckedNodeId> {
        self.unique_node_id.get(name).copied()
    }

    /// `DuplicateDeclaringNode` when a declaration sharing its node id holds
    /// `name`: the refusal that takes precedence over every other item
    /// disposition, `DuplicateRequest` included.
    fn duplicate_node_refusal(&self, name: &str) -> Option<ExactFunctionRefusal> {
        self.duplicate_node_by_name.get(name).map(|node_id| {
            ExactFunctionRefusal::DuplicateDeclaringNode {
                node_id: (*node_id).clone(),
            }
        })
    }

    /// The declaring node id an item naming `name` applies, or the refusal
    /// for it: the node-id check first, then absence, then name ambiguity.
    fn resolve(&self, name: &str) -> Result<&'a CheckedNodeId, ExactFunctionRefusal> {
        if let Some(refusal) = self.duplicate_node_refusal(name) {
            return Err(refusal);
        }
        if self.is_ambiguous(name) {
            return Err(ExactFunctionRefusal::AmbiguousFunctionName {
                name: name.to_owned(),
            });
        }
        self.unique_node_id(name)
            .ok_or_else(|| ExactFunctionRefusal::UnknownFunction {
                name: name.to_owned(),
            })
    }
}

fn item_disposition<'a>(
    names: &FunctionNames<'_>,
    resolved: &BTreeMap<&CheckedNodeId, Result<(), ExactFunctionRefusal>>,
    survivor_index: &BTreeMap<&CheckedNodeId, (usize, &'a ExactFunctionDeclaration)>,
    source: (&ContractPackageV1, &FunctionSourceIndex<'_>),
    item: &ExactFunctionItem,
    admitted: PackageItem<'_>,
) -> Result<
    (
        String,
        GeneratedExactFunctionClaim,
        &'a ExactFunctionDeclaration,
    ),
    ExactFunctionRefusal,
> {
    let (package, source_index) = source;
    let graph = &source_index.graph;
    let (closure, occurrences) = admitted_binary_body(admitted, graph)?;
    let node = closure.root;
    if &*node.semantic_form != "call" {
        return Err(ExactFunctionRefusal::FormMismatch {
            found: node.semantic_form.to_string(),
        });
    }
    // An item naming a function that WAS declared but failed Stage 1
    // classification carries that function's own typed refusal reason
    // (AC-10, AC-11, AC-12), never a generic "unknown function" -- that
    // disposition is reserved for a name absent from the request's own
    // declarations entirely. A name held by a declaration whose node id is
    // shared is refused as `DuplicateDeclaringNode`, and a name declared more
    // than once on distinct node ids as `AmbiguousFunctionName`: both ARE
    // declared, just unresolvably, so never `UnknownFunction`.
    let function_node_id = names.resolve(&item.function)?;
    if let Some(Err(refusal)) = resolved.get(function_node_id) {
        return Err(refusal.clone());
    }
    let Some(&(index, declaration)) = survivor_index.get(function_node_id) else {
        return Err(ExactFunctionRefusal::UnknownFunction {
            name: item.function.clone(),
        });
    };
    if item.argument_node_ids.len() != declaration.parameters.len() {
        return Err(ExactFunctionRefusal::ArityMismatch {
            expected: declaration.parameters.len(),
            found: item.argument_node_ids.len(),
        });
    }
    if graph.operator(&node.node_id) != Some(ApplicationOperator::Call) {
        return Err(ExactFunctionRefusal::BodyMismatch);
    }
    let source_function_id = source_index
        .source_function_node_id(&declaration.name, &declaration.node_id)
        .ok_or(ExactFunctionRefusal::BodyMismatch)?;
    if !source_call_matches(
        graph
            .term(&node.node_id)
            .ok_or(ExactFunctionRefusal::BodyMismatch)?,
        graph,
        source_function_id,
        &item.argument_node_ids,
    ) {
        return Err(ExactFunctionRefusal::BodyMismatch);
    }
    for argument_id in &item.argument_node_ids {
        let argument = graph
            .node(argument_id)
            .ok_or(ExactFunctionRefusal::BodyMismatch)?;
        let type_node = graph.node(&argument.semantic_type).ok_or_else(|| {
            ExactFunctionRefusal::UnknownTypeNode {
                type_node_id: argument.semantic_type.clone(),
            }
        })?;
        let type_node_id = if type_node.node_tag.as_ref() == "bounded_domain" {
            &type_node.semantic_type
        } else {
            &argument.semantic_type
        };
        resolve_operand_type(graph, type_node_id)?;
    }
    // The readable stem: the applied function's name. The caller settles the final name.
    let stem = format!("call_{}", bounded_readable_component(&declaration.name));
    Ok((
        stem,
        GeneratedExactFunctionClaim {
            oracle_symbol: String::new(),
            contract_package_identity: package.identity().clone(),
            node_id: item.call_node_id.clone(),
            function_node_id: source_function_id.clone(),
            package_id: package.wire().package.source.package_id.clone(),
            semantic_type: node.semantic_type.clone(),
            source_map: occurrences.to_vec(),
            claims: closure.claims,
            bounds: closure.bounds,
            dependencies: closure
                .nodes
                .iter()
                .filter(|entry| entry.node.node_id != item.call_node_id)
                .map(|entry| entry.node.node_id.clone())
                .collect(),
            function: declaration.name.clone(),
            function_origin: RecordedOrigin::Body {
                function: declaration.name.clone(),
                index,
            },
            declaration_keys: Vec::new(),
        },
        declaration,
    ))
}

// ---------------------------------------------------------------------------
// Item ordering key (AC-13)
// ---------------------------------------------------------------------------

/// The total order and duplicate-detection key: the `call` expression
/// node's id, then the applied function's declaring node id (absent ranks
/// before present, for an item naming an unknown function), then each
/// argument operand's source node id, every node id compared by digest
/// domain then digest, and last the function name for the items whose
/// preceding fields cannot tell them apart (FR-021-AC-24): `String` order is
/// byte-wise over UTF-8 and case-sensitive.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct ItemKey {
    call_node_id: CheckedNodeId,
    function_node_id: Option<CheckedNodeId>,
    argument_node_ids: Vec<CheckedNodeId>,
    /// The item's function name, set only when `function_node_id` cannot
    /// identify the function: the name is absent from the declarations, so two
    /// unknown names on one call node would otherwise share a key and the
    /// second would be lost as a `DuplicateRequest`; or a declaration sharing
    /// its declaring node id holds the name (FR-021-AC-22), so two items
    /// naming different members of such a pair would otherwise collapse into
    /// one claim. The same name requested twice still shares one key.
    name: Option<String>,
}

impl ItemKey {
    fn of(
        item: &ExactFunctionItem,
        function_node_id_by_name: &BTreeMap<&str, &CheckedNodeId>,
        names: &FunctionNames<'_>,
    ) -> Self {
        let function_node_id = function_node_id_by_name
            .get(item.function.as_str())
            .map(|id| (*id).clone());
        let needs_name =
            function_node_id.is_none() || names.duplicate_node_refusal(&item.function).is_some();
        Self {
            call_node_id: item.call_node_id.clone(),
            function_node_id,
            argument_node_ids: item.argument_node_ids.clone(),
            name: needs_name.then(|| item.function.clone()),
        }
    }
}

// ---------------------------------------------------------------------------
// Rendering
// ---------------------------------------------------------------------------

#[derive(Default)]
struct SourceBuilder {
    functions: String,
}

impl SourceBuilder {
    fn item(&mut self, rendered: &str) {
        self.functions.push_str(rendered);
    }

    fn finish(self) -> String {
        format!("{}{}", owner_render::HEADER, self.functions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type Graph<'a> = crate::oracle::checked_graph::CheckedGraph<'a, CheckedTerm>;
    use crate::oracle::failed_records::{failed_record, unrecognised_kinds};
    use ix_trace_rs::trace;
    use quire_contract_model::CheckedPackageLimit;

    #[trace("TC-030")]
    #[test]
    fn function_preflight_reuses_one_graph_index_across_items() {
        let node = |ordinal: usize| {
            let id = CheckedNodeId {
                domain: "quire.checked-semantic-node/v1".into(),
                digest: format!("{ordinal:064x}").into(),
            };
            CheckedSemanticNodeV2 {
                node_id: id.clone(),
                schema_version: "quire.checked-semantic-graph/v2".into(),
                node_tag: "expression".into(),
                semantic_form: "binary".into(),
                semantic_type: id,
                dependencies: Vec::new(),
                occurrences: Vec::new(),
                recursion_group: None,
                nominal_identity_preimage: None,
                declaration: None,
                owner: None,
                body: CheckedTerm::aggregate(Vec::new()),
            }
        };
        let mut nodes = (0..32).map(node).collect::<Vec<_>>();
        nodes[0].node_tag = "function".into();
        nodes[0].semantic_form = "pure_function".into();
        let _ = take_function_preflight_graph_visits();
        let _ = take_function_preflight_graph_lookup_work();
        let graph = FunctionPreflightGraph::from_nodes(
            &nodes,
            CheckedSemanticId {
                domain: "quire.package.semantic/v2".into(),
                algorithm: "sha256".into(),
                digest: "0".repeat(64).into(),
            },
        );
        for ordinal in [0, 15, 31, 0, 31] {
            assert_eq!(
                graph
                    .get(&nodes[ordinal].node_id)
                    .map(|found| &found.node_id),
                Some(&nodes[ordinal].node_id)
            );
        }
        let reference = |node: &CheckedSemanticNodeV2| CheckedTerm::Reference {
            target: node.node_id.clone(),
        };
        let call = CheckedTerm::Application {
            operator: ApplicationOperator::Call,
            operation: serde_json::json!({"identity": "quire.op.function.call"}),
            result_type: nodes[0].semantic_type.clone(),
            arguments: vec![
                reference(&nodes[0]),
                reference(&nodes[15]),
                reference(&nodes[31]),
            ],
        };
        assert!(source_call_matches(
            &call,
            &graph,
            &nodes[0].node_id,
            &[nodes[15].node_id.clone(), nodes[31].node_id.clone()]
        ));
        assert!(!source_call_matches(
            &call,
            &graph,
            &nodes[0].node_id,
            &[nodes[31].node_id.clone(), nodes[15].node_id.clone()]
        ));
        assert_eq!(take_function_preflight_graph_visits(), nodes.len());
        let lookup_work = take_function_preflight_graph_lookup_work();
        assert_eq!(lookup_work.len(), 12);
        // Twelve indexed lookups stay below a full 32-node walk each.
        assert!(
            lookup_work
                .iter()
                .all(|&comparisons| (1..=10).contains(&comparisons)),
            "{lookup_work:?}"
        );
    }

    #[trace("FR-021-AC-27", "TC-031")]
    #[test]
    fn ac27_inline_scalar_literal_is_retained_with_its_admitted_type() {
        let owner = CheckedNodeId {
            domain: "quire.checked-semantic-node/v1".into(),
            digest: "1".repeat(64).into(),
        };
        let type_node = CheckedNodeId {
            domain: owner.domain.clone(),
            digest: "2".repeat(64).into(),
        };
        let literal = CheckedTerm::Literal {
            type_id: type_node.clone(),
            value_kind: LiteralKind::Integer,
            value: serde_json::json!("7"),
        };
        let types = BTreeMap::from([(type_node.clone(), schedule::ValueKind::Integer)]);
        assert_eq!(
            decode_schedule_operand(&literal, &owner, &types),
            Ok(schedule::GraphOperand::Literal {
                value_kind: schedule::ValueKind::Integer,
                value: "7".to_owned(),
            })
        );
        let malformed = CheckedTerm::Literal {
            type_id: type_node,
            value_kind: LiteralKind::Integer,
            value: serde_json::json!("seven"),
        };
        assert!(matches!(
            decode_schedule_operand(&malformed, &owner, &types),
            Err(schedule::ScheduleRefusal::MalformedTerm(_))
        ));
    }

    fn typed_fixture_node(
        ordinal: usize,
        tag: &str,
        form: &str,
        body: Option<quire_contract_model::CheckedTerm>,
    ) -> crate::oracle::checked_graph::fixture::Node {
        let id = CheckedNodeId {
            domain: "quire.checked-semantic-node/v1".into(),
            digest: format!("{ordinal:064x}").into(),
        };
        crate::oracle::checked_graph::fixture::Node::new(
            CheckedSemanticNodeV2 {
                node_id: id.clone(),
                schema_version: "quire.checked-semantic-graph/v2".into(),
                node_tag: tag.into(),
                semantic_form: form.into(),
                semantic_type: id,
                dependencies: Vec::new(),
                occurrences: Vec::new(),
                recursion_group: None,
                nominal_identity_preimage: None,
                declaration: None,
                owner: None,
                body: CheckedTerm::aggregate(Vec::new()),
            },
            body,
        )
    }

    /// Private typed terms guard the schedule projection policy. These raw
    /// spellings do not claim admission through the package reader.
    #[trace("TC-031")]
    #[test]
    fn schedule_literals_keep_raw_spelling_and_refusal_precedence() {
        use quire_contract_model::{CheckedTerm, LiteralKind};
        let owner = typed_fixture_node(1, "expression", "binary", None)
            .node_id
            .clone();
        let ty = typed_fixture_node(2, "scalar_type", "integer", None)
            .node_id
            .clone();
        let types = BTreeMap::from([(ty.clone(), schedule::ValueKind::Integer)]);
        for spelling in [
            "0",
            "01",
            "-0",
            "-01",
            "999999999999999999999999999999999999999999999999",
        ] {
            let literal = CheckedTerm::Literal {
                type_id: ty.clone(),
                value_kind: LiteralKind::Integer,
                value: serde_json::json!(spelling),
            };
            assert_eq!(
                decode_schedule_operand(&literal, &owner, &types),
                Ok(schedule::GraphOperand::Literal {
                    value_kind: schedule::ValueKind::Integer,
                    value: spelling.into()
                })
            );
        }
        for value in [
            serde_json::json!("+1"),
            serde_json::json!(""),
            serde_json::json!("-"),
            serde_json::json!("--1"),
            serde_json::json!("1x"),
            serde_json::json!(1),
        ] {
            let literal = CheckedTerm::Literal {
                type_id: ty.clone(),
                value_kind: LiteralKind::Integer,
                value,
            };
            assert_eq!(
                decode_schedule_operand(&literal, &owner, &types),
                Err(schedule::ScheduleRefusal::MalformedTerm(owner.clone()))
            );
            assert_eq!(
                decode_schedule_operand(&literal, &owner, &BTreeMap::new()),
                Err(schedule::ScheduleRefusal::WrongType(owner.clone()))
            );
        }
        let unsupported = CheckedTerm::Literal {
            type_id: ty,
            value_kind: LiteralKind::Text,
            value: serde_json::Value::Null,
        };
        assert_eq!(
            decode_schedule_operand(&unsupported, &owner, &BTreeMap::new()),
            Err(schedule::ScheduleRefusal::UnsupportedLiteral(owner.clone()))
        );
        assert_eq!(
            decode_reference(&unsupported, &owner),
            Err(schedule::ScheduleRefusal::UnsupportedLiteral(owner))
        );
    }

    #[trace("TC-031")]
    #[test]
    fn first_body_binding_full_keys_and_name_body_ambiguity_stay_distinct() {
        use quire_contract_model::{CheckedDeclaration, CheckedTerm, LiteralKind};
        let body = typed_fixture_node(10, "expression", "binary", None);
        let other = typed_fixture_node(11, "expression", "binary", None);
        let reference = |target: &CheckedNodeId| CheckedTerm::Reference {
            target: target.clone(),
        };
        let binding = |value: CheckedTerm| CheckedTerm::Binding {
            name: "body".into(),
            value: Box::new(value),
        };
        let function = |ordinal, target: &CheckedNodeId| {
            let mut node = typed_fixture_node(
                ordinal,
                "function",
                "pure_function",
                Some(CheckedTerm::Aggregate {
                    members: vec![binding(reference(target))],
                }),
            );
            node.declaration = Some(CheckedDeclaration {
                qualified_name: vec!["same".into()],
            });
            node
        };
        let first = function(20, &body.node_id);
        let same_name_other_body = function(21, &other.node_id);
        let nodes = [&body, &other, &first, &same_name_other_body];
        let graph: Graph<'_> = nodes
            .into_iter()
            .map(|node| (&node.node_id, node))
            .collect();
        let index = FunctionSourceIndex::new(graph);
        let graph = &index.graph;
        assert_eq!(
            index.source_function_node_id("same", &body.node_id),
            Some(&first.node_id)
        );
        let declaration = ExactFunctionDeclaration {
            node_id: body.node_id.clone(),
            name: "same".to_owned(),
            parameters: Vec::new(),
            result_type: body.node_id.clone(),
            body: ExactFunctionBody::Scalar {
                operator: IntegerOperator::Add,
            },
            capability_requirements: Vec::new(),
        };
        for shape in [
            owner_render::DirectBody::Scalar(BinaryIntegerOperator::Add),
            owner_render::DirectBody::Equality(EqualityOperatorKind::Equal),
        ] {
            let emitted = owner_render::checked_item(&index, "same", &declaration, shape)
                .expect("unique callable declaration");
            assert!(emitted.contains(first.node_id.digest.as_ref()));
            assert!(!emitted.contains(body.node_id.digest.as_ref()));
        }
        assert_eq!(index.source_function_node_id("Same", &body.node_id), None);
        let duplicate = function(22, &body.node_id);
        let nodes = [&body, &other, &first, &same_name_other_body, &duplicate];
        let ambiguous: Graph<'_> = nodes
            .into_iter()
            .map(|node| (&node.node_id, node))
            .collect();
        let ambiguous_index = FunctionSourceIndex::new(ambiguous);
        assert_eq!(
            ambiguous_index.source_function_node_id("same", &body.node_id),
            None
        );
        let malformed_first = CheckedTerm::Aggregate {
            members: vec![
                binding(CheckedTerm::Literal {
                    type_id: body.node_id.clone(),
                    value_kind: LiteralKind::Integer,
                    value: serde_json::json!("1"),
                }),
                binding(reference(&body.node_id)),
            ],
        };
        assert_eq!(
            binding_value(&malformed_first, "body")
                .and_then(|term| reference_in_graph(term, graph)),
            None
        );
        let mut malformed_function = function(23, &body.node_id);
        malformed_function.body = Some(malformed_first.clone());
        let damaged: Graph<'_> = [&body, &malformed_function]
            .into_iter()
            .map(|node| (&node.node_id, node))
            .collect();
        let damaged_index = FunctionSourceIndex::new(damaged);
        assert_eq!(
            damaged_index.source_function_node_id("same", &body.node_id),
            None
        );

        let mut foreign = body.node_id.clone();
        foreign.domain = "different-domain".into();
        let first_dangling = CheckedTerm::Aggregate {
            members: vec![
                binding(reference(&foreign)),
                binding(reference(&body.node_id)),
            ],
        };
        assert_eq!(
            binding_value(&first_dangling, "body").and_then(|term| reference_in_graph(term, graph)),
            None
        );
        assert_eq!(reference_in_graph(&reference(&foreign), graph), None);
        assert_eq!(reference_node_id(&reference(&foreign), graph.nodes()), None);
        assert_eq!(
            reference_node_id(&reference(&body.node_id), graph.nodes()),
            Some(&body.node_id)
        );
    }

    #[trace("TC-031")]
    #[test]
    fn referenced_function_requires_result_but_nonapplication_expression_uses_its_type() {
        let integer = typed_fixture_node(1, "scalar_type", "integer", None);
        let boolean = typed_fixture_node(2, "scalar_type", "boolean", None);
        let mut function = typed_fixture_node(
            3,
            "function",
            "pure_function",
            Some(CheckedTerm::Aggregate { members: vec![] }),
        );
        function.semantic_type = integer.node_id.clone();
        let mut expression = typed_fixture_node(
            4,
            "expression",
            "binary",
            Some(CheckedTerm::Aggregate { members: vec![] }),
        );
        expression.semantic_type = integer.node_id.clone();
        let mut application = typed_fixture_node(
            5,
            "expression",
            "binary",
            Some(CheckedTerm::Application {
                operator: ApplicationOperator::Binary,
                operation: serde_json::json!({"identity": "quire.op.integer.eq"}),
                arguments: vec![],
                result_type: boolean.node_id.clone(),
            }),
        );
        application.semantic_type = integer.node_id.clone();
        let graph: Graph<'_> = [&integer, &boolean, &function, &expression, &application]
            .into_iter()
            .map(|node| (&node.node_id, node))
            .collect();
        let types = BTreeMap::from([
            (integer.node_id.clone(), schedule::ValueKind::Integer),
            (boolean.node_id.clone(), schedule::ValueKind::Boolean),
        ]);
        let reference = |target: &CheckedNodeId| CheckedTerm::Reference {
            target: target.clone(),
        };
        assert_eq!(
            schedule_term_value_kind(
                &reference(&function.node_id),
                &graph,
                &types,
                &function.node_id
            ),
            Err(schedule::ScheduleRefusal::MalformedTerm(
                function.node_id.clone()
            ))
        );
        assert_eq!(
            schedule_term_value_kind(
                &reference(&expression.node_id),
                &graph,
                &types,
                &function.node_id
            ),
            Ok(schedule::ValueKind::Integer)
        );
        assert_eq!(
            schedule_term_value_kind(
                &reference(&application.node_id),
                &graph,
                &types,
                &function.node_id
            ),
            Ok(schedule::ValueKind::Boolean)
        );
    }

    #[test]
    fn generated_function_claim_serializes_package_identity_and_requested_key() {
        let identity: CheckedSemanticId = serde_json::from_value(serde_json::json!({
            "domain": "quire.contract-ir.semantic/v1",
            "algorithm": "sha256",
            "digest": "a".repeat(64),
        }))
        .expect("package identity");
        let node_id: CheckedNodeId = serde_json::from_value(serde_json::json!({
            "domain": "quire.checked-semantic-node/v1",
            "digest": "b".repeat(64),
        }))
        .expect("requested key");
        let generated = GeneratedExactFunctionClaim {
            oracle_symbol: "oracle_f".to_owned(),
            contract_package_identity: identity.clone(),
            node_id: node_id.clone(),
            function_node_id: node_id.clone(),
            package_id: identity.clone(),
            semantic_type: node_id.clone(),
            source_map: Vec::new(),
            claims: Vec::new(),
            bounds: Vec::new(),
            dependencies: Vec::new(),
            function: "f".to_owned(),
            function_origin: RecordedOrigin::Body {
                function: "f".to_owned(),
                index: 0,
            },
            declaration_keys: Vec::new(),
        };
        let value = serde_json::to_value(generated).expect("claim serializes");
        assert_eq!(
            value["contract_package_identity"],
            serde_json::json!(identity)
        );
        assert_eq!(value["node_id"], serde_json::json!(node_id));
        assert_eq!(value["function_node_id"], serde_json::json!(node_id));
        assert!(value.get("ir_id").is_none());
        for key in ["claims", "bounds", "dependencies"] {
            assert_eq!(value[key], serde_json::json!([]), "{key}");
        }
    }

    /// The public flat scheduler preserves Contract IR's per-record limit details for its
    /// separate call-item batch as well as the function-body batch.
    #[trace("FR-021-AC-23", "TC-031")]
    #[test]
    fn schedule_call_item_lowering_failure_keeps_its_own_limit_details() {
        assert_eq!(
            schedule_lowering_failure(&failed_record(CheckedPackageLimit::Work, 23, 24)),
            Some(schedule::ScheduleRefusal::LoweringWorkExhausted {
                limit: 23,
                consumed: 24,
            })
        );
        assert_eq!(
            schedule_lowering_failure(&failed_record(CheckedPackageLimit::Bytes, 31, 32)),
            Some(schedule::ScheduleRefusal::LoweringByteLimitExceeded {
                limit: 31,
                consumed: 32,
            })
        );
        for (kind, name) in unrecognised_kinds() {
            assert_eq!(
                schedule_lowering_failure(&failed_record(kind, 41, 42)),
                Some(schedule::ScheduleRefusal::LoweringLimitUnrecognised {
                    limit_kind: name,
                    limit: 41,
                    consumed: 42,
                }),
                "{name}"
            );
        }
    }
}
