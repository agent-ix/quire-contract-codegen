//! The scalar family: lowering an IR-confirmed V2 exact-scalar claim and rendering its Kani
//! harness (FR-015).

use crate::oracle::operator_catalog::{
    IntegerOperator, Operator as CatalogOperator, ScalarKaniOperator,
};
use quire_contract_model::{CheckedNodeId, CheckedSemanticNodeV2, CheckedTerm};

#[cfg(test)]
use crate::{
    core::artifact::{Artifact, MAX_GENERATED_SOURCE_BYTES},
    kani::abi::adapter_options,
};

use crate::{
    core::ir::checked_term::{aggregate_members, bound_members, literal_count, literal_integer},
    core::ir::scalar_bounds::{
        COLLECTION_BOUNDS_MEMBERS, INTEGER_RANGE_MEMBERS, TEXT_BOUNDS_MEMBERS,
    },
    kani::abi::{i64_literal, readable_component, KaniSolver},
    kani::generate::outcome::{DerivedDomain, KaniObligationRequest, UnsupportedObligation},
    kani::generate::record::{harness_path, record},
    kani::identity::{
        KaniScalarObligationHarness, ScalarObligationArgument, ScalarObligationIdentity,
    },
    oracle::scalar::{lower::OperandRange, ExactScalarClaim, GeneratedScalarClaim},
};

/// An IR-confirmed V2 exact-scalar claim this generator can render: bounded integer arithmetic
/// or Boolean implication. Other confirmed scalar families have no renderer yet.
pub(super) struct LoweredScalarClaim {
    pub(super) node_id: CheckedNodeId,
    pub(super) operation_identity: String,
    /// The oracle's own self-contained source (`GeneratedScalarClaim::oracle_source`), embedded
    /// verbatim ahead of the harness module, exactly as a V1 harness embeds its `ClauseOracle`s.
    oracle_source: String,
    native_integer_selection: Option<crate::core::native_integer::NativeIntegerSelection>,
    oracle_symbol: String,
    boolean_right_may_stop: bool,
    operation: ScalarOperation,
    /// Each operand's inclusive range, in call order.
    operands: Vec<(i64, i64)>,
    inferred_operands: Vec<crate::kani::identity::InferredScalarOperand>,
    /// The result's inclusive range: the checked domain a completed result must lie in.
    lower: i64,
    upper: i64,
    pub(super) module_symbol: String,
    pub(super) harness_symbol: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ScalarOperation {
    Add,
    Subtract,
    Multiply,
    Negate,
    Implies,
}

/// One scalar operation this generator renders. The four integer operations use the catalogued
/// Kani projection; Boolean implication uses its admitted Contract IR identity. Each difference
/// in operand names and independent expectation is checked by an exhaustive match.
impl ScalarOperation {
    /// The renderable operation for an IR-confirmed operation identity, or `None` when this
    /// generator has no scalar-harness renderer for it. This selects, among identities IR already
    /// confirmed, which ones this generator can turn into a harness.
    fn of(operation_identity: &str) -> Option<Self> {
        if operation_identity == crate::kani::identity::BOOLEAN_IMPLIES_IDENTITY {
            return Some(Self::Implies);
        }
        match CatalogOperator::from_identity(operation_identity)?
            .entry()
            .scalar_kani?
        {
            ScalarKaniOperator::Add => Some(Self::Add),
            ScalarKaniOperator::Subtract => Some(Self::Subtract),
            ScalarKaniOperator::Multiply => Some(Self::Multiply),
            ScalarKaniOperator::Negate => Some(Self::Negate),
        }
    }

    /// The oracle's parameter names, in call order, which are also the harness's variable names.
    fn operand_names(self) -> &'static [&'static str] {
        match self {
            Self::Negate => &["operand"],
            Self::Add | Self::Subtract | Self::Multiply | Self::Implies => &["left", "right"],
        }
    }

    /// The integer result as a Rust expression over the harness's native `i64` operands
    /// (`<name>_native`), evaluated in `i128` where a sum, difference, product or negation of `i64`
    /// values cannot overflow. It shares no code with the embedded oracle, which evaluates through
    /// `quire-exact`'s `rt::Integer`; it is the independent statement of the clause that
    /// [`render_scalar`] compares the oracle's result with. The implication arm is used by
    /// its separate Boolean renderer over symbolic `bool` operands.
    fn native_expression(self) -> String {
        let native = |index: usize| format!("i128::from({}_native)", self.operand_names()[index]);
        match self {
            Self::Add => format!("{} + {}", native(0), native(1)),
            Self::Subtract => format!("{} - {}", native(0), native(1)),
            Self::Multiply => format!("{} * {}", native(0), native(1)),
            Self::Negate => format!("-{}", native(0)),
            Self::Implies => "!left || right".to_owned(),
        }
    }

    /// The operation's operands in call order. Integer ranges come from `range_at`; Boolean
    /// implication uses the two encoded Boolean values `0..=1` for each operand.
    fn operands<E>(
        self,
        mut range_at: impl FnMut(usize) -> Result<(i64, i64), E>,
    ) -> Result<ScalarOperands, E> {
        Ok(match self {
            Self::Negate => ScalarOperands::Negate(range_at(0)?),
            Self::Add => ScalarOperands::Binary(BinaryOperation::Add, range_at(0)?, range_at(1)?),
            Self::Subtract => {
                ScalarOperands::Binary(BinaryOperation::Subtract, range_at(0)?, range_at(1)?)
            }
            Self::Multiply => {
                ScalarOperands::Binary(BinaryOperation::Multiply, range_at(0)?, range_at(1)?)
            }
            Self::Implies => ScalarOperands::Binary(BinaryOperation::Implies, (0, 1), (0, 1)),
        })
    }
}

/// The two-operand operations of [`ScalarOperation`].
#[derive(Clone, Copy)]
enum BinaryOperation {
    Add,
    Subtract,
    Multiply,
    Implies,
}

/// The operand ranges of one [`ScalarOperation`], of the operation's own arity: one range for
/// `Negate`, two for the others, so no combination of operation and operand count is left over.
/// Boolean ranges encode `false` as zero and `true` as one.
#[derive(Clone, Copy)]
enum ScalarOperands {
    Negate((i64, i64)),
    Binary(BinaryOperation, (i64, i64), (i64, i64)),
}

impl ScalarOperands {
    /// The ranges in call order, one per operand name of the operation.
    fn ranges(self) -> Vec<(i64, i64)> {
        match self {
            Self::Negate(operand) => vec![operand],
            Self::Binary(_, left, right) => vec![left, right],
        }
    }

    /// The least and greatest exact integer results over the inclusive `i64` operand ranges,
    /// computed in `i128`; Boolean implication reaches both encoded truth values.
    fn reachable(self) -> (i128, i128) {
        let wide = |(low, high): (i64, i64)| (i128::from(low), i128::from(high));
        match self {
            Self::Negate(operand) => {
                let (low, high) = wide(operand);
                (-high, -low)
            }
            Self::Binary(operation, left, right) => {
                let ((a, b), (c, d)) = (wide(left), wide(right));
                match operation {
                    BinaryOperation::Add => (a + c, b + d),
                    BinaryOperation::Subtract => (a - d, b - c),
                    BinaryOperation::Multiply => {
                        let corners = [a * c, a * d, b * c, b * d];
                        (
                            corners.into_iter().min().unwrap_or_default(),
                            corners.into_iter().max().unwrap_or_default(),
                        )
                    }
                    BinaryOperation::Implies => (0, 1),
                }
            }
        }
    }
}

