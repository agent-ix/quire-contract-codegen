//! FR-018: composite/structural equality oracle generation over admitted
//! CheckedPackage V2 input.
//!
//! Generation-time coverage only (no execution of generated code): dispositions,
//! refusal reasons, schedules, ordering/determinism, symbol disambiguation,
//! the manifest and declaration keys. Execution-level criteria (AC-2, AC-8's
//! `check_type` guard, AC-9's denial injection, AC-11's complementary
//! outcomes) are covered by `tests/composite_equality_agreement.rs`.

use std::{
    collections::BTreeSet,
    fs,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use quire_contract_codegen::{
    generate_composite_equality_oracles, ClaimDisposition, CompositeEqualityClaim,
    CompositeEqualityItem, CompositeEqualityOracles, CompositeEqualityRefusal,
    DeclarationRefusalCause, EqualityOperandDescriptor, EqualityOperatorKind, IllTypedCauseKind,
    RecordedSchedule, UpstreamBlocker, COMPOSITE_EQUALITY_CRATE_NAME,
};
use quire_contract_ir::{
    CheckedNodeId, CheckedNodeTag, CheckedPackageV2, CompleteLoweringProfileV2,
    CompleteLoweringRecordV2,
};
use quire_contract_runtime::exact::{
    CardinalityBound, CollectionKind, CollectionType, CompositeDeclaration, CompositeShape,
    FieldDeclaration, IeeeWidth, NodeKey, ObjectTypeDeclaration, Presence, TypeEnvironment,
    ValueType,
};
use sha2::{Digest, Sha256};

// package.rs holds a process-global `application_registry()` static keyed by small integer
// fixture codes that this file and `composite_equality_agreement.rs` each pick independently,
// on the assumption of an isolated registry (verified: centralizing this module produced real
// cross-file code collisions and Mutex-poisoning cascades). Kept duplicated on purpose.
#[allow(clippy::duplicate_mod)]
#[path = "../composite_equality_support/package.rs"]
mod package;

use package::*;

const BLESS: &str = "QUIRE_CODEGEN_BLESS";

/// Independently recomputed `DescriptorKey::digest` (FR-018-AC-11): a SHA-256
/// over the expression node id, the operator's rank, then each operand's
/// source and conversion-target node ids, mirroring `hash_node_id` and
/// `hash_optional_node_id` byte for byte, and over no rendered name. Kept
/// deliberately independent of `src/composite_equality.rs` (no `pub` symbol
/// there is reused) so a generator mutation to the real digest cannot also
/// mutate this check.
fn recomputed_symbol_digest(
    node_id: &CheckedNodeId,
    operator: EqualityOperatorKind,
    left_source: &CheckedNodeId,
    left_target: Option<&CheckedNodeId>,
    right_source: &CheckedNodeId,
    right_target: Option<&CheckedNodeId>,
) -> String {
    fn hash_node_id(hasher: &mut Sha256, node_id: &CheckedNodeId) {
        hasher.update((node_id.domain.len() as u64).to_le_bytes());
        hasher.update(node_id.domain.as_bytes());
        hasher.update((node_id.digest.len() as u64).to_le_bytes());
        hasher.update(node_id.digest.as_bytes());
    }
    fn hash_optional_node_id(hasher: &mut Sha256, node_id: Option<&CheckedNodeId>) {
        match node_id {
            None => hasher.update([0_u8]),
            Some(node_id) => {
                hasher.update([1_u8]);
                hash_node_id(hasher, node_id);
            }
        }
    }
    let mut hasher = Sha256::new();
    hash_node_id(&mut hasher, node_id);
    hasher.update([operator as u8]);
    hash_node_id(&mut hasher, left_source);
    hash_optional_node_id(&mut hasher, left_target);
    hash_node_id(&mut hasher, right_source);
    hash_optional_node_id(&mut hasher, right_target);
    hasher
        .finalize()
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

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

fn generate(
    package: &CheckedPackageV2,
    items: &[CompositeEqualityItem],
) -> CompositeEqualityOracles {
    generate_composite_equality_oracles(package, items).expect("generation succeeds")
}

fn contents<'o>(oracles: &'o CompositeEqualityOracles, path: &str) -> &'o str {
    &oracles
        .artifacts
        .iter()
        .find(|artifact| artifact.path == path)
        .unwrap_or_else(|| panic!("artifact {path}"))
        .contents
}

