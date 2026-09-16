//! FR-014: exact complete-V1 scalar oracle generation over admitted
//! CheckedPackage V2 input.

use std::{
    collections::BTreeMap,
    fs,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use quire_contract_codegen::{
    generate_exact_scalar_oracles, ExactScalarDisposition, ExactScalarItem, ExactScalarOracles,
    ExactScalarRefusal, ScalarForm, UpstreamBlocker, EXACT_SCALAR_CLAIM_MAP_VERSION,
    EXACT_SCALAR_CRATE_NAME, RUNTIME_REVISION,
};
use quire_contract_ir::CheckedPackageV2;
use serde_json::Value;

#[path = "exact_scalar_support/package.rs"]
mod package;

use package::*;

/// Set to regenerate the committed golden files instead of comparing them.
const BLESS: &str = "QUIRE_CODEGEN_BLESS";

struct TemporaryDirectory(PathBuf);

impl TemporaryDirectory {
    fn new(prefix: &str) -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("{prefix}-{}-{nonce}", std::process::id()));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).unwrap();
        Self(path)
    }
}

impl Drop for TemporaryDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn generate(package: &CheckedPackageV2, items: &[ExactScalarItem]) -> ExactScalarOracles {
    generate_exact_scalar_oracles(package, items).expect("generation succeeds")
}

fn contents<'o>(oracles: &'o ExactScalarOracles, path: &str) -> &'o str {
    &oracles
        .artifacts
        .iter()
        .find(|artifact| artifact.path == path)
        .unwrap_or_else(|| panic!("artifact {path}"))
        .contents
}

fn dispositions(oracles: &ExactScalarOracles) -> BTreeMap<String, &ExactScalarDisposition> {
    oracles
        .claim_map
        .items
        .iter()
        .map(|claim| (claim.node_id.digest.to_string(), &claim.result))
        .collect()
}

fn refusal_of(oracles: &ExactScalarOracles, code: u32) -> ExactScalarRefusal {
    match dispositions(oracles).get(&key(code)) {
        Some(ExactScalarDisposition::Refused { refusal }) => refusal.clone(),
        other => panic!("node {code} is not refused: {other:?}"),
    }
}

fn symbol(code: u32) -> String {
    format!("oracle_{}", key(code))
}

/// Trace: FR-014-AC-4, TC-024.
#[test]
fn tc_024_generation_matches_the_committed_golden_files() {
    let oracles = generate(&corpus_package().admit(), &golden_items());
    let fixtures = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/exact_scalar");
    for (artifact, golden) in [
        ("Cargo.toml", "Cargo.toml.golden"),
        ("src/lib.rs", "lib.rs.golden"),
        ("claim-map.json", "claim-map.json.golden"),
    ] {
        let path = fixtures.join(golden);
        if std::env::var_os(BLESS).is_some() {
            fs::write(&path, contents(&oracles, artifact)).expect("write golden");
        }
        let expected = fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("{}: {error}; set {BLESS}=1", path.display()));
        assert_eq!(
            contents(&oracles, artifact),
            expected,
            "{artifact} drifted from {golden}"
        );
    }
    for artifact in &oracles.artifacts {
        assert_eq!(
            artifact.sha256,
            sha256_hex(artifact.contents.as_bytes()),
            "{}",
            artifact.path
        );
    }
}

/// Trace: FR-014-AC-4, TC-024.
#[test]
fn tc_024_generation_is_byte_deterministic_across_runs_and_request_orders() {
    let package = corpus_package().admit();
    let items = golden_items();
    let first = generate(&package, &items);
    assert_eq!(generate(&package, &items), first);
    assert_eq!(generate(&corpus_package().admit(), &items), first);

    let mut reversed = items.clone();
    reversed.reverse();
    assert_eq!(generate(&package, &reversed), first);

    // A deterministic interleaving that moves every item.
    let (even, odd): (Vec<_>, Vec<_>) = items
        .iter()
        .cloned()
        .enumerate()
        .partition(|(position, _)| position % 2 == 0);
    let interleaved = odd
        .into_iter()
        .chain(even)
        .map(|(_, item)| item)
        .collect::<Vec<_>>();
    assert_ne!(
        interleaved
            .iter()
            .map(|item| &item.node_id)
            .collect::<Vec<_>>(),
        items.iter().map(|item| &item.node_id).collect::<Vec<_>>()
    );
    assert_eq!(generate(&package, &interleaved), first);
}

