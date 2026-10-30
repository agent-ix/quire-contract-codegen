// SPDX-License-Identifier: AGPL-3.0-or-later

//! Source-authored FR-315 query association for FR-045-AC-4.

use std::{
    collections::BTreeMap,
    ffi::OsString,
    num::{NonZeroU64, NonZeroUsize},
    time::Duration,
};

use ix_trace_rs::trace;
use qsl_replay::{
    acquire_checked_closure, encode_refinement_query, refinement_step_sources, DependencyInput,
    DocumentRef, NodeKey, RefinementQueryKind, RefinementQueryRequest, RefinementQuerySubject,
    RefinementUniverse, ReplayLimits, ScalarLimits, SourceIdentity, StageLimits,
};
use quire_exact::Cancel;
use serde_json::{json, Value};

use super::{
    refinement_handoff, RefinementExpected, RefinementHandoffError, RefinementRole, RefinementRun,
};
use crate::{
    kani::run::process::ProcessLimits,
    routed::smt::{run_solver_query, QuerySelection},
};

const ABSTRACT: &str = "ir1120.test/abstract";
const CONCRETE: &str = "ir1120.test/concrete";
const ZERO_DIGEST: &str = "sha256:0000000000000000000000000000000000000000000000000000000000000000";

fn key(package: &str, name: &str) -> String {
    format!("ix://{package}/{name}")
}

fn origin(identity: &str) -> Value {
    json!({"generated": {"generatorIdentity": identity, "generatorVersion": "1.0.0",
        "inputIdentities": [identity]}})
}

fn model(package: &str, operations: &[&str]) -> (Vec<u8>, [u8; 32], String) {
    let object = key(package, "Object");
    let population = key(package, "objects");
    let constructs: Vec<_> = [
        ("object_type", "quire.meaning.model.object-type/v1"),
        ("population", "quire.meaning.model.population/v1"),
    ]
    .into_iter()
    .map(|(name, meaning)| {
        json!({
            "kind": {"module": package, "name": name}, "moduleVersion": "1.0.0",
            "manifestDigest": ZERO_DIGEST,
            "construct": {"identity": "none", "shape": "record", "members": {}, "meaning": meaning}
        })
    })
    .collect();
    let operations: Vec<_> = operations
        .iter()
        .map(|name| {
            let identity = key(package, &format!("Object/{name}"));
            json!({
                "identity": identity, "name": name, "params": [], "pre": [], "post": [],
                "origin": {"source": {"sourceIdentity": key(package, "spec"), "path": "spec.qspec",
                    "startLine": 1, "startColumn": 1}},
                "frame": {"modifies": [], "creates": [], "deletes": []}
            })
        })
        .collect();
    let document = json!({
        "contractVersion": "2.0.0",
        "canonicalization": {"algorithm": "rfc8785-v1", "digest": "sha256-jcs"},
        "source": {"identity": key(package, "spec"), "version": "1.0.0",
            "dialect": "spec-bundle", "digest": ZERO_DIGEST},
        "package": {"identity": package, "version": "1.0.0", "manifestDigest": ZERO_DIGEST,
            "mappingVersions": [], "profileVersions": [], "lockDigest": ZERO_DIGEST},
        "occurrences": [], "extensions": [], "constructs": constructs,
        "types": [{"identity": object, "displayName": "Object",
            "kind": {"module": package, "name": "object_type"}, "roles": [],
            "origin": origin(&object), "constraints": [], "extensions": [],
            "unknownPolicy": "reject", "supertypes": [],
            "fields": [{"identity": key(package, "Object/flag"), "name": "flag",
                "typeRef": "ix://quire/native/Boolean", "presence": "required",
                "nullable": false, "defaultKind": "none",
                "multiplicity": {"lower": 1, "upper": 1, "ordered": false, "unique": true},
                "origin": origin(&key(package, "Object/flag"))}],
            "operations": operations, "relationships": []}],
        "populations": [{"identity": population, "displayName": "objects",
            "kind": {"module": package, "name": "population"},
            "members": [object], "extent": "closed", "origin": origin(&population)}]
    });
    let limits = quire_canonical::Limits::new(1 << 20);
    let bytes = quire_canonical::to_vec(&document, limits).expect("model bytes");
    let digest = quire_canonical::sha256(&document, limits).expect("model digest");
    (bytes, *digest.as_bytes(), digest.to_string())
}