/// All claims for `node` code, in claim-map order.
fn claims_for(oracles: &CompositeEqualityOracles, node: u32) -> Vec<&CompositeEqualityClaim> {
    oracles
        .claim_map
        .items
        .iter()
        .filter(|claim| claim.node_id.digest.as_ref() == code_id(node).digest.as_ref())
        .collect()
}

fn only_claim(oracles: &CompositeEqualityOracles, node: u32) -> &CompositeEqualityClaim {
    let claims = claims_for(oracles, node);
    assert_eq!(
        claims.len(),
        1,
        "expected exactly one claim for node {node}"
    );
    claims[0]
}

fn generated(
    claim: &CompositeEqualityClaim,
) -> &quire_contract_codegen::GeneratedCompositeEqualityClaim {
    match &claim.result {
        ClaimDisposition::Generated(generated) => generated,
        ClaimDisposition::Refused { refusal } => {
            panic!("expected generated, got refusal {refusal:?}")
        }
    }
}

fn refused(claim: &CompositeEqualityClaim) -> &CompositeEqualityRefusal {
    match &claim.result {
        ClaimDisposition::Refused { refusal } => refusal,
        ClaimDisposition::Generated(_) => panic!("expected refusal, got generated"),
    }
}

/// Trace: FR-018-AC-1, TC-029.
#[test]
fn tc_029_ac1_every_item_gets_one_disposition_refused_siblings_unchanged() {
    let package = corpus_package().admit();
    let items = vec![
        item(
            E_RECORD,
            EqualityOperatorKind::Equal,
            typed(R_POINT),
            typed(R_POINT),
        ),
        item(
            E_DUP,
            EqualityOperatorKind::Equal,
            typed(R_DUP),
            typed(R_DUP),
        ),
    ];
    let oracles = generate(&package, &items);
    assert_eq!(oracles.claim_map.items.len(), 2);
    let _ = generated(only_claim(&oracles, E_RECORD));
    assert!(matches!(
        refused(only_claim(&oracles, E_DUP)),
        CompositeEqualityRefusal::Declaration {
            cause: DeclarationRefusalCause::DuplicateMember { .. },
            ..
        }
    ));
    let lib = contents(&oracles, "src/lib.rs");
    assert_eq!(
        lib.matches("pub fn oracle_").count(),
        1,
        "the refused item must contribute no oracle function"
    );
    assert_eq!(
        lib.matches("pub fn environment_").count(),
        1,
        "the refused item must contribute no environment constructor"
    );
}

/// Trace: FR-018-AC-3, TC-029.
#[test]
fn tc_029_ac3_claim_descriptor_matches_the_request() {
    // Left and right carry different source types (a bounded-integer-to-integer
    // conversion on the left only) so a descriptor-recording bug that swaps or
    // drops an operand side is observable, not masked by a symmetric request.
    // E_CONV's own body agrees with exactly this descriptor (its natural
    // usage elsewhere in this module and in the agreement test) -- since
    // codegen#82, an arbitrary node id can no longer stand in for one whose
    // body disagrees with the requested descriptor.
    let package = corpus_package().admit();
    let left = converted(T_INTEGER_BOUNDED, T_INTEGER);
    let right = typed(T_INTEGER);
    let requested = item(
        E_CONV,
        EqualityOperatorKind::Equal,
        left.clone(),
        right.clone(),
    );
    let oracles = generate(&package, std::slice::from_ref(&requested));
    let claim = generated(only_claim(&oracles, E_CONV));
    assert_eq!(claim.descriptor.operator, EqualityOperatorKind::Equal);
    assert_eq!(claim.descriptor.left_source_type, left.source_type);
    assert_eq!(
        claim.descriptor.left_conversion_target,
        left.conversion_target
    );
    assert_eq!(claim.descriptor.right_source_type, right.source_type);
    assert_eq!(
        claim.descriptor.right_conversion_target,
        right.conversion_target
    );
}

