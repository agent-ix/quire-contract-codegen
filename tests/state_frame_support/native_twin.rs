//! The hand-mirrored QSL twin of the state-frame fixture: the `test/bank` domain package, the
//! native unit whose `post` clause and operation frame mirror the checked package the harnesses
//! are generated from, and the invocation documents of one concrete run.
//!
//! The twin's fields, their ranges and the fields its frame grants come from `model`, the same
//! source the Rust fixture's checked package is built from. The rest is tied to the Rust side only
//! by names: the model `Bank`, its object `Account` and its operation `deposit`. The request and
//! envelope that put a run before QSL are built by the crate under test
//! (`quire_contract_codegen::FrameReplay`), which asks QSL for the node identities a
//! counterexample names; they are not the fixture package's node ids. The envelope's obligation
//! identity and witness come from the frame obligation CG generated and the playback decoded
//! against it, passed in as a [`Counterexample`].
//!
//! Two things here are stand-ins, both blocked on QSL-345. The twin's unit and domain package are
//! hand-mirrored from the Rust fixture's checked package, because QSL's `call_site` does not
//! return the checked package it lowered, so nothing ties the two by identity. The envelope's
//! declared domains are empty, because QSL's facade exports no type to build one from. Nothing
//! here checks either. The originating-counterexample identity and the backend manifest digest
//! are inputs of the proving run that CG does not compute.

use super::model;

