//! Symbolic reference-graph reachability obligations over QSL-emitted, IR-admitted packages
//! (FR-015-AC-98, -100, -105, -106 healthy half, -107, -108; TC-025).
//!
//! QSL emits every package through its public `call_site` facade from native source and a
//! test-authored domain document; no Contract IR package wire is constructed here. Slice A covers
//! the relationship-end form. A `reaches_field` application has no population link the owner
//! accessor can read, so it refuses typed `unsupported` (`NotReachesApplication`).

use crate::scratch_crate::{runtime_dependency, write_manifest, TemporaryDirectory};
use ix_trace_rs::trace;
use qsl_replay::{call_site, ClauseName, DependencyInput, Identifier, SourceIdentity};
use quire_contract_codegen::{
    execute_kani_obligation, negotiate_kani_obligations, GraphAccessorRefusal,
    GraphUnsupportedReason, KaniExecutableHarness, KaniExecutionEvidence, KaniExecutionRequest,
    KaniGraphObligationHarness, KaniInstallation, KaniObligationOutcome, KaniObligationRequest,
    KaniRunOutcome, ObligationDisposition, ObligationItem, ProofCeilings, UnsupportedObligation,
};
use quire_contract_model::{
    CheckedNodeId, CheckedPackageEvidence, CheckedPackageReadLimits, CheckedPackageV2,
    CheckedPackageV2ReadResult, CheckedTerm,
};
use serde_json::{json, Value};
use std::{fs, path::PathBuf, time::Duration};

/// One QSL-emitted and IR-admitted graph package.
struct Authored {
    package: CheckedPackageV2,
}

fn relation(name: &str, lower: u64, upper: Option<u64>, unique: bool) -> Value {
    let mut multiplicity = json!({"lower": lower, "ordered": false, "unique": unique});
    if let Some(upper) = upper {
        multiplicity["upper"] = json!(upper);
    }
    // An end with no upper bound is refused by QSL's checker before IR admission, so a real
    // package never reaches this generator with one.
    json!({
        "identity": format!("ix://ir268.test/domain/relationship/Node-{name}"),
        "category": "structural", "composite": false, "direction": "source-to-target",
        "origin": {"source": {"sourceIdentity": "ix://ir268.test/spec", "path": "spec.qspec", "startLine": 1, "startColumn": 1}},
        "sourceEnd": {"type": "ix://ir268.test/domain/Node", "role": name,
                      "multiplicity": {"lower": 0, "upper": 9, "ordered": false, "unique": unique}},
        "targetEnd": {"type": "ix://ir268.test/domain/Node", "multiplicity": multiplicity}
    })
}

fn document() -> Value {
    let mut document = super::kani_obligations_symbolic_domains::model_document();
    document["types"][1]["relationships"] = json!([
        relation("step", 0, Some(1), true),
        relation("kids", 0, Some(3), true),
        relation("litter", 0, Some(3), false),
        relation("two", 2, Some(3), false),
        relation("empty", 0, Some(0), true),
        relation("loop", 1, Some(1), true)
    ]);
    // A parameter the operation clauses can name as a second reference operand.
    let mut target = document["types"][1]["fields"]
        .as_array()
        .unwrap()
        .iter()
        .find(|field| field["name"] == "next")
        .unwrap()
        .clone();
    target["identity"] = json!("ix://ir268.test/domain/Node/visit/target");
    target["name"] = json!("target");
    target["origin"]["generated"]["inputIdentities"] =
        json!(["ix://ir268.test/domain/Node/visit/target"]);
    document["types"][1]["operations"] = json!([{
        "identity": "ix://ir268.test/domain/Node/visit", "name": "visit",
        "params": [target], "pre": [], "post": [],
        "origin": {"source": {"sourceIdentity": "ix://ir268.test/spec", "path": "spec.qspec", "startLine": 1, "startColumn": 1}},
        "frame": {"modifies": [], "creates": [], "deletes": []}
    }]);
    document
}

