//! Native generated-ABI acceptance uses genuine source compilation, never synthetic keys.
use ix_trace_rs::trace;
use quire_contract_codegen::{
    generate_composite_equality_oracles, ClaimDisposition, CompositeEqualityItem,
    CompositeEqualityOracles, CompositeEqualityRefusal, ContractDriverResultSelection,
    ContractRequestPosition, EqualityOperandDescriptor, EqualityOperatorKind, IllTypedCauseKind,
    OracleGenerationError,
};
use quire_contract_model::{
    CheckedNodeId, CheckedNodeTag, CheckedPackageEvidence, CheckedPackageReadLimits,
    CheckedPackageV2, CheckedPackageV2ReadResult, CompleteLoweringProfileV2, ContractDependencies,
    ContractPackageReadResult, ContractPackageWireV1, LoweringProfileRef,
};

use super::admitted_enum_fixture as fixture;

const CASES: [(&str, &str, EqualityOperatorKind); 4] = [
    ("ordered_same", "Ordered", EqualityOperatorKind::Equal),
    (
        "ordered_different",
        "Ordered",
        EqualityOperatorKind::NotEqual,
    ),
    ("unordered_same", "Unordered", EqualityOperatorKind::Equal),
    ("other_same", "Other", EqualityOperatorKind::Equal),
];

pub(super) fn checked(
    compiled: &qsl_replay::AdmittedCompilation,
    libraries: &[(String, qsl_replay::AdmittedCompilation)],
) -> CheckedPackageV2 {
    let mut evidence = CheckedPackageEvidence::new();
    evidence.support_feature("quire.value.complete/v1");
    for (position, (identity, library)) in libraries.iter().enumerate() {
        let admitted = checked(library, &libraries[..position]);
        evidence
            .insert_dependency_package(identity.clone(), admitted)
            .expect("retain actual original library");
    }

    match CheckedPackageV2::read(
        compiled.package().bytes(),
        CheckedPackageReadLimits::bounded(),
        &evidence,
    ) {
        CheckedPackageV2ReadResult::Admitted(package) => *package,
        other => panic!("real emitted package must admit: {other:?}"),
    }
}

pub(super) fn application(package: &CheckedPackageV2, function: &str) -> CheckedNodeId {
    let declaration = package
        .graph()
        .nodes
        .iter()
        .find(|node| {
            node.declaration
                .as_ref()
                .is_some_and(|decl| decl.qualified_name == [Box::<str>::from(function)])
        })
        .expect("real function declaration");
    let regions = package
        .source_map()
        .iter()
        .filter(|entry| entry.node_id == declaration.node_id)
        .flat_map(|entry| &entry.regions)
        .collect::<Vec<_>>();
    package
        .graph()
        .nodes
        .iter()
        .find(|node| {
            node.semantic_form.as_ref() == "binary"
                && package
                    .source_map()
                    .iter()
                    .filter(|entry| entry.node_id == node.node_id)
                    .any(|entry| {
                        entry.regions.iter().any(|region| {
                            regions.iter().any(|outer| {
                                outer.source == region.source
                                    && outer.start <= region.start
                                    && region.end <= outer.end
                            })
                        })
                    })
        })
        .expect("the function's real equality expression")
        .node_id
        .clone()
}

fn generate(
    compiled: &qsl_replay::AdmittedCompilation,
    libraries: &[(String, qsl_replay::AdmittedCompilation)],
) -> CompositeEqualityOracles {
    let checked = checked(compiled, libraries);
    let items = enum_items(&checked, compiled.context());
    generate_items(&checked, Some(compiled.context()), &items, &[]).expect("real enum generation")
}

