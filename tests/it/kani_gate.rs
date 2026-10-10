//! Contract controls for the real-Kani lane gate (NFR-006, TC-045).
//! The stand-ins exercise gate control flow; the actual Kani lane is run separately.

use std::{
    collections::BTreeSet,
    fs,
    io::{BufRead, BufReader},
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    process::{Command, Output, Stdio},
    sync::atomic::{AtomicU64, Ordering},
    time::{Duration, Instant},
};
use syn::{visit::Visit, Expr, Item, Lit, Meta};

static NEXT_SANDBOX: AtomicU64 = AtomicU64::new(0);

struct Sandbox {
    root: PathBuf,
    bin: PathBuf,
}

impl Sandbox {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "cg-kani-gate-{}-{}",
            std::process::id(),
            NEXT_SANDBOX.fetch_add(1, Ordering::Relaxed)
        ));
        let bin = root.join("bin");
        fs::create_dir_all(&bin).expect("fresh gate stand-in directory");
        let sandbox = Self { root, bin };
        sandbox.write_executable(
            "git",
            r##"#!/usr/bin/env bash
set -euo pipefail
case "$1" in
  merge-base)
    if [[ "${2-}" == --is-ancestor ]]; then
      [[ "${FAKE_MAIN_ANCESTOR:-yes}" == yes ]]
    else
      printf 'base\n'
    fi ;;
  diff)
    [[ " $* " == *" --no-renames "* ]] || exit 3
    if [[ " $* " == *" origin/main "* ]]; then
      printf '%s' "${FAKE_MAIN_PATHS:-}"
    else
      printf '%s' "${FAKE_BRANCH_PATHS:-}"
    fi ;;
  rev-parse)
    if [[ "${FAKE_MOVE_HEAD:-no}" == yes ]]; then
      count=$(cat "$FAKE_HEAD_COUNT" 2>/dev/null || printf 0)
      printf '%s\n' "$((count + 1))" > "$FAKE_HEAD_COUNT"
      printf 'head%s\n' "$count"
    else
      printf 'head\n'
    fi ;;
  status)
    if [[ "${FAKE_DIRTY:-no}" == yes ]]; then
      printf ' M changed.rs\n'
    elif [[ "${FAKE_MOVE_TREE:-no}" == yes ]]; then
      count=$(cat "$FAKE_TREE_COUNT" 2>/dev/null || printf 0)
      printf '%s\n' "$((count + 1))" > "$FAKE_TREE_COUNT"
      if ((count > 0)); then printf ' M changed.rs\n'; fi
    fi ;;
  *) printf 'unexpected git call: %s\n' "$*" >&2; exit 2 ;;
