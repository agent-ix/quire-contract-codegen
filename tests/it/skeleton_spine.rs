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

use qsl_replay::WitnessValue;
use qsl_replay::{
    call_site, ByteDigest, CallSiteRefusal, CanonicalAssignment, Category, DependencyInput,
    DependencySelectionsCause, DigestDomain, DigestRecord, Identifier, ObligationIdentity,
    QualifiedName, ReplayRefusal, ReplaySource, ScalarLimits, SourceIdentity, StageLimits, Verdict,
    WireNodeId, WitnessSettlement, MAX_ENCODED_BYTES,
};
use quire_contract_codegen::{
    decode_falsification, execute_kani_obligation, replay_counterexample,
    replay_counterexample_through, replay_falsification, DependencyLock, DependencyLockError,
    EvidenceFailureCause, KaniExecutionRequest, KaniInstallation, KaniObligationHarness,
    KaniRunOutcome, LockedSource, ObligationKind, ReplayInputs, ReplayPackage, ReplayPackageError,
    ReplayParameter, ReplayVerdict, SpineReplayError,
};

use super::kani_obligations::{
    bound_package, supported_contract_harnesses, write_crate, REAL_KANI_TIMEOUT,
};

const SUBJECT_PATH: &str = "crate::subject::withdraw";

const HEALTHY_SUBJECT: &str = "pub mod subject {\n    pub fn withdraw(amount_current: i64, balance_pre: i64) -> i64 {\n        balance_pre - amount_current\n    }\n}\n";
const PROFILE: &str = "profile v = \"quire.value.complete/v1\";\n";
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

/// A native twin whose clause is false at exactly one point, `(amount, balance)`, and true
/// everywhere else in the declared domain. Replaying a witness against it reproduces the
/// violation only when the witness carries those two values, so the verdict cannot be reached
/// by a constant, a literal or a witness other than the one the prover found.
fn point_source(amount: i64, balance: i64) -> String {
    format!(
        "language \"ix:native\" edition \"1-draft\";\n{PROFILE}\
         function {FUNCTION} using v(amount_current: Int[0, 1000], balance_pre: Int[0, 1000]): \
         Boolean pure {{ amount_current < {amount} or amount_current > {amount} \
         or balance_pre < {balance} or balance_pre > {balance} }}\n"
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
fn inputs(source: &str, dependencies: Vec<DependencyLock>) -> ReplayInputs {
    let s1 = ScalarLimits {
        text_input_bytes: u64::try_from(MAX_ENCODED_BYTES).unwrap(),
        ..UNLIMITED
    };
    ReplayInputs {
        source: locked(IDENTITY, source.as_bytes()),
        dependencies,
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

/// The request for `package` in the tests whose subject is not the identity: the harness of the
/// hand-built package, with its arguments renamed to the function's own parameters, so the slot is
/// the identity `ReplayPackage::request` derives and not a value the test fills in.
fn request_of(package: &ReplayPackage, source: ReplaySource) -> qsl_replay::ReplayRequestWire {
    let mut identity = spine_harness().identity;
    let template = identity.arguments[0].clone();
    identity.arguments = package
        .parameters()
        .iter()
        .map(|parameter| quire_contract_codegen::ObligationBinding {
            identifier: parameter.argument.to_owned(),
            ..template.clone()
        })
        .collect();
    package
        .request(&identity, source)
        .expect("every parameter has a bound harness argument")
}

/// Compiles the hand-mirrored native twin `source` and locates `function` in it.
fn compile_native_twin(source: &str, function: &str) -> ReplayPackage {
    ReplayPackage::new(inputs(source, Vec::new()), function)
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
        |source| request_of(&package, source),
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
    Verdict::from_category(Category::Violation)
}

fn success() -> Verdict {
    Verdict::from_category(Category::Success)
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
    assert_eq!(result.category(), Category::Violation);
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
            |witness| request_of(&package, witness),
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
    let build = |witness| request_of(&package, witness);

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
        let mut wire = request_of(&package, witness);
        wire.package_id = request_of(&stale, ReplaySource::Input(Vec::new())).package_id;
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
                value: WitnessValue::Integer(value),
            })
            .collect();
        request_of(&package, ReplaySource::Input(input))
    });
    assert!(matches!(wrong_arm, Err(SpineReplayError::WrongArm)));
}

