//! Authored QSL symbolic graph controls (FR-015).
use ix_trace_rs::trace;
use qsl_replay::{call_site, DependencyInput, Identifier, QualifiedName, SourceIdentity};
use quire_contract_model::{
    CheckedPackageEvidence, CheckedPackageReadLimits, CheckedPackageV2, CheckedPackageV2ReadResult,
};
use serde_json::{json, Value};

fn fixture(edge: &str) -> CheckedPackageV2 {
    fixture_condition(edge, "not reaches(self, self, EDGE)", 3, true)
}

fn fixture_condition(edge: &str, condition: &str, maximum: u64, bounded: bool) -> CheckedPackageV2 {
    fixture_data(edge, condition, maximum, bounded).package
}
struct AuthoredGraph {
    package: CheckedPackageV2,
    emitted_bytes: Vec<u8>,
    source: String,
    model_bytes: Vec<u8>,
    model_digest: [u8; 32],
    site: qsl_replay::ClauseSite,
    package_id: qsl_replay::DigestRecord,
}
fn fixture_data(edge: &str, condition: &str, maximum: u64, bounded: bool) -> AuthoredGraph {
    fixture_source(edge, condition, maximum, bounded, "invariant", "")
}
fn fixture_source(
    edge: &str,
    condition: &str,
    maximum: u64,
    bounded: bool,
    form: &str,
    extra: &str,
) -> AuthoredGraph {
    try_fixture_source(edge, condition, maximum, bounded, form, extra).unwrap()
}
fn try_fixture_source(
    edge: &str,
    condition: &str,
    maximum: u64,
    bounded: bool,
    form: &str,
    extra: &str,
) -> Result<AuthoredGraph, qsl_replay::CallSiteRefusal> {
    let mut document = super::kani_obligations_symbolic_domains::model_document();
    document["populations"][0]["displayName"] = json!("CollectionNodes");
    let relation = |name: &str, lower: u64, upper: u64, unique: bool| {
        json!({
            "identity": format!("ix://ir268.test/domain/relationship/Node-{name}"),
            "category": "structural", "composite": false, "direction": "source-to-target",
            "origin": {"source": {"sourceIdentity": "ix://ir268.test/spec", "path": "spec.qspec", "startLine": 1, "startColumn": 1}},
            "sourceEnd": {"type": "ix://ir268.test/domain/Node", "role": name, "multiplicity": {"lower":0,"upper":9,"ordered":false,"unique":unique}},
            "targetEnd": {"type": "ix://ir268.test/domain/Node", "multiplicity": {"lower":lower,"upper":upper,"ordered":false,"unique":unique}}
        })
    };
    let mut wild = relation("wild", 0, 3, false);
    wild["targetEnd"]["multiplicity"]
        .as_object_mut()
        .unwrap()
        .remove("upper");
    document["types"][1]["operations"] = json!([{ "identity":"ix://ir268.test/domain/Node/visit","name":"visit","params":[],"pre":[],"post":[],"origin":{"source":{"sourceIdentity":"ix://ir268.test/spec","path":"spec.qspec","startLine":1,"startColumn":1}},"frame":{"modifies":[],"creates":[],"deletes":[]} }]);
    if condition.contains("target") {
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
        document["types"][1]["operations"][0]["params"] = json!([target]);
    }
    document["types"][1]["relationships"] = json!([
        relation("step", 0, 1, true),
        relation("kids", 0, 3, true),
        relation("litter", 0, 3, false),
        relation("two", 2, 3, false),
        relation("empty", 0, 0, true),
        relation("loop", 1, 1, true),
        wild
    ]);
    if form == "subtype" {
        let mut child = document["types"][1].clone();
        child["identity"] = json!("ix://ir268.test/domain/SpecialNode");
        child["displayName"] = json!("SpecialNode");
        child["supertypes"] = json!(["ix://ir268.test/domain/Node"]);
        child["fields"] = json!([]);
        child["relationships"] = json!([]);
        child["operations"] = json!([]);
        document["types"].as_array_mut().unwrap().push(child);
    }
    let limits = quire_canonical::Limits::new(1 << 20);
    let bytes = quire_canonical::to_vec(&document, limits).unwrap();
    let digest = quire_canonical::sha256(&document, limits)
        .unwrap()
        .to_string();
    let bound = if bounded {
        format!("bound Nodes on M::CollectionNodes [{maximum}]\n")
    } else {
        String::new()
    };
    let condition = condition.replace("EDGE", edge);
    let declaration = if form == "subtype" {
        format!("invariant Graph using v on M::SpecialNode at current {{ {condition} }}")
    } else if form == "invariant" {
        format!("invariant Graph using v on M::Node at current {{ {condition} }}")
    } else {
        format!("{form} Graph using v on M::Node::visit {{ {condition} }}")
    };
    let source = format!("language \"ix:native\" edition \"1-draft\";\nprofile v = \"quire.model.complete/v1\";\nmodel M = \"ir268.test/domain\" version \"1.0.0\" digest \"sha256-jcs:{digest}\";\n{bound}predicate inspect using v (input: M::Node): Boolean {{ true }}\n{declaration}\n{extra}\n");
    let selected = qsl_replay::ClauseName(Identifier::new("Graph").unwrap());
    let site = call_site(
        SourceIdentity::new("agent-ix", "graph.native", "git", "fixture"),
        "graph.native",
        source.as_bytes(),
        [bytes.as_slice()],
        &DependencyInput::default(),
        &selected,
    )
    .map_err(|refusal| *refusal)?;
    let mut evidence = CheckedPackageEvidence::new();
    evidence.support_feature("quire.value.complete/v1");
    evidence
        .insert_domain_package_document(digest, bytes.clone())
        .unwrap();
    let package = match CheckedPackageV2::read(
        &site.package,
        CheckedPackageReadLimits::bounded(),
        &evidence,
    ) {
        CheckedPackageV2ReadResult::Admitted(package) => *package,
        other => panic!("strict IR admission: {other:?}"),
    };
    let model_digest = *quire_canonical::sha256(&document, limits)
        .unwrap()
        .as_bytes();
    Ok(AuthoredGraph {
        package,
        emitted_bytes: site.package,
        source,
        model_bytes: bytes,
        model_digest,
        site: site.site,
        package_id: site.package_id,
    })
}
use quire_contract_codegen::{
    negotiate_kani_obligations, GraphUnsupportedReason, KaniGraphObligationHarness,
    KaniObligationOutcome, KaniObligationRequest, ObligationDisposition, ObligationItem,
    UnsupportedObligation,
};
use quire_contract_model::CheckedNodeId;
use std::time::Duration;
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
fn outcome(package: &CheckedPackageV2, path: &[usize]) -> KaniObligationOutcome {
    outcome_at(package, &clause(package), path)
}
fn outcome_at(
    package: &CheckedPackageV2,
    clause: &CheckedNodeId,
    path: &[usize],
) -> KaniObligationOutcome {
    negotiate_kani_obligations(&KaniObligationRequest {
        ceilings: crate::common::proof_ceilings::proof_ceilings_with_wall_clock(
            Duration::from_secs(600),
        ),
        items: &[ObligationItem::GraphClaim {
            package,
            clause_id: clause,
            argument_path: path,
        }],
        subject_path: "crate::graph_subject",
        unwind: 8,
    })
    .unwrap()
}
fn harness(package: &CheckedPackageV2, path: &[usize]) -> KaniGraphObligationHarness {
    harness_at(package, &clause(package), path)
}
fn harness_at(
    package: &CheckedPackageV2,
    clause: &CheckedNodeId,
    path: &[usize],
) -> KaniGraphObligationHarness {
    let KaniObligationOutcome::Emitted {
        records,
        mut graph_harnesses,
        ..
    } = outcome_at(package, clause, path)
    else {
        panic!("graph request is valid")
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
#[trace("FR-015-AC-98", "FR-015-AC-105", "TC-025")]
#[test]
fn authored_relationship_domains_use_population_and_typed_destination_multiplicity() {
    for (edge, lower, upper, unique) in [
        ("step", 0, 1, true),
        ("kids", 0, 3, true),
        ("litter", 0, 3, false),
        ("two", 2, 3, false),
        ("empty", 0, 0, true),
        ("loop", 1, 1, true),
    ] {
        let package = fixture(edge);
        let harness = harness(&package, &[0]);
        assert_eq!(harness.identity.maximum_population, 3);
        assert_eq!(
            (
                harness.identity.edge.lower,
                harness.identity.edge.upper,
                harness.identity.edge.unique
            ),
            (lower, upper, unique)
        );
        assert!(harness
            .rust
            .contents
            .contains("object_count >= 0_i64 && object_count <= 3_i64"));
        assert!(harness
            .rust
            .contents
            .contains(&format!("degree_0 >= {lower}")));
        assert_eq!(
            harness.rust.contents.contains("edge_0_1 != edge_0_0"),
            unique && upper > 1
        );
        assert!(
            harness.rust.contents.rfind("kani::cover!").unwrap()
                > harness.rust.contents.rfind("assert!").unwrap()
        );
        assert!(!harness.rust.contents.contains("max_expansions"));
    }
}
#[trace("FR-015-AC-107", "FR-015-AC-108", "TC-025")]
#[test]
fn actual_field_producer_has_no_population_link_and_location_refuses_first() {
    for edge in ["next", "maybe", "many"] {
        let package = fixture(edge);
        let reach = package
            .graph()
            .nodes
            .iter()
            .find(|node| {
                node.body
                    .pointer("/operation/identity")
                    .and_then(Value::as_str)
                    == Some("quire.op.model.reaches_field")
            })
            .unwrap();
        assert!(reach.body.pointer("/operation/member/population").is_none());
        for (path, expected) in [
            (&[0][..], GraphUnsupportedReason::PopulationUnavailable),
            (&[1][..], GraphUnsupportedReason::MissingClaim),
        ] {
            let KaniObligationOutcome::Emitted {
                records,
                graph_harnesses,
                ..
            } = outcome(&package, path)
            else {
                panic!("valid per-item refusal")
            };
            assert!(graph_harnesses.is_empty());
            assert!(
                matches!(&records[0].disposition,ObligationDisposition::Unsupported { reason: UnsupportedObligation::GraphClaimNotRendered { reason,.. }} if reason==&expected)
            );
        }
    }
}
#[trace("FR-015-AC-100", "FR-015-AC-107", "TC-025")]
#[test]
fn repeated_occurrences_keep_their_paths_and_equal_inputs_regenerate_identically() {
    let package = fixture_condition(
        "kids",
        "reaches(self, self, EDGE) or not reaches(self, self, EDGE)",
        3,
        true,
    );
    let first = harness(&package, &[0]);
    let second = harness(&package, &[1, 0]);
    assert_ne!(first.identity, second.identity);
    assert_eq!(first, harness(&package, &[0]));
    let changed = fixture_condition(
        "kids",
        "reaches(self, self, EDGE) or not reaches(self, self, EDGE)",
        2,
        true,
    );
    assert_ne!(first.identity, harness(&changed, &[0]).identity);
}
#[trace("FR-015-AC-102", "FR-015-AC-107", "TC-025")]
#[test]
fn refused_graph_sibling_leaves_supported_artifact_byte_identical() {
    let package = fixture("kids");
    let clause = clause(&package);
    let items = [
        ObligationItem::GraphClaim {
            package: &package,
            clause_id: &clause,
            argument_path: &[0],
        },
        ObligationItem::GraphClaim {
            package: &package,
            clause_id: &clause,
            argument_path: &[9],
        },
    ];
    let KaniObligationOutcome::Emitted {
        records,
        graph_harnesses,
        ..
    } = negotiate_kani_obligations(&KaniObligationRequest {
        ceilings: crate::common::proof_ceilings::proof_ceilings_with_wall_clock(
            Duration::from_secs(600),
        ),
        items: &items,
        subject_path: "crate::graph_subject",
        unwind: 8,
    })
    .unwrap()
    else {
        panic!("mixed graph request settles")
    };
    assert_eq!(records.len(), 2);
    assert_eq!(graph_harnesses, vec![harness(&package, &[0])]);
}
#[trace("FR-015-AC-108", "TC-025")]
#[test]
fn authored_unbound_relationship_refuses_unavailable_without_inventing_a_binding() {
    let package = fixture_condition("kids", "not reaches(self, self, EDGE)", 3, false);
    let KaniObligationOutcome::Emitted {
        records,
        graph_harnesses,
        ..
    } = outcome(&package, &[0])
    else {
        panic!("settles")
    };
    assert!(graph_harnesses.is_empty());
    assert!(matches!(
        &records[0].disposition,
        ObligationDisposition::Unsupported {
            reason: UnsupportedObligation::GraphClaimNotRendered {
                reason: GraphUnsupportedReason::PopulationUnavailable,
                ..
            }
        }
    ));
}

fn native_clause(fixture: &AuthoredGraph, edge: &str, targets: &[Vec<usize>]) -> bool {
    native_roles(fixture, &[(edge, targets)])
}
fn native_roles(fixture: &AuthoredGraph, roles: &[(&str, &[Vec<usize>])]) -> bool {
    use qsl_replay::{
        AnchorKind, ByteDigest, ClauseSelectionInput, DigestDomain, DigestRecord, DocumentRef,
        EvaluatedValue, ReplayLimits, ReplayRequestWire, ReplayResult, ReplaySource,
        SelectedAnchor, SelectedObject, StateClauseCounterexample, StateEnvironment,
        WitnessEnvelope, WitnessPacket,
    };
    use std::collections::BTreeMap;
    let targets = roles[0].1;
    assert!(roles.iter().all(|(_, graph)| graph.len() == targets.len()));
    let selected = |index: usize| json!({"population":"ix://ir268.test/domain/CollectionNodes","key":format!("node{index}")});
    let objects: Vec<_> = targets.iter().enumerate().map(|(index,_)| json!({"key":format!("node{index}"),"type":"ix://ir268.test/domain/Node","fields":{
        "items":{"sequence":[]},"flags":{"sequence":[]},"next":{"reference":selected(index)},"maybe":{"absent":{}},"many":{"sequence":[]}
    }})).collect();
    let relationships: Vec<_> = roles.iter().map(|(edge, graph)| {
        let instances:Vec<_> = graph.iter().enumerate().flat_map(|(from, targets)| targets.iter().map(move |to| json!({"source":selected(from),"target":selected(*to)}))).collect();
        json!({"relationship":format!("ix://ir268.test/domain/relationship/Node-{edge}"),"complete":true,"instances":instances})
    }).collect();
    let snapshot = json!({"format":"quire.state.snapshot/v1","identity":{"authority":"agent-ix","identity":"graph-current","revision_namespace":"git","revision":"fixture"},"observation":"current","anchor":{"kind":"initialization","name":"init"},"model":{"identity":"ir268.test/domain","version":"1.0.0","digest":format!("sha256-jcs:{}",fixture.model_digest.iter().map(|byte|format!("{byte:02x}")).collect::<String>())},"populations":[{"population":"ix://ir268.test/domain/CollectionNodes","complete":true,"objects":objects}],"relationships":relationships});
    let limits = quire_canonical::Limits::new(1 << 20);
    let snapshot_bytes = quire_canonical::to_vec(&snapshot, limits).unwrap();
    let snapshot_digest = *quire_canonical::sha256(&snapshot, limits)
        .unwrap()
        .as_bytes();
    let source_digest = DigestRecord::mint(
        DigestDomain::SourceBytesV1,
        ByteDigest::of(fixture.source.as_bytes()).as_bytes(),
    );
    let model_digest = DigestRecord::mint(DigestDomain::Sha256Jcs, fixture.model_digest);
    let snapshot_record = DigestRecord::mint(DigestDomain::Sha256Jcs, snapshot_digest);
    let selected_function = QualifiedName::new(vec![Identifier::new("inspect").unwrap()]).unwrap();
    let accounting = super::kani_obligations_symbolic_domains::replay_scalar_limits();
    let wire = ReplayRequestWire {
        profile_selections: vec![],
        package_id: (
            Some(DigestDomain::PackageSemanticV2.as_str().into()),
            fixture.package_id.hex(),
        ),
        source_digests: vec![(
            "agent-ix".into(),
            "graph.native".into(),
            "git".into(),
            "fixture".into(),
            Some(DigestDomain::SourceBytesV1.as_str().into()),
            source_digest.hex(),
        )],
        dependencies: vec![],
        selected_function: selected_function.clone(),
        source: ReplaySource::Input(vec![]),
        obligation_identity: [0; 32],
        backend: "graph-native-control".into(),
        state_environment: StateEnvironment::new(vec![]),
        accounting_limits: accounting,
        stage_limits: BTreeMap::new(),
        supplied_limits: None,
        object_environment: None,
        declared_domains: vec![],
        byte_provision: vec![
            (
                Some(DigestDomain::SourceBytesV1.as_str().into()),
                source_digest.hex(),
                fixture.source.as_bytes().to_vec(),
            ),
            (
                Some(DigestDomain::Sha256Jcs.as_str().into()),
                model_digest.hex(),
                fixture.model_bytes.clone(),
            ),
            (
                Some(DigestDomain::Sha256Jcs.as_str().into()),
                snapshot_record.hex(),
                snapshot_bytes,
            ),
        ],
    };
    let packet = WitnessPacket {
        obligation_identity: Some([0; 32]),
        occurrence_key: Some(fixture.site.occurrence.clone()),
        clause_node: Some(fixture.site.node),
        selected_function: Some(selected_function),
        package_id: Some(wire.package_id.clone()),
        source_digests: Some(wire.source_digests.clone()),
        profile_selections: Some(vec![]),
        run_limits: Some(accounting),
        declared_domains: Some(vec![]),
        backend: Some(wire.backend.clone()),
        trace_position: Some(None),
        source: Some(ReplaySource::Input(vec![])),
        family_payload: Some(StateClauseCounterexample {
            clause: Identifier::new("Graph").unwrap(),
            observation: ClauseSelectionInput::Current {
                snapshot: DocumentRef {
                    authority: "agent-ix".into(),
                    identity: "graph-current".into(),
                    revision_namespace: "git".into(),
                    revision: "fixture".into(),
                    digest: snapshot_digest,
                },
                anchor: SelectedAnchor {
                    kind: AnchorKind::Initialization,
                    name: "init".into(),
                },
                self_object: Some(SelectedObject {
                    population: "ix://ir268.test/domain/CollectionNodes".into(),
                    key: "node0".into(),
                }),
            },
            witness: None,
        }),
    };
    let envelope = WitnessEnvelope::reconstruct(packet, ReplayLimits::default()).unwrap();
    let replay = qsl_replay::replay_state_clause(wire, &envelope, ReplayLimits::default()).unwrap();
    let ReplayResult::Input(input) = replay.result() else {
        panic!("native input replay")
    };
    let Some(EvaluatedValue::Boolean(value)) = input.value() else {
        panic!("native authored graph clause value: {:?}", replay.result())
    };
    #[derive(serde::Serialize)]
    struct NativeObservation<'a> {
        clause: &'a str,
        value: bool,
        snapshot: &'a Value,
    }
    let directory =
        std::path::PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("kani-graph-evidence");
    std::fs::create_dir_all(&directory).unwrap();
    let label = format!(
        "native-{}-{}",
        fixture.package_id.hex(),
        snapshot_record.hex()
    );
    std::fs::write(
        directory.join(format!("{label}.json")),
        serde_json::to_vec_pretty(&NativeObservation {
            clause: fixture.site.name.as_str(),
            value,
            snapshot: &snapshot,
        })
        .unwrap(),
    )
    .unwrap();
    value
}
#[trace("FR-015-AC-98", "FR-015-AC-106", "TC-025")]
#[test]
fn same_authored_clause_native_replay_distinguishes_isolation_self_loop_and_selected_role() {
    for edge in ["step", "kids", "litter"] {
        let fixture = fixture_data(edge, "not reaches(self, self, EDGE)", 3, true);
        assert!(native_clause(&fixture, edge, &[vec![]]));
        assert!(!native_clause(&fixture, edge, &[vec![0]]));
        assert!(native_clause(&fixture, edge, &[vec![1], vec![]]));
        assert!(!native_clause(&fixture, edge, &[vec![1], vec![0]]));
        if edge == "litter" {
            assert!(!native_clause(&fixture, edge, &[vec![0, 0, 0]]));
        }
    }
}

fn run_graph_kani(
    harness: &KaniGraphObligationHarness,
    label: &str,
) -> quire_contract_codegen::KaniExecutionEvidence {
    use super::scratch_crate::{write_manifest, TemporaryDirectory};
    use quire_contract_codegen::{
        execute_kani_obligation, KaniExecutableHarness, KaniExecutionRequest, KaniInstallation,
    };
    use std::{fs, path::PathBuf};
    let scratch = TemporaryDirectory::new("quire-symbolic-graph");
    write_manifest(&scratch.path,"[package]\nname = \"generated-symbolic-graph\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n[workspace]\n");
    fs::write(scratch.path.join("src/lib.rs"), &harness.rust.contents).unwrap();
    fs::write(
        scratch.path.join("build.rs"),
        "fn main() { println!(\"cargo:rustc-check-cfg=cfg(kani)\"); }\n",
    )
    .unwrap();
    let installation = KaniInstallation::discover().expect("installed Kani");
    let cancel = quire_exact::Cancel::new();
    let evidence = execute_kani_obligation(&KaniExecutionRequest {
        cancel: &cancel,
        installation: &installation,
        harness: KaniExecutableHarness::from(harness),
        crate_directory: &scratch.path,
        target_directory: &PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("kani-graph"),
        home_directory: None,
    })
    .unwrap_or_else(|error| panic!("{label}: {error}"));
    let directory = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("kani-graph-evidence");
    fs::create_dir_all(&directory).unwrap();
    fs::write(
        directory.join(format!("{label}.json")),
        serde_json::to_vec_pretty(&evidence).unwrap(),
    )
    .unwrap();
    fs::write(
        directory.join(format!("{label}.rs")),
        &harness.rust.contents,
    )
    .unwrap();
    evidence
}
#[trace("FR-015-AC-106", "TC-025")]
#[test]
#[ignore = "kani lane: run serially through make kani-gate"]
fn tc_025_real_kani_graph_relationship_claim_verifies_and_negated_claim_falsifies() {
    use quire_contract_codegen::KaniRunOutcome;
    for edge in ["step", "kids", "litter"] {
        let healthy = fixture_data(
            edge,
            "reaches(self, self, EDGE) or not reaches(self, self, EDGE)",
            3,
            true,
        );
        retain_authored(&healthy, &format!("{edge}-healthy"));
        let healthy_harness = harness(&healthy.package, &[0]);
        let verified = run_graph_kani(&healthy_harness, &format!("{edge}-healthy"));
        assert_eq!(verified.outcome, KaniRunOutcome::Verified);
        assert!(native_clause(&healthy, edge, &[vec![]]));
        assert!(native_clause(&healthy, edge, &[vec![0]]));
        let mutant = fixture_data(
            edge,
            "not (reaches(self, self, EDGE) or not reaches(self, self, EDGE))",
            3,
            true,
        );
        retain_authored(&mutant, &format!("{edge}-negated"));
        let falsified = run_graph_kani(
            &harness(&mutant.package, &[0, 0]),
            &format!("{edge}-negated"),
        );
        let KaniRunOutcome::Falsified { counterexample } = falsified.outcome else {
            panic!("negated admitted claim falsifies: {:?}", falsified.outcome)
        };
        assert!(
            counterexample.contains("the symbolic graph satisfies the admitted clause condition")
        );
        let values = playback_values(&counterexample);
        assert!(
            values[0] > 0 && values[0] <= 3,
            "nonempty in-population playback"
        );
        assert!(values[1] >= 0 && values[1] < values[0]);
        assert_eq!(values[1], values[2]);
        let upper = usize::try_from(healthy_harness.identity.edge.upper).unwrap();
        assert_eq!(values.len(), 3 + 3 * (upper + 1));
        let mut targets = Vec::new();
        for row in 0..usize::try_from(values[0]).unwrap() {
            let degree = usize::try_from(values[3 + row * (upper + 1)]).unwrap();
            assert!(degree <= upper);
            let edges = values[4 + row * (upper + 1)..4 + row * (upper + 1) + degree]
                .iter()
                .map(|target| {
                    assert!(*target >= 0 && *target < values[0]);
                    usize::try_from(*target).unwrap()
                })
                .collect::<Vec<_>>();
            if healthy_harness.identity.edge.unique {
                let distinct = edges.iter().collect::<std::collections::BTreeSet<_>>();
                assert_eq!(distinct.len(), edges.len());
            }
            targets.push(edges);
        }
        // The clause uses self for both operands; replay the actual selected object by moving
        // it to index zero and preserving every target identity under the same permutation.
        let start = usize::try_from(values[1]).unwrap();
        if start != 0 {
            targets.swap(0, start);
            for row in &mut targets {
                for target in row {
                    if *target == 0 {
                        *target = start;
                    } else if *target == start {
                        *target = 0;
                    }
                }
            }
        }
        assert!(!native_clause(&mutant, edge, &targets));
    }
    let zero = fixture_data("kids", "not reaches(self,self,EDGE)", 0, true);
    retain_authored(&zero, "zero-population-healthy");
    assert_eq!(
        run_graph_kani(
            &harness_at(&zero.package, &main_clause(&zero), &[0]),
            "zero-population-healthy"
        )
        .outcome,
        KaniRunOutcome::Verified
    );
}
fn playback_values(counterexample: &str) -> Vec<i64> {
    let (_, rest) = counterexample
        .split_once("let concrete_vals: Vec<Vec<u8>> = vec![")
        .expect("actual Kani concrete byte vectors");
    let (body, _) = rest
        .split_once("]; ")
        .or_else(|| rest.split_once("];\n"))
        .expect("end vectors");
    body.split("vec![")
        .skip(1)
        .map(|row| {
            let (bytes, _) = row.split_once(']').unwrap();
            let bytes = bytes
                .split(',')
                .filter_map(|byte| {
                    let byte = byte.trim();
                    if byte.is_empty() {
                        None
                    } else {
                        Some(byte.parse::<u8>().unwrap())
                    }
                })
                .collect::<Vec<_>>();
            i64::from_le_bytes(bytes.try_into().expect("i64 slot"))
        })
        .collect()
}

#[trace("FR-015-AC-101", "FR-015-AC-105", "TC-025")]
#[test]
fn zero_population_is_retained_and_inadequate_resources_refuse_without_domain_reduction() {
    let zero = fixture_condition("kids", "not reaches(self, self, EDGE)", 0, true);
    let generated = harness(&zero, &[0]);
    assert_eq!(generated.identity.maximum_population, 0);
    assert!(generated.rust.contents.contains("degrees: &[i64; 0]"));
    let package = fixture("kids");
    let clause = clause(&package);
    let item = ObligationItem::GraphClaim {
        package: &package,
        clause_id: &clause,
        argument_path: &[0],
    };
    let normal = crate::common::proof_ceilings::proof_ceilings();
    for (ceilings, unwind, expected) in [
        (
            normal,
            3,
            GraphUnsupportedReason::UnwindBelowDomain {
                required: 4,
                supplied: 3,
            },
        ),
        (
            quire_contract_codegen::ProofCeilings {
                memory_bytes: std::num::NonZeroU64::new(1).unwrap(),
                ..normal
            },
            8,
            GraphUnsupportedReason::DomainExceedsMemoryCeiling {
                required: 129,
                supplied: 1,
            },
        ),
    ] {
        let KaniObligationOutcome::Emitted {
            records,
            graph_harnesses,
            ..
        } = negotiate_kani_obligations(&KaniObligationRequest {
            ceilings,
            items: &[item],
            subject_path: "crate::graph_subject",
            unwind,
        })
        .unwrap()
        else {
            panic!("per-item resource refusal")
        };
        assert!(graph_harnesses.is_empty());
        assert!(
            matches!(&records[0].disposition,ObligationDisposition::Unsupported { reason:UnsupportedObligation::GraphClaimNotRendered { reason,.. }} if reason==&expected)
        );
    }
}

fn run_seeded_graph(
    harness: &KaniGraphObligationHarness,
    source: &str,
    label: &str,
) -> quire_contract_codegen::KaniRunOutcome {
    use super::scratch_crate::{write_manifest, TemporaryDirectory};
    use std::{fs, path::PathBuf, process::Command};
    let scratch = TemporaryDirectory::new("quire-seeded-graph");
    write_manifest(&scratch.path,"[package]\nname = \"seeded-symbolic-graph\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n[workspace]\n");
    syn::parse_file(source).expect("seeded generated Rust remains valid");
    fs::write(scratch.path.join("src/lib.rs"), source).unwrap();
    fs::write(
        scratch.path.join("build.rs"),
        "fn main() { println!(\"cargo:rustc-check-cfg=cfg(kani)\"); }\n",
    )
    .unwrap();
    let report = scratch.path.join("report.json");
    let launcher = std::env::var_os("KANI_GATE_LAUNCHER").unwrap_or_else(|| "cargo-kani".into());
    let output = Command::new("timeout")
        .arg("600")
        .arg(launcher)
        .arg("kani")
        .args(&harness.identity.options)
        .args(["-Z", "unstable-options", "--export-json"])
        .arg(&report)
        .env(
            "CARGO_TARGET_DIR",
            PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("kani-graph-mutants"),
        )
        .current_dir(&scratch.path)
        .output()
        .expect("installed Kani mutant execution");
    let console = format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let report = fs::read(report)
        .unwrap_or_else(|error| panic!("{label}: backend exported no report: {error}: {console}"));
    let classified = quire_contract_codegen::classify_kani_run(
        output.status.success(),
        Some(&report),
        &console,
        None,
    )
    .unwrap();
    let directory = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("kani-graph-evidence");
    fs::create_dir_all(&directory).unwrap();
    fs::write(directory.join(format!("{label}.report.json")), report).unwrap();
    fs::write(directory.join(format!("{label}.console")), console).unwrap();
    fs::write(directory.join(format!("{label}.rs")), source).unwrap();
    classified.outcome
}

fn retain_authored(fixture: &AuthoredGraph, label: &str) {
    let directory =
        std::path::PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("kani-graph-evidence");
    std::fs::create_dir_all(&directory).unwrap();
    std::fs::write(directory.join(format!("{label}.native")), &fixture.source).unwrap();
    std::fs::write(
        directory.join(format!("{label}.model.json")),
        &fixture.model_bytes,
    )
    .unwrap();
    std::fs::write(
        directory.join(format!("{label}.package.json")),
        &fixture.emitted_bytes,
    )
    .unwrap();
}

fn main_clause(fixture: &AuthoredGraph) -> CheckedNodeId {
    fixture
        .package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_id.digest.as_ref() == fixture.site.node.to_string())
        .unwrap()
        .node_id
        .clone()
}
fn playback_graph(harness: &KaniGraphObligationHarness, counterexample: &str) -> Vec<Vec<usize>> {
    assert!(counterexample.contains("the symbolic graph satisfies the admitted clause condition"));
    let values = playback_values(counterexample);
    let maximum = usize::try_from(harness.identity.maximum_population).unwrap();
    let upper = usize::try_from(harness.identity.edge.upper).unwrap();
    assert_eq!(values.len(), 3 + maximum * (upper + 1));
    let count = usize::try_from(values[0]).unwrap();
    assert!(count > 0 && count <= maximum);
    let start = usize::try_from(values[1]).unwrap();
    assert!(start < count);
    assert_eq!(values[1], values[2]);
    let mut graph = Vec::new();
    for row in 0..count {
        let degree = usize::try_from(values[3 + row * (upper + 1)]).unwrap();
        assert!(degree >= usize::try_from(harness.identity.edge.lower).unwrap() && degree <= upper);
        let edges = values[4 + row * (upper + 1)..4 + row * (upper + 1) + degree]
            .iter()
            .map(|target| {
                let target = usize::try_from(*target).unwrap();
                assert!(target < count);
                target
            })
            .collect::<Vec<_>>();
        if harness.identity.edge.unique {
            assert_eq!(
                edges
                    .iter()
                    .collect::<std::collections::BTreeSet<_>>()
                    .len(),
                edges.len()
            );
        }
        graph.push(edges);
    }
    if start != 0 {
        graph.swap(0, start);
        for row in &mut graph {
            for target in row {
                if *target == 0 {
                    *target = start;
                } else if *target == start {
                    *target = 0;
                }
            }
        }
    }
    graph
}

