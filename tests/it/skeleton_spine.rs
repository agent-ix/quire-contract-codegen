//! FR-016: the skeleton spine over one Boolean clause. A Kani obligation is generated
//! from a hand-built bound package, proved, falsified by an injected violation, and the
//! counterexample is replayed natively through QSL's layer-6 `replay` facade, which settles the
//! same violation.
//!
//! What the input passes through, in ADR-011 terms: the clause enters at the Contract IR side as
//! a hand-written `BoundPackage` projection (no contract is compiled, so E1 to E4 and the
//! contract-to-IR step do not run), CG generates the Kani obligation from it (E7), the installed
//! prover proves and falsifies it (E8), and the decoded counterexample is replayed by
//! `qsl_replay::replay` (E9), which recompiles QSL source and evaluates the function. The QSL
//! source here is a hand-mirrored native twin of the subject and clause, tied to the Rust side
//! only by the clause and its argument names. The replay request's limits are unlimited
//! stand-ins, because no proving run carries limits. The package id and parameter node ids come
//! from `qsl_replay::call_site`, and every request type from `qsl-replay`'s own re-exports.
//!
//! The default lane covers the replay adapter against QSL. The `kani` lane (`make kani`,
//! `#[ignore]` here, not part of `make ci`) runs the real prover.

use std::{fs, path::PathBuf};

use qsl_replay::{
    ByteDigest, CanonicalAssignment, DependencySelectionsCause, DigestDomain, DigestRecord,
    ProofCategory, ReplayRefusal, ReplaySource, ScalarLimits, StageLimits, Verdict, WireNodeId,
    WitnessSettlement, MAX_ENCODED_BYTES,
};
use quire_contract_codegen::{
    decode_falsification, execute_kani_obligation, replay_counterexample, replay_falsification,
    DependencyLock, EvidenceFailureCause, KaniExecutionRequest, KaniInstallation,
    KaniObligationHarness, KaniRunOutcome, LockedSource, ReplayInputs, ReplayPackage,
    ReplayPackageError, ReplayParameter, ReplayVerdict, SpineReplayError,
};
use quire_contract_ir::kani::WitnessValue;

use super::kani_obligations::{
    bound_package, supported_contract_harnesses, write_crate, REAL_KANI_TIMEOUT,
};

const SUBJECT_PATH: &str = "crate::subject::withdraw";

const HEALTHY_SUBJECT: &str = "pub mod subject {\n    pub fn withdraw(amount_current: i64, balance_pre: i64) -> i64 {\n        balance_pre - amount_current\n    }\n}\n";
const PROFILE: &str = "profile v = \"quire.value.complete/v1\" version \"1-draft.2\" digest \
    \"sha256:c8c7ae9fbe783286369ecc83f006190f83be4c3c8fc585766617c90f27a25b16\";\n";
const AUTHORITY: &str = "agent-ix";
const IDENTITY: &str = "test:skeleton-spine";
const NAMESPACE: &str = "git";
const REVISION: &str = "r1";
const FUNCTION: &str = "balance_never_grows";

/// The native twin of the clause over the subject: the QSL function the replay evaluates.
fn native_source(post_balance: &str) -> String {
    format!(
        "language \"ix:native\" edition \"1-draft\";\n{PROFILE}\
         function {FUNCTION} using v(amount_current: Int[0, 1000], balance_pre: Int[0, 1000]): \
         Boolean pure {{ {post_balance} <= balance_pre }}\n"
    )
}

const UNLIMITED: ScalarLimits = ScalarLimits {
    integer_bits: u64::MAX,
    decimal_digits: u64::MAX,
    scale_expansion: u64::MAX,
    text_input_bytes: u64::MAX,
    text_scalars: u64::MAX,
    normalized_scalars: u64::MAX,
    unit_edges: u64::MAX,
    value_occurrences: u64::MAX,
    work_units: u64::MAX,
    result_units: u64::MAX,
};

