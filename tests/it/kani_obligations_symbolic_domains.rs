//! Source-backed symbolic collection and graph obligation fixtures.
//!
//! QSL emits every admitted package through its public `call_site` facade from
//! native source and a test-authored domain model document. No Contract IR
//! package wire is constructed in this module.

use crate::scratch_crate::{runtime_dependency, write_manifest, TemporaryDirectory};
use ix_trace_rs::trace;
use qsl_replay::{
    call_site, object_reference_witness, replay, write_object_environment, ByteDigest,
    DeclarationKey, DependencyInput, DigestDomain, DigestRecord, DomainPackageRef,
    EnvironmentLabels, EnvironmentModel, EnvironmentObject, EnvironmentPopulation, EvaluatedValue,
    Identifier, ModelNormalizationLimits, QualifiedName, ReplayLimits, ReplayRequestWire,
    ReplayResult, ReplaySource, RequestTargetWire, ScalarLimits, SelectedObject, SnapshotValue,
    SourceIdentity, StateEnvironment, WitnessValue,
};
use quire_contract_codegen::{
    execute_kani_obligation, negotiate_kani_obligations, CollectionElementIdentity,
    CollectionKindIdentity, CollectionPredicateIdentity, KaniCollectionObligationHarness,
    KaniExecutableHarness, KaniExecutionRequest, KaniInstallation, KaniObligationOutcome,
    KaniObligationRequest, KaniRunOutcome, ObligationDisposition, ObligationItem, ProofCeilings,
    UnsupportedObligation,
};
use quire_contract_model::{
    CheckedMemberType, CheckedModelObjectFields, CheckedNodeId, CheckedPackageEvidence,
    CheckedPackageReadLimits, CheckedPackageV2, CheckedPackageV2ReadResult, CheckedSemanticNodeV2,
    CheckedTerm,
};
use quire_semantic_value::catalog::Category;
use serde_json::{json, Value};
use std::{collections::BTreeMap, env, fs, path::PathBuf, time::Duration};

const NODE_ID: &str = "ix://ir268.test/domain/Node";
const SMALL_ID: &str = "ix://ir268.test/domain/SmallInt";
const POPULATION_ID: &str = "ix://ir268.test/domain/CollectionNodes";
const SOURCE_ID: &str = "ix://ir268.test/spec";
const ZERO_SHA256: &str = "sha256:0000000000000000000000000000000000000000000000000000000000000000";

fn application_identity(node: &CheckedSemanticNodeV2) -> Option<&str> {
    let CheckedTerm::Application { operation, .. } = &node.body else {
        return None;
    };
    operation.identity()
}

fn argument_target(node: &CheckedSemanticNodeV2, index: usize) -> Option<&CheckedNodeId> {
    let CheckedTerm::Application { arguments, .. } = &node.body else {
        return None;
    };
    match arguments.get(index)? {
        CheckedTerm::Reference { target } => Some(target),
        _ => None,
    }
}

fn named_member<'a>(node: &'a CheckedSemanticNodeV2, name: &str) -> Option<&'a CheckedTerm> {
    let CheckedTerm::Aggregate { members } = &node.body else {
        return None;
    };
    members.iter().find_map(|member| match member {
        CheckedTerm::Binding {
            name: member_name,
            value,
        } if member_name.as_ref() == name => Some(value.as_ref()),
        _ => None,
    })
}

fn model_document() -> Value {
    let field = |name: &str,
                 type_ref: &str,
                 lower: u64,
                 upper: u64,
                 ordered: bool,
                 unique: bool,
                 nullable: bool,
                 presence: &str| {
        json!({
            "identity": format!("{NODE_ID}/{name}"),
            "name": name,
            "typeRef": type_ref,
            "presence": presence,
            "nullable": nullable,
            "defaultKind": "none",
            "origin": {"generated": {
                "generatorIdentity": SOURCE_ID,
                "generatorVersion": "1.0.0",
                "inputIdentities": [format!("{NODE_ID}/{name}")]
            }},
            "multiplicity": {
                "lower": lower,
                "upper": upper,
                "ordered": ordered,
                "unique": unique
            }
        })
    };
    json!({
        "contractVersion": "2.0.0",
        "canonicalization": {"algorithm": "rfc8785-v1", "digest": "sha256-jcs"},
        "source": {
            "identity": SOURCE_ID,
            "version": "1.0.0",
            "dialect": "spec-bundle",
            "digest": ZERO_SHA256
        },
        "package": {
            "identity": "ir268.test/domain", "version": "1.0.0",
            "manifestDigest": ZERO_SHA256,
            "mappingVersions": [], "profileVersions": [], "lockDigest": ZERO_SHA256
        },
        "constructs": [
            {"kind": {"module": "ir268.test/domain", "name": "entity"},
             "moduleVersion": "1.0.0", "manifestDigest": ZERO_SHA256,
             "construct": {"identity": "none", "shape": "record", "members": {},
                            "meaning": "quire.meaning.model.object-type/v1"}},
            {"kind": {"module": "ir268.test/domain", "name": "value_type"},
             "moduleVersion": "1.0.0", "manifestDigest": ZERO_SHA256,
             "construct": {"identity": "none", "shape": "record", "members": {},
                            "meaning": "quire.meaning.model.value-type/v1"}},
            {"kind": {"module": "ir268.test/domain", "name": "population"},
             "moduleVersion": "1.0.0", "manifestDigest": ZERO_SHA256,
             "construct": {"identity": "none", "shape": "record", "members": {},
                            "meaning": "quire.meaning.model.population/v1"}}
        ],
        "occurrences": [], "extensions": [],
        "types": [
            {
                "identity": SMALL_ID,
                "displayName": "SmallInt",
                "kind": {"module": "ir268.test/domain", "name": "value_type"},
                "roles": [],
                "origin": {"generated": {
                    "generatorIdentity": SOURCE_ID,
                    "generatorVersion": "1.0.0",
                    "inputIdentities": [SMALL_ID]
                }},
                "extensions": [], "unknownPolicy": "reject",
                "scalar": "integer",
                "constraints": [
                    {
                        "identity": "ix://ir268.test/domain/SmallInt/min",
                        "keyword": "min", "operands": {"value": "0"},
                        "appliesTo": "ix://quire/native/Integer", "diagnosticCode": "ir268.minimum",
                        "origin": {"generated": {
                            "generatorIdentity": SOURCE_ID,
                            "generatorVersion": "1.0.0",
                            "inputIdentities": [SMALL_ID]
                        }}
                    },
                    {
                        "identity": "ix://ir268.test/domain/SmallInt/max",
                        "keyword": "max", "operands": {"value": "3"},
                        "appliesTo": "ix://quire/native/Integer", "diagnosticCode": "ir268.maximum",
                        "origin": {"generated": {
                            "generatorIdentity": SOURCE_ID,
                            "generatorVersion": "1.0.0",
                            "inputIdentities": [SMALL_ID]
                        }}
                    }
                ]
            },
            {
                "identity": NODE_ID,
                "displayName": "Node",
                "kind": {"module": "ir268.test/domain", "name": "entity"},
                "roles": [],
                "origin": {"generated": {
                    "generatorIdentity": SOURCE_ID,
                    "generatorVersion": "1.0.0",
                    "inputIdentities": [NODE_ID]
                }},
                "constraints": [], "extensions": [],
                "unknownPolicy": "reject", "supertypes": [],
                "fields": [
                    field("items", SMALL_ID, 0, 3, true, false, false, "required"),
                    field("flags", "ix://quire/native/Boolean", 0, 2, true, false, false, "required"),
                    field("next", NODE_ID, 1, 1, false, true, false, "required"),
                    field("maybe", NODE_ID, 1, 1, false, true, false, "optional"),
                    field("many", NODE_ID, 0, 2, true, false, false, "required")
                ],
                "operations": []
            }
        ],
        "populations": [{
            "identity": POPULATION_ID,
            "displayName": "CollectionNodes",
            "kind": {"module": "ir268.test/domain", "name": "population"},
            "members": [NODE_ID],
            "extent": "closed",
            "origin": {"generated": {
                "generatorIdentity": SOURCE_ID,
                "generatorVersion": "1.0.0",
                "inputIdentities": [POPULATION_ID]
            }}
        }]
    })
}

// `deref(input)` is object navigation, admitted under QSL's model layer.
fn source_unit_with_body(model_digest: &str, body: &str) -> String {
    format!(
        "language \"ix:native\" edition \"1-draft\";\n\
         profile v = \"quire.model.complete/v1\";\n\
         model M = \"ir268.test/domain\" version \"1.0.0\" digest \"sha256-jcs:{model_digest}\";\n\
         predicate inspect using v (input: M::Node): Boolean {{ {body} }}\n"
    )
}

struct Emitted {
    package: CheckedPackageV2,
    function_digest: String,
    object_type: CheckedNodeId,
    fields: CheckedModelObjectFields,
    model_bytes: Vec<u8>,
    model_digest: [u8; 32],
    source: String,
}

fn emitted() -> Emitted {
    emitted_from_document_and_body(
        model_document(),
        "forall(item in deref(input).items: item >= 0) and exists(item in deref(input).items: item = 2)",
    )
}

fn emitted_from_document_and_body(document: Value, body: &str) -> Emitted {
    try_emitted_from_document_and_body(document, body)
        .unwrap_or_else(|error| panic!("QSL/IR admits the source-authored fixture: {error}"))
}

