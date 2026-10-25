//! Paired proof and native execution of one retained routed scalar artifact (FR-017).

use std::{
    fmt, fs,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
    time::Instant,
};

use qsl_replay::{DigestDomain, DigestRecord};
use quire_canonical::{FixedShape, Limits};
use quire_exact::Cancel;
use quire_exact::{
    ChargePoint, Incomplete, Integer, IntegerInterval, LimitKind, Outcome, Refusal, ScalarLimits,
    Undefined,
};
use serde::Serialize;

use crate::{
    core::{
        artifact::{Artifact, ArtifactBundle, PublicationDiagnostic},
        canonical::{content_digest_with_limits, DigestError},
        profile::{oracle_crate_manifest, scalar_observer_manifest},
    },
    kani::{
        classify::{KaniInconclusiveReason, KaniRunOutcome},
        identity::{KaniScalarObligationHarness, ScalarIntegerRepresentation},
        output::report::KaniFailureCause,
        run::{
            execute::{
                execute_kani_obligation_readonly, KaniExecutionEvidence, KaniExecutionRefusal,
                KaniExecutionRequest,
            },
            harness::KaniExecutableHarness,
            launch::{run_bounded_launcher, LaunchOutcome},
            namespace::BackendCommand,
            tool::KaniInstallation,
        },
    },
    oracle::scalar::EXACT_SCALAR_CRATE_NAME,
};

static NEXT_SNAPSHOT: AtomicU64 = AtomicU64::new(0);
const SCALAR_PROOF_SCHEMA: &str = "agent-ix/cg-kani-scalar-proof-content/v3";
const LOCKED_DEPENDENCIES: [(&str, &str); 2] = [
    (
        "quire-contract-runtime",
        "https://github.com/agent-ix/quire-contract-runtime",
    ),
    ("quire-exact", "https://github.com/agent-ix/quire-exact"),
];

#[derive(Serialize, FixedShape)]
struct CanonicalArtifact {
    path: String,
    contents: String,
}

#[derive(Serialize, FixedShape)]
struct ScalarProofPreimage {
    schema: String,
    artifacts: Vec<CanonicalArtifact>,
}

/// Input retained by a caller that already admitted the routed scalar occurrence.
///
/// A digest and crate path deliberately are not inputs: the runner owns both.
pub struct KaniScalarNativeRequest<'a> {
    /// Caller cancellation.
    pub cancel: &'a Cancel,
    /// Installed bounded Kani launcher.
    pub installation: &'a KaniInstallation,
    /// The generated scalar harness retained by the routed result.
    pub harness: &'a KaniScalarObligationHarness,
    /// Complete generated oracle package returned by routed generation.
    pub oracle_artifacts: &'a [Artifact],
    /// Concrete integer operands in the generated oracle's argument order.
    pub operands: &'a [i64],
    /// Limits retained by the admitted native request.
    pub scalar_limits: ScalarLimits,
    /// Caller-owned Cargo target root; execution files remain in a private snapshot.
    pub target_directory: &'a Path,
}

/// A measured identity of the exact staged package, harness, adapter and dependency lock.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ScalarProofContentIdentity([u8; 32]);

impl ScalarProofContentIdentity {
    /// SHA-256 bytes of the RFC 8785 canonical staged package preimage.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

/// The opaque result of proving and natively invoking one staged scalar artifact.
///
/// ```
/// use quire_contract_codegen::{DigestRecord, KaniScalarNativeReceipt};
/// fn same_type(receipt: &KaniScalarNativeReceipt) -> &qsl_replay::DigestRecord {
///     let record: &DigestRecord = receipt.proof_content_digest();
///     record
/// }
/// ```
/// ```compile_fail
/// use quire_contract_codegen::KaniScalarNativeReceipt;
/// let receipt = KaniScalarNativeReceipt {};
/// ```
/// ```compile_fail,E0616
/// use quire_contract_codegen::KaniScalarNativeReceipt;
/// fn mutate(receipt: &mut KaniScalarNativeReceipt) { receipt.entries.clear(); }
/// ```
/// ```compile_fail,E0596
/// use quire_contract_codegen::KaniScalarNativeReceipt;
/// fn mutate(receipt: &mut KaniScalarNativeReceipt) { receipt.staged_entries()[0].1.clear(); }
/// ```
/// ```compile_fail,E0594
/// use quire_contract_codegen::KaniScalarNativeReceipt;
/// fn mutate(receipt: &mut KaniScalarNativeReceipt) { receipt.meter_observation().limits().work_units = 0; }
/// ```
/// ```compile_fail,E0277
/// use quire_contract_codegen::KaniScalarNativeReceipt;
/// let _: KaniScalarNativeReceipt = serde_json::from_str("{}").unwrap();
/// ```
#[derive(Debug)]
pub struct KaniScalarNativeReceipt {
    identity: ScalarProofContentIdentity,
    digest: DigestRecord,
    entries: Vec<(String, Vec<u8>)>,
    cleanup_diagnostic: Option<String>,
    harness_identity: crate::kani::identity::ScalarObligationIdentity,
    operands: Vec<i64>,
    scalar_limits: ScalarLimits,
    proof: KaniExecutionEvidence,
    native: Outcome<Integer>,
    meter: NativeMeterObservation,
}

/// Meter counters observed after the actual native oracle call.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NativeMeterObservation {
    limits: ScalarLimits,
    consumed: [u64; 10],
    admitted_charges: Vec<ChargePoint>,
}

impl NativeMeterObservation {
    /// Actual configured limits decoded from the native meter.
    #[must_use]
    pub const fn limits(&self) -> &ScalarLimits {
        &self.limits
    }

    /// Per-counter amounts consumed in `ScalarLimitsV1` field order.
    #[must_use]
    pub const fn consumed(&self) -> &[u64; 10] {
        &self.consumed
    }

    /// Named charges admitted by the runtime meter, in occurrence order.
    #[must_use]
    pub fn admitted_charges(&self) -> &[ChargePoint] {
        &self.admitted_charges
    }
}

impl KaniScalarNativeReceipt {
    /// QSL observation digest minted from the measured proof-content identity.
    #[must_use]
    pub const fn proof_content_digest(&self) -> &DigestRecord {
        &self.digest
    }

    /// Exact owned staged inputs in ascending UTF-8 path order.
    #[must_use]
    pub fn staged_entries(&self) -> &[(String, Vec<u8>)] {
        &self.entries
    }

    /// Snapshot removal failure, if any; excluded from proof identity.
    #[must_use]
    pub fn cleanup_diagnostic(&self) -> Option<&str> {
        self.cleanup_diagnostic.as_deref()
    }

    /// Measured identity over exact staged package and harness bytes.
    #[must_use]
    pub const fn proof_content_identity(&self) -> &ScalarProofContentIdentity {
        &self.identity
    }

    /// Scalar claim whose generated harness ran.
    #[must_use]
    pub const fn harness_identity(&self) -> &crate::kani::identity::ScalarObligationIdentity {
        &self.harness_identity
    }

    /// Concrete ordered operands passed to the staged oracle.
    #[must_use]
    pub fn operands(&self) -> &[i64] {
        &self.operands
    }

    /// Exact limits installed in the native meter.
    #[must_use]
    pub const fn scalar_limits(&self) -> &ScalarLimits {
        &self.scalar_limits
    }

    /// Backend evidence for the same staged harness source.
    #[must_use]
    pub const fn proof_evidence(&self) -> &KaniExecutionEvidence {
        &self.proof
    }

    /// Actual typed runtime result returned by the staged native oracle invocation.
    pub const fn native_outcome(&self) -> &Outcome<Integer> {
        &self.native
    }

    /// Runtime meter observation from the same native invocation.
    #[must_use]
    pub const fn meter_observation(&self) -> &NativeMeterObservation {
        &self.meter
    }
}

/// Why the private scalar proof/native operation produced no receipt.
///
/// The frozen code catalog is available at the crate root:
/// ```
/// use quire_contract_codegen::{
///     SCALAR_NATIVE_CODE_INVALID_REQUEST,
///     SCALAR_NATIVE_CODE_ARTIFACT_PACKAGE,
///     SCALAR_NATIVE_CODE_SNAPSHOT_IO,
///     SCALAR_NATIVE_CODE_KANI_EXECUTION,
///     SCALAR_NATIVE_CODE_PROOF_NOT_VERIFIED,
///     SCALAR_NATIVE_CODE_ORACLE_STOP,
///     SCALAR_NATIVE_CODE_NATIVE_LAUNCH,
///     SCALAR_NATIVE_CODE_DEPENDENCY_LOCK,
///     SCALAR_NATIVE_CODE_NATIVE_RESULT,
///     SCALAR_NATIVE_CODE_SNAPSHOT_CHANGED,
///     SCALAR_NATIVE_CODE_IDENTITY_ENCODE,
///     SCALAR_NATIVE_CODE_CODEGEN_LOCK,
/// };
/// let codes = [
///     SCALAR_NATIVE_CODE_INVALID_REQUEST,
///     SCALAR_NATIVE_CODE_ARTIFACT_PACKAGE,
///     SCALAR_NATIVE_CODE_SNAPSHOT_IO,
///     SCALAR_NATIVE_CODE_KANI_EXECUTION,
///     SCALAR_NATIVE_CODE_PROOF_NOT_VERIFIED,
///     SCALAR_NATIVE_CODE_ORACLE_STOP,
///     SCALAR_NATIVE_CODE_NATIVE_LAUNCH,
///     SCALAR_NATIVE_CODE_DEPENDENCY_LOCK,
///     SCALAR_NATIVE_CODE_NATIVE_RESULT,
///     SCALAR_NATIVE_CODE_SNAPSHOT_CHANGED,
///     SCALAR_NATIVE_CODE_IDENTITY_ENCODE,
///     SCALAR_NATIVE_CODE_CODEGEN_LOCK,
/// ];
/// assert_eq!(codes.into_iter().collect::<std::collections::BTreeSet<_>>().len(), 12);
/// ```
///
/// Downstream consumers must leave room for future error variants:
/// ```compile_fail,E0004
/// use quire_contract_codegen::KaniScalarNativeError as E;
/// fn exhaustive(error: E) {
///     match error {
///         E::InvalidRequest(_) | E::ArtifactPackage(_) | E::Snapshot(_) | E::Kani(_)
///         | E::ProofNotVerified(_) | E::OracleStop(_) | E::NativeLaunch(_)
///         | E::DependencyLock { .. } | E::NativeResult(_) | E::SnapshotChanged
///         | E::IdentityEncode(_) | E::CodegenLock => (),
///     }
/// }
/// ```
/// ```
/// use quire_contract_codegen::{KaniScalarNativeError as E, KaniScalarNativeErrorClass as C};
/// fn consumer(error: E) -> &'static str {
///     let _ = match &error { E::InvalidRequest(_) => true, _ => false };
///     match error.class() { C::CompletionRefusal => error.code(), C::EnvironmentFault => error.code() }
/// }
/// ```
#[derive(Debug)]
#[non_exhaustive]
pub enum KaniScalarNativeError {
    /// The selected artifacts or call do not match the retained scalar identity.
    InvalidRequest(&'static str),
    /// The retained generated package failed the standard artifact bounds or path contract.
    ArtifactPackage(PublicationDiagnostic),
    /// Creating, writing or measuring the private snapshot failed.
    Snapshot(std::io::Error),
    /// Existing Kani execution refused or could not classify the staged harness.
    Kani(crate::kani::run::execute::KaniExecutionRefusal),
    /// Kani completed without a verified result.
    ProofNotVerified(Box<KaniExecutionEvidence>),
    /// The staged oracle stopped before producing a runtime outcome.
    OracleStop(ScalarOracleStopKind),
    /// The bounded native build or invocation did not complete successfully.
    NativeLaunch(String),
    /// The staged oracle package could not be pinned to Contract Codegen's runtime lock revision.
    DependencyLock {
        /// Runtime revision required by this codegen lock.
        expected: String,
        /// Runtime revision written into the staged package, when present.
        actual: Option<String>,
    },
    /// The native helper returned malformed or unsupported structured output.
    NativeResult(String),
    /// Staged bytes changed during either child execution.
    SnapshotChanged,
    /// Canonical proof-content encoding failed within its bound.
    IdentityEncode(DigestError),
    /// Codegen's embedded lock has no runtime revision.
    CodegenLock,
}

/// Stable scalar-native `invalid_request` error code.
pub const SCALAR_NATIVE_CODE_INVALID_REQUEST: &str = "invalid_request";
/// Stable scalar-native `artifact_package` error code.
pub const SCALAR_NATIVE_CODE_ARTIFACT_PACKAGE: &str = "artifact_package";
/// Stable scalar-native `snapshot_io` error code.
pub const SCALAR_NATIVE_CODE_SNAPSHOT_IO: &str = "snapshot_io";
/// Stable scalar-native `kani_execution` error code.
pub const SCALAR_NATIVE_CODE_KANI_EXECUTION: &str = "kani_execution";
/// Stable scalar-native `proof_not_verified` error code.
pub const SCALAR_NATIVE_CODE_PROOF_NOT_VERIFIED: &str = "proof_not_verified";
/// Stable scalar-native `oracle_stop` error code.
pub const SCALAR_NATIVE_CODE_ORACLE_STOP: &str = "oracle_stop";
/// Stable scalar-native `native_launch` error code.
pub const SCALAR_NATIVE_CODE_NATIVE_LAUNCH: &str = "native_launch";
/// Stable scalar-native `dependency_lock` error code.
pub const SCALAR_NATIVE_CODE_DEPENDENCY_LOCK: &str = "dependency_lock";
/// Stable scalar-native `native_result` error code.
pub const SCALAR_NATIVE_CODE_NATIVE_RESULT: &str = "native_result";
/// Stable scalar-native `snapshot_changed` error code.
pub const SCALAR_NATIVE_CODE_SNAPSHOT_CHANGED: &str = "snapshot_changed";
/// Stable scalar-native `identity_encode` error code.
pub const SCALAR_NATIVE_CODE_IDENTITY_ENCODE: &str = "identity_encode";
/// Stable scalar-native `codegen_lock` error code.
pub const SCALAR_NATIVE_CODE_CODEGEN_LOCK: &str = "codegen_lock";

/// Closed consumer classification of scalar-native completion failures.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KaniScalarNativeErrorClass {
    /// The request, generated content or proof deterministically refused completion.
    CompletionRefusal,
    /// The environment, integrity boundary or structured decoder failed.
    EnvironmentFault,
}

