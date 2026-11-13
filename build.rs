//! Exposes the version of the `quire-exact` crate this build links as
//! `CG_QUIRE_EXACT_VERSION` (FR-056, the kernel version).
//!
//! `quire-exact` exports no version constant and Cargo gives a build script no dependency
//! versions, so the version is read from the `Cargo.lock` beside this manifest. A lock file that
//! is absent or has no such package gives `unknown` and a build warning, never a build failure.

use std::{env, fs, path::PathBuf};

fn quire_exact_version(lock: &str) -> Option<&str> {
    lock.split("[[package]]").find_map(|package| {
        let field = |key: &str| {
            package.lines().find_map(|line| {
                line.strip_prefix(key)?
                    .strip_prefix(" = \"")?
                    .strip_suffix('"')
            })
        };
        (field("name")? == "quire-exact")
            .then(|| field("version"))
            .flatten()
    })
}

fn main() {
    let lock = env::var_os("CARGO_MANIFEST_DIR")
        .map(|directory| PathBuf::from(directory).join("Cargo.lock"));
    if let Some(lock) = &lock {
        println!("cargo:rerun-if-changed={}", lock.display());
    }
    let text = lock.and_then(|lock| fs::read_to_string(lock).ok());
    let version = text.as_deref().and_then(quire_exact_version);
    if version.is_none() {
        println!("cargo:warning=no quire-exact package in Cargo.lock; kernel version is `unknown`");
    }
    println!(
        "cargo:rustc-env=CG_QUIRE_EXACT_VERSION={}",
        version.unwrap_or("unknown")
    );
}