/// Trace: FR-018-AC-4, TC-029.
#[test]
fn tc_029_ac4_schedule_matches_checked_equality_for_every_shape() {
    let package = corpus_package().admit();
    let items = vec![
        item(
            E_RECORD,
            EqualityOperatorKind::Equal,
            typed(R_POINT),
            typed(R_POINT),
        ),
        item(
            E_TUPLE,
            EqualityOperatorKind::Equal,
            typed(TUP_PAIR),
            typed(TUP_PAIR),
        ),
        item(
            E_OPTION,
            EqualityOperatorKind::Equal,
            typed(OPT_INT),
            typed(OPT_INT),
        ),
        item(
            E_COLLECTION,
            EqualityOperatorKind::Equal,
            typed(SEQ_INT),
            typed(SEQ_INT),
        ),
        item(
            E_TEXT,
            EqualityOperatorKind::Equal,
            typed(T_TEXT),
            typed(T_TEXT),
        ),
        item(
            E_ENUM,
            EqualityOperatorKind::Equal,
            EqualityOperandDescriptor::typed(enum_type_id()),
            EqualityOperandDescriptor::typed(enum_type_id()),
        ),
    ];
    let oracles = generate(&package, &items);
    assert_eq!(
        generated(only_claim(&oracles, E_RECORD)).schedule,
        RecordedSchedule::Plan
    );
    assert_eq!(
        generated(only_claim(&oracles, E_TUPLE)).schedule,
        RecordedSchedule::Plan
    );
    assert_eq!(
        generated(only_claim(&oracles, E_OPTION)).schedule,
        RecordedSchedule::Plan
    );
    assert_eq!(
        generated(only_claim(&oracles, E_COLLECTION)).schedule,
        RecordedSchedule::Plan
    );
    assert_eq!(
        generated(only_claim(&oracles, E_TEXT)).schedule,
        RecordedSchedule::Text
    );
    assert_eq!(
        generated(only_claim(&oracles, E_ENUM)).schedule,
        RecordedSchedule::Enum
    );
    // Quantity is out of scope for this generator (module doc, disclosed):
    // FR-018's Inputs vocabulary excludes dimension/unit nodes, so no vector
    // here exercises `EqualitySchedule::Quantity`.
}

/// Trace: FR-018-AC-5, TC-029.
#[test]
fn tc_029_ac5_a_disallowed_conversion_refuses_at_generation_time() {
    let package = corpus_package().admit();
    let requested = item(
        E_BAD_CONVERT,
        EqualityOperatorKind::Equal,
        converted(T_TEXT, T_INTEGER),
        typed(T_TEXT),
    );
    let oracles = generate(&package, std::slice::from_ref(&requested));
    assert!(matches!(
        refused(only_claim(&oracles, E_BAD_CONVERT)),
        CompositeEqualityRefusal::IllTyped {
            cause: IllTypedCauseKind::TypeMismatch
        }
    ));
    let lib = contents(&oracles, "src/lib.rs");
    assert_eq!(lib.matches("pub fn oracle_").count(), 0);
}