fn authored(form: &str, condition: &str, maximum: Option<u64>) -> Authored {
    let document = document();
    let limits = quire_canonical::Limits::new(1 << 20);
    let bytes = quire_canonical::to_vec(&document, limits).unwrap();
    let bound = maximum.map_or_else(String::new, |maximum| {
        format!("bound Nodes on M::CollectionNodes [{maximum}]\n")
    });
    let declaration = if form == "invariant" {
        format!("invariant Graph using v on M::Node at current {{ {condition} }}")
    } else {
        let keyword = if form == "precondition" {
            "pre"
        } else {
            "post"
        };
        format!("{keyword} Graph using v on M::Node::visit {{ {condition} }}")
    };
    let source = format!(
        "language \"ix:native\" edition \"1-draft\";\nprofile v = \"quire.model.complete/v1\";\nmodel M = \"ir268.test/domain\";\n{bound}predicate inspect using v (input: M::Node): Boolean {{ true }}\n{declaration}\n"
    );
    let site = call_site(
        SourceIdentity::new("agent-ix", "graph.native"),
        "graph".to_owned(),
        "graph.native",
        source.as_bytes(),
        [bytes.as_slice()],
        &DependencyInput::default(),
        &ClauseName(Identifier::new("Graph").unwrap()),
    )
    .unwrap_or_else(|error| panic!("QSL emits the authored graph clause: {error:?}"));
    let mut evidence = CheckedPackageEvidence::new();
    evidence.support_feature("quire.value.complete/v1");
    evidence
        .insert_domain_package_document("ir268.test/domain", bytes)
        .unwrap();
    let package = match CheckedPackageV2::read(
        &site.package,
        CheckedPackageReadLimits::bounded(),
        &evidence,
    ) {
        CheckedPackageV2ReadResult::Admitted(package) => *package,
        other => panic!("strict IR admission: {other:?}"),
    };
    Authored { package }
}

/// `invariant` over `self` with a three-object population, the default for most controls.
fn fixture(condition: &str) -> Authored {
    authored("invariant", condition, Some(3))
}

fn clause(package: &CheckedPackageV2) -> CheckedNodeId {
    package
        .graph()
        .nodes
        .iter()
        .find(|node| node.semantic_form.as_ref() == "state_clause")
        .unwrap()
        .node_id
        .clone()
}

fn ceilings() -> ProofCeilings {
    crate::common::proof_ceilings::proof_ceilings_with_wall_clock(Duration::from_secs(600))
}

fn outcome_with(
    package: &CheckedPackageV2,
    paths: &[&[usize]],
    unwind: u32,
    subject_path: &str,
) -> KaniObligationOutcome {
    let clause = clause(package);
    let items: Vec<ObligationItem<'_>> = paths
        .iter()
        .map(|path| ObligationItem::GraphClaim {
            package,
            clause_id: &clause,
            argument_path: path,
        })
        .collect();
    negotiate_kani_obligations(&KaniObligationRequest {
        ceilings: ceilings(),
        items: &items,
        subject_path,
        unwind,
    })
    .unwrap()
}

fn emitted(package: &CheckedPackageV2, path: &[usize]) -> KaniGraphObligationHarness {
    let KaniObligationOutcome::Emitted {
        records,
        mut graph_harnesses,
        ..
    } = outcome_with(package, &[path], 8, "crate::graph_subject")
    else {
        panic!("a graph request is valid")
    };
    assert!(
        matches!(
            records[0].disposition,
            ObligationDisposition::Supported { .. }
        ),
        "{:?}",
        records[0].disposition
    );
    assert_eq!(graph_harnesses.len(), 1);
    graph_harnesses.remove(0)
}

fn disposition(package: &CheckedPackageV2, path: &[usize], unwind: u32) -> ObligationDisposition {
    let KaniObligationOutcome::Emitted {
        mut records,
        graph_harnesses,
        ..
    } = outcome_with(package, &[path], unwind, "crate::graph_subject")
    else {
        panic!("a per-item refusal is not a request rejection")
    };
    assert!(graph_harnesses.is_empty());
    records.remove(0).disposition
}

fn refused_with(disposition: &ObligationDisposition, expected: &GraphUnsupportedReason) -> bool {
    matches!(disposition,
        ObligationDisposition::Unsupported {
            reason: UnsupportedObligation::GraphClaimNotRendered { reason, .. }
        } if reason == expected)
}