fn enum_items(
    checked: &CheckedPackageV2,
    context: &qsl_replay::AdmittedContext,
) -> Vec<CompositeEqualityItem> {
    let mut items = Vec::new();
    for (function, name, operator) in CASES {
        let admitted = context
            .enums()
            .find(|item| {
                item.qualified_name()
                    .iter()
                    .map(|part| part.as_str())
                    .eq([name])
            })
            .expect("real declared enum");
        let type_id = checked
            .graph()
            .nodes
            .iter()
            .find(|node| node.node_id.digest.as_ref() == admitted.key().to_string())
            .expect("IR retains that exact admitted enum declaration")
            .node_id
            .clone();
        items.push(CompositeEqualityItem {
            node_id: application(&checked, function),
            operator,
            left: EqualityOperandDescriptor::typed(type_id.clone()),
            right: EqualityOperandDescriptor::typed(type_id),
        });
    }
    items
}

/// Lower only the unchanged authentic package. Extra roots select genuine caller types;
/// no graph or provenance fields are rewritten to manufacture negative inputs.
pub(super) fn generate_items(
    checked: &CheckedPackageV2,
    context: Option<&qsl_replay::AdmittedContext>,
    items: &[CompositeEqualityItem],
    extra_roots: &[CheckedNodeId],
) -> Result<CompositeEqualityOracles, OracleGenerationError> {
    generate_items_with_limits(checked, context, items, extra_roots, Default::default())
}

pub(super) fn generate_items_with_limits(
    checked: &CheckedPackageV2,
    context: Option<&qsl_replay::AdmittedContext>,
    items: &[CompositeEqualityItem],
    extra_roots: &[CheckedNodeId],
    limits: quire_contract_codegen::CompositeEqualityLimits,
) -> Result<CompositeEqualityOracles, OracleGenerationError> {
    let mut roots = items
        .iter()
        .map(|item| item.node_id.clone())
        .collect::<Vec<_>>();
    roots.sort();
    let mut available = std::collections::BTreeMap::new();
    for (position, node_id) in roots.iter().enumerate() {
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
                .expect("canonical request retains every descriptor's source item")
        })
        .collect::<Vec<_>>();
    let profile = CompleteLoweringProfileV2 {
        reference: LoweringProfileRef {
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
    };
    let lowering_roots = roots.iter().chain(extra_roots).cloned().collect::<Vec<_>>();
    let lowered = checked.lower(&lowering_roots, &profile);
    let package = match ContractPackageWireV1::read(
        lowered
            .package
            .canonical_bytes()
            .expect("actual lowered bytes"),
        CheckedPackageReadLimits::bounded(),
        &ContractDependencies::new(),
    ) {
        ContractPackageReadResult::Admitted(package) => package,
        other => panic!("lowered real source must admit: {other:?}"),
    };
    let mut requested = roots
        .into_iter()
        .map(|node_id| ContractRequestPosition::DriverItem {
            node_id,
            result_selection: ContractDriverResultSelection::NotApplicable,
        })
        .collect::<Vec<_>>();
    requested.extend(
        extra_roots
            .iter()
            .cloned()
            .map(|node_id| ContractRequestPosition::ContextRoot { node_id }),
    );
    generate_composite_equality_oracles(&package, context, &requested, &positions, items, limits)
}