fn collection_identity(
    fixture: &Emitted,
    operation: &str,
    subject_path: &str,
    unwind: u32,
) -> quire_contract_codegen::CollectionObligationIdentity {
    let function_id = fixture
        .package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_id.digest.as_ref() == fixture.function_digest)
        .unwrap()
        .node_id
        .clone();
    let quantifier = fixture
        .package
        .graph()
        .nodes
        .iter()
        .find(|node| application_identity(node) == Some(operation))
        .unwrap()
        .node_id
        .clone();
    let item = ObligationItem::CollectionClaim {
        package: &fixture.package,
        function_id: &function_id,
        node_id: &quantifier,
    };
    let KaniObligationOutcome::Emitted {
        collection_harnesses,
        ..
    } = negotiate_kani_obligations(&KaniObligationRequest {
        ceilings: crate::common::proof_ceilings::proof_ceilings_with_wall_clock(
            Duration::from_secs(600),
        ),
        items: std::slice::from_ref(&item),
        subject_path,
        unwind,
    })
    .unwrap()
    else {
        panic!("the source-authored collection identity is supported");
    };
    collection_harnesses[0].identity.clone()
}

fn collection_harness(fixture: &Emitted, operation: &str) -> KaniCollectionObligationHarness {
    collection_harness_with_ceilings(
        fixture,
        operation,
        crate::common::proof_ceilings::proof_ceilings_with_wall_clock(Duration::from_secs(600)),
    )
}

fn collection_harness_with_ceilings(
    fixture: &Emitted,
    operation: &str,
    ceilings: ProofCeilings,
) -> KaniCollectionObligationHarness {
    let function_id = fixture
        .package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_id.digest.as_ref() == fixture.function_digest)
        .unwrap()
        .node_id
        .clone();
    let quantifier = fixture
        .package
        .graph()
        .nodes
        .iter()
        .find(|node| application_identity(node) == Some(operation))
        .unwrap()
        .node_id
        .clone();
    let item = ObligationItem::CollectionClaim {
        package: &fixture.package,
        function_id: &function_id,
        node_id: &quantifier,
    };
    let KaniObligationOutcome::Emitted {
        records,
        collection_harnesses,
        ..
    } = negotiate_kani_obligations(&KaniObligationRequest {
        ceilings,
        items: std::slice::from_ref(&item),
        subject_path: "crate::collection_subject",
        unwind: 8,
    })
    .unwrap()
    else {
        panic!("the source-authored collection claim has a supported disposition");
    };
    assert!(matches!(
        records[0].disposition,
        ObligationDisposition::Supported { .. }
    ));
    collection_harnesses.into_iter().next().unwrap()
}

/// The symbolic collection harness source, for the cover-last guard (FR-015-AC-58).
pub(crate) fn guard_sources() -> Vec<(&'static str, String)> {
    let harness = collection_harness(&emitted(), "quire.op.collection.forall");
    vec![("symbolic collection", harness.rust.contents)]
}

fn collection_item_outcome(
    fixture: &Emitted,
    operation: &str,
    unwind: u32,
) -> KaniObligationOutcome {
    let function_id = fixture
        .package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_id.digest.as_ref() == fixture.function_digest)
        .expect("selected function exists")
        .node_id
        .clone();
    let quantifier = fixture
        .package
        .graph()
        .nodes
        .iter()
        .find(|node| application_identity(node) == Some(operation))
        .expect("selected quantifier exists")
        .node_id
        .clone();
    let item = ObligationItem::CollectionClaim {
        package: &fixture.package,
        function_id: &function_id,
        node_id: &quantifier,
    };
    negotiate_kani_obligations(&KaniObligationRequest {
        ceilings: crate::common::proof_ceilings::proof_ceilings_with_wall_clock(
            Duration::from_secs(600),
        ),
        items: std::slice::from_ref(&item),
        subject_path: "crate::test_subject",
        unwind,
    })
    .expect("valid collection request")
}

/// QSL-admitted collection claims still emit a symbolic V2 harness after finite corpus retirement.
#[trace("FR-015-AC-110", "TC-023", "TC-025")]
#[test]
fn collection_claim_uses_the_v2_symbolic_harness_without_a_finite_corpus_request() {
    let fixture = emitted();
    let outcome = collection_item_outcome(&fixture, "quire.op.collection.forall", 8);
    let KaniObligationOutcome::Emitted {
        records,
        collection_harnesses,
        ..
    } = outcome
    else {
        panic!("the admitted collection claim must emit a V2 harness");
    };
    assert_eq!(records.len(), 1);
    assert!(matches!(
        records[0].disposition,
        ObligationDisposition::Supported { .. }
    ));
    assert_eq!(collection_harnesses.len(), 1);
    let harness = &collection_harnesses[0].rust.contents;
    assert!(
        harness.contains("kani::any"),
        "symbolic input remains: {harness}"
    );
    assert!(
        !harness.contains("Vec<i128>"),
        "finite corpus input returned: {harness}"
    );
}

fn write_collection_scratch(harness: &KaniCollectionObligationHarness) -> TemporaryDirectory {
    let scratch = TemporaryDirectory::new("quire-kani-symbolic-domains-collection");
    let directory = &scratch.path;
    fs::write(directory.join("src/lib.rs"), &harness.rust.contents)
        .expect("write the generated artifact byte for byte");
    write_manifest(
        directory,
        &format!(
            "[package]\nname = \"generated-kani-collection\"\nversion = \"0.0.0\"\nedition = \"2021\"\npublish = false\n\n[dependencies]\n{}\n\n[workspace]\n",
            runtime_dependency(&["exact"])
        ),
    );
    fs::write(
        directory.join("build.rs"),
        "fn main() { println!(\"cargo:rustc-check-cfg=cfg(kani)\"); }\n",
    )
    .expect("allow Kani's generated cfg");
    scratch
}

fn run_collection_kani(
    installation: &KaniInstallation,
    harness: &KaniCollectionObligationHarness,
    label: &str,
) -> quire_contract_codegen::KaniExecutionEvidence {
    let scratch = write_collection_scratch(harness);
    let crate_directory = &scratch.path;
    let cancel = quire_exact::Cancel::new();
    let evidence = execute_kani_obligation(&KaniExecutionRequest {
        cancel: &cancel,
        installation,
        harness: KaniExecutableHarness::from(harness),
        crate_directory,
        target_directory: &PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("kani-collection"),
        home_directory: None,
    })
    .unwrap_or_else(|refusal| panic!("{label}: {refusal}"));
    let evidence_directory =
        PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("kani-collection-evidence");
    fs::create_dir_all(&evidence_directory).unwrap();
    fs::write(
        evidence_directory.join(format!("{label}.json")),
        serde_json::to_string_pretty(&evidence).unwrap(),
    )
    .unwrap();
    fs::write(
        evidence_directory.join(format!("{label}.harness.rs")),
        &harness.rust.contents,
    )
    .unwrap();
    evidence
}

/// Parse the byte vectors in one Kani concrete playback block without interpreting the comments.
/// The collection harness declares its i64 length first, then the three i64 array slots, all in
/// declaration order, so these byte vectors directly witness that typed argument layout.
fn collection_playback_byte_vectors(counterexample: &str) -> Vec<Vec<u8>> {
    let body = counterexample
        .split_once("let concrete_vals: Vec<Vec<u8>> = vec![")
        .expect("Kani playback contains concrete_vals")
        .1
        .split_once("];")
        .expect("Kani playback closes concrete_vals")
        .0;
    body.lines()
        .map(str::trim)
        .filter(|line| line.starts_with("vec!["))
        .map(|line| {
            line.trim_start_matches("vec![")
                .trim_end_matches("],")
                .split(',')
                .map(|byte| byte.trim().parse::<u8>().expect("playback byte is u8"))
                .collect()
        })
        .collect()
}

fn i64_playback_bytes(value: i64) -> Vec<u8> {
    value.to_le_bytes().to_vec()
}

fn collection_playback_values(counterexample: &str) -> Vec<i64> {
    let slots = collection_playback_byte_vectors(counterexample);
    let decode =
        |bytes: &[u8]| i64::from_le_bytes(bytes.try_into().expect("i64 slot has eight bytes"));
    let length = usize::try_from(decode(&slots[0])).expect("the symbolic length is nonnegative");
    assert!((1..=3).contains(&length));
    assert_eq!(slots.len(), 4);
    slots[1..=length].iter().map(|slot| decode(slot)).collect()
}

