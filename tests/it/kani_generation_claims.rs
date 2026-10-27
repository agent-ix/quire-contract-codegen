//! Current-build module claims and real mutation controls (FR-041, TC-056).

use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
};

use ix_trace_rs::trace;
use quire_contract_codegen::{
    classify_kani_run as classify_typed, select_counterexample, ClassifiedRun, KaniCheckResult,
    KaniCheckStatus, KaniReportRefusal, KaniRunOutcome, ObligationKind,
};
use serde::Deserialize;
use syn::{spanned::Spanned, Item};

const DECLARATION: &str = "scripts/kani_claims.json";

fn classify_kani_run(
    exited_successfully: bool,
    report: Option<&[u8]>,
    text: &str,
    kind: Option<ObligationKind>,
) -> Result<ClassifiedRun, KaniReportRefusal> {
    classify_typed(
        exited_successfully,
        report,
        select_counterexample(text, None),
        kind,
    )
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Mutation {
    from: String,
    to: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Claim {
    name: String,
    harness: String,
    subject: String,
    expectation: String,
    mutation: Mutation,
    #[serde(default)]
    hold: Option<ClaimHold>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ClaimHold {
    reason: String,
    expiry: String,
    follow_up: String,
}

const RETIRED_V1_CLAIM: &str = "kani_fr_003_4_proof_boolean_transition_module";
const RETIRED_V1_REASON: &str =
    "V1 deleted by owner ruling 2026-10-27; no genuine V2 named independent expectation/control exists yet";
const RETIRED_V1_EXPIRY: &str =
    "until the V2 named independent expectation + real positive/falsifying control exists";

fn validate_claim_holds(declaration: &Declaration) -> usize {
    let mut held = 0;
    for claim in &declaration.modules {
        if let Some(hold) = &claim.hold {
            assert_eq!(
                claim.name, RETIRED_V1_CLAIM,
                "only retired V1 claim may be held"
            );
            assert_eq!(hold.reason, RETIRED_V1_REASON);
            assert_eq!(hold.expiry, RETIRED_V1_EXPIRY);
            assert_eq!(hold.follow_up, "IR-1047");
            println!(
                "kani-claims: held={} reason={} expiry={} follow_up={}",
                claim.name, hold.reason, hold.expiry, hold.follow_up
            );
            held += 1;
        }
    }
    assert!(held <= 1, "only one retired V1 claim may be held");
    held
}

#[test]
fn retired_v1_claim_is_visible_as_a_single_bounded_hold() {
    let declaration: Declaration = serde_json::from_slice(
        &fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join(DECLARATION)).unwrap(),
    )
    .unwrap();
    assert_eq!(validate_claim_holds(&declaration), 1);
    assert_eq!(
        declaration.modules.len(),
        1,
        "no active claim may be skipped"
    );
    let generated = &crate::kani_generation::guard_sources()[0].1;
    assert!(!module_extents(generated).contains_key(RETIRED_V1_CLAIM));
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Declaration {
    modules: Vec<Claim>,
    helper_mutations: BTreeMap<String, Mutation>,
}

#[derive(Debug, Deserialize)]
struct Report {
    harness_metadata: Vec<Harness>,
    verification_results: VerificationResults,
}

#[derive(Debug, Deserialize)]
struct VerificationResults {
    summary: VerificationSummary,
    results: Vec<VerificationResult>,
}

#[derive(Debug, Deserialize)]
struct VerificationSummary {
    executed: u32,
    status: String,
}

#[derive(Debug, Deserialize)]
struct VerificationResult {
    harness_id: String,
    status: String,
}

#[derive(Debug, Deserialize)]
struct Harness {
    pretty_name: String,
    source: HarnessSource,
    goto_file: PathBuf,
}

#[derive(Debug, Deserialize)]
struct HarnessSource {
    file: String,
    start_line: usize,
    end_line: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Extent {
    start: usize,
    end: usize,
}

fn module_extents(source: &str) -> BTreeMap<String, Extent> {
    let file = syn::parse_file(source).expect("generated Rust must parse");
    file.items
        .iter()
        .filter_map(|item| match item {
            Item::Mod(module) if module.content.is_some() => Some((
                module.ident.to_string(),
                Extent {
                    start: module.span().start().line,
                    end: module.span().end().line,
                },
            )),
            _ => None,
        })
        .collect()
}

fn function_extents(source: &str) -> BTreeMap<String, Extent> {
    let file = syn::parse_file(source).expect("generated Rust must parse");
    file.items
        .iter()
        .filter_map(|item| match item {
            Item::Fn(function) => Some((
                function.sig.ident.to_string(),
                Extent {
                    start: function.span().start().line,
                    end: function.span().end().line,
                },
            )),
            _ => None,
        })
        .collect()
}

fn claim_census(
    declaration: &Declaration,
    base_names: &BTreeSet<String>,
    compiled: &BTreeSet<String>,
    extents: &BTreeMap<String, Extent>,
    checks: &[KaniCheckResult],
) -> Result<(), String> {
    if declaration.modules.is_empty() {
        return Err("empty claimed-module declaration".to_owned());
    }
    let current: BTreeSet<_> = declaration
        .modules
        .iter()
        .map(|claim| claim.name.clone())
        .collect();
    if current.len() != declaration.modules.len() {
        return Err("duplicate claimed module".to_owned());
    }
    let removed: Vec<_> = base_names.difference(&current).cloned().collect();
    if !removed.is_empty() {
        return Err(format!("removed claimed module(s): {}", removed.join(", ")));
    }
    for claim in &declaration.modules {
        if claim.hold.is_some() {
            if compiled.contains(&claim.name) || extents.contains_key(&claim.name) {
                return Err(format!("held V1 module is still generated: {}", claim.name));
            }
            continue;
        }
        if !compiled.contains(&claim.name) {
            return Err(format!("absent compiled module: {}", claim.name));
        }
        let extent = extents
            .get(&claim.name)
            .ok_or_else(|| format!("absent source extent: {}", claim.name))?;
        let success = checks.iter().any(|check| {
            check.status == KaniCheckStatus::Success
                && check.location.file == "src/lib.rs"
                && check
                    .location
                    .line
                    .is_some_and(|line| (extent.start..=extent.end).contains(&(line as usize)))
        });
        if !success {
            return Err(format!("unreached claimed module: {}", claim.name));
        }
    }
    Ok(())
}

fn output_text(output: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

fn trusted_launcher() -> PathBuf {
    let launcher = std::env::var_os("KANI_GATE_LAUNCHER")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(std::env::var_os("HOME").expect("Kani home"))
                .join(".cargo/bin/cargo-kani")
        });
    assert!(
        launcher.is_absolute() && launcher.is_file(),
        "installed real Kani launcher is required at {}",
        launcher.display()
    );
    launcher
}

fn execute_kani(
    directory: &Path,
    target: &Path,
    report: &Path,
    harness: &str,
    timeout: &str,
) -> (Output, Option<Vec<u8>>) {
    let _ = fs::remove_file(report);
    let output = Command::new("/usr/bin/timeout")
        .arg(timeout)
        .arg(trusted_launcher())
        .args([
            "-Z",
            "function-contracts",
            "-Z",
            "unstable-options",
            "-Z",
            "concrete-playback",
            "--concrete-playback",
            "print",
            "--harness-timeout",
            "120s",
            "--harness",
            harness,
            "--exact",
            "--force-build",
            "--export-json",
        ])
        .arg(report)
        .env("CARGO_TARGET_DIR", target)
        .current_dir(directory)
        .output()
        .expect("real cargo kani must launch");
    let bytes = fs::read(report).ok();
    (output, bytes)
}

fn run_kani(directory: &Path, target: &Path, report: &Path, harness: &str) -> (Output, Vec<u8>) {
    let (output, bytes) = execute_kani(directory, target, report, harness, "180s");
    assert!(
        !matches!(output.status.code(), Some(124 | 137)),
        "Kani launcher timed out or was killed: {}",
        output_text(&output)
    );
    let bytes =
        bytes.unwrap_or_else(|| panic!("Kani produced no report: {}", output_text(&output)));
    (output, bytes)
}

fn base_names() -> BTreeSet<String> {
    let base = Command::new("git")
        .args(["merge-base", "HEAD", "origin/main"])
        .output()
        .expect("git merge-base must launch");
    assert!(base.status.success(), "{}", output_text(&base));
    let base = String::from_utf8(base.stdout).expect("UTF-8 commit");
    base_names_at(Path::new(env!("CARGO_MANIFEST_DIR")), base.trim())
        .unwrap_or_else(|error| panic!("{error}"))
}

fn base_names_at(directory: &Path, base: &str) -> Result<BTreeSet<String>, String> {
    let tree = Command::new("git")
        .arg("-C")
        .arg(directory)
        .args(["ls-tree", "-z", "--name-only", base, "--", DECLARATION])
        .output()
        .map_err(|error| format!("cannot inspect base claim tree: {error}"))?;
    if !tree.status.success() {
        return Err(format!(
            "cannot inspect base claim tree: {}",
            output_text(&tree)
        ));
    }
    if tree.stdout.is_empty() {
        return Ok(BTreeSet::new());
    }
    if tree.stdout != format!("{DECLARATION}\0").as_bytes() {
        return Err(format!(
            "unexpected base claim tree entry: {:?}",
            tree.stdout
        ));
    }
    let object = format!("{base}:{DECLARATION}");
    let previous = Command::new("git")
        .arg("-C")
        .arg(directory)
        .args(["show", &object])
        .output()
        .map_err(|error| format!("cannot read base claim declaration: {error}"))?;
    if !previous.status.success() {
        return Err(format!(
            "cannot read base claim declaration: {}",
            output_text(&previous)
        ));
    }
    let declaration: Declaration = serde_json::from_slice(&previous.stdout)
        .map_err(|error| format!("base claim declaration must parse: {error}"))?;
    Ok(declaration
        .modules
        .into_iter()
        .map(|claim| claim.name)
        .collect())
}

fn graph_for(
    report: &Report,
    target: &Path,
    harness_name: &str,
) -> BTreeMap<String, BTreeSet<String>> {
    let harness = report
        .harness_metadata
        .iter()
        .find(|harness| harness.pretty_name == harness_name)
        .expect("selected compiled harness");
    let goto = harness.goto_file.to_string_lossy();
    let linked = goto
        .strip_suffix(".symtab.out")
        .map(|stem| PathBuf::from(format!("{stem}.out")))
        .expect("Kani goto artifact shape");
    assert!(
        linked.starts_with(target) && linked.is_file(),
        "current linked GOTO artifact"
    );
    let map_file = linked.with_extension("pretty_name_map.json");
    let names: BTreeMap<String, Option<String>> =
        serde_json::from_slice(&fs::read(map_file).expect("current pretty-name map"))
            .expect("Kani pretty-name map must parse");
    let version = Command::new(trusted_launcher())
        .arg("--version")
        .output()
        .expect("cargo-kani version must launch");
    assert!(version.status.success(), "{}", output_text(&version));
    let line = String::from_utf8(version.stdout).expect("UTF-8 Kani version");
    let version = line
        .lines()
        .next()
        .and_then(|line| line.strip_prefix("Kani Rust Verifier "))
        .and_then(|line| line.split_whitespace().next())
        .expect("installed Kani version");
    let kani_home = std::env::var_os("KANI_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(std::env::var_os("HOME").expect("Kani home")).join(".kani")
        });
    let instrument = kani_home.join(format!("kani-{version}/bin/goto-instrument"));
    let call_graph = Command::new(instrument)
        .arg("--call-graph")
        .arg(&linked)
        .output()
        .expect("installed goto-instrument must launch");
    assert!(call_graph.status.success(), "{}", output_text(&call_graph));
    let raw = String::from_utf8(call_graph.stdout).expect("UTF-8 call graph");
    fs::write(
        target.join(format!("ir19-call-graph-{harness_name}.txt")),
        &raw,
    )
    .expect("retain graph evidence");
    mapped_graph(&raw, &names)
}

fn mapped_graph(
    raw: &str,
    names: &BTreeMap<String, Option<String>>,
) -> BTreeMap<String, BTreeSet<String>> {
    let mut graph = BTreeMap::<String, BTreeSet<String>>::new();
    let mapped = |symbol: &str| {
        names
            .get(symbol)
            .and_then(Clone::clone)
            .unwrap_or_else(|| format!("#unresolved:{symbol}"))
    };
    for line in raw.lines() {
        let Some((caller, callee)) = line.split_once(" -> ") else {
            continue;
        };
        let caller = mapped(caller);
        let callee = mapped(callee);
        graph.entry(callee.clone()).or_default();
        graph.entry(caller).or_default().insert(callee);
    }
    for name in names.values().flatten() {
        graph.entry(name.clone()).or_default();
    }
    graph
}

fn reachable(graph: &BTreeMap<String, BTreeSet<String>>, root: &str) -> BTreeSet<String> {
    assert!(graph.contains_key(root), "missing compiled root {root}");
    let mut visited = BTreeSet::new();
    let mut queue = VecDeque::from([root.to_owned()]);
    while let Some(next) = queue.pop_front() {
        assert!(
            !next.starts_with("#unresolved:"),
            "unresolved reachable call: {next}"
        );
        if visited.insert(next.clone()) {
            queue.extend(graph[&next].iter().cloned());
        }
    }
    visited
}

fn replace_exactly_once(source: &str, mutation: &Mutation) -> String {
    assert_ne!(mutation.from, mutation.to, "unchanged mutant");
    assert_eq!(
        source.matches(&mutation.from).count(),
        1,
        "mutation site changed or missing"
    );
    source.replacen(&mutation.from, &mutation.to, 1)
}

fn mutation_line(source: &str, mutation: &Mutation) -> usize {
    let at = source.find(&mutation.from).expect("mutation site exists");
    source[..at].bytes().filter(|byte| *byte == b'\n').count() + 1
}

fn falsifying_mutant<'a>(
    original: &ClassifiedRun,
    mutant: &'a ClassifiedRun,
) -> Result<Vec<&'a KaniCheckResult>, String> {
    match &mutant.outcome {
        KaniRunOutcome::Falsified { counterexample }
            if counterexample.contains("kani::concrete_playback_run(") => {}
        outcome => {
            return Err(format!(
                "mutant did not yield concrete falsification: {outcome:?}"
            ))
        }
    }
    let failures: Vec<_> = mutant
        .checks
        .iter()
        .filter(|check| {
            check.status == KaniCheckStatus::Failure
                && original.checks.iter().any(|prior| {
                    prior.status == KaniCheckStatus::Success
                        && prior.location == check.location
                        && prior.class == check.class
                })
        })
        .collect();
    if failures.is_empty() {
        return Err("no newly failed proposition check".to_owned());
    }
    Ok(failures)
}

