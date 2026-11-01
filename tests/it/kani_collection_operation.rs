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
    let bytes = quire_canonical::to_vec(doc, quire_canonical::Limits::new(1 << 20)).unwrap();
    let source = format!("language \"ix:native\" edition \"1-draft\";\nprofile v = \"quire.model.complete/v1\";\nmodel M = \"{MODEL}\";\n{preconditions}\npost CollectionReady using v on M::Node::step {{ forall(item in self.z_items: item >= 0) }}\n");
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
pub trait Draw { fn draw() -> Self; }
impl Draw for i64 { fn draw() -> Self { DRAWS.lock().unwrap().pop_front().expect("all symbolic draws supplied") } }
impl Draw for bool { fn draw() -> Self { match <i64 as Draw>::draw() { 0 => false, 1 => true, _ => panic!("Boolean draw") } } }
impl<T: Draw, const N: usize> Draw for [T; N] { fn draw() -> Self { std::array::from_fn(|_| T::draw()) } }
pub fn any<T: Draw>() -> T { T::draw() }
pub fn assume(condition: bool) { assert!(condition, "symbolic input excluded"); }
pub fn seed(values: Vec<i64>) { *DRAWS.lock().unwrap() = values.into(); }
pub fn cover(_condition: bool) { COVERS.fetch_add(1, Ordering::SeqCst); }
pub fn finish() { assert!(DRAWS.lock().unwrap().is_empty(), "every declared state and parameter draw is consumed"); assert_eq!(COVERS.load(Ordering::SeqCst), 1); }
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
        .map(|part| quire_exact::Value::Integer(quire_exact::Integer::from(part.parse::<i64>().unwrap()))).collect::<Vec<_>>();
    assert_eq!(collection.elements().len(), expected.len());
    for (actual, expected) in collection.elements().iter().zip(&expected) {
        let (quire_exact::Value::Integer(actual), quire_exact::Value::Integer(expected)) = (actual, expected) else { panic!("integer elements"); };
        assert_eq!(actual, expected, "the symbolic input order is retained");
    }
    let action = std::env::args().nth(1).unwrap();
    match action.as_str() {
        "field-type" => *unread = quire_exact::Value::Integer(quire_exact::Integer::from(0_i64)),
        "field-domain" => *count = quire_exact::Value::Integer(quire_exact::Integer::from(99_i64)),
        "duplicate" | "element-domain" | "cardinality" | "collection-kind" => {
            let mut ty = collection.collection_type().clone();
            let values = match action.as_str() {
                "duplicate" => vec![1,1], "element-domain" => vec![99], "cardinality" => vec![0,1,2,3], _ => vec![1,2],
            }.into_iter().map(|value: i64| quire_exact::Value::Integer(quire_exact::Integer::from(value))).collect();
            if action == "collection-kind" { ty = quire_exact::CollectionType::new(quire_exact::CollectionKind::Sequence, ty.element().clone(), ty.bound()); }
            *items = quire_exact::from_admitted(ty, values);
        }
        "healthy" => {},
        _ => panic!("unknown subject action"),
    }
}
"#;
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