esac
"##,
        );
        sandbox.write_executable(
            "cargo",
            r##"#!/usr/bin/env bash
set -euo pipefail
if [[ " $* " == *" --list "* ]]; then
  for ((i=0; i<${FAKE_LIST_COUNT:-2}; i++)); do
    printf 'kani_obligations::proof_%s: test\n' "$i"
  done
else
  printf 'started\n' >> "$FAKE_MARKER"
  if ! flock -n "$FAKE_LOCK" true; then
    printf 'held\n' >> "$FAKE_LOCK_MARKER"
  fi
  if [[ "${FAKE_RUN_FAIL:-no}" == yes ]]; then
    printf 'test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out\n'
    exit 1
  fi
  printf 'test result: ok. %s passed; 0 failed; 0 ignored; 0 measured; 0 filtered out\n' "${FAKE_RUN_COUNT:-2}"
fi
"##,
        );
        sandbox.write_executable(
            "cargo-kani",
            "#!/usr/bin/env bash\nprintf '%s\\n' \"${FAKE_KANI_VERSION-Kani 1.0}\"\n",
        );
        sandbox
    }

    fn write_executable(&self, name: &str, contents: &str) {
        let path = self.bin.join(name);
        fs::write(&path, contents).expect("write stand-in");
        let mut permissions = fs::metadata(&path)
            .expect("stand-in metadata")
            .permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(path, permissions).expect("make stand-in executable");
    }

    fn command(&self, script: &str) -> Command {
        let mut command = Command::new("/usr/bin/bash");
        command.arg(script);
        command.current_dir(env!("CARGO_MANIFEST_DIR"));
        command.env("PATH", format!("{}:/usr/bin:/bin", self.bin.display()));
        command.env("FAKE_MARKER", self.root.join("run-marker"));
        command.env("FAKE_LOCK_MARKER", self.root.join("lock-marker"));
        command.env("FAKE_LOCK", self.root.join("lane.lock"));
        command.env("FAKE_HEAD_COUNT", self.root.join("head-count"));
        command.env("FAKE_TREE_COUNT", self.root.join("tree-count"));
        command
    }

    fn gate(&self) -> Command {
        let mut command = self.command("scripts/kani_gate.sh");
        command.args([
            self.root
                .join("lane.lock")
                .to_str()
                .expect("UTF-8 scratch path"),
            "--",
            "cargo",
            "+1.98.1",
            "test",
            "--locked",
            "--test",
            "it",
            "--",
            "--ignored",
            "--test-threads=1",
            "kani_obligations",
        ]);
        command
    }

    fn marker(&self) -> bool {
        self.root.join("run-marker").exists()
    }
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.root).expect("remove own gate scratch directory");
    }
}

fn stdout(output: &Output) -> String {
    String::from_utf8(output.stdout.clone()).expect("UTF-8 gate output")
}

fn assert_failed_without_run(output: &Output, sandbox: &Sandbox) {
    assert!(!output.status.success(), "{}", stdout(output));
    assert!(!sandbox.marker(), "the real lane must not start");
}

/// Trace: NFR-006-AC-1, NFR-006-AC-2, NFR-006-AC-3, TC-045.
#[test]
fn scope_classifies_changed_paths_and_both_rename_directions() {
    let sandbox = Sandbox::new();
    let required = [
        "src/kani/generate/negotiate.rs",
        "src/oracle/boolean_v1.rs",
        "src/routed/generate.rs",
        "src/replay/witness.rs",
        "src/core/identity.rs",
        "src/publication/mod.rs",
        "tests/it/kani_batching.rs",
        "tests/it/skeleton_spine.rs",
        "tests/it/bounded_kani_corpus.rs",
        "tests/it/kani_obligations_state_frame.rs",
        "tests/it/scratch_crate.rs",
        "tests/exact_scalar_support/package.rs",
        "tests/checked_package_support/base.rs",
        "tests/checked_package_support/rekey.rs",
        "tests/state_frame_support/model.rs",
        "schemas/kani-proof-graph-v2.schema.json",
        "schemas/generated-rust-kani-v2.schema.json",
        "Cargo.toml",
        "Cargo.lock",
    ];
    for path in required {
        let output = sandbox
            .command("scripts/kani_scope.sh")
            .env("FAKE_BRANCH_PATHS", format!("{path}\n"))
            .output()
            .expect("scope command");
        assert!(output.status.success());
        assert_eq!(stdout(&output), format!("required\n{path}\n"));
    }
    for path in [
        "spec/kani/functional/FR-017-kani-execution-evidence.md",
        "reviews/REV-018-bound-coverage-observations.md",
        "src/strategy/mod.rs",
        "src/evidence/mod.rs",
        "Makefile",
        ".github/workflows/ci.yml",
    ] {
        let output = sandbox
            .command("scripts/kani_scope.sh")
            .env("FAKE_BRANCH_PATHS", format!("{path}\n"))
            .output()
            .expect("scope command");
        assert_eq!(stdout(&output), "not required\n");
    }
    for paths in [
        "src/kani/old.rs\nsrc/strategy/new.rs\n",
        "src/strategy/old.rs\nsrc/kani/new.rs\n",
    ] {
        let output = sandbox
            .command("scripts/kani_scope.sh")
            .env("FAKE_BRANCH_PATHS", paths)
            .output()
            .expect("rename scope");
        assert_eq!(stdout(&output).lines().next(), Some("required"));
        assert!(stdout(&output).contains("src/kani/"));
    }
}

