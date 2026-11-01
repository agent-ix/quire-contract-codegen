//! Whole-object collection operation contexts from authentic QSL emission and IR admission.

use ix_trace_rs::trace;
use qsl_replay::{call_site, ClauseName, DependencyInput, Identifier, SourceIdentity};
use quire_contract_codegen::{
    negotiate_kani_obligations, CollectionKindIdentity, KaniObligationOutcome,
    KaniObligationRequest, KaniV2ClauseObligationHarness, ObligationItem,
};
use quire_contract_model::{
    CheckedNodeId, CheckedPackageEvidence, CheckedPackageReadLimits, CheckedPackageV2,
    CheckedPackageV2ReadResult, CheckedTerm,
};
use serde_json::{json, Value};
use std::time::Duration;

const MODEL: &str = "ir1091.operation/domain";
const OBJECT: &str = "ix://ir1091.operation/domain/Node";
const SMALL: &str = "ix://ir1091.operation/domain/Small";

fn origin(identity: &str) -> Value {
    json!({"generated":{"generatorIdentity":"ix://ir1091.operation/spec", "generatorVersion":"1.0.0", "inputIdentities":[identity]}})
}

fn document() -> Value {
    let field = |name: &str, ty: &str, lower: u64, upper: u64, ordered: bool, unique: bool| {
        let identity = format!("{OBJECT}/{name}");
        json!({"identity":identity,"name":name,"typeRef":ty,"presence":"required","nullable":false,
            "defaultKind":"none","origin":origin(&identity),"multiplicity":{"lower":lower,"upper":upper,"ordered":ordered,"unique":unique}})
    };
    let parameter = |name: &str, ty: &str| {
        let identity = format!("{OBJECT}/step/{name}");
        json!({"identity":identity,"name":name,"typeRef":ty,"presence":"required","nullable":false,"defaultKind":"none",
            "multiplicity":{"lower":1,"upper":1,"ordered":false,"unique":true},"origin":origin(&identity)})
    };
    json!({
        "contractVersion":"2.0.0", "canonicalization":{"algorithm":"rfc8785-v1"},
        "source":{"identity":"ix://ir1091.operation/spec","version":"1.0.0","dialect":"spec-bundle"},
        "package":{"identity":MODEL,"version":"1.0.0","mappingVersions":[],"profileVersions":[]},
        "constructs":[
            {"kind":{"module":MODEL,"name":"entity"},"moduleVersion":"1.0.0","construct":{"identity":"none","shape":"record","members":{},"meaning":"quire.meaning.model.object-type/v1"}},
            {"kind":{"module":MODEL,"name":"value_type"},"moduleVersion":"1.0.0","construct":{"identity":"none","shape":"record","members":{},"meaning":"quire.meaning.model.value-type/v1"}}
        ],
        "occurrences":[],"extensions":[],
        "types":[
            {"identity":SMALL,"displayName":"Small","kind":{"module":MODEL,"name":"value_type"},"roles":[],"origin":origin(SMALL),
                "extensions":[],"unknownPolicy":"reject","scalar":"integer","constraints":[
                    {"identity":format!("{SMALL}/min"),"keyword":"min","operands":{"value":"0"},"appliesTo":"ix://quire/native/Integer","diagnosticCode":"operation.minimum","origin":origin(SMALL)},
                    {"identity":format!("{SMALL}/max"),"keyword":"max","operands":{"value":"3"},"appliesTo":"ix://quire/native/Integer","diagnosticCode":"operation.maximum","origin":origin(SMALL)}]},
            {"identity":OBJECT,"displayName":"Node","kind":{"module":MODEL,"name":"entity"},"roles":[],"origin":origin(OBJECT),
                "constraints":[],"extensions":[],"unknownPolicy":"reject","supertypes":[],
                "fields":[field("z_items",SMALL,0,3,true,true),field("a_unread","ix://quire/native/Boolean",1,1,false,true),field("m_count",SMALL,1,1,false,true)],
                "operations":[{"identity":format!("{OBJECT}/step"),"name":"step","params":[parameter("limit",SMALL),parameter("enabled","ix://quire/native/Boolean")],
                    "pre":[],"post":[],"origin":origin(OBJECT),"frame":{"modifies":[format!("{OBJECT}/z_items"),format!("{OBJECT}/m_count"),format!("{OBJECT}/a_unread")],"creates":[],"deletes":[]}}]}
        ],"populations":[]
    })
}

fn fixture(doc: &Value, preconditions: &str) -> CheckedPackageV2 {
    fixture_with_body(
        doc,
        preconditions,
        "forall(item in self.z_items: item >= 0)",
    )
}

