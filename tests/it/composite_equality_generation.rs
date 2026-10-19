//! FR-018: composite/structural equality oracle generation over admitted
//! CheckedPackage V2 input.
//!
//! Generation-time coverage only (no execution of generated code): dispositions,
//! refusal reasons, schedules, ordering/determinism, symbol disambiguation,
//! the manifest and declaration keys. Execution-level criteria (AC-2, AC-8's
//! `check_type` guard, AC-9's denial injection, AC-11's complementary
//! outcomes) are covered by `tests/composite_equality_agreement.rs`.

use ix_trace_rs::trace;
use std::{collections::BTreeSet, fs, ops::Deref, path::PathBuf, process::Command};

use crate::common::panic_scan::{non_test_code, panic_tokens_in};
use crate::scratch_crate::{runtime_dependency, seed_lock, TemporaryDirectory};
use quire_contract_codegen::{
    generate_composite_equality_oracles, ClaimDisposition, CompositeEqualityClaim,
    CompositeEqualityItem, CompositeEqualityLimits, CompositeEqualityOracles,
    CompositeEqualityRefusal, ContractRequestPosition, DeclarationRefusalCause,
    EqualityOperandDescriptor, EqualityOperatorKind, IllTypedCauseKind, OracleGenerationError,
    RecordedSchedule, UpstreamBlocker, COMPOSITE_EQUALITY_CRATE_NAME,
};
use quire_contract_model::{
    derive_structural_candidates, CheckedNodeTag, CheckedPackageLimit, CheckedPackageReadLimits,
    CheckedPackageRefusalCause, CheckedPackageRefusalCode, CheckedPackageV2,
    CheckedPackageV2ReadResult, CheckedSemanticNodeV2, CompleteLoweringProfileV2,
    CompleteLoweringRecordV2, ContractDependencies, ContractFeatureRecordV1,
    ContractPackageReadResult, ContractPackageWireV1, StructuralCandidateError,
};
use quire_contract_runtime::exact::{
    CardinalityBound, CollectionKind, CollectionType, CompositeDeclaration, CompositeShape,
    FieldDeclaration, IeeeWidth, NodeKey, ObjectTypeDeclaration, Presence, TypeEnvironment,
    ValueType,
};
use serde_json::json;

// package.rs holds a process-global `application_registry()` static keyed by small integer
// fixture codes that this file and `composite_equality_agreement.rs` each pick independently,
// on the assumption of an isolated registry (verified: centralizing this module produced real
// cross-file code collisions and Mutex-poisoning cascades). Kept duplicated on purpose.
#[allow(clippy::duplicate_mod)]
#[path = "../composite_equality_support/package.rs"]
mod package;

use package::*;

/// Structural equality reaching IEEE is reader-ineligible.
#[trace("FR-037-AC-1", "TC-052")]
#[test]
fn tc_052_nested_ieee_equality_is_refused_by_reader() {
    let (result, wire, ids) = nested_ieee_package().read_resolved();
    let node_id = ids.resolve(&code_id(E_NESTED_IEEE));
    let position = wire["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes")
        .iter()
        .position(|node| node["node_id"]["digest"].as_str() == Some(node_id.digest.as_ref()))
        .expect("nested IEEE equality");
    let CheckedPackageV2ReadResult::Refused(refusal) = result else {
        panic!("nested IEEE equality unexpectedly admitted: {result:?}");
    };
    assert_eq!(refusal.code, CheckedPackageRefusalCode::IllTyped);
    assert_eq!(
        refusal.cause,
        Some(CheckedPackageRefusalCause::OperatorIneligible)
    );
    assert_eq!(
        refusal.path.as_ref().map(|path| path.as_str().to_owned()),
        Some(format!(
            "/semantic_graph/nodes/{position}/body/operation/leaves"
        ))
    );
    assert_eq!(refusal.locus, Some(node_id));
}

/// A sequence body has one element reference.
#[trace("FR-037-AC-1", "TC-052")]
#[test]
fn tc_052_two_reference_sequence_body_is_refused_by_reader() {
    let (_, mut wire, ids) = corpus_package().read_resolved();
    let node_id = ids.resolve(&code_id(SEQ_R_FLOAT));
    let bound = ids.resolve(&code_id(CB_SMALL));
    let nodes = wire["semantic_graph"]["nodes"]
        .as_array_mut()
        .expect("nodes");
    let position = nodes
        .iter()
        .position(|node| node["node_id"]["digest"].as_str() == Some(node_id.digest.as_ref()))
        .expect("sequence");
    nodes[position]["body"]["members"]
        .as_array_mut()
        .expect("members")
        .push(json!({
            "term": "reference",
            "target": {"domain": bound.domain, "digest": bound.digest},
        }));
    let draft: Vec<CheckedSemanticNodeV2> =
        serde_json::from_value(wire["semantic_graph"]["nodes"].clone()).expect("draft nodes");
    assert!(matches!(
        derive_structural_candidates(
            &draft,
            quire_contract_model::CheckedPackageReadLimits::bounded().bytes,
            quire_contract_model::CheckedPackageReadLimits::bounded().work,
        ),
        Err(StructuralCandidateError::InvalidClosedShape { position: refused, node_id: invalid })
            if refused == position && invalid == node_id
    ));
    let result = read_mutated_wire(wire);
    let CheckedPackageV2ReadResult::Refused(refusal) = result else {
        panic!("two-reference sequence unexpectedly admitted: {result:?}");
    };
    assert_eq!(refusal.code, CheckedPackageRefusalCode::InvalidPackage);
    assert_eq!(
        refusal.cause,
        Some(CheckedPackageRefusalCause::StaleNodeKey)
    );
    assert_eq!(
        refusal.path.as_ref().map(|path| path.as_str().to_owned()),
        Some(format!("/semantic_graph/nodes/{position}/node_id"))
    );
    assert_eq!(refusal.locus, Some(node_id));
}

struct FixturePackage {
    checked: CheckedPackageV2,
    ids: FixtureIds,
}

impl FixturePackage {
    fn key(&self, code: u32) -> quire_contract_model::CheckedNodeId {
        self.ids.resolve(&code_id(code))
    }
}

impl Deref for FixturePackage {
    type Target = CheckedPackageV2;

    fn deref(&self) -> &Self::Target {
        &self.checked
    }
}

fn admit(builder: PackageBuilder) -> FixturePackage {
    let (checked, ids) = builder.admit_resolved();
    FixturePackage { checked, ids }
}

fn admit_with(
    builder: &PackageBuilder,
    limits: quire_contract_model::CheckedPackageReadLimits,
) -> FixturePackage {
    let (checked, ids) = builder.admit_with_resolved(limits);
    FixturePackage { checked, ids }
}

#[derive(Debug, Eq, PartialEq)]
pub(super) struct FixtureOracles {
    output: CompositeEqualityOracles,
    ids: FixtureIds,
}

impl FixtureOracles {
    fn key(&self, code: u32) -> quire_contract_model::CheckedNodeId {
        self.ids.resolve(&code_id(code))
    }

    fn runtime_key(&self, code: u32) -> String {
        self.key(code).digest.to_string()
    }
}

impl Deref for FixtureOracles {
    type Target = CompositeEqualityOracles;

    fn deref(&self) -> &Self::Target {
        &self.output
    }
}

fn generate(package: &FixturePackage, items: &[CompositeEqualityItem]) -> FixtureOracles {
    generate_with_context(package, items, &[])
}

fn generate_with_context(
    package: &FixturePackage,
    items: &[CompositeEqualityItem],
    extra_context: &[quire_contract_model::CheckedNodeId],
) -> FixtureOracles {
    let items = items
        .iter()
        .cloned()
        .map(|item| package.ids.resolve_item(item))
        .collect::<Vec<_>>();
    let mut requested = items
        .iter()
        .map(|item| item.node_id.clone())
        .collect::<Vec<_>>();
    // The complete lowering request is identical under descriptor permutation;
    // positions below still map each original item to its own accounting root.
    requested.sort();
    // Bounds are separate checked roots, not dependencies of the expression or
    // composite type. Select only bounds of the caller's operand types; unrelated
    // recursive or malformed fixture nodes must stay outside this FR-330 package.
    let selected_types = items
        .iter()
        .flat_map(|item| {
            [
                Some(&item.left.source_type),
                item.left.conversion_target.as_ref(),
                Some(&item.right.source_type),
                item.right.conversion_target.as_ref(),
            ]
            .into_iter()
            .flatten()
        })
        .collect::<BTreeSet<_>>();
    let graph = &package.checked.graph().nodes;
    let selected_groups = graph
        .iter()
        .filter(|node| selected_types.contains(&node.node_id))
        .filter_map(|node| node.recursion_group.as_ref())
        .collect::<BTreeSet<_>>();
    let mut context_roots = graph
        .iter()
        .filter(|node| {
            (CheckedNodeTag::from_wire(&node.node_tag) == Some(CheckedNodeTag::BoundedDomain)
                && selected_types.contains(&node.semantic_type))
                || node
                    .recursion_group
                    .as_ref()
                    .is_some_and(|group| selected_groups.contains(group))
        })
        .map(|node| node.node_id.clone())
        .collect::<Vec<_>>();
    for node_id in extra_context {
        if !context_roots.contains(node_id) {
            context_roots.push(node_id.clone());
        }
    }
    let lowering_roots = requested
        .iter()
        .chain(&context_roots)
        .cloned()
        .collect::<Vec<_>>();
    let lowered = package.checked.lower(&lowering_roots, &admitted_profile());
    let admitted = match ContractPackageWireV1::read(
        lowered.package.canonical_bytes().expect("FR-330 bytes"),
        CheckedPackageReadLimits::bounded(),
        &ContractDependencies::new(),
    ) {
        ContractPackageReadResult::Admitted(package) => package,
        other => panic!("IR must admit its package: {other:?}"),
    };
    let mut available = std::collections::BTreeMap::new();
    for (position, node_id) in requested.iter().enumerate() {
        available
            .entry(node_id.clone())
            .or_insert_with(std::collections::VecDeque::new)
            .push_back(position);
    }
    let positions = items
        .iter()
        .map(|item| {
            available
                .get_mut(&item.node_id)
                .and_then(std::collections::VecDeque::pop_front)
                .expect("the canonical request retains every selected equality item")
        })
        .collect::<Vec<_>>();
    let mut roots = requested
        .iter()
        .cloned()
        .map(|node_id| ContractRequestPosition::DriverItem {
            node_id,
            result_selection: quire_contract_codegen::ContractDriverResultSelection::NotApplicable,
        })
        .collect::<Vec<_>>();
    roots.extend(
        context_roots
            .into_iter()
            .map(|node_id| ContractRequestPosition::ContextRoot { node_id }),
    );
    let output = generate_composite_equality_oracles(
        &admitted,
        &roots,
        &positions,
        &items,
        Default::default(),
    )
    .expect("generation succeeds");
    FixtureOracles {
        output,
        ids: package.ids.clone(),
    }
}

