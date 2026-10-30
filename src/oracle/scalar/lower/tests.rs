// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Scalar lowering and derivation controls.

use super::super::*;
use super::*;
pub(in crate::oracle::scalar) fn app_term(operation: Value) -> CheckedTerm {
    CheckedTerm::Application {
        operator: ApplicationOperator::Binary,
        operation: serde_json::from_value(operation).expect("fixture operation"),
        result_type: node_id('1'),
        arguments: Vec::new(),
    }
}

type Graph<'a> = crate::oracle::checked_graph::CheckedGraph<'a, CheckedTerm>;
use crate::oracle::checked_graph::fixture::Node as FixtureNode;
use ix_trace_rs::trace;
use quire_contract_model::{CheckedNodePathSegment, CheckedPackageIncomplete, CheckedPackageLimit};

#[trace("FR-014-AC-3")]
#[test]
fn hybrid_family_is_unsupported_and_preserves_node_identity() {
    let node = node_id('b');
    assert_eq!(
        unsupported_family(&node, CheckedNodeTag::Hybrid),
        ExactScalarRefusal::Unsupported {
            unsupported_node_id: node,
            node_tag: "hybrid",
        }
    );
}

pub(in crate::oracle::scalar) fn node_id(digit: char) -> CheckedNodeId {
    CheckedNodeId {
        package: "fixture".into(),
        path: vec![CheckedNodePathSegment::Name(digit.to_string().into())],
        ordinal: 0,
    }
}

fn numbered_node_id(value: u64) -> CheckedNodeId {
    CheckedNodeId {
        package: "fixture".into(),
        path: vec![CheckedNodePathSegment::Name(value.to_string().into())],
        ordinal: 0,
    }
}

fn inference_node(
    id: CheckedNodeId,
    tag: &str,
    form: &str,
    semantic_type: CheckedNodeId,
    body: impl Into<Option<CheckedTerm>>,
) -> FixtureNode {
    FixtureNode::new(
        CheckedSemanticNodeV2 {
            node_id: id,
            owner: None,
            owner_model: None,
            schema_version: "quire.checked-semantic-graph/v2".into(),
            node_tag: tag.into(),
            semantic_form: form.into(),
            semantic_type,
            dependencies: Vec::new(),
            occurrences: Vec::new(),
            recursion_group: None,
            declaration: None,
            body: CheckedTerm::aggregate(Vec::new()),
        },
        body.into(),
    )
}

fn inference_application_node(
    id: CheckedNodeId,
    integer_type: &CheckedNodeId,
    operation: &str,
    arguments: Vec<CheckedTerm>,
) -> FixtureNode {
    inference_node(
        id,
        "expression",
        "binary",
        integer_type.clone(),
        CheckedTerm::Application {
            operator: ApplicationOperator::from_wire(if operation.ends_with("negate") {
                "unary"
            } else {
                "binary"
            })
            .expect("closed fixture kind"),
            operation: serde_json::from_value(serde_json::json!({"identity": operation}))
                .expect("fixture operation"),
            result_type: node_id('0'),
            arguments,
        },
    )
}

fn reference(id: &CheckedNodeId) -> CheckedTerm {
    CheckedTerm::Reference { target: id.clone() }
}

fn inference_graph(nodes: &[FixtureNode]) -> Graph<'_> {
    nodes.iter().map(|node| (&node.node_id, node)).collect()
}

