// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Scalar descriptor derivation controls.

use super::super::lower::tests::{app_term, node_id, v2_node};
use super::super::lower::{catalogued_operation, operation_confirmed};
use super::super::*;
use super::*;
use crate::oracle::checked_graph::fixture::Node as FixtureNode;
use ix_trace_rs::trace;
use quire_contract_model::CompleteContractNodeV2;
use quire_exact::{TextProfile, TextType};
use std::collections::BTreeMap;

type Graph<'a> = crate::oracle::checked_graph::CheckedGraph<'a, CheckedTerm>;

// -- Derivation (IR-294) ---------------------------------------------------------------

/// One descriptor per catalogued identity the forward map can name.
fn every_operation() -> Vec<ExactScalarOperation> {
    use ExactScalarOperation as Op;
    let domain = IntegerDomain::Mathematical;
    let decimal = DecimalType::new(0_i64.into(), 9_i64.into(), 0, 0, RoundingMode::NearestEven)
        .expect("type");
    let text = TextType::new(0, 4, TextProfile::Nfc).expect("text");
    let mut all = Vec::new();
    for operator in [
        BooleanOperator::And,
        BooleanOperator::Or,
        BooleanOperator::Not,
        BooleanOperator::Implies,
    ] {
        all.push(Op::Boolean {
            operator,
            right_may_stop: false,
        });
    }
    for operands in [EqualityOperandKind::Boolean, EqualityOperandKind::Integer] {
        for operator in [EqualityOperator::Equal, EqualityOperator::NotEqual] {
            all.push(Op::Equality { operator, operands });
        }
    }
    for operator in [
        IntegerOperator::Add,
        IntegerOperator::Subtract,
        IntegerOperator::Multiply,
        IntegerOperator::Negate,
    ] {
        all.push(Op::IntegerArithmetic {
            operator,
            domain: domain.clone(),
        });
    }
    for profile in DivisionProfile::ALL {
        all.push(Op::IntegerDivision {
            profile,
            domain: domain.clone(),
        });
    }
    all.push(Op::IntegerModulo { domain });
    for operator in [
        RationalOperator::Add,
        RationalOperator::Subtract,
        RationalOperator::Multiply,
        RationalOperator::Negate,
    ] {
        all.push(Op::RationalArithmetic {
            operator,
            domain: None,
        });
    }
    for operator in [RationalOperator::Divide, RationalOperator::IntegerDivide] {
        all.push(Op::RationalArithmetic {
            operator,
            domain: None,
        });
    }
    for operator in [
        OrderingOperator::Less,
        OrderingOperator::LessOrEqual,
        OrderingOperator::Greater,
        OrderingOperator::GreaterOrEqual,
    ] {
        for operands in [
            OrderingOperandKind::Integer,
            OrderingOperandKind::Rational,
            OrderingOperandKind::Decimal,
        ] {
            all.push(Op::Ordering { operator, operands });
        }
    }
    for operator in [
        DecimalOperator::Add,
        DecimalOperator::Subtract,
        DecimalOperator::Multiply,
        DecimalOperator::Negate,
        DecimalOperator::Divide,
        DecimalOperator::Round,
    ] {
        all.push(Op::DecimalArithmetic {
            operator,
            target: decimal.clone(),
        });
    }
    for width in [IeeeWidth::Binary32, IeeeWidth::Binary64] {
        for operator in [
            IeeeArithmeticOperator::Add,
            IeeeArithmeticOperator::Subtract,
            IeeeArithmeticOperator::Multiply,
            IeeeArithmeticOperator::Divide,
        ] {
            all.push(Op::IeeeArithmetic {
                operator,
                width,
                rounding: RoundingMode::NearestEven,
            });
        }
        for comparison in [
            IeeeComparison::NumericEqual,
            IeeeComparison::TotalOrder,
            IeeeComparison::BitIdentical,
        ] {
            all.push(Op::IeeeComparison { comparison, width });
        }
    }
    all.push(Op::IeeeWidthConversion {
        source: IeeeWidth::Binary64,
        target: IeeeWidth::Binary32,
        rounding: RoundingMode::NearestEven,
    });
    all.push(Op::IeeeWidthConversion {
        source: IeeeWidth::Binary32,
        target: IeeeWidth::Binary64,
        rounding: RoundingMode::NearestEven,
    });
    all.push(Op::TextAdmission { text_type: text });
    for operator in ComparisonOperator::ALL {
        all.push(Op::TextComparison { operator });
        all.push(Op::EnumComparison { operator });
        all.push(Op::QuantityComparison { operator });
    }
    all.push(Op::QuantityConversion);
    for operator in [
        QuantityOperator::Add,
        QuantityOperator::Subtract,
        QuantityOperator::Multiply,
        QuantityOperator::Divide,
        QuantityOperator::Power,
    ] {
        all.push(Op::QuantityArithmetic { operator });
    }
    all
}

