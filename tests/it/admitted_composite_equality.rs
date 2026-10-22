// SPDX-License-Identifier: AGPL-3.0-or-later
//! Authentic source-backed record/tuple generation and native owner agreement.
use super::admitted_composite_fixture as fixture;
use super::admitted_enum_equality::{
    checked, direct_source_item, generate_items, generate_items_with_libraries,
};
use ix_trace_rs::trace;
use quire_contract_codegen::{
    ClaimDisposition, CompositeEqualityLimits, CompositeEqualityRefusal, EqualityOperatorKind,
    OracleGenerationError,
};

const FUNCTIONS: [&str; 6] = [
    "pair_same",
    "pair_different",
    "holder_same",
    "holder_different",
    "other_pair_same",
    "other_holder_same",
];

#[trace(
    "FR-018-AC-2",
    "FR-018-AC-8",
    "FR-018-AC-9",
    "FR-018-AC-39",
    "FR-018-AC-42",
    "TC-029"
)]
#[test]
fn genuine_record_tuple_closure_and_native_oracles_use_original_owner_keys() {
    for (mode, transitive) in [("direct", false), ("transitive", true)] {
        let (compiled, libraries) = fixture::imported(transitive);
        let package = checked(&compiled, &libraries);
        let items = FUNCTIONS
            .iter()
            .map(|name| {
                let mut item = direct_source_item(&package, name);
                if name.ends_with("different") {
                    item.operator = EqualityOperatorKind::NotEqual;
                }
                item
            })
            .collect::<Vec<_>>();
        let generated = generate_items_with_libraries(
            &package,
            Some(compiled.context()),
            &items,
            &[],
            Default::default(),
            &libraries,
        )
        .expect("real complete composite closure");
        assert_eq!(generated.claim_map.items.len(), FUNCTIONS.len());
        let mut names = String::new();
        for (name, item) in FUNCTIONS.iter().zip(&items) {
            let claim = generated
                .claim_map
                .items
                .iter()
                .find(|claim| claim.node_id == item.node_id)
                .expect("actual function claim");
            let ClaimDisposition::Generated(claim) = &claim.result else {
                panic!("genuine composite refused: {:?}", claim.result)
            };
            names.push_str(&format!(
                "pub use self::{{{} as environment_{name}, {} as oracle_{name}}};\n",
                claim.environment_symbol, claim.oracle_symbol
            ));
        }
        for artifact in &generated.artifacts {
            if artifact.path.ends_with(".rs") {
                assert!(
                    !artifact.contents.contains("NodeKey::from_"),
                    "emitted keys must come from owner context"
                );
            }
        }
        let reverse = items.iter().rev().cloned().collect::<Vec<_>>();
        assert_eq!(
            generated,
            generate_items_with_libraries(
                &package,
                Some(compiled.context()),
                &reverse,
                &[],
                Default::default(),
                &libraries
            )
            .expect("same canonical request")
        );
        super::exact_scalar_agreement::run_agreement_cases(
            &format!("admitted-composite-equality-{mode}"),
            generated
                .artifacts
                .iter()
                .map(|artifact| (artifact.path.as_str(), artifact.contents.as_str())),
            &[("names.rs", names.as_str()), ("context-mode", mode)],
            "tests/admitted_composite_support/agreement_cases.rs",
        );
        let selected = &items[0];
        let absent = generate_items_with_libraries(
            &package,
            None,
            std::slice::from_ref(selected),
            &[],
            Default::default(),
            &libraries,
        )
        .expect("typed missing capability");
        assert!(
            matches!(&absent.claim_map.items[0].result, ClaimDisposition::Refused {
            refusal: CompositeEqualityRefusal::MissingAdmittedContext { type_node_id }
        } if type_node_id == &selected.left.source_type)
        );
        let exhausted = generate_items_with_libraries(
            &package,
            None,
            std::slice::from_ref(selected),
            &[],
            CompositeEqualityLimits {
                type_resolution_work_units: 0,
                ..Default::default()
            },
            &libraries,
        )
        .expect("work refusal before capability lookup");
        assert!(matches!(
            &exhausted.claim_map.items[0].result,
            ClaimDisposition::Refused {
                refusal: CompositeEqualityRefusal::TypeResolutionWorkExhausted {
                    limit: 0,
                    consumed: 1
                }
            }
        ));
        let other = fixture::compile(fixture::SOURCE, &qsl_replay::DependencyInput::default());
        assert_ne!(
            other.context().package_id(),
            compiled.context().package_id()
        );
        for requested in [Vec::new(), vec![selected.clone()]] {
            assert!(matches!(
                generate_items_with_libraries(
                    &package,
                    Some(other.context()),
                    &requested,
                    &[],
                    Default::default(),
                    &libraries
                ),
                Err(OracleGenerationError::ContextPackageMismatch { .. })
            ));
        }
    }
}