#[trace(
    "FR-018-AC-33",
    "FR-018-AC-34",
    "FR-018-AC-36",
    "FR-018-AC-37",
    "TC-029"
)]
#[test]
fn genuine_enum_generated_native_abi_agrees_with_owner() {
    for (mode, transitive) in [("direct", false), ("transitive", true)] {
        let (compiled, libraries) = fixture::imported(transitive);
        assert!(
            !fixture::SOURCE.contains("::"),
            "all enum cases are absent as literals"
        );
        let oracles = generate(&compiled, &libraries);
        assert_eq!(oracles.claim_map.items.len(), CASES.len());
        let mut names = String::new();
        for ((function, _, _), item) in CASES.into_iter().zip(&oracles.claim_map.items) {
            // Claim order is descriptor-key order; locate each claim by its actual function expression.
            let node = application(&checked(&compiled, &libraries), function);
            let claim = oracles
                .claim_map
                .items
                .iter()
                .find(|entry| entry.node_id == node)
                .expect("claim for real expression");
            let ClaimDisposition::Generated(generated) = &claim.result else {
                panic!("real enum refused: {:?}", item.result)
            };
            names.push_str(&format!(
                "pub use self::{{{} as environment_{function}, {} as oracle_{function}}};\n",
                generated.environment_symbol, generated.oracle_symbol
            ));
        }
        let manifest = oracles
            .artifacts
            .iter()
            .find(|artifact| artifact.path == "Cargo.toml")
            .expect("manifest");
        assert!(manifest.contents.contains("qsl-replay ="));
        assert!(!manifest.contents.contains("test-support"));
        super::exact_scalar_agreement::run_agreement_cases(
            &format!("admitted-enum-equality-{mode}"),
            oracles
                .artifacts
                .iter()
                .map(|artifact| (artifact.path.as_str(), artifact.contents.as_str())),
            &[("names.rs", names.as_str()), ("context-mode", mode)],
            "tests/admitted_enum_support/agreement_cases.rs",
        );
        let ordered_node = application(&checked(&compiled, &libraries), "ordered_same");
        let ordered = oracles
            .claim_map
            .items
            .iter()
            .find(|claim| claim.node_id == ordered_node)
            .expect("ordered same claim");
        let ClaimDisposition::Generated(ordered) = &ordered.result else {
            panic!("ordered claim refused")
        };
        let symbol = ordered
            .oracle_symbol
            .strip_prefix("oracle_")
            .expect("oracle symbol prefix");
        let descriptor =
            format!("let left_source = left_source_{symbol}(context, &mut enum_members)?;");
        let source = &oracles
            .artifacts
            .iter()
            .find(|artifact| artifact.path == "src/lib.rs")
            .expect("library")
            .contents;
        let case_lookup = "lookup_enum_case(Some(context), wire, case.case())?";
        let mutations = [
        ("operator", "environment.check_equality(sv::EqualityOperator::Equal".to_owned(),
            "environment.check_equality(sv::EqualityOperator::NotEqual".to_owned()),
        ("descriptor", descriptor, "let left_source = exact::ValueType::Boolean;".to_owned()),
        ("case", case_lookup.to_owned(),
            "lookup_enum_case(Some(context), wire, &exact::Identifier::new(\"READY\").map_err(|_| ReconstructionError::Enum)?)?".to_owned()),
    ];
        for (mutation, from, to) in mutations {
            assert!(
                source.contains(&from),
                "mutation must change actual emitted code: {mutation}"
            );
            let changed = source.replacen(&from, &to, 1);
            assert_ne!(&changed, source);
            super::exact_scalar_agreement::run_agreement_mutant(
            &format!("admitted-enum-equality-{mode}-{mutation}"),
            oracles.artifacts.iter().map(|artifact| (artifact.path.as_str(),
                if artifact.path == "src/lib.rs" { changed.as_str() } else { artifact.contents.as_str() })),
            &[("names.rs", names.as_str()), ("context-mode", mode)],
            "tests/admitted_enum_support/agreement_cases.rs",
            "complete_genuine_enum_pairs_match_outcomes_ordered_charges_counters_and_every_denial",
        );
        }
    }
}

const INTEGER_SOURCE: &str = "language \"ix:native\" edition \"1-draft\";\n\
profile v = \"quire.value.complete/v1\";\n\
function integer_same using v(a: Integer,b: Integer): Boolean pure { a = b }\n";

fn integer_item(package: &CheckedPackageV2) -> (CompositeEqualityItem, CheckedNodeId) {
    let scalar = |form: &str| {
        package
            .graph()
            .nodes
            .iter()
            .find(|node| {
                CheckedNodeTag::from_wire(&node.node_tag) == Some(CheckedNodeTag::ScalarType)
                    && node.semantic_form.as_ref() == form
            })
            .expect("actual source emits this scalar type")
            .node_id
            .clone()
    };
    let integer = scalar("integer");
    (
        CompositeEqualityItem {
            node_id: application(package, "integer_same"),
            operator: EqualityOperatorKind::Equal,
            left: EqualityOperandDescriptor::typed(integer.clone()),
            right: EqualityOperandDescriptor::typed(integer),
        },
        scalar("boolean"),
    )
}

