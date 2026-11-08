// SPDX-License-Identifier: AGPL-3.0-or-later
//! Authentic QSL emission, public IR admission, then the constrained family.

use qsl_replay::{
    call_site, ClauseName, DependencyInput, Identifier, SnapshotValue, SourceIdentity,
};
use quire_contract_codegen::*;
use quire_contract_model::{
    CheckedNodeId, CheckedPackageEvidence, CheckedPackageReadLimits, CheckedPackageV2,
    CheckedPackageV2ReadResult, CheckedTerm, StateClauseKind,
};
use serde_json::{json, Value};
use std::{collections::BTreeMap, num::NonZeroU64, time::Duration};

pub(super) const MODEL: &str = "cg/constrained";
pub(super) const OBJECT: &str = "ix://cg/constrained/Node";
pub(super) const SMALL: &str = "ix://cg/constrained/Small";
pub(super) fn origin() -> Value {
    json!({"generated":{"generatorIdentity":"ix://cg/constrained/spec","generatorVersion":"1.0.0","inputIdentities":["ix://cg/constrained/spec"]}})
}
pub(super) fn document() -> Value {
    let slot = |owner: &str, name: &str, ty: &str| json!({"identity":format!("{owner}/{name}"),"name":name,"typeRef":ty,"presence":"required","nullable":false,"defaultKind":"none","origin":origin(),"multiplicity":{"lower":1,"upper":1,"ordered":false,"unique":true}});
    let op = |name: &str, parameter: &str, ty: &str, result: bool| {
        let owner = format!("{OBJECT}/{name}");
        let mut value = json!({"identity":owner,"name":name,"params":[slot(&owner,parameter,ty)],"pre":[],"post":[],"origin":origin(),"frame":{"modifies":[format!("{OBJECT}/n"),format!("{OBJECT}/flag"),format!("{OBJECT}/aux")],"creates":[],"deletes":[]}});
        if result {
            value["returns"] = json!({"typeRef":"ix://quire/native/Boolean","nullable":false,"multiplicity":{"lower":1,"upper":1,"ordered":false,"unique":true}});
        }
        value
    };
    json!({"contractVersion":"2.0.0","canonicalization":{"algorithm":"rfc8785-v1"},"source":{"identity":"ix://cg/constrained/spec","version":"1.0.0","dialect":"spec-bundle"},"package":{"identity":MODEL,"version":"1.0.0","mappingVersions":[],"profileVersions":[]},"constructs":[
        {"kind":{"module":MODEL,"name":"entity"},"moduleVersion":"1.0.0","construct":{"identity":"none","shape":"record","members":{},"meaning":"quire.meaning.model.object-type/v1"}},
        {"kind":{"module":MODEL,"name":"population"},"moduleVersion":"1.0.0","construct":{"identity":"none","shape":"record","members":{},"meaning":"quire.meaning.model.population/v1"}},
        {"kind":{"module":MODEL,"name":"value_type"},"moduleVersion":"1.0.0","construct":{"identity":"none","shape":"record","members":{},"meaning":"quire.meaning.model.value-type/v1"}}
    ],"occurrences":[],"extensions":[],"types":[
        {"identity":SMALL,"displayName":"Small","kind":{"module":MODEL,"name":"value_type"},"roles":[],"origin":origin(),"extensions":[],"unknownPolicy":"reject","scalar":"integer","constraints":[
            {"identity":format!("{SMALL}/min"),"keyword":"min","operands":{"value":"0"},"appliesTo":"ix://quire/native/Integer","diagnosticCode":"small.minimum","origin":origin()},
            {"identity":format!("{SMALL}/max"),"keyword":"max","operands":{"value":"3"},"appliesTo":"ix://quire/native/Integer","diagnosticCode":"small.maximum","origin":origin()}]},
        {"identity":OBJECT,"displayName":"Node","kind":{"module":MODEL,"name":"entity"},"roles":[],"origin":origin(),"constraints":[],"extensions":[],"unknownPolicy":"reject","supertypes":[],"fields":[slot(OBJECT,"n",SMALL),slot(OBJECT,"flag","ix://quire/native/Boolean"),slot(OBJECT,"aux",SMALL)],"operations":[op("nudge","delta",SMALL,false),op("idle","enabled","ix://quire/native/Boolean",true)]}
    ],"populations":[{"identity":format!("ix://{MODEL}/nodes"),"displayName":"nodes","kind":{"module":MODEL,"name":"population"},"members":[OBJECT],"extent":"closed","origin":origin()}]})
}

