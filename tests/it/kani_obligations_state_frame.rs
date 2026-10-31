//! FR-015 (IR-412): one `state_clause` postcondition over one integer field becomes an
//! operation-contract Kani harness and a frame-effect Kani harness, both scoped to the clause's
//! operation.
//!
//! The package is hand-built in the shape QSL lowers `post Deposit ... { self.balance >=
//! pre(self.balance) }` to: a `postcondition` `state_clause` whose condition is one
//! `quire.op.integer.ge` over `record.project(deref(self), balance)` and `pre` of the same read,
//! anchored on the `deposit` operation whose frame modifies `balance` only. The framed object
//! declares its own fields, so IR admits the field reads without a domain package.
//!
//! The default lane measures the generated source, identity and refusals. The `kani` lane
//! (`#[ignore]` here, run serially by `make kani`, whose `kani_obligations` filter this module's
//! name matches) proves the harnesses with the installed backend: a healthy subject verifies,
//! each seeded defect is falsified for the intended property, and regenerating the frame from a
//! mutated package turns a green proof red.

use quire_contract_codegen as scalar_fixture_types;
#[path = "../state_frame_support/model.rs"]
pub(crate) mod model;
#[path = "../state_frame_support/native_twin.rs"]
pub(crate) mod native_twin;
#[path = "../exact_scalar_support/package.rs"]
#[allow(clippy::duplicate_mod)]
mod package;
#[path = "../state_frame_support/subject.rs"]
// The Kani crates run every subject variant; the test binary executes only some natively.
#[allow(dead_code)]
pub(crate) mod subject;

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::PathBuf,
    time::Duration,
};

use ix_trace_rs::trace;
use native_twin::{
    playback_text, Invocation, Run, Tamper, Twin, CLAUSES, INVOCATION_DOCUMENT, PRE_DOCUMENT,
};
use package::{
    application, code_id, key, literal, member, op, op_full, parameter_body, reference, Bound,
    PackageBuilder, FUNCTION, T_BOOLEAN, T_INTEGER, T_TEXT,
};
use qsl_replay::{
    CallSiteRefusal, DisagreementCause, FrameChange, FrameIdentityMismatch, InconclusiveCause,
    ObligationIdentity, OperationSite, ReplayRefusal, ReplayResult, ReplaySource, TerminalValue,
    Verdict, WitnessSettlement,
};
use quire_contract_codegen::{
    derive_v2_clause_descriptor, generate_v2_clause_claim_map, generate_v2_clause_native_oracle,
    generate_v2_clause_oracle,
};
use quire_contract_codegen::{
    execute_kani_obligation, generate_state_frame_obligations, negotiate_kani_native_obligations,
    negotiate_kani_obligations, BoundNotResolvedCause, FrameReplay, FrameReplayError,
    InvalidObligationItem, KaniExecutionRequest, KaniInstallation, KaniObligationError,
    KaniObligationOutcome, KaniObligationRequest, KaniRunOutcome, ModuleSymbol,
    ObligationDisposition, ObligationItem, ObligationKind, ObligationRecord, PreStateFault,
    ProofDependencyKind, ProofDependencyRequest, ProofDependencyState, ProofExecutionState,
    ProofReadiness, ScopeMember, StateComparison, StateFieldDomain, StateFrameHarness,
    StateFrameIdentity, StateFrameLoweringRefusal, StateFrameObligations, StateFrameProperty,
    StateFrameRefusal, StateFrameRequest, StateFrameRole, UnsupportedFrameEffect,
    UnsupportedObligation, MAX_OBLIGATION_ITEMS, MAX_OBLIGATION_UNWIND,
};
use quire_contract_codegen::{HarnessSymbol, StateFrameRecordError};
use quire_contract_model::{
    CheckedMemberType, CheckedNodeId, CheckedPackageReadLimits, CheckedPackageV2, CheckedTerm,
};
use quire_semantic_value::catalog::Category;
use serde_json::{json, Value};

const OBJECT: u32 = 4001;
const REFERENCE: u32 = 4002;
const SELF: u32 = 4003;
const FRAME: u32 = 4004;
const ANCHOR: u32 = 4005;
const DEREF: u32 = 4006;
const OTHER: u32 = 4011;
const OTHER_OBJECT: u32 = 4012;
const RESULT: u32 = 4014;

const STATE_FIELDS: [&str; 2] = [model::FIELDS[0].0, model::FIELDS[1].0];
const STATE_PATH: &str = "crate::subject::Account";
const SUBJECT_PATH: &str = "crate::subject::deposit";
const SUBJECT_SOURCE: &str = include_str!("../state_frame_support/subject.rs");

#[trace("FR-014-AC-38")]
#[test]
fn qsl_emitted_state_clause_derives_a_confirmed_field_backed_oracle() {
    let (package, clause) = Twin::new().emitted_package("BalanceNeverDrops");
    let generated = generate_v2_clause_oracle(&package, &clause)
        .expect("the admitted state clause condition is supported");
    assert_eq!(generated.descriptor.node_id, clause);
    assert_eq!(
        generated.provenance,
        quire_contract_codegen::OperationProvenance::IrConfirmed
    );
    assert!(generated.source.contains("pub fn oracle_"));
    assert!(generated.source.contains("order_numbers"));
    assert!(generated.source.contains("pre_balance"));
    assert!(generated.source.contains("current_balance"));
    assert_eq!(generated.descriptor.inputs.len(), 2);
    assert!(generated
        .descriptor
        .inputs
        .iter()
        .all(|input| input.lower == 0 && input.upper == 1000));
    assert_eq!(generated.source_map.len(), 1);
    let entry = &generated.source_map[0];
    assert_eq!(entry.node_id, clause);
    assert_eq!(entry.role, "oracle_evaluation");
    assert_eq!(entry.expected_consequents, Some(0));
    let line = generated
        .source
        .lines()
        .nth((entry.probe.line - 1) as usize)
        .unwrap();
    let start = (entry.probe.start_column - 1) as usize;
    let end = (entry.probe.end_column - 1) as usize;
    assert_eq!(&line[start..end], "pub");

    let scratch = super::scratch_crate::TemporaryDirectory::new("quire-v2-clause-oracle");
    fs::write(scratch.path.join("src/lib.rs"), format!(
        "{}\n#[test]\nfn clause_values() {{\n    use rt::{{Meter, ScalarLimits}};\n    let limits = ScalarLimits {{ integer_bits: 1000, decimal_digits: 1000, scale_expansion: 1000, text_input_bytes: 1000, text_scalars: 1000, normalized_scalars: 1000, unit_edges: 1000, value_occurrences: 1000, work_units: 1000, result_units: 1000 }};\n    let mut meter = Meter::new(limits);\n    let current = rt::Integer::from(5i64);\n    let pre = rt::Integer::from(4i64);\n    assert!(matches!(oracle_clause(&current, &pre, &mut meter), Ok(rt::Outcome::Completed(true))));\n    let mut meter = Meter::new(limits);\n    assert!(matches!(oracle_clause(&pre, &current, &mut meter), Ok(rt::Outcome::Completed(false))));\n    let lower = rt::Integer::from(0i64);\n    let upper = rt::Integer::from(1000i64);\n    let mut meter = Meter::new(limits);\n    assert!(matches!(oracle_clause(&lower, &upper, &mut meter), Ok(rt::Outcome::Completed(false))));\n    let mut meter = Meter::new(limits);\n    assert!(matches!(oracle_clause(&upper, &lower, &mut meter), Ok(rt::Outcome::Completed(true))));\n    let mut meter = Meter::new(ScalarLimits {{ work_units: 0, ..limits }});\n    assert!(matches!(oracle_clause(&current, &pre, &mut meter), Ok(rt::Outcome::Incomplete(_))));\n}}\n",
        generated.source
    )).unwrap();
    super::scratch_crate::write_manifest(
        &scratch.path,
        &format!(
            "[package]\nname = \"generated-v2-clause\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[dependencies]\n{}\nquire-exact = {{ {} }}\n\n[workspace]\n",
            super::scratch_crate::runtime_dependency(&["exact"]),
            quire_contract_codegen::EXACT_DEPENDENCY_SOURCE
        ),
    );
    let output = std::process::Command::new("cargo")
        .args(["test", "--offline", "--quiet"])
        .env("CARGO_TARGET_DIR", scratch.path.join("target"))
        .env("RUSTFLAGS", "-Dwarnings")
        .current_dir(&scratch.path)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "generated clause failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[trace("FR-014-AC-38")]
#[test]
fn clause_claim_map_carries_the_derived_descriptor_and_typed_disposition() {
    let (package, clause) = Twin::new().emitted_package("BalanceNeverDrops");
    let clause_body = &package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_id == clause)
        .unwrap()
        .body;
    let CheckedTerm::Application { arguments, .. } = clause_body else {
        panic!("state clause is an application");
    };
    let CheckedTerm::Reference {
        target: condition_node,
    } = &arguments[2]
    else {
        panic!("state clause names a condition node");
    };
    let condition_node = condition_node.clone();
    let map = generate_v2_clause_claim_map(&package, &clause);
    assert_eq!(map.source.name.as_ref(), package.name());
    assert_eq!(&map.source.lock, package.lock());
    assert_eq!(map.items.len(), 1);
    assert_eq!(map.items[0].node_id, clause);
    let quire_contract_codegen::ClaimDisposition::Generated(generated) = &map.items[0].result
    else {
        panic!("the clause claim was not generated");
    };
    assert_eq!(generated.descriptor.node_id, clause);
    assert_eq!(generated.descriptor.condition_node_id, Some(condition_node));
    assert_eq!(
        generated.descriptor.operation_identities,
        ["quire.op.integer.ge"]
    );
    assert_eq!(
        derive_v2_clause_descriptor(&package, &clause),
        Ok(generated.descriptor.clone())
    );
    assert_eq!(
        generated.provenance,
        quire_contract_codegen::OperationProvenance::IrConfirmed
    );
    let encoded = serde_json::to_value(&map).unwrap();
    assert_eq!(
        encoded["items"][0]["node_id"],
        serde_json::to_value(&clause).unwrap()
    );
    assert!(!encoded.to_string().contains("pub fn oracle_clause"));
    let non_clause = package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_tag.as_ref() == "expression")
        .unwrap();
    let refused = generate_v2_clause_claim_map(&package, &non_clause.node_id);
    assert_eq!(refused.items.len(), 1);
    assert_eq!(refused.items[0].node_id, non_clause.node_id);
    assert!(matches!(
        refused.items[0].result,
        quire_contract_codegen::ClaimDisposition::Refused {
            refusal: quire_contract_codegen::V2ClauseRefusal::NotClause
        }
    ));
}

#[trace("FR-014-AC-38")]
#[test]
fn admitted_clause_refuses_non_clause_node() {
    let twin = Twin::new();
    let (package, _) = twin.emitted_package("BalanceNeverDrops");
    let non_clause = package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_tag.as_ref() == "expression")
        .expect("an expression node");
    assert_eq!(
        generate_v2_clause_oracle(&package, &non_clause.node_id),
        Err(quire_contract_codegen::V2ClauseRefusal::NotClause),
    );
}

#[trace("FR-014-AC-38", "TC-024")]
#[test]
fn clause_oracle_refuses_a_clause_from_another_admitted_package() {
    let admitted = fixture(&Shape::HEALTHY);
    let foreign = fixture(&Shape {
        variant: 146,
        condition: Condition::InlineTrue,
        ..Shape::HEALTHY
    });
    assert!(!admitted
        .package
        .graph()
        .nodes
        .iter()
        .any(|node| node.node_id == foreign.clause));
    assert_eq!(
        generate_v2_clause_oracle(&admitted.package, &foreign.clause),
        Err(quire_contract_codegen::V2ClauseRefusal::NotClause)
    );
}

