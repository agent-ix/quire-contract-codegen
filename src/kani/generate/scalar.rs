//! The scalar family: lowering an IR-confirmed V2 exact-scalar claim and rendering its Kani
//! harness (FR-015).

use quire_contract_model::{CheckedNodeId, CheckedSemanticNodeV2};

use crate::{
    core::artifact::MAX_GENERATED_SOURCE_BYTES,
    kani::abi::{adapter_options, i64_literal, readable_component, KaniSolver},
    kani::generate::outcome::{DerivedDomain, KaniObligationRequest, UnsupportedObligation},
    kani::generate::record::{artifact, harness_path, record},
    kani::identity::{
        KaniScalarObligationHarness, ScalarObligationArgument, ScalarObligationIdentity,
    },
    oracle::scalar::{
        aggregate_members, bound_members, literal_count, literal_integer, ExactScalarClaim,
        GeneratedScalarClaim, OperandRange, COLLECTION_BOUNDS_MEMBERS, INTEGER_RANGE_MEMBERS,
        TEXT_BOUNDS_MEMBERS,
    },
};

/// An IR-confirmed V2 exact-scalar claim this generator knows how to render a Kani harness for.
/// Scoped today to `IntegerArithmetic` (`quire.op.integer.{add,sub,mul,negate}`): its operands and
/// result are all `rt::Integer`, and `quire_contract_runtime::exact::Integer: From<i64>` gives a
/// direct `kani::any()` bridge with no construction of a compound runtime type. Every other
/// confirmed family (division's `QuotientRemainder`, rational, decimal, IEEE, text, enum, quantity)
/// has no renderer yet -- see [`lower_scalar_claim`].
pub(super) struct LoweredScalarClaim {
    pub(super) node_id: CheckedNodeId,
    pub(super) operation_identity: String,
    /// The oracle's own self-contained source (`GeneratedScalarClaim::oracle_source`), embedded
    /// verbatim ahead of the harness module, exactly as a V1 harness embeds its `ClauseOracle`s.
    oracle_source: String,
    oracle_symbol: String,
    operation: ScalarOperation,
    /// Each operand's inclusive range, in call order.
    operands: Vec<(i64, i64)>,
    /// The result's inclusive range: the checked domain a completed result must lie in.
    lower: i64,
    upper: i64,
    pub(super) module_symbol: String,
    pub(super) harness_symbol: String,
}

/// One integer operation this generator renders a scalar harness for. Everything that differs
/// between the four -- the catalogued identity, the operand names, the exact result, the range of
/// reachable results -- is an exhaustive method here, so adding an operation is one new variant the
/// compiler checks at every use.
#[derive(Clone, Copy)]
enum ScalarOperation {
    Add,
    Subtract,
    Multiply,
    Negate,
}

impl ScalarOperation {
    /// The renderable operation for an IR-confirmed operation identity, or `None` when this
    /// generator has no scalar-harness renderer for it. Recognizing these four exact catalogued
    /// identities is not a vocabulary bridge: it only recognizes, among identities IR already
    /// confirmed, which ones this generator additionally knows how to turn into a harness.
    fn of(operation_identity: &str) -> Option<Self> {
        match operation_identity {
            "quire.op.integer.add" => Some(Self::Add),
            "quire.op.integer.sub" => Some(Self::Subtract),
            "quire.op.integer.mul" => Some(Self::Multiply),
            "quire.op.integer.negate" => Some(Self::Negate),
            _ => None,
        }
    }