/// Replay the same source function used by the collection harness against an
/// object environment whose `items` field is the supplied concrete collection.
fn replay_collection_function(fixture: &Emitted, values: &[i64]) -> ReplayResult {
    const OBJECT_KEY: &str = "collection-subject";
    let source_identity =
        SourceIdentity::new("agent-ix", "ir268-symbolic.native", "git", "fixture");
    let selection = QualifiedName::new(vec![Identifier::new("inspect").unwrap()]).unwrap();
    let site = call_site(
        source_identity,
        "ir268-symbolic.native",
        fixture.source.as_bytes(),
        [fixture.model_bytes.as_slice()],
        &DependencyInput::default(),
        &selection,
    )
    .expect("the exact source function remains callable by QSL");

    let model = EnvironmentModel {
        identity: "ir268.test/domain".to_owned(),
        version: "1.0.0".to_owned(),
        digest: fixture.model_digest,
    };
    let type_identity = DeclarationKey {
        package: model.identity.clone(),
        node: NODE_ID.to_owned(),
    };
    let packages = BTreeMap::from([(fixture.model_digest, fixture.model_bytes.clone())]);
    let model_selections = [DomainPackageRef {
        identity: model.identity.clone(),
        version: model.version.clone(),
        digest: fixture.model_digest,
    }];
    let reference = object_reference_witness(
        &model_selections,
        &packages,
        ModelNormalizationLimits::default(),
        &type_identity,
        OBJECT_KEY,
    )
    .unwrap_or_else(|error| panic!("the admitted object reference is constructible: {error:?}"));
    assert!(matches!(&reference, WitnessValue::Reference { .. }));

    let selected_object = || SelectedObject {
        population: POPULATION_ID.to_owned(),
        key: OBJECT_KEY.to_owned(),
    };
    let environment = write_object_environment(
        EnvironmentLabels {
            authority: "agent-ix".to_owned(),
            identity: "ix://ir268.test/collection/environment".to_owned(),
            revision_namespace: "git".to_owned(),
            revision: "fixture".to_owned(),
        },
        &model,
        &[EnvironmentPopulation {
            population: POPULATION_ID.to_owned(),
            complete: true,
            objects: vec![EnvironmentObject {
                key: OBJECT_KEY.to_owned(),
                type_identity,
                fields: vec![
                    (
                        "items".to_owned(),
                        SnapshotValue::sequence(
                            values
                                .iter()
                                .map(|value| SnapshotValue::integer(value.to_string()))
                                .collect(),
                        ),
                    ),
                    ("flags".to_owned(), SnapshotValue::sequence(Vec::new())),
                    (
                        "next".to_owned(),
                        SnapshotValue::reference(selected_object()),
                    ),
                    (
                        "maybe".to_owned(),
                        SnapshotValue::present(SnapshotValue::reference(selected_object())),
                    ),
                    ("many".to_owned(), SnapshotValue::sequence(Vec::new())),
                ],
            }],
        }],
    )
    .expect("QSL writes the environment through its authoritative builder");

    let source_digest = DigestRecord::mint(
        DigestDomain::SourceBytesV1,
        ByteDigest::of(fixture.source.as_bytes()).as_bytes(),
    );
    let model_digest = DigestRecord::mint(DigestDomain::Sha256Jcs, fixture.model_digest);
    let request = ReplayRequestWire {
        profile_selections: Vec::new(),
        package_id: (
            Some(DigestDomain::PackageSemanticV2.as_str().to_owned()),
            site.package_id.hex(),
        ),
        source_digests: vec![(
            "agent-ix".to_owned(),
            "ir268-symbolic.native".to_owned(),
            "git".to_owned(),
            "fixture".to_owned(),
            Some(DigestDomain::SourceBytesV1.as_str().to_owned()),
            source_digest.hex(),
        )],
        dependencies: Vec::new(),
        replay_target: RequestTargetWire::function(selection, site.site.declaration.clone()),
        source: ReplaySource::Input(vec![qsl_replay::CanonicalAssignment {
            parameter: site.site.parameters[0].1,
            value: reference,
        }]),
        obligation_identity: [0; 32],
        backend: "kani-backend-1".to_owned(),
        state_environment: StateEnvironment::new(Vec::new()),
        accounting_limits: replay_scalar_limits(),
        stage_limits: BTreeMap::new(),
        supplied_limits: None,
        object_environment: Some(environment.member().clone()),
        declared_domains: Vec::new(),
        byte_provision: vec![
            (
                Some(DigestDomain::SourceBytesV1.as_str().to_owned()),
                source_digest.hex(),
                fixture.source.as_bytes().to_vec(),
            ),
            (
                Some(DigestDomain::Sha256Jcs.as_str().to_owned()),
                model_digest.hex(),
                fixture.model_bytes.clone(),
            ),
            environment.provision_entry(),
        ],
    };
    replay(request, ReplayLimits::default()).expect("QSL admits and executes the native replay")
}

fn assert_native_collection_result(fixture: &Emitted, values: &[i64], expected: bool) {
    let replayed = replay_collection_function(fixture, values);
    assert_eq!(
        replayed.category(),
        if expected {
            Category::Success
        } else {
            Category::Violation
        }
    );
    let ReplayResult::Input(input) = replayed else {
        panic!("native collection replay uses the input arm");
    };
    assert_eq!(
        input.value(),
        Some(EvaluatedValue::Boolean(expected)),
        "native replay evaluates the source-authored collection predicate"
    );
}

#[test]
fn collection_playback_decodes_length_then_element_slots_from_real_byte_shape() {
    let playback = r#"/// Test generated for harness `kob_collection_example::prove_collection_example`
///
/// Check for `assertion`: "collection claim"
fn kani_concrete_playback() {
    let concrete_vals: Vec<Vec<u8>> = vec![
        // 1
        vec![1, 0, 0, 0, 0, 0, 0, 0],
        // 0
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        // 0
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        // 0
        vec![0, 0, 0, 0, 0, 0, 0, 0],
    ];
    kani::concrete_playback_run(concrete_vals, prove_collection_example);
}"#;
    assert_eq!(
        collection_playback_byte_vectors(playback),
        vec![
            i64_playback_bytes(1),
            i64_playback_bytes(0),
            i64_playback_bytes(0),
            i64_playback_bytes(0),
        ]
    );
}

fn try_emitted_from_document_and_body(document: Value, body: &str) -> Result<Emitted, String> {
    let limits = quire_canonical::Limits::new(1 << 20);
    let document_bytes = quire_canonical::to_vec(&document, limits)
        .expect("the authored model document has bounded JCS bytes");
    let model_digest_value = quire_canonical::sha256(&document, limits)
        .expect("the authored model document has a JCS digest");
    let model_digest = model_digest_value.to_string();
    let model_digest_bytes = *model_digest_value.as_bytes();
    let unit = source_unit_with_body(&model_digest, body);
    let selection = QualifiedName::new(vec![Identifier::new("inspect").expect("function name")])
        .expect("qualified function name");
    let site = call_site(
        SourceIdentity::new("agent-ix", "ir268-symbolic.native", "git", "fixture"),
        "ir268-symbolic.native",
        unit.as_bytes(),
        [document_bytes.as_slice()],
        &DependencyInput::default(),
        &selection,
    )
    .map_err(|error| format!("{error:?}"))?;

    let mut evidence = CheckedPackageEvidence::new();
    evidence.support_feature("quire.value.complete/v1");
    evidence
        .insert_domain_package_document(model_digest.clone(), document_bytes.clone())
        .expect("the exact selected model document is admitted as evidence");
    let package = match CheckedPackageV2::read(
        &site.package,
        CheckedPackageReadLimits::bounded(),
        &evidence,
    ) {
        CheckedPackageV2ReadResult::Admitted(package) => *package,
        other => {
            return Err(format!(
                "QSL's emitted symbolic fixture is not admitted: {other:?}"
            ))
        }
    };
    let object_fields: Vec<_> = package
        .graph()
        .nodes
        .iter()
        .filter_map(|node| {
            package
                .model_object_fields(&node.node_id)
                .ok()
                .map(|fields| (node.node_id.clone(), fields))
        })
        .collect();
    let [(object_type, fields)] = object_fields.as_slice() else {
        return Err(format!(
            "the emitted package has {} selected model objects",
            object_fields.len()
        ));
    };
    assert!(
        !package.package_id().digest.is_empty(),
        "the admitted package has a semantic identity"
    );
    let function_digest = site.site.function.to_string();
    Ok(Emitted {
        package,
        function_digest,
        object_type: object_type.clone(),
        fields: fields.clone(),
        model_bytes: document_bytes,
        model_digest: model_digest_bytes,
        source: unit,
    })
}