fn locked(identity: &str, bytes: &[u8]) -> LockedSource {
    LockedSource {
        authority: AUTHORITY.to_owned(),
        identity: identity.to_owned(),
        namespace: NAMESPACE.to_owned(),
        revision: REVISION.to_owned(),
        bytes: bytes.to_vec(),
    }
}

/// The proving run's lock for the native twin `source`: unlimited stand-in limits, because no
/// proving run carries limits, and a stand-in backend manifest.
fn inputs(source: &str, function: &str, dependencies: Vec<DependencyLock>) -> ReplayInputs {
    let s1 = ScalarLimits {
        text_input_bytes: u64::try_from(MAX_ENCODED_BYTES).unwrap(),
        ..UNLIMITED
    };
    ReplayInputs {
        source: locked(IDENTITY, source.as_bytes()),
        dependencies,
        function: function.to_owned(),
        backend_manifest: DigestRecord::mint(
            DigestDomain::ToolManifestJcsV1,
            ByteDigest::of(b"kani").as_bytes(),
        ),
        accounting_limits: UNLIMITED,
        stage_limits: StageLimits {
            s1,
            s2: UNLIMITED,
            s3: UNLIMITED,
            s4: UNLIMITED,
        },
    }
}

/// Compiles the hand-mirrored native twin `source` and locates `function` in it.
fn compile_native_twin(source: &str, function: &str) -> ReplayPackage {
    ReplayPackage::new(inputs(source, function, Vec::new()))
        .expect("the native twin compiles and declares the function")
}

/// Replays `values` against `native`, the QSL source compiled and recompiled.
fn replay_against(
    native: &str,
    values: &[(String, WitnessValue)],
) -> Result<qsl_replay::WitnessArmResult, SpineReplayError> {
    let package = compile_native_twin(native, FUNCTION);
    replay_falsification(
        "module::proof",
        "balance-never-grows",
        values,
        &package.parameters(),
        |source| package.request("counterexample", source),
    )
}

fn values(amount: i64, balance: i64) -> Vec<(String, WitnessValue)> {
    vec![
        ("amount_current".to_owned(), WitnessValue::Integer(amount)),
        ("balance_pre".to_owned(), WitnessValue::Integer(balance)),
    ]
}

const VIOLATING_TWIN: &str = "balance_pre + amount_current";
const HEALTHY_TWIN: &str = "balance_pre - amount_current";

fn violation() -> Verdict {
    Verdict::from_category(ProofCategory::Violation)
}

fn success() -> Verdict {
    Verdict::from_category(ProofCategory::Success)
}

/// A falsifying input that satisfies the proved precondition (`amount <= balance`) replays
/// through QSL's `replay` and settles the same violation: the twin carrying the injected
/// violation evaluates the clause to false at the input. QSL evaluated it; nothing here
/// supplies the verdict.
///
/// Trace: FR-016-AC-9, TC-026
#[test]
fn tc_026_a_falsifying_input_replays_through_qsl_to_the_same_violation() {
    let result =
        replay_against(&native_source(VIOLATING_TWIN), &values(1, 5)).expect("the replay settles");
    assert_eq!(
        result.settlement(),
        WitnessSettlement::ReproducedWithEvaluatedWitness
    );
    assert_eq!(result.category(), ProofCategory::Violation);
    assert!(result.disagreement().is_none());
    assert!(
        result.charges().work_units > 0,
        "QSL charged the evaluation"
    );
}

/// The verdict is QSL's evaluation at the witness point: the same twin holds the clause at
/// `amount = 0`, so a witness that is not a counterexample settles `inconclusive` with the
/// proved violation and the replayed success both named.
///
/// Trace: FR-016-AC-9, FR-016-AC-10, TC-026
#[test]
fn tc_026_the_replayed_verdict_depends_on_the_witness_value() {
    let result =
        replay_against(&native_source(VIOLATING_TWIN), &values(0, 5)).expect("the replay settles");
    assert_eq!(result.settlement(), WitnessSettlement::Inconclusive);
    let cause = result.disagreement().expect("both verdicts are named");
    assert_eq!((cause.proved(), cause.replayed()), (violation(), success()));
}