/// Trace: NFR-006-AC-5, NFR-006-AC-6, NFR-006-AC-7, TC-045.
#[test]
fn gate_runs_once_under_lock_and_refuses_missing_tests_or_failures() {
    let sandbox = Sandbox::new();
    let output = sandbox.gate().output().expect("passing gate");
    assert!(output.status.success(), "{}", stdout(&output));
    assert!(stdout(&output).contains("result=passed ran=2 expected=2"));
    assert!(stdout(&output).contains("kani=Kani 1.0 tree=clean"));
    assert!(!stdout(&output).contains("head="));
    assert_eq!(
        fs::read_to_string(sandbox.root.join("run-marker"))
            .unwrap()
            .lines()
            .count(),
        1
    );
    assert_eq!(
        fs::read_to_string(sandbox.root.join("lock-marker")).unwrap(),
        "held\n"
    );

    for (listed, ran, fails) in [("0", "0", "no"), ("2", "1", "no"), ("2", "2", "yes")] {
        let _ = fs::remove_file(sandbox.root.join("run-marker"));
        let output = sandbox
            .gate()
            .env("FAKE_LIST_COUNT", listed)
            .env("FAKE_RUN_COUNT", ran)
            .env("FAKE_RUN_FAIL", fails)
            .output()
            .expect("negative gate");
        assert!(!output.status.success(), "{}", stdout(&output));
        assert!(stdout(&output).contains("result=failed"));
        if listed == "0" {
            assert!(!sandbox.marker());
        }
    }
}

