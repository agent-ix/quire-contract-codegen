// Implements: FR-001

use std::{env, process::Command};

const UNKNOWN_REVISION: &str = "0000000000000000000000000000000000000000";
const UNKNOWN_RECORDED_AT: &str = "1970-01-01T00:00:00Z";

fn command_output(program: &str, arguments: &[&str]) -> Option<String> {
    let output = Command::new(program).args(arguments).output().ok()?;
    if !output.status.success() {
        return None;
    }
    String::from_utf8(output.stdout)
        .ok()
        .map(|value| value.trim().to_owned())
}

fn valid_revision(value: &str) -> bool {
    value.len() == 40
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

/// Reports whether a recorded time is the RFC 3339 form the shared attestation
/// schema requires.
///
/// `git show -s --format=%cI` always produces this form. `QUIRE_CODEGEN_ARCHIVE_RECORDED_AT`
/// is whatever an archive build supplies, and until generated artifacts moved onto the
/// shared proof-attestation shape nothing downstream constrained it. Now it lands in
/// `observed_at`, which the packaged schema declares `format: date-time` and which
/// `quoin change-assurance seal-attestation` enforces -- measured: a body carrying
/// `Mon Aug 31 12:00:00 2026 +0000` is refused with exit 2. An unvalidated value would
/// therefore make every artifact that build ever emits unsealable, discoverable only in a
/// downstream repository. Its sibling `QUIRE_CODEGEN_ARCHIVE_REVISION` was validated all
/// along; this closes the same hole on the same input.
///
/// A value that is not this form is rejected in favour of the unavailable marker. Saying
/// the time was not available is worse evidence than saying it, and better than emitting
/// attestations no tool will seal.
fn valid_recorded_at(value: &str) -> bool {
    let bytes = value.as_bytes();
    let punctuation = [(4, b'-'), (7, b'-'), (10, b'T'), (13, b':'), (16, b':')];
    let digits = [0, 1, 2, 3, 5, 6, 8, 9, 11, 12, 14, 15, 17, 18];
    if !punctuation
        .iter()
        .all(|(at, byte)| bytes.get(*at) == Some(byte))
        || !digits
            .iter()
            .all(|at| bytes.get(*at).is_some_and(u8::is_ascii_digit))
    {
        return false;
    }
    match &bytes[19..] {
        b"Z" => true,
        offset if offset.len() == 6 && matches!(offset[0], b'+' | b'-') && offset[3] == b':' => {
            offset[1..3].iter().all(u8::is_ascii_digit)
                && offset[4..6].iter().all(u8::is_ascii_digit)
        }
        _ => false,
    }
}

fn main() {
    let git_revision =
        command_output("git", &["rev-parse", "HEAD"]).filter(|revision| valid_revision(revision));
    let archive_revision = env::var("QUIRE_CODEGEN_ARCHIVE_REVISION")
        .ok()
        .filter(|revision| valid_revision(revision));
    let revision_available = git_revision.is_some() || archive_revision.is_some();
    let revision = git_revision
        .or(archive_revision)
        .unwrap_or_else(|| UNKNOWN_REVISION.to_owned());
    let recorded_at = command_output("git", &["show", "-s", "--format=%cI", "HEAD"])
        .or_else(|| env::var("QUIRE_CODEGEN_ARCHIVE_RECORDED_AT").ok())
        .filter(|value| valid_recorded_at(value))
        .unwrap_or_else(|| UNKNOWN_RECORDED_AT.to_owned());
    let status = command_output(
        "git",
        &[
            "status",
            "--porcelain=v1",
            "--untracked-files=all",
            "--",
            ".",
        ],
    );
    let source_dirty =
        status.as_ref().map_or(true, |value| !value.is_empty()) || !revision_available;
    let rustc = env::var("RUSTC").unwrap_or_else(|_| "rustc".to_owned());
    let toolchain = command_output(&rustc, &["--version"])
        .unwrap_or_else(|| "rustc identity unavailable".to_owned());

    println!("cargo:rustc-env=QUIRE_CODEGEN_SOURCE_REVISION={revision}");
    println!("cargo:rustc-env=QUIRE_CODEGEN_RECORDED_AT={recorded_at}");
    println!("cargo:rustc-env=QUIRE_CODEGEN_SOURCE_REVISION_AVAILABLE={revision_available}");
    println!("cargo:rustc-env=QUIRE_CODEGEN_SOURCE_DIRTY={source_dirty}");
    println!(
        "cargo:rustc-env=QUIRE_CODEGEN_TARGET={}",
        env::var("TARGET").unwrap_or_else(|_| "unknown-target".to_owned())
    );
    println!(
        "cargo:rustc-env=QUIRE_CODEGEN_TARGET_OS={}",
        env::var("CARGO_CFG_TARGET_OS").unwrap_or_else(|_| "unknown-os".to_owned())
    );
    println!("cargo:rustc-env=QUIRE_CODEGEN_TOOLCHAIN={toolchain}");
    for path in [
        "build.rs",
        "Cargo.toml",
        "Cargo.lock",
        "src",
        "schemas",
        "spec/functional/FR-001-deterministic-oracles.md",
    ] {
        println!("cargo:rerun-if-changed={path}");
    }
    println!("cargo:rerun-if-env-changed=QUIRE_CODEGEN_ARCHIVE_REVISION");
    println!("cargo:rerun-if-env-changed=QUIRE_CODEGEN_ARCHIVE_RECORDED_AT");
    if let Some(git_head) = command_output("git", &["rev-parse", "--git-path", "HEAD"]) {
        println!("cargo:rerun-if-changed={git_head}");
    }
    if let Some(symbolic_head) =
        command_output("git", &["rev-parse", "--symbolic-full-name", "HEAD"])
    {
        if symbolic_head != "HEAD" {
            if let Some(path) = command_output("git", &["rev-parse", "--git-path", &symbolic_head])
            {
                println!("cargo:rerun-if-changed={path}");
            }
        }
    }
}