/// The healthy twin holds the clause at a real counterexample's input, so QSL settles
/// `inconclusive` naming both verdicts: replay never repairs a disagreement into a
/// reproduction.
///
/// Trace: FR-016-AC-4, FR-016-AC-10, TC-026
#[test]
fn tc_026_a_healthy_native_twin_does_not_reproduce_the_violation() {
    let result =
        replay_against(&native_source(HEALTHY_TWIN), &values(1, 5)).expect("the replay settles");
    assert_eq!(result.settlement(), WitnessSettlement::Inconclusive);
    let cause = result.disagreement().expect("both verdicts are named");
    assert_eq!((cause.proved(), cause.replayed()), (violation(), success()));
}

/// A decoded value no replay parameter binds is refused before any call.
///
/// Trace: FR-016-AC-11, TC-026
#[test]
fn tc_026_a_value_with_no_parameter_is_refused() {
    let mut extra = values(1, 5);
    extra.push(("stray".to_owned(), WitnessValue::Integer(1)));
    let refusal = replay_against(&native_source(VIOLATING_TWIN), &extra)
        .expect_err("`stray` names no parameter");
    assert!(
        matches!(&refusal, SpineReplayError::UnboundArgument { argument } if argument == "stray"),
        "{refusal}"
    );
}

/// A Boolean value replays as the integer 1 or 0: `flag(b) = b` is false for `false`, which
/// reproduces the violation, and true for `true`, which does not.
///
/// Trace: FR-016-AC-9, TC-026
#[test]
fn tc_026_a_boolean_value_replays_as_zero_or_one() {
    let source = format!(
        "language \"ix:native\" edition \"1-draft\";\n{PROFILE}\
         function flag using v(b: Boolean): Boolean pure {{ b }}\n"
    );
    let package = compile_native_twin(&source, "flag");
    let replay = |value: bool| {
        replay_falsification(
            "module::proof",
            "flag",
            &[("b".to_owned(), WitnessValue::Boolean(value))],
            &package.parameters(),
            |witness| package.request("flag", witness),
        )
        .expect("the replay settles")
    };
    assert_eq!(
        replay(false).settlement(),
        WitnessSettlement::ReproducedWithEvaluatedWitness
    );
    assert_eq!(replay(true).settlement(), WitnessSettlement::Inconclusive);
}

/// Every refusal the adapter has is its own typed error: a delimiter in a transcript field, a
/// node id that makes the transcript inadmissible, a request QSL refuses, and a request whose
/// source is not a witness.
///
/// Trace: FR-016-AC-11, TC-026
#[test]
fn tc_026_each_adapter_refusal_is_its_own_typed_error() {
    let native = native_source(VIOLATING_TWIN);
    let package = compile_native_twin(&native, FUNCTION);
    let parameters = package.parameters();
    let build = |witness| package.request("x", witness);

    let delimiter = replay_falsification("a|b", "c", &values(1, 5), &parameters, build);
    assert!(matches!(delimiter, Err(SpineReplayError::FieldDelimiter)));

    let bad_node = [ReplayParameter {
        argument: "amount_current",
        node_id: "x>>>y",
    }];
    let transcript = replay_falsification(
        "h",
        "c",
        &[("amount_current".to_owned(), WitnessValue::Integer(1))],
        &bad_node,
        build,
    );
    assert!(matches!(transcript, Err(SpineReplayError::Transcript(_))));

    let stale = compile_native_twin(&native_source(HEALTHY_TWIN), FUNCTION);
    let refused = replay_falsification("h", "c", &values(1, 5), &parameters, |witness| {
        let mut wire = package.request("x", witness);
        wire.package_id = stale
            .request("x", ReplaySource::Input(Vec::new()))
            .package_id;
        wire
    });
    assert!(matches!(refused, Err(SpineReplayError::Refused(_))));

    let wrong_arm = replay_falsification("h", "c", &values(1, 5), &parameters, |_| {
        let input = package
            .parameters()
            .iter()
            .zip([1_i64, 5])
            .map(|(parameter, value)| CanonicalAssignment {
                parameter: WireNodeId::from_hex(parameter.node_id).expect("a node id"),
                value,
            })
            .collect();
        package.request("x", ReplaySource::Input(input))
    });
    assert!(matches!(wrong_arm, Err(SpineReplayError::WrongArm)));
}

