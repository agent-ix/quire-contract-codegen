//! Consume the public replay facade's checker-issued nominal identities.

use super::{CompositeEqualityRefusal, ItemCheckError, OracleGenerationError, TypeClosure};
use qsl_replay::{AdmittedCompositeForm, AdmittedContext, AdmittedLookupRefusal};
use quire_contract_model::{CheckedNodeId, CheckedSemanticNodeV2, CheckedTerm};
use quire_exact::{EnumShape, NodeKey, ValueType};
use quire_semantic_value::enumeration::EnumDeclaration;

/// Authenticated enum shape and its wire lookup recipe, retained for generation.
#[derive(Clone)]
pub(super) struct EnumBinding {
    pub node: CheckedNodeId,
    pub shape: EnumShape,
    pub qualified: Vec<String>,
    pub ordered: bool,
}

fn admitted_key(
    closure: &TypeClosure,
    wire: &CheckedNodeId,
) -> Result<NodeKey, CompositeEqualityRefusal> {
    closure.node_keys.get(wire).cloned().ok_or_else(|| {
        CompositeEqualityRefusal::MalformedAdmittedWire {
            declaration: wire.clone(),
        }
    })
}

pub(super) fn lookup_refusal(cause: AdmittedLookupRefusal) -> CompositeEqualityRefusal {
    match cause {
        AdmittedLookupRefusal::Unknown { declaration, case } => {
            CompositeEqualityRefusal::UnknownEnum {
                declaration: declaration.to_string(),
                case: case.map(|case| case.as_str().to_owned()),
            }
        }
        AdmittedLookupRefusal::WrongKind { declaration } => {
            CompositeEqualityRefusal::WrongAdmittedKind {
                declaration: declaration.to_string(),
            }
        }
        AdmittedLookupRefusal::Foreign {
            declaration,
            package,
        } => CompositeEqualityRefusal::ForeignEnum {
            declaration: declaration.to_string(),
            package_name: package.to_string(),
        },
    }
}

pub(super) fn require_context<'ctx>(
    admitted: Option<&'ctx AdmittedContext>,
    declaration: &CheckedNodeId,
) -> Result<&'ctx AdmittedContext, CompositeEqualityRefusal> {
    admitted.ok_or_else(|| CompositeEqualityRefusal::MissingAdmittedContext {
        type_node_id: declaration.clone(),
    })
}

pub(super) fn resolve_enum<Body>(
    admitted: &AdmittedContext,
    closure: &mut TypeClosure,
    node: &CheckedSemanticNodeV2<Body>,
    term: Option<&CheckedTerm>,
) -> Result<ValueType, ItemCheckError> {
    let mismatch = || CompositeEqualityRefusal::EnumContextMismatch {
        declaration: node.node_id.clone(),
    };
    let key = admitted_key(closure, &node.node_id)?;
    let actual = admitted.enum_declaration(&key).map_err(lookup_refusal)?;
    let Some(CheckedTerm::NominalBody(body)) = term else {
        return Err(mismatch().into());
    };
    let (expected_ordered, expected_members) = body.enum_details().ok_or_else(mismatch)?;
    if actual.is_ordered() != expected_ordered
        || !actual
            .qualified_name()
            .iter()
            .map(|part| part.as_str())
            .eq(node
                .declaration
                .as_ref()
                .ok_or_else(mismatch)?
                .qualified_name
                .iter()
                .map(|part| part.as_ref()))
        || !actual
            .cases()
            .iter()
            .map(|case| case.case().as_str())
            .eq(expected_members.iter().map(|case| case.as_ref()))
    {
        return Err(mismatch().into());
    }
    let declaration = EnumDeclaration::new(
        actual.key().clone(),
        actual.is_ordered(),
        actual
            .cases()
            .iter()
            .map(|case| case.case().as_str().to_owned())
            .collect(),
    )
    .map_err(OracleGenerationError::EnumReconstruction)?;
    let mut variants = Vec::with_capacity(actual.cases().len());
    for case in actual.cases() {
        let selected = admitted
            .enum_case(&key, case.case())
            .map_err(lookup_refusal)?;
        let value = declaration
            .member(selected.case().as_str(), selected.key().clone())
            .map_err(OracleGenerationError::EnumReconstruction)?;
        variants.push(value.variant());
        closure.enum_members.record(value);
    }
    let shape = EnumShape::new(actual.is_ordered(), variants);
    if !closure
        .enums
        .iter()
        .any(|binding| binding.node == node.node_id)
    {
        closure.enums.push(EnumBinding {
            node: node.node_id.clone(),
            shape: shape.clone(),
            ordered: actual.is_ordered(),
            qualified: actual
                .qualified_name()
                .iter()
                .map(|part| part.as_str().to_owned())
                .collect(),
        });
    }
    Ok(ValueType::Enum(shape))
}