#[trace("FR-015-AC-96", "TC-025")]
#[test]
fn qsl_emits_and_ir_admits_the_authored_collection_and_reference_shapes() {
    let fixture = emitted();
    assert!(!fixture.package.package_id().digest.is_empty());
    let forall = fixture
        .package
        .graph()
        .nodes
        .iter()
        .find(|node| application_identity(node) == Some("quire.op.collection.forall"))
        .expect("QSL lowered the source forall to its catalogued operation");
    let exists = fixture
        .package
        .graph()
        .nodes
        .iter()
        .find(|node| application_identity(node) == Some("quire.op.collection.exists"))
        .expect("QSL lowered the source exists to its catalogued operation");
    let collection_digest = argument_target(forall, 0)
        .expect("forall collection operand identity")
        .digest
        .as_ref();
    let collection_node = fixture
        .package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_id.digest.as_ref() == collection_digest)
        .expect("the collection operand resolves in the admitted graph");
    let type_node = fixture
        .package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_id == collection_node.semantic_type)
        .expect("the collection operand type resolves in the admitted graph");
    let exists_collection_digest = argument_target(exists, 0)
        .expect("exists collection operand identity")
        .digest
        .as_ref();
    let exists_collection = fixture
        .package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_id.digest.as_ref() == exists_collection_digest)
        .expect("exists collection operand resolves in admitted graph");
    assert_eq!(forall.node_tag.as_ref(), "expression");
    assert_eq!(forall.semantic_form.as_ref(), "quantify");
    assert_eq!(exists.node_tag.as_ref(), "expression");
    assert_eq!(exists.semantic_form.as_ref(), "quantify");
    assert_eq!(exists_collection.node_tag.as_ref(), "expression");
    assert_eq!(exists_collection.semantic_form.as_ref(), "query");
    assert_eq!(exists_collection.body, collection_node.body);
    assert_eq!(
        exists_collection.semantic_type,
        collection_node.semantic_type
    );
    assert_eq!(collection_node.node_tag.as_ref(), "expression");
    assert_eq!(collection_node.semantic_form.as_ref(), "query");
    let CheckedTerm::Application {
        operation: projection,
        ..
    } = &collection_node.body
    else {
        panic!("forall operand is a model-field projection");
    };
    assert_eq!(projection.identity(), Some("quire.op.record.project"));
    let (declaration, name) = projection
        .projection_field()
        .expect("authenticated operation view")
        .expect("admitted field projection");
    assert_eq!(name, "items");
    assert_eq!(
        declaration.digest.as_ref(),
        fixture.object_type.digest.as_ref(),
        "projection declaration is the admitted model object containing items"
    );
    let receiver_digest = argument_target(collection_node, 0)
        .expect("projection receiver identity")
        .digest
        .as_ref();
    let receiver = fixture
        .package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_id.digest.as_ref() == receiver_digest)
        .expect("projection receiver resolves in admitted graph");
    let parameter_digest = argument_target(receiver, 0)
        .expect("deref receiver parameter identity")
        .digest
        .as_ref();
    let parameter = fixture
        .package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_id.digest.as_ref() == parameter_digest)
        .expect("deref receiver parameter resolves in admitted graph");
    assert_eq!(parameter.node_tag.as_ref(), "value");
    assert_eq!(parameter.semantic_form.as_ref(), "parameter");
    assert!(
        matches!(
            named_member(parameter, "name"),
            Some(CheckedTerm::Literal { value, .. }) if value.as_str() == Some("input")
        ),
        "the projection receiver is the source function's model-typed input"
    );
    assert_eq!(type_node.semantic_form.as_ref(), "collection_bounds");
    let sequence_type = fixture
        .package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_id == type_node.semantic_type)
        .expect("sequence type resolves in admitted graph");
    assert_eq!(sequence_type.semantic_form.as_ref(), "sequence");
    let bound = |name: &str| match named_member(type_node, name) {
        Some(CheckedTerm::Literal { value, .. }) => value.as_str()?.parse::<u64>().ok(),
        _ => None,
    };
    assert_eq!(bound("min"), Some(0));
    assert_eq!(bound("max"), Some(3));
    let function = fixture
        .package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_id.digest.as_ref() == fixture.function_digest)
        .expect("QSL FunctionSite identity names an admitted function node");
    let Some(CheckedTerm::Reference { target }) = named_member(function, "body") else {
        panic!("the admitted function body has a target identity");
    };
    let function_body = target.digest.as_ref();
    let body_node = fixture
        .package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_id.digest.as_ref() == function_body)
        .expect("the admitted function body target resolves");
    assert!(body_node.dependencies.contains(&forall.node_id));
    assert!(body_node.dependencies.contains(&exists.node_id));
    assert!(
        fixture.package.source_map().iter().any(|entry| {
            entry.node_id == forall.node_id
                && entry.role == quire_contract_model::CheckedOccurrenceRole::Expression
                && entry.ordinal == 0
                && !entry.regions.is_empty()
        }),
        "the forall claim has an admitted expression source path"
    );
    assert_eq!(
        fixture
            .fields
            .field("items")
            .and_then(|field| field.member_type()),
        Some(&CheckedMemberType::Collection {
            kind: quire_contract_model::CheckedCollectionKind::Sequence,
            element: Box::new(CheckedMemberType::IntRange { lower: 0, upper: 3 }),
            bounds: Some((0, 3)),
        })
    );
    assert_eq!(
        fixture
            .fields
            .field("flags")
            .and_then(|field| field.member_type()),
        Some(&CheckedMemberType::Collection {
            kind: quire_contract_model::CheckedCollectionKind::Sequence,
            element: Box::new(CheckedMemberType::Boolean),
            bounds: Some((0, 2)),
        })
    );
    assert!(
        matches!(fixture.fields.field("maybe").and_then(|field| field.member_type()), Some(CheckedMemberType::Option(inner)) if **inner == CheckedMemberType::Reference(fixture.object_type.clone())),
        "optional member type: {:?}",
        fixture
            .fields
            .field("maybe")
            .and_then(|field| field.member_type())
    );
    assert_eq!(
        fixture
            .fields
            .field("next")
            .and_then(|field| field.member_type()),
        Some(&CheckedMemberType::Reference(fixture.object_type.clone()))
    );
    assert!(
        matches!(fixture.fields.field("many").and_then(|field| field.member_type()), Some(CheckedMemberType::Collection { element, bounds: Some((0, 2)), .. }) if **element == CheckedMemberType::Reference(fixture.object_type.clone()))
    );
}

#[trace(
    "FR-015-AC-96",
    "FR-015-AC-99",
    "FR-015-AC-100",
    "TC-025",
    "TC-001",
    "NFR-002-AC-1",
    "NFR-002-AC-2"
)]
#[test]
fn negotiation_renders_the_selected_sequence_claims_deterministically() {
    let fixture = emitted();
    let function_id = fixture
        .package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_id.digest.as_ref() == fixture.function_digest)
        .expect("the selected function exists")
        .node_id
        .clone();
    for (operation, assertion) in [
        ("quire.op.collection.forall", "item >= 0"),
        ("quire.op.collection.exists", "item == 2"),
    ] {
        let quantifier = fixture
            .package
            .graph()
            .nodes
            .iter()
            .find(|node| application_identity(node) == Some(operation))
            .expect("the selected QSL quantifier exists")
            .node_id
            .clone();
        let item = ObligationItem::CollectionClaim {
            package: &fixture.package,
            function_id: &function_id,
            node_id: &quantifier,
        };
        let request = KaniObligationRequest {
            ceilings: crate::common::proof_ceilings::proof_ceilings_with_wall_clock(
                Duration::from_secs(600),
            ),
            items: std::slice::from_ref(&item),
            subject_path: "crate::unused",
            unwind: 8,
        };
        let first = negotiate_kani_obligations(&request).expect("valid bounded request");
        let second = negotiate_kani_obligations(&request).expect("deterministic replay");
        let changed_subject = negotiate_kani_obligations(&KaniObligationRequest {
            subject_path: "crate::other_subject",
            ..request
        })
        .expect("a changed subject has a separate identity");
        let (records, harnesses) = match first {
            KaniObligationOutcome::Emitted {
                records,
                collection_harnesses,
                ..
            } => (records, collection_harnesses),
            other => panic!("collection request was rejected: {other:#?}"),
        };
        assert!(
            matches!(
                records[0].disposition,
                ObligationDisposition::Supported { .. }
            ),
            "collection disposition: {:#?}",
            records[0]
        );
        let harness = &harnesses[0];
        crate::generated_headers::assert_header(
            &harness.rust.path,
            &harness.rust.contents,
            &[
                &format!("{}/{}", function_id.domain, function_id.digest),
                &format!("{}/{}", quantifier.domain, quantifier.digest),
            ],
        );
        for node in [
            &harness.identity.projection_id,
            &harness.identity.declaration_id,
            &harness.identity.element_type_id,
            &harness.identity.predicate_id,
        ] {
            crate::generated_headers::assert_header(
                &harness.rust.path,
                &harness.rust.contents,
                &[&format!("{}/{}", node.domain, node.digest)],
            );
        }
        assert_eq!(harness.identity.function_id, function_id);
        assert_eq!(harness.identity.quantifier_id, quantifier);
        assert_eq!(
            harness.identity.package_digest,
            fixture.package.package_id().digest.to_string()
        );
        assert_eq!(harness.identity.quantifier_id.digest.len(), 64);
        let module = harness.identity.module_symbol.as_str();
        let harness_name = harness.identity.harness_symbol.as_str();
        assert!(module.len() < 255, "module filename component: {module}");
        assert!(
            harness_name.len() < 255,
            "harness filename component: {harness_name}"
        );
        assert!(format!("{module}::{harness_name}").len() < 255);
        assert_eq!(harness.identity.subject_path, "crate::unused");
        assert_eq!(
            harness.identity.solver,
            quire_contract_codegen::KaniSolver::Cadical
        );
        assert_eq!(harness.identity.unwind, 8);
        assert_eq!(harness.identity.field_name, "items");
        assert_eq!(harness.identity.minimum_length, 0);
        assert_eq!(harness.identity.maximum_length, 3);
        assert_eq!(
            harness.identity.collection_kind,
            CollectionKindIdentity::Sequence
        );
        assert_eq!(
            harness.identity.element,
            CollectionElementIdentity::Integer {
                minimum: 0,
                maximum: 3
            }
        );
        if operation.ends_with("forall") {
            assert_eq!(
                harness.identity.predicate,
                CollectionPredicateIdentity::IntegerGreaterOrEqual { threshold: 0 }
            );
        } else {
            assert_eq!(
                harness.identity.predicate,
                CollectionPredicateIdentity::IntegerEqual { value: 2 }
            );
        }
        assert!(
            harness
                .rust
                .contents
                .contains("let selected_items_len: i64 = kani::any();"),
            "{}",
            harness.rust.contents
        );
        assert!(harness
            .rust
            .contents
            .contains("let selected_items: [i64; 3] = kani::any();"));
        assert!(harness
            .rust
            .contents
            .contains("kani::assume(selected_items_len >= 0 && selected_items_len <= 3);"));
        assert!(harness
            .rust
            .contents
            .contains("kani::assume(item >= 0 && item <= 3);"));
        assert!(harness.rust.contents.contains("index as usize"));
        assert!(
            !harness.rust.contents.contains("sort(") && !harness.rust.contents.contains("dedup(")
        );
        assert!(
            !harness
                .rust
                .contents
                .contains("selected_items[0] != selected_items[1]"),
            "a Sequence with unique=false must allow duplicate adjacent elements"
        );
        let exact_harness = format!(
            "{}::{}",
            harness.identity.module_symbol, harness.identity.harness_symbol
        );
        let expected_options: Vec<String> = [
            "-Z",
            "function-contracts",
            "-Z",
            "concrete-playback",
            "--harness",
            exact_harness.as_str(),
            "--exact",
            "--unwind",
            "8",
            "--solver",
            "cadical",
            "--output-format",
            "regular",
            "--concrete-playback",
            "print",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect();
        assert_eq!(harness.identity.options, expected_options);
        assert!(harness.rust.contents.contains(assertion));
        assert!(harness
            .rust
            .contents
            .contains("let selected_items_len: i64 = kani::any();"));
        assert!(harness
            .rust
            .contents
            .contains("let selected_items: [i64; 3] = kani::any();"));
        assert!(!harness.rust.contents.contains("Vec<"));
        assert!(!harness.rust.contents.contains("vec!["));
        assert!(harness
            .rust
            .contents
            .contains(if operation.ends_with("forall") {
                ".all("
            } else {
                ".any("
            }));
        assert!(
            harness.rust.contents.find("assert!(claim").unwrap()
                < harness.rust.contents.find("kani::cover!").unwrap()
        );
        assert_eq!(harness.rust.contents.matches("kani::cover!(").count(), 1);
        let lines: Vec<_> = harness.rust.contents.lines().map(str::trim).collect();
        assert!(lines[lines.len() - 3].starts_with("kani::cover!("));
        assert_eq!(&lines[lines.len() - 2..], &["}", "}"]);
        assert!(harness
            .rust
            .contents
            .contains("kani::cover!(selected_items_len == 0 || selected_items_len == 3"));
        let second_harnesses = match second {
            KaniObligationOutcome::Emitted {
                collection_harnesses,
                ..
            } => collection_harnesses,
            other => panic!("second collection request was rejected: {other:#?}"),
        };
        assert_eq!(harness.identity, second_harnesses[0].identity);
        assert_eq!(harness.rust.contents, second_harnesses[0].rust.contents);
        assert_eq!(harness.record.contents, second_harnesses[0].record.contents);
        let changed_subject_harness = match changed_subject {
            KaniObligationOutcome::Emitted {
                collection_harnesses,
                ..
            } => collection_harnesses,
            other => panic!("changed subject request was rejected: {other:#?}"),
        };
        assert_ne!(harness.identity, changed_subject_harness[0].identity);
        assert_ne!(
            harness.identity.module_symbol,
            changed_subject_harness[0].identity.module_symbol
        );
        assert_ne!(
            harness.identity.harness_symbol,
            changed_subject_harness[0].identity.harness_symbol
        );
    }
}

