//! Admission-only QSL fixtures for request-level clause callers.
//!
//! These retain the state-frame fixture's authored domain/source facts, but depend only on
//! QSL and the strict checked-package reader. No CG facade, replay builder or execution path.

use super::model;
use qsl_replay::{call_site, ClauseName, DependencyInput, Identifier, SourceIdentity};
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

fn range_type(field: &str) -> String {
    format!("ix://{PACKAGE}/{field}_range")
}

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

fn domain_document(granted: &[&str]) -> Vec<u8> {
    let account = account_type();
    let bound = |field: &str, keyword: &str, value: i64| {
        let range = range_type(field);
        json!({
            "identity": format!("{range}/constraints/{keyword}"),
            "keyword": keyword,
            "operands": {"value": value.to_string()},
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
        "canonicalization": {"algorithm": "rfc8785-v1", "digest": "sha256-jcs"},
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

pub struct Twin {
    pub unit: Vec<u8>,
    pub domain: Vec<u8>,
}

impl Twin {
    pub fn new() -> Self {
        Self::build(&model::GRANTED, &CLAUSES, 0)
    }

    pub fn with_deposit_parameter() -> Self {
        Self::with_deposit_declaration(true, false)
    }

    pub fn with_bounded_other_condition() -> Self {
        let mut twin = Self::with_deposit_parameter();
        let mut domain: Value = serde_json::from_slice(&twin.domain).expect("domain document");
        let account = domain["types"]
            .as_array_mut()
            .expect("types")
            .iter_mut()
            .find(|ty| ty["identity"] == account_type())
            .expect("account type");
        let deposit = account["operations"]
            .as_array_mut()
            .expect("operations")
            .iter_mut()
            .find(|operation| operation["name"] == "deposit")
            .expect("deposit operation");
        deposit["params"][0]["typeRef"] = json!(range_type("balance"));
        twin.domain = domain.to_string().into_bytes();
        let source = unit_source(&hex(&jcs_digest(&twin.domain)), &[CLAUSES[0]], 0);
        twin.unit = source
            .replace("self.balance >= pre(self.balance)", "self.balance >= other")
            .into_bytes();
        twin
    }

    pub fn with_bounded_other_precondition() -> Self {
        let mut twin = Self::with_bounded_other_condition();
        let source = String::from_utf8(twin.unit).expect("native unit");
        let changed = source.replace(
            "post BalanceNeverDrops using v on Bank::Account::deposit { self.balance >= other }",
            "pre OtherBelowLimit using v on Bank::Account::deposit { other < 1000 }",
        );
        assert_ne!(
            changed, source,
            "the precondition replaced the postcondition"
        );
        twin.unit = changed.into_bytes();
        twin
    }

    fn with_deposit_declaration(parameter: bool, result: bool) -> Self {
        let mut twin = Self::new();
        let mut domain: Value = serde_json::from_slice(&twin.domain).expect("domain document");
        let account = domain["types"]
            .as_array_mut()
            .expect("types")
            .iter_mut()
            .find(|ty| ty["identity"] == account_type())
            .expect("account type");
        let deposit = account["operations"]
            .as_array_mut()
            .expect("operations")
            .iter_mut()
            .find(|operation| operation["name"] == "deposit")
            .expect("deposit operation");
        if parameter {
            let identity = format!("{}/deposit/other", account_type());
            deposit["params"] = json!([{
                "identity": identity,
                "name": "other",
                "typeRef": "ix://quire/native/Boolean",
                "presence": "required",
                "nullable": false,
                "defaultKind": "none",
                "multiplicity": {"lower": 1, "upper": 1, "ordered": false, "unique": true},
                "origin": generated(&identity),
            }]);
        }
        if result {
            deposit["returns"] = json!({
                "typeRef": "ix://quire/native/Boolean",
                "multiplicity": {"lower": 1, "upper": 1, "ordered": false, "unique": true},
                "nullable": false,
            });
        }
        twin.domain = domain.to_string().into_bytes();
        twin.unit = unit_source(&hex(&jcs_digest(&twin.domain)), &CLAUSES, 0).into_bytes();
        twin
    }

    pub fn with_distinct_balance_clauses() -> Self {
        let mut twin = Self::build(
            &model::GRANTED,
            &[
                CLAUSES[0],
                ("SpareA", "deposit", "balance"),
                ("SpareB", "deposit", "balance"),
                ("SpareC", "deposit", "balance"),
            ],
            0,
        );
        let mut source = String::from_utf8(twin.unit).expect("native unit");
        for (name, operator) in [("SpareA", ">"), ("SpareB", "<"), ("SpareC", "<=")] {
            let original = format!(
                "post {name} using v on Bank::Account::deposit {{ self.balance >= pre(self.balance) }}"
            );
            let changed = format!(
                "post {name} using v on Bank::Account::deposit {{ self.balance {operator} pre(self.balance) }}"
            );
            let updated = source.replace(&original, &changed);
            assert_ne!(updated, source, "{name} is a distinct clause");
            source = updated;
        }
        twin.unit = source.into_bytes();
        twin
    }

    pub fn with_mixed_coverage_clauses() -> Self {
        let mut twin = Self::build(
            &model::GRANTED,
            &[
                CLAUSES[0],
                ("SpareA", "deposit", "balance"),
                ("SpareB", "deposit", "balance"),
                ("SpareC", "deposit", "balance"),
            ],
            0,
        );
        let mut source = String::from_utf8(twin.unit).expect("native unit");
        for (name, condition) in [
            (
                "BalanceNeverDrops",
                "self.balance >= 0 implies self.balance <= 1000",
            ),
            (
                "SpareA",
                "self.balance >= 0 implies (self.balance <= 1000 implies self.balance >= pre(self.balance))",
            ),
            ("SpareB", "self.balance < pre(self.balance)"),
            ("SpareC", "self.balance <= pre(self.balance)"),
        ] {
            let original = format!(
                "post {name} using v on Bank::Account::deposit {{ self.balance >= pre(self.balance) }}"
            );
            let changed = format!(
                "post {name} using v on Bank::Account::deposit {{ {condition} }}"
            );
            let updated = source.replace(&original, &changed);
            assert_ne!(updated, source, "{name} condition changed");
            source = updated;
        }
        twin.unit = source.into_bytes();
        twin
    }

    pub fn build(granted: &[&str], clauses: &[(&str, &str, &str)], blank_lines: usize) -> Self {
        let domain = domain_document(granted);
        let unit = unit_source(&hex(&jcs_digest(&domain)), clauses, blank_lines).into_bytes();
        Self { unit, domain }
    }

    pub fn emitted_package(&self, clause: &str) -> (CheckedPackageV2, CheckedNodeId) {
        let located = call_site(
            SourceIdentity::new(AUTHORITY, IDENTITY, "git", "1"),
            IDENTITY,
            &self.unit,
            [self.domain.as_slice()],
            &DependencyInput::default(),
            &ClauseName(Identifier::new(clause).expect("clause identifier")),
        )
        .expect("authored clause is located");
        let mut evidence = CheckedPackageEvidence::new();
        evidence
            .insert_domain_package_document(hex(&jcs_digest(&self.domain)), self.domain.clone())
            .expect("retain authored domain document");
        evidence.support_feature("quire.value.complete/v1");
        let package = match CheckedPackageV2::read(
            &located.package,
            CheckedPackageReadLimits::bounded(),
            &evidence,
        ) {
            CheckedPackageV2ReadResult::Admitted(package) => *package,
            other => panic!("QSL emitted package is admitted: {other:?}"),
        };
        let node = serde_json::from_value(json!({"domain": "quire.checked-semantic-node/v1", "digest": located.site.node.to_string()}))
            .expect("owner clause identity");
        (package, node)
    }
}