/// An authenticated composite lookup recipe, retained for emitted reconstruction.
#[derive(Clone)]
pub(super) struct CompositeBinding {
    pub node: CheckedNodeId,
    pub qualified: Vec<String>,
    pub form: AdmittedCompositeForm,
    pub declaration: qsl_replay::AdmittedComposite,
}

pub(super) fn composite_lookup_refusal(cause: AdmittedLookupRefusal) -> CompositeEqualityRefusal {
    match cause {
        AdmittedLookupRefusal::Unknown {
            declaration,
            case: _,
        } => CompositeEqualityRefusal::UnknownComposite {
            declaration: declaration.to_string(),
        },
        AdmittedLookupRefusal::Foreign {
            declaration,
            package,
        } => CompositeEqualityRefusal::ForeignComposite {
            declaration: declaration.to_string(),
            package_name: package.to_string(),
        },
        AdmittedLookupRefusal::WrongKind { declaration } => {
            CompositeEqualityRefusal::WrongAdmittedKind {
                declaration: declaration.to_string(),
            }
        }
    }
}

pub(super) fn resolve_composite<Body>(
    admitted: &AdmittedContext,
    closure: &mut TypeClosure,
    node: &CheckedSemanticNodeV2<Body>,
) -> Result<NodeKey, CompositeEqualityRefusal> {
    let mismatch = || CompositeEqualityRefusal::CompositeContextMismatch {
        declaration: node.node_id.clone(),
    };
    let key = admitted_key(closure, &node.node_id)?;
    let actual = admitted
        .composite_declaration(&key)
        .map_err(composite_lookup_refusal)?;
    let key = actual.key().clone();
    let expected_form = match node.semantic_form.as_ref() {
        "record" => AdmittedCompositeForm::Record,
        "tuple" => AdmittedCompositeForm::Tuple,
        "union" => AdmittedCompositeForm::Union,
        _ => return Err(mismatch()),
    };
    let expected_name = node.declaration.as_ref().ok_or_else(mismatch)?;
    if actual.node() != &key
        || actual.form() != expected_form
        || !actual
            .qualified_name()
            .iter()
            .map(|part| part.as_str())
            .eq(expected_name
                .qualified_name
                .iter()
                .map(|part| part.as_ref()))
    {
        return Err(mismatch());
    }
    closure
        .composite_bindings
        .entry(actual.key().clone())
        .or_insert_with(|| CompositeBinding {
            node: node.node_id.clone(),
            form: actual.form(),
            declaration: actual.clone(),
            qualified: actual
                .qualified_name()
                .iter()
                .map(|part| part.as_str().to_owned())
                .collect(),
        });
    Ok(actual.key().clone())
}

#[cfg(test)]
#[path = "../../../tests/admitted_composite_support/fixture.rs"]
pub(super) mod composite_fixture;

#[cfg(test)]
mod tests {
    use super::*;
    use ix_trace_rs::trace;

    /// Authentic owner lookups retain wrong-kind, unknown, foreign and malformed refusals.
    #[trace("FR-018-AC-8", "FR-018-AC-39", "FR-018-AC-46", "TC-029")]
    #[test]
    fn actual_composite_lookup_refusals_preserve_kind_identity_and_foreign_owner() {
        for transitive in [false, true] {
            let (compiled, libraries) = super::composite_fixture::imported(transitive);
            let context = compiled.context();
            let marker = context.enums().next().expect("real own enum");
            let wrong = marker.key().clone();
            let cause = context
                .composite_declaration(&wrong)
                .expect_err("own enum is wrong kind");
            assert_eq!(
                composite_lookup_refusal(cause),
                CompositeEqualityRefusal::WrongAdmittedKind {
                    declaration: wrong.to_string(),
                }
            );
            let unknown = NodeKey::new(
                "test/unknown",
                vec![quire_exact::PathSegment::Text("missing".to_owned())],
                0,
            )
            .expect("structural unknown key");
            let cause = context
                .composite_declaration(&unknown)
                .expect_err("not an admitted declaration");
            assert_eq!(
                composite_lookup_refusal(cause),
                CompositeEqualityRefusal::UnknownComposite {
                    declaration: unknown.to_string(),
                }
            );
            let leaf = &libraries
                .iter()
                .find(|(identity, _)| identity == "test/composites")
                .expect("original library")
                .1;
            let owner = leaf.context().package_name().to_string();
            for actual in leaf.context().composites() {
                let key = actual.node().clone();
                let cause = context
                    .composite_declaration(&key)
                    .expect_err("foreign closure declaration");
                assert_eq!(
                    composite_lookup_refusal(cause),
                    CompositeEqualityRefusal::ForeignComposite {
                        declaration: key.to_string(),
                        package_name: owner.clone(),
                    }
                );
            }
            let malformed = CheckedNodeId {
                package: "admitted-composites".into(),
                path: Vec::new(),
                ordinal: 0,
            };
            assert_eq!(
                admitted_key(&TypeClosure::default(), &malformed),
                Err(CompositeEqualityRefusal::MalformedAdmittedWire {
                    declaration: malformed
                })
            );
        }
    }
}