/// Admission normally prevents malformed cycles and references; the inference boundary still
/// reports each cause distinctly if a damaged graph reaches it.
#[trace("FR-014-AC-46", "TC-024")]
#[test]
fn tc_024_nested_inference_reports_missing_cycle_unsupported_and_bad_literals() {
    let integer = node_id('0');
    let scalar = inference_node(
        integer.clone(),
        "scalar_type",
        "integer",
        integer.clone(),
        None,
    );
    let missing = node_id('b');
    let root_id = node_id('a');
    let root = inference_application_node(
        root_id.clone(),
        &integer,
        "quire.op.integer.negate",
        vec![reference(&missing)],
    );
    let nodes = [scalar.clone(), root];
    let graph = inference_graph(&nodes);
    assert_eq!(
        infer_integer_range(&graph, &root_id, 2),
        Err(ExactScalarRefusal::InvalidBody {
            body_node_id: missing.clone()
        })
    );

    // A target-less Reference cannot be constructed through CheckedTerm. An absent
    // retained body still preserves the private InvalidBody/root-key defense.
    let missing_body = inference_node(
        root_id.clone(),
        "expression",
        "binary",
        integer.clone(),
        None,
    );
    let nodes = [scalar.clone(), missing_body];
    let graph = inference_graph(&nodes);
    assert_eq!(
        infer_integer_range(&graph, &root_id, 2),
        Err(ExactScalarRefusal::InvalidBody {
            body_node_id: root_id.clone()
        })
    );

    let cycle = inference_application_node(
        root_id.clone(),
        &integer,
        "quire.op.integer.negate",
        vec![reference(&root_id)],
    );
    let nodes = [scalar.clone(), cycle];
    let graph = inference_graph(&nodes);
    assert_eq!(
        infer_integer_range(&graph, &root_id, 0),
        Err(ExactScalarRefusal::InvalidBody {
            body_node_id: root_id.clone()
        })
    );

    let unsupported = inference_application_node(
        root_id.clone(),
        &integer,
        "quire.op.integer.divide",
        Vec::new(),
    );
    let nodes = [scalar.clone(), unsupported];
    let graph = inference_graph(&nodes);
    assert_eq!(
        infer_integer_range(&graph, &root_id, 1),
        Err(ExactScalarRefusal::OperandUnsupported {
            position: 1,
            term: "quire.op.integer.divide".to_owned()
        })
    );

    for (literal_value, found) in [("text", Some("text")), ("01", Some("integer"))] {
        let literal_id = node_id('c');
        let value_kind = if literal_value == "text" {
            "text"
        } else {
            "integer"
        };
        let literal = inference_node(
            literal_id.clone(),
            "value",
            "literal",
            integer.clone(),
            CheckedTerm::Literal {
                type_id: node_id('0'),
                value_kind: LiteralKind::from_wire(value_kind).expect("closed fixture kind"),
                value: serde_json::json!(literal_value),
            },
        );
        let unary = inference_application_node(
            root_id.clone(),
            &integer,
            "quire.op.integer.negate",
            vec![reference(&literal_id)],
        );
        let nodes = [scalar.clone(), literal, unary];
        let graph = inference_graph(&nodes);
        assert_eq!(
            infer_integer_range(&graph, &root_id, 3),
            Err(ExactScalarRefusal::OperandTypeMismatch {
                position: 3,
                expected: ScalarForm::Integer,
                found: found.map(str::to_owned),
            })
        );
    }
}

#[trace("FR-014-AC-45", "FR-014-AC-46", "TC-024")]
#[test]
fn tc_024_nested_inference_accepts_inline_canonical_integer_literals() {
    let integer = node_id('0');
    let bounded = node_id('1');
    let parameter = node_id('2');
    let root = node_id('3');
    let scalar = inference_node(
        integer.clone(),
        "scalar_type",
        "integer",
        integer.clone(),
        None,
    );
    let bounded_type = inference_node(
        bounded.clone(),
        "bounded_domain",
        "integer_range",
        integer.clone(),
        CheckedTerm::Aggregate {
            members: vec![
                CheckedTerm::Binding {
                    name: ("min").into(),
                    value: Box::new(CheckedTerm::Literal {
                        type_id: node_id('0'),
                        value_kind: LiteralKind::Integer,
                        value: serde_json::json!("0"),
                    }),
                },
                CheckedTerm::Binding {
                    name: ("max").into(),
                    value: Box::new(CheckedTerm::Literal {
                        type_id: node_id('0'),
                        value_kind: LiteralKind::Integer,
                        value: serde_json::json!("9"),
                    }),
                },
            ],
        },
    );
    let value = inference_node(parameter.clone(), "value", "parameter", bounded, None);
    let application = inference_application_node(
        root.clone(),
        &integer,
        "quire.op.integer.add",
        vec![
            reference(&parameter),
            CheckedTerm::Literal {
                type_id: node_id('0'),
                value_kind: LiteralKind::Integer,
                value: serde_json::json!("1"),
            },
        ],
    );
    let nodes = [scalar, bounded_type, value, application];
    let graph = inference_graph(&nodes);
    assert_eq!(
        infer_integer_range(&graph, &root, 0),
        Ok(("1".to_owned(), "10".to_owned()))
    );
}