/// Trace: FR-018 Behavior ("disagrees with its descriptor's arity or
/// operand types"), TC-029 step 1 ("a descriptor disagreeing with its
/// operand types"), codegen#82.
///
/// `E_OPERAND_MISMATCH`'s body names both operands `T_INTEGER`
/// (`package::binary_body(T_INTEGER, T_INTEGER)`). A descriptor naming a
/// different type for either position must refuse before generation --
/// checked left, then right, so the two requests below exercise both
/// positions and each reports the position that actually disagreed.
#[test]
fn tc_029_a_body_operand_disagreeing_with_the_descriptor_refuses_before_generation() {
    let package = corpus_package().admit();

    // Both positions disagree: the refusal names position 0 (left), checked
    // first.
    let both_wrong = item(
        E_OPERAND_MISMATCH,
        EqualityOperatorKind::Equal,
        typed(T_TEXT),
        typed(T_TEXT),
    );
    let oracles = generate(&package, std::slice::from_ref(&both_wrong));
    match refused(only_claim(&oracles, E_OPERAND_MISMATCH)) {
        CompositeEqualityRefusal::OperandTypeMismatch {
            position,
            expected,
            found,
        } => {
            assert_eq!(*position, 0);
            assert_eq!(*expected, code_id(T_TEXT));
            assert_eq!(*found, Some(code_id(T_INTEGER)));
        }
        other => panic!("expected OperandTypeMismatch, got {other:?}"),
    }
    let lib = contents(&oracles, "src/lib.rs");
    assert_eq!(
        lib.matches("pub fn oracle_").count(),
        0,
        "a body/descriptor disagreement must contribute no oracle function"
    );

    // Only the right position disagrees: the refusal names position 1, not
    // position 0 -- proving the check inspects both positions rather than
    // stopping unconditionally at the first.
    let right_wrong = item(
        E_OPERAND_MISMATCH,
        EqualityOperatorKind::Equal,
        typed(T_INTEGER),
        typed(T_TEXT),
    );
    let oracles = generate(&package, std::slice::from_ref(&right_wrong));
    match refused(only_claim(&oracles, E_OPERAND_MISMATCH)) {
        CompositeEqualityRefusal::OperandTypeMismatch {
            position,
            expected,
            found,
        } => {
            assert_eq!(*position, 1);
            assert_eq!(*expected, code_id(T_TEXT));
            assert_eq!(*found, Some(code_id(T_INTEGER)));
        }
        other => panic!("expected OperandTypeMismatch, got {other:?}"),
    }

    // A request whose descriptor agrees with the body at both positions
    // generates: proves the check is a genuine agreement test, not an
    // unconditional refusal of this node id.
    let agrees = item(
        E_OPERAND_MISMATCH,
        EqualityOperatorKind::Equal,
        typed(T_INTEGER),
        typed(T_INTEGER),
    );
    let oracles = generate(&package, std::slice::from_ref(&agrees));
    let _ = generated(only_claim(&oracles, E_OPERAND_MISMATCH));
}

/// Trace: FR-018-AC-6, TC-029.
#[test]
fn tc_029_ac6_ieee_at_any_depth_is_operator_ineligible() {
    let package = corpus_package().admit();
    let requested = item(
        E_NESTED_IEEE,
        EqualityOperatorKind::Equal,
        typed(SEQ_R_FLOAT),
        typed(SEQ_R_FLOAT),
    );
    let oracles = generate(&package, std::slice::from_ref(&requested));
    assert!(matches!(
        refused(only_claim(&oracles, E_NESTED_IEEE)),
        CompositeEqualityRefusal::IllTyped {
            cause: IllTypedCauseKind::OperatorIneligible
        }
    ));

    // Reconstruct SEQ_R_FLOAT (a sequence of R_FLOAT { f: Float64 }) exactly
    // as the runtime environment sees it, and confirm the refusal above
    // agrees with an independent, direct `contains_ieee` call over that
    // reconstructed type at both its depths (the record field, and the
    // sequence wrapping it) rather than merely riding on the same code path.
    let float_record_key = NodeKey::from_hex(&key(R_FLOAT)).unwrap();
    let environment = TypeEnvironment::new(
        vec![CompositeDeclaration::new(
            float_record_key,
            "R_FLOAT",
            CompositeShape::Record(vec![FieldDeclaration::new(
                "f",
                ValueType::Float(IeeeWidth::Binary64),
                Presence::Required,
            )]),
        )],
        core::iter::empty::<ObjectTypeDeclaration>(),
    )
    .unwrap();
    let record_type = ValueType::Composite(float_record_key);
    let sequence_type = ValueType::Collection(Box::new(CollectionType::new(
        CollectionKind::Sequence,
        record_type.clone(),
        CardinalityBound::new(0, 8).unwrap(),
    )));
    // The leaf itself: `ValueType::Float(_) => return true` is `contains_ieee`'s
    // base case, unasserted by the record/sequence checks above (both of those
    // pass by *reaching* an IEEE leaf through a composite, never by naming the
    // leaf type directly).
    assert!(environment.contains_ieee(&ValueType::Float(IeeeWidth::Binary64)));
    assert!(environment.contains_ieee(&record_type));
    assert!(environment.contains_ieee(&sequence_type));

    // Negative control: an unconditional `true` (or a check that never visits
    // a field) would pass every assertion above. A structurally identical
    // record and sequence, but with an `Integer` field instead of `Float64`,
    // must report `false` at both depths.
    let not_float_record_key = NodeKey::from_hex(&key(R_NOT_FLOAT)).unwrap();
    let not_ieee_environment = TypeEnvironment::new(
        vec![CompositeDeclaration::new(
            not_float_record_key,
            "R_NOT_FLOAT",
            CompositeShape::Record(vec![FieldDeclaration::new(
                "f",
                ValueType::Integer,
                Presence::Required,
            )]),
        )],
        core::iter::empty::<ObjectTypeDeclaration>(),
    )
    .unwrap();
    let not_float_record_type = ValueType::Composite(not_float_record_key);
    let not_float_sequence_type = ValueType::Collection(Box::new(CollectionType::new(
        CollectionKind::Sequence,
        not_float_record_type.clone(),
        CardinalityBound::new(0, 8).unwrap(),
    )));
    assert!(!not_ieee_environment.contains_ieee(&ValueType::Integer));
    assert!(!not_ieee_environment.contains_ieee(&not_float_record_type));
    assert!(!not_ieee_environment.contains_ieee(&not_float_sequence_type));
}