/// The readable stem of a scalar obligation's names: its confirmed operation.
pub(super) fn scalar_stem(operation: &str) -> String {
    format!(
        "kob_scalar_{}",
        readable_component(operation.strip_prefix("quire.op.").unwrap_or(operation))
    )
}

/// Why [`lower_scalar_claim`] could not lower an IR-confirmed claim -- distinct causes that
/// `classify_claim` reports as distinct [`UnsupportedObligation`] reasons, rather than folding
/// them into one the way a single `Option` return would. A claim with no `checked_bounds` entry,
/// or whose first checked bound has no `integer_range` among the derived domains, is a claim map
/// this generator did not produce (for the `IntegerArithmetic` family `check_parameters` records
/// exactly one such bound), and is the [`ScalarLoweringRefusal::NoRenderer`] refusal.
pub(super) enum ScalarLoweringRefusal {
    /// No renderer for its family yet, or a claim map this generator did not produce.
    NoRenderer,
    /// Every operand range combines to results outside the result range, so no input the harness
    /// assumes completes and its non-vacuity cover could never be met.
    ResultUnreachable {
        /// The result range's inclusive lower bound.
        lower: i64,
        /// The result range's inclusive upper bound.
        upper: i64,
        /// The least result the operand ranges produce, canonical decimal.
        reachable_lower: String,
        /// The greatest result the operand ranges produce, canonical decimal.
        reachable_upper: String,
    },
    /// The `integer_range`'s lower or upper endpoint, or a literal operand, does not fit `i64`.
    BoundNotI64 {
        /// Inclusive lower bound, canonical decimal.
        lower: String,
        /// Inclusive upper bound, canonical decimal.
        upper: String,
    },
}

/// Lowers one IR-confirmed V2 claim to a renderable scalar harness. Boolean implication takes
/// two symbolic Boolean operands; for integer arithmetic, the result range is the
/// claim's first checked bound; each operand ranges over its own bound where the node's argument
/// is typed by one, exactly at its value where it is a literal (`operand_ranges`, by position),
/// and over the result range otherwise. A claim whose operand ranges produce no result inside the
/// result range is refused rather than rendered with a cover no input can meet.
pub(super) fn lower_scalar_claim(
    claim: &ExactScalarClaim,
    generated: &GeneratedScalarClaim,
    derived: &[DerivedDomain],
    operand_ranges: &[OperandRange],
) -> Result<LoweredScalarClaim, ScalarLoweringRefusal> {
    let operation =
        ScalarOperation::of(&claim.operation.identity).ok_or(ScalarLoweringRefusal::NoRenderer)?;
    if operation == ScalarOperation::Implies {
        return Ok(LoweredScalarClaim {
            node_id: claim.node_id.clone(),
            operation_identity: claim.operation.identity.clone(),
            oracle_source: generated.oracle_source.clone(),
            native_integer_selection: None,
            oracle_symbol: generated.symbol.clone(),
            boolean_right_may_stop: generated.boolean_right_may_stop,
            operation,
            operands: vec![(0, 1); 2],
            inferred_operands: Vec::new(),
            lower: 0,
            upper: 1,
            module_symbol: format!("{}_module", scalar_stem(&claim.operation.identity)),
            harness_symbol: format!("{}_proof", scalar_stem(&claim.operation.identity)),
        });
    }
    // `check_parameters` records exactly one checked bound for every `IntegerArithmetic` claim,
    // so a claim without one is a claim map this generator did not produce, which has no renderer.
    let Some(bound_id) = generated.checked_bounds.first() else {
        return Err(ScalarLoweringRefusal::NoRenderer);
    };
    let range_of = |id: &CheckedNodeId| {
        derived.iter().find_map(|domain| match domain {
            DerivedDomain::IntegerRange {
                bound,
                lower,
                upper,
            } if bound == id => Some((lower, upper)),
            _ => None,
        })
    };
    // `check_parameters` parsed the same node with the `literal_integer` that `derive_domain`
    // uses, so a first checked bound with no `IntegerRange` is likewise a claim map this
    // generator did not produce.
    let Some((lower, upper)) = range_of(bound_id) else {
        return Err(ScalarLoweringRefusal::NoRenderer);
    };
    let to_i64 = |(lower, upper): (&String, &String)| match (lower.parse(), upper.parse()) {
        (Ok(lower), Ok(upper)) => Ok((lower, upper)),
        _ => Err(ScalarLoweringRefusal::BoundNotI64 {
            lower: lower.clone(),
            upper: upper.clone(),
        }),
    };
    let (lower, upper) = to_i64((lower, upper))?;
    let mut inferred_operands = Vec::new();
    let operands = operation.operands(|position| match operand_ranges.get(position) {
        None | Some(OperandRange::Result) => Ok((lower, upper)),
        Some(OperandRange::Literal(value)) => to_i64((value, value)),
        // An own bound `check_parameters` recorded is in `checked_bounds` and a derived
        // integer range; one that is not is a claim map this generator did not produce,
        // which has no renderer.
        Some(OperandRange::Bound(id)) => generated
            .checked_bounds
            .contains(id)
            .then(|| range_of(id))
            .flatten()
            .ok_or(ScalarLoweringRefusal::NoRenderer)
            .and_then(to_i64),
        Some(OperandRange::Inferred {
            source,
            lower,
            upper,
        }) => {
            inferred_operands.push(crate::kani::identity::InferredScalarOperand {
                position,
                source_node_id: source.clone(),
                lower: lower.clone(),
                upper: upper.clone(),
            });
            to_i64((lower, upper))
        }
    })?;
    let (reachable_lower, reachable_upper) = operands.reachable();
    if reachable_upper < i128::from(lower) || reachable_lower > i128::from(upper) {
        return Err(ScalarLoweringRefusal::ResultUnreachable {
            lower,
            upper,
            reachable_lower: reachable_lower.to_string(),
            reachable_upper: reachable_upper.to_string(),
        });
    }
    // The readable stem; `assign_names` settles the final name across the request.
    let base = scalar_stem(&claim.operation.identity);
    let module_symbol = format!("{base}_module");
    let harness_symbol = format!("{base}_proof");
    Ok(LoweredScalarClaim {
        node_id: claim.node_id.clone(),
        operation_identity: claim.operation.identity.clone(),
        oracle_source: generated.oracle_source.clone(),
        native_integer_selection: None,
        oracle_symbol: generated.symbol.clone(),
        boolean_right_may_stop: false,
        operation,
        operands: operands.ranges(),
        inferred_operands,
        lower,
        upper,
        module_symbol,
        harness_symbol,
    })
}

impl LoweredScalarClaim {
    /// Select the exact kernel native route from the full original operand intervals.
    pub(super) fn select_native_integer(&mut self) -> Result<(), UnsupportedObligation> {
        let operator = match self.operation {
            ScalarOperation::Add => IntegerOperator::Add,
            ScalarOperation::Subtract => IntegerOperator::Subtract,
            ScalarOperation::Multiply => IntegerOperator::Multiply,
            ScalarOperation::Negate => IntegerOperator::Negate,
            ScalarOperation::Implies => return Ok(()),
        };
        let refusal = || UnsupportedObligation::NativeIntegerCapacity {
            node_id: self.node_id.clone(),
            operation_identity: self.operation_identity.clone(),
            operands: self
                .operands
                .iter()
                .map(
                    |(lower, upper)| crate::core::native_integer::NativeIntegerIntervalDecision {
                        lower: lower.to_string(),
                        upper: upper.to_string(),
                    },
                )
                .collect(),
        };
        let (source, selected) = crate::oracle::scalar::native::select(
            &self.node_id,
            &self.operation_identity,
            &self.oracle_symbol,
            operator,
            &self.operands,
            (self.lower, self.upper),
        )
        .ok_or_else(refusal)?;
        self.oracle_source = source;
        self.native_integer_selection = Some(selected);
        Ok(())
    }
}