fn fixture_with_body(doc: &Value, preconditions: &str, body: &str) -> CheckedPackageV2 {
    let bytes = quire_canonical::to_vec(doc, quire_canonical::Limits::new(1 << 20)).unwrap();
    let source = format!("language \"ix:native\" edition \"1-draft\";\nprofile v = \"quire.model.complete/v1\";\nmodel M = \"{MODEL}\";\n{preconditions}\npost CollectionReady using v on M::Node::step {{ {body} }}\n");
    let site = call_site(
        SourceIdentity::new("agent-ix", "ir1091-operation.native", "git", "fixture"),
        "ir1091-operation".to_owned(),
        "ir1091-operation.native",
        source.as_bytes(),
        [bytes.as_slice()],
        &DependencyInput::default(),
        &ClauseName(Identifier::new("CollectionReady").unwrap()),
    )
    .expect("QSL emits the authored operation clauses");
    let mut evidence = CheckedPackageEvidence::new();
    evidence.support_feature("quire.value.complete/v1");
    evidence
        .insert_domain_package_document(MODEL, bytes)
        .unwrap();
    match CheckedPackageV2::read(
        &site.package,
        CheckedPackageReadLimits::bounded(),
        &evidence,
    ) {
        CheckedPackageV2ReadResult::Admitted(package) => *package,
        other => panic!("strict reader must admit QSL emission: {other:?}"),
    }
}

fn clauses(package: &CheckedPackageV2) -> Vec<CheckedNodeId> {
    package.graph().nodes.iter().filter(|node| matches!(package.term(&node.node_id), Some(CheckedTerm::Application { operation, .. }) if operation.state_clause_kind().ok().flatten().is_some()))
        .map(|node| node.node_id.clone()).collect()
}

fn negotiate(package: &CheckedPackageV2, ids: &[CheckedNodeId]) -> KaniObligationOutcome {
    let items = ids
        .iter()
        .map(|clause| ObligationItem::V2Clause {
            package,
            clause,
            dependencies: None,
        })
        .collect::<Vec<_>>();
    negotiate_kani_obligations(&KaniObligationRequest {
        ceilings: crate::common::proof_ceilings::proof_ceilings_with_wall_clock(
            Duration::from_secs(600),
        ),
        items: &items,
        subject_path: "crate::subject",
        unwind: 8,
    })
    .unwrap()
}

fn postcondition(outcome: KaniObligationOutcome) -> KaniV2ClauseObligationHarness {
    let KaniObligationOutcome::Emitted {
        v2_clause_harnesses,
        records,
        ..
    } = outcome
    else {
        panic!("valid request")
    };
    v2_clause_harnesses
        .into_iter()
        .find(|harness| {
            harness.identity.kind == quire_contract_codegen::ObligationKind::Postcondition
        })
        .unwrap_or_else(|| panic!("one collection postcondition harness: {records:?}"))
}

#[test]
#[trace("TC-025", "FR-015-AC-96", "FR-015-AC-99")]
fn operation_context_retains_every_field_and_parameter_in_declared_order() {
    let package = fixture(&document(), "");
    let ids = clauses(&package);
    let harness = postcondition(negotiate(&package, &ids));
    let context = harness.identity.collection_operation.as_ref().unwrap();
    assert_eq!(
        context
            .fields
            .iter()
            .map(|field| field.field_name.as_str())
            .collect::<Vec<_>>(),
        ["z_items", "a_unread", "m_count"]
    );
    assert_eq!(
        context.fields[0].collection_kind,
        Some(CollectionKindIdentity::OrderedSet)
    );
    assert_eq!(
        context
            .parameters
            .iter()
            .map(|value| value.name.as_str())
            .collect::<Vec<_>>(),
        ["limit", "enabled"]
    );
    assert!(context.fields.iter().all(|field| package
        .model_object_fields(&field.declaring_node_id)
        .unwrap()
        .fields()
        .iter()
        .any(|actual| actual.name() == field.field_name
            && *actual.declaring_node_id() == field.declaring_node_id)));
    assert!(harness.rust.contents.contains("build_typed_prestate"));
    assert!(harness.rust.contents.contains("inspect_typed_poststate"));
    assert!(harness.rust.contents.contains("crate::subject(&mut post.field_0, &mut post.field_1, &mut post.field_2, parameter_0, parameter_1)"));
}

struct Executable {
    scratch: crate::scratch_crate::TemporaryDirectory,
}

