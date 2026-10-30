//! Consume the public replay facade's checker-issued nominal identities.

use super::{CompositeEqualityRefusal, ItemCheckError, OracleGenerationError, TypeClosure};
use qsl_replay::{AdmittedCompositeForm, AdmittedContext, AdmittedLookupRefusal};
use quire_contract_model::{CheckedNodeId, CheckedSemanticNodeV2};
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
) -> Result<ValueType, ItemCheckError> {
    let mismatch = || CompositeEqualityRefusal::EnumContextMismatch {
        declaration: node.node_id.clone(),
    };
    let key = crate::core::ir::node_key::checked_node_key(&node.node_id).ok_or_else(mismatch)?;
    let actual = admitted.enum_declaration(&key).map_err(lookup_refusal)?;
    let expected = node.body.enum_declaration().ok_or_else(mismatch)?;
    if actual.is_ordered() != expected.ordered()
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
            .eq(expected.members().iter().map(|case| case.as_ref()))
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
    let key = crate::core::ir::node_key::checked_node_key(&node.node_id).ok_or_else(mismatch)?;
    let actual = admitted
        .composite_declaration(&key)
        .map_err(composite_lookup_refusal)?;
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

    /// These are private owner-refusal mapping checks, not a forged public package admission.
    #[trace("FR-018-AC-8", "FR-018-AC-39", "TC-029")]
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
                let wire = actual.node().clone();
                let cause = context
                    .composite_declaration(&wire)
                    .expect_err("foreign closure declaration");
                assert_eq!(
                    composite_lookup_refusal(cause),
                    CompositeEqualityRefusal::ForeignComposite {
                        declaration: wire.to_string(),
                        package_name: owner.clone(),
                    }
                );
            }
        }
    }
}