fn only_result(
    oracles: &CompositeEqualityOracles,
) -> &ClaimDisposition<
    quire_contract_codegen::GeneratedCompositeEqualityClaim,
    CompositeEqualityRefusal,
> {
    assert_eq!(oracles.claim_map.items.len(), 1);
    &oracles.claim_map.items[0].result
}

/// The source/body/context stay authentic; only the caller's descriptor disagrees.
pub(super) fn assert_authentic_operand_descriptor_mismatch() {
    let compiled = fixture::compile(INTEGER_SOURCE);
    let package = checked(&compiled, &[]);
    let (valid, boolean) = integer_item(&package);
    let healthy = generate_items(
        &package,
        Some(compiled.context()),
        std::slice::from_ref(&valid),
        &[],
    )
    .expect("healthy authentic equality");
    assert!(matches!(
        only_result(&healthy),
        ClaimDisposition::Generated(_)
    ));
    for position in [0, 1] {
        let mut bad = valid.clone();
        bad.right.source_type = boolean.clone();
        if position == 0 {
            bad.left.source_type = boolean.clone();
        }
        let output = generate_items(
            &package,
            Some(compiled.context()),
            &[bad],
            std::slice::from_ref(&boolean),
        )
        .expect("descriptor disagreement is a per-item refusal");
        assert!(
            matches!(only_result(&output), ClaimDisposition::Refused { refusal:
            CompositeEqualityRefusal::OperandTypeMismatch { position: actual, expected, found }
        } if *actual == position && expected == &boolean && found.as_ref() == Some(&valid.left.source_type))
        );
        assert_no_oracle(&output);
    }
}

/// An independently declared conversion target does not change the admitted source expression.
pub(super) fn assert_authentic_disallowed_caller_conversion() {
    let compiled = fixture::compile(INTEGER_SOURCE);
    let package = checked(&compiled, &[]);
    let (mut item, boolean) = integer_item(&package);
    let healthy = generate_items(
        &package,
        Some(compiled.context()),
        std::slice::from_ref(&item),
        &[],
    )
    .expect("same source without a caller conversion is healthy");
    assert!(matches!(
        only_result(&healthy),
        ClaimDisposition::Generated(_)
    ));
    item.left.conversion_target = Some(boolean.clone());
    let output = generate_items(&package, Some(compiled.context()), &[item], &[boolean])
        .expect("disallowed caller conversion is a per-item refusal");
    assert!(matches!(
        only_result(&output),
        ClaimDisposition::Refused {
            refusal: CompositeEqualityRefusal::IllTyped {
                cause: IllTypedCauseKind::TypeMismatch
            }
        }
    ));
    assert_no_oracle(&output);
}

fn assert_no_oracle(oracles: &CompositeEqualityOracles) {
    let source = &oracles
        .artifacts
        .iter()
        .find(|item| item.path == "src/lib.rs")
        .expect("emitted library")
        .contents;
    assert!(
        !source.contains("pub fn oracle_"),
        "refused item emits no oracle"
    );
}