#[cfg(test)]
mod native_selection_tests {
    use super::{LoweredScalarClaim, ScalarOperation};
    use crate::kani::generate::outcome::UnsupportedObligation;
    use ix_trace_rs::trace;

    fn lowered(operation: ScalarOperation, operands: Vec<(i64, i64)>) -> LoweredScalarClaim {
        LoweredScalarClaim {
            node_id: serde_json::from_value(serde_json::json!({
                "domain": "quire.checked-semantic-node/v1",
                "digest": "2".repeat(64),
            }))
            .expect("checked node identity"),
            operation_identity: match operation {
                ScalarOperation::Add => "quire.op.integer.add",
                ScalarOperation::Subtract => "quire.op.integer.sub",
                ScalarOperation::Multiply => "quire.op.integer.mul",
                ScalarOperation::Negate => "quire.op.integer.negate",
                ScalarOperation::Implies => "quire.op.boolean.implies",
            }
            .to_owned(),
            oracle_source: String::new(),
            native_integer_selection: None,
            oracle_symbol: "checked_oracle".to_owned(),
            boolean_right_may_stop: false,
            operation,
            operands,
            inferred_operands: Vec::new(),
            lower: -100,
            upper: 100,
            module_symbol: "native_plan".to_owned(),
            harness_symbol: "check".to_owned(),
        }
    }

    #[trace("FR-015-AC-125", "TC-025")]
    #[test]
    fn native_selection_uses_owner_plan_for_all_arithmetic_operations() {
        for (operation, operands, capacity) in [
            (ScalarOperation::Add, vec![(-2, 3), (4, 5)], (2, 8)),
            (ScalarOperation::Subtract, vec![(-2, 3), (4, 5)], (-7, -1)),
            (ScalarOperation::Multiply, vec![(-2, 3), (4, 5)], (-10, 15)),
            (ScalarOperation::Negate, vec![(-2, 3)], (-3, 2)),
        ] {
            let mut claim = lowered(operation, operands.clone());
            claim.select_native_integer().unwrap();
            let selection = claim.native_integer_selection.unwrap();
            let [decision] = selection.operations.as_slice() else {
                panic!("one selected kernel plan");
            };
            let actual = decision.capacity.as_ref().unwrap();
            assert_eq!(
                (
                    actual.lower.parse::<i128>().unwrap(),
                    actual.upper.parse::<i128>().unwrap()
                ),
                capacity
            );
            assert_eq!(decision.operands.len(), operands.len());
            assert!(claim.oracle_source.contains("nx::NativeIntegerPlan::new"));
            assert!(claim
                .oracle_source
                .contains("nx::evaluate_native_integer_arithmetic"));
        }
    }

    #[trace("FR-015-AC-125", "TC-025")]
    #[test]
    fn native_selection_refuses_complete_multiply_population_outside_signed32() {
        let mut claim = lowered(
            ScalarOperation::Multiply,
            vec![(0, i64::from(i32::MAX) + 1), (0, 0)],
        );
        assert!(matches!(
            claim.select_native_integer(),
            Err(UnsupportedObligation::NativeIntegerCapacity { .. })
        ));
        assert!(claim.native_integer_selection.is_none());
        assert!(claim.oracle_source.is_empty());
    }
}

pub(super) fn derive_domain<Body>(
    bound: &CheckedSemanticNodeV2<Body>,
    term: Option<&CheckedTerm>,
) -> DerivedDomain {
    let members = aggregate_members(term);
    match (&*bound.semantic_form, members) {
        ("integer_range", Some(members)) => {
            match bound_members(members, INTEGER_RANGE_MEMBERS)
                .map(|[lower, upper]| (literal_integer(lower), literal_integer(upper)))
            {
                Some((Some(lower), Some(upper))) => DerivedDomain::IntegerRange {
                    bound: bound.node_id.clone(),
                    lower: lower.to_string(),
                    upper: upper.to_string(),
                },
                _ => not_symbolic(bound),
            }
        }
        _ => not_symbolic(bound),
    }
}

fn not_symbolic<Body>(bound: &CheckedSemanticNodeV2<Body>) -> DerivedDomain {
    DerivedDomain::NotSymbolic {
        bound: bound.node_id.clone(),
        form: bound.semantic_form.to_string(),
    }
}

/// The canonical inclusive limits of a range bound that admits no value.
pub(super) fn unsatisfiable<Body>(
    bound: &CheckedSemanticNodeV2<Body>,
    term: Option<&CheckedTerm>,
) -> Option<(String, String)> {
    let members = aggregate_members(term)?;
    match &*bound.semantic_form {
        "integer_range" => {
            let [lower, upper] = bound_members(members, INTEGER_RANGE_MEMBERS)?;
            let (lower, upper) = (literal_integer(lower)?, literal_integer(upper)?);
            (lower > upper).then(|| (lower.to_string(), upper.to_string()))
        }
        "text_bounds" => {
            let [minimum, maximum, _] = bound_members(members, TEXT_BOUNDS_MEMBERS)?;
            count_bounds(minimum, maximum)
        }
        "collection_bounds" => {
            let [minimum, maximum] = bound_members(members, COLLECTION_BOUNDS_MEMBERS)?;
            count_bounds(minimum, maximum)
        }
        _ => None,
    }
}

/// The canonical limits of a cardinality range whose minimum exceeds its maximum.
fn count_bounds(minimum: &CheckedTerm, maximum: &CheckedTerm) -> Option<(String, String)> {
    let (minimum, maximum) = (literal_count(minimum)?, literal_count(maximum)?);
    (minimum > maximum).then(|| (minimum.to_string(), maximum.to_string()))
}