#[trace("TC-025", "FR-015-AC-98")]
#[trace("TC-025", "FR-015-AC-105")]
#[test]
fn relationship_domains_use_the_population_and_the_typed_destination_multiplicity() {
    for (edge, lower, upper, unique) in [
        ("step", 0, 1, true),
        ("kids", 0, 3, true),
        ("litter", 0, 3, false),
        ("two", 2, 3, false),
        ("empty", 0, 0, true),
        ("loop", 1, 1, true),
    ] {
        let authored = fixture(&format!("not reaches(self, self, {edge})"));
        let harness = emitted(&authored.package, &[0]);
        assert_eq!(harness.identity.maximum_population, 3, "{edge}");
        let found = &harness.identity.edge;
        assert_eq!(
            (found.lower, found.upper, found.unique),
            (lower, upper, unique),
            "{edge}"
        );
        let source = &harness.rust.contents;
        assert!(
            source.contains("object_count >= 0_i64 && object_count <= 3_i64"),
            "{edge}: {source}"
        );
        assert!(source.contains(&format!("degree_0 >= {lower}")), "{edge}");
        assert_eq!(
            source.contains("edge_0_1 != edge_0_0"),
            unique && upper > 1,
            "{edge}: uniqueness assumptions follow the end"
        );
        assert!(
            source.rfind("kani::cover!").unwrap() > source.rfind("assert!").unwrap(),
            "{edge}: the cover follows the assertion"
        );
        assert!(!source.contains("max_expansions"));
        assert!(
            !source.contains("rt::Integer") && !source.contains("Integer::"),
            "{edge}: native integer carriage only"
        );
    }
}

#[trace("TC-025", "FR-015-AC-105")]
#[test]
fn the_population_maximum_is_the_authored_bound_including_zero_and_one() {
    for maximum in [0_u64, 1, 4] {
        let authored = authored("invariant", "not reaches(self, self, kids)", Some(maximum));
        let harness = emitted(&authored.package, &[0]);
        assert_eq!(harness.identity.maximum_population, maximum);
        assert!(harness
            .rust
            .contents
            .contains(&format!("object_count <= {maximum}_i64")));
    }
}

#[trace("TC-025", "FR-015-AC-98")]
#[trace("TC-025", "FR-015-AC-108")]
#[test]
fn a_claim_without_a_population_settles_unsupported_naming_it() {
    let authored = authored("invariant", "not reaches(self, self, kids)", None);
    assert!(refused_with(
        &disposition(&authored.package, &[0], 8),
        &GraphUnsupportedReason::PopulationUnavailable
    ));
}

#[trace("TC-025", "FR-015-AC-107")]
#[trace("TC-025", "FR-015-AC-108")]
#[test]
fn field_edge_forms_refuse_typed_not_reaches_application_and_location_comes_first() {
    for edge in ["next", "maybe", "many"] {
        let authored = fixture(&format!("not reaches(self, self, {edge})"));
        let field = authored
            .package
            .graph()
            .nodes
            .iter()
            .find(|node| {
                matches!(&node.body, CheckedTerm::Application { operation, .. }
                    if operation.identity() == Some("quire.op.model.reaches_field"))
            })
            .unwrap_or_else(|| panic!("{edge} lowers to a field-edge application"));
        assert!(matches!(&field.body, CheckedTerm::Application { .. }));
        assert!(
            refused_with(
                &disposition(&authored.package, &[0], 8),
                &GraphUnsupportedReason::PopulationAccessorRefused {
                    refusal: GraphAccessorRefusal::NotReachesApplication
                }
            ),
            "{edge}"
        );
        // A path that names no reachability application is refused by location first.
        assert!(
            refused_with(
                &disposition(&authored.package, &[1], 8),
                &GraphUnsupportedReason::MissingClaim
            ),
            "{edge}"
        );
    }
}

#[trace("TC-025", "FR-015-AC-107")]
#[test]
fn locations_refuse_without_a_harness_and_never_cross_clauses() {
    let authored = fixture("not reaches(self, self, kids)");
    // The negation, not the application, sits at the empty path; a deeper path leaves the tree.
    for path in [&[][..], &[0, 0][..], &[7][..]] {
        assert!(
            refused_with(
                &disposition(&authored.package, path, 8),
                &GraphUnsupportedReason::MissingClaim
            ),
            "{path:?}"
        );
    }
    // A node that is not a state clause cannot hold a claim.
    let other = authored
        .package
        .graph()
        .nodes
        .iter()
        .find(|node| node.semantic_form.as_ref() != "state_clause")
        .unwrap()
        .node_id
        .clone();
    let item = ObligationItem::GraphClaim {
        package: &authored.package,
        clause_id: &other,
        argument_path: &[0],
    };
    let KaniObligationOutcome::Emitted { records, .. } =
        negotiate_kani_obligations(&KaniObligationRequest {
            ceilings: ceilings(),
            items: &[item],
            subject_path: "crate::graph_subject",
            unwind: 8,
        })
        .unwrap()
    else {
        panic!("a per-item refusal")
    };
    assert!(refused_with(
        &records[0].disposition,
        &GraphUnsupportedReason::MissingClaim
    ));
}