impl Executable {
    fn compile(harness: &KaniV2ClauseObligationHarness) -> Self {
        use std::{fs, process::Command};
        let scratch =
            crate::scratch_crate::TemporaryDirectory::new("ir1091-whole-object-execution");
        // Only proof-driver attributes are removed. The generated domain construction,
        // subject invocation, schema authorities and assertions execute without alteration.
        let generated = harness
            .rust
            .contents
            .replace("#[cfg(kani)]", "")
            .replace("#[kani::proof]", "")
            .replace(
                &format!("fn {}()", harness.identity.harness_symbol),
                &format!("pub(super) fn {}()", harness.identity.harness_symbol),
            );
        let driver = r#"
use std::sync::{Mutex, atomic::{AtomicUsize, Ordering}};
static DRAWS: Mutex<std::collections::VecDeque<i64>> = Mutex::new(std::collections::VecDeque::new());
static COVERS: AtomicUsize = AtomicUsize::new(0);
static COVER_REACHED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub trait Draw { fn draw() -> Self; }
impl Draw for i64 { fn draw() -> Self { DRAWS.lock().unwrap().pop_front().expect("all symbolic draws supplied") } }
impl Draw for bool { fn draw() -> Self { match <i64 as Draw>::draw() { 0 => false, 1 => true, _ => panic!("Boolean draw") } } }
impl<T: Draw, const N: usize> Draw for [T; N] { fn draw() -> Self { std::array::from_fn(|_| T::draw()) } }
pub fn any<T: Draw>() -> T { T::draw() }
pub fn assume(condition: bool) { assert!(condition, "symbolic input excluded"); }
pub fn seed(values: Vec<i64>) { *DRAWS.lock().unwrap() = values.into(); }
pub fn cover(condition: bool) { COVERS.fetch_add(1, Ordering::SeqCst); if condition { COVER_REACHED.store(true, Ordering::SeqCst); } }
pub fn finish() { assert!(DRAWS.lock().unwrap().is_empty(), "every declared state and parameter draw is consumed"); assert_eq!(COVERS.load(Ordering::SeqCst), 1); if std::env::args().nth(4).as_deref() == Some("cover") { assert!(COVER_REACHED.load(Ordering::SeqCst), "an admitted draw reaches the actual generated cover"); } }
#[macro_export] macro_rules! cover { ($condition:expr, $message:expr) => { $crate::cover($condition) }; }
"#;
        fs::write(scratch.path.join("kani.rs"), driver).unwrap();
        let output = Command::new("timeout")
            .args([
                "120s",
                "rustc",
                "--edition=2021",
                "--crate-name",
                "kani",
                "--crate-type=rlib",
                "kani.rs",
                "-o",
                "libkani.rlib",
            ])
            .current_dir(&scratch.path)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "compile deterministic draw driver: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let subject = r#"
fn subject(items: &mut quire_exact::Value, unread: &mut quire_exact::Value, count: &mut quire_exact::Value, limit: i64, enabled: bool) {
    assert_eq!(limit, 2, "operation parameter order and value");
    assert!(enabled, "Boolean parameter reaches the subject");
    assert!(matches!(unread, quire_exact::Value::Boolean(true)), "unread field reaches the subject");
    assert!(matches!(count, quire_exact::Value::Integer(value) if value == &quire_exact::Integer::from(2_i64)), "declaration order binds the count field");
    let quire_exact::Value::Collection(collection) = items else { panic!("exact collection ABI"); };
    let expected = std::env::args().nth(3).unwrap_or_else(|| "1,2".into()).split(',').filter(|part| !part.is_empty())
        .map(|part| part.parse::<i64>().unwrap()).collect::<Vec<_>>();
    assert_eq!(collection.elements().len(), expected.len());
    for (actual, expected) in collection.elements().iter().zip(&expected) {
        match actual {
            quire_exact::Value::Integer(actual) => assert_eq!(actual, &quire_exact::Integer::from(*expected), "the symbolic input order is retained"),
            quire_exact::Value::Boolean(actual) => assert_eq!(*actual, *expected != 0, "Boolean occurrence order is retained"),
            _ => panic!("supported exact primitive elements"),
        }
    }
    let action = std::env::args().nth(1).unwrap();
    match action.as_str() {
        "field-type" => *unread = quire_exact::Value::Integer(quire_exact::Integer::from(0_i64)),
        "field-domain" => *count = quire_exact::Value::Integer(quire_exact::Integer::from(99_i64)),
        "duplicate" | "element-domain" | "cardinality" | "collection-kind" | "below-minimum" | "reorder" => {
            let mut ty = collection.collection_type().clone();
            let values = match action.as_str() {
                "duplicate" => vec![1,1], "element-domain" => vec![99], "cardinality" => vec![0,1,2,3], "below-minimum" => vec![], "reorder" => vec![2,1], _ => vec![1,2],
            }.into_iter().map(|value: i64| quire_exact::Value::Integer(quire_exact::Integer::from(value))).collect();
            if action == "collection-kind" { ty = quire_exact::CollectionType::new(quire_exact::CollectionKind::Sequence, ty.element().clone(), ty.bound()); }
            *items = quire_exact::from_admitted(ty, values);
        }
        "healthy" | "result-domain" => {},
        _ => panic!("unknown subject action"),
    }
}
"#;
        let context = harness.identity.collection_operation.as_ref().unwrap();
        let subject = if let Some(result) = &context.result {
            let (ty, value) = match result.domain {
                quire_contract_codegen::CollectionElementIdentity::Boolean => ("bool", "true"),
                quire_contract_codegen::CollectionElementIdentity::Integer { .. } => {
                    ("i64", "if action == \"result-domain\" { 99 } else { 2 }")
                }
            };
            let subject = subject.replace(
                "limit: i64, enabled: bool) {",
                &format!("limit: i64, enabled: bool) -> {ty} {{"),
            );
            let last = subject.rfind('}').unwrap();
            format!("{}\n{value}\n}}\n", &subject[..last])
        } else {
            subject.to_owned()
        };
        let main = format!("\nfn main() {{ let draws = std::env::args().nth(2).unwrap().split(',').map(|part| part.parse::<i64>().unwrap()).collect(); kani::seed(draws); {}::{}(); kani::finish(); }}", harness.identity.module_symbol, harness.identity.harness_symbol);
        fs::write(
            scratch.path.join("main.rs"),
            format!("{subject}\n{generated}\n{main}"),
        )
        .unwrap();
        let deps = std::env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf();
        let exact = fs::read_dir(&deps)
            .unwrap()
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .find(|path| {
                path.file_name()
                    .unwrap()
                    .to_string_lossy()
                    .starts_with("libquire_exact-")
                    && path
                        .extension()
                        .is_some_and(|extension| extension == "rlib")
            })
            .expect("current locked exact library");
        let output = Command::new("timeout")
            .args(["120s", "rustc", "--edition=2021", "main.rs", "-L"])
            .arg(format!("dependency={}", deps.display()))
            .arg("--extern")
            .arg(format!("quire_exact={}", exact.display()))
            .args(["--extern", "kani=libkani.rlib", "-o", "operation"])
            .current_dir(&scratch.path)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "the actual generated Rust must compile: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        Self { scratch }
    }