#[trace("FR-014-AC-38")]
#[test]
fn admitted_inline_clause_keeps_the_clause_identity() {
    let admitted = fixture(&Shape {
        variant: 146,
        condition: Condition::InlineTrue,
        ..Shape::HEALTHY
    });
    let generated = generate_v2_clause_oracle(&admitted.package, &admitted.clause)
        .expect("the admitted inline Boolean condition");
    assert_eq!(generated.descriptor.node_id, admitted.clause);
    assert_eq!(
        generated.provenance,
        quire_contract_codegen::OperationProvenance::IrConfirmed
    );
    assert!(generated.descriptor.inputs.is_empty());
    assert_eq!(generated.source_map[0].expected_consequents, Some(0));
    assert!(generated.source.contains("Ok(true)"));
    let scratch = super::scratch_crate::TemporaryDirectory::new("quire-v2-inline-clause");
    fs::write(
        scratch.path.join("src/lib.rs"),
        format!(
            "{}\n#[test]\nfn inline_condition() {{\n    let mut meter = rt::Meter::new(rt::ScalarLimits {{ integer_bits: 1, decimal_digits: 1, scale_expansion: 1, text_input_bytes: 1, text_scalars: 1, normalized_scalars: 1, unit_edges: 1, value_occurrences: 1, work_units: 1, result_units: 1 }});\n    assert!(matches!(oracle_clause(&mut meter), Ok(rt::Outcome::Completed(true))));\n}}\n",
            generated.source
        ),
    )
    .unwrap();
    super::scratch_crate::write_manifest(
        &scratch.path,
        &format!(
            "[package]\nname = \"generated-v2-inline-clause\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[dependencies]\n{}\nquire-exact = {{ {} }}\n\n[workspace]\n",
            super::scratch_crate::runtime_dependency(&["exact"]),
            quire_contract_codegen::EXACT_DEPENDENCY_SOURCE
        ),
    );
    let output = std::process::Command::new("cargo")
        .args(["test", "--offline", "--quiet"])
        .env("CARGO_TARGET_DIR", scratch.path.join("target"))
        .env("RUSTFLAGS", "-Dwarnings")
        .current_dir(&scratch.path)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "inline clause failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[trace("FR-014-AC-38")]
#[test]
fn qsl_emitted_pre_of_an_expression_is_refused() {
    let (package, clause) = Twin::with_balance_condition("self.balance >= pre(self.balance + 1)")
        .emitted_package("BalanceNeverDrops");
    assert_eq!(
        generate_v2_clause_oracle(&package, &clause),
        Err(quire_contract_codegen::V2ClauseRefusal::UnsupportedCondition),
    );
}

#[trace("FR-014-AC-38")]
#[test]
fn qsl_emitted_negated_clause_composes_the_boolean_oracle() {
    let (package, clause) =
        Twin::with_negated_balance_clause().emitted_package("BalanceNeverDrops");
    let generated = generate_v2_clause_oracle(&package, &clause)
        .expect("a QSL emitted negated clause condition");
    assert_eq!(generated.descriptor.node_id, clause);
    assert!(generated.source.contains("evaluate_boolean"));
    assert!(generated.source.contains("order_numbers"));
    assert_eq!(generated.source_map[0].expected_consequents, Some(0));
}

#[trace("FR-014-AC-38")]
#[test]
fn qsl_model_field_range_is_the_clause_input_range() {
    let (package, clause) =
        Twin::with_field_ranges((-5, 10), (50, 60)).emitted_package("BalanceNeverDrops");
    let generated = generate_v2_clause_oracle(&package, &clause)
        .expect("a clause over the selected balance field");
    assert_eq!(generated.descriptor.inputs.len(), 2);
    assert!(generated
        .descriptor
        .inputs
        .iter()
        .all(|input| (input.lower, input.upper) == (-5, 10)));
}

#[trace("FR-014-AC-38")]
#[test]
fn qsl_bounded_operation_parameter_becomes_a_clause_input() {
    let (package, clause) =
        Twin::with_bounded_other_condition().emitted_package("BalanceNeverDrops");
    let generated = generate_v2_clause_oracle(&package, &clause)
        .expect("a bounded operation parameter is supported");
    assert_eq!(generated.descriptor.node_id, clause);
    assert_eq!(generated.descriptor.inputs.len(), 2);
    assert!(generated
        .descriptor
        .inputs
        .iter()
        .any(|input| input.name == "parameter_other" && (input.lower, input.upper) == (0, 1000)));
    assert!(generated
        .descriptor
        .inputs
        .iter()
        .any(|input| input.name == "current_balance" && (input.lower, input.upper) == (0, 1000)));
    let scratch = super::scratch_crate::TemporaryDirectory::new("quire-v2-param-clause");
    fs::write(
        scratch.path.join("src/lib.rs"),
        format!(
            "{}\n#[test]\nfn parameter_values() {{\n    let five = rt::Integer::from(5i64);\n    let four = rt::Integer::from(4i64);\n    let limits = rt::ScalarLimits {{ integer_bits: 1000, decimal_digits: 1000, scale_expansion: 1000, text_input_bytes: 1000, text_scalars: 1000, normalized_scalars: 1000, unit_edges: 1000, value_occurrences: 1000, work_units: 1000, result_units: 1000 }};\n    let mut meter = rt::Meter::new(limits);\n    assert!(matches!(oracle_clause(&five, &four, &mut meter), Ok(rt::Outcome::Completed(true))));\n    let mut meter = rt::Meter::new(limits);\n    assert!(matches!(oracle_clause(&four, &five, &mut meter), Ok(rt::Outcome::Completed(false))));\n}}\n",
            generated.source
        ),
    )
    .unwrap();
    super::scratch_crate::write_manifest(
        &scratch.path,
        &format!(
            "[package]\nname = \"generated-v2-param-clause\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[dependencies]\n{}\nquire-exact = {{ {} }}\n\n[workspace]\n",
            super::scratch_crate::runtime_dependency(&["exact"]),
            quire_contract_codegen::EXACT_DEPENDENCY_SOURCE
        ),
    );
    let output = std::process::Command::new("cargo")
        .args(["test", "--offline", "--quiet"])
        .env("CARGO_TARGET_DIR", scratch.path.join("target"))
        .env("RUSTFLAGS", "-Dwarnings")
        .current_dir(&scratch.path)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "parameter clause failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

/// A V2 precondition is selected by its checked clause node, draws the operation parameter
/// inside the model's bound, and emits its own cover-only harness with the exact clause oracle.
#[trace("TC-025")]
#[test]
fn qsl_parameter_precondition_is_a_separate_v2_cover_obligation() {
    let (package, clause) =
        Twin::with_bounded_other_precondition().emitted_package("OtherBelowLimit");
    let oracle = generate_v2_clause_oracle(&package, &clause).expect("QSL precondition oracle");
    assert_eq!(oracle.descriptor.inputs.len(), 1);
    assert_eq!(oracle.descriptor.inputs[0].name, "parameter_other");
    assert_eq!(
        (
            oracle.descriptor.inputs[0].lower,
            oracle.descriptor.inputs[0].upper
        ),
        (0, 1000)
    );

    let outcome = negotiate(&[ObligationItem::V2Clause {
        package: &package,
        clause: &clause,
        dependencies: None,
    }]);
    let [record] = outcome.records() else {
        panic!("one selected precondition");
    };
    assert!(matches!(
        record.disposition,
        ObligationDisposition::Supported { .. }
    ));
    let KaniObligationOutcome::Emitted {
        v2_clause_harnesses,
        ..
    } = outcome
    else {
        panic!("the precondition is admitted");
    };
    let [harness] = v2_clause_harnesses.as_slice() else {
        panic!("one precondition harness");
    };
    assert_eq!(harness.identity.kind, ObligationKind::Precondition);
    assert_eq!(harness.rust.contents.matches("kani::cover!(").count(), 1);
    assert!(!harness.rust.contents.contains("assert!("));
    assert!(harness.rust.contents.contains("parameter_other >= 0"));
    assert!(harness.rust.contents.contains("parameter_other <= 1000"));
    assert!(harness.rust.contents.contains(&oracle.source));
}

#[trace("FR-029-AC-11", "TC-040")]
#[test]
fn ordinary_v2_clause_cannot_settle_as_a_collection_totality_failure() {
    let (package, clause) =
        Twin::with_bounded_other_precondition().emitted_package("OtherBelowLimit");
    let outcome = negotiate(&[ObligationItem::V2Clause {
        package: &package,
        clause: &clause,
        dependencies: None,
    }]);
    let KaniObligationOutcome::Emitted {
        v2_clause_harnesses,
        ..
    } = outcome
    else {
        panic!("ordinary V2 clause is emitted");
    };
    let [harness] = v2_clause_harnesses.as_slice() else {
        panic!("one ordinary V2 clause harness");
    };
    assert!(harness.identity.collection_precondition.is_none());
    let transcript = format!(
        "/// Test generated for harness `{}::{}`\n/// Check for `assertion`: \"ordinary clause\"\nlet concrete_vals: Vec<Vec<u8>> = vec![];",
        harness.identity.module_symbol.as_str(),
        harness.identity.harness_symbol.as_str(),
    );
    let playback = quire_contract_codegen::parse_assertion_playback(
        &transcript,
        harness.identity.harness_symbol.as_str(),
        harness.identity.module_symbol.as_str(),
        4096,
        0,
    )
    .expect("synthetic assertion parses");
    let error = quire_contract_codegen::settle_v2_collection_precondition_falsification(
        harness,
        &playback,
        quire_contract_codegen::WitnessDecodeLimits {
            bytes: 4096,
            values: 0,
        },
    )
    .expect_err("an ordinary clause cannot be treated as collection totality");
    assert_eq!(error.code, "cg_witness_not_collection_precondition");
}

/// A malformed declaration rejects the V2 request before any harness is exposed.
#[trace("FR-015-AC-22", "FR-015-AC-44", "TC-005", "TC-025")]
#[test]
fn v2_clause_census_refuses_invalid_declared_dependencies() {
    let (package, clause) =
        Twin::with_bounded_other_precondition().emitted_package("OtherBelowLimit");
    let required = ProofDependencyRequest {
        proof_id: "proof_a",
        kind: ProofDependencyKind::Required,
        state: ProofDependencyState::Passed,
        original_path: None,
        replacement_path: None,
    };
    let evaluate = |dependencies: &[ProofDependencyRequest<'_>]| {
        let item = [ObligationItem::V2Clause {
            package: &package,
            clause: &clause,
            dependencies: Some(dependencies),
        }];
        negotiate_kani_obligations(&KaniObligationRequest {
            ceilings: crate::common::proof_ceilings::proof_ceilings(),
            items: &item,
            subject_path: REQUEST_SUBJECT,
            unwind: 3,
        })
        .expect("request shape")
    };
    let invalid = [
        (
            vec![ProofDependencyRequest {
                proof_id: "",
                ..required
            }],
            InvalidObligationItem::EmptyProofDependencyIdentity { index: 0 },
        ),
        (
            vec![required, required],
            InvalidObligationItem::DuplicateProofDependencyIdentity {
                index: 1,
                first_index: 0,
            },
        ),
        (
            vec![ProofDependencyRequest {
                state: ProofDependencyState::Assumed,
                ..required
            }],
            InvalidObligationItem::InconsistentProofDependency { index: 0 },
        ),
        (
            vec![ProofDependencyRequest {
                original_path: Some("crate::predicate"),
                ..required
            }],
            InvalidObligationItem::InconsistentProofDependency { index: 0 },
        ),
        (
            vec![ProofDependencyRequest {
                kind: ProofDependencyKind::Assumed,
                state: ProofDependencyState::Assumed,
                original_path: Some("crate::predicate"),
                ..required
            }],
            InvalidObligationItem::NonRequiredProofDependency { index: 0 },
        ),
    ];
    for (dependencies, expected) in invalid {
        let KaniObligationOutcome::Rejected { records } = evaluate(&dependencies) else {
            panic!("invalid V2 census must expose no harness");
        };
        assert!(matches!(
            &records[0].disposition,
            ObligationDisposition::InvalidRequest { reason } if reason == &expected
        ));
    }
}

/// The normalized census is part of the sealed identity while generation has no run verdict.
#[trace("FR-015-AC-25", "FR-015-AC-45", "TC-005", "TC-025")]
#[test]
fn v2_clause_census_is_order_independent_and_records_readiness() {
    let (package, clause) =
        Twin::with_bounded_other_precondition().emitted_package("OtherBelowLimit");
    let dependency = |proof_id, state| ProofDependencyRequest {
        proof_id,
        kind: ProofDependencyKind::Required,
        state,
        original_path: None,
        replacement_path: None,
    };
    let evaluate = |dependencies: &[ProofDependencyRequest<'_>]| {
        let item = [ObligationItem::V2Clause {
            package: &package,
            clause: &clause,
            dependencies: Some(dependencies),
        }];
        let outcome = negotiate_kani_obligations(&KaniObligationRequest {
            ceilings: crate::common::proof_ceilings::proof_ceilings(),
            items: &item,
            subject_path: REQUEST_SUBJECT,
            unwind: 3,
        })
        .expect("request shape");
        let KaniObligationOutcome::Emitted {
            v2_clause_harnesses,
            ..
        } = outcome
        else {
            panic!("valid V2 census admits the clause");
        };
        let [harness] = v2_clause_harnesses.as_slice() else {
            panic!("one V2 clause harness");
        };
        harness.clone()
    };
    let passed = [
        dependency("proof_b", ProofDependencyState::Passed),
        dependency("proof_a", ProofDependencyState::Passed),
    ];
    let reversed = [passed[1], passed[0]];
    let ready = evaluate(&passed);
    let reordered = evaluate(&reversed);
    assert_eq!(ready, reordered);
    assert_eq!(ready.identity.readiness, ProofReadiness::Ready);
    assert_eq!(
        ready.identity.proof_execution_state,
        ProofExecutionState::NotRun
    );
    assert_eq!(ready.identity.dependencies[0].proof_id, "proof_a");
    assert_eq!(ready.identity.dependencies[1].proof_id, "proof_b");
    let persisted = |harness: &quire_contract_codegen::KaniV2ClauseObligationHarness| -> Value {
        serde_json::from_str(&harness.record.contents).expect("persisted V2 identity record")
    };
    let assert_census = |record: &Value, first_state: &str, readiness: &str| {
        assert_eq!(
            record["identity"]["dependencies"],
            json!([
                {"proofId": "proof_a", "kind": "required", "state": first_state, "sourceSite": null},
                {"proofId": "proof_b", "kind": "required", "state": "passed", "sourceSite": null}
            ])
        );
        assert_eq!(record["identity"]["readiness"], readiness);
        assert_eq!(record["identity"]["proofExecutionState"], "not_run");
    };
    let ready_record = persisted(&ready);
    let reordered_record = persisted(&reordered);
    assert_census(&ready_record, "passed", "ready");
    assert_census(&reordered_record, "passed", "ready");
    assert_eq!(ready_record, reordered_record);
    for state in [ProofDependencyState::Missing, ProofDependencyState::Failed] {
        let incomplete = evaluate(&[dependency("proof_a", state), passed[0]]);
        assert_eq!(incomplete.identity.readiness, ProofReadiness::Incomplete);
        assert_ne!(ready.identity, incomplete.identity);
        assert_ne!(ready.record.contents, incomplete.record.contents);
        assert_eq!(
            incomplete.identity.proof_execution_state,
            ProofExecutionState::NotRun
        );
        assert_census(
            &persisted(&incomplete),
            match state {
                ProofDependencyState::Missing => "missing",
                ProofDependencyState::Failed => "failed",
                _ => unreachable!("the loop names only incomplete states"),
            },
            "incomplete",
        );
    }
}

#[trace("FR-015-AC-125", "TC-025")]
#[test]
fn native_v2_precondition_keeps_original_parameter_bounds_and_ordering() {
    let (package, clause) =
        Twin::with_bounded_other_precondition().emitted_package("OtherBelowLimit");
    let native = generate_v2_clause_native_oracle(&package, &clause)
        .expect("the original checked ordering is native eligible");
    let general = generate_v2_clause_oracle(&package, &clause).unwrap();
    assert_eq!(native.descriptor, general.descriptor);
    assert_eq!(native.descriptor.inputs[0].lower, 0);
    assert_eq!(native.descriptor.inputs[0].upper, 1000);
    assert!(native
        .source
        .contains("nx::evaluate_native_integer_ordering"));
    assert!(native.source.contains("nx::NativeInteger::new"));
    assert!(native.native_integer_selection.is_some());
    syn::parse_file(&native.source).expect("rendered native oracle is valid Rust syntax");

    let item = ObligationItem::V2Clause {
        package: &package,
        clause: &clause,
        dependencies: None,
    };
    let outcome = negotiate_kani_native_obligations(&KaniObligationRequest {
        ceilings: crate::common::proof_ceilings::proof_ceilings_with_wall_clock(KANI_TIMEOUT),
        items: &[item],
        subject_path: REQUEST_SUBJECT,
        unwind: 3,
    })
    .expect("the authentic clause request is well formed");
    let KaniObligationOutcome::Emitted {
        v2_clause_harnesses,
        ..
    } = outcome
    else {
        panic!("the native clause is emitted");
    };
    let [harness] = v2_clause_harnesses.as_slice() else {
        panic!("one original native clause harness");
    };
    assert_eq!(harness.identity.unwind, 3);
    assert_eq!(
        harness.identity.solver,
        quire_contract_codegen::KaniSolver::Cadical
    );
    assert_eq!(harness.identity.arguments[0].minimum, 0);
    assert_eq!(harness.identity.arguments[0].maximum, 1000);
    assert_eq!(
        harness.identity.integer_representation,
        quire_contract_codegen::ScalarIntegerRepresentation::Native
    );
    assert_eq!(
        harness.identity.native_integer_selection,
        native.native_integer_selection
    );
    assert!(harness.rust.contents.contains(&native.source));
    syn::parse_file(&harness.rust.contents)
        .expect("the native clause harness is valid Rust syntax");
}

#[trace("FR-015-AC-126", "TC-025")]
#[test]
fn native_v2_postcondition_keeps_guarded_arithmetic_and_independent_expectation() {
    let (package, clause) =
        Twin::with_bounded_amount_implication().emitted_package("BalanceNeverDrops");
    let oracle = generate_v2_clause_native_oracle(&package, &clause)
        .expect("the checked guarded arithmetic is native eligible");
    let item = ObligationItem::V2Clause {
        package: &package,
        clause: &clause,
        dependencies: None,
    };
    let outcome = negotiate_kani_native_obligations(&KaniObligationRequest {
        ceilings: crate::common::proof_ceilings::proof_ceilings_with_wall_clock(KANI_TIMEOUT),
        items: &[item],
        subject_path: REQUEST_SUBJECT,
        unwind: 3,
    })
    .expect("the original clause request is well formed");
    let KaniObligationOutcome::Emitted {
        v2_clause_harnesses,
        ..
    } = outcome
    else {
        panic!("the native postcondition is emitted");
    };
    let [harness] = v2_clause_harnesses.as_slice() else {
        panic!("one original postcondition harness");
    };
    assert_eq!(harness.identity.kind, ObligationKind::Postcondition);
    assert_eq!(harness.identity.arguments[0].minimum, 0);
    assert_eq!(harness.identity.arguments[0].maximum, 1000);
    assert_eq!(harness.identity.unwind, 3);
    assert_eq!(
        harness.identity.solver,
        quire_contract_codegen::KaniSolver::Cadical
    );
    assert_eq!(
        harness.identity.native_integer_selection,
        oracle.native_integer_selection
    );
    assert!(harness.rust.contents.contains("nx::NativeIntegerPlan::new"));
    assert!(!harness.rust.contents.contains("_i128_i128"));
    assert!(harness.rust.contents.contains("arithmetic subterm agrees"));
    assert!(harness.rust.contents.contains("subject == expected"));
    syn::parse_file(&harness.rust.contents).expect("the native harness is valid Rust syntax");
}

/// Decode only the actual helper-mutant playback against its healthy sealed source and record.
pub(super) fn decode_native_v2_helper_mutant_playback() {
    let path = |name| {
        PathBuf::from(std::env::var_os(name).unwrap_or_else(|| panic!("explicit {name} required")))
    };
    let (package, clause) =
        Twin::with_bounded_amount_implication().emitted_package("BalanceNeverDrops");
    let item = ObligationItem::V2Clause {
        package: &package,
        clause: &clause,
        dependencies: None,
    };
    let outcome = negotiate_kani_native_obligations(&KaniObligationRequest {
        ceilings: crate::common::proof_ceilings::proof_ceilings_with_wall_clock(KANI_TIMEOUT),
        items: &[item],
        subject_path: REQUEST_SUBJECT,
        unwind: 3,
    })
    .expect("the original clause request is well formed");
    let KaniObligationOutcome::Emitted {
        v2_clause_harnesses,
        ..
    } = outcome
    else {
        panic!("the native postcondition is emitted");
    };
    let [harness] = v2_clause_harnesses.as_slice() else {
        panic!("one original postcondition harness");
    };
    assert!(fs::read_to_string(path("IR705_HELPER_HEALTHY_SOURCE"))
        .unwrap()
        .contains(&harness.rust.contents));
    assert_eq!(
        fs::read(path("IR705_HELPER_HEALTHY_RECORD")).unwrap(),
        harness.record.contents.as_bytes()
    );
    let transcript = fs::read_to_string(path("IR705_HELPER_COUNTEREXAMPLE")).unwrap();
    let limits = quire_contract_codegen::WitnessDecodeLimits {
        bytes: 1024 * 1024,
        values: 1,
    };
    let playback = quire_contract_codegen::parse_assertion_playback(
        &transcript,
        harness.identity.harness_symbol.as_str(),
        harness.identity.module_symbol.as_str(),
        limits.bytes,
        limits.values,
    )
    .expect("actual backend playback parses once");
    let decoded =
        quire_contract_codegen::decode_v2_clause_falsification(harness, &playback, limits)
            .expect("actual playback joins the original sealed harness");
    let [(name, qsl_replay::WitnessValue::Integer(amount))] = decoded.as_slice() else {
        panic!("one typed original integer draw: {decoded:?}");
    };
    assert_eq!(name, &harness.identity.arguments[0].identifier);
    assert!((0..=1000).contains(amount));
    let receipt = json!([{ "identifier": name, "type": "integer", "value": amount.to_string() }]);
    fs::write(
        path("IR705_HELPER_DECODED_WITNESS"),
        serde_json::to_vec_pretty(&receipt).unwrap(),
    )
    .unwrap();
}

#[trace("FR-016-AC-2", "TC-026")]
#[test]
fn native_v2_decoder_enforces_original_inclusive_bounds() {
    let (package, clause) =
        Twin::with_bounded_amount_implication().emitted_package("BalanceNeverDrops");
    let item = ObligationItem::V2Clause {
        package: &package,
        clause: &clause,
        dependencies: None,
    };
    let outcome = negotiate_kani_native_obligations(&KaniObligationRequest {
        ceilings: crate::common::proof_ceilings::proof_ceilings_with_wall_clock(KANI_TIMEOUT),
        items: &[item],
        subject_path: REQUEST_SUBJECT,
        unwind: 3,
    })
    .unwrap();
    let KaniObligationOutcome::Emitted {
        v2_clause_harnesses,
        ..
    } = outcome
    else {
        panic!("native clause emitted");
    };
    let [harness] = v2_clause_harnesses.as_slice() else {
        panic!("one harness");
    };
    let [argument] = harness.identity.arguments.as_slice() else {
        panic!("one parameter");
    };
    assert_eq!((argument.minimum, argument.maximum), (0, 1000));
    // These hand-built bytes test only the parser and cannot count as Kani proof evidence.
    let decode = |value: i64| {
        let bytes = value.to_le_bytes().map(|byte| byte.to_string()).join(", ");
        let transcript = format!(
            "/// Test generated for harness `{}::{}`\n\
/// Check for `assertion`: \"synthetic domain decoder control\"\n\
#[test]\n\
fn kani_concrete_playback_synthetic() {{\n\
    let concrete_vals: Vec<Vec<u8>> = vec![\n\
        // {value}\n\
        vec![{bytes}],\n\
    ];\n\
    kani::concrete_playback_run(concrete_vals, synthetic);\n\
}}\n",
            harness.identity.module_symbol, harness.identity.harness_symbol,
        );
        let limits = quire_contract_codegen::WitnessDecodeLimits {
            bytes: 4096,
            values: 1,
        };
        let playback = quire_contract_codegen::parse_assertion_playback(
            &transcript,
            harness.identity.harness_symbol.as_str(),
            harness.identity.module_symbol.as_str(),
            limits.bytes,
            limits.values,
        )
        .expect("synthetic assertion parses once");
        quire_contract_codegen::decode_v2_clause_falsification(harness, &playback, limits)
    };
    for value in [0_i64, 1000] {
        let decoded = decode(value).expect("inclusive endpoint decoded");
        assert_eq!(
            decoded,
            vec![(
                argument.identifier.clone(),
                qsl_replay::WitnessValue::Integer(i128::from(value))
            )]
        );
    }
    for value in [-1_i64, 1001] {
        let refusal = decode(value).expect_err("outside the checked domain");
        assert_eq!(refusal.code, "cg_witness_out_of_domain");
    }
}

#[trace("FR-015-AC-126", "TC-025")]
#[test]
fn native_v2_postcondition_oracle_typechecks_with_direct_dependencies() {
    let (package, clause) =
        Twin::with_bounded_amount_implication().emitted_package("BalanceNeverDrops");
    let item = ObligationItem::V2Clause {
        package: &package,
        clause: &clause,
        dependencies: None,
    };
    let outcome = negotiate_kani_native_obligations(&KaniObligationRequest {
        ceilings: crate::common::proof_ceilings::proof_ceilings_with_wall_clock(KANI_TIMEOUT),
        items: &[item],
        subject_path: REQUEST_SUBJECT,
        unwind: 3,
    })
    .unwrap();
    let KaniObligationOutcome::Emitted {
        v2_clause_harnesses,
        ..
    } = outcome
    else {
        panic!("native clause emitted");
    };
    let [harness] = v2_clause_harnesses.as_slice() else {
        panic!("one harness");
    };
    let directory = super::scratch_crate::TemporaryDirectory::new("ir705-native-v2-typecheck");
    fs::create_dir_all(directory.path.join("src")).unwrap();
    write_v2_precondition_fixture(&directory.path, harness);
    super::scratch_crate::resolve_seeded_lock(&directory.path);
    let output = std::process::Command::new("cargo")
        .args(["check", "--locked", "--offline", "--lib", "--quiet"])
        .env(
            "CARGO_TARGET_DIR",
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target"),
        )
        .current_dir(&directory.path)
        .output()
        .expect("typecheck generated native oracle");
    assert!(
        output.status.success(),
        "generated native oracle did not typecheck: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(harness.rust.contents.matches("#[kani::proof]").count(), 1);
    assert_eq!(harness.rust.contents.matches("#[cfg(kani)]").count(), 1);
    let stubs = r#"extern crate self as kani;
pub fn any<T: Default>() -> T { T::default() }
pub fn assume(_: bool) {}
#[macro_export]
macro_rules! cover { ($condition:expr, $message:expr) => { let _ = ($condition, $message); }; }
pub mod request { pub fn unrelated(_: i64) -> bool { true } }
"#;
    let source = format!(
        "{stubs}{}",
        harness
            .rust
            .contents
            .replace("#[kani::proof]", "#[test]")
            .replace("#[cfg(kani)]", "")
    );
    fs::write(directory.path.join("src/lib.rs"), source).expect("active proof body");
    let output = std::process::Command::new("cargo")
        .args(["check", "--locked", "--offline", "--tests", "--quiet"])
        .env("RUSTFLAGS", "-Dwarnings")
        .env(
            "CARGO_TARGET_DIR",
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target"),
        )
        .current_dir(&directory.path)
        .output()
        .expect("typecheck the generated native V2 proof body");
    assert!(
        output.status.success(),
        "native V2 proof body failed typecheck: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[trace("FR-015-AC-125", "TC-025")]
#[test]
fn native_v2_chained_probe_uses_selected_intermediate_interval() {
    let (package, clause) =
        Twin::with_bounded_amount_condition("amount < 1000 implies (amount + 1) * 2 <= 2000")
            .emitted_package("BalanceNeverDrops");
    let outcome = negotiate_kani_native_obligations(&KaniObligationRequest {
        ceilings: crate::common::proof_ceilings::proof_ceilings_with_wall_clock(KANI_TIMEOUT),
        items: &[ObligationItem::V2Clause {
            package: &package,
            clause: &clause,
            dependencies: None,
        }],
        subject_path: REQUEST_SUBJECT,
        unwind: 3,
    })
    .expect("the checked clause request");
    let KaniObligationOutcome::Emitted {
        v2_clause_harnesses,
        ..
    } = outcome
    else {
        panic!("the bounded native clause is admitted");
    };
    let [harness] = v2_clause_harnesses.as_slice() else {
        panic!("one native postcondition harness");
    };
    let selected = harness
        .identity
        .native_integer_selection
        .as_ref()
        .expect("native plan decisions");
    let multiply = selected
        .operations
        .iter()
        .find(|decision| decision.operation_identity == "quire.op.integer.mul")
        .expect("checked Multiply after bounded Add");
    assert_eq!(multiply.operands[0].lower, "1");
    assert_eq!(multiply.operands[0].upper, "1001");
    assert!(
        harness
            .rust
            .contents
            .contains("NativeIntegerInterval::new(1_i128, 1001_i128)"),
        "the generated probe must use the selected completed interval"
    );
    assert!(harness.rust.contents.contains("if "));
    assert!(harness.rust.contents.contains("arithmetic subterm agrees"));
}

/// The admitted package and selected node fix the generated bytes, while backend and subject
/// request facts remain distinct harness identities.
#[trace("FR-015-AC-47", "TC-025")]
#[test]
fn qsl_v2_precondition_harness_identity_tracks_request_facts() {
    let (package, clause) =
        Twin::with_bounded_other_precondition().emitted_package("OtherBelowLimit");
    let item = ObligationItem::V2Clause {
        package: &package,
        clause: &clause,
        dependencies: None,
    };
    let generated = |unwind, subject_path| {
        let request = KaniObligationRequest {
            ceilings: crate::common::proof_ceilings::proof_ceilings(),
            items: std::slice::from_ref(&item),
            subject_path,
            unwind,
        };
        let KaniObligationOutcome::Emitted {
            v2_clause_harnesses,
            ..
        } = negotiate_kani_obligations(&request).expect("valid request")
        else {
            panic!("one valid precondition");
        };
        let [harness] = v2_clause_harnesses.as_slice() else {
            panic!("one generated harness");
        };
        harness.clone()
    };
    let first = generated(3, "crate::subject::deposit");
    let same = generated(3, "crate::subject::deposit");
    let changed_unwind = generated(4, "crate::subject::deposit");
    let changed_subject = generated(3, "crate::subject::other");
    assert_eq!(first, same);
    assert_ne!(first.identity, changed_unwind.identity);
    assert_ne!(first.record, changed_unwind.record);
    assert_ne!(first.identity, changed_subject.identity);
    assert_ne!(first.record, changed_subject.record);
}

fn v2_precondition_fixture_harness(
    twin: Twin,
    clause_name: &str,
) -> quire_contract_codegen::KaniV2ClauseObligationHarness {
    let (package, clause) = twin.emitted_package(clause_name);
    let item = ObligationItem::V2Clause {
        package: &package,
        clause: &clause,
        dependencies: None,
    };
    let outcome = negotiate_kani_obligations(&KaniObligationRequest {
        ceilings: crate::common::proof_ceilings::proof_ceilings(),
        items: std::slice::from_ref(&item),
        subject_path: REQUEST_SUBJECT,
        unwind: 8,
    })
    .expect("valid precondition request");
    let KaniObligationOutcome::Emitted {
        v2_clause_harnesses,
        ..
    } = outcome
    else {
        panic!("the V2 precondition is admitted");
    };
    let [harness] = v2_clause_harnesses.as_slice() else {
        panic!("one V2 precondition harness");
    };
    harness.clone()
}

fn write_v2_precondition_fixture(
    directory: &std::path::Path,
    harness: &quire_contract_codegen::KaniV2ClauseObligationHarness,
) {
    fs::write(directory.join("src/lib.rs"), &harness.rust.contents).expect("generated crate");
    super::scratch_crate::write_manifest(
        directory,
        &format!(
            "[package]\nname = \"generated-v2-precondition\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[dependencies]\n{}\nquire-exact = {{ {} }}\n\n[workspace]\n",
            super::scratch_crate::runtime_dependency(&["exact"]),
            quire_contract_codegen::EXACT_DEPENDENCY_SOURCE
        ),
    );
    fs::write(
        directory.join("build.rs"),
        "fn main() { println!(\"cargo:rustc-check-cfg=cfg(kani)\"); }\n",
    )
    .expect("build script");
}

#[trace("TC-025", "FR-014-AC-8")]
#[test]
fn tc_025_v2_precondition_fixture_compiles_with_its_direct_dependencies() {
    let harness =
        v2_precondition_fixture_harness(Twin::with_bounded_other_precondition(), "OtherBelowLimit");
    let directory = scratch("v2-precondition-compile");
    write_v2_precondition_fixture(&directory, &harness);
    super::scratch_crate::resolve_seeded_lock(&directory);
    eprintln!("actual V2 precondition fixture: {}", directory.display());
    let output = std::process::Command::new("cargo")
        .args(["check", "--locked", "--offline", "--lib", "--quiet"])
        .env(
            "CARGO_TARGET_DIR",
            PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("v2-precondition-native"),
        )
        .env("RUSTFLAGS", "-Dwarnings")
        .current_dir(&directory)
        .output()
        .expect("compile the actual generated V2 precondition without Kani");
    assert!(
        output.status.success(),
        "actual V2 precondition fixture did not compile: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let manifest = fs::read_to_string(directory.join("Cargo.toml")).expect("fixture manifest");
    let source = syn::parse_file(&harness.rust.contents).expect("generated harness Rust");
    let mut imports = BTreeSet::new();
    for item in source.items {
        if let syn::Item::Use(item) = item {
            let root = match item.tree {
                syn::UseTree::Path(path) => path.ident,
                syn::UseTree::Name(name) => name.ident,
                syn::UseTree::Rename(rename) => rename.ident,
                _ => panic!("unexpected root import syntax"),
            };
            imports.insert(root.to_string().replace('_', "-"));
        }
    }
    assert!(!imports.is_empty(), "the actual oracle imports its owners");
    for dependency in imports {
        assert!(
            manifest
                .lines()
                .any(|line| line.starts_with(&format!("{dependency} = "))),
            "fixture manifest omits imported crate {dependency}"
        );
    }
    fs::remove_dir_all(directory).expect("remove the compiled fixture");
}

#[trace("FR-014-AC-38")]
#[test]
fn qsl_emitted_clause_composes_boolean_and_integer_operations() {
    let mut compiled = String::new();
    for (index, (expression, expected)) in [
        ("self.balance + 1 >= pre(self.balance)", true),
        ("self.balance - 1 >= pre(self.balance)", false),
        ("self.balance * 2 >= pre(self.balance)", true),
        ("-self.balance <= pre(self.balance)", true),
        ("self.balance >= pre(self.balance) and self.balance >= 0", true),
        ("self.balance >= pre(self.balance) or self.balance >= 0", true),
        ("self.balance >= pre(self.balance) implies self.balance >= 0", true),
        (
            "self.balance >= pre(self.balance) implies (self.balance >= 0 implies self.balance <= 1000)",
            true,
        ),
        ("self.balance = pre(self.balance)", true),
        ("self.balance != pre(self.balance)", false),
        ("self.balance < pre(self.balance)", false),
        ("self.balance <= pre(self.balance)", true),
        ("self.balance > pre(self.balance)", false),
        ("(self.balance >= 0) = (pre(self.balance) >= 0)", true),
        ("(self.balance >= 0) != (pre(self.balance) >= 0)", false),
    ]
    .into_iter()
    .enumerate()
    {
        let (package, clause) =
            Twin::with_balance_condition(expression).emitted_package("BalanceNeverDrops");
        let generated = generate_v2_clause_oracle(&package, &clause)
            .unwrap_or_else(|error| panic!("{expression}: {error:?}"));
        assert_eq!(generated.descriptor.node_id, clause);
        assert!(generated.source.contains("pub fn oracle_clause("));
        let count = u32::try_from(expression.matches("implies").count()).unwrap();
        assert_eq!(generated.source_map[0].expected_consequents, Some(count));
        assert_eq!(generated.source_map.len(), 1 + count as usize);
        for consequent in generated.source_map.iter().skip(1) {
            assert_eq!(consequent.role, "implication_consequent");
            assert_eq!(consequent.node_id, clause);
            let line = generated
                .source
                .lines()
                .nth((consequent.probe.line - 1) as usize)
                .unwrap();
            let start = (consequent.probe.start_column - 1) as usize;
            let end = (consequent.probe.end_column - 1) as usize;
            assert!(line[start..end].starts_with("value_"));
        }
        let arguments = "&value, ".repeat(generated.descriptor.inputs.len());
        compiled.push_str(&format!(
            "pub mod case_{index} {{\n{}\n#[test]\nfn runs() {{\n    let value = rt::Integer::from(0i64);\n    let mut meter = rt::Meter::new(rt::ScalarLimits {{ integer_bits: 1000, decimal_digits: 1000, scale_expansion: 1000, text_input_bytes: 1000, text_scalars: 1000, normalized_scalars: 1000, unit_edges: 1000, value_occurrences: 1000, work_units: 1000, result_units: 1000 }});\n    assert!(matches!(oracle_clause({arguments}&mut meter), Ok(rt::Outcome::Completed({expected}))));\n}}\n}}\n",
            generated.source,
        ));
    }
    let scratch = super::scratch_crate::TemporaryDirectory::new("quire-v2-clause-corpus");
    fs::write(scratch.path.join("src/lib.rs"), compiled).unwrap();
    super::scratch_crate::write_manifest(
        &scratch.path,
        &format!(
            "[package]\nname = \"generated-v2-clause-corpus\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[dependencies]\n{}\nquire-exact = {{ {} }}\nquire-semantic-value = {{ {} }}\n\n[workspace]\n",
            super::scratch_crate::runtime_dependency(&["exact"]),
            quire_contract_codegen::EXACT_DEPENDENCY_SOURCE,
            quire_contract_codegen::SEMANTIC_VALUE_DEPENDENCY_SOURCE
        ),
    );
    let clause_corpus_cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let output = std::process::Command::new(clause_corpus_cargo)
        .args(["test", "--offline", "--quiet"])
        .env("CARGO_TARGET_DIR", scratch.path.join("target"))
        .env("RUSTFLAGS", "-Dwarnings")
        .current_dir(&scratch.path)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "generated clause corpus failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[trace("FR-014-AC-38")]
#[test]
fn skipped_v2_clause_consequent_has_a_measured_zero_native_probe() {
    let (package, clause) =
        Twin::with_balance_condition("self.balance > pre(self.balance) implies self.balance >= 0")
            .emitted_package("BalanceNeverDrops");
    let generated = generate_v2_clause_oracle(&package, &clause).unwrap();
    let consequents = generated
        .source_map
        .iter()
        .filter(|region| region.role == "implication_consequent")
        .collect::<Vec<_>>();
    assert_eq!(consequents.len(), 1);
    let scratch = super::scratch_crate::TemporaryDirectory::new("quire-v2-clause-native-probe");
    fs::write(
        scratch.path.join("src/lib.rs"),
        format!(
            "{}\n#[test]\nfn skipped_consequent() {{\n    let zero = rt::Integer::from(0i64);\n    let mut meter = rt::Meter::new(rt::ScalarLimits {{ integer_bits: 1000, decimal_digits: 1000, scale_expansion: 1000, text_input_bytes: 1000, text_scalars: 1000, normalized_scalars: 1000, unit_edges: 1000, value_occurrences: 1000, work_units: 1000, result_units: 1000 }});\n    assert!(matches!(oracle_clause(&zero, &zero, &mut meter), Ok(rt::Outcome::Completed(true))));\n}}\n",
            generated.source
        ),
    )
    .unwrap();
    super::scratch_crate::write_manifest(
        &scratch.path,
        &format!(
            "[package]\nname = \"generated-v2-clause-native-probe\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[dependencies]\n{}\nquire-exact = {{ {} }}\n\n[workspace]\n",
            super::scratch_crate::runtime_dependency(&["exact"]),
            quire_contract_codegen::EXACT_DEPENDENCY_SOURCE
        ),
    );
    let toolchain = format!("+{}", env!("CARGO_PKG_RUST_VERSION"));
    let sysroot = std::process::Command::new("rustc")
        .args([&toolchain, "--print", "sysroot"])
        .output()
        .unwrap();
    assert!(sysroot.status.success());
    let tools = PathBuf::from(String::from_utf8(sysroot.stdout).unwrap().trim())
        .join("lib/rustlib/x86_64-unknown-linux-gnu/bin");
    for tool in ["llvm-cov", "llvm-profdata"] {
        assert!(tools.join(tool).is_file(), "qualified tool missing");
    }
    let output = std::process::Command::new("cargo")
        .args([
            &toolchain,
            "llvm-cov",
            "--offline",
            "--json",
            "--output-path",
        ])
        .arg(scratch.path.join("coverage.json"))
        .current_dir(&scratch.path)
        .env("CARGO_TARGET_DIR", scratch.path.join("target"))
        .env("LLVM_COV", tools.join("llvm-cov"))
        .env("LLVM_PROFDATA", tools.join("llvm-profdata"))
        .env("CARGO_PROFILE_TEST_OPT_LEVEL", "0")
        .env_remove("RUSTFLAGS")
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "native LLVM: {}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let coverage = fs::read(scratch.path.join("coverage.json")).unwrap();
    let measured =
        quire_contract_codegen::parse_llvm_coverage(&coverage, scratch.path.to_str().unwrap())
            .unwrap();
    assert_eq!(
        measured
            .observe(&consequents[0].artifact_path, consequents[0].probe)
            .unwrap()
            .count(),
        0
    );
}

/// One variant of the fixture. Each variant owns its node codes, because the test binary's
/// fixture registry refuses one code bound to two different bodies.
struct Shape {
    variant: u32,
    clause: &'static str,
    modifies: &'static [&'static str],
    frame_effect: FrameEffect,
    condition: Condition,
    /// The range the object's `balance` member declares; `None` types it by the plain integer.
    balance_bound: Option<(i64, i64)>,
    audit_bound: (i64, i64),
    /// Whether the clause binds a `result` parameter, as FR-341 binds one exactly when the
    /// operation declares a result.
    result: bool,
    /// The name of the object's `balance` member, which the clause's reads carry in the graph.
    condition_field: &'static str,
    /// What the object's `balance` member holds, when it is not the `balance_bound` reference.
    member: Member,
    /// Types `audit` by the plain integer too, so with an unbounded `balance` no bound is
    /// reachable from the clause and lowering itself refuses it for a missing bound.
    audit_unbounded: bool,
    /// `0` shares the framed object, frame, anchor and `self` of every other shape of scope `0`,
    /// as two clauses of one operation do. Any other value gives the shape nodes of its own, so
    /// shapes of different scopes whose objects or frames differ share one package.
    scope: u32,
    /// Makes a `function` node, outside the lowering profile, reachable from the framed object,
    /// so lowering refuses the clause for an unsupported family.
    reaches_function: bool,
}

/// What the operation a fixture's clause anchors declares beyond `self`, for the state-clause
/// replay, which supports only an operation that declares neither.
#[derive(Clone, Copy)]
pub(crate) enum Declares {
    /// One parameter, `other`.
    Parameter,
    /// A result.
    Result,
}

/// The object's `balance` member, for the grounds on which it gives the clause no `i64` range.
#[derive(Clone, Copy, PartialEq)]
enum Member {
    /// A reference to the `balance_bound` range or plain integer type.
    Declared,
    /// A reference to a bound that is not an `integer_range`.
    RationalBound,
    /// A reference to an `integer_range` whose maximum is one past `i64::MAX`.
    WideRange,
    /// A literal value, not a reference.
    Literal,
}

/// What a frame does beyond granting fields.
#[derive(Clone, Copy, PartialEq)]
enum FrameEffect {
    None,
    Creates,
    Deletes,
    /// `deletes_scoped` holds the `receiver` entry: the operation deletes `self`.
    DeletesSelf,
    /// `deletes_scoped` holds a `parameter` entry naming the operation's `other` parameter.
    DeletesParameter,
    GrantsRelationship,
    GrantsForeignField,
}

#[derive(Clone, Copy)]
enum Condition {
    /// A direct Boolean condition in the clause's third argument.
    InlineTrue,
    /// `post(self.balance) >= pre(self.balance)`.
    PostGePre,
    /// `post(self.balance) >= post(self.balance)`.
    PostGePost,
    /// `post(self.balance) >= pre(self.audit)`.
    BalanceAgainstAudit,
    /// `post(self.balance) >= pre(other.balance)`, `other` a second parameter of the object type.
    BalanceAgainstOther,
    /// `post(self.balance) >= 0`.
    BalanceAgainstLiteral,
    /// `not (post(self.balance) >= pre(self.balance))`.
    Negated,
    /// `(post(self.balance) >= pre(self.balance)) and (post(self.balance) >= pre(self.balance))`:
    /// an operator outside the six integer comparisons.
    Conjunction,
}

impl Shape {
    const HEALTHY: Self = Self {
        variant: 0,
        clause: "postcondition",
        modifies: &model::GRANTED,
        frame_effect: FrameEffect::None,
        condition: Condition::PostGePre,
        balance_bound: Some(model::FIELDS[0].1),
        audit_bound: model::FIELDS[1].1,
        result: false,
        condition_field: "balance",
        member: Member::Declared,
        audit_unbounded: false,
        scope: 0,
        reaches_function: false,
    };

    fn code(&self, base: u32) -> u32 {
        50_000 + 1_000 * self.variant + base
    }

    /// The code of a node the shape shares with its scope: the object, its frame, anchor and
    /// parameters, and the dereference of `self`.
    fn scoped(&self, base: u32) -> u32 {
        if self.scope == 0 {
            base
        } else {
            80_000 + 1_000 * self.scope + base
        }
    }
}

fn node_ref(digest: &str) -> Value {
    serde_json::to_value(package::id(digest)).expect("fixture node id")
}

/// `project(<deref>, field)` over `object`'s `field`, typed by `bound`.
fn field_read_through(
    builder: &mut PackageBuilder,
    code: u32,
    deref: &str,
    (object, field, bound): (&str, &str, &str),
) -> String {
    let member = json!({"kind": "field", "declaration": node_ref(object), "name": field});
    builder.application_code_with(
        code,
        "expression",
        "query",
        bound,
        application(
            "query",
            op_full("quire.op.record.project", vec![], None, Some(member)),
            bound,
            vec![reference(deref)],
        ),
        &[deref.to_owned(), object.to_owned()],
    );
    key(code)
}

fn pre_read(builder: &mut PackageBuilder, code: u32, read: &str, bound: &str) -> String {
    builder.application_bounded(
        code,
        "expression",
        "pre_read",
        bound,
        application(
            "pre",
            op("quire.op.state.pre"),
            bound,
            vec![reference(read)],
        ),
        &[],
    );
    key(code)
}

fn state_frame_base_package() -> PackageBuilder {
    let mut builder = PackageBuilder::default();
    for (code, form) in [
        (T_BOOLEAN, "boolean"),
        (T_INTEGER, "integer"),
        (T_TEXT, "text"),
    ] {
        let ty = key(code);
        builder.code(
            code,
            "scalar_type",
            form,
            &ty,
            json!({"term": "aggregate", "members": []}),
        );
    }
    let relation = key(package::RELATION);
    builder.code(
        package::RELATION,
        "relation",
        "relationship",
        &relation,
        json!({"term": "aggregate", "members": []}),
    );
    let boolean = key(T_BOOLEAN);
    builder.application_code(
        FUNCTION,
        "function",
        "pure_function",
        &boolean,
        application(
            "unary",
            op("quire.op.boolean.not"),
            &boolean,
            vec![literal("boolean", "true")],
        ),
    );
    builder
}

fn package_for(shape: &Shape) -> PackageBuilder {
    let mut builder = state_frame_base_package();
    add_shape(&mut builder, shape);
    builder
}

/// A package holding every shape, each over its own object, frame, anchor and clause.
fn package_of(shapes: &[&Shape]) -> PackageBuilder {
    let mut builder = state_frame_base_package();
    for shape in shapes {
        add_shape(&mut builder, shape);
    }
    builder
}

/// Adds the shape's nodes to `builder`. Every node code is the shape's own, so two shapes with
/// different variants share a package.
fn add_shape(builder: &mut PackageBuilder, shape: &Shape) {
    let balance_key = match (shape.member, shape.balance_bound) {
        (Member::RationalBound, _) => builder.bound(&Bound::Rational(1, 2, 1, 2)),
        (Member::WideRange, _) => builder.bound(&Bound::Raw {
            form: "integer_range",
            bounded: "integer",
            body: json!({"term": "aggregate", "members": [
                member("min", literal("integer", "0")),
                member("max", literal("integer", "9223372036854775808")),
            ]}),
            foreign: vec![],
        }),
        (Member::Literal, _) => key(T_INTEGER),
        (Member::Declared, Some((minimum, maximum))) => {
            builder.bound(&Bound::Integer(minimum, maximum))
        }
        (Member::Declared, None) => key(T_INTEGER),
    };
    let balance_value = match shape.member {
        Member::Literal => literal("integer", "0"),
        Member::Declared | Member::RationalBound | Member::WideRange => reference(&balance_key),
    };
    // A second integer range keeps a bound reachable when `balance` names none, unless the shape
    // asks for none to be.
    let audit_key = if shape.audit_unbounded {
        key(T_INTEGER)
    } else {
        builder.bound(&Bound::Integer(shape.audit_bound.0, shape.audit_bound.1))
    };
    let object = key(shape.scoped(OBJECT));
    let mut object_dependencies = vec![balance_key.clone(), audit_key.clone()];
    if shape.reaches_function {
        object_dependencies.push(key(FUNCTION));
    }
    builder.node_with(
        &object,
        "model",
        "object_type",
        &key(T_BOOLEAN),
        json!({"term": "aggregate", "members": [
            member(shape.condition_field, balance_value),
            member("audit", reference(&audit_key)),
        ]}),
        &object_dependencies,
    );
    let other_object = key(shape.scoped(OTHER_OBJECT));
    builder.node_with(
        &other_object,
        "model",
        "object_type",
        &key(T_BOOLEAN),
        json!({"term": "aggregate", "members": []}),
        &[],
    );
    // Every shape selects the same declared relationship. Reuse its admitted
    // node, owner and source occurrence instead of minting a second key for that declaration.
    let relationship = key(package::RELATION);
    let reference_type = key(shape.scoped(REFERENCE));
    builder.node_with(
        &reference_type,
        "composite_type",
        "reference",
        &reference_type,
        json!({"term": "aggregate", "members": [reference(&object)]}),
        std::slice::from_ref(&object),
    );
    builder.code(
        shape.scoped(SELF),
        "value",
        "parameter",
        &reference_type,
        parameter_body("self", 0),
    );
    builder.code(
        shape.scoped(OTHER),
        "value",
        "parameter",
        &reference_type,
        parameter_body("other", 1),
    );
    if shape.result {
        builder.code(
            shape.scoped(RESULT),
            "value",
            "parameter",
            &key(T_BOOLEAN),
            parameter_body("result", 1),
        );
    }
    let field_entry = |declaration: &str, scope: &str, name: &str| json!({"kind": "field", "declaration": node_ref(declaration), "name": name, "scope": scope});
    let mut modifies = shape
        .modifies
        .iter()
        .map(|name| field_entry(&object, "receiver", name))
        .collect::<Vec<_>>();
    let mut frame_dependencies = vec![object.clone()];
    let (mut creates, mut deletes) = (vec![], vec![]);
    let mut deletes_scoped = vec![];
    match shape.frame_effect {
        FrameEffect::None => {}
        FrameEffect::Creates => creates.push(node_ref(&object)),
        FrameEffect::Deletes => deletes.push(node_ref(&object)),
        FrameEffect::DeletesSelf => deletes_scoped.push(json!({"kind": "receiver"})),
        FrameEffect::DeletesParameter => {
            deletes_scoped.push(json!({"kind": "parameter", "name": "other"}));
        }
        FrameEffect::GrantsRelationship => {
            modifies.push(json!({"kind": "relationship", "declaration": node_ref(&relationship)}));
            frame_dependencies.push(relationship.clone());
        }
        FrameEffect::GrantsForeignField => {
            modifies.push(field_entry(&other_object, "every-object", "balance"));
            frame_dependencies.push(other_object.clone());
        }
    }
    let frame = key(shape.scoped(FRAME));
    let anchor = key(shape.scoped(ANCHOR));
    builder.node_with(
        &frame,
        "state",
        "frame",
        &object,
        json!({"term": "frame", "modifies": modifies, "creates": creates, "deletes": deletes, "deletes_scoped": deletes_scoped}),
        &frame_dependencies,
    );
    builder.node_with(
        &anchor,
        "state",
        "operation_anchor",
        &object,
        json!({"term": "aggregate", "members": [
            member("context", reference(&object)),
            member("operation", literal("text", "deposit")),
            member("frame", reference(&frame)),
        ]}),
        &[object.clone(), frame],
    );
    builder.application_bounded(
        shape.scoped(DEREF),
        "expression",
        "deref",
        &object,
        application(
            "deref",
            op("quire.op.model.deref"),
            &object,
            vec![reference(&key(shape.scoped(SELF)))],
        ),
        &[],
    );
    let deref = key(shape.scoped(DEREF));
    builder.application_bounded(
        shape.code(420),
        "expression",
        "deref",
        &object,
        application(
            "deref",
            op("quire.op.model.deref"),
            &object,
            vec![reference(&key(shape.scoped(OTHER)))],
        ),
        &[],
    );
    // The clause's reads are typed by an integer range even where the member gives none, so the
    // comparison is well typed and the refusal is the engine's, not IR's.
    let read_key = if shape.member == Member::Declared {
        balance_key.clone()
    } else {
        audit_key.clone()
    };
    let post_balance = field_read_through(
        builder,
        shape.code(410),
        &deref,
        (&object, shape.condition_field, &read_key),
    );
    let pre_balance = pre_read(builder, shape.code(411), &post_balance, &read_key);
    let post_audit = field_read_through(
        builder,
        shape.code(400),
        &deref,
        (&object, "audit", &audit_key),
    );
    let pre_audit = pre_read(builder, shape.code(401), &post_audit, &audit_key);
    let other = key(shape.code(420));
    let post_other = field_read_through(
        builder,
        shape.code(421),
        &other,
        (&object, shape.condition_field, &read_key),
    );
    let pre_other = pre_read(builder, shape.code(422), &post_other, &read_key);
    let zero = literal("integer", "0");
    let (left, right) = match shape.condition {
        Condition::InlineTrue
        | Condition::PostGePre
        | Condition::Negated
        | Condition::Conjunction => (reference(&post_balance), reference(&pre_balance)),
        Condition::PostGePost => (reference(&post_balance), reference(&post_balance)),
        Condition::BalanceAgainstAudit => (reference(&post_balance), reference(&pre_audit)),
        Condition::BalanceAgainstOther => (reference(&post_balance), reference(&pre_other)),
        Condition::BalanceAgainstLiteral => (reference(&post_balance), zero),
    };
    builder.application_bounded(
        shape.code(200),
        "expression",
        "binary",
        &key(T_BOOLEAN),
        application(
            "binary",
            op("quire.op.integer.ge"),
            &key(T_BOOLEAN),
            vec![left, right],
        ),
        &[],
    );
    let mut condition = key(shape.code(200));
    if matches!(shape.condition, Condition::Negated) {
        builder.application_bounded(
            shape.code(210),
            "expression",
            "unary",
            &key(T_BOOLEAN),
            application(
                "unary",
                op("quire.op.boolean.not"),
                &key(T_BOOLEAN),
                vec![reference(&condition)],
            ),
            &[],
        );
        condition = key(shape.code(210));
    }
    if matches!(shape.condition, Condition::Conjunction) {
        builder.application_bounded(
            shape.code(210),
            "expression",
            "binary",
            &key(T_BOOLEAN),
            application(
                "binary",
                op("quire.op.boolean.and"),
                &key(T_BOOLEAN),
                vec![reference(&condition), reference(&condition)],
            ),
            &[],
        );
        condition = key(shape.code(210));
    }
    // FR-341 binder order: `self`, then `result` when present, then the operation's parameters.
    let mut parameters = vec![reference(&key(shape.scoped(SELF)))];
    if shape.result {
        parameters.push(reference(&key(shape.scoped(RESULT))));
    }
    if matches!(shape.condition, Condition::BalanceAgainstOther) {
        parameters.push(reference(&key(shape.scoped(OTHER))));
    }
    builder.application_bounded(
        shape.code(300),
        "state",
        "state_clause",
        &key(T_BOOLEAN),
        application(
            "state_clause",
            op_full(
                "quire.op.state.clause",
                vec![],
                None,
                Some(json!({"kind": "state_clause", "clause": shape.clause})),
            ),
            &key(T_BOOLEAN),
            vec![
                json!({"term": "aggregate", "members": parameters}),
                reference(&anchor),
                if matches!(shape.condition, Condition::InlineTrue) {
                    literal("boolean", "true")
                } else {
                    reference(&condition)
                },
            ],
        ),
        &[],
    );
}

pub(crate) struct Fixture {
    pub(crate) package: CheckedPackageV2,
    pub(crate) clause: CheckedNodeId,
    object: CheckedNodeId,
    anchor: CheckedNodeId,
    frame: CheckedNodeId,
}

/// The node ids a shape's nodes have in whatever package holds it.
struct Ids {
    clause: CheckedNodeId,
    object: CheckedNodeId,
    anchor: CheckedNodeId,
    frame: CheckedNodeId,
}

impl Ids {
    fn resolved(self, fixture_ids: &package::FixtureIds) -> Self {
        Self {
            clause: fixture_ids.resolve(&self.clause),
            object: fixture_ids.resolve(&self.object),
            anchor: fixture_ids.resolve(&self.anchor),
            frame: fixture_ids.resolve(&self.frame),
        }
    }
}

fn ids(shape: &Shape) -> Ids {
    let id = |digest: String| -> CheckedNodeId {
        serde_json::from_value(node_ref(&digest)).expect("node id")
    };
    Ids {
        clause: code_id(shape.code(300)),
        object: id(key(shape.scoped(OBJECT))),
        anchor: id(key(shape.scoped(ANCHOR))),
        frame: id(key(shape.scoped(FRAME))),
    }
}

fn fixture_with_resolver(shape: &Shape) -> (Fixture, package::FixtureIds) {
    let (package, fixture_ids) = package_for(shape).admit_resolved();
    let Ids {
        clause,
        object,
        anchor,
        frame,
    } = ids(shape).resolved(&fixture_ids);
    let fixture = Fixture {
        package,
        clause,
        object,
        anchor,
        frame,
    };
    (fixture, fixture_ids)
}

fn fixture(shape: &Shape) -> Fixture {
    fixture_with_resolver(shape).0
}

/// Source-backed fixtures for IR-295's exact planned state-frame refusal selectors.
pub(crate) fn ir295_selector_fixture(negated: bool, grants_every_field: bool) -> Fixture {
    let modifies: &'static [&'static str] = if grants_every_field {
        &["audit", "balance"]
    } else {
        &model::GRANTED
    };
    let shape = Shape {
        variant: if negated {
            10
        } else if grants_every_field {
            1902
        } else {
            0
        },
        modifies,
        condition: if negated {
            Condition::Negated
        } else {
            Condition::PostGePre
        },
        ..Shape::HEALTHY
    };
    fixture(&shape)
}

pub(crate) fn ir295_healthy_fixture() -> Fixture {
    emitted_fixture(&Twin::new(), "BalanceNeverDrops")
}

pub(crate) fn ir295_state_fields() -> &'static [&'static str] {
    &STATE_FIELDS
}

/// The selected model fixture, with graph identities taken from QSL's emitted package.
pub(crate) fn emitted_fixture(twin: &Twin, clause_name: &str) -> Fixture {
    let (package, clause) = twin.emitted_package(clause_name);
    let node = |form: &str| {
        package
            .graph()
            .nodes
            .iter()
            .find(|node| node.semantic_form.as_ref() == form)
            .unwrap_or_else(|| panic!("emitted package has no {form} node"))
            .node_id
            .clone()
    };
    let (object, anchor, frame) = (node("object_type"), node("operation_anchor"), node("frame"));
    Fixture {
        package,
        clause,
        object,
        anchor,
        frame,
    }
}

fn request<'a>(fixture: &'a Fixture, fields: &'a [&'a str]) -> StateFrameRequest<'a> {
    StateFrameRequest {
        ceilings: crate::common::proof_ceilings::proof_ceilings_with_wall_clock(KANI_TIMEOUT),
        package: &fixture.package,
        clause: &fixture.clause,
        state_path: STATE_PATH,
        state_fields: fields,
        subject_path: SUBJECT_PATH,
        unwind: 4,
    }
}

fn generate(fixture: &Fixture) -> StateFrameObligations {
    generate_state_frame_obligations(&request(fixture, &STATE_FIELDS))
        .unwrap_or_else(|refusal| panic!("the fixture must generate: {refusal}"))
}

#[trace("FR-028-AC-1")]
#[test]
fn both_state_obligation_identities_change_with_either_required_ceiling() {
    let twin = Twin::new();
    let (package, clause) = twin.emitted_package("BalanceNeverDrops");
    let mut requested = StateFrameRequest {
        ceilings: crate::common::proof_ceilings::proof_ceilings_with_wall_clock(KANI_TIMEOUT),
        package: &package,
        clause: &clause,
        state_path: STATE_PATH,
        state_fields: &STATE_FIELDS,
        subject_path: SUBJECT_PATH,
        unwind: 4,
    };
    let baseline = generate_state_frame_obligations(&requested).unwrap();
    for memory in [true, false] {
        requested.ceilings = baseline.frame.identity.ceilings;
        if memory {
            requested.ceilings.memory_bytes = std::num::NonZeroU64::new(128 * 1024 * 1024).unwrap();
        } else {
            requested.ceilings.wall_clock /= 2;
        }
        let changed = generate_state_frame_obligations(&requested).unwrap();
        for (original, changed) in [
            (&baseline.frame, &changed.frame),
            (&baseline.postcondition, &changed.postcondition),
        ] {
            assert_ne!(original.identity, changed.identity);
            assert_ne!(original.record.contents, changed.record.contents);
            assert_eq!(changed.identity.ceilings, requested.ceilings);
        }
    }
}

/// The check text of the frame harness's assertion over `audit`, as Kani prints it.
const FORBIDDEN_CHECK: &str =
    "operation `deposit` changed `audit`, which its frame does not modify";

/// Generate both roles directly from the checked package QSL emits for the twin's clause.
fn generated_from_twin(
    twin: &Twin,
    clause_name: &str,
    fields: &[&str],
    subject_path: &str,
) -> StateFrameObligations {
    let (package, clause) = twin.emitted_package(clause_name);
    generate_state_frame_obligations(&StateFrameRequest {
        ceilings: crate::common::proof_ceilings::proof_ceilings_with_wall_clock(KANI_TIMEOUT),
        package: &package,
        clause: &clause,
        state_path: STATE_PATH,
        state_fields: fields,
        subject_path,
        unwind: 4,
    })
    .expect("the emitted clause generates both roles")
}

/// The healthy twin's frame identity from QSL's emitted checked package.
fn frame_harness(twin: &Twin) -> StateFrameIdentity {
    generated_from_twin(twin, "BalanceNeverDrops", &STATE_FIELDS, SUBJECT_PATH)
        .frame
        .identity
}

/// The run of `harness` whose playback binds `values`, in the harness's draw order.
fn run_of(harness: &StateFrameIdentity, values: [i64; 2]) -> Run {
    Run {
        playback: playback_text(harness, FORBIDDEN_CHECK, &values),
        harness: harness.clone(),
        decode_limits: native_twin::frame_decode_limits(),
    }
}

#[trace("TC-026", "FR-016-AC-6")]
#[test]
fn tc_026_frame_replay_refuses_poisoned_suffixes_at_both_decode_bounds() {
    let twin = Twin::new();
    let harness = frame_harness(&twin);
    let invocation = twin.invocation("account", (5, 0), (6, 1));
    let run = run_of(&harness, [5, 0]);
    let limits = quire_contract_codegen::WitnessDecodeLimits {
        bytes: run.playback.len(),
        values: 2,
    };
    twin.try_frame_replay_with_limits("deposit", &invocation, "account", "audit", &run, limits)
        .expect("at both bounds");
    for (poisoned, limits, code) in [
        (
            format!("{}{}", run.playback, run.playback),
            limits,
            "cg_witness_decode_bytes_limit",
        ),
        (
            run.playback
                .replace("    ];", "        // 0\n        vec![not_a_byte],\n    ];"),
            quire_contract_codegen::WitnessDecodeLimits {
                bytes: usize::MAX,
                values: 2,
            },
            "cg_witness_decode_values_limit",
        ),
    ] {
        let mut candidate = run.clone();
        candidate.playback = poisoned.clone();
        let error = match twin.try_frame_replay_with_limits(
            "deposit",
            &invocation,
            "account",
            "audit",
            &candidate,
            limits,
        ) {
            Ok(_) => panic!("over-bound playback must refuse"),
            Err(error) => error,
        };
        assert!(matches!(error, FrameReplayError::Decode(cause) if cause.code == code));
        candidate.playback = poisoned;
        let control = twin.try_frame_replay_with_limits(
            "deposit",
            &invocation,
            "account",
            "audit",
            &candidate,
            quire_contract_codegen::WitnessDecodeLimits {
                bytes: candidate.playback.len(),
                values: 3,
            },
        );
        let error = match control {
            Ok(_) => panic!("poisoned suffix must refuse when read"),
            Err(error) => error,
        };
        let poisoned_code = if code == "cg_witness_decode_bytes_limit" {
            "kani_witness_multiple_assertions_refused"
        } else {
            "kani_witness_byte_invalid"
        };
        assert!(matches!(error, FrameReplayError::Decode(cause) if cause.code == poisoned_code));
    }
}

#[trace("TC-035", "FR-024-AC-30")]
#[test]
fn tc_035_frame_replay_rechecks_parsed_playback_bounds() {
    let twin = Twin::new();
    let harness = frame_harness(&twin);
    let invocation = twin.invocation("account", (5, 0), (6, 1));
    let run = run_of(&harness, [5, 0]);
    let admitted = quire_contract_codegen::WitnessDecodeLimits {
        bytes: run.playback.len(),
        values: 2,
    };
    twin.try_frame_replay_with_parse_limits(
        "deposit",
        &invocation,
        "account",
        "audit",
        &run,
        admitted,
        admitted,
    )
    .expect("complete parsed playback is admitted at both limits");
    for (requested, code) in [
        (
            quire_contract_codegen::WitnessDecodeLimits {
                bytes: admitted.bytes - 1,
                values: admitted.values,
            },
            "cg_witness_decode_bytes_limit",
        ),
        (
            quire_contract_codegen::WitnessDecodeLimits {
                bytes: admitted.bytes,
                values: admitted.values - 1,
            },
            "cg_witness_decode_values_limit",
        ),
    ] {
        let error = match twin.try_frame_replay_with_parse_limits(
            "deposit",
            &invocation,
            "account",
            "audit",
            &run,
            requested,
            admitted,
        ) {
            Ok(_) => panic!("replay must refuse a playback parsed under wider limits"),
            Err(error) => error,
        };
        assert!(matches!(error, FrameReplayError::Decode(cause) if cause.code == code));
    }
    for (run_limits, code) in [
        (
            quire_contract_codegen::WitnessDecodeLimits {
                bytes: admitted.bytes - 1,
                values: admitted.values,
            },
            "cg_witness_decode_bytes_limit",
        ),
        (
            quire_contract_codegen::WitnessDecodeLimits {
                bytes: admitted.bytes,
                values: admitted.values - 1,
            },
            "cg_witness_decode_values_limit",
        ),
    ] {
        let mut narrow_run = run.clone();
        narrow_run.decode_limits = run_limits;
        let error = match twin.try_frame_replay_with_parse_limits(
            "deposit",
            &invocation,
            "account",
            "audit",
            &narrow_run,
            admitted,
            admitted,
        ) {
            Ok(_) => panic!("replay must refuse a playback parsed above the proving-run limit"),
            Err(error) => error,
        };
        assert!(matches!(error, FrameReplayError::Decode(cause) if cause.code == code));
    }
}

/// The forbidden-write run at the pre state `(balance, audit) = (5, 0)`.
fn forbidden_run(twin: &Twin) -> Run {
    run_of(&frame_harness(twin), [5, 0])
}

/// The identity the replay of `operation` over `run` puts in the request, the invocation at
/// `(5, 0)` named for every operation.
fn minted(twin: &Twin, operation: &str, run: &Run) -> ObligationIdentity {
    let invocation = twin.invocation("account", (5, 0), (6, 1));
    twin.try_frame_replay(operation, &invocation, "account", "audit", run)
        .unwrap_or_else(|error| panic!("the replay of `{operation}` builds: {error}"))
        .wire
        .obligation_identity
}

/// The identity issued with QSL's selected operation-frame claim.
fn producer_frame_identity(site: &OperationSite) -> ObligationIdentity {
    assert_eq!(site.frame_occurrence.node(), &site.frame);
    site.obligation_identity().clone()
}

/// The state-clause and frame-effect harness sources, for the cover-last guard (FR-015-AC-58).
pub(crate) fn guard_sources() -> Vec<(&'static str, String)> {
    let generated = generate(&emitted_fixture(&Twin::new(), "BalanceNeverDrops"));
    let precondition =
        v2_precondition_fixture_harness(Twin::with_bounded_other_precondition(), "OtherBelowLimit");
    vec![
        ("v2 precondition", precondition.rust.contents),
        ("state clause", generated.postcondition.rust.contents),
        ("frame effect", generated.frame.rust.contents),
    ]
}

fn refusal(shape: &Shape, fields: &[&str]) -> StateFrameRefusal {
    let fixture = fixture(shape);
    generate_state_frame_obligations(&request(&fixture, fields))
        .expect_err("the request must be refused")
}

/// The reader-admitted frame retains the producer-assigned object and integer-bound
/// references, even when the clause uses a second parameter.
#[test]
fn state_frame_producer_ids_connect_object_and_bound() {
    let shape = Shape {
        variant: 12,
        condition: Condition::BalanceAgainstOther,
        ..Shape::HEALTHY
    };
    let package = package_for(&shape).admit();
    let object = package::id(&key(shape.scoped(OBJECT)));
    let frame = package::id(&key(shape.scoped(FRAME)));
    let (lower, upper) = model::FIELDS[0].1;
    let bound = package::id(&Bound::Integer(lower, upper).key());
    let graph = &package.graph().nodes;
    let admitted_frame = graph
        .iter()
        .find(|node| node.node_id == frame)
        .expect("frame admitted");
    let admitted_object = graph
        .iter()
        .find(|node| node.node_id == object)
        .expect("object admitted");
    assert!(admitted_frame.dependencies.contains(&object));
    assert!(admitted_object.dependencies.contains(&bound));
}

/// Rekeying preserves both grants in each frame even when
/// their final declaration keys sort in a different order from the authored keys.
#[trace("FR-015-AC-29", "TC-025")]
#[test]
fn tc_025_frame_grants_rekey_to_admitted_order_without_dropping_effects() {
    for (variant, effect, kind) in [
        (8, FrameEffect::GrantsRelationship, "relationship"),
        (9, FrameEffect::GrantsForeignField, "field"),
    ] {
        let shape = Shape {
            variant,
            frame_effect: effect,
            ..Shape::HEALTHY
        };
        let (package, ids) = package_for(&shape).admit_resolved();
        let frame = ids.resolve(&package::id(&key(shape.scoped(FRAME))));
        let node = package
            .graph()
            .nodes
            .iter()
            .find(|node| node.node_id == frame)
            .expect("frame admitted");
        let CheckedTerm::Frame { modifies, .. } = &node.body else {
            panic!("frame grants are typed");
        };
        assert_eq!(
            modifies.len(),
            2,
            "the ordinary field and added grant remain"
        );
        let object = ids.resolve(&package::id(&key(shape.scoped(OBJECT))));
        let added = if effect == FrameEffect::GrantsRelationship {
            ids.resolve(&code_id(package::RELATION))
        } else {
            ids.resolve(&package::id(&key(shape.scoped(OTHER_OBJECT))))
        };
        assert_eq!(
            modifies
                .iter()
                .filter(|entry| matches!(entry, quire_contract_model::CheckedFrameModification::Field { declaration, .. } if declaration == &object))
                .count(),
            1,
            "the ordinary field grant remains"
        );
        assert_eq!(
            modifies
                .iter()
                .filter(|entry| match (kind, entry) {
                    (
                        "field",
                        quire_contract_model::CheckedFrameModification::Field {
                            declaration, ..
                        },
                    ) => declaration == &added,
                    (
                        "relationship",
                        quire_contract_model::CheckedFrameModification::Relationship {
                            declaration,
                        },
                    ) => declaration == &added,
                    _ => false,
                })
                .count(),
            1,
            "the frame keeps its {kind} grant by declaration"
        );
    }
}

// ---- default lane ------------------------------------------------------------

/// A postcondition clause yields two separate harnesses whose identity is scoped to the
/// operation, anchor, frame and object, whose bounds and granted and forbidden fields are read
/// from the IR, and each carries exactly one non-vacuity cover.
#[trace(
    "FR-015-AC-7",
    "FR-015-AC-26",
    "FR-015-AC-27",
    "FR-015-AC-28",
    "TC-025",
    "TC-001",
    "NFR-002-AC-1",
    "NFR-002-AC-2"
)]
#[test]
fn tc_025_a_postcondition_yields_a_contract_harness_and_a_scoped_frame_harness() {
    let fixture = emitted_fixture(&Twin::new(), "BalanceNeverDrops");
    let generated = generate(&fixture);
    for harness in [&generated.postcondition, &generated.frame] {
        crate::generated_headers::assert_header(
            &harness.rust.path,
            &harness.rust.contents,
            &[
                "deposit",
                &format!("{:?}", fixture.clause).escape_debug().to_string(),
            ],
        );
        for node in [
            &harness.identity.scope.object,
            &harness.identity.scope.anchor,
            &harness.identity.scope.frame,
        ] {
            crate::generated_headers::assert_header(
                &harness.rust.path,
                &harness.rust.contents,
                &[&format!("{node:?}").escape_debug().to_string()],
            );
        }
    }
    let scope = &generated.postcondition.identity.scope;
    assert_eq!(scope.operation, "deposit");
    assert_eq!(scope.object, fixture.object);
    assert_eq!(scope.anchor, fixture.anchor);
    assert_eq!(scope.frame, fixture.frame);
    assert_eq!(&generated.frame.identity.scope, scope);
    assert_eq!(generated.postcondition.identity.clause, fixture.clause);

    assert_eq!(
        generated.postcondition.identity.property,
        StateFrameProperty::Postcondition {
            field: "balance".to_owned(),
            comparison: StateComparison::Ge,
            left_is_pre: false,
        }
    );
    // Every field the object's IR bounds is assumed in range, so a counterexample is a state
    // the model admits.
    let domain = |field: &str| StateFieldDomain {
        field: field.to_owned(),
        minimum: 0,
        maximum: 1000,
    };
    for harness in [&generated.postcondition, &generated.frame] {
        assert_eq!(
            harness.identity.domains,
            vec![domain("balance"), domain("audit")]
        );
        let source = &harness.rust.contents;
        assert!(source.contains("kani::assume(pre.balance >= 0_i64 && pre.balance <= 1000_i64);"));
        assert!(source.contains("kani::assume(pre.audit >= 0_i64 && pre.audit <= 1000_i64);"));
    }
    assert_eq!(
        generated.frame.identity.property,
        StateFrameProperty::Frame {
            granted: vec!["balance".to_owned()],
            checked: vec!["audit".to_owned()],
        }
    );

    let post = &generated.postcondition.rust.contents;
    assert!(post.contains("let mut post = pre.clone();"));
    assert!(post.contains("crate::subject::deposit(&mut post);"));
    assert!(post.contains("post.balance >= pre.balance"));
    let frame = &generated.frame.rust.contents;
    assert!(frame.contains("assert!(post.audit == pre.audit,"));
    assert!(!frame.contains("post.balance == pre.balance"));
    for source in [post, frame] {
        assert_eq!(source.matches("#[kani::proof]").count(), 1);
        assert_eq!(source.matches("kani::cover!(").count(), 1);
        // The cover ends the harness, after every assertion: an assertion that fails does not
        // return, so the cover cannot share a failing valuation (IR-451).
        let (assertion, cover) = (source.rfind("assert!("), source.find("kani::cover!("));
        assert!(
            assertion.zip(cover).is_some_and(|(a, c)| a < c),
            "the cover must follow the last assertion:\n{source}"
        );
    }
    assert_eq!(post.matches("assert!(").count(), 1);
    assert_eq!(frame.matches("assert!(").count(), 1);

    // The persisted record carries the same scoped identity.
    let record: Value = serde_json::from_str(&generated.frame.record.contents).expect("record");
    assert_eq!(record["identity"]["scope"]["operation"], "deposit");
    assert_eq!(record["identity"]["property"]["kind"], "frame");
    assert_eq!(record["identity"]["scope"]["frame"], json!(fixture.frame));

    // Equal inputs regenerate byte-identically, and the unwind bound is part of the identity.
    assert_eq!(generate(&fixture), generated);
    let unwound = generate_state_frame_obligations(&StateFrameRequest {
        ceilings: crate::common::proof_ceilings::proof_ceilings_with_wall_clock(KANI_TIMEOUT),
        unwind: 5,
        ..request(&fixture, &STATE_FIELDS)
    })
    .expect("generates");
    assert_ne!(
        unwound.frame.identity.options,
        generated.frame.identity.options
    );
}

