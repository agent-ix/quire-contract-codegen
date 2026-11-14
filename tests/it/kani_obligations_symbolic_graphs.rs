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
    CheckedNodeId, CheckedOccurrence, CheckedOccurrenceRole, CheckedPackageEvidence,
    CheckedPackageReadLimits, CheckedPackageV2, CheckedPackageV2ReadResult, CheckedTerm,
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
    document_with(Some(3))
}

/// The model document, with the upper bound of the `kids` end replaced (`None` is unbounded).
fn document_with(kids_upper: Option<u64>) -> Value {
    let mut document = super::kani_obligations_symbolic_domains::model_document();
    document["types"][1]["relationships"] = json!([
        relation("step", 0, Some(1), true),
        relation("kids", 0, kids_upper, true),
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

fn declaration(form: &str, name: &str, condition: &str) -> String {
    if form == "invariant" {
        format!("invariant {name} using v on M::Node at current {{ {condition} }}")
    } else {
        let keyword = if form == "precondition" {
            "pre"
        } else {
            "post"
        };
        format!("{keyword} {name} using v on M::Node::visit {{ {condition} }}")
    }
}

/// Emit through QSL and admit through Contract IR, or return the refusal text of either stage.
fn try_authored_with(
    document: &Value,
    declarations: &[String],
    bound: &str,
) -> Result<Authored, String> {
    try_authored_clause(document, declarations, bound, "Graph")
}

/// As `try_authored_with`, selecting which declared clause the QSL call site emits.
fn try_authored_clause(
    document: &Value,
    declarations: &[String],
    bound: &str,
    clause_name: &str,
) -> Result<Authored, String> {
    let limits = quire_canonical::Limits::new(1 << 20);
    let bytes = quire_canonical::to_vec(document, limits).unwrap();
    let source = format!(
        "language \"ix:native\" edition \"1-draft\";\nprofile v = \"quire.model.complete/v1\";\nmodel M = \"ir268.test/domain\";\n{bound}predicate inspect using v (input: M::Node): Boolean {{ true }}\n{}\n",
        declarations.join("\n")
    );
    let site = call_site(
        SourceIdentity::new("agent-ix", "graph.native"),
        "graph".to_owned(),
        "graph.native",
        source.as_bytes(),
        [bytes.as_slice()],
        &DependencyInput::default(),
        &ClauseName(Identifier::new(clause_name).unwrap()),
    )
    .map_err(|error| format!("{error:?}"))?;
    let mut evidence = CheckedPackageEvidence::new();
    evidence.support_feature("quire.value.complete/v1");
    evidence
        .insert_domain_package_document("ir268.test/domain", bytes)
        .unwrap();
    match CheckedPackageV2::read(
        &site.package,
        CheckedPackageReadLimits::bounded(),
        &evidence,
    ) {
        CheckedPackageV2ReadResult::Admitted(package) => Ok(Authored { package: *package }),
        other => Err(format!("strict IR admission: {other:?}")),
    }
}

fn bound_line(maximum: Option<u64>) -> String {
    maximum.map_or_else(String::new, |maximum| {
        format!("bound Nodes on M::CollectionNodes [{maximum}]\n")
    })
}

fn authored_declarations(declarations: &[String], maximum: Option<u64>) -> Authored {
    try_authored_with(&document(), declarations, &bound_line(maximum))
        .unwrap_or_else(|error| panic!("QSL emits and IR admits the authored clause: {error}"))
}

fn authored(form: &str, condition: &str, maximum: Option<u64>) -> Authored {
    authored_declarations(&[declaration(form, "Graph", condition)], maximum)
}

/// `invariant` over `self` with a three-object population, the default for most controls.
fn fixture(condition: &str) -> Authored {
    authored("invariant", condition, Some(3))
}

fn clause(package: &CheckedPackageV2) -> CheckedNodeId {
    package
        .resolved_graph()
        .nodes
        .iter()
        .find(|node| node.semantic_form.as_ref() == "state_clause")
        .unwrap()
        .node_id
        .clone()
}

/// Every source occurrence of one clause node, in package order.
fn occurrences(package: &CheckedPackageV2, clause: &CheckedNodeId) -> Vec<CheckedOccurrence> {
    package
        .resolved_graph()
        .nodes
        .iter()
        .find(|node| &node.node_id == clause)
        .unwrap()
        .occurrences
        .clone()
}

/// The first occurrence of the clause node: the one a single-clause package has.
fn first_occurrence(package: &CheckedPackageV2, clause: &CheckedNodeId) -> CheckedOccurrence {
    occurrences(package, clause).remove(0)
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
    let occurrence = first_occurrence(package, &clause);
    let items: Vec<ObligationItem<'_>> = paths
        .iter()
        .map(|path| ObligationItem::GraphClaim {
            package,
            node_id: &clause,
            occurrence: &occurrence,
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
            .resolved_graph()
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
        .resolved_graph()
        .nodes
        .iter()
        .find(|node| node.semantic_form.as_ref() != "state_clause")
        .unwrap()
        .node_id
        .clone();
    let other_occurrence = CheckedOccurrence {
        role: CheckedOccurrenceRole::Expression,
        ordinal: 0,
    };
    let item = ObligationItem::GraphClaim {
        package: &authored.package,
        node_id: &other,
        occurrence: &other_occurrence,
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

/// A precondition or postcondition graph claim has no defined harness (the body never calls the
/// subject and assumes no sibling preconditions), so it settles `unsupported` and renders nothing.
#[trace("TC-025", "FR-015-AC-105")]
#[trace("TC-025", "FR-015-AC-108")]
#[test]
fn operation_clause_graph_claims_settle_unsupported_with_no_harness() {
    for form in ["precondition", "postcondition"] {
        let authored = authored(
            form,
            "reaches(self, target, kids) implies reaches(self, target, kids)",
            Some(3),
        );
        assert!(
            refused_with(
                &disposition(&authored.package, &[0], 8),
                &GraphUnsupportedReason::OperationClauseNotSupported
            ),
            "{form}"
        );
    }
    // The location check still comes first: a path naming no reachability is `MissingClaim`.
    let authored = authored(
        "precondition",
        "reaches(self, target, kids) implies reaches(self, target, kids)",
        Some(3),
    );
    assert!(refused_with(
        &disposition(&authored.package, &[7], 8),
        &GraphUnsupportedReason::MissingClaim
    ));
    // A claim over `self` for both operands forces start and target equal.
    let same = fixture("not reaches(self, self, kids)");
    assert!(emitted(&same.package, &[0])
        .rust
        .contents
        .contains("start == target"));
}

/// The recorded components of an identity other than the package lock, so a variant that also
/// changes the package cannot hide a component that failed to change.
fn components(
    harness: &KaniGraphObligationHarness,
) -> (u64, &quire_contract_codegen::GraphEdgeIdentity, &str, u32) {
    let identity = &harness.identity;
    (
        identity.maximum_population,
        &identity.edge,
        identity.subject_path.as_str(),
        identity.unwind,
    )
}

#[trace("TC-025", "FR-015-AC-100")]
#[test]
fn every_identity_component_changes_the_harness_and_equal_inputs_regenerate_it() {
    let base = fixture("not reaches(self, self, kids)");
    let first = emitted(&base.package, &[0]);
    assert_eq!(first, emitted(&base.package, &[0]), "byte-identical");
    assert_eq!(first.identity.maximum_population, 3);
    assert_eq!(
        (first.identity.edge.lower, first.identity.edge.upper),
        (0, 3)
    );
    let other_edge = emitted(&fixture("not reaches(self, self, litter)").package, &[0]);
    let other_extent = emitted(
        &authored("invariant", "not reaches(self, self, kids)", Some(4)).package,
        &[0],
    );
    // Only the authored multiplicity of the end changes; the extent, relationship and claim stay.
    let narrowed = try_authored_with(
        &document_with(Some(2)),
        &[declaration(
            "invariant",
            "Graph",
            "not reaches(self, self, kids)",
        )],
        &bound_line(Some(3)),
    )
    .unwrap();
    let other_multiplicity = emitted(&narrowed.package, &[0]);
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
    let KaniObligationOutcome::Emitted {
        graph_harnesses: other_path,
        ..
    } = outcome_with(
        &fixture("reaches(self, self, kids) or not reaches(self, self, kids)").package,
        &[&[1, 0]],
        8,
        "crate::graph_subject",
    )
    else {
        panic!("valid")
    };
    for other in [
        &other_edge,
        &other_extent,
        &other_multiplicity,
        &other_condition,
        &other_subject[0],
        &other_unwind[0],
        &other_path[0],
    ] {
        assert_ne!(
            first.identity.harness_symbol, other.identity.harness_symbol,
            "{:?}",
            other.identity
        );
        assert_ne!(first.identity, other.identity);
    }
    // Each component is recorded in its own right: the variant differs in that component.
    assert_ne!(components(&first).0, components(&other_extent).0, "extent");
    assert_eq!(components(&other_extent).1, components(&first).1);
    assert_eq!(components(&other_multiplicity).0, components(&first).0);
    assert_eq!(components(&other_multiplicity).1.upper, 2);
    assert_eq!(
        components(&other_multiplicity).1.declaration,
        first.identity.edge.declaration
    );
    assert_ne!(
        components(&other_edge).1.declaration,
        first.identity.edge.declaration
    );
    assert_ne!(components(&other_edge).1.unique, first.identity.edge.unique);
    assert_ne!(components(&other_subject[0]).2, components(&first).2);
    assert_ne!(components(&other_unwind[0]).3, components(&first).3);
    assert_ne!(
        other_path[0].identity.argument_path,
        first.identity.argument_path
    );
}

/// The identity records the solver and both ordered option vectors, and a run launches exactly
/// the recorded vector (run evidence preserves it).
#[trace("TC-025", "FR-015-AC-100")]
#[test]
fn graph_run_evidence_preserves_every_ordered_identity_option() {
    use std::os::unix::fs::PermissionsExt;

    let fixture = fixture("not reaches(self, self, kids)");
    let harness = emitted(&fixture.package, &[0]);
    let scratch = TemporaryDirectory::new("quire-kani-symbolic-graph-options");
    fs::write(scratch.path.join("src/lib.rs"), &harness.rust.contents).unwrap();
    write_manifest(
        &scratch.path,
        &format!(
            "[package]\nname = \"generated-kani-graph\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[dependencies]\n{}\n\n[workspace]\n",
            runtime_dependency(&["exact"])
        ),
    );
    let launcher = scratch.path.join("cargo-kani");
    fs::write(
        &launcher,
        "#!/bin/sh\nprintf '%s\\n' \"$@\" > arguments.txt\nexit 1\n",
    )
    .unwrap();
    fs::set_permissions(&launcher, fs::Permissions::from_mode(0o755)).unwrap();
    let target_directory = scratch.path.join("target");
    fs::create_dir(&target_directory).unwrap();
    let installation = KaniInstallation::new(&launcher);
    let cancel = quire_exact::Cancel::new();
    let evidence = execute_kani_obligation(&KaniExecutionRequest {
        cancel: &cancel,
        installation: &installation,
        harness: KaniExecutableHarness::from(&harness),
        crate_directory: &scratch.path,
        target_directory: &target_directory,
        home_directory: None,
    })
    .expect("a bounded launcher returns run evidence even without a proof report");
    let received: Vec<String> = fs::read_to_string(scratch.path.join("arguments.txt"))
        .expect("the launcher received arguments")
        .lines()
        .map(str::to_owned)
        .collect();
    assert_eq!(evidence.arguments, received);
    assert_eq!(received[0], "kani");
    let options = &harness.identity.options;
    assert_eq!(&received[1..=options.len()], options.as_slice());
    assert!(options.windows(2).any(|pair| pair == ["--unwind", "8"]));
    assert!(options
        .windows(2)
        .any(|pair| pair == ["--solver", "cadical"]));
    assert!(!harness.identity.witness_options.is_empty());
    assert_eq!(evidence.unwind, harness.identity.unwind);
    assert_eq!(evidence.solver, harness.identity.solver);
}

#[trace("TC-025", "FR-015-AC-101")]
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
    let occurrence = first_occurrence(&authored.package, &clause);
    let item = ObligationItem::GraphClaim {
        package: &authored.package,
        node_id: &clause,
        occurrence: &occurrence,
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

fn dispositions_at(
    package: &CheckedPackageV2,
    items: &[(&CheckedNodeId, &CheckedOccurrence, &[usize])],
) -> (Vec<ObligationDisposition>, Vec<KaniGraphObligationHarness>) {
    let items: Vec<ObligationItem<'_>> = items
        .iter()
        .map(|(clause, occurrence, path)| ObligationItem::GraphClaim {
            package,
            node_id: clause,
            occurrence,
            argument_path: path,
        })
        .collect();
    let KaniObligationOutcome::Emitted {
        records,
        graph_harnesses,
        ..
    } = negotiate_kani_obligations(&KaniObligationRequest {
        ceilings: ceilings(),
        items: &items,
        subject_path: "crate::graph_subject",
        unwind: 8,
    })
    .unwrap()
    else {
        panic!("per-item refusals are not request rejections")
    };
    (
        records
            .into_iter()
            .map(|record| record.disposition)
            .collect(),
        graph_harnesses,
    )
}

/// A field-edge and a relationship-end reachability in one condition are two items. The
/// relationship-end item at position 0 is refused because the condition names a second
/// application (`UnsupportedClaimShape`), and the field-edge item at position 1 settles by its own
/// accessor refusal: neither disposition follows from the other.
#[trace("TC-025", "FR-015-AC-107")]
#[trace("TC-025", "FR-015-AC-108")]
#[test]
fn a_field_edge_and_a_relationship_end_reachability_are_separate_items() {
    let authored = fixture("reaches(self, self, kids) and reaches(self, self, next)");
    let clause = clause(&authored.package);
    let occurrence = first_occurrence(&authored.package, &clause);
    let (dispositions, harnesses) = dispositions_at(
        &authored.package,
        &[
            (&clause, &occurrence, &[0]),
            (&clause, &occurrence, &[1]),
            (&clause, &occurrence, &[2]),
        ],
    );
    assert!(harnesses.is_empty());
    assert!(refused_with(
        &dispositions[0],
        &GraphUnsupportedReason::UnsupportedClaimShape
    ));
    assert!(refused_with(
        &dispositions[1],
        &GraphUnsupportedReason::PopulationAccessorRefused {
            refusal: GraphAccessorRefusal::NotReachesApplication
        }
    ));
    assert!(refused_with(
        &dispositions[2],
        &GraphUnsupportedReason::MissingClaim
    ));
}

#[trace("TC-025", "FR-015-AC-107")]
#[test]
fn a_collection_claim_and_a_clause_without_reachability_settle_missing_claim() {
    for condition in [
        "forall(item in self.items: item >= 0)",
        "inspect(self)",
        "true",
    ] {
        let authored = fixture(condition);
        for path in [&[][..], &[0][..], &[0, 0][..]] {
            assert!(
                refused_with(
                    &disposition(&authored.package, path, 8),
                    &GraphUnsupportedReason::MissingClaim
                ),
                "{condition} at {path:?}"
            );
        }
    }
}

/// Two distinct reachability applications, or a node that is not a Boolean combinator of the one
/// selected application, are claim forms the encoding cannot express.
#[trace("TC-025", "FR-015-AC-108")]
#[test]
fn a_claim_the_encoding_cannot_express_settles_unsupported_claim_shape() {
    for condition in [
        "reaches(self, self, kids) and reaches(self, self, litter)",
        "reaches(self, self, kids) or inspect(self)",
    ] {
        let authored = fixture(condition);
        assert!(
            refused_with(
                &disposition(&authored.package, &[0], 8),
                &GraphUnsupportedReason::UnsupportedClaimShape
            ),
            "{condition}"
        );
    }
}

/// A population maximum beyond `u64` is refused by QSL before a package exists, so the
/// generator's `DomainNotRepresentable` for the authored maximum is reachable only through a
/// package QSL does not emit.
#[trace("TC-025", "FR-015-AC-108")]
#[test]
fn qsl_refuses_a_population_maximum_beyond_u64_before_a_package_exists() {
    let error = match try_authored_with(
        &document(),
        &[declaration(
            "invariant",
            "Graph",
            "not reaches(self, self, kids)",
        )],
        "bound Nodes on M::CollectionNodes [18446744073709551616]\n",
    ) {
        Ok(_) => panic!("a bound beyond u64 admits no package"),
        Err(error) => error,
    };
    assert!(error.contains("IllTyped"), "{error}");
}

/// An unbounded relationship end is refused before a package exists, so `requires_bound` for an
/// edge cannot be reached through a real package.
#[trace("TC-025", "FR-015-AC-108")]
#[test]
fn qsl_or_ir_refuses_an_unbounded_end_before_a_package_exists() {
    let error = match try_authored_with(
        &document_with(None),
        &[declaration(
            "invariant",
            "Graph",
            "not reaches(self, self, kids)",
        )],
        &bound_line(Some(3)),
    ) {
        Ok(_) => panic!("an end with no authored upper bound admits no package"),
        Err(error) => error,
    };
    assert!(error.contains("UnsupportedConstruct"), "{error}");
}

// ---------------------------------------------------------------------------------------------
// Installed-Kani lane (FR-015-AC-106; the `{0,3}` runs are smoke runs, see their docs). Run serially through the kani lane.
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

/// Smoke run over a unique `{0,3}` end: the claim is a tautology, so `Verified` shows only that
/// the harness compiles, draws and covers a non-empty maximum population. It earns no
/// healthy-case credit (FR-015-AC-106).
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

/// The same tautology smoke run over a non-unique `{0,3}` end; no healthy-case credit.
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

/// Transitivity over the whole population: three quantifier-bound objects and three reachability
/// applications of the one end.
const TRANSITIVITY: &str = "forall(a in M::CollectionNodes : forall(b in M::CollectionNodes : forall(c in M::CollectionNodes : (reaches(a, b, kids) and reaches(b, c, kids)) implies reaches(a, c, kids))))";

/// The three reachability applications of `TRANSITIVITY`, by argument path through the three
/// quantifier binders: `reaches(a, b)`, `reaches(b, c)` and `reaches(a, c)`.
const TRANSITIVITY_PATHS: [&[usize]; 3] = [&[1, 1, 1, 0, 0], &[1, 1, 1, 0, 1], &[1, 1, 1, 1]];

#[trace("TC-025", "FR-015-AC-98")]
#[trace("TC-025", "FR-015-AC-105")]
#[trace("TC-025", "FR-015-AC-107")]
#[test]
fn a_transitivity_claim_with_three_quantifier_bound_operands_is_three_items() {
    let authored = fixture(TRANSITIVITY);
    let KaniObligationOutcome::Emitted {
        records,
        graph_harnesses,
        ..
    } = outcome_with(
        &authored.package,
        &TRANSITIVITY_PATHS,
        8,
        "crate::graph_subject",
    )
    else {
        panic!("a valid request")
    };
    assert!(
        records
            .iter()
            .all(|record| matches!(record.disposition, ObligationDisposition::Supported { .. })),
        "{records:?}"
    );
    assert_eq!(graph_harnesses.len(), 3);
    for (harness, path) in graph_harnesses.iter().zip(TRANSITIVITY_PATHS) {
        assert_eq!(harness.identity.argument_path, path);
        assert_eq!(harness.identity.maximum_population, 3);
        let source = &harness.rust.contents;
        // Three nested quantifiers over the live objects and one closure, no caller bound.
        for depth in 0..3 {
            assert!(
                source.contains(&format!("for q{depth} in 0..3")),
                "{source}"
            );
        }
        assert!(source.contains("paths[q0][q1]") && source.contains("paths[q1][q2]"));
        assert!(source.contains("paths[q0][q2]"));
        assert!(!source.contains("rt::Integer") && !source.contains("max_expansions"));
    }
    // Each application is its own node, so each item names a distinct claim.
    let applications: std::collections::BTreeSet<_> = graph_harnesses
        .iter()
        .map(|harness| format!("{:?}", harness.identity.application_id))
        .collect();
    assert_eq!(applications.len(), 3);
    let symbols: std::collections::BTreeSet<_> = graph_harnesses
        .iter()
        .map(|harness| harness.identity.harness_symbol.clone())
        .collect();
    assert_eq!(symbols.len(), 3);
}

/// The deepest nesting the oracle unrolls is four binders; the claim at that depth is supported
/// and its harness evaluates four nested quantifiers.
#[trace("TC-025", "FR-015-AC-105")]
#[trace("TC-025", "FR-015-AC-108")]
#[test]
fn four_nested_binders_are_the_deepest_supported_claim() {
    let condition = "forall(a in M::CollectionNodes : forall(b in M::CollectionNodes : forall(c in M::CollectionNodes : forall(d in M::CollectionNodes : reaches(a, d, kids) or not reaches(a, d, kids)))))";
    let authored = fixture(condition);
    let harness = emitted(&authored.package, &[1, 1, 1, 1, 0]);
    let source = &harness.rust.contents;
    for depth in 0..4 {
        assert!(
            source.contains(&format!("for q{depth} in 0..3")),
            "{source}"
        );
    }
    assert!(!source.contains("for q4 in"), "{source}");
}

#[trace("TC-025", "FR-015-AC-108")]
#[test]
fn quantified_claims_the_encoding_cannot_express_settle_unsupported_claim_shape() {
    let deep = (0..5).fold("reaches(a0, a0, kids)".to_owned(), |body, depth| {
        format!("forall(a{} in M::CollectionNodes : {body})", 4 - depth)
    });
    for (label, condition, path) in [
        (
            "a second relationship under the quantifier",
            "forall(a in M::CollectionNodes : reaches(a, a, kids) and reaches(a, a, litter))"
                .to_owned(),
            vec![1, 0],
        ),
        (
            "more nested binders than the oracle unrolls",
            deep,
            vec![1, 1, 1, 1, 1],
        ),
    ] {
        let authored = fixture(&condition);
        assert!(
            refused_with(
                &disposition(&authored.package, &path, 8),
                &GraphUnsupportedReason::UnsupportedClaimShape
            ),
            "{label}"
        );
    }
}

/// One structure declared under two clause names is one package node with two source
/// occurrences. The claim is keyed by the node and its occurrence, so each occurrence is its own
/// item with its own identity, and a clause with another structure is another node.
#[trace("TC-025", "FR-015-AC-107")]
#[trace("TC-025", "FR-015-AC-100")]
#[test]
fn one_node_under_two_clauses_is_two_items_with_different_identities() {
    let condition = "not reaches(self, self, kids)";
    let authored = authored_declarations(
        &[
            declaration("invariant", "Graph", condition),
            declaration("invariant", "Other", condition),
            declaration("invariant", "Third", "not reaches(self, self, litter)"),
        ],
        Some(3),
    );
    let package = &authored.package;
    let clauses: Vec<_> = package
        .resolved_graph()
        .nodes
        .iter()
        .filter(|node| node.semantic_form.as_ref() == "state_clause")
        .collect();
    assert_eq!(clauses.len(), 2, "two structures, three declarations");
    let shared = clauses
        .iter()
        .find(|node| node.occurrences.len() == 2)
        .expect("the shared structure is one node with two occurrences");
    let (shared_id, ordinals) = (shared.node_id.clone(), shared.occurrences.clone());
    assert_ne!(ordinals[0], ordinals[1]);

    let items: Vec<ObligationItem<'_>> = ordinals
        .iter()
        .map(|occurrence| ObligationItem::GraphClaim {
            package,
            node_id: &shared_id,
            occurrence,
            argument_path: &[0],
        })
        .collect();
    let KaniObligationOutcome::Emitted {
        records,
        graph_harnesses,
        ..
    } = negotiate_kani_obligations(&KaniObligationRequest {
        ceilings: ceilings(),
        items: &items,
        subject_path: "crate::graph_subject",
        unwind: 8,
    })
    .unwrap()
    else {
        panic!("a valid request")
    };
    assert!(
        records
            .iter()
            .all(|record| matches!(record.disposition, ObligationDisposition::Supported { .. })),
        "{:?}",
        records
            .iter()
            .map(|record| &record.disposition)
            .collect::<Vec<_>>()
    );
    let [first, second] = [&graph_harnesses[0].identity, &graph_harnesses[1].identity];
    assert_eq!(first.clause_id, second.clause_id);
    assert_eq!(first.occurrence, ordinals[0]);
    assert_eq!(second.occurrence, ordinals[1]);
    assert_ne!(first, second);
    assert_ne!(first.harness_symbol, second.harness_symbol);

    // The same node, occurrence and path twice is a duplicate item, which rejects the request.
    let repeated: Vec<ObligationItem<'_>> = (0..2)
        .map(|_| ObligationItem::GraphClaim {
            package,
            node_id: &shared_id,
            occurrence: &ordinals[0],
            argument_path: &[0],
        })
        .collect();
    let KaniObligationOutcome::Rejected { records } =
        negotiate_kani_obligations(&KaniObligationRequest {
            ceilings: ceilings(),
            items: &repeated,
            subject_path: "crate::graph_subject",
            unwind: 8,
        })
        .unwrap()
    else {
        panic!("a repeated claim rejects the request")
    };
    assert!(matches!(
        records[1].disposition,
        ObligationDisposition::InvalidRequest { .. }
    ));
    // An occurrence the node does not carry names no claim.
    let absent = CheckedOccurrence {
        role: ordinals[0].role.clone(),
        ordinal: 99,
    };
    let (dispositions, harnesses) = dispositions_at(package, &[(&shared_id, &absent, &[0])]);
    assert!(harnesses.is_empty());
    assert!(refused_with(
        &dispositions[0],
        &GraphUnsupportedReason::MissingClaim
    ));
}