    /// The oracle's parameter names, in call order, which are also the harness's variable names.
    fn operand_names(self) -> &'static [&'static str] {
        match self {
            Self::Negate => &["operand"],
            Self::Add | Self::Subtract | Self::Multiply => &["left", "right"],
        }
    }

    /// The exact result as a Rust expression over the harness's native `i64` operands
    /// (`<name>_native`), evaluated in `i128` where a sum, difference, product or negation of `i64`
    /// values cannot overflow. It shares no code with the embedded oracle, which evaluates through
    /// `quire_contract_runtime`'s `rt::Integer`; it is the independent statement of the clause that
    /// [`render_scalar`] compares the oracle's result with.
    fn native_expression(self) -> String {
        let native = |index: usize| format!("i128::from({}_native)", self.operand_names()[index]);
        match self {
            Self::Add => format!("{} + {}", native(0), native(1)),
            Self::Subtract => format!("{} - {}", native(0), native(1)),
            Self::Multiply => format!("{} * {}", native(0), native(1)),
            Self::Negate => format!("-{}", native(0)),
        }
    }

    /// The operation's operands, one inclusive `i64` range per operand name, in call order, each
    /// read by `range_at` from its position. The operand count is the operation's own, so the
    /// result carries exactly the operands [`ScalarOperands::reachable`] needs.
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
        })
    }
}

/// The two-operand operations of [`ScalarOperation`].
#[derive(Clone, Copy)]
enum BinaryOperation {
    Add,
    Subtract,
    Multiply,
}

/// The operand ranges of one [`ScalarOperation`], of the operation's own arity: one range for
/// `Negate`, two for the others, so no combination of operation and operand count is left over.
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

    /// The least and greatest exact results over the inclusive `i64` operand ranges. Each is exact
    /// in `i128`. The extremes of `+`, `-` and unary `-` lie at the range endpoints, and so do
    /// those of `*`, which is bilinear.
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

/// Lowers one IR-confirmed V2 claim to a renderable scalar harness. The result range is the
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
        oracle_symbol: generated.symbol.clone(),
        operation,
        operands: operands.ranges(),
        lower,
        upper,
        module_symbol,
        harness_symbol,
    })
}