/// A frame that grants nothing checks every field, so a different frame is a different proof.
#[trace("FR-015-AC-28", "TC-025")]
#[test]
fn tc_025_the_frame_harness_follows_the_frame_node_not_the_caller() {
    let generated = generate(&emitted_fixture(
        &Twin::build(&[], &CLAUSES, 0),
        "BalanceNeverDrops",
    ));
    assert_eq!(
        generated.frame.identity.property,
        StateFrameProperty::Frame {
            granted: vec![],
            checked: vec!["balance".to_owned(), "audit".to_owned()],
        }
    );
    let frame = &generated.frame.rust.contents;
    assert!(frame.contains("assert!(post.balance == pre.balance,"));
    assert!(frame.contains("assert!(post.audit == pre.audit,"));
}

/// Every shape outside one integer comparison of pre and post reads of one field through
/// `self`, and every frame effect with no finite encoding, is refused by name, with no harness.
/// Each row fails if the refusal it names is removed: the shape would otherwise generate.
#[trace("FR-015-AC-29", "TC-025")]
#[test]
fn tc_025_shapes_without_a_finite_encoding_are_refused_by_name() {
    let refused = |shape: Shape| refusal(&shape, &STATE_FIELDS);
    let with = |variant, edit: fn(Shape) -> Shape| {
        refused(edit(Shape {
            variant,
            ..Shape::HEALTHY
        }))
    };
    let condition_not_supported = |refusal: StateFrameRefusal| {
        matches!(refusal, StateFrameRefusal::ConditionNotSupported { .. })
    };
    let unsupported_effect = |refusal: StateFrameRefusal, expected: UnsupportedFrameEffect| {
        matches!(
            refusal,
            StateFrameRefusal::FrameEffectUnsupported { effect, .. } if effect == expected
        )
    };

    // Not a postcondition.
    assert!(matches!(
        with(2, |shape| Shape { clause: "precondition", ..shape }),
        StateFrameRefusal::NotAPostcondition { clause } if clause == "precondition"
    ));
    // The condition.
    assert_eq!(
        with(3, |shape| Shape {
            condition: Condition::PostGePost,
            ..shape
        }),
        StateFrameRefusal::ObservationsSameSide
    );
    assert_eq!(
        with(4, |shape| Shape {
            condition: Condition::BalanceAgainstAudit,
            ..shape
        }),
        StateFrameRefusal::ObservationsDiffer {
            left: "balance".to_owned(),
            right: "audit".to_owned()
        }
    );
    assert!(condition_not_supported(with(10, |shape| Shape {
        condition: Condition::Negated,
        ..shape
    })));
    assert!(condition_not_supported(with(11, |shape| Shape {
        condition: Condition::BalanceAgainstLiteral,
        ..shape
    })));
    // A read through a second parameter of the object type is not a read of `self`.
    assert!(condition_not_supported(with(12, |shape| Shape {
        condition: Condition::BalanceAgainstOther,
        ..shape
    })));
    // The frame.
    assert!(unsupported_effect(
        with(5, |shape| Shape {
            frame_effect: FrameEffect::Creates,
            ..shape
        }),
        UnsupportedFrameEffect::Creates
    ));
    assert!(unsupported_effect(
        with(7, |shape| Shape {
            frame_effect: FrameEffect::Deletes,
            ..shape
        }),
        UnsupportedFrameEffect::Deletes
    ));
    // A scoped delete is a delete: of the receiver, or of a reference parameter.
    assert!(unsupported_effect(
        with(13, |shape| Shape {
            frame_effect: FrameEffect::DeletesSelf,
            ..shape
        }),
        UnsupportedFrameEffect::Deletes
    ));
    assert!(unsupported_effect(
        with(14, |shape| Shape {
            frame_effect: FrameEffect::DeletesParameter,
            ..shape
        }),
        UnsupportedFrameEffect::Deletes
    ));
    assert!(unsupported_effect(
        with(8, |shape| Shape {
            frame_effect: FrameEffect::GrantsRelationship,
            ..shape
        }),
        UnsupportedFrameEffect::Relationship
    ));
    assert!(unsupported_effect(
        with(9, |shape| Shape {
            frame_effect: FrameEffect::GrantsForeignField,
            ..shape
        }),
        UnsupportedFrameEffect::ForeignField
    ));
    // A field the frame grants or the clause reads that the caller's state lacks.
    assert_eq!(
        refusal(&Shape::HEALTHY, &["audit"]),
        StateFrameRefusal::UnknownStateField {
            field: "balance".to_owned()
        }
    );
    // A frame granting every state field forbids nothing.
    assert!(matches!(
        refusal(&Shape::HEALTHY, &["balance"]),
        StateFrameRefusal::NothingForbidden { .. }
    ));
}