fn initial(
    package: &str,
    model_digest: [u8; 32],
    snapshots: &mut BTreeMap<[u8; 32], Vec<u8>>,
) -> DocumentRef {
    let name = format!("{package}/start");
    let snapshot = json!({
        "format": "quire.state.snapshot/v1",
        "identity": {"authority": "test", "identity": name,
            "revision_namespace": "fixture", "revision": "fixture:1"},
        "observation": "current",
        "model": {"identity": package, "version": "1.0.0",
            "digest": format!("sha256-jcs:{}", hex(&model_digest))},
        "anchor": {"kind": "initialization", "name": "start"},
        "populations": [{"population": key(package, "objects"), "complete": true,
            "objects": [{"key": "only", "type": key(package, "Object"),
                "fields": {"flag": {"boolean": false}}}]}]
    });
    let limits = quire_canonical::Limits::new(1 << 20);
    let bytes = quire_canonical::to_vec(&snapshot, limits).expect("snapshot bytes");
    let digest = *quire_canonical::sha256(&snapshot, limits)
        .expect("snapshot digest")
        .as_bytes();
    snapshots.insert(digest, bytes);
    DocumentRef {
        authority: "test".to_owned(),
        identity: name,
        revision_namespace: "fixture".to_owned(),
        revision: "fixture:1".to_owned(),
        digest,
    }
}

