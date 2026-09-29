//! FR-016 and FR-023: one Boolean clause goes from contract to Contract IR to a generated Kani
//! obligation, is proved, falsified by an injected violation, and the counterexample is replayed
//! natively through QSL's layer-6 `replay` facade, which settles the same violation.
//!
//! The clause is `balance_post <= balance_pre` over `withdraw`. Its native twin is a QSL
//! function of the same arguments that computes the post balance the way the subject does. The
//! proving run's package identity and parameter node ids come from compiling that QSL source,
//! because the replay request names them; the replay itself is `qsl_replay::replay`, which
//! recompiles the source and evaluates the function.
//!
//! The default lane covers the replay adapter against QSL and the gate's own logic. The `kani`
//! lane (`make kani`, `#[ignore]` here) runs the real prover.

use std::{collections::BTreeMap, fs, path::PathBuf};

use qsl_foundation::{
    digest::{ByteDigest, DigestDomain, DigestRecord},
    SourceIdentity,
};
use qsl_replay::{
    spine::{compile, DependencyInput, SpineLimits},
    ProofCategory, QualifiedName, ReplayRequestWire, ReplaySource, StageLimits, StateEnvironment,
    WitnessSettlement, MAX_ENCODED_BYTES,
};
use quire_contract_codegen::{
    claimed_module_gate, decode_falsification, execute_kani_obligation_with_transcript,
    replay_falsification, KaniExecutionRequest, KaniInstallation, KaniObligationHarness,
    KaniRunOutcome, KaniToolPins, ModuleGateFailure, ModuleStatus, ReplayParameter,
    SpineReplayError,
};
use quire_contract_ir::kani::WitnessValue;
use quire_exact::{Identifier, ScalarLimits};
use sha2::{Digest, Sha256};

use super::kani_obligations::{
    bound_package, pins, supported_contract_harnesses, write_crate, REAL_KANI_TIMEOUT,
};

const SUBJECT_PATH: &str = "crate::subject::withdraw";
/// The claimed modules the gate publishes: the generated obligation module, then the function
/// under proof's module.
const CLAIMED_MODULES: &str = include_str!("../fixtures/skeleton_spine/claimed-modules.txt");

const HEALTHY_SUBJECT: &str = "pub mod subject {\n    pub fn withdraw(amount_current: i64, balance_pre: i64) -> i64 {\n        balance_pre - amount_current\n    }\n}\n\n/// Compiled but never called by the proof.\npub mod dead {\n    pub fn unused() -> i64 {\n        0\n    }\n}\n";
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

/// What the proving run would carry for one compiled source: the package identity and each
/// parameter's node id.
struct Proved {
    package_id: String,
    parameters: Vec<(String, String)>,
}