/// Each field is assumed in the range its own member declares, so two fields with different
/// ranges each keep theirs, and the clause field never takes another field's.
#[trace("FR-015-AC-27", "TC-025")]
#[test]
fn tc_025_each_field_is_assumed_in_its_own_declared_range() {
    let generated = generate(&emitted_fixture(
        &Twin::with_field_ranges((5, 900), (0, 50)),
        "BalanceNeverDrops",
    ));
    let domain = |field: &str, minimum, maximum| StateFieldDomain {
        field: field.to_owned(),
        minimum,
        maximum,
    };
    let expected = vec![domain("balance", 5, 900), domain("audit", 0, 50)];
    assert_eq!(generated.postcondition.identity.domains, expected);
    assert_eq!(generated.frame.identity.domains, expected);
    let post = &generated.postcondition.rust.contents;
    assert!(post.contains("kani::assume(pre.balance >= 5_i64 && pre.balance <= 900_i64);"));
    assert!(post.contains("kani::assume(pre.audit >= 0_i64 && pre.audit <= 50_i64);"));
}

/// Two clauses of one operation share an anchor and frame, yet each harness lands at its own
/// module and record path.
#[trace("FR-015-AC-26", "TC-025")]
#[test]
fn tc_025_two_clauses_of_one_operation_never_share_a_frame_record_path() {
    let twin = Twin::new();
    let first = generate(&emitted_fixture(&twin, "BalanceNeverDrops"));
    let second = generate(&emitted_fixture(&twin, "AuditNeverDrops"));
    assert_eq!(first.frame.identity.scope, second.frame.identity.scope);
    assert_ne!(first.frame.identity.clause, second.frame.identity.clause);
    assert_ne!(first.frame.record.path, second.frame.record.path);
    assert_ne!(first.frame.rust.path, second.frame.rust.path);
}

/// A request that is not well formed, and a node that is not a clause, are refused before any
/// harness is generated.
#[trace("FR-015-AC-29", "TC-025")]
#[test]
fn tc_025_malformed_requests_and_non_clause_nodes_are_refused() {
    let fixture = fixture(&Shape::HEALTHY);
    let refused = |request: StateFrameRequest<'_>| {
        generate_state_frame_obligations(&request).expect_err("refused")
    };
    assert_eq!(
        refused(StateFrameRequest {
            ceilings: crate::common::proof_ceilings::proof_ceilings_with_wall_clock(KANI_TIMEOUT),
            unwind: 0,
            ..request(&fixture, &STATE_FIELDS)
        }),
        StateFrameRefusal::UnwindOutOfRange { unwind: 0 }
    );
    assert_eq!(
        refused(StateFrameRequest {
            ceilings: crate::common::proof_ceilings::proof_ceilings_with_wall_clock(KANI_TIMEOUT),
            subject_path: "not a path",
            ..request(&fixture, &STATE_FIELDS)
        }),
        StateFrameRefusal::InvalidPath {
            path: "not a path".to_owned()
        }
    );
    assert_eq!(
        refused(request(&fixture, &["balance", "balance"])),
        StateFrameRefusal::InvalidField {
            name: "balance".to_owned()
        }
    );
    let frame_node = refused(StateFrameRequest {
        ceilings: crate::common::proof_ceilings::proof_ceilings_with_wall_clock(KANI_TIMEOUT),
        clause: &fixture.frame,
        ..request(&fixture, &STATE_FIELDS)
    });
    assert!(matches!(
        frame_node,
        StateFrameRefusal::NotAStateClause { ref semantic_form, .. } if semantic_form == "frame"
    ));
    let absent = package::id(&key(9999));
    assert!(matches!(
        refused(StateFrameRequest {
            ceilings: crate::common::proof_ceilings::proof_ceilings_with_wall_clock(KANI_TIMEOUT),
            clause: &absent,
            ..request(&fixture, &STATE_FIELDS)
        }),
        StateFrameRefusal::NotLowered { refusal }
            if matches!(*refusal, StateFrameLoweringRefusal::InvalidInput { .. })
    ));
}

/// A field name read from the graph that is not a Rust identifier is a malformed clause at the
/// node: not `InvalidField` and not `ConditionNotSupported`. IR admits a name that is an ASCII
/// identifier but a Rust keyword, so a keyword reaches the engine in a clause's read and in a
/// frame's grant, and each is refused there. A field name the caller supplies that is not an identifier is still
/// `InvalidField`, naming it. Each assertion fails if the refusal moves to the other code.
#[trace("FR-015-AC-67", "TC-025")]
#[test]
fn tc_025_a_non_identifier_graph_field_name_is_a_malformed_clause() {
    let read = Shape {
        variant: 15,
        condition_field: "not-an-identifier",
        ..Shape::HEALTHY
    };
    let (read_fixture, read_ids) = fixture_with_resolver(&read);
    assert_eq!(
        generate_state_frame_obligations(&request(&read_fixture, &STATE_FIELDS))
            .expect_err("the graph field name must be refused"),
        StateFrameRefusal::MalformedClause {
            at: read_ids.resolve(&code_id(read.code(410)))
        }
    );
    // A Rust keyword is not an identifier to `syn` yet is one to IR's own check of a frame's
    // granted name, so IR admits it and the engine alone refuses it: in a read at the read's
    // node, and in a frame's grant at the frame node.
    let keyword_read = Shape {
        variant: 16,
        condition_field: "type",
        ..Shape::HEALTHY
    };
    let (keyword_fixture, keyword_ids) = fixture_with_resolver(&keyword_read);
    assert_eq!(
        generate_state_frame_obligations(&request(&keyword_fixture, &STATE_FIELDS))
            .expect_err("the keyword field name must be refused"),
        StateFrameRefusal::MalformedClause {
            at: keyword_ids.resolve(&code_id(keyword_read.code(410)))
        }
    );
    let keyword_grant = Shape {
        variant: 17,
        condition_field: "type",
        modifies: &["type"],
        ..Shape::HEALTHY
    };
    assert_eq!(
        refusal(&keyword_grant, &STATE_FIELDS),
        StateFrameRefusal::MalformedClause {
            at: fixture(&keyword_grant).frame
        }
    );
    // The caller's own field list is the one place `InvalidField` is still named.
    assert_eq!(
        refusal(&Shape::HEALTHY, &["balance", "not-an-identifier"]),
        StateFrameRefusal::InvalidField {
            name: "not-an-identifier".to_owned()
        }
    );
}

/// A hand-built model/object_type body cannot supply a range after the model accessor refuses
/// that object. Its apparent bounded members are irrelevant to the typed refusal.
#[trace("FR-015-AC-66", "FR-015-AC-78", "TC-025")]
#[test]
fn tc_025_an_unselected_model_body_never_supplies_a_range() {
    let fixture = fixture(&Shape::HEALTHY);
    assert_eq!(
        generate_state_frame_obligations(&request(&fixture, &STATE_FIELDS))
            .expect_err("the model accessor refuses the body"),
        StateFrameRefusal::BoundNotResolved {
            field: "balance".to_owned(),
            cause: BoundNotResolvedCause::ModelFieldsUnavailable {
                object: fixture.object,
                error: quire_contract_model::CheckedModelFieldsError::NotModelObjectType,
            },
        }
    );
}