/// Trace: FR-018-AC-7, TC-029.
#[test]
fn tc_029_ac7_each_family_gets_its_own_distinct_blocker() {
    let package = corpus_package().admit();
    let items = vec![
        item(
            M_BARE,
            EqualityOperatorKind::Equal,
            typed(T_INTEGER),
            typed(T_INTEGER),
        ),
        item(
            F_BARE,
            EqualityOperatorKind::Equal,
            typed(T_INTEGER),
            typed(T_INTEGER),
        ),
        item(
            S_BARE,
            EqualityOperatorKind::Equal,
            typed(T_INTEGER),
            typed(T_INTEGER),
        ),
        item(
            T_BARE,
            EqualityOperatorKind::Equal,
            typed(T_INTEGER),
            typed(T_INTEGER),
        ),
        item(
            E_REFERENCE,
            EqualityOperatorKind::Equal,
            typed(REF_TYPE),
            typed(T_INTEGER),
        ),
        item(
            E_CALL,
            EqualityOperatorKind::Equal,
            typed(T_INTEGER),
            typed(T_INTEGER),
        ),
    ];
    let oracles = generate(&package, &items);

    let blocker_of = |node: u32| match refused(only_claim(&oracles, node)) {
        CompositeEqualityRefusal::BlockedOnUpstream { issue, .. } => *issue,
        other => panic!("node {node}: expected BlockedOnUpstream, got {other:?}"),
    };
    assert_eq!(blocker_of(M_BARE), UpstreamBlocker::QuireSpecLanguage120);
    assert_eq!(blocker_of(S_BARE), UpstreamBlocker::QuireSpecLanguage121);
    assert_eq!(blocker_of(T_BARE), UpstreamBlocker::QuireSpecLanguage121);
    assert_eq!(blocker_of(F_BARE), UpstreamBlocker::QuireContractRuntime34);
    assert_eq!(blocker_of(E_CALL), UpstreamBlocker::QuireContractRuntime34);
    assert_eq!(
        blocker_of(E_REFERENCE),
        UpstreamBlocker::QuireSpecLanguage120
    );
    for node in [M_BARE, F_BARE, S_BARE, T_BARE, E_REFERENCE, E_CALL] {
        assert!(claims_for(&oracles, node)
            .iter()
            .all(|claim| matches!(claim.result, ClaimDisposition::Refused { .. })));
    }
}