#[trace("TC-025", "FR-015-AC-107")]
#[trace("TC-025", "FR-015-AC-100")]
#[test]
fn one_reachability_written_twice_is_two_items_with_distinct_identities() {
    let authored = fixture("reaches(self, self, kids) or not reaches(self, self, kids)");
    let KaniObligationOutcome::Emitted {
        records,
        graph_harnesses,
        ..
    } = outcome_with(
        &authored.package,
        &[&[0], &[1, 0]],
        8,
        "crate::graph_subject",
    )
    else {
        panic!("a valid request")
    };
    assert_eq!(records.len(), 2);
    assert_eq!(graph_harnesses.len(), 2);
    let [first, second] = [&graph_harnesses[0], &graph_harnesses[1]];
    assert_eq!(first.identity.argument_path, vec![0]);
    assert_eq!(second.identity.argument_path, vec![1, 0]);
    assert_ne!(
        first.identity.harness_symbol,
        second.identity.harness_symbol
    );
    assert_eq!(
        first.identity.application_id,
        second.identity.application_id
    );
}

#[trace("TC-025", "FR-015-AC-107")]
#[test]
fn a_refused_sibling_leaves_a_supported_harness_unchanged() {
    let authored = fixture("reaches(self, self, kids) or not reaches(self, self, kids)");
    let alone = emitted(&authored.package, &[0]);
    let KaniObligationOutcome::Emitted {
        records,
        graph_harnesses,
        ..
    } = outcome_with(&authored.package, &[&[0], &[5]], 8, "crate::graph_subject")
    else {
        panic!("a valid request")
    };
    assert!(matches!(
        records[0].disposition,
        ObligationDisposition::Supported { .. }
    ));
    assert!(refused_with(
        &records[1].disposition,
        &GraphUnsupportedReason::MissingClaim
    ));
    assert_eq!(graph_harnesses.len(), 1);
    assert_eq!(graph_harnesses[0].rust, alone.rust);
    assert_eq!(graph_harnesses[0].identity, alone.identity);
}

#[trace("TC-025", "FR-015-AC-105")]
#[test]
fn operation_clauses_draw_the_same_symbolic_graph_for_two_reference_operands() {
    for form in ["precondition", "postcondition"] {
        let authored = authored(
            form,
            "reaches(self, target, kids) implies reaches(self, target, kids)",
            Some(3),
        );
        let harness = emitted(&authored.package, &[0]);
        assert_eq!(
            harness.identity.kind,
            if form == "precondition" {
                quire_contract_codegen::ObligationKind::Precondition
            } else {
                quire_contract_codegen::ObligationKind::Postcondition
            }
        );
        // `self` and `target` are distinct operands: nothing forces them equal.
        assert!(!harness.rust.contents.contains("start == target"), "{form}");
    }
    let same = fixture("not reaches(self, self, kids)");
    assert!(emitted(&same.package, &[0])
        .rust
        .contents
        .contains("start == target"));
}

#[trace("TC-025", "FR-015-AC-100")]
#[test]
fn every_identity_component_changes_the_harness_and_equal_inputs_regenerate_it() {
    let base = fixture("not reaches(self, self, kids)");
    let first = emitted(&base.package, &[0]);
    assert_eq!(first, emitted(&base.package, &[0]), "byte-identical");
    let renamed = |harness: &KaniGraphObligationHarness| harness.identity.harness_symbol.clone();
    let other_edge = emitted(&fixture("not reaches(self, self, litter)").package, &[0]);
    let other_extent = emitted(
        &authored("invariant", "not reaches(self, self, kids)", Some(4)).package,
        &[0],
    );
    let other_condition = emitted(
        &fixture("reaches(self, self, kids) or not reaches(self, self, kids)").package,
        &[0],
    );
    let KaniObligationOutcome::Emitted {
        graph_harnesses: other_subject,
        ..
    } = outcome_with(&base.package, &[&[0]], 8, "crate::other_subject")
    else {
        panic!("valid")
    };
    let KaniObligationOutcome::Emitted {
        graph_harnesses: other_unwind,
        ..
    } = outcome_with(&base.package, &[&[0]], 9, "crate::graph_subject")
    else {
        panic!("valid")
    };
    for other in [
        &other_edge,
        &other_extent,
        &other_condition,
        &other_subject[0],
        &other_unwind[0],
    ] {
        assert_ne!(renamed(&first), renamed(other));
        assert_ne!(first.identity, other.identity);
    }
}