/// One dependency selection of the proved lock, with its own source.
fn dependency_lock() -> DependencyLock {
    DependencyLock {
        identity: "test/units".to_owned(),
        version: "1".to_owned(),
        package_id: DigestRecord::mint(DigestDomain::PackageSemanticV2, [7; 32]),
        sources: vec![locked("lib-units", b"a dependency source")],
    }
}

/// The request's package reference carries one `dependencies` entry per lock selection, copied
/// field for field with the dependency's own sources and labelled with each record's own digest
/// domain, in ascending identity order whatever order the lock lists them in. A source shared
/// by the unit and a dependency, or by two dependencies, is provided once.
///
/// Trace: TC-026
#[test]
fn tc_026_the_request_package_reference_carries_the_lock_dependencies() {
    let native = native_source(VIOLATING_TWIN);
    // Minted under a domain other than the one QSL expects, so the wire must carry the record's
    // own label rather than a fixed one.
    let mut lock = dependency_lock();
    lock.package_id = DigestRecord::mint(DigestDomain::VerificationJcs, [7; 32]);
    let mut earlier = dependency_lock();
    earlier.identity = "test/aaa".to_owned();
    earlier.sources = vec![
        locked("lib-shared", native.as_bytes()),
        locked("lib-aaa", b"shared dependency bytes"),
    ];
    let mut shared = dependency_lock();
    shared.identity = "test/mmm".to_owned();
    shared.sources = vec![locked("lib-mmm", b"shared dependency bytes")];
    let mut lock_inputs = inputs(&native, FUNCTION, vec![lock.clone(), shared, earlier]);
    lock_inputs.backend_manifest = DigestRecord::mint(DigestDomain::VerificationJcs, [9; 32]);
    let package = ReplayPackage::new(lock_inputs).expect("the twin compiles");
    let wire = package.request("counterexample", ReplaySource::Input(Vec::new()));

    let identities: Vec<_> = wire
        .dependencies
        .iter()
        .map(|d| d.identity.as_str())
        .collect();
    assert_eq!(identities, ["test/aaa", "test/mmm", "test/units"]);
    assert_eq!(
        wire.backend.1.as_deref(),
        Some(DigestDomain::VerificationJcs.as_str())
    );
    let entry = &wire.dependencies[2];
    assert_eq!(
        entry.package_id.0.as_deref(),
        Some(DigestDomain::VerificationJcs.as_str())
    );
    assert_eq!(entry.version, lock.version);
    assert_eq!(
        entry.package_id,
        (
            Some(lock.package_id.domain().as_str().to_owned()),
            lock.package_id.hex()
        )
    );
    let [source] = entry.sources.as_slice() else {
        panic!("the dependency's own lock sources: {:?}", entry.sources);
    };
    assert_eq!(
        (source.0.as_str(), source.1.as_str()),
        (AUTHORITY, "lib-units")
    );
    assert!(wire
        .byte_provision
        .iter()
        .any(|(_, digest, bytes)| *digest == source.5 && bytes == b"a dependency source"));

    // Three distinct byte strings across five source references: the unit's source (also
    // listed by `test/aaa`), the shared bytes (listed by two dependencies), and `lib-units`.
    let digests: Vec<_> = wire
        .byte_provision
        .iter()
        .map(|(_, digest, _)| digest)
        .collect();
    assert_eq!(digests.len(), 3, "{digests:?}");

    let without = compile_native_twin(&native, FUNCTION)
        .request("counterexample", ReplaySource::Input(Vec::new()));
    assert!(without.dependencies.is_empty());
}

