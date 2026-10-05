//! The hand-mirrored QSL twin of the state-frame fixture: the `test/bank` domain package, the
//! native unit whose `post` clause and operation frame mirror the checked package the harnesses
//! are generated from, and the invocation documents of one concrete run.
//!
//! The twin's fields, their ranges and the fields its frame grants come from `model`, the same
//! source the Rust fixture's checked package is built from. The rest is tied to the Rust side only
//! by names: the model `Bank`, its object `Account` and its operation `deposit`. The request and
//! envelope that put a run before QSL are built by the crate under test
//! (`quire_contract_codegen::FrameReplay`), which asks QSL for the node identities a
//! counterexample names; they are not the fixture package's node ids. The frame replay is given
//! the falsified harness's identity and its playback and mints the obligation identity itself, so
//! the twin supplies none. The fixture's checked package is hand-built and its node ids are its
//! own, while QSL names the nodes of the package it compiles from the twin's unit, so
//! [`Twin::aligned`] gives a harness identity QSL's anchor and frame, as the identity of a harness
//! generated from QSL's own emitted package carries them.

use super::model;

use qsl_replay::{
    call_site, CallSiteRefusal, ClaimedChange, ClauseName, ClauseSite, DependencyInput,
    DigestDomain, DigestRecord, DocumentRef, FrameReplayResult, Identifier, OccurrenceKey,
    OperationName, OperationSite, ReplayRefusal, ScalarLimits, SelectedObject, SourceIdentity,
    StageLimits, WitnessEnvelope, MAX_ENCODED_BYTES,
};
use quire_contract_codegen::{
    DependencyLock, DocumentLabel, FrameReplay, FrameReplayError, FrameReplayInputs, LockedSource,
    ProvidedDocument, ReplayInputs, StateClauseReplayInputs, StateFrameIdentity,
    StateObjectAddress,
};
use quire_contract_model::{
    CheckedNodeId, CheckedPackageEvidence, CheckedPackageReadLimits, CheckedPackageV2,
    CheckedPackageV2ReadResult,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const AUTHORITY: &str = "agent-ix";
const IDENTITY: &str = "test:state-frame-twin";
const PACKAGE: &str = "test/bank";
const PLACEHOLDER_DIGEST: &str =
    "sha256:0000000000000000000000000000000000000000000000000000000000000000";

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
/// `transfer` no clause names, and one closed population. Neither operation declares a parameter
/// or a result, the shape the state-clause replay supports.
fn domain_document(granted: &[&str]) -> Vec<u8> {
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
            "pre": [],
            "post": [],
            "origin": {"source": {
                "sourceIdentity": format!("ix://{PACKAGE}/spec"),
                "path": "spec.qspec",
                "startLine": 1,
                "startColumn": 1,
            }},
            "frame": {
                "modifies": granted.iter().map(|name| format!("{account}/{name}")).collect::<Vec<_>>(),
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

/// The clauses of the twin's unit: `(name, operation, field)`, each `post <name> ... on
/// Bank::Account::<operation> { self.<field> >= pre(self.<field>) }`.
pub const CLAUSES: [(&str, &str, &str); 2] = [
    ("BalanceNeverDrops", "deposit", "balance"),
    ("AuditNeverDrops", "deposit", "audit"),
];

fn unit_source(model_digest: &str, clauses: &[(&str, &str, &str)], blank_lines: usize) -> String {
    let header = format!(
        "language \"ix:native\" edition \"1-draft\";\n\
         profile v = \"quire.value.complete/v1\";\n\
         model Bank = {PACKAGE:?} version \"1.0.0\" digest \"sha256-jcs:{model_digest}\";\n"
    );
    let clauses = clauses
        .iter()
        .map(|(name, operation, field)| {
            format!(
                "post {name} using v on Bank::Account::{operation} {{ \
                 self.{field} >= pre(self.{field}) }}\n"
            )
        })
        .collect::<String>();
    format!("{header}{}{clauses}", "\n".repeat(blank_lines))
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

/// One falsified frame run: the harness's identity and the playback text its run printed.
#[derive(Clone)]
pub struct Run {
    /// The harness's identity, as the generator persisted it.
    pub harness: StateFrameIdentity,
    /// The concrete-playback text.
    pub playback: String,
}

/// The playback block Kani prints for `harness`'s falsified assertion `check`, binding one `i64`
/// per entry of `values` in order: the block the decoder reads, with each value's own decoded
/// comment before its bytes. Built here for the default lane; the `kani` lane replays the real
/// text.
pub fn playback_text(harness: &StateFrameIdentity, check: &str, values: &[i64]) -> String {
    let entries = values
        .iter()
        .map(|value| {
            let bytes = value
                .to_le_bytes()
                .iter()
                .map(u8::to_string)
                .collect::<Vec<_>>()
                .join(", ");
            format!("        // {value}\n        vec![{bytes}],\n")
        })
        .collect::<String>();
    format!(
        "/// Test generated for harness `{path}`\n\
         /// Check for `assertion`: \"{check}\"\n\
         #[test]\n\
         fn kani_concrete_playback_check_1() {{\n\
         \x20   let concrete_vals: Vec<Vec<u8>> = vec![\n\
         {entries}\
         \x20   ];\n\
         \x20   kani::concrete_playback_run(concrete_vals, check);\n\
         }}\n",
        path = harness.harness_path(),
    )
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

/// The position of the invocation document in [`Invocation`]'s documents.
pub const INVOCATION_DOCUMENT: usize = 0;
/// The position of the pre snapshot.
pub const PRE_DOCUMENT: usize = 1;

impl Invocation {
    /// This invocation with the document at `index` edited as JSON and addressed by the digest
    /// of its new bytes. An edited pre snapshot is re-linked from the invocation, which is
    /// re-addressed in turn, so every provided document still hashes to its digest and the
    /// edit is what the replay reads.
    pub fn edited(&self, index: usize, edit: impl FnOnce(&mut Value)) -> Self {
        let mut documents = self.documents.clone();
        let mut value: Value =
            serde_json::from_slice(&documents[index].1).expect("a JSON document");
        edit(&mut value);
        documents[index].1 = value.to_string().into_bytes();
        documents[index].0.digest = jcs_digest(&documents[index].1);
        if index == PRE_DOCUMENT {
            let mut invocation: Value =
                serde_json::from_slice(&documents[INVOCATION_DOCUMENT].1).expect("JSON");
            invocation["pre"]["digest"] = json!(format!(
                "sha256-jcs:{}",
                hex(&documents[PRE_DOCUMENT].0.digest)
            ));
            documents[INVOCATION_DOCUMENT].1 = invocation.to_string().into_bytes();
            documents[INVOCATION_DOCUMENT].0.digest = jcs_digest(&documents[INVOCATION_DOCUMENT].1);
        }
        Self {
            reference: documents[INVOCATION_DOCUMENT].0.clone(),
            documents,
        }
    }

    /// This invocation with the bytes of the document at `index` replaced and its digest left
    /// as it was: a provided document whose bytes do not match the digest it is addressed by.
    pub fn replaced(&self, index: usize, bytes: &[u8]) -> Self {
        let mut documents = self.documents.clone();
        documents[index].1 = bytes.to_vec();
        Self {
            reference: self.reference.clone(),
            documents,
        }
    }

    /// This invocation with the document at `index` not provided.
    pub fn without(&self, index: usize) -> Self {
        let mut documents = self.documents.clone();
        documents.remove(index);
        Self {
            reference: self.reference.clone(),
            documents,
        }
    }
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
        Self::build(&model::GRANTED, &CLAUSES, 0)
    }

    /// A twin whose operations' frames modify `granted`, whose unit holds `clauses` and has
    /// `blank_lines` empty lines before the first of them.
    pub fn build(granted: &[&str], clauses: &[(&str, &str, &str)], blank_lines: usize) -> Self {
        let domain = domain_document(granted);
        let unit = unit_source(&hex(&jcs_digest(&domain)), clauses, blank_lines).into_bytes();
        Self { unit, domain }
    }

    /// The `OperationSite` `qsl_replay::call_site` returns for `Bank::Account::<operation>`,
    /// read directly and not through the crate under test.
    pub fn operation_site(&self, operation: &str) -> Result<OperationSite, Box<CallSiteRefusal>> {
        call_site(
            SourceIdentity::new(AUTHORITY, IDENTITY, "git", "1"),
            IDENTITY,
            &self.unit,
            [self.domain.as_slice()],
            &DependencyInput::default(),
            &self.operation(operation),
        )
        .map(|located| located.site)
    }

    /// The checked package QSL compiles from the twin's unit and the node of the clause named
    /// `clause`, both as `qsl_replay::call_site` returns them: the package bytes are read and
    /// admitted by the model reader, with the twin's domain package as the evidence of its model
    /// selection. Nothing here is hand-built, so a harness generated from it carries the node
    /// ids QSL itself names.
    pub fn emitted_package(&self, clause: &str) -> (CheckedPackageV2, CheckedNodeId) {
        let name = ClauseName(Identifier::new(clause).expect("identifier"));
        let located = call_site(
            SourceIdentity::new(AUTHORITY, IDENTITY, "git", "1"),
            IDENTITY,
            &self.unit,
            [self.domain.as_slice()],
            &DependencyInput::default(),
            &name,
        )
        .expect("the clause is located");
        let mut evidence = CheckedPackageEvidence::new();
        evidence
            .insert_domain_package_document(hex(&jcs_digest(&self.domain)), self.domain.clone());
        evidence.support_feature("quire.value.complete/v1");
        let package = match CheckedPackageV2::read(
            &located.package,
            CheckedPackageReadLimits::bounded(),
            &evidence,
        ) {
            CheckedPackageV2ReadResult::Admitted(package) => *package,
            other => panic!("QSL's emitted package is admitted: {other:?}"),
        };
        let clause_node = serde_json::from_value(json!({
            "domain": "quire.checked-semantic-node/v1",
            "digest": located.site.node.to_string(),
        }))
        .expect("a node id");
        (package, clause_node)
    }

    /// `harness` with the scope's anchor and frame the ones QSL names for `operation` in this
    /// twin's unit.
    pub fn aligned(&self, harness: &StateFrameIdentity, operation: &str) -> StateFrameIdentity {
        let site = self
            .operation_site(operation)
            .expect("the twin's operation is located");
        let node = |id: qsl_replay::WireNodeId| -> quire_contract_model::CheckedNodeId {
            serde_json::from_value(json!({
                "domain": "quire.checked-semantic-node/v1",
                "digest": id.to_string(),
            }))
            .expect("a node id")
        };
        let mut aligned = harness.clone();
        aligned.scope.anchor = node(site.anchor);
        aligned.scope.frame = node(site.frame);
        aligned
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
            "result": null,
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
    /// `deposit`, claiming `field` of `account` was written, from the falsified `run`.
    pub fn frame_replay(
        &self,
        invocation: &Invocation,
        account: &str,
        field: &str,
        run: &Run,
    ) -> FrameReplay {
        self.try_frame_replay("deposit", invocation, account, field, run)
            .expect("the twin's operation frame is located")
    }

    /// [`Self::frame_replay`] for the operation named `operation` of `Bank::Account`.
    pub fn try_frame_replay(
        &self,
        operation: &str,
        invocation: &Invocation,
        account: &str,
        field: &str,
        run: &Run,
    ) -> Result<FrameReplay, FrameReplayError> {
        FrameReplay::new(FrameReplayInputs {
            run: self.run(),
            packages: self.packages(),
            state_documents: invocation
                .documents
                .iter()
                .map(|(reference, bytes)| document(reference, bytes))
                .collect(),
            operation: self.operation(operation),
            invocation: invocation.reference.clone(),
            change: ClaimedChange::FieldWrite {
                object: SelectedObject {
                    population: population(),
                    key: account.to_owned(),
                },
                field: field.to_owned(),
            },
            harness: run.harness.clone(),
            playback: run.playback.clone(),
        })
    }

    /// The proving run's lock: the unit, no dependency, and the limits the twin replays under.
    fn run(&self) -> ReplayInputs {
        let unlimited = limits(u64::MAX);
        ReplayInputs {
            source: LockedSource {
                authority: AUTHORITY.to_owned(),
                identity: IDENTITY.to_owned(),
                namespace: "git".to_owned(),
                revision: "1".to_owned(),
                bytes: self.unit.clone(),
            },
            dependencies: Vec::<DependencyLock>::new(),
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
        }
    }

    /// The domain package, provided by its `sha256-jcs` digest.
    fn packages(&self) -> Vec<ProvidedDocument> {
        vec![ProvidedDocument {
            digest: DigestRecord::mint(DigestDomain::Sha256Jcs, jcs_digest(&self.domain)),
            bytes: self.domain.clone(),
        }]
    }

    /// The `ClauseSite` `qsl_replay::call_site` returns for the clause named `clause`: the
    /// identities QSL itself gives it, read directly and not through the crate under test.
    pub fn clause_site(&self, clause: &str) -> Result<ClauseSite, Box<CallSiteRefusal>> {
        let name = ClauseName(Identifier::new(clause).expect("identifier"));
        call_site(
            SourceIdentity::new(AUTHORITY, IDENTITY, "git", "1"),
            IDENTITY,
            &self.unit,
            [self.domain.as_slice()],
            &DependencyInput::default(),
            &name,
        )
        .map(|located| located.site)
    }

    /// `Bank::Account::<operation>`.
    fn operation(&self, operation: &str) -> OperationName {
        let identifier = |name| Identifier::new(name).expect("identifier");
        OperationName {
            model: identifier("Bank"),
            object: identifier("Account"),
            operation: identifier(operation),
        }
    }

    /// The state-clause replay inputs for the clause `clause` of `deposit` on `account`, over the
    /// pre state the playback binds and the post state the subject ran to, each
    /// `(balance, audit)`, with the admitted `package` and `clause_node` the harness was
    /// generated from.
    pub fn state_clause_inputs<'a>(
        &self,
        package: &'a CheckedPackageV2,
        clause_node: &'a CheckedNodeId,
        clause: &str,
        playback: (i64, i64),
        post: (i64, i64),
    ) -> StateClauseReplayInputs<'a> {
        let label = |name: &str| DocumentLabel {
            authority: "test".to_owned(),
            identity: name.to_owned(),
            revision_namespace: "ns".to_owned(),
            revision: "1".to_owned(),
        };
        let [balance, audit] = model::FIELDS.map(|(name, _)| name.to_owned());
        StateClauseReplayInputs {
            run: self.run(),
            packages: self.packages(),
            package,
            clause_node,
            state_fields: model::FIELDS.map(|(name, _)| name.to_owned()).to_vec(),
            operation: self.operation("deposit"),
            clause: ClauseName(Identifier::new(clause).expect("identifier")),
            object: StateObjectAddress {
                population: population(),
                key: "account".to_owned(),
                object_type: account_type(),
            },
            invocation_label: label("twin-invocation"),
            pre_label: label("twin-pre"),
            post_label: label("twin-post"),
            playback: vec![(balance.clone(), playback.0), (audit.clone(), playback.1)],
            post_state: vec![(balance, post.0), (audit, post.1)],
            obligation_identity: [1; 32],
        }
    }

    /// Replays `invocation` as a counterexample to the frame of `deposit`, claiming `field` of
    /// `account` was written.
    pub fn replay(
        &self,
        invocation: &Invocation,
        account: &str,
        field: &str,
        run: &Run,
    ) -> Result<FrameReplayResult, ReplayRefusal> {
        self.replay_tampered(invocation, account, field, run, Tamper::Nothing)
    }

    /// [`Self::replay`] with one envelope identity made to differ from the payload's.
    pub fn replay_tampered(
        &self,
        invocation: &Invocation,
        account: &str,
        field: &str,
        run: &Run,
        tamper: Tamper,
    ) -> Result<FrameReplayResult, ReplayRefusal> {
        let FrameReplay { wire, mut packet } = self.frame_replay(invocation, account, field, run);
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