#[trace("TC-025", "FR-015-AC-98")]
#[test]
fn a_domain_beyond_the_unwind_or_the_memory_ceiling_refuses_without_reducing_it() {
    let authored = authored("invariant", "not reaches(self, self, kids)", Some(40));
    assert!(matches!(
        disposition(&authored.package, &[0], 8),
        ObligationDisposition::Unsupported {
            reason: UnsupportedObligation::GraphClaimNotRendered {
                reason: GraphUnsupportedReason::UnwindBelowDomain {
                    required: 41,
                    supplied: 8
                },
                ..
            }
        }
    ));
    let clause = clause(&authored.package);
    let item = ObligationItem::GraphClaim {
        package: &authored.package,
        clause_id: &clause,
        argument_path: &[0],
    };
    let KaniObligationOutcome::Emitted { records, .. } =
        negotiate_kani_obligations(&KaniObligationRequest {
            ceilings: ProofCeilings {
                memory_bytes: std::num::NonZeroU64::new(1024).unwrap(),
                wall_clock: Duration::from_secs(60),
            },
            items: &[item],
            subject_path: "crate::graph_subject",
            unwind: 64,
        })
        .unwrap()
    else {
        panic!("valid")
    };
    assert!(matches!(
        records[0].disposition,
        ObligationDisposition::Unsupported {
            reason: UnsupportedObligation::GraphClaimNotRendered {
                reason: GraphUnsupportedReason::DomainExceedsMemoryCeiling { .. },
                ..
            }
        }
    ));
}

// ---------------------------------------------------------------------------------------------
// Installed-Kani lane (FR-015-AC-106 healthy half). Run serially through the kani lane.
// ---------------------------------------------------------------------------------------------

fn run_graph_kani(
    installation: &KaniInstallation,
    harness: &KaniGraphObligationHarness,
    label: &str,
) -> KaniExecutionEvidence {
    let scratch = TemporaryDirectory::new("quire-kani-symbolic-graph");
    fs::write(scratch.path.join("src/lib.rs"), &harness.rust.contents)
        .expect("write the generated artifact byte for byte");
    write_manifest(
        &scratch.path,
        &format!(
            "[package]\nname = \"generated-kani-graph\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[dependencies]\n{}\n\n[workspace]\n",
            runtime_dependency(&["exact"])
        ),
    );
    fs::write(
        scratch.path.join("build.rs"),
        "fn main() { println!(\"cargo:rustc-check-cfg=cfg(kani)\"); }\n",
    )
    .expect("allow Kani's generated cfg");
    let cancel = quire_exact::Cancel::new();
    let started = std::time::Instant::now();
    let evidence = execute_kani_obligation(&KaniExecutionRequest {
        cancel: &cancel,
        installation,
        harness: KaniExecutableHarness::from(harness),
        crate_directory: &scratch.path,
        target_directory: &PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("kani-graph"),
        home_directory: None,
    })
    .unwrap_or_else(|refusal| panic!("{label}: {refusal}"));
    eprintln!(
        "GRAPH-KANI {label}: {:?} in {:?}",
        evidence.outcome,
        started.elapsed()
    );
    let directory = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("kani-graph-evidence");
    fs::create_dir_all(&directory).unwrap();
    fs::write(
        directory.join(format!("{label}.json")),
        serde_json::to_string_pretty(&evidence).unwrap(),
    )
    .unwrap();
    fs::write(
        directory.join(format!("{label}.harness.rs")),
        &harness.rust.contents,
    )
    .unwrap();
    evidence
}

fn verified(label: &str, condition: &str, maximum: u64, path: &[usize]) {
    let installation = KaniInstallation::discover().expect("cargo-kani is installed");
    let authored = authored("invariant", condition, Some(maximum));
    let harness = emitted(&authored.package, path);
    let evidence = run_graph_kani(&installation, &harness, label);
    assert_eq!(evidence.outcome, KaniRunOutcome::Verified, "{label}");
    assert_eq!(evidence.exit_code, Some(0));
    assert_eq!(evidence.unwind, 8);
}

