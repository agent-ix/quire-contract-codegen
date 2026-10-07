// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Constructor and public facade controls, without a claim of generated backend verification.

use std::collections::BTreeMap;

use qsl_replay::{
    compile_package, parity_obligation, DependencyInput, DigestDomain, DigestRecord, Domain,
    DomainKey, FiniteBound, InconclusiveCause, Integer, OperandIdentity, ParityBoundRefusal,
    ProofBound, Refinement, ReplayLimits, ReplayRefusal, ScalarIdentityMismatch, ScalarLimits,
    SourceIdentity, StageLimits, SuppliedLibrary, TerminalValue, VerifiedShadow,
    VerifiedShadowResult, WireNodeId,
};
use quire_contract_codegen::{
    CompositeBuildError, DependencyLock, LockedSource, OriginalCompositeEqContext, ReplayInputs,
};
use quire_contract_model::{
    CheckedCompositeOperandDomain, CheckedCompositeOperandError, CheckedNodeId, CheckedOccurrence,
    CheckedPackageEvidence, CheckedPackageReadLimits, CheckedPackageV2, CheckedPackageV2ReadResult,
    CheckedScalarOperandChild,
};

const SOURCE: &str = "language \"ix:native\" edition \"1-draft\";\n\
    profile v = \"quire.value.complete/v1\";\n\
    record Q { x: Integer; }\n\
    function f using v(a: Sequence<Int[0, 9]>[0, 3], b: Sequence<Int[0, 9]>[0, 3]): Boolean pure { a = b }\n\
    function g using v(a: Q): Boolean pure { a = Q { x: 1 } }\n\
    function h using v(): Boolean pure { Q { x: 1 } = Q { x: 2 } }\n\
    function same using v(a: Q): Boolean pure { a = a }\n";

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
        SourceIdentity::new("test", "original-eq.native", "git", "fixture"),
        "original-eq.native",
        source.as_bytes(),
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

fn application(package: &CheckedPackageV2, function: &str) -> (CheckedNodeId, CheckedOccurrence) {
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
            node.body
                .pointer("/operation/identity")
                .and_then(serde_json::Value::as_str)
                == Some("quire.op.structural.eq")
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
                Some(WireNodeId::from_hex(&child.digest).unwrap())
            } else {
                None
            }
        })
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .flat_map(|node| {
            [
                ProofBound {
                    domain: DomainKey::Node { node, path: vec![] },
                    bound: FiniteBound::cardinality(3),
                },
                ProofBound {
                    domain: DomainKey::Node {
                        node,
                        path: vec![0],
                    },
                    bound: FiniteBound::integer_range(Integer::from(0_i64), Integer::from(9_i64))
                        .unwrap(),
                },
            ]
        })
        .collect()
}