/// Trace: NFR-006-AC-5, TC-045. A positive observed waiter precedes the
/// no-start assertion; elapsed time is only a deadlock ceiling.
#[test]
fn gate_waits_for_the_existing_lock_holder() {
    let sandbox = Sandbox::new();
    let lock = sandbox.root.join("lane.lock");
    let mut holder = Command::new("flock")
        .args([
            "-x",
            lock.to_str().expect("UTF-8 lock path"),
            "/usr/bin/bash",
            "-c",
            "printf 'READY\\n'; read -r _ || true",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("start lock holder");
    let mut ready = String::new();
    BufReader::new(holder.stdout.take().expect("holder stdout"))
        .read_line(&mut ready)
        .expect("read lock acquisition signal");
    assert_eq!(ready, "READY\n");

    let gate = sandbox
        .gate()
        .stdout(Stdio::piped())
        .spawn()
        .expect("start waiting gate");
    let needle = format!(
        "flock -x {} /usr/bin/bash scripts/kani_gate.sh",
        lock.display()
    );
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        let processes = Command::new("ps")
            .args(["-eo", "args="])
            .output()
            .expect("observe lock waiter");
        if String::from_utf8_lossy(&processes.stdout)
            .lines()
            .any(|line| line.contains(&needle))
        {
            break;
        }
        assert!(
            Instant::now() < deadline,
            "gate never attempted the host lock"
        );
        std::thread::yield_now();
    }
    assert!(!sandbox.marker(), "lane started before lock acquisition");
    drop(holder.stdin.take());
    assert!(holder.wait().expect("release holder").success());
    let output = gate.wait_with_output().expect("gate after lock release");
    assert!(output.status.success(), "{}", stdout(&output));
    assert!(sandbox.marker(), "lane must start after lock release");
}

/// Trace: NFR-006-AC-8, NFR-006-AC-9, NFR-006-AC-10, NFR-006-AC-14, TC-045.
#[test]
fn gate_refuses_missing_launcher_dirty_or_moving_tree_and_stale_main() {
    let sandbox = Sandbox::new();
    fs::remove_file(sandbox.bin.join("cargo-kani")).expect("remove own launcher stand-in");
    let output = sandbox.gate().output().expect("absent launcher");
    assert_failed_without_run(&output, &sandbox);
    assert_eq!(stdout(&output), "kani-gate: not run: launcher absent\n");
    sandbox.write_executable(
        "cargo-kani",
        "#!/usr/bin/env bash\nprintf '%s\\n' \"${FAKE_KANI_VERSION-Kani 1.0}\"\n",
    );
    for (name, value) in [
        ("FAKE_DIRTY", "yes"),
        ("FAKE_KANI_VERSION", ""),
        ("FAKE_MAIN_PATHS", "src/kani/new.rs\n"),
    ] {
        let mut command = sandbox.gate();
        command.env(name, value);
        if name == "FAKE_MAIN_PATHS" {
            command.env("FAKE_MAIN_ANCESTOR", "no");
        }
        let output = command.output().expect("negative gate");
        assert_failed_without_run(&output, &sandbox);
        assert!(stdout(&output).contains("result=failed"));
        if name == "FAKE_MAIN_PATHS" {
            assert!(String::from_utf8_lossy(&output.stderr).contains("src/kani/new.rs"));
        }
    }
    let output = sandbox
        .gate()
        .env("FAKE_MOVE_HEAD", "yes")
        .output()
        .expect("moving head");
    assert!(!output.status.success());
    assert!(stdout(&output).contains("result=failed"));
    let output = sandbox
        .gate()
        .env("FAKE_MOVE_TREE", "yes")
        .output()
        .expect("moving tree");
    assert!(!output.status.success());
    assert!(stdout(&output).contains("tree=dirty"));
    let output = sandbox
        .gate()
        .env("FAKE_MAIN_PATHS", "spec/readme.md\n")
        .env("FAKE_MAIN_ANCESTOR", "no")
        .output()
        .expect("unrelated main change");
    assert!(output.status.success(), "{}", stdout(&output));
}

/// Trace: NFR-006-AC-4, NFR-006-AC-12, TC-045.
#[test]
fn make_targets_share_one_real_lane_and_ci_excludes_it() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let dry = |target| {
        Command::new("make")
            .arg("-n")
            .arg(target)
            .current_dir(root)
            .output()
            .expect("make dry run")
    };
    let kani = stdout(&dry("kani"));
    let gate = stdout(&dry("kani-gate"));
    let shared = kani
        .trim()
        .strip_prefix("flock /tmp/agent-e-heavy-build.lock ")
        .expect("Kani command under lock");
    let gated = gate
        .trim()
        .strip_prefix("/usr/bin/bash scripts/kani_gate.sh /tmp/agent-e-heavy-build.lock -- ")
        .expect("verified Kani command under the same lock");
    assert_eq!(gated, shared, "gate must run the identical full command");
    for filter in [
        "kani_obligations",
        "skeleton_spine",
        "kani_witness_join",
        "bounded_kani_corpus",
        "kani_generation",
        "kani_batching",
    ] {
        assert!(shared.contains(filter));
    }
    assert!(shared.contains("--ignored --test-threads=1"));
    let ci = stdout(&dry("ci"));
    assert!(!ci.contains("kani_gate.sh"));
    assert!(!ci.contains("flock /tmp/agent-e-heavy-build.lock"));
}

struct IncludeTargets(Vec<String>);

impl<'ast> Visit<'ast> for IncludeTargets {
    fn visit_macro(&mut self, mac: &'ast syn::Macro) {
        if mac.path.is_ident("include")
            || mac.path.is_ident("include_str")
            || mac.path.is_ident("include_bytes")
        {
            let target = syn::parse2::<syn::LitStr>(mac.tokens.clone())
                .expect("lane include must name a literal path");
            self.0.push(target.value());
        }
        syn::visit::visit_macro(self, mac);
    }

    fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {
        for attr in &item.attrs {
            if attr.path().is_ident("path") {
                let Meta::NameValue(value) = &attr.meta else {
                    panic!("path attribute must name a file");
                };
                let Expr::Lit(value) = &value.value else {
                    panic!("path attribute must use a string literal");
                };
                let Lit::Str(value) = &value.lit else {
                    panic!("path attribute must use a string literal");
                };
                self.0.push(value.value());
            }
        }
        syn::visit::visit_item_mod(self, item);
    }
}