fn prove_identity(source: &str) -> Proved {
    let compiled = compile(
        SourceIdentity::new(AUTHORITY, IDENTITY, NAMESPACE, REVISION),
        IDENTITY,
        source.as_bytes(),
        &BTreeMap::new(),
        &DependencyInput::default(),
        SpineLimits::default(),
    )
    .expect("the native twin compiles");
    let graph = compiled.package.graph();
    let callable = graph.callable(FUNCTION).expect("the function is declared");
    let nodes = graph
        .semantic_graph()
        .node(callable.identity)
        .and_then(|node| node.function_parameters())
        .expect("a function node");
    let parameters = callable
        .parameters
        .iter()
        .zip(nodes)
        .map(|((name, _), node)| (name.clone(), node.to_string()))
        .collect();
    Proved {
        package_id: compiled.emitted.package_id().hex(),
        parameters,
    }
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

/// The replay request for the proving run: the package reference and limits are the run's, the
/// source is digest-addressed in the byte provision.
fn request(
    source: &str,
    proved: &Proved,
    backend: &KaniToolPins,
    counterexample: &str,
    replay_source: ReplaySource,
) -> ReplayRequestWire {
    let digest = source_digest(source.as_bytes());
    let s1 = ScalarLimits {
        text_input_bytes: u64::try_from(MAX_ENCODED_BYTES).unwrap(),
        ..UNLIMITED
    };
    let backend_digest = Sha256::digest(serde_json::to_vec(backend).unwrap());
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
        selected_function: QualifiedName::new(vec![Identifier::new(FUNCTION).unwrap()]).unwrap(),
        source: replay_source,
        originating_counterexample_identity: Sha256::digest(counterexample.as_bytes()).into(),
        backend: (
            format!("kani-{}", backend.kani_version),
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

fn replay_parameters(proved: &Proved) -> Vec<ReplayParameter<'_>> {
    proved
        .parameters
        .iter()
        .map(|(argument, node_id)| ReplayParameter { argument, node_id })
        .collect()
}

/// Replays `values` against `native`, the QSL source proved and recompiled.
fn replay_against(
    native: &str,
    values: &[(String, WitnessValue)],
) -> Result<qsl_replay::WitnessArmResult, SpineReplayError> {
    let proved = prove_identity(native);
    replay_falsification(
        "module::proof",
        "balance-never-grows",
        values,
        &replay_parameters(&proved),
        |source| request(native, &proved, &pins(), "counterexample", source),
    )
}

fn values(amount: i64, balance: i64) -> Vec<(String, WitnessValue)> {
    vec![
        ("amount_current".to_owned(), WitnessValue::Integer(amount)),
        ("balance_pre".to_owned(), WitnessValue::Integer(balance)),
    ]
}

/// A falsifying input replays through QSL's `replay` and settles the same violation: the native
/// twin that carries the injected violation evaluates the clause to false at the input. QSL
/// evaluated it; nothing here supplies the verdict.
///
/// Trace: FR-016-AC-9, TC-026
#[test]
fn tc_026_a_falsifying_input_replays_through_qsl_to_the_same_violation() {
    let result = replay_against(
        &native_source("balance_pre + amount_current"),
        &values(1, 0),
    )
    .expect("the replay settles");
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

/// The same input against the healthy twin holds the clause, so QSL settles `inconclusive` with
/// both verdicts named: replay never repairs a disagreement into a reproduction.
///
/// Trace: FR-016-AC-4, FR-016-AC-9, TC-026
#[test]
fn tc_026_a_healthy_native_twin_does_not_reproduce_the_violation() {
    let result = replay_against(
        &native_source("balance_pre - amount_current"),
        &values(1, 0),
    )
    .expect("the replay settles");
    assert_eq!(result.settlement(), WitnessSettlement::Inconclusive);
    assert!(result.disagreement().is_some());
}

/// A decoded value no replay parameter binds is refused, and a Boolean replays as 0 or 1.
///
/// Trace: FR-016-AC-8, TC-026
#[test]
fn tc_026_a_value_with_no_parameter_is_refused() {
    let mut extra = values(1, 0);
    extra.push(("stray".to_owned(), WitnessValue::Boolean(true)));
    let refusal = replay_against(&native_source("balance_pre + amount_current"), &extra)
        .expect_err("`stray` names no parameter");
    assert!(
        matches!(&refusal, SpineReplayError::UnboundArgument { argument } if argument == "stray"),
        "{refusal}"
    );
}

fn claimed() -> Vec<&'static str> {
    CLAIMED_MODULES
        .lines()
        .filter(|line| !line.is_empty())
        .collect()
}

/// Runs `subject` under the pinned prover with `harness`, returning the classified outcome and
/// the prover's transcript.
fn prove(harness: &KaniObligationHarness, subject: &str) -> (KaniRunOutcome, String) {
    let installation = KaniInstallation::discover().expect("cargo-kani is installed");
    let directory = write_crate(harness, subject);
    let (evidence, transcript) = execute_kani_obligation_with_transcript(&KaniExecutionRequest {
        installation: &installation,
        harness: harness.into(),
        crate_directory: &directory,
        target_directory: &PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("kani-spine"),
        timeout: REAL_KANI_TIMEOUT,
    })
    .unwrap_or_else(|refusal| panic!("{refusal}"));
    let _ = fs::remove_dir_all(directory);
    (evidence.outcome, transcript.expect("the run concluded"))
}

/// The spine. The healthy subject verifies and the gate is green with a discharged check in
/// every claimed module; an unclaimed-by-the-proof module is `unreached`; a violation injected
/// into the subject module, and another into the generated obligation module, each turn the gate
/// red; and the subject's counterexample replays through QSL to the same violation.
///
/// Trace: FR-016-AC-9, FR-023-AC-1, FR-023-AC-2, FR-023-AC-3, TC-026, TC-034
#[test]
#[ignore = "kani lane: run serially through `make kani`"]
fn tc_034_one_boolean_clause_goes_from_contract_through_kani_to_native_replay() {
    let package = bound_package(1000);
    let harness = supported_contract_harnesses(&package, &pins(), SUBJECT_PATH).remove(1);
    let claimed = claimed();
    assert_eq!(
        claimed,
        [harness.identity.module_symbol.as_str(), "subject"],
        "the checked-in claimed-module list names the generated module and the subject"
    );

    // Proved: every claimed module has a discharged check.
    let (outcome, transcript) = prove(&harness, HEALTHY_SUBJECT);
    assert_eq!(outcome, KaniRunOutcome::Verified);
    let reports = claimed_module_gate(&claimed, &outcome, &transcript).expect("the gate is green");
    assert!(reports
        .iter()
        .all(|report| matches!(report.status, ModuleStatus::Discharged { .. })));

    // A module the prover compiles but never reaches is unreached, and the gate fails.
    let mut with_dead = claimed.clone();
    with_dead.push("dead");
    assert_eq!(
        claimed_module_gate(&with_dead, &outcome, &transcript),
        Err(ModuleGateFailure::Unreached(vec!["dead".to_owned()]))
    );

    // Mutation control inside the generated obligation module: the ensures bound is tightened
    // past what the subject satisfies at zero.
    let mut mutated = harness.clone();
    let bound = "*post_state >= 0_i64";
    assert_eq!(mutated.rust.contents.matches(bound).count(), 1);
    mutated.rust.contents = mutated.rust.contents.replace(bound, "*post_state >= 1_i64");
    let (outcome, transcript) = prove(&mutated, HEALTHY_SUBJECT);
    assert!(
        matches!(outcome, KaniRunOutcome::Falsified { .. }),
        "{outcome:?}"
    );
    assert!(matches!(
        claimed_module_gate(&claimed, &outcome, &transcript),
        Err(ModuleGateFailure::NotVerified(_))
    ));

    // Mutation control inside the subject module, which is the injected violation: the gate is
    // red and the prover prints a counterexample.
    // The injected violation: the subject credits instead of debiting.
    let violating = HEALTHY_SUBJECT.replace(
        "balance_pre - amount_current",
        "balance_pre + amount_current",
    );
    assert_ne!(violating, HEALTHY_SUBJECT);
    let (outcome, transcript) = prove(&harness, &violating);
    let KaniRunOutcome::Falsified { counterexample } = &outcome else {
        panic!("the injected violation must be falsified: {outcome:?}");
    };
    assert!(matches!(
        claimed_module_gate(&claimed, &outcome, &transcript),
        Err(ModuleGateFailure::NotVerified(_))
    ));

    // The counterexample decodes to typed values, and QSL replays them to the same violation.
    let decoded = decode_falsification(
        &harness.identity.harness_symbol,
        &harness.identity.module_symbol,
        &harness.identity.arguments,
        counterexample,
    )
    .expect("the counterexample decodes");
    let native = native_source("balance_pre + amount_current");
    let proved = prove_identity(&native);
    let result = replay_falsification(
        &format!(
            "{}::{}",
            harness.identity.module_symbol, harness.identity.harness_symbol
        ),
        harness.identity.clause.clause().as_str(),
        &decoded,
        &replay_parameters(&proved),
        |source| request(&native, &proved, &pins(), counterexample, source),
    )
    .expect("QSL settles the replay");
    assert_eq!(
        result.settlement(),
        WitnessSettlement::ReproducedWithEvaluatedWitness
    );
    assert_eq!(result.category(), ProofCategory::Violation);

    // The same counterexample against the healthy twin does not reproduce.
    let healthy = native_source("balance_pre - amount_current");
    let proved = prove_identity(&healthy);
    let result = replay_falsification(
        "module::proof",
        "balance-never-grows",
        &decoded,
        &replay_parameters(&proved),
        |source| request(&healthy, &proved, &pins(), counterexample, source),
    )
    .expect("QSL settles the replay");
    assert_eq!(result.settlement(), WitnessSettlement::Inconclusive);
}