/// Trace: FR-041-AC-1, FR-041-AC-2, FR-041-AC-3, FR-041-AC-4, FR-041-AC-5, NFR-006-AC-19.
#[test]
#[ignore = "kani lane: run serially through make kani-gate"]
fn real_kani_claims_have_discharge_and_falsifying_mutations() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let declaration: Declaration = serde_json::from_slice(
        &fs::read(root.join(DECLARATION)).expect("checked-in module claims"),
    )
    .expect("module claim declaration must parse");
    let held = validate_claim_holds(&declaration);
    assert!(
        !declaration.modules.is_empty(),
        "claim declaration must retain its held row"
    );
    if held == declaration.modules.len() {
        let generated = &crate::kani_generation::guard_sources()[0].1;
        let modules = module_extents(generated);
        claim_census(&declaration, &base_names(), &BTreeSet::new(), &modules, &[])
            .unwrap_or_else(|error| panic!("{error}"));
        println!("kani-claims: active=0 held={held}");
        return;
    }
    let target = root.join("target-codex-backends");
    let directory = target.join("ir19-claim-crate");
    fs::create_dir_all(directory.join("src")).expect("scratch source directory");
    let generated = &crate::kani_generation::guard_sources()[0].1;
    let source = format!(
        "{generated}\npub fn subject(input: bool, pre_state: bool) -> bool {{ input || pre_state }}\n"
    );
    let modules = module_extents(&source);
    let functions = function_extents(&source);
    let source_file = directory.join("src/lib.rs");
    let original_file = directory.join("original.rs");
    fs::write(&original_file, &source).expect("retain original generated source");
    fs::write(&source_file, &source).expect("unmodified generated source");
    crate::scratch_crate::write_manifest(
        &directory,
        &format!(
            "[package]\nname = \"generated-kani-check\"\nversion = \"0.0.0\"\nedition = \"2021\"\n\n[dependencies]\n{}\n\n[workspace]\n",
            crate::scratch_crate::runtime_dependency(&["exact"])
        ),
    );
    fs::write(
        directory.join("build.rs"),
        "fn main() { println!(\"cargo:rustc-check-cfg=cfg(kani)\"); }\n",
    )
    .expect("Kani check-cfg declaration");
    let mut originals = Vec::new();
    for (index, claim) in declaration.modules.iter().enumerate() {
        if claim.hold.is_some() {
            continue;
        }
        let original_report = directory.join(format!("original-{index}-report.json"));
        let (output, bytes) = run_kani(&directory, &target, &original_report, &claim.harness);
        assert!(output.status.success(), "{}", output_text(&output));
        let classified = classify_kani_run(true, Some(&bytes), &output_text(&output), None)
            .expect("typed unmodified Kani report");
        assert!(matches!(classified.outcome, KaniRunOutcome::Verified));
        let report: Report = serde_json::from_slice(&bytes).expect("Kani harness metadata");
        assert_eq!(report.verification_results.summary.executed, 1);
        assert_eq!(report.verification_results.results.len(), 1);
        assert_eq!(
            report.verification_results.results[0].harness_id,
            claim.harness
        );
        originals.push((claim, classified, report));
    }
    let compiled: BTreeSet<_> = originals
        .iter()
        .flat_map(|(_, _, report)| &report.harness_metadata)
        .map(|harness| {
            assert_eq!(harness.source.file, "src/lib.rs");
            assert!(harness.source.start_line <= harness.source.end_line);
            harness
                .pretty_name
                .split_once("::")
                .expect("harness module qualification")
                .0
                .to_owned()
        })
        .collect();
    let checks: Vec<_> = originals
        .iter()
        .flat_map(|(_, classified, _)| classified.checks.iter().cloned())
        .collect();
    claim_census(&declaration, &base_names(), &compiled, &modules, &checks)
        .unwrap_or_else(|error| panic!("{error}"));
    let mut observed_shared = BTreeSet::new();
    for (claim, original, report) in originals {
        let graph = graph_for(&report, &target, &claim.harness);
        let subject = reachable(&graph, &claim.subject);
        let expectation = reachable(&graph, &claim.expectation);
        assert!(
            !expectation.contains(&claim.subject)
                && expectation
                    .iter()
                    .all(|name| !name.contains("quire_exact::")),
            "expectation must be independent of subject and quire-exact"
        );
        let shared: BTreeSet<_> = subject.intersection(&expectation).cloned().collect();
        fs::write(
            target.join(format!("ir19-shared-helpers-{}.json", claim.name)),
            serde_json::to_vec_pretty(&shared).expect("shared helper evidence"),
        )
        .expect("retain computed helper evidence");
        for name in &shared {
            assert!(
                declaration.helper_mutations.contains_key(name),
                "missing shared-helper mutation recipe: {name}"
            );
        }
        observed_shared.extend(shared.iter().cloned());
        let mut controls = vec![(claim.name.as_str(), &claim.mutation)];
        controls.extend(
            shared
                .iter()
                .map(|name| (name.as_str(), &declaration.helper_mutations[name])),
        );
        for (index, (name, mutation)) in controls.into_iter().enumerate() {
            let mutant = replace_exactly_once(&source, mutation);
            let extent = if index == 0 {
                modules.get(name)
            } else {
                functions.get(name)
            }
            .unwrap_or_else(|| panic!("mutation owner absent from generated source: {name}"));
            assert!(
                (extent.start..=extent.end).contains(&mutation_line(&source, mutation)),
                "mutation outside claimed owner: {name}"
            );
            fs::write(&source_file, &mutant).expect("write source mutant");
            let diff = Command::new("git")
                .args(["diff", "--no-index", "--"])
                .arg(&original_file)
                .arg(&source_file)
                .output()
                .expect("mutation diff must launch");
            let evidence = directory.join(format!("mutation-{}-{index}-{name}.txt", claim.name));
            fs::write(&evidence, output_text(&diff)).expect("retain exact mutant source");
            let report_path =
                directory.join(format!("mutation-{}-{index}-report.json", claim.name));
            let (output, bytes) = run_kani(&directory, &target, &report_path, &claim.harness);
            let classified = classify_kani_run(
                output.status.success(),
                Some(&bytes),
                &output_text(&output),
                None,
            )
            .expect("typed mutant Kani report");
            let mutant_report: Report =
                serde_json::from_slice(&bytes).expect("mutant execution metadata");
            assert_eq!(mutant_report.verification_results.summary.executed, 1);
            assert_eq!(
                mutant_report.verification_results.summary.status,
                "completed"
            );
            assert_eq!(mutant_report.verification_results.results.len(), 1);
            assert_eq!(
                mutant_report.verification_results.results[0].harness_id,
                claim.harness
            );
            assert_eq!(
                mutant_report.verification_results.results[0].status,
                "Failure"
            );
            let failures = falsifying_mutant(&original, &classified)
                .unwrap_or_else(|error| panic!("{name}: {error}"));
            let inconclusive = classify_kani_run(output.status.success(), Some(&bytes), "", None)
                .expect("same report without concrete playback");
            assert!(matches!(
                inconclusive.outcome,
                KaniRunOutcome::Inconclusive { .. }
            ));
            assert!(falsifying_mutant(&original, &inconclusive).is_err());
            fs::write(directory.join(format!("mutation-{}-{index}-command.txt", claim.name)),
                format!("cwd={} CARGO_TARGET_DIR={} /usr/bin/timeout 180s {} -Z function-contracts -Z unstable-options -Z concrete-playback --concrete-playback print --harness-timeout 120s --harness {} --exact --force-build --export-json {}\n{}\n{failures:#?}\n", directory.display(), target.display(), trusted_launcher().display(), claim.harness, report_path.display(), output_text(&output)))
                .expect("retain invocation and failing checks");
            fs::write(&source_file, &source).expect("restore source before next mutant");
        }
        let verifying = replace_exactly_once(
            &source,
            &Mutation {
                from: claim.mutation.from.clone(),
                to: format!("({}) || false", claim.mutation.from),
            },
        );
        fs::write(&source_file, verifying).expect("write verifying mutant");
        let verifying_report = directory.join(format!("negative-{}-verifying.json", claim.name));
        let (output, bytes) = run_kani(&directory, &target, &verifying_report, &claim.harness);
        assert!(output.status.success(), "{}", output_text(&output));
        let classified = classify_kani_run(true, Some(&bytes), &output_text(&output), None)
            .expect("typed verifying mutant");
        assert!(matches!(classified.outcome, KaniRunOutcome::Verified));
        assert!(falsifying_mutant(&original, &classified).is_err());

        fs::write(
            &source_file,
            format!("compile_error!(\"IR-19 negative control\");\n{source}"),
        )
        .expect("write compile-failing mutant");
        let compile_report = directory.join(format!("negative-{}-compile.json", claim.name));
        let (output, bytes) =
            execute_kani(&directory, &target, &compile_report, &claim.harness, "180s");
        assert!(
            !output.status.success() && bytes.is_none(),
            "compile-failing mutant acquired a report"
        );
        let classified = classify_kani_run(false, bytes.as_deref(), &output_text(&output), None)
            .expect("compile failure is inconclusive");
        assert!(falsifying_mutant(&original, &classified).is_err());

        fs::write(&source_file, &source).expect("restore source before timeout control");
        let timeout_report = directory.join(format!("negative-{}-timeout.json", claim.name));
        let (output, bytes) = execute_kani(
            &directory,
            &target,
            &timeout_report,
            &claim.harness,
            "0.01s",
        );
        assert!(
            output.status.code() == Some(124) && bytes.is_none(),
            "timed-out mutant acquired a verdict: {}",
            output_text(&output)
        );
        let classified = classify_kani_run(false, bytes.as_deref(), &output_text(&output), None)
            .expect("timeout is inconclusive");
        assert!(falsifying_mutant(&original, &classified).is_err());
        assert!(classify_kani_run(true, None, "skipped execution", None).is_err());
    }
    let catalog: BTreeSet<_> = declaration.helper_mutations.keys().cloned().collect();
    assert_eq!(observed_shared, catalog, "stale helper mutation recipe");
    println!("Kani claim evidence: {}", directory.display());
}