/// A healthy claim over a unique `{0,3}` end: every graph the domain admits satisfies it.
#[trace("TC-025", "FR-015-AC-106")]
#[test]
#[ignore = "kani lane: run serially through make kani-gate"]
fn real_kani_unique_end_claim_verifies_with_its_cover() {
    verified(
        "unique-0-3",
        "reaches(self, self, kids) or not reaches(self, self, kids)",
        3,
        &[0],
    );
}

/// The same claim over a non-unique `{0,3}` end, where repeated targets are admitted.
#[trace("TC-025", "FR-015-AC-106")]
#[test]
#[ignore = "kani lane: run serially through make kani-gate"]
fn real_kani_non_unique_end_claim_verifies_with_its_cover() {
    verified(
        "nonunique-0-3",
        "reaches(self, self, litter) or not reaches(self, self, litter)",
        3,
        &[0],
    );
}

/// An end with no edges at all never reaches: a positive-length search finds nothing, so an
/// isolated `start = target` is unreachable.
#[trace("TC-025", "FR-015-AC-106")]
#[test]
#[ignore = "kani lane: run serially through make kani-gate"]
fn real_kani_empty_end_is_unreachable_even_for_start_equal_target() {
    verified("empty-0-0", "not reaches(self, self, empty)", 3, &[0]);
}

/// With one object and a `{1,1}` end the lower bound forces the single edge, and it can only
/// target the object itself, so the self-loop is reachable.
#[trace("TC-025", "FR-015-AC-106")]
#[test]
#[ignore = "kani lane: run serially through make kani-gate"]
fn real_kani_forced_edge_makes_the_self_loop_reachable() {
    verified("forced-self-loop", "reaches(self, self, loop)", 1, &[]);
}

/// A claim that does not hold on every admitted graph is falsified with an in-domain playback.
#[trace("TC-025", "FR-015-AC-106")]
#[test]
#[ignore = "kani lane: run serially through make kani-gate"]
fn real_kani_unsatisfied_claim_is_falsified_with_an_in_domain_playback() {
    let installation = KaniInstallation::discover().expect("cargo-kani is installed");
    let authored = authored("invariant", "not reaches(self, self, kids)", Some(3));
    let harness = emitted(&authored.package, &[0]);
    let evidence = run_graph_kani(&installation, &harness, "falsified-not-reaches");
    let KaniRunOutcome::Falsified { counterexample } = &evidence.outcome else {
        panic!("a cycle reaches itself: {evidence:?}");
    };
    assert!(counterexample.contains("kani::concrete_playback_run"));
    assert!(counterexample.contains(harness.identity.harness_symbol.as_str()));
    // Decode the playback in the generated draw order and check, natively and independently of
    // the generated closure, that it is an in-domain graph in which the object reaches itself.
    let slots: Vec<i64> = counterexample
        .split("vec![")
        .skip(2)
        .map(|vector| {
            let bytes: Vec<u8> = vector
                .split(']')
                .next()
                .unwrap()
                .split(',')
                .map(|byte| byte.trim().parse().unwrap())
                .collect();
            i64::from_le_bytes(bytes.try_into().unwrap())
        })
        .collect();
    let (count, start, target) = (slots[0] as usize, slots[1] as usize, slots[2] as usize);
    assert!(count <= 3 && start < count && target == start);
    let rows: Vec<(usize, Vec<usize>)> = (0..3)
        .map(|row| {
            let base = 3 + row * 4;
            let degree = slots[base] as usize;
            let targets: Vec<usize> = slots[base + 1..base + 1 + degree]
                .iter()
                .map(|target| *target as usize)
                .collect();
            (degree, targets)
        })
        .collect();
    for (row, (degree, targets)) in rows.iter().enumerate().take(count) {
        assert!(
            *degree <= 3 && targets.iter().all(|target| *target < count),
            "row {row}"
        );
        let mut distinct = targets.clone();
        distinct.sort_unstable();
        distinct.dedup();
        assert_eq!(
            distinct.len(),
            targets.len(),
            "a unique end repeats no target"
        );
    }
    let mut seen = vec![false; count];
    let mut frontier = rows[start].1.clone();
    while let Some(next) = frontier.pop() {
        if !seen[next] {
            seen[next] = true;
            frontier.extend(rows[next].1.iter().copied());
        }
    }
    assert!(
        seen[target],
        "the playback graph reaches {target} from {start}"
    );
}