    fn run(&self, action: &str, draws: &str, expected_order: &str) -> std::process::Output {
        std::process::Command::new("timeout")
            .arg("30s")
            .arg(self.scratch.path.join("operation"))
            .args([action, draws, expected_order])
            .output()
            .unwrap()
    }
}

#[test]
#[trace("TC-025", "FR-015-AC-96", "FR-015-AC-99")]
fn operation_context_executes_exact_state_and_rejects_every_bad_produced_axis() {
    let package = fixture(&document(), "");
    let harness = postcondition(negotiate(&package, &clauses(&package)));
    let executable = Executable::compile(&harness);
    // Declared length, all three symbolic element slots, unread Boolean, count,
    // integer operation parameter, Boolean operation parameter. No corpus is emitted.
    for (draws, order) in [
        ("0,3,2,1,1,2,2,1", ""),
        ("1,3,2,1,1,2,2,1", "3"),
        ("2,2,1,0,1,2,2,1", "2,1"),
        ("3,3,0,2,1,2,2,1", "3,0,2"),
    ] {
        let output = executable.run("healthy", draws, order);
        assert!(
            output.status.success(),
            "inclusive lengths and element/order preservation: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let excluded = executable.run("healthy", "2,1,1,0,1,2,2,1", "1,1");
    assert!(!excluded.status.success());
    assert!(
        String::from_utf8_lossy(&excluded.stderr).contains("symbolic input excluded"),
        "unique input excludes duplicates before subject invocation"
    );
    for action in [
        "duplicate",
        "element-domain",
        "cardinality",
        "collection-kind",
        "field-type",
        "field-domain",
    ] {
        let output = executable.run(action, "2,1,2,0,1,2,2,1", "1,2");
        assert!(!output.status.success(), "produced {action} must fail");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("every produced"), "{action} fails a produced-domain assertion, not an input assumption or setup: {stderr}");
        assert!(!stderr.contains("symbolic input excluded"));
    }
}

#[test]
#[trace("TC-025", "FR-015-AC-96", "FR-015-AC-99")]
fn operation_context_requires_and_assumes_same_anchor_preconditions() {
    let package = fixture(&document(), "pre Ready using v on M::Node::step { self.m_count >= limit and enabled }\npre ItemsReady using v on M::Node::step { forall(item in self.z_items: item >= 0) }");
    let ids = clauses(&package);
    let harness = postcondition(negotiate(&package, &ids));
    assert_eq!(
        harness
            .identity
            .collection_operation
            .as_ref()
            .unwrap()
            .preconditions
            .len(),
        2
    );
    let executable = Executable::compile(&harness);
    let output = executable.run("healthy", "2,1,2,0,1,2,2,1", "1,2");
    assert!(
        output.status.success(),
        "supported siblings share prestate and parameters: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let output = executable.run("healthy", "2,1,2,0,1,1,2,1", "1,2");
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("symbolic input excluded"));
    let post = ids.iter().find(|id| matches!(package.term(id), Some(CheckedTerm::Application { operation, .. }) if operation.state_clause_kind().ok().flatten() == Some(quire_contract_model::StateClauseKind::Postcondition))).unwrap();
    let outcome = negotiate(&package, std::slice::from_ref(post));
    let KaniObligationOutcome::Emitted {
        records,
        v2_clause_harnesses,
        ..
    } = outcome
    else {
        panic!("valid incomplete item census")
    };
    assert!(v2_clause_harnesses.is_empty());
    assert!(matches!(
        records[0].disposition,
        quire_contract_codegen::ObligationDisposition::Unsupported {
            reason: quire_contract_codegen::UnsupportedObligation::CollectionClaimNotRendered {
                reason:
                    quire_contract_codegen::CollectionUnsupportedReason::MissingSiblingPrecondition,
                ..
            }
        }
    ));
}

#[test]
#[trace("TC-025", "FR-015-AC-96", "FR-015-AC-99")]
fn operation_context_checks_result_domain_and_the_authored_postcondition() {
    let mut doc = document();
    doc["types"][1]["operations"][0]["returns"] = json!({"typeRef":SMALL,"nullable":false,"multiplicity":{"lower":1,"upper":1,"ordered":false,"unique":true}});
    let package = fixture(&doc, "");
    let harness = postcondition(negotiate(&package, &clauses(&package)));
    let context = harness.identity.collection_operation.as_ref().unwrap();
    assert_eq!(context.result.as_ref().unwrap().name, "result");
    assert_eq!(
        context
            .parameters
            .iter()
            .map(|value| value.name.as_str())
            .collect::<Vec<_>>(),
        ["limit", "enabled"]
    );
    let executable = Executable::compile(&harness);
    let output = executable.run("healthy", "2,1,2,0,1,2,2,1", "1,2");
    assert!(
        output.status.success(),
        "declared primitive result: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let output = executable.run("result-domain", "2,1,2,0,1,2,2,1", "1,2");
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr)
        .contains("the produced result retains its admitted domain"));

    drop(executable);
    let package = fixture_with_body(&document(), "", "exists(item in self.z_items: item = 2)");
    let harness = postcondition(negotiate(&package, &clauses(&package)));
    let executable = Executable::compile(&harness);
    let output = executable.run("healthy", "2,1,3,0,1,2,2,1", "1,3");
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr)
        .contains("the produced state satisfies the authored collection postcondition"));
}