#[trace("FR-014-AC-46", "FR-015-AC-104", "TC-024", "TC-033")]
#[test]
fn tc_024_nested_inference_enforces_depth_before_reusing_memoized_nodes() {
    let integer = node_id('0');
    let bounded = node_id('1');
    let parameter = node_id('2');
    let shared = node_id('3');
    let root = numbered_node_id(200);
    let chain = (101..=163).map(numbered_node_id).collect::<Vec<_>>();
    let scalar = inference_node(
        integer.clone(),
        "scalar_type",
        "integer",
        integer.clone(),
        None,
    );
    let bounded_type = inference_node(
        bounded.clone(),
        "bounded_domain",
        "integer_range",
        integer.clone(),
        CheckedTerm::Aggregate {
            members: vec![
                CheckedTerm::Binding {
                    name: ("min").into(),
                    value: Box::new(CheckedTerm::Literal {
                        type_id: node_id('0'),
                        value_kind: LiteralKind::Integer,
                        value: serde_json::json!("0"),
                    }),
                },
                CheckedTerm::Binding {
                    name: ("max").into(),
                    value: Box::new(CheckedTerm::Literal {
                        type_id: node_id('0'),
                        value_kind: LiteralKind::Integer,
                        value: serde_json::json!("9"),
                    }),
                },
            ],
        },
    );
    let value = inference_node(parameter.clone(), "value", "parameter", bounded, None);
    let shared_expression = inference_application_node(
        shared.clone(),
        &integer,
        "quire.op.integer.add",
        vec![
            reference(&parameter),
            CheckedTerm::Literal {
                type_id: node_id('0'),
                value_kind: LiteralKind::Integer,
                value: serde_json::json!("1"),
            },
        ],
    );
    let mut nodes = vec![scalar, bounded_type, value, shared_expression];
    for (index, node_id) in chain.iter().enumerate() {
        let next = chain.get(index + 1).unwrap_or(&shared);
        nodes.push(inference_application_node(
            node_id.clone(),
            &integer,
            "quire.op.integer.add",
            vec![reference(&parameter), reference(next)],
        ));
    }
    nodes.push(inference_application_node(
        root.clone(),
        &integer,
        "quire.op.integer.add",
        vec![reference(&shared), reference(&chain[0])],
    ));
    let graph = inference_graph(&nodes);
    assert_eq!(
        infer_integer_range(&graph, &root, 0),
        Err(ExactScalarRefusal::OperandRangeLimitExceeded {
            node_id: shared,
            resource_kind: OperandRangeLimitKind::Depth,
            limit: 64,
            consumed: 65,
        })
    );
}