impl KaniScalarNativeError {
    /// Stable machine discriminant, independent of human diagnostics.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::InvalidRequest(_) => SCALAR_NATIVE_CODE_INVALID_REQUEST,
            Self::ArtifactPackage(_) => SCALAR_NATIVE_CODE_ARTIFACT_PACKAGE,
            Self::Snapshot(_) => SCALAR_NATIVE_CODE_SNAPSHOT_IO,
            Self::Kani(_) => SCALAR_NATIVE_CODE_KANI_EXECUTION,
            Self::ProofNotVerified(_) => SCALAR_NATIVE_CODE_PROOF_NOT_VERIFIED,
            Self::OracleStop(_) => SCALAR_NATIVE_CODE_ORACLE_STOP,
            Self::NativeLaunch(_) => SCALAR_NATIVE_CODE_NATIVE_LAUNCH,
            Self::DependencyLock { .. } => SCALAR_NATIVE_CODE_DEPENDENCY_LOCK,
            Self::NativeResult(_) => SCALAR_NATIVE_CODE_NATIVE_RESULT,
            Self::SnapshotChanged => SCALAR_NATIVE_CODE_SNAPSHOT_CHANGED,
            Self::IdentityEncode(_) => SCALAR_NATIVE_CODE_IDENTITY_ENCODE,
            Self::CodegenLock => SCALAR_NATIVE_CODE_CODEGEN_LOCK,
        }
    }

    /// Classify the carried outcome or nested refusal without changing evidence.
    #[must_use]
    pub fn class(&self) -> KaniScalarNativeErrorClass {
        use KaniScalarNativeErrorClass::{CompletionRefusal, EnvironmentFault};
        match self {
            Self::InvalidRequest(_) | Self::ArtifactPackage(_) | Self::OracleStop(_) => {
                CompletionRefusal
            }
            Self::Snapshot(_)
            | Self::NativeLaunch(_)
            | Self::DependencyLock { .. }
            | Self::NativeResult(_)
            | Self::SnapshotChanged
            | Self::IdentityEncode(_)
            | Self::CodegenLock => EnvironmentFault,
            Self::Kani(refusal) => match refusal {
                KaniExecutionRefusal::HarnessNotInCrate { .. }
                | KaniExecutionRefusal::HarnessProvenanceMismatch { .. }
                | KaniExecutionRefusal::OutputOverLimit { .. }
                | KaniExecutionRefusal::PlaybackForNonMember { .. } => CompletionRefusal,
                KaniExecutionRefusal::Tool(_)
                | KaniExecutionRefusal::MemoryMechanismUnavailable { .. }
                | KaniExecutionRefusal::MemoryObservationFailed { .. }
                | KaniExecutionRefusal::BatchMemoryExhausted { .. }
                | KaniExecutionRefusal::BatchTimedOut { .. }
                | KaniExecutionRefusal::BatchCancelled { .. }
                | KaniExecutionRefusal::OutputUnread { .. }
                | KaniExecutionRefusal::Report(_) => EnvironmentFault,
            },
            Self::ProofNotVerified(evidence) => {
                proof_failure_class(&evidence.outcome, evidence.failure_cause.as_ref())
            }
        }
    }
}

fn proof_failure_class(
    outcome: &KaniRunOutcome,
    cause: Option<&KaniFailureCause>,
) -> KaniScalarNativeErrorClass {
    use KaniScalarNativeErrorClass::{CompletionRefusal, EnvironmentFault};
    match outcome {
        KaniRunOutcome::Verified | KaniRunOutcome::Cancelled { .. } => EnvironmentFault,
        KaniRunOutcome::Falsified { .. } | KaniRunOutcome::CoverUnsatisfied { .. } => {
            CompletionRefusal
        }
        KaniRunOutcome::Inconclusive { reason } => match reason {
            KaniInconclusiveReason::MissingCoverSummary
            | KaniInconclusiveReason::VacuousProof
            | KaniInconclusiveReason::UnwindBoundExhausted => CompletionRefusal,
            KaniInconclusiveReason::TimedOut
            | KaniInconclusiveReason::MemoryExhausted
            | KaniInconclusiveReason::NoVerdict => EnvironmentFault,
            KaniInconclusiveReason::FailedWithoutCounterexample => match cause {
                None | Some(KaniFailureCause::PropertiesFailed) => CompletionRefusal,
                Some(
                    KaniFailureCause::Timeout
                    | KaniFailureCause::OutOfMemory
                    | KaniFailureCause::ExitCode { .. }
                    | KaniFailureCause::Unknown { .. },
                ) => EnvironmentFault,
            },
        },
    }
}

/// Closed set of stop tags emitted by the generated scalar oracle adapter.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScalarOracleStopKind {
    /// Runtime rejected a generated operand or type.
    IllTyped,
    /// Runtime rejected a generated constant.
    InvalidConstant,
    /// An IEEE operand width did not match.
    OperandWidth,
    /// The native kernel rejected plan or operand admission.
    NativeAdmission,
}

impl fmt::Display for KaniScalarNativeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for KaniScalarNativeError {}

/// Prove a private staged scalar harness, then invoke its embedded public oracle natively.
pub fn execute_kani_scalar_native(
    request: &KaniScalarNativeRequest<'_>,
) -> Result<KaniScalarNativeReceipt, KaniScalarNativeError> {
    execute_with_cleanup(request, |path| fs::remove_dir_all(path))
}

// A narrow filesystem dependency keeps removal failures testable after genuine execution.
pub(crate) fn execute_with_cleanup(
    request: &KaniScalarNativeRequest<'_>,
    remove: impl FnOnce(&Path) -> std::io::Result<()>,
) -> Result<KaniScalarNativeReceipt, KaniScalarNativeError> {
    validate_request(request)?;
    let snapshot = PrivateSnapshot::create()?;
    let package = stage_package(request, snapshot.path())?;
    write_observer(
        &request.harness.identity.oracle_symbol,
        request.harness.identity.arguments.len(),
        request.harness.identity.integer_representation,
        snapshot.path(),
    )?;
    generate_lockfile(request, snapshot.path())?;
    generate_lockfile(request, &snapshot.path().join("observer"))?;
    let entries = capture_entries(&package, snapshot.path())?;
    seal_snapshot(snapshot.path())?;
    ensure_snapshot_matches(snapshot.path(), &entries)?;
    let identity = identity_from_entries(&entries, Limits::new(256 * 1024 * 1024))?;
    let proof_target = request.target_directory.join("kani-scalar-proof");
    let evidence = execute_kani_obligation_readonly(
        &KaniExecutionRequest {
            cancel: request.cancel,
            installation: request.installation,
            harness: KaniExecutableHarness::Scalar(request.harness),
            crate_directory: snapshot.path(),
            target_directory: &proof_target,
            home_directory: None,
        },
        snapshot.path(),
    )
    .map_err(KaniScalarNativeError::Kani)?;
    ensure_snapshot_matches(snapshot.path(), &entries)?;
    if evidence.outcome != KaniRunOutcome::Verified {
        return Err(KaniScalarNativeError::ProofNotVerified(Box::new(evidence)));
    }
    ensure_snapshot_matches(snapshot.path(), &entries)?;

    let (native, meter) = execute_native(request, snapshot.path())?;
    ensure_snapshot_matches(snapshot.path(), &entries)?;

    let cleanup_diagnostic = snapshot
        .cleanup_with(remove)
        .err()
        .map(|error| format!("private snapshot removal failed: {error}"));
    Ok(KaniScalarNativeReceipt {
        digest: DigestRecord::mint(DigestDomain::Sha256Jcs, *identity.as_bytes()),
        entries,
        cleanup_diagnostic,
        identity,
        harness_identity: request.harness.identity.clone(),
        operands: request.operands.to_vec(),
        scalar_limits: request.scalar_limits,
        proof: evidence,
        native,
        meter,
    })
}

fn validate_request(request: &KaniScalarNativeRequest<'_>) -> Result<(), KaniScalarNativeError> {
    if !request.harness.has_valid_provenance() {
        return Err(KaniScalarNativeError::InvalidRequest(
            "retained scalar harness changed after generation",
        ));
    }
    if !request
        .harness
        .matches_artifact_package(request.oracle_artifacts)
    {
        return Err(KaniScalarNativeError::InvalidRequest(
            "retained scalar package changed after routed generation",
        ));
    }
    ArtifactBundle::new(request.oracle_artifacts.to_vec())
        .map_err(KaniScalarNativeError::ArtifactPackage)?;
    let identity = &request.harness.identity;
    if identity.operation_identity == crate::kani::identity::BOOLEAN_IMPLIES_IDENTITY {
        return Err(KaniScalarNativeError::InvalidRequest(
            "Boolean scalar proofs have no integer native observer",
        ));
    }
    let expected_rust_path = format!("src/generated/{}.rs", identity.module_symbol);
    let expected_record_path = format!("kani-obligations/{}.json", identity.module_symbol);
    let record: serde_json::Value = serde_json::from_str(&request.harness.record.contents)
        .map_err(|_| {
            KaniScalarNativeError::InvalidRequest("retained scalar record is invalid JSON")
        })?;
    if request.harness.rust.path != expected_rust_path
        || request.harness.record.path != expected_record_path
        || record.get("rustPath").and_then(serde_json::Value::as_str)
            != Some(&request.harness.rust.path)
        || record.get("identity")
            != Some(&serde_json::to_value(identity).map_err(|_| {
                KaniScalarNativeError::InvalidRequest(
                    "retained scalar identity cannot be serialized",
                )
            })?)
    {
        return Err(KaniScalarNativeError::InvalidRequest(
            "retained scalar source record does not match its identity",
        ));
    }
    if identity.arguments.len() != request.operands.len() {
        return Err(KaniScalarNativeError::InvalidRequest(
            "operand count differs from harness identity",
        ));
    }
    if identity.oracle_symbol.is_empty() || identity.oracle_symbol.contains("::") {
        return Err(KaniScalarNativeError::InvalidRequest(
            "oracle symbol is not a local Rust function",
        ));
    }
    for (argument, value) in identity.arguments.iter().zip(request.operands) {
        if *value < argument.minimum || *value > argument.maximum {
            return Err(KaniScalarNativeError::InvalidRequest(
                "operand lies outside its retained bounds",
            ));
        }
    }
    let parsed = syn::parse_file(&request.harness.rust.contents)
        .map_err(|_| KaniScalarNativeError::InvalidRequest("harness source is invalid Rust"))?;
    let emitted = request
        .oracle_artifacts
        .iter()
        .find(|artifact| artifact.path == "src/lib.rs")
        .ok_or(KaniScalarNativeError::InvalidRequest(
            "emitted oracle library is absent",
        ))?;
    let emitted_parsed = syn::parse_file(&emitted.contents).map_err(|_| {
        KaniScalarNativeError::InvalidRequest("emitted oracle library is invalid Rust")
    })?;
    fn find_oracle<'a>(file: &'a syn::File, symbol: &str) -> Option<&'a syn::ItemFn> {
        file.items.iter().find_map(|item| match item {
            syn::Item::Fn(function) if function.sig.ident == symbol => Some(function),
            _ => None,
        })
    }
    let Some(harness_oracle) = find_oracle(&parsed, &identity.oracle_symbol) else {
        return Err(KaniScalarNativeError::InvalidRequest(
            "harness source does not define the selected oracle",
        ));
    };
    let Some(emitted_oracle) = find_oracle(&emitted_parsed, &identity.oracle_symbol) else {
        return Err(KaniScalarNativeError::InvalidRequest(
            "emitted package does not define the selected oracle",
        ));
    };
    use quote::ToTokens;
    if !matches!(harness_oracle.vis, syn::Visibility::Public(_))
        || harness_oracle.to_token_stream().to_string()
            != emitted_oracle.to_token_stream().to_string()
    {
        return Err(KaniScalarNativeError::InvalidRequest(
            "selected oracle differs between harness and emitted package",
        ));
    }
    if harness_oracle.sig.inputs.len() != identity.arguments.len() + 1 {
        return Err(KaniScalarNativeError::InvalidRequest(
            "selected oracle arity differs from retained ordered operands",
        ));
    }
    let parameters = harness_oracle
        .sig
        .inputs
        .iter()
        .filter_map(|argument| match argument {
            syn::FnArg::Typed(argument) => match argument.pat.as_ref() {
                syn::Pat::Ident(pattern) => Some(pattern.ident.to_string()),
                _ => None,
            },
            syn::FnArg::Receiver(_) => None,
        })
        .collect::<Vec<_>>();
    if parameters.len() != identity.arguments.len() + 1
        || parameters[..identity.arguments.len()]
            .iter()
            .zip(&identity.arguments)
            .any(|(parameter, argument)| parameter != &argument.identifier)
        || parameters
            .last()
            .is_none_or(|parameter| parameter != "meter")
    {
        return Err(KaniScalarNativeError::InvalidRequest(
            "selected oracle argument order differs from retained scalar identity",
        ));
    }
    if request.oracle_artifacts.len() != 4
        || [
            "Cargo.toml",
            "src/lib.rs",
            "source-maps/lib.json",
            "claim-map.json",
        ]
        .iter()
        .any(|path| {
            !request
                .oracle_artifacts
                .iter()
                .any(|item| item.path == *path)
        })
    {
        return Err(KaniScalarNativeError::InvalidRequest(
            "emitted scalar package must contain exactly its generated manifest, library, coverage map and claim map",
        ));
    }
    let Some(manifest) = request
        .oracle_artifacts
        .iter()
        .find(|artifact| artifact.path == "Cargo.toml")
    else {
        return Err(KaniScalarNativeError::InvalidRequest(
            "emitted scalar package is missing Cargo.toml",
        ));
    };
    if manifest.contents != oracle_crate_manifest(EXACT_SCALAR_CRATE_NAME) {
        return Err(KaniScalarNativeError::InvalidRequest(
            "emitted scalar manifest differs from the canonical generated manifest",
        ));
    }
    Ok(())
}

