// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Constructor and public facade controls, without a claim of generated backend verification.

use std::collections::BTreeMap;

use ix_trace_rs::trace;
use qsl_replay::{
    compile_package, parity_obligation, DependencyInput, DigestDomain, DigestRecord, Domain,
    DomainKey, DomainKind, EqualityOperator, FiniteBound, InconclusiveCause, Integer, NodeKey,
    OperandIdentity, ParityBoundRefusal, ProofBound, Refinement, ReplayLimits, ReplayRefusal,
    ReplayRequest, ReplayTarget, ScalarIdentityMismatch, ScalarLimits, SourceIdentity, StageLimits,
    SuppliedLibrary, TerminalValue, VerifiedShadow, VerifiedShadowResult,
};
use quire_contract_codegen::{
    CompositeBuildError, CompositePacketMember, DependencyLock, LockedSource,
    OriginalCompositeEqContext, OriginalCompositeEqRequest, ReplayInputs,
};
use quire_contract_model::{
    CheckedCompositeOperandDomain, CheckedCompositeOperandError, CheckedNodeId,
    CheckedNodePathSegment, CheckedOccurrence, CheckedPackageEvidence, CheckedPackageReadLimits,
    CheckedPackageV2, CheckedPackageV2ReadResult, CheckedScalarOperandChild, CheckedTerm,
};
use quire_exact::PathSegment;

fn node_key(id: &CheckedNodeId) -> NodeKey {
    let path = id
        .path
        .iter()
        .map(|segment| match segment {
            CheckedNodePathSegment::Name(name) => PathSegment::Text(name.to_string()),
            CheckedNodePathSegment::Node(child) => PathSegment::Node(node_key(child)),
        })
        .collect();
    NodeKey::new(id.package.to_string(), path, id.ordinal).expect("admitted structural node")
}

fn checked_operation_identity(body: &CheckedTerm) -> Option<&str> {
    let CheckedTerm::Application { operation, .. } = body else {
        return None;
    };
    operation.identity()
}

const SOURCE: &str = "language \"ix:native\" edition \"1-draft\";\n\
    profile v = \"quire.value.complete/v1\";\n\
    record Q { x: Integer; }\n\
    function f using v(a: Sequence<Int[0, 9]>[0, 3], b: Sequence<Int[0, 9]>[0, 3]): Boolean pure { a = b }\n\
    function n using v(a: Sequence<Int[0, 9]>[0, 3], b: Sequence<Int[0, 9]>[0, 3]): Boolean pure { a != b }\n\
    function g using v(a: Q): Boolean pure { a = Q { x: 1 } }\n\
    function m using v(a: Q): Boolean pure { a != Q { x: 1 } }\n\
    function h using v(): Boolean pure { Q { x: 1 } = Q { x: 2 } }\n\
    function same using v(a: Q): Boolean pure { a = a }\n";

const REPEATED_SOURCE: &str = "language \"ix:native\" edition \"1-draft\";\n\
    profile v = \"quire.value.complete/v1\";\n\
    function repeated using v(a: Sequence<Int[0, 9]>[0, 3], b: Sequence<Int[0, 9]>[0, 3]): Boolean pure { (a = b) and (a = b) }\n";

fn limits() -> ScalarLimits {
    ScalarLimits {
        integer_bits: 1_000,
        decimal_digits: 1_000,
        scale_expansion: 1_000,
        text_input_bytes: 1_000,
        text_scalars: 1_000,
        normalized_scalars: 1_000,
        unit_edges: 1_000,
        value_occurrences: 1_000,
        work_units: 1_000,
        result_units: 1_000,
    }
}

fn inputs(source: &str) -> ReplayInputs {
    ReplayInputs {
        decode_limits: crate::unbounded_decode_limits(),
        package_name: "original-eq".to_owned(),
        source: LockedSource {
            authority: "test".to_owned(),
            identity: "original-eq.native".to_owned(),
            namespace: "git".to_owned(),
            revision: "fixture".to_owned(),
            bytes: source.as_bytes().to_vec(),
        },
        dependencies: Vec::new(),
        accounting_limits: limits(),
        stage_limits: BTreeMap::new(),
        declared_domains: Vec::new(),
        replay_limits: ReplayLimits::default(),
    }
}

fn checked(source: &str) -> CheckedPackageV2 {
    checked_with_dependencies(source, &DependencyInput::default(), &evidence())
}

fn evidence() -> CheckedPackageEvidence {
    let mut evidence = CheckedPackageEvidence::new();
    evidence.support_feature("quire.value.complete/v1");
    evidence
}

fn checked_with_dependencies(
    source: &str,
    dependencies: &DependencyInput,
    evidence: &CheckedPackageEvidence,
) -> CheckedPackageV2 {
    read_checked(compiled(source, dependencies).bytes(), evidence)
}

fn compiled(source: &str, dependencies: &DependencyInput) -> qsl_replay::CompiledPackage {
    compile_package(
        qsl_replay::CompileUnit {
            source: SourceIdentity::new("test", "original-eq.native", "git", "fixture"),
            package_name: "original-eq".to_owned(),
            path: "original-eq.native",
            bytes: source.as_bytes(),
        },
        [],
        dependencies,
        StageLimits::default(),
        ReplayLimits::default(),
    )
    .expect("fixture compiles through public QSL facade")
}

fn read_checked(bytes: &[u8], evidence: &CheckedPackageEvidence) -> CheckedPackageV2 {
    match CheckedPackageV2::read(bytes, CheckedPackageReadLimits::bounded(), evidence) {
        CheckedPackageV2ReadResult::Admitted(package) => *package,
        other => panic!("QSL fixture must admit in IR: {other:?}"),
    }
}

