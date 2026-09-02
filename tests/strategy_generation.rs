use std::{
    fs,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use quire_contract_codegen::{
    generate_enum_strategy, generate_i64_strategy, DerivationManifest, EnumStrategyCampaign,
    EnumStrategyRequest, GenerationErrorCode, GenerationTerminalState, ManifestContext,
    StrategyCampaign, StrategyConstraint, StrategyErrorCode, StrategyRequest,
    IR_CANDIDATE_REVISION,
};
use quire_contract_ir::{PackageId, RequirementId, RequirementRef, RequirementRevision};

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
        PackageId::new("agent-ix/strategy-test").unwrap(),
        RequirementId::new("FR-002").unwrap(),
        RequirementRevision::new(2).unwrap(),
    )
}

fn manifest_context() -> ManifestContext<'static> {
    ManifestContext {
        candidate_revision: IR_CANDIDATE_REVISION,
        contribution_method: "generated",
        reviewers: &["@strategy-test-reviewer"],
        result_status: "conclusive",
        result_summary: "strategy generated from a typed constraint request",
        requirement_refs: &["FR-002"],
    }
}

fn function_name(source: &str) -> &str {
    source
        .lines()
        .find(|line| line.contains("BoxedStrategy"))
        .and_then(|line| line.strip_prefix("pub fn "))
        .and_then(|signature| signature.split('(').next())
        .unwrap()
}

fn embedded_source(source: &str) -> String {
    let mut lines = source.lines();
    assert_eq!(lines.next(), Some("#![deny(missing_docs)]"));
    let crate_docs = lines.next().unwrap().trim_start_matches("//!");
    format!("//{crate_docs}\n{}\n", lines.collect::<Vec<_>>().join("\n"))
}