/// Trace: FR-014-AC-1, FR-014-AC-2, TC-024.
#[test]
fn tc_024_every_scalar_family_generates_one_oracle_calling_its_runtime_operator() {
    let oracles = generate(&corpus_package().admit(), &golden_items());
    let lib = contents(&oracles, "src/lib.rs");
    let calls = [
        (1001, "rt::evaluate_integer(rt::IntegerOperation::Add("),
        (1002, "rt::evaluate_integer(rt::IntegerOperation::Negate("),
        (1011, "rt::divide(rt::DivisionProfile::Truncating,"),
        (1012, "rt::divide(rt::DivisionProfile::Floor,"),
        (1013, "rt::divide(rt::DivisionProfile::Euclidean,"),
        (1014, "rt::divide(rt::DivisionProfile::Truncating,"),
        (1021, "rt::modulo("),
        (1031, "rt::evaluate_rational(rt::RationalOperation::Add("),
        (1032, "rt::evaluate_rational(rt::RationalOperation::Divide("),
        (
            1033,
            "rt::evaluate_rational(rt::RationalOperation::IntegerDivide(",
        ),
        (1041, "rt::OrderingOperands::Integer("),
        (1042, "rt::OrderingOperands::Decimal("),
        (1043, "rt::OrderingOperands::Rational("),
        (1051, "rt::evaluate_decimal(rt::DecimalOperation::Add("),
        (1052, "rt::evaluate_decimal(rt::DecimalOperation::Divide("),
        (1053, "rt::evaluate_decimal(rt::DecimalOperation::Round("),
        (1061, "rt::evaluate_ieee(rt::IeeeOperation::Add("),
        (1062, "rt::evaluate_ieee(rt::IeeeOperation::Divide("),
        (1063, "rt::compare_ieee(rt::IeeeComparison::TotalOrder,"),
        (1064, "rt::convert_ieee_width("),
        (1071, "rt::admit_text("),
        (1072, "rt::compare_text(rt::ComparisonOperator::Less,"),
        (1073, "rt::compare_enum(rt::ComparisonOperator::Less,"),
        (1081, "rt::evaluate_quantity(rt::QuantityOperation::Add("),
        (
            1082,
            "rt::evaluate_quantity(rt::QuantityOperation::Multiply(",
        ),
        (1083, "rt::evaluate_quantity(rt::QuantityOperation::Power("),
        (1084, "rt::compare_quantity(rt::ComparisonOperator::Less,"),
        (1085, "rt::QuantityTarget::Exact"),
        (1086, "rt::QuantityTarget::Decimal("),
        (1087, "rt::QuantityTarget::Integer {"),
    ];
    let generated = dispositions(&oracles)
        .into_iter()
        .filter(|(_, result)| matches!(result, ExactScalarDisposition::Generated(_)))
        .map(|(digest, _)| digest)
        .collect::<Vec<_>>();
    assert_eq!(
        generated,
        calls.iter().map(|(code, _)| key(*code)).collect::<Vec<_>>(),
        "exactly the corpus is generated"
    );
    assert_eq!(corpus().len(), calls.len());
    for (code, call) in calls {
        let body = function_body(lib, &symbol(code));
        assert!(
            body.contains(call),
            "{code} does not call `{call}`:\n{body}"
        );
        assert!(body.contains("meter"), "{code} does not meter");
    }
    assert_eq!(lib.matches("\npub fn oracle_").count(), calls.len());
}

/// The text of one generated function, from its signature to the closing brace.
fn function_body<'l>(lib: &'l str, symbol: &str) -> &'l str {
    let start = lib
        .find(&format!("pub fn {symbol}("))
        .unwrap_or_else(|| panic!("{symbol} is generated"));
    let end = lib[start..].find("\n}\n").expect("function end") + start + 3;
    &lib[start..end]
}