#[trace("FR-018-AC-33", "TC-029")]
#[test]
fn public_generator_rejects_another_authentic_source_context() {
    let compiled = fixture::compile(INTEGER_SOURCE);
    let other = fixture::compile(fixture::SOURCE);
    let package = checked(&compiled, &[]);
    let (item, _) = integer_item(&package);
    let expected = compiled.context().package_id().record();
    let actual = other.context().package_id().record();
    assert_ne!(expected, actual, "the two authentic source packages differ");
    let result = generate_items(&package, Some(other.context()), &[item], &[]);
    match result {
        Err(OracleGenerationError::ContextPackageMismatch {
            expected: found_expected,
            actual_domain,
            actual_digest,
        }) => {
            assert_eq!(found_expected.domain.as_ref(), expected.domain().as_str());
            assert_eq!(found_expected.digest.as_ref(), expected.hex());
            assert_eq!(actual_domain, actual.domain().as_str());
            assert_eq!(actual_digest, actual.hex());
        }
        other => {
            panic!("expected whole-generation context refusal without artifacts, got {other:?}")
        }
    }
}

pub(super) fn direct_source_item(
    package: &CheckedPackageV2,
    function: &str,
) -> CompositeEqualityItem {
    let node_id = application(package, function);
    let expression = package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_id == node_id)
        .expect("actual expression");
    let arguments = expression
        .body
        .get("arguments")
        .and_then(serde_json::Value::as_array)
        .expect("actual binary arguments");
    assert_eq!(arguments.len(), 2);
    let operand = |position: usize| {
        assert_eq!(arguments[position]["term"], "reference");
        let target: CheckedNodeId = serde_json::from_value(arguments[position]["target"].clone())
            .expect("actual IR reference ID, never a reconstructed semantic key");
        let parameter = package
            .graph()
            .nodes
            .iter()
            .find(|node| node.node_id == target)
            .expect("original source parameter");
        assert_ne!(parameter.semantic_form.as_ref(), "conversion");
        EqualityOperandDescriptor::typed(parameter.semantic_type.clone())
    };
    CompositeEqualityItem {
        node_id,
        operator: EqualityOperatorKind::Equal,
        left: operand(0),
        right: operand(1),
    }
}

#[trace("FR-018-AC-4", "FR-018-AC-14", "FR-018-AC-38", "TC-029")]
#[test]
fn authentic_text_option_and_primitive_sequence_generate_with_real_bounds() {
    for (kind, expected_schedule, reconstruction) in [
        (
            "Text[0, 16; nfc]",
            quire_contract_codegen::RecordedSchedule::Text,
            "rebuild_text(0, 16,",
        ),
        (
            "Option<Int[0, 9]>",
            quire_contract_codegen::RecordedSchedule::Plan,
            "exact::ValueType::option(",
        ),
        (
            "Sequence<Int[0, 9]>[0, 3]",
            quire_contract_codegen::RecordedSchedule::Plan,
            "rebuild_cardinality(0, 3)?",
        ),
    ] {
        let source = format!("language \"ix:native\" edition \"1-draft\";\nprofile v = \"quire.value.complete/v1\";\nfunction f using v(a: {kind}, b: {kind}): Boolean pure {{ a = b }}\n");
        let compiled = fixture::compile(&source);
        let package = checked(&compiled, &[]);
        let item = direct_source_item(&package, "f");
        let bounds = package
            .graph()
            .nodes
            .iter()
            .filter(|node| {
                CheckedNodeTag::from_wire(&node.node_tag) == Some(CheckedNodeTag::BoundedDomain)
            })
            .map(|node| node.node_id.clone())
            .collect::<Vec<_>>();
        assert!(
            !bounds.is_empty(),
            "source profile carries actual admitted bounds"
        );
        let output = generate_items(&package, None, std::slice::from_ref(&item), &bounds)
            .expect("actual context-free source package");
        let with_context = generate_items(&package, Some(compiled.context()), &[item], &bounds)
            .expect("same primitive recipe with genuine capability");
        assert_eq!(
            output, with_context,
            "presence does not change a context-free recipe"
        );
        let ClaimDisposition::Generated(claim) = only_result(&output) else {
            panic!(
                "source-backed primitive profile refused: {:?}",
                only_result(&output)
            )
        };
        assert_eq!(claim.schedule, expected_schedule);
        let emitted = &output
            .artifacts
            .iter()
            .find(|item| item.path == "src/lib.rs")
            .expect("emitted source")
            .contents;
        assert!(
            emitted.contains(reconstruction),
            "preserve actual source profile reconstruction: {kind}"
        );
        assert!(
            !emitted.contains("NodeKey::"),
            "primitive profiles need no composite key constructor"
        );
    }
}