// Adversarial emitted-package metadata is admitted through IR's owning reader; no report is built.
fn checked_with_extra_source(
    source: &str,
    extra: &quire_contract_model::CheckedSourceRef,
) -> CheckedPackageV2 {
    let compiled = compiled(source, &DependencyInput::default());
    let mut wire: serde_json::Value = serde_json::from_slice(compiled.bytes()).unwrap();
    wire["lock"]["sources"]
        .as_array_mut()
        .unwrap()
        .push(serde_json::to_value(extra).unwrap());
    let bytes = quire_canonical::to_vec(&wire, quire_canonical::Limits::new(1 << 20)).unwrap();
    read_checked(&bytes, &evidence())
}

fn content_identity() -> DigestRecord {
    // A canonical fixture binding input. It establishes no generated-artifact authentication.
    let digest =
        quire_canonical::sha256(&"constructor-fixture", quire_canonical::Limits::new(1_000))
            .unwrap();
    DigestRecord::mint(DigestDomain::Sha256Jcs, *digest.as_bytes())
}

fn context(package: CheckedPackageV2, function: &str) -> OriginalCompositeEqContext {
    OriginalCompositeEqContext::new(
        package,
        inputs(SOURCE),
        function,
        content_identity(),
        CheckedPackageReadLimits::bounded(),
    )
    .unwrap()
}

// Existing FR-033 controls use the claim producer before invoking the new packet-only request
// boundary. FR-024-AC-44/46 swap packets explicitly in their dedicated control below.
trait OriginalRequestFixture {
    fn request(
        &self,
        node: &CheckedNodeId,
        occurrence: &CheckedOccurrence,
        operator: EqualityOperator,
        harness_bounds: &[ProofBound],
        work_limit: u64,
    ) -> Result<OriginalCompositeEqRequest, CompositeBuildError>;
}

impl OriginalRequestFixture for OriginalCompositeEqContext {
    fn request(
        &self,
        node: &CheckedNodeId,
        occurrence: &CheckedOccurrence,
        operator: EqualityOperator,
        harness_bounds: &[ProofBound],
        work_limit: u64,
    ) -> Result<OriginalCompositeEqRequest, CompositeBuildError> {
        let packet = self.parity_packet(node, occurrence, operator, harness_bounds, work_limit)?;
        self.request_from_packet(
            packet,
            node,
            occurrence,
            operator,
            harness_bounds,
            work_limit,
        )
    }
}

fn application(package: &CheckedPackageV2, function: &str) -> (CheckedNodeId, CheckedOccurrence) {
    application_with_operator(package, function, "quire.op.structural.eq")
}

fn application_with_operator(
    package: &CheckedPackageV2,
    function: &str,
    operator: &str,
) -> (CheckedNodeId, CheckedOccurrence) {
    let declaration = package
        .graph()
        .nodes
        .iter()
        .find(|node| {
            node.declaration.as_ref().is_some_and(|declaration| {
                declaration.qualified_name == [Box::<str>::from(function)]
            })
        })
        .unwrap();
    let regions = package
        .source_map()
        .iter()
        .filter(|entry| entry.node_id == declaration.node_id)
        .flat_map(|entry| &entry.regions)
        .collect::<Vec<_>>();
    let application = package
        .graph()
        .nodes
        .iter()
        .find(|node| {
            checked_operation_identity(&node.body) == Some(operator)
                && package
                    .source_map()
                    .iter()
                    .filter(|entry| entry.node_id == node.node_id)
                    .any(|entry| {
                        entry.regions.iter().any(|region| {
                            regions.iter().any(|outer| {
                                outer.source == region.source
                                    && outer.start <= region.start
                                    && region.end <= outer.end
                            })
                        })
                    })
        })
        .unwrap();
    (
        application.node_id.clone(),
        application.occurrences[0].clone(),
    )
}

fn parameter_bounds(
    package: &CheckedPackageV2,
    node: &CheckedNodeId,
    occurrence: &CheckedOccurrence,
) -> Vec<ProofBound> {
    let operands = package
        .composite_application_operands(node, occurrence, 100_000)
        .unwrap();
    operands
        .operands
        .iter()
        .filter_map(|operand| {
            if let CheckedCompositeOperandDomain::Parameter { .. } = operand.domain {
                let CheckedScalarOperandChild::GraphChild(child) = &operand.child else {
                    panic!("graph child")
                };
                Some(node_key(child))
            } else {
                None
            }
        })
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .flat_map(|node| {
            [
                ProofBound::new(
                    DomainKey::Node {
                        node: node.clone(),
                        path: vec![],
                    },
                    Some(DomainKind::Collection),
                    FiniteBound::cardinality(3),
                )
                .unwrap(),
                ProofBound::new(
                    DomainKey::Node {
                        node,
                        path: vec![0],
                    },
                    Some(DomainKind::Integer),
                    FiniteBound::integer_range(Integer::from(0_i64), Integer::from(9_i64)).unwrap(),
                )
                .unwrap(),
            ]
        })
        .collect()
}

/// Original parameter/parameter construction reaches a genuine QSL verified report and converter.
/// This is a constructor control with supplied verified evidence, not a generated-family proof.
#[trace("FR-033-AC-1", "FR-033-AC-9", "FR-033-AC-11", "TC-048")]
#[test]
fn original_parameter_operands_build_positional_identity_and_public_verified_report() {
    let package = checked(SOURCE);
    let (node, occurrence) = application(&package, "f");
    let bounds = parameter_bounds(&package, &node, &occurrence);
    let mut supplied_bounds = bounds.clone();
    supplied_bounds.push(
        ProofBound::new(
            DomainKey::Population {
                member_type: node_key(&node),
                ordinal: 0,
            },
            Some(DomainKind::Population),
            FiniteBound::cardinality(7),
        )
        .unwrap(),
    );
    let original = package
        .composite_application_operands(&node, &occurrence, 100_000)
        .unwrap();
    let request = context(package, "f")
        .request(
            &node,
            &occurrence,
            EqualityOperator::Equal,
            &supplied_bounds,
            100_000,
        )
        .unwrap();
    assert_eq!(request.operands(), &original);
    assert_eq!(request.preimage().arguments.len(), 2);
    for (argument, operand) in request.preimage().arguments.iter().zip(&original.operands) {
        let CheckedScalarOperandChild::GraphChild(child) = &operand.child else {
            panic!("graph child")
        };
        assert_eq!(
            argument.identity,
            OperandIdentity::GraphChild(node_key(child))
        );
        let Domain::Bounds(entries) = &argument.domain else {
            panic!("composite bounds")
        };
        assert_eq!(entries.entries().len(), 2);
        assert!(entries.entries().iter().all(|entry| matches!(entry.domain(), DomainKey::Node { node, .. } if *node == node_key(child))));
    }
    let obligation = parity_obligation(request.preimage()).unwrap();
    let verified = VerifiedShadow {
        success_checks: 1,
        refinement: Refinement::NotExhausted,
    };
    let report = request.settle_verified(verified).unwrap();
    assert_eq!(report.sent().obligation, obligation);
    assert_eq!(report.sent().harness_bounds, bounds);
    assert_eq!(report.sent().limits, limits());
    assert_eq!(report.sent().content_identity, content_identity());
    assert_eq!(report.report().claim(), report.sent());
    let settlement = report.settlement().unwrap();
    assert!(matches!(settlement.result(), VerifiedShadowResult::Tested));
    assert_eq!(settlement.terminal_value(), TerminalValue::Tested);
}

