//! Test-only observation of the installed Kani launcher's stdout before CG classifies a run.
//!
//! The wrapper delegates to the installed executable through a symlink and forwards its stdout
//! unchanged while `tee` records a second, independent view. It lives in the test's scratch
//! crate, so neither the executable nor its output is copied into the repository.

use std::{
    fs,
    os::unix::fs::{symlink, PermissionsExt},
    path::{Path, PathBuf},
};

use quire_contract_codegen::KaniInstallation;

/// One test-only launcher and the stdout it records from the same invocation.
pub(crate) struct KaniStdoutCapture {
    pub(crate) installation: KaniInstallation,
    stdout: PathBuf,
}

impl KaniStdoutCapture {
    /// Install an observing launcher beside a scratch crate without changing the real launcher.
    pub(crate) fn new(directory: &Path, real: &KaniInstallation) -> Self {
        let launcher = directory.join("observed-cargo-kani");
        let delegate = directory.join("observed-cargo-kani.real");
        let stdout = directory.join("observed-cargo-kani.stdout");
        symlink(
            fs::canonicalize(&real.launcher).expect("resolve installed Kani launcher"),
            &delegate,
        )
        .expect("link installed Kani launcher for observation");
        fs::write(
            &launcher,
            "#!/usr/bin/env bash\nset -o pipefail\n\"$0.real\" \"$@\" | tee \"$0.stdout\"\n",
        )
        .expect("write observing launcher");
        fs::set_permissions(&launcher, fs::Permissions::from_mode(0o700))
            .expect("make observing launcher executable");
        Self {
            installation: KaniInstallation::new(launcher),
            stdout,
        }
    }

    /// Whole stdout emitted by the delegated launcher, independent of CG's retained playback.
    pub(crate) fn stdout(&self) -> String {
        fs::read_to_string(&self.stdout).expect("read observed Kani stdout")
    }
}