pub(super) fn authentic_text_oracles() -> CompositeEqualityOracles {
    let compiled = fixture::compile("language \"ix:native\" edition \"1-draft\";\nprofile v = \"quire.value.complete/v1\";\nfunction f using v(a: Text[0, 16; nfc],b: Text[0, 16; nfc]): Boolean pure { a = b }\n");
    let package = checked(&compiled, &[]);
    let item = direct_source_item(&package, "f");
    let bounds = package
        .graph()
        .nodes
        .iter()
        .filter(|node| {
            CheckedNodeTag::from_wire(&node.node_tag) == Some(CheckedNodeTag::BoundedDomain)
        })
        .map(|node| node.node_id.clone())
        .collect::<Vec<_>>();
    generate_items(&package, Some(compiled.context()), &[item], &bounds)
        .expect("genuine source text equality")
}

/// Independent enum/primitive slice; the legacy record/tuple golden corpus remains separate.
#[trace("FR-018-AC-10", "FR-018-AC-11", "TC-029")]
#[test]
fn authentic_enum_request_order_repeatability_and_dual_operator_symbols() {
    let compiled = fixture::compile(fixture::SOURCE);
    let package = checked(&compiled, &[]);
    let mut items = enum_items(&package, compiled.context());
    let mut dual = items[0].clone();
    dual.operator = EqualityOperatorKind::NotEqual;
    items.push(dual);
    let forward = generate_items(&package, Some(compiled.context()), &items, &[]).expect("forward");
    items.reverse();
    let reverse = generate_items(&package, Some(compiled.context()), &items, &[]).expect("reverse");
    assert_eq!(
        forward, reverse,
        "same canonical request gives identical artifacts and claims"
    );
    let fresh = fixture::compile(fixture::SOURCE);
    let fresh_package = checked(&fresh, &[]);
    let repeated = generate_items(&fresh_package, Some(fresh.context()), &items, &[])
        .expect("fresh compilation");
    assert_eq!(forward, repeated);
    let mut symbols = std::collections::BTreeSet::new();
    for claim in &forward.claim_map.items {
        let ClaimDisposition::Generated(generated) = &claim.result else {
            panic!("healthy enum refused")
        };
        assert!(symbols.insert(&generated.oracle_symbol));
        assert!(symbols.insert(&generated.environment_symbol));
        assert_eq!(
            claim.operation.provenance,
            quire_contract_codegen::CompositeOperationProvenance::CallerDeclared {
                blocked_on: quire_contract_codegen::UpstreamBlocker::OperationIdentityNotConsumed,
            }
        );
    }
    let ordered = application(&package, "ordered_same");
    let dual_claims = forward
        .claim_map
        .items
        .iter()
        .filter(|claim| claim.node_id == ordered)
        .collect::<Vec<_>>();
    assert_eq!(
        dual_claims.len(),
        2,
        "same node's two caller operators are distinct items"
    );
    assert_ne!(
        dual_claims[0].operation.identity,
        dual_claims[1].operation.identity
    );
}

