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
    call_site, ByteDigest, CanonicalAssignment, DigestDomain, DigestRecord, Identifier,
    ProofCategory, QualifiedName, ReplayRequestWire, ReplaySource, ScalarLimits, SourceIdentity,
    StageLimits, StateEnvironment, Verdict, WireNodeId, WitnessSettlement, MAX_ENCODED_BYTES,
};
use quire_contract_codegen::{
    decode_falsification, execute_kani_obligation, replay_falsification, KaniExecutionRequest,
    KaniInstallation, KaniObligationHarness, KaniRunOutcome, ReplayParameter, SpineReplayError,
};
use quire_contract_ir::kani::WitnessValue;
use sha2::{Digest, Sha256};

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

/// The package identity and each parameter's node id of one compiled QSL source, which the
/// replay request names.
struct Compiled {
    function: &'static str,
    package_id: String,
    parameters: Vec<(String, String)>,
}

/// Compiles the hand-mirrored native twin `source` and reads `function`'s identity from it.
fn compile_native_twin(source: &str, function: &'static str) -> Compiled {
    let site = call_site(
        SourceIdentity::new(AUTHORITY, IDENTITY, NAMESPACE, REVISION),
        IDENTITY,
        source.as_bytes(),
        &selection(function),
    )
    .expect("the native twin compiles and declares the function");
    Compiled {
        function,
        package_id: site.package_id.hex(),
        parameters: site
            .parameters
            .iter()
            .map(|(name, node)| (name.as_str().to_owned(), node.to_string()))
            .collect(),
    }
}

fn selection(function: &str) -> QualifiedName {
    QualifiedName::new(vec![Identifier::new(function).unwrap()]).unwrap()
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

fn source_digest(bytes: &[u8]) -> DigestRecord {
    DigestRecord::mint(
        DigestDomain::SourceBytesV1,
        ByteDigest::of(bytes).as_bytes(),
    )
}

/// The replay request for the compiled twin: the source is digest-addressed in the byte
/// provision, and the limits are unlimited stand-ins.
fn request(
    source: &str,
    proved: &Compiled,
    counterexample: &str,
    replay_source: ReplaySource,
) -> ReplayRequestWire {
    let digest = source_digest(source.as_bytes());
    let s1 = ScalarLimits {
        text_input_bytes: u64::try_from(MAX_ENCODED_BYTES).unwrap(),
        ..UNLIMITED
    };
    // The replay wire names the backend that found the counterexample; a fixed name stands in.
    let backend_digest = Sha256::digest(b"kani");
    ReplayRequestWire {
        contract_version: "quire.native-runtime/v1".to_owned(),
        capability_vocabulary: Some("quire.capability-kind/v1".to_owned()),
        profile_selections: vec![],
        package_id: (
            Some(DigestDomain::PackageSemanticV2.as_str().to_owned()),
            proved.package_id.clone(),
        ),
        package_contract_version: "quire.checked-package/v2".to_owned(),
        source_digests: vec![(
            AUTHORITY.to_owned(),
            IDENTITY.to_owned(),
            NAMESPACE.to_owned(),
            REVISION.to_owned(),
            Some(DigestDomain::SourceBytesV1.as_str().to_owned()),
            digest.hex(),
        )],
        dependencies: Vec::new(),
        selected_function: selection(proved.function),
        source: replay_source,
        originating_counterexample_identity: Sha256::digest(counterexample.as_bytes()).into(),
        backend: (
            "kani".to_owned(),
            Some(DigestDomain::ToolManifestJcsV1.as_str().to_owned()),
            DigestRecord::mint(DigestDomain::ToolManifestJcsV1, backend_digest.into()).hex(),
        ),
        state_environment: StateEnvironment::new(vec![]),
        accounting_limits: UNLIMITED,
        stage_limits: StageLimits {
            s1,
            s2: UNLIMITED,
            s3: UNLIMITED,
            s4: UNLIMITED,
        },
        byte_provision: vec![(
            Some(DigestDomain::SourceBytesV1.as_str().to_owned()),
            digest.hex(),
            source.as_bytes().to_vec(),
        )],
    }
}

fn replay_parameters(proved: &Compiled) -> Vec<ReplayParameter<'_>> {
    proved
        .parameters
        .iter()
        .map(|(argument, node_id)| ReplayParameter { argument, node_id })
        .collect()
}

/// Replays `values` against `native`, the QSL source compiled and recompiled.
fn replay_against(
    native: &str,
    values: &[(String, WitnessValue)],
) -> Result<qsl_replay::WitnessArmResult, SpineReplayError> {
    let proved = compile_native_twin(native, FUNCTION);
    replay_falsification(
        "module::proof",
        "balance-never-grows",
        values,
        &replay_parameters(&proved),
        |source| request(native, &proved, "counterexample", source),
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
    let compiled = compile_native_twin(&source, "flag");
    let replay = |value: bool| {
        replay_falsification(
            "module::proof",
            "flag",
            &[("b".to_owned(), WitnessValue::Boolean(value))],
            &replay_parameters(&compiled),
            |witness| request(&source, &compiled, "flag", witness),
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
    let compiled = compile_native_twin(&native, FUNCTION);
    let parameters = replay_parameters(&compiled);
    let build = |witness| request(&native, &compiled, "x", witness);

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
        let mut wire = request(&native, &compiled, "x", witness);
        wire.package_id.1 = stale.package_id.clone();
        wire
    });
    assert!(matches!(refused, Err(SpineReplayError::Refused(_))));

    let wrong_arm = replay_falsification("h", "c", &values(1, 5), &parameters, |_| {
        let input = compiled
            .parameters
            .iter()
            .zip([1_i64, 5])
            .map(|((_, node), value)| CanonicalAssignment {
                parameter: WireNodeId::from_hex(node).expect("a node id"),
                value,
            })
            .collect();
        request(&native, &compiled, "x", ReplaySource::Input(input))
    });
    assert!(matches!(wrong_arm, Err(SpineReplayError::WrongArm)));
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
    let native = native_source(VIOLATING_TWIN);
    let proved = compile_native_twin(&native, FUNCTION);
    let result = replay_falsification(
        &format!(
            "{}::{}",
            harness.identity.module_symbol, harness.identity.harness_symbol
        ),
        harness.identity.clause.clause().as_str(),
        &decoded,
        &replay_parameters(&proved),
        |source| request(&native, &proved, counterexample, source),
    )
    .expect("QSL settles the replay");
    assert_eq!(
        result.settlement(),
        WitnessSettlement::ReproducedWithEvaluatedWitness
    );
    assert_eq!(result.category(), ProofCategory::Violation);

    // The same counterexample against the healthy twin does not reproduce.
    let healthy = native_source(HEALTHY_TWIN);
    let proved = compile_native_twin(&healthy, FUNCTION);
    let result = replay_falsification(
        "module::proof",
        "balance-never-grows",
        &decoded,
        &replay_parameters(&proved),
        |source| request(&healthy, &proved, counterexample, source),
    )
    .expect("QSL settles the replay");
    assert_eq!(result.settlement(), WitnessSettlement::Inconclusive);
    let cause = result.disagreement().expect("both verdicts are named");
    assert_eq!((cause.proved(), cause.replayed()), (violation(), success()));
}
