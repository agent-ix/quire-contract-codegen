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
    let field_count = 7;
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
        // Each matched row changes its own field and preserves the other six.
        // The resulting symbolic frame contradiction differs for every row.
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
            qsl_replay::CallerLimits::default()
                .spine
                .package
                .with_output_bytes(1 << 20)
                .emit(),
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
                    refusal: qsl_replay::SmtEncodingRefusal::Refused(
                        qsl_replay::Refusal::RefinementAnyRow { row: 0 }
                    ),
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
        if let Some(directory) = std::env::var_os("QUIRE_SMT_EVIDENCE_DIR") {
            let directory = std::path::Path::new(&directory);
            std::fs::create_dir_all(directory).expect("external evidence directory");
            for record in execution.records() {
                let name = match record.role() {
                    RefinementRole::Base => "base".to_owned(),
                    RefinementRole::Step { row } => format!("step-{row}"),
                };
                for (suffix, bytes) in [
                    ("query.smt2", record.query_bytes()),
                    ("stdout", record.process().stdout.as_deref()),
                    ("stderr", record.process().stderr.as_deref()),
                    ("alethe", record.proof_bytes()),
                ] {
                    if let Some(bytes) = bytes {
                        std::fs::write(directory.join(format!("{name}.{suffix}")), bytes)
                            .expect("external actual invocation evidence");
                    }
                }
            }
        }
        for (expected, record) in execution.expected().iter().zip(execution.records()) {
            assert_eq!(record.item(), request.refinement_key());
            assert!(std::ptr::eq(record.subjects().request(), request));
            assert_eq!(record.query_bytes(), Some(expected.query_bytes.as_slice()));
            assert_eq!(
                record.status(),
                crate::routed::smt::SolverStatus::Unsat,
                "role {:?}, stderr bytes {:?}, stdout bytes {:?}",
                record.role(),
                record.process().stderr.as_ref().map(Vec::len),
                record.process().stdout.as_ref().map(Vec::len)
            );
            assert!(record.proof_bytes().is_some());
        }
        for (index, left) in execution.records().iter().enumerate() {
            for right in &execution.records()[index + 1..] {
                assert_ne!(
                    left.query_bytes(),
                    right.query_bytes(),
                    "roles {:?}/{:?} retain distinct authoritative queries",
                    left.role(),
                    right.role()
                );
                assert_ne!(
                    left.proof_bytes(),
                    right.proof_bytes(),
                    "roles {:?}/{:?} retain distinct actual emitted proofs",
                    left.role(),
                    right.role()
                );
            }
        }
        let handoff = execution
            .handoff()
            .expect("complete authenticated collection");
        assert_eq!(execution.handoff().expect("repeat"), handoff);
        let limits = qsl_replay::CertificateCheckLimits::default();
        let certificate = request
            .assemble_smt_certificate(&handoff, &limits, &cancel)
            .expect("public QSL assembly");
        assert_eq!(certificate.steps.len(), 7);
        for (record, part) in execution
            .records()
            .iter()
            .zip(std::iter::once(&certificate.base).chain(&certificate.steps))
        {
            assert_eq!(Some(part.query.as_slice()), record.query_bytes());
            assert_eq!(Some(part.proof.as_slice()), record.proof_bytes());
        }
        let checked = request
            .check_smt_proof(&certificate, &limits, &cancel)
            .expect("QSL native proof reader completes on real emitted Alethe");
        assert!(
            matches!(
                checked,
                qsl_replay::SmtProofCheck::Verified
                    | qsl_replay::SmtProofCheck::Unverifiable {
                        part: qsl_replay::QueryPart::Step { row: Some(0) },
                        step: 2,
                    }
            ),
            "unchanged real proof must be Verified or the independently observed unsupported symm command: {checked:?}"
        );
    });
}