#[trace("FR-015-AC-96", "FR-015-AC-101", "TC-025")]
#[test]
fn collection_claims_nested_under_not_or_disjunction_are_refused() {
    for body in [
        "not(forall(item in deref(input).items: item >= 0))",
        "forall(item in deref(input).items: item >= 0) or exists(item in deref(input).items: item = 2)",
    ] {
        let fixture = emitted_from_document_and_body(model_document(), body);
        let outcome = collection_item_outcome(&fixture, "quire.op.collection.forall", 8);
        let KaniObligationOutcome::Emitted {
            records,
            collection_harnesses,
            ..
        } = outcome
        else {
            panic!("unsupported nested claim is an item refusal");
        };
        assert!(matches!(
            records[0].disposition,
            ObligationDisposition::Unsupported {
                reason: UnsupportedObligation::CollectionClaimNotRendered {
                    reason: quire_contract_codegen::CollectionUnsupportedReason::UnsupportedClaimShape,
                    ..
                }
            }
        ), "body was {body}");
        assert!(collection_harnesses.is_empty(), "body was {body}");
    }
}

#[trace("FR-015-AC-101", "TC-025")]
#[test]
fn collection_unwind_must_cover_maximum_length_and_loop_exit() {
    let fixture = emitted();
    let outcome = collection_item_outcome(&fixture, "quire.op.collection.forall", 3);
    let KaniObligationOutcome::Emitted {
        records,
        collection_harnesses,
        ..
    } = outcome
    else {
        panic!("insufficient unwind is a typed item refusal");
    };
    assert!(matches!(
        records[0].disposition,
        ObligationDisposition::Unsupported {
            reason: UnsupportedObligation::CollectionClaimNotRendered {
                reason: quire_contract_codegen::CollectionUnsupportedReason::UnwindBelowCollectionBound {
                    required: 4,
                    supplied: 3,
                },
                ..
            }
        }
    ));
    assert!(collection_harnesses.is_empty());
}

#[trace("FR-015-AC-96", "FR-015-AC-99", "TC-025")]
#[test]
fn out_of_range_integer_literal_is_rendered_as_a_falsifiable_claim() {
    let fixture = emitted_from_document_and_body(
        model_document(),
        "forall(item in deref(input).items: item >= 4)",
    );
    let outcome = collection_item_outcome(&fixture, "quire.op.collection.forall", 4);
    let KaniObligationOutcome::Emitted {
        records,
        collection_harnesses,
        ..
    } = outcome
    else {
        panic!("encodable out-of-range literal is supported");
    };
    assert!(matches!(
        records[0].disposition,
        ObligationDisposition::Supported { .. }
    ));
    assert!(collection_harnesses[0].rust.contents.contains("item >= 4"));
    assert!(collection_harnesses[0]
        .rust
        .contents
        .contains("item >= 0 && item <= 3"));
}

#[trace("FR-015-AC-96", "FR-015-AC-100", "TC-025")]
#[test]
fn boolean_sequence_uses_symbolic_slots_for_both_boolean_values() {
    let fixture = emitted_from_document_and_body(
        model_document(),
        "forall(flag in deref(input).flags: flag != false)",
    );
    let function_id = fixture
        .package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_id.digest.as_ref() == fixture.function_digest)
        .unwrap()
        .node_id
        .clone();
    let quantifier = fixture
        .package
        .graph()
        .nodes
        .iter()
        .find(|node| application_identity(node) == Some("quire.op.collection.forall"))
        .unwrap()
        .node_id
        .clone();
    let item = ObligationItem::CollectionClaim {
        package: &fixture.package,
        function_id: &function_id,
        node_id: &quantifier,
    };
    let KaniObligationOutcome::Emitted {
        records,
        collection_harnesses,
        ..
    } = negotiate_kani_obligations(&KaniObligationRequest {
        ceilings: crate::common::proof_ceilings::proof_ceilings_with_wall_clock(
            Duration::from_secs(600),
        ),
        items: std::slice::from_ref(&item),
        subject_path: "crate::boolean_subject",
        unwind: 8,
    })
    .unwrap()
    else {
        panic!("the admitted Boolean sequence has a finite symbolic domain");
    };
    assert!(matches!(
        records[0].disposition,
        ObligationDisposition::Supported { .. }
    ));
    let harness = &collection_harnesses[0];
    assert_eq!(harness.identity.field_name, "flags");
    assert_eq!(harness.identity.minimum_length, 0);
    assert_eq!(harness.identity.maximum_length, 2);
    assert_eq!(harness.identity.element, CollectionElementIdentity::Boolean);
    assert!(harness
        .rust
        .contents
        .contains("let selected_flags: [bool; 2] = kani::any();"));
    assert!(harness.rust.contents.contains("item != false"));
    assert!(harness.rust.contents.contains("index as usize"));
}

#[trace("FR-015-AC-96", "FR-015-AC-99", "FR-015-AC-100", "TC-025")]
#[test]
fn nonzero_sequence_bounds_are_inclusive_in_the_symbolic_domain() {
    let mut document = model_document();
    let node = document["types"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|ty| ty.get("identity").and_then(Value::as_str) == Some(NODE_ID))
        .unwrap();
    let items = node["fields"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|field| field.get("name").and_then(Value::as_str) == Some("items"))
        .unwrap();
    items["multiplicity"]["lower"] = json!(1);
    let small_int = document["types"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|ty| ty.get("identity").and_then(Value::as_str) == Some(SMALL_ID))
        .unwrap();
    small_int["constraints"][0]["operands"]["value"] = json!("1");

    let fixture =
        emitted_from_document_and_body(document, "forall(item in deref(input).items: item >= 1)");
    let harness = collection_harness(&fixture, "quire.op.collection.forall");
    assert_eq!(harness.identity.minimum_length, 1);
    assert_eq!(harness.identity.maximum_length, 3);
    assert_eq!(
        harness.identity.element,
        CollectionElementIdentity::Integer {
            minimum: 1,
            maximum: 3,
        }
    );
    assert!(harness
        .rust
        .contents
        .contains("kani::assume(selected_items_len >= 1 && selected_items_len <= 3);"));
    assert!(harness
        .rust
        .contents
        .contains("kani::assume(item >= 1 && item <= 3);"));
    assert!(harness
        .rust
        .contents
        .contains("kani::cover!(selected_items_len == 1 || selected_items_len == 3"));
}

#[trace("FR-015-AC-100", "TC-025")]
#[test]
fn collection_identity_tracks_domains_claim_subject_and_unwind() {
    let base_document = model_document();
    let base_body = "forall(item in deref(input).items: item >= 0)";
    let base = emitted_from_document_and_body(base_document.clone(), base_body);
    let identity = collection_identity(&base, "quire.op.collection.forall", "crate::subject", 8);

    let mut cardinality_document = base_document.clone();
    let field = cardinality_document["types"][1]["fields"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|field| field.get("name").and_then(Value::as_str) == Some("items"))
        .unwrap();
    field["multiplicity"]["upper"] = json!(4);
    let cardinality = emitted_from_document_and_body(cardinality_document, base_body);
    assert_ne!(
        identity,
        collection_identity(
            &cardinality,
            "quire.op.collection.forall",
            "crate::subject",
            8,
        )
    );

    let mut element_document = base_document.clone();
    let range = element_document["types"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|ty| ty.get("identity").and_then(Value::as_str) == Some(SMALL_ID))
        .unwrap();
    range["constraints"][1]["operands"]["value"] = json!("4");
    let element = emitted_from_document_and_body(element_document, base_body);
    assert_ne!(
        identity,
        collection_identity(&element, "quire.op.collection.forall", "crate::subject", 8,)
    );

    let claim = emitted_from_document_and_body(
        base_document.clone(),
        "forall(item in deref(input).items: item >= 1)",
    );
    assert_ne!(
        identity,
        collection_identity(&claim, "quire.op.collection.forall", "crate::subject", 8,)
    );
    assert_ne!(
        identity,
        collection_identity(
            &base,
            "quire.op.collection.forall",
            "crate::other_subject",
            8,
        )
    );
    assert_ne!(
        identity,
        collection_identity(&base, "quire.op.collection.forall", "crate::subject", 9,)
    );
}