/// Admission re-validates every body before lowering, so an admitted package
/// cannot produce these records; they are built directly.
#[trace("FR-014-AC-3", "TC-024")]
#[test]
fn tc_024_invalid_and_incomplete_bodies_are_typed_refusals() {
    let refusal = match quire_contract_model::CheckedPackageV2::read(
        b"{}",
        quire_contract_model::CheckedPackageReadLimits::bounded(),
        &quire_contract_model::CheckedPackageEvidence::new(),
    ) {
        quire_contract_model::CheckedPackageV2ReadResult::Refused(refusal) => refusal,
        other => panic!("an empty package is refused by the reader: {other:?}"),
    };
    let invalid = CompleteLoweringRecordV2::InvalidBody {
        node_id: node_id('a'),
        body_node_id: node_id('b'),
        refusal,
    };
    assert_eq!(
        lowered(&invalid).err(),
        Some(ExactScalarRefusal::InvalidBody {
            body_node_id: node_id('b')
        })
    );
    let incomplete = CompleteLoweringRecordV2::BodyIncomplete {
        node_id: node_id('a'),
        body_node_id: node_id('c'),
        incomplete: CheckedPackageIncomplete {
            limit_kind: CheckedPackageLimit::Nodes,
            limit: 128,
            consumed: 129,
            path: None,
        },
    };
    assert_eq!(
        lowered(&incomplete).err(),
        Some(ExactScalarRefusal::BodyIncomplete {
            body_node_id: node_id('c')
        })
    );
}

/// A `failed` record is refused by its limit kind through the one shared classifier: `work`
/// as `LoweringWorkExhausted`, `bytes` as `LoweringByteLimitExceeded` with the record's own
/// `limit` and `consumed` (the per-node case Contract IR's FR-038-AC-95 defines and CG's
/// public API cannot reach), and every other kind as `LoweringLimitUnrecognised` under its
/// snake_case name, never as work exhaustion and without a panic.
#[trace("FR-014-AC-40", "FR-014-AC-41", "TC-024")]
#[test]
fn tc_024_a_failed_record_is_refused_by_its_limit_kind() {
    use crate::oracle::failed_records::{failed_record, unrecognised_kinds};

    assert_eq!(
        lowered(&failed_record(CheckedPackageLimit::Work, 65_536, 65_537)).err(),
        Some(ExactScalarRefusal::LoweringWorkExhausted {
            limit: 65_536,
            consumed: 65_537
        })
    );
    assert_eq!(
        lowered(&failed_record(CheckedPackageLimit::Bytes, 1_000, 1_001)).err(),
        Some(ExactScalarRefusal::LoweringByteLimitExceeded {
            limit: 1_000,
            consumed: 1_001
        })
    );
    for (kind, name) in unrecognised_kinds() {
        assert_eq!(
            lowered(&failed_record(kind, 7, 9)).err(),
            Some(ExactScalarRefusal::LoweringLimitUnrecognised {
                limit_kind: name,
                limit: 7,
                consumed: 9
            }),
            "{name}"
        );
    }
}

/// quire-contract-ir's `validate_operations` requires `operation.identity`
/// on every application node before admission, so no admitted package can
/// carry a node without it and this node is built directly -- the same
/// reason the record cases above are.
///
/// The guard is still worth compiling. That admission check is recent
/// (IR-216) and is itself the kind of thing that can regress; if it did,
/// the node would arrive here. Before IR-224 this refusal was unreachable,
/// because `operation_confirmed` was consulted first and answers `false`
/// for a missing member exactly as it does for a disagreeing one -- so a
/// violated upstream invariant was reported as
/// `CallerDeclared { OperationIdentityNotConsumed }`, blaming this
/// generator for not consuming an identity the node never carried.
///
/// What this test covers and what it does not: it calls
/// [`catalogued_operation_identity`] directly, so it pins that function's
/// refusal and would pass under either call order. The reordering in
/// `generate_exact_scalar_oracles` is **not** covered by any test, and
/// cannot be: reaching it needs an admitted package containing an
/// application node with no `operation.identity`, which is the very thing
/// admission refuses to produce. Do not read this test as evidence that the
/// generator refuses such a node end to end.
#[trace("FR-014-AC-3", "TC-024")]
#[test]
fn tc_024_catalogued_operation_identity_refuses_a_node_whose_operation_has_no_identity() {
    let inner: CheckedSemanticNodeV2 = serde_json::from_value(serde_json::json!({
        "node_id": {
            "domain": "quire.checked-semantic-node/v1",
            "digest": "d".repeat(64),
        },
        "schema_version": "quire.checked-semantic-graph/v2",
        "node_tag": "expression",
        "semantic_form": "application",
        "semantic_type": {
            "domain": "quire.checked-semantic-node/v1",
            "digest": "e".repeat(64),
        },
        "dependencies": [],
        "occurrences": [],
        // `operation` present, `operation.identity` absent: the exact shape
        // admission is supposed to make impossible.
        "body": {
            "term": "application",
            "operator": "binary",
            "operation": { "mode": { "kind": "rounding", "value": "truncate" } },
            "result_type": {
                "domain": "quire.checked-semantic-node/v1",
                "digest": "e".repeat(64),
            },
            "arguments": [],
        },
    }))
    .expect("a node whose operation carries no identity");
    let node = CompleteContractNodeV2 {
        node: inner,
        node_tag: CheckedNodeTag::Expression,
        source_map: vec![],
        semantic_type: node_id('e'),
        dependencies: vec![],
        bounds: vec![],
        claims: vec![],
    };

    let fixture = FixtureNode::new(
        node.node.clone(),
        Some(app_term(serde_json::json!({
            "mode": { "kind": "rounding", "value": "truncate" }
        }))),
    );
    let key = fixture.node_id.clone();
    let graph: Graph<'_> = [(&key, &fixture)].into_iter().collect();
    assert_eq!(
        catalogued_operation_identity(&graph, &node),
        Err(ExactScalarRefusal::MissingOperationIdentity {
            node_id: node_id('d'),
        }),
        "a node missing the member IR admission guarantees must be refused by name, \
             not reported as an identity this generator declined to consume"
    );
}
// -- QSL shape (FR-322): bounded_domain-typed parameters and binding-shaped bounds ---------