/// Trace: FR-018-AC-8, TC-029 (generation-time half; the `check_type` guard
/// half is `tc_029_ac8_check_type_guards_the_oracle` in the agreement test).
#[test]
fn tc_029_ac8_a_duplicate_field_is_refused_with_its_declaration_cause() {
    let package = corpus_package().admit();
    let requested = item(
        E_DUP,
        EqualityOperatorKind::Equal,
        typed(R_DUP),
        typed(R_DUP),
    );
    let oracles = generate(&package, std::slice::from_ref(&requested));
    match refused(only_claim(&oracles, E_DUP)) {
        CompositeEqualityRefusal::Declaration {
            cause: DeclarationRefusalCause::DuplicateMember { name },
            ..
        } => assert_eq!(name, "x"),
        other => panic!("expected DuplicateMember, got {other:?}"),
    }
}

/// Trace: FR-018-AC-10, TC-029.
#[test]
fn tc_029_ac10_bytes_are_deterministic_across_request_permutations() {
    let package = corpus_package().admit();
    let mut items = golden_items();
    let forward = generate(&package, &items);
    items.reverse();
    let reversed = generate(&package, &items);
    assert_eq!(
        serde_json::to_string(&forward.claim_map).unwrap(),
        serde_json::to_string(&reversed.claim_map).unwrap(),
    );
    assert_eq!(
        contents(&forward, "src/lib.rs"),
        contents(&reversed, "src/lib.rs")
    );

    // Two descriptors over one node id (E_RECORD `=` and `!=`) are both
    // generated, in either request order; they are not a duplicate.
    for oracles in [&forward, &reversed] {
        assert_eq!(claims_for(oracles, E_RECORD).len(), 2);
        for claim in claims_for(oracles, E_RECORD) {
            let _ = generated(claim);
        }
    }
}

/// Trace: FR-018-AC-10, TC-029.
#[test]
fn tc_029_ac10_claim_map_entries_ascend_by_the_descriptor_key() {
    let package = corpus_package().admit();
    let oracles = generate(&package, &golden_items());
    let keys: Vec<(String, u8)> = oracles
        .claim_map
        .items
        .iter()
        .map(|claim| {
            let operator = match &claim.result {
                ClaimDisposition::Generated(g) => g.descriptor.operator as u8,
                ClaimDisposition::Refused { .. } => 0,
            };
            (claim.node_id.digest.to_string(), operator)
        })
        .collect();
    let mut sorted = keys.clone();
    sorted.sort();
    assert_eq!(
        keys, sorted,
        "claim map entries must already ascend by descriptor key"
    );
}

/// Trace: FR-018-AC-10, TC-029: the committed golden crate.
#[test]
fn tc_029_ac10_generation_matches_the_committed_golden_files() {
    let oracles = generate(&corpus_package().admit(), &golden_items());
    let fixtures =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/composite_equality");
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
}

/// Trace: FR-018-AC-11, TC-029 (generation-time half; complementary outcomes
/// are `tc_029_ac11_complementary_outcomes` in the agreement test).
#[test]
fn tc_029_ac11_two_operators_over_one_node_get_distinct_symbols_and_are_caller_declared() {
    let package = corpus_package().admit();
    let oracles = generate(&package, &golden_items());
    let claims = claims_for(&oracles, E_RECORD);
    assert_eq!(claims.len(), 2);
    let equal = claims
        .iter()
        .find(|c| generated(c).descriptor.operator == EqualityOperatorKind::Equal)
        .unwrap();
    let not_equal = claims
        .iter()
        .find(|c| generated(c).descriptor.operator == EqualityOperatorKind::NotEqual)
        .unwrap();
    assert_ne!(
        generated(equal).oracle_symbol,
        generated(not_equal).oracle_symbol
    );
    assert_ne!(
        generated(equal).environment_symbol,
        generated(not_equal).environment_symbol
    );
    // The symbol itself is that digest (TC-029 step 7), not merely distinct
    // from its sibling: a symbol derived from a request ordinal plus the
    // operator would also pass the two `assert_ne!`s above without being
    // this digest.
    for claim in [equal, not_equal] {
        let descriptor = &generated(claim).descriptor;
        let expected = format!(
            "oracle_{}",
            recomputed_symbol_digest(
                &claim.node_id,
                descriptor.operator,
                &descriptor.left_source_type,
                descriptor.left_conversion_target.as_ref(),
                &descriptor.right_source_type,
                descriptor.right_conversion_target.as_ref(),
            )
        );
        assert_eq!(generated(claim).oracle_symbol, expected);
    }
    for claim in &claims {
        assert_eq!(
            claim.operation.provenance,
            quire_contract_codegen::CompositeOperationProvenance::CallerDeclared {
                blocked_on: UpstreamBlocker::OperationIdentityNotConsumed
            }
        );
    }
    assert!(oracles
        .claim_map
        .blocked
        .contains(&UpstreamBlocker::OperationIdentityNotConsumed));

    let lib = contents(&oracles, "src/lib.rs");
    let mut functions: Vec<&str> = lib
        .lines()
        .filter(|line| {
            line.starts_with("pub fn oracle_") || line.starts_with("pub fn environment_")
        })
        .collect();
    let total = functions.len();
    functions.sort();
    functions.dedup();
    assert_eq!(
        functions.len(),
        total,
        "every generated symbol line must be unique"
    );
}