pub(super) fn derive_domain(bound: &CheckedSemanticNodeV2) -> DerivedDomain {
    let members = aggregate_members(&bound.body);
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

fn not_symbolic(bound: &CheckedSemanticNodeV2) -> DerivedDomain {
    DerivedDomain::NotSymbolic {
        bound: bound.node_id.clone(),
        form: bound.semantic_form.to_string(),
    }
}

/// The canonical inclusive limits of a range bound that admits no value.
pub(super) fn unsatisfiable(bound: &CheckedSemanticNodeV2) -> Option<(String, String)> {
    let members = aggregate_members(&bound.body)?;
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
fn count_bounds(
    minimum: &serde_json::Value,
    maximum: &serde_json::Value,
) -> Option<(String, String)> {
    let (minimum, maximum) = (literal_count(minimum)?, literal_count(maximum)?);
    (minimum > maximum).then(|| (minimum.to_string(), maximum.to_string()))
}

/// Renders one IR-confirmed V2 exact-scalar claim to a `kani::proof` that the embedded oracle
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
        node_id: lowered.node_id.clone(),
        operation_identity: lowered.operation_identity.clone(),
        oracle_symbol: lowered.oracle_symbol.clone(),
        module_symbol: path.module,
        harness_symbol: path.harness,
        arguments,
        solver: KaniSolver::Cadical,
        unwind: request.unwind,
        options,
    };
    let mut source = format!(
        "// SPDX-License-Identifier: MIT OR Apache-2.0\n\
// Obligation: exact-scalar `{}`\n\n",
        lowered.operation_identity,
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
    let rust = artifact(
        format!("src/generated/{}.rs", lowered.module_symbol),
        source,
    );
    let record = record(&lowered.module_symbol, &identity, &rust)?;
    Ok(KaniScalarObligationHarness {
        identity,
        rust,
        record,
    })
}

#[cfg(test)]
mod tests {
    use serde_json::{json, Value};

    use super::*;
    use crate::oracle::{
        claim::ClaimDisposition,
        scalar::{OperationClaim, OperationProvenance},
    };

    /// The symbolic domain is read from binding-shaped `min`/`max` members (the shape QSL emits,
    /// FR-322); a bare-literal member is not symbolic.
    ///
    /// Trace: FR-015-AC-5, TC-025.
    #[test]
    fn tc_025_derive_domain_reads_binding_shaped_range_members() {
        let node = |members: Value| -> CheckedSemanticNodeV2 {
            serde_json::from_value(json!({
                "node_id": {"domain": "quire.checked-semantic-node/v1", "digest": "0".repeat(64)},
                "schema_version": "quire.checked-semantic-graph/v2",
                "node_tag": "bounded_domain",
                "semantic_form": "integer_range",
                "semantic_type": {"domain": "quire.checked-semantic-node/v1", "digest": "1".repeat(64)},
                "dependencies": [],
                "occurrences": [],
                "body": {"term": "aggregate", "members": members},
            }))
            .expect("node")
        };
        let literal = |value: &str| {
            json!({"term": "literal",
                "type": {"domain": "quire.checked-semantic-node/v1", "digest": "1".repeat(64)},
                "value_kind": "integer", "value": value})
        };
        let bound = node(json!([
            {"term": "binding", "name": "min", "value": literal("0")},
            {"term": "binding", "name": "max", "value": literal("9")},
        ]));
        assert_eq!(
            derive_domain(&bound),
            DerivedDomain::IntegerRange {
                bound: bound.node_id.clone(),
                lower: "0".to_owned(),
                upper: "9".to_owned(),
            }
        );
        let bare = node(json!([literal("0"), literal("9")]));
        assert!(matches!(
            derive_domain(&bare),
            DerivedDomain::NotSymbolic { .. }
        ));
    }

    /// Bound members are looked up by name in any order: swapped `min`/`max` members still read
    /// the right range, while a duplicated or unknown name is not a range.
    ///
    /// Trace: FR-015-AC-5, TC-025.
    #[test]
    fn tc_025_range_members_are_read_by_name_not_position() {
        let node = |members: Value| -> CheckedSemanticNodeV2 {
            serde_json::from_value(json!({
                "node_id": {"domain": "quire.checked-semantic-node/v1", "digest": "0".repeat(64)},
                "schema_version": "quire.checked-semantic-graph/v2",
                "node_tag": "bounded_domain",
                "semantic_form": "integer_range",
                "semantic_type": {"domain": "quire.checked-semantic-node/v1", "digest": "1".repeat(64)},
                "dependencies": [],
                "occurrences": [],
                "body": {"term": "aggregate", "members": members},
            }))
            .expect("node")
        };
        let member = |name: &str, value: &str| {
            json!({"term": "binding", "name": name, "value":
                {"term": "literal", "value_kind": "integer", "value": value}})
        };
        let swapped = node(json!([member("max", "9"), member("min", "0")]));
        assert_eq!(
            derive_domain(&swapped),
            DerivedDomain::IntegerRange {
                bound: swapped.node_id.clone(),
                lower: "0".to_owned(),
                upper: "9".to_owned(),
            }
        );
        // `[max=0, min=9]` written high-first is an inverted range, not the range `[0, 9]`.
        assert_eq!(
            unsatisfiable(&node(json!([member("max", "0"), member("min", "9")]))),
            Some(("9".to_owned(), "0".to_owned()))
        );
        for members in [
            json!([member("min", "0"), member("min", "9")]),
            json!([member("min", "0"), member("high", "9")]),
            json!([member("min", "0")]),
        ] {
            assert!(matches!(
                derive_domain(&node(members.clone())),
                DerivedDomain::NotSymbolic { .. }
            ));
            assert_eq!(unsatisfiable(&node(members)), None);
        }
    }

    /// Trace: FR-015-AC-5, TC-025.
    #[test]
    fn tc_025_inverted_ranges_are_unsatisfiable_and_ordered_ranges_are_not() {
        let node = |form: &str, members: Value| -> CheckedSemanticNodeV2 {
            serde_json::from_value(serde_json::json!({
                "node_id": {"domain": "quire.checked-semantic-node/v1", "digest": "0".repeat(64)},
                "schema_version": "quire.checked-semantic-graph/v2",
                "node_tag": "bounded_domain",
                "semantic_form": form,
                "semantic_type": {"domain": "quire.checked-semantic-node/v1", "digest": "1".repeat(64)},
                "dependencies": [],
                "occurrences": [],
                "body": {"term": "aggregate", "members": members},
            }))
            .expect("node")
        };
        // The QSL-emitted shape: each bound member is a `binding` carrying its literal.
        let member = |name: &str, kind: &str, value: &str| {
            serde_json::json!({"term":"binding","name":name,"value":
                {"term":"literal","value_kind":kind,"value":value}})
        };
        let (lo, hi) = (
            |value: &str| member("min", "integer", value),
            |value: &str| member("max", "integer", value),
        );
        let text = |value: &str| member("text_profile", "text", value);
        assert_eq!(
            unsatisfiable(&node(
                "integer_range",
                serde_json::json!([lo("5"), hi("-5")])
            )),
            Some(("5".to_owned(), "-5".to_owned()))
        );
        assert_eq!(
            unsatisfiable(&node(
                "integer_range",
                serde_json::json!([lo("-5"), hi("5")])
            )),
            None
        );
        assert_eq!(
            unsatisfiable(&node(
                "integer_range",
                serde_json::json!([lo("5"), hi("5")])
            )),
            None
        );
        assert_eq!(
            unsatisfiable(&node(
                "text_bounds",
                serde_json::json!([lo("4"), hi("0"), text("nfc")])
            )),
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
    fn tc_026_a_domain_outside_i64_is_a_typed_refusal_not_a_panic() {
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
                ir_id: serde_json::from_value(serde_json::json!({
                    "domain": "quire.checked-semantic-node/v1",
                    "algorithm": "sha256",
                    "digest": "3".repeat(64),
                }))
                .expect("ir id"),
                semantic_form: "expression".to_owned(),
                semantic_type: node_id("4"),
                source_map: Vec::new(),
                claims: Vec::new(),
                bounds: vec![bound.clone()],
                checked_bounds: vec![bound.clone()],
                dependencies: Vec::new(),
                oracle_source: String::new(),
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
                ir_id: serde_json::from_value(json!({
                    "domain": "quire.checked-semantic-node/v1",
                    "algorithm": "sha256",
                    "digest": "3".repeat(64),
                }))
                .expect("ir id"),
                semantic_form: "expression".to_owned(),
                semantic_type: checked_node_id("4"),
                source_map: Vec::new(),
                claims: Vec::new(),
                bounds: checked_bounds.clone(),
                checked_bounds,
                dependencies: Vec::new(),
                oracle_source: String::new(),
            })),
        }
    }

    /// A claim with no checked bound, and one whose first checked bound has no `IntegerRange`
    /// among the derived domains, are the `NoRenderer` refusal: a claim map this generator did not
    /// produce, with no panic.
    ///
    /// Trace: NFR-005-AC-2, TC-042.
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
    /// extremes.
    ///
    /// Trace: NFR-005-AC-1, TC-042.
    #[test]
    fn tc_042_ac1_operands_of_each_operation_have_its_own_arity_and_reach_exact_extremes() {
        let range = |position: usize| -> Result<(i64, i64), ()> {
            Ok(if position == 0 { (-2, 3) } else { (4, 5) })
        };
        let reach = |operation: ScalarOperation| {
            operation
                .operands(range)
                .map(|operands| (operands.ranges().len(), operands.reachable()))
        };
        assert_eq!(reach(ScalarOperation::Negate), Ok((1, (-3, 2))));
        assert_eq!(reach(ScalarOperation::Add), Ok((2, (2, 8))));
        assert_eq!(reach(ScalarOperation::Subtract), Ok((2, (-7, -1))));
        assert_eq!(reach(ScalarOperation::Multiply), Ok((2, (-10, 15))));
    }
}