/// Eq literal graph children keep singleton source domains, not parameter-bound substitutions.
#[trace("FR-033-AC-1", "FR-033-AC-11", "TC-048")]
#[test]
fn original_literal_operands_retain_graph_children_and_empty_bounds() {
    for (function, literals) in [("g", 1), ("h", 2)] {
        let package = checked(SOURCE);
        let (node, occurrence) = application(&package, function);
        let original = context(package, function);
        let baseline = original
            .request(&node, &occurrence, EqualityOperator::Equal, &[], 100_000)
            .unwrap();
        let bounds = baseline
            .operands()
            .operands
            .iter()
            .map(|operand| {
                let CheckedScalarOperandChild::GraphChild(child) = &operand.child else {
                    panic!("graph child")
                };
                ProofBound::new(
                    DomainKey::Node {
                        node: node_key(child),
                        path: vec![0],
                    },
                    Some(DomainKind::Integer),
                    FiniteBound::integer_range(Integer::from(0_i64), Integer::from(9_i64)).unwrap(),
                )
                .unwrap()
            })
            .collect::<Vec<_>>();
        let request = original
            .request(
                &node,
                &occurrence,
                EqualityOperator::Equal,
                &bounds,
                100_000,
            )
            .unwrap();
        let mut measured_literals = 0;
        for (operand, argument) in request
            .operands()
            .operands
            .iter()
            .zip(&request.preimage().arguments)
        {
            if operand.domain == CheckedCompositeOperandDomain::Literal {
                measured_literals += 1;
                let CheckedScalarOperandChild::GraphChild(child) = &operand.child else {
                    panic!("literal graph child")
                };
                assert_eq!(
                    argument.identity,
                    OperandIdentity::GraphChild(node_key(child))
                );
                let Domain::Bounds(entries) = &argument.domain else {
                    panic!("literal bounds")
                };
                assert!(entries.entries().is_empty());
            } else {
                let OperandIdentity::GraphChild(ref child) = argument.identity else {
                    panic!("parameter graph child")
                };
                let Domain::Bounds(entries) = &argument.domain else {
                    panic!("parameter bounds")
                };
                assert_eq!(entries.entries().len(), 1);
                assert!(entries.entries().iter().all(
                    |entry| matches!(entry.domain(), DomainKey::Node { node, .. } if *node == child)
                ));
            }
        }
        assert_eq!(measured_literals, literals);
        let report = baseline
            .settle_verified(VerifiedShadow {
                success_checks: 1,
                refinement: Refinement::NotExhausted,
            })
            .unwrap();
        assert!(matches!(
            report.settlement().unwrap().result(),
            VerifiedShadowResult::Tested
        ));
    }
}

/// Ne keeps the original parameter order, operator and positional O-09 arguments.
#[trace("FR-033-AC-1", "FR-033-AC-11", "TC-048")]
#[test]
fn original_ne_parameter_operands_keep_order_and_operator() {
    let package = checked(SOURCE);
    let (node, occurrence) = application_with_operator(&package, "n", "quire.op.structural.ne");
    let original = package
        .composite_application_operands(&node, &occurrence, 100_000)
        .unwrap();
    let bounds = parameter_bounds(&package, &node, &occurrence);
    let request = context(package, "n")
        .request(
            &node,
            &occurrence,
            EqualityOperator::NotEqual,
            &bounds,
            100_000,
        )
        .unwrap();
    assert_eq!(request.operands(), &original);
    assert_eq!(request.preimage().arguments.len(), 2);
    for (argument, operand) in request.preimage().arguments.iter().zip(&original.operands) {
        let CheckedScalarOperandChild::GraphChild(child) = &operand.child else {
            panic!("graph child")
        };
        assert_eq!(
            argument.identity,
            OperandIdentity::GraphChild(node_key(child))
        );
    }
    assert_ne!(
        request.preimage().arguments[0].identity,
        request.preimage().arguments[1].identity
    );
    let obligation = parity_obligation(request.preimage()).unwrap();
    let report = request
        .settle_verified(VerifiedShadow {
            success_checks: 1,
            refinement: Refinement::NotExhausted,
        })
        .unwrap();
    assert_eq!(report.sent().operator, EqualityOperator::NotEqual);
    assert_eq!(report.sent().obligation, obligation);
    assert_eq!(report.report().claim(), report.sent());
    assert_eq!(
        report.settlement().unwrap().terminal_value(),
        TerminalValue::Tested
    );
}