// ---- the StateFrame arm of negotiation (IR-461) -----------------------------

/// Every shape the arm's tests negotiate, in one admitted package, each over nodes of its own.
/// The index of a shape in this list is its `const` below.
const PRE: usize = 0;
const OK: usize = 1;
const TWO_FIELDS: usize = 3;
const RELATIONSHIP_FRAME: usize = 4;
const GRANTS_ALL: usize = 5;
const REACHES_FUNCTION: usize = 8;
const NEGATION: usize = 9;
const LITERAL_OPERAND: usize = 10;
const CONJUNCTION: usize = 11;
const OTHER_PARAMETER: usize = 12;
const SAME_SIDE: usize = 13;
const CREATES: usize = 14;
const DELETES: usize = 15;
const FOREIGN_FIELD: usize = 16;
const KEYWORD_READ: usize = 19;
const NEGATION_GRANTS_ALL: usize = 23;
const DELETES_SELF: usize = 24;
const DELETES_PARAMETER: usize = 25;

fn arm_shapes() -> Vec<Shape> {
    let edits: Vec<fn(Shape) -> Shape> = vec![
        |shape| Shape {
            clause: "precondition",
            ..shape
        },
        |shape| shape,
        |shape| Shape {
            member: Member::Literal,
            ..shape
        },
        |shape| Shape {
            condition: Condition::BalanceAgainstAudit,
            ..shape
        },
        |shape| Shape {
            frame_effect: FrameEffect::GrantsRelationship,
            ..shape
        },
        |shape| Shape {
            modifies: &["audit", "balance"],
            ..shape
        },
        |shape| Shape {
            balance_bound: None,
            ..shape
        },
        |shape| Shape {
            balance_bound: None,
            audit_unbounded: true,
            ..shape
        },
        |shape| Shape {
            reaches_function: true,
            ..shape
        },
        |shape| Shape {
            condition: Condition::Negated,
            ..shape
        },
        |shape| Shape {
            condition: Condition::BalanceAgainstLiteral,
            ..shape
        },
        |shape| Shape {
            condition: Condition::Conjunction,
            ..shape
        },
        |shape| Shape {
            condition: Condition::BalanceAgainstOther,
            ..shape
        },
        |shape| Shape {
            condition: Condition::PostGePost,
            ..shape
        },
        |shape| Shape {
            frame_effect: FrameEffect::Creates,
            ..shape
        },
        |shape| Shape {
            frame_effect: FrameEffect::Deletes,
            ..shape
        },
        |shape| Shape {
            frame_effect: FrameEffect::GrantsForeignField,
            ..shape
        },
        |shape| Shape {
            member: Member::RationalBound,
            balance_bound: None,
            ..shape
        },
        |shape| Shape {
            member: Member::WideRange,
            balance_bound: None,
            ..shape
        },
        |shape| Shape {
            condition_field: "type",
            ..shape
        },
        |shape| shape,
        |shape| shape,
        |shape| shape,
        |shape| Shape {
            condition: Condition::Negated,
            modifies: &["audit", "balance"],
            ..shape
        },
        |shape| Shape {
            frame_effect: FrameEffect::DeletesSelf,
            ..shape
        },
        |shape| Shape {
            frame_effect: FrameEffect::DeletesParameter,
            ..shape
        },
    ];
    edits
        .into_iter()
        .enumerate()
        .map(|(index, edit)| {
            let scope = u32::try_from(index + 1).expect("a small index");
            edit(Shape {
                variant: 40 + scope,
                scope,
                ..Shape::HEALTHY
            })
        })
        .collect()
}

/// A package of the selected shapes, with their original arm-table indices.
struct World {
    package: CheckedPackageV2,
    ids: BTreeMap<usize, Ids>,
    function: CheckedNodeId,
}

fn world_for(indices: &[usize]) -> World {
    let shapes = arm_shapes();
    let selected = indices
        .iter()
        .copied()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .map(|index| (index, &shapes[index]))
        .collect::<Vec<_>>();
    // The selected shape group can exceed the default byte ceiling. Each test selects only the
    // shapes whose clauses it uses.
    let limits = CheckedPackageReadLimits {
        bytes: 8 << 20,
        ..CheckedPackageReadLimits::bounded()
    };
    let (package, fixture_ids) =
        package_of(&selected.iter().map(|(_, shape)| *shape).collect::<Vec<_>>())
            .admit_with_resolved(limits);
    World {
        package,
        function: fixture_ids.resolve(&code_id(FUNCTION)),
        ids: selected
            .iter()
            .map(|(index, shape)| (*index, ids(shape).resolved(&fixture_ids)))
            .collect(),
    }
}

impl World {
    fn ids(&self, shape: usize) -> &Ids {
        &self.ids[&shape]
    }
}

/// The subject the request's own `subject_path` names, which no `StateFrame` item reads.
const REQUEST_SUBJECT: &str = "crate::request::unrelated";

/// The fields of a `StateFrame` item, so a test can change one and keep the rest.
#[derive(Clone, Copy)]
struct Spec<'a> {
    package: &'a CheckedPackageV2,
    clause: &'a CheckedNodeId,
    role: StateFrameRole,
    state_path: &'a str,
    state_fields: &'a [&'a str],
    subject_path: &'a str,
}

impl<'a> Spec<'a> {
    fn item(self) -> ObligationItem<'a> {
        ObligationItem::StateFrame {
            package: self.package,
            clause: self.clause,
            role: self.role,
            state_path: self.state_path,
            state_fields: self.state_fields,
            subject_path: self.subject_path,
        }
    }
}

fn selected_spec<'a>(
    package: &'a CheckedPackageV2,
    clause: &'a CheckedNodeId,
    role: StateFrameRole,
    state_fields: &'a [&'a str],
) -> Spec<'a> {
    Spec {
        package,
        clause,
        role,
        state_path: STATE_PATH,
        state_fields,
        subject_path: SUBJECT_PATH,
    }
}

fn spec(world: &World, shape: usize, role: StateFrameRole) -> Spec<'_> {
    Spec {
        package: &world.package,
        clause: &world.ids(shape).clause,
        role,
        state_path: STATE_PATH,
        state_fields: &STATE_FIELDS,
        subject_path: SUBJECT_PATH,
    }
}

fn item(world: &World, shape: usize, role: StateFrameRole) -> ObligationItem<'_> {
    spec(world, shape, role).item()
}

fn negotiate(items: &[ObligationItem<'_>]) -> KaniObligationOutcome {
    negotiate_kani_obligations(&KaniObligationRequest {
        ceilings: crate::common::proof_ceilings::proof_ceilings_with_wall_clock(KANI_TIMEOUT),
        items,
        subject_path: REQUEST_SUBJECT,
        unwind: 4,
    })
    .expect("a well-formed request")
}

/// The record `item` has in a request holding it alone, placed at `request_index`.
fn alone(item: ObligationItem<'_>, request_index: usize) -> ObligationRecord {
    let record = negotiate(&[item]).records()[0].clone();
    ObligationRecord {
        request_index,
        ..record
    }
}

fn refusal_of(record: &ObligationRecord) -> &StateFrameRefusal {
    match &record.disposition {
        ObligationDisposition::Unsupported {
            reason: UnsupportedObligation::StateFrameRefused { refusal },
        } => refusal,
        other => panic!("expected unsupported state_frame_refused, got {other:?}"),
    }
}

fn is_supported(record: &ObligationRecord) -> bool {
    matches!(record.disposition, ObligationDisposition::Supported { .. })
}

fn emitted_state_frame(
    outcome: KaniObligationOutcome,
) -> (Vec<ObligationRecord>, Vec<StateFrameHarness>) {
    match outcome {
        KaniObligationOutcome::Emitted {
            records,
            state_frame_harnesses,
            ..
        } => (records, state_frame_harnesses),
        KaniObligationOutcome::Rejected { records } => panic!("unexpected rejection: {records:#?}"),
    }
}

/// One record per item in request order, `postcondition` for the contract role and `frame` for
/// the frame role, whatever an earlier item's refusal: a request of three refused items has three
/// records, and each later item's record is the record it has alone. An implementation that
/// returns at the first refusal reads one record, and fails the counts here.
#[trace("FR-015-AC-59", "TC-025")]
#[test]
fn tc_025_every_state_frame_item_has_a_record_whatever_an_earlier_item_refused() {
    use StateFrameRole::{Contract, Frame};
    let twin = Twin::with_unbounded_audit_clause();
    let (package, balance) = twin.emitted_package("BalanceNeverDrops");
    let (audit_package, audit) = twin.emitted_package("AuditNeverDrops");
    let (also_package, also) = twin.emitted_package("BalanceAlso");
    assert_eq!(package.name(), audit_package.name());
    assert_eq!(package.lock(), audit_package.lock());
    assert_eq!(package.name(), also_package.name());
    assert_eq!(package.lock(), also_package.lock());
    let items = [
        selected_spec(&package, &audit, Contract, &STATE_FIELDS).item(),
        selected_spec(&package, &balance, Contract, &STATE_FIELDS).item(),
        selected_spec(&package, &balance, Frame, &STATE_FIELDS).item(),
        selected_spec(&package, &also, Frame, &["balance"]).item(),
        selected_spec(&package, &audit, Frame, &STATE_FIELDS).item(),
    ];
    let (records, harnesses) = emitted_state_frame(negotiate(&items));
    assert_eq!(records.len(), items.len(), "one record per item");
    assert_eq!(
        records
            .iter()
            .map(|record| record.request_index)
            .collect::<Vec<_>>(),
        (0..items.len()).collect::<Vec<_>>()
    );
    let kinds = records.iter().map(|record| record.kind).collect::<Vec<_>>();
    let (post, frame) = (
        Some(ObligationKind::Postcondition),
        Some(ObligationKind::Frame),
    );
    assert_eq!(kinds, [post, post, frame, frame, frame]);
    assert!(matches!(
        records[0].disposition,
        ObligationDisposition::RequiresBound { .. }
    ));
    let supported = records.iter().map(is_supported).collect::<Vec<_>>();
    assert_eq!(supported, [false, true, true, false, false]);
    assert!(matches!(
        refusal_of(&records[3]),
        StateFrameRefusal::NothingForbidden { .. }
    ));
    assert!(matches!(
        records[4].disposition,
        ObligationDisposition::RequiresBound { .. }
    ));
    assert_eq!(harnesses.len(), 2, "one harness per supported record");
    // Each later item's record is the record it has alone.
    for (index, item) in items.iter().enumerate().skip(1) {
        assert_eq!(records[index], alone(*item, index), "item {index}");
    }

    // Three refused items are three records.
    let refused = [items[0], items[3], items[4]];
    let outcome = negotiate(&refused);
    assert_eq!(outcome.records().len(), 3);
    assert!(outcome.records().iter().all(|record| !is_supported(record)));

    // A later invalid item is a record too, and the request is rejected with every record.
    let bad_path = Spec {
        package: &package,
        clause: &also,
        role: Contract,
        state_path: "not a path",
        state_fields: &STATE_FIELDS,
        subject_path: SUBJECT_PATH,
    }
    .item();
    let outcome = negotiate(&[items[0], bad_path]);
    let KaniObligationOutcome::Rejected { records } = outcome else {
        panic!("an invalid item rejects the request");
    };
    assert_eq!(records.len(), 2);
    assert_eq!(records[1], alone(bad_path, 1));
}

/// A supported item's harness leaves in `state_frame_harnesses`, in request order and
/// byte-identical to the harness `generate_state_frame_obligations` returns for its clause and
/// role; the record carries its symbol. The harness calls the item's own subject path, and the
/// request's `subject_path` is not read.
///
/// This integration test is traced only to TC-025: it compares with the public two-role entry,
/// while the crate-internal role-parity test backs FR-015-AC-60 by comparing both direct roles
/// with their negotiated items over a QSL-emitted fixture.
#[trace("TC-025")]
#[test]
fn tc_025_a_supported_state_frame_item_returns_the_harness_the_engine_generates() {
    use StateFrameRole::{Contract, Frame};
    let fixture = emitted_fixture(&Twin::new(), "BalanceNeverDrops");
    let selected = |role| ObligationItem::StateFrame {
        package: &fixture.package,
        clause: &fixture.clause,
        role,
        state_path: STATE_PATH,
        state_fields: &STATE_FIELDS,
        subject_path: SUBJECT_PATH,
    };
    let items = [selected(Frame), selected(Contract)];
    let (records, harnesses) = emitted_state_frame(negotiate(&items));
    let engine = generate_state_frame_obligations(&StateFrameRequest {
        ceilings: crate::common::proof_ceilings::proof_ceilings_with_wall_clock(KANI_TIMEOUT),
        package: &fixture.package,
        clause: &fixture.clause,
        state_path: STATE_PATH,
        state_fields: &STATE_FIELDS,
        subject_path: SUBJECT_PATH,
        unwind: 4,
    })
    .expect("the selected clause generates");
    // Request order: the frame item came first.
    assert_eq!(
        harnesses,
        vec![engine.frame.clone(), engine.postcondition.clone()]
    );
    for (record, harness) in records.iter().zip(&harnesses) {
        assert_eq!(
            record.disposition,
            ObligationDisposition::Supported {
                harness_symbol: harness.identity.harness_symbol.clone()
            }
        );
    }
    // The harness calls the item's own subject, never the request's.
    let own_subject = Spec {
        package: &fixture.package,
        clause: &fixture.clause,
        role: Contract,
        state_path: STATE_PATH,
        state_fields: &STATE_FIELDS,
        subject_path: "crate::subject::withdraw",
    }
    .item();
    let (_, harnesses) = emitted_state_frame(negotiate(&[own_subject]));
    let source = &harnesses[0].rust.contents;
    assert!(source.contains("crate::subject::withdraw(&mut post)"));
    assert!(!source.contains(REQUEST_SUBJECT));
    assert_eq!(
        harnesses[0].identity.subject_path,
        "crate::subject::withdraw"
    );
}

/// An unbounded integer read gives IR's lowering `RequiresBound` record. Both roles carry its
/// exact type identity, while a bounded sibling clause in the same selected package still emits.
#[trace("FR-015-AC-61", "TC-025")]
#[test]
fn tc_025_a_state_frame_item_with_no_bound_is_requires_bound() {
    use StateFrameRole::{Contract, Frame};
    let twin = Twin::with_unbounded_audit_clause();
    let (package, audit) = twin.emitted_package("AuditNeverDrops");
    let balance = twin.emitted_package("BalanceNeverDrops").1;
    let expected = generate_state_frame_obligations(&StateFrameRequest {
        ceilings: crate::common::proof_ceilings::proof_ceilings_with_wall_clock(KANI_TIMEOUT),
        package: &package,
        clause: &audit,
        state_path: STATE_PATH,
        state_fields: &STATE_FIELDS,
        subject_path: SUBJECT_PATH,
        unwind: 4,
    })
    .expect_err("the unbounded audit read does not lower");
    let unbounded_type = match expected {
        StateFrameRefusal::NotLowered { refusal } => match *refusal {
            StateFrameLoweringRefusal::RequiresBound { unbounded_type, .. } => unbounded_type,
            other => panic!("expected RequiresBound, got {other:?}"),
        },
        other => panic!("expected a lowering refusal, got {other:?}"),
    };
    let items = [
        selected_spec(&package, &audit, Contract, &STATE_FIELDS).item(),
        selected_spec(&package, &audit, Frame, &STATE_FIELDS).item(),
        selected_spec(&package, &balance, Frame, &STATE_FIELDS).item(),
    ];
    let (records, harnesses) = emitted_state_frame(negotiate(&items));
    let requires_bound = |record: &ObligationRecord| match &record.disposition {
        ObligationDisposition::RequiresBound { unbounded_type } => unbounded_type.clone(),
        other => panic!("expected requires_bound, got {other:?}"),
    };
    assert_eq!(requires_bound(&records[0]), unbounded_type);
    assert_eq!(requires_bound(&records[1]), unbounded_type);
    assert!(is_supported(&records[2]));
    assert_eq!(harnesses.len(), 1);
    assert_eq!(harnesses[0].identity.clause, balance);
}

/// A lowering record for an unsupported family is `unsupported` `NoFiniteEncoding` naming the
/// record's node and family, and a node that is not a `state_clause` is `UnknownNodeKind`; neither
/// has a harness.
#[trace("FR-015-AC-62", "TC-025")]
#[test]
fn tc_025_a_state_frame_item_outside_the_encoding_has_a_named_reason() {
    use StateFrameRole::{Contract, Frame};
    let world = world_for(&[OK, REACHES_FUNCTION]);
    let frame_node = Spec {
        clause: &world.ids(OK).frame,
        ..spec(&world, OK, Contract)
    }
    .item();
    let items = [
        item(&world, REACHES_FUNCTION, Contract),
        item(&world, REACHES_FUNCTION, Frame),
        frame_node,
    ];
    let (records, harnesses) = emitted_state_frame(negotiate(&items));
    let no_finite_encoding = ObligationDisposition::Unsupported {
        reason: UnsupportedObligation::NoFiniteEncoding {
            node_id: world.function.clone(),
            node_tag: "function",
        },
    };
    assert_eq!(records[0].disposition, no_finite_encoding);
    assert_eq!(records[1].disposition, no_finite_encoding);
    assert_eq!(
        records[2].disposition,
        ObligationDisposition::Unsupported {
            reason: UnsupportedObligation::UnknownNodeKind {
                node_id: world.ids(OK).frame.clone(),
                node_tag: "state".to_owned(),
                semantic_form: "frame".to_owned(),
            }
        }
    );
    assert!(harnesses.is_empty());
}

/// Every shape the arm does not render is `unsupported` `StateFrameRefused` carrying the
/// engine's refusal, never `NoFiniteEncoding`, and has no harness. Each row names the role whose
/// harness the ground refuses; the rows that ground both roles are asked of both. A row fails if
/// its refusal moves to another reason or to another variant.
#[trace("FR-015-AC-63", "TC-025")]
#[test]
fn tc_025_a_state_frame_item_the_arm_does_not_render_carries_the_engines_refusal() {
    use StateFrameRole::{Contract, Frame};
    type Check = fn(&StateFrameRefusal) -> bool;
    let rows: Vec<(&str, usize, StateFrameRole, Check)> = vec![
        ("negation", NEGATION, Contract, |refusal| {
            matches!(refusal, StateFrameRefusal::ConditionNotSupported { .. })
        }),
        ("literal operand", LITERAL_OPERAND, Contract, |refusal| {
            matches!(refusal, StateFrameRefusal::ConditionNotSupported { .. })
        }),
        (
            "operator outside the six",
            CONJUNCTION,
            Contract,
            |refusal| matches!(refusal, StateFrameRefusal::ConditionNotSupported { .. }),
        ),
        ("another parameter", OTHER_PARAMETER, Contract, |refusal| {
            matches!(refusal, StateFrameRefusal::ConditionNotSupported { .. })
        }),
        ("two fields", TWO_FIELDS, Contract, |refusal| {
            matches!(
                refusal,
                StateFrameRefusal::ObservationsDiffer { left, right }
                    if left == "balance" && right == "audit"
            )
        }),
        ("one side twice", SAME_SIDE, Contract, |refusal| {
            matches!(refusal, StateFrameRefusal::ObservationsSameSide)
        }),
        ("frame creates", CREATES, Frame, |refusal| {
            matches!(
                refusal,
                StateFrameRefusal::FrameEffectUnsupported {
                    effect: UnsupportedFrameEffect::Creates,
                    ..
                }
            )
        }),
        ("frame deletes", DELETES, Frame, |refusal| {
            matches!(
                refusal,
                StateFrameRefusal::FrameEffectUnsupported {
                    effect: UnsupportedFrameEffect::Deletes,
                    ..
                }
            )
        }),
        ("frame deletes self", DELETES_SELF, Frame, |refusal| {
            matches!(
                refusal,
                StateFrameRefusal::FrameEffectUnsupported {
                    effect: UnsupportedFrameEffect::Deletes,
                    ..
                }
            )
        }),
        (
            "frame deletes a parameter",
            DELETES_PARAMETER,
            Frame,
            |refusal| {
                matches!(
                    refusal,
                    StateFrameRefusal::FrameEffectUnsupported {
                        effect: UnsupportedFrameEffect::Deletes,
                        ..
                    }
                )
            },
        ),
        (
            "frame grants a relationship",
            RELATIONSHIP_FRAME,
            Frame,
            |refusal| {
                matches!(
                    refusal,
                    StateFrameRefusal::FrameEffectUnsupported {
                        effect: UnsupportedFrameEffect::Relationship,
                        ..
                    }
                )
            },
        ),
        (
            "frame grants a foreign field",
            FOREIGN_FIELD,
            Frame,
            |refusal| {
                matches!(
                    refusal,
                    StateFrameRefusal::FrameEffectUnsupported {
                        effect: UnsupportedFrameEffect::ForeignField,
                        ..
                    }
                )
            },
        ),
        ("frame grants every field", GRANTS_ALL, Frame, |refusal| {
            matches!(refusal, StateFrameRefusal::NothingForbidden { .. })
        }),
        // Grounds of the clause itself refuse both roles.
        ("a precondition, contract", PRE, Contract, |refusal| {
            matches!(refusal, StateFrameRefusal::NotAPostcondition { .. })
        }),
        ("a precondition, frame", PRE, Frame, |refusal| {
            matches!(refusal, StateFrameRefusal::NotAPostcondition { .. })
        }),
        (
            "a malformed read, contract",
            KEYWORD_READ,
            Contract,
            |refusal| matches!(refusal, StateFrameRefusal::MalformedClause { .. }),
        ),
        ("a malformed read, frame", KEYWORD_READ, Frame, |refusal| {
            matches!(refusal, StateFrameRefusal::MalformedClause { .. })
        }),
    ];
    // Each group is one request over one package. Small packages keep the reader-oracle fixture
    // comfortably below its cumulative work cap without changing any row's refusal assertion.
    let mut checked_creates = false;
    for group in rows.chunks(4) {
        let world = world_for(
            &group
                .iter()
                .map(|(_, shape, _, _)| *shape)
                .collect::<Vec<_>>(),
        );
        let items = group
            .iter()
            .map(|(_, shape, role, _)| item(&world, *shape, *role))
            .collect::<Vec<_>>();
        let (records, harnesses) = emitted_state_frame(negotiate(&items));
        assert_eq!(records.len(), group.len());
        assert!(harnesses.is_empty(), "no refused item has a harness");
        for ((label, shape, _, check), record) in group.iter().zip(&records) {
            assert!(
                check(refusal_of(record)),
                "{label}: got {:?}",
                record.disposition
            );
            if *shape == CREATES {
                // The effect a frame refusal names is the frame's own node.
                let StateFrameRefusal::FrameEffectUnsupported { frame, .. } = refusal_of(record)
                else {
                    panic!("a frame effect");
                };
                assert_eq!(frame, &world.ids(CREATES).frame);
                checked_creates = true;
            }
        }
    }
    assert!(checked_creates, "the frame-creates row was checked");
}

/// An invalid item is a record of its own with its own code, the request is `Rejected` with every
/// item's record, and a supported item of the request is accounted but its bytes are withheld.
#[trace("FR-015-AC-64", "TC-025")]
#[test]
fn tc_025_an_invalid_state_frame_item_rejects_the_request_with_every_record() {
    use StateFrameRole::{Contract, Frame};
    let twin = Twin::with_distinct_balance_clauses();
    let fixture = emitted_fixture(&twin, "BalanceNeverDrops");
    let spare_a = twin.emitted_package("SpareA").1;
    let spare_b = twin.emitted_package("SpareB").1;
    let spare_c = twin.emitted_package("SpareC").1;
    let other = emitted_fixture(&Twin::build(&[], &CLAUSES, 0), "BalanceNeverDrops");
    let absent = package::id(&key(9999));
    let items = [
        selected_spec(&fixture.package, &fixture.clause, Contract, &STATE_FIELDS).item(),
        Spec {
            state_path: "not a path",
            ..selected_spec(&fixture.package, &spare_a, Contract, &STATE_FIELDS)
        }
        .item(),
        Spec {
            subject_path: "not a path",
            ..selected_spec(&fixture.package, &spare_a, Frame, &STATE_FIELDS)
        }
        .item(),
        Spec {
            state_fields: &["balance", "bad-name"],
            ..selected_spec(&fixture.package, &spare_b, Contract, &STATE_FIELDS)
        }
        .item(),
        Spec {
            state_fields: &["audit"],
            ..selected_spec(&fixture.package, &spare_c, Contract, &STATE_FIELDS)
        }
        .item(),
        Spec {
            clause: &absent,
            ..selected_spec(&fixture.package, &fixture.clause, Frame, &STATE_FIELDS)
        }
        .item(),
        selected_spec(&fixture.package, &fixture.clause, Contract, &STATE_FIELDS).item(),
        Spec {
            package: &other.package,
            clause: &other.clause,
            ..selected_spec(&fixture.package, &fixture.clause, Frame, &STATE_FIELDS)
        }
        .item(),
    ];
    let KaniObligationOutcome::Rejected { records } = negotiate(&items) else {
        panic!("an invalid item rejects the request, and a rejection carries no harness");
    };
    assert_eq!(records.len(), items.len());
    let invalid = |record: &ObligationRecord| match &record.disposition {
        ObligationDisposition::InvalidRequest { reason } => reason.clone(),
        other => panic!("expected invalid_request, got {other:?}"),
    };
    // Each engine-invalid item alone, beside a supported one, rejects the request: the engine's
    // invalid grounds are not only the arm's duplicate and mixed-package grounds.
    for engine_invalid in &items[1..=5] {
        let outcome = negotiate(&[items[0], *engine_invalid]);
        assert!(
            matches!(outcome, KaniObligationOutcome::Rejected { .. }),
            "{:?}",
            outcome.records()[1]
        );
    }
    assert!(is_supported(&records[0]), "accounted though withheld");
    assert_eq!(
        invalid(&records[1]),
        InvalidObligationItem::InvalidStatePath {
            path: "not a path".to_owned()
        }
    );
    assert_eq!(
        invalid(&records[2]),
        InvalidObligationItem::InvalidStatePath {
            path: "not a path".to_owned()
        }
    );
    assert_eq!(
        invalid(&records[3]),
        InvalidObligationItem::InvalidStateField {
            name: "bad-name".to_owned()
        }
    );
    assert_eq!(
        invalid(&records[4]),
        InvalidObligationItem::InvalidStateField {
            name: "balance".to_owned()
        }
    );
    assert_eq!(invalid(&records[5]), InvalidObligationItem::UnknownNode);
    assert_eq!(
        invalid(&records[6]),
        InvalidObligationItem::DuplicateItem { first_index: 0 }
    );
    assert_eq!(
        invalid(&records[7]),
        InvalidObligationItem::MixedStatePackages
    );
}

/// A request holding `StateFrame` items is refused whole, with the existing error and no record,
/// for no items, too many, an unparsable request subject path (checked though no `StateFrame`
/// item reads it) and an unwind bound outside the range.
#[trace("FR-015-AC-65", "TC-025")]
#[test]
fn tc_025_a_state_frame_request_is_refused_whole_for_the_requests_own_faults() {
    let world = world_for(&[OK]);
    let one = [item(&world, OK, StateFrameRole::Contract)];
    let refuse = |items: &[ObligationItem<'_>], subject_path: &str, unwind: u32| {
        negotiate_kani_obligations(&KaniObligationRequest {
            ceilings: crate::common::proof_ceilings::proof_ceilings_with_wall_clock(KANI_TIMEOUT),
            items,
            subject_path,
            unwind,
        })
        .expect_err("the request is refused")
    };
    assert_eq!(
        refuse(&[], REQUEST_SUBJECT, 4),
        KaniObligationError::EmptyRequest
    );
    let too_many = vec![one[0]; MAX_OBLIGATION_ITEMS + 1];
    assert_eq!(
        refuse(&too_many, REQUEST_SUBJECT, 4),
        KaniObligationError::TooManyItems {
            count: MAX_OBLIGATION_ITEMS + 1
        }
    );
    assert_eq!(
        refuse(&one, "not a path", 4),
        KaniObligationError::InvalidSubjectPath
    );
    assert_eq!(
        refuse(&one, REQUEST_SUBJECT, 0),
        KaniObligationError::InvalidUnwind { unwind: 0 }
    );
    assert_eq!(
        refuse(&one, REQUEST_SUBJECT, MAX_OBLIGATION_UNWIND + 1),
        KaniObligationError::InvalidUnwind {
            unwind: MAX_OBLIGATION_UNWIND + 1
        }
    );
}

/// The single-clause entry names the refusal it always named first for a clause both roles
/// refuse: the condition's shape before a frame granting every field. The first assertion fails
/// if the roles are simply run frame-first; the second pins the refusal for a missing granted
/// field and does not tell the orders apart.
#[trace("FR-015-AC-29", "TC-025")]
#[test]
fn tc_025_the_single_clause_entry_keeps_its_first_refusal_when_both_roles_refuse() {
    let world = world_for(&[NEGATION_GRANTS_ALL, GRANTS_ALL]);
    let refuse = |shape: usize, fields: &[&str]| {
        generate_state_frame_obligations(&StateFrameRequest {
            ceilings: crate::common::proof_ceilings::proof_ceilings_with_wall_clock(KANI_TIMEOUT),
            package: &world.package,
            clause: &world.ids(shape).clause,
            state_path: STATE_PATH,
            state_fields: fields,
            subject_path: SUBJECT_PATH,
            unwind: 4,
        })
        .expect_err("both roles refuse")
    };
    // A negation (contract) on a frame granting every field (frame).
    assert!(matches!(
        refuse(NEGATION_GRANTS_ALL, &STATE_FIELDS),
        StateFrameRefusal::ConditionNotSupported { .. }
    ));
    // A granted field the state lacks (frame) precedes the check that nothing is forbidden.
    assert_eq!(
        refuse(GRANTS_ALL, &["audit"]),
        StateFrameRefusal::UnknownStateField {
            field: "balance".to_owned()
        }
    );
}

/// The two roles of a clause settle independently: a frame granting every field leaves the
/// contract item `supported`, a negation leaves the frame item `supported`, and a refused
/// lowering refuses both alike. `generate_state_frame_obligations` returns the refusal of the
/// failing role, and a supported harness is the role's own.
///
/// This integration test is traced only to TC-025: it checks harness role and clause, while the
/// crate-internal role-parity test backs FR-015-AC-68 with direct `generate_state_frame_role`
/// comparisons over the same QSL-emitted fixture.
#[trace("TC-025")]
#[test]
fn tc_025_the_two_roles_of_a_state_clause_settle_independently() {
    use StateFrameRole::{Contract, Frame};
    let engine = |fixture: &Fixture| {
        generate_state_frame_obligations(&StateFrameRequest {
            ceilings: crate::common::proof_ceilings::proof_ceilings_with_wall_clock(KANI_TIMEOUT),
            package: &fixture.package,
            clause: &fixture.clause,
            state_path: STATE_PATH,
            state_fields: &STATE_FIELDS,
            subject_path: SUBJECT_PATH,
            unwind: 4,
        })
        .expect_err("the clause has a role the engine refuses")
    };
    let roles = |fixture: &Fixture| {
        let items = [
            selected_spec(&fixture.package, &fixture.clause, Contract, &STATE_FIELDS).item(),
            selected_spec(&fixture.package, &fixture.clause, Frame, &STATE_FIELDS).item(),
        ];
        emitted_state_frame(negotiate(&items))
    };
    let all = emitted_fixture(
        &Twin::build(&["balance", "audit"], &[CLAUSES[0]], 0),
        "BalanceNeverDrops",
    );
    let (records, harnesses) = roles(&all);
    assert_eq!(
        records.iter().map(is_supported).collect::<Vec<_>>(),
        [true, false]
    );
    assert_eq!(harnesses.len(), 1);
    assert!(matches!(
        harnesses[0].identity.property,
        StateFrameProperty::Postcondition { .. }
    ));
    assert_eq!(harnesses[0].identity.clause, all.clause);
    assert_eq!(&engine(&all), refusal_of(&records[1]));

    let negated = emitted_fixture(&Twin::with_negated_balance_clause(), "BalanceNeverDrops");
    let (records, harnesses) = roles(&negated);
    assert_eq!(
        records.iter().map(is_supported).collect::<Vec<_>>(),
        [false, true]
    );
    assert_eq!(harnesses.len(), 1);
    assert!(matches!(
        harnesses[0].identity.property,
        StateFrameProperty::Frame { .. }
    ));
    assert_eq!(harnesses[0].identity.clause, negated.clause);
    assert_eq!(&engine(&negated), refusal_of(&records[0]));

    let unbounded = emitted_fixture(&Twin::with_unbounded_audit_clause(), "AuditNeverDrops");
    let (records, harnesses) = roles(&unbounded);
    assert_eq!(
        records.iter().map(is_supported).collect::<Vec<_>>(),
        [false, false]
    );
    assert!(harnesses.is_empty());
    // A refused lowering refuses both items alike.
    assert_eq!(records[0].disposition, records[1].disposition);
    assert!(matches!(
        engine(&unbounded),
        StateFrameRefusal::NotLowered { .. }
    ));
}

/// `replay_frame` refuses an envelope whose `clause_node` or `occurrence_key` is not its
/// payload's, naming both; the same run with an agreeing envelope replays, so each refusal is the
/// difference and not the run.
#[trace("TC-025")]
#[test]
fn tc_025_replay_frame_refuses_an_envelope_that_disagrees_with_its_payload() {
    let twin = Twin::new();
    let run = forbidden_run(&twin);
    let invocation = twin.invocation("account", (5, 0), (6, 0));
    assert!(twin
        .replay_tampered(&invocation, "account", "audit", &run, Tamper::Nothing)
        .is_ok());

    let clause = twin.replay_tampered(&invocation, "account", "audit", &run, Tamper::ClauseNode);
    assert!(matches!(
        clause,
        Err(ReplayRefusal::FrameIdentity(mismatch))
            if matches!(*mismatch, FrameIdentityMismatch::EnvelopeFrame { .. })
    ));
    let occurrence =
        twin.replay_tampered(&invocation, "account", "audit", &run, Tamper::Occurrence);
    assert!(matches!(
        occurrence,
        Err(ReplayRefusal::FrameIdentity(mismatch))
            if matches!(*mismatch, FrameIdentityMismatch::EnvelopeOccurrence { .. })
    ));
}

/// The frame-replay payload's identities are QSL's answer for the operation, and the envelope
/// names the payload's own frame node and frame occurrence.
#[trace("FR-015-AC-33", "FR-015-AC-34", "TC-025")]
#[test]
fn tc_025_the_frame_replay_envelope_names_the_payloads_frame_and_occurrence() {
    let twin = Twin::new();
    let invocation = twin.invocation("account", (5, 0), (6, 1));
    let replay = twin.frame_replay(&invocation, "account", "audit", &forbidden_run(&twin));
    let payload = replay.packet.family_payload.as_ref().expect("a payload");
    assert_ne!(payload.anchor, payload.frame);
    assert_eq!(payload.occurrence.node(), &payload.frame);
    assert_eq!(replay.packet.clause_node.as_ref(), Some(&payload.frame));
    assert_eq!(
        replay.packet.occurrence_key.as_ref(),
        Some(&payload.occurrence)
    );
}

/// The non-Kani frame path carries QSL's call-site identities through the packet and succeeds
/// through `FrameReplay::replay`, whose implementation calls `qsl_replay::replay_frame`.
#[trace("TC-035", "FR-024-AC-6", "FR-024-AC-41")]
#[test]
fn tc_035_frame_packet_reaches_qsl_with_call_site_identities() {
    let twin = Twin::new();
    let site = twin.operation_site("deposit").expect("operation site");
    let invocation = twin.invocation("account", (5, 0), (6, 1));
    let replay = twin.frame_replay(&invocation, "account", "audit", &forbidden_run(&twin));
    let payload = replay
        .packet
        .family_payload
        .as_ref()
        .expect("frame payload");
    assert_eq!(payload.frame, site.frame);
    assert_eq!(payload.anchor, site.anchor);
    assert_eq!(payload.occurrence, site.frame_occurrence);
    assert_eq!(replay.packet.clause_node, Some(site.frame));
    assert_eq!(
        replay.packet.replay_target.frame_operation.as_ref(),
        Some(&site.frame_operation)
    );
    assert!(replay.packet.replay_target.function.is_none());
    assert!(replay.packet.replay_target.state_clause.is_none());
    assert_eq!(
        replay.wire.replay_target.members,
        replay.packet.replay_target
    );
    assert!(!replay.wire.replay_target.arguments);
    assert!(replay.wire.replay_target.occurrence_key.is_none());
    assert_eq!(
        replay.packet.occurrence_key.as_ref(),
        Some(&site.frame_occurrence)
    );
    let result = replay.replay().expect("QSL admits the frame envelope");
    assert!(matches!(result.result(), ReplayResult::Witness(_)));
}

/// `FrameReplay::replay` returns QSL's result without Kani: a forbidden write settles a reproduced
/// violation that names the written field, and a write the frame grants is a respected frame.
#[trace("FR-015-AC-36", "TC-025")]
#[test]
fn tc_025_frame_replay_settles_a_forbidden_and_a_granted_write() {
    let twin = Twin::new();
    let run = forbidden_run(&twin);
    let forbidden = twin.invocation("account", (5, 0), (6, 1));
    let result = twin
        .frame_replay(&forbidden, "account", "audit", &run)
        .replay()
        .expect("the replay settles");
    let ReplayResult::Witness(arm) = result.result() else {
        panic!("a witness-sourced replay settles on the witness arm");
    };
    assert_eq!(
        arm.settlement(),
        WitnessSettlement::ReproducedWithEvaluatedWitness
    );
    assert_eq!(arm.category(), Category::Violation);
    let Some(FrameChange::FieldWrite { object, field, .. }) =
        result.found().map(|found| &found.change)
    else {
        panic!("the replay found a field write: {:?}", result.found());
    };
    assert_eq!((object.as_str(), field.as_str()), ("account", "audit"));

    let granted = twin.invocation("account", (5, 0), (6, 0));
    let result = twin
        .frame_replay(&granted, "account", "balance", &run)
        .replay()
        .expect("the replay settles");
    let ReplayResult::Witness(arm) = result.result() else {
        panic!("a witness-sourced replay settles on the witness arm");
    };
    assert_eq!(arm.settlement(), WitnessSettlement::Inconclusive);
    assert_eq!(
        arm.disagreement(),
        Some(&DisagreementCause::Verdicts {
            proved: Verdict::from_category(Category::Violation),
            replayed: Verdict::from_category(Category::Success),
        })
    );
    assert!(result.found().is_none());
}

/// An operation the unit names no frame for is refused by the call site before any request is
/// built, whether the domain package declares it (`transfer`, which no clause names) or not
/// (`withdraw`).
#[trace("FR-015-AC-35", "TC-025")]
#[test]
fn tc_025_an_operation_with_no_frame_is_refused_when_the_request_is_built() {
    let twin = Twin::new();
    let invocation = twin.invocation("account", (5, 0), (6, 0));
    for operation in ["transfer", "withdraw"] {
        let mut harness = frame_harness(&twin);
        harness.scope.operation = operation.to_owned();
        let run = run_of(&harness, [5, 0]);
        let refusal = twin
            .try_frame_replay(operation, &invocation, "account", "audit", &run)
            .err()
            .unwrap_or_else(|| panic!("the unit names no frame for `{operation}`"));
        assert!(
            matches!(
                &refusal,
                FrameReplayError::CallSite(cause)
                    if matches!(
                        &**cause,
                        CallSiteRefusal::UnknownOperation { selection, .. }
                            if selection.operation.as_str() == operation
                    )
            ),
            "{operation}: {refusal}"
        );
    }
}

// ---- frame obligation identity, witness and ties (FR-024, IR-459) -------------

/// The refusal of a replay that must be refused.
fn refused(result: Result<FrameReplay, FrameReplayError>) -> FrameReplayError {
    match result {
        Ok(_) => panic!("the replay must be refused"),
        Err(error) => error,
    }
}

/// The frame replay of `operation` over `run` at the pre state `(5, 0)`.
fn replay_of(twin: &Twin, operation: &str, run: &Run) -> Result<FrameReplay, FrameReplayError> {
    let invocation = twin.invocation("account", (5, 0), (6, 1));
    twin.try_frame_replay(operation, &invocation, "account", "audit", run)
}

/// The request's and the envelope's obligation identity equal the structural identity of the
/// site `qsl_replay::call_site` returns for the operation, and the envelope's
/// `clause_node` and `occurrence_key` are the same `function` and `declaration` members.
#[trace(
    "FR-024-AC-47",
    "FR-024-AC-48",
    "FR-024-AC-50",
    "FR-024-AC-51",
    "TC-035"
)]
#[test]
fn tc_035_the_request_and_envelope_carry_the_producer_frame_claim() {
    let twin = Twin::new();
    let site = twin
        .operation_site("deposit")
        .expect("the operation is located");
    let replay = replay_of(&twin, "deposit", &forbidden_run(&twin)).expect("builds");
    let expected = producer_frame_identity(&site);
    assert_eq!(replay.wire.obligation_identity, expected);
    assert_eq!(replay.packet.obligation_identity, Some(expected.clone()));
    assert_eq!(expected.model(), site.obligation_identity().model());
    assert_eq!(expected.ordinal(), site.obligation_identity().ordinal());
    assert_eq!(replay.packet.clause_node, Some(site.frame.clone()));
    assert_eq!(
        replay.packet.occurrence_key.as_ref(),
        Some(&site.frame_occurrence)
    );
    assert_eq!(
        expected.node(),
        &site.frame,
        "no stand-in node reaches the request"
    );
    assert!(
        !replay.wire.replay_target.arguments,
        "a frame has no arguments"
    );
    assert!(replay.wire.replay_target.occurrence_key.is_none());
    let payload = replay
        .packet
        .family_payload
        .as_ref()
        .expect("frame payload");
    assert_eq!(payload.frame, site.frame);
    assert_eq!(payload.occurrence, site.frame_occurrence);
    replay.replay().expect("equal producer claim replays");
}

