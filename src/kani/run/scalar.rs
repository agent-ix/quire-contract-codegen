//! Paired proof and native execution of one retained routed scalar artifact (FR-017).

use std::{
    fmt, fs,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
    time::Instant,
};

use quire_canonical::{FixedShape, Limits};
use quire_contract_runtime::exact::{
    ChargePoint, Incomplete, Integer, LimitKind, Outcome, Refusal, ScalarLimits, Undefined,
};
use quire_exact::Cancel;
use serde::Serialize;

use crate::{
    core::{
        artifact::{Artifact, ArtifactBundle, PublicationDiagnostic},
        canonical::content_digest_with_limits,
        profile::oracle_crate_manifest,
    },
    kani::{
        classify::KaniRunOutcome,
        identity::KaniScalarObligationHarness,
        run::{
            execute::{
                execute_kani_obligation_readonly, KaniExecutionEvidence, KaniExecutionRequest,
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
    /// SHA-256 bytes of the canonical length-framed staged package identity.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

/// The opaque result of proving and natively invoking one staged scalar artifact.
#[derive(Debug)]
pub struct KaniScalarNativeReceipt {
    identity: ScalarProofContentIdentity,
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
    consumed: [u64; 10],
    admitted_charges: Vec<ChargePoint>,
}

impl NativeMeterObservation {
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
#[derive(Debug)]
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
    validate_request(request)?;
    let snapshot = PrivateSnapshot::create()?;
    let package = stage_package(request, snapshot.path())?;
    write_observer(&request.harness.identity, snapshot.path())?;
    generate_lockfile(request, snapshot.path())?;
    seal_snapshot(snapshot.path())?;
    let identity = measure(&package, snapshot.path())?;
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
    if evidence.outcome != KaniRunOutcome::Verified {
        return Err(KaniScalarNativeError::ProofNotVerified(Box::new(evidence)));
    }
    if measure(&package, snapshot.path())? != identity {
        return Err(KaniScalarNativeError::SnapshotChanged);
    }

    let (native, meter) = execute_native(request, snapshot.path())?;
    if measure(&package, snapshot.path())? != identity {
        return Err(KaniScalarNativeError::SnapshotChanged);
    }

    Ok(KaniScalarNativeReceipt {
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
    if request.oracle_artifacts.len() != 3
        || ["Cargo.toml", "src/lib.rs", "claim-map.json"]
            .iter()
            .any(|path| {
                !request
                    .oracle_artifacts
                    .iter()
                    .any(|item| item.path == *path)
            })
    {
        return Err(KaniScalarNativeError::InvalidRequest(
            "emitted scalar package must contain exactly its generated manifest, library and claim map",
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

fn measure(
    staged: &[(String, Vec<u8>)],
    root: &Path,
) -> Result<ScalarProofContentIdentity, KaniScalarNativeError> {
    let mut entries = staged.to_vec();
    entries.push((
        "Cargo.lock".to_owned(),
        fs::read(root.join("Cargo.lock")).map_err(KaniScalarNativeError::Snapshot)?,
    ));
    entries.push((
        "src/bin/ir791_native.rs".to_owned(),
        fs::read(root.join("src/bin/ir791_native.rs")).map_err(KaniScalarNativeError::Snapshot)?,
    ));
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    ensure_snapshot_matches(root, &entries)?;
    let mut canonical_artifacts = Vec::with_capacity(entries.len());
    for (path, expected) in entries {
        let contents = String::from_utf8(expected)
            .map_err(|_| KaniScalarNativeError::InvalidRequest("staged artifact is not UTF-8"))?;
        canonical_artifacts.push(CanonicalArtifact { path, contents });
    }
    canonical_identity(canonical_artifacts)
}

fn canonical_identity(
    artifacts: Vec<CanonicalArtifact>,
) -> Result<ScalarProofContentIdentity, KaniScalarNativeError> {
    let preimage = ScalarProofPreimage {
        schema: "agent-ix/cg-kani-scalar-proof-content/v2".to_owned(),
        artifacts,
    };
    let digest =
        content_digest_with_limits(&preimage, Limits::new(256 * 1024 * 1024)).map_err(|error| {
            KaniScalarNativeError::NativeResult(format!("canonical identity failed: {error}"))
        })?;
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
    identity: &crate::kani::identity::ScalarObligationIdentity,
    root: &Path,
) -> Result<(), KaniScalarNativeError> {
    let operand_bindings = (0..identity.arguments.len())
        .map(|index| format!(" let operand_{index} = next_i64(&mut arguments)?;"))
        .collect::<String>();
    let parameters = (0..identity.arguments.len())
        .map(|index| format!("&rt::Integer::from(operand_{index})"))
        .collect::<Vec<_>>()
        .join(", ");
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
use quire_contract_runtime::exact as rt;
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
   Ok(rt::Outcome::Completed(value)) => println!("IR791_NATIVE_V1\tcompleted\t{{}}", value),
   Ok(rt::Outcome::Undefined(rt::Undefined::DivisionByZero)) => println!("IR791_NATIVE_V1\tundefined\tdivision_by_zero"),
   Ok(rt::Outcome::Undefined(rt::Undefined::IeeeNotFinite)) => println!("IR791_NATIVE_V1\tundefined\tieee_not_finite"),
   Ok(rt::Outcome::Undefined(rt::Undefined::EmptyReduction)) => println!("IR791_NATIVE_V1\tundefined\tempty_reduction"),
   Ok(rt::Outcome::Undefined(rt::Undefined::NoneValue)) => println!("IR791_NATIVE_V1\tundefined\tnone_value"),
   Ok(rt::Outcome::Refused(rt::Refusal::IntegerOutOfDomain)) => println!("IR791_NATIVE_V1\trefused\tinteger_out_of_domain"),
   Ok(rt::Outcome::Incomplete(value)) => println!("IR791_NATIVE_V1\tincomplete\t{{}}\t{{}}\t{{}}\t{{}}\t{{}}", value.limit_kind.as_str(), value.limit, value.consumed, value.next_charge, value.charge_point.as_str()),
   Ok(_) => println!("IR791_NATIVE_V1\tunsupported"),
   Err(generated::OracleStop::IllTyped(_)) => println!("IR791_NATIVE_V1\toracle_stop\till_typed"),
   Err(generated::OracleStop::InvalidConstant) => println!("IR791_NATIVE_V1\toracle_stop\tinvalid_constant"),
   Err(generated::OracleStop::OperandWidth) => println!("IR791_NATIVE_V1\toracle_stop\toperand_width"),
 }}
 print!("IR791_METER_V1");
 for kind in rt::LimitKind::ALL {{ print!("\t{{}}", meter.consumed(kind)); }}
 for point in meter.admitted_charges() {{ print!("\t{{}}", point.as_str()); }}
 println!();
 Ok(())
}}
"#,
        identity.oracle_symbol, parameters,
    );
    write_private(&root.join("src/bin/ir791_native.rs"), helper.as_bytes())?;
    Ok(())
}

fn execute_native(
    request: &KaniScalarNativeRequest<'_>,
    root: &Path,
) -> Result<(Outcome<Integer>, NativeMeterObservation), KaniScalarNativeError> {
    let mut command = BackendCommand::new("cargo");
    let mut arguments = vec![
        "run".to_owned(),
        "--quiet".to_owned(),
        "--locked".to_owned(),
        "--bin".to_owned(),
        "ir791_native".to_owned(),
        "--".to_owned(),
    ];
    arguments.extend(request.operands.iter().map(ToString::to_string));
    arguments.extend(limits_array(request.scalar_limits).map(|value| value.to_string()));
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
    let LaunchOutcome::Completed {
        exited_successfully: true,
        streams,
        ..
    } = result.outcome
    else {
        return Err(KaniScalarNativeError::NativeLaunch(format!(
            "native cargo run did not complete successfully: {:?}",
            result.outcome
        )));
    };
    let text = streams.merged_text();
    let result_lines = text
        .lines()
        .filter(|line| line.starts_with("IR791_NATIVE_V1\t"));
    let mut result_lines = result_lines;
    let result = result_lines.next().ok_or_else(|| {
        KaniScalarNativeError::NativeResult("missing structured native result".into())
    })?;
    if result_lines.next().is_some() {
        return Err(KaniScalarNativeError::NativeResult(
            "multiple structured native result records".into(),
        ));
    }
    let fields = result.split('\t').collect::<Vec<_>>();
    let native = match fields.as_slice() {
        ["IR791_NATIVE_V1", "completed", value] => {
            Outcome::Completed(value.parse::<Integer>().map_err(|error| {
                KaniScalarNativeError::NativeResult(format!(
                    "invalid typed Integer result: {error}"
                ))
            })?)
        }
        ["IR791_NATIVE_V1", "refused", "integer_out_of_domain"] => {
            Outcome::Refused(Refusal::IntegerOutOfDomain)
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
    let meter_line = text
        .lines()
        .find(|line| line.starts_with("IR791_METER_V1\t"))
        .ok_or_else(|| {
            KaniScalarNativeError::NativeResult("missing structured meter observation".into())
        })?;
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
            consumed,
            admitted_charges,
        },
    ))
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
    let expected = lock_revision(
        include_str!("../../../Cargo.lock"),
        "quire-contract-runtime",
        "https://github.com/agent-ix/quire-contract-runtime",
    )
    .ok_or(KaniScalarNativeError::InvalidRequest(
        "Contract Codegen lock does not name one runtime git revision",
    ))?;
    let mut update = BackendCommand::new("cargo");
    update.args(&[
        "update".into(),
        "--offline".into(),
        "-p".into(),
        "quire-contract-runtime".into(),
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
    let actual = fs::read_to_string(root.join("Cargo.lock"))
        .ok()
        .and_then(|lock| {
            lock_revision(
                &lock,
                "quire-contract-runtime",
                "https://github.com/agent-ix/quire-contract-runtime",
            )
        });
    if actual.as_deref() != Some(expected.as_str()) {
        return Err(KaniScalarNativeError::DependencyLock { expected, actual });
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

impl Drop for PrivateSnapshot {
    fn drop(&mut self) {
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
        let _ = fs::remove_dir_all(&self.0);
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
            if path.is_dir() {
                visit(root, &path, output)?;
            } else if path.is_file() {
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Trace: FR-017-AC-28.
    #[test]
    fn tc_027_changed_staged_bytes_and_unexpected_files_refuse_identity() {
        let root = std::env::temp_dir().join(format!(
            "ir791-snapshot-control-{}-{}",
            std::process::id(),
            NEXT_SNAPSHOT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&root).unwrap();
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

    /// Trace: FR-017-AC-29.
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
}