#[trace("FR-015-AC-98", "FR-015-AC-107", "TC-025")]
#[test]
fn authored_pre_post_and_invariant_forms_keep_the_clause_kind() {
    use quire_contract_codegen::ObligationKind;
    for (form, kind) in [
        ("invariant", ObligationKind::Invariant),
        ("pre", ObligationKind::Precondition),
        ("post", ObligationKind::Postcondition),
    ] {
        let fixture = fixture_source("kids", "not reaches(self, self, EDGE)", 3, true, form, "");
        let generated = harness_at(&fixture.package, &main_clause(&fixture), &[0]);
        assert_eq!(generated.identity.kind, kind);
    }
}
#[trace("FR-015-AC-107", "TC-025")]
#[test]
fn the_same_reachability_under_two_actual_clauses_has_distinct_identities() {
    let fixture = fixture_source(
        "kids",
        "reaches(self, self, EDGE)",
        3,
        true,
        "invariant",
        "invariant Other using v on M::Node at current { not reaches(self,self,kids) }",
    );
    let main = main_clause(&fixture);
    let other = fixture
        .package
        .graph()
        .nodes
        .iter()
        .find(|node| node.semantic_form.as_ref() == "state_clause" && node.node_id != main)
        .unwrap();
    let left = harness_at(&fixture.package, &main, &[]);
    let right = harness_at(&fixture.package, &other.node_id, &[0]);
    assert_eq!(left.identity.application_id, right.identity.application_id);
    assert_ne!(left.identity.clause_id, right.identity.clause_id);
    assert_ne!(left.identity.module_symbol, right.identity.module_symbol);
}
#[trace("TC-025")]
#[test]
fn the_locked_producer_refuses_an_unbounded_relationship_before_ir_admission() {
    let refusal = try_fixture_source(
        "wild",
        "not reaches(self,self,EDGE)",
        3,
        true,
        "invariant",
        "",
    )
    .err()
    .expect("the locked producer does not emit an unbounded relationship claim");
    assert!(matches!(
        refusal,
        qsl_replay::CallSiteRefusal::Compile {
            code: qsl_replay::Code::UnsupportedConstruct,
            ..
        }
    ));
    assert!(refusal.to_string().contains("expression-form"));
}