/// One dependency selection of the proved lock, with its own source.
fn dependency_lock() -> DependencyLock {
    DependencyLock {
        identity: "test/units".to_owned(),
        version: "1".to_owned(),
        package_id: DigestRecord::mint(DigestDomain::PackageSemanticV2, [7; 32]),
        source: locked("lib-units", b"a dependency source"),
    }
}

/// The request's package reference carries one `dependencies` entry per lock selection, copied
/// field for field with the dependency's own sources and labelled with each record's own digest
/// domain, in ascending identity order whatever order the lock lists them in. A source shared
/// by the unit and a dependency, or by two dependencies, is provided once.
///
/// Trace: FR-016-AC-15, TC-026
#[test]
fn tc_026_the_request_package_reference_carries_the_lock_dependencies() {
    let native = native_source(VIOLATING_TWIN);
    // Minted under a domain other than the one QSL expects, so the wire must carry the record's
    // own label rather than a fixed one.
    let mut lock = dependency_lock();
    lock.package_id = DigestRecord::mint(DigestDomain::VerificationJcs, [7; 32]);
    let mut earlier = dependency_lock();
    earlier.identity = "test/aaa".to_owned();
    earlier.source = locked("lib-shared", native.as_bytes());
    let mut shared = dependency_lock();
    shared.identity = "test/mmm".to_owned();
    shared.source = locked("lib-mmm", b"a dependency source");
    let mut lock_inputs = inputs(&native, vec![lock.clone(), shared, earlier]);
    lock_inputs.backend_manifest = DigestRecord::mint(DigestDomain::VerificationJcs, [9; 32]);
    let package = ReplayPackage::new(lock_inputs, FUNCTION).expect("the twin compiles");
    let wire = request_of(&package, ReplaySource::Input(Vec::new()));

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

    // Two distinct byte strings across four source references: the unit's source (also
    // held by `test/aaa`) and the dependency bytes `test/mmm` and `test/units` both hold.
    let digests: Vec<_> = wire
        .byte_provision
        .iter()
        .map(|(_, digest, _)| digest)
        .collect();
    assert_eq!(digests.len(), 2, "{digests:?}");

    let without = request_of(
        &compile_native_twin(&native, FUNCTION),
        ReplaySource::Input(Vec::new()),
    );
    assert!(without.dependencies.is_empty());
}

/// A lock that selects one library twice has no admitted request, so the package is refused
/// rather than emitting an entry list QSL refuses.
///
/// Trace: TC-026
#[test]
fn tc_026_a_lock_repeating_a_dependency_is_refused() {
    let refusal = ReplayPackage::new(
        inputs(
            &native_source(VIOLATING_TWIN),
            vec![dependency_lock(), dependency_lock()],
        ),
        FUNCTION,
    )
    .expect_err("the repeated identity is refused");
    assert!(
        matches!(
            &refusal,
            ReplayPackageError::Dependencies(DependencyLockError::Duplicate { identity })
                if identity == "test/units"
        ),
        "{refusal}"
    );
}

const UNITS_IDENTITY: &str = "test:units";
const BIG: &str = "function big using v(x: Int[0, 9]): Boolean pure { x > 5 }\n";

/// A library `test/units` holding `body`, as a lock source.
fn units_library(body: &str) -> LockedSource {
    let source = format!("language \"ix:native\" edition \"1-draft\";\n{PROFILE}{body}");
    locked(UNITS_IDENTITY, source.as_bytes())
}