/// TC-004.
#[test]
fn tc_004_supported_populations_are_directly_shaped_and_shrink_inside_constraints() {
    let requirement = requirement();
    let range = generate_i64_strategy(&StrategyRequest {
        requirement: &requirement,
        manifest: manifest_context(),
        strategy_id: "range-broad",
        constraint: StrategyConstraint::InclusiveRange {
            minimum: 10,
            maximum: 20,
        },
        campaign: StrategyCampaign::Broad,
    })
    .unwrap();
    let range_again = generate_i64_strategy(&StrategyRequest {
        requirement: &requirement,
        manifest: manifest_context(),
        strategy_id: "range-broad",
        constraint: StrategyConstraint::InclusiveRange {
            minimum: 10,
            maximum: 20,
        },
        campaign: StrategyCampaign::Broad,
    })
    .unwrap();
    assert_eq!(range, range_again);
    let derivation: DerivationManifest = serde_json::from_str(&range.manifest.contents).unwrap();
    assert_eq!(derivation.outputs[0].uri, range.rust.path);
    assert_eq!(derivation.outputs[0].role, "generated-rust-strategy");

    let correlated = generate_i64_strategy(&StrategyRequest {
        requirement: &requirement,
        manifest: manifest_context(),
        strategy_id: "correlated-broad",
        constraint: StrategyConstraint::CorrelatedOffset {
            primary_minimum: -20,
            primary_maximum: 20,
            offset_minimum: 2,
            offset_maximum: 4,
        },
        campaign: StrategyCampaign::Broad,
    })
    .unwrap();
    assert!(!correlated.rust.contents.contains("prop_filter"));

    let residual = generate_i64_strategy(&StrategyRequest {
        requirement: &requirement,
        manifest: manifest_context(),
        strategy_id: "residual-broad",
        constraint: StrategyConstraint::ResidualExclusion {
            minimum: 0,
            maximum: 3,
            excluded: &[0, 1, 2],
        },
        campaign: StrategyCampaign::Broad,
    })
    .unwrap();
    assert!(residual.rust.contents.contains("ExpectedDomain"));
    assert!(residual.rust.contents.contains("::Rejected"));

    let range_function = function_name(&range.rust.contents);
    let correlated_function = function_name(&correlated.rust.contents);
    let residual_function = function_name(&residual.rust.contents);
    let temporary = TemporaryDirectory::new("quire-generated-strategies");
    fs::write(
        temporary.0.join("Cargo.toml"),
        "[package]\nname = \"generated-strategy-check\"\nversion = \"0.0.0\"\nedition = \"2021\"\n\n[dependencies]\nproptest = { version = \"=1.5.0\", default-features = false, features = [\"std\"] }\nquire-contract-runtime = { git = \"https://github.com/agent-ix/quire-contract-runtime\", rev = \"e360dad8a3e0e54f9b8457ff7f3748be0f2acdb3\" }\n",
    )
    .unwrap();
    let executed_tests = format!(
        r#"

#[cfg(test)]
mod generated_tests {{
    use super::*;
    use proptest::strategy::{{Strategy as _, ValueTree as _}};
    use proptest::test_runner::TestRunner;

    #[test]
    fn supported_shrinks_stay_shaped_and_residuals_stay_rejected() {{
        let mut runner = TestRunner::deterministic();
        for _ in 0..64 {{
            let mut tree = {range_function}().new_tree(&mut runner).unwrap();
            loop {{
                let case = tree.current();
                assert!((10..=20).contains(&case.primary));
                assert_eq!(format!("{{:?}}", case.expected), "Accepted");
                assert!(!case.expected.expects_rejection());
                case.expected
                    .verify(quire_contract_runtime::VerdictKind::Passed)
                    .unwrap();
                assert!(case
                    .expected
                    .verify(quire_contract_runtime::VerdictKind::RejectedPrecondition)
                    .is_err());
                if !tree.simplify() {{
                    break;
                }}
            }}

            let mut tree = {correlated_function}().new_tree(&mut runner).unwrap();
            loop {{
                let case = tree.current();
                let related = case.related.unwrap();
                assert!((-20..=20).contains(&case.primary));
                assert!((2..=4).contains(&(related - case.primary)));
                assert_eq!(format!("{{:?}}", case.expected), "Accepted");
                if !tree.simplify() {{
                    break;
                }}
            }}
        }}

        let mut accepted = 0;
        let mut rejected = 0;
        for _ in 0..128 {{
            let case = {residual_function}().new_tree(&mut runner).unwrap().current();
            match format!("{{:?}}", case.expected).as_str() {{
                "Accepted" => {{
                    case.expected
                        .verify(quire_contract_runtime::VerdictKind::FailedPostcondition)
                        .unwrap();
                    accepted += 1;
                }}
                "Rejected" => {{
                    assert!(case.expected.expects_rejection());
                    case.expected
                        .verify(quire_contract_runtime::VerdictKind::RejectedPrecondition)
                        .unwrap();
                    rejected += 1;
                }}
                other => panic!("unexpected domain {{other}}"),
            }}
        }}
        assert!(accepted > 0);
        assert!(rejected > 0);
    }}
}}
"#
    );
    fs::write(
        temporary.0.join("src/lib.rs"),
        format!(
            "{}\n{}\n{}\n{}",
            range.rust.contents,
            embedded_source(&correlated.rust.contents),
            embedded_source(&residual.rust.contents),
            executed_tests
        ),
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
        "generated strategies did not compile and execute:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

/// TC-004.
#[test]
fn tc_004_boundary_campaigns_tag_inside_and_immediately_outside_values() {
    let requirement = requirement();
    let range = generate_i64_strategy(&StrategyRequest {
        requirement: &requirement,
        manifest: manifest_context(),
        strategy_id: "range-boundary",
        constraint: StrategyConstraint::InclusiveRange {
            minimum: 10,
            maximum: 20,
        },
        campaign: StrategyCampaign::Boundary,
    })
    .unwrap();
    for expected in [
        "primary: 9, related: None, expected:",
        "primary: 10, related: None, expected:",
        "primary: 20, related: None, expected:",
        "primary: 21, related: None, expected:",
    ] {
        assert!(range.rust.contents.contains(expected), "missing {expected}");
    }
    assert_eq!(
        range
            .rust
            .contents
            .matches("related: None, expected:")
            .count(),
        4
    );

    let membership = generate_i64_strategy(&StrategyRequest {
        requirement: &requirement,
        manifest: manifest_context(),
        strategy_id: "membership-boundary",
        constraint: StrategyConstraint::Membership { values: &[3, 7] },
        campaign: StrategyCampaign::Boundary,
    })
    .unwrap();
    for value in [2, 3, 4, 6, 7, 8] {
        assert!(
            membership
                .rust
                .contents
                .contains(&format!("primary: {value},")),
            "missing adjacent membership case {value}"
        );
    }

    let residual = generate_i64_strategy(&StrategyRequest {
        requirement: &requirement,
        manifest: manifest_context(),
        strategy_id: "residual-boundary",
        constraint: StrategyConstraint::ResidualExclusion {
            minimum: 0,
            maximum: 10,
            excluded: &[4, 8],
        },
        campaign: StrategyCampaign::Boundary,
    })
    .unwrap();
    for value in [3, 4, 5, 7, 8, 9] {
        assert!(
            residual
                .rust
                .contents
                .contains(&format!("primary: {value},")),
            "missing residual exclusion boundary case {value}"
        );
    }

    let correlated = generate_i64_strategy(&StrategyRequest {
        requirement: &requirement,
        manifest: manifest_context(),
        strategy_id: "correlated-boundary",
        constraint: StrategyConstraint::CorrelatedOffset {
            primary_minimum: 10,
            primary_maximum: 20,
            offset_minimum: 2,
            offset_maximum: 4,
        },
        campaign: StrategyCampaign::Boundary,
    })
    .unwrap();
    for expected in [
        "primary: 9, related: Some(11)",
        "primary: 10, related: Some(11)",
        "primary: 10, related: Some(12)",
        "primary: 20, related: Some(24)",
        "primary: 20, related: Some(25)",
        "primary: 21, related: Some(25)",
    ] {
        assert!(
            correlated.rust.contents.contains(expected),
            "missing {expected}"
        );
    }
    assert!(correlated.rust.contents.matches("::Accepted").count() >= 4);
    assert!(correlated.rust.contents.matches("::Rejected").count() >= 4);

    let pinned = generate_i64_strategy(&StrategyRequest {
        requirement: &requirement,
        manifest: manifest_context(),
        strategy_id: "state-pinned",
        constraint: StrategyConstraint::Membership { values: &[3, 7] },
        campaign: StrategyCampaign::StatePinned { value: 4 },
    })
    .unwrap();
    assert!(pinned.rust.contents.contains("primary: 4"));
    assert!(pinned.rust.contents.contains("::Rejected"));

    let no_event = generate_i64_strategy(&StrategyRequest {
        requirement: &requirement,
        manifest: manifest_context(),
        strategy_id: "no-event",
        constraint: StrategyConstraint::CorrelatedOffset {
            primary_minimum: 0,
            primary_maximum: 10,
            offset_minimum: 0,
            offset_maximum: 2,
        },
        campaign: StrategyCampaign::NoEvent { value: 5 },
    })
    .unwrap();
    assert!(no_event
        .rust
        .contents
        .contains("primary: 5, related: Some(5)"));
    assert!(no_event.rust.contents.contains("::Accepted"));
}

/// TC-004.
#[test]
fn tc_004_boundary_pinned_and_no_event_bodies_compile_and_execute_with_denied_warnings() {
    let requirement = requirement();
    let boundary = generate_i64_strategy(&StrategyRequest {
        requirement: &requirement,
        manifest: manifest_context(),
        strategy_id: "compiled-boundary",
        constraint: StrategyConstraint::ResidualExclusion {
            minimum: 0,
            maximum: 10,
            excluded: &[4, 8],
        },
        campaign: StrategyCampaign::Boundary,
    })
    .unwrap();
    let pinned = generate_i64_strategy(&StrategyRequest {
        requirement: &requirement,
        manifest: manifest_context(),
        strategy_id: "compiled-pinned",
        constraint: StrategyConstraint::Membership { values: &[3, 7] },
        campaign: StrategyCampaign::StatePinned { value: 3 },
    })
    .unwrap();
    let no_event = generate_i64_strategy(&StrategyRequest {
        requirement: &requirement,
        manifest: manifest_context(),
        strategy_id: "compiled-no-event",
        constraint: StrategyConstraint::CorrelatedOffset {
            primary_minimum: 0,
            primary_maximum: 10,
            offset_minimum: 0,
            offset_maximum: 2,
        },
        campaign: StrategyCampaign::NoEvent { value: 5 },
    })
    .unwrap();
    let boundary_function = function_name(&boundary.rust.contents);
    let pinned_function = function_name(&pinned.rust.contents);
    let no_event_function = function_name(&no_event.rust.contents);
    let temporary = TemporaryDirectory::new("quire-generated-campaign-shapes");
    fs::write(
        temporary.0.join("Cargo.toml"),
        "[package]\nname = \"generated-campaign-shapes\"\nversion = \"0.0.0\"\nedition = \"2021\"\n\n[dependencies]\nproptest = { version = \"=1.5.0\", default-features = false, features = [\"std\"] }\nquire-contract-runtime = { git = \"https://github.com/agent-ix/quire-contract-runtime\", rev = \"e360dad8a3e0e54f9b8457ff7f3748be0f2acdb3\" }\n",
    )
    .unwrap();
    let generated_tests = format!(
        r#"

#[cfg(test)]
mod generated_tests {{
    use super::*;
    use proptest::strategy::{{Strategy as _, ValueTree as _}};
    use proptest::test_runner::TestRunner;

    #[test]
    fn every_campaign_body_executes() {{
        let mut runner = TestRunner::deterministic();
        let boundary = {boundary_function}().new_tree(&mut runner).unwrap().current();
        boundary.expected.verify(if (0..=10).contains(&boundary.primary)
            && ![4, 8].contains(&boundary.primary) {{
            quire_contract_runtime::VerdictKind::Passed
        }} else {{
            quire_contract_runtime::VerdictKind::RejectedPrecondition
        }}).unwrap();
        let pinned = {pinned_function}().new_tree(&mut runner).unwrap().current();
        assert_eq!(pinned.primary, 3);
        let no_event = {no_event_function}().new_tree(&mut runner).unwrap().current();
        assert_eq!(no_event.related, Some(no_event.primary));
    }}
}}
"#
    );
    fs::write(
        temporary.0.join("src/lib.rs"),
        format!(
            "{}\n{}\n{}\n{}",
            boundary.rust.contents,
            embedded_source(&pinned.rust.contents),
            embedded_source(&no_event.rust.contents),
            generated_tests,
        ),
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
        "generated campaign shapes did not compile and execute:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

/// TC-003, TC-004.
#[test]
fn tc_004_invalid_shapes_fail_with_structured_diagnostics() {
    let requirement = requirement();
    let reversed = generate_i64_strategy(&StrategyRequest {
        requirement: &requirement,
        manifest: manifest_context(),
        strategy_id: "reversed",
        constraint: StrategyConstraint::InclusiveRange {
            minimum: 2,
            maximum: 1,
        },
        campaign: StrategyCampaign::Broad,
    })
    .unwrap_err();
    assert_eq!(reversed.code, StrategyErrorCode::InvalidRange);

    let duplicate = generate_i64_strategy(&StrategyRequest {
        requirement: &requirement,
        manifest: manifest_context(),
        strategy_id: "duplicates",
        constraint: StrategyConstraint::Membership { values: &[1, 1] },
        campaign: StrategyCampaign::Broad,
    })
    .unwrap_err();
    assert_eq!(duplicate.code, StrategyErrorCode::InvalidMembership);

    let overflow = generate_i64_strategy(&StrategyRequest {
        requirement: &requirement,
        manifest: manifest_context(),
        strategy_id: "overflow",
        constraint: StrategyConstraint::CorrelatedOffset {
            primary_minimum: i64::MAX,
            primary_maximum: i64::MAX,
            offset_minimum: 1,
            offset_maximum: 1,
        },
        campaign: StrategyCampaign::Broad,
    })
    .unwrap_err();
    assert_eq!(overflow.code, StrategyErrorCode::CorrelationOverflow);
    assert_eq!(
        overflow.terminal_state,
        GenerationTerminalState::InvalidInput
    );

    let pinned_correlation = generate_i64_strategy(&StrategyRequest {
        requirement: &requirement,
        manifest: manifest_context(),
        strategy_id: "pinned-correlation",
        constraint: StrategyConstraint::CorrelatedOffset {
            primary_minimum: 0,
            primary_maximum: 10,
            offset_minimum: 0,
            offset_maximum: 2,
        },
        campaign: StrategyCampaign::StatePinned { value: 5 },
    })
    .unwrap_err();
    assert_eq!(
        pinned_correlation.code,
        StrategyErrorCode::UnsupportedCampaignConstraint
    );
    assert_eq!(
        pinned_correlation.terminal_state,
        GenerationTerminalState::Unsupported
    );

    let full_width_boundary = generate_i64_strategy(&StrategyRequest {
        requirement: &requirement,
        manifest: manifest_context(),
        strategy_id: "full-width-boundary",
        constraint: StrategyConstraint::InclusiveRange {
            minimum: i64::MIN,
            maximum: i64::MAX,
        },
        campaign: StrategyCampaign::Boundary,
    })
    .unwrap_err();
    assert_eq!(
        full_width_boundary.code,
        StrategyErrorCode::UnsupportedCampaignConstraint
    );

    let invalid_manifest = ManifestContext {
        candidate_revision: "not-a-revision",
        ..manifest_context()
    };
    let manifest_failure = generate_i64_strategy(&StrategyRequest {
        requirement: &requirement,
        manifest: invalid_manifest,
        strategy_id: "invalid-manifest",
        constraint: StrategyConstraint::InclusiveRange {
            minimum: 0,
            maximum: 1,
        },
        campaign: StrategyCampaign::Broad,
    })
    .unwrap_err();
    assert_eq!(
        manifest_failure.generation_code,
        Some(GenerationErrorCode::InvalidManifestContext)
    );
    assert_eq!(
        manifest_failure.terminal_state,
        GenerationTerminalState::InvalidInput
    );
}

/// TC-004.
#[test]
fn tc_004_customer_enum_memberships_are_directly_shaped_and_validated() {
    let requirement = requirement();
    let broad = generate_enum_strategy(&EnumStrategyRequest {
        requirement: &requirement,
        manifest: manifest_context(),
        strategy_id: "mode-membership",
        enum_type: "Mode",
        variants: &["Idle", "Active"],
        campaign: EnumStrategyCampaign::Broad,
    })
    .unwrap();
    assert!(!broad.rust.contents.contains("prop_filter"));
    assert!(broad.rust.contents.contains("Mode::Idle"));
    assert!(broad.rust.contents.contains("Mode::Active"));

    let no_event = generate_enum_strategy(&EnumStrategyRequest {
        requirement: &requirement,
        manifest: manifest_context(),
        strategy_id: "mode-no-event",
        enum_type: "Mode",
        variants: &["Idle", "Active"],
        campaign: EnumStrategyCampaign::NoEvent { variant: "Idle" },
    })
    .unwrap();
    assert!(no_event
        .rust
        .contents
        .contains("current: Mode::Idle, related: Some(Mode::Idle)"));

    let temporary = TemporaryDirectory::new("quire-generated-enum-strategy");
    fs::write(
        temporary.0.join("Cargo.toml"),
        "[package]\nname = \"generated-enum-strategy-check\"\nversion = \"0.0.0\"\nedition = \"2021\"\n\n[dependencies]\nproptest = { version = \"=1.5.0\", default-features = false, features = [\"std\"] }\nquire-contract-runtime = { git = \"https://github.com/agent-ix/quire-contract-runtime\", rev = \"e360dad8a3e0e54f9b8457ff7f3748be0f2acdb3\" }\n",
    )
    .unwrap();
    let broad_function = function_name(&broad.rust.contents);
    let no_event_function = function_name(&no_event.rust.contents);
    let tests = format!(
        r#"

{broad_source}
{no_event_source}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Customer mode used by the generated strategy compile test.
pub enum Mode {{
    /// Idle mode.
    Idle,
    /// Active mode.
    Active,
    /// Mode outside the generated membership.
    Unsupported,
}}

#[cfg(test)]
mod generated_tests {{
    use super::*;
    use proptest::strategy::{{Strategy as _, ValueTree as _}};
    use proptest::test_runner::TestRunner;

    #[test]
    fn enum_strategy_never_leaves_membership() {{
        let mut runner = TestRunner::deterministic();
        for _ in 0..128 {{
            let case = {broad_function}().new_tree(&mut runner).unwrap().current();
            assert!(matches!(case.current, Mode::Idle | Mode::Active));
            assert!(case.related.is_none());
        }}
        let case = {no_event_function}().new_tree(&mut runner).unwrap().current();
        assert!(matches!(case.current, Mode::Idle));
        assert!(matches!(case.related, Some(Mode::Idle)));
    }}
}}
"#,
        broad_source = broad.rust.contents,
        no_event_source = embedded_source(&no_event.rust.contents),
    );
    fs::write(temporary.0.join("src/lib.rs"), tests).unwrap();
    let output = Command::new(env!("CARGO"))
        .args(["test", "--offline", "--quiet"])
        .env("CARGO_TARGET_DIR", temporary.0.join("target"))
        .env("RUSTFLAGS", "-Dwarnings")
        .current_dir(&temporary.0)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "generated enum strategy did not compile and execute:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let outside = generate_enum_strategy(&EnumStrategyRequest {
        requirement: &requirement,
        manifest: manifest_context(),
        strategy_id: "mode-pinned",
        enum_type: "Mode",
        variants: &["Idle", "Active"],
        campaign: EnumStrategyCampaign::StatePinned {
            variant: "Unsupported",
        },
    })
    .unwrap_err();
    assert_eq!(outside.code, StrategyErrorCode::InvalidMembership);
}