#[trace("FR-015-AC-106", "TC-025")]
#[test]
#[ignore = "kani lane: run serially through make kani-gate"]
fn tc_025_real_kani_positive_length_evaluator_rejects_zero_length_and_self_loop_mutants() {
    use quire_contract_codegen::KaniRunOutcome;
    for (edge, condition, label, mutated_reachable) in [
        (
            "empty",
            "not reaches(self,self,EDGE)",
            "zero-length",
            "start == target || reachable",
        ),
        (
            "loop",
            "reaches(self,self,EDGE)",
            "self-loop-rejected",
            "start != target && reachable",
        ),
    ] {
        let fixture = fixture_data(edge, condition, 1, true);
        retain_authored(&fixture, &format!("{label}-authored"));
        let generated = harness_at(
            &fixture.package,
            &main_clause(&fixture),
            if edge == "empty" { &[0] } else { &[] },
        );
        assert_eq!(
            run_graph_kani(&generated, &format!("{label}-healthy")).outcome,
            KaniRunOutcome::Verified
        );
        // Change the evaluator for every occurrence, leaving the authored condition intact.
        let needle = "let selected_reachable = reachable;";
        let source = generated.rust.contents.replacen(
            needle,
            &format!("let reachable = {mutated_reachable};\nlet selected_reachable = reachable;"),
            1,
        );
        assert_ne!(source, generated.rust.contents);
        let KaniRunOutcome::Falsified { counterexample } =
            run_seeded_graph(&generated, &source, label)
        else {
            panic!("{label}: the evaluator fault must falsify the authored clause")
        };
        let graph = playback_graph(&generated, &counterexample);
        assert!(native_clause(&fixture, edge, &graph));
        assert_eq!(
            graph,
            if edge == "empty" {
                vec![vec![]]
            } else {
                vec![vec![0]]
            }
        );
    }
}

