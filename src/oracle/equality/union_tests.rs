//! Host controls for the actual generator's admitted Union closure and seals.

use super::*;
use admitted::composite_fixture as fixture;
use ix_trace_rs::trace;
use quire_contract_model::{
    CheckedPackageEvidence, CheckedPackageReadLimits, CheckedPackageV2, CheckedPackageV2ReadResult,
};

fn compilation() -> (CheckedPackageV2, qsl_replay::AdmittedContext) {
    let compiled = fixture::compile(fixture::UNION_SOURCE, &Default::default());
    let mut evidence = CheckedPackageEvidence::new();
    evidence.support_feature("quire.value.complete/v1");
    let read = CheckedPackageV2::read(
        compiled.package().bytes(),
        CheckedPackageReadLimits::bounded(),
        &evidence,
    );
    let CheckedPackageV2ReadResult::Admitted(package) = read else {
        panic!("authentic source must pass the strict reader: {read:?}");
    };
    (*package, compiled.into_parts().1)
}

fn declaration<'a>(
    context: &'a qsl_replay::AdmittedContext,
    name: &str,
) -> &'a qsl_replay::AdmittedComposite {
    context
        .composites()
        .find(|item| {
            item.qualified_name()
                .iter()
                .map(|part| part.as_str())
                .eq([name])
        })
        .expect("original declaration")
}

fn closure(package: &CheckedPackageV2, context: &qsl_replay::AdmittedContext) -> TypeClosure {
    let graph = package
        .graph()
        .nodes
        .iter()
        .map(|node| (&node.node_id, node))
        .collect();
    let root = declaration(context, "Wrap");
    let id = package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_id.digest.as_ref() == root.node().to_string())
        .expect("original Union node")
        .node_id
        .clone();
    let mut closure = TypeClosure::default();
    assert!(
        matches!(resolve_type(Some(context), &graph, &BTreeMap::new(), &mut closure, &id),
        Ok(ValueType::Composite(key)) if key == root.key())
    );
    closure
}

fn environment(closure: &TypeClosure) -> TypeEnvironment {
    match admit_environment(
        None,
        closure.composites.values().cloned().collect(),
        &closure.composite_bindings,
    ) {
        Ok(environment) => environment,
        _ => panic!("authentic closure must admit and seal"),
    }
}

fn meter() -> exact::Meter {
    exact::Meter::new(exact::ScalarLimits {
        integer_bits: u64::MAX,
        decimal_digits: u64::MAX,
        scale_expansion: u64::MAX,
        text_input_bytes: u64::MAX,
        text_scalars: u64::MAX,
        normalized_scalars: u64::MAX,
        unit_edges: u64::MAX,
        value_occurrences: u64::MAX,
        work_units: u64::MAX,
        result_units: u64::MAX,
    })
}

#[trace("TC-029", "FR-018-AC-44")]
#[test]
fn union_closure_seals_owner_members_and_admits_each_nested_payload() {
    let (package, context) = compilation();
    let closure = closure(&package, &context);
    assert_eq!(
        closure.composites.len(),
        4,
        "Wrap, Shape, Other and Payload"
    );
    let environment = environment(&closure);
    let payload = declaration(&context, "Payload");
    let record = environment
        .record(
            payload.key(),
            vec![(
                "flag",
                exact::FieldValue::Present(exact::Value::Boolean(true)),
            )],
        )
        .expect("record payload");
    let mut nested = Vec::new();
    for name in ["Shape", "Other"] {
        let actual = declaration(&context, name);
        assert_ne!(
            actual.key(),
            actual.node(),
            "type handle and settled node differ"
        );
        assert_eq!(environment.union_key(actual.key()), Some(actual.node()));
        let members = actual.union_members().expect("owner Union");
        assert_eq!(
            members
                .iter()
                .map(|m| m.identifier().as_str())
                .collect::<Vec<_>>(),
            ["Empty", "Flag", "Boxed"]
        );
        assert_eq!(members[2].payload(), [ValueType::Composite(payload.key())]);
        let CompositeShape::Union(rebuilt) = environment
            .composite(actual.key())
            .expect("closure")
            .shape()
        else {
            panic!("Union shape retained");
        };
        for (member, rebuilt) in members.iter().zip(rebuilt) {
            assert_eq!(rebuilt.identifier(), member.identifier());
            assert_eq!(rebuilt.positions(), member.payload());
            assert_eq!(rebuilt.member(), Some(member.member()));
        }
        for (member, values) in members.iter().zip([
            vec![],
            vec![exact::Value::Boolean(true)],
            vec![record.clone()],
        ]) {
            let value = environment
                .union(actual.key(), member.variant(), values.clone())
                .expect("every member constructs");
            let independent = exact::UnionValue::from_admitted(member.member().clone(), values);
            assert!(environment
                .admits(&ValueType::Composite(actual.key()), &independent)
                .expect("owner binding admitted"));
            let operand = EqualityOperand::typed(ValueType::Composite(actual.key()));
            let checked = environment
                .check_equality(
                    sv::EqualityOperator::Equal,
                    operand.clone(),
                    operand,
                    &closure.enum_members,
                )
                .expect("Union equality");
            let mut meter = meter();
            assert_eq!(
                checked.evaluate(&environment, &value, &independent, &mut meter),
                exact::Outcome::Completed(true)
            );
            assert!(
                meter.consumed(exact::LimitKind::WorkUnits) > 0,
                "real equality evaluation"
            );
            nested.push(value);
        }
    }
    let shape = declaration(&context, "Shape");
    let other = declaration(&context, "Other");
    assert_ne!(shape.key(), other.key());
    assert_ne!(shape.node(), other.node());
    assert!(!environment
        .admits(&ValueType::Composite(other.key()), &nested[0])
        .expect("membership runs"));
    let wrap = declaration(&context, "Wrap");
    let members = wrap.union_members().expect("Wrap members");
    assert_eq!(
        members[2].payload(),
        [
            ValueType::Composite(shape.key()),
            ValueType::Composite(other.key())
        ]
    );
    for (member, values) in members.iter().zip([
        vec![],
        vec![nested[1].clone()],
        vec![nested[2].clone(), nested[5].clone()],
    ]) {
        let value = environment
            .union(wrap.key(), member.variant(), values)
            .expect("nested Union payload");
        assert!(environment
            .admits(&ValueType::Composite(wrap.key()), &value)
            .expect("nested membership"));
    }
    let unsealed = TypeEnvironment::new(closure.composites.values().cloned(), [])
        .expect("unsealed declarations");
    assert!(
        unsealed
            .union(
                shape.key(),
                shape.union_members().unwrap()[0].variant(),
                vec![]
            )
            .is_err(),
        "sealing is necessary"
    );
}