fn hex(bytes: &[u8; 32]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn authored_rows(reverse: bool) -> String {
    let mut names: Vec<_> = (0..7).map(|index| format!("op{index}")).collect();
    if reverse {
        names.reverse();
    }
    names
        .into_iter()
        .map(|name| format!("step Concrete::Object::{name} -> stutter;\n"))
        .collect()
}

fn authored_unit(abstract_digest: &str, concrete_digest: &str, reverse: bool) -> String {
    format!(
        "language \"ix:native\" edition \"1-draft\";\n\
         profile inf = \"quire.temporal.infinite-trace/v1\";\n\
         model Abstract = \"{ABSTRACT}\" version \"1.0.0\" digest \"sha256-jcs:{abstract_digest}\";\n\
         model Concrete = \"{CONCRETE}\" version \"1.0.0\" digest \"sha256-jcs:{concrete_digest}\";\n\
         refinement SevenRows using inf abstract Abstract concrete Concrete {{\n\
         population Abstract::objects from Concrete::objects;\n\
         object Abstract::Object from Concrete::Object {{ flag = self.flag; }}\n\
         {}\
         }}\n",
        authored_rows(reverse)
    )
}

type Expected = RefinementExpected<NodeKey, [String; 2]>;
type Run = RefinementRun<NodeKey, [String; 2]>;

fn source_records(reverse: bool) -> (Vec<u64>, Vec<Expected>, Vec<Run>) {
    let (abstract_bytes, abstract_digest, abstract_hex) = model(ABSTRACT, &[]);
    let names: Vec<_> = (0..7).map(|index| format!("op{index}")).collect();
    let names: Vec<_> = names.iter().map(String::as_str).collect();
    let (concrete_bytes, concrete_digest, concrete_hex) = model(CONCRETE, &names);
    let mut packages = BTreeMap::new();
    packages.insert(abstract_digest, abstract_bytes.clone());
    packages.insert(concrete_digest, concrete_bytes.clone());
    let mut snapshots = BTreeMap::new();
    let abstract_initial = initial(ABSTRACT, abstract_digest, &mut snapshots);
    let concrete_initial = initial(CONCRETE, concrete_digest, &mut snapshots);
    let source = authored_unit(&abstract_hex, &concrete_hex, reverse);
    let compiled = acquire_checked_closure(
        SourceIdentity::new("agent-ix", "ir1120-refinement", "git", "fixture"),
        "ir1120-refinement.native",
        source.as_bytes(),
        packages.values().map(Vec::as_slice),
        &DependencyInput::default(),
        StageLimits::default(),
        ReplayLimits::default(),
    )
    .expect("authored seven-row refinement admits");
    let handle = compiled.handle();
    let refinement = handle
        .refinement_node_key("SevenRows")
        .expect("declared refinement key");
    let subject = |package: &str, initial| {
        RefinementQuerySubject::new(
            &packages,
            &snapshots,
            vec![initial],
            vec![RefinementUniverse::for_population(
                package,
                "objects",
                vec!["only".to_owned()],
            )],
            1_000_000,
            ScalarLimits {
                integer_bits: 1 << 20,
                decimal_digits: 1 << 20,
                scale_expansion: 1 << 20,
                text_input_bytes: 1 << 20,
                text_scalars: 1 << 20,
                normalized_scalars: 1 << 20,
                unit_edges: 1 << 20,
                value_occurrences: 1 << 20,
                work_units: 1 << 30,
                result_units: 1 << 30,
            },
        )
    };
    let request = RefinementQueryRequest::new(
        handle,
        refinement,
        subject(CONCRETE, concrete_initial),
        subject(ABSTRACT, abstract_initial),
    );
    let sources = refinement_step_sources(&request).expect("canonical source row mapping");
    assert_eq!(sources.len(), 7);
    let mut sorted = sources.clone();
    sorted.sort_unstable();
    assert_eq!(sorted, (0..7).collect::<Vec<_>>());
    let mut expected = Vec::new();
    let mut records = Vec::new();
    for role in
        std::iter::once(RefinementRole::Base).chain((0..7).map(|row| RefinementRole::Step { row }))
    {
        let kind = match role {
            RefinementRole::Base => RefinementQueryKind::Initial,
            RefinementRole::Step { row } => RefinementQueryKind::Step {
                row: u64::try_from(row).expect("seven rows fit u64"),
            },
        };
        let query = encode_refinement_query(&request, kind).expect("QSL canonical query");
        let execution = run_solver_query(
            QuerySelection {
                product: (),
                item: refinement,
                instance: (),
                depth: NonZeroU64::MIN,
                kind: role,
                query_bytes: query.clone(),
            },
            "/bin/sh".into(),
            vec![
                OsString::from("-c"),
                OsString::from("cat >&2; printf 'unsat\\n((proof))'"),
                OsString::from("--dump-proofs"),
                OsString::from("--proof-format-mode=alethe"),
            ],
            ProcessLimits {
                input_bytes: NonZeroUsize::new(1 << 20).expect("limit"),
                wall_clock: Duration::from_secs(5),
                memory_bytes: NonZeroU64::new(128 << 20).expect("limit"),
                output_bytes: NonZeroUsize::new(1 << 20).expect("limit"),
            },
            Cancel::new(),
        )
        .expect("bounded solver invocation");
        assert_eq!(
            execution.process().stderr.as_deref(),
            Some(query.as_slice())
        );
        let attempt = execution.query().association.attempt();
        let subjects = [CONCRETE.to_owned(), ABSTRACT.to_owned()];
        expected.push(Expected {
            item: refinement,
            subjects: subjects.clone(),
            role,
            attempt,
            query_bytes: query,
        });
        records.push(Run::from_execution(subjects, execution));
    }
    (sources, expected, records)
}

/// The source row's written position must never replace its canonical Step ordinal.
#[trace("TC-059", "FR-045-AC-4")]
#[test]
fn source_row_permutation_keeps_canonical_refinement_query_bindings() {
    let (first_sources, first_expected, first_records) = source_records(false);
    let (reordered_sources, reordered_expected, mut reordered_records) = source_records(true);
    assert_ne!(first_sources, reordered_sources);
    assert_eq!(first_expected.len(), 8);
    for (before, after) in first_expected.iter().zip(&reordered_expected) {
        assert_eq!(before.item, after.item);
        assert_eq!(before.subjects, after.subjects);
        assert_eq!(before.role, after.role);
        assert_eq!(before.query_bytes, after.query_bytes);
        assert_ne!(before.attempt, after.attempt);
    }
    assert!(refinement_handoff(7, &first_expected, &first_records).is_ok());
    reordered_records.reverse();
    assert!(refinement_handoff(7, &reordered_expected, &reordered_records).is_ok());
    let row = first_sources
        .iter()
        .position(|&source| source == 0)
        .expect("op0 is the first authored step");
    assert_ne!(
        u64::try_from(row).expect("seven rows fit u64"),
        reordered_sources[row]
    );
    assert_eq!(
        reordered_sources[row], 6,
        "op0 moves to the last source row"
    );
    let source_position = usize::try_from(reordered_sources[row]).expect("source position");
    let index = reordered_records
        .iter()
        .position(|record| record.role == RefinementRole::Step { row })
        .expect("selected canonical row");
    reordered_records[index].role = RefinementRole::Step {
        row: source_position,
    };
    assert_eq!(
        refinement_handoff(7, &reordered_expected, &reordered_records),
        Err(RefinementHandoffError::Shape)
    );
    reordered_records[index].role = RefinementRole::Step { row };
    reordered_records[index]
        .query_bytes
        .as_mut()
        .expect("query present")[0] ^= 1;
    assert_eq!(
        refinement_handoff(7, &reordered_expected, &reordered_records),
        Err(RefinementHandoffError::Association)
    );
    reordered_records[index]
        .query_bytes
        .as_mut()
        .expect("query present")[0] ^= 1;
    let base = reordered_records
        .iter()
        .position(|record| record.role == RefinementRole::Base)
        .expect("base record");
    reordered_records[base].role = RefinementRole::Step { row };
    reordered_records[index].role = RefinementRole::Base;
    assert_eq!(
        refinement_handoff(7, &reordered_expected, &reordered_records),
        Err(RefinementHandoffError::Association)
    );
}