/// A packet cannot replace the selected frame claim's QSL identity while the request keeps it.
/// This checks the actual replay entry, after the unmodified pair has settled successfully.
#[trace("FR-024-AC-47", "FR-024-AC-51", "TC-035")]
#[test]
fn tc_035_a_changed_envelope_identity_refuses_before_frame_replay() {
    let twin = Twin::new();
    let run = forbidden_run(&twin);
    let baseline = replay_of(&twin, "deposit", &run).expect("frame replay builds");
    baseline.replay().expect("the matching claim settles");

    let mut changed = replay_of(&twin, "deposit", &run).expect("frame replay builds");
    let original = changed.wire.obligation_identity.clone();
    changed.packet.obligation_identity = Some(ObligationIdentity::new(
        original.node().clone(),
        original.model().cloned(),
        original.ordinal() + 1,
    ));
    assert_ne!(changed.packet.obligation_identity, Some(original));
    assert!(
        matches!(
            changed.replay(),
            Err(FrameReplayError::ClaimMismatch(
                quire_contract_codegen::FrameClaimMember::EnvelopeIdentity
            ))
        ),
        "a changed envelope identity must refuse before frame execution"
    );
}

/// A held frame node is insufficient if its harness changes which state field it grants.
/// Both field lists still cover the same state fields, so the full selected-claim comparison is
/// required after the cheap field-set check.
#[trace("FR-024-AC-49", "FR-024-AC-51", "TC-035")]
#[test]
fn tc_035_a_held_frame_node_rejects_changed_grants() {
    let twin = Twin::new();
    let baseline = frame_harness(&twin);
    let mut changed = baseline.clone();
    changed.property = StateFrameProperty::Frame {
        granted: vec!["audit".to_owned()],
        checked: vec!["balance".to_owned()],
    };
    assert_ne!(changed.property, baseline.property);
    assert_eq!(changed.scope.frame, baseline.scope.frame);
    let error = refused(replay_of(&twin, "deposit", &run_of(&changed, [5, 0])));
    assert!(
        matches!(error, FrameReplayError::FieldSetMismatch { .. }),
        "a changed grant under the held frame node must refuse: {error}"
    );
}

/// A harness whose frame grants nothing, replayed over a unit whose frame grants nothing, gives
/// a different identity in the request and in the envelope; each is the minted identity of its
/// own site.
#[trace("FR-024-AC-49", "FR-024-AC-51", "TC-035")]
#[test]
fn tc_035_a_changed_grant_changes_the_identity_in_the_request_and_the_envelope() {
    let twin = Twin::new();
    let base = replay_of(&twin, "deposit", &forbidden_run(&twin)).expect("builds");

    let granting_nothing = Twin::build(&[], &CLAUSES, 0);
    let harness = generated_from_twin(
        &granting_nothing,
        "BalanceNeverDrops",
        &STATE_FIELDS,
        SUBJECT_PATH,
    )
    .frame
    .identity;
    let changed =
        replay_of(&granting_nothing, "deposit", &run_of(&harness, [5, 0])).expect("builds");

    assert_ne!(
        changed.wire.obligation_identity,
        base.wire.obligation_identity
    );
    assert_ne!(
        changed.packet.obligation_identity,
        base.packet.obligation_identity
    );
    let site = granting_nothing
        .operation_site("deposit")
        .expect("the operation is located");
    assert_eq!(
        changed.wire.obligation_identity,
        producer_frame_identity(&site)
    );
    assert_eq!(
        changed.packet.obligation_identity,
        Some(changed.wire.obligation_identity.clone())
    );
}

/// The identity is measured through `qsl_replay::call_site`: it changes with the frame's grants
/// and between two operations of one object whose frames are equal text, and it does not change
/// when the unit is shifted by blank lines or when a second clause names the same operation.
#[trace("FR-024-AC-47", "FR-024-AC-49", "TC-035")]
#[test]
fn tc_035_the_frame_identity_follows_what_call_site_names() {
    let identity = |twin: &Twin, operation: &str| {
        producer_frame_identity(&twin.operation_site(operation).expect("operation site"))
    };
    let twin = Twin::new();
    let base = identity(&twin, "deposit");

    // Two units that differ only in the frame's `modifies` grants.
    let both = Twin::build(&["balance", "audit"], &CLAUSES, 0);
    assert_ne!(base, identity(&both, "deposit"), "grants");

    // Two operations of one object whose frames are equal text.
    let two_operations = Twin::build(
        &model::GRANTED,
        &[CLAUSES[0], ("TransferKeepsBalance", "transfer", "balance")],
        0,
    );
    assert_ne!(
        identity(&two_operations, "deposit"),
        identity(&two_operations, "transfer"),
        "operations with equal frame text"
    );

    // The same operation in a unit shifted by blank lines.
    let shifted = Twin::build(&model::GRANTED, &CLAUSES, 3);
    assert_eq!(base, identity(&shifted, "deposit"), "blank lines");

    // Two postcondition clauses of one operation: one frame, whatever the harness's clause.
    let first = twin.clause_site("BalanceNeverDrops").expect("a clause");
    let second = twin.clause_site("AuditNeverDrops").expect("a clause");
    assert_ne!(first.node, second.node, "the clauses are two nodes");
    let mut harness = frame_harness(&twin);
    harness.clause = harness.scope.object.clone();
    assert_eq!(base, minted(&twin, "deposit", &run_of(&harness, [5, 0])));

    // A unit with one postcondition clause on `deposit` and the unit with two mint one identity.
    let one_clause = Twin::build(&model::GRANTED, &[CLAUSES[0]], 0);
    assert_eq!(
        base,
        identity(&one_clause, "deposit"),
        "one clause against two on one operation"
    );
}

/// The edge the spec states and does not hide: occurrence ordinals run over the operations the
/// unit's clauses name, so a clause added on an operation that sorts earlier moves this
/// operation's frame occurrence. The identity of `transfer` is read before and after a clause on
/// `deposit` is added, and it differs.
#[trace("FR-024-AC-49", "TC-035")]
#[test]
fn tc_035_a_clause_on_an_earlier_sorting_operation_moves_the_frame_occurrence() {
    let identity = |twin: &Twin| {
        producer_frame_identity(&twin.operation_site("transfer").expect("operation site"))
    };
    let transfer = ("TransferKeepsBalance", "transfer", "balance");
    let alone = Twin::build(&model::GRANTED, &[transfer], 0);
    let after = Twin::build(&model::GRANTED, &[CLAUSES[0], transfer], 0);
    assert_ne!(identity(&alone), identity(&after));
}

/// The identity does not change when any one of the harness's own members changes: its clause
/// node, module symbol, harness symbol, state path, subject path, unwind bound, options, the
/// ranges of its fields or the order of its state fields. The solver is not edited: `KaniSolver`
/// has one variant (`Cadical`), so no other solver can be built, and the identity function takes
/// no solver.
#[trace("FR-024-AC-47", "FR-024-AC-49", "TC-035")]
#[test]
fn tc_035_the_frame_identity_names_none_of_the_harness_members() {
    let twin = Twin::new();
    let harness = frame_harness(&twin);
    let base = minted(&twin, "deposit", &run_of(&harness, [5, 0]));
    type Edit = Box<dyn Fn(&mut StateFrameIdentity)>;
    let edits: Vec<(&str, Edit)> = vec![
        (
            "clause node",
            Box::new(|h| h.clause = h.scope.object.clone()),
        ),
        (
            "module symbol",
            Box::new(|h| h.module_symbol = ModuleSymbol::try_from("renamed_module").unwrap()),
        ),
        (
            "harness symbol",
            Box::new(|h| h.harness_symbol = HarnessSymbol::try_from("renamed_check").unwrap()),
        ),
        (
            "state path",
            Box::new(|h| h.state_path = "crate::other::State".to_owned()),
        ),
        (
            "subject path",
            Box::new(|h| h.subject_path = "crate::other::operate".to_owned()),
        ),
        ("unwind", Box::new(|h| h.unwind += 3)),
        (
            "options",
            Box::new(|h| h.options.push("--extra".to_owned())),
        ),
        (
            "ranges",
            Box::new(|h| {
                for domain in &mut h.domains {
                    domain.minimum -= 10;
                    domain.maximum += 10;
                }
            }),
        ),
    ];
    for (name, edit) in edits {
        let mut edited = harness.clone();
        edit(&mut edited);
        assert_ne!(edited, harness, "{name} is an edit");
        assert_eq!(
            base,
            minted(&twin, "deposit", &run_of(&edited, [5, 0])),
            "{name}"
        );
    }
    let mut reordered = harness.clone();
    reordered.state_fields.reverse();
    assert_eq!(
        base,
        minted(&twin, "deposit", &run_of(&reordered, [0, 5])),
        "state field order"
    );
}