/// A lock that selects one library twice has no admitted request, so the package is refused
/// rather than emitting an entry list QSL refuses.
///
/// Trace: TC-026
#[test]
fn tc_026_a_lock_repeating_a_dependency_is_refused() {
    let refusal = ReplayPackage::new(inputs(
        &native_source(VIOLATING_TWIN),
        FUNCTION,
        vec![dependency_lock(), dependency_lock()],
    ))
    .expect_err("the repeated identity is refused");
    assert!(
        matches!(&refusal, ReplayPackageError::DuplicateDependency { identity } if identity == "test/units"),
        "{refusal}"
    );
}

/// `ReplayPackage` compiles the unit through `qsl_replay::call_site`, which admits no dependency
/// input, so the unit selects no library and QSL refuses a request naming a dependency as
/// unselected. Replaying a unit that imports a locked dependency is unreachable through
/// qsl-replay's public API until `call_site` accepts one.
///
/// Trace: TC-026
#[test]
fn tc_026_qsl_refuses_a_dependency_the_standalone_unit_does_not_select() {
    let package = ReplayPackage::new(inputs(
        &native_source(VIOLATING_TWIN),
        FUNCTION,
        vec![dependency_lock()],
    ))
    .expect("the twin compiles");
    let refusal = replay_falsification(
        "module::proof",
        "balance-never-grows",
        &values(1, 5),
        &package.parameters(),
        |source| package.request("counterexample", source),
    )
    .expect_err("QSL refuses the unselected dependency");
    assert!(
        matches!(
            &refusal,
            SpineReplayError::Refused(cause)
                if matches!(
                    cause.as_ref(),
                    ReplayRefusal::DependencySelections(DependencySelectionsCause::Unselected { identity })
                        if identity.as_str() == "test/units"
                )
        ),
        "{refusal}"
    );
}

/// The harness of the hand-built package, the obligation the synthetic transcripts name.
fn spine_harness() -> KaniObligationHarness {
    let package = bound_package(1000);
    supported_contract_harnesses(&package, SUBJECT_PATH).remove(1)
}

/// A Kani playback transcript for `harness` carrying the two argument values in persisted
/// order (`amount_current`, `balance_pre`).
fn playback(harness: &KaniObligationHarness, amount: i64, balance: i64) -> String {
    let identity = &harness.identity;
    let values = [amount, balance]
        .map(|value| {
            let bytes = value.to_le_bytes().map(|byte| byte.to_string()).join(", ");
            format!("        // {value}\n        vec![{bytes}],\n")
        })
        .concat();
    format!(
        "/// Test generated for harness `{}::{}`\n\
         /// Check for `assertion`: \"synthetic assertion\"\n\
         #[test]\n\
         fn kani_concrete_playback_synthetic() {{\n\
             let concrete_vals: Vec<Vec<u8>> = vec![\n{values}    ];\n\
             kani::concrete_playback_run(concrete_vals, synthetic);\n\
         }}\n",
        identity.module_symbol, identity.harness_symbol
    )
}

/// A counterexample in domain that the violating twin falsifies is a reproduced violation.
///
/// Trace: FR-016-AC-3, TC-026
#[test]
fn tc_026_an_in_domain_counterexample_the_twin_falsifies_is_reproduced() {
    let harness = spine_harness();
    let package = compile_native_twin(&native_source(VIOLATING_TWIN), FUNCTION);
    let verdict = replay_counterexample(&harness.identity, &playback(&harness, 1, 5), &package)
        .expect("the replay settles");
    assert_eq!(verdict, ReplayVerdict::Reproduced);
}