fn stage_package(
    request: &KaniScalarNativeRequest<'_>,
    root: &Path,
) -> Result<Vec<(String, Vec<u8>)>, KaniScalarNativeError> {
    let mut artifacts = request.oracle_artifacts.to_vec();
    artifacts.sort_by(|a, b| a.path.cmp(&b.path));
    let mut unique = artifacts
        .iter()
        .map(|artifact| artifact.path.clone())
        .collect::<std::collections::BTreeSet<_>>();
    if artifacts
        .iter()
        .any(|artifact| !unique.contains(&artifact.path))
        || unique.len() != artifacts.len()
        || unique.contains("Cargo.lock")
        || unique.contains("src/oracle-artifacts-lib.rs")
        || unique.contains("src/bin/ir791_native.rs")
        || artifacts
            .iter()
            .any(|artifact| artifact.path.starts_with("observer/"))
    {
        return Err(KaniScalarNativeError::InvalidRequest(
            "emitted package has duplicate or runner-reserved artifact paths",
        ));
    }
    let mut staged = Vec::with_capacity(artifacts.len() + 1);
    for artifact in artifacts {
        let path = Path::new(&artifact.path);
        if path.is_absolute()
            || path
                .components()
                .any(|part| !matches!(part, std::path::Component::Normal(_)))
        {
            return Err(KaniScalarNativeError::InvalidRequest(
                "artifact path is not a safe relative path",
            ));
        }
        if artifact.path == "src/lib.rs" {
            let retained = root.join("src/oracle-artifacts-lib.rs");
            write_private(&retained, artifact.contents.as_bytes())?;
            staged.push((
                "src/oracle-artifacts-lib.rs".to_owned(),
                artifact.contents.into_bytes(),
            ));
            write_private(
                &root.join("src/lib.rs"),
                request.harness.rust.contents.as_bytes(),
            )?;
            staged.push((
                "src/lib.rs".to_owned(),
                request.harness.rust.contents.as_bytes().to_vec(),
            ));
        } else {
            let full = root.join(path);
            write_private(&full, artifact.contents.as_bytes())?;
            staged.push((artifact.path, artifact.contents.into_bytes()));
        }
    }
    if !unique.insert(request.harness.record.path.clone()) {
        return Err(KaniScalarNativeError::InvalidRequest(
            "harness record collides with emitted package path",
        ));
    }
    let record_path = root.join(&request.harness.record.path);
    write_private(&record_path, request.harness.record.contents.as_bytes())?;
    staged.push((
        request.harness.record.path.clone(),
        request.harness.record.contents.as_bytes().to_vec(),
    ));
    Ok(staged)
}

fn capture_entries(
    staged: &[(String, Vec<u8>)],
    root: &Path,
) -> Result<Vec<(String, Vec<u8>)>, KaniScalarNativeError> {
    let mut entries = staged.to_vec();
    for path in [
        "Cargo.lock",
        "observer/Cargo.lock",
        "observer/Cargo.toml",
        "src/bin/ir791_native.rs",
    ] {
        let bytes = fs::read(root.join(path)).map_err(KaniScalarNativeError::Snapshot)?;
        std::str::from_utf8(&bytes).map_err(|error| {
            KaniScalarNativeError::Snapshot(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                error,
            ))
        })?;
        entries.push((path.to_owned(), bytes));
    }
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    if entries.windows(2).any(|pair| pair[0].0 == pair[1].0) {
        return Err(KaniScalarNativeError::SnapshotChanged);
    }
    ensure_snapshot_matches(root, &entries)?;
    Ok(entries)
}

fn identity_from_entries(
    entries: &[(String, Vec<u8>)],
    limits: Limits,
) -> Result<ScalarProofContentIdentity, KaniScalarNativeError> {
    let artifacts = entries
        .iter()
        .map(|(path, bytes)| {
            let contents = String::from_utf8(bytes.clone()).map_err(|error| {
                KaniScalarNativeError::Snapshot(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    error,
                ))
            })?;
            Ok(CanonicalArtifact {
                path: path.clone(),
                contents,
            })
        })
        .collect::<Result<Vec<_>, KaniScalarNativeError>>()?;
    canonical_identity_with_limits(artifacts, limits)
}

#[cfg(test)]
fn canonical_identity(
    artifacts: Vec<CanonicalArtifact>,
) -> Result<ScalarProofContentIdentity, KaniScalarNativeError> {
    canonical_identity_with_limits(artifacts, Limits::new(256 * 1024 * 1024))
}

fn canonical_identity_with_limits(
    artifacts: Vec<CanonicalArtifact>,
    limits: Limits,
) -> Result<ScalarProofContentIdentity, KaniScalarNativeError> {
    let preimage = ScalarProofPreimage {
        schema: SCALAR_PROOF_SCHEMA.to_owned(),
        artifacts,
    };
    let digest = content_digest_with_limits(&preimage, limits)
        .map_err(KaniScalarNativeError::IdentityEncode)?;
    Ok(ScalarProofContentIdentity(digest))
}

fn limits_array(limits: ScalarLimits) -> [u64; 10] {
    [
        limits.integer_bits,
        limits.decimal_digits,
        limits.scale_expansion,
        limits.text_input_bytes,
        limits.text_scalars,
        limits.normalized_scalars,
        limits.unit_edges,
        limits.value_occurrences,
        limits.work_units,
        limits.result_units,
    ]
}

fn write_observer(
    oracle_symbol: &str,
    argument_count: usize,
    representation: ScalarIntegerRepresentation,
    root: &Path,
) -> Result<(), KaniScalarNativeError> {
    let operand_bindings = (0..argument_count)
        .map(|index| format!(" let operand_{index} = next_i64(&mut arguments)?;"))
        .collect::<String>();
    let parameters = (0..argument_count)
        .map(|index| match representation {
            ScalarIntegerRepresentation::General => format!("&rt::Integer::from(operand_{index})"),
            ScalarIntegerRepresentation::Native => {
                format!("&rt::NativeInteger::new(i128::from(operand_{index}))")
            }
        })
        .collect::<Vec<_>>()
        .join(", ");
    let completed_value = match representation {
        ScalarIntegerRepresentation::General => "value",
        ScalarIntegerRepresentation::Native => "value.value()",
    };
    let stop_arms = match representation {
        ScalarIntegerRepresentation::General => "   Err(generated::OracleStop::IllTyped(_)) => println!(\"IR791_NATIVE_V1\\toracle_stop\\till_typed\"),\n   Err(generated::OracleStop::InvalidConstant) => println!(\"IR791_NATIVE_V1\\toracle_stop\\tinvalid_constant\"),\n   Err(generated::OracleStop::OperandWidth) => println!(\"IR791_NATIVE_V1\\toracle_stop\\toperand_width\"),",
        ScalarIntegerRepresentation::Native => "   Err(generated::OracleStop::NativeAdmission(reason)) => { let _ = reason; println!(\"IR791_NATIVE_V1\\toracle_stop\\tnative_admission\"); },",
    };
    let limits = [
        "integer_bits",
        "decimal_digits",
        "scale_expansion",
        "text_input_bytes",
        "text_scalars",
        "normalized_scalars",
        "unit_edges",
        "value_occurrences",
        "work_units",
        "result_units",
    ]
    .into_iter()
    .map(|field| format!(" {field}: next_u64(&mut arguments)?,"))
    .collect::<String>();
    let helper = format!(
        r#"
#[path = "../lib.rs"] mod generated;
use quire_exact as rt;
fn next_argument(arguments: &mut impl Iterator<Item = String>) -> Result<String, std::io::Error> {{
 arguments.next().ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "missing native request argument"))
}}
fn next_i64(arguments: &mut impl Iterator<Item = String>) -> Result<i64, Box<dyn std::error::Error>> {{
 Ok(next_argument(arguments)?.parse()?)
}}
fn next_u64(arguments: &mut impl Iterator<Item = String>) -> Result<u64, Box<dyn std::error::Error>> {{
 Ok(next_argument(arguments)?.parse()?)
}}
fn main() -> Result<(), Box<dyn std::error::Error>> {{
 let mut arguments = std::env::args().skip(1);
{operand_bindings}
 let limits = rt::ScalarLimits {{
{limits}
 }};
 if arguments.next().is_some() {{ return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "extra native request argument").into()); }}
 let mut meter = rt::Meter::new(limits);
 let result = generated::{}({}, &mut meter);
 match result {{
   Ok(rt::Outcome::Completed(value)) => println!("IR791_NATIVE_V1\tcompleted\t{{}}", {completed_value}),
   Ok(rt::Outcome::Undefined(rt::Undefined::DivisionByZero)) => println!("IR791_NATIVE_V1\tundefined\tdivision_by_zero"),
   Ok(rt::Outcome::Undefined(rt::Undefined::IeeeNotFinite)) => println!("IR791_NATIVE_V1\tundefined\tieee_not_finite"),
   Ok(rt::Outcome::Undefined(rt::Undefined::EmptyReduction)) => println!("IR791_NATIVE_V1\tundefined\tempty_reduction"),
   Ok(rt::Outcome::Undefined(rt::Undefined::NoneValue)) => println!("IR791_NATIVE_V1\tundefined\tnone_value"),
   Ok(rt::Outcome::Refused(rt::Refusal::IntegerOutOfDomain {{ target }})) => println!("IR791_NATIVE_V1\trefused\tinteger_out_of_domain\t{{}}\t{{}}", target.lower(), target.upper()),
   Ok(rt::Outcome::Incomplete(value)) => println!("IR791_NATIVE_V1\tincomplete\t{{}}\t{{}}\t{{}}\t{{}}\t{{}}", value.limit_kind.as_str(), value.limit, value.consumed, value.next_charge, value.charge_point.as_str()),
   Ok(_) => println!("IR791_NATIVE_V1\tunsupported"),
{stop_arms}
 }}
 let used = meter.limits();
 println!("IR791_LIMITS_V1\t{{}}\t{{}}\t{{}}\t{{}}\t{{}}\t{{}}\t{{}}\t{{}}\t{{}}\t{{}}", used.integer_bits, used.decimal_digits, used.scale_expansion, used.text_input_bytes, used.text_scalars, used.normalized_scalars, used.unit_edges, used.value_occurrences, used.work_units, used.result_units);
 print!("IR791_METER_V1");
 for kind in rt::LimitKind::ALL {{ print!("\t{{}}", meter.consumed(kind)); }}
 for charge in meter.admitted_charges() {{ print!("\t{{}}", charge.point.as_str()); }}
 println!();
 Ok(())
}}
"#,
        oracle_symbol, parameters,
    );
    write_private(&root.join("src/bin/ir791_native.rs"), helper.as_bytes())?;
    write_private(
        &root.join("observer/Cargo.toml"),
        scalar_observer_manifest().as_bytes(),
    )?;
    Ok(())
}

fn execute_native(
    request: &KaniScalarNativeRequest<'_>,
    root: &Path,
) -> Result<(Outcome<Integer>, NativeMeterObservation), KaniScalarNativeError> {
    let mut command = BackendCommand::new(request.installation.scalar_native_launcher());
    let arguments = native_arguments(request.operands, request.scalar_limits);
    command.args(&arguments);
    command.current_dir(root);
    command.read_only_bind(root);
    command.env(
        "CARGO_TARGET_DIR",
        request.target_directory.join("kani-scalar-native"),
    );
    let deadline = Instant::now().checked_add(request.harness.identity.ceilings.wall_clock);
    let result = run_bounded_launcher(
        command,
        request.harness.identity.ceilings,
        std::num::NonZeroUsize::MIN,
        deadline,
        request.cancel,
        request.installation.owners(),
    )
    .map_err(|error| {
        KaniScalarNativeError::NativeLaunch(match error {
            crate::kani::run::launch::BoundedLaunchError::Unavailable(cause) => {
                format!("native memory enforcement unavailable: {cause}")
            }
            crate::kani::run::launch::BoundedLaunchError::Io(cause) => {
                format!("native launcher I/O failed: {cause}")
            }
        })
    })?;
    decode_native_launch(result.outcome, request.scalar_limits)
}