pub(in crate::oracle::scalar) fn v2_node(
    digit: char,
    tag: &str,
    form: &str,
    semantic_type: char,
    body: impl Into<Option<CheckedTerm>>,
) -> FixtureNode {
    inference_node(node_id(digit), tag, form, node_id(semantic_type), body)
}

/// A `{term: binding, name, value: <literal>}` member, the shape QSL emits and IR reads;
/// the literal carries `type` as FR-322 requires.
fn binding_member(name: &str, value: &str) -> CheckedTerm {
    CheckedTerm::Binding { name: (name).into(), value: Box::new(CheckedTerm::Literal { type_id: serde_json::from_value(serde_json::json!({"domain": "quire.checked-semantic-node/v1", "digest": "1".repeat(64)})).expect("fixture type key"), value_kind: LiteralKind::Integer, value: serde_json::json!(value) }) }
}

/// A `text`-kind binding member.
fn text_member(name: &str, value: &str) -> CheckedTerm {
    CheckedTerm::Binding { name: (name).into(), value: Box::new(CheckedTerm::Literal { type_id: serde_json::from_value(serde_json::json!({"domain": "quire.checked-semantic-node/v1", "digest": "1".repeat(64)})).expect("fixture type key"), value_kind: LiteralKind::Text, value: serde_json::json!(value) }) }
}

fn interval_of(lower: &str, upper: &str) -> IntegerInterval {
    IntegerInterval::new(
        lower.parse().expect("integer"),
        upper.parse().expect("integer"),
    )
    .expect("interval")
}

fn range_body(members: Vec<CheckedTerm>) -> CheckedTerm {
    CheckedTerm::Aggregate { members }
}

/// Nodes `1` (Integer scalar type), `2` (`Int[0,9]`, an `integer_range` bounded_domain over
/// `1`), `3` (a parameter typed by `2`), `4` (Text scalar type) and `5` (a parameter typed by
/// a `text_bounds` bounded_domain `6` over `4`).
fn bounded_parameter_nodes() -> Vec<FixtureNode> {
    vec![
        v2_node('1', "scalar_type", "integer", '1', range_body(vec![])),
        v2_node(
            '2',
            "bounded_domain",
            "integer_range",
            '1',
            range_body(vec![binding_member("min", "0"), binding_member("max", "9")]),
        ),
        v2_node('3', "value", "parameter", '2', range_body(vec![])),
        v2_node('4', "scalar_type", "text", '4', range_body(vec![])),
        v2_node(
            '6',
            "bounded_domain",
            "text_bounds",
            '4',
            range_body(vec![]),
        ),
        v2_node('5', "value", "parameter", '6', range_body(vec![])),
    ]
}