/// Ne literal graph children retain their singleton source domain and the real operation.
#[trace("FR-033-AC-1", "FR-033-AC-11", "TC-048")]
#[test]
fn original_ne_literal_operand_keeps_empty_bounds() {
    let package = checked(SOURCE);
    let (node, occurrence) = application_with_operator(&package, "m", "quire.op.structural.ne");
    let request = context(package, "m")
        .request(&node, &occurrence, EqualityOperator::NotEqual, &[], 100_000)
        .unwrap();
    assert_eq!(request.operands().operands.len(), 2);
    assert_eq!(
        request.operands().operands[1].domain,
        CheckedCompositeOperandDomain::Literal
    );
    let CheckedScalarOperandChild::GraphChild(child) = &request.operands().operands[1].child else {
        panic!("literal graph child")
    };
    assert_eq!(
        request.preimage().arguments[1].identity,
        OperandIdentity::GraphChild(node_key(child))
    );
    let Domain::Bounds(entries) = &request.preimage().arguments[1].domain else {
        panic!("literal bounds")
    };
    assert!(entries.entries().is_empty());
    let report = request
        .settle_verified(VerifiedShadow {
            success_checks: 1,
            refinement: Refinement::NotExhausted,
        })
        .unwrap();
    assert_eq!(report.sent().operator, EqualityOperator::NotEqual);
    assert_eq!(report.report().claim(), report.sent());
    assert_eq!(
        report.settlement().unwrap().terminal_value(),
        TerminalValue::Tested
    );
}

/// A caller cannot select Eq for a Ne node or Ne for an Eq node.
#[trace("FR-033-AC-1", "TC-048")]
#[test]
fn original_operator_mismatch_refuses_with_typed_operators() {
    for (function, node_operator, requested) in [
        ("f", EqualityOperator::Equal, EqualityOperator::NotEqual),
        ("n", EqualityOperator::NotEqual, EqualityOperator::Equal),
    ] {
        let package = checked(SOURCE);
        let identity = match node_operator {
            EqualityOperator::Equal => "quire.op.structural.eq",
            EqualityOperator::NotEqual => "quire.op.structural.ne",
        };
        let (node, occurrence) = application_with_operator(&package, function, identity);
        assert!(matches!(
            context(package, function).request(&node, &occurrence, requested, &[], 100_000),
            Err(CompositeBuildError::OperatorMismatch { requested: actual_requested, actual })
                if actual_requested == requested && actual == node_operator
        ));
    }
}

/// One retained context resolves distinct operation nodes across repeated requests.
#[trace("FR-033-AC-1", "TC-048")]
#[test]
fn repeated_requests_keep_each_nodes_original_operation() {
    let package = checked(SOURCE);
    let (equal_node, equal_occurrence) = application(&package, "f");
    let (not_equal_node, not_equal_occurrence) =
        application_with_operator(&package, "n", "quire.op.structural.ne");
    let context = context(package, "f");

    let equal = context
        .request(
            &equal_node,
            &equal_occurrence,
            EqualityOperator::Equal,
            &[],
            100_000,
        )
        .unwrap();
    let not_equal = context
        .request(
            &not_equal_node,
            &not_equal_occurrence,
            EqualityOperator::NotEqual,
            &[],
            100_000,
        )
        .unwrap();

    assert_eq!(equal.preimage().node, node_key(&equal_node));
    assert_eq!(not_equal.preimage().node, node_key(&not_equal_node));
    assert!(matches!(
        context.request(
            &not_equal_node,
            &not_equal_occurrence,
            EqualityOperator::Equal,
            &[],
            100_000,
        ),
        Err(CompositeBuildError::OperatorMismatch {
            requested: EqualityOperator::Equal,
            actual: EqualityOperator::NotEqual,
        })
    ));
}

/// Repeated parameter positions remain distinct O-09 arguments.
#[trace("FR-033-AC-11", "TC-048")]
#[test]
fn self_comparison_keeps_two_positional_arguments() {
    let package = checked(SOURCE);
    let (node, occurrence) = application(&package, "same");
    let request = context(package, "same")
        .request(&node, &occurrence, EqualityOperator::Equal, &[], 100_000)
        .unwrap();
    assert_eq!(request.preimage().arguments.len(), 2);
    assert_eq!(
        request.preimage().arguments[0].identity,
        request.preimage().arguments[1].identity
    );
    let report = request
        .settle_verified(VerifiedShadow {
            success_checks: 1,
            refinement: Refinement::NotExhausted,
        })
        .unwrap();
    assert!(matches!(
        report.settlement().unwrap().result(),
        VerifiedShadowResult::Tested
    ));
}

/// Changed bytes and another source lock refuse before any report can be created.
#[trace("FR-033-AC-1", "FR-033-AC-9", "TC-048")]
#[test]
fn wrong_source_refuses_before_invocation() {
    let mut wrong_source = inputs(SOURCE);
    wrong_source.source.bytes.push(b'\n');
    assert!(matches!(
        OriginalCompositeEqContext::new(
            checked(SOURCE),
            wrong_source,
            "f",
            content_identity(),
            CheckedPackageReadLimits::bounded()
        ),
        Err(CompositeBuildError::SourceMismatch)
    ));
    let another_package = checked(&SOURCE.replace("Int[0, 9]", "Int[0, 8]"));
    assert!(matches!(
        OriginalCompositeEqContext::new(
            another_package,
            inputs(SOURCE),
            "f",
            content_identity(),
            CheckedPackageReadLimits::bounded()
        ),
        Err(CompositeBuildError::SourceMismatch)
    ));
}

/// A genuinely different admitted graph retains the supplied source in its lock but refuses recompilation.
/// Malformed package IDs cannot be constructed through IR admission and are not coverage claims.
#[trace("FR-033-AC-1", "FR-033-AC-9", "TC-048")]
#[test]
fn different_semantic_package_refuses_before_invocation() {
    let original = checked(SOURCE);
    let changed_source = SOURCE.replace("Int[0, 9]", "Int[0, 8]");
    let changed = checked_with_extra_source(&changed_source, &original.lock().sources[0]);
    assert_ne!(changed.lock(), original.lock());
    let (node, occurrence) = application(&changed, "f");
    assert!(matches!(
        context(changed, "f").request(&node, &occurrence, EqualityOperator::Equal, &[], 100_000),
        Err(CompositeBuildError::PackageMismatch)
    ));
}

