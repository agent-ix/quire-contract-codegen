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
    acquire_checked_closure, encode_refinement_query, CompileUnit, DependencyInput, DocumentRef,
    NodeKey, RefinementQueryKind, RefinementQueryRequest, RefinementQuerySubject,
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

fn key(package: &str, name: &str) -> String {
    format!("ix://{package}/{name}")
}

fn origin(identity: &str) -> Value {
    json!({"generated": {"generatorIdentity": identity, "generatorVersion": "1.0.0",
        "inputIdentities": [identity]}})
}

fn model_with_fields(package: &str, operations: &[&str], field_count: usize) -> Vec<u8> {
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
            "construct": {"identity": "none", "shape": "record", "members": {}, "meaning": meaning}
        })
    })
    .collect();
    let operations: Vec<_> = operations
        .iter()
        .map(|name| {
            let identity = key(package, &format!("Object/{name}"));
            let field_index = name[2..].parse::<usize>().expect("authored operation") % field_count;
            let field = key(package, &format!("Object/flag{field_index}"));
            json!({
                "identity": identity, "name": name, "params": [], "pre": [], "post": [],
                "origin": {"source": {"sourceIdentity": key(package, "spec"), "path": "spec.qspec",
                    "startLine": 1, "startColumn": 1}},
                "frame": {"modifies": [], "modifies_receiver": [field],
                    "creates": [], "deletes": []}
            })
        })
        .collect();
    let fields: Vec<_> = (0..field_count)
        .map(|index| {
            let name = format!("flag{index}");
            let identity = key(package, &format!("Object/{name}"));
            json!({"identity": identity, "name": name,
                "typeRef": "ix://quire/native/Boolean", "presence": "required",
                "nullable": false, "defaultKind": "none",
                "multiplicity": {"lower": 1, "upper": 1, "ordered": false, "unique": true},
                "origin": origin(&identity)})
        })
        .collect();
    let document = json!({
        "contractVersion": "2.0.0",
        "canonicalization": {"algorithm": "rfc8785-v1"},
        "source": {"identity": key(package, "spec"), "version": "1.0.0",
            "dialect": "spec-bundle"},
        "package": {"identity": package, "version": "1.0.0",
            "mappingVersions": [], "profileVersions": []},
        "occurrences": [], "extensions": [], "constructs": constructs,
        "types": [{"identity": object, "displayName": "Object",
            "kind": {"module": package, "name": "object_type"}, "roles": [],
            "origin": origin(&object), "constraints": [], "extensions": [],
            "unknownPolicy": "reject", "supertypes": [],
            "fields": fields,
            "operations": operations, "relationships": []}],
        "populations": [{"identity": population, "displayName": "objects",
            "kind": {"module": package, "name": "population"},
            "members": [object], "extent": "closed", "origin": origin(&population)}]
    });
    let limits = quire_canonical::Limits::new(1 << 20);
    quire_canonical::to_vec(&document, limits).expect("model bytes")
}

fn initial_with_fields(
    package: &str,
    snapshots: &mut BTreeMap<DocumentRef, Vec<u8>>,
    field_count: usize,
) -> DocumentRef {
    let name = format!("{package}/start");
    let fields: BTreeMap<_, _> = (0..field_count)
        .map(|index| (format!("flag{index}"), json!({"boolean": false})))
        .collect();
    let snapshot = json!({
        "format": "quire.state.snapshot/v1",
        "identity": {"authority": "test", "identity": name,
            "revision_namespace": "fixture", "revision": "fixture:1"},
        "observation": "current",
        "model": {"identity": package},
        "anchor": {"kind": "initialization", "name": "start"},
        "populations": [{"population": key(package, "objects"), "complete": true,
            "objects": [{"key": "only", "type": key(package, "Object"),
                "fields": fields}]}]
    });
    let limits = quire_canonical::Limits::new(1 << 20);
    let bytes = quire_canonical::to_vec(&snapshot, limits).expect("snapshot bytes");
    let reference = DocumentRef {
        authority: "test".to_owned(),
        identity: name,
    };
    snapshots.insert(reference.clone(), bytes);
    reference
}