/// Renders one IR-confirmed V2 exact-scalar claim. Boolean implication is dispatched to its
/// compiled-subject proof; for integer arithmetic, the harness proves that the embedded oracle
/// computes the clause's own arithmetic. The harness evaluates the operation a second time, in
/// native `i128` over the same symbolic `i64` operands ([`ScalarOperation::native_expression`]), and asserts the
/// oracle agrees with it in both directions: the outcome is `Ok(Completed(value))` with `value`
/// exactly the native result when that result lies within the checked domain (the result bound's
/// own literal bounds), and `Ok(Refused(_))` when it does not. Any other outcome fails. An oracle
/// that computed a different operation, dropped a carry, or mis-bounded the result therefore
/// falsifies the proof; asserting only that a completed value lies in the domain would restate the
/// check the oracle itself makes before returning `Completed`.
///
/// Each operand's `kani::assume` is its own bound (the `bounded_domain` typing it; a literal
/// operand, which is a constant, takes the result's). The operation is genuinely partial over that
/// domain (`quire.op.integer.add` over `[-1000,1000]` admits `600 + 600`), which is why the
/// refused direction is part of the property rather than an unconditional `Completed`. The meter
/// is unlimited, so `Incomplete` cannot occur. A `kani::cover!` on the `Completed` branch is the
/// FR-015-AC-7 non-vacuity guard: the property is checked on at least one completing input.
///
/// This generator renders the harness; it does not run the Kani/CBMC solver. The `kani` lane
/// (`tests/it/kani_obligations.rs`) proves it and runs the mutated-arithmetic control.
pub(super) fn render_scalar(
    request: &KaniObligationRequest<'_>,
    lowered: &LoweredScalarClaim,
) -> Result<KaniScalarObligationHarness, UnsupportedObligation> {
    if lowered.operation == ScalarOperation::Implies {
        return render_boolean_implies(request, lowered);
    }
    use super::spec::{
        Argument, Cover, ExprText, HarnessSpec, Init, LetBinding, LetType, ProofKind, Statement,
    };
    use crate::kani::abi::KaniPrimitiveType;
    let expr = |text: String| ExprText::new(text);
    let binding = |name: &str, ty, init| LetBinding {
        name: name.to_owned(),
        ty,
        mutable: false,
        init,
    };
    let scalar_limits = crate::kani::abi::SCALAR_HARNESS_LIMITS;
    let mut body = Vec::new();
    for (name, (minimum, maximum)) in lowered
        .operation
        .operand_names()
        .iter()
        .zip(&lowered.operands)
    {
        body.push(Statement::Input {
            name: (*name).to_owned(),
            ty: KaniPrimitiveType::I64,
            domain: (i128::from(*minimum), i128::from(*maximum)),
            then: vec![
                binding(
                    &format!("{name}_native"),
                    None,
                    Init::Expression(expr((*name).to_owned())?),
                ),
                binding(
                    name,
                    None,
                    Init::Expression(expr(if lowered.native_integer_selection.is_some() {
                        format!("nx::NativeInteger::new(i128::from({name}))")
                    } else {
                        format!("rt::Integer::from({name})")
                    })?),
                ),
            ],
        });
    }
    body.push(Statement::Let(LetBinding {
        name: "meter".to_owned(),
        ty: None,
        mutable: true,
        init: Init::ScalarMeter(scalar_limits),
    }));
    let mut args = lowered
        .operation
        .operand_names()
        .iter()
        .map(|name| Argument::Borrow((*name).to_owned()))
        .collect::<Vec<_>>();
    args.push(Argument::BorrowMut("meter".to_owned()));
    body.push(Statement::Call {
        result: "outcome".to_owned(),
        callee: lowered.oracle_symbol.clone(),
        args,
    });
    body.push(Statement::Let(binding(
        "exact",
        Some(LetType::I128),
        Init::Expression(expr(lowered.operation.native_expression())?),
    )));
    body.push(Statement::Let(binding(
        "admitted",
        None,
        Init::Expression(expr(format!(
            "exact >= i128::from({}) && exact <= i128::from({})",
            i64_literal(lowered.lower),
            i64_literal(lowered.upper)
        ))?),
    )));
    let completed_value = if lowered.native_integer_selection.is_some() {
        "value.value() == exact"
    } else {
        "*value == rt::Integer::from(exact)"
    };
    body.push(Statement::Let(binding("sound", None, Init::Expression(expr(format!("match &outcome {{\nOk(rt::Outcome::Completed(value)) => admitted && {completed_value},\nOk(rt::Outcome::Refused(_)) => !admitted,\n_ => false,\n}}"))?))));
    body.push(Statement::Assert { condition: expr("sound".to_owned())?, message: format!("the oracle must complete with exactly the native {} result when it lies in the checked domain, and refuse otherwise", lowered.operation_identity) });
    body.push(Statement::Let(binding(
        "completed",
        None,
        Init::Expression(expr(
            "matches!(outcome, Ok(rt::Outcome::Completed(_)))".to_owned(),
        )?),
    )));
    let spec = HarnessSpec::new(
        format!("exact-scalar {}/{} `{}`", lowered.node_id.domain, lowered.node_id.digest, lowered.operation_identity),
        harness_path(&lowered.module_symbol, &lowered.harness_symbol)?,
        lowered.oracle_source.clone(), ProofKind::Proof, body,
        vec![Cover { condition: expr("completed".to_owned())?, message: "the generated oracle's Ok(Outcome::Completed(_)) branch is reachable within its checked domain".to_owned() }],
        request.unwind, KaniSolver::Cadical,
    )?;
    let rust = super::render::render(&spec)?;
    let identity = ScalarObligationIdentity {
        integer_representation: if lowered.native_integer_selection.is_some() {
            crate::kani::identity::ScalarIntegerRepresentation::Native
        } else {
            crate::kani::identity::ScalarIntegerRepresentation::General
        },
        native_integer_selection: lowered.native_integer_selection.clone(),
        ceilings: request.ceilings,
        node_id: lowered.node_id.clone(),
        operation_identity: lowered.operation_identity.clone(),
        oracle_symbol: lowered.oracle_symbol.clone(),
        module_symbol: spec.path().module.clone(),
        harness_symbol: spec.path().harness.clone(),
        arguments: lowered
            .operation
            .operand_names()
            .iter()
            .zip(&lowered.operands)
            .map(|(name, (minimum, maximum))| ScalarObligationArgument {
                identifier: (*name).to_owned(),
                minimum: *minimum,
                maximum: *maximum,
            })
            .collect(),
        inferred_operands: lowered.inferred_operands.clone(),
        solver: spec.solver(),
        unwind: spec.unwind(),
        options: spec.options().to_vec(),
    };
    let record = record(&lowered.module_symbol, &identity, &rust)?;
    Ok(KaniScalarObligationHarness::generated_with_scalar_limits(
        identity,
        rust,
        record,
        scalar_limits,
    ))
}