/// The number of `ExactScalarOperation` variants: [`expected_family`]'s match names each, and
/// adding one fails to compile there until it has an arm. `every_operation` must then carry a
/// descriptor of it, which the coverage assertion below checks.
const OPERATION_VARIANTS: usize = 17;

/// The family each descriptor belongs to, by an exhaustive match with no wildcard arm over
/// `ExactScalarOperation` and, where the operator selects the identity, over the operator.
/// `None` for the one descriptor with no catalogued identity (same-width IEEE conversion).
fn expected_family(operation: &ExactScalarOperation) -> Option<Family> {
    use ExactScalarOperation as Op;
    Some(match operation {
        Op::Boolean { operator, .. } => Family::Boolean(*operator),
        Op::Equality { operator, operands } => Family::Equality(*operator, *operands),
        Op::IntegerArithmetic { operator, .. } => Family::IntegerArithmetic(*operator),
        Op::IntegerDivision { .. } => Family::IntegerDivision,
        Op::IntegerModulo { .. } => Family::IntegerModulo,
        Op::RationalArithmetic { operator, .. } => match operator {
            RationalOperator::Divide | RationalOperator::IntegerDivide => Family::RationalDivision,
            RationalOperator::Add
            | RationalOperator::Subtract
            | RationalOperator::Multiply
            | RationalOperator::Negate => Family::RationalArithmetic(*operator),
        },
        Op::Ordering { operator, operands } => Family::Ordering(*operator, *operands),
        Op::DecimalArithmetic { operator, .. } => Family::DecimalArithmetic(*operator),
        Op::IeeeArithmetic {
            operator, width, ..
        } => Family::IeeeArithmetic(*operator, *width),
        Op::IeeeComparison { comparison, .. } => Family::IeeeComparison(*comparison),
        Op::IeeeWidthConversion { source, target, .. } => match (source, target) {
            (IeeeWidth::Binary64, IeeeWidth::Binary32) => Family::IeeeToFloat32,
            (IeeeWidth::Binary32, IeeeWidth::Binary64) => Family::IeeeToFloat64,
            _ => return None,
        },
        Op::TextAdmission { .. } => Family::NumericConvert,
        Op::TextComparison { operator } => Family::TextComparison(*operator),
        Op::EnumComparison { operator } => Family::EnumComparison(*operator),
        Op::QuantityConversion => Family::QuantityConversion,
        Op::QuantityArithmetic { operator } => Family::QuantityArithmetic(*operator),
        Op::QuantityComparison { operator } => Family::QuantityComparison(*operator),
    })
}

/// The inverse identity match and the forward `catalogued_operation` are two exhaustive
/// matches over one vocabulary, kept aligned by this round trip: every descriptor's forward
/// identity inverts to the family that descriptor belongs to, no two families share an
/// identity, and `every_operation` covers every `ExactScalarOperation` variant. A catalogued
/// identity added to one map and not the other fails here.
#[trace("FR-014-AC-18", "FR-014-AC-19", "TC-024")]
#[test]
fn tc_024_the_identity_inverse_matches_the_forward_catalogue_for_every_descriptor() {
    let operations = every_operation();
    let covered = operations
        .iter()
        .map(std::mem::discriminant)
        .collect::<std::collections::HashSet<_>>();
    assert_eq!(
        covered.len(),
        OPERATION_VARIANTS,
        "every variant has a descriptor"
    );
    let mut seen = BTreeMap::new();
    for operation in &operations {
        let family = expected_family(operation).expect("every descriptor here is catalogued");
        let identity = catalogued_operation(operation)
            .expect("every descriptor here is a known RT variant")
            .expect("every descriptor here has a catalogued identity")
            .identity;
        assert_eq!(
            family_of_identity(identity),
            Some(family),
            "{identity} inverts to the family of {operation:?}"
        );
        if let Some(previous) = seen.insert(identity, family) {
            assert_eq!(previous, family, "{identity} names two families");
        }
    }
}

fn zero_literal() -> CheckedTerm {
    CheckedTerm::Literal {
        type_id: node_id('1'),
        value_kind: LiteralKind::Integer,
        value: serde_json::json!("0"),
    }
}

fn application_node(body: impl Into<Option<CheckedTerm>>) -> (CheckedNodeId, FixtureNode) {
    let node = v2_node('a', "expression", "binary", '1', body);
    (node.node_id.clone(), node)
}