#[trace("FR-018-AC-10", "FR-018-AC-11", "TC-029")]
#[test]
fn same_shaped_record_and_tuple_names_keep_distinct_repeatable_owner_identities() {
    let dependencies = qsl_replay::DependencyInput::default();
    let compiled = fixture::compile(fixture::SOURCE, &dependencies);
    let fresh = fixture::compile(fixture::SOURCE, &dependencies);
    let declarations = compiled.context().composites().collect::<Vec<_>>();
    assert_eq!(declarations.len(), 4);
    let keys = declarations
        .iter()
        .map(|item| item.key())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(keys.len(), 4, "same-shaped declarations are not aliases");
    for actual in declarations {
        let wire = qsl_replay::WireNodeId::from_digest(*actual.node().as_bytes());
        assert_eq!(
            compiled
                .context()
                .composite_declaration(wire)
                .expect("repeat lookup"),
            actual
        );
        assert_eq!(
            fresh
                .context()
                .composite_declaration(wire)
                .expect("same real source"),
            actual
        );
    }
}

/// Retain source, complete package and capability together; no expressions are appended or rekeyed.
pub(super) fn assert_authentic_recursive_item_generation() {
    let source = concat!(
        "language \"ix:native\" edition \"1-draft\";\nprofile v = \"quire.value.complete/v1\";\n",
        "record List { next: List?; }\nrecord Tree { kids: Sequence<Tree>[0, 3]; }\n",
        "function list_same using v(a: List,b: List): Boolean pure { a = b }\n",
        "function tree_same using v(a: Tree,b: Tree): Boolean pure { a = b }\n",
    );
    let compiled = fixture::compile(source, &qsl_replay::DependencyInput::default());
    let package = checked(&compiled, &[]);
    let batch = ["list_same", "tree_same"].map(|function| direct_source_item(&package, function));
    let both = generate_items(&package, Some(compiled.context()), &batch, &[])
        .expect("both original recursive equalities lower together");
    assert_eq!(both.claim_map.items.len(), 2);
    assert!(both
        .claim_map
        .items
        .iter()
        .all(|claim| matches!(claim.result, ClaimDisposition::Generated(_))));
    let batch_source = &both
        .artifacts
        .iter()
        .find(|artifact| artifact.path == "src/lib.rs")
        .unwrap()
        .contents;
    assert_eq!(batch_source.matches("pub fn oracle_").count(), 2);
    assert_eq!(batch_source.matches("pub fn environment_").count(), 2);
    for (function, name, field, optional) in [
        ("list_same", "List", "next", true),
        ("tree_same", "Tree", "kids", false),
    ] {
        let item = direct_source_item(&package, function);
        let actual = compiled
            .context()
            .composites()
            .find(|declaration| {
                declaration
                    .qualified_name()
                    .iter()
                    .map(|part| part.as_str())
                    .eq([name])
            })
            .expect("genuine recursive source declaration");
        assert_eq!(actual.form(), qsl_replay::AdmittedCompositeForm::Record);
        assert_eq!(
            actual.node().to_string(),
            item.left.source_type.digest.as_ref()
        );
        assert_ne!(
            actual.key(),
            actual.node(),
            "wire identity is not the kernel declaration key"
        );
        assert_eq!(item.left.source_type, item.right.source_type);
        let output = generate_items(
            &package,
            Some(compiled.context()),
            std::slice::from_ref(&item),
            &[],
        )
        .expect("actual source and original capability lower together");
        assert_eq!(output.claim_map.items.len(), 1);
        let ClaimDisposition::Generated(claim) = &output.claim_map.items[0].result else {
            panic!(
                "authentic recursive item refused: {:?}",
                output.claim_map.items[0].result
            )
        };
        assert_eq!(claim.declaration_keys, vec![item.left.source_type.clone()]);
        assert_eq!(
            claim.declaration_runtime_keys,
            vec![actual.key().to_string()]
        );
        let lib = &output
            .artifacts
            .iter()
            .find(|artifact| artifact.path == "src/lib.rs")
            .expect("emitted library")
            .contents;
        assert_eq!(lib.matches("pub fn oracle_").count(), 1);
        assert_eq!(lib.matches("pub fn environment_").count(), 1);
        assert!(lib.contains(&claim.oracle_symbol));
        let recipe = format!("lookup_composite_key(_context, {:?}, &[{name:?}], qsl_replay::AdmittedCompositeForm::Record)?", item.left.source_type.digest);
        assert!(lib.contains(&format!("sv::CompositeDeclaration::new({recipe}, ")));
        assert!(lib.contains(&format!("sv::FieldDeclaration::new({field:?}, {{")));
        assert!(lib.contains(&format!("exact::ValueType::Composite({recipe})")));
        let presence = if optional {
            "exact::Presence::Optional)"
        } else {
            "exact::Presence::Required)"
        };
        assert!(lib.contains(presence));
        if !optional {
            assert!(lib.contains("exact::CollectionKind::Sequence"));
            assert!(lib.contains("rebuild_cardinality(0, 3)?"));
        }
        assert!(!lib.contains("NodeKey::from_"));
    }
}