/// The `package_id` QSL compiles `library` to on its own: the identity an importing unit binds.
fn library_package_id(library: &LockedSource) -> DigestRecord {
    let selection = QualifiedName::new(vec![Identifier::new("big").unwrap()]).unwrap();
    call_site(
        SourceIdentity::new(
            &library.authority,
            &library.identity,
            &library.namespace,
            &library.revision,
        ),
        &library.identity,
        &library.bytes,
        [],
        &DependencyInput::default(),
        &selection,
    )
    .expect("the library compiles and declares `big`")
    .package_id
}

/// A unit `q(x) = u::big(x)` importing `library` under the digest the library compiles to, and
/// the lock selecting `library` at that `package_id`.
fn importing_inputs(library: &LockedSource) -> (ReplayInputs, DependencyLock) {
    let package_id = library_package_id(library);
    let unit = format!(
        "language \"ix:native\" edition \"1-draft\";\n{PROFILE}\
         import \"test/units\" version \"2\" digest \"{}\" as u;\n\
         function q using v(x: Int[0, 9]): Boolean pure {{ u::big(x) }}\n",
        package_id.hex()
    );
    let lock = DependencyLock {
        identity: "test/units".to_owned(),
        version: "2".to_owned(),
        package_id,
        source: library.clone(),
    };
    (inputs(&unit, vec![lock.clone()]), lock)
}

fn replay_q(
    package: &ReplayPackage,
    x: i64,
) -> Result<qsl_replay::WitnessArmResult, SpineReplayError> {
    replay_falsification(
        "module::proof",
        "q",
        &[("x".to_owned(), WitnessValue::Integer(x))],
        &package.parameters(),
        |source| request_of(package, source),
    )
}

/// A unit that imports a locked dependency compiles with it and replays through
/// `qsl_replay::replay`: the dependency's `big` is evaluated, so `x = 3` (not big) reproduces the
/// proved violation and `x = 7` (big) does not.
///
/// Trace: FR-016-AC-14, TC-026
#[test]
fn tc_026_a_unit_importing_a_locked_dependency_replays_to_a_reproduced_verdict() {
    let (lock_inputs, _) = importing_inputs(&units_library(BIG));
    let package =
        ReplayPackage::new(lock_inputs, "q").expect("the unit compiles with its dependency");
    let wire = request_of(&package, ReplaySource::Input(Vec::new()));
    assert_eq!(wire.dependencies.len(), 1);

    let reproduced = replay_q(&package, 3).expect("the replay settles");
    assert_eq!(
        reproduced.settlement(),
        WitnessSettlement::ReproducedWithEvaluatedWitness
    );
    assert_eq!(reproduced.category(), Category::Violation);

    let held = replay_q(&package, 7).expect("the replay settles");
    assert_eq!(held.settlement(), WitnessSettlement::Inconclusive);
}

/// The dependency is compiled from the lock's own source, not assumed: a unit whose import names
/// no supplied library is refused at the call site.
///
/// Trace: FR-016-AC-16, TC-026
#[test]
fn tc_026_an_import_with_no_locked_dependency_is_refused() {
    let (mut lock_inputs, _) = importing_inputs(&units_library(BIG));
    lock_inputs.dependencies.clear();
    let refusal =
        ReplayPackage::new(lock_inputs, "q").expect_err("no library satisfies the import");
    assert!(
        matches!(&refusal, ReplayPackageError::CallSite(cause) if matches!(**cause, CallSiteRefusal::Import { .. })),
        "{refusal}"
    );
}

