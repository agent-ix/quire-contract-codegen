//! Native generated-ABI acceptance uses genuine source compilation, never synthetic keys.
use ix_trace_rs::trace;
use quire_contract_codegen::{
    generate_composite_equality_oracles, ClaimDisposition, CompositeEqualityItem,
    CompositeEqualityOracles, ContractDriverResultSelection, ContractRequestPosition,
    EqualityOperandDescriptor, EqualityOperatorKind, CompositeEqualityRefusal, IllTypedCauseKind,
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

fn checked(compiled: &qsl_replay::AdmittedCompilation, libraries: &[(String, qsl_replay::AdmittedCompilation)]) -> CheckedPackageV2 {
    let mut evidence = CheckedPackageEvidence::new();
    evidence.support_feature("quire.value.complete/v1");
    for (position, (identity, library)) in libraries.iter().enumerate() {
        let admitted = checked(library, &libraries[..position]);
        evidence.insert_dependency_package(identity.clone(), admitted).expect("retain actual original library");
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

fn application(package: &CheckedPackageV2, function: &str) -> CheckedNodeId {
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

fn generate(compiled: &qsl_replay::AdmittedCompilation, libraries: &[(String, qsl_replay::AdmittedCompilation)]) -> CompositeEqualityOracles {
    let checked = checked(compiled, libraries);
    let mut items = Vec::new();
    for (function, name, operator) in CASES {
        let admitted = compiled
            .context()
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
    generate_items(&checked, compiled.context(), &items, &[]).expect("real enum generation")
}

/// Lower only the unchanged authentic package. Extra roots select genuine caller types;
/// no graph or provenance fields are rewritten to manufacture negative inputs.
fn generate_items(
    checked: &CheckedPackageV2,
    context: &qsl_replay::AdmittedContext,
    items: &[CompositeEqualityItem],
    extra_roots: &[CheckedNodeId],
) -> Result<CompositeEqualityOracles, OracleGenerationError> {
    let roots = items
        .iter()
        .map(|item| item.node_id.clone())
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
    requested.extend(extra_roots.iter().cloned().map(|node_id| {
        ContractRequestPosition::ContextRoot { node_id }
    }));
    generate_composite_equality_oracles(
        &package,
        context,
        &requested,
        &(0..items.len()).collect::<Vec<_>>(),
        items,
        Default::default(),
    )
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
    let ordered = oracles.claim_map.items.iter().find(|claim| claim.node_id == ordered_node)
        .expect("ordered same claim");
    let ClaimDisposition::Generated(ordered) = &ordered.result else { panic!("ordered claim refused") };
    let symbol = ordered.oracle_symbol.strip_prefix("oracle_").expect("oracle symbol prefix");
    let descriptor = format!("let left_source = left_source_{symbol}(context, &mut enum_members)?;");
    let source = &oracles.artifacts.iter().find(|artifact| artifact.path == "src/lib.rs")
        .expect("library").contents;
    let case_lookup = "lookup_enum_case(context, wire, case.case())?";
    let mutations = [
        ("operator", "environment.check_equality(sv::EqualityOperator::Equal".to_owned(),
            "environment.check_equality(sv::EqualityOperator::NotEqual".to_owned()),
        ("descriptor", descriptor, "let left_source = exact::ValueType::Boolean;".to_owned()),
        ("case", case_lookup.to_owned(),
            "lookup_enum_case(context, wire, &exact::Identifier::new(\"READY\").map_err(|_| ReconstructionError::Enum)?)?".to_owned()),
    ];
    for (mutation, from, to) in mutations {
        assert!(source.contains(&from), "mutation must change actual emitted code: {mutation}");
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
        package.graph().nodes.iter().find(|node| {
            CheckedNodeTag::from_wire(&node.node_tag) == Some(CheckedNodeTag::ScalarType)
                && node.semantic_form.as_ref() == form
        }).expect("actual source emits this scalar type").node_id.clone()
    };
    let integer = scalar("integer");
    (CompositeEqualityItem {
        node_id: application(package, "integer_same"),
        operator: EqualityOperatorKind::Equal,
        left: EqualityOperandDescriptor::typed(integer.clone()),
        right: EqualityOperandDescriptor::typed(integer),
    }, scalar("boolean"))
}

fn only_result(oracles: &CompositeEqualityOracles) -> &ClaimDisposition<quire_contract_codegen::GeneratedCompositeEqualityClaim, CompositeEqualityRefusal> {
    assert_eq!(oracles.claim_map.items.len(), 1);
    &oracles.claim_map.items[0].result
}

/// The source/body/context stay authentic; only the caller's descriptor disagrees.
#[trace("FR-018-AC-15", "TC-029")]
#[test]
fn authentic_operand_descriptor_mismatch_retains_left_then_right_refusal() {
    let compiled = fixture::compile(INTEGER_SOURCE);
    let package = checked(&compiled, &[]);
    let (valid, boolean) = integer_item(&package);
    let healthy = generate_items(&package, compiled.context(), std::slice::from_ref(&valid), &[])
        .expect("healthy authentic equality");
    assert!(matches!(only_result(&healthy), ClaimDisposition::Generated(_)));
    for position in [0, 1] {
        let mut bad = valid.clone();
        bad.right.source_type = boolean.clone();
        if position == 0 {
            bad.left.source_type = boolean.clone();
        }
        let output = generate_items(&package, compiled.context(), &[bad], std::slice::from_ref(&boolean))
            .expect("descriptor disagreement is a per-item refusal");
        assert!(matches!(only_result(&output), ClaimDisposition::Refused { refusal:
            CompositeEqualityRefusal::OperandTypeMismatch { position: actual, expected, found }
        } if *actual == position && expected == &boolean && found.as_ref() == Some(&valid.left.source_type)));
        assert_no_oracle(&output);
    }
}

/// An independently declared conversion target does not change the admitted source expression.
#[trace("FR-018-AC-5", "TC-029")]
#[test]
fn authentic_integer_to_boolean_caller_conversion_is_ill_typed() {
    let compiled = fixture::compile(INTEGER_SOURCE);
    let package = checked(&compiled, &[]);
    let (mut item, boolean) = integer_item(&package);
    let healthy = generate_items(&package, compiled.context(), std::slice::from_ref(&item), &[])
        .expect("same source without a caller conversion is healthy");
    assert!(matches!(only_result(&healthy), ClaimDisposition::Generated(_)));
    item.left.conversion_target = Some(boolean.clone());
    let output = generate_items(&package, compiled.context(), &[item], &[boolean])
        .expect("disallowed caller conversion is a per-item refusal");
    assert!(matches!(only_result(&output), ClaimDisposition::Refused { refusal:
        CompositeEqualityRefusal::IllTyped { cause: IllTypedCauseKind::TypeMismatch }
    }));
    assert_no_oracle(&output);
}

fn assert_no_oracle(oracles: &CompositeEqualityOracles) {
    let source = &oracles.artifacts.iter().find(|item| item.path == "src/lib.rs")
        .expect("emitted library").contents;
    assert!(!source.contains("pub fn oracle_"), "refused item emits no oracle");
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
    let result = generate_items(&package, other.context(), &[item], &[]);
    match result {
        Err(OracleGenerationError::ContextPackageMismatch {
            expected: found_expected, actual_domain, actual_digest,
        }) => {
            assert_eq!(found_expected.domain.as_ref(), expected.domain().as_str());
            assert_eq!(found_expected.digest.as_ref(), expected.hex());
            assert_eq!(actual_domain, actual.domain().as_str());
            assert_eq!(actual_digest, actual.hex());
        }
        other => panic!("expected whole-generation context refusal without artifacts, got {other:?}"),
    }
}