/// Original parameter/parameter construction reaches a genuine QSL verified report and converter.
/// This is a constructor control with supplied verified evidence, not a generated-family proof.
/// Trace: FR-033-AC-1, FR-033-AC-9, FR-033-AC-11, TC-048
#[test]
fn original_parameter_operands_build_positional_identity_and_public_verified_report() {
    let package = checked(SOURCE);
    let (node, occurrence) = application(&package, "f");
    let bounds = parameter_bounds(&package, &node, &occurrence);
    let mut supplied_bounds = bounds.clone();
    supplied_bounds.push(ProofBound {
        domain: DomainKey::Population {
            member_type: WireNodeId::from_hex(&node.digest).unwrap(),
            ordinal: 0,
        },
        bound: FiniteBound::cardinality(7),
    });
    let original = package
        .composite_application_operands(&node, &occurrence, 100_000)
        .unwrap();
    let request = context(package, "f")
        .request(&node, &occurrence, &supplied_bounds, 100_000)
        .unwrap();
    assert_eq!(request.operands(), &original);
    assert_eq!(request.preimage().arguments.len(), 2);
    for (argument, operand) in request.preimage().arguments.iter().zip(&original.operands) {
        let CheckedScalarOperandChild::GraphChild(child) = &operand.child else {
            panic!("graph child")
        };
        assert_eq!(
            argument.identity,
            OperandIdentity::GraphChild(WireNodeId::from_hex(&child.digest).unwrap())
        );
        let Domain::Bounds(entries) = &argument.domain else {
            panic!("composite bounds")
        };
        assert_eq!(entries.entries().len(), 2);
        assert!(entries.entries().iter().all(|entry| matches!(entry.domain, DomainKey::Node { node, .. } if node.to_string() == child.digest.as_ref())));
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
/// Trace: FR-033-AC-1, FR-033-AC-11, TC-048
#[test]
fn original_literal_operands_retain_graph_children_and_empty_bounds() {
    for (function, literals) in [("g", 1), ("h", 2)] {
        let package = checked(SOURCE);
        let (node, occurrence) = application(&package, function);
        let original = context(package, function);
        let baseline = original.request(&node, &occurrence, &[], 100_000).unwrap();
        let bounds = baseline
            .operands()
            .operands
            .iter()
            .map(|operand| {
                let CheckedScalarOperandChild::GraphChild(child) = &operand.child else {
                    panic!("graph child")
                };
                ProofBound {
                    domain: DomainKey::Node {
                        node: WireNodeId::from_hex(&child.digest).unwrap(),
                        path: vec![0],
                    },
                    bound: FiniteBound::integer_range(Integer::from(0_i64), Integer::from(9_i64))
                        .unwrap(),
                }
            })
            .collect::<Vec<_>>();
        let request = original
            .request(&node, &occurrence, &bounds, 100_000)
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
                    OperandIdentity::GraphChild(WireNodeId::from_hex(&child.digest).unwrap())
                );
                let Domain::Bounds(entries) = &argument.domain else {
                    panic!("literal bounds")
                };
                assert!(entries.entries().is_empty());
            } else {
                let OperandIdentity::GraphChild(child) = argument.identity else {
                    panic!("parameter graph child")
                };
                let Domain::Bounds(entries) = &argument.domain else {
                    panic!("parameter bounds")
                };
                assert_eq!(entries.entries().len(), 1);
                assert!(entries.entries().iter().all(
                    |entry| matches!(entry.domain, DomainKey::Node { node, .. } if node == child)
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

/// Repeated parameter positions remain distinct O-09 arguments.
/// Trace: FR-033-AC-11, TC-048
#[test]
fn self_comparison_keeps_two_positional_arguments() {
    let package = checked(SOURCE);
    let (node, occurrence) = application(&package, "same");
    let request = context(package, "same")
        .request(&node, &occurrence, &[], 100_000)
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
/// Trace: FR-033-AC-1, FR-033-AC-9, TC-048
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
/// Trace: FR-033-AC-1, FR-033-AC-9, TC-048
#[test]
fn different_semantic_package_refuses_before_invocation() {
    let original = checked(SOURCE);
    let changed_source = SOURCE.replace("Int[0, 9]", "Int[0, 8]");
    let changed = checked_with_extra_source(&changed_source, &original.lock().sources[0]);
    assert_ne!(changed.package_id(), original.package_id());
    let (node, occurrence) = application(&changed, "f");
    assert!(matches!(
        context(changed, "f").request(&node, &occurrence, &[], 100_000),
        Err(CompositeBuildError::PackageMismatch)
    ));
}

/// Extra retained dependency selection and altered admitted source-lock context refuse distinctly.
/// Trace: FR-033-AC-1, FR-033-AC-9, TC-048
#[test]
fn retained_selection_and_recompiled_context_mismatch_refuse() {
    let package = checked(SOURCE);
    let mut retained = inputs(SOURCE);
    retained.dependencies.push(DependencyLock {
        identity: "test/extra".to_owned(),
        package_id: compiled(SOURCE, &DependencyInput::default()).package_id(),
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
    assert_eq!(changed.package_id(), checked(SOURCE).package_id());
    let (node, occurrence) = application(&changed, "f");
    assert!(matches!(
        context(changed, "f").request(&node, &occurrence, &[], 100_000),
        Err(CompositeBuildError::ContextMismatch)
    ));
}

/// Absent nodes/occurrences refuse structurally before evidence is accepted; no disagreement is supplied.
/// Trace: FR-033-AC-1, FR-033-AC-9, TC-048
#[test]
fn wrong_node_and_occurrence_refuse_before_invocation() {
    let package = checked(SOURCE);
    let (node, occurrence) = application(&package, "f");
    let context = context(package, "f");
    let mut absent = node.clone();
    absent.digest = "ff".repeat(32).into_boxed_str();
    assert!(
        matches!(context.request(&absent, &occurrence, &[], 100_000), Err(CompositeBuildError::Operands(cause)) if matches!(*cause, CheckedCompositeOperandError::UnknownNode { .. }))
    );
    let mut wrong_occurrence = occurrence.clone();
    wrong_occurrence.ordinal = u64::MAX;
    assert!(
        matches!(context.request(&node, &wrong_occurrence, &[], 100_000), Err(CompositeBuildError::Operands(cause)) if matches!(*cause, CheckedCompositeOperandError::MissingOccurrence { .. }))
    );
}

/// QSL owns body membership and refuses an admitted node from another function before Disagreed.
/// Trace: FR-033-AC-1, FR-033-AC-9, FR-033-AC-12, TC-048
#[test]
fn another_functions_node_reaches_binding_checked_qsl_refusal() {
    let package = checked(SOURCE);
    let (node, occurrence) = application(&package, "g");
    let request = context(package, "f")
        .request(&node, &occurrence, &[], 100_000)
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
        matches!(&**refusal, ReplayRefusal::ScalarIdentity(cause) if matches!(&**cause, ScalarIdentityMismatch::Function { node: refused, .. } if *refused == WireNodeId::from_hex(&node.digest).unwrap()))
    );
    assert_eq!(
        settlement.terminal_value(),
        TerminalValue::Inconclusive(InconclusiveCause::ReplayRefused(refusal.code()))
    );
}

/// An actual nonoperand Node bound survives CG's filter and QSL refuses it before Disagreed.
/// Trace: FR-033-AC-9, FR-033-AC-11, FR-033-AC-12, TC-048
#[test]
fn unrelated_node_bound_reaches_binding_checked_qsl_refusal() {
    let package = checked(SOURCE);
    let (node, occurrence) = application(&package, "f");
    let key = DomainKey::Node {
        node: WireNodeId::from_hex(&node.digest).unwrap(),
        path: vec![],
    };
    let mut bounds = parameter_bounds(&package, &node, &occurrence);
    bounds.push(ProofBound {
        domain: key.clone(),
        bound: FiniteBound::cardinality(3),
    });
    let request = context(package, "f")
        .request(&node, &occurrence, &bounds, 100_000)
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
/// Trace: FR-033-AC-1, FR-033-AC-11, TC-048
#[test]
fn original_operand_projection_obeys_work_ceiling() {
    let package = checked(SOURCE);
    let (node, occurrence) = application(&package, "f");
    assert!(
        matches!(context(package, "f").request(&node, &occurrence, &[], 0), Err(CompositeBuildError::Operands(cause)) if matches!(*cause, CheckedCompositeOperandError::WorkLimit { limit: 0, consumed } if consumed > 0))
    );
}

/// Original non-default stage limits reach QSL recompilation without substituting defaults.
/// Trace: FR-033-AC-1, FR-033-AC-9, TC-048
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
        matches!(context.request(&node, &occurrence, &[], 100_000), Err(CompositeBuildError::Recompile(cause)) if matches!(*cause, qsl_replay::ReplayRefusal::Recompile(_)))
    );
}

/// The public encoder rejects duplicate bounds within an authentic parameter position.
/// Trace: FR-033-AC-11, TC-048
#[test]
fn duplicate_positional_bound_refuses_without_an_obligation() {
    let package = checked(SOURCE);
    let (node, occurrence) = application(&package, "f");
    let mut bounds = parameter_bounds(&package, &node, &occurrence);
    bounds.push(bounds[0].clone());
    assert!(matches!(
        context(package, "f").request(&node, &occurrence, &bounds, 100_000),
        Err(CompositeBuildError::Identity(
            qsl_replay::IdentityEncodeError::DuplicateKey { .. }
        ))
    ));
}

/// An authentic imported root refuses until its original admitted dependencies can be retained.
/// Trace: FR-033-AC-1, FR-033-AC-9, TC-048
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
            SourceIdentity::new("test", "eq-library.native", "git", "fixture"),
            "eq-library.native",
            &source.bytes,
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
                package_id: compiled.package_id(),
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
        .insert_dependency_package(original_library.identity.clone(), original_admitted);
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