/// Extra retained dependency selection and altered admitted source-lock context refuse distinctly.
#[trace("FR-033-AC-1", "FR-033-AC-9", "TC-048")]
#[test]
fn retained_selection_and_recompiled_context_mismatch_refuse() {
    let package = checked(SOURCE);
    let mut retained = inputs(SOURCE);
    retained.dependencies.push(DependencyLock {
        identity: "test/extra".to_owned(),
        source: retained.source.clone(),
    });
    assert!(matches!(
        OriginalCompositeEqContext::new(
            package,
            retained,
            "f",
            content_identity(),
            CheckedPackageReadLimits::bounded()
        ),
        Err(CompositeBuildError::ContextMismatch)
    ));
    let extra = checked(&format!("{SOURCE}\n"));
    let changed = checked_with_extra_source(SOURCE, &extra.lock().sources[0]);
    assert_ne!(changed.lock(), checked(SOURCE).lock());
    let (node, occurrence) = application(&changed, "f");
    assert!(matches!(
        context(changed, "f").request(&node, &occurrence, EqualityOperator::Equal, &[], 100_000),
        Err(CompositeBuildError::ContextMismatch)
    ));
}

/// Absent nodes/occurrences refuse structurally before evidence is accepted; no disagreement is supplied.
#[trace("FR-033-AC-1", "FR-033-AC-9", "TC-048")]
#[test]
fn wrong_node_and_occurrence_refuse_before_invocation() {
    let package = checked(SOURCE);
    let (node, occurrence) = application(&package, "f");
    let context = context(package, "f");
    let mut absent = node.clone();
    absent
        .path
        .push(CheckedNodePathSegment::Name("absent".into()));
    assert!(
        matches!(context.request(&absent, &occurrence, EqualityOperator::Equal, &[], 100_000), Err(CompositeBuildError::Operands(cause)) if matches!(*cause, CheckedCompositeOperandError::UnknownNode { .. }))
    );
    let mut wrong_occurrence = occurrence.clone();
    wrong_occurrence.ordinal = u64::MAX;
    assert!(
        matches!(context.request(&node, &wrong_occurrence, EqualityOperator::Equal, &[], 100_000), Err(CompositeBuildError::Operands(cause)) if matches!(*cause, CheckedCompositeOperandError::MissingOccurrence { .. }))
    );
}

/// Operator parity uses the independently produced, admitted packet for its own checked claim.
#[trace("FR-024-AC-44", "FR-024-AC-46", "TC-035")]
#[test]
fn tc_035_original_composite_request_requires_its_own_packet() {
    let package = checked(SOURCE);
    let (node, occurrence) = application(&package, "f");
    let context = context(package, "f");
    let packet = context
        .parity_packet(&node, &occurrence, EqualityOperator::Equal, &[], 100_000)
        .expect("the original checked claim supplies a packet");
    let request = context
        .request_from_packet(
            packet.clone(),
            &node,
            &occurrence,
            EqualityOperator::Equal,
            &[],
            100_000,
        )
        .expect("the matching packet can build a request");
    assert_eq!(
        request.wire().replay_target.occurrence_key.as_ref(),
        packet.occurrence_key.as_ref()
    );
    assert_eq!(request.wire().backend, packet.backend.as_deref().unwrap());
    let readback = ReplayRequest::decode(request.wire().clone(), ReplayLimits::default())
        .expect("the packet-backed request has a typed QSL readback");
    assert_eq!(
        readback
            .replay_target()
            .and_then(ReplayTarget::occurrence_key),
        packet.occurrence_key.as_ref()
    );

    let mut wrong_arm = packet.clone();
    wrong_arm.source = Some(qsl_replay::EnvelopeSource::Witness(
        qsl_replay::Witness::parse("<<<assertion|h|c|>>>").expect("valid fixture witness"),
    ));
    assert!(matches!(
        context.request_from_packet(
            wrong_arm,
            &node,
            &occurrence,
            EqualityOperator::Equal,
            &[],
            100_000,
        ),
        Err(CompositeBuildError::PacketMismatch(
            CompositePacketMember::SourceArm
        ))
    ));

    let mut wrong_subject = context
        .parity_packet(&node, &occurrence, EqualityOperator::Equal, &[], 100_000)
        .expect("the checked claim supplies another packet");
    wrong_subject.clause_node = Some(
        NodeKey::new(
            "original-eq",
            vec![PathSegment::Text("other".to_owned())],
            0,
        )
        .unwrap(),
    );
    assert!(matches!(
        context.request_from_packet(
            wrong_subject,
            &node,
            &occurrence,
            EqualityOperator::Equal,
            &[],
            100_000,
        ),
        Err(CompositeBuildError::PacketMismatch(
            CompositePacketMember::Subject
        ))
    ));

    let mut wrong_occurrence = context
        .parity_packet(&node, &occurrence, EqualityOperator::Equal, &[], 100_000)
        .expect("the checked claim supplies another packet");
    wrong_occurrence.occurrence_key = Some(qsl_replay::OccurrenceKey::new(
        node_key(&node),
        qsl_replay::Origin::new(qsl_replay::Role::new("expression"), u64::MAX),
    ));
    assert!(matches!(
        context.request_from_packet(
            wrong_occurrence,
            &node,
            &occurrence,
            EqualityOperator::Equal,
            &[],
            100_000,
        ),
        Err(CompositeBuildError::PacketMismatch(
            CompositePacketMember::Occurrence
        ))
    ));

    let mut wrong_target = context
        .parity_packet(&node, &occurrence, EqualityOperator::Equal, &[], 100_000)
        .expect("the checked claim supplies another packet");
    wrong_target.replay_target = qsl_replay::ReplayTargetMembers::function(
        qsl_replay::QualifiedName::new(vec![
            qsl_replay::Identifier::new("g").expect("valid function name")
        ])
        .expect("valid function selection"),
    );
    assert!(matches!(
        context.request_from_packet(
            wrong_target,
            &node,
            &occurrence,
            EqualityOperator::Equal,
            &[],
            100_000,
        ),
        Err(CompositeBuildError::PacketMismatch(
            CompositePacketMember::Target
        ))
    ));

    let mut wrong_obligation = context
        .parity_packet(&node, &occurrence, EqualityOperator::Equal, &[], 100_000)
        .expect("the checked claim supplies another packet");
    wrong_obligation.obligation_identity = Some(qsl_replay::ObligationIdentity::new(
        node_key(&node),
        None,
        1,
    ));
    assert!(matches!(
        context.request_from_packet(
            wrong_obligation,
            &node,
            &occurrence,
            EqualityOperator::Equal,
            &[],
            100_000,
        ),
        Err(CompositeBuildError::PacketMismatch(
            CompositePacketMember::Obligation
        ))
    ));

    let mut missing_key = context
        .parity_packet(&node, &occurrence, EqualityOperator::Equal, &[], 100_000)
        .expect("the checked claim supplies another packet");
    missing_key.occurrence_key = None;
    assert!(matches!(
        context.request_from_packet(
            missing_key,
            &node,
            &occurrence,
            EqualityOperator::Equal,
            &[],
            100_000,
        ),
        Err(CompositeBuildError::PacketAdmission(ref cause))
            if matches!(**cause, qsl_replay::WitnessRefusal::MissingMember("occurrence_key"))
    ));
}