#[trace("FR-015-AC-100", "TC-025")]
#[cfg(target_os = "linux")]
#[test]
fn collection_run_evidence_preserves_every_ordered_identity_option() {
    use std::os::unix::fs::PermissionsExt;

    let fixture = emitted_from_document_and_body(
        model_document(),
        "forall(item in deref(input).items: item >= 0)",
    );
    let harness = collection_harness_with_ceilings(
        &fixture,
        "quire.op.collection.forall",
        ProofCeilings {
            memory_bytes: std::num::NonZeroU64::new(128 * 1024 * 1024).unwrap(),
            wall_clock: Duration::from_secs(10),
        },
    );
    let scratch = write_collection_scratch(&harness);
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
    .expect("bounded launcher returns run evidence even without a proof report");
    let received: Vec<String> = fs::read_to_string(scratch.path.join("arguments.txt"))
        .expect("launcher received arguments")
        .lines()
        .map(str::to_owned)
        .collect();
    assert_eq!(
        evidence.arguments, received,
        "evidence must reflect the launched argv"
    );
    assert_eq!(received[0], "kani");
    let options_end = 1 + harness.identity.options.len();
    assert_eq!(
        &received[1..options_end],
        harness.identity.options.as_slice(),
        "the complete ordered identity options must reach the launcher"
    );
    assert_eq!(
        &received[options_end..options_end + 3],
        ["-Z", "unstable-options", "--export-json"]
    );
    assert_eq!(received.len(), options_end + 4);
    assert!(received[options_end + 3].starts_with(target_directory.to_str().unwrap()));
    assert!(harness
        .identity
        .options
        .windows(2)
        .any(|pair| pair == ["--unwind", "8"]));
    assert!(harness
        .identity
        .options
        .windows(2)
        .any(|pair| pair == ["--solver", "cadical"]));
    assert_eq!(evidence.unwind, harness.identity.unwind);
    assert_eq!(evidence.solver, harness.identity.solver);
    assert_eq!(evidence.exit_code, Some(1));
}

fn ordered_set_document() -> Value {
    let mut document = model_document();
    let node = document["types"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|ty| ty.get("identity").and_then(Value::as_str) == Some(NODE_ID))
        .unwrap();
    let items = node["fields"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|field| field.get("name").and_then(Value::as_str) == Some("items"))
        .unwrap();
    items["multiplicity"]["unique"] = json!(true);
    document
}

#[trace("FR-015-AC-96", "TC-025")]
#[test]
fn feasible_ordered_set_emits_symbolic_ordered_unique_slots() {
    let fixture = emitted_from_document_and_body(
        ordered_set_document(),
        "forall(item in deref(input).items: item >= 0)",
    );
    let harness = collection_harness(&fixture, "quire.op.collection.forall");
    assert_eq!(
        harness.identity.collection_kind,
        CollectionKindIdentity::OrderedSet
    );
    assert_eq!(
        (
            harness.identity.minimum_length,
            harness.identity.maximum_length
        ),
        (0, 3)
    );
    assert_eq!(
        harness.identity.element,
        CollectionElementIdentity::Integer {
            minimum: 0,
            maximum: 3,
        }
    );
    let source = &harness.rust.contents;
    assert!(source.contains("let selected_items_len: i64 = kani::any();"));
    assert!(source.contains("let selected_items: [i64; 3] = kani::any();"));
    assert!(source.contains("selected_items_len >= 0 && selected_items_len <= 3"));
    assert!(source.contains("item >= 0 && item <= 3"));
    assert!(source.contains("for prior in 0..index"));
    assert!(source.contains("kani::assume(item != selected_items[prior as usize]);"));
    assert!(source.contains("selected_items[index as usize]"));
    assert!(source.contains("(0..selected_items_len).all(|index|"));

    let sequence = emitted_from_document_and_body(
        model_document(),
        "forall(item in deref(input).items: item >= 0)",
    );
    let sequence_harness = collection_harness(&sequence, "quire.op.collection.forall");
    assert_eq!(
        sequence_harness.identity.collection_kind,
        CollectionKindIdentity::Sequence
    );
    assert_ne!(harness.identity, sequence_harness.identity);
    assert!(!sequence_harness
        .rust
        .contents
        .contains("for prior in 0..index"));
}

#[trace("FR-015-AC-96", "TC-025")]
#[test]
fn boolean_ordered_set_keeps_both_values_and_distinct_active_slots() {
    let mut document = model_document();
    let node = document["types"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|ty| ty.get("identity").and_then(Value::as_str) == Some(NODE_ID))
        .unwrap();
    let flags = node["fields"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|field| field.get("name").and_then(Value::as_str) == Some("flags"))
        .unwrap();
    flags["multiplicity"]["unique"] = json!(true);
    let fixture = emitted_from_document_and_body(
        document,
        "forall(flag in deref(input).flags: flag != false)",
    );
    let harness = collection_harness(&fixture, "quire.op.collection.forall");
    assert_eq!(
        harness.identity.collection_kind,
        CollectionKindIdentity::OrderedSet
    );
    assert_eq!(harness.identity.maximum_length, 2);
    assert_eq!(harness.identity.element, CollectionElementIdentity::Boolean);
    let source = &harness.rust.contents;
    assert!(source.contains("let selected_flags: [bool; 2] = kani::any();"));
    assert!(source.contains("kani::assume(item != selected_flags[prior as usize]);"));
    assert!(source.contains("let item = selected_flags[index as usize];"));
}

#[test]
fn ordered_set_over_capacity_keeps_the_existing_typed_refusal_without_a_harness() {
    let mut document = model_document();
    let node = document["types"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|ty| ty.get("identity").and_then(Value::as_str) == Some(NODE_ID))
        .unwrap();
    let items = node["fields"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|field| field.get("name").and_then(Value::as_str) == Some("items"))
        .unwrap();
    items["multiplicity"]["unique"] = json!(true);
    let element = document["types"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|ty| ty.get("identity").and_then(Value::as_str) == Some(SMALL_ID))
        .unwrap();
    element["constraints"][1]["operands"]["value"] = json!("1");
    let fixture =
        emitted_from_document_and_body(document, "forall(item in deref(input).items: item >= 0)");
    assert!(matches!(
        fixture.fields.field("items").unwrap().member_type(),
        Some(CheckedMemberType::Collection {
            kind: quire_contract_model::CheckedCollectionKind::OrderedSet,
            ..
        })
    ));
    let function_id = fixture
        .package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_id.digest.as_ref() == fixture.function_digest)
        .unwrap()
        .node_id
        .clone();
    let quantifier = fixture
        .package
        .graph()
        .nodes
        .iter()
        .find(|node| application_identity(node) == Some("quire.op.collection.forall"))
        .unwrap()
        .node_id
        .clone();
    let item = ObligationItem::CollectionClaim {
        package: &fixture.package,
        function_id: &function_id,
        node_id: &quantifier,
    };
    let KaniObligationOutcome::Emitted {
        records,
        collection_harnesses,
        ..
    } = negotiate_kani_obligations(&KaniObligationRequest {
        ceilings: crate::common::proof_ceilings::proof_ceilings_with_wall_clock(
            Duration::from_secs(600),
        ),
        items: std::slice::from_ref(&item),
        subject_path: "crate::ordered_set",
        unwind: 8,
    })
    .unwrap()
    else {
        panic!("the unsupported collection shape settles per item");
    };
    assert!(matches!(
        records[0].disposition,
        ObligationDisposition::Unsupported {
            reason: UnsupportedObligation::CollectionClaimNotRendered {
                reason: quire_contract_codegen::CollectionUnsupportedReason::UnsupportedFieldType,
                ..
            }
        }
    ));
    assert!(collection_harnesses.is_empty());
}

#[trace("FR-015-AC-97", "FR-015-AC-102", "TC-025")]
#[test]
fn unsupported_collection_predicate_is_refused_without_harness() {
    let fixture = emitted_from_document_and_body(
        model_document(),
        "forall(item in deref(input).items: item > 0)",
    );
    let function_id = fixture
        .package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_id.digest.as_ref() == fixture.function_digest)
        .unwrap()
        .node_id
        .clone();
    let quantifier = fixture
        .package
        .graph()
        .nodes
        .iter()
        .find(|node| application_identity(node) == Some("quire.op.collection.forall"))
        .unwrap()
        .node_id
        .clone();
    let item = ObligationItem::CollectionClaim {
        package: &fixture.package,
        function_id: &function_id,
        node_id: &quantifier,
    };
    let KaniObligationOutcome::Emitted {
        records,
        collection_harnesses,
        ..
    } = negotiate_kani_obligations(&KaniObligationRequest {
        ceilings: crate::common::proof_ceilings::proof_ceilings_with_wall_clock(
            Duration::from_secs(600),
        ),
        items: std::slice::from_ref(&item),
        subject_path: "crate::unsupported_predicate",
        unwind: 8,
    })
    .unwrap()
    else {
        panic!("an unsupported predicate is an item disposition");
    };
    assert!(matches!(
        records[0].disposition,
        ObligationDisposition::Unsupported {
            reason: UnsupportedObligation::CollectionClaimNotRendered {
                reason: quire_contract_codegen::CollectionUnsupportedReason::UnsupportedClaimShape,
                ..
            }
        }
    ));
    assert!(collection_harnesses.is_empty());
}