fn render_boolean_implies(
    request: &KaniObligationRequest<'_>,
    lowered: &LoweredScalarClaim,
) -> Result<KaniScalarObligationHarness, UnsupportedObligation> {
    let scalar_limits = crate::kani::abi::SCALAR_HARNESS_LIMITS;
    use super::spec::{
        Argument, Cover, ExprText, HarnessSpec, Init, LetBinding, ProofKind, Statement,
    };
    use crate::kani::abi::KaniPrimitiveType;

    let expr = |source: &str| ExprText::new(source.to_owned());
    let body = vec![
        Statement::Input {
            name: "left".to_owned(),
            ty: KaniPrimitiveType::Boolean,
            domain: (0, 1),
            then: Vec::new(),
        },
        Statement::Input {
            name: "right".to_owned(),
            ty: KaniPrimitiveType::Boolean,
            domain: (0, 1),
            then: Vec::new(),
        },
        Statement::Let(LetBinding {
            name: "meter".to_owned(),
            ty: None,
            mutable: true,
            init: Init::ScalarMeter(scalar_limits),
        }),
        Statement::Call {
            result: "outcome".to_owned(),
            callee: lowered.oracle_symbol.clone(),
            args: vec![
                Argument::Value("left".to_owned()),
                if lowered.boolean_right_may_stop {
                    Argument::BooleanOutcomeClosure("right".to_owned())
                } else {
                    Argument::Value("right".to_owned())
                },
                Argument::BorrowMut("meter".to_owned()),
            ],
        },
        Statement::Let(LetBinding {
            name: "native".to_owned(),
            ty: None,
            mutable: false,
            init: Init::Expression(expr(&lowered.operation.native_expression())?),
        }),
        Statement::SubjectImplicationCall,
        Statement::Assert {
            condition: expr("subject == native")?,
            message: "the compiled implication subject must equal native implication".to_owned(),
        },
        Statement::Let(LetBinding {
            name: "completed".to_owned(),
            ty: None,
            mutable: false,
            init: Init::Expression(expr("match &outcome { Ok(rt::Outcome::Completed(value)) => *value == native, _ => false }")?),
        }),
        Statement::Assert {
            condition: expr("completed")?,
            message: "the Boolean oracle must complete with native implication".to_owned(),
        },
    ];
    let spec = HarnessSpec::new(
        format!(
            "exact-scalar {}/{} `{}`",
            lowered.node_id.domain, lowered.node_id.digest, lowered.operation_identity
        ),
        harness_path(&lowered.module_symbol, &lowered.harness_symbol)?,
        lowered.oracle_source.clone(),
        ProofKind::Proof,
        body,
        vec![Cover {
            condition: expr("completed")?,
            message: "the Boolean oracle Completed branch is reachable".to_owned(),
        }],
        request.unwind,
        KaniSolver::Cadical,
    )?;
    let rust = super::render::render(&spec)?;
    let identity = ScalarObligationIdentity {
        integer_representation: crate::kani::identity::ScalarIntegerRepresentation::General,
        native_integer_selection: None,
        ceilings: request.ceilings,
        node_id: lowered.node_id.clone(),
        operation_identity: lowered.operation_identity.clone(),
        oracle_symbol: lowered.oracle_symbol.clone(),
        module_symbol: spec.path().module.clone(),
        harness_symbol: spec.path().harness.clone(),
        arguments: lowered
            .operation
            .operand_names()
            .iter()
            .map(|name| ScalarObligationArgument {
                identifier: (*name).to_owned(),
                minimum: 0,
                maximum: 1,
            })
            .collect(),
        inferred_operands: Vec::new(),
        solver: spec.solver(),
        unwind: spec.unwind(),
        options: spec.options().to_vec(),
    };
    let record = record(&lowered.module_symbol, &identity, &rust)?;
    Ok(KaniScalarObligationHarness::generated_with_scalar_limits(
        identity,
        rust,
        record,
        scalar_limits,
    ))
}

// Deliberately false claim built before the normal renderer seals its output. The embedded
// generated oracle and its package remain intact; only the independently asserted bound differs.
#[cfg(test)]
pub(super) fn false_bound_harness_for_test(
    original: &KaniScalarObligationHarness,
    artifacts: &[Artifact],
) -> KaniScalarObligationHarness {
    let identity = &original.identity;
    let lowered = LoweredScalarClaim {
        node_id: identity.node_id.clone(),
        operation_identity: identity.operation_identity.clone(),
        native_integer_selection: None,
        oracle_source: artifacts
            .iter()
            .find(|artifact| artifact.path == "src/lib.rs")
            .unwrap()
            .contents
            .clone(),
        oracle_symbol: identity.oracle_symbol.clone(),
        boolean_right_may_stop: false,
        operation: ScalarOperation::of(&identity.operation_identity).unwrap(),
        operands: identity
            .arguments
            .iter()
            .map(|argument| (argument.minimum, argument.maximum))
            .collect(),
        inferred_operands: identity.inferred_operands.clone(),
        lower: 0,
        upper: 5,
        module_symbol: identity.module_symbol.as_str().to_owned(),
        harness_symbol: identity.harness_symbol.as_str().to_owned(),
    };
    let mut harness = render_scalar(
        &KaniObligationRequest {
            ceilings: identity.ceilings,
            items: &[],
            subject_path: "crate::subject",
            unwind: identity.unwind,
        },
        &lowered,
    )
    .unwrap();
    harness.bind_artifact_package(artifacts);
    harness
}

#[cfg(test)]
fn render_scalar_reference(
    request: &KaniObligationRequest<'_>,
    lowered: &LoweredScalarClaim,
) -> Result<KaniScalarObligationHarness, UnsupportedObligation> {
    let path = harness_path(&lowered.module_symbol, &lowered.harness_symbol)?;
    let options = adapter_options(
        &path.to_string(),
        request.unwind,
        KaniSolver::Cadical,
        false,
    );
    let lower = i64_literal(lowered.lower);
    let upper = i64_literal(lowered.upper);
    let names = lowered.operation.operand_names();
    let declarations = names
        .iter()
        .zip(&lowered.operands)
        .map(|(name, (minimum, maximum))| {
            let (minimum, maximum) = (i64_literal(*minimum), i64_literal(*maximum));
            format!(
                "        let {name}: i64 = kani::any();\n\
        kani::assume({name} >= {minimum} && {name} <= {maximum});\n\
        let {name}_native = {name};\n\
        let {name} = rt::Integer::from({name});\n"
            )
        })
        .collect::<String>();
    let call_args = names
        .iter()
        .map(|name| format!("&{name}"))
        .collect::<Vec<_>>()
        .join(", ");
    let arguments = names
        .iter()
        .zip(&lowered.operands)
        .map(|(name, (minimum, maximum))| ScalarObligationArgument {
            identifier: (*name).to_owned(),
            minimum: *minimum,
            maximum: *maximum,
        })
        .collect();
    let body = format!(
        "#[cfg(kani)]\n\
mod {module} {{\n\
    use super::*;\n\
\n\
    #[kani::proof]\n\
    fn {harness}() {{\n\
{declarations}\
        let mut meter = rt::Meter::new(rt::ScalarLimits {{\n\
            integer_bits: u64::MAX,\n\
            decimal_digits: u64::MAX,\n\
            scale_expansion: u64::MAX,\n\
            text_input_bytes: u64::MAX,\n\
            text_scalars: u64::MAX,\n\
            normalized_scalars: u64::MAX,\n\
            unit_edges: u64::MAX,\n\
            value_occurrences: u64::MAX,\n\
            work_units: u64::MAX,\n\
            result_units: u64::MAX,\n\
        }});\n\
        let outcome = {symbol}({call_args}, &mut meter);\n\
        let exact: i128 = {exact};\n\
        let admitted = exact >= i128::from({lower}) && exact <= i128::from({upper});\n\
        let sound = match &outcome {{\n\
            Ok(rt::Outcome::Completed(value)) => admitted && *value == rt::Integer::from(exact),\n\
            Ok(rt::Outcome::Refused(_)) => !admitted,\n\
            _ => false,\n\
        }};\n\
        assert!(sound, \"the oracle must complete with exactly the native {operation} result when it lies in the checked domain, and refuse otherwise\");\n\
        let completed = matches!(outcome, Ok(rt::Outcome::Completed(_)));\n\
        kani::cover!(completed, \"the generated oracle's Ok(Outcome::Completed(_)) branch is reachable within its checked domain\");\n\
    }}\n\
}}\n",
        module = lowered.module_symbol,
        harness = lowered.harness_symbol,
        symbol = lowered.oracle_symbol,
        exact = lowered.operation.native_expression(),
        operation = lowered.operation_identity,
    );
    let identity = ScalarObligationIdentity {
        integer_representation: crate::kani::identity::ScalarIntegerRepresentation::General,
        native_integer_selection: None,
        ceilings: request.ceilings,
        node_id: lowered.node_id.clone(),
        operation_identity: lowered.operation_identity.clone(),
        oracle_symbol: lowered.oracle_symbol.clone(),
        module_symbol: path.module,
        harness_symbol: path.harness,
        arguments,
        inferred_operands: lowered.inferred_operands.clone(),
        solver: KaniSolver::Cadical,
        unwind: request.unwind,
        options,
    };
    let mut source = format!(
        "// SPDX-License-Identifier: MIT OR Apache-2.0\n\
// @generated by quire-contract-codegen. Do not edit.\n\
// Obligation: exact-scalar {}/{} `{}`\n\n",
        lowered.node_id.domain, lowered.node_id.digest, lowered.operation_identity,
    );
    source.push_str(&lowered.oracle_source);
    source.push('\n');
    source.push_str(&body);
    if source.len() > MAX_GENERATED_SOURCE_BYTES {
        return Err(UnsupportedObligation::ResourceLimitExceeded {
            bytes: source.len(),
        });
    }
    if let Err(error) = syn::parse_file(&source) {
        return Err(UnsupportedObligation::InvalidGeneratedSyntax {
            error: error.to_string(),
        });
    }
    let rust = Artifact::new(
        format!("src/generated/{}.rs", lowered.module_symbol),
        source,
    );
    let record = record(&lowered.module_symbol, &identity, &rust)?;
    Ok(KaniScalarObligationHarness::generated_with_scalar_limits(
        identity,
        rust,
        record,
        crate::kani::abi::SCALAR_HARNESS_LIMITS,
    ))
}