fn source(path: &Path) -> syn::File {
    syn::parse_file(&fs::read_to_string(path).expect("read source")).expect("parse lane source")
}

fn tagged_ignore(item: &syn::ItemFn) -> (bool, bool) {
    let mut ignored = false;
    let mut tagged = false;
    for attr in &item.attrs {
        if !attr.path().is_ident("ignore") {
            continue;
        }
        ignored = true;
        if let Meta::NameValue(value) = &attr.meta {
            if let Expr::Lit(value) = &value.value {
                if let Lit::Str(value) = &value.lit {
                    tagged = value.value().starts_with("kani lane:");
                }
            }
        }
    }
    (ignored, tagged)
}

/// Trace: NFR-006-AC-11, NFR-006-AC-18, TC-045.
#[test]
fn ignored_lane_selection_and_transitive_include_targets_match_scope() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let sandbox = Sandbox::new();
    let filters = [
        "kani_obligations",
        "skeleton_spine",
        "kani_witness_join",
        "bounded_kani_corpus",
        "kani_generation",
        "kani_batching",
    ];
    let mut pending = Vec::new();
    let mut selected_count = 0;
    for entry in fs::read_dir(root.join("tests/it")).expect("read it modules") {
        let entry = entry.expect("module entry");
        let path = entry.path();
        if path.extension().is_none_or(|ext| ext != "rs")
            || path.file_name().is_some_and(|name| name == "main.rs")
        {
            continue;
        }
        let stem = path.file_stem().expect("module stem").to_string_lossy();
        let file = source(&path);
        let selected_module = filters.iter().any(|filter| stem.contains(filter));
        for item in &file.items {
            let Item::Fn(function) = item else { continue };
            let (ignored, tagged) = tagged_ignore(function);
            if ignored {
                let full_name = format!("{stem}::{}", function.sig.ident);
                let selected = filters.iter().any(|filter| full_name.contains(filter));
                assert_eq!(tagged, selected, "ignored lane selection: {full_name}");
                if selected {
                    selected_count += 1;
                }
            }
        }
        if selected_module {
            pending.push(path);
        }
    }
    assert!(selected_count > 0, "the real lane must select tests");

    let mut seen = BTreeSet::new();
    while let Some(path) = pending.pop() {
        let canonical = path.canonicalize().expect("included path exists");
        if !seen.insert(canonical.clone()) {
            continue;
        }
        if canonical.extension().is_some_and(|ext| ext != "rs") {
            continue;
        }
        let file = source(&canonical);
        let mut includes = IncludeTargets(Vec::new());
        includes.visit_file(&file);
        for target in includes.0 {
            let resolved = canonical
                .parent()
                .expect("naming file parent")
                .join(target)
                .canonicalize()
                .expect("included target exists");
            let relative = resolved
                .strip_prefix(root)
                .expect("target inside repository")
                .to_str()
                .expect("UTF-8 target")
                .replace('\\', "/");
            let output = sandbox
                .command("scripts/kani_scope.sh")
                .env("FAKE_BRANCH_PATHS", format!("{relative}\n"))
                .output()
                .expect("classify included target");
            assert_eq!(
                stdout(&output),
                format!("required\n{relative}\n"),
                "transitive include target must be in the Kani-touching set"
            );
            pending.push(resolved);
        }
    }
    for required in [
        "tests/checked_package_support/base.rs",
        "tests/checked_package_support/rekey.rs",
        "tests/state_frame_support/subject.rs",
        "schemas/kani-proof-graph-v2.schema.json",
        "schemas/generated-rust-kani-v2.schema.json",
        "schemas/kani-corpus-proof-graph-v1.schema.json",
    ] {
        assert!(
            seen.iter().any(|path| path.ends_with(required)),
            "transitive walk missed {required}"
        );
    }
}