use qsl_replay::{
    ClaimedChange, DigestDomain, DigestRecord, DocumentRef, FrameReplayResult, Identifier,
    OccurrenceKey, OperationName, ReplayRefusal, ScalarLimits, SelectedObject, StageLimits,
    WitnessEnvelope, MAX_ENCODED_BYTES,
};
use quire_contract_codegen::{
    DependencyLock, FrameReplay, FrameReplayError, FrameReplayInputs, FrameWitness, LockedSource,
    ProvidedDocument, ReplayInputs, StateFrameIdentity,
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
/// operation `deposit` whose frame modifies the fields `model` grants, a declared operation
/// `transfer` no clause names, and one closed population.
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
    // `transfer` is declared by the domain package, but no clause of the unit names it.
    let operation = |name: &str| {
        json!({
            "identity": format!("{account}/{name}"),
            "name": name,
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
        "operations": [operation("deposit"), operation("transfer")],
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

/// The frame obligation a counterexample falsified and the playback decoded against it.
pub struct Counterexample {
    /// The obligation.
    pub obligation: StateFrameIdentity,
    /// Its decoded playback.
    pub witness: FrameWitness,
}

/// The twin: the domain package and the native unit selecting it.
pub struct Twin {
    unit: Vec<u8>,
    domain: Vec<u8>,
}

/// One invocation of `deposit` on account `account`, with its pre and post snapshots.
pub struct Invocation {
    reference: DocumentRef,
    documents: Vec<(DocumentRef, Vec<u8>)>,
}

fn limits(seed: u64) -> ScalarLimits {
    ScalarLimits {
        integer_bits: seed,
        decimal_digits: seed,
        scale_expansion: seed,
        text_input_bytes: seed,
        text_scalars: seed,
        normalized_scalars: seed,
        unit_edges: seed,
        value_occurrences: seed,
        work_units: seed,
        result_units: seed,
    }
}

fn document(reference: &DocumentRef, bytes: &[u8]) -> ProvidedDocument {
    ProvidedDocument {
        digest: DigestRecord::mint(DigestDomain::Sha256Jcs, reference.digest),
        bytes: bytes.to_vec(),
    }
}

impl Twin {
    /// Builds the twin.
    pub fn new() -> Self {
        let domain = domain_document();
        let unit = unit_source(&hex(&jcs_digest(&domain))).into_bytes();
        Self { unit, domain }
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

    /// The frame-replay request and envelope for `invocation` as a counterexample to the frame of
    /// `deposit`, claiming `field` of `account` was written.
    pub fn frame_replay(
        &self,
        counterexample: &Counterexample,
        invocation: &Invocation,
        account: &str,
        field: &str,
    ) -> FrameReplay {
        self.try_frame_replay("deposit", counterexample, invocation, account, field)
            .expect("the twin's operation frame is located")
    }

    /// [`Self::frame_replay`] for the operation named `operation` of `Bank::Account`.
    pub fn try_frame_replay(
        &self,
        operation: &str,
        counterexample: &Counterexample,
        invocation: &Invocation,
        account: &str,
        field: &str,
    ) -> Result<FrameReplay, FrameReplayError> {
        let identifier = |name| Identifier::new(name).expect("identifier");
        let unlimited = limits(u64::MAX);
        let run = ReplayInputs {
            source: LockedSource {
                authority: AUTHORITY.to_owned(),
                identity: IDENTITY.to_owned(),
                namespace: "git".to_owned(),
                revision: "1".to_owned(),
                bytes: self.unit.clone(),
            },
            dependencies: Vec::<DependencyLock>::new(),
            backend_manifest: DigestRecord::mint(DigestDomain::ToolManifestJcsV1, [3; 32]),
            accounting_limits: limits(1_000_000),
            stage_limits: StageLimits {
                s1: ScalarLimits {
                    text_input_bytes: u64::try_from(MAX_ENCODED_BYTES).expect("a small bound"),
                    ..unlimited
                },
                s2: unlimited,
                s3: unlimited,
                s4: unlimited,
            },
        };
        FrameReplay::new(FrameReplayInputs {
            run,
            packages: vec![ProvidedDocument {
                digest: DigestRecord::mint(DigestDomain::Sha256Jcs, jcs_digest(&self.domain)),
                bytes: self.domain.clone(),
            }],
            state_documents: invocation
                .documents
                .iter()
                .map(|(reference, bytes)| document(reference, bytes))
                .collect(),
            operation: OperationName {
                model: identifier("Bank"),
                object: identifier("Account"),
                operation: identifier(operation),
            },
            invocation: invocation.reference.clone(),
            change: ClaimedChange::FieldWrite {
                object: SelectedObject {
                    population: population(),
                    key: account.to_owned(),
                },
                field: field.to_owned(),
            },
            obligation: counterexample.obligation.clone(),
            witness: counterexample.witness.clone(),
            counterexample_identity: [2; 32],
        })
    }

    /// Replays `invocation` as a counterexample to the frame of `deposit`, claiming `field` of
    /// `account` was written.
    pub fn replay(
        &self,
        counterexample: &Counterexample,
        invocation: &Invocation,
        account: &str,
        field: &str,
    ) -> Result<FrameReplayResult, ReplayRefusal> {
        self.replay_tampered(counterexample, invocation, account, field, Tamper::Nothing)
    }

    /// [`Self::replay`] with one envelope identity made to differ from the payload's.
    pub fn replay_tampered(
        &self,
        counterexample: &Counterexample,
        invocation: &Invocation,
        account: &str,
        field: &str,
        tamper: Tamper,
    ) -> Result<FrameReplayResult, ReplayRefusal> {
        let FrameReplay { wire, mut packet } =
            self.frame_replay(counterexample, invocation, account, field);
        let payload = packet
            .family_payload
            .as_ref()
            .expect("the payload is built");
        // The anchor node stands in for any node other than the frame.
        let other = payload.anchor;
        match tamper {
            Tamper::Nothing => {}
            Tamper::ClauseNode => packet.clause_node = Some(other),
            Tamper::Occurrence => {
                packet.occurrence_key = Some(OccurrenceKey::new(
                    other,
                    payload.occurrence.origin().clone(),
                ));
            }
        }
        let envelope =
            WitnessEnvelope::reconstruct(packet).expect("a complete packet reconstructs");
        qsl_replay::replay_frame(wire, &envelope)
    }
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