fn admitted_profile() -> CompleteLoweringProfileV2 {
    CompleteLoweringProfileV2 {
        reference: quire_contract_model::LoweringProfileRef {
            authority: "agent-ix".into(),
            identity: "quire.codegen.composite-equality/v1".into(),
        },
        supported_tags: [
            CheckedNodeTag::ScalarType,
            CheckedNodeTag::CompositeType,
            CheckedNodeTag::BoundedDomain,
            CheckedNodeTag::Value,
            CheckedNodeTag::Expression,
            CheckedNodeTag::Claim,
            CheckedNodeTag::Correspondence,
        ]
        .into(),
        require_bounds: false,
        work_limit: 65_536,
    }
}

#[trace("FR-018-AC-31", "TC-029")]
#[test]
fn invalid_type_resolution_work_limit_refuses_before_lowering() {
    let package = admit(corpus_package());
    let lowered = package.checked.lower(&[], &admitted_profile());
    let admitted = match ContractPackageWireV1::read(
        lowered.package.canonical_bytes().expect("FR-330 bytes"),
        CheckedPackageReadLimits::bounded(),
        &ContractDependencies::new(),
    ) {
        ContractPackageReadResult::Admitted(package) => package,
        other => panic!("IR must admit its package: {other:?}"),
    };
    let result = generate_composite_equality_oracles(
        &admitted,
        &[],
        &[],
        &[],
        CompositeEqualityLimits {
            type_resolution_work_units: u64::MAX,
            ..CompositeEqualityLimits::default()
        },
    );
    assert_eq!(
        result,
        Err(OracleGenerationError::InvalidTypeResolutionWorkLimit)
    );
}

/// The direct case keeps its claim identity and typed cause, schedules no source/location, and leaves a healthy equality sibling byte-identical.
#[trace("FR-018-AC-32", "TC-029")]
#[test]
fn tc_029_ac32_direct_control_case_refusal_preserves_healthy_equality_sibling() {
    use quire_contract_model::LoweringProfileRef;
    use serde_json::json;
    use std::{fs, path::PathBuf};

    const BOOLEAN: u32 = 930_101;
    const EQUALITY: u32 = 930_102;
    let qspec = PathBuf::from(std::env::var_os("QSPEC_DIR").expect("published QSpec checkout"));
    let wire: serde_json::Value = serde_json::from_slice(
        &fs::read(qspec.join("proposals/checked-package-v2/fixtures/positive-union-nodes.json"))
            .expect("read authoritative QSpec fixture"),
    )
    .expect("QSpec fixture JSON");
    let nodes = wire["semantic_graph"]["nodes"].as_array().unwrap();
    let node_ref = |digest: &str| json!({"domain": NODE_DOMAIN, "digest": digest});
    let case_id: quire_contract_model::CheckedNodeId = serde_json::from_value(
        nodes
            .iter()
            .find(|node| node["semantic_form"] == "case")
            .unwrap()["node_id"]
            .clone(),
    )
    .unwrap();
    let integer_type = nodes
        .iter()
        .find(|node| node["node_tag"] == "scalar_type" && node["semantic_form"] == "integer")
        .unwrap()["node_id"]["digest"]
        .as_str()
        .unwrap()
        .to_owned();
    let integer_literals: Vec<String> = nodes
        .iter()
        .filter(|node| node["node_tag"] == "value" && node["semantic_form"] == "literal")
        .take(2)
        .map(|node| node["node_id"]["digest"].as_str().unwrap().to_owned())
        .collect();
    assert_eq!(integer_literals.len(), 2);

    let mut builder = PackageBuilder::from_wire(wire);
    builder
        .code(
            BOOLEAN,
            "scalar_type",
            "boolean",
            BOOLEAN,
            aggregate(vec![]),
        )
        .application_node_tagged(
            EQUALITY,
            "expression",
            "binary",
            &key(BOOLEAN),
            json!({
                "term":"application", "operator":"binary",
                "operation":{"identity":"quire.op.integer.eq","laws":[],"mode":null,"member":null,"leaves":[]},
                "result_type":node_ref(&key(BOOLEAN)),
                "arguments":integer_literals.iter().map(|digest| json!({"term":"reference","target":node_ref(digest)})).collect::<Vec<_>>(),
            }),
        );
    let (admission, _, ids) = builder.read_resolved();
    let package = match admission {
        CheckedPackageV2ReadResult::Admitted(package) => *package,
        other => panic!("extended QSpec fixture should admit: {other:?}"),
    };
    let case_id = ids.resolve(&case_id);
    let equality_id = ids.resolve(&code_id(EQUALITY));
    let integer_id = ids.resolve(&id(&integer_type));
    let lowered = package.lower(
        &[case_id.clone(), equality_id.clone()],
        &CompleteLoweringProfileV2 {
            reference: LoweringProfileRef {
                authority: "agent-ix".into(),
                identity: "quire.codegen.composite-equality/v1".into(),
            },
            supported_tags: BTreeSet::from([
                CheckedNodeTag::ScalarType,
                CheckedNodeTag::CompositeType,
                CheckedNodeTag::BoundedDomain,
                CheckedNodeTag::Value,
                CheckedNodeTag::Expression,
                CheckedNodeTag::Claim,
                CheckedNodeTag::Correspondence,
            ]),
            require_bounds: false,
            work_limit: 65_536,
        },
    );
    let contract = match ContractPackageWireV1::read(
        lowered.package.canonical_bytes().unwrap(),
        CheckedPackageReadLimits::bounded(),
        &ContractDependencies::new(),
    ) {
        ContractPackageReadResult::Admitted(package) => *package,
        other => panic!("lowered QSpec sibling request should admit: {other:?}"),
    };
    let case_item = CompositeEqualityItem {
        node_id: case_id.clone(),
        operator: EqualityOperatorKind::Equal,
        left: EqualityOperandDescriptor::typed(integer_id.clone()),
        right: EqualityOperandDescriptor::typed(integer_id.clone()),
    };
    let equality_item = CompositeEqualityItem {
        node_id: equality_id.clone(),
        operator: EqualityOperatorKind::Equal,
        left: EqualityOperandDescriptor::typed(integer_id.clone()),
        right: EqualityOperandDescriptor::typed(integer_id),
    };
    let requests =
        [case_id.clone(), equality_id.clone()].map(|node_id| ContractRequestPosition::DriverItem {
            node_id,
            result_selection: quire_contract_codegen::ContractDriverResultSelection::NotApplicable,
        });
    let combined = generate_composite_equality_oracles(
        &contract,
        &requests,
        &[0, 1],
        &[case_item, equality_item.clone()],
        Default::default(),
    )
    .unwrap();
    let equality_only = generate_composite_equality_oracles(
        &contract,
        &requests,
        &[1],
        &[equality_item],
        Default::default(),
    )
    .unwrap();

    let refused = combined
        .claim_map
        .items
        .iter()
        .find(|claim| claim.node_id == case_id)
        .expect("case claim retained");
    assert!(matches!(
        &refused.result,
        ClaimDisposition::Refused {
            refusal: CompositeEqualityRefusal::Unsupported { unsupported_node_id, node_tag: "expression" }
        } if unsupported_node_id == &case_id
    ));
    let generated = combined
        .claim_map
        .items
        .iter()
        .find(|claim| claim.node_id == equality_id)
        .expect("healthy equality claim retained");
    assert!(matches!(&generated.result, ClaimDisposition::Generated(_)));
    let equality_only_claim = equality_only
        .claim_map
        .items
        .iter()
        .find(|claim| claim.node_id == equality_id)
        .expect("standalone healthy equality claim");
    assert_eq!(generated.result, equality_only_claim.result);
    assert_eq!(
        contents(&combined, "src/lib.rs"),
        contents(&equality_only, "src/lib.rs")
    );
    assert!(!contents(&combined, "src/lib.rs").contains(case_id.digest.as_ref()));
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
fn claims_for(oracles: &FixtureOracles, node: u32) -> Vec<&CompositeEqualityClaim> {
    let requested = oracles.key(node);
    oracles
        .claim_map
        .items
        .iter()
        .filter(|claim| claim.node_id == requested)
        .collect()
}

fn only_claim(oracles: &FixtureOracles, node: u32) -> &CompositeEqualityClaim {
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

#[trace("FR-018-AC-24", "TC-029")]
#[test]
fn tc_029_ac24_recursive_list_and_tree_items_emit_one_oracle_each() {
    let list = generate(
        &admit(corpus_package()),
        &[item(
            E_SELF,
            EqualityOperatorKind::Equal,
            typed(R_SELF),
            typed(R_SELF),
        )],
    );
    let tree = generate(
        &admit(recursive_tree_package()),
        &[item(
            E_TREE_CYCLE,
            EqualityOperatorKind::Equal,
            typed(R_TREE_CYCLE),
            typed(R_TREE_CYCLE),
        )],
    );
    for (oracles, expression, record) in
        [(&list, E_SELF, R_SELF), (&tree, E_TREE_CYCLE, R_TREE_CYCLE)]
    {
        let claim = generated(only_claim(oracles, expression));
        assert_eq!(claim.declaration_keys, vec![oracles.key(record)]);
        let lib = contents(oracles, "src/lib.rs");
        assert_eq!(lib.matches("pub fn oracle_").count(), 1);
        assert_eq!(lib.matches("pub fn environment_").count(), 1);
        assert!(lib.contains(&claim.oracle_symbol));
    }
}

#[trace("FR-018-AC-24", "FR-018-AC-26", "FR-018-AC-27", "TC-029")]
#[test]
fn tc_029_public_qsl_recursive_items_generate_through_reader() {
    let (builder, list, tree) = qsl_recursive_items_package();
    assert_recursive_item_generation(builder, list, tree);
}

fn assert_recursive_item_generation(
    builder: PackageBuilder,
    list: quire_contract_model::CheckedNodeId,
    tree: quire_contract_model::CheckedNodeId,
) {
    fn emitted_key(node: &quire_contract_model::CheckedNodeId) -> String {
        let key = NodeKey::from_hex(&node.digest).expect("a checked node digest is a runtime key");
        let bytes = key
            .as_bytes()
            .iter()
            .map(u8::to_string)
            .collect::<Vec<_>>()
            .join(", ");
        format!("rt::NodeKey::from_bytes([{bytes}])")
    }

    fn composites_block<'a>(source: &'a str, oracle_symbol: &str) -> &'a str {
        let stem = oracle_symbol
            .strip_prefix("oracle_")
            .expect("generated oracle symbol");
        let marker = format!("fn composites_{stem}()");
        let (_, block) = source
            .split_once(&marker)
            .expect("item's declaration block");
        block.split_once("\n}\n").expect("declaration block end").0
    }

    let package = admit(builder);
    let items = [
        CompositeEqualityItem {
            node_id: code_id(E_QSPEC_LIST),
            operator: EqualityOperatorKind::Equal,
            left: EqualityOperandDescriptor::typed(list.clone()),
            right: EqualityOperandDescriptor::typed(list.clone()),
        },
        CompositeEqualityItem {
            node_id: code_id(E_QSPEC_TREE),
            operator: EqualityOperatorKind::Equal,
            left: EqualityOperandDescriptor::typed(tree.clone()),
            right: EqualityOperandDescriptor::typed(tree.clone()),
        },
    ];
    let oracles = generate(&package, &items);
    let list_claim = generated(only_claim(&oracles, E_QSPEC_LIST));
    let tree_claim = generated(only_claim(&oracles, E_QSPEC_TREE));
    let list = package.ids.resolve(&list);
    let tree = package.ids.resolve(&tree);
    assert_eq!(list_claim.declaration_keys, vec![list.clone()]);
    assert_eq!(tree_claim.declaration_keys, vec![tree.clone()]);
    let lib = contents(&oracles, "src/lib.rs");
    assert_eq!(lib.matches("pub fn oracle_").count(), 2);
    assert_eq!(lib.matches("pub fn environment_").count(), 2);
    let list_block = composites_block(lib, &list_claim.oracle_symbol);
    let list_key = emitted_key(&list);
    assert!(
        list_block.contains(&format!("rt::CompositeDeclaration::new({list_key}, ")),
        "List declaration: {list_block}"
    );
    assert!(list_block.contains("rt::FieldDeclaration::new(\"next\", {"));
    assert!(list_block.contains(&format!("rt::ValueType::Composite({list_key})")));
    assert!(list_block.contains("rt::Presence::Optional)"));
    let tree_block = composites_block(lib, &tree_claim.oracle_symbol);
    let tree_key = emitted_key(&tree);
    assert!(
        tree_block.contains(&format!("rt::CompositeDeclaration::new({tree_key}, ")),
        "Tree declaration: {tree_block}"
    );
    assert!(tree_block.contains("rt::FieldDeclaration::new(\"kids\", {"));
    assert!(tree_block.contains(&format!("rt::ValueType::Composite({tree_key})")));
    assert!(tree_block.contains("rt::CollectionKind::Sequence"));
    assert!(tree_block.contains("rebuild_cardinality(0, 3)?"));
    assert!(tree_block.contains("rt::Presence::Required)"));
}