fn authored_rows(row_count: usize, reverse: bool) -> String {
    let mut names: Vec<_> = (0..row_count).map(|index| format!("op{index}")).collect();
    if reverse {
        names.reverse();
    }
    names
        .into_iter()
        .map(|name| format!("step Concrete::Object::{name} -> stutter;\n"))
        .collect()
}

fn authored_unit(row_count: usize, reverse: bool) -> String {
    let mapping = (0..7)
        .map(|index| format!("flag{index} = self.flag{index};"))
        .collect::<Vec<_>>()
        .join(" ");
    format!(
        "language \"ix:native\" edition \"1-draft\";\n\
         profile inf = \"quire.temporal.infinite-trace/v1\";\n\
         model Abstract = \"{ABSTRACT}\";\n\
         model Concrete = \"{CONCRETE}\";\n\
         refinement SevenRows using inf abstract Abstract concrete Concrete {{\n\
         population Abstract::objects from Concrete::objects;\n\
         object Abstract::Object from Concrete::Object {{ {mapping} }}\n\
         {}\
         }}\n",
        authored_rows(row_count, reverse)
    )
}

type Expected = RefinementExpected<NodeKey, [String; 2]>;
type Run = RefinementRun<NodeKey, [String; 2]>;

fn source_records(reverse: bool) -> (Vec<Expected>, Vec<Run>) {
    source_records_with_rows(7, reverse)
}