/// A decoded value outside its argument's domain is evidence failure, and the replay never
/// runs: the same value inside the domain reproduces against the same twin.
///
/// Trace: FR-016-AC-2, TC-026
#[test]
fn tc_026_an_out_of_domain_counterexample_is_evidence_failure() {
    let harness = spine_harness();
    let package = compile_native_twin(&native_source(VIOLATING_TWIN), FUNCTION);
    let verdict = replay_counterexample(&harness.identity, &playback(&harness, 1, 5000), &package)
        .expect("the verdict is reached");
    assert_eq!(
        verdict,
        ReplayVerdict::EvidenceFailure(EvidenceFailureCause::Domain {
            argument: "balance_pre".to_owned()
        })
    );
}

/// Each class of malformed transcript is a decode evidence failure carrying the decoder's own
/// cause code: no playback block, a playback block with too few values for the schema, and one
/// whose value has the wrong byte width.
///
/// Trace: FR-016-AC-1, TC-026
#[test]
fn tc_026_each_malformed_counterexample_class_is_a_decode_evidence_failure() {
    let harness = spine_harness();
    let package = compile_native_twin(&native_source(VIOLATING_TWIN), FUNCTION);
    let one_value = playback(&harness, 1, 5).replacen(
        "        // 5\n        vec![5, 0, 0, 0, 0, 0, 0, 0],\n",
        "",
        1,
    );
    let short_value =
        playback(&harness, 1, 5).replacen("vec![5, 0, 0, 0, 0, 0, 0, 0]", "vec![5, 0]", 1);
    for (transcript, code) in [
        ("not a playback block".to_owned(), None),
        (one_value, Some("kani_witness_arity_mismatch")),
        (short_value, Some("kani_witness_width_mismatch")),
    ] {
        let verdict = replay_counterexample(&harness.identity, &transcript, &package)
            .expect("the verdict is reached");
        let ReplayVerdict::EvidenceFailure(EvidenceFailureCause::Decode(failure)) = &verdict else {
            panic!("expected a decode failure for {transcript:?}: {verdict:?}");
        };
        if let Some(code) = code {
            assert_eq!(failure.code, code, "{transcript:?}");
        }
    }
}

/// A transcript recorded for another harness is a decode evidence failure and is never
/// replayed: the same values under the right harness reproduce.
///
/// Trace: FR-016-AC-5, TC-026
#[test]
fn tc_026_a_counterexample_for_another_harness_is_never_replayed() {
    let harness = spine_harness();
    let package = compile_native_twin(&native_source(VIOLATING_TWIN), FUNCTION);
    let other = playback(&harness, 1, 5).replace(&harness.identity.harness_symbol, "sibling");
    let verdict =
        replay_counterexample(&harness.identity, &other, &package).expect("the verdict is reached");
    assert!(
        matches!(
            &verdict,
            ReplayVerdict::EvidenceFailure(EvidenceFailureCause::Decode(failure))
                if failure.code == "cg_witness_harness_identity_mismatch"
        ),
        "{verdict:?}"
    );
}

/// A replay that runs and does not reproduce the violation is evidence failure naming QSL's
/// settlement, never a clause success.
///
/// Trace: FR-016-AC-4, TC-026
#[test]
fn tc_026_a_counterexample_the_twin_holds_is_evidence_failure() {
    let harness = spine_harness();
    let package = compile_native_twin(&native_source(HEALTHY_TWIN), FUNCTION);
    let verdict = replay_counterexample(&harness.identity, &playback(&harness, 1, 5), &package)
        .expect("the replay settles");
    assert_eq!(
        verdict,
        ReplayVerdict::EvidenceFailure(EvidenceFailureCause::Verdict {
            settlement: WitnessSettlement::Inconclusive,
            category: ProofCategory::Success,
        })
    );
}