#[cfg(test)]
mod tests {
    use ix_trace_rs::trace;
    use quire_contract_model::LiteralKind;
    use serde_json::json;

    use super::*;

    /// A quantity conversion has no scalar harness renderer: the operation lookup declines it, so
    /// no harness is rendered for it.
    #[trace("FR-014-AC-2", "TC-024")]
    #[test]
    fn a_quantity_conversion_has_no_scalar_harness_renderer() {
        assert!(ScalarOperation::of("quire.op.quantity.convert").is_none());
        assert!(ScalarOperation::of("quire.op.integer.add").is_some());
    }

    use crate::oracle::{
        claim::ClaimDisposition,
        scalar::{OperationClaim, OperationProvenance},
    };

    /// The scalar Kani projection selects four catalogued integer entries and implication.
    #[trace(
        "FR-038-AC-3",
        "FR-038-AC-4",
        "FR-015-AC-37",
        "FR-015-AC-118",
        "TC-025"
    )]
    #[test]
    fn scalar_projection_selects_arithmetic_and_boolean_implication() {
        assert_eq!(
            ScalarOperation::of("quire.op.integer.add"),
            Some(ScalarOperation::Add)
        );
        assert_eq!(
            ScalarOperation::of("quire.op.integer.sub"),
            Some(ScalarOperation::Subtract)
        );
        assert_eq!(
            ScalarOperation::of("quire.op.integer.mul"),
            Some(ScalarOperation::Multiply)
        );
        assert_eq!(
            ScalarOperation::of("quire.op.integer.negate"),
            Some(ScalarOperation::Negate)
        );
        assert_eq!(
            ScalarOperation::of("quire.op.boolean.implies"),
            Some(ScalarOperation::Implies)
        );
        assert_eq!(ScalarOperation::of("quire.op.integer.lt"), None);
        assert_eq!(ScalarOperation::of("quire.op.integer.eq"), None);
        assert_eq!(ScalarOperation::of("quire.op.integer.unknown"), None);
        assert_eq!(
            ScalarOperation::Subtract.operand_names(),
            &["left", "right"]
        );
        assert_eq!(
            ScalarOperation::Subtract.native_expression(),
            "i128::from(left_native) - i128::from(right_native)"
        );
    }

    // Explicit typed unit fixtures; these do not claim public reader admission.
    fn bound_fixture(
        form: &str,
        members: Vec<CheckedTerm>,
    ) -> (CheckedSemanticNodeV2, CheckedTerm) {
        let term = CheckedTerm::Aggregate { members };
        let node = serde_json::from_value(json!({
            "node_id": {"domain": "quire.checked-semantic-node/v1", "digest": "0".repeat(64)},
            "schema_version": "quire.checked-semantic-graph/v2",
            "node_tag": "bounded_domain",
            "semantic_form": form,
            "semantic_type": {"domain": "quire.checked-semantic-node/v1", "digest": "1".repeat(64)},
            "dependencies": [],
            "occurrences": [],
            "body": &term,
        }))
        .expect("node");
        (node, term)
    }

    fn bound_literal(kind: LiteralKind, value: &str) -> CheckedTerm {
        CheckedTerm::Literal {
            type_id: checked_node_id("1"),
            value_kind: kind,
            value: json!(value),
        }
    }

    fn bound_member(name: &str, value: CheckedTerm) -> CheckedTerm {
        CheckedTerm::Binding {
            name: name.into(),
            value: Box::new(value),
        }
    }

    /// The symbolic domain is read from binding-shaped `min`/`max` members (the shape QSL emits,
    /// FR-322); a bare-literal member is not symbolic.
    #[trace("FR-015-AC-5", "TC-025")]
    #[test]
    fn tc_025_derive_domain_reads_binding_shaped_range_members() {
        let literal = |value: &str| bound_literal(LiteralKind::Integer, value);
        let (bound, term) = bound_fixture(
            "integer_range",
            vec![
                bound_member("min", literal("0")),
                bound_member("max", literal("9")),
            ],
        );
        assert_eq!(
            derive_domain(&bound, Some(&term)),
            DerivedDomain::IntegerRange {
                bound: bound.node_id.clone(),
                lower: "0".to_owned(),
                upper: "9".to_owned(),
            }
        );
        let (bare, term) = bound_fixture("integer_range", vec![literal("0"), literal("9")]);
        assert!(matches!(
            derive_domain(&bare, Some(&term)),
            DerivedDomain::NotSymbolic { .. }
        ));
    }

    /// Bound members are looked up by name in any order: swapped `min`/`max` members still read
    /// the right range, while a duplicated or unknown name is not a range.
    #[trace("FR-015-AC-5", "TC-025")]
    #[test]
    fn tc_025_range_members_are_read_by_name_not_position() {
        let member = |name: &str, value: &str| {
            bound_member(name, bound_literal(LiteralKind::Integer, value))
        };
        let (swapped, term) = bound_fixture(
            "integer_range",
            vec![member("max", "9"), member("min", "0")],
        );
        assert_eq!(
            derive_domain(&swapped, Some(&term)),
            DerivedDomain::IntegerRange {
                bound: swapped.node_id.clone(),
                lower: "0".to_owned(),
                upper: "9".to_owned(),
            }
        );
        // `[max=0, min=9]` written high-first is an inverted range, not the range `[0, 9]`.
        let (inverted, term) = bound_fixture(
            "integer_range",
            vec![member("max", "0"), member("min", "9")],
        );
        assert_eq!(
            unsatisfiable(&inverted, Some(&term)),
            Some(("9".to_owned(), "0".to_owned()))
        );
        for members in [
            vec![member("min", "0"), member("min", "9")],
            vec![member("min", "0"), member("high", "9")],
            vec![member("min", "0")],
        ] {
            let (bound, term) = bound_fixture("integer_range", members);
            assert!(matches!(
                derive_domain(&bound, Some(&term)),
                DerivedDomain::NotSymbolic { .. }
            ));
            assert_eq!(unsatisfiable(&bound, Some(&term)), None);
        }
    }

    #[trace("FR-015-AC-5", "TC-025")]
    #[test]
    fn tc_025_inverted_ranges_are_unsatisfiable_and_ordered_ranges_are_not() {
        let (lo, hi) = (
            |value: &str| bound_member("min", bound_literal(LiteralKind::Integer, value)),
            |value: &str| bound_member("max", bound_literal(LiteralKind::Integer, value)),
        );
        let text =
            |value: &str| bound_member("text_profile", bound_literal(LiteralKind::Text, value));
        let (bound, term) = bound_fixture("integer_range", vec![lo("5"), hi("-5")]);
        assert_eq!(
            unsatisfiable(&bound, Some(&term)),
            Some(("5".to_owned(), "-5".to_owned()))
        );
        let (bound, term) = bound_fixture("integer_range", vec![lo("-5"), hi("5")]);
        assert_eq!(unsatisfiable(&bound, Some(&term)), None);
        let (bound, term) = bound_fixture("integer_range", vec![lo("5"), hi("5")]);
        assert_eq!(unsatisfiable(&bound, Some(&term)), None);
        let (bound, term) = bound_fixture("text_bounds", vec![lo("4"), hi("0"), text("nfc")]);
        assert_eq!(
            unsatisfiable(&bound, Some(&term)),
            Some(("4".to_owned(), "0".to_owned()))
        );
    }

    // Deliberately untraced: no FR-015 AC states this. AC-3 ("unbounded or non-finite... refused")
    // is the closest in subject but does not describe this case -- the domain below is bounded
    // and finite, just outside `i64` -- so citing it would misdescribe what this test proves.
    //
    // An empty `checked_bounds`, and a first checked bound with no `IntegerRange` domain, are the
    // `NoRenderer` refusal (NFR-005-AC-2, the `tc_042_ac2_*` tests below).
    #[test]
    fn a_domain_outside_i64_is_a_typed_refusal_not_a_panic() {
        let node_id = |digest: &str| -> CheckedNodeId {
            serde_json::from_value(serde_json::json!({
                "domain": "quire.checked-semantic-node/v1",
                "digest": digest.repeat(64),
            }))
            .expect("node id")
        };
        let bound = node_id("7");
        let claim = ExactScalarClaim {
            node_id: node_id("2"),
            operation: OperationClaim {
                identity: "quire.op.integer.add".to_owned(),
                provenance: OperationProvenance::IrConfirmed,
            },
            result: ClaimDisposition::Generated(Box::new(GeneratedScalarClaim {
                symbol: "oracle_test".to_owned(),
                contract_package_identity: serde_json::from_value(serde_json::json!({
                    "domain": "quire.contract-ir.semantic/v1",
                    "algorithm": "sha256",
                    "digest": "3".repeat(64),
                }))
                .expect("contract package identity"),
                node_id: node_id("2"),
                package_id: serde_json::from_value(serde_json::json!({
                    "domain": "quire.checked-package.semantic/v2",
                    "algorithm": "sha256",
                    "digest": "5".repeat(64),
                }))
                .expect("source package identity"),
                semantic_form: "expression".to_owned(),
                semantic_type: node_id("4"),
                quantity_conversion_target: None,
                source_map: Vec::new(),
                claims: Vec::new(),
                bounds: vec![bound.clone()],
                checked_bounds: vec![bound.clone()],
                dependencies: Vec::new(),
                oracle_source: String::new(),
                boolean_right_may_stop: false,
            })),
        };
        let ClaimDisposition::Generated(generated) = &claim.result else {
            unreachable!("built as Generated above");
        };
        let lower = "-99999999999999999999999999".to_owned();
        let upper = "99999999999999999999999999".to_owned();
        let derived = vec![DerivedDomain::IntegerRange {
            bound,
            lower: lower.clone(),
            upper: upper.clone(),
        }];
        match lower_scalar_claim(&claim, generated, &derived, &[]) {
            Err(ScalarLoweringRefusal::BoundNotI64 {
                lower: got_lower,
                upper: got_upper,
            }) => {
                assert_eq!(got_lower, lower);
                assert_eq!(got_upper, upper);
            }
            Err(ScalarLoweringRefusal::NoRenderer) => {
                panic!("quire.op.integer.add has a renderer")
            }
            Err(ScalarLoweringRefusal::ResultUnreachable { .. }) => {
                panic!("the domain does not fit i64, so no reachability is computed")
            }
            Ok(_) => panic!("an arbitrary-precision Integer domain outside i64 must not lower"),
        }
    }

    fn checked_node_id(digit: &str) -> CheckedNodeId {
        serde_json::from_value(json!({
            "domain": "quire.checked-semantic-node/v1",
            "digest": digit.repeat(64),
        }))
        .expect("node id")
    }

    /// An `integer.add` claim whose generated record names `checked_bounds`.
    fn add_claim(checked_bounds: Vec<CheckedNodeId>) -> ExactScalarClaim {
        ExactScalarClaim {
            node_id: checked_node_id("2"),
            operation: OperationClaim {
                identity: "quire.op.integer.add".to_owned(),
                provenance: OperationProvenance::IrConfirmed,
            },
            result: ClaimDisposition::Generated(Box::new(GeneratedScalarClaim {
                symbol: "oracle_test".to_owned(),
                contract_package_identity: serde_json::from_value(json!({
                    "domain": "quire.contract-ir.semantic/v1",
                    "algorithm": "sha256",
                    "digest": "3".repeat(64),
                }))
                .expect("contract package identity"),
                node_id: checked_node_id("2"),
                package_id: serde_json::from_value(json!({
                    "domain": "quire.checked-package.semantic/v2",
                    "algorithm": "sha256",
                    "digest": "5".repeat(64),
                }))
                .expect("source package identity"),
                semantic_form: "expression".to_owned(),
                semantic_type: checked_node_id("4"),
                quantity_conversion_target: None,
                source_map: Vec::new(),
                claims: Vec::new(),
                bounds: checked_bounds.clone(),
                checked_bounds,
                dependencies: Vec::new(),
                oracle_source: String::new(),
                boolean_right_may_stop: false,
            })),
        }
    }

    /// A claim with no checked bound, and one whose first checked bound has no `IntegerRange`
    /// among the derived domains, are the `NoRenderer` refusal: a claim map this generator did not
    /// produce, with no panic.
    #[trace("NFR-005-AC-2", "NFR-005-M-2", "TC-042")]
    #[test]
    fn tc_042_ac2_a_claim_without_a_derivable_first_bound_has_no_renderer() {
        let bound = checked_node_id("7");
        let not_symbolic = [DerivedDomain::NotSymbolic {
            bound: bound.clone(),
            form: "text_bounds".to_owned(),
        }];
        for (checked_bounds, derived) in [
            (Vec::new(), Vec::new()),
            (vec![bound.clone()], Vec::new()),
            (vec![bound.clone()], not_symbolic.to_vec()),
        ] {
            let claim = add_claim(checked_bounds);
            let ClaimDisposition::Generated(generated) = &claim.result else {
                panic!("built as Generated above");
            };
            assert!(
                matches!(
                    lower_scalar_claim(&claim, generated, &derived, &[]),
                    Err(ScalarLoweringRefusal::NoRenderer)
                ),
                "checked bounds {:?}",
                generated.checked_bounds
            );
        }
    }

    /// The operand ranges of each operation are of its own arity and combine to the exact
    /// extremes, for small ranges and for operands at `i64::MIN` and `i64::MAX`, where the
    /// results exceed `i64` and are exact in `i128`. This keeps the behaviour of the fixed-arity
    /// operands that replaced the `reachable` arm NFR-005 removed.
    #[trace("TC-042")]
    #[test]
    fn tc_042_scalar_operands_have_their_own_arity_and_reach_exact_extremes() {
        let reach = |operation: ScalarOperation, low: i64, high: i64, other: (i64, i64)| {
            operation
                .operands(|position| -> Result<(i64, i64), ()> {
                    Ok(if position == 0 { (low, high) } else { other })
                })
                .map(|operands| (operands.ranges().len(), operands.reachable()))
        };
        let small = |operation| reach(operation, -2, 3, (4, 5));
        assert_eq!(small(ScalarOperation::Negate), Ok((1, (-3, 2))));
        assert_eq!(small(ScalarOperation::Add), Ok((2, (2, 8))));
        assert_eq!(small(ScalarOperation::Subtract), Ok((2, (-7, -1))));
        assert_eq!(small(ScalarOperation::Multiply), Ok((2, (-10, 15))));

        // The least product is the `b * c` corner here: 1*-5 = -5, 1*-4 = -4, 2*-5 = -10,
        // 2*-4 = -8.
        assert_eq!(
            reach(ScalarOperation::Multiply, 1, 2, (-5, -4)),
            Ok((2, (-10, -4)))
        );

        // With every operand over the whole `i64` range, written by hand in powers of two:
        // `i64::MIN` is -2^63 and `i64::MAX` is 2^63 - 1.
        let (p63, p64, p126) = (1i128 << 63, 1i128 << 64, 1i128 << 126);
        let wide = |operation| reach(operation, i64::MIN, i64::MAX, (i64::MIN, i64::MAX));
        assert_eq!(wide(ScalarOperation::Negate), Ok((1, (1 - p63, p63))));
        assert_eq!(wide(ScalarOperation::Add), Ok((2, (-p64, p64 - 2))));
        assert_eq!(wide(ScalarOperation::Subtract), Ok((2, (1 - p64, p64 - 1))));
        assert_eq!(wide(ScalarOperation::Multiply), Ok((2, (p63 - p126, p126))));
    }
    #[trace("FR-015-AC-115", "TC-025")]
    #[test]
    fn scalar_port_is_differentially_identical_for_all_operations_and_boundaries() {
        use crate::core::canonical::{content_digest, preimage_value};
        use crate::kani::{
            generate::outcome::MAX_OBLIGATION_UNWIND, identity::InferredScalarOperand,
        };
        let mut digest_successes = 0;
        let mut digest_refusals = 0;
        for (operation, identity) in [
            (ScalarOperation::Add, "quire.op.integer.add"),
            (ScalarOperation::Subtract, "quire.op.integer.sub"),
            (ScalarOperation::Multiply, "quire.op.integer.mul"),
            (ScalarOperation::Negate, "quire.op.integer.negate"),
        ] {
            let count = operation.operand_names().len();
            let mut cases = vec![
                (vec![(-9, 9); count], false),
                (vec![(1, 1); count], false),
                (vec![(-4, 7); count], true),
            ];
            for position in 0..count {
                for extreme in [i64::MIN, i64::MAX] {
                    let mut operands = vec![(-9, 9); count];
                    operands[position] = (extreme, extreme);
                    cases.push((operands, false));
                }
            }
            for (operands, inferred) in cases {
                for unwind in [2, MAX_OBLIGATION_UNWIND] {
                    for seconds in [30, 600] {
                        let request = KaniObligationRequest {
                            ceilings: crate::kani::test_support::proof_ceilings::proof_ceilings_with_wall_clock(std::time::Duration::from_secs(seconds)),
                            items: &[], subject_path: "crate::unused", unwind,
                        };
                        let lowered = LoweredScalarClaim {
                            node_id: checked_node_id("2"),
                            operation_identity: identity.into(),
                            oracle_source: "fn oracle_test() {}".into(),
                            native_integer_selection: None,
                            oracle_symbol: "oracle_test".into(),
                            boolean_right_may_stop: false,
                            operation,
                            operands: operands.clone(),
                            inferred_operands: if inferred {
                                vec![InferredScalarOperand {
                                    position: 0,
                                    source_node_id: checked_node_id("3"),
                                    lower: "-4".into(),
                                    upper: "7".into(),
                                }]
                            } else {
                                vec![]
                            },
                            lower: i64::MIN,
                            upper: i64::MAX,
                            module_symbol: "scalar_control".into(),
                            harness_symbol: "check".into(),
                        };
                        let actual = render_scalar(&request, &lowered).unwrap();
                        let reference = render_scalar_reference(&request, &lowered).unwrap();
                        assert_eq!(actual.rust, reference.rust, "{identity}, {operands:?}, inferred={inferred}, unwind={unwind}, seconds={seconds}");
                        assert_eq!(actual.identity, reference.identity);
                        assert_eq!(actual.identity.options, reference.identity.options);
                        assert_eq!(actual.record, reference.record);
                        assert_eq!(actual, reference);
                        let digest = content_digest(&preimage_value(&actual.identity).unwrap())
                            .map_err(|error| format!("{error:?}"));
                        let extreme = operands
                            .iter()
                            .flat_map(|(low, high)| [*low, *high])
                            .find(|value| matches!(*value, i64::MIN | i64::MAX));
                        if let Some(extreme) = extreme {
                            let expected = quire_canonical::Error::IntegerMagnitudeAboveMaximum(
                                i128::from(extreme),
                            );
                            assert_eq!(digest, Err(format!("DigestError(Encode({expected:?}))")));
                            digest_refusals += 1;
                        } else {
                            assert!(
                                digest.is_ok(),
                                "ordinary-domain identity must have a digest: {digest:?}"
                            );
                            digest_successes += 1;
                        }
                        assert_eq!(
                            digest,
                            content_digest(&preimage_value(&reference.identity).unwrap())
                                .map_err(|error| format!("{error:?}"))
                        );
                        assert_eq!(
                            content_digest(
                                &preimage_value(&[&actual.rust, &actual.record]).unwrap()
                            )
                            .unwrap(),
                            content_digest(
                                &preimage_value(&[&reference.rust, &reference.record]).unwrap()
                            )
                            .unwrap()
                        );
                    }
                }
            }
        }
        assert!(
            digest_successes > 0,
            "ordinary identities have equal derived digests"
        );
        assert!(
            digest_refusals > 0,
            "i64 extremes retain RFC 8785 magnitude refusals"
        );
    }

    #[trace("FR-015-AC-115", "TC-025")]
    #[test]
    fn scalar_port_preserves_resource_and_syntax_refusals() {
        let request = KaniObligationRequest {
            ceilings: crate::kani::test_support::proof_ceilings::proof_ceilings(),
            items: &[],
            subject_path: "crate::unused",
            unwind: 2,
        };
        for oracle_source in [
            " ".repeat(MAX_GENERATED_SOURCE_BYTES),
            "?".repeat(MAX_GENERATED_SOURCE_BYTES),
            "?".into(),
        ] {
            let lowered = LoweredScalarClaim {
                node_id: checked_node_id("2"),
                operation_identity: "quire.op.integer.add".into(),
                oracle_source,
                native_integer_selection: None,
                oracle_symbol: "oracle_test".into(),
                boolean_right_may_stop: false,
                operation: ScalarOperation::Add,
                operands: vec![(-1, 1); 2],
                inferred_operands: vec![],
                lower: -1,
                upper: 1,
                module_symbol: "scalar_control".into(),
                harness_symbol: "check".into(),
            };
            assert_eq!(
                render_scalar(&request, &lowered).unwrap_err(),
                render_scalar_reference(&request, &lowered).unwrap_err()
            );
        }
    }
}
