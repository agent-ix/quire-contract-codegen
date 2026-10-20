//! Private orchestration of the AC106 original-domain / actual-sampler controls.
//! No production API is exposed for the carrier or bound mutations.
#[path = "../graph_support/fixture.rs"]
mod fixture;
#[path = "../graph_support/model.rs"]
mod model;
include!("../graph_support/domain_predicate.rs");
#[path = "../graph_support/sampler.rs"]
mod sampler;
use crate::kani::{
    classify::KaniRunOutcome,
    generate::{
        graph::{bound_mutant_for_test, BoundMutation},
        negotiate::negotiate_kani_obligations,
        outcome::{KaniObligationOutcome, KaniObligationRequest, ObligationItem},
    },
    identity::KaniGraphObligationHarness,
    test_support::proof_ceilings::proof_ceilings_with_wall_clock,
};
use fixture::AuthoredGraph;
use ix_trace_rs::trace;
use quire_contract_model::{
    selected_population::{
        select_population, CheckedSelectedPopulationOutcome, CheckedSelectedPopulationSource,
    },
    CheckedNodeId,
};
use sampler::SamplerObservation;
use serde::Serialize;
use serde_json::{json, Value};
use std::{fs, path::PathBuf, process::Command, time::Duration};

struct Original {
    fixture: AuthoredGraph,
    clause: CheckedNodeId,
    application: CheckedNodeId,
    population: CheckedNodeId,
    edge_declaration: CheckedNodeId,
    role: String,
    domain: AuthoredDomain,
}
impl Original {
    fn new(role: &str) -> Self {
        let fixture = fixture::try_fixture_source(
            fixture::ModelInput {
                document: model::model_document(),
                incoming_upper: 20,
            },
            role,
            "reaches(self,self,EDGE) = reaches(self,self,EDGE)",
            4,
            true,
            "invariant",
            "",
        )
        .unwrap();
        // D is fixed from the original authored inputs and checked projections BEFORE the
        // generator (healthy or mutant) is invoked. No harness metadata supplies D.
        let declared_max = fixture
            .source
            .lines()
            .find_map(|line| {
                line.strip_prefix("bound Nodes on M::CollectionNodes [")?
                    .strip_suffix(']')?
                    .parse::<u64>()
                    .ok()
            })
            .unwrap();
        assert_eq!(declared_max, 4);
        let document: Value = serde_json::from_slice(&fixture.model_bytes).unwrap();
        let authored = document["types"][1]["relationships"]
            .as_array()
            .unwrap()
            .iter()
            .find(|relation| relation["sourceEnd"]["role"] == role)
            .unwrap();
        assert_eq!(authored["sourceEnd"]["type"], authored["targetEnd"]["type"]);
        assert_eq!(
            authored["sourceEnd"]["multiplicity"]["upper"], 20,
            "the inverse admits every edge of the full5x4 union carrier"
        );
        assert_eq!(
            document["populations"][0]["members"][0],
            authored["sourceEnd"]["type"]
        );
        let multiplicity = &authored["targetEnd"]["multiplicity"];
        let lower = multiplicity["lower"].as_u64().unwrap();
        let upper = multiplicity["upper"].as_u64().unwrap();
        let unique = multiplicity["unique"].as_bool().unwrap();
        assert_eq!((lower, upper, unique), (0, 3, role == "kids"));
        let clause = fixture
            .package
            .graph()
            .nodes
            .iter()
            .find(|node| node.node_id.digest.as_ref() == fixture.site.node.to_string())
            .unwrap()
            .node_id
            .clone();
        let application = fixture
            .package
            .graph()
            .nodes
            .iter()
            .find(|node| {
                node.body
                    .pointer("/operation/identity")
                    .and_then(Value::as_str)
                    == Some("quire.op.model.reaches")
                    && node
                        .body
                        .pointer("/operation/member/name")
                        .and_then(Value::as_str)
                        == Some(role)
            })
            .unwrap()
            .node_id
            .clone();
        let selector = fixture
            .package
            .reaches_population(&application)
            .unwrap()
            .unwrap();
        let CheckedSelectedPopulationOutcome::Finite(population) =
            select_population(CheckedSelectedPopulationSource::Admitted {
                package: &fixture.package,
                selector,
            })
        else {
            panic!("original authored finite population projection")
        };
        let end = fixture.package.relationship_end(&application).unwrap();
        assert_eq!((population.minimum, population.maximum), (0, declared_max));
        assert_eq!(
            (end.lower, end.upper, end.unique, end.ordered),
            (lower, Some(upper), unique, false)
        );
        assert_eq!(population.object_type, end.destination_type);
        let body = &fixture
            .package
            .graph()
            .nodes
            .iter()
            .find(|node| node.node_id == application)
            .unwrap()
            .body;
        assert_eq!(
            body["arguments"][0], body["arguments"][1],
            "original self operands coincide"
        );
        let relationship: CheckedNodeId =
            serde_json::from_value(body["operation"]["member"]["declaration"].clone()).unwrap();
        assert_eq!(end.relationship, relationship);
        Self {
            fixture,
            clause,
            application,
            population: population.binding,
            edge_declaration: relationship,
            role: role.into(),
            domain: AuthoredDomain {
                maximum: i64::try_from(declared_max).unwrap(),
                lower: i64::try_from(lower).unwrap(),
                upper: i64::try_from(upper).unwrap(),
                unique,
                same_operand: true,
            },
        }
    }
    fn generated(&self, mutation: Option<BoundMutation>) -> KaniGraphObligationHarness {
        let items = [ObligationItem::GraphClaim {
            package: &self.fixture.package,
            clause_id: &self.clause,
            argument_path: &[0],
        }];
        let request = KaniObligationRequest {
            ceilings: proof_ceilings_with_wall_clock(Duration::from_secs(600)),
            items: &items,
            subject_path: "crate::graph_subject",
            unwind: 8,
        };
        let generated = if let Some(mutation) = mutation {
            bound_mutant_for_test(
                &request,
                &self.fixture.package,
                &self.clause,
                &[0],
                mutation,
            )
        } else {
            let KaniObligationOutcome::Emitted {
                mut graph_harnesses,
                ..
            } = negotiate_kani_obligations(&request).unwrap()
            else {
                panic!("healthy original request")
            };
            assert_eq!(graph_harnesses.len(), 1);
            graph_harnesses.remove(0)
        };
        assert_eq!(generated.identity.clause_id, self.clause);
        assert_eq!(generated.identity.application_id, self.application);
        assert_eq!(generated.identity.argument_path, [0]);
        assert_eq!(generated.identity.population_id, self.population);
        assert_eq!(generated.identity.edge.declaration, self.edge_declaration);
        generated
    }
}
fn boundary(count: i64, degree: i64, unique: bool) -> CandidateGraph {
    let mut graph = CandidateGraph {
        count,
        start: 0,
        target: 0,
        degrees: [0; 5],
        edges: [[0; 4]; 5],
    };
    graph.degrees[0] = degree;
    if unique {
        for slot in 0..usize::try_from(degree).unwrap() {
            graph.edges[0][slot] = i64::try_from(slot).unwrap();
        }
    }
    graph
}