const OTHER_ROLE: &str = "invariant OtherRole using v on M::Node at current { reaches(self,self,litter) = reaches(self,self,litter) }";
fn role_fixture() -> AuthoredGraph {
    fixture_source(
        "kids",
        "reaches(self,self,EDGE) = reaches(self,self,EDGE)",
        3,
        true,
        "invariant",
        OTHER_ROLE,
    )
}
fn role_control(fixture: &AuthoredGraph, generated: &KaniGraphObligationHarness) -> String {
    let other = fixture
        .package
        .graph()
        .nodes
        .iter()
        .find(|node| {
            node.body
                .pointer("/operation/member/name")
                .and_then(Value::as_str)
                == Some("litter")
                && node
                    .body
                    .pointer("/operation/identity")
                    .and_then(Value::as_str)
                    == Some("quire.op.model.reaches")
        })
        .unwrap();
    let end = fixture.package.relationship_end(&other.node_id).unwrap();
    assert_eq!((end.lower, end.upper, end.unique), (0, Some(3), false));
    let own = fixture
        .package
        .reaches_population(&generated.identity.application_id)
        .unwrap()
        .unwrap();
    assert_eq!(
        fixture
            .package
            .reaches_population(&other.node_id)
            .unwrap()
            .unwrap(),
        own
    );
    let n = generated.identity.maximum_population;
    let upper = end.upper.unwrap();
    // A real second role carrier: its admitted lower bound permits a complete empty extent.
    // The same carrier, traversal, original package and baseline occurrence serve both runs.
    let carrier=format!("let litter_degrees = [0_i64; {n}];\nlet litter_edges = [[0_i64; {upper}]; {n}];\nlet mut litter_paths = [[false; {n}]; {n}];\nfor row in 0..{n} {{ for edge in 0..{upper} {{ if row < count as usize && edge < litter_degrees[row] as usize {{ litter_paths[row][litter_edges[row][edge] as usize] = true; }} }} }}\nfor via in 0..{n} {{ for from in 0..{n} {{ for to in 0..{n} {{ litter_paths[from][to] = litter_paths[from][to] || (litter_paths[from][via] && litter_paths[via][to]); }} }} }}\nlet litter_reachable = if count == 0 {{ false }} else {{ litter_paths[start as usize][target as usize] }};\nlet selected_role = GraphControlRole::Kids;\nlet selected_reachable = match selected_role {{ GraphControlRole::Kids => reachable, GraphControlRole::Litter => litter_reachable }};");
    let source =
        generated
            .rust
            .contents
            .replacen("let selected_reachable = reachable;", &carrier, 1);
    assert_ne!(source, generated.rust.contents);
    format!("{source}\nenum GraphControlRole {{ Kids, Litter }}\n")
}
#[trace("FR-015-AC-98", "FR-015-AC-106", "TC-025")]
#[test]
fn actual_second_role_carrier_and_native_snapshot_preserve_both_relationships() {
    let fixture = role_fixture();
    let generated = harness_at(&fixture.package, &main_clause(&fixture), &[0]);
    let source = role_control(&fixture, &generated);
    assert!(source.contains("litter_paths[row][litter_edges[row][edge] as usize]"));
    assert!(native_roles(
        &fixture,
        &[("kids", &[vec![0]]), ("litter", &[vec![]])]
    ));
}
#[trace("FR-015-AC-106", "TC-025")]
#[test]
#[ignore = "kani lane: run serially through make kani-gate"]
fn tc_025_real_kani_wrong_role_dispatch_falsifies_the_same_authored_clause() {
    use quire_contract_codegen::KaniRunOutcome;
    let fixture = role_fixture();
    retain_authored(&fixture, "wrong-role-authored");
    let generated = harness_at(&fixture.package, &main_clause(&fixture), &[0]);
    assert_eq!(
        run_graph_kani(&generated, "role-base-healthy").outcome,
        KaniRunOutcome::Verified
    );
    let healthy = role_control(&fixture, &generated);
    assert_eq!(
        run_seeded_graph(&generated, &healthy, "role-carrier-healthy"),
        KaniRunOutcome::Verified
    );
    let mutant = healthy.replacen(
        "let selected_role = GraphControlRole::Kids;",
        "let selected_role = GraphControlRole::Litter;",
        1,
    );
    assert_ne!(healthy, mutant);
    let KaniRunOutcome::Falsified { counterexample } =
        run_seeded_graph(&generated, &mutant, "wrong-role")
    else {
        panic!("dispatch to the admitted litter role must falsify")
    };
    let kids = playback_graph(&generated, &counterexample);
    let litter = vec![Vec::new(); kids.len()];
    assert!(native_roles(
        &fixture,
        &[("kids", &kids), ("litter", &litter)]
    ));
    // Both extents are original-domain inputs: kids uses decoded checked degrees and targets;
    // litter is the complete empty extent carried and traversed by both control sources.
    assert!(litter.iter().all(Vec::is_empty));
    let evidence = std::path::PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join("kani-graph-evidence/wrong-role.graphs.json");
    std::fs::write(
        evidence,
        serde_json::to_vec_pretty(
            &json!({"kids":kids,"litter":litter,"originalClause":fixture.site.name.as_str()}),
        )
        .unwrap(),
    )
    .unwrap();
}