/// A harness whose property is a postcondition is `NotAFrame`, and a frame whose granted and
/// checked fields are not exactly the state fields is `FieldSetMismatch`, whether a field is
/// neither granted nor checked, granted and checked, or no state field at all. Each is refused
/// before the call site: the operation the unit does not name would otherwise be a
/// `CallSite` refusal, as it is for the harness that passes the checks.
#[trace("FR-024-AC-50", "TC-035")]
#[test]
fn tc_035_a_non_frame_and_a_mismatched_field_set_are_refused_before_the_call_site() {
    let twin = Twin::new();
    let mut base = frame_harness(&twin);
    base.scope.operation = "withdraw".to_owned();
    let refusal = |harness: &StateFrameIdentity| {
        refused(replay_of(&twin, "withdraw", &run_of(harness, [5, 0])))
    };
    assert!(
        matches!(refusal(&base), FrameReplayError::CallSite(_)),
        "the control reaches the call site"
    );

    let mut postcondition = base.clone();
    postcondition.property =
        generated_from_twin(&twin, "BalanceNeverDrops", &STATE_FIELDS, SUBJECT_PATH)
            .postcondition
            .identity
            .property;
    assert!(matches!(
        refusal(&postcondition),
        FrameReplayError::NotAFrame
    ));

    let frame = |granted: &[&str], checked: &[&str]| {
        let mut harness = base.clone();
        harness.property = StateFrameProperty::Frame {
            granted: granted.iter().map(|field| (*field).to_owned()).collect(),
            checked: checked.iter().map(|field| (*field).to_owned()).collect(),
        };
        harness
    };
    for (granted, checked) in [
        (&["balance"][..], &[][..]),
        (&["balance"], &["audit", "balance"]),
        (&["balance"], &["audit", "other"]),
        (&["balance", "audit"], &["audit"]),
    ] {
        let error = refusal(&frame(granted, checked));
        let FrameReplayError::FieldSetMismatch {
            state_fields,
            granted: held,
            checked: also,
        } = error
        else {
            panic!("{granted:?} / {checked:?}: {error}");
        };
        assert_eq!(state_fields, ["balance", "audit"]);
        assert_eq!(held, granted);
        assert_eq!(also, checked);
    }
    assert!(
        matches!(
            refusal(&frame(&["audit"], &["balance"])),
            FrameReplayError::CallSite(_)
        ),
        "the same fields in the other split are a frame over the state"
    );
}

/// The harness record carries `state_fields` in the order the harness draws them, and the
/// identity read back from the record equals the generated one. A harness regenerated from equal
/// inputs has a byte-identical record, and a record without `state_fields` is not read as a
/// frame identity.
#[trace("FR-024-AC-23", "TC-035")]
#[test]
fn tc_035_the_record_carries_the_draw_order_and_a_record_without_it_is_not_read() {
    let fixture = emitted_fixture(&Twin::new(), "BalanceNeverDrops");
    let generated = generate(&fixture);
    let record: Value = serde_json::from_str(&generated.frame.record.contents).expect("record");
    assert_eq!(
        record["identity"]["state_fields"],
        json!(["balance", "audit"])
    );
    assert_eq!(generate(&fixture).frame.record, generated.frame.record);
    assert_eq!(
        StateFrameIdentity::from_record(&generated.frame.record.contents).expect("a frame record"),
        generated.frame.identity
    );

    let reversed = generate_state_frame_obligations(&StateFrameRequest {
        ceilings: crate::common::proof_ceilings::proof_ceilings_with_wall_clock(KANI_TIMEOUT),
        state_fields: &["audit", "balance"],
        ..request(&fixture, &STATE_FIELDS)
    })
    .expect("the fields generate in either order");
    let reversed_record: Value =
        serde_json::from_str(&reversed.frame.record.contents).expect("record");
    assert_eq!(
        reversed_record["identity"]["state_fields"],
        json!(["audit", "balance"])
    );
    assert_ne!(reversed.frame.record, generated.frame.record);

    let mut without = record.clone();
    without["identity"]
        .as_object_mut()
        .expect("an identity object")
        .remove("state_fields");
    assert!(StateFrameIdentity::from_record(&without.to_string()).is_err());
    let mut extra = record;
    extra["identity"]["unknown"] = json!(1);
    assert!(StateFrameIdentity::from_record(&extra.to_string()).is_err());
    assert!(StateFrameIdentity::from_record("{}").is_err());
}

/// A persisted unranged field records its type reason, and duplicate names cannot be decoded.
#[trace("FR-015-AC-81")]
#[test]
fn tc_025_unranged_fields_have_a_reason_and_no_duplicate_record_names() {
    let twin = Twin::without_audit_range();
    let generated = generated_from_twin(&twin, "BalanceNeverDrops", &STATE_FIELDS, SUBJECT_PATH);
    assert_eq!(generated.frame.identity.unranged.len(), 1);
    assert_eq!(generated.frame.identity.unranged[0].field, "audit");
    assert_eq!(
        generated.frame.identity.unranged[0].reason,
        quire_contract_codegen::StateUnrangedReason::TypeNotRange
    );
    let mut record: Value = serde_json::from_str(&generated.frame.record.contents).expect("record");
    let first = record["identity"]["unranged"][0].clone();
    record["identity"]["unranged"]
        .as_array_mut()
        .expect("unranged list")
        .push(first);
    assert!(matches!(
        StateFrameIdentity::from_record(&record.to_string()),
        Err(StateFrameRecordError::RepeatedField { field }) if field == "audit"
    ));
}

/// A present text field has no checked member type; a bounded integer field wider than `i64`
/// retains its exact `i128` endpoint. Neither becomes an `i64` assumption or disappears from the
/// persisted draw list.
#[trace("FR-015-AC-78", "FR-015-AC-81")]
#[test]
fn tc_025_model_member_types_without_i64_ranges_remain_unranged() {
    let cases = [
        (
            Twin::build(&["balance"], &[CLAUSES[0]], 0)
                .emitted_package_with_unread_string_audit("BalanceNeverDrops"),
            None,
        ),
        (
            Twin::build(&["balance"], &[CLAUSES[0]], 0)
                .emitted_package_with_wide_audit_range("BalanceNeverDrops"),
            Some((0_i128, 9_223_372_036_854_775_808_i128)),
        ),
    ];
    for ((package, clause), check) in cases {
        let object = package
            .graph()
            .nodes
            .iter()
            .find(|node| &*node.semantic_form == "object_type")
            .expect("object declaration");
        let fields = package
            .model_object_fields(&object.node_id)
            .expect("model table");
        let member_type = fields.field("audit").expect("present audit").member_type();
        match check {
            None => assert!(member_type.is_none()),
            Some((lower, upper)) => assert_eq!(
                member_type,
                Some(&CheckedMemberType::IntRange { lower, upper })
            ),
        }
        let generated = generate_state_frame_obligations(&StateFrameRequest {
            ceilings: crate::common::proof_ceilings::proof_ceilings_with_wall_clock(KANI_TIMEOUT),
            package: &package,
            clause: &clause,
            state_path: STATE_PATH,
            state_fields: &STATE_FIELDS,
            subject_path: SUBJECT_PATH,
            unwind: 4,
        })
        .expect("balance clause still generates");
        assert_eq!(generated.frame.identity.domains.len(), 1);
        assert_eq!(generated.frame.identity.domains[0].field, "balance");
        assert_eq!(generated.frame.identity.unranged.len(), 1);
        assert_eq!(generated.frame.identity.unranged[0].field, "audit");
        assert_eq!(
            generated.frame.identity.unranged[0].reason,
            quire_contract_codegen::StateUnrangedReason::TypeNotRange
        );
        assert_eq!(
            StateFrameIdentity::from_record(&generated.frame.record.contents)
                .expect("record decodes"),
            generated.frame.identity
        );
    }
}

/// An unread selected-model variant has a typed member but no integer range. The bounded
/// balance control still generates, while the variant stays unranged in the persisted identity.
#[trace("FR-015-AC-78", "FR-015-AC-81")]
#[test]
fn selected_model_variant_field_is_unranged_in_generated_frame_identity() {
    let (package, clause) = Twin::build(&["balance"], &[CLAUSES[0]], 0)
        .emitted_package_with_unread_variant_audit("BalanceNeverDrops");
    let object = package
        .graph()
        .nodes
        .iter()
        .find(|node| &*node.semantic_form == "object_type")
        .expect("object declaration");
    let fields = package
        .model_object_fields(&object.node_id)
        .expect("selected model fields");
    assert!(matches!(
        fields.field("audit").expect("present audit").member_type(),
        Some(CheckedMemberType::Variant(_))
    ));
    assert_eq!(
        fields
            .field("balance")
            .expect("present balance")
            .member_type(),
        Some(&CheckedMemberType::IntRange {
            lower: 0,
            upper: 1000,
        })
    );
    let generated = generate_state_frame_obligations(&StateFrameRequest {
        ceilings: crate::common::proof_ceilings::proof_ceilings_with_wall_clock(KANI_TIMEOUT),
        package: &package,
        clause: &clause,
        state_path: STATE_PATH,
        state_fields: &STATE_FIELDS,
        subject_path: SUBJECT_PATH,
        unwind: 4,
    })
    .expect("the balance clause still generates");
    assert_eq!(
        generated.frame.identity.domains,
        vec![StateFieldDomain {
            field: "balance".to_owned(),
            minimum: 0,
            maximum: 1000,
        }]
    );
    assert_eq!(generated.frame.identity.unranged.len(), 1);
    assert_eq!(generated.frame.identity.unranged[0].field, "audit");
    assert_eq!(
        generated.frame.identity.unranged[0].reason,
        quire_contract_codegen::StateUnrangedReason::TypeNotRange
    );
    assert_eq!(
        StateFrameIdentity::from_record(&generated.frame.record.contents)
            .expect("the frame record decodes"),
        generated.frame.identity
    );
}

/// The playback is decoded against the harness's draw order, not the order of its ranges or its
/// grants: a harness that draws `audit` first reads the first playback value as `audit`, and the
/// pre snapshot tie sees that.
#[trace("FR-024-AC-23", "FR-024-AC-25", "TC-035")]
#[test]
fn tc_035_the_playback_is_decoded_in_the_harnesss_draw_order() {
    let twin = Twin::new();
    let harness = generated_from_twin(
        &twin,
        "BalanceNeverDrops",
        &["audit", "balance"],
        SUBJECT_PATH,
    )
    .frame
    .identity;
    // Draw order `audit`, `balance`: the playback `[0, 5]` is the pre state `balance = 5`,
    // `audit = 0` the invocation holds.
    let run = run_of(&harness, [0, 5]);
    let replay = replay_of(&twin, "deposit", &run).expect("the playback ties to the pre state");
    let ReplaySource::Witness(witness) = &replay.wire.source else {
        panic!("a frame replay is a witness replay");
    };
    assert!(
        witness.transcript().ends_with("|audit=0;balance=5>>>"),
        "{}",
        witness.transcript()
    );
    // The same values read in declaration order are the pre state `balance = 0`, `audit = 5`.
    let swapped = refused(replay_of(&twin, "deposit", &run_of(&harness, [5, 0])));
    assert!(
        matches!(
            swapped,
            FrameReplayError::PreState(PreStateFault::Differs { ref field, decoded: 5, snapshot: 0 })
                if field == "audit"
        ),
        "{swapped}"
    );
}

/// An emitted Boolean model field is drawn without an integer domain. QSL refuses an integer
/// playback against that same source model; CG retains the field and persisted reason as context.
#[trace("FR-015-AC-81", "TC-035")]
#[test]
fn tc_035_a_field_with_no_declared_range_is_decoded_and_not_checked() {
    let twin = Twin::without_audit_range();
    let harness = generated_from_twin(&twin, "BalanceNeverDrops", &STATE_FIELDS, SUBJECT_PATH)
        .frame
        .identity;
    assert_eq!(harness.state_fields, ["balance", "audit"]);
    assert_eq!(harness.domains.len(), 1);
    assert_eq!(harness.domains[0].field, "balance");
    assert_eq!(harness.unranged.len(), 1);
    assert_eq!(harness.unranged[0].field, "audit");
    assert_eq!(
        harness.unranged[0].reason,
        quire_contract_codegen::StateUnrangedReason::TypeNotRange
    );
    let audit = 5_000_000_000_i64;
    let invocation = twin.invocation("account", (5, audit), (5, audit));
    let replay = twin
        .try_frame_replay(
            "deposit",
            &invocation,
            "account",
            "audit",
            &run_of(&harness, [5, audit]),
        )
        .expect("the unranged playback builds");
    let error = replay
        .replay()
        .expect_err("QSL refuses an integer for the selected Boolean field");
    let FrameReplayError::Refused { refusal, unranged } = &error else {
        panic!("expected QSL refusal: {error}")
    };
    assert_eq!(unranged, &harness.unranged);
    let terminal = quire_contract_codegen::run_terminal_value(
        &crate::terminal_map::synthetic_evidence(
            KaniRunOutcome::Falsified {
                counterexample: "model-type mismatch".to_owned(),
            },
            0,
        ),
        Some((&error).into()),
    )
    .expect("a falsified run has a settlement");
    assert_eq!(
        terminal,
        TerminalValue::Inconclusive(InconclusiveCause::ReplayRefused(refusal.code()))
    );
}

/// The same QSL source model supplies CG's selected field range and QSL replay within the
/// integer range its current source representation can express. The out-of-range playback is
/// stopped by CG's declared-domain check before QSL replay.
#[trace("FR-015-AC-77", "FR-024-AC-32", "TC-035")]
#[test]
fn tc_035_a_source_model_range_within_jcs_safe_integer_replays_without_override() {
    let twin = Twin::with_field_ranges((0, 1000), (0, 1_000_000));
    let (package, clause) = twin.emitted_package("BalanceNeverDrops");
    let object = package
        .graph()
        .nodes
        .iter()
        .find(|node| &*node.semantic_form == "object_type")
        .expect("selected object declaration");
    assert_eq!(
        package
            .model_object_fields(&object.node_id)
            .expect("selected fields")
            .field("audit")
            .expect("present audit")
            .member_type(),
        Some(&CheckedMemberType::IntRange {
            lower: 0,
            upper: 1_000_000,
        })
    );
    let harness = generate_state_frame_obligations(&StateFrameRequest {
        ceilings: crate::common::proof_ceilings::proof_ceilings_with_wall_clock(KANI_TIMEOUT),
        package: &package,
        clause: &clause,
        state_path: STATE_PATH,
        state_fields: &STATE_FIELDS,
        subject_path: SUBJECT_PATH,
        unwind: 4,
    })
    .expect("the source model generates")
    .frame
    .identity;
    assert_eq!(
        harness
            .domains
            .iter()
            .find(|domain| domain.field == "audit")
            .map(|domain| (domain.minimum, domain.maximum)),
        Some((0, 1_000_000))
    );
    let invocation = twin.invocation("account", (5, 999_999), (5, 999_999));
    twin.try_frame_replay(
        "deposit",
        &invocation,
        "account",
        "audit",
        &run_of(&harness, [5, 999_999]),
    )
    .expect("matching source-model replay builds")
    .replay()
    .expect("QSL admits the source-model pre state");
    let outside = 1_000_001;
    let invocation = twin.invocation("account", (5, outside), (5, outside));
    assert!(matches!(
        twin.try_frame_replay(
            "deposit",
            &invocation,
            "account",
            "audit",
            &run_of(&harness, [5, outside]),
        ),
        Err(FrameReplayError::OutOfDomain { field, value })
            if field == "audit" && value == outside
    ));
}

/// A checked package admitted against a selected wide model records `TypeNotRange` and keeps
/// that context on refusal. QSL replay recompiles the original 0..1000 source model in this
/// fixture, so this test does not establish same-model replay of the wide override.
#[trace("FR-015-AC-81", "TC-025")]
#[test]
fn tc_025_wide_model_override_preserves_unranged_context_without_a_replay_claim() {
    let twin = Twin::build(&["balance"], &[CLAUSES[0]], 0);
    let (package, clause) = twin.emitted_package_with_wide_audit_range("BalanceNeverDrops");
    let object = package
        .graph()
        .nodes
        .iter()
        .find(|node| &*node.semantic_form == "object_type")
        .expect("selected object declaration");
    assert_eq!(
        package
            .model_object_fields(&object.node_id)
            .expect("selected fields")
            .field("audit")
            .expect("present audit")
            .member_type(),
        Some(&CheckedMemberType::IntRange {
            lower: 0,
            upper: i128::from(i64::MAX) + 1,
        })
    );
    let harness = generate_state_frame_obligations(&StateFrameRequest {
        ceilings: crate::common::proof_ceilings::proof_ceilings_with_wall_clock(KANI_TIMEOUT),
        package: &package,
        clause: &clause,
        state_path: STATE_PATH,
        state_fields: &STATE_FIELDS,
        subject_path: SUBJECT_PATH,
        unwind: 4,
    })
    .expect("the selected clause generates")
    .frame
    .identity;
    assert_eq!(harness.state_fields, ["balance", "audit"]);
    assert_eq!(
        harness
            .domains
            .iter()
            .map(|domain| domain.field.as_str())
            .collect::<Vec<_>>(),
        ["balance"],
        "only the ranged field has a domain"
    );
    assert_eq!(harness.unranged.len(), 1);
    assert_eq!(harness.unranged[0].field, "audit");
    assert_eq!(
        harness.unranged[0].reason,
        quire_contract_codegen::StateUnrangedReason::TypeNotRange
    );
    let unranged = -1_i64;
    let invocation = twin.invocation("account", (5, unranged), (5, unranged));
    let replay = twin
        .try_frame_replay(
            "deposit",
            &invocation,
            "account",
            "audit",
            &run_of(&harness, [5, unranged]),
        )
        .unwrap_or_else(|error| panic!("an unranged value is not refused: {error}"));
    let ReplaySource::Witness(witness) = &replay.wire.source else {
        panic!("a frame replay is a witness replay");
    };
    assert!(
        witness.transcript().ends_with("|balance=5;audit=-1>>>"),
        "{}",
        witness.transcript()
    );
    let error = replay
        .replay()
        .expect_err("QSL refuses the pre state outside the original source model range");
    assert!(
        matches!(
            &error,
            FrameReplayError::Refused { unranged, .. }
                if unranged == &harness.unranged
                    && unranged.len() == 1
                    && unranged[0].field == "audit"
                    && unranged[0].reason == quire_contract_codegen::StateUnrangedReason::TypeNotRange
        ),
        "{error}"
    );
    let FrameReplayError::Refused { refusal, .. } = &error else {
        unreachable!("the QSL refusal was checked above")
    };
    let terminal = quire_contract_codegen::run_terminal_value(
        &crate::terminal_map::synthetic_evidence(
            KaniRunOutcome::Falsified {
                counterexample: "model-type mismatch".to_owned(),
            },
            0,
        ),
        Some((&error).into()),
    )
    .expect("a falsified run has a settlement");
    assert_eq!(
        terminal,
        TerminalValue::Inconclusive(InconclusiveCause::ReplayRefused(refusal.code()))
    );
    let inside = 5_i64;
    let invocation = twin.invocation("account", (5, inside), (5, inside));
    let replay = twin
        .try_frame_replay(
            "deposit",
            &invocation,
            "account",
            "audit",
            &run_of(&harness, [5, inside]),
        )
        .expect("an in-range playback builds");
    replay.replay().expect("the in-range pre state is admitted");
}

/// The transcript `Witness::parse` is given is the one rendering function's, over the decoded
/// values, the harness path and the check text the decode names: it is not a fixed text, and a
/// different playback gives a different transcript.
#[trace("FR-024-AC-25", "FR-024-AC-2", "TC-035")]
#[test]
fn tc_035_the_frame_witness_is_rendered_from_the_decoded_playback() {
    let twin = Twin::new();
    let harness = frame_harness(&twin);
    for (balance, audit) in [(5, 0), (7, 3)] {
        let invocation = twin.invocation("account", (balance, audit), (balance, audit + 1));
        let replay = twin
            .try_frame_replay(
                "deposit",
                &invocation,
                "account",
                "audit",
                &run_of(&harness, [balance, audit]),
            )
            .expect("builds");
        let expected = format!(
            "<<<assertion|{}|{FORBIDDEN_CHECK}|balance={balance};audit={audit}>>>",
            harness.harness_path()
        );
        let ReplaySource::Witness(witness) = &replay.wire.source else {
            panic!("a frame replay is a witness replay");
        };
        assert_eq!(witness.transcript(), expected);
        assert_eq!(
            replay.packet.source,
            Some(qsl_replay::EnvelopeSource::Witness(witness.clone()))
        );
    }
}

/// The playback of another harness, a playback with the wrong number of values and one with a
/// wrong-width value each return a typed decode refusal carrying the decoder's cause, and the
/// call site is not reached: the operation the unit does not name would otherwise be a
/// `CallSite` refusal, as it is for the playback that decodes.
#[trace("FR-024-AC-25", "TC-035")]
#[test]
fn tc_035_a_playback_that_does_not_decode_is_refused_before_the_call_site() {
    let twin = Twin::new();
    let mut harness = frame_harness(&twin);
    harness.scope.operation = "withdraw".to_owned();
    let good = run_of(&harness, [5, 0]);
    assert!(
        matches!(
            refused(replay_of(&twin, "withdraw", &good)),
            FrameReplayError::CallSite(_)
        ),
        "the control reaches the call site"
    );

    let mut sibling = harness.clone();
    sibling.module_symbol = ModuleSymbol::try_from("sibling_module").unwrap();
    let other_harness = Run {
        playback: playback_text(&sibling, FORBIDDEN_CHECK, &[5, 0]),
        harness: harness.clone(),
        decode_limits: good.decode_limits,
    };
    let short = Run {
        playback: playback_text(&harness, FORBIDDEN_CHECK, &[5]),
        harness: harness.clone(),
        decode_limits: good.decode_limits,
    };
    let wrong_width = Run {
        playback: good
            .playback
            .replacen("vec![5, 0, 0, 0, 0, 0, 0, 0]", "vec![5, 0, 0, 0]", 1),
        harness: harness.clone(),
        decode_limits: good.decode_limits,
    };
    assert_ne!(wrong_width.playback, good.playback);
    for (run, code) in [
        (&other_harness, "cg_witness_harness_identity_mismatch"),
        (&short, "kani_witness_arity_mismatch"),
        (&wrong_width, "kani_witness_width_mismatch"),
    ] {
        let error = refused(replay_of(&twin, "withdraw", run));
        let FrameReplayError::Decode(cause) = &error else {
            panic!("{code}: {error}");
        };
        assert_eq!(cause.code, code, "{error}");
    }
}

/// A decoded value at either end of its field's declared range is admitted, and the values one
/// below and one above are refused naming the field and the value, before the call site (the
/// operation the unit does not name would otherwise be a `CallSite` refusal).
#[trace("FR-024-AC-26", "FR-024-AC-3", "TC-035")]
#[test]
fn tc_035_a_decoded_value_outside_its_declared_range_is_refused_at_the_endpoints() {
    let twin = Twin::new();
    let harness = frame_harness(&twin);
    assert_eq!(
        harness
            .domains
            .iter()
            .map(|domain| domain.field.as_str())
            .collect::<Vec<_>>(),
        harness.state_fields,
        "every field of the fixture has a range, in draw order"
    );
    let mut unlocatable = harness.clone();
    unlocatable.scope.operation = "withdraw".to_owned();
    for (position, domain) in harness.domains.iter().enumerate() {
        let at = |value: i64| {
            let mut values = [0, 0];
            values[position] = value;
            values
        };
        for admitted in [domain.minimum, domain.maximum] {
            let values = at(admitted);
            let invocation =
                twin.invocation("account", (values[0], values[1]), (values[0], values[1]));
            twin.try_frame_replay(
                "deposit",
                &invocation,
                "account",
                "audit",
                &run_of(&harness, values),
            )
            .unwrap_or_else(|error| panic!("{} = {admitted} is admitted: {error}", domain.field))
            .replay()
            .unwrap_or_else(|error| panic!("{} = {admitted} reaches QSL: {error}", domain.field));
        }
        for outside in [domain.minimum - 1, domain.maximum + 1] {
            let error = refused(replay_of(
                &twin,
                "withdraw",
                &run_of(&unlocatable, at(outside)),
            ));
            assert!(
                matches!(
                    &error,
                    FrameReplayError::OutOfDomain { field, value }
                        if *field == domain.field && *value == outside
                ),
                "{} = {outside}: {error}",
                domain.field
            );
        }
    }
}

/// The pre snapshot the invocation names must hold the decoded values. The invocation of the
/// playback's own pre state settles `reproduced-with-evaluated-witness`, `violation`, naming the
/// written field; the invocation of a different pre state, in either field, is refused naming the
/// field and both values.
#[trace("FR-024-AC-27", "TC-035")]
#[test]
fn tc_035_the_invocation_of_the_playbacks_own_pre_state_replays_and_another_is_refused() {
    let twin = Twin::new();
    let run = forbidden_run(&twin);
    let own = twin.invocation("account", (5, 0), (6, 1));
    let result = twin
        .frame_replay(&own, "account", "audit", &run)
        .replay()
        .expect("the replay settles");
    let ReplayResult::Witness(arm) = result.result() else {
        panic!("a witness-sourced replay settles on the witness arm");
    };
    assert_eq!(
        arm.settlement(),
        WitnessSettlement::ReproducedWithEvaluatedWitness
    );
    assert_eq!(arm.category(), Category::Violation);
    let Some(FrameChange::FieldWrite { field, .. }) = result.found().map(|found| &found.change)
    else {
        panic!("the replay found a field write: {:?}", result.found());
    };
    assert_eq!(field.as_str(), "audit");

    for (pre, field, snapshot) in [((6, 0), "balance", 6), ((5, 1), "audit", 1)] {
        let other = twin.invocation("account", pre, (6, 1));
        let error = refused(twin.try_frame_replay("deposit", &other, "account", "audit", &run));
        assert!(
            matches!(
                &error,
                FrameReplayError::PreState(PreStateFault::Differs {
                    field: named,
                    decoded,
                    snapshot: held,
                }) if named == field && *held == snapshot && *decoded == if field == "balance" { 5 } else { 0 }
            ),
            "{pre:?}: {error}"
        );
    }
}