/// Trace: FR-041-AC-2.
#[test]
fn a_success_in_one_module_does_not_discharge_an_unreachable_neighbor() {
    use quire_contract_codegen::{KaniCheckClass, KaniCheckLocation};
    let declaration = Declaration {
        modules: ["first", "second"]
            .into_iter()
            .map(|name| Claim {
                name: name.to_owned(),
                harness: String::new(),
                subject: String::new(),
                expectation: String::new(),
                mutation: Mutation {
                    from: String::new(),
                    to: String::new(),
                },
                hold: None,
            })
            .collect(),
        helper_mutations: BTreeMap::new(),
    };
    let compiled = ["first", "second"].into_iter().map(str::to_owned).collect();
    let extents = BTreeMap::from([
        ("first".to_owned(), Extent { start: 1, end: 4 }),
        ("second".to_owned(), Extent { start: 5, end: 8 }),
    ]);
    let at = |line: u32, status| KaniCheckResult {
        id: line.into(),
        class: KaniCheckClass::from("assertion".to_owned()),
        location: KaniCheckLocation {
            file: "src/lib.rs".to_owned(),
            line: Some(line),
        },
        status,
    };
    let checks = [
        at(3, KaniCheckStatus::Success),
        at(7, KaniCheckStatus::Unreachable),
    ];
    assert_eq!(
        claim_census(&declaration, &BTreeSet::new(), &compiled, &extents, &checks),
        Err("unreached claimed module: second".to_owned())
    );
    let checks = [
        at(3, KaniCheckStatus::Success),
        at(7, KaniCheckStatus::Success),
    ];
    assert!(claim_census(&declaration, &BTreeSet::new(), &compiled, &extents, &checks).is_ok());
    for status in [
        KaniCheckStatus::Satisfied,
        KaniCheckStatus::Unknown,
        KaniCheckStatus::Failure,
    ] {
        let checks = [at(3, KaniCheckStatus::Success), at(7, status)];
        assert_eq!(
            claim_census(&declaration, &BTreeSet::new(), &compiled, &extents, &checks),
            Err("unreached claimed module: second".to_owned())
        );
    }
    let checks = [
        at(3, KaniCheckStatus::Success),
        KaniCheckResult {
            location: KaniCheckLocation {
                file: "unknown".to_owned(),
                line: None,
            },
            ..at(7, KaniCheckStatus::Success)
        },
    ];
    assert_eq!(
        claim_census(&declaration, &BTreeSet::new(), &compiled, &extents, &checks),
        Err("unreached claimed module: second".to_owned())
    );
}