/// Trace: FR-014-AC-1, FR-014-AC-3, FR-014-AC-7, TC-024.
#[test]
fn tc_024_refused_items_are_typed_emit_no_code_and_leave_siblings_unchanged() {
    let package = corpus_package().admit();
    let oracles = generate(&package, &golden_items());
    let lib = contents(&oracles, "src/lib.rs");
    let blocked = |code, tag, issue| ExactScalarRefusal::BlockedOnUpstream {
        unsupported_node_id: code_id(code),
        node_tag: tag,
        issue,
    };
    let unsupported = |code, tag| ExactScalarRefusal::Unsupported {
        unsupported_node_id: code_id(code),
        node_tag: tag,
    };
    let expected = [
        (
            COMPOSITE,
            blocked(
                COMPOSITE,
                "composite_type",
                UpstreamBlocker::QuireSpecLanguage119,
            ),
        ),
        (
            FUNCTION,
            blocked(FUNCTION, "function", UpstreamBlocker::QuireSpecLanguage119),
        ),
        (
            CALLS_FUNCTION,
            blocked(FUNCTION, "function", UpstreamBlocker::QuireSpecLanguage119),
        ),
        (
            MODEL,
            blocked(MODEL, "model", UpstreamBlocker::QuireSpecLanguage120),
        ),
        (
            RELATION,
            blocked(RELATION, "relation", UpstreamBlocker::QuireSpecLanguage120),
        ),
        (STATE, unsupported(STATE, "state")),
        (TEMPORAL, unsupported(TEMPORAL, "temporal")),
        (PROTOCOL, unsupported(PROTOCOL, "protocol")),
        (MISSING, ExactScalarRefusal::InvalidInput),
        (
            V_INTEGER,
            ExactScalarRefusal::NotExpression { node_tag: "value" },
        ),
        (DUPLICATED, ExactScalarRefusal::DuplicateRequest),
        (
            WRONG_BODY,
            ExactScalarRefusal::BodyMismatch {
                expected_operator: "binary",
                expected_arguments: 2,
            },
        ),
        (
            WRONG_OPERAND,
            ExactScalarRefusal::OperandTypeMismatch {
                position: 1,
                expected: ScalarForm::Integer,
                found: Some("decimal".to_owned()),
            },
        ),
        (
            WRONG_RESULT,
            ExactScalarRefusal::ResultTypeMismatch {
                expected: ScalarForm::Decimal,
                found: Some("integer".to_owned()),
            },
        ),
        (
            WRONG_ARITY,
            ExactScalarRefusal::FormMismatch {
                expected: "unary",
                found: "binary".to_owned(),
            },
        ),
    ];
    for (code, refusal) in &expected {
        assert_eq!(refusal_of(&oracles, *code), *refusal, "node {code}");
        assert!(
            !lib.contains(&key(*code)),
            "refused node {code} left code behind"
        );
    }
    // One disposition per distinct requested node.
    assert_eq!(
        oracles.claim_map.items.len(),
        corpus().len() + expected.len()
    );

    // Each generated function is byte-identical with or without the refused items.
    let generated_only = corpus()
        .into_iter()
        .map(|expression| ExactScalarItem {
            node_id: code_id(expression.code),
            operation: expression.operation,
        })
        .collect::<Vec<_>>();
    let alone = generate(&package, &generated_only);
    assert_eq!(contents(&alone, "src/lib.rs"), lib);
    let generated_claims = |oracles: &ExactScalarOracles| {
        oracles
            .claim_map
            .items
            .iter()
            .filter(|claim| matches!(claim.result, ExactScalarDisposition::Generated(_)))
            .cloned()
            .collect::<Vec<_>>()
    };
    assert_eq!(generated_claims(&alone), generated_claims(&oracles));

    // A refusal of one copy of a duplicated sibling does not depend on its
    // descriptors' arrival order, and a single copy of the same node generates.
    let single = generate(
        &package,
        &[ExactScalarItem {
            node_id: code_id(DUPLICATED),
            operation: golden_items()
                .into_iter()
                .find(|item| item.node_id == code_id(DUPLICATED))
                .expect("duplicated item")
                .operation,
        }],
    );
    assert!(matches!(
        single.claim_map.items[0].result,
        ExactScalarDisposition::Generated(_)
    ));
}