fn native_arguments(operands: &[i64], limits: ScalarLimits) -> Vec<String> {
    let mut arguments = vec![
        "run".to_owned(),
        "--quiet".to_owned(),
        "--locked".to_owned(),
        "--manifest-path".to_owned(),
        "observer/Cargo.toml".to_owned(),
        "--example".to_owned(),
        "ir791_native".to_owned(),
        "--".to_owned(),
    ];
    arguments.extend(operands.iter().map(ToString::to_string));
    arguments.extend(limits_array(limits).map(|value| value.to_string()));
    arguments
}

fn decode_native_launch(
    outcome: LaunchOutcome,
    limits: ScalarLimits,
) -> Result<(Outcome<Integer>, NativeMeterObservation), KaniScalarNativeError> {
    let LaunchOutcome::Completed {
        exited_successfully: true,
        streams,
        ..
    } = outcome
    else {
        return Err(KaniScalarNativeError::NativeLaunch(format!(
            "native cargo run did not complete successfully: {outcome:?}"
        )));
    };
    decode_native_output(&streams.merged_text(), limits)
}

fn decode_native_output(
    text: &str,
    expected_limits: ScalarLimits,
) -> Result<(Outcome<Integer>, NativeMeterObservation), KaniScalarNativeError> {
    if text.lines().any(|line| {
        line.starts_with("IR791_")
            && !matches!(
                line.split('\t').next(),
                Some("IR791_NATIVE_V1" | "IR791_LIMITS_V1" | "IR791_METER_V1")
            )
    }) {
        return Err(KaniScalarNativeError::NativeResult(
            "unknown helper record".into(),
        ));
    }
    let result = single_record(text, "IR791_NATIVE_V1")?;
    let fields = result.split('\t').collect::<Vec<_>>();
    let native = match fields.as_slice() {
        ["IR791_NATIVE_V1", "completed", value] => {
            Outcome::Completed(value.parse::<Integer>().map_err(|error| {
                KaniScalarNativeError::NativeResult(format!(
                    "invalid typed Integer result: {error}"
                ))
            })?)
        }
        ["IR791_NATIVE_V1", "refused", "integer_out_of_domain", lower, upper] => {
            let parse_bound = |bound: &str| {
                bound.parse::<Integer>().map_err(|error| {
                    KaniScalarNativeError::NativeResult(format!(
                        "invalid integer domain bound: {error}"
                    ))
                })
            };
            let target =
                IntegerInterval::new(parse_bound(lower)?, parse_bound(upper)?).map_err(|_| {
                    KaniScalarNativeError::NativeResult("empty integer target interval".into())
                })?;
            Outcome::Refused(Refusal::IntegerOutOfDomain {
                target: Box::new(target),
            })
        }
        ["IR791_NATIVE_V1", "undefined", reason] => Outcome::Undefined(parse_undefined(reason)?),
        ["IR791_NATIVE_V1", "incomplete", kind, limit, consumed, next, point] => {
            Outcome::Incomplete(Incomplete {
                limit_kind: LimitKind::ALL
                    .into_iter()
                    .find(|item| item.as_str() == *kind)
                    .ok_or_else(|| {
                        KaniScalarNativeError::NativeResult("unknown incomplete limit kind".into())
                    })?,
                limit: parse_u64(limit)?,
                consumed: parse_u64(consumed)?,
                next_charge: next.parse::<Integer>().map_err(|error| {
                    KaniScalarNativeError::NativeResult(format!(
                        "invalid incomplete charge: {error}"
                    ))
                })?,
                charge_point: ChargePoint::ALL
                    .into_iter()
                    .find(|item| item.as_str() == *point)
                    .ok_or_else(|| {
                        KaniScalarNativeError::NativeResult(
                            "unknown incomplete charge point".into(),
                        )
                    })?,
            })
        }
        ["IR791_NATIVE_V1", "oracle_stop", "ill_typed"] => {
            return Err(KaniScalarNativeError::OracleStop(
                ScalarOracleStopKind::IllTyped,
            ))
        }
        ["IR791_NATIVE_V1", "oracle_stop", "invalid_constant"] => {
            return Err(KaniScalarNativeError::OracleStop(
                ScalarOracleStopKind::InvalidConstant,
            ))
        }
        ["IR791_NATIVE_V1", "oracle_stop", "operand_width"] => {
            return Err(KaniScalarNativeError::OracleStop(
                ScalarOracleStopKind::OperandWidth,
            ))
        }
        ["IR791_NATIVE_V1", "oracle_stop", "native_admission"] => {
            return Err(KaniScalarNativeError::OracleStop(
                ScalarOracleStopKind::NativeAdmission,
            ))
        }
        ["IR791_NATIVE_V1", "unsupported"] => {
            return Err(KaniScalarNativeError::NativeResult(
                "unsupported runtime result".into(),
            ))
        }
        ["IR791_NATIVE_V1", kind, _] => {
            return Err(KaniScalarNativeError::NativeResult(format!(
                "unsupported runtime result kind {kind}"
            )))
        }
        _ => {
            return Err(KaniScalarNativeError::NativeResult(
                "malformed native result record".into(),
            ))
        }
    };
    let limits_line = single_record(text, "IR791_LIMITS_V1")?;
    let values = limits_line
        .split('\t')
        .skip(1)
        .map(parse_u64)
        .collect::<Result<Vec<_>, _>>()?;
    let values: [u64; 10] = values
        .try_into()
        .map_err(|_| KaniScalarNativeError::NativeResult("meter limits have wrong arity".into()))?;
    let limits = ScalarLimits {
        integer_bits: values[0],
        decimal_digits: values[1],
        scale_expansion: values[2],
        text_input_bytes: values[3],
        text_scalars: values[4],
        normalized_scalars: values[5],
        unit_edges: values[6],
        value_occurrences: values[7],
        work_units: values[8],
        result_units: values[9],
    };
    if limits != expected_limits {
        return Err(KaniScalarNativeError::NativeResult(
            "native meter limits differ from request".into(),
        ));
    }
    let meter_line = single_record(text, "IR791_METER_V1")?;
    let meter_fields = meter_line.split('\t').skip(1).collect::<Vec<_>>();
    if meter_fields.len() < 10 {
        return Err(KaniScalarNativeError::NativeResult(
            "meter observation omits counters".into(),
        ));
    }
    let mut consumed = [0_u64; 10];
    for (slot, value) in consumed.iter_mut().zip(&meter_fields[..10]) {
        *slot = parse_u64(value)?;
    }
    let mut admitted_charges = Vec::new();
    for name in &meter_fields[10..] {
        let point = ChargePoint::ALL
            .into_iter()
            .find(|point| point.as_str() == *name)
            .ok_or_else(|| {
                KaniScalarNativeError::NativeResult("unknown admitted charge point".into())
            })?;
        admitted_charges.push(point);
    }
    Ok((
        native,
        NativeMeterObservation {
            limits,
            consumed,
            admitted_charges,
        },
    ))
}

fn single_record<'a>(text: &'a str, tag: &str) -> Result<&'a str, KaniScalarNativeError> {
    let mut lines = text
        .lines()
        .filter(|line| line.split('\t').next() == Some(tag));
    let record = lines
        .next()
        .ok_or_else(|| KaniScalarNativeError::NativeResult(format!("missing {tag} record")))?;
    if lines.next().is_some() {
        return Err(KaniScalarNativeError::NativeResult(format!(
            "multiple {tag} records"
        )));
    }
    Ok(record)
}

fn parse_u64(value: &str) -> Result<u64, KaniScalarNativeError> {
    value.parse().map_err(|error| {
        KaniScalarNativeError::NativeResult(format!("invalid meter integer: {error}"))
    })
}

fn parse_undefined(value: &str) -> Result<Undefined, KaniScalarNativeError> {
    match value {
        "division_by_zero" => Ok(Undefined::DivisionByZero),
        "ieee_not_finite" => Ok(Undefined::IeeeNotFinite),
        "empty_reduction" => Ok(Undefined::EmptyReduction),
        "none_value" => Ok(Undefined::NoneValue),
        _ => Err(KaniScalarNativeError::NativeResult(
            "unknown typed undefined reason".into(),
        )),
    }
}

fn generate_lockfile(
    request: &KaniScalarNativeRequest<'_>,
    root: &Path,
) -> Result<(), KaniScalarNativeError> {
    let mut command = BackendCommand::new("cargo");
    command.args(&["generate-lockfile".into(), "--offline".into()]);
    command.current_dir(root);
    let deadline = Instant::now().checked_add(request.harness.identity.ceilings.wall_clock);
    let result = run_bounded_launcher(
        command,
        request.harness.identity.ceilings,
        std::num::NonZeroUsize::MIN,
        deadline,
        request.cancel,
        request.installation.owners(),
    )
    .map_err(|error| {
        KaniScalarNativeError::NativeLaunch(match error {
            crate::kani::run::launch::BoundedLaunchError::Unavailable(cause) => {
                format!("lockfile memory enforcement unavailable: {cause}")
            }
            crate::kani::run::launch::BoundedLaunchError::Io(cause) => {
                format!("lockfile launcher I/O failed: {cause}")
            }
        })
    })?;
    match result.outcome {
        LaunchOutcome::Completed {
            exited_successfully: true,
            ..
        } => {}
        outcome => {
            return Err(KaniScalarNativeError::NativeLaunch(format!(
                "lockfile generation did not complete successfully: {outcome:?}"
            )))
        }
    };
    for (package, repository) in LOCKED_DEPENDENCIES {
        let expected = if package == "quire-contract-runtime" {
            codegen_runtime_revision(include_str!("../../../Cargo.lock"))?
        } else {
            lock_revision(include_str!("../../../Cargo.lock"), package, repository)
                .ok_or(KaniScalarNativeError::CodegenLock)?
        };
        let mut update = BackendCommand::new("cargo");
        update.args(&[
            "update".into(),
            "--offline".into(),
            "-p".into(),
            package.into(),
            "--precise".into(),
            expected.clone(),
        ]);
        update.current_dir(root);
        let deadline = Instant::now().checked_add(request.harness.identity.ceilings.wall_clock);
        let updated = run_bounded_launcher(
            update,
            request.harness.identity.ceilings,
            std::num::NonZeroUsize::MIN,
            deadline,
            request.cancel,
            request.installation.owners(),
        )
        .map_err(|error| {
            KaniScalarNativeError::NativeLaunch(match error {
                crate::kani::run::launch::BoundedLaunchError::Unavailable(cause) => {
                    format!("runtime lock memory enforcement unavailable: {cause}")
                }
                crate::kani::run::launch::BoundedLaunchError::Io(cause) => {
                    format!("runtime lock launcher I/O failed: {cause}")
                }
            })
        })?;
        if !matches!(
            updated.outcome,
            LaunchOutcome::Completed {
                exited_successfully: true,
                ..
            }
        ) {
            return Err(KaniScalarNativeError::NativeLaunch(format!(
                "runtime lock update failed: {:?}",
                updated.outcome
            )));
        }
        if package == "quire-contract-runtime" {
            verify_runtime_revision(&expected, fs::read_to_string(root.join("Cargo.lock")))?;
        } else {
            verify_dependency_revision(
                &expected,
                fs::read_to_string(root.join("Cargo.lock")),
                package,
                repository,
            )?;
        }
    }
    Ok(())
}

fn codegen_runtime_revision(lock: &str) -> Result<String, KaniScalarNativeError> {
    lock_revision(
        lock,
        "quire-contract-runtime",
        "https://github.com/agent-ix/quire-contract-runtime",
    )
    .ok_or(KaniScalarNativeError::CodegenLock)
}

fn verify_runtime_revision(
    expected: &str,
    lock: std::io::Result<String>,
) -> Result<(), KaniScalarNativeError> {
    verify_dependency_revision(
        expected,
        lock,
        "quire-contract-runtime",
        "https://github.com/agent-ix/quire-contract-runtime",
    )
}

fn verify_dependency_revision(
    expected: &str,
    lock: std::io::Result<String>,
    package: &str,
    repository: &str,
) -> Result<(), KaniScalarNativeError> {
    let lock = match lock {
        Err(error) if error.kind() == std::io::ErrorKind::InvalidData => {
            return Err(KaniScalarNativeError::Snapshot(error));
        }
        other => other.ok(),
    };
    let actual = lock.and_then(|lock| lock_revision(&lock, package, repository));
    if actual.as_deref() != Some(expected) {
        return Err(KaniScalarNativeError::DependencyLock {
            expected: expected.to_owned(),
            actual,
        });
    }
    Ok(())
}

fn lock_revision(lock: &str, package: &str, repository: &str) -> Option<String> {
    lock.split("[[package]]").skip(1).find_map(|entry| {
        let name = format!("name = \"{package}\"");
        if !entry.lines().any(|line| line.trim() == name) {
            return None;
        }
        entry.lines().find_map(|line| {
            let source = line.trim().strip_prefix("source = \"")?.strip_suffix('"')?;
            let prefix = format!("git+{repository}?");
            let (_, revision) = source.strip_prefix(&prefix)?.rsplit_once('#')?;
            (revision.len() == 40 && revision.bytes().all(|byte| byte.is_ascii_hexdigit()))
                .then(|| revision.to_owned())
        })
    })
}