#[trace("FR-015-AC-98", "FR-015-AC-105", "TC-025")]
#[test]
fn a_subtype_operand_uses_the_near_ends_covering_population_without_a_subtype_cap() {
    let fixture = fixture_source(
        "kids",
        "not reaches(self,self,EDGE)",
        3,
        true,
        "subtype",
        "",
    );
    let generated = harness_at(&fixture.package, &main_clause(&fixture), &[0]);
    let end = fixture
        .package
        .relationship_end(&generated.identity.application_id)
        .unwrap();
    assert_eq!(generated.identity.object_type, end.destination_type);
    assert_eq!(generated.identity.maximum_population, 3);
    assert!(generated
        .rust
        .contents
        .contains("start < object_count && target < object_count"));
}

#[trace("FR-015-AC-100", "TC-025")]
#[test]
fn graph_identity_binds_subject_unwind_solver_and_complete_ordered_options() {
    let fixture = fixture_data("kids", "not reaches(self,self,EDGE)", 3, true);
    let clause = main_clause(&fixture);
    let item = ObligationItem::GraphClaim {
        package: &fixture.package,
        clause_id: &clause,
        argument_path: &[0],
    };
    let mut identities = Vec::new();
    for (subject, unwind) in [
        ("crate::first", 8),
        ("crate::second", 8),
        ("crate::first", 9),
    ] {
        let KaniObligationOutcome::Emitted {
            mut graph_harnesses,
            ..
        } = negotiate_kani_obligations(&KaniObligationRequest {
            ceilings: crate::common::proof_ceilings::proof_ceilings(),
            items: &[item],
            subject_path: subject,
            unwind,
        })
        .unwrap()
        else {
            panic!("supported graph identity")
        };
        let identity = graph_harnesses.remove(0).identity;
        assert_eq!(identity.subject_path, subject);
        assert_eq!(identity.unwind, unwind);
        let options = &identity.options;
        assert!(options
            .windows(2)
            .any(|pair| pair == ["--solver", "cadical"]));
        assert!(options
            .windows(2)
            .any(|pair| pair == ["--unwind", &unwind.to_string()]));
        assert!(options.windows(2).any(|pair| pair
            == [
                "--harness",
                &format!("{}::{}", identity.module_symbol, identity.harness_symbol)
            ]));
        assert_eq!(options.last().unwrap(), "print");
        identities.push(identity);
    }
    assert_ne!(identities[0], identities[1]);
    assert_ne!(identities[0], identities[2]);
    assert_ne!(identities[0].harness_symbol, identities[1].harness_symbol);
    assert_ne!(identities[0].harness_symbol, identities[2].harness_symbol);
}