pub(super) struct Fixture {
    pub(super) source: String,
    pub(super) document: Vec<u8>,
    pub(super) name: String,
    pub(super) package: CheckedPackageV2,
    pub(super) context: CheckedNodeId,
    pub(super) safety: CheckedNodeId,
    pub(super) constraints: Vec<CheckedNodeId>,
}
pub(super) fn fixture(doc: &Value, name: &str, safety: &str, constraints: &[&str]) -> Fixture {
    let bytes = quire_canonical::to_vec(doc, quire_canonical::Limits::new(1 << 20)).unwrap();
    let mut source=format!("language \"ix:native\" edition \"1-draft\";\nprofile v = \"quire.model.complete/v1\";\nmodel M = \"{MODEL}\";\ninvariant Safe using v on M::Node at current {{ {safety} }}\n");
    for (index, body) in constraints.iter().enumerate() {
        source.push_str(&format!(
            "constraint Limit{index} using v on M::Node at current {{ {body} }}\n"
        ));
    }
    fixture_source(bytes, name, source)
}
pub(super) fn fixture_source(bytes: Vec<u8>, name: &str, source: String) -> Fixture {
    let site = call_site(
        SourceIdentity::new("agent-ix", format!("{name}.native")),
        name.to_owned(),
        "constrained.native",
        source.as_bytes(),
        [bytes.as_slice()],
        &DependencyInput::default(),
        &ClauseName(Identifier::new("Safe").unwrap()),
    )
    .expect("authentic QSL clause emission");
    let mut evidence = CheckedPackageEvidence::new();
    evidence.support_feature("quire.value.complete/v1");
    evidence
        .insert_domain_package_document(MODEL, bytes.clone())
        .unwrap();
    let package = match CheckedPackageV2::read(
        &site.package,
        CheckedPackageReadLimits::bounded(),
        &evidence,
    ) {
        CheckedPackageV2ReadResult::Admitted(package) => *package,
        other => panic!("public IR admission: {other:?}"),
    };
    let context = package
        .resolved_graph()
        .nodes
        .iter()
        .find(|node| node.semantic_form.as_ref() == "object_type")
        .unwrap()
        .node_id
        .clone();
    let clauses = |kind| {
        package.resolved_graph().nodes.iter().filter(|node|matches!(package.term(&node.node_id),Some(CheckedTerm::Application{operation,..}) if operation.state_clause_kind().ok().flatten()==Some(kind))).map(|node|node.node_id.clone()).collect::<Vec<_>>()
    };
    let safety = clauses(StateClauseKind::Invariant)
        .into_iter()
        .next()
        .unwrap();
    let constraints = clauses(StateClauseKind::Constraint);
    Fixture {
        source,
        document: bytes,
        name: name.to_owned(),
        package,
        context,
        safety,
        constraints,
    }
}
pub(super) fn initial(n: i64) -> BTreeMap<String, SnapshotValue> {
    BTreeMap::from([
        ("n".into(), SnapshotValue::integer(n.to_string())),
        ("flag".into(), SnapshotValue::boolean(false)),
        ("aux".into(), SnapshotValue::integer("0")),
    ])
}
pub(super) fn bindings(f: &Fixture) -> Vec<ConstrainedOperationBinding<'_>> {
    vec![
        ConstrainedOperationBinding {
            declaring_node: &f.context,
            name: "idle",
            subject_path: "crate::subject::idle",
        },
        ConstrainedOperationBinding {
            declaring_node: &f.context,
            name: "nudge",
            subject_path: "crate::subject::nudge",
        },
    ]
}
pub(super) fn request<'a>(
    f: &'a Fixture,
    initial: &'a BTreeMap<String, SnapshotValue>,
    bindings: &'a [ConstrainedOperationBinding<'a>],
) -> ConstrainedMethodRequest<'a> {
    ConstrainedMethodRequest {
        safety: ConstrainedClauseSelection {
            package: &f.package,
            clause: &f.safety,
        },
        constraint: Some(ConstrainedClauseSelection {
            package: &f.package,
            clause: &f.constraints[0],
        }),
        context: &f.context,
        initial,
        operations: bindings,
        maximum_steps: 3,
        evaluator_limits: quire_exact::ScalarLimits {
            integer_bits: u64::MAX,
            decimal_digits: u64::MAX,
            scale_expansion: u64::MAX,
            text_input_bytes: u64::MAX,
            text_scalars: u64::MAX,
            normalized_scalars: u64::MAX,
            unit_edges: u64::MAX,
            value_occurrences: u64::MAX,
            work_units: u64::MAX,
            result_units: u64::MAX,
        },
        census_work: 100_000,
    }
}

