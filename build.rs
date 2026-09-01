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