/// Trace: FR-041-AC-1.
#[test]
fn empty_removed_and_absent_compiled_claims_fail() {
    let claim = || Claim {
        name: "claimed".to_owned(),
        harness: String::new(),
        subject: String::new(),
        expectation: String::new(),
        mutation: Mutation {
            from: String::new(),
            to: String::new(),
        },
        hold: None,
    };
    let mut declaration = Declaration {
        modules: Vec::new(),
        helper_mutations: BTreeMap::new(),
    };
    let empty = BTreeSet::new();
    let extents = BTreeMap::new();
    assert_eq!(
        claim_census(&declaration, &empty, &empty, &extents, &[]),
        Err("empty claimed-module declaration".to_owned())
    );
    declaration.modules.push(claim());
    assert_eq!(
        claim_census(&declaration, &empty, &empty, &extents, &[]),
        Err("absent compiled module: claimed".to_owned())
    );
    declaration.modules[0].name = "renamed".to_owned();
    let base = ["claimed".to_owned()].into_iter().collect();
    assert_eq!(
        claim_census(&declaration, &base, &empty, &extents, &[]),
        Err("removed claimed module(s): claimed".to_owned())
    );
}

/// An absent base declaration differs from an unreadable blob.
#[trace("FR-041-AC-1")]
#[test]
fn base_claims_allow_proven_absence_but_refuse_failed_object_read() {
    let scratch = crate::scratch_crate::TemporaryDirectory::new("quire-kani-claims-base");
    let directory = &scratch.path;
    let git = |args: &[&str]| {
        let output = Command::new("git")
            .arg("-C")
            .arg(directory)
            .args(args)
            .output()
            .expect("scratch Git must launch");
        assert!(output.status.success(), "{}", output_text(&output));
        String::from_utf8(output.stdout).expect("UTF-8 Git output")
    };
    git(&["init", "-q"]);
    git(&[
        "-c",
        "user.name=IR-19",
        "-c",
        "user.email=ir19@example.invalid",
        "commit",
        "--allow-empty",
        "-qm",
        "empty base",
    ]);
    let absent_base = git(&["rev-parse", "HEAD"]);
    assert_eq!(
        base_names_at(directory, absent_base.trim()),
        Ok(BTreeSet::new())
    );

    fs::create_dir_all(directory.join("scripts")).expect("scratch declaration directory");
    fs::write(
        directory.join(DECLARATION),
        r#"{"modules":[{"name":"claimed","harness":"claimed::proof","subject":"subject","expectation":"oracle","mutation":{"from":"a","to":"b"}}],"helper_mutations":{}}"#,
    )
    .expect("scratch declaration");
    git(&["add", DECLARATION]);
    git(&[
        "-c",
        "user.name=IR-19",
        "-c",
        "user.email=ir19@example.invalid",
        "commit",
        "-qm",
        "declaration base",
    ]);
    let declared_base = git(&["rev-parse", "HEAD"]);
    assert_eq!(
        base_names_at(directory, declared_base.trim()),
        Ok(BTreeSet::from(["claimed".to_owned()]))
    );
    let blob = git(&["rev-parse", &format!("HEAD:{DECLARATION}")]);
    let blob = blob.trim();
    fs::remove_file(
        directory
            .join(".git/objects")
            .join(&blob[..2])
            .join(&blob[2..]),
    )
    .expect("remove own loose blob to simulate a failed object read");
    assert!(base_names_at(directory, declared_base.trim())
        .expect_err("unreadable declared blob must fail closed")
        .contains("cannot read base claim declaration"));
}

