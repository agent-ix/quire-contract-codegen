//! Exposes the version of the `quire-exact` crate this build links as
//! `CG_QUIRE_EXACT_VERSION`, read from `Cargo.lock` (FR-056, the kernel version).

use std::{env, fs, path::PathBuf};

fn main() {
    let manifest_dir = env::var_os("CARGO_MANIFEST_DIR").map(PathBuf::from);
    let lock = manifest_dir
        .expect("cargo sets CARGO_MANIFEST_DIR")
        .join("Cargo.lock");
    println!("cargo:rerun-if-changed={}", lock.display());
    let text = fs::read_to_string(&lock).expect("Cargo.lock is readable");
    let mut lines = text.lines();
    let version = loop {
        match lines.next() {
            Some("name = \"quire-exact\"") => {
                let line = lines.next().expect("a version follows the package name");
                break line
                    .strip_prefix("version = \"")
                    .and_then(|rest| rest.strip_suffix('"'))
                    .expect("a quoted version")
                    .to_owned();
            }
            Some(_) => {}
            None => panic!("Cargo.lock has no quire-exact package"),
        }
    };
    println!("cargo:rustc-env=CG_QUIRE_EXACT_VERSION={version}");
}