/// The request's dependency entry carries the lock's recorded `package_id`, which QSL checks
/// against the dependency it recompiles: a lock recording another identity is refused.
///
/// Trace: FR-016-AC-17, TC-026
#[test]
fn tc_026_a_lock_recording_another_dependency_identity_is_refused() {
    let (mut lock_inputs, _) = importing_inputs(&units_library(BIG));
    lock_inputs.dependencies[0].package_id =
        DigestRecord::mint(DigestDomain::PackageSemanticV2, [7; 32]);
    let package =
        ReplayPackage::new(lock_inputs, "q").expect("the unit compiles with its dependency");
    let refusal = replay_q(&package, 3).expect_err("QSL refuses the stale dependency identity");
    assert!(
        matches!(
            &refusal,
            SpineReplayError::Refused(cause)
                if matches!(
                    cause.as_ref(),
                    ReplayRefusal::DependencyIdentityMismatch { identity, .. }
                        if identity.as_str() == "test/units"
                )
        ),
        "{refusal}"
    );
}

/// A lock selecting a library the unit does not import compiles, but QSL refuses the request
/// naming it as unselected.
///
/// Trace: FR-016-AC-18, TC-026
#[test]
fn tc_026_qsl_refuses_a_dependency_the_unit_does_not_select() {
    let package = ReplayPackage::new(
        inputs(&native_source(VIOLATING_TWIN), vec![dependency_lock()]),
        FUNCTION,
    )
    .expect("the twin compiles");
    let refusal = replay_falsification(
        "module::proof",
        "balance-never-grows",
        &values(1, 5),
        &package.parameters(),
        |source| request_of(&package, source),
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

/// Two lock libraries whose sources share an owner are no dependency input, so the package is
/// refused before the call site compiles anything.
///
/// Trace: FR-016-AC-19, TC-026
#[test]
fn tc_026_libraries_sharing_a_source_owner_are_refused() {
    let first = dependency_lock();
    let mut second = dependency_lock();
    second.identity = "test/other".to_owned();
    let refusal = ReplayPackage::new(
        inputs(&native_source(VIOLATING_TWIN), vec![first, second]),
        FUNCTION,
    )
    .expect_err("one source owner per library");
    assert!(
        matches!(
            &refusal,
            ReplayPackageError::Dependencies(DependencyLockError::Input(_))
        ),
        "{refusal}"
    );
}

/// A lock library whose source has the unit's own owner is refused by QSL's call site.
///
/// Trace: FR-016-AC-20, TC-026
#[test]
fn tc_026_a_library_sharing_the_units_source_owner_is_refused() {
    let mut lock = dependency_lock();
    lock.source = locked(IDENTITY, b"a dependency source");
    let refusal = ReplayPackage::new(inputs(&native_source(VIOLATING_TWIN), vec![lock]), FUNCTION)
        .expect_err("the unit and a library never share an owner");
    assert!(
        matches!(
            &refusal,
            ReplayPackageError::CallSite(cause)
                if matches!(**cause, CallSiteRefusal::DependencyInput(_))
        ),
        "{refusal}"
    );
}

// ---- The function-contract obligation identity (FR-016-AC-21 to AC-23) -------------------------

/// A second function over the same parameters as [`FUNCTION`], and a two-conjunct function.
const SIBLING: &str = "balance_never_shrinks";
const TWO_CONJUNCTS: &str = "balance_within_limit";

/// A unit declaring `FUNCTION`, a sibling with the same parameters and a two-conjunct function,
/// with `preface` before the first declaration.
fn identity_unit(preface: &str) -> String {
    let parameters = "amount_current: Int[0, 1000], balance_pre: Int[0, 1000]";
    format!(
        "language \"ix:native\" edition \"1-draft\";\n{PROFILE}{preface}\
         function {FUNCTION} using v({parameters}): Boolean pure {{ balance_pre - amount_current <= balance_pre }}\n\
         function {SIBLING} using v({parameters}): Boolean pure {{ balance_pre <= balance_pre + amount_current }}\n\
         function {TWO_CONJUNCTS} using v({parameters}): Boolean pure \
         {{ balance_pre - amount_current <= balance_pre and amount_current <= 1000 }}\n"
    )
}

/// The identity of `function` in `source` for the harness `identity`.
fn identity_of(
    source: &str,
    function: &str,
    identity: &quire_contract_codegen::KaniObligationIdentity,
) -> ObligationIdentity {
    compile_native_twin(source, function)
        .obligation_identity(identity)
        .expect("the harness's arguments name the function's parameters")
}

/// The `FunctionSite` QSL itself locates for `function`, read without CG.
fn site_of(source: &str, function: &str) -> qsl_replay::FunctionSite {
    let selection = QualifiedName::new(vec![Identifier::new(function).unwrap()]).unwrap();
    call_site(
        SourceIdentity::new(AUTHORITY, IDENTITY, NAMESPACE, REVISION),
        IDENTITY,
        source.as_bytes(),
        [],
        &DependencyInput::default(),
        &selection,
    )
    .expect("the unit compiles and declares the function")
    .site
}

/// The golden spelling of an integer-range domain: RFC 8785 text, members in key order, bounds as
/// decimal strings.
fn range_text(minimum: i64, maximum: i64) -> String {
    format!("{{\"maximum\":\"{maximum}\",\"minimum\":\"{minimum}\",\"type\":\"integerRange\"}}")
}

/// The golden spelling of a Boolean domain.
const BOOLEAN_TEXT: &str = "{\"type\":\"boolean\"}";

/// O-09's digest recomputed without CG's encoder or CG's serde spellings: the RFC 8785 text
/// written out by hand, members in key order, hashed with SHA-256. `kind` is the kind's spelling,
/// written by the caller, and `arguments` are `(identifier, domain text)` in ascending identifier
/// order, each joined to the parameter node id of QSL's own `FunctionSite`.
fn recomputed(
    site: &qsl_replay::FunctionSite,
    kind: &str,
    arguments: &[(&str, String)],
) -> [u8; 32] {
    use sha2::{Digest, Sha256};
    assert!(
        arguments.windows(2).all(|pair| pair[0].0 < pair[1].0),
        "the golden arguments are ascending by identifier"
    );
    let argument = |(name, domain): &(&str, String)| {
        let parameter = site
            .parameters
            .iter()
            .find(|(declared, _)| declared.as_str() == *name)
            .map(|(_, node)| node.to_string())
            .expect("the function declares the parameter");
        format!("{{\"domain\":{domain},\"parameter\":\"{parameter}\"}}")
    };
    let text = format!(
        "{{\"arguments\":[{}],\"declaration\":{{\"node\":\"{}\",\"ordinal\":0,\
         \"role\":\"declaration\"}},\"function\":\"{}\",\"kind\":\"{kind}\"}}",
        arguments.iter().map(argument).collect::<Vec<_>>().join(","),
        site.function,
        site.function,
    );
    Sha256::digest(text.as_bytes()).into()
}

/// The golden arguments of the hand-built package's harness: `amount_current` and `balance_pre`,
/// each with the bounds the harness declares.
fn golden_arguments(
    identity: &quire_contract_codegen::KaniObligationIdentity,
) -> [(&'static str, String); 2] {
    let text = |index: usize| {
        let bounds = identity.arguments[index]
            .integer_bounds
            .as_ref()
            .expect("the argument is bounded");
        range_text(bounds.minimum, bounds.maximum)
    };
    [("amount_current", text(0)), ("balance_pre", text(1))]
}

/// The request's `obligation_identity` slot holds the O-09 function-contract digest recomputed
/// independently from the members QSL's `FunctionSite` carries, the harness's kind and its
/// arguments, and it is not the digest of the transcript.
///
/// Trace: FR-016-AC-21, TC-026
#[test]
fn tc_026_the_request_slot_is_the_recomputed_function_contract_digest() {
    let harness = spine_harness();
    assert_eq!(harness.identity.kind, ObligationKind::Postcondition);
    let source = identity_unit("");
    let package = compile_native_twin(&source, FUNCTION);
    let wire = package
        .request(&harness.identity, ReplaySource::Input(Vec::new()))
        .expect("the identity is built");
    let expected = recomputed(
        &site_of(&source, FUNCTION),
        "postcondition",
        &golden_arguments(&harness.identity),
    );
    assert_eq!(wire.obligation_identity, expected);
    let transcript = playback(&harness, 1, 5);
    assert_ne!(
        wire.obligation_identity,
        ByteDigest::of(transcript.as_bytes()).as_bytes()
    );
}

/// The request `replay_counterexample` hands QSL carries the recomputed O-09 digest in its slot:
/// the request is captured at the executor seam and then replayed for real, so a slot filled
/// with any other value fails here.
///
/// Trace: FR-016-AC-21, TC-026
#[test]
fn tc_026_the_request_replay_counterexample_sends_carries_the_recomputed_digest() {
    let harness = spine_harness();
    let source = identity_unit("");
    let package = compile_native_twin(&source, FUNCTION);
    let mut sent = Vec::new();
    // The function holds for every in-domain input, so this replays to an evidence failure; the
    // verdict is not what is under test, the request is.
    replay_counterexample_through(
        &harness.identity,
        &playback(&harness, 1, 5),
        &package,
        |wire| {
            sent.push(wire.obligation_identity);
            qsl_replay::replay(wire)
        },
    )
    .expect("QSL settles the replay");
    let expected = recomputed(
        &site_of(&source, FUNCTION),
        "postcondition",
        &golden_arguments(&harness.identity),
    );
    assert_eq!(sent, vec![expected]);
}

/// A Boolean argument has its own golden spelling, and the arguments are ordered by identifier
/// even where that order differs from the order of the parameters' node ids.
///
/// Trace: FR-016-AC-21, TC-026
#[test]
fn tc_026_a_boolean_argument_and_identifier_order_match_the_golden_text() {
    let harness = spine_harness();
    let source = format!(
        "language \"ix:native\" edition \"1-draft\";\n{PROFILE}\
         function flagged using v(flag: Boolean, amount_current: Int[0, 1000]): Boolean pure \
         {{ flag or amount_current <= 1000 }}\n"
    );
    let mut identity = harness.identity.clone();
    let amount = identity.arguments[0].clone();
    let flag = quire_contract_codegen::ObligationBinding {
        identifier: "flag".to_owned(),
        primitive_type: quire_contract_codegen::KaniPrimitiveType::Boolean,
        integer_bounds: None,
        ..amount.clone()
    };
    // The harness order is the reverse of the declared order too.
    identity.arguments = vec![amount.clone(), flag];
    let site = site_of(&source, "flagged");
    let declared: Vec<_> = site
        .parameters
        .iter()
        .map(|(name, node)| (name.as_str().to_owned(), node.to_string()))
        .collect();
    assert_eq!(
        declared[0].0, "flag",
        "declared order is not identifier order"
    );
    let bounds = amount.integer_bounds.as_ref().expect("bounded");
    let expected = recomputed(
        &site,
        "postcondition",
        &[
            ("amount_current", range_text(bounds.minimum, bounds.maximum)),
            ("flag", BOOLEAN_TEXT.to_owned()),
        ],
    );
    let obligation = compile_native_twin(&source, "flagged")
        .obligation_identity(&identity)
        .expect("the identity is built");
    assert_eq!(*obligation.as_bytes(), expected);
}

/// O-09's `arguments` are the function's parameters: a harness that leaves a parameter without an
/// argument, or declares an integer with no bound or a Boolean with bounds, has no identity.
///
/// Trace: FR-016-AC-21, TC-026
#[test]
fn tc_026_a_harness_that_is_not_the_functions_parameters_has_no_identity() {
    use quire_contract_codegen::ObligationIdentityError as Refusal;
    let harness = spine_harness();
    let package = compile_native_twin(&identity_unit(""), FUNCTION);

    let mut missing = harness.identity.clone();
    missing.arguments.truncate(1);
    assert!(matches!(
        package.obligation_identity(&missing),
        Err(Refusal::UnboundParameter { parameter }) if parameter == "balance_pre"
    ));

    let mut unbounded = harness.identity.clone();
    unbounded.arguments[0].integer_bounds = None;
    assert!(matches!(
        package.obligation_identity(&unbounded),
        Err(Refusal::UnboundedDomain { argument }) if argument == "amount_current"
    ));

    let mut bounded_boolean = harness.identity.clone();
    bounded_boolean.arguments[0].primitive_type =
        quire_contract_codegen::KaniPrimitiveType::Boolean;
    assert!(matches!(
        package.obligation_identity(&bounded_boolean),
        Err(Refusal::BoundedBoolean { argument }) if argument == "amount_current"
    ));
}

/// Two functions with the same parameters have different identities; the identity of one function
/// is the same across a recompile that adds only comments and blank lines and across a change of
/// the harness's source span; it differs between two kinds over the same function and when an
/// argument's domain differs.
///
/// Trace: FR-016-AC-22, TC-026
#[test]
fn tc_026_the_identity_separates_functions_kinds_and_domains_and_ignores_text_and_span() {
    let harnesses = supported_contract_harnesses(&bound_package(1000), SUBJECT_PATH);
    let base = &harnesses[1].identity;
    let source = identity_unit("");
    let identity = identity_of(&source, FUNCTION, base);

    assert_eq!(
        site_of(&source, FUNCTION).parameters,
        site_of(&source, SIBLING).parameters,
        "the two functions share their parameters"
    );
    assert_ne!(identity, identity_of(&source, SIBLING, base));

    let commented = identity_unit("// a comment\n\n\n// another\n");
    assert_ne!(commented, source, "the recompiled bytes differ");
    assert_eq!(identity, identity_of(&commented, FUNCTION, base));

    let mut respanned = base.clone();
    respanned.source_span = harnesses[0].identity.source_span.clone();
    assert_ne!(respanned.source_span, base.source_span);
    assert_eq!(identity, identity_of(&source, FUNCTION, &respanned));

    let other_kind = harnesses
        .iter()
        .map(|harness| &harness.identity)
        .find(|other| other.kind != base.kind && other.arguments == base.arguments)
        .expect("a harness of another kind over the same arguments");
    assert_ne!(identity, identity_of(&source, FUNCTION, other_kind));

    let mut narrowed = base.clone();
    narrowed.arguments[1]
        .integer_bounds
        .as_mut()
        .expect("the argument is bounded")
        .maximum -= 1;
    assert_ne!(identity, identity_of(&source, FUNCTION, &narrowed));
}

/// A function whose body has two conjuncts, replayed under one kind, has one identity: the
/// identity is the digest of the function, declaration, kind and arguments, so it equals the
/// recomputed digest, which has no conjunct member.
///
/// Trace: FR-016-AC-22, TC-026
#[test]
fn tc_026_a_two_conjunct_function_has_one_identity_per_kind() {
    let harness = spine_harness();
    let source = identity_unit("");
    let expected = recomputed(
        &site_of(&source, TWO_CONJUNCTS),
        "postcondition",
        &golden_arguments(&harness.identity),
    );
    let package = compile_native_twin(&source, TWO_CONJUNCTS);
    let obligation = package.obligation_identity(&harness.identity).unwrap();
    assert_eq!(*obligation.as_bytes(), expected);
    assert_eq!(
        obligation,
        package.obligation_identity(&harness.identity).unwrap()
    );
}

/// With the function, declaration, kind and arguments fixed, an unrelated declaration added to
/// the unit leaves the identity unchanged, and a harness argument that names no parameter is
/// refused rather than guessed.
///
/// Trace: FR-016-AC-23, TC-026
#[test]
fn tc_026_an_unrelated_declaration_does_not_change_the_identity() {
    let harness = spine_harness();
    let alone = native_source("balance_pre - amount_current");
    let before = identity_of(&alone, FUNCTION, &harness.identity);
    let extended =
        format!("{alone}function unrelated using v(z: Int[0, 5]): Boolean pure {{ z <= 5 }}\n");
    assert_eq!(before, identity_of(&extended, FUNCTION, &harness.identity));

    let mut unbound = harness.identity.clone();
    unbound.arguments[0].identifier = "not_a_parameter".to_owned();
    let package = compile_native_twin(&alone, FUNCTION);
    assert!(matches!(
        package.obligation_identity(&unbound),
        Err(quire_contract_codegen::ObligationIdentityError::UnboundArgument { argument })
            if argument == "not_a_parameter"
    ));
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
    let other =
        playback(&harness, 1, 5).replace(harness.identity.harness_symbol.as_str(), "sibling");
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
            category: Category::Success,
        })
    );
}