/// Trace: FR-014-AC-5, TC-024.
#[test]
fn tc_024_claim_map_carries_identity_source_bounds_and_operation_per_item() {
    let wire = corpus_package().wire();
    let package = corpus_package().admit();
    let oracles = generate(&package, &golden_items());
    let map = &oracles.claim_map;
    assert_eq!(map.version, EXACT_SCALAR_CLAIM_MAP_VERSION);
    assert_eq!(map.runtime_revision, RUNTIME_REVISION);
    assert_eq!(map.package_id, *package.package_id());
    let json: Value = serde_json::from_str(contents(&oracles, "claim-map.json")).expect("json");
    assert_eq!(json, serde_json::to_value(map).expect("typed map"));

    let digests = map
        .items
        .iter()
        .map(|claim| claim.node_id.digest.to_string())
        .collect::<Vec<_>>();
    let mut sorted = digests.clone();
    sorted.sort();
    assert_eq!(digests, sorted, "entries ascend by node digest");

    let lowered = package.lower(
        &corpus()
            .iter()
            .map(|expression| code_id(expression.code))
            .collect::<Vec<_>>(),
        &quire_contract_ir::CompleteLoweringProfileV2 {
            supported_tags: [
                quire_contract_ir::CheckedNodeTag::ScalarType,
                quire_contract_ir::CheckedNodeTag::Value,
                quire_contract_ir::CheckedNodeTag::Expression,
            ]
            .into(),
            require_bounds: false,
            work_limit: u64::MAX,
        },
    );
    for (expression, record) in corpus().iter().zip(&lowered.records) {
        let quire_contract_ir::CompleteLoweringRecordV2::Lowered { node } = record else {
            panic!("corpus node {} lowers", expression.code);
        };
        let claim = map
            .items
            .iter()
            .find(|claim| claim.node_id == code_id(expression.code))
            .expect("claim");
        assert!(!claim.operation.is_empty());
        let ExactScalarDisposition::Generated(generated) = &claim.result else {
            panic!("corpus node {} generates", expression.code);
        };
        assert_eq!(generated.symbol, symbol(expression.code));
        assert_eq!(generated.ir_id, node.ir_id);
        assert_eq!(generated.semantic_form, expression.form);
        assert_eq!(generated.semantic_type, node.semantic_type);
        assert_eq!(generated.claims, node.claims);
        assert_eq!(generated.bounds, node.bounds);
        assert_eq!(generated.dependencies, node.dependencies);
        let source_map = wire["source_map"]
            .as_array()
            .expect("source map")
            .iter()
            .filter(|entry| entry["node_id"]["digest"] == key(expression.code))
            .cloned()
            .collect::<Vec<_>>();
        assert_eq!(source_map.len(), 1);
        assert_eq!(
            serde_json::to_value(&generated.source_map).expect("map"),
            Value::Array(source_map)
        );
    }

    // Distinct descriptors produce distinct identities.
    let identities = map
        .items
        .iter()
        .filter(|claim| matches!(claim.result, ExactScalarDisposition::Generated(_)))
        .map(|claim| claim.operation.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(identities.len(), corpus().len());
}

/// Trace: FR-014-AC-8, FR-014-CON-1, FR-014-CON-2, TC-024.
#[test]
fn tc_024_generated_crate_is_unpublished_pinned_charge_free_and_compiles() {
    let oracles = generate(&corpus_package().admit(), &golden_items());
    let manifest = contents(&oracles, "Cargo.toml");
    assert!(manifest.contains(&format!("name = \"{EXACT_SCALAR_CRATE_NAME}\"")));
    assert!(manifest.contains("publish = false"));
    assert!(manifest.contains(&format!("rev = \"{RUNTIME_REVISION}\"")));
    assert!(manifest.contains("features = [\"exact\"]"));

    let lib = contents(&oracles, "src/lib.rs");
    for forbidden in [
        "ChargePoint",
        "charge(",
        "Resource::",
        "WorkLimits",
        "panic!",
        "unwrap(",
        "expect(",
        "unreachable!",
        "todo!",
        "unsafe {",
        "unsafe fn",
    ] {
        assert!(
            !lib.contains(forbidden),
            "generated source contains `{forbidden}`"
        );
    }

    let directory = TemporaryDirectory::new("quire-exact-scalar-oracles");
    fs::create_dir_all(directory.0.join("src")).unwrap();
    for artifact in &oracles.artifacts {
        fs::write(directory.0.join(&artifact.path), &artifact.contents).unwrap();
    }
    let output = Command::new(env!("CARGO"))
        .args(["build", "--offline", "--quiet"])
        .env("CARGO_TARGET_DIR", directory.0.join("target"))
        .env("RUSTFLAGS", "-Dwarnings")
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .current_dir(&directory.0)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "generated crate did not compile against runtime {RUNTIME_REVISION}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn sha256_hex(bytes: &[u8]) -> String {
    use sha2::Digest as _;
    format!("{:x}", sha2::Sha256::digest(bytes))
}
