use std::{
    fs,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use quire_contract_codegen::{
    generate_tristate_harness, DerivationManifest, GenerationTerminalState, HarnessErrorCode,
    HarnessRequest, ManifestContext, IR_CANDIDATE_REVISION,
};
use quire_contract_ir::{
    AnchorName, BooleanOperator, ClauseId, DeclarationEnvironment, ExecutionPoint, Expression,
    ExpressionKind, PackageId, RequirementId, RequirementRef, RequirementRevision,
    SourceDocumentId, SourceIdentity, SourceLocation, SourceRevision, SourceSpan, StateObservation,
    SymbolName, TypedExpression, ValueDeclaration, ValueDeclarationKind, ValueType,
};

struct TemporaryDirectory(PathBuf);

impl TemporaryDirectory {
    fn new(prefix: &str) -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("{prefix}-{}-{nonce}", std::process::id()));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(path.join("src")).unwrap();
        Self(path)
    }
}

impl Drop for TemporaryDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn requirement() -> RequirementRef {
    RequirementRef::new(
        PackageId::new("agent-ix/harness-test").unwrap(),
        RequirementId::new("FR-002").unwrap(),
        RequirementRevision::new(1).unwrap(),
    )
}

fn span(start: u64, end: u64) -> SourceSpan {
    let source = SourceIdentity::new(
        SourceDocumentId::new("harness-test").unwrap(),
        SourceRevision::new(1).unwrap(),
    );
    SourceSpan::new(
        SourceLocation::new(source.clone(), 1, start as u32 + 1, start).unwrap(),
        SourceLocation::new(source, 1, end as u32 + 1, end).unwrap(),
    )
    .unwrap()
}

fn manifest_context() -> ManifestContext<'static> {
    ManifestContext {
        candidate_revision: IR_CANDIDATE_REVISION,
        contribution_method: "generated",
        reviewers: &["@harness-test-reviewer"],
        result_status: "conclusive",
        result_summary: "test harness generated from typed Boolean clauses",
        requirement_refs: &["FR-002"],
    }
}

fn harness_function_name(source: &str) -> &str {
    source
        .lines()
        .find(|line| line.starts_with("pub fn harness_") && !line.contains("_shell"))
        .and_then(|line| line.strip_prefix("pub fn "))
        .and_then(|signature| signature.split('<').next())
        .unwrap()
}

fn typed_clauses() -> (DeclarationEnvironment, TypedExpression, TypedExpression) {
    let environment = DeclarationEnvironment::new(
        requirement(),
        vec![],
        vec![
            ValueDeclaration::new(
                SymbolName::new("enabled").unwrap(),
                ValueDeclarationKind::Input,
                ValueType::Boolean,
                span(0, 1),
            ),
            ValueDeclaration::new(
                SymbolName::new("state").unwrap(),
                ValueDeclarationKind::State,
                ValueType::Boolean,
                span(1, 2),
            ),
        ],
        vec![],
    )
    .unwrap();
    let reference = |name: &str, observation: StateObservation, at: u64| {
        Expression::new(
            ExpressionKind::ValueReference {
                name: SymbolName::new(name).unwrap(),
                observation,
            },
            span(at, at + 1),
        )
    };
    let precondition = Expression::new(
        ExpressionKind::Boolean {
            operator: BooleanOperator::ShortCircuitAnd,
            left: Box::new(reference("enabled", StateObservation::Current, 2)),
            right: Box::new(reference("state", StateObservation::Pre, 3)),
        },
        span(2, 4),
    );
    let postcondition = reference("state", StateObservation::Post, 4);
    let precondition = environment
        .check_expression(
            &precondition,
            &ValueType::Boolean,
            &ExecutionPoint::Pre {
                operation: AnchorName::new("update").unwrap(),
            },
            true,
        )
        .unwrap();
    let postcondition = environment
        .check_expression(
            &postcondition,
            &ValueType::Boolean,
            &ExecutionPoint::Handler {
                name: AnchorName::new("update").unwrap(),
            },
            true,
        )
        .unwrap();
    (environment, precondition, postcondition)
}