/// The production Base-only case retains unavailable execution as non-proof evidence.
#[trace("TC-059", "FR-045-AC-4", "FR-045-AC-7")]
#[test]
fn production_refinement_zero_rows_preserves_missing_tool() {
    use crate::routed::smt::refinement_driver::prepare_refinement;
    with_source_request(0, false, false, false, |request| {
        let cancel = Cancel::new();
        let execution = prepare_refinement(request, &cancel)
            .expect("Initial encodes")
            .execute(
                std::path::Path::new("/nonexistent/ir1120/solver"),
                &[],
                ProcessLimits {
                    input_bytes: NonZeroUsize::new(1 << 20).expect("finite"),
                    wall_clock: Duration::from_secs(1),
                    memory_bytes: NonZeroU64::new(128 << 20).expect("finite"),
                    output_bytes: NonZeroUsize::new(1 << 20).expect("finite"),
                },
                &cancel,
            )
            .expect("unavailable execution is a typed record");
        assert_eq!(execution.records().len(), 1);
        assert_eq!(execution.records()[0].role(), RefinementRole::Base);
        assert_eq!(
            execution.records()[0].status(),
            crate::routed::smt::SolverStatus::MissingTool
        );
        assert!(!execution.records()[0].process().started);
        assert!(execution.records()[0].proof_bytes().is_none());
        assert_eq!(
            execution.handoff(),
            Err(RefinementHandoffError::NonProof(
                crate::routed::smt::SolverStatus::MissingTool
            ))
        );
    });
}

/// Production preparation/execution follows named rows through both source
/// orders. Synthetic stdout supplies association/framing credit only.
#[trace("TC-059", "FR-045-AC-4")]
#[test]
fn production_refinement_source_permutation_preserves_named_query_associations() {
    use crate::routed::smt::refinement_driver::prepare_refinement;
    let collect = |reverse| {
        with_source_request(7, reverse, false, false, |request| {
            let authoritative = std::iter::once(RefinementQueryKind::Initial)
                .chain((0..7).map(|row| RefinementQueryKind::Step { row }))
                .map(|kind| {
                    encode_refinement_query(request, kind).expect("authoritative QSL query")
                })
                .collect::<Vec<_>>();
            let cancel = Cancel::new();
            let execution = prepare_refinement(request, &cancel)
                .expect("all source queries prepare")
                .execute(
                    std::path::Path::new("/bin/sh"),
                    &[
                        "-c".into(),
                        "cat >&2; printf 'unsat\\n((proof))'".into(),
                        "--dump-proofs".into(),
                        "--proof-format-mode=alethe".into(),
                    ],
                    ProcessLimits {
                        input_bytes: NonZeroUsize::new(1 << 20).expect("finite"),
                        wall_clock: Duration::from_secs(5),
                        memory_bytes: NonZeroU64::new(128 << 20).expect("finite"),
                        output_bytes: NonZeroUsize::new(1 << 20).expect("finite"),
                    },
                    &cancel,
                )
                .expect("production direct execution");
            assert_eq!(execution.records().len(), 8);
            let mut retained = Vec::new();
            for (slot, ((expected, record), query)) in execution
                .expected()
                .iter()
                .zip(execution.records())
                .zip(authoritative)
                .enumerate()
            {
                let role = if slot == 0 {
                    RefinementRole::Base
                } else {
                    RefinementRole::Step { row: slot - 1 }
                };
                assert_eq!(record.role(), role);
                assert_eq!(expected.role, role);
                assert_eq!(record.item(), request.refinement_key());
                assert!(std::ptr::eq(record.subjects().request(), request));
                assert_eq!(record.query_bytes(), Some(query.as_slice()));
                assert_eq!(
                    record.process().stderr.as_deref(),
                    Some(query.as_slice()),
                    "the selected process receives exactly the independently encoded query"
                );
                assert_eq!(expected.query_bytes, query);
                assert_eq!(record.attempt(), expected.attempt);
                assert_eq!(record.status(), crate::routed::smt::SolverStatus::Unsat);
                retained.push((role, query, record.attempt()));
            }
            assert!(
                execution.handoff().is_ok(),
                "framing/association only, no QSL proof credit"
            );
            retained
        })
    };
    let first = collect(false);
    let reversed = collect(true);
    assert_eq!(
        first[0].1, reversed[0].1,
        "Base is unchanged by the row permutation"
    );
    for named_row in 0..7 {
        let old = &first[named_row + 1];
        let new = &reversed[7 - named_row];
        assert_eq!(old.0, RefinementRole::Step { row: named_row });
        assert_eq!(new.0, RefinementRole::Step { row: 6 - named_row });
        assert_eq!(
            old.1, new.1,
            "op{named_row}'s actual query follows its declared position"
        );
        assert_ne!(old.2, new.2, "new genuine invocations have new attempts");
    }
    let attempts = first
        .iter()
        .chain(&reversed)
        .map(|entry| entry.2)
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(
        attempts.len(),
        16,
        "every production invocation has its own attempt"
    );
    for left in 1..8 {
        for right in left + 1..8 {
            assert_ne!(
                first[left].1, first[right].1,
                "named rows have distinct QSL queries"
            );
        }
    }
}

