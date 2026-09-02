//! Sealing an emitted attestation body through the pinned Quoin CLI.
//!
//! Shared by every test that emits an attestation, because there are four body
//! shapes — oracle Rust, oracle source map, harness, strategy — and a second copy
//! of this helper would be a second definition of what "the shared shape" means.
//!
//! A missing prerequisite is a failure here, never a skip. The whole point of
//! these helpers is that the shape is measured against the tool that owns it, and
//! a test that stands down when that tool is absent reports the same green as one
//! that ran.

// Each test binary uses a subset of these, and an unused one here is not dead
// code in the crate — it is code the other binary uses.
#![allow(dead_code)]

use std::{
    fs,
    io::Write as _,
    path::Path,
    process::{Command, Stdio},
};

use jsonschema::{Draft, JSONSchema};
use quire_contract_codegen::Artifact;

/// Run the pinned Quoin CLI. Its absence is a failure and never a skip.
pub fn quoin(arguments: &[&str], stdin: Option<&str>) -> (i32, String, String) {
    let mut command = Command::new("quoin");
    command
        .args(arguments)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(if stdin.is_some() {
            Stdio::piped()
        } else {
            Stdio::null()
        });
    let mut child = command.spawn().unwrap_or_else(|error| {
        panic!(
            "the pinned quoin CLI could not be run ({error}). It is required: these tests \
             validate the emitted attestation against the shape Quoin itself accepts, and a \
             test that stands down when its tool is absent reports the same green as one that ran."
        )
    });
    if let Some(input) = stdin {
        child
            .stdin
            .as_mut()
            .expect("quoin stdin")
            .write_all(input.as_bytes())
            .expect("write the attestation body to quoin");
    }
    let output = child.wait_with_output().expect("quoin did not complete");
    (
        output.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&output.stdout).into_owned(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
    )
}

/// The packaged proof-attestation schema, read from the tool that publishes it.
///
/// Fetched rather than copied into this repository. A local copy is a second
/// statement of the contract that drifts silently; these are the bytes the sealing
/// and verification code was written against.
pub fn packaged_attestation_schema() -> serde_json::Value {
    let (code, stdout, stderr) = quoin(
        &[
            "change-assurance",
            "schema",
            "--name",
            "proof-attestation-v1.schema.json",
        ],
        None,
    );
    assert_eq!(code, 0, "quoin refused to publish the schema: {stderr}");
    serde_json::from_str(&stdout).expect("the packaged schema is JSON")
}

/// A validator over the packaged schema, with format assertion switched on.
///
/// Formats are annotations by default, so `observed_at: "not a time"` would
/// validate. The shared schema declares `format: date-time` and the CLI enforces
/// it, so this validator is told to as well; otherwise the two halves of a
/// conformance check disagree about what the schema says.
pub fn packaged_attestation_validator(schema: &serde_json::Value) -> JSONSchema {
    JSONSchema::options()
        .with_draft(Draft::Draft202012)
        .should_validate_formats(true)
        .compile(schema)
        .expect("the packaged attestation schema compiles")
}

/// Seal one emitted attestation body over the artifact it describes.
///
/// The media type is taken from the body's own `--output-media-type`, which is the
/// generator's statement about the bytes. Re-deriving it from the file extension
/// here would make the seal agree with a guess rather than with the generator.
pub fn seal_attestation(
    body: &str,
    artifact: &Artifact,
    directory: &Path,
) -> (i32, String, String) {
    let output = directory.join(
        artifact
            .path
            .rsplit('/')
            .next()
            .expect("an artifact path has a final component"),
    );
    fs::write(&output, &artifact.contents).expect("write the retained output");
    let parsed: serde_json::Value = serde_json::from_str(body).expect("the body is JSON");
    let argv: Vec<String> = parsed["command"]["argv"]
        .as_array()
        .expect("the body declares an argv")
        .iter()
        .map(|item| item.as_str().expect("an argv element").to_owned())
        .collect();
    let media_type = argv
        .iter()
        .position(|item| item == "--output-media-type")
        .and_then(|at| argv.get(at + 1))
        .cloned()
        .expect("the body declares its output media type");
    quoin(
        &[
            "change-assurance",
            "seal-attestation",
            "--input",
            "-",
            "--output",
            output.to_str().expect("a UTF-8 output path"),
            "--media-type",
            &media_type,
            "--json",
        ],
        Some(body),
    )
}

/// Seal one body, require the sealed form to validate, and return it.
pub fn seal_and_validate(
    body: &str,
    artifact: &Artifact,
    directory: &Path,
    validator: &JSONSchema,
) -> serde_json::Value {
    let (code, stdout, stderr) = seal_attestation(body, artifact, directory);
    assert_eq!(
        code, 0,
        "quoin refused the emitted attestation body for {}: {stderr}",
        artifact.path
    );
    let sealed: serde_json::Value = serde_json::from_str(&stdout).expect("the sealed form is JSON");
    assert!(
        validator.validate(&sealed).is_ok(),
        "the sealed attestation for {} does not validate against the packaged schema: {stdout}",
        artifact.path
    );
    assert_eq!(
        sealed["retained_output"]["size_bytes"].as_u64().unwrap() as usize,
        artifact.contents.len()
    );
    sealed
}