#[derive(Serialize)]
struct DomainWitness<'a> {
    classification: &'a str,
    property: &'a str,
    clause: &'a CheckedNodeId,
    argument_path: [usize; 1],
    application: &'a CheckedNodeId,
    population: &'a CheckedNodeId,
    edge_declaration: &'a CheckedNodeId,
    role: &'a str,
    count: i64,
    start: Option<i64>,
    target: Option<i64>,
    degrees: Vec<i64>,
    targets: Vec<Vec<i64>>,
    original_bounds: (i64, i64, i64, bool),
    generated_domain: &'a SamplerObservation,
    d: bool,
    g: bool,
}
fn witness<'a>(
    original: &'a Original,
    observed: &'a SamplerObservation,
    graph: &CandidateGraph,
    property: &'a str,
) -> DomainWitness<'a> {
    let d = authored_admits(graph, original.domain);
    let g = observed.admits(graph);
    let count = usize::try_from(graph.count).unwrap();
    assert!(count <= 5);
    let degrees = graph.degrees[..count].to_vec();
    let targets = (0..count)
        .map(|row| graph.edges[row][..usize::try_from(graph.degrees[row]).unwrap()].to_vec())
        .collect();
    DomainWitness {
        classification: match (d, g) {
            (true, false) => "omitted-original-valid-completeness",
            (false, true) => "mutant-only-conformance",
            (true, true) => "common-domain-boundary",
            _ => "rejected-by-both",
        },
        property,
        clause: &original.clause,
        argument_path: [0],
        application: &original.application,
        population: &original.population,
        edge_declaration: &original.edge_declaration,
        role: &original.role,
        count: graph.count,
        start: (count > 0).then_some(graph.start),
        target: (count > 0).then_some(graph.target),
        degrees,
        targets,
        original_bounds: (
            original.domain.maximum,
            original.domain.lower,
            original.domain.upper,
            original.domain.unique,
        ),
        generated_domain: observed,
        d,
        g,
    }
}
fn probe_condition(graph: &CandidateGraph, d: bool, g: bool) -> String {
    let mut conditions = vec![
        format!("d == {d}"),
        format!("generated == {g}"),
        format!("candidate.count == {}", graph.count),
        "candidate.start == 0".into(),
        "candidate.target == 0".into(),
    ];
    for row in 0..usize::try_from(graph.count).unwrap() {
        conditions.push(format!(
            "candidate.degrees[{row}] == {}",
            graph.degrees[row]
        ));
        for slot in 0..usize::try_from(graph.degrees[row]).unwrap() {
            conditions.push(format!(
                "candidate.edges[{row}][{slot}] == {}",
                graph.edges[row][slot]
            ));
        }
    }
    conditions.join(" && ")
}
fn oracle_source(
    original: &Original,
    sampler: &KaniGraphObligationHarness,
    observed: &SamplerObservation,
    assertions: bool,
    cover: &str,
) -> String {
    let mut body = String::new();
    // The complete union carrier is independent of D/G. Every count0..5, degree0..4,
    // target0..4 and operand0..4 is drawn; there are no membership assumptions.
    for (name, maximum) in [("object_count", 5), ("start", 4), ("target", 4)] {
        body.push_str(&format!(
            "let {name}: i64 = kani::any(); kani::assume({name} >= 0 && {name} <= {maximum});\n"
        ));
    }
    for row in 0..5 {
        body.push_str(&format!("let degree_{row}: i64 = kani::any(); kani::assume(degree_{row} >= 0 && degree_{row} <= 4);\n"));
        for slot in 0..4 {
            body.push_str(&format!("let edge_{row}_{slot}: i64 = kani::any(); kani::assume(edge_{row}_{slot} >= 0 && edge_{row}_{slot} <= 4);\n"));
        }
    }
    let degrees = (0..5)
        .map(|r| format!("degree_{r}"))
        .collect::<Vec<_>>()
        .join(",");
    let rows = (0..5)
        .map(|r| {
            format!(
                "[{}]",
                (0..4)
                    .map(|s| format!("edge_{r}_{s}"))
                    .collect::<Vec<_>>()
                    .join(",")
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    body.push_str(&format!("let candidate = CandidateGraph {{ count: object_count, start, target, degrees: [{degrees}], edges: [{rows}] }};\n"));
    let d = original.domain;
    body.push_str(&format!("let d = authored_admits(&candidate, AuthoredDomain {{ maximum: {}, lower: {}, upper: {}, unique: {}, same_operand: {} }});\nlet generated = generated_admits(&candidate);\n",d.maximum,d.lower,d.upper,d.unique,d.same_operand));
    if assertions {
        body.push_str("assert!(!generated || d, \"generated graph domain conforms to original authored domain\");\nassert!(!d || generated, \"generated graph domain includes every original authored graph\");\n");
    }
    body.push_str(&format!(
        "kani::cover!({cover}, \"domain carrier boundary witness\");\n"
    ));
    let source = format!("// SPDX-License-Identifier: MIT OR Apache-2.0\n// Private AC106 domain oracle; production reachability is not asserted here.\n{}\n{}\n#[cfg(kani)]\nmod {} {{ use super::*; #[kani::proof] fn {}() {{ {body} }} }}\n",
        include_str!("../graph_support/domain_predicate.rs").replacen("//!", "//", 1), observed.source(), sampler.identity.module_symbol, sampler.identity.harness_symbol);
    let parsed = syn::parse_file(&source).unwrap();
    struct Assumptions(usize, usize);
    impl<'a> syn::visit::Visit<'a> for Assumptions {
        fn visit_expr_call(&mut self, call: &'a syn::ExprCall) {
            use quote::ToTokens;
            if call.func.to_token_stream().to_string() == "kani :: assume" {
                self.0 += 1;
            }
            syn::visit::visit_expr_call(self, call);
        }
        fn visit_macro(&mut self, mac: &'a syn::Macro) {
            if mac.path.segments.last().is_some_and(|p| p.ident == "cover") {
                self.1 += 1;
            }
            syn::visit::visit_macro(self, mac);
        }
    }
    let mut count = Assumptions(0, 0);
    syn::visit::Visit::visit_file(&mut count, &parsed);
    assert_eq!(
        (count.0, count.1),
        (28, 1),
        "only the carrier scalar ranges may filter candidates; one final cover"
    );
    source
}

#[derive(Serialize)]
struct DomainExecutionEvidence {
    purpose: &'static str,
    launcher_path: String,
    arguments: Vec<String>,
    unwind: u32,
    solver: String,
    exit_code: Option<i32>,
    outcome: KaniRunOutcome,
    checks: Vec<crate::kani::output::report::KaniCheckResult>,
    success_checks: u32,
    tool_versions: Value,
    solver_version_observation: String,
    outer_scope_before: Value,
    outer_scope_after: Value,
}
struct DomainRun {
    evidence: DomainExecutionEvidence,
    console: String,
}
fn evidence_directory() -> PathBuf {
    // Library tests do not have CARGO_TARGET_TMPDIR; use the explicitly owned build target.
    PathBuf::from(
        std::env::var_os("CARGO_TARGET_DIR").expect("explicit own target for domain controls"),
    )
    .join("tmp/kani-graph-domain-evidence")
}
fn outer_scope_observation() -> Value {
    let membership =
        fs::read_to_string("/proc/self/cgroup").expect("actual outer cgroup membership");
    let path = membership
        .lines()
        .find_map(|line| line.strip_prefix("0::"))
        .expect("unified cgroup scope");
    let directory = PathBuf::from("/sys/fs/cgroup").join(path.trim_start_matches('/'));
    let read = |name| {
        fs::read_to_string(directory.join(name))
            .ok()
            .map(|s| s.trim().to_owned())
    };
    json!({"membership": path, "memory_max": read("memory.max"), "swap_max": read("memory.swap.max"),
        "memory_current": read("memory.current"), "scope_memory_peak": read("memory.peak"),
        "observed_unix_millis": std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(),
        "meaning": "actual enclosing test scope; not an isolated per-backend memory attestation"})
}
fn run_domain(sampler: &KaniGraphObligationHarness, source: &str, label: &str) -> DomainRun {
    let directory = evidence_directory().join(label);
    fs::create_dir_all(directory.join("src")).unwrap();
    fs::write(directory.join("Cargo.toml"),"[package]\nname = \"graph-domain-control\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n[workspace]\n").unwrap();
    fs::write(directory.join("src/lib.rs"), source).unwrap();
    fs::write(directory.join("sampler.rs"), &sampler.rust.contents).unwrap();
    fs::write(
        directory.join("build.rs"),
        "fn main() { println!(\"cargo:rustc-check-cfg=cfg(kani)\"); }\n",
    )
    .unwrap();
    let launcher = std::env::var_os("KANI_GATE_LAUNCHER").unwrap_or_else(|| "cargo-kani".into());
    let launcher_path = if PathBuf::from(&launcher).is_absolute() {
        PathBuf::from(&launcher)
    } else {
        std::env::split_paths(&std::env::var_os("PATH").unwrap())
            .map(|p| p.join(&launcher))
            .find(|p| p.is_file())
            .expect("installed cargo-kani")
    };
    let report_path = directory.join("report.json");
    let _ = fs::remove_file(&report_path);
    let mut arguments = vec!["kani".into()];
    arguments.extend(sampler.identity.options.clone());
    arguments.extend([
        "-Z".into(),
        "unstable-options".into(),
        "--export-json".into(),
        report_path.to_string_lossy().into_owned(),
    ]);
    let outer_scope_before = outer_scope_observation();
    fs::write(
        directory.join("outer-scope-before.json"),
        serde_json::to_vec_pretty(&outer_scope_before).unwrap(),
    )
    .unwrap();
    let output = Command::new("timeout")
        .arg("600")
        .arg(&launcher_path)
        .args(&arguments)
        .env(
            "CARGO_TARGET_DIR",
            evidence_directory().join("backend-target"),
        )
        .current_dir(&directory)
        .output()
        .expect("actual installed domain control");
    let outer_scope_after = outer_scope_observation();
    fs::write(
        directory.join("outer-scope-after.json"),
        serde_json::to_vec_pretty(&outer_scope_after).unwrap(),
    )
    .unwrap();
    let console = format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    fs::write(directory.join("console.log"), &console).unwrap();
    let bytes = fs::read(&report_path)
        .unwrap_or_else(|e| panic!("{label}: no actual Kani report: {e}: {console}"));
    let raw: Value = serde_json::from_slice(&bytes).unwrap();
    assert!(raw["tools"]["kani"].as_str().is_some());
    assert!(raw["tools"]["rustc"].as_str().is_some());
    assert!(raw["tools"]["cbmc"].as_str().is_some());
    let solver_version_observation = console
        .lines()
        .find(|line| line.starts_with("Solving with CaDiCaL "))
        .expect("the installed solver must report its actual version")
        .to_owned();
    let classified = crate::kani::classify::classify_kani_run(
        output.status.success(),
        Some(&bytes),
        &console,
        None,
    )
    .unwrap();
    let evidence = DomainExecutionEvidence {
        purpose: "separate AC106 domain oracle; no original-clause truth claim",
        launcher_path: launcher_path.to_string_lossy().into_owned(),
        arguments,
        unwind: sampler.identity.unwind,
        solver: "cadical".into(),
        exit_code: output.status.code(),
        outcome: classified.outcome,
        checks: classified.checks,
        success_checks: classified.success_checks,
        tool_versions: raw["tools"].clone(),
        solver_version_observation,
        outer_scope_before,
        outer_scope_after,
    };
    fs::write(
        directory.join("execution-evidence.json"),
        serde_json::to_vec_pretty(&evidence).unwrap(),
    )
    .unwrap();
    DomainRun { evidence, console }
}
fn decode(console: &str, property: &str) -> CandidateGraph {
    let (_, console) = console
        .split_once(property)
        .expect("backend property identity");
    let (_, rest) = console
        .split_once("let concrete_vals: Vec<Vec<u8>> = vec![")
        .expect("actual backend boundary playback");
    let body = rest
        .split_once("]; ")
        .or_else(|| rest.split_once("];\n"))
        .unwrap()
        .0;
    let values: Vec<_> = body
        .split("vec![")
        .skip(1)
        .map(|row| {
            let bytes: Vec<u8> = row
                .split_once(']')
                .unwrap()
                .0
                .split(',')
                .filter_map(|s| {
                    let s = s.trim();
                    (!s.is_empty()).then(|| s.parse().unwrap())
                })
                .collect();
            i64::from_le_bytes(bytes.try_into().unwrap())
        })
        .collect();
    assert_eq!(values.len(), 28);
    let mut graph = CandidateGraph {
        count: values[0],
        start: values[1],
        target: values[2],
        degrees: [0; 5],
        edges: [[0; 4]; 5],
    };
    for row in 0..5 {
        graph.degrees[row] = values[3 + row * 5];
        graph.edges[row].copy_from_slice(&values[4 + row * 5..8 + row * 5]);
    }
    graph
}
fn retain(original: &Original, observed: &SamplerObservation, label: &str) {
    let dir = evidence_directory().join(label);
    fs::create_dir_all(&dir).unwrap();
    for (name, bytes) in [
        ("original.native", original.fixture.source.as_bytes()),
        ("original.model.json", &original.fixture.model_bytes),
        ("original.package.json", &original.fixture.emitted_bytes),
    ] {
        fs::write(dir.join(name), bytes).unwrap();
    }
    fs::write(
        dir.join("observed-sampler.json"),
        serde_json::to_vec_pretty(observed).unwrap(),
    )
    .unwrap();
    fs::write(dir.join("original-provenance.json"),serde_json::to_vec_pretty(&json!({"package":original.fixture.package_id.hex(),"clause_site":format!("{:?}",original.fixture.site),"model_digest":original.fixture.model_digest})).unwrap()).unwrap();
}
fn record_witness(
    original: &Original,
    observed: &SamplerObservation,
    graph: &CandidateGraph,
    label: &str,
    property: &str,
) {
    fs::write(
        evidence_directory()
            .join(label)
            .join("typed-domain-witness.json"),
        serde_json::to_vec_pretty(&witness(original, observed, graph, property)).unwrap(),
    )
    .unwrap();
}

#[trace("FR-015-AC-106", "TC-025")]
#[test]
fn actual_sampler_observation_preserves_capacity_guards_padding_and_authored_domain_independence() {
    for role in ["kids", "litter"] {
        let original = Original::new(role);
        let healthy = original.generated(None);
        let observed = SamplerObservation::parse(
            &healthy.rust.contents,
            healthy.identity.harness_symbol.as_str(),
        );
        assert_eq!(observed.population_capacity, 4);
        assert_eq!(observed.row_capacities, [3; 4]);
        let source = oracle_source(
            &original,
            &healthy,
            &observed,
            true,
            "d && generated && candidate.count == 4",
        );
        assert!(source.contains("generated graph domain conforms to original authored domain"));
        for count in 0..=5 {
            for degree in 0..=4 {
                let mut graph = boundary(count, degree, original.domain.unique);
                if count == 0 {
                    graph.degrees = [4; 5];
                    graph.edges = [[4; 4]; 5];
                }
                assert_eq!(
                    authored_admits(&graph, original.domain),
                    observed.admits(&graph)
                );
            }
        }
        for (mutation, count, degree, d, g) in [
            (BoundMutation::PopulationPlus, 5, 0, false, true),
            (BoundMutation::PopulationMinus, 4, 0, true, false),
            (BoundMutation::EdgeUpperPlus, 4, 4, false, true),
        ] {
            let mutant = original.generated(Some(mutation));
            let sampler = SamplerObservation::parse(
                &mutant.rust.contents,
                mutant.identity.harness_symbol.as_str(),
            );
            let graph = boundary(count, degree, original.domain.unique);
            assert_eq!(
                (
                    authored_admits(&graph, original.domain),
                    sampler.admits(&graph)
                ),
                (d, g)
            );
        }
        let mut invalid = boundary(4, 1, original.domain.unique);
        invalid.edges[0][0] = 4;
        assert!(!observed.admits(&invalid));
        assert!(!authored_admits(&invalid, original.domain));
        let repeated = boundary(4, 3, false);
        assert_eq!(observed.admits(&repeated), role == "litter");
        let mut dormant = boundary(1, 0, false);
        dormant.degrees[4] = 4;
        dormant.edges[4] = [4; 4];
        assert!(authored_admits(&dormant, original.domain));
        assert!(observed.admits(&dormant));
        let broken = healthy.rust.contents.replacen(
            "edge < degrees[row] as usize",
            "edge <= degrees[row] as usize",
            1,
        );
        assert_ne!(broken, healthy.rust.contents);
        assert!(std::panic::catch_unwind(|| SamplerObservation::parse(
            &broken,
            healthy.identity.harness_symbol.as_str()
        ))
        .is_err());
        let capacity_cut =
            healthy
                .rust
                .contents
                .replacen("degrees: &[i64; 4]", "degrees: &[i64; 3]", 1);
        assert_ne!(capacity_cut, healthy.rust.contents);
        assert!(std::panic::catch_unwind(|| SamplerObservation::parse(
            &capacity_cut,
            healthy.identity.harness_symbol.as_str()
        ))
        .is_err());
        let changed =
            healthy
                .rust
                .contents
                .replacen("object_count <= 4_i64", "object_count <= 3_i64", 1);
        assert_ne!(changed, healthy.rust.contents);
        let changed = SamplerObservation::parse(&changed, healthy.identity.harness_symbol.as_str());
        assert!(
            !changed.admits(&boundary(4, 0, false)),
            "actual range rejection, independent of metadata"
        );
        if original.domain.unique {
            let without_uniqueness = healthy
                .rust
                .contents
                .lines()
                .filter(|line| !line.contains(" != "))
                .collect::<Vec<_>>()
                .join("\n");
            assert_ne!(without_uniqueness, healthy.rust.contents);
            let changed = SamplerObservation::parse(
                &without_uniqueness,
                healthy.identity.harness_symbol.as_str(),
            );
            assert!(
                changed.admits(&repeated),
                "actual assumption removal changes G"
            );
            assert!(
                !authored_admits(&repeated, original.domain),
                "D retains the original authored uniqueness"
            );
        }
    }
}

#[trace("FR-015-AC-106", "TC-025")]
#[test]
#[ignore = "installed Kani: run this private domain control explicitly and serially"]
fn tc_025_real_kani_graph_domain_conformance_completeness_and_bound_mutants() {
    for role in ["kids", "litter"] {
        let original = Original::new(role);
        let healthy = original.generated(None);
        let observed = SamplerObservation::parse(
            &healthy.rust.contents,
            healthy.identity.harness_symbol.as_str(),
        );
        let max = boundary(4, 0, original.domain.unique);
        let label = format!("{role}-healthy-domain");
        retain(&original, &observed, &label);
        let source = oracle_source(
            &original,
            &healthy,
            &observed,
            true,
            &probe_condition(&max, true, true),
        );
        let run = run_domain(&healthy, &source, &label);
        assert_eq!(run.evidence.outcome, KaniRunOutcome::Verified);
        let graph = decode(&run.console, "Check for `cover`");
        assert_eq!(graph.count, 4);
        assert!(authored_admits(&graph, original.domain) && observed.admits(&graph));
        record_witness(
            &original,
            &observed,
            &graph,
            &label,
            "domain carrier boundary witness",
        );
        for degree in [0, 3] {
            let graph = boundary(4, degree, original.domain.unique);
            let label = format!("{role}-healthy-degree-{degree}");
            retain(&original, &observed, &label);
            let source = oracle_source(
                &original,
                &healthy,
                &observed,
                true,
                &probe_condition(&graph, true, true),
            );
            let run = run_domain(&healthy, &source, &label);
            assert_eq!(run.evidence.outcome, KaniRunOutcome::Verified);
            let actual = decode(&run.console, "Check for `cover`");
            assert_eq!(actual.degrees[0], degree);
            assert!(authored_admits(&actual, original.domain) && observed.admits(&actual));
            record_witness(
                &original,
                &observed,
                &actual,
                &label,
                "domain carrier boundary witness",
            );
        }
        for (mutation, name, count, degree, d, g, property) in [
            (
                BoundMutation::PopulationPlus,
                "population-plus-one",
                5,
                0,
                false,
                true,
                "generated graph domain conforms to original authored domain",
            ),
            (
                BoundMutation::PopulationMinus,
                "population-minus-one",
                4,
                0,
                true,
                false,
                "generated graph domain includes every original authored graph",
            ),
            (
                BoundMutation::EdgeUpperPlus,
                "edge-upper-plus-one",
                4,
                4,
                false,
                true,
                "generated graph domain conforms to original authored domain",
            ),
        ] {
            let mutant = original.generated(Some(mutation));
            let observed = SamplerObservation::parse(
                &mutant.rust.contents,
                mutant.identity.harness_symbol.as_str(),
            );
            let label = format!("{role}-{name}");
            retain(&original, &observed, &label);
            let source = oracle_source(
                &original,
                &mutant,
                &observed,
                true,
                "d && generated && candidate.count > 0",
            );
            let run = run_domain(&mutant, &source, &label);
            let KaniRunOutcome::Falsified { counterexample } = run.evidence.outcome else {
                panic!("{label}: actual bound domain must falsify its domain assertion")
            };
            assert!(
                counterexample.contains(property),
                "only the appropriate domain assertion qualifies"
            );
            let actual = decode(&counterexample, property);
            assert_eq!(
                (
                    authored_admits(&actual, original.domain),
                    observed.admits(&actual)
                ),
                (d, g)
            );
            record_witness(&original, &observed, &actual, &label, property);
            let boundary = boundary(count, degree, original.domain.unique);
            let label = format!("{label}-boundary-probe");
            retain(&original, &observed, &label);
            let source = oracle_source(
                &original,
                &mutant,
                &observed,
                false,
                &probe_condition(&boundary, d, g),
            );
            let run = run_domain(&mutant, &source, &label);
            assert_eq!(run.evidence.outcome, KaniRunOutcome::Verified);
            let actual = decode(&run.console, "Check for `cover`");
            assert_eq!((actual.count, actual.degrees[0]), (count, degree));
            assert_eq!(
                (
                    authored_admits(&actual, original.domain),
                    observed.admits(&actual)
                ),
                (d, g)
            );
            record_witness(
                &original,
                &observed,
                &actual,
                &label,
                "domain carrier boundary witness",
            );
        }
    }
}