/// The replay verdict is a function of the exact witness values: the twin that is false only at
/// `(1, 5)` is reproduced by the transcript carrying `(1, 5)` and by no neighbouring transcript,
/// so no constant or substituted witness can stand in for the decoded one.
///
/// Trace: FR-016-AC-9, TC-026
#[test]
fn tc_026_the_replay_verdict_is_decided_by_the_exact_witness_values() {
    let harness = spine_harness();
    let package = compile_native_twin(&point_source(1, 5), FUNCTION);
    let replay = |amount: i64, balance: i64| {
        replay_counterexample(
            &harness.identity,
            &playback(&harness, amount, balance),
            &package,
        )
        .expect("the replay settles")
    };
    assert_eq!(replay(1, 5), ReplayVerdict::Reproduced);
    for (amount, balance) in [(0, 5), (2, 5), (1, 4), (1, 6), (5, 1)] {
        assert_eq!(
            replay(amount, balance),
            ReplayVerdict::EvidenceFailure(EvidenceFailureCause::Verdict {
                settlement: WitnessSettlement::Inconclusive,
                category: Category::Success,
            }),
            "({amount}, {balance}) is not the witness point"
        );
    }
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
        harness.identity.harness_symbol.as_str(),
        harness.identity.module_symbol.as_str(),
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

    // The verdict is decided by the values Kani found: a twin false only at the decoded point is
    // reproduced by the real transcript, and a twin false only at a neighbouring point is not.
    let (amount, balance) = (get("amount_current"), get("balance_pre"));
    let point = compile_native_twin(&point_source(amount, balance), FUNCTION);
    assert_eq!(
        replay_counterexample(&harness.identity, counterexample, &point)
            .expect("QSL settles the replay"),
        ReplayVerdict::Reproduced,
        "the replay did not evaluate the decoded witness ({amount}, {balance})"
    );
    let neighbour = if amount < 1000 {
        amount + 1
    } else {
        amount - 1
    };
    let elsewhere = compile_native_twin(&point_source(neighbour, balance), FUNCTION);
    assert!(
        matches!(
            replay_counterexample(&harness.identity, counterexample, &elsewhere)
                .expect("QSL settles the replay"),
            ReplayVerdict::EvidenceFailure(_)
        ),
        "a twin false only away from the witness must not reproduce"
    );

    // The same counterexample against the healthy twin does not reproduce.
    let healthy_twin = compile_native_twin(&native_source(HEALTHY_TWIN), FUNCTION);
    assert_eq!(
        replay_counterexample(&harness.identity, counterexample, &healthy_twin)
            .expect("QSL settles the replay"),
        ReplayVerdict::EvidenceFailure(EvidenceFailureCause::Verdict {
            settlement: WitnessSettlement::Inconclusive,
            category: Category::Success,
        })
    );
}