/// Runs `subject` under the installed prover with `harness`, returning the classified outcome.
fn prove(harness: &KaniObligationHarness, subject: &str) -> KaniRunOutcome {
    let installation = KaniInstallation::discover().expect("cargo-kani is installed");
    let directory = write_crate(harness, subject);
    let evidence = execute_kani_obligation(&KaniExecutionRequest {
        installation: &installation,
        harness: harness.into(),
        crate_directory: &directory,
        target_directory: &PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("kani-spine"),
        timeout: REAL_KANI_TIMEOUT,
    })
    .unwrap_or_else(|refusal| panic!("{refusal}"));
    let _ = fs::remove_dir_all(directory);
    evidence.outcome
}

/// The spine over the hand-built bound package. The healthy subject verifies; a violation
/// injected into the subject is falsified, and its counterexample replays through QSL to the
/// same violation.
///
/// Trace: FR-016-AC-9, FR-016-AC-10, TC-026
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_026_one_boolean_clause_goes_from_a_bound_package_through_kani_to_native_replay() {
    let package = bound_package(1000);
    let harness = supported_contract_harnesses(&package, SUBJECT_PATH).remove(1);

    assert_eq!(prove(&harness, HEALTHY_SUBJECT), KaniRunOutcome::Verified);

    // Mutation control inside the generated obligation module: the ensures bound is tightened
    // past what the subject satisfies at zero, so the same healthy subject is falsified.
    let mut mutated = harness.clone();
    let bound = "*post_state >= 0_i64";
    assert_eq!(mutated.rust.contents.matches(bound).count(), 1);
    mutated.rust.contents = mutated.rust.contents.replace(bound, "*post_state >= 1_i64");
    let outcome = prove(&mutated, HEALTHY_SUBJECT);
    assert!(
        matches!(outcome, KaniRunOutcome::Falsified { .. }),
        "the tightened ensures must be falsified: {outcome:?}"
    );

    // The injected violation: the subject credits instead of debiting.
    let violating = HEALTHY_SUBJECT.replace(
        "balance_pre - amount_current",
        "balance_pre + amount_current",
    );
    assert_ne!(violating, HEALTHY_SUBJECT);
    let outcome = prove(&harness, &violating);
    let KaniRunOutcome::Falsified { counterexample } = &outcome else {
        panic!("the injected violation must be falsified: {outcome:?}");
    };

    // The counterexample decodes to typed values, and QSL replays them to the same violation.
    let decoded = decode_falsification(
        &harness.identity.harness_symbol,
        &harness.identity.module_symbol,
        &harness.identity.arguments,
        counterexample,
    )
    .expect("the counterexample decodes");
    let get = |name: &str| match decoded.iter().find(|(n, _)| n == name) {
        Some((_, WitnessValue::Integer(value))) => *value,
        other => panic!("{name} decodes to an integer, got {other:?}"),
    };
    assert!(
        get("amount_current") <= get("balance_pre"),
        "the counterexample satisfies the proved precondition"
    );
    let violating_twin = compile_native_twin(&native_source(VIOLATING_TWIN), FUNCTION);
    assert_eq!(
        replay_counterexample(&harness.identity, counterexample, &violating_twin)
            .expect("QSL settles the replay"),
        ReplayVerdict::Reproduced
    );

    // The same counterexample against the healthy twin does not reproduce.
    let healthy_twin = compile_native_twin(&native_source(HEALTHY_TWIN), FUNCTION);
    assert_eq!(
        replay_counterexample(&harness.identity, counterexample, &healthy_twin)
            .expect("QSL settles the replay"),
        ReplayVerdict::EvidenceFailure(EvidenceFailureCause::Verdict {
            settlement: WitnessSettlement::Inconclusive,
            category: ProofCategory::Success,
        })
    );
}