#[test]
#[trace("TC-025", "FR-015-AC-96", "FR-015-AC-97", "FR-015-AC-102")]
fn unsupported_whole_object_axes_have_typed_refusals_and_no_harness() {
    use quire_contract_codegen::{
        CollectionUnsupportedReason as Reason, ObligationDisposition, UnsupportedObligation,
    };
    for (axis, reason) in [
        ("state", Reason::UnsupportedObjectState),
        ("parameter", Reason::UnsupportedOperationParameter),
        ("result", Reason::UnsupportedOperationResult),
        ("sibling", Reason::UnsupportedSiblingPrecondition),
    ] {
        let mut doc = document();
        let pre = match axis {
            "state" => {
                doc["types"][1]["fields"][1]["presence"] = json!("optional");
                ""
            }
            "parameter" => {
                doc["types"][1]["operations"][0]["params"][0]["typeRef"] =
                    json!("ix://quire/native/Integer");
                ""
            }
            "result" => {
                doc["types"][1]["operations"][0]["returns"] = json!({"typeRef":"ix://quire/native/String","nullable":false,"multiplicity":{"lower":1,"upper":1,"ordered":false,"unique":true}});
                ""
            }
            "sibling" => {
                "pre Ready using v on M::Node::step { exists(item in self.z_items: item > limit) }"
            }
            _ => unreachable!(),
        };
        let package = fixture(&doc, pre);
        let post = clauses(&package).into_iter().find(|id| matches!(package.term(id), Some(CheckedTerm::Application { operation, .. }) if operation.state_clause_kind().ok().flatten() == Some(quire_contract_model::StateClauseKind::Postcondition))).unwrap();
        let outcome = negotiate(&package, &[post]);
        let KaniObligationOutcome::Emitted {
            records,
            v2_clause_harnesses,
            collection_harnesses,
            ..
        } = outcome
        else {
            panic!("typed {axis} refusal")
        };
        assert!(v2_clause_harnesses.is_empty() && collection_harnesses.is_empty());
        assert!(
            matches!(&records[0].disposition, ObligationDisposition::Unsupported { reason: UnsupportedObligation::CollectionClaimNotRendered { reason: actual, .. } } if *actual == reason),
            "{axis}: {:?}",
            records[0].disposition
        );
    }
}