struct PrivateSnapshot(PathBuf);

impl PrivateSnapshot {
    fn create() -> Result<Self, KaniScalarNativeError> {
        let base = std::env::temp_dir();
        let path = base.join(format!(
            "cg-kani-scalar-{}-{}",
            std::process::id(),
            NEXT_SNAPSHOT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).map_err(KaniScalarNativeError::Snapshot)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&path, fs::Permissions::from_mode(0o700))
                .map_err(KaniScalarNativeError::Snapshot)?;
        }
        fs::create_dir_all(path.join("src/bin")).map_err(KaniScalarNativeError::Snapshot)?;
        Ok(Self(path))
    }
    fn path(&self) -> &Path {
        &self.0
    }
}

impl PrivateSnapshot {
    fn cleanup(&self) -> std::io::Result<()> {
        self.cleanup_with(|path| fs::remove_dir_all(path))
    }

    fn cleanup_with(
        &self,
        remove: impl FnOnce(&Path) -> std::io::Result<()>,
    ) -> std::io::Result<()> {
        fn unseal(path: &Path) {
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries.flatten() {
                    let child = entry.path();
                    if child.is_dir() {
                        unseal(&child);
                    }
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::PermissionsExt;
                        let _ = fs::set_permissions(
                            &child,
                            fs::Permissions::from_mode(if child.is_dir() { 0o700 } else { 0o600 }),
                        );
                    }
                }
            }
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let _ = fs::set_permissions(path, fs::Permissions::from_mode(0o700));
            }
        }
        unseal(&self.0);
        remove(&self.0)
    }
}

impl Drop for PrivateSnapshot {
    fn drop(&mut self) {
        let _ = self.cleanup();
    }
}

fn write_private(path: &Path, bytes: &[u8]) -> Result<(), KaniScalarNativeError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(KaniScalarNativeError::Snapshot)?;
    }
    fs::write(path, bytes).map_err(KaniScalarNativeError::Snapshot)?;
    Ok(())
}

fn snapshot_files(
    root: &Path,
) -> Result<std::collections::BTreeSet<String>, KaniScalarNativeError> {
    fn visit(
        root: &Path,
        directory: &Path,
        output: &mut Vec<String>,
    ) -> Result<(), KaniScalarNativeError> {
        for entry in fs::read_dir(directory).map_err(KaniScalarNativeError::Snapshot)? {
            let entry = entry.map_err(KaniScalarNativeError::Snapshot)?;
            let path = entry.path();
            let file_type = entry.file_type().map_err(KaniScalarNativeError::Snapshot)?;
            if file_type.is_dir() {
                visit(root, &path, output)?;
            } else if file_type.is_file() {
                let relative = path
                    .strip_prefix(root)
                    .map_err(|_| KaniScalarNativeError::SnapshotChanged)?;
                output.push(relative.to_string_lossy().replace('\\', "/"));
            } else {
                return Err(KaniScalarNativeError::SnapshotChanged);
            }
        }
        Ok(())
    }
    let mut paths = Vec::new();
    visit(root, root, &mut paths)?;
    Ok(paths.into_iter().collect())
}

fn ensure_snapshot_matches(
    root: &Path,
    expected: &[(String, Vec<u8>)],
) -> Result<(), KaniScalarNativeError> {
    for (path, contents) in expected {
        if fs::read(root.join(path)).map_err(KaniScalarNativeError::Snapshot)? != *contents {
            return Err(KaniScalarNativeError::SnapshotChanged);
        }
    }
    let expected_paths = expected
        .iter()
        .map(|(path, _)| path.clone())
        .collect::<std::collections::BTreeSet<_>>();
    if snapshot_files(root)? != expected_paths {
        return Err(KaniScalarNativeError::SnapshotChanged);
    }
    Ok(())
}

fn seal_snapshot(root: &Path) -> Result<(), KaniScalarNativeError> {
    fn seal(path: &Path) -> Result<(), KaniScalarNativeError> {
        for entry in fs::read_dir(path).map_err(KaniScalarNativeError::Snapshot)? {
            let child = entry.map_err(KaniScalarNativeError::Snapshot)?.path();
            if child.is_dir() {
                seal(&child)?;
            } else {
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    fs::set_permissions(&child, fs::Permissions::from_mode(0o444))
                        .map_err(KaniScalarNativeError::Snapshot)?;
                }
            }
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(path, fs::Permissions::from_mode(0o555))
                .map_err(KaniScalarNativeError::Snapshot)?;
        }
        Ok(())
    }
    seal(root)
}