fn genuine_asymmetric_integer_equality() -> (
    qsl_replay::AdmittedCompilation,
    CheckedPackageV2,
    CompositeEqualityItem,
) {
    let compiled = fixture::compile("language \"ix:native\" edition \"1-draft\";\nprofile v = \"quire.value.complete/v1\";\nfunction f using v(a: Int[0, 9],b: Integer): Boolean pure { convert<Integer>(a) = b }\n");
    let package = checked(&compiled, &[]);
    assert!(
        !package
            .graph()
            .nodes
            .iter()
            .any(|node| node.semantic_form.as_ref() == "conversion"),
        "this measured source grounds caller descriptors, not a conversion-reference path"
    );
    let source = package
        .graph()
        .nodes
        .iter()
        .find(|node| node.semantic_form.as_ref() == "integer_range")
        .expect("real source's bounded integer type")
        .node_id
        .clone();
    let integer = package
        .graph()
        .nodes
        .iter()
        .find(|node| node.semantic_form.as_ref() == "integer")
        .expect("real Integer type")
        .node_id
        .clone();
    let item = CompositeEqualityItem {
        node_id: application(&package, "f"),
        operator: EqualityOperatorKind::Equal,
        left: EqualityOperandDescriptor::converted(source, integer.clone()),
        right: EqualityOperandDescriptor::typed(integer),
    };
    (compiled, package, item)
}

pub(super) fn assert_authentic_conversion_descriptor() {
    let (compiled, package, item) = genuine_asymmetric_integer_equality();
    let output = generate_items(
        &package,
        Some(compiled.context()),
        std::slice::from_ref(&item),
        &[],
    )
    .expect("actual bounded/base descriptor generation");
    let ClaimDisposition::Generated(claim) = only_result(&output) else {
        panic!(
            "valid caller conversion descriptor refused: {:?}",
            only_result(&output)
        )
    };
    assert_eq!(claim.descriptor.operator, item.operator);
    assert_eq!(claim.descriptor.left_source_type, item.left.source_type);
    assert_eq!(
        claim.descriptor.left_conversion_target,
        item.left.conversion_target
    );
    assert_eq!(claim.descriptor.right_source_type, item.right.source_type);
    assert_eq!(
        claim.descriptor.right_conversion_target,
        item.right.conversion_target
    );
}

/// Regression over the measured genuine source: a bounded sibling never narrows Integer.
#[trace("FR-018-AC-43", "TC-029")]
#[test]
fn genuine_bounded_parameter_keeps_other_integer_and_target_unbounded() {
    let (compiled, package, item) = genuine_asymmetric_integer_equality();
    let output = generate_items(&package, Some(compiled.context()), &[item], &[])
        .expect("same actual source with an admitted caller equality conversion");
    let ClaimDisposition::Generated(claim) = only_result(&output) else {
        panic!("genuine numeric item refused: {:?}", only_result(&output))
    };
    let names = format!(
        "pub use self::{{{} as environment_numeric, {} as oracle_numeric}};\n",
        claim.environment_symbol, claim.oracle_symbol
    );
    super::exact_scalar_agreement::run_agreement_cases(
        "admitted-integer-sibling-bound",
        output
            .artifacts
            .iter()
            .map(|item| (item.path.as_str(), item.contents.as_str())),
        &[("names.rs", names.as_str())],
        "tests/admitted_integer_support/agreement_cases.rs",
    );
}