fn reference_to(digit: char) -> CheckedTerm {
    CheckedTerm::Reference {
        target: serde_json::from_value(serde_json::json!({
            "domain": "quire.checked-semantic-node/v1", "digest": digit.to_string().repeat(64),
        }))
        .expect("fixture type key"),
    }
}

/// A conversion narrows a node only with the catalogued narrow identity and exactly one
/// argument, a reference to that node. IR refuses a two-argument narrow at admission, so this
/// reaches the check through a raw graph node.
#[trace("FR-014-AC-27", "TC-024")]
#[test]
fn tc_024_a_conversion_narrows_only_its_one_referenced_argument() {
    let target = node_id('2');
    let conversion = |digit, identity: &str, arguments: Vec<CheckedTerm>| {
        v2_node(
            digit,
            "expression",
            "conversion",
            '9',
            CheckedTerm::Application {
                operator: ApplicationOperator::Convert,
                operation: serde_json::from_value(serde_json::json!({"identity": identity}))
                    .expect("fixture operation"),
                result_type: node_id('0'),
                arguments,
            },
        )
    };
    let narrows_fixture = |node: &FixtureNode, target: &CheckedNodeId| {
        let graph: Graph<'_> = [(&node.node_id, node)].into_iter().collect();
        narrows(&graph, node, target)
    };
    assert!(narrows_fixture(
        &conversion('a', NARROW_IDENTITY, vec![reference_to('2')]),
        &target
    ));
    assert!(!narrows_fixture(
        &conversion(
            'b',
            NARROW_IDENTITY,
            vec![reference_to('2'), reference_to('3')]
        ),
        &target
    ));
    assert!(!narrows_fixture(
        &conversion('c', "quire.op.numeric.convert", vec![reference_to('2')]),
        &target
    ));
    assert!(!narrows_fixture(
        &conversion('d', NARROW_IDENTITY, vec![reference_to('3')]),
        &target
    ));
}

/// A reference to a parameter typed by a `bounded_domain` is classified by the domain's base
/// scalar type, not refused as `found: None`.
#[trace("FR-014-AC-17", "TC-024")]
#[test]
fn tc_024_reference_typed_by_a_bounded_domain_is_classified_by_its_base_scalar_type() {
    let nodes = bounded_parameter_nodes();
    let graph: Graph<'_> = nodes.iter().map(|node| (&node.node_id, node)).collect();
    assert_eq!(
        check_operand(&graph, 0, &reference_to('3'), ScalarForm::Integer),
        Ok(())
    );
    assert_eq!(
        check_operand(&graph, 1, &reference_to('5'), ScalarForm::Integer),
        Err(ExactScalarRefusal::OperandTypeMismatch {
            position: 1,
            expected: ScalarForm::Integer,
            found: Some("text".to_owned()),
        }),
        "a bounded text parameter is a text operand, not an integer one"
    );
}