/// Trace: FR-018-AC-12, TC-029.
///
/// The `integer()` reconstruction helper is emitted only when some rendered
/// bound actually calls it (a reachable bounded `Int` or `Decimal`).
/// Unconditional emission would be unused, and hence dead code, for a
/// request that reaches neither — which would fail this same test's
/// `-D warnings` build below for that request, even though the full corpus
/// (which does reach one, via TUP_PAIR) never observes it.
#[test]
fn tc_029_ac12_integer_helper_omitted_when_unreached() {
    let package = corpus_package().admit();
    let request = [item(
        E_TEXT,
        EqualityOperatorKind::Equal,
        typed(T_TEXT),
        typed(T_TEXT),
    )];
    let oracles = generate(&package, &request);
    let lib = contents(&oracles, "src/lib.rs");
    assert!(
        !lib.contains("fn integer("),
        "no reachable bounded Int/Decimal in this request; `integer()` must be omitted:\n{lib}"
    );
}

/// Trace: FR-018-AC-12, TC-029.
#[test]
fn tc_029_ac12_manifest_is_unpublished_pinned_and_charge_free() {
    let oracles = generate(&corpus_package().admit(), &golden_items());
    let manifest = contents(&oracles, "Cargo.toml");
    assert!(manifest.contains(&format!("name = \"{COMPOSITE_EQUALITY_CRATE_NAME}\"")));
    assert!(manifest.contains("publish = false"));
    assert!(manifest.contains(&format!(
        "rev = \"{}\"",
        quire_contract_codegen::RUNTIME_REVISION
    )));
    assert!(manifest.contains("features = [\"exact\"]"));

    let lib = contents(&oracles, "src/lib.rs");
    for forbidden in [
        "ChargePoint",
        "charge(",
        "pair_events",
        "EqualityPlan",
        "WorkLimits",
    ] {
        assert!(
            !lib.contains(forbidden),
            "generated source contains `{forbidden}`"
        );
    }

    // The oracle function bodies specifically must contain none of the
    // panic surface the FR forbids; the environment/composites helpers are
    // allowed the disclosed `.expect(...)` on already-validated literals.
    for symbol_line in lib
        .lines()
        .filter(|line| line.starts_with("pub fn oracle_"))
    {
        let start = lib.find(symbol_line).unwrap();
        let body_start = lib[start..].find("{\n").map(|i| start + i).unwrap();
        let end = lib[body_start..]
            .find("\n}\n")
            .map(|i| body_start + i)
            .unwrap();
        let body = &lib[body_start..end];
        for forbidden in [
            "unwrap(",
            "expect(",
            "panic!",
            "unreachable!",
            "todo!",
            "unsafe",
        ] {
            assert!(
                !body.contains(forbidden),
                "oracle function body contains `{forbidden}`:\n{body}"
            );
        }
    }

    let directory = TemporaryDirectory::new("quire-composite-equality-oracles");
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
        "generated crate did not compile: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

/// Trace: FR-018-AC-13, TC-029.
#[test]
fn tc_029_ac13_declaration_keys_are_the_reached_v2_node_ids() {
    let package = corpus_package().admit();
    let items = vec![
        item(
            E_RECORD,
            EqualityOperatorKind::Equal,
            typed(R_POINT),
            typed(R_POINT),
        ),
        item(
            E_PAIR_OF_POINTS,
            EqualityOperatorKind::Equal,
            typed(R_PAIR_OF_POINTS),
            typed(R_PAIR_OF_POINTS),
        ),
    ];
    let oracles = generate(&package, &items);

    let single = generated(only_claim(&oracles, E_RECORD));
    assert_eq!(single.declaration_keys, vec![code_id(R_POINT)]);
    assert_eq!(
        single.declaration_runtime_keys,
        vec![key(R_POINT)],
        "declaration_runtime_keys must equal NodeKey::from_hex of declaration_keys"
    );

    let nested = generated(only_claim(&oracles, E_PAIR_OF_POINTS));
    assert_eq!(
        nested.declaration_keys,
        vec![code_id(R_POINT), code_id(R_PAIR_OF_POINTS)],
        "declaration keys ascend by digest domain then digest"
    );
    assert_eq!(
        nested.declaration_runtime_keys,
        vec![key(R_POINT), key(R_PAIR_OF_POINTS)]
    );

    // FR-018-AC-13 also names node id, IR id, package id, source map, claims
    // and the selected schedule. Recompute each independently — never by
    // reading it back off the golden claim-map — using the same public
    // `CheckedPackageV2::lower` the generator calls, but with a
    // self-constructed, maximally permissive profile (`CheckedNodeTag::ALL`)
    // so this recomputation shares no private profile constant with the
    // generator. `ir_id` is a pure digest over the lowered closure's own
    // content (`LOWERED_NODE_PREIMAGE`), so it, `claims` and `source_map`
    // must agree regardless of which permissive profile computed them.
    let profile = CompleteLoweringProfileV2 {
        supported_tags: CheckedNodeTag::ALL.into_iter().collect::<BTreeSet<_>>(),
        require_bounds: false,
        work_limit: 10_000,
    };
    assert_eq!(only_claim(&oracles, E_RECORD).node_id, code_id(E_RECORD));
    assert_eq!(
        only_claim(&oracles, E_PAIR_OF_POINTS).node_id,
        code_id(E_PAIR_OF_POINTS)
    );
    assert_eq!(single.schedule, RecordedSchedule::Plan);
    assert_eq!(nested.schedule, RecordedSchedule::Plan);
    assert_eq!(single.package_id, *package.package_id());
    assert_eq!(nested.package_id, *package.package_id());

    let single_lowering = package.lower(&[code_id(E_RECORD)], &profile);
    assert_eq!(single_lowering.package_id, *package.package_id());
    match &single_lowering.records[..] {
        [CompleteLoweringRecordV2::Lowered { node }] => {
            assert_eq!(
                single.ir_id, node.ir_id,
                "ir_id must equal the independently lowered node's"
            );
            assert_eq!(
                single.claims, node.claims,
                "claims must equal the independently lowered node's"
            );
            assert_eq!(
                single.source_map, node.source_map,
                "source_map must equal the independently lowered node's"
            );
        }
        other => panic!("expected exactly one Lowered record, found {other:?}"),
    }

    let nested_lowering = package.lower(&[code_id(E_PAIR_OF_POINTS)], &profile);
    match &nested_lowering.records[..] {
        [CompleteLoweringRecordV2::Lowered { node }] => {
            assert_eq!(
                nested.ir_id, node.ir_id,
                "ir_id must equal the independently lowered node's"
            );
            assert_eq!(
                nested.claims, node.claims,
                "claims must equal the independently lowered node's"
            );
            assert_eq!(
                nested.source_map, node.source_map,
                "source_map must equal the independently lowered node's"
            );
        }
        other => panic!("expected exactly one Lowered record, found {other:?}"),
    }
}