/// Optional local conformance against the private QSpec fixture. The public
/// QSL facade case above exercises the same reader-to-oracle path in CI.
#[trace("FR-018-AC-24", "FR-018-AC-26", "FR-018-AC-27", "TC-029")]
#[test]
fn tc_029_authoritative_qspec_recursive_items_generate_through_reader() {
    const FIXTURE: &str = "proposals/checked-package-v2/fixtures/positive-recursive-records.json";
    let Some(repo) = std::env::var_os("QSPEC_REPO") else {
        println!("SKIP local QSpec conformance: QSPEC_REPO is unset; private fixture is not available in public CI");
        return;
    };
    let output = Command::new("git")
        .arg("-C")
        .arg(repo)
        .arg("show")
        .arg(format!("origin/main:{FIXTURE}"))
        .output()
        .expect("read authoritative QSpec fixture");
    assert!(
        output.status.success(),
        "git show QSpec fixture: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let mut evidence = quire_contract_model::CheckedPackageEvidence::new();
    evidence.support_feature("quire.value.complete/v1");
    let wire: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("QSpec fixture JSON");
    let canonical = serde_json::to_vec(&wire).expect("canonical QSpec fixture");
    let read = CheckedPackageV2::read(
        &canonical,
        quire_contract_model::CheckedPackageReadLimits::bounded(),
        &evidence,
    );
    assert!(
        matches!(read, CheckedPackageV2ReadResult::Admitted(_)),
        "original QSpec fixture: {read:?}"
    );
    let (builder, list, tree) = recursive_items_package_from_wire(wire);
    assert_recursive_item_generation(builder, list, tree);
}

fn refused(claim: &CompositeEqualityClaim) -> &CompositeEqualityRefusal {
    match &claim.result {
        ClaimDisposition::Refused { refusal } => refusal,
        ClaimDisposition::Generated(_) => panic!("expected refusal, got generated"),
    }
}

/// A call whose lowered contract package exceeds the Contract IR byte ceiling
/// has no package for CG to consume. Contract IR FR-038-AC-95 retains the exact
/// typed Failed(Bytes) records and publishes no package or generated artifact.
#[trace("FR-018-AC-20", "TC-029")]
#[test]
fn tc_029_ac20_byte_ceiling_has_failed_records_and_no_contract_package() {
    use crate::common::byte_ceiling::{
        limits_under, lowered_package_length, measuring_profile, LARGEST_CEILING,
    };
    const REQUESTS: u32 = 40;

    let builder = byte_ceiling_package();
    // One admitted node under distinct descriptors keeps the checked wire
    // small while each requested feature adds its own lowering accounting.
    // The first descriptor is valid, so the exact-length control generates.
    let items = (0..REQUESTS)
        .map(|offset| {
            let left = if offset == 0 {
                typed(T_BOOLEAN)
            } else {
                EqualityOperandDescriptor::typed(id(&key(30_000 + offset)))
            };
            item(
                BYTE_CHAIN_BASE,
                EqualityOperatorKind::Equal,
                left,
                typed(T_BOOLEAN),
            )
        })
        .collect::<Vec<_>>();
    let checked_length = u64::try_from(serde_json::to_vec(&builder.wire()).unwrap().len()).unwrap();
    let measured = admit_with(&builder, limits_under(LARGEST_CEILING));
    let requested = vec![measured.key(BYTE_CHAIN_BASE); items.len()];
    let lowered_length = lowered_package_length(
        &measured,
        &requested,
        &measuring_profile(false, "quire.codegen.composite-equality/v1"),
    );
    let ceiling = lowered_length - 1;
    assert!(
        ceiling >= checked_length,
        "the lowered package ({lowered_length} bytes) is longer than the checked package \
         ({checked_length} bytes), so a ceiling one byte below it admits the checked package"
    );
    // At the lowered package's own length no record fails for bytes: the ceiling is exact.
    let exact = generate(&admit_with(&builder, limits_under(lowered_length)), &items);
    assert!(exact.claim_map.items.iter().all(|claim| !matches!(
        claim.result,
        ClaimDisposition::Refused {
            refusal: CompositeEqualityRefusal::PackageAccounting { .. }
        }
    )));
    assert!(exact.claim_map.items.iter().any(|claim| {
        matches!(
            &claim.result,
            ClaimDisposition::Generated(generated)
                if generated.descriptor.left_source_type == measured.key(T_BOOLEAN)
        )
    }));

    let under = admit_with(&builder, limits_under(ceiling));
    let failed = under.checked.lower(&requested, &admitted_profile());
    assert_eq!(failed.records.len(), items.len());
    let mut consumed_seen = BTreeSet::new();
    for (record, expected_node) in failed.records.iter().zip(&requested) {
        let CompleteLoweringRecordV2::Failed {
            node_id,
            limit_kind: CheckedPackageLimit::Bytes,
            limit,
            consumed,
            ..
        } = record
        else {
            panic!("expected a typed Contract IR byte-ceiling refusal: {record:?}");
        };
        assert_eq!(node_id, expected_node);
        assert_eq!(*limit, ceiling);
        assert!(
            *consumed > ceiling,
            "{consumed} is above the ceiling {ceiling}"
        );
        consumed_seen.insert(*consumed);
    }
    assert_eq!(
        consumed_seen.len(),
        1,
        "one shared `consumed`: {consumed_seen:?}"
    );
    assert!(failed.package.canonical_bytes().is_none());
    assert!(failed.package.wire().is_none());
}

#[trace("FR-018-AC-1", "TC-029")]
#[test]
fn tc_029_ac1_every_item_gets_one_disposition_refused_siblings_unchanged() {
    let package = admit(corpus_package());
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

#[trace("FR-018-AC-3", "TC-029")]
#[test]
fn tc_029_ac3_claim_descriptor_matches_the_request() {
    // Left and right carry different source types (a bounded-integer-to-integer
    // conversion on the left only) so a descriptor-recording bug that swaps or
    // drops an operand side is observable, not masked by a symmetric request.
    // E_CONV's own body agrees with exactly this descriptor (its natural
    // usage elsewhere in this module and in the agreement test) -- since
    // codegen#82, an arbitrary node id can no longer stand in for one whose
    // body disagrees with the requested descriptor.
    let package = admit(corpus_package());
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
    assert_eq!(
        claim.descriptor.left_source_type,
        package.ids.resolve(&left.source_type)
    );
    assert_eq!(
        claim.descriptor.left_conversion_target,
        left.conversion_target
            .as_ref()
            .map(|id| package.ids.resolve(id))
    );
    assert_eq!(
        claim.descriptor.right_source_type,
        package.ids.resolve(&right.source_type)
    );
    assert_eq!(
        claim.descriptor.right_conversion_target,
        right
            .conversion_target
            .as_ref()
            .map(|id| package.ids.resolve(id))
    );
}

#[trace("FR-018-AC-4", "TC-029")]
#[test]
fn tc_029_admitted_one_reference_sequence_resolves_its_separate_bound() {
    let package = admit(corpus_package());
    let sequence = package.ids.resolve(&code_id(SEQ_INT));
    let bound = package.ids.resolve(&code_id(CB_INT));
    let graph = &package.checked.graph().nodes;
    let sequence_node = graph.iter().find(|node| node.node_id == sequence).unwrap();
    let bound_node = graph.iter().find(|node| node.node_id == bound).unwrap();
    assert_eq!(sequence_node.semantic_form.as_ref(), "sequence");
    assert_eq!(
        sequence_node.body["members"].as_array().unwrap().len(),
        1,
        "reader-admitted sequence has one element reference"
    );
    assert_eq!(bound_node.semantic_form.as_ref(), "collection_bounds");
    assert_eq!(bound_node.semantic_type, sequence);
    let oracles = generate(
        &package,
        &[item(
            E_COLLECTION,
            EqualityOperatorKind::Equal,
            typed(SEQ_INT),
            typed(SEQ_INT),
        )],
    );
    let claim = only_claim(&oracles, E_COLLECTION);
    assert!(
        matches!(&claim.result, ClaimDisposition::Generated(_)),
        "one-reference admitted sequence resolves: {claim:#?}"
    );
}

#[trace("FR-018-AC-4", "TC-029")]
#[test]
fn tc_029_ac4_schedule_matches_checked_equality_for_every_shape() {
    let package = admit(corpus_package());
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

/// Sequence cardinality comes from
/// the admitted bound index, and its absence is a typed refusal.
#[trace("FR-018-AC-4", "FR-018-AC-16", "TC-029")]
#[test]
fn tc_029_a_sequence_without_a_separate_collection_bound_is_refused() {
    let package = admit(unbounded_sequence_package());
    let oracles = generate(
        &package,
        &[item(
            E_COLLECTION,
            EqualityOperatorKind::Equal,
            typed(SEQ_INT),
            typed(SEQ_INT),
        )],
    );
    assert_eq!(
        refused(only_claim(&oracles, E_COLLECTION)),
        &CompositeEqualityRefusal::MissingBound {
            bounded_type: oracles.key(SEQ_INT),
            expected_form: "collection_bounds",
        }
    );
}

#[trace("FR-018-AC-5", "TC-029")]
#[test]
fn tc_029_ac5_a_disallowed_conversion_refuses_at_generation_time() {
    let package = admit(corpus_package());
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

/// FR-018 Behavior ("disagrees with its descriptor's arity or
/// operand types"), TC-029 step 1 ("a descriptor disagreeing with its
/// operand types"), codegen#82.
///
/// `E_OPERAND_MISMATCH`'s body names both operands `T_INTEGER`
/// (`package::binary_body(T_INTEGER, T_INTEGER)`). A descriptor naming a
/// different type for either position must refuse before generation --
/// checked left, then right, so the two requests below exercise both
/// positions and each reports the position that actually disagreed.
#[trace("FR-018", "TC-029")]
#[test]
fn tc_029_a_body_operand_disagreeing_with_the_descriptor_refuses_before_generation() {
    let package = admit(corpus_package());

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
            assert_eq!(*expected, oracles.key(T_TEXT));
            assert_eq!(*found, Some(oracles.key(T_INTEGER)));
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
            assert_eq!(*expected, oracles.key(T_TEXT));
            assert_eq!(*found, Some(oracles.key(T_INTEGER)));
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

#[trace("FR-018-AC-6", "TC-029")]
#[test]
fn tc_029_ac6_ieee_at_any_depth_is_operator_ineligible() {
    let package = admit(corpus_package());
    // Reconstruct SEQ_R_FLOAT (a sequence of R_FLOAT { f: Float64 }) exactly
    // as the runtime environment sees it, and confirm the reader refusal
    // asserted by `tc_052_nested_ieee_equality_is_refused_by_reader`
    // agrees with an independent, direct `contains_ieee` call over that
    // reconstructed type at both its depths (the record field, and the
    // sequence wrapping it) rather than merely riding on the same code path.
    let float_record_key = NodeKey::from_hex(package.key(R_FLOAT).digest.as_ref()).unwrap();
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

/// An operand is a `reference` to its own node. A reference to a
/// conversion node is read as the type of what it converts: the descriptor's `source_type` must
/// equal it, and a descriptor naming any other source refuses at that position, reporting the
/// converted operand's type as found.
#[trace("FR-018-AC-15", "TC-029")]
#[test]
fn tc_029_ac15_a_reference_to_a_conversion_is_read_as_the_type_it_converts() {
    let package = admit(corpus_package());
    let wrong_source = item(
        E_CONV_CHARGE,
        EqualityOperatorKind::Equal,
        converted(T_INTEGER, T_DECIMAL_SMALL),
        typed(T_DECIMAL_SMALL),
    );
    let oracles = generate(&package, std::slice::from_ref(&wrong_source));
    match refused(only_claim(&oracles, E_CONV_CHARGE)) {
        CompositeEqualityRefusal::OperandTypeMismatch {
            position,
            expected,
            found,
        } => {
            assert_eq!(*position, 0);
            assert_eq!(*expected, oracles.key(T_INTEGER));
            assert_eq!(*found, Some(oracles.key(T_INTEGER_BOUNDED)));
        }
        other => panic!("expected OperandTypeMismatch, got {other:?}"),
    }
    let right_source = item(
        E_CONV_CHARGE,
        EqualityOperatorKind::Equal,
        converted(T_INTEGER_BOUNDED, T_DECIMAL_SMALL),
        typed(T_DECIMAL_SMALL),
    );
    let oracles = generate(&package, std::slice::from_ref(&right_source));
    assert!(matches!(
        only_claim(&oracles, E_CONV_CHARGE).result,
        ClaimDisposition::Generated(_)
    ));
}

/// A conversion of a conversion is read through to the first node
/// that is not a conversion: the source type is the innermost operand's, not the inner
/// conversion's result type.
#[trace("FR-018-AC-15", "TC-029")]
#[test]
fn tc_029_ac15_nested_conversions_are_read_through_to_the_innermost_operand() {
    let package = admit(nested_conversion_package());
    let innermost = item(
        E_NESTED_CONV,
        EqualityOperatorKind::Equal,
        converted(T_INTEGER_BOUNDED, T_DECIMAL_WIDE),
        typed(T_DECIMAL_WIDE),
    );
    let oracles = generate(&package, std::slice::from_ref(&innermost));
    assert!(matches!(
        only_claim(&oracles, E_NESTED_CONV).result,
        ClaimDisposition::Generated(_)
    ));
    let inner_result = item(
        E_NESTED_CONV,
        EqualityOperatorKind::Equal,
        converted(T_RATIONAL_WIDE, T_DECIMAL_WIDE),
        typed(T_DECIMAL_WIDE),
    );
    let oracles = generate(&package, std::slice::from_ref(&inner_result));
    match refused(only_claim(&oracles, E_NESTED_CONV)) {
        CompositeEqualityRefusal::OperandTypeMismatch {
            position,
            expected,
            found,
        } => {
            assert_eq!(*position, 0);
            assert_eq!(*expected, oracles.key(T_RATIONAL_WIDE));
            assert_eq!(*found, Some(oracles.key(T_INTEGER_BOUNDED)));
        }
        other => panic!("expected OperandTypeMismatch, got {other:?}"),
    }
}

/// A reference to an application that is not a conversion is read
/// as that node's own `semantic_type`, never as its first argument's type.
#[trace("FR-018-AC-15", "TC-029")]
#[test]
fn tc_029_ac15_a_non_conversion_application_operand_is_read_as_its_own_type() {
    let package = admit(application_operand_package());
    let own_type = item(
        E_APPLICATION_OPERAND,
        EqualityOperatorKind::Equal,
        typed(T_RATIONAL_WIDE),
        typed(T_RATIONAL_WIDE),
    );
    let oracles = generate(&package, std::slice::from_ref(&own_type));
    assert!(matches!(
        only_claim(&oracles, E_APPLICATION_OPERAND).result,
        ClaimDisposition::Generated(_)
    ));
    let first_argument_type = item(
        E_APPLICATION_OPERAND,
        EqualityOperatorKind::Equal,
        typed(T_INTEGER),
        typed(T_RATIONAL_WIDE),
    );
    let oracles = generate(&package, std::slice::from_ref(&first_argument_type));
    match refused(only_claim(&oracles, E_APPLICATION_OPERAND)) {
        CompositeEqualityRefusal::OperandTypeMismatch {
            position, found, ..
        } => {
            assert_eq!(*position, 0);
            assert_eq!(*found, Some(oracles.key(T_RATIONAL_WIDE)));
        }
        other => panic!("expected OperandTypeMismatch, got {other:?}"),
    }
}

/// Reader-admissible unsupported families retain
/// their own generator blockers; reference composites have a reader-stage
/// control in `tc_029_ac7_a_direct_reference_operand_is_refused_by_ir_today`.
#[trace("FR-018-AC-7", "TC-029")]
#[test]
fn tc_029_ac7_each_family_gets_its_own_distinct_blocker() {
    let package = admit(corpus_package());
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
            E_CALL,
            EqualityOperatorKind::Equal,
            typed(T_INTEGER),
            typed(T_INTEGER),
        ),
    ];
    let oracles = generate(&package, &items);

    for (code, tag) in [
        (M_BARE, CheckedNodeTag::Model),
        (F_BARE, CheckedNodeTag::Function),
        (S_BARE, CheckedNodeTag::State),
        (T_BARE, CheckedNodeTag::Temporal),
    ] {
        let key = oracles.key(code);
        let CompositeEqualityRefusal::PackageAccounting {
            record: boxed_record,
            occurrences,
        } = refused(only_claim(&oracles, code))
        else {
            panic!("node {code}: expected exact FR-330 Unsupported accounting");
        };
        assert!(matches!(
            boxed_record.as_ref(),
            ContractFeatureRecordV1::Unsupported {
                node_id,
                unsupported_node_id,
                node_tag,
            } if node_id == &key && unsupported_node_id == &key && node_tag.as_ref() == tag.as_wire()
        ));
        let expected = package
            .checked
            .source_map()
            .iter()
            .filter(|source| source.node_id == key)
            .cloned()
            .collect::<Vec<_>>();
        assert_eq!(occurrences, &expected);
    }
    assert!(matches!(
        refused(only_claim(&oracles, E_CALL)),
        CompositeEqualityRefusal::BlockedOnUpstream {
            issue: UpstreamBlocker::QuireContractRuntime34,
            ..
        }
    ));
    for node in [M_BARE, F_BARE, S_BARE, T_BARE, E_CALL] {
        assert!(claims_for(&oracles, node)
            .iter()
            .all(|claim| matches!(claim.result, ClaimDisposition::Refused { .. })));
    }
}

/// The direct `reference` composite form: an equality over two
/// `REF_TYPE` operands is refused by Contract IR at admission, before this generator runs, so the
/// generator's `QuireSpecLanguage120` blocker is not reached for it today. This pins the exact
/// refusal (code, cause, pointer, locus); `quire.op.reference.eq` needs a `Reference<X>` naming a
/// model object type of a selected document, which `REF_TYPE` does not. Unblocked by a corpus
/// with such a model declaration.
#[trace("FR-018-AC-7", "TC-029")]
#[test]
fn tc_029_ac7_a_direct_reference_operand_is_refused_by_ir_today() {
    let (result, wire, ids) = direct_reference_package().read_resolved();
    let node_id = ids.resolve(&code_id(E_REFERENCE_DIRECT));
    let position = wire["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes")
        .iter()
        .position(|node| node["node_id"]["digest"].as_str() == Some(node_id.digest.as_ref()))
        .expect("the node is in the wire");
    let CheckedPackageV2ReadResult::Refused(refusal) = result else {
        panic!("IR now admits a direct reference operand ({result:?}); add it to the corpus");
    };
    assert_eq!(refusal.code, CheckedPackageRefusalCode::IllTyped);
    assert_eq!(
        refusal.cause,
        Some(CheckedPackageRefusalCause::OperatorIneligible)
    );
    assert_eq!(
        refusal.path.as_ref().map(|path| path.as_str().to_owned()),
        Some(format!("/semantic_graph/nodes/{position}/body/arguments/0"))
    );
    assert_eq!(refusal.locus, Some(node_id));
}

/// FR-018-AC-8 (generation-time half; the `check_type` guard
/// half is `tc_029_ac8_check_type_guards_the_oracle` in the agreement test).
#[trace("FR-018-AC-8", "TC-029")]
#[test]
fn tc_029_ac8_a_duplicate_field_is_refused_with_its_declaration_cause() {
    let package = admit(corpus_package());
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

#[trace("FR-018-AC-10", "TC-029")]
#[test]
fn tc_029_ac10_bytes_are_deterministic_across_request_permutations() {
    let package = admit(corpus_package());
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

#[trace("FR-018-AC-10", "TC-029")]
#[test]
fn tc_029_ac10_claim_map_entries_ascend_by_the_descriptor_key() {
    let package = admit(corpus_package());
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

/// The generator's current output for the whole corpus request, which
/// `composite_equality_agreement` builds and executes.
/// The `names.rs` the composite agreement cases `include!`: each executed oracle and environment
/// under `{oracle,environment}_{code}_{operator}`, and the base package's enum declaration key.
pub(super) fn agreement_names(oracles: &FixtureOracles) -> String {
    let executed: &[(u32, &str, &str)] = &[
        (E_RECORD, "equal", "e_record_equal"),
        (E_TUPLE, "equal", "e_tuple_equal"),
        (E_OPTION, "equal", "e_option_equal"),
        (E_COLLECTION, "equal", "e_collection_equal"),
        (E_SELF, "equal", "e_self_equal"),
        (E_PAIR_OF_POINTS, "equal", "e_pair_of_points_equal"),
        (E_RECORD, "not_equal", "e_record_not_equal"),
        (E_TEXT, "equal", "e_text_equal"),
        (E_ENUM, "equal", "e_enum_equal"),
        (E_CONV, "equal", "e_conv_equal"),
        (E_CONV_CHARGE, "equal", "e_conv_charge_equal"),
        (E_CONV_RAT_RAT, "equal", "e_conv_rat_rat_equal"),
        (E_CONV_RAT_INT, "equal", "e_conv_rat_int_equal"),
        (E_CONV_DEC_RAT, "equal", "e_conv_dec_rat_equal"),
        (E_CONV_DEC_DEC, "equal", "e_conv_dec_dec_equal"),
        (E_CONV_DEC_INT, "equal", "e_conv_dec_int_equal"),
    ];
    let mut aliases = Vec::new();
    for (code, operator, alias) in executed {
        let identity = format!("equality.{operator}");
        let generated = oracles
            .claim_map
            .items
            .iter()
            .find(|claim| {
                claim.node_id == oracles.key(*code) && claim.operation.identity == identity
            })
            .and_then(|claim| match &claim.result {
                ClaimDisposition::Generated(generated) => Some(generated),
                ClaimDisposition::Refused { .. } => None,
            })
            .unwrap_or_else(|| panic!("node {code} {operator} is not generated"));
        aliases.push((generated.oracle_symbol.clone(), format!("oracle_{alias}")));
        aliases.push((
            generated.environment_symbol.clone(),
            format!("environment_{alias}"),
        ));
    }
    super::exact_scalar_agreement::names_file(
        quire_contract_codegen::COMPOSITE_EQUALITY_CRATE_NAME,
        &aliases,
        &format!(
            "pub const ENUM_TYPE_DIGEST: &str = {:?};\n\
             pub const R_POINT_DIGEST: &str = {:?};\n\
             pub const TUP_PAIR_DIGEST: &str = {:?};\n\
             pub const R_SELF_DIGEST: &str = {:?};\n\
             pub const R_PAIR_OF_POINTS_DIGEST: &str = {:?};\n",
            enum_type_digest(),
            oracles.runtime_key(R_POINT),
            oracles.runtime_key(TUP_PAIR),
            oracles.runtime_key(R_SELF),
            oracles.runtime_key(R_PAIR_OF_POINTS),
        ),
    )
}

pub(super) fn corpus_oracles() -> FixtureOracles {
    generate(&admit(corpus_package()), &golden_items())
}

/// Generating the corpus twice, each time from a freshly built and admitted
/// package, yields equal output: every artifact's bytes and the claim map.
#[trace("FR-018-AC-10", "TC-029", "TC-001", "NFR-002-AC-1", "NFR-002-AC-2")]
#[test]
fn tc_029_ac10_generation_is_repeatable_from_a_fresh_package() {
    let first = corpus_oracles();
    let second = corpus_oracles();
    assert_eq!(first, second);
    let subjects = first
        .claim_map
        .items
        .iter()
        .filter(|claim| matches!(&claim.result, ClaimDisposition::Generated(_)))
        .map(|claim| format!("{}/{}", claim.node_id.domain, claim.node_id.digest))
        .collect::<Vec<String>>();
    let subject_refs = subjects.iter().map(String::as_str).collect::<Vec<_>>();
    let declaration_subjects = first
        .claim_map
        .items
        .iter()
        .filter_map(|claim| match &claim.result {
            ClaimDisposition::Generated(generated) => Some(&generated.declaration_keys),
            ClaimDisposition::Refused { .. } => None,
        })
        .flatten()
        .map(|id| format!("{}/{}", id.domain, id.digest))
        .collect::<Vec<_>>();
    let declaration_refs = declaration_subjects
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    let lib = contents(&first, "src/lib.rs");
    crate::generated_headers::assert_header("src/lib.rs", lib, &declaration_refs);
    for artifact in first
        .artifacts
        .iter()
        .filter(|artifact| artifact.path.ends_with(".rs"))
    {
        crate::generated_headers::assert_header(&artifact.path, &artifact.contents, &subject_refs);
    }
    for path in ["Cargo.toml", "src/lib.rs", "claim-map.json"] {
        assert_eq!(contents(&first, path), contents(&second, path), "{path}");
    }
}

/// FR-018-AC-11 (generation-time half; complementary outcomes
/// are `tc_029_ac11_complementary_outcomes` in the agreement test).
#[trace("FR-018-AC-11", "TC-029")]
#[test]
fn tc_029_ac11_two_operators_over_one_node_get_distinct_symbols_and_are_caller_declared() {
    let package = admit(corpus_package());
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

/// The `rebuild_integer` reconstruction helper is emitted only when some rendered
/// bound actually calls it (a reachable bounded `Int` or `Decimal`).
/// Unconditional emission would be unused, and hence dead code, for a
/// request that reaches neither — which would fail this same test's
/// `-D warnings` build below for that request, even though the full corpus
/// (which does reach one, via TUP_PAIR) never observes it.
#[trace("FR-018-AC-12", "TC-029")]
#[test]
fn tc_029_ac12_integer_helper_omitted_when_unreached() {
    let package = admit(corpus_package());
    let request = [item(
        E_TEXT,
        EqualityOperatorKind::Equal,
        typed(T_TEXT),
        typed(T_TEXT),
    )];
    let oracles = generate(&package, &request);
    let lib = contents(&oracles, "src/lib.rs");
    assert!(
        !lib.contains("fn rebuild_integer(") && !lib.contains("fn rebuild_interval("),
        "no reachable bounded Int/Decimal in this request; the integer helpers must be omitted:\n{lib}"
    );
}

#[trace("FR-018-AC-12", "TC-029")]
#[test]
fn tc_029_ac12_manifest_is_unpublished_and_charge_free() {
    let oracles = generate(&admit(corpus_package()), &golden_items());
    let manifest = contents(&oracles, "Cargo.toml");
    assert!(manifest.contains(&format!("name = \"{COMPOSITE_EQUALITY_CRATE_NAME}\"")));
    assert!(manifest.contains("publish = false"));
    assert!(manifest.contains(&runtime_dependency(&["exact"])));
    assert!(!manifest.contains("rev ="));

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
    // panic surface the FR forbids (the whole crate's scan is AC-17's).
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
    fs::create_dir_all(directory.path.join("src")).unwrap();
    for artifact in &oracles.artifacts {
        fs::write(directory.path.join(&artifact.path), &artifact.contents).unwrap();
    }
    seed_lock(&directory.path);
    let output = Command::new(env!("CARGO"))
        .args(["build", "--offline", "--quiet"])
        .env("CARGO_TARGET_DIR", directory.path.join("target"))
        .env("RUSTFLAGS", "-Dwarnings")
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .current_dir(&directory.path)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "generated crate did not compile: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[trace("FR-018-AC-13", "TC-029")]
#[test]
fn tc_029_ac13_declaration_keys_are_the_reached_v2_node_ids() {
    let package = admit(corpus_package());
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
    assert_eq!(single.declaration_keys, vec![oracles.key(R_POINT)]);
    assert_eq!(
        single.declaration_runtime_keys,
        vec![oracles.runtime_key(R_POINT)],
        "declaration_runtime_keys must equal NodeKey::from_hex of declaration_keys"
    );

    let nested = generated(only_claim(&oracles, E_PAIR_OF_POINTS));
    assert_eq!(
        nested.declaration_keys,
        vec![oracles.key(R_POINT), oracles.key(R_PAIR_OF_POINTS)],
        "declaration keys ascend by digest domain then digest"
    );
    assert_eq!(
        nested.declaration_runtime_keys,
        vec![
            oracles.runtime_key(R_POINT),
            oracles.runtime_key(R_PAIR_OF_POINTS)
        ]
    );

    // FR-018-AC-13 also names node id, package id, source map, claims
    // and the selected schedule. Recompute each independently — never by
    // reading it back off the generated claim-map — using the same public
    // `CheckedPackageV2::lower` the generator calls, but with a
    // self-constructed, maximally permissive profile (`CheckedNodeTag::ALL`)
    // so this recomputation shares no private profile constant with the
    // generator. `claims` and `source_map` must agree regardless of which
    // permissive profile computed them.
    let profile = CompleteLoweringProfileV2 {
        reference: quire_contract_model::LoweringProfileRef {
            authority: "agent-ix".into(),
            identity: "quire.codegen.composite-equality/v1".into(),
        },
        supported_tags: CheckedNodeTag::ALL.iter().copied().collect::<BTreeSet<_>>(),
        require_bounds: false,
        work_limit: 10_000,
    };
    assert_eq!(
        only_claim(&oracles, E_RECORD).node_id,
        oracles.key(E_RECORD)
    );
    assert_eq!(
        only_claim(&oracles, E_PAIR_OF_POINTS).node_id,
        oracles.key(E_PAIR_OF_POINTS)
    );
    assert_eq!(single.schedule, RecordedSchedule::Plan);
    assert_eq!(nested.schedule, RecordedSchedule::Plan);
    assert_eq!(single.package_id, *package.package_id());
    assert_eq!(nested.package_id, *package.package_id());

    let single_lowering = package.lower(&[package.key(E_RECORD)], &profile);
    assert_eq!(
        *single_lowering.package.source_package_id(),
        *package.package_id(),
        "source_package_id is the admitted SOURCE package's identity; package_id() is a \
         different digest, over the lowered closure"
    );
    match &single_lowering.records[..] {
        [CompleteLoweringRecordV2::Lowered { node }] => {
            assert_eq!(single.node_id, oracles.key(E_RECORD));
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

    let nested_lowering = package.lower(&[package.key(E_PAIR_OF_POINTS)], &profile);
    match &nested_lowering.records[..] {
        [CompleteLoweringRecordV2::Lowered { node }] => {
            assert_eq!(nested.node_id, oracles.key(E_PAIR_OF_POINTS));
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

/// The generated source of `E_TUPLE` over `TUP_PAIR` built from `extras` and `members`.
fn tuple_source(extras: &[u32], members: &[u32]) -> (FixtureOracles, String) {
    let package = admit(tuple_members_package(extras, members));
    let items = vec![item(
        E_TUPLE,
        EqualityOperatorKind::Equal,
        typed(TUP_PAIR),
        typed(TUP_PAIR),
    )];
    // These two scalar members name their base type; their separate bounded
    // domains are caller-authored context roots, not body dependencies.
    let context = members
        .iter()
        .filter_map(|member| match *member {
            T_DECIMAL_SMALL => Some(BD_DECIMAL),
            T_RATIONAL_NARROW => Some(BD_RATIONAL_NARROW),
            _ => None,
        })
        .map(|code| package.key(code))
        .collect::<Vec<_>>();
    let oracles = generate_with_context(&package, &items, &context);
    let lib = contents(&oracles, "src/lib.rs").to_owned();
    (oracles, lib)
}

/// The refusal `E_TUPLE` gets over `TUP_PAIR` built from `extras` and `members`.
fn tuple_refusal(extras: &[u32], members: &[u32]) -> (CompositeEqualityRefusal, FixtureIds) {
    let package = admit(tuple_members_package(extras, members));
    let items = vec![item(
        E_TUPLE,
        EqualityOperatorKind::Equal,
        typed(TUP_PAIR),
        typed(TUP_PAIR),
    )];
    let oracles = generate(&package, &items);
    (refused(only_claim(&oracles, E_TUPLE)).clone(), oracles.ids)
}

/// The corpus tuple `(Int[-100, 100], Text[0, 16; nfc])` names the
/// `text_bounds` node as its text position, as QSL emits a text type; the generated declaration
/// is that tuple, read from the bound's own members.
#[trace("FR-018-AC-16", "TC-029")]
#[test]
fn tc_029_ac16_a_text_bounds_member_reconstructs_its_own_text_type() {
    let (oracles, lib) = tuple_source(&[], &[T_INTEGER_BOUNDED, BD_TEXT]);
    let _ = generated(only_claim(&oracles, E_TUPLE));
    let int = "rt::ValueType::Int(rebuild_interval(\"-100\", \"100\")?)";
    let text = "rt::ValueType::Text(rebuild_text(0, 16, rt::TextProfile::Nfc)?)";
    let tuple = lib
        .find("rt::CompositeShape::Tuple(vec![")
        .expect("tuple declaration");
    let int_position = lib[tuple..].find(int).expect("integer tuple member");
    let text_position = lib[tuple..].find(text).expect("text tuple member");
    assert!(
        int_position < text_position,
        "tuple members must retain their order"
    );
}

/// A numeric member that names its `bounded_domain` node reads as
/// the same type as one that names the base scalar the bound hangs off, for integer, decimal
/// and rational bases.
#[trace("FR-018-AC-16", "TC-029")]
#[test]
fn tc_029_ac16_numeric_bounded_domain_members_read_as_their_bounded_scalars() {
    let (_, through_scalars) = tuple_source(
        &[],
        &[
            T_INTEGER_BOUNDED,
            BD_TEXT,
            T_DECIMAL_SMALL,
            T_RATIONAL_NARROW,
        ],
    );
    let (_, through_domains) =
        tuple_source(&[], &[BD_INTEGER, BD_TEXT, BD_DECIMAL, BD_RATIONAL_NARROW]);
    assert!(
        through_scalars.contains("rt::ValueType::Decimal(")
            && through_scalars.contains("rt::ValueType::Rational(")
    );
    for reconstruction in [
        "rt::ValueType::Int(rebuild_interval(\"-100\", \"100\")?)",
        "rt::ValueType::Decimal(rebuild_decimal(\"-100\", \"100\", 0, 0, rt::RoundingMode::NearestEven)?)",
        "rt::ValueType::Rational(rebuild_rational(rebuild_interval(\"-10\", \"10\")?, rebuild_interval(\"1\", \"1\")?)?)",
    ] {
        assert!(through_scalars.contains(reconstruction), "scalar member: {reconstruction}");
        assert!(through_domains.contains(reconstruction), "domain member: {reconstruction}");
    }
}

/// A `bounded_domain` member reads its own bound and never a sibling
/// bound over the same base: with a second `text_bounds` over the text scalar, naming either
/// reads that node's `min` and `max`, neither is ambiguous.
#[trace("FR-018-AC-16", "TC-029")]
#[test]
fn tc_029_ac16_a_bounded_domain_member_never_reads_a_sibling_bound() {
    let (oracles, lib) = tuple_source(&[BD_TEXT_SIBLING], &[T_INTEGER_BOUNDED, BD_TEXT_SIBLING]);
    let _ = generated(only_claim(&oracles, E_TUPLE));
    assert!(lib.contains("rebuild_text(1, 5, rt::TextProfile::Nfc)?"));
    assert!(!lib.contains("rebuild_text(0, 16,"));
    let (oracles, lib) = tuple_source(&[BD_TEXT_SIBLING], &[T_INTEGER_BOUNDED, BD_TEXT]);
    let _ = generated(only_claim(&oracles, E_TUPLE));
    assert!(lib.contains("rebuild_text(0, 16, rt::TextProfile::Nfc)?"));
    assert!(!lib.contains("rebuild_text(1, 5,"));
}

/// A bound whose form is not the one its base scalar reads is
/// refused as missing the form the base needs, not read as an unbounded type.
#[trace("FR-018-AC-16", "TC-029")]
#[test]
fn tc_029_ac16_a_bound_form_that_does_not_fit_its_base_is_refused() {
    let (refusal, ids) = tuple_refusal(&[BD_INTEGER_WRONG_FORM], &[BD_INTEGER_WRONG_FORM, BD_TEXT]);
    match refusal {
        CompositeEqualityRefusal::MissingBound {
            bounded_type,
            expected_form,
        } => {
            assert_eq!(bounded_type, ids.resolve(&code_id(BD_INTEGER_WRONG_FORM)));
            assert_eq!(expected_form, "integer_range");
        }
        other => panic!("expected MissingBound, got {other:?}"),
    }
}

/// A tuple containing IEEE is
/// refused by the checked reader before equality generation.
#[trace("FR-018-AC-16", "FR-018-AC-6", "TC-029")]
#[test]
fn tc_029_ac16_a_float_rounding_member_is_refused_as_operator_ineligible() {
    let (result, wire, ids) =
        tuple_members_package(&[BD_FLOAT_ROUNDING], &[BD_FLOAT_ROUNDING, BD_TEXT]).read_resolved();
    let node_id = ids.resolve(&code_id(E_TUPLE));
    let position = wire["semantic_graph"]["nodes"]
        .as_array()
        .expect("nodes")
        .iter()
        .position(|node| node["node_id"]["digest"].as_str() == Some(node_id.digest.as_ref()))
        .expect("tuple equality");
    let CheckedPackageV2ReadResult::Refused(refusal) = result else {
        panic!("IEEE tuple unexpectedly admitted: {result:?}");
    };
    assert_eq!(refusal.code, CheckedPackageRefusalCode::IllTyped);
    assert_eq!(
        refusal.cause,
        Some(CheckedPackageRefusalCause::OperatorIneligible)
    );
    assert_eq!(
        refusal.path.as_ref().map(|path| path.as_str().to_owned()),
        Some(format!(
            "/semantic_graph/nodes/{position}/body/operation/leaves"
        ))
    );
    assert_eq!(refusal.locus, Some(node_id));
}

/// The original bounded-domain negatives over
/// Boolean, an enum, and a record are rejected by the checked reader.
#[trace("FR-018-AC-16", "TC-029")]
#[test]
fn tc_029_ac16_a_bounded_domain_over_a_base_without_a_bound_form_is_refused() {
    for bound in [BD_BOOLEAN, BD_ENUM, BD_OVER_COMPOSITE] {
        let (_, mut wire, ids) = corpus_package().read_resolved();
        let authored = tuple_members_package(&[bound], &[bound, BD_TEXT]).raw_wire();
        let mut bad = authored["semantic_graph"]["nodes"]
            .as_array()
            .expect("authored nodes")
            .iter()
            .find(|node| node["node_id"]["digest"] == key(bound))
            .expect("authored bad bound")
            .clone();
        ids.resolve_value(&mut bad);
        let nodes = wire["semantic_graph"]["nodes"]
            .as_array_mut()
            .expect("nodes");
        let position = nodes.len();
        nodes.push(bad);
        let node_id = code_id(bound);
        let draft: Vec<CheckedSemanticNodeV2> =
            serde_json::from_value(wire["semantic_graph"]["nodes"].clone()).expect("draft nodes");
        assert!(matches!(
            derive_structural_candidates(
                &draft,
                quire_contract_model::CheckedPackageReadLimits::bounded().bytes,
                quire_contract_model::CheckedPackageReadLimits::bounded().work,
            ),
            Err(StructuralCandidateError::InvalidClosedShape { position: refused, node_id: invalid })
                if refused == position && invalid == node_id
        ));
        let result = read_mutated_wire(wire);
        let CheckedPackageV2ReadResult::Refused(refusal) = result else {
            panic!("bound {bound} unexpectedly admitted: {result:?}");
        };
        assert_eq!(refusal.code, CheckedPackageRefusalCode::InvalidPackage);
        assert_eq!(
            refusal.cause,
            Some(CheckedPackageRefusalCause::StaleNodeKey)
        );
        assert_eq!(
            refusal.path.as_ref().map(|path| path.as_str().to_owned()),
            Some(format!("/semantic_graph/nodes/{position}/node_id"))
        );
        assert_eq!(refusal.locus, Some(node_id));
    }
}

/// One `fn` of the emitted source: its name, its signature (up to the body's `{`), its body, and
/// where it starts.
struct EmittedFn<'s> {
    name: &'s str,
    signature: &'s str,
    body: &'s str,
    start: usize,
}

/// The offset of the delimiter closing the one opened at `open_at` in `text`.
fn matching_close(text: &str, open_at: usize, open: char, close: char) -> Option<usize> {
    let mut depth = 0usize;
    for (offset, character) in text.get(open_at..)?.char_indices() {
        if character == open {
            depth += 1;
        } else if character == close {
            depth = depth.checked_sub(1)?;
            if depth == 0 {
                return Some(open_at + offset);
            }
        }
    }
    None
}

/// Every top-level `fn` and `pub fn` of the emitted source.
fn emitted_fns(lib: &str) -> Vec<EmittedFn<'_>> {
    let mut fns = Vec::new();
    let mut start = 0;
    for line in lib.split_inclusive('\n') {
        let declaration = line.strip_prefix("pub ").unwrap_or(line);
        if let Some(rest) = declaration.strip_prefix("fn ") {
            let name_end = rest
                .find(|c: char| !(c.is_alphanumeric() || c == '_'))
                .unwrap_or(rest.len());
            let body_open = lib[start..].find('{').map(|at| start + at);
            let body_close = body_open.and_then(|open| matching_close(lib, open, '{', '}'));
            if let (Some(open), Some(close)) = (body_open, body_close) {
                fns.push(EmittedFn {
                    name: &rest[..name_end],
                    signature: &lib[start..open],
                    body: &lib[open..=close],
                    start,
                });
            }
        }
        start += line.len();
    }
    fns
}

/// The helpers the emitted source reconstructs a bound through.
const REBUILD_HELPERS: [&str; 6] = [
    "rebuild_integer",
    "rebuild_interval",
    "rebuild_rational",
    "rebuild_decimal",
    "rebuild_text",
    "rebuild_cardinality",
];

/// The `ReconstructionError` variant each helper fails with, and nothing else.
const HELPER_VARIANTS: [(&str, &str); 6] = [
    ("rebuild_integer", "Integer"),
    ("rebuild_interval", "Interval"),
    ("rebuild_rational", "Rational"),
    ("rebuild_decimal", "Decimal"),
    ("rebuild_text", "Text"),
    ("rebuild_cardinality", "Cardinality"),
];

/// The emitted functions that return `Result<_, ReconstructionError>`: the helpers, and the
/// per-item functions that build a declaration or a type from them.
fn returns_reconstruction(name: &str) -> bool {
    [
        "rebuild_",
        "composites_",
        "left_source_",
        "left_target_",
        "right_source_",
        "right_target_",
    ]
    .iter()
    .any(|prefix| name.starts_with(prefix))
}

/// Every way `lib` departs from FR-018-AC-17's structural clauses, by description.
fn structural_violations(lib: &str) -> Vec<String> {
    let mut violations = Vec::new();
    let fns = emitted_fns(lib);
    let reconstructing: Vec<&EmittedFn<'_>> = fns
        .iter()
        .filter(|function| returns_reconstruction(function.name))
        .collect();
    for function in &reconstructing {
        let returns = function
            .signature
            .rsplit_once("->")
            .map(|(_, ret)| ret.trim());
        if !returns.is_some_and(|ret| {
            ret.starts_with("Result<") && ret.ends_with(", ReconstructionError>")
        }) {
            violations.push(format!(
                "`{}` does not return Result<_, ReconstructionError>",
                function.name
            ));
        }
        let call = format!("{}(", function.name);
        for (at, _) in lib.match_indices(&call) {
            let before = &lib[..at];
            let own_signature = before.ends_with("fn ");
            let inside_name = before
                .chars()
                .next_back()
                .is_some_and(|c| c.is_alphanumeric() || c == '_');
            if own_signature || inside_name {
                continue;
            }
            let Some(close) = matching_close(lib, at + function.name.len(), '(', ')') else {
                violations.push(format!("call of `{}` has no closing paren", function.name));
                continue;
            };
            let after = &lib[close + 1..];
            if after.starts_with('?') {
                continue;
            }
            let block_open = after
                .trim_start()
                .starts_with('{')
                .then(|| close + 1 + (after.len() - after.trim_start().len()));
            let block = block_open
                .and_then(|open| matching_close(lib, open, '{', '}').map(|end| &lib[open..=end]));
            let handled = before.ends_with("match ")
                && block.is_some_and(|block| {
                    block.contains("Err(")
                        && (block.contains("EnvironmentError::Reconstruction")
                            || (block.contains("Outcome::Refused(")
                                && block.contains("Refusal::CheckedInvariant")))
                });
            if !handled {
                violations.push(format!(
                    "call of `{}` is not followed by `?` or a match refusing the failure",
                    function.name
                ));
            }
        }
    }
    for constructor in [
        "IntegerInterval::new",
        "RationalDomain::new",
        "TextType::new",
        "CardinalityBound::new",
        "DecimalType::new",
        ".parse(",
    ] {
        for (at, _) in lib.match_indices(constructor) {
            let enclosing = fns.iter().rfind(|function| function.start <= at);
            if !enclosing.is_some_and(|function| function.name.starts_with("rebuild_")) {
                violations.push(format!("`{constructor}` outside a reconstruction helper"));
            }
        }
    }
    for function in &fns {
        let Some((_, variant)) = HELPER_VARIANTS
            .iter()
            .find(|(helper, _)| *helper == function.name)
        else {
            continue;
        };
        let body: String = function
            .body
            .chars()
            .filter(|character| !character.is_whitespace())
            .collect();
        if body
            .matches(&format!(".map_err(|_|ReconstructionError::{variant})"))
            .count()
            != 1
            || body.matches("ReconstructionError::").count() != 1
        {
            violations.push(format!(
                "`{}` does not fail with exactly ReconstructionError::{variant}",
                function.name
            ));
        }
        for combinator in [
            "or_else",
            "unwrap_or",
            ".or(",
            ".ok(",
            "map_or",
            "match",
            "Ok(",
            "Err(",
        ] {
            if body.contains(combinator) {
                violations.push(format!(
                    "`{}` handles its failure with `{combinator}`",
                    function.name
                ));
            }
        }
    }
    violations
}

/// The offsets of a `[` that directly follows an identifier character, `)` or `]`: an index or
/// slice expression.
fn index_expressions(lib: &str) -> Vec<usize> {
    lib.char_indices()
        .filter(|(at, character)| {
            *character == '['
                && lib[..*at]
                    .chars()
                    .next_back()
                    .is_some_and(|before| before.is_alphanumeric() || "_)]".contains(before))
        })
        .map(|(at, _)| at)
        .collect()
}

/// The text of the emitted `pub enum name { .. }`.
fn emitted_enum<'l>(lib: &'l str, name: &str) -> &'l str {
    let header = format!("pub enum {name} {{");
    let open = lib
        .find(&header)
        .unwrap_or_else(|| panic!("the emitted source defines no `{header}`"))
        + header.len()
        - 1;
    let close = matching_close(lib, open, '{', '}').expect("enum body closes");
    &lib[open..=close]
}

/// The `src/lib.rs` the generator returns for the corpus request
/// (every generated item) holds no panic token, no silent fallback, and no index or slice
/// expression.
#[trace("FR-018-AC-17", "TC-029")]
#[test]
fn tc_029_ac17_emitted_source_has_no_panic_site_fallback_or_index_expression() {
    let oracles = corpus_oracles();
    let lib = contents(&oracles, "src/lib.rs");
    assert!(
        lib.matches("pub fn oracle_").count() > 1,
        "the corpus must generate more than one item"
    );
    assert_eq!(panic_tokens_in(lib), Vec::<String>::new());
    for fallback in [".ok()", ".unwrap_or(", ".unwrap_or_default("] {
        assert!(
            !lib.contains(fallback),
            "the emitted source holds `{fallback}`"
        );
    }
    assert_eq!(
        index_expressions(lib),
        Vec::<usize>::new(),
        "an index or slice expression in the emitted source"
    );
}

/// Every reconstruction in the corpus crate goes through a helper
/// returning `Result<_, ReconstructionError>`, every call of one is propagated or refused, no
/// runtime constructor or integer parse is called outside a helper, and the two error enums have
/// the variants the criterion lists.
#[trace("FR-018-AC-17", "TC-029")]
#[test]
fn tc_029_ac17_emitted_reconstruction_is_typed_and_refused_never_unwrapped() {
    let oracles = corpus_oracles();
    let lib = contents(&oracles, "src/lib.rs");
    let fns = emitted_fns(lib);
    for helper in REBUILD_HELPERS {
        assert!(
            fns.iter().any(|function| function.name == helper),
            "the corpus crate does not exercise `{helper}`"
        );
    }
    assert_eq!(structural_violations(lib), Vec::<String>::new());
    // The refusal paths exist: the environment constructors map a failure to
    // `EnvironmentError::Reconstruction`, and the oracle functions to `CheckedInvariant`.
    let environments: Vec<_> = fns
        .iter()
        .filter(|function| function.name.starts_with("environment_"))
        .collect();
    assert!(!environments.is_empty());
    for function in environments {
        assert!(function
            .signature
            .contains("Result<rt::TypeEnvironment, EnvironmentError>"));
        assert!(function
            .body
            .contains("Err(error) => return Err(EnvironmentError::Reconstruction(error))"));
    }
    for function in fns
        .iter()
        .filter(|function| function.name.starts_with("oracle_"))
    {
        assert!(function
            .body
            .contains("Err(_) => return rt::Outcome::Refused(rt::Refusal::CheckedInvariant)"));
    }

    let environment_error = emitted_enum(lib, "EnvironmentError");
    assert!(environment_error.contains("Declaration(rt::InvalidDeclaration),"));
    assert!(environment_error.contains("Reconstruction(ReconstructionError),"));
    let variants: Vec<&str> = emitted_enum(lib, "ReconstructionError")
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with("///") && *line != "{" && *line != "}")
        .collect();
    assert_eq!(
        variants,
        [
            "Integer,",
            "Interval,",
            "Rational,",
            "Decimal,",
            "Text,",
            "Cardinality,"
        ],
        "ReconstructionError must have exactly these unit variants"
    );
}

/// The structural checker names each way a source can depart from
/// the criterion, so the corpus test above is a check that can fail.
#[trace("FR-018-AC-17", "TC-029")]
#[test]
fn tc_029_ac17_the_structural_checker_names_each_departure() {
    let wrong_return = "fn rebuild_interval(a: &str) -> rt::IntegerInterval {\n    todo_body\n}\n";
    assert!(structural_violations(wrong_return)
        .iter()
        .any(|violation| violation.contains("does not return Result")));

    let unpropagated = "fn rebuild_text() -> Result<rt::TextType, ReconstructionError> {\n    Err(ReconstructionError::Text)\n}\n\
                        fn composites_x() -> Result<Vec<u8>, ReconstructionError> {\n    let t = rebuild_text();\n    Ok(Vec::new())\n}\n";
    assert!(structural_violations(unpropagated)
        .iter()
        .any(|violation| violation.contains("not followed by `?`")));

    let silent_match = "fn rebuild_text() -> Result<rt::TextType, ReconstructionError> {\n    Err(ReconstructionError::Text)\n}\n\
                        fn composites_x() -> Result<Vec<u8>, ReconstructionError> {\n    let t = match rebuild_text() {\n        Ok(t) => t,\n        Err(_) => rt::TextType::unbounded(),\n    };\n    Ok(Vec::new())\n}\n";
    assert!(structural_violations(silent_match)
        .iter()
        .any(|violation| violation.contains("not followed by `?`")));

    let inline_constructor = "fn composites_x() -> Result<Vec<u8>, ReconstructionError> {\n    let t = rt::TextType::new(0, 1, p);\n    Ok(Vec::new())\n}\n";
    assert!(structural_violations(inline_constructor)
        .iter()
        .any(|violation| violation.contains("outside a reconstruction helper")));

    let inline_parse = "fn oracle_x() -> u8 {\n    \"1\".parse()\n}\n";
    assert!(structural_violations(inline_parse)
        .iter()
        .any(|violation| violation.contains(".parse(")));

    let helper = |name: &str, body: &str| {
        format!("fn {name}(a: u64) -> Result<rt::X, ReconstructionError> {{\n    {body}\n}}\n")
    };
    let wrong_variant = helper(
        "rebuild_text",
        "rt::TextType::new(a).map_err(|_| ReconstructionError::Cardinality)",
    );
    assert!(structural_violations(&wrong_variant)
        .iter()
        .any(|violation| violation.contains("exactly ReconstructionError::Text")));
    let silent_widen = helper(
        "rebuild_cardinality",
        "rt::CardinalityBound::new(a, 1).or_else(|_| rt::CardinalityBound::new(0, u64::MAX)).map_err(|_| ReconstructionError::Cardinality)",
    );
    assert!(structural_violations(&silent_widen)
        .iter()
        .any(|violation| violation.contains("`or_else`")));
    let swallowed = helper(
        "rebuild_rational",
        "Ok(rt::RationalDomain::new(a).ok().unwrap_or_default()) // map_err(|_| ReconstructionError::Rational)",
    );
    assert!(structural_violations(&swallowed)
        .iter()
        .any(|violation| violation.contains("`.ok(`")));
    let good = helper(
        "rebuild_text",
        "rt::TextType::new(a).map_err(|_| ReconstructionError::Text)",
    );
    assert_eq!(structural_violations(&good), Vec::<String>::new());

    assert_eq!(
        index_expressions("let a = v[0]; let b = f()[1]; let c = x[1][2];").len(),
        4
    );
    assert!(index_expressions("let a = vec![1]; let b = f([1]); let c = [1];").is_empty());
}

/// The composite-equality generator's non-test code holds no panic
/// token, counting the string literals it emits.
#[trace("FR-018-AC-19", "TC-029")]
#[test]
fn tc_029_ac19_the_equality_generator_source_holds_no_panic_token() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/oracle/equality/mod.rs");
    let source = fs::read_to_string(path).expect("read the generator source");
    let code = non_test_code(&source);
    // The scan must have removed the test module and nothing before it: the last generator
    // function sits just above the module, and the module's own tests are gone.
    assert!(code.contains("fn profile_path(") && code.contains("fn render_value_type("));
    assert!(!code.contains("mod tests") && !code.contains("fn tc_029_"));
    assert_eq!(
        panic_tokens_in(&code),
        Vec::<String>::new(),
        "src/oracle/equality/mod.rs"
    );
}