/// The bound is read from binding-shaped members (`min`, `max`); the bare-literal shape this
/// generator once read is refused as unreadable.
#[trace("FR-014-AC-16", "TC-024")]
#[test]
fn tc_024_integer_range_is_read_from_binding_members_and_refuses_bare_literals() {
    let bound_id = node_id('2');
    let mut nodes = bounded_parameter_nodes();
    let bare = |value: &str| CheckedTerm::Literal {
        type_id: node_id('0'),
        value_kind: LiteralKind::Integer,
        value: serde_json::json!(value),
    };
    nodes.push(v2_node(
        '7',
        "bounded_domain",
        "integer_range",
        '1',
        range_body(vec![bare("0"), bare("9")]),
    ));
    let graph: Graph<'_> = nodes.iter().map(|node| (&node.node_id, node)).collect();
    let members = |digit: char| {
        aggregate_members(graph.term(&node_id(digit)))
            .expect("aggregate body")
            .to_vec()
    };
    assert_eq!(
        read_integer_range(&members('2')),
        Some(interval_of("0", "9"))
    );
    assert_eq!(read_integer_range(&members('7')), None);

    // Through `Bounds::equal`, as `check_parameters` reads it: the descriptor parameter equals
    // the binding-shaped bound, and the bare one is `UnreadableBound`.
    let node = |bound: char| CompleteContractNodeV2 {
        node: v2_node('8', "expression", "application", '1', range_body(vec![])).metadata,
        node_tag: CheckedNodeTag::Expression,
        source_map: vec![],
        semantic_type: node_id('1'),
        dependencies: vec![],
        bounds: vec![node_id(bound)],
        claims: vec![],
    };
    let declared = interval_of("0", "9");
    let (good, bad) = (node('2'), node('7'));
    assert_eq!(
        Bounds {
            graph: &graph,
            node: BoundView::complete(&good)
        }
        .equal(BoundForm::IntegerRange, read_integer_range, Some(&declared)),
        Ok(bound_id)
    );
    assert_eq!(
        Bounds {
            graph: &graph,
            node: BoundView::complete(&bad)
        }
        .equal(BoundForm::IntegerRange, read_integer_range, Some(&declared)),
        Err(ExactScalarRefusal::UnreadableBound {
            bound: node_id('7')
        })
    );
}

/// Bound members are looked up by name, in any order (QSpec's reader does the same); a
/// missing, duplicate or unknown name is refused.
#[trace("FR-014-AC-16", "TC-024")]
#[test]
fn tc_024_bound_members_are_looked_up_by_name_in_any_order() {
    // Swapped integer range.
    assert_eq!(
        read_integer_range(&[binding_member("max", "9"), binding_member("min", "0")]),
        Some(interval_of("0", "9"))
    );
    // A shuffled rational range still puts numerator and denominator in their own intervals.
    let rational = read_rational_range(&[
        binding_member("denominator_max", "7"),
        binding_member("numerator_min", "-3"),
        binding_member("denominator_min", "2"),
        binding_member("numerator_max", "5"),
    ])
    .expect("shuffled rational range");
    assert_eq!(
        rational,
        RationalDomain::new(interval_of("-3", "5"), interval_of("2", "7")).expect("domain")
    );
    // A shuffled decimal range and text bounds.
    let decimal = read_decimal_range(&[
        text_member("rounding", "nearest-even"),
        binding_member("scale_max", "2"),
        binding_member("coefficient_max", "1000"),
        binding_member("scale_min", "0"),
        binding_member("coefficient_min", "-1000"),
    ])
    .expect("shuffled decimal range");
    assert_eq!(
        decimal,
        DecimalType::new(
            "-1000".parse().expect("integer"),
            "1000".parse().expect("integer"),
            0,
            2,
            RoundingMode::NearestEven,
        )
        .expect("decimal type")
    );
    assert!(read_text_bounds(&[
        text_member("text_profile", "nfc"),
        binding_member("max", "64"),
        binding_member("min", "0"),
    ])
    .is_some());
    // Refusals: duplicate, wrong, missing and extra names.
    assert_eq!(
        read_integer_range(&[binding_member("min", "0"), binding_member("min", "9")]),
        None
    );
    assert_eq!(
        read_integer_range(&[binding_member("min", "0"), binding_member("high", "9")]),
        None
    );
    assert_eq!(read_integer_range(&[binding_member("min", "0")]), None);
    assert_eq!(
        read_integer_range(&[
            binding_member("min", "0"),
            binding_member("max", "9"),
            binding_member("max", "9"),
        ]),
        None
    );
    // A rational range whose members carry the integer-range names is not a rational range.
    assert_eq!(
        read_rational_range(&[
            binding_member("min", "0"),
            binding_member("max", "9"),
            binding_member("min", "1"),
            binding_member("max", "2"),
        ]),
        None
    );
}