/// An invocation or pre snapshot that is not provided, is not of the shape `state_clause`
/// writes, lacks the object, or lacks a field or holds a non-integer for it is refused naming what
/// is missing. Every provided document here hashes to its digest (`Invocation::edited`
/// re-addresses it), so these refusals are the tie's own.
#[trace("FR-024-AC-27", "TC-035")]
#[test]
fn tc_035_a_pre_state_that_cannot_be_read_from_the_invocation_is_refused_by_name() {
    let twin = Twin::new();
    let run = forbidden_run(&twin);
    let base = twin.invocation("account", (5, 0), (6, 1));
    let fault = |invocation: &Invocation| {
        let error = refused(twin.try_frame_replay("deposit", invocation, "account", "audit", &run));
        match error {
            FrameReplayError::PreState(fault) => fault,
            other => panic!("not a pre-state refusal: {other}"),
        }
    };
    assert_eq!(
        fault(&base.without(INVOCATION_DOCUMENT)),
        PreStateFault::InvocationNotProvided
    );
    // An invocation that does not name its pre snapshot or address its object is not the
    // document `state_clause` writes.
    for member in ["pre", "self"] {
        assert_eq!(
            fault(&base.edited(INVOCATION_DOCUMENT, |document| {
                document.as_object_mut().expect("object").remove(member);
            })),
            PreStateFault::InvocationUnreadable,
            "{member}"
        );
    }
    assert!(matches!(
        fault(&base.without(PRE_DOCUMENT)),
        PreStateFault::PreNotProvided { .. }
    ));
    assert_eq!(
        fault(&base.edited(PRE_DOCUMENT, |document| {
            document["populations"] = json!("none");
        })),
        PreStateFault::PreUnreadable
    );
    assert_eq!(
        fault(&base.edited(PRE_DOCUMENT, |document| {
            document["populations"][0]["objects"][0]["key"] = json!("another");
        })),
        PreStateFault::ObjectMissing {
            population: "ix://test/bank/accounts".to_owned(),
            key: "account".to_owned(),
        }
    );
    assert_eq!(
        fault(&base.edited(PRE_DOCUMENT, |document| {
            document["populations"][0]["objects"][0]["fields"]
                .as_object_mut()
                .expect("fields")
                .remove("audit");
        })),
        PreStateFault::FieldMissing {
            field: "audit".to_owned()
        }
    );
    assert_eq!(
        fault(&base.edited(PRE_DOCUMENT, |document| {
            document["populations"][0]["objects"][0]["fields"]["balance"] =
                json!({"integer": "five"});
        })),
        PreStateFault::FieldNotInteger {
            field: "balance".to_owned()
        }
    );
}

/// A provided document whose bytes do not match the digest it is addressed by is QSL's refusal,
/// with its own code, and not a pre-state refusal: the tie reads the documents only after QSL's
/// request decode has checked them. The same replay over documents that hash to their digests
/// builds.
#[trace("FR-024-AC-27", "TC-035")]
#[test]
fn tc_035_a_document_that_does_not_match_its_digest_is_qsls_refusal_not_a_pre_state_one() {
    let twin = Twin::new();
    let run = forbidden_run(&twin);
    let base = twin.invocation("account", (5, 0), (6, 1));
    for index in [INVOCATION_DOCUMENT, PRE_DOCUMENT] {
        let mismatched = base.replaced(index, br#"{"unrelated":true}"#);
        let error =
            refused(twin.try_frame_replay("deposit", &mismatched, "account", "audit", &run));
        assert!(
            matches!(
                &error,
                FrameReplayError::Refused { refusal, .. }
                    if matches!(**refusal, ReplayRefusal::Request(_))
            ),
            "{index}: {error}"
        );
    }
    twin.try_frame_replay("deposit", &base, "account", "audit", &run)
        .map(|_| ())
        .unwrap_or_else(|error| panic!("the matching documents build: {error}"));
}

/// A state field listed twice is refused: the decoder refuses the schema, and a record that lists
/// one is not read as a frame identity.
#[trace("FR-024-AC-23", "TC-035")]
#[test]
fn tc_035_a_repeated_state_field_is_refused_by_the_decoder_and_the_record() {
    let twin = Twin::new();
    let mut harness = frame_harness(&twin);
    harness.state_fields = vec!["balance".to_owned(), "balance".to_owned()];
    harness.property = StateFrameProperty::Frame {
        granted: vec!["balance".to_owned()],
        checked: vec!["balance".to_owned()],
    };
    let error = refused(replay_of(&twin, "deposit", &run_of(&harness, [5, 0])));
    assert!(
        matches!(
            &error,
            FrameReplayError::Decode(cause)
                if cause.code == "cg_witness_schema_duplicate_binding" && cause.context == "balance"
        ),
        "{error}"
    );

    let generated =
        generated_from_twin(&twin, "BalanceNeverDrops", &STATE_FIELDS, SUBJECT_PATH).frame;
    let mut record: Value = serde_json::from_str(&generated.record.contents).expect("record");
    record["identity"]["state_fields"] = json!(["balance", "balance"]);
    let error = StateFrameIdentity::from_record(&record.to_string())
        .expect_err("a repeated field is not a draw order");
    assert!(
        matches!(&error, StateFrameRecordError::RepeatedField { field } if field == "balance"),
        "{error}"
    );
}

/// A harness scoped to another operation than the one requested is a `ScopeMismatch` naming
/// `operation`, before the call site (the requested operation the unit does not name would
/// otherwise be a `CallSite` refusal). A harness whose anchor or frame is not the one the call
/// site names is a `ScopeMismatch` naming that member after it.
#[trace("FR-024-AC-28", "TC-035")]
#[test]
fn tc_035_a_harness_scope_that_is_not_the_requested_operation_is_refused_by_member() {
    let twin = Twin::new();
    let fixture = emitted_fixture(&Twin::build(&[], &CLAUSES, 0), "BalanceNeverDrops");
    let site = twin.operation_site("deposit").expect("located");
    let aligned = frame_harness(&twin);

    let error = refused(replay_of(&twin, "withdraw", &run_of(&aligned, [5, 0])));
    assert!(
        matches!(
            &error,
            FrameReplayError::ScopeMismatch { member: ScopeMember::Operation, harness, named }
                if harness == "deposit" && named == "withdraw"
        ),
        "{error}"
    );

    // A second QSL-selected model with a different frame has different node ids.
    let unaligned = generate(&fixture).frame.identity;
    let error = refused(replay_of(&twin, "deposit", &run_of(&unaligned, [5, 0])));
    assert!(
        matches!(
            &error,
            FrameReplayError::ScopeMismatch { member: ScopeMember::Anchor, harness, named }
                if *harness == format!("{:?}", fixture.anchor) && *named == site.anchor.to_string()
        ),
        "{error}"
    );

    let mut other_frame = aligned.clone();
    other_frame.scope.frame = fixture.frame.clone();
    let error = refused(replay_of(&twin, "deposit", &run_of(&other_frame, [5, 0])));
    assert!(
        matches!(
            &error,
            FrameReplayError::ScopeMismatch { member: ScopeMember::Frame, harness, named }
                if *harness == format!("{:?}", fixture.frame) && *named == site.frame.to_string()
        ),
        "{error}"
    );
    replay_of(&twin, "deposit", &run_of(&aligned, [5, 0])).expect("the aligned scope builds");
}

/// The anchor and frame nodes of the package QSL itself emits from the twin's unit
/// (`call_site`'s package bytes, admitted by the model reader) have, as `CheckedNodeId`s, exactly
/// the ids `call_site` names as the site's `anchor` and `frame`, and the anchor binds that frame:
/// the node ids the generator reads from a package and the wire ids the replay requires are one
/// id, with no rebase.
#[trace("FR-024-AC-28", "FR-024-AC-30", "TC-035")]
#[test]
fn tc_035_the_node_ids_of_the_package_qsl_emits_are_the_ids_call_site_names() {
    let twin = Twin::new();
    let (package, _) = twin.emitted_package("BalanceNeverDrops");
    let site = twin.operation_site("deposit").expect("located");
    let node_of = |form: &str| {
        let found = package
            .graph()
            .nodes
            .iter()
            .filter(|node| &*node.node_tag == "state" && &*node.semantic_form == form)
            .collect::<Vec<_>>();
        let [only] = found[..] else {
            panic!("one `{form}` node in the package, found {}", found.len());
        };
        only
    };
    let anchor = node_of("operation_anchor");
    let frame = node_of("frame");
    assert!(native_twin::node_matches_site(
        &anchor.node_id,
        site.anchor.view()
    ));
    assert!(native_twin::node_matches_site(
        &frame.node_id,
        site.frame.view()
    ));
    let bound_frame = match &anchor.body {
        CheckedTerm::OperationAnchorBody(body) => {
            body.members.iter().find_map(|member| match member {
                CheckedTerm::Binding { name, value } if name.as_ref() == "frame" => {
                    match value.as_ref() {
                        CheckedTerm::Reference { target } => Some(target),
                        _ => None,
                    }
                }
                _ => None,
            })
        }
        _ => None,
    };
    assert_eq!(bound_frame, Some(&frame.node_id));
    let fixture_scope =
        generated_from_twin(&twin, "BalanceNeverDrops", &STATE_FIELDS, SUBJECT_PATH)
            .frame
            .identity
            .scope;
    assert_eq!(fixture_scope.anchor, anchor.node_id);
    assert_eq!(fixture_scope.frame, frame.node_id);
}

/// QSL's emitted object body is empty; the admitted model's accessor supplies both ranges,
/// including the unread `audit` field.
#[trace("FR-015-AC-77", "FR-015-AC-79")]
#[test]
fn tc_025_the_generator_uses_model_fields_of_the_package_qsl_emits() {
    let twin = Twin::new();
    let (package, clause) = twin.emitted_package("BalanceNeverDrops");
    let object = package
        .graph()
        .nodes
        .iter()
        .find(|node| &*node.semantic_form == "object_type")
        .expect("the package holds the object type");
    assert_eq!(
        match &object.body {
            CheckedTerm::Aggregate { members } => Some(members.len()),
            _ => None,
        },
        Some(0),
        "QSL emits the object type with no members in its body"
    );
    let generated = generate_state_frame_obligations(&StateFrameRequest {
        ceilings: crate::common::proof_ceilings::proof_ceilings_with_wall_clock(KANI_TIMEOUT),
        package: &package,
        clause: &clause,
        state_path: STATE_PATH,
        state_fields: &STATE_FIELDS,
        subject_path: SUBJECT_PATH,
        unwind: 4,
    })
    .expect("the accessor supplies both ranges");
    let expected = model::FIELDS
        .map(|(field, (minimum, maximum))| StateFieldDomain {
            field: field.to_owned(),
            minimum,
            maximum,
        })
        .to_vec();
    assert_eq!(generated.frame.identity.domains, expected);
    assert_eq!(generated.postcondition.identity.domains, expected);
    assert_eq!(generated.frame.identity.state_fields, STATE_FIELDS);
    assert_eq!(generated.frame.identity.scope.object, object.node_id);
    assert!(generated.frame.identity.unranged.is_empty());
    let site = twin.operation_site("deposit").expect("located");
    assert_eq!(
        generated.frame.identity.scope.anchor,
        package
            .graph()
            .nodes
            .iter()
            .find(|node| native_twin::node_matches_site(&node.node_id, site.anchor.view()))
            .expect("anchor node")
            .node_id
    );
    assert_eq!(
        generated.frame.identity.scope.frame,
        package
            .graph()
            .nodes
            .iter()
            .find(|node| native_twin::node_matches_site(&node.node_id, site.frame.view()))
            .expect("frame node")
            .node_id
    );

    let balance_only = Twin::build(&model::GRANTED, &[CLAUSES[0]], 0);
    let unread = generated_from_twin(
        &balance_only,
        "BalanceNeverDrops",
        &STATE_FIELDS,
        SUBJECT_PATH,
    );
    assert_eq!(unread.frame.identity.domains, expected);
    assert_eq!(unread.postcondition.identity.domains, expected);

    let audit = generated_from_twin(&twin, "AuditNeverDrops", &STATE_FIELDS, SUBJECT_PATH);
    assert_eq!(audit.frame.identity.domains, expected);
    assert_eq!(audit.postcondition.identity.domains, expected);
    assert_eq!(audit.frame.identity.state_fields, STATE_FIELDS);
}

/// The accessor's sorted table validates names but never changes the caller's draw order.
#[trace("FR-015-AC-77", "FR-015-AC-79")]
#[test]
fn tc_025_model_field_table_preserves_the_requested_draw_order() {
    let twin = Twin::new();
    let (package, clause) = twin.emitted_package("BalanceNeverDrops");
    let fields = ["audit", "balance"];
    let generated = generate_state_frame_obligations(&StateFrameRequest {
        ceilings: crate::common::proof_ceilings::proof_ceilings_with_wall_clock(KANI_TIMEOUT),
        package: &package,
        clause: &clause,
        state_path: STATE_PATH,
        state_fields: &fields,
        subject_path: SUBJECT_PATH,
        unwind: 4,
    })
    .expect("both named fields resolve");
    assert_eq!(generated.frame.identity.state_fields, fields);
    assert_eq!(
        generated
            .frame
            .identity
            .domains
            .iter()
            .map(|domain| domain.field.as_str())
            .collect::<Vec<_>>(),
        fields
    );

    let absent = generate_state_frame_obligations(&StateFrameRequest {
        ceilings: crate::common::proof_ceilings::proof_ceilings_with_wall_clock(KANI_TIMEOUT),
        state_fields: &["balance", "ghost", "audit"],
        ..StateFrameRequest {
            ceilings: crate::common::proof_ceilings::proof_ceilings_with_wall_clock(KANI_TIMEOUT),
            package: &package,
            clause: &clause,
            state_path: STATE_PATH,
            state_fields: &fields,
            subject_path: SUBJECT_PATH,
            unwind: 4,
        }
    })
    .expect_err("a requested name absent from the accessor refuses");
    assert!(matches!(
        absent,
        StateFrameRefusal::BoundNotResolved {
            field,
            cause: BoundNotResolvedCause::MemberAbsent { .. },
        } if field == "ghost"
    ));
}

/// The frame envelope declares no domain: `declared_domains` is present and empty.
#[trace("FR-024-AC-29", "TC-035")]
#[test]
fn tc_035_the_frame_envelope_declares_no_domain() {
    let twin = Twin::new();
    let replay = replay_of(&twin, "deposit", &forbidden_run(&twin)).expect("builds");
    assert_eq!(replay.packet.declared_domains, Some(Vec::new()));
}

// ---- kani lane ---------------------------------------------------------------

const KANI_TIMEOUT: Duration = Duration::from_secs(600);

fn scratch(name: &str) -> PathBuf {
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("state-frame-{name}-{}-{nonce}", std::process::id()));
    fs::create_dir_all(path.join("src")).expect("scratch");
    path
}

/// Runs `harness` with the real prover over the subject module every generated harness names.
pub(crate) fn prove(harness: &StateFrameHarness) -> KaniRunOutcome {
    prove_evidence(harness).outcome
}

/// Retains real run evidence for terminal projection after replay.
pub(crate) fn prove_evidence(
    harness: &StateFrameHarness,
) -> quire_contract_codegen::KaniExecutionEvidence {
    let cancel = quire_exact::Cancel::new();
    let installation = KaniInstallation::discover().expect("cargo-kani is installed");
    let directory = scratch("crate");
    fs::write(
        directory.join("src/lib.rs"),
        format!(
            "//! Generated obligation check crate.\n\n{}\npub mod subject {{\n{SUBJECT_SOURCE}}}\n",
            harness.rust.contents
        ),
    )
    .expect("lib");
    fs::write(
        directory.join("Cargo.toml"),
        "[package]\nname = \"generated-state-frame\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[workspace]\n",
    )
    .expect("manifest");
    fs::write(
        directory.join("build.rs"),
        "fn main() { println!(\"cargo:rustc-check-cfg=cfg(kani)\"); }\n",
    )
    .expect("build script");
    let evidence = execute_kani_obligation(&KaniExecutionRequest {
        cancel: &cancel,
        installation: &installation,
        harness: harness.into(),
        crate_directory: &directory,
        target_directory: &PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("state-frame-kani"),
        home_directory: None,
    })
    .unwrap_or_else(|refusal| panic!("the run must start: {refusal}"));
    let _ = fs::remove_dir_all(directory);
    evidence
}

/// The counterexample of a falsified proof, which must name `reason`.
pub(crate) fn falsified(outcome: KaniRunOutcome, reason: &str) -> String {
    let KaniRunOutcome::Falsified { counterexample } = outcome else {
        panic!("expected a falsified proof for `{reason}`, got {outcome:?}");
    };
    assert!(
        counterexample.contains(reason),
        "the counterexample must name `{reason}`:\n{counterexample}"
    );
    counterexample
}

/// The `(balance, audit)` values Kani's concrete playback assigns the harness's two symbolic
/// `i64` fields, in the order the harness declares them.
pub(crate) fn playback_state(counterexample: &str) -> (i64, i64) {
    let values = counterexample
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with("vec![") && !line.contains("concrete_vals"))
        .map(|line| {
            let bytes = line
                .trim_start_matches("vec![")
                .trim_end_matches("],")
                .split(',')
                .map(|byte| byte.trim().parse::<u8>().expect("a playback byte"))
                .collect::<Vec<_>>();
            i64::from_le_bytes(bytes.try_into().expect("eight bytes per i64"))
        })
        .collect::<Vec<_>>();
    let [balance, audit] = values[..] else {
        panic!("two symbolic fields in the playback, found {values:?}");
    };
    (balance, audit)
}

/// The operation contract verifies for a healthy subject and is falsified, for the postcondition
/// and no other reason, when the subject is mutated to debit.
#[trace("FR-015-AC-30", "FR-015-AC-80", "TC-025")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_025_real_kani_proves_the_state_postcondition_and_a_mutated_subject_falsifies_it() {
    let twin = Twin::new();
    assert_eq!(
        prove(
            &generated_from_twin(&twin, "BalanceNeverDrops", &STATE_FIELDS, SUBJECT_PATH)
                .postcondition
        ),
        KaniRunOutcome::Verified
    );
    falsified(
        prove(
            &generated_from_twin(
                &twin,
                "BalanceNeverDrops",
                &STATE_FIELDS,
                "crate::subject::deposit_debiting",
            )
            .postcondition,
        ),
        "postcondition `post.balance >= pre.balance` failed",
    );
}

/// A violation whose valuation the cover must share is still a counterexample. Both state fields
/// are bounded to the single value 0, so the one valuation the harness can draw is the failing
/// one and the non-vacuity cover takes it too, whatever model the solver picks; a harness whose
/// cover preceded its assertion would print only the cover's playback here and classify as a
/// failure with no counterexample (IR-451).
#[trace("FR-015-AC-7", "TC-025")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_025_real_kani_a_violation_at_the_only_valuation_is_a_counterexample_not_the_covers_playback()
{
    let single = Twin::with_field_ranges((0, 0), (0, 0));
    let counterexample = falsified(
        prove(
            &generated_from_twin(
                &single,
                "BalanceNeverDrops",
                &STATE_FIELDS,
                "crate::subject::deposit_debiting",
            )
            .postcondition,
        ),
        "postcondition `post.balance >= pre.balance` failed",
    );
    assert_eq!(playback_state(&counterexample), (0, 0));
    let counterexample = falsified(
        prove(
            &generated_from_twin(
                &single,
                "BalanceNeverDrops",
                &STATE_FIELDS,
                "crate::subject::deposit_touching_audit",
            )
            .frame,
        ),
        "changed `audit`, which its frame does not modify",
    );
    assert_eq!(playback_state(&counterexample), (0, 0));
}

/// An effect the frame allows verifies, an effect it forbids is falsified naming the forbidden
/// field, and regenerating the frame from a package whose `modifies` is mutated to grant nothing
/// turns the allowed proof red.
#[trace("FR-015-AC-31", "FR-015-AC-80", "TC-025")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_025_real_kani_proves_allowed_and_forbidden_frame_effects_and_a_mutated_frame_falsifies() {
    let twin = Twin::new();
    // ALLOWED: only `balance` changes, and the frame grants it.
    assert_eq!(
        prove(&generated_from_twin(&twin, "BalanceNeverDrops", &STATE_FIELDS, SUBJECT_PATH).frame),
        KaniRunOutcome::Verified
    );
    // FORBIDDEN: `audit` changes too.
    falsified(
        prove(
            &generated_from_twin(
                &twin,
                "BalanceNeverDrops",
                &STATE_FIELDS,
                "crate::subject::deposit_touching_audit",
            )
            .frame,
        ),
        "changed `audit`, which its frame does not modify",
    );
    // MUTATION CONTROL: the same allowed subject against a frame that grants nothing.
    let emptied = Twin::build(&[], &CLAUSES, 0);
    falsified(
        prove(
            &generated_from_twin(&emptied, "BalanceNeverDrops", &STATE_FIELDS, SUBJECT_PATH).frame,
        ),
        "changed `balance`, which its frame does not modify",
    );
}

/// The forbidden frame counterexample Kani finds is executed natively, then replayed through
/// `FrameReplay::new` and `replay` from its harness's identity and the bounded parse of its
/// complete retained playback text:
/// no obligation identity and no transcript are supplied, the decoded pre state is tied to the
/// invocation of the native run, and QSL reproduces the violation on the written field.
#[trace(
    "FR-015-AC-32",
    "FR-024-AC-30",
    "FR-024-AC-32",
    "FR-024-AC-34",
    "TC-025",
    "TC-035"
)]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_035_real_kani_frame_counterexample_replays_through_qsl() {
    let twin = Twin::new();
    let forbidden = generated_from_twin(
        &twin,
        "BalanceNeverDrops",
        &STATE_FIELDS,
        "crate::subject::deposit_touching_audit",
    );
    let counterexample = falsified(
        prove(&forbidden.frame),
        "changed `audit`, which its frame does not modify",
    );
    let (balance, audit) = playback_state(&counterexample);
    let mut account = subject::Account { balance, audit };
    subject::deposit_touching_audit(&mut account);
    assert_ne!(
        account.audit, audit,
        "the native run reproduces the forbidden write"
    );
    let invocation = twin.invocation(
        "account",
        (balance, audit),
        (account.balance, account.audit),
    );
    let run = Run {
        harness: forbidden.frame.identity,
        playback: counterexample,
        decode_limits: native_twin::frame_decode_limits(),
    };
    let result = twin
        .frame_replay(&invocation, "account", "audit", &run)
        .replay()
        .expect("the replay settles");
    let ReplayResult::Witness(arm) = result.result() else {
        panic!("a witness-sourced replay settles on the witness arm");
    };
    assert_eq!(
        arm.settlement(),
        WitnessSettlement::ReproducedWithEvaluatedWitness
    );
    assert_eq!(arm.category(), Category::Violation);
    let Some(FrameChange::FieldWrite { object, field, .. }) =
        result.found().map(|found| &found.change)
    else {
        panic!("the replay found a field write: {:?}", result.found());
    };
    assert_eq!((object.as_str(), field.as_str()), ("account", "audit"));
}

/// The allowed effect proves, and its run replays as a frame the invocation respects. A verified
/// harness yields no playback, so the run carries a hand-built one at the pre state `(5, 0)`; the
/// forbidden counterexample is replayed from its real playback by
/// `tc_035_real_kani_frame_counterexample_replays_through_qsl`.
#[trace("FR-015-AC-32", "TC-025")]
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_025_real_kani_the_allowed_frame_effect_replays_as_a_respected_frame() {
    let twin = Twin::new();
    let allowed = generated_from_twin(&twin, "BalanceNeverDrops", &STATE_FIELDS, SUBJECT_PATH);
    assert_eq!(prove(&allowed.frame), KaniRunOutcome::Verified);
    let mut account = subject::Account {
        balance: 5,
        audit: 0,
    };
    subject::deposit(&mut account);
    let invocation = twin.invocation("account", (5, 0), (account.balance, account.audit));
    let harness = allowed.frame.identity;
    let run = run_of(&harness, [5, 0]);
    let result = twin
        .replay(&invocation, "account", "balance", &run)
        .expect("the replay settles");
    let ReplayResult::Witness(arm) = result.result() else {
        panic!("a witness-sourced replay settles on the witness arm");
    };
    assert_eq!(arm.settlement(), WitnessSettlement::Inconclusive);
    assert_eq!(
        arm.disagreement(),
        Some(&DisagreementCause::Verdicts {
            proved: Verdict::from_category(Category::Violation),
            replayed: Verdict::from_category(Category::Success),
        })
    );
    assert!(result.found().is_none());
}