/// The owner-admitted enum behind one declaration node: the shape the owner route admitted, each
/// checked member of it in declared order, and the declaration facts a harness records.
#[derive(Clone, Debug)]
pub(crate) struct OwnerEnum {
    pub(crate) node: CheckedNodeId,
    pub(crate) shape: EnumShape,
    pub(crate) members: Vec<quire_exact::EnumMember>,
    pub(crate) qualified: Vec<String>,
    pub(crate) cases: Vec<String>,
}

/// Why the owner route supplied no enum declaration.
#[derive(Debug)]
pub(crate) enum OwnerEnumRefusal {
    /// No admitted context accompanied the request.
    NoContext,
    /// The context, the IR declaration or the owner lookup did not admit the declaration.
    NotAdmitted(String),
}

/// Resolve `node`, an `enum` declaration of `graph`, through the owner's admitted context.
pub(crate) fn owner_enum(
    admitted: Option<&AdmittedContext>,
    graph: &crate::oracle::checked_graph::CheckedGraph<'_, quire_contract_model::CheckedTerm>,
    node: &CheckedNodeId,
) -> Result<OwnerEnum, OwnerEnumRefusal> {
    let context = admitted.ok_or(OwnerEnumRefusal::NoContext)?;
    let declaration = graph.node(node).ok_or_else(|| {
        OwnerEnumRefusal::NotAdmitted("the type node is not in the package".to_owned())
    })?;
    let mut closure = TypeClosure {
        node_keys: graph.node_keys(),
        ..TypeClosure::default()
    };
    let resolved =
        resolve_enum(context, &mut closure, declaration, graph.term(node)).map_err(|error| {
            OwnerEnumRefusal::NotAdmitted(match error {
                ItemCheckError::Refusal(refusal) => format!("{refusal:?}"),
                ItemCheckError::Generation(error) => format!("{error:?}"),
            })
        })?;
    let ValueType::Enum(shape) = &resolved else {
        return Err(OwnerEnumRefusal::NotAdmitted(
            "the declaration did not resolve to an enum".to_owned(),
        ));
    };
    let shape = shape.clone();
    let key = admitted_key(&closure, node)
        .map_err(|error| OwnerEnumRefusal::NotAdmitted(format!("{error:?}")))?;
    let actual = context
        .enum_declaration(&key)
        .map_err(|error| OwnerEnumRefusal::NotAdmitted(format!("{error:?}")))?;
    let owner_declaration = EnumDeclaration::new(
        actual.key().clone(),
        actual.is_ordered(),
        actual
            .cases()
            .iter()
            .map(|case| case.case().as_str().to_owned())
            .collect(),
    )
    .map_err(|error| OwnerEnumRefusal::NotAdmitted(format!("{error:?}")))?;
    let mut members = Vec::with_capacity(actual.cases().len());
    for (position, case) in actual.cases().iter().enumerate() {
        let selected = context
            .enum_case(&key, case.case())
            .map_err(|error| OwnerEnumRefusal::NotAdmitted(format!("{error:?}")))?;
        let value = owner_declaration
            .member(selected.case().as_str(), selected.key().clone())
            .map_err(|error| OwnerEnumRefusal::NotAdmitted(format!("{error:?}")))?;
        let rank = u32::try_from(position)
            .map_err(|_| OwnerEnumRefusal::NotAdmitted("member rank exceeds u32".to_owned()))?;
        members.push(quire_exact::EnumMember::new(value.variant(), rank));
    }
    Ok(OwnerEnum {
        node: node.clone(),
        shape,
        members,
        qualified: actual
            .qualified_name()
            .iter()
            .map(|part| part.as_str().to_owned())
            .collect(),
        cases: actual
            .cases()
            .iter()
            .map(|case| case.case().as_str().to_owned())
            .collect(),
    })
}