/// Trace: FR-041-AC-3.
#[test]
fn transitive_graph_intersection_finds_shared_helper() {
    let graph = BTreeMap::from([
        ("subject".to_owned(), BTreeSet::from(["left".to_owned()])),
        ("left".to_owned(), BTreeSet::from(["helper".to_owned()])),
        (
            "expectation".to_owned(),
            BTreeSet::from(["right".to_owned()]),
        ),
        ("right".to_owned(), BTreeSet::from(["helper".to_owned()])),
        ("helper".to_owned(), BTreeSet::new()),
    ]);
    let subject = reachable(&graph, "subject");
    let expectation = reachable(&graph, "expectation");
    assert_eq!(
        subject
            .intersection(&expectation)
            .cloned()
            .collect::<BTreeSet<_>>(),
        BTreeSet::from(["helper".to_owned()])
    );
}

/// Trace: FR-041-AC-3. A reachable GOTO edge without a pretty name is opaque.
#[test]
fn current_graph_refuses_unmapped_reachable_call() {
    let names = BTreeMap::from([("root-id".to_owned(), Some("root".to_owned()))]);
    let graph = mapped_graph("root-id -> missing-id\n", &names);
    assert_eq!(
        graph["root"],
        BTreeSet::from(["#unresolved:missing-id".to_owned()])
    );
    assert!(std::panic::catch_unwind(|| reachable(&graph, "root")).is_err());
}