#[test]
#[trace("TC-025", "FR-015-AC-96", "FR-015-AC-99", "FR-029-AC-11")]
fn whole_object_precondition_totality_and_playback_keep_all_state_and_parameters() {
    let package = fixture(
        &document(),
        "pre Ready using v on M::Node::step { forall(item in self.z_items: item >= 1) }",
    );
    let ids = clauses(&package);
    let KaniObligationOutcome::Emitted {
        v2_clause_harnesses,
        records,
        ..
    } = negotiate(&package, &ids)
    else {
        panic!("valid request")
    };
    let harness = v2_clause_harnesses
        .iter()
        .find(|harness| {
            harness.identity.kind == quire_contract_codegen::ObligationKind::Precondition
        })
        .unwrap_or_else(|| panic!("precondition context: {records:?}"));
    let context = harness.identity.collection_operation.as_ref().unwrap();
    assert_eq!(context.fields.len(), 3);
    assert_eq!(context.parameters.len(), 2);
    let executable = Executable::compile(harness);
    for draws in ["2,1,2,0,1,2,2,1", "2,0,1,0,1,2,2,1"] {
        let output = executable.run("must-not-call", draws, "");
        assert!(
            output.status.success(),
            "both true and false preconditions complete without invoking the subject: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let entries = [2_i64, 0, 1, 0, 1, 2, 2, 1]
        .iter()
        .enumerate()
        .map(|(index, value)| {
            let bytes = if index == 4 || index == 7 {
                vec![u8::try_from(*value).unwrap()]
            } else {
                value.to_le_bytes().to_vec()
            };
            format!(
                "// {value}\nvec![{}],",
                bytes
                    .iter()
                    .map(u8::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let transcript = format!("/// Test generated for harness `{}::{}`\n/// Check for `assertion`: \"\"the selected precondition completes a Boolean\"\"\nlet concrete_vals: Vec<Vec<u8>> = vec![\n{entries}\n];", harness.identity.module_symbol, harness.identity.harness_symbol);
    let limits = quire_contract_codegen::WitnessDecodeLimits {
        bytes: 4096,
        values: 8,
    };
    let playback = quire_contract_codegen::parse_assertion_playback(
        &transcript,
        harness.identity.harness_symbol.as_str(),
        harness.identity.module_symbol.as_str(),
        limits.bytes,
        limits.values,
    )
    .unwrap();
    let (witness, settlement) =
        quire_contract_codegen::settle_v2_collection_precondition_falsification(
            harness, &playback, limits,
        )
        .unwrap();
    assert_eq!(
        witness,
        vec![
            (
                "self.z_items".into(),
                qsl_replay::WitnessValue::OrderedSet(vec![
                    qsl_replay::WitnessValue::Integer(0),
                    qsl_replay::WitnessValue::Integer(1)
                ])
            ),
            (
                "self.a_unread".into(),
                qsl_replay::WitnessValue::Boolean(true)
            ),
            ("self.m_count".into(), qsl_replay::WitnessValue::Integer(2)),
            ("limit".into(), qsl_replay::WitnessValue::Integer(2)),
            ("enabled".into(), qsl_replay::WitnessValue::Boolean(true)),
        ]
    );
    assert!(matches!(
        settlement,
        quire_contract_codegen::ReplaySettlement::CgDefect
    ));
}

#[test]
#[trace("TC-025", "FR-015-AC-96", "FR-015-AC-101")]
fn unread_collection_bounds_and_full_width_integer_domains_are_not_replaced() {
    let mut doc = document();
    doc["types"][1]["fields"][2]["multiplicity"] =
        json!({"lower":0,"upper":9,"ordered":true,"unique":false});
    let package = fixture(&doc, "");
    let KaniObligationOutcome::Emitted {
        records,
        v2_clause_harnesses,
        ..
    } = negotiate(&package, &clauses(&package))
    else {
        panic!("valid request")
    };
    assert!(v2_clause_harnesses.is_empty());
    assert!(matches!(records[0].disposition, quire_contract_codegen::ObligationDisposition::Unsupported {
        reason: quire_contract_codegen::UnsupportedObligation::CollectionClaimNotRendered { reason: quire_contract_codegen::CollectionUnsupportedReason::UnwindBelowCollectionBound { required:10, supplied:8 }, .. }
    }));
    let mut doc = document();
    doc["types"][0]["constraints"][0]["operands"]["value"] = json!(i64::MIN.to_string());
    doc["types"][0]["constraints"][1]["operands"]["value"] = json!(i64::MAX.to_string());
    let package = fixture(&doc, "");
    let harness = postcondition(negotiate(&package, &clauses(&package)));
    assert!(
        harness.rust.contents.contains("i64::MIN")
            || harness.rust.contents.contains("-9223372036854775808")
    );
    assert!(harness.rust.contents.contains("9223372036854775807"));
    let context = harness.identity.collection_operation.as_ref().unwrap();
    let encoded = serde_json::to_value(context).unwrap();
    assert_eq!(
        encoded["fields"][0]["bounds"]["element"]["minimum"],
        i64::MIN
    );
    assert_eq!(
        encoded["fields"][0]["bounds"]["element"]["maximum"],
        i64::MAX
    );
    let executable = Executable::compile(&harness);
    let output = executable.run(
        "healthy",
        "1,9223372036854775807,0,0,1,2,2,1",
        "9223372036854775807",
    );
    assert!(
        output.status.success(),
        "the real maximum remains in the generated domain: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let output = executable.run(
        "healthy",
        "1,-9223372036854775808,0,0,1,2,2,1",
        "-9223372036854775808",
    );
    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr)
            .contains("the produced state satisfies the authored collection postcondition"),
        "the real minimum reaches the authored assertion; it was not assumed away"
    );
}

#[test]
#[trace("TC-025", "FR-015-AC-96", "FR-015-AC-99")]
fn retained_prestate_order_is_an_authored_obligation_separate_from_collection_domain() {
    let mut doc = document();
    doc["types"][1]["fields"][0]["multiplicity"]["lower"] = json!(2);
    let package = fixture_with_body(
        &doc,
        "",
        "forall(item in self.z_items: item >= 0) and self.z_items = pre(self.z_items)",
    );
    let harness = postcondition(negotiate(&package, &clauses(&package)));
    let executable = Executable::compile(&harness);
    let healthy = executable.run("healthy", "2,1,2,0,1,2,2,1", "1,2");
    assert!(
        healthy.status.success(),
        "same ordered members: {}",
        String::from_utf8_lossy(&healthy.stderr)
    );
    let reordered = executable.run("reorder", "2,1,2,0,1,2,2,1", "1,2");
    assert!(!reordered.status.success());
    let stderr = String::from_utf8_lossy(&reordered.stderr);
    assert!(stderr.contains("the produced state satisfies the authored collection postcondition"), "a valid reordered set passes inspection and fails only its order-preservation clause: {stderr}");
    assert!(!stderr.contains("every produced collection"));
    let below = executable.run("below-minimum", "2,1,2,0,1,2,2,1", "1,2");
    assert!(!below.status.success());
    assert!(String::from_utf8_lossy(&below.stderr).contains("every produced collection"));
}

#[test]
#[trace("TC-025", "FR-015-AC-96", "FR-015-AC-100")]
fn whole_object_identity_regenerates_and_changes_with_unread_domains() {
    let package = fixture(&document(), "");
    let ids = clauses(&package);
    let first = postcondition(negotiate(&package, &ids));
    let second = postcondition(negotiate(&package, &ids));
    assert_eq!(first, second);
    let mut doc = document();
    doc["types"][1]["fields"][1]["typeRef"] = json!(SMALL);
    let changed = fixture(&doc, "");
    let changed = postcondition(negotiate(&changed, &clauses(&changed)));
    assert_ne!(first.identity, changed.identity);
    assert_ne!(first.identity.module_symbol, changed.identity.module_symbol);
    let context = changed.identity.collection_operation.unwrap();
    assert!(matches!(
        context.fields[1].bounds,
        quire_contract_codegen::SymbolicBounds::Integer {
            minimum: 0,
            maximum: 3
        }
    ));
}

#[test]
#[trace("TC-025", "FR-015-AC-96")]
fn boolean_ordered_set_operation_draws_both_values_and_enforces_distinct_slots() {
    let mut doc = document();
    doc["types"][1]["fields"][0]["typeRef"] = json!("ix://quire/native/Boolean");
    doc["types"][1]["fields"][0]["multiplicity"]["upper"] = json!(2);
    let package = fixture_with_body(&doc, "", "exists(flag in self.z_items: flag != false)");
    let harness = postcondition(negotiate(&package, &clauses(&package)));
    let executable = Executable::compile(&harness);
    for (draws, order) in [
        ("1,1,0,1,2,2,1", "1"),
        ("2,0,1,1,2,2,1", "0,1"),
        ("2,1,0,1,2,2,1", "1,0"),
    ] {
        let output = executable.run("healthy", draws, order);
        assert!(
            output.status.success(),
            "both Boolean values and both orders remain possible: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let output = executable.run("healthy", "1,0,1,1,2,2,1", "0");
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr)
        .contains("the produced state satisfies the authored collection postcondition"));
    let output = executable.run("healthy", "2,1,1,1,2,2,1", "1,1");
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("symbolic input excluded"));
}

#[test]
#[trace("TC-025", "FR-015-AC-96", "FR-015-AC-100")]
fn inherited_fields_retain_the_actual_declaring_node_in_whole_object_identity() {
    let mut doc = document();
    let base_id = "ix://ir1091.operation/domain/Base";
    let mut base = doc["types"][1].clone();
    base["identity"] = json!(base_id);
    base["displayName"] = json!("Base");
    base["origin"] = origin(base_id);
    base["operations"] = json!([]);
    let mut inherited = doc["types"][1]["fields"][0].clone();
    inherited["identity"] = json!(format!("{base_id}/z_items"));
    base["fields"] = json!([inherited]);
    doc["types"][1]["fields"].as_array_mut().unwrap().remove(0);
    doc["types"][1]["supertypes"] = json!([base_id]);
    doc["types"][1]["operations"][0]["frame"]["modifies"][0] = json!(format!("{base_id}/z_items"));
    doc["types"].as_array_mut().unwrap().push(base);
    let package = fixture(&doc, "");
    let harness = postcondition(negotiate(&package, &clauses(&package)));
    let context = harness.identity.collection_operation.unwrap();
    assert_eq!(context.fields.len(), 3);
    let inherited = context
        .fields
        .iter()
        .find(|field| field.field_name == "z_items")
        .unwrap();
    let own = context
        .fields
        .iter()
        .find(|field| field.field_name == "a_unread")
        .unwrap();
    assert_ne!(inherited.declaring_node_id, own.declaring_node_id);
    assert_eq!(
        package
            .model_object_fields(&inherited.declaring_node_id)
            .unwrap()
            .fields()
            .iter()
            .map(|field| field.name())
            .collect::<Vec<_>>(),
        ["z_items"]
    );
}

#[test]
#[trace("TC-025", "FR-015-AC-96", "FR-015-AC-99")]
fn postcondition_nonvacuity_accepts_intermediate_lengths_selected_by_sibling_preconditions() {
    let mut doc = document();
    doc["types"][1]["fields"][0]["typeRef"] = json!("ix://quire/native/Boolean");
    doc["types"][1]["fields"][0]["multiplicity"]["upper"] = json!(2);
    let package = fixture_with_body(&doc,
        "pre OnlyTrue using v on M::Node::step { forall(flag in self.z_items: flag != false) }\npre NonEmpty using v on M::Node::step { exists(flag in self.z_items: flag != false) }",
        "exists(flag in self.z_items: flag != false)");
    let harness = postcondition(negotiate(&package, &clauses(&package)));
    assert_eq!(
        harness
            .identity
            .collection_operation
            .as_ref()
            .unwrap()
            .preconditions
            .len(),
        2
    );
    let executable = Executable::compile(&harness);
    let output = std::process::Command::new("timeout")
        .arg("30s")
        .arg(executable.scratch.path.join("operation"))
        .args(["healthy", "1,1,0,1,2,2,1", "1", "cover"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "only length1 satisfies both sibling preconditions; it must witness the real cover: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