#[trace("FR-018-AC-39", "FR-018-AC-40", "FR-018-AC-41", "TC-029")]
#[test]
fn optional_capability_absence_is_typed_and_wrong_presence_is_global() {
    let source = format!(
        "{}{}",
        fixture::SOURCE,
        INTEGER_SOURCE.lines().last().expect("integer function")
    );
    let compiled = fixture::compile(&source);
    let other = fixture::compile(INTEGER_SOURCE);
    let package = checked(&compiled, &[]);
    let enums = enum_items(&package, compiled.context());
    let selected = &enums[0];
    let missing = generate_items(&package, None, std::slice::from_ref(selected), &[])
        .expect("absence refuses only the nominal item");
    assert!(
        matches!(only_result(&missing), ClaimDisposition::Refused { refusal:
        CompositeEqualityRefusal::MissingAdmittedContext { type_node_id }
    } if type_node_id == &selected.left.source_type)
    );
    assert_no_oracle(&missing);
    let limited = generate_items_with_limits(
        &package,
        None,
        std::slice::from_ref(selected),
        &[],
        quire_contract_codegen::CompositeEqualityLimits {
            type_resolution_work_units: 0,
            ..Default::default()
        },
    )
    .expect("charged entry refuses before capability demand");
    assert!(matches!(
        only_result(&limited),
        ClaimDisposition::Refused {
            refusal: CompositeEqualityRefusal::TypeResolutionWorkExhausted {
                limit: 0,
                consumed: 1
            }
        }
    ));
    assert_no_oracle(&limited);
    let (integer, _) = integer_item(&package);
    for items in [Vec::new(), vec![integer.clone()], vec![selected.clone()]] {
        let wrong = generate_items(&package, Some(other.context()), &items, &[]);
        assert!(
            matches!(
                wrong,
                Err(OracleGenerationError::ContextPackageMismatch { .. })
            ),
            "Some(wrong) cannot become None, including empty requests: {wrong:?}"
        );
    }
    for left in [true, false] {
        let mut target = integer.clone();
        let descriptor = if left {
            &mut target.left
        } else {
            &mut target.right
        };
        descriptor.conversion_target = Some(selected.left.source_type.clone());
        let result = generate_items(
            &package,
            None,
            &[target.clone()],
            std::slice::from_ref(&selected.left.source_type),
        )
        .expect("actual nominal target reached after primitive source");
        assert!(
            matches!(only_result(&result), ClaimDisposition::Refused { refusal:
            CompositeEqualityRefusal::MissingAdmittedContext { type_node_id }
        } if type_node_id == &selected.left.source_type)
        );
        assert_no_oracle(&result);
        let present = generate_items(
            &package,
            Some(compiled.context()),
            &[target],
            std::slice::from_ref(&selected.left.source_type),
        )
        .expect("genuine lookup succeeds, conversion checker keeps its precise refusal");
        assert!(matches!(
            only_result(&present),
            ClaimDisposition::Refused {
                refusal: CompositeEqualityRefusal::IllTyped {
                    cause: IllTypedCauseKind::TypeMismatch
                }
            }
        ));
    }
    let mixed = generate_items(&package, None, &[selected.clone(), integer], &[])
        .expect("absent nominal capability does not abort primitive sibling");
    assert_eq!(
        mixed
            .claim_map
            .items
            .iter()
            .filter(|claim| matches!(claim.result, ClaimDisposition::Generated(_)))
            .count(),
        1
    );
    assert_eq!(
        mixed
            .claim_map
            .items
            .iter()
            .filter(|claim| matches!(
                claim.result,
                ClaimDisposition::Refused {
                    refusal: CompositeEqualityRefusal::MissingAdmittedContext { .. }
                }
            ))
            .count(),
        1
    );
}

#[trace("FR-018-AC-39", "FR-018-AC-41", "TC-029")]
#[test]
fn nominal_capability_demand_follows_nested_option_and_collection_elements() {
    for kind in ["Option<Ordered>", "Sequence<Ordered>[0, 3]"] {
        let source = format!(
            "{}function nested using v(a: {kind},b: {kind}): Boolean pure {{ a = b }}\n",
            fixture::SOURCE
        );
        let compiled = fixture::compile(&source);
        let package = checked(&compiled, &[]);
        let item = direct_source_item(&package, "nested");
        let enum_id = enum_items(&package, compiled.context())[0]
            .left
            .source_type
            .clone();
        let result = generate_items(&package, None, std::slice::from_ref(&item), &[])
            .expect("outer structural type cannot hide a reached enum");
        assert!(
            matches!(only_result(&result), ClaimDisposition::Refused { refusal:
            CompositeEqualityRefusal::MissingAdmittedContext { type_node_id }
        } if type_node_id == &enum_id)
        );
        assert_no_oracle(&result);
        let present = generate_items(&package, Some(compiled.context()), &[item], &[])
            .expect("genuine capability reconstructs complete nested enum closure");
        assert!(matches!(
            only_result(&present),
            ClaimDisposition::Generated(_)
        ));
    }
}