fn with_source_request<T>(
    row_count: usize,
    reverse: bool,
    matched: bool,
    any: bool,
    body: impl FnOnce(&RefinementQueryRequest<'_>) -> T,
) -> T {
    let names: Vec<_> = (0..row_count).map(|index| format!("op{index}")).collect();
    let names: Vec<_> = names.iter().map(String::as_str).collect();
    let field_count = if matched { 1 } else { 7 };
    let abstract_bytes =
        model_with_fields(ABSTRACT, if matched { &names } else { &[] }, field_count);
    let concrete_bytes = model_with_fields(CONCRETE, &names, field_count);
    let mut packages = BTreeMap::new();
    packages.insert(ABSTRACT.to_owned(), abstract_bytes);
    packages.insert(CONCRETE.to_owned(), concrete_bytes);
    let mut snapshots = BTreeMap::new();
    let abstract_initial = initial_with_fields(ABSTRACT, &mut snapshots, field_count);
    let concrete_initial = initial_with_fields(CONCRETE, &mut snapshots, field_count);
    let mut source = authored_unit(row_count, reverse);
    if matched {
        // Seven named matching operations over one Boolean keep the genuine
        // positive proof finite without weakening any individual transition.
        for index in 1..7 {
            source = source.replace(&format!("flag{index} = self.flag{index};"), "");
        }
        for name in &names {
            source = source.replace(
                &format!("step Concrete::Object::{name} -> stutter;"),
                &format!("step Concrete::Object::{name} -> Abstract::Object::{name}(self);"),
            );
        }
    }
    if any {
        source = source.replace("-> stutter;", "-> any;");
    }
    let compiled = acquire_checked_closure(
        CompileUnit {
            source: SourceIdentity::new("agent-ix", "ir1120-refinement"),
            package_name: "ir1120-refinement".to_owned(),
            path: "ir1120-refinement.native",
            bytes: source.as_bytes(),
        },
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
        refinement.clone(),
        subject(CONCRETE, concrete_initial),
        subject(ABSTRACT, abstract_initial),
    );
    body(&request)
}

fn source_records_with_rows(row_count: usize, reverse: bool) -> (Vec<Expected>, Vec<Run>) {
    with_source_request(row_count, reverse, false, false, |request| {
        let refinement = request.refinement_key().clone();
        let mut expected = Vec::new();
        let mut records = Vec::new();
        for role in std::iter::once(RefinementRole::Base)
            .chain((0..row_count).map(|row| RefinementRole::Step { row }))
        {
            let kind = match role {
                RefinementRole::Base => RefinementQueryKind::Initial,
                RefinementRole::Step { row } => RefinementQueryKind::Step {
                    row: u64::try_from(row).expect("seven rows fit u64"),
                },
            };
            let query = encode_refinement_query(request, kind).expect("QSL canonical query");
            let execution = run_solver_query(
                QuerySelection {
                    product: (),
                    item: refinement.clone(),
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
                item: refinement.clone(),
                subjects: subjects.clone(),
                role,
                attempt,
                query_bytes: query,
            });
            records.push(Run::from_execution(subjects, execution));
        }
        (expected, records)
    })
}

/// A named step row carries its query bytes to its new declaration-order position.
#[trace("TC-059", "FR-045-AC-4")]
#[test]
fn source_row_permutation_moves_refinement_query_bindings_with_named_rows() {
    let (first_expected, first_records) = source_records(false);
    let (reordered_expected, mut reordered_records) = source_records(true);
    assert_eq!(first_expected.len(), 8);
    assert_eq!(reordered_expected.len(), 8);
    for left in 1..8 {
        for right in left + 1..8 {
            assert_ne!(
                first_expected[left].query_bytes,
                first_expected[right].query_bytes,
                "Step rows {} and {} must have distinct canonical queries",
                left - 1,
                right - 1
            );
        }
    }
    for source_row in 0..7 {
        let before = &first_expected[source_row + 1];
        let after = &reordered_expected[7 - source_row];
        assert_eq!(before.role, RefinementRole::Step { row: source_row });
        assert_eq!(
            after.role,
            RefinementRole::Step {
                row: 6 - source_row
            }
        );
        assert_eq!(before.item, after.item);
        assert_eq!(before.subjects, after.subjects);
        assert_eq!(
            before.query_bytes, after.query_bytes,
            "op{source_row} query moves with its row"
        );
        assert_ne!(before.attempt, after.attempt);
    }
    assert_eq!(first_expected[0].role, RefinementRole::Base);
    assert_eq!(reordered_expected[0].role, RefinementRole::Base);
    assert_eq!(
        first_expected[0].query_bytes,
        reordered_expected[0].query_bytes
    );
    assert!(refinement_handoff(7, &first_expected, &first_records).is_ok());
    reordered_records.reverse();
    assert!(refinement_handoff(7, &reordered_expected, &reordered_records).is_ok());
    let first = reordered_records
        .iter()
        .position(|record| record.role == RefinementRole::Step { row: 0 })
        .expect("op6 at row zero");
    let last = reordered_records
        .iter()
        .position(|record| record.role == RefinementRole::Step { row: 6 })
        .expect("op0 at row six");
    reordered_records[first].role = RefinementRole::Step { row: 6 };
    reordered_records[last].role = RefinementRole::Step { row: 0 };
    assert_eq!(
        refinement_handoff(7, &reordered_expected, &reordered_records),
        Err(RefinementHandoffError::Association)
    );
    reordered_records[first].role = RefinementRole::Step { row: 0 };
    reordered_records[last].role = RefinementRole::Step { row: 6 };
    reordered_records[last]
        .query_bytes
        .as_mut()
        .expect("query present")[0] ^= 1;
    assert_eq!(
        refinement_handoff(7, &reordered_expected, &reordered_records),
        Err(RefinementHandoffError::Association)
    );
    reordered_records[last]
        .query_bytes
        .as_mut()
        .expect("query present")[0] ^= 1;
    let base = reordered_records
        .iter()
        .position(|record| record.role == RefinementRole::Base)
        .expect("base record");
    reordered_records[base].role = RefinementRole::Step { row: 6 };
    reordered_records[last].role = RefinementRole::Base;
    assert_eq!(
        refinement_handoff(7, &reordered_expected, &reordered_records),
        Err(RefinementHandoffError::Association)
    );
}

/// A source-authored refinement with no step rows still requires one Base record.
#[trace("TC-059", "FR-045-AC-4")]
#[test]
fn source_refinement_with_zero_step_rows_has_only_base() {
    let (expected, records) = source_records_with_rows(0, false);
    assert_eq!(expected.len(), 1);
    assert_eq!(records.len(), 1);
    assert_eq!(expected[0].role, RefinementRole::Base);
    assert_eq!(records[0].role(), RefinementRole::Base);
    let bytes = refinement_handoff(0, &expected, &records).expect("Base-only handoff");
    assert_eq!(
        u64::from_le_bytes(bytes[..8].try_into().expect("record count")),
        1
    );
}

/// A genuine FR-315 refusal is retained before any solver can be selected.
#[trace("TC-059", "FR-045-AC-7")]
#[test]
fn production_refinement_preserves_encoding_refusal_and_cancellation() {
    use crate::routed::smt::refinement_driver::{prepare_refinement, RefinementPreparationError};
    with_source_request(1, false, false, true, |request| {
        let refusal = prepare_refinement(request, &Cancel::new());
        assert!(
            matches!(
                refusal,
                Err(RefinementPreparationError::Encoding {
                    role: Some(RefinementRole::Step { row: 0 }),
                    refusal: qsl_replay::SmtEncodingRefusal::Refused(_),
                })
            ),
            "the admitted any row must retain the original encoding refusal"
        );
        let cancel = Cancel::new();
        cancel.cancel(quire_exact::CancelCause::Requested);
        assert!(matches!(
            prepare_refinement(request, &cancel),
            Err(RefinementPreparationError::Cancelled(
                quire_exact::CancelCause::Requested
            ))
        ));
    });
}

/// Real cvc5 proofs cross the public request adapter without a copied certificate.
#[trace("TC-059", "FR-045-AC-4", "FR-045-AC-5")]
#[test]
fn production_refinement_live_solver_handoff_reaches_qsl_checker() {
    use crate::routed::smt::refinement_driver::prepare_refinement;
    let executable = std::env::var_os("QUIRE_CVC5_BIN")
        .expect("live refinement qualification requires QUIRE_CVC5_BIN");
    with_source_request(7, false, true, false, |request| {
        let cancel = Cancel::new();
        let prepared = prepare_refinement(request, &cancel).expect("matched queries encode");
        let execution = prepared
            .execute(
                std::path::Path::new(&executable),
                &[
                    "--lang=smt2".into(),
                    "--dump-proofs".into(),
                    "--proof-format-mode=alethe".into(),
                ],
                ProcessLimits {
                    input_bytes: NonZeroUsize::new(16 << 20).expect("finite"),
                    wall_clock: Duration::from_secs(60),
                    memory_bytes: NonZeroU64::new(1 << 30).expect("finite"),
                    output_bytes: NonZeroUsize::new(64 << 20).expect("finite"),
                },
                &cancel,
            )
            .expect("direct bounded solver invocations");
        assert_eq!(execution.records().len(), 8);
        for (expected, record) in execution.expected().iter().zip(execution.records()) {
            assert_eq!(record.item(), request.refinement_key());
            assert!(std::ptr::eq(record.subjects().request(), request));
            assert_eq!(record.query_bytes(), Some(expected.query_bytes.as_slice()));
            assert_eq!(
                record.status(),
                crate::routed::smt::SolverStatus::Unsat,
                "role {:?}, stderr {:?}, stdout {:?}",
                record.role(),
                record.process().stderr,
                record.process().stdout
            );
            assert!(record.proof_bytes().is_some());
        }
        let handoff = execution
            .handoff()
            .expect("complete authenticated collection");
        assert_eq!(execution.handoff().expect("repeat"), handoff);
        let limits = qsl_replay::CertificateCheckLimits::default();
        let certificate = request
            .assemble_smt_certificate(&handoff, &limits, &cancel)
            .expect("public QSL assembly");
        let checked = request
            .check_smt_proof(&certificate, &limits, &cancel)
            .expect("QSL proof reader accepts real emitted Alethe");
        assert!(
            matches!(checked, qsl_replay::SmtProofCheck::Verified),
            "real proof result: {checked:?}"
        );
    });
}