#[trace("FR-015-AC-101", "TC-025")]
#[test]
fn collection_domain_larger_than_the_request_memory_ceiling_is_refused_without_harness() {
    let fixture = emitted();
    let function_id = fixture
        .package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_id.digest.as_ref() == fixture.function_digest)
        .unwrap()
        .node_id
        .clone();
    let quantifier = fixture
        .package
        .graph()
        .nodes
        .iter()
        .find(|node| application_identity(node) == Some("quire.op.collection.forall"))
        .unwrap()
        .node_id
        .clone();
    let item = ObligationItem::CollectionClaim {
        package: &fixture.package,
        function_id: &function_id,
        node_id: &quantifier,
    };
    let ceilings =
        crate::common::proof_ceilings::proof_ceilings_with_wall_clock(Duration::from_secs(600));
    let request = KaniObligationRequest {
        ceilings: ProofCeilings {
            memory_bytes: std::num::NonZeroU64::new(31).unwrap(),
            ..ceilings
        },
        items: std::slice::from_ref(&item),
        subject_path: "crate::unused",
        unwind: 8,
    };
    let KaniObligationOutcome::Emitted {
        records,
        collection_harnesses,
        ..
    } = negotiate_kani_obligations(&request).unwrap()
    else {
        panic!("resource refusal is settled per item");
    };
    assert!(matches!(
        records[0].disposition,
        ObligationDisposition::Unsupported {
            reason: UnsupportedObligation::CollectionClaimNotRendered {
                reason: quire_contract_codegen::CollectionUnsupportedReason::SymbolicDomainExceedsMemoryCeiling {
                    minimum_required_bytes: 32,
                    ceiling_bytes: 31,
                },
                ..
            }
        }
    ));
    assert!(collection_harnesses.is_empty());
}

#[trace("FR-015-AC-101", "TC-025")]
#[test]
fn canonical_model_wire_rejects_cardinality_above_i64() {
    let mut document = model_document();
    let node = document["types"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|ty| ty.get("identity").and_then(Value::as_str) == Some(NODE_ID))
        .unwrap();
    let items = node["fields"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|field| field.get("name").and_then(Value::as_str) == Some("items"))
        .unwrap();
    items["multiplicity"]["upper"] = json!(i64::MAX as u64 + 1);
    let error = quire_canonical::to_vec(&document, quire_canonical::Limits::new(1 << 20))
        .expect_err("the canonical model wire admits only signed-i64 magnitudes");
    let error = format!("{error:?}");
    assert!(error.contains("IntegerMagnitudeAboveMaximum"), "{error}");
    assert!(error.contains("9223372036854775808"), "{error}");
}

#[trace("FR-015-AC-97", "FR-015-AC-102", "TC-025")]
#[test]
fn qsl_admitted_collection_without_an_upper_bound_requires_bound_without_harness() {
    let mut document = model_document();
    let node = document["types"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|ty| ty.get("identity").and_then(Value::as_str) == Some(NODE_ID))
        .unwrap();
    let items = node["fields"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|field| field.get("name").and_then(Value::as_str) == Some("items"))
        .unwrap();
    items["multiplicity"]["lower"] = json!(0);
    items["multiplicity"]
        .as_object_mut()
        .unwrap()
        .remove("upper");
    let fixture = try_emitted_from_document_and_body(
        document,
        "forall(item in deref(input).items: item >= 0)",
    )
    .expect("QSL admits an optional lower bound when the minimum is zero");
    assert!(matches!(
        fixture.fields.field("items").unwrap().member_type(),
        Some(CheckedMemberType::Collection { bounds: None, .. })
    ));
    let function_id = fixture
        .package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_id.digest.as_ref() == fixture.function_digest)
        .unwrap()
        .node_id
        .clone();
    let quantifier = fixture
        .package
        .graph()
        .nodes
        .iter()
        .find(|node| application_identity(node) == Some("quire.op.collection.forall"))
        .unwrap();
    let projected_type = fixture
        .package
        .graph()
        .nodes
        .iter()
        .find(|node| application_identity(node) == Some("quire.op.record.project"))
        .unwrap()
        .semantic_type
        .clone();
    let item = ObligationItem::CollectionClaim {
        package: &fixture.package,
        function_id: &function_id,
        node_id: &quantifier.node_id,
    };
    let KaniObligationOutcome::Emitted {
        records,
        collection_harnesses,
        ..
    } = negotiate_kani_obligations(&KaniObligationRequest {
        ceilings: crate::common::proof_ceilings::proof_ceilings_with_wall_clock(
            Duration::from_secs(600),
        ),
        items: std::slice::from_ref(&item),
        subject_path: "crate::unused",
        unwind: 8,
    })
    .expect("a missing maximum settles per item")
    else {
        panic!("requires-bound is an ordinary disposition");
    };
    assert!(matches!(
        records[0].disposition,
        ObligationDisposition::RequiresBound { ref unbounded_type }
            if unbounded_type == &projected_type
    ));
    assert!(collection_harnesses.is_empty());
}

#[trace("FR-015-AC-97", "TC-025")]
#[test]
fn unbounded_sequence_of_reference_elements_is_unsupported_not_requires_bound() {
    let mut document = model_document();
    let node = document["types"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|ty| ty.get("identity").and_then(Value::as_str) == Some(NODE_ID))
        .unwrap();
    let items = node["fields"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|field| field.get("name").and_then(Value::as_str) == Some("items"))
        .unwrap();
    items["typeRef"] = json!(NODE_ID);
    items["multiplicity"]
        .as_object_mut()
        .unwrap()
        .remove("upper");
    let fixture =
        emitted_from_document_and_body(document, "forall(item in deref(input).items: true)");
    assert!(matches!(
        fixture.fields.field("items").unwrap().member_type(),
        Some(CheckedMemberType::Collection {
            element,
            bounds: None,
            ..
        }) if matches!(element.as_ref(), CheckedMemberType::Reference(_))
    ));
    let outcome = collection_item_outcome(&fixture, "quire.op.collection.forall", 8);
    let KaniObligationOutcome::Emitted {
        records,
        collection_harnesses,
        ..
    } = outcome
    else {
        panic!("unsupported member element is an item refusal");
    };
    assert!(matches!(
        records[0].disposition,
        ObligationDisposition::Unsupported {
            reason: UnsupportedObligation::CollectionClaimNotRendered {
                reason: quire_contract_codegen::CollectionUnsupportedReason::UnsupportedFieldType,
                ..
            }
        }
    ));
    assert!(collection_harnesses.is_empty());
}

#[trace("FR-015-AC-97", "TC-025")]
#[test]
fn qsl_rejects_a_missing_upper_bound_with_a_positive_minimum() {
    let mut document = model_document();
    let node = document["types"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|ty| ty.get("identity").and_then(Value::as_str) == Some(NODE_ID))
        .unwrap();
    let items = node["fields"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|field| field.get("name").and_then(Value::as_str) == Some("items"))
        .unwrap();
    items["multiplicity"]["lower"] = json!(1);
    items["multiplicity"]
        .as_object_mut()
        .unwrap()
        .remove("upper");
    let error = match try_emitted_from_document_and_body(
        document,
        "forall(item in deref(input).items: item >= 0)",
    ) {
        Ok(_) => panic!("a positive minimum requires an authored finite upper bound"),
        Err(error) => error,
    };
    assert!(error.contains("UnknownRequiredFeature"), "{error}");
    assert!(error.contains("Node/items"), "{error}");
}

#[trace("FR-015-AC-97", "FR-015-AC-102", "TC-025")]
#[test]
fn finite_sequence_of_unbounded_integer_elements_requires_bound_when_qsl_admits_it() {
    let mut document = model_document();
    let node = document["types"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|ty| ty.get("identity").and_then(Value::as_str) == Some(NODE_ID))
        .unwrap();
    let items = node["fields"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|field| field.get("name").and_then(Value::as_str) == Some("items"))
        .unwrap();
    items["typeRef"] = json!("ix://quire/native/Integer");
    let fixture = try_emitted_from_document_and_body(
        document,
        "forall(item in deref(input).items: item >= 0) and exists(item in deref(input).items: item = 2)",
    )
    .unwrap_or_else(|error| panic!("QSL/IR unbounded element source route: {error}"));
    assert!(matches!(
        fixture.fields.field("items").unwrap().member_type(),
        Some(CheckedMemberType::Collection {
            element,
            bounds: Some((0, 3)),
            ..
        }) if matches!(element.as_ref(), CheckedMemberType::Integer)
    ));
    let function_id = fixture
        .package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_id.digest.as_ref() == fixture.function_digest)
        .unwrap()
        .node_id
        .clone();
    let quantifier = fixture
        .package
        .graph()
        .nodes
        .iter()
        .find(|node| application_identity(node) == Some("quire.op.collection.forall"))
        .unwrap();
    let projection = fixture
        .package
        .graph()
        .nodes
        .iter()
        .find(|node| application_identity(node) == Some("quire.op.record.project"))
        .unwrap();
    let collection_type = fixture
        .package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_id == projection.semantic_type)
        .unwrap();
    let sequence_type = fixture
        .package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_id == collection_type.semantic_type)
        .unwrap();
    let CheckedTerm::Aggregate { members } = &sequence_type.body else {
        panic!("sequence type has a typed element member");
    };
    let Some(CheckedTerm::Reference {
        target: element_type,
    }) = members.first()
    else {
        panic!("sequence type element references its type");
    };
    let element_type = element_type.clone();
    let item = ObligationItem::CollectionClaim {
        package: &fixture.package,
        function_id: &function_id,
        node_id: &quantifier.node_id,
    };
    let KaniObligationOutcome::Emitted {
        records,
        collection_harnesses,
        ..
    } = negotiate_kani_obligations(&KaniObligationRequest {
        ceilings: crate::common::proof_ceilings::proof_ceilings_with_wall_clock(
            Duration::from_secs(600),
        ),
        items: std::slice::from_ref(&item),
        subject_path: "crate::unbounded_element",
        unwind: 8,
    })
    .unwrap()
    else {
        panic!("the element RequiresBound settles per item");
    };
    assert!(matches!(
        records[0].disposition,
        ObligationDisposition::RequiresBound { ref unbounded_type }
            if unbounded_type == &element_type
    ));
    assert!(collection_harnesses.is_empty());
}