/// The lowered form of a hand-built node, as IR lowering would return it.
fn lowered_node(node: &FixtureNode) -> CompleteContractNodeV2 {
    CompleteContractNodeV2 {
        node: node.metadata.clone(),
        node_tag: CheckedNodeTag::Expression,
        source_map: vec![],
        semantic_type: node.semantic_type.clone(),
        dependencies: vec![],
        bounds: vec![],
        claims: vec![],
        ir_id: serde_json::from_value(serde_json::json!({
            "domain": "quire.contract-ir.semantic/v1",
            "algorithm": "sha-256",
            "digest": "f".repeat(64),
        }))
        .expect("an ir id"),
    }
}

fn derive_hand_built(
    body: impl Into<Option<CheckedTerm>>,
) -> Result<ExactScalarOperation, ExactScalarRefusal> {
    let (id, node) = application_node(body);
    let graph: Graph<'_> = [(&id, &node)].into_iter().collect();
    let record = CompleteLoweringRecordV2::Lowered {
        node: Box::new(lowered_node(&node)),
    };
    derive_operation(&graph, &id, &record)
}

fn no_claim(reason: ClaimDerivationRefusal) -> ExactScalarRefusal {
    ExactScalarRefusal::NoDerivableClaim { reason }
}

/// A node absent from the graph, and one whose record did not lower, keep the refusal
/// `generate_exact_scalar_oracles` gives them, before any identity is read.
#[trace("FR-014-AC-18", "FR-022-AC-8", "TC-024")]
#[test]
fn tc_024_derivation_checks_the_graph_and_the_lowering_before_the_identity() {
    let (id, node) = application_node(zero_literal());
    let graph: Graph<'_> = [(&id, &node)].into_iter().collect();
    let record = CompleteLoweringRecordV2::InvalidInput {
        node_id: id.clone(),
    };
    assert_eq!(
        derive_operation(&graph, &node_id('9'), &record),
        Err(ExactScalarRefusal::InvalidInput)
    );
    assert_eq!(
        derive_operation(&graph, &id, &record),
        Err(ExactScalarRefusal::InvalidInput),
        "an unlowered record is refused before the body is read"
    );
}

#[trace("FR-014-AC-48", "TC-024")]
#[test]
fn quantity_conversion_confirmation_requires_identity_and_exact_mode() {
    for identity in [QUANTITY_CONVERT_IDENTITY, "quire.op.quantity.add"] {
        for mode in ["exact", "nearest-even"] {
            let (_, node) = application_node(app_term(serde_json::json!({
                "identity": identity, "mode": {"kind":"rounding", "value": mode}
            })));
            let graph: Graph<'_> = [(&node.node_id, &node)].into_iter().collect();
            assert_eq!(
                operation_confirmed(
                    &graph,
                    &lowered_node(&node),
                    &ExactScalarOperation::QuantityConversion
                )
                .unwrap(),
                identity == QUANTITY_CONVERT_IDENTITY && mode == "exact"
            );
        }
    }
}

/// Scalar targets cannot enter an admitted package; mode validation runs first at this seam.
#[trace("FR-014-AC-50", "TC-024")]
#[test]
fn quantity_conversion_rejects_scalar_targets_after_mode() {
    for form in ["decimal", "integer", "compound_unit"] {
        let target = v2_node('b', "scalar_type", form, 'b', None);
        for mode in ["exact", "nearest-even", "toward-zero"] {
            let (id, node) = application_node(app_term(serde_json::json!({
                "identity": QUANTITY_CONVERT_IDENTITY,
                "mode": {"kind": "rounding", "value": mode},
                "member": {"kind": "type_argument", "declaration": target.node_id},
            })));
            let graph: Graph<'_> = [(&id, &node), (&target.node_id, &target)]
                .into_iter()
                .collect();
            let record = CompleteLoweringRecordV2::Lowered {
                node: Box::new(lowered_node(&node)),
            };
            let reason = if mode == "exact" {
                ClaimDerivationRefusal::OperandFormsNotDerivable {
                    operation_identity: QUANTITY_CONVERT_IDENTITY.to_owned(),
                }
            } else {
                ClaimDerivationRefusal::ModeNotDerivable {
                    operation_identity: QUANTITY_CONVERT_IDENTITY.to_owned(),
                }
            };
            assert_eq!(
                derive_operation(&graph, &id, &record),
                Err(no_claim(reason)),
                "{form}: {mode}"
            );
        }
    }
}