// This control never constructs proof evidence or a paired receipt.
#[cfg(test)]
pub(crate) fn qualify_staged_native_for_test(request: &KaniScalarNativeRequest<'_>) {
    use std::process::Command;
    validate_request(request).unwrap();
    let snapshot = PrivateSnapshot::create().unwrap();
    let root = snapshot.path();
    let staged = stage_package(request, root).unwrap();
    write_observer(
        &request.harness.identity.oracle_symbol,
        request.operands.len(),
        request.harness.identity.integer_representation,
        root,
    )
    .unwrap();
    for directory in [root.to_owned(), root.join("observer")] {
        assert!(!directory.join("Cargo.lock").exists());
        generate_lockfile(request, &directory).unwrap();
        let lock = fs::read_to_string(directory.join("Cargo.lock")).unwrap();
        for (name, repository) in LOCKED_DEPENDENCIES {
            assert_eq!(
                lock_revision(&lock, name, repository),
                lock_revision(include_str!("../../../Cargo.lock"), name, repository)
            );
        }
        eprintln!("fresh native qualification {}: {lock}", directory.display());
    }
    let entries = capture_entries(&staged, root).unwrap();
    assert_eq!(entries.len(), 10);
    let identity = identity_from_entries(&entries, Limits::new(256 * 1024 * 1024)).unwrap();
    assert_eq!(
        *identity.as_bytes(),
        crate::kani::test_support::scalar_native_digest::independent_digest(&entries)
    );
    for artifact in request.oracle_artifacts {
        let retained = if artifact.path == "src/lib.rs" {
            "src/oracle-artifacts-lib.rs"
        } else {
            &artifact.path
        };
        assert_eq!(
            fs::read(root.join(retained)).unwrap(),
            artifact.contents.as_bytes()
        );
    }
    seal_snapshot(root).unwrap();
    let run = |directory: &Path, arguments: &[String]| {
        let qualification_cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
        eprintln!(
            "native qualification cargo {arguments:?} in {}",
            directory.display()
        );
        let output = Command::new(qualification_cargo)
            .args(arguments)
            .current_dir(directory)
            .env("CARGO_TARGET_DIR", request.target_directory)
            .output()
            .unwrap();
        eprintln!(
            "status: {}\nstdout: {}\nstderr: {}",
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        output
    };
    let args = |arguments: &[&str]| {
        arguments
            .iter()
            .map(|s| (*s).to_owned())
            .collect::<Vec<_>>()
    };
    assert!(
        run(root, &args(&["build", "--offline", "--locked", "--lib"]))
            .status
            .success()
    );
    ensure_snapshot_matches(root, &entries).unwrap();
    assert!(run(
        root,
        &args(&[
            "build",
            "--offline",
            "--locked",
            "--manifest-path",
            "observer/Cargo.toml",
            "--example",
            "ir791_native"
        ])
    )
    .status
    .success());
    ensure_snapshot_matches(root, &entries).unwrap();
    let output = run(
        root,
        &native_arguments(request.operands, request.scalar_limits),
    );
    assert!(output.status.success());
    let (outcome, observation) = decode_native_output(
        std::str::from_utf8(&output.stdout).unwrap(),
        request.scalar_limits,
    )
    .unwrap();
    assert_eq!(outcome, Outcome::Completed(Integer::from(5_i64)));
    assert_eq!(observation.limits(), &request.scalar_limits);
    assert!(observation.consumed().iter().any(|value| *value > 0));
    assert_eq!(
        observation.admitted_charges(),
        &[
            ChargePoint::IntegerArithmeticOperands,
            ChargePoint::IntegerArithmeticArithmetic,
            ChargePoint::IntegerArithmeticResultRetain,
        ]
    );
    ensure_snapshot_matches(root, &entries).unwrap();
    let (bounded_outcome, bounded_observation) = execute_native(request, root).unwrap();
    assert_eq!(bounded_outcome, outcome);
    assert_eq!(bounded_observation, observation);
    ensure_snapshot_matches(root, &entries).unwrap();
    eprintln!("sealed direct and bounded native outcome {outcome:?}; limits {:?}; counters {:?}; ordered charges {:?}", observation.limits(), observation.consumed(), observation.admitted_charges());

    // An independent negative package changes only the dev feature and resolves its own locks.
    let negative = PrivateSnapshot::create().unwrap();
    stage_package(request, negative.path()).unwrap();
    write_observer(
        &request.harness.identity.oracle_symbol,
        request.operands.len(),
        request.harness.identity.integer_representation,
        negative.path(),
    )
    .unwrap();
    let manifest = scalar_observer_manifest().replace(", features = [\"test-support\"]", "");
    assert!(!manifest.contains("test-support"));
    write_private(
        &negative.path().join("observer/Cargo.toml"),
        manifest.as_bytes(),
    )
    .unwrap();
    for directory in [negative.path().to_owned(), negative.path().join("observer")] {
        assert!(!directory.join("Cargo.lock").exists());
        generate_lockfile(request, &directory).unwrap();
    }
    assert_eq!(
        fs::read(negative.path().join("src/bin/ir791_native.rs")).unwrap(),
        fs::read(root.join("src/bin/ir791_native.rs")).unwrap()
    );
    let output = run(
        negative.path(),
        &args(&[
            "build",
            "--offline",
            "--locked",
            "--manifest-path",
            "observer/Cargo.toml",
            "--example",
            "ir791_native",
        ]),
    );
    let errors = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success());
    assert!(
        errors.contains("E0599") && errors.contains("admitted_charges"),
        "{errors}"
    );
    assert!(
        !errors.contains("E0080"),
        "negative control must fail for the missing dev-only API"
    );
    ensure_snapshot_matches(root, &entries).unwrap();

    let mut proof_arguments = vec!["kani".to_owned()];
    proof_arguments.extend(request.harness.identity.options.clone());
    proof_arguments.extend(args(&["--lib", "--only-codegen"]));
    let proof = run(root, &proof_arguments);
    ensure_snapshot_matches(root, &entries).unwrap();
    assert!(
        proof.status.success(),
        "production proof-target compilation failed; no verification was requested"
    );
    eprintln!("production library and proof target compiled without observer test-support; no backend proof or paired receipt");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kani::test_support::scalar_native_digest::independent_digest;
    use ix_trace_rs::trace;
    use std::process::Command;

    #[trace("FR-015-AC-125")]
    #[test]
    fn native_observer_preserves_native_operands_result_and_admission_stop() {
        let snapshot = PrivateSnapshot::create().unwrap();
        fs::create_dir_all(snapshot.path().join("src/bin")).unwrap();
        fs::create_dir_all(snapshot.path().join("observer")).unwrap();
        write_observer(
            "checked_oracle",
            2,
            ScalarIntegerRepresentation::Native,
            snapshot.path(),
        )
        .unwrap();
        let source = fs::read_to_string(snapshot.path().join("src/bin/ir791_native.rs")).unwrap();
        assert!(source.contains("rt::NativeInteger::new(i128::from(operand_0))"));
        assert!(source.contains("rt::NativeInteger::new(i128::from(operand_1))"));
        assert!(source.contains("value.value()"));
        assert!(source.contains("generated::OracleStop::NativeAdmission(reason)"));
        assert!(source.contains("let _ = reason;"));
        assert!(!source.contains("generated::OracleStop::IllTyped(_)"));
        syn::parse_file(&source).expect("the native observer is valid Rust syntax");
        let node_id = serde_json::from_value(serde_json::json!({
            "domain": "quire.checked-semantic-node/v1",
            "digest": "2".repeat(64),
        }))
        .expect("checked source key");
        let (oracle, _) = crate::oracle::scalar::native::select(
            &node_id,
            "quire.op.integer.add",
            "checked_oracle",
            crate::oracle::operator_catalog::IntegerOperator::Add,
            &[(0, 9), (1, 1)],
            (0, 9),
        )
        .expect("original native scalar plan");
        fs::write(
            snapshot.path().join("Cargo.toml"),
            oracle_crate_manifest(EXACT_SCALAR_CRATE_NAME),
        )
        .expect("production manifest");
        fs::write(snapshot.path().join("src/lib.rs"), oracle).expect("production oracle");
        let owner_lock = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.lock");
        for directory in [snapshot.path().to_owned(), snapshot.path().join("observer")] {
            fs::copy(&owner_lock, directory.join("Cargo.lock")).expect("seeded owner lock");
            let output = Command::new("cargo")
                .args(["metadata", "--offline", "--format-version", "1"])
                .env(
                    "CARGO_TARGET_DIR",
                    Path::new(env!("CARGO_MANIFEST_DIR")).join("target"),
                )
                .current_dir(&directory)
                .output()
                .expect("resolve observer lock");
            assert!(
                output.status.success(),
                "seeded observer metadata: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
        let output = Command::new("cargo")
            .args([
                "check",
                "--offline",
                "--locked",
                "--example",
                "ir791_native",
            ])
            .env("RUSTFLAGS", "-Dwarnings")
            .env(
                "CARGO_TARGET_DIR",
                Path::new(env!("CARGO_MANIFEST_DIR")).join("target"),
            )
            .current_dir(snapshot.path().join("observer"))
            .output()
            .expect("typecheck native observer");
        assert!(
            output.status.success(),
            "native observer failed typecheck: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    #[trace("FR-014-AC-8", "FR-017-AC-27", "FR-017-AC-30", "TC-024", "TC-027")]
    #[test]
    fn tc_027_staged_native_observer_compiles_with_locked_kernel_and_charge_log() {
        let snapshot = PrivateSnapshot::create().unwrap();
        let root = snapshot.path();
        fs::create_dir_all(root.join("src")).unwrap();
        fs::write(
            root.join("Cargo.toml"),
            oracle_crate_manifest(EXACT_SCALAR_CRATE_NAME),
        )
        .unwrap();
        fs::write(
            root.join("src/lib.rs"),
            "pub enum OracleStop { IllTyped(String), InvalidConstant, OperandWidth }\n\
             pub fn oracle(value: &quire_exact::Integer, _meter: &mut quire_exact::Meter)\n\
             -> Result<quire_exact::Outcome<quire_exact::Integer>, OracleStop> {\n\
             Ok(quire_exact::Outcome::Completed(value.clone())) }\n",
        )
        .unwrap();
        write_observer("oracle", 1, ScalarIntegerRepresentation::General, root).unwrap();

        let run = |directory: &Path, args: &[&str]| {
            let observer_cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
            let output = Command::new(observer_cargo)
                .args(args)
                .current_dir(directory)
                .env(
                    "CARGO_TARGET_DIR",
                    Path::new(env!("CARGO_MANIFEST_DIR")).join("target"),
                )
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "cargo {args:?}: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            output
        };
        let cg_lock = include_str!("../../../Cargo.lock");
        for directory in [root.to_owned(), root.join("observer")] {
            assert!(
                !directory.join("Cargo.lock").exists(),
                "each package resolves independently"
            );
            run(&directory, &["generate-lockfile", "--offline"]);
            for (package, repository) in [
                (
                    "quire-contract-runtime",
                    "https://github.com/agent-ix/quire-contract-runtime",
                ),
                ("quire-exact", "https://github.com/agent-ix/quire-exact"),
            ] {
                assert!(LOCKED_DEPENDENCIES.contains(&(package, repository)));
                let expected = lock_revision(cg_lock, package, repository).unwrap();
                run(
                    &directory,
                    &["update", "--offline", "-p", package, "--precise", &expected],
                );
                let staged_lock = fs::read_to_string(directory.join("Cargo.lock")).unwrap();
                assert_eq!(
                    lock_revision(&staged_lock, package, repository),
                    Some(expected.clone())
                );
                eprintln!(
                    "independent lock {}: {package} {repository} {expected}",
                    directory.display()
                );
            }
            let lock = fs::read_to_string(directory.join("Cargo.lock")).unwrap();
            let normalization = lock
                .split("[[package]]")
                .find(|entry| entry.contains("name = \"unicode-normalization\""));
            eprintln!(
                "{} normalization selection: {normalization:?}",
                directory.display()
            );
        }
        for (manifest, observer) in [("Cargo.toml", false), ("observer/Cargo.toml", true)] {
            let metadata = run(
                root,
                &[
                    "metadata",
                    "--offline",
                    "--locked",
                    "--format-version",
                    "1",
                    "--manifest-path",
                    manifest,
                ],
            );
            let metadata: serde_json::Value = serde_json::from_slice(&metadata.stdout).unwrap();
            let package = metadata["packages"]
                .as_array()
                .unwrap()
                .iter()
                .find(|package| {
                    package["manifest_path"].as_str() == Some(root.join(manifest).to_str().unwrap())
                })
                .unwrap();
            for dependency in package["dependencies"].as_array().unwrap() {
                assert_eq!(dependency["kind"].as_str(), observer.then_some("dev"));
            }
            let exact_package = metadata["packages"]
                .as_array()
                .unwrap()
                .iter()
                .find(|package| package["name"] == "quire-exact")
                .unwrap();
            let node = metadata["resolve"]["nodes"]
                .as_array()
                .unwrap()
                .iter()
                .find(|node| node["id"] == exact_package["id"])
                .unwrap();
            assert_eq!(
                node["features"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|feature| feature == "test-support"),
                observer
            );
            if observer {
                let targets = package["targets"].as_array().unwrap();
                assert_eq!(targets.len(), 1);
                assert_eq!(targets[0]["kind"], serde_json::json!(["example"]));
                assert_eq!(
                    fs::canonicalize(targets[0]["src_path"].as_str().unwrap()).unwrap(),
                    fs::canonicalize(root.join("src/bin/ir791_native.rs")).unwrap()
                );
            }
        }
        let before = snapshot_files(root)
            .unwrap()
            .into_iter()
            .map(|path| {
                let bytes = fs::read(root.join(&path)).unwrap();
                (path, bytes)
            })
            .collect::<Vec<_>>();
        seal_snapshot(root).unwrap();
        ensure_snapshot_matches(root, &before).unwrap();
        run(
            root,
            &[
                "build",
                "--offline",
                "--locked",
                "--manifest-path",
                "observer/Cargo.toml",
                "--example",
                "ir791_native",
            ],
        );
        ensure_snapshot_matches(root, &before).unwrap();
    }

    #[trace("FR-017-AC-28")]
    #[test]
    fn tc_027_changed_staged_bytes_and_unexpected_files_refuse_identity() {
        let root = crate::core::test_support::discover_scratch("scalar-snapshot-control");
        fs::write(root.join("source.rs"), b"original").unwrap();
        let expected = vec![("source.rs".to_owned(), b"original".to_vec())];
        assert!(ensure_snapshot_matches(&root, &expected).is_ok());
        fs::write(root.join("source.rs"), b"changed").unwrap();
        assert!(matches!(
            ensure_snapshot_matches(&root, &expected),
            Err(KaniScalarNativeError::SnapshotChanged)
        ));
        fs::write(root.join("source.rs"), b"original").unwrap();
        fs::write(root.join("unmeasured.rs"), b"extra").unwrap();
        assert!(matches!(
            ensure_snapshot_matches(&root, &expected),
            Err(KaniScalarNativeError::SnapshotChanged)
        ));
        fs::remove_dir_all(root).unwrap();
    }

    #[trace("FR-017-AC-29")]
    #[test]
    fn tc_027_scalar_proof_content_identity_tracks_staged_bytes_only() {
        let make_artifact = || CanonicalArtifact {
            path: "src/lib.rs".into(),
            contents: "pub fn oracle() {}".into(),
        };
        let baseline = canonical_identity(vec![make_artifact()]).unwrap();
        let same_staged_bytes = canonical_identity(vec![make_artifact()]).unwrap();
        let changed_bytes = canonical_identity(vec![CanonicalArtifact {
            path: "src/lib.rs".into(),
            contents: "pub fn changed_oracle() {}".into(),
        }])
        .unwrap();
        assert_eq!(baseline, same_staged_bytes);
        assert_ne!(baseline, changed_bytes);
    }
    fn unequal_limits() -> ScalarLimits {
        ScalarLimits {
            integer_bits: 101,
            decimal_digits: 102,
            scale_expansion: 103,
            text_input_bytes: 104,
            text_scalars: 105,
            normalized_scalars: 106,
            unit_edges: 107,
            value_occurrences: 108,
            work_units: 109,
            result_units: 110,
        }
    }

    fn helper_output(values: &[u64]) -> String {
        format!("IR791_NATIVE_V1\tcompleted\t5\nIR791_LIMITS_V1\t{}\nIR791_METER_V1\t0\t1\t2\t3\t4\t5\t6\t7\t8\t9\tinteger-arithmetic.result-retain\n",
            values.iter().map(ToString::to_string).collect::<Vec<_>>().join("\t"))
    }

    #[trace("FR-017-AC-43", "FR-017-AC-50")]
    #[test]
    fn native_decoder_retains_actual_limits_and_rejects_each_mismatch() {
        let expected = unequal_limits();
        let values = limits_array(expected);
        let text = helper_output(&values);
        let (native, meter) = decode_native_output(&text, expected).unwrap();
        assert_eq!(native, Outcome::Completed(5_i64.into()));
        assert_eq!(*meter.limits(), expected);
        assert_eq!(meter.consumed(), &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(
            meter.admitted_charges(),
            &[ChargePoint::IntegerArithmeticResultRetain]
        );
        for index in 0..10 {
            let mut changed = values;
            changed[index] += 1;
            assert_native_result(decode_native_output(&helper_output(&changed), expected));
            let mut swapped = values;
            swapped.swap(index, (index + 1) % 10);
            assert_native_result(decode_native_output(&helper_output(&swapped), expected));
        }
        for malformed in [
            text.replace("IR791_LIMITS_V1", "UNKNOWN_LIMITS"),
            text.replace("\t101\t", "\tbad\t"),
            text.replace("\t101\t", "\t18446744073709551616\t"),
            text.replace("IR791_LIMITS_V1\t", "IR791_LIMITS_V1"),
            text.replace("\t101\t", "\t-1\t"),
            text.replace("\t110\n", "\n"),
            text.replace("\t110\n", "\t110\t111\n"),
            format!("{text}IR791_LIMITS_V1\t101\n"),
            format!("{text}IR791_METER_V1\t0\n"),
            format!("{text}IR791_NATIVE_V1\tcompleted\t6\n"),
            text.replace("completed\t5", "unsupported"),
            text.replace("completed\t5", "completed\tbogus"),
            text.replace("completed\t5", "unknown\t5"),
            text.replace("IR791_METER_V1", "IR791_METER_V2"),
            text.replace("integer-arithmetic.result-retain", "unknown_charge"),
            "".into(),
        ] {
            assert_native_result(decode_native_output(&malformed, expected));
        }
        for (name, kind) in [
            ("ill_typed", ScalarOracleStopKind::IllTyped),
            ("invalid_constant", ScalarOracleStopKind::InvalidConstant),
            ("operand_width", ScalarOracleStopKind::OperandWidth),
        ] {
            let error = decode_native_output(
                &text.replace("completed\t5", &format!("oracle_stop\t{name}")),
                expected,
            )
            .unwrap_err();
            assert!(matches!(&error, KaniScalarNativeError::OracleStop(value) if *value == kind));
            assert_eq!(error.code(), "oracle_stop");
            assert_eq!(error.class(), KaniScalarNativeErrorClass::CompletionRefusal);
        }
    }

    fn assert_native_result(
        value: Result<(Outcome<Integer>, NativeMeterObservation), KaniScalarNativeError>,
    ) {
        let error = value.unwrap_err();
        assert!(
            matches!(error, KaniScalarNativeError::NativeResult(_)),
            "{error:?}"
        );
        assert_eq!(error.code(), "native_result");
        assert_eq!(error.class(), KaniScalarNativeErrorClass::EnvironmentFault);
    }

    fn literal_entries() -> Vec<(String, Vec<u8>)> {
        [
            ("Cargo.lock", "lock\n"),
            ("Cargo.toml", "manifest"),
            ("claim-map.json", "{}"),
            ("kani-obligations/m.json", "record"),
            ("observer/Cargo.lock", "observer-lock\n"),
            ("observer/Cargo.toml", "observer-manifest"),
            ("source-maps/lib.json", "source-map"),
            ("src/bin/ir791_native.rs", "adapter"),
            ("src/lib.rs", "harness"),
            ("src/oracle-artifacts-lib.rs", "oracle"),
        ]
        .into_iter()
        .map(|(path, contents)| (path.to_owned(), contents.as_bytes().to_vec()))
        .collect()
    }

    #[trace("FR-017-AC-43", "FR-017-AC-45", "FR-017-AC-46", "FR-017-AC-49")]
    #[test]
    fn literal_ten_entry_jcs_golden_and_mutation_controls() {
        use sha2::{Digest, Sha256};
        const GOLDEN: &str = concat!(
            "{\"artifacts\":[",
            "{\"contents\":\"lock\\n\",\"path\":\"Cargo.lock\"},",
            "{\"contents\":\"manifest\",\"path\":\"Cargo.toml\"},",
            "{\"contents\":\"{}\",\"path\":\"claim-map.json\"},",
            "{\"contents\":\"record\",\"path\":\"kani-obligations/m.json\"},",
            "{\"contents\":\"observer-lock\\n\",\"path\":\"observer/Cargo.lock\"},",
            "{\"contents\":\"observer-manifest\",\"path\":\"observer/Cargo.toml\"},",
            "{\"contents\":\"source-map\",\"path\":\"source-maps/lib.json\"},",
            "{\"contents\":\"adapter\",\"path\":\"src/bin/ir791_native.rs\"},",
            "{\"contents\":\"harness\",\"path\":\"src/lib.rs\"},",
            "{\"contents\":\"oracle\",\"path\":\"src/oracle-artifacts-lib.rs\"}",
            "],\"schema\":\"agent-ix/cg-kani-scalar-proof-content/v3\"}"
        );
        let entries = literal_entries();
        let identity = identity_from_entries(&entries, Limits::new(256 * 1024 * 1024)).unwrap();
        let golden: [u8; 32] = Sha256::digest(GOLDEN.as_bytes()).into();
        assert_eq!(
            (
                SCALAR_PROOF_SCHEMA,
                golden
                    .iter()
                    .map(|byte| format!("{byte:02x}"))
                    .collect::<String>()
            ),
            (
                "agent-ix/cg-kani-scalar-proof-content/v3",
                "68a4416d5e11d9cd244a8fb9d7ffbca3890597c32d25bc915a01b271a2da79df".to_owned()
            )
        );
        assert_eq!(*identity.as_bytes(), golden);
        assert_eq!(golden, independent_digest(&entries));
        let record = DigestRecord::mint(DigestDomain::Sha256Jcs, *identity.as_bytes());
        assert_eq!(record.domain().as_str(), "sha256-jcs");
        assert_eq!(record.as_bytes(), identity.as_bytes());
        for index in 0..entries.len() {
            for mutation in 0..5 {
                let mut changed = entries.clone();
                match mutation {
                    0 => {
                        changed.remove(index);
                    }
                    1 => changed[index].0.push('x'),
                    2 => changed.swap(index, (index + 1) % entries.len()),
                    3 => changed.insert(index, changed[index].clone()),
                    4 => changed[index].1[0] ^= 1,
                    _ => unreachable!(),
                }
                let independent = independent_digest(&changed);
                assert_ne!(independent, golden, "mutation {mutation} member {index}");
                let measured =
                    identity_from_entries(&changed, Limits::new(256 * 1024 * 1024)).unwrap();
                assert_eq!(*measured.as_bytes(), independent);
                assert_ne!(measured, identity);
            }
        }
        let error = identity_from_entries(&entries, Limits::new(0)).unwrap_err();
        assert!(matches!(error, KaniScalarNativeError::IdentityEncode(_)));
        assert_eq!(error.code(), "identity_encode");
        assert_eq!(error.class(), KaniScalarNativeErrorClass::EnvironmentFault);
    }

    #[trace("FR-017-AC-46", "FR-017-AC-48")]
    #[test]
    fn owned_measurement_survives_explicit_cleanup_and_reports_invalid_utf8() {
        let snapshot = PrivateSnapshot::create().unwrap();
        let entries = literal_entries();
        for (path, bytes) in &entries {
            write_private(&snapshot.path().join(path), bytes).unwrap();
        }
        let supplied = entries
            .iter()
            .filter(|(path, _)| {
                !matches!(
                    path.as_str(),
                    "Cargo.lock"
                        | "observer/Cargo.lock"
                        | "observer/Cargo.toml"
                        | "src/bin/ir791_native.rs"
                )
            })
            .cloned()
            .collect::<Vec<_>>();
        let captured = capture_entries(&supplied, snapshot.path()).unwrap();
        assert_eq!(captured, entries);
        let digest = independent_digest(&captured);
        seal_snapshot(snapshot.path()).unwrap();
        let failure = snapshot
            .cleanup_with(|_| Err(std::io::Error::other("forced removal failure")))
            .unwrap_err();
        assert!(!failure.to_string().is_empty());
        assert!(snapshot.path().exists());
        assert_eq!(captured, entries);
        assert_eq!(independent_digest(&captured), digest);
        snapshot.cleanup().unwrap();
        assert!(
            !snapshot.path().exists(),
            "cleanup finishes before data is observed"
        );
        assert_eq!(independent_digest(&captured), digest);
        for path in [
            "Cargo.lock",
            "observer/Cargo.lock",
            "observer/Cargo.toml",
            "src/bin/ir791_native.rs",
        ] {
            let snapshot = PrivateSnapshot::create().unwrap();
            for (name, bytes) in &entries {
                write_private(&snapshot.path().join(name), bytes).unwrap();
            }
            fs::write(snapshot.path().join(path), [0xff]).unwrap();
            let error = capture_entries(&supplied, snapshot.path()).unwrap_err();
            assert!(
                matches!(error, KaniScalarNativeError::Snapshot(ref io) if io.kind() == std::io::ErrorKind::InvalidData)
            );
            assert_eq!(error.code(), "snapshot_io");
            assert_eq!(error.class(), KaniScalarNativeErrorClass::EnvironmentFault);
        }
        let snapshot = PrivateSnapshot::create().unwrap();
        for (path, bytes) in &entries {
            write_private(&snapshot.path().join(path), bytes).unwrap();
        }
        let mut duplicate = supplied;
        duplicate.push(duplicate[0].clone());
        assert!(matches!(
            capture_entries(&duplicate, snapshot.path()),
            Err(KaniScalarNativeError::SnapshotChanged)
        ));
    }

    #[trace("FR-017-AC-42", "FR-017-AC-44")]
    #[test]
    fn lock_failures_distinguish_codegen_configuration_from_staged_lock() {
        let error = codegen_runtime_revision("no runtime here").unwrap_err();
        assert!(matches!(error, KaniScalarNativeError::CodegenLock));
        assert_eq!(error.code(), "codegen_lock");
        assert_eq!(error.class(), KaniScalarNativeErrorClass::EnvironmentFault);
        for lock in [
            Ok("no runtime here".into()),
            Err(std::io::Error::other("unreadable")),
        ] {
            let error = verify_runtime_revision("expected", lock).unwrap_err();
            assert!(
                matches!(&error, KaniScalarNativeError::DependencyLock { expected, actual } if expected == "expected" && actual.is_none())
            );
            assert_eq!(error.code(), "dependency_lock");
            assert_eq!(error.class(), KaniScalarNativeErrorClass::EnvironmentFault);
        }
    }

    #[trace("FR-017-AC-42", "FR-017-AC-46")]
    #[test]
    fn runtime_revision_read_preserves_dependency_mismatch_and_unreadable_lock_faults() {
        let snapshot = PrivateSnapshot::create().unwrap();
        let path = snapshot.path().join("Cargo.lock");
        let expected = "a".repeat(40);
        let actual = "b".repeat(40);
        let lock = format!("[[package]]\nname = \"quire-contract-runtime\"\nsource = \"git+https://github.com/agent-ix/quire-contract-runtime?branch=main#{actual}\"\n");
        fs::write(&path, lock).unwrap();
        let error = verify_runtime_revision(&expected, fs::read_to_string(&path)).unwrap_err();
        assert!(
            matches!(&error, KaniScalarNativeError::DependencyLock { expected: wanted, actual: Some(found) } if wanted == &expected && found == &actual)
        );
        assert_eq!(error.class(), KaniScalarNativeErrorClass::EnvironmentFault);
        assert!(verify_runtime_revision(&actual, fs::read_to_string(&path)).is_ok());
        fs::remove_file(&path).unwrap();
        let error = verify_runtime_revision(&expected, fs::read_to_string(&path)).unwrap_err();
        assert!(matches!(
            error,
            KaniScalarNativeError::DependencyLock { actual: None, .. }
        ));
        assert_eq!(error.code(), "dependency_lock");
    }

    // Fabricated evidence is used only to cover the classification vocabulary.
    fn classification_evidence(
        outcome: KaniRunOutcome,
        failure_cause: Option<KaniFailureCause>,
    ) -> KaniExecutionEvidence {
        use crate::kani::{
            abi::KaniSolver,
            run::memory::{MemoryMechanism, MemoryObservation},
        };
        KaniExecutionEvidence {
            ceilings: crate::kani::test_support::proof_ceilings::proof_ceilings_with_wall_clock(
                std::time::Duration::from_secs(1),
            ),
            memory: MemoryObservation {
                mechanism: MemoryMechanism::LinuxPidNamespaceProcfsTreeRss,
                peak_resident_bytes: None,
            },
            symbolic_arguments: Vec::new(),
            kind: None,
            harness_path: "table-only".into(),
            launcher_path: "not-invoked".into(),
            arguments: Vec::new(),
            unwind: 1,
            solver: KaniSolver::Cadical,
            exit_code: None,
            outcome,
            success_checks: 0,
            checks: Vec::new(),
            batch: None,
            diagnostics: None,
            failure_cause,
        }
    }

    // These literal expectations are an independent frozen table. Each match is exhaustive:
    // adding a vocabulary item requires an explicit expected row even if production is updated.
    fn expected_error(error: &KaniScalarNativeError) -> (&'static str, KaniScalarNativeErrorClass) {
        use KaniScalarNativeError as E;
        use KaniScalarNativeErrorClass::{CompletionRefusal as R, EnvironmentFault as F};
        match error {
            E::InvalidRequest(_) => ("invalid_request", R),
            E::ArtifactPackage(_) => ("artifact_package", R),
            E::Snapshot(_) => ("snapshot_io", F),
            E::Kani(nested) => ("kani_execution", expected_nested(nested).0),
            E::ProofNotVerified(evidence) => (
                "proof_not_verified",
                expected_outcome(&evidence.outcome, evidence.failure_cause.as_ref()),
            ),
            E::OracleStop(_) => ("oracle_stop", R),
            E::NativeLaunch(_) => ("native_launch", F),
            E::DependencyLock { .. } => ("dependency_lock", F),
            E::NativeResult(_) => ("native_result", F),
            E::SnapshotChanged => ("snapshot_changed", F),
            E::IdentityEncode(_) => ("identity_encode", F),
            E::CodegenLock => ("codegen_lock", F),
        }
    }

    fn expected_outcome(
        outcome: &KaniRunOutcome,
        cause: Option<&KaniFailureCause>,
    ) -> KaniScalarNativeErrorClass {
        use KaniScalarNativeErrorClass::{CompletionRefusal as R, EnvironmentFault as F};
        match outcome {
            KaniRunOutcome::Verified => F,
            KaniRunOutcome::Falsified { .. } => R,
            KaniRunOutcome::CoverUnsatisfied { .. } => R,
            KaniRunOutcome::Cancelled { .. } => F,
            KaniRunOutcome::Inconclusive { reason } => expected_reason(*reason, cause),
        }
    }

    fn expected_reason(
        reason: KaniInconclusiveReason,
        cause: Option<&KaniFailureCause>,
    ) -> KaniScalarNativeErrorClass {
        use KaniScalarNativeErrorClass::{CompletionRefusal as R, EnvironmentFault as F};
        match reason {
            KaniInconclusiveReason::FailedWithoutCounterexample => expected_cause(cause),
            KaniInconclusiveReason::NoVerdict => F,
            KaniInconclusiveReason::MissingCoverSummary => R,
            KaniInconclusiveReason::VacuousProof => R,
            KaniInconclusiveReason::UnwindBoundExhausted => R,
            KaniInconclusiveReason::TimedOut => F,
            KaniInconclusiveReason::MemoryExhausted => F,
        }
    }

    fn expected_cause(cause: Option<&KaniFailureCause>) -> KaniScalarNativeErrorClass {
        use KaniScalarNativeErrorClass::{CompletionRefusal as R, EnvironmentFault as F};
        match cause {
            None => R,
            Some(KaniFailureCause::PropertiesFailed) => R,
            Some(KaniFailureCause::Timeout) => F,
            Some(KaniFailureCause::OutOfMemory) => F,
            Some(KaniFailureCause::ExitCode { .. }) => F,
            Some(KaniFailureCause::Unknown { .. }) => F,
        }
    }

    fn expected_nested(
        nested: &KaniExecutionRefusal,
    ) -> (KaniScalarNativeErrorClass, Option<&'static str>) {
        use KaniScalarNativeErrorClass::{CompletionRefusal as R, EnvironmentFault as F};
        match nested {
            KaniExecutionRefusal::BatchCancelled { .. } => (F, None),
            KaniExecutionRefusal::Tool(_) => (F, None),
            KaniExecutionRefusal::MemoryMechanismUnavailable { .. } => (F, None),
            KaniExecutionRefusal::MemoryObservationFailed { .. } => (F, None),
            KaniExecutionRefusal::BatchMemoryExhausted { .. } => (F, None),
            KaniExecutionRefusal::HarnessNotInCrate { .. } => (R, None),
            KaniExecutionRefusal::HarnessProvenanceMismatch { .. } => (R, None),
            KaniExecutionRefusal::Report(_) => (F, None),
            KaniExecutionRefusal::OutputOverLimit { .. } => (R, Some("kani_output_over_limit")),
            KaniExecutionRefusal::OutputUnread { .. } => (F, Some("kani_output_unread")),
            KaniExecutionRefusal::BatchTimedOut { .. } => (F, None),
            KaniExecutionRefusal::PlaybackForNonMember { .. } => (R, None),
        }
    }

    #[trace("FR-017-AC-41", "FR-017-AC-42", "FR-017-AC-44")]
    #[test]
    fn frozen_outer_code_and_class_table_has_exactly_twelve_distinct_codes() {
        use KaniScalarNativeError as E;
        let errors = [
            E::InvalidRequest("table"),
            E::ArtifactPackage(
                ArtifactBundle::new(vec![Artifact::new("../invalid", "")]).unwrap_err(),
            ),
            E::Snapshot(std::io::Error::other("table")),
            E::Kani(KaniExecutionRefusal::HarnessNotInCrate {
                harness_path: "table".into(),
            }),
            E::ProofNotVerified(Box::new(classification_evidence(
                KaniRunOutcome::Falsified {
                    counterexample: "table".into(),
                },
                None,
            ))),
            E::OracleStop(ScalarOracleStopKind::IllTyped),
            E::NativeLaunch("table".into()),
            E::DependencyLock {
                expected: "table".into(),
                actual: None,
            },
            E::NativeResult("table".into()),
            E::SnapshotChanged,
            identity_from_entries(&literal_entries(), Limits::new(0)).unwrap_err(),
            E::CodegenLock,
        ];
        let mut codes = std::collections::BTreeSet::new();
        for error in errors {
            let (code, class) = expected_error(&error);
            assert_eq!(error.code(), code);
            assert_eq!(error.class(), class);
            assert!(codes.insert(error.code()), "duplicate code {code}");
        }
        assert_eq!(codes.len(), 12);
    }

    #[trace("FR-017-AC-42", "FR-017-AC-44")]
    #[test]
    fn frozen_evidence_classification_covers_all_outcomes_reasons_and_causes_unchanged() {
        use KaniInconclusiveReason as I;
        let outcomes = [
            KaniRunOutcome::Verified,
            KaniRunOutcome::Falsified {
                counterexample: "table-only".into(),
            },
            KaniRunOutcome::CoverUnsatisfied {
                satisfied: 0,
                total: 1,
            },
            KaniRunOutcome::Cancelled {
                source: quire_contract_ir::kani::KaniCancellationSource::Requested,
            },
            KaniRunOutcome::Inconclusive {
                reason: I::FailedWithoutCounterexample,
            },
            KaniRunOutcome::Inconclusive {
                reason: I::NoVerdict,
            },
            KaniRunOutcome::Inconclusive {
                reason: I::MissingCoverSummary,
            },
            KaniRunOutcome::Inconclusive {
                reason: I::VacuousProof,
            },
            KaniRunOutcome::Inconclusive {
                reason: I::UnwindBoundExhausted,
            },
            KaniRunOutcome::Inconclusive {
                reason: I::TimedOut,
            },
            KaniRunOutcome::Inconclusive {
                reason: I::MemoryExhausted,
            },
        ];
        for outcome in outcomes {
            let evidence = classification_evidence(outcome, None);
            let error = KaniScalarNativeError::ProofNotVerified(Box::new(evidence.clone()));
            assert_eq!((error.code(), error.class()), expected_error(&error));
            let KaniScalarNativeError::ProofNotVerified(retained) = error else {
                unreachable!()
            };
            assert_eq!(*retained, evidence);
        }
        for cause in [
            None,
            Some(KaniFailureCause::PropertiesFailed),
            Some(KaniFailureCause::Timeout),
            Some(KaniFailureCause::OutOfMemory),
            Some(KaniFailureCause::ExitCode { code: 7 }),
            Some(KaniFailureCause::Unknown {
                exit_status: "other".into(),
            }),
        ] {
            for failed_check in [false, true] {
                let mut evidence = classification_evidence(
                    KaniRunOutcome::Inconclusive {
                        reason: I::FailedWithoutCounterexample,
                    },
                    cause.clone(),
                );
                if failed_check {
                    evidence.checks = crate::kani::classify::classify_kani_run(
                        false,
                        Some(&crate::kani::test_support::report(
                            "Failure",
                            &[("Failure", "assertion")],
                        )),
                        None,
                        None,
                    )
                    .unwrap()
                    .checks;
                    assert_eq!(evidence.checks.len(), 1);
                }
                let error = KaniScalarNativeError::ProofNotVerified(Box::new(evidence.clone()));
                assert_eq!((error.code(), error.class()), expected_error(&error));
                let KaniScalarNativeError::ProofNotVerified(retained) = error else {
                    unreachable!()
                };
                assert_eq!(*retained, evidence);
            }
        }
    }

    #[trace("FR-017-AC-42", "FR-017-AC-44")]
    #[test]
    fn frozen_nested_refusal_table_preserves_nested_output_codes() {
        use crate::kani::{
            output::report::KaniReportRefusal,
            run::{
                launch::CaptureStream,
                memory::{MemoryMechanism, MemoryObservation},
                tool::{KaniTool, KaniToolError},
            },
        };
        let refusals = [
            KaniExecutionRefusal::HarnessNotInCrate {
                harness_path: "table".into(),
            },
            KaniExecutionRefusal::HarnessProvenanceMismatch {
                harness_path: "table".into(),
            },
            KaniExecutionRefusal::OutputOverLimit {
                stream: CaptureStream::Stdout,
                limit: 1,
                harnesses: 1,
            },
            KaniExecutionRefusal::PlaybackForNonMember {
                harness: "table".into(),
            },
            KaniExecutionRefusal::Tool(KaniToolError::Missing {
                tool: KaniTool::Launcher,
                path: "table".into(),
            }),
            KaniExecutionRefusal::MemoryMechanismUnavailable {
                cause: std::io::Error::other("table"),
            },
            KaniExecutionRefusal::MemoryObservationFailed {
                detail: "table".into(),
            },
            KaniExecutionRefusal::BatchMemoryExhausted {
                members: 1,
                memory_bytes: 1,
                memory: MemoryObservation {
                    mechanism: MemoryMechanism::LinuxPidNamespaceProcfsTreeRss,
                    peak_resident_bytes: Some(2),
                },
            },
            KaniExecutionRefusal::BatchTimedOut {
                members: 1,
                timeout: std::time::Duration::from_secs(1),
            },
            KaniExecutionRefusal::BatchCancelled {
                source: quire_contract_ir::kani::KaniCancellationSource::Requested,
                members: 1,
            },
            KaniExecutionRefusal::OutputUnread {
                stream: CaptureStream::Stdout,
                detail: "table".into(),
            },
            KaniExecutionRefusal::Report(KaniReportRefusal::Missing),
        ];
        for nested in refusals {
            let (expected_class, expected_code) = expected_nested(&nested);
            assert_eq!(nested.code(), expected_code);
            let error = KaniScalarNativeError::Kani(nested);
            assert_eq!(
                (error.code(), error.class()),
                ("kani_execution", expected_class)
            );
            let KaniScalarNativeError::Kani(nested) = error else {
                unreachable!()
            };
            assert_eq!(nested.code(), expected_code);
        }
    }
    #[trace("FR-017-AC-42")]
    #[test]
    fn native_launch_failure_is_an_environment_fault_without_a_receipt() {
        let error = decode_native_launch(LaunchOutcome::TimedOut, unequal_limits()).unwrap_err();
        assert!(matches!(error, KaniScalarNativeError::NativeLaunch(_)));
        assert_eq!(error.code(), "native_launch");
        assert_eq!(error.class(), KaniScalarNativeErrorClass::EnvironmentFault);
    }
    #[trace("FR-017-AC-46")]
    #[test]
    fn regression_non_utf8_lock_uses_the_early_runtime_revision_read_path() {
        let snapshot = PrivateSnapshot::create().unwrap();
        fs::write(snapshot.path().join("Cargo.lock"), [0xff]).unwrap();
        let error = verify_runtime_revision(
            "expected",
            fs::read_to_string(snapshot.path().join("Cargo.lock")),
        )
        .unwrap_err();
        assert!(
            matches!(error, KaniScalarNativeError::Snapshot(ref io) if io.kind() == std::io::ErrorKind::InvalidData),
            "{error:?}"
        );
        assert_eq!(error.code(), "snapshot_io");
        assert_eq!(error.class(), KaniScalarNativeErrorClass::EnvironmentFault);
    }

    #[trace("FR-017-AC-43", "FR-017-AC-50")]
    #[test]
    fn regression_bare_native_result_records_are_counted_and_refused() {
        let limits = unequal_limits();
        let valid = helper_output(&limits_array(limits));
        for text in [
            format!("{valid}IR791_NATIVE_V1\n"),
            format!("IR791_NATIVE_V1\n{valid}"),
            valid.replace("IR791_NATIVE_V1\tcompleted\t5", "IR791_NATIVE_V1"),
        ] {
            assert_native_result(decode_native_output(&text, limits));
        }
    }
    #[trace(
        "FR-017-AC-29",
        "FR-017-AC-46",
        "FR-017-AC-48",
        "FR-017-AC-52",
        "FR-017-AC-53",
        "TC-027"
    )]
    #[test]
    fn companion_staging_retains_every_original_artifact_and_one_canonical_adapter() {
        let (harness, artifacts) =
            crate::kani::test_support::scalar_source_fixture(std::time::Duration::from_secs(600));
        let snapshot = PrivateSnapshot::create().unwrap();
        let cancel = Cancel::new();
        let installation = crate::kani::run::tool::KaniInstallation::new("cargo-kani");
        let limits = unequal_limits();
        let request = KaniScalarNativeRequest {
            cancel: &cancel,
            installation: &installation,
            harness: &harness,
            oracle_artifacts: &artifacts,
            operands: &[4, 1],
            scalar_limits: limits,
            target_directory: Path::new("unused-by-host-staging"),
        };
        validate_request(&request).unwrap();
        assert_eq!(artifacts.len(), 4);
        let staged = stage_package(&request, snapshot.path()).unwrap();
        write_observer(
            &harness.identity.oracle_symbol,
            harness.identity.arguments.len(),
            harness.identity.integer_representation,
            snapshot.path(),
        )
        .unwrap();
        write_private(&snapshot.path().join("Cargo.lock"), b"host-lock").unwrap();
        write_private(
            &snapshot.path().join("observer/Cargo.lock"),
            b"independent-host-lock",
        )
        .unwrap();
        let entries = capture_entries(&staged, snapshot.path()).unwrap();
        assert_eq!(entries.len(), 10);
        for artifact in &artifacts {
            let path = if artifact.path == "src/lib.rs" {
                "src/oracle-artifacts-lib.rs"
            } else {
                &artifact.path
            };
            assert_eq!(
                &entries.iter().find(|(name, _)| name == path).unwrap().1,
                artifact.contents.as_bytes()
            );
        }
        assert_eq!(
            &entries
                .iter()
                .find(|(path, _)| path == "src/lib.rs")
                .unwrap()
                .1,
            harness.rust.contents.as_bytes()
        );
        assert_eq!(
            &entries
                .iter()
                .find(|(path, _)| path == &harness.record.path)
                .unwrap()
                .1,
            harness.record.contents.as_bytes()
        );
        let adapter = fs::read_to_string(snapshot.path().join("src/bin/ir791_native.rs")).unwrap();
        assert!(adapter.contains("#[path = \"../lib.rs\"] mod generated;"));
        assert!(!snapshot.path().join("examples").exists());
        assert_eq!(
            fs::read_to_string(snapshot.path().join("observer/Cargo.toml")).unwrap(),
            scalar_observer_manifest()
        );
        for (path, bytes) in &entries {
            write_private(&snapshot.path().join(path), b"replacement").unwrap();
            assert!(
                matches!(
                    ensure_snapshot_matches(snapshot.path(), &entries),
                    Err(KaniScalarNativeError::SnapshotChanged)
                ),
                "{path}"
            );
            write_private(&snapshot.path().join(path), bytes).unwrap();
        }
        for extra in [
            "extra.rs",
            "observer/extra.rs",
            "observer/.cargo/config.toml",
        ] {
            write_private(&snapshot.path().join(extra), b"unmeasured").unwrap();
            assert!(
                matches!(
                    ensure_snapshot_matches(snapshot.path(), &entries),
                    Err(KaniScalarNativeError::SnapshotChanged)
                ),
                "{extra}"
            );
            fs::remove_file(snapshot.path().join(extra)).unwrap();
        }
        ensure_snapshot_matches(snapshot.path(), &entries).unwrap();
        seal_snapshot(snapshot.path()).unwrap();
        ensure_snapshot_matches(snapshot.path(), &entries).unwrap();
        let digest = independent_digest(&entries);
        snapshot.cleanup().unwrap();
        assert_eq!(independent_digest(&entries), digest);
        assert!(!snapshot.path().exists());
    }

    #[trace("FR-017-AC-29", "FR-017-AC-52", "TC-027")]
    #[test]
    fn native_command_selects_only_the_companion_and_preserves_ordered_inputs() {
        let limits = unequal_limits();
        let args = native_arguments(&[4, -1], limits);
        assert_eq!(
            &args[..9],
            [
                "run",
                "--quiet",
                "--locked",
                "--manifest-path",
                "observer/Cargo.toml",
                "--example",
                "ir791_native",
                "--",
                "4"
            ]
        );
        assert_eq!(args[9], "-1");
        assert_eq!(
            args[10..],
            limits_array(limits).map(|value| value.to_string())
        );
        assert!(!args
            .iter()
            .any(|arg| arg.contains("features") || arg.contains("test-support")));
    }

    #[cfg(unix)]
    #[trace("FR-017-AC-29", "FR-017-AC-53", "TC-027")]
    #[test]
    fn snapshot_rejects_same_bytes_replaced_by_a_symlink() {
        let snapshot = PrivateSnapshot::create().unwrap();
        let root = snapshot.path();
        write_private(&root.join("original"), b"bytes").unwrap();
        std::os::unix::fs::symlink(root.join("original"), root.join("linked")).unwrap();
        let expected = vec![
            ("original".to_owned(), b"bytes".to_vec()),
            ("linked".to_owned(), b"bytes".to_vec()),
        ];
        assert!(matches!(
            ensure_snapshot_matches(root, &expected),
            Err(KaniScalarNativeError::SnapshotChanged)
        ));
    }
}