/// Trace: FR-041-AC-2, TC-056. Separate real harnesses share one source file.
#[test]
#[ignore = "kani lane: run serially through make kani-gate"]
fn real_kani_two_module_check_attribution() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let target = root.join("target-codex-backends");
    let directory = target.join("ir19-two-module-crate");
    fs::create_dir_all(directory.join("src")).expect("two-module crate directory");
    let source = "mod first {\n  #[kani::proof]\n  fn proof() { let x: bool = kani::any(); assert!(x || !x); }\n}\nmod second {\n  #[kani::proof]\n  fn proof() { kani::assume(false); assert!(true); }\n}\n";
    fs::write(directory.join("src/lib.rs"), source).expect("two-module source");
    fs::write(
        directory.join("Cargo.toml"),
        "[package]\nname = \"ir19-two-module-control\"\nversion = \"0.0.0\"\nedition = \"2021\"\n[workspace]\n",
    )
    .expect("two-module manifest");
    let declaration = Declaration {
        modules: ["first", "second"]
            .into_iter()
            .map(|name| Claim {
                name: name.to_owned(),
                harness: format!("{name}::proof"),
                subject: String::new(),
                expectation: String::new(),
                mutation: Mutation {
                    from: String::new(),
                    to: String::new(),
                },
                hold: None,
            })
            .collect(),
        helper_mutations: BTreeMap::new(),
    };
    let mut checks = Vec::new();
    let mut compiled = BTreeSet::new();
    for claim in &declaration.modules {
        let report_path = directory.join(format!("{}-report.json", claim.name));
        let (output, bytes) = run_kani(&directory, &target, &report_path, &claim.harness);
        let classified = classify_kani_run(
            output.status.success(),
            Some(&bytes),
            &output_text(&output),
            None,
        )
        .expect("typed two-module report");
        let report: Report = serde_json::from_slice(&bytes).expect("two-module metadata");
        assert_eq!(report.verification_results.summary.executed, 1);
        assert_eq!(
            report.verification_results.results[0].harness_id,
            claim.harness
        );
        compiled.extend(report.harness_metadata.iter().map(|harness| {
            assert_eq!(harness.source.file, "src/lib.rs");
            harness
                .pretty_name
                .split_once("::")
                .expect("module harness")
                .0
                .to_owned()
        }));
        checks.extend(classified.checks);
    }
    assert_eq!(
        claim_census(
            &declaration,
            &BTreeSet::new(),
            &compiled,
            &module_extents(source),
            &checks
        ),
        Err("unreached claimed module: second".to_owned())
    );
    let corrected = source.replace(
        "kani::assume(false); assert!(true);",
        "let x: bool = kani::any(); assert!(x || !x);",
    );
    fs::write(directory.join("src/lib.rs"), &corrected).expect("reachable second module");
    let report_path = directory.join("second-reachable-report.json");
    let (output, bytes) = run_kani(&directory, &target, &report_path, "second::proof");
    assert!(output.status.success(), "{}", output_text(&output));
    let classified = classify_kani_run(true, Some(&bytes), &output_text(&output), None)
        .expect("reachable second report");
    let first_checks: Vec<_> = checks
        .into_iter()
        .filter(|check| check.location.line == Some(3))
        .collect();
    let all_checks: Vec<_> = first_checks.into_iter().chain(classified.checks).collect();
    assert!(claim_census(
        &declaration,
        &BTreeSet::new(),
        &compiled,
        &module_extents(&corrected),
        &all_checks
    )
    .is_ok());
}