/// TC-004.
#[test]
fn tc_004_generated_harness_binds_clauses_and_executes_all_three_terminal_paths() {
    let (environment, precondition_expression, postcondition_expression) = typed_clauses();
    let precondition = ClauseId::new("precondition-main").unwrap();
    let postcondition = ClauseId::new("postcondition-main").unwrap();
    let request = HarnessRequest {
        requirement: environment.owner(),
        precondition_clause: &precondition,
        postcondition_clause: &postcondition,
        precondition: &precondition_expression,
        postcondition: &postcondition_expression,
        execution_point: "handler:update",
        manifest: manifest_context(),
    };
    let first = generate_tristate_harness(&request).unwrap();
    let second = generate_tristate_harness(&request).unwrap();
    assert_eq!(first, second);
    let derivation: DerivationManifest = serde_json::from_str(&first.manifest.contents).unwrap();
    assert_eq!(derivation.outputs[0].uri, first.rust.path);
    assert_eq!(derivation.outputs[0].role, "generated-rust-harness");
    assert!(first.rust.contents.starts_with("#![deny(missing_docs)]\n"));
    assert!(first
        .rust
        .contents
        .contains("let pre_state = snapshot(state);"));
    assert!(first
        .rust
        .contents
        .contains("oracle_fr_002_1_precondition_main"));
    assert!(first
        .rust
        .contents
        .contains("oracle_fr_002_1_postcondition_main"));
    assert!(
        first.rust.contents.find("if !precondition").unwrap()
            < first.rust.contents.find("invoke(input, state);").unwrap()
    );
    assert!(
        first.rust.contents.find("invoke(input, state);").unwrap()
            < first.rust.contents.find("if postcondition").unwrap()
    );

    let temporary = TemporaryDirectory::new("quire-generated-harness");
    let manifest = "[package]\nname = \"generated-harness-check\"\nversion = \"0.0.0\"\nedition = \"2021\"\n\n[dependencies]\nproptest = { version = \"=1.5.0\", default-features = false, features = [\"std\"] }\nquire-contract-runtime = { git = \"https://github.com/agent-ix/quire-contract-runtime\", rev = \"e360dad8a3e0e54f9b8457ff7f3748be0f2acdb3\", features = [\"proptest\"] }\n";
    fs::write(temporary.0.join("Cargo.toml"), manifest).unwrap();
    let tests = r#"

#[cfg(test)]
mod generated_tests {
    use super::*;
    use quire_contract_runtime::{
        CampaignReport, ClauseId, ClauseKind, ClauseOutcome, ContractIdentity, ExecutionPoint,
        Observation, RequirementId, RevisionId, VerdictKind,
    };

    fn blank() -> Observation<'static> {
        Observation::new(
            ClauseId::new("blank"),
            ClauseKind::Guard,
            ClauseOutcome::NotEvaluated,
            None,
        )
    }

    #[test]
    fn rejection_does_not_invoke_and_is_recorded() {
        let mut state = true;
        let mut observations = [blank(); 2];
        let verdict = HARNESS_FN(
            false,
            &mut state,
            |_, _| panic!("rejected subject must not run"),
            &mut observations,
        );
        assert_eq!(verdict.kind(), VerdictKind::RejectedPrecondition);
        assert!(state);
        assert_eq!(verdict.context().observations[0].outcome, ClauseOutcome::Rejected);
        assert_eq!(verdict.context().observations[1].outcome, ClauseOutcome::NotEvaluated);

        let mut report = CampaignReport::new(ContractIdentity::new(
            RequirementId::new("FR-002"),
            RevisionId::new("1"),
        ));
        let mut observations = [blank(); 2];
        let result = HARNESS_FN_proptest(
            &mut report,
            true,
            false,
            &mut state,
            |_, _| panic!("rejected subject must not run"),
            &mut observations,
        );
        assert!(result.is_err());
        assert_eq!(report.counts().accepted(), 0);
        assert_eq!(report.counts().rejected(), 1);
        assert_eq!(report.counts().failed(), 0);
        assert_eq!(report.counts().discarded(), 0);
        assert!(HARNESS_FN_conclude_campaign(&report).is_err());
        HARNESS_FN_record_discard(&mut report);
        assert_eq!(report.counts().discarded(), 1);
        assert!(HARNESS_FN_conclude_campaign(&report).is_err());
    }

    #[test]
    fn pass_and_failure_observe_generated_pre_and_post_clauses() {
        for (next, expected) in [
            (true, VerdictKind::Passed),
            (false, VerdictKind::FailedPostcondition),
        ] {
            let mut state = true;
            let mut observations = [blank(); 2];
            let verdict = HARNESS_FN(
                true,
                &mut state,
                move |_, post| *post = next,
                &mut observations,
            );
            assert_eq!(verdict.kind(), expected);
            assert_eq!(state, next);
            assert_eq!(verdict.context().execution_point, ExecutionPoint::new("handler:update"));
        }
    }

    #[test]
    fn accepted_campaign_concludes_with_retained_counts() {
        let mut state = true;
        let mut observations = [blank(); 2];
        let mut mismatch_report = CampaignReport::new(ContractIdentity::new(
            RequirementId::new("FR-002"),
            RevisionId::new("1"),
        ));
        assert!(HARNESS_FN_proptest(
            &mut mismatch_report,
            true,
            true,
            &mut state,
            |_, _| {},
            &mut observations,
        )
        .is_err());
        assert_eq!(mismatch_report.counts().accepted(), 1);

        let mut observations = [blank(); 2];
        let mut report = CampaignReport::new(ContractIdentity::new(
            RequirementId::new("FR-002"),
            RevisionId::new("1"),
        ));
        HARNESS_FN_proptest(
            &mut report,
            false,
            true,
            &mut state,
            |_, _| {},
            &mut observations,
        )
        .unwrap();
        HARNESS_FN_record_discard(&mut report);
        let summary = HARNESS_FN_conclude_campaign(&report).unwrap();
        assert_eq!(summary.accepted, 1);
        assert_eq!(summary.rejected, 0);
        assert_eq!(summary.failed, 0);
        assert_eq!(summary.discarded, 1);
    }
}
"#
    .replace("HARNESS_FN", harness_function_name(&first.rust.contents));
    fs::write(
        temporary.0.join("src/lib.rs"),
        format!("{}{}", first.rust.contents, tests),
    )
    .unwrap();
    let output = Command::new(env!("CARGO"))
        .args(["test", "--offline", "--quiet"])
        .env("CARGO_TARGET_DIR", temporary.0.join("target"))
        .env("RUSTFLAGS", "-Dwarnings")
        .current_dir(&temporary.0)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "generated harness did not compile and execute:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