#[trace("FR-015-AC-98", "FR-015-AC-105", "TC-025")]
#[test]
fn distinct_operation_operands_are_drawn_independently_from_the_same_population() {
    for form in ["pre", "post"] {
        let fixture = fixture_source("kids", "reaches(self,target,EDGE)", 3, true, form, "");
        let generated = harness_at(&fixture.package, &main_clause(&fixture), &[]);
        assert!(generated
            .rust
            .contents
            .contains("start < object_count && target < object_count"));
        assert!(!generated
            .rust
            .contents
            .contains("kani::assume(start == target)"));
        assert_eq!(generated.identity.maximum_population, 3);
    }
}
#[trace("FR-015-AC-107", "TC-025")]
#[test]
fn a_collection_condition_and_a_clause_without_reachability_refuse_missing_claim() {
    for condition in ["true", "forall(item in self.items: item >= 0)"] {
        let fixture = fixture_data("kids", condition, 3, true);
        let KaniObligationOutcome::Emitted {
            records,
            graph_harnesses,
            ..
        } = outcome_at(&fixture.package, &main_clause(&fixture), &[])
        else {
            panic!("per-item graph location refusal")
        };
        assert!(graph_harnesses.is_empty());
        assert!(matches!(
            &records[0].disposition,
            ObligationDisposition::Unsupported {
                reason: UnsupportedObligation::GraphClaimNotRendered {
                    reason: GraphUnsupportedReason::MissingClaim,
                    ..
                }
            }
        ));
    }
}