/// Read the assertion unit test for this harness directly from observed launcher stdout.
/// Cover playbacks and blocks headed for another harness are not falsifying witnesses.
pub(crate) fn assertion_playback<'a>(stdout: &'a str, harness: &str) -> &'a str {
    let heading = format!("Concrete playback unit test for `{harness}`:");
    stdout
        .split(&heading)
        .skip(1)
        .filter_map(|after_heading| after_heading.split_once("```").map(|(_, rest)| rest))
        .filter_map(|after_open| after_open.split_once("```").map(|(block, _)| block.trim()))
        .find(|block| {
            block.contains("kani::concrete_playback_run")
                && !block
                    .lines()
                    .any(|line| line.starts_with("/// Check for `cover`"))
        })
        .expect("the observed stdout has this harness's assertion playback")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scratch_crate::TemporaryDirectory;
    use ix_trace_rs::trace;

    use qsl_replay::{ReplayLimits, ScalarLimits, WitnessValue, DEFAULT_REPLAY_INPUT_BYTES};
    use quire_contract_codegen::{
        decode_falsification, execute_kani_obligation, replay_counterexample, KaniExecutionRequest,
        KaniObligationIdentity, KaniRunOutcome, LockedSource, ReplayInputs, ReplayPackage,
        ReplayVerdict,
    };
    use std::{collections::BTreeMap, process::Command};

    fn synthetic_playback(harness: &str, amount: i64, balance: i64) -> String {
        let values = [amount, balance]
            .map(|value| {
                let bytes = value.to_le_bytes().map(|byte| byte.to_string()).join(", ");
                format!("        // {value}\n        vec![{bytes}],\n")
            })
            .concat();
        format!(
            "/// Test generated for harness `{harness}`\n\
             /// Check for `assertion`: \"synthetic assertion\"\n\
             #[test]\n\
             fn kani_concrete_playback_synthetic() {{\n\
                 let concrete_vals: Vec<Vec<u8>> = vec![\n{values}    ];\n\
                 kani::concrete_playback_run(concrete_vals, synthetic);\n\
             }}\n"
        )
    }

    fn native_twin(identity: &KaniObligationIdentity) -> ReplayPackage {
        let source = "language \"ix:native\" edition \"1-draft\";\n\
                      profile v = \"quire.value.complete/v1\";\n\
                      function balance_never_grows using v(amount_current: Int[0, 1000], \
                      balance_pre: Int[0, 1000]): Boolean pure { balance_pre + amount_current \
                      <= balance_pre }\n";
        let limits = ScalarLimits {
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
        ReplayPackage::new(
            ReplayInputs {
                decode_limits: crate::unbounded_decode_limits(),
                source: LockedSource {
                    authority: "agent-ix".to_owned(),
                    identity: "test:kani-stdout-binding".to_owned(),
                    namespace: "git".to_owned(),
                    revision: "r1".to_owned(),
                    bytes: source.as_bytes().to_vec(),
                },
                dependencies: Vec::new(),
                accounting_limits: limits,
                stage_limits: BTreeMap::from([(
                    "s1.input_bytes".to_owned(),
                    DEFAULT_REPLAY_INPUT_BYTES,
                )]),
                supplied_limits: None,
                declared_domains: Vec::new(),
                replay_limits: ReplayLimits::default(),
            },
            "balance_never_grows",
        )
        .expect("native twin compiles")
        .with_kani_identity(identity)
        .expect("the retained proof identity binds the twin")
    }

    /// The observer passes through the launcher's stdout and exit status while retaining its
    /// own byte-identical copy, including a nonzero exit as Kani uses for falsification.
    #[trace("FR-016-AC-25", "TC-026")]
    #[test]
    fn tc_026_observing_launcher_retains_actual_stdout_and_exit() {
        let scratch = TemporaryDirectory::new("kani-stdout-capture-support");
        let fake = scratch.path.join("stand-in-kani");
        fs::write(&fake, "#!/bin/sh\nprintf 'actual\\nplayback\\n'\nexit 1\n")
            .expect("write stand-in launcher");
        fs::set_permissions(&fake, fs::Permissions::from_mode(0o700))
            .expect("make stand-in executable");
        let capture = KaniStdoutCapture::new(&scratch.path, &KaniInstallation::new(fake.clone()));
        let output = Command::new(&capture.installation.launcher)
            .output()
            .expect("run observing launcher");
        assert_eq!(output.status.code(), Some(1));
        assert_eq!(output.stdout, b"actual\nplayback\n");
        assert_eq!(capture.stdout().as_bytes(), output.stdout);
    }

    /// The installed launcher resolves its resources even when invoked through the observer.
    #[trace("FR-016-AC-25", "TC-026")]
    #[test]
    #[ignore = "kani lane: requires installed cargo-kani"]
    fn tc_026_observing_installed_launcher_preserves_version_output() {
        let real = KaniInstallation::discover().expect("cargo-kani is installed");
        let scratch = TemporaryDirectory::new("kani-stdout-capture-version");
        let capture = KaniStdoutCapture::new(&scratch.path, &real);
        let expected = Command::new(&real.launcher)
            .arg("--version")
            .output()
            .expect("run installed launcher directly");
        let observed = Command::new(&capture.installation.launcher)
            .arg("--version")
            .output()
            .expect("run installed launcher through observer");
        assert_eq!(observed.status, expected.status);
        assert_eq!(observed.stdout, expected.stdout);
        assert_eq!(observed.stderr, expected.stderr);
        assert_eq!(capture.stdout().as_bytes(), expected.stdout);
    }

    /// Selection reads the first eligible block for this harness and preserves its test text.
    #[trace("FR-016-AC-25", "TC-026")]
    #[test]
    fn tc_026_observed_stdout_selects_only_the_named_assertion_block() {
        let stdout = "Concrete playback unit test for `h`:\n```\n/// Check for `cover`\nkani::concrete_playback_run(cover);\n```\n\
                      Concrete playback unit test for `other`:\n```\n/// Check for `assertion`\nkani::concrete_playback_run(other);\n```\n\
                      Concrete playback unit test for `h`:\n```\n/// Check for `assertion`\nkani::concrete_playback_run(first);\n```\n\
                      Concrete playback unit test for `h`:\n```\n/// Check for `assertion`\nkani::concrete_playback_run(second);\n```\n";
        let first = "/// Check for `assertion`\nkani::concrete_playback_run(first);";
        let second = "/// Check for `assertion`\nkani::concrete_playback_run(second);";
        let selected = assertion_playback(stdout, "h");
        assert_eq!(selected, first);
        assert_ne!(
            selected, second,
            "the later target block cannot replace the first"
        );
    }
}