#[trace("TC-029", "FR-018-AC-44")]
#[test]
fn union_lookup_and_seal_faults_keep_typed_causes_before_equality() {
    let (package, context) = compilation();
    let healthy = closure(&package, &context);
    let shape = declaration(&context, "Shape");
    let original = package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_id.digest.as_ref() == shape.node().to_string())
        .unwrap();
    let unchanged = environment(&healthy).union_key(shape.key());
    for form in ["record", "tuple"] {
        let mut node = original.clone();
        node.semantic_form = form.into();
        assert_eq!(
            admitted::resolve_composite(&context, &mut TypeClosure::default(), &node),
            Err(CompositeEqualityRefusal::CompositeContextMismatch {
                declaration: node.node_id.clone()
            })
        );
    }
    let mut renamed = original.clone();
    renamed.declaration.as_mut().unwrap().qualified_name = vec!["Wrong".into()];
    assert!(matches!(
        admitted::resolve_composite(&context, &mut TypeClosure::default(), &renamed),
        Err(CompositeEqualityRefusal::CompositeContextMismatch { .. })
    ));
    let mut unknown = original.clone();
    unknown.node_id.digest = "07".repeat(32).into();
    assert_eq!(
        admitted::resolve_composite(&context, &mut TypeClosure::default(), &unknown),
        Err(CompositeEqualityRefusal::UnknownComposite {
            declaration: "07".repeat(32)
        })
    );
    let mut wrong_kind = original.clone();
    let marker = context.enums().next().expect("genuine enum");
    wrong_kind.node_id.digest = marker.key().to_string().into();
    assert_eq!(
        admitted::resolve_composite(&context, &mut TypeClosure::default(), &wrong_kind),
        Err(CompositeEqualityRefusal::WrongAdmittedKind {
            declaration: marker.key().to_string()
        })
    );
    let library = qsl_replay::SuppliedLibrary {
        identity: "test/union".to_owned(),
        source: qsl_replay::SourceIdentity::new(
            "test",
            "admitted-composites.native",
            "git",
            "fixture",
        ),
        path: "admitted-composites.native".to_owned(),
        bytes: fixture::UNION_SOURCE.as_bytes().to_vec(),
    };
    let dependencies = qsl_replay::DependencyInput::new([library]).unwrap();
    let importer = qsl_replay::compile_admitted(
        qsl_replay::SourceIdentity::new("test", "union-importer.native", "git", "fixture"),
        "union-importer.native",
        br#"language "ix:native" edition "1-draft"; profile v = "quire.value.complete/v1";
        import "test/union" as u;
        function healthy using v(a: Boolean,b: Boolean): Boolean pure { a = b }"#,
        [],
        &dependencies,
        Default::default(),
        Default::default(),
    )
    .expect("distinct authentic importer source");
    let owner = context.package_id().record();
    assert_eq!(
        admitted::resolve_composite(importer.context(), &mut TypeClosure::default(), original),
        Err(CompositeEqualityRefusal::ForeignComposite {
            declaration: shape.node().to_string(),
            package_domain: owner.domain().as_str().to_owned(),
            package_digest: owner.hex(),
        })
    );
    // Swap a genuine owner declaration under a different handle: no forged member/key.
    let mut bindings = healthy.composite_bindings.clone();
    bindings.get_mut(&shape.key()).unwrap().declaration = declaration(&context, "Wrap").clone();
    bindings.remove(&declaration(&context, "Wrap").key());
    let failure = admit_environment(
        None,
        healthy.composites.values().cloned().collect(),
        &bindings,
    );
    assert!(matches!(
        failure,
        Err(ItemCheckError::Refusal(
            CompositeEqualityRefusal::Declaration {
                cause: DeclarationRefusalCause::Type {
                    cause: IllTypedCauseKind::TypeMismatch
                },
                ..
            }
        ))
    ));
    assert_eq!(
        environment(&healthy).union_key(shape.key()),
        unchanged,
        "failed item leaves healthy closure unchanged"
    );
}
