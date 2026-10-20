//! Authored QSL emission and strict IR admission shared by graph controls.
use qsl_replay::{call_site, DependencyInput, Identifier, SourceIdentity};
use quire_contract_model::{CheckedPackageEvidence, CheckedPackageReadLimits, CheckedPackageV2, CheckedPackageV2ReadResult};
use serde_json::{json, Value};
pub(crate) struct AuthoredGraph {
    pub(crate) package: CheckedPackageV2,
    pub(crate) emitted_bytes: Vec<u8>,
    pub(crate) source: String,
    pub(crate) model_bytes: Vec<u8>,
    pub(crate) model_digest: [u8; 32],
    pub(crate) site: qsl_replay::ClauseSite,
    pub(crate) package_id: qsl_replay::DigestRecord,
}
pub(crate) struct ModelInput {
    pub(crate) document: Value,
    pub(crate) incoming_upper: u64,
}
pub(crate) fn try_fixture_source(
    model: ModelInput,
    edge: &str,
    condition: &str,
    maximum: u64,
    bounded: bool,
    form: &str,
    extra: &str,
) -> Result<AuthoredGraph, qsl_replay::CallSiteRefusal> {
    let mut document = model.document;
    document["populations"][0]["displayName"] = json!("CollectionNodes");
    let relation = |name: &str, lower: u64, upper: u64, unique: bool| {
        json!({
            "identity": format!("ix://ir268.test/domain/relationship/Node-{name}"),
            "category": "structural", "composite": false, "direction": "source-to-target",
            "origin": {"source": {"sourceIdentity": "ix://ir268.test/spec", "path": "spec.qspec", "startLine": 1, "startColumn": 1}},
            "sourceEnd": {"type": "ix://ir268.test/domain/Node", "role": name, "multiplicity": {"lower":0,"upper":model.incoming_upper,"ordered":false,"unique":unique}},
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