/// A node with no `operation.identity` (which admission forbids), a node that is not an
/// application, and one naming an identity outside the derivable set each get their own
/// typed refusal.
#[trace("FR-014-AC-18", "TC-024")]
#[test]
fn tc_024_derivation_refuses_a_missing_identity_a_non_application_and_a_foreign_identity() {
    assert_eq!(
        derive_hand_built(app_term(serde_json::json!({}))),
        Err(no_claim(ClaimDerivationRefusal::MissingOperationIdentity))
    );
    assert_eq!(
        derive_hand_built(zero_literal()),
        Err(no_claim(ClaimDerivationRefusal::NotApplication {
            node_tag: "expression".to_owned()
        }))
    );
    assert_eq!(
        derive_hand_built(app_term(
            serde_json::json!({"identity": "quire.op.rational.eq"})
        )),
        Err(no_claim(ClaimDerivationRefusal::OperationNotDerivable {
            operation_identity: "quire.op.rational.eq".to_owned()
        }))
    );
}

/// `integer.div` names its profile only through `operation.laws`: none, or two different
/// profiles, select no profile; a foreign definition selects none either.
#[trace("FR-014-AC-18", "TC-024")]
#[test]
fn tc_024_derivation_reads_the_division_profile_from_the_laws_and_nowhere_else() {
    let laws = |identities: &[&str]| {
        app_term(serde_json::json!({
            "identity": "quire.op.integer.div",
            "laws": identities
                .iter()
                .map(|identity| serde_json::json!({
                    "role": "integer_division", "definition": {"identity": identity}
                }))
                .collect::<Vec<_>>(),
        }))
    };
    let profile_of = |body: CheckedTerm| {
        let (id, node) = application_node(body);
        let graph: Graph<'_> = [(&id, &node)].into_iter().collect();
        let lowered = lowered_node(&node);
        Derivation {
            graph: &graph,
            node: &node,
            bounds: BoundView::complete(&lowered),
            identity: "quire.op.integer.div",
        }
        .division_profile()
    };
    let law_refusal = Err(no_claim(ClaimDerivationRefusal::LawNotDerivable {
        operation_identity: "quire.op.integer.div".to_owned(),
    }));
    assert_eq!(profile_of(laws(&[])), law_refusal);
    assert_eq!(profile_of(laws(&["quire.value.other/v1"])), law_refusal);
    assert_eq!(
        profile_of(laws(&[
            DivisionProfile::Floor.definition_identity(),
            DivisionProfile::Euclidean.definition_identity(),
        ])),
        law_refusal
    );
    for profile in DivisionProfile::ALL {
        assert_eq!(
            profile_of(laws(&[profile.definition_identity()])),
            Ok(profile)
        );
    }
}

/// `operation.mode` must carry the rounding of the bound it is read against: absent, of
/// another kind or another value is `ModeNotDerivable`.
#[trace("FR-014-AC-18", "TC-024")]
#[test]
fn tc_024_derivation_requires_the_mode_to_agree_with_the_bound_rounding() {
    let derivation_of = |mode: Value| {
        let (id, node) = application_node(app_term(
            serde_json::json!({"identity": "quire.op.decimal.add", "mode": mode}),
        ));
        let graph: Graph<'_> = [(&id, &node)].into_iter().collect();
        let lowered = lowered_node(&node);
        Derivation {
            graph: &graph,
            node: &node,
            bounds: BoundView::complete(&lowered),
            identity: "quire.op.decimal.add",
        }
        .require_mode(RoundingMode::NearestEven)
    };
    let refusal = Err(no_claim(ClaimDerivationRefusal::ModeNotDerivable {
        operation_identity: "quire.op.decimal.add".to_owned(),
    }));
    assert_eq!(derivation_of(Value::Null), refusal);
    assert_eq!(
        derivation_of(serde_json::json!({"kind": "text_profile", "value": "nearest-even"})),
        refusal
    );
    assert_eq!(
        derivation_of(serde_json::json!({"kind": "rounding", "value": "toward-zero"})),
        refusal
    );
    assert_eq!(
        derivation_of(serde_json::json!({"kind": "rounding", "value": "nearest-even"})),
        Ok(())
    );
}

/// Catalogued identities derivation refuses (checked against the operation catalog): every
/// one is outside `family_of_identity`, so it is `OperationNotDerivable`.
#[trace("FR-014-AC-18", "TC-024")]
#[test]
fn tc_024_the_refused_catalogue_identities_have_no_family() {
    for identity in [
        "quire.op.rational.eq",
        "quire.op.rational.ne",
        "quire.op.decimal.eq",
        "quire.op.decimal.ne",
        "quire.op.integer.rem",
        "quire.op.ieee.float32.sqrt",
        "quire.op.ieee.float64.sqrt",
        "quire.op.ieee.float32.fma",
        "quire.op.ieee.float64.fma",
        "quire.op.ieee.from_exact",
        "quire.op.ieee.to_rational",
        "quire.op.numeric.narrow",
        "quire.op.rational.narrow",
        "quire.op.text.size",
    ] {
        assert_eq!(family_of_identity(identity), None, "{identity}");
    }
}