/// Two independently selected occurrences of one structural-equality node each retain their
/// own admitted packet key, and swapping those complete packets refuses before request creation.
#[trace("FR-024-AC-44", "FR-024-AC-46", "TC-035")]
#[test]
fn tc_035_original_composite_packets_bind_distinct_real_occurrences() {
    let package = checked(REPEATED_SOURCE);
    let application = package
        .graph()
        .nodes
        .iter()
        .find(|node| {
            checked_operation_identity(&node.body) == Some("quire.op.structural.eq")
                && node.occurrences.len() >= 2
        })
        .expect("QSL emits one structural equality node with two actual occurrences");
    let node = application.node_id.clone();
    let first = application.occurrences[0].clone();
    let second = application.occurrences[1].clone();
    assert_ne!(first, second);
    assert_ne!(second.ordinal, 0);
    let context = OriginalCompositeEqContext::new(
        package,
        inputs(REPEATED_SOURCE),
        "repeated",
        content_identity(),
        CheckedPackageReadLimits::bounded(),
    )
    .expect("the repeated source selects one checked function");
    let first_packet = context
        .parity_packet(&node, &first, EqualityOperator::Equal, &[], 100_000)
        .expect("the first checked occurrence produces a packet");
    let second_packet = context
        .parity_packet(&node, &second, EqualityOperator::Equal, &[], 100_000)
        .expect("the second checked occurrence produces a packet");
    assert_ne!(first_packet.occurrence_key, second_packet.occurrence_key);
    for (packet, occurrence) in [(&first_packet, &first), (&second_packet, &second)] {
        let request = context
            .request_from_packet(
                packet.clone(),
                &node,
                occurrence,
                EqualityOperator::Equal,
                &[],
                100_000,
            )
            .expect("the matching independently produced packet is accepted");
        let readback = ReplayRequest::decode(request.wire().clone(), ReplayLimits::default())
            .expect("QSL reads the packet-backed occurrence");
        assert_eq!(
            readback
                .replay_target()
                .and_then(ReplayTarget::occurrence_key),
            packet.occurrence_key.as_ref()
        );
    }
    assert!(matches!(
        context.request_from_packet(
            second_packet,
            &node,
            &first,
            EqualityOperator::Equal,
            &[],
            100_000,
        ),
        Err(CompositeBuildError::PacketMismatch(
            CompositePacketMember::Occurrence
        ))
    ));
}

/// QSL owns body membership and refuses an admitted node from another function before Disagreed.
#[trace("FR-033-AC-1", "FR-033-AC-9", "FR-033-AC-12", "TC-048")]
#[test]
fn another_functions_node_reaches_binding_checked_qsl_refusal() {
    let package = checked(SOURCE);
    let (node, occurrence) = application(&package, "g");
    let request = context(package, "f")
        .request(&node, &occurrence, EqualityOperator::Equal, &[], 100_000)
        .unwrap();
    let obligation = parity_obligation(request.preimage()).unwrap();
    let report = request
        .settle_verified(VerifiedShadow {
            success_checks: 1,
            refinement: Refinement::Disagreed,
        })
        .unwrap();
    assert_eq!(report.sent().obligation, obligation);
    assert_eq!(report.report().claim(), report.sent());
    let settlement = report.settlement().unwrap();
    let VerifiedShadowResult::Refused(refusal) = settlement.result() else {
        panic!("QSL membership refusal: {:?}", settlement.result())
    };
    assert!(
        matches!(&**refusal, ReplayRefusal::ScalarIdentity(cause) if matches!(&**cause, ScalarIdentityMismatch::Function { node: refused, .. } if *refused == node_key(&node)))
    );
    assert_eq!(
        settlement.terminal_value(),
        TerminalValue::Inconclusive(InconclusiveCause::ReplayRefused(refusal.code()))
    );
}