#[trace("FR-018-AC-2", "FR-018-AC-8", "FR-018-AC-39", "FR-018-AC-42", "TC-029")]
#[test]
fn genuine_composite_host_generation_preserves_closure_and_capability_ordering() {
    for transitive in [false, true] {
        let (compiled, libraries) = fixture::imported(transitive);
        let package = checked(&compiled, &libraries);
        let items = FUNCTIONS
            .iter()
            .map(|name| {
                let mut item = direct_source_item(&package, name);
                if name.ends_with("different") {
                    item.operator = EqualityOperatorKind::NotEqual;
                }
                item
            })
            .collect::<Vec<_>>();
        let generated = generate_items_with_libraries(
            &package,
            Some(compiled.context()),
            &items,
            &[],
            Default::default(),
            &libraries,
        )
        .expect("real complete composite closure");
        assert_eq!(generated.claim_map.items.len(), FUNCTIONS.len());
        for claim in &generated.claim_map.items {
            assert!(
                matches!(claim.result, ClaimDisposition::Generated(_)),
                "genuine closure refused: {:?}",
                claim.result
            );
        }
        for artifact in &generated.artifacts {
            if artifact.path.ends_with(".rs") {
                assert!(
                    !artifact.contents.contains("NodeKey::from_"),
                    "emitted keys must come from owner context"
                );
            }
        }
        let reverse = items.iter().rev().cloned().collect::<Vec<_>>();
        assert_eq!(
            generated,
            generate_items_with_libraries(
                &package,
                Some(compiled.context()),
                &reverse,
                &[],
                Default::default(),
                &libraries
            )
            .expect("same canonical request")
        );
        let selected = &items[0];
        let absent = generate_items_with_libraries(
            &package,
            None,
            std::slice::from_ref(selected),
            &[],
            Default::default(),
            &libraries,
        )
        .expect("typed missing capability");
        assert!(
            matches!(&absent.claim_map.items[0].result, ClaimDisposition::Refused {
            refusal: CompositeEqualityRefusal::MissingAdmittedContext { type_node_id }
        } if type_node_id == &selected.left.source_type)
        );
        let exhausted = generate_items_with_libraries(
            &package,
            None,
            std::slice::from_ref(selected),
            &[],
            CompositeEqualityLimits {
                type_resolution_work_units: 0,
                ..Default::default()
            },
            &libraries,
        )
        .expect("work refusal before capability lookup");
        assert!(matches!(
            &exhausted.claim_map.items[0].result,
            ClaimDisposition::Refused {
                refusal: CompositeEqualityRefusal::TypeResolutionWorkExhausted {
                    limit: 0,
                    consumed: 1
                }
            }
        ));
        let other = fixture::compile(fixture::SOURCE, &qsl_replay::DependencyInput::default());
        assert_ne!(
            other.context().package_id(),
            compiled.context().package_id()
        );
        for requested in [Vec::new(), vec![selected.clone()]] {
            assert!(matches!(
                generate_items_with_libraries(
                    &package,
                    Some(other.context()),
                    &requested,
                    &[],
                    Default::default(),
                    &libraries
                ),
                Err(OracleGenerationError::ContextPackageMismatch { .. })
            ));
        }
    }
}

const UNION_SOURCE: &str = r#"language "ix:native" edition "1-draft";
profile v = "quire.value.complete/v1";
record Payload { flag: Boolean; }
union Shape { Empty, Flag(Boolean), Boxed(Payload) }
union Other { Empty, Flag(Boolean), Boxed(Payload) }
union Wrap { Plain, Nested(Shape), Both(Shape, Other) }
function shape_same using v(a: Shape,b: Shape): Boolean pure { a = b }
function other_same using v(a: Other,b: Other): Boolean pure { a = b }
function wrap_same using v(a: Wrap,b: Wrap): Boolean pure { a = b }
function healthy using v(a: Boolean,b: Boolean): Boolean pure { a = b }
"#;

#[trace("TC-029", "FR-018-AC-44")]
#[test]
fn genuine_union_host_generation_closes_nested_owner_declarations() {
    let compiled = fixture::compile(UNION_SOURCE, &qsl_replay::DependencyInput::default());
    let package = checked(&compiled, &[]);
    let items = ["shape_same", "other_same", "wrap_same", "healthy"]
        .map(|name| direct_source_item(&package, name));
    let generated = generate_items(&package, Some(compiled.context()), &items, &[])
        .expect("genuine union batch");
    for item in &items {
        let claim = generated.claim_map.items.iter().find(|claim| claim.node_id == item.node_id)
            .expect("one disposition per requested equality");
        assert!(matches!(claim.result, ClaimDisposition::Generated(_)),
            "genuine union equality must generate: {:?}", claim.result);
    }
}
