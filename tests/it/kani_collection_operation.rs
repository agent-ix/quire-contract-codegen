//! Whole-object collection operation contexts from authentic QSL emission and IR admission.

use ix_trace_rs::trace;
use qsl_replay::{call_site, ClauseName, DependencyInput, Identifier, SourceIdentity};
use quire_contract_codegen::{
    negotiate_kani_obligations, CollectionKindIdentity, KaniObligationOutcome,
    KaniObligationRequest, KaniV2ClauseObligationHarness, ObligationItem,
};
use quire_contract_model::{
    CheckedNodeId, CheckedPackageEvidence, CheckedPackageReadLimits, CheckedPackageV2,
    CheckedPackageV2ReadResult, CheckedTerm, StateClauseKind,
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
        "source":{"identity":"ix://ir1091.operation/spec","version":"1.0.0","title":"whole object collection operation","language":"en"},
        "module":{"identity":MODEL,"version":"1.0.0","requires":[]},
        "kindModules":[{"identity":MODEL,"version":"1.0.0","kinds":[
            {"kind":{"module":MODEL,"name":"entity"},"moduleVersion":"1.0.0","construct":{"identity":"none","shape":"record","members":{},"meaning":"quire.meaning.model.object-type/v1"}},
            {"kind":{"module":MODEL,"name":"value_type"},"moduleVersion":"1.0.0","construct":{"identity":"none","shape":"record","members":{},"meaning":"quire.meaning.model.value-type/v1"}}
        ]}],
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
    let source = format!("language \"ix:native\" edition \"1-draft\";\nprofile v = \"quire.model.complete/v1\";\nmodel M = \"{MODEL}\";\n{preconditions}\npostcondition CollectionReady using v on M::Node::step {{ forall(item in self.z_items: item >= 0) }}\n");
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