/// An actual nonoperand Node bound survives CG's filter and QSL refuses it before Disagreed.
#[trace("FR-033-AC-9", "FR-033-AC-11", "FR-033-AC-12", "TC-048")]
#[test]
fn unrelated_node_bound_reaches_binding_checked_qsl_refusal() {
    let package = checked(SOURCE);
    let (node, occurrence) = application(&package, "f");
    let (other_node, other_occurrence) = application(&package, "g");
    let other_operands = package
        .composite_application_operands(&other_node, &other_occurrence, 100_000)
        .unwrap();
    let other_parameter = other_operands
        .operands
        .iter()
        .find(|operand| {
            matches!(
                &operand.domain,
                CheckedCompositeOperandDomain::Parameter { .. }
            )
        })
        .expect("g has a record parameter with an Integer field");
    let CheckedScalarOperandChild::GraphChild(child) = &other_parameter.child else {
        panic!("parameter graph child")
    };
    let key = DomainKey::Node {
        node: node_key(child),
        path: vec![0],
    };
    let mut bounds = parameter_bounds(&package, &node, &occurrence);
    assert!(bounds.iter().all(|bound| bound.domain() != &key));
    bounds.push(
        ProofBound::new(
            key.clone(),
            Some(DomainKind::Integer),
            FiniteBound::integer_range(Integer::from(0_i64), Integer::from(9_i64)).unwrap(),
        )
        .unwrap(),
    );
    let request = context(package, "f")
        .request(
            &node,
            &occurrence,
            EqualityOperator::Equal,
            &bounds,
            100_000,
        )
        .unwrap();
    let obligation = parity_obligation(request.preimage()).unwrap();
    let report = request
        .settle_verified(VerifiedShadow {
            success_checks: 1,
            refinement: Refinement::Disagreed,
        })
        .unwrap();
    assert_eq!(report.sent().obligation, obligation);
    assert_eq!(report.sent().harness_bounds, bounds);
    assert_eq!(report.report().claim(), report.sent());
    let settlement = report.settlement().unwrap();
    let VerifiedShadowResult::Refused(refusal) = settlement.result() else {
        panic!("QSL bound refusal: {:?}", settlement.result())
    };
    assert!(
        matches!(&**refusal, ReplayRefusal::ParityBound(cause) if matches!(&**cause, ParityBoundRefusal::HarnessUnknownKey { key: refused } if *refused == key))
    );
    assert_eq!(
        settlement.terminal_value(),
        TerminalValue::Inconclusive(InconclusiveCause::ReplayRefused(refusal.code()))
    );
}

/// The projection's zero work budget stops in IR without traversing a CG-owned membership rule.
#[trace("FR-033-AC-1", "FR-033-AC-11", "TC-048")]
#[test]
fn original_operand_projection_obeys_work_ceiling() {
    let package = checked(SOURCE);
    let (node, occurrence) = application(&package, "f");
    assert!(
        matches!(context(package, "f").request(&node, &occurrence, EqualityOperator::Equal, &[], 0), Err(CompositeBuildError::Operands(cause)) if matches!(*cause, CheckedCompositeOperandError::WorkLimit { limit: 0, consumed } if consumed > 0))
    );
}

/// Original non-default stage limits reach QSL recompilation without substituting defaults.
#[trace("FR-033-AC-1", "FR-033-AC-9", "TC-048")]
#[test]
fn original_stage_limit_stops_recompile_before_invocation() {
    let package = checked(SOURCE);
    let (node, occurrence) = application(&package, "f");
    let mut retained = inputs(SOURCE);
    retained.stage_limits.insert("s1.input_bytes".to_owned(), 1);
    let context = OriginalCompositeEqContext::new(
        package,
        retained,
        "f",
        content_identity(),
        CheckedPackageReadLimits::bounded(),
    )
    .unwrap();
    assert!(
        matches!(context.request(&node, &occurrence, EqualityOperator::Equal, &[], 100_000), Err(CompositeBuildError::Recompile(cause)) if matches!(*cause, qsl_replay::AdmittedRefusal::Resource(qsl_replay::AdmittedResource::StageLimit(_))))
    );
}

/// The public encoder rejects duplicate bounds within an authentic parameter position.
#[trace("FR-033-AC-11", "TC-048")]
#[test]
fn duplicate_positional_bound_refuses_without_an_obligation() {
    let package = checked(SOURCE);
    let (node, occurrence) = application(&package, "f");
    let mut bounds = parameter_bounds(&package, &node, &occurrence);
    bounds.push(bounds[0].clone());
    assert!(matches!(
        context(package, "f").request(
            &node,
            &occurrence,
            EqualityOperator::Equal,
            &bounds,
            100_000
        ),
        Err(CompositeBuildError::Identity(
            qsl_replay::IdentityEncodeError::DuplicateKey { .. }
        ))
    ));
}

/// An authentic imported root refuses until its original admitted dependencies can be retained.
#[trace("FR-033-AC-1", "FR-033-AC-9", "TC-048")]
#[test]
fn original_imported_context_refuses_before_invocation() {
    let source = SOURCE.replace("record Q", "import \"test/eq-library\" as d;\nrecord Q");
    let library = |comparison: &str| {
        let bytes = format!("language \"ix:native\" edition \"1-draft\";\nprofile v = \"quire.value.complete/v1\";\nfunction big using v(x: Int[0, 9]): Boolean pure {{ {comparison} }}\n").into_bytes();
        let source = LockedSource {
            authority: "test".to_owned(),
            identity: "eq-library.native".to_owned(),
            namespace: "git".to_owned(),
            revision: "fixture".to_owned(),
            bytes,
        };
        let compiled = compile_package(
            qsl_replay::CompileUnit {
                source: SourceIdentity::new("test", "eq-library.native", "git", "fixture"),
                package_name: "test/eq-library".to_owned(),
                path: "eq-library.native",
                bytes: &source.bytes,
            },
            [],
            &DependencyInput::default(),
            StageLimits::default(),
            ReplayLimits::default(),
        )
        .unwrap();
        let admitted = match CheckedPackageV2::read(
            compiled.bytes(),
            CheckedPackageReadLimits::bounded(),
            &evidence(),
        ) {
            CheckedPackageV2ReadResult::Admitted(package) => *package,
            other => panic!("genuine library admits: {other:?}"),
        };
        (
            DependencyLock {
                identity: "test/eq-library".to_owned(),
                source,
            },
            admitted,
        )
    };
    let (original_library, original_admitted) = library("x > 1");
    let dependencies = DependencyInput::new(vec![SuppliedLibrary {
        identity: original_library.identity.clone(),
        source: SourceIdentity::new("test", "eq-library.native", "git", "fixture"),
        path: "eq-library.native".to_owned(),
        bytes: original_library.source.bytes.clone(),
    }])
    .unwrap();
    let mut retained_evidence = evidence();
    retained_evidence
        .insert_dependency_package(original_library.identity.clone(), original_admitted)
        .expect("retain original equality library");
    let package = checked_with_dependencies(&source, &dependencies, &retained_evidence);
    let mut retained = inputs(&source);
    retained.dependencies.push(original_library);
    assert!(matches!(
        OriginalCompositeEqContext::new(
            package,
            retained,
            "f",
            content_identity(),
            CheckedPackageReadLimits::bounded(),
        ),
        Err(CompositeBuildError::ImportedContextUnsupported)
    ));
}