/// The first selected process replaces its own executable with a directory.
/// Its real completed record survives the next invocation's typed selection
/// error. Synthetic proof stdout supplies no live proof-reader credit.
#[trace("TC-059", "FR-045-AC-7")]
#[test]
fn production_refinement_retains_completed_records_after_a_later_mechanism_error() {
    use crate::{
        kani::run::process::{ProcessError, ProcessFailurePhase},
        routed::smt::{
            refinement_driver::{prepare_refinement, RefinementExecutionError},
            SolverRunError,
        },
    };
    use std::os::unix::fs::PermissionsExt;
    let directory = std::env::temp_dir().join(format!(
        "cg-refinement-selected-executable-{}",
        std::process::id(),
    ));
    std::fs::create_dir(&directory).expect("create exclusive owned executable directory");
    let executable = directory.join("selected-solver");
    std::fs::write(
        &executable,
        "#!/bin/sh\ncat >&2\nrm -- \"$0\"\nmkdir -- \"$0\"\nprintf 'unsat\\n((proof))'\n",
    )
    .expect("write the actual selected executable");
    std::fs::set_permissions(&executable, std::fs::Permissions::from_mode(0o700))
        .expect("make the selected executable runnable");
    with_source_request(1, false, false, false, |request| {
        let base_query = encode_refinement_query(request, RefinementQueryKind::Initial)
            .expect("independent authoritative Base bytes");
        let cancel = Cancel::new();
        let result = prepare_refinement(request, &cancel)
            .expect("both queries prepare")
            .execute(
                &executable,
                &["--dump-proofs".into(), "--proof-format-mode=alethe".into()],
                ProcessLimits {
                    input_bytes: NonZeroUsize::new(1 << 20).expect("finite"),
                    wall_clock: Duration::from_secs(5),
                    memory_bytes: NonZeroU64::new(128 << 20).expect("finite"),
                    output_bytes: NonZeroUsize::new(1 << 20).expect("finite"),
                },
                &cancel,
            );
        let Err(RefinementExecutionError::Process {
            role,
            error,
            completed,
        }) = result
        else {
            panic!("the later non-file selection must preserve a typed process error");
        };
        assert_eq!(role, RefinementRole::Step { row: 0 });
        assert!(
            executable.is_dir(),
            "the first genuine process completed its deterministic change"
        );
        let SolverRunError::Process(ProcessError::Io { phase, source }) = error else {
            panic!("the original native process-selection error remains distinct");
        };
        assert_eq!(phase, ProcessFailurePhase::Selection);
        assert_eq!(source.kind(), std::io::ErrorKind::Other);
        assert_eq!(completed.records().len(), 1);
        assert_eq!(completed.expected().len(), 1);
        let record = &completed.records()[0];
        let expected = &completed.expected()[0];
        assert_eq!(record.role(), RefinementRole::Base);
        assert_eq!(record.item(), request.refinement_key());
        assert!(std::ptr::eq(record.subjects().request(), request));
        assert_eq!(record.query_bytes(), Some(base_query.as_slice()));
        assert_eq!(
            record.process().stderr.as_deref(),
            Some(base_query.as_slice())
        );
        assert_eq!(
            record.process().stdout.as_deref(),
            Some(b"unsat\n((proof))".as_slice())
        );
        assert_eq!(record.proof_bytes(), Some(b"(proof)".as_slice()));
        assert!(record.process().started);
        assert!(matches!(
            record.process().outcome,
            crate::kani::run::process::ProcessOutcome::Completed { success: true, .. }
        ));
        assert_eq!(record.status(), crate::routed::smt::SolverStatus::Unsat);
        assert_eq!(expected.item, *request.refinement_key());
        assert_eq!(expected.role, record.role());
        assert_eq!(expected.attempt, record.attempt());
        assert_eq!(expected.query_bytes, base_query);
        assert_eq!(
            completed.handoff(),
            Err(RefinementHandoffError::Shape),
            "a retained prefix is not a complete certificate collection"
        );
    });
    std::fs::remove_dir_all(directory).expect("remove owned executable directory");
}
