//! The hand-mirrored QSL twin of the state-frame fixture: the `test/bank` domain package, the
//! native unit whose `post` clause and operation frame mirror the checked package the harnesses
//! are generated from, the invocation documents of one concrete run, and the `replay_frame`
//! request and envelope that put that run before QSL.
//!
//! The twin's fields, their ranges and the fields its frame grants come from `model`, the same
//! source the Rust fixture's checked package is built from. The rest is tied to the Rust side only
//! by names: the model `Bank`, its object `Account` and its operation `deposit`. The node
//! identities a counterexample names come from the twin's own compile, because `qsl_replay` does
//! not let a caller mint an occurrence key, so they are not the fixture package's node ids. The
//! envelope's obligation and originating-counterexample identities are fixed stand-ins: CG
//! computes no identity for a frame obligation, and QSL only requires that one be present.

use super::model;

use std::collections::BTreeMap;

use qsl_replay::{
    replay_frame,
    spine::{compile, default_accounting, Compiled, DependencyInput, SpineLimits},
    ByteDigest, ClaimedChange, DigestDomain, DigestRecord, DocumentRef, FrameCounterexample,
    FrameOperation, FrameReplayResult, Identifier, OccurrenceKey, ProfileSelection, QualifiedName,
    ReplayRefusal, ReplayRequestWire, ReplaySource, SelectedObject, SourceIdentity, StageLimits,
    StateEnvironment, WireNodeId, Witness, WitnessEnvelope, WitnessPacket, MAX_ENCODED_BYTES,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const AUTHORITY: &str = "agent-ix";
const IDENTITY: &str = "test:state-frame-twin";
const PACKAGE: &str = "test/bank";
const PLACEHOLDER_DIGEST: &str =
    "sha256:0000000000000000000000000000000000000000000000000000000000000000";
const PROFILE_DIGEST: &str =
    "sha256:c8c7ae9fbe783286369ecc83f006190f83be4c3c8fc585766617c90f27a25b16";

fn account_type() -> String {
    format!("ix://{PACKAGE}/Account")
}

fn population() -> String {
    format!("ix://{PACKAGE}/accounts")
}

/// The value type of a field: one per field, each with the range `model` gives it.
fn range_type(field: &str) -> String {
    format!("ix://{PACKAGE}/{field}_range")
}

/// RFC 8785 text of `value`. The documents here hold only strings, integers and booleans, so
/// sorted members and serde's compact escapes are the canonical form.
fn canonical(value: &Value) -> String {
    match value {
        Value::Object(members) => {
            let mut ordered = members.iter().collect::<Vec<_>>();
            ordered.sort_by(|a, b| a.0.cmp(b.0));
            let members = ordered
                .iter()
                .map(|(name, member)| format!("{}:{}", json!(name), canonical(member)))
                .collect::<Vec<_>>();
            format!("{{{}}}", members.join(","))
        }
        Value::Array(items) => {
            let items = items.iter().map(canonical).collect::<Vec<_>>();
            format!("[{}]", items.join(","))
        }
        scalar => scalar.to_string(),
    }
}

fn jcs_digest(bytes: &[u8]) -> [u8; 32] {
    let value: Value = serde_json::from_slice(bytes).expect("the document is JSON");
    Sha256::digest(canonical(&value).as_bytes()).into()
}

fn hex(bytes: &[u8; 32]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn generated(identity: &str) -> Value {
    json!({"generated": {
        "generatorIdentity": identity,
        "generatorVersion": "1.0.0",
        "inputIdentities": [identity],
    }})
}

fn construct(name: &str, meaning: &str) -> Value {
    json!({
        "kind": {"module": PACKAGE, "name": name},
        "moduleVersion": "1.0.0",
        "manifestDigest": PLACEHOLDER_DIGEST,
        "construct": {
            "identity": "none", "shape": "record", "members": {}, "meaning": meaning,
        },
    })
}

fn field(owner: &str, name: &str) -> Value {
    let identity = format!("{owner}/{name}");
    json!({
        "identity": identity,
        "name": name,
        "typeRef": range_type(name),
        "presence": "required",
        "nullable": false,
        "defaultKind": "none",
        "multiplicity": {"lower": 1, "upper": 1, "ordered": false, "unique": true},
        "origin": generated(&identity),
    })
}

/// The `test/bank` domain package: `Account` with the integer fields and ranges of `model`, one
/// operation `deposit` whose frame modifies the fields `model` grants, and one closed population.
fn domain_document() -> Vec<u8> {
    let account = account_type();
    let bound = |field: &str, keyword: &str, value: i64| {
        let range = range_type(field);
        json!({
            "identity": format!("{range}/constraints/{keyword}"),
            "keyword": keyword,
            "operands": {"value": value},
            "appliesTo": "ix://quire/native/Integer",
            "diagnosticCode": format!("bound.{keyword}"),
            "origin": generated(&range),
        })
    };
    let value_type = |(field, (minimum, maximum)): (&str, (i64, i64))| {
        let range = range_type(field);
        json!({
            "identity": range,
            "displayName": range,
            "kind": {"module": PACKAGE, "name": "value_type"},
            "roles": [],
            "origin": generated(&range),
            "extensions": [],
            "unknownPolicy": "reject",
            "scalar": "integer",
            "constraints": [bound(field, "min", minimum), bound(field, "max", maximum)],
        })
    };
    let mut types = model::FIELDS.map(value_type).to_vec();
    types.push(json!({
        "identity": account,
        "displayName": account,
        "kind": {"module": PACKAGE, "name": "object_type"},
        "roles": [],
        "origin": generated(&account),
        "constraints": [],
        "extensions": [],
        "unknownPolicy": "reject",
        "supertypes": [],
        "fields": model::FIELDS.map(|(name, _)| field(&account, name)),
        "operations": [{
            "identity": format!("{account}/deposit"),
            "name": "deposit",
            "params": [],
            "returns": {
                "typeRef": "ix://quire/native/Boolean",
                "multiplicity": {"lower": 1, "upper": 1, "ordered": false, "unique": true},
                "nullable": false,
            },
            "pre": [],
            "post": [],
            "origin": {"source": {
                "sourceIdentity": format!("ix://{PACKAGE}/spec"),
                "path": "spec.qspec",
                "startLine": 1,
                "startColumn": 1,
            }},
            "frame": {
                "modifies": model::GRANTED.map(|name| format!("{account}/{name}")),
                "creates": [],
                "deletes": [],
            },
        }],
    }));
    json!({
        "contractVersion": "2.0.0",
        "source": {
            "identity": format!("ix://{PACKAGE}/spec"),
            "version": "1.0.0",
            "dialect": "spec-bundle",
            "digest": PLACEHOLDER_DIGEST,
        },
        "package": {
            "identity": PACKAGE,
            "version": "1.0.0",
            "manifestDigest": PLACEHOLDER_DIGEST,
            "mappingVersions": [],
            "profileVersions": [],
            "lockDigest": PLACEHOLDER_DIGEST,
        },
        "occurrences": [],
        "extensions": [],
        "constructs": [
            construct("object_type", "quire.meaning.model.object-type/v1"),
            construct("population", "quire.meaning.model.population/v1"),
            construct("value_type", "quire.meaning.model.value-type/v1"),
        ],
        "types": types,
        "populations": [{
            "identity": population(),
            "displayName": population(),
            "kind": {"module": PACKAGE, "name": "population"},
            "members": [account],
            "extent": "closed",
            "origin": generated(&population()),
        }],
    })
    .to_string()
    .into_bytes()
}

fn unit_source(model_digest: &str) -> String {
    format!(
        "language \"ix:native\" edition \"1-draft\";\n\
         profile v = \"quire.value.complete/v1\" version \"1-draft.2\" digest \"{PROFILE_DIGEST}\";\n\
         model Bank = {PACKAGE:?} version \"1.0.0\" digest \"sha256-jcs:{model_digest}\";\n\
         post BalanceNeverDrops using v on Bank::Account::deposit {{ \
         self.balance >= pre(self.balance) }}\n"
    )
}

/// Which envelope identity a replay makes differ from the payload's.
#[derive(Clone, Copy)]
pub enum Tamper {
    /// Neither: the envelope agrees with its payload.
    Nothing,
    /// The envelope's `clause_node`.
    ClauseNode,
    /// The envelope's `occurrence_key`.
    Occurrence,
}

/// The compiled twin.
pub struct Twin {
    unit: Vec<u8>,
    domain: Vec<u8>,
    compiled: Compiled,
}

/// One invocation of `deposit` on account `account`, with its pre and post snapshots.
pub struct Invocation {
    reference: DocumentRef,
    documents: Vec<(DocumentRef, Vec<u8>)>,
}

impl Twin {
    /// Compiles the twin.
    pub fn compile() -> Self {
        let domain = domain_document();
        let digest = jcs_digest(&domain);
        let unit = unit_source(&hex(&digest)).into_bytes();
        let packages = BTreeMap::from([(digest, domain.clone())]);
        let compiled = compile(
            SourceIdentity::new(AUTHORITY, IDENTITY, "git", "1"),
            IDENTITY,
            &unit,
            &packages,
            &DependencyInput::default(),
            SpineLimits::default(),
        )
        .unwrap_or_else(|refusal| panic!("the twin compiles: {refusal:?}"));
        Self {
            unit,
            domain,
            compiled,
        }
    }

    /// The invocation of `deposit` on `account` from `pre` to `post`, each `(balance, audit)`.
    pub fn invocation(&self, account: &str, pre: (i64, i64), post: (i64, i64)) -> Invocation {
        let model = format!("sha256-jcs:{}", hex(&jcs_digest(&self.domain)));
        let model_header = json!({"identity": PACKAGE, "version": "1.0.0", "digest": model});
        let snapshot = |name: &str, observation: &str, (balance, audit): (i64, i64)| {
            let label = label(name);
            let bytes = json!({
                "format": "quire.state.snapshot/v1",
                "identity": identity_json(&label),
                "observation": observation,
                "model": model_header,
                "populations": [{
                    "population": population(), "complete": true,
                    "objects": [{
                        "key": account, "type": account_type(),
                        "fields": {
                            "balance": {"integer": balance.to_string()},
                            "audit": {"integer": audit.to_string()},
                        },
                    }],
                }],
            })
            .to_string()
            .into_bytes();
            stamped(label, bytes)
        };
        let (pre_ref, pre_bytes) = snapshot("twin-pre", "pre", pre);
        let (post_ref, post_bytes) = snapshot("twin-post", "post", post);
        let invocation_label = label("twin-invocation");
        let invocation_bytes = json!({
            "format": "quire.state.invocation/v1",
            "identity": identity_json(&invocation_label),
            "model": model_header,
            "context": account_type(),
            "operation": "deposit",
            "self": {"population": population(), "key": account},
            "pre": {
                "identity": identity_json(&pre_ref),
                "digest": format!("sha256-jcs:{}", hex(&pre_ref.digest)),
            },
            "post": {
                "identity": identity_json(&post_ref),
                "digest": format!("sha256-jcs:{}", hex(&post_ref.digest)),
            },
            "parameters": {},
            "result": {"boolean": true},
            "created": [],
            "deleted": [],
        })
        .to_string()
        .into_bytes();
        let (reference, bytes) = stamped(invocation_label, invocation_bytes);
        Invocation {
            documents: vec![
                (reference.clone(), bytes),
                (pre_ref, pre_bytes),
                (post_ref, post_bytes),
            ],
            reference,
        }
    }

    /// Replays `invocation` as a counterexample to the frame of `deposit`, claiming `field` of
    /// `account` was written.
    pub fn replay(
        &self,
        invocation: &Invocation,
        account: &str,
        field: &str,
    ) -> Result<FrameReplayResult, ReplayRefusal> {
        self.replay_tampered(invocation, account, field, Tamper::Nothing)
    }

    /// [`Self::replay`] with one envelope identity made to differ from the payload's.
    pub fn replay_tampered(
        &self,
        invocation: &Invocation,
        account: &str,
        field: &str,
        tamper: Tamper,
    ) -> Result<FrameReplayResult, ReplayRefusal> {
        let package_id = DigestRecord::mint(
            DigestDomain::PackageSemanticV2,
            *self.compiled.emitted.package_id().as_bytes(),
        );
        let graph = self.compiled.package.graph();
        let identifier = |name| Identifier::new(name).expect("identifier");
        let selection = graph
            .resolve_operation(
                &identifier("Bank"),
                &identifier("Account"),
                &identifier("deposit"),
            )
            .expect("Bank::Account resolves against the twin's model");
        let (_, frame) = graph
            .operation_frame(&selection)
            .expect("deposit is named by BalanceNeverDrops");
        let payload = FrameCounterexample {
            operation: FrameOperation {
                object: QualifiedName::new(vec![
                    Identifier::new("Bank").expect("identifier"),
                    Identifier::new("Account").expect("identifier"),
                ])
                .expect("a qualified name"),
                operation: Identifier::new("deposit").expect("identifier"),
            },
            anchor: WireNodeId::from_digest(*frame.anchor().as_bytes()),
            frame: WireNodeId::from_digest(*frame.frame().as_bytes()),
            occurrence: OccurrenceKey::new(
                WireNodeId::from_digest(*frame.frame().as_bytes()),
                frame.frame_origin().clone(),
            ),
            invocation: invocation.reference.clone(),
            change: ClaimedChange::FieldWrite {
                object: SelectedObject {
                    population: population(),
                    key: account.to_owned(),
                },
                field: field.to_owned(),
            },
        };
        let envelope = self.envelope(package_id, payload, tamper);
        replay_frame(self.request(package_id, invocation), &envelope)
    }

    fn source_reference(&self) -> (String, String, String, String, Option<String>, String) {
        let digest = DigestRecord::mint(
            DigestDomain::SourceBytesV1,
            ByteDigest::of(&self.unit).as_bytes(),
        );
        (
            AUTHORITY.to_owned(),
            IDENTITY.to_owned(),
            "git".to_owned(),
            "1".to_owned(),
            Some(DigestDomain::SourceBytesV1.as_str().to_owned()),
            digest.hex(),
        )
    }

    fn envelope(
        &self,
        package_id: DigestRecord,
        payload: FrameCounterexample,
        tamper: Tamper,
    ) -> WitnessEnvelope<FrameCounterexample> {
        // The anchor node stands in for any node other than the frame.
        let other = payload.anchor;
        let occurrence = match tamper {
            Tamper::Occurrence => OccurrenceKey::new(other, payload.occurrence.origin().clone()),
            Tamper::Nothing | Tamper::ClauseNode => payload.occurrence.clone(),
        };
        let clause_node = match tamper {
            Tamper::ClauseNode => other,
            Tamper::Nothing | Tamper::Occurrence => payload.frame,
        };
        let backend = (
            "kani-backend-1".to_owned(),
            Some(DigestDomain::ToolManifestJcsV1.as_str().to_owned()),
            DigestRecord::mint(DigestDomain::ToolManifestJcsV1, [3; 32]).hex(),
        );
        WitnessEnvelope::reconstruct(WitnessPacket {
            obligation_identity: Some([1; 32]),
            occurrence_key: Some(occurrence),
            clause_node: Some(clause_node),
            selected_function: Some(
                QualifiedName::new(vec![Identifier::new("deposit").expect("identifier")])
                    .expect("a qualified name"),
            ),
            package_id: Some((
                Some(DigestDomain::PackageSemanticV2.as_str().to_owned()),
                package_id.hex(),
            )),
            package_contract_version: Some("quire.checked-package/v2".to_owned()),
            source_digests: Some(vec![self.source_reference()]),
            profile_selections: Some(vec![ProfileSelection::new(
                "quire.profile.v1".to_owned(),
                "finite-state".to_owned(),
            )]),
            run_limits: Some(default_accounting(1_000_000)),
            declared_domains: Some(Vec::new()),
            backend: Some(backend),
            trace_position: Some(None),
            source: Some(witness_source()),
            family_payload: Some(payload),
        })
        .expect("a complete packet reconstructs")
    }

    fn request(&self, package_id: DigestRecord, invocation: &Invocation) -> ReplayRequestWire {
        let jcs = |digest: [u8; 32]| {
            (
                Some(DigestDomain::Sha256Jcs.as_str().to_owned()),
                DigestRecord::mint(DigestDomain::Sha256Jcs, digest).hex(),
            )
        };
        let (source_domain, source_hex) = {
            let reference = self.source_reference();
            (reference.4, reference.5)
        };
        let mut byte_provision = vec![(source_domain, source_hex, self.unit.clone()), {
            let (domain, hex) = jcs(jcs_digest(&self.domain));
            (domain, hex, self.domain.clone())
        }];
        for (reference, bytes) in &invocation.documents {
            let (domain, hex) = jcs(reference.digest);
            byte_provision.push((domain, hex, bytes.clone()));
        }
        let unlimited = default_accounting(u64::MAX);
        ReplayRequestWire {
            contract_version: "quire.native-runtime/v1".to_owned(),
            capability_vocabulary: Some("quire.capability-kind/v1".to_owned()),
            profile_selections: Vec::new(),
            package_id: (
                Some(DigestDomain::PackageSemanticV2.as_str().to_owned()),
                package_id.hex(),
            ),
            package_contract_version: "quire.checked-package/v2".to_owned(),
            source_digests: vec![self.source_reference()],
            dependencies: Vec::new(),
            selected_function: QualifiedName::new(vec![
                Identifier::new("deposit").expect("identifier")
            ])
            .expect("a qualified name"),
            source: witness_source(),
            originating_counterexample_identity: [2; 32],
            backend: (
                "kani-backend-1".to_owned(),
                Some(DigestDomain::ToolManifestJcsV1.as_str().to_owned()),
                DigestRecord::mint(DigestDomain::ToolManifestJcsV1, [3; 32]).hex(),
            ),
            state_environment: StateEnvironment::new(Vec::new()),
            accounting_limits: default_accounting(1_000_000),
            stage_limits: StageLimits {
                s1: qsl_replay::ScalarLimits {
                    text_input_bytes: u64::try_from(MAX_ENCODED_BYTES).expect("a small bound"),
                    ..unlimited
                },
                s2: unlimited,
                s3: unlimited,
                s4: unlimited,
            },
            byte_provision,
        }
    }
}

fn witness_source() -> ReplaySource {
    ReplaySource::Witness(
        Witness::parse("<<<assertion|frame_harness|frame|>>>").expect("a witness"),
    )
}

fn label(name: &str) -> DocumentRef {
    DocumentRef {
        authority: "test".to_owned(),
        identity: name.to_owned(),
        revision_namespace: "ns".to_owned(),
        revision: "1".to_owned(),
        digest: [0; 32],
    }
}

fn identity_json(reference: &DocumentRef) -> Value {
    json!({
        "authority": reference.authority,
        "identity": reference.identity,
        "revision_namespace": reference.revision_namespace,
        "revision": reference.revision,
    })
}

fn stamped(label: DocumentRef, bytes: Vec<u8>) -> (DocumentRef, Vec<u8>) {
    let digest = jcs_digest(&bytes);
    (DocumentRef { digest, ..label }, bytes)
}
