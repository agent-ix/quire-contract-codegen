//! Current-build module claims and real mutation controls (FR-041, TC-056).

use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
};

use quire_contract_codegen::{classify_kani_run, KaniCheckResult, KaniCheckStatus, KaniRunOutcome};
use serde::Deserialize;
use syn::{spanned::Spanned, Item};

const DECLARATION: &str = "scripts/kani_claims.json";

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
    subject: String,
    expectation: String,
    mutation: Mutation,
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

fn run_kani(directory: &Path, target: &Path, report: &Path) -> (Output, Vec<u8>) {
    let _ = fs::remove_file(report);
    let output = Command::new("/usr/bin/timeout")
        .args([
            "180s",
            "cargo",
            "kani",
            "-Z",
            "function-contracts",
            "-Z",
            "unstable-options",
            "--harness-timeout",
            "120s",
            "--force-build",
            "--export-json",
        ])
        .arg(report)
        .env("CARGO_TARGET_DIR", target)
        .current_dir(directory)
        .output()
        .expect("real cargo kani must launch");
    let bytes = fs::read(report).unwrap_or_else(|error| {
        panic!(
            "Kani produced no report ({error}): {}",
            output_text(&output)
        )
    });
    (output, bytes)
}

fn base_names() -> BTreeSet<String> {
    let base = Command::new("git")
        .args(["merge-base", "HEAD", "origin/main"])
        .output()
        .expect("git merge-base must launch");
    assert!(base.status.success(), "{}", output_text(&base));
    let base = String::from_utf8(base.stdout).expect("UTF-8 commit");
    let object = format!("{}:{}", base.trim(), DECLARATION);
    let previous = Command::new("git")
        .args(["show", &object])
        .output()
        .expect("git show must launch");
    if !previous.status.success() {
        return BTreeSet::new();
    }
    let declaration: Declaration =
        serde_json::from_slice(&previous.stdout).expect("base claim declaration must parse");
    declaration
        .modules
        .into_iter()
        .map(|claim| claim.name)
        .collect()
}

fn graph_for(report: &Report, target: &Path) -> BTreeMap<String, BTreeSet<String>> {
    let harness = report.harness_metadata.first().expect("compiled harness");
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
    let version = Command::new("cargo-kani")
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
    fs::write(target.join("ir19-call-graph.txt"), &raw).expect("retain graph evidence");
    let mut graph = BTreeMap::<String, BTreeSet<String>>::new();
    for line in raw.lines() {
        let Some((caller, callee)) = line.split_once(" -> ") else {
            continue;
        };
        let caller = names
            .get(caller)
            .map(|name| {
                name.clone()
                    .unwrap_or_else(|| format!("#unresolved:{caller}"))
            })
            .unwrap_or_else(|| caller.to_owned());
        let callee = names
            .get(callee)
            .map(|name| {
                name.clone()
                    .unwrap_or_else(|| format!("#unresolved:{callee}"))
            })
            .unwrap_or_else(|| callee.to_owned());
        graph.entry(callee.clone()).or_default();
        graph.entry(caller).or_default().insert(callee);
    }
    for name in names.into_values().flatten() {
        graph.entry(name).or_default();
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

/// Trace: FR-041-AC-1, FR-041-AC-2, FR-041-AC-3, FR-041-AC-4, FR-041-AC-5, NFR-006-AC-19.
#[test]
#[ignore = "kani lane: run serially through make kani-gate"]
fn real_kani_claims_have_discharge_and_falsifying_mutations() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let declaration: Declaration = serde_json::from_slice(
        &fs::read(root.join(DECLARATION)).expect("checked-in module claims"),
    )
    .expect("module claim declaration must parse");
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
    let original_report = directory.join("original-report.json");
    let (original_output, original_bytes) = run_kani(&directory, &target, &original_report);
    assert!(
        original_output.status.success(),
        "{}",
        output_text(&original_output)
    );
    let original = classify_kani_run(
        true,
        Some(&original_bytes),
        &output_text(&original_output),
        None,
    )
    .expect("typed unmodified Kani report");
    assert!(matches!(original.outcome, KaniRunOutcome::Verified));
    let report: Report = serde_json::from_slice(&original_bytes).expect("Kani harness metadata");
    let compiled: BTreeSet<_> = report
        .harness_metadata
        .iter()
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
    claim_census(
        &declaration,
        &base_names(),
        &compiled,
        &modules,
        &original.checks,
    )
    .unwrap_or_else(|error| panic!("{error}"));
    let graph = graph_for(&report, &target);
    for claim in &declaration.modules {
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
            target.join("ir19-shared-helpers.json"),
            serde_json::to_vec_pretty(&shared).expect("shared helper evidence"),
        )
        .expect("retain computed helper evidence");
        let expected_mutations: BTreeSet<_> =
            declaration.helper_mutations.keys().cloned().collect();
        assert_eq!(
            shared, expected_mutations,
            "missing or stale helper mutation recipe"
        );
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
            let evidence = directory.join(format!("mutation-{index}-{name}.txt"));
            fs::write(&evidence, output_text(&diff)).expect("retain exact mutant source");
            let report_path = directory.join(format!("mutation-{index}-report.json"));
            let (output, bytes) = run_kani(&directory, &target, &report_path);
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
                report.harness_metadata[0].pretty_name
            );
            assert_eq!(
                mutant_report.verification_results.results[0].status,
                "Failure"
            );
            let failures: Vec<_> = classified
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
            assert!(
                !failures.is_empty(),
                "{name}: no newly failed proposition check"
            );
            fs::write(directory.join(format!("mutation-{index}-command.txt")),
                format!("cwd={} CARGO_TARGET_DIR={} /usr/bin/timeout 180s cargo kani -Z function-contracts -Z unstable-options --harness-timeout 120s --force-build --export-json {}\n{}\n{failures:#?}\n", directory.display(), target.display(), report_path.display(), output_text(&output)))
                .expect("retain invocation and failing checks");
            fs::write(&source_file, &source).expect("restore source before next mutant");
        }
    }
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
                subject: String::new(),
                expectation: String::new(),
                mutation: Mutation {
                    from: String::new(),
                    to: String::new(),
                },
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
        subject: String::new(),
        expectation: String::new(),
        mutation: Mutation {
            from: String::new(),
            to: String::new(),
        },
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