#[trace("FR-015-AC-97", "TC-025")]
#[test]
fn bounded_sequences_of_string_uuid_and_timestamp_require_element_bounds() {
    for (type_ref, expected, semantic_form) in [
        ("ix://quire/native/String", "string", "string"),
        ("ix://quire/native/UUID", "uuid", "uuid"),
        ("ix://quire/native/Timestamp", "timestamp", "timestamp"),
    ] {
        let mut document = model_document();
        let node = document["types"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|ty| ty.get("identity").and_then(Value::as_str) == Some(NODE_ID))
            .unwrap();
        let items = node["fields"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|field| field.get("name").and_then(Value::as_str) == Some("items"))
            .unwrap();
        items["typeRef"] = json!(type_ref);
        let fixture = try_emitted_from_document_and_body(
            document,
            "forall(item in deref(input).items: true)",
        )
        .unwrap_or_else(|error| panic!("QSL admits Sequence<{expected}>: {error}"));
        let CheckedMemberType::Collection { element, .. } = fixture
            .fields
            .field("items")
            .unwrap()
            .member_type()
            .expect("admitted collection member")
        else {
            panic!("model field remains a collection");
        };
        assert!(matches!(
            (expected, element.as_ref()),
            ("string", CheckedMemberType::String)
                | ("uuid", CheckedMemberType::Uuid)
                | ("timestamp", CheckedMemberType::Timestamp)
        ));
        let outcome = collection_item_outcome(&fixture, "quire.op.collection.forall", 4);
        let KaniObligationOutcome::Emitted {
            records,
            collection_harnesses,
            ..
        } = outcome
        else {
            panic!("missing element bound is an item disposition");
        };
        let element_type = fixture
            .package
            .graph()
            .nodes
            .iter()
            .find(|node| {
                node.node_tag.as_ref() == "scalar_type"
                    && node.semantic_form.as_ref() == semantic_form
            })
            .expect("exact admitted element type node")
            .node_id
            .clone();
        assert!(matches!(
            records[0].disposition,
            ObligationDisposition::RequiresBound { ref unbounded_type }
                if unbounded_type == &element_type
        ));
        assert!(collection_harnesses.is_empty());
    }
}

#[trace("FR-015-AC-99", "TC-025")]
#[test]
fn native_collection_replay_evaluates_the_authored_healthy_and_mutated_functions() {
    let healthy = emitted_from_document_and_body(
        model_document(),
        "forall(item in deref(input).items: item >= 0)",
    );
    let mutated = emitted_from_document_and_body(
        model_document(),
        "forall(item in deref(input).items: item >= 1)",
    );
    assert_native_collection_result(&healthy, &[1, 2], true);
    assert_native_collection_result(&mutated, &[0], false);
}

#[trace("FR-015-AC-99", "TC-025")]
#[test]
#[ignore = "kani lane: run serially through make kani-gate"]
fn tc_025_real_kani_collection_claim_verifies_and_mutated_claim_falsifies() {
    let installation = KaniInstallation::discover().expect("cargo-kani is installed");
    let healthy = emitted_from_document_and_body(
        model_document(),
        "forall(item in deref(input).items: item >= 0)",
    );
    let healthy_harness = collection_harness(&healthy, "quire.op.collection.forall");
    let healthy_evidence = run_collection_kani(&installation, &healthy_harness, "healthy");
    assert_eq!(healthy_evidence.outcome, KaniRunOutcome::Verified);
    assert_eq!(healthy_evidence.exit_code, Some(0));
    assert_eq!(healthy_evidence.unwind, 8);
    assert_eq!(healthy_evidence.solver, healthy_harness.identity.solver);
    assert_native_collection_result(&healthy, &[1, 2], true);

    // This is a source-authored claim mutation, recompiled and admitted by QSL, then passed
    // through the same generator. Its counterexample uses the selected field's in-domain 0.
    let mutated = emitted_from_document_and_body(
        model_document(),
        "forall(item in deref(input).items: item >= 1)",
    );
    let mutated_harness = collection_harness(&mutated, "quire.op.collection.forall");
    assert_ne!(healthy_harness.identity, mutated_harness.identity);
    let mutated_evidence = run_collection_kani(&installation, &mutated_harness, "mutated");
    let KaniRunOutcome::Falsified { counterexample } = &mutated_evidence.outcome else {
        panic!("the seeded lower-threshold mutation is falsified: {mutated_evidence:?}");
    };
    assert!(counterexample.contains("kani::concrete_playback_run"));
    assert!(counterexample.contains(mutated_harness.identity.harness_symbol.as_str()));
    let values = collection_playback_values(counterexample);
    assert!(values.iter().all(|value| (0..=3).contains(value)));
    assert!(values.iter().any(|value| *value < 1));
    assert_native_collection_result(&mutated, &values, false);
    assert_ne!(mutated_evidence.exit_code, Some(0));
}

#[trace("FR-015-AC-96", "TC-025")]
#[test]
#[ignore = "kani lane: run serially through make kani-gate"]
fn tc_025_real_kani_ordered_set_claim_and_mutant_keep_distinct_active_values() {
    let installation = KaniInstallation::discover().expect("cargo-kani is installed");
    let mut document = ordered_set_document();
    let node = document["types"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|ty| ty.get("identity").and_then(Value::as_str) == Some(NODE_ID))
        .unwrap();
    let items = node["fields"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|field| field.get("name").and_then(Value::as_str) == Some("items"))
        .unwrap();
    items["multiplicity"]["lower"] = json!(2);
    let healthy = emitted_from_document_and_body(
        document.clone(),
        "forall(item in deref(input).items: item >= 0)",
    );
    let healthy_harness = collection_harness(&healthy, "quire.op.collection.forall");
    assert_eq!(
        run_collection_kani(&installation, &healthy_harness, "ordered-set-healthy").outcome,
        KaniRunOutcome::Verified
    );
    let mutant =
        emitted_from_document_and_body(document, "forall(item in deref(input).items: item >= 1)");
    let mutant_harness = collection_harness(&mutant, "quire.op.collection.forall");
    let evidence = run_collection_kani(&installation, &mutant_harness, "ordered-set-mutant");
    let KaniRunOutcome::Falsified { counterexample } = evidence.outcome else {
        panic!("the ordered-set mutant must falsify: {evidence:?}");
    };
    let values = collection_playback_values(&counterexample);
    assert!((2..=3).contains(&values.len()));
    assert!(values.contains(&0));
    for (index, value) in values.iter().enumerate() {
        assert!(
            !values[..index].contains(value),
            "the backend admitted duplicate active values"
        );
    }
    assert_native_collection_result(&mutant, &values, false);
}

#[trace("FR-015-AC-97", "FR-015-AC-102", "TC-025")]
#[test]
fn collection_negotiation_accounts_for_each_supported_or_refused_item() {
    let fixture = emitted();
    let function_id = fixture
        .package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_id.digest.as_ref() == fixture.function_digest)
        .expect("the selected function exists")
        .node_id
        .clone();
    let quantifier = fixture
        .package
        .graph()
        .nodes
        .iter()
        .find(|node| application_identity(node) == Some("quire.op.collection.forall"))
        .expect("the source forall exists")
        .node_id
        .clone();
    let bad_function = CheckedNodeId {
        domain: function_id.domain.clone(),
        digest: "unadmitted-function".into(),
    };
    let items = [
        ObligationItem::CollectionClaim {
            package: &fixture.package,
            function_id: &function_id,
            node_id: &quantifier,
        },
        ObligationItem::CollectionClaim {
            package: &fixture.package,
            function_id: &bad_function,
            node_id: &quantifier,
        },
    ];
    let outcome = negotiate_kani_obligations(&KaniObligationRequest {
        ceilings: crate::common::proof_ceilings::proof_ceilings_with_wall_clock(
            Duration::from_secs(600),
        ),
        items: &items,
        subject_path: "crate::unused",
        unwind: 8,
    })
    .expect("each item receives an independent disposition");
    let KaniObligationOutcome::Emitted {
        records,
        collection_harnesses,
        ..
    } = outcome
    else {
        panic!("typed refusal must not reject a neighboring valid item");
    };
    assert_eq!(records.len(), 2);
    assert_eq!(collection_harnesses.len(), 1);
    assert!(matches!(
        records[0].disposition,
        ObligationDisposition::Supported { .. }
    ));
    assert!(matches!(
        records[1].disposition,
        ObligationDisposition::Unsupported {
            reason: UnsupportedObligation::CollectionClaimNotRendered {
                reason: quire_contract_codegen::CollectionUnsupportedReason::UnsupportedClaimShape,
                ..
            }
        }
    ));
}

/// The replay accounting ceilings: generous, so a bounded collection witness is never refused for budget.
fn replay_scalar_limits() -> ScalarLimits {
    ScalarLimits {
        integer_bits: 1_000,
        decimal_digits: 1_000,
        scale_expansion: 1_000,
        text_input_bytes: 1_000,
        text_scalars: 1_000,
        normalized_scalars: 1_000,
        unit_edges: 1_000,
        value_occurrences: 1_000,
        work_units: 1_000,
        result_units: 1_000,
    }
}