/// The immutable capability is exposed only by a successfully authenticated request.
#[trace("FR-018-AC-33", "TC-029")]
#[test]
fn original_request_retains_the_actual_compilation_context() {
    let package = checked(SOURCE);
    let (node, occurrence) = application(&package, "f");
    let expected_name = package.name().to_owned();
    let request = context(package, "f")
        .request(&node, &occurrence, EqualityOperator::Equal, &[], 100_000)
        .expect("authentic original request");
    assert_eq!(
        request.admitted_context().package_name().to_string(),
        expected_name
    );
}

/// A later IR re-read ceiling remains distinct from source compiler limits.
#[trace("FR-018-AC-35", "TC-029")]
#[test]
fn original_request_preserves_the_owning_reader_limit() {
    let package = checked(SOURCE);
    let (node, occurrence) = application(&package, "f");
    let mut read_limits = CheckedPackageReadLimits::bounded();
    read_limits.bytes = 1;
    let original = OriginalCompositeEqContext::new(
        package,
        inputs(SOURCE),
        "f",
        content_identity(),
        read_limits,
    )
    .expect("retaining already-admitted inputs does not re-read them");
    assert!(matches!(
        original.request(&node, &occurrence, EqualityOperator::Equal, &[], 100_000),
        Err(CompositeBuildError::PackageReadLimit(_))
    ));
}

/// An invalid reader-relative stage request is not relabeled as ill-typed input.
#[trace("FR-018-AC-35", "TC-029")]
#[test]
fn original_request_preserves_the_above_reader_resource_refusal() {
    let package = checked(SOURCE);
    let (node, occurrence) = application(&package, "f");
    let mut retained = inputs(SOURCE);
    retained
        .stage_limits
        .insert("s1.input_bytes".to_owned(), u64::MAX);
    let original = OriginalCompositeEqContext::new(
        package,
        retained,
        "f",
        content_identity(),
        CheckedPackageReadLimits::bounded(),
    )
    .expect("retain authentic source before request-stage validation");
    assert!(
        matches!(original.request(&node, &occurrence, EqualityOperator::Equal, &[], 100_000),
        Err(CompositeBuildError::Recompile(cause))
            if matches!(*cause, qsl_replay::AdmittedRefusal::Resource(qsl_replay::AdmittedResource::AboveReader(_))))
    );
}

/// Reusing an admitted context preserves all publicly observable construction data.
/// This is native construction evidence, not supplied backend or verified-shadow evidence.
#[trace("FR-033-AC-1", "TC-048")]
#[test]
fn retained_context_kr_families_preserve_complete_request_outputs_and_refused_siblings() {
    for unrelated in [4, 8, 16] {
        let mut source = SOURCE.to_owned();
        for index in 0..unrelated {
            source.push_str(&format!(
                "function unrelated{index} using v(a: Integer): Integer pure {{ a }}\n"
            ));
        }
        let mut expected_across_request_counts = None;
        for requests in [2, 4, 8] {
            let package = checked(&source);
            let graph_nodes = package.graph().nodes.len();
            let original_package_name = package.name().to_owned();
            let (equal_node, equal_occurrence) = application(&package, "f");
            let (not_equal_node, not_equal_occurrence) =
                application_with_operator(&package, "n", "quire.op.structural.ne");
            let retained = OriginalCompositeEqContext::new(
                package,
                inputs(&source),
                "f",
                content_identity(),
                CheckedPackageReadLimits::bounded(),
            )
            .expect("authentic source supplies one retained context");
            let mut expected_in_context = None;
            for _ in 0..requests {
                let equal = retained
                    .request(
                        &equal_node,
                        &equal_occurrence,
                        EqualityOperator::Equal,
                        &[],
                        100_000,
                    )
                    .expect("equal request from the original admitted package");
                let not_equal = retained
                    .request(
                        &not_equal_node,
                        &not_equal_occurrence,
                        EqualityOperator::NotEqual,
                        &[],
                        100_000,
                    )
                    .expect("not-equal request retains its original operation");
                for request in [&equal, &not_equal] {
                    assert_eq!(
                        request.admitted_context().package_name().to_string(),
                        original_package_name
                    );
                }
                let refusal = retained
                    .request(
                        &equal_node,
                        &equal_occurrence,
                        EqualityOperator::NotEqual,
                        &[],
                        100_000,
                    )
                    .expect_err(
                        "a wrong operator must not inherit a healthy sibling's disposition",
                    );
                assert!(matches!(
                    refusal,
                    CompositeBuildError::OperatorMismatch {
                        requested: EqualityOperator::NotEqual,
                        actual: EqualityOperator::Equal
                    }
                ));
                // Typed equality checks include every positional operand and consumed_work.
                // Debug additionally retains the private claim/limits in the native snapshot;
                // it is neither wire serialization nor an assertion of solver origin.
                let snapshot = (
                    equal.preimage().clone(),
                    equal.operands().clone(),
                    not_equal.preimage().clone(),
                    not_equal.operands().clone(),
                    format!("{equal:?}"),
                    format!("{not_equal:?}"),
                );
                if let Some(expected) = &expected_in_context {
                    assert_eq!(&snapshot, expected);
                } else {
                    expected_in_context = Some(snapshot);
                }
            }
            if let Some(expected) = &expected_across_request_counts {
                assert_eq!(&expected_in_context, expected);
            } else {
                expected_across_request_counts = Some(expected_in_context);
            }
            println!("retained composite baseline K={requests} R={unrelated} G={graph_nodes}: Eq/Ne outputs, original package identities and typed refused siblings preserved");
        }
    }
}