pub(super) fn proof_ceilings() -> ProofCeilings {
    ProofCeilings {
        memory_bytes: NonZeroU64::new(4 * 1024 * 1024 * 1024).unwrap(),
        wall_clock: Duration::from_secs(120),
    }
}
pub(super) fn emit_method(
    request: &ConstrainedMethodRequest<'_>,
) -> Result<KaniConstrainedMethodHarness, ConstrainedMethodRefusal> {
    emit_method_with_unwind(request, 8)
}
pub(super) fn emit_method_with_unwind(
    request: &ConstrainedMethodRequest<'_>,
    unwind: u32,
) -> Result<KaniConstrainedMethodHarness, ConstrainedMethodRefusal> {
    let item = ObligationItem::ConstrainedMethod { request };
    let outcome = negotiate_kani_obligations(&KaniObligationRequest {
        ceilings: proof_ceilings(),
        unwind,
        items: &[item],
        subject_path: "crate::subject",
    })
    .unwrap();
    match outcome {
        KaniObligationOutcome::Emitted {
            mut constrained_harnesses,
            records,
            ..
        } => {
            if let Some(harness) = constrained_harnesses.pop() {
                assert!(constrained_harnesses.is_empty());
                Ok(harness)
            } else {
                match records[0].disposition.clone() {
                    ObligationDisposition::Unsupported {
                        reason: UnsupportedObligation::ConstrainedMethod { reason },
                    } => Err(reason),
                    other => panic!("exact method refusal: {other:?}"),
                }
            }
        }
        KaniObligationOutcome::Rejected { records } => match records[0].disposition.clone() {
            ObligationDisposition::InvalidRequest {
                reason: InvalidObligationItem::ConstrainedMethod { reason },
            } => Err(reason),
            other => panic!("exact method selection refusal: {other:?}"),
        },
    }
}

pub(super) fn owner_evaluation(
    f: &Fixture,
    clause: &str,
    n: i64,
    flag: bool,
    aux: i64,
    accounting: quire_exact::ScalarLimits,
) -> qsl_replay::ClauseEvaluation {
    let population = format!("ix://{MODEL}/nodes");
    let fields = BTreeMap::from([
        (
            "n",
            SnapshotValue::integer(n.to_string()).to_json().unwrap(),
        ),
        ("flag", SnapshotValue::boolean(flag).to_json().unwrap()),
        (
            "aux",
            SnapshotValue::integer(aux.to_string()).to_json().unwrap(),
        ),
    ]);
    let snapshot = json!({
        "format":"quire.state.snapshot/v1",
        "identity":{"authority":"agent-ix","identity":"constrained-current","revision_namespace":"git","revision":"fixture"},
        "observation":"current","model":{"identity":MODEL},
        "anchor":{"kind":"handler","name":"control"},
        "populations":[{"population":population,"complete":true,"objects":[{"key":"self","type":OBJECT,"fields":fields}]}]
    });
    qsl_replay::evaluate_clause(qsl_replay::ClauseEvaluationRequest {
        source: SourceIdentity::new("agent-ix", format!("{}.native", f.name)),
        package_name: f.name.clone(),
        path: "constrained.native".into(),
        bytes: f.source.as_bytes().to_vec(),
        packages: vec![f.document.clone()],
        dependencies: DependencyInput::default(),
        clause: clause.into(),
        observation: qsl_replay::ClauseObservation::Current {
            snapshot: serde_json::to_vec(&snapshot).unwrap(),
            anchor: qsl_replay::SelectedAnchor {
                kind: qsl_replay::AnchorKind::Handler,
                name: "control".into(),
            },
            self_object: Some(qsl_replay::SelectedObject {
                population,
                key: "self".into(),
            }),
        },
        limits: qsl_replay::SpineLimits::default(),
        observation_limits: qsl_replay::ObservationLimits::default(),
        accounting,
    })
}