/// TC-004.
#[test]
fn tc_004_state_only_and_dependency_free_harnesses_compile_with_denied_warnings() {
    let literal =
        |value, at| Expression::new(ExpressionKind::BooleanLiteral { value }, span(at, at + 1));
    let execution_pre = ExecutionPoint::Pre {
        operation: AnchorName::new("update").unwrap(),
    };
    let execution_post = ExecutionPoint::Handler {
        name: AnchorName::new("update").unwrap(),
    };

    let dependency_free_environment =
        DeclarationEnvironment::new(requirement(), vec![], vec![], vec![]).unwrap();
    let dependency_free_pre = dependency_free_environment
        .check_expression(
            &literal(true, 10),
            &ValueType::Boolean,
            &execution_pre,
            true,
        )
        .unwrap();
    let dependency_free_post = dependency_free_environment
        .check_expression(
            &literal(true, 11),
            &ValueType::Boolean,
            &execution_post,
            true,
        )
        .unwrap();

    let state_environment = DeclarationEnvironment::new(
        requirement(),
        vec![],
        vec![ValueDeclaration::new(
            SymbolName::new("state").unwrap(),
            ValueDeclarationKind::State,
            ValueType::Boolean,
            span(12, 13),
        )],
        vec![],
    )
    .unwrap();
    let state_pre = state_environment
        .check_expression(
            &literal(true, 13),
            &ValueType::Boolean,
            &execution_pre,
            true,
        )
        .unwrap();
    let state_post_expression = Expression::new(
        ExpressionKind::ValueReference {
            name: SymbolName::new("state").unwrap(),
            observation: StateObservation::Post,
        },
        span(14, 15),
    );
    let state_post = state_environment
        .check_expression(
            &state_post_expression,
            &ValueType::Boolean,
            &execution_post,
            true,
        )
        .unwrap();

    let precondition = ClauseId::new("precondition-shape").unwrap();
    let postcondition = ClauseId::new("postcondition-shape").unwrap();
    let dependency_free = generate_tristate_harness(&HarnessRequest {
        requirement: dependency_free_environment.owner(),
        precondition_clause: &precondition,
        postcondition_clause: &postcondition,
        precondition: &dependency_free_pre,
        postcondition: &dependency_free_post,
        execution_point: "handler:dependency-free",
        manifest: manifest_context(),
    })
    .unwrap();
    let state_only = generate_tristate_harness(&HarnessRequest {
        requirement: state_environment.owner(),
        precondition_clause: &precondition,
        postcondition_clause: &postcondition,
        precondition: &state_pre,
        postcondition: &state_post,
        execution_point: "handler:state-only",
        manifest: manifest_context(),
    })
    .unwrap();

    for (name, artifact, invocation) in [
        (
            "dependency-free",
            dependency_free,
            "let verdict = HARNESS_FN(|| {}, &mut observations);",
        ),
        (
            "state-only",
            state_only,
            "let mut state = false; let verdict = HARNESS_FN(&mut state, |state| *state = true, &mut observations);",
        ),
    ] {
        let function = harness_function_name(&artifact.rust.contents);
        let temporary = TemporaryDirectory::new(&format!("quire-generated-harness-{name}"));
        fs::write(
            temporary.0.join("Cargo.toml"),
            "[package]\nname = \"generated-harness-shape\"\nversion = \"0.0.0\"\nedition = \"2021\"\n\n[dependencies]\nproptest = { version = \"=1.5.0\", default-features = false, features = [\"std\"] }\nquire-contract-runtime = { git = \"https://github.com/agent-ix/quire-contract-runtime\", rev = \"e360dad8a3e0e54f9b8457ff7f3748be0f2acdb3\", features = [\"proptest\"] }\n",
        )
        .unwrap();
        let generated_test = format!(
            "\n#[cfg(test)] mod generated_tests {{ use super::*; use quire_contract_runtime::{{ClauseId, ClauseKind, ClauseOutcome, Observation, VerdictKind}}; #[test] fn shape_executes() {{ let mut observations = [Observation::new(ClauseId::new(\"blank\"), ClauseKind::Guard, ClauseOutcome::NotEvaluated, None); 2]; {} assert_eq!(verdict.kind(), VerdictKind::Passed); }} }}\n",
            invocation.replace("HARNESS_FN", function),
        );
        fs::write(
            temporary.0.join("src/lib.rs"),
            format!("{}{}", artifact.rust.contents, generated_test),
        )
        .unwrap();
        let output = Command::new(env!("CARGO"))
            .args(["test", "--offline", "--quiet"])
            .env("CARGO_TARGET_DIR", temporary.0.join("target"))
            .env("RUSTFLAGS", "-Dwarnings")
            .current_dir(&temporary.0)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "generated {name} harness did not compile and execute:\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

/// TC-003, TC-004.
#[test]
fn tc_004_invalid_execution_point_is_a_structured_failure_without_artifact() {
    let (environment, precondition_expression, postcondition_expression) = typed_clauses();
    let precondition = ClauseId::new("precondition-main").unwrap();
    let postcondition = ClauseId::new("postcondition-main").unwrap();
    let diagnostic = generate_tristate_harness(&HarnessRequest {
        requirement: environment.owner(),
        precondition_clause: &precondition,
        postcondition_clause: &postcondition,
        precondition: &precondition_expression,
        postcondition: &postcondition_expression,
        execution_point: "bad\npoint",
        manifest: manifest_context(),
    })
    .unwrap_err();
    assert_eq!(diagnostic[0].code, HarnessErrorCode::InvalidExecutionPoint);
    assert_eq!(
        diagnostic[0].terminal_state,
        GenerationTerminalState::InvalidInput
    );
    assert_eq!(diagnostic[0].path, "execution_point");
}

/// TC-003, TC-004.
#[test]
fn tc_004_multiple_state_bindings_fail_closed() {
    let environment = DeclarationEnvironment::new(
        requirement(),
        vec![],
        ["left", "right"]
            .into_iter()
            .enumerate()
            .map(|(index, value)| {
                ValueDeclaration::new(
                    SymbolName::new(value).unwrap(),
                    ValueDeclarationKind::State,
                    ValueType::Boolean,
                    span(index as u64, index as u64 + 1),
                )
            })
            .collect(),
        vec![],
    )
    .unwrap();
    let state = |name: &str, at: u64| {
        Expression::new(
            ExpressionKind::ValueReference {
                name: SymbolName::new(name).unwrap(),
                observation: StateObservation::Post,
            },
            span(at, at + 1),
        )
    };
    let postcondition = Expression::new(
        ExpressionKind::Boolean {
            operator: BooleanOperator::TotalAnd,
            left: Box::new(state("left", 2)),
            right: Box::new(state("right", 3)),
        },
        span(2, 4),
    );
    let precondition = environment
        .check_expression(
            &Expression::new(ExpressionKind::BooleanLiteral { value: true }, span(4, 5)),
            &ValueType::Boolean,
            &ExecutionPoint::Pre {
                operation: AnchorName::new("update").unwrap(),
            },
            true,
        )
        .unwrap();
    let postcondition = environment
        .check_expression(
            &postcondition,
            &ValueType::Boolean,
            &ExecutionPoint::Handler {
                name: AnchorName::new("update").unwrap(),
            },
            true,
        )
        .unwrap();
    let precondition_clause = ClauseId::new("precondition-main").unwrap();
    let postcondition_clause = ClauseId::new("postcondition-main").unwrap();
    let diagnostics = generate_tristate_harness(&HarnessRequest {
        requirement: environment.owner(),
        precondition_clause: &precondition_clause,
        postcondition_clause: &postcondition_clause,
        precondition: &precondition,
        postcondition: &postcondition,
        execution_point: "handler:update",
        manifest: manifest_context(),
    })
    .unwrap_err();
    assert_eq!(
        diagnostics[0].code,
        HarnessErrorCode::UnsupportedHarnessBinding
    );
    assert_eq!(diagnostics[0].path, "clauses.dependencies");
    assert_eq!(
        diagnostics[0].terminal_state,
        GenerationTerminalState::Unsupported
    );
}
