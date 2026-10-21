//! Consume the public replay facade's checker-issued nominal identities.

use super::{CompositeEqualityRefusal, ItemCheckError, OracleGenerationError, TypeClosure};
use qsl_replay::{AdmittedCompositeForm, AdmittedContext, AdmittedLookupRefusal, WireNodeId};
use quire_contract_model::{CheckedNodeId, CheckedSemanticNodeV2, NominalIdentityPreimage};
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
            CompositeEqualityRefusal::WrongAdmittedKind { declaration: declaration.to_string() }
        }
        AdmittedLookupRefusal::Foreign {
            declaration,
            package,
        } => {
            let package = package.record();
            CompositeEqualityRefusal::ForeignEnum {
                declaration: declaration.to_string(),
                package_domain: package.domain().as_str().to_owned(),
                package_digest: package.hex(),
            }
        }
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

pub(super) fn resolve_enum(
    admitted: &AdmittedContext,
    closure: &mut TypeClosure,
    node: &CheckedSemanticNodeV2,
) -> Result<ValueType, ItemCheckError> {
    let mismatch = || CompositeEqualityRefusal::EnumContextMismatch {
        declaration: node.node_id.clone(),
    };
    let wire = WireNodeId::from_hex(&node.node_id.digest).ok_or_else(mismatch)?;
    let actual = admitted.enum_declaration(wire).map_err(lookup_refusal)?;
    let Some(NominalIdentityPreimage::EnumDeclaration(expected)) = &node.nominal_identity_preimage
    else {
        return Err(mismatch().into());
    };
    if actual.is_ordered() != expected.ordered
        || !actual
            .qualified_name()
            .iter()
            .map(|part| part.as_str())
            .eq(expected
                .qualified_declaration
                .iter()
                .map(|part| part.as_ref()))
        || !actual
            .cases()
            .iter()
            .map(|case| case.case().as_str())
            .eq(expected.members.iter().map(|case| case.as_ref()))
    {
        return Err(mismatch().into());
    }
    let declaration = EnumDeclaration::new(
        actual.key(),
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
            .enum_case(wire, case.case())
            .map_err(lookup_refusal)?;
        let value = declaration
            .member(selected.case().as_str(), selected.key())
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

/// An authenticated record/tuple lookup recipe, retained for emitted reconstruction.
#[derive(Clone)]
pub(super) struct CompositeBinding {
    pub node: CheckedNodeId,
    pub qualified: Vec<String>,
    pub form: AdmittedCompositeForm,
}

pub(super) fn composite_lookup_refusal(cause: AdmittedLookupRefusal) -> CompositeEqualityRefusal {
    match cause {
        AdmittedLookupRefusal::Unknown { declaration, case: _ } => {
            CompositeEqualityRefusal::UnknownComposite { declaration: declaration.to_string() }
        }
        AdmittedLookupRefusal::Foreign { declaration, package } => {
            let package = package.record();
            CompositeEqualityRefusal::ForeignComposite {
                declaration: declaration.to_string(),
                package_domain: package.domain().as_str().to_owned(),
                package_digest: package.hex(),
            }
        }
        AdmittedLookupRefusal::WrongKind { declaration } => {
            CompositeEqualityRefusal::WrongAdmittedKind { declaration: declaration.to_string() }
        }
    }
}

pub(super) fn resolve_composite(
    admitted: &AdmittedContext,
    closure: &mut TypeClosure,
    node: &CheckedSemanticNodeV2,
) -> Result<NodeKey, CompositeEqualityRefusal> {
    let mismatch = || CompositeEqualityRefusal::CompositeContextMismatch {
        declaration: node.node_id.clone(),
    };
    let wire = WireNodeId::from_hex(&node.node_id.digest).ok_or_else(mismatch)?;
    let actual = admitted.composite_declaration(wire).map_err(composite_lookup_refusal)?;
    let expected_form = match node.semantic_form.as_ref() {
        "record" => AdmittedCompositeForm::Record,
        "tuple" => AdmittedCompositeForm::Tuple,
        _ => return Err(mismatch()),
    };
    let expected_name = node.declaration.as_ref().ok_or_else(mismatch)?;
    if actual.node().to_string() != node.node_id.digest.as_ref() || actual.form() != expected_form || !actual.qualified_name().iter()
        .map(|part| part.as_str())
        .eq(expected_name.qualified_name.iter().map(|part| part.as_ref()))
    {
        return Err(mismatch());
    }
    closure.composite_bindings.entry(actual.key()).or_insert_with(|| CompositeBinding {
        node: node.node_id.clone(), form: actual.form(),
        qualified: actual.qualified_name().iter().map(|part| part.as_str().to_owned()).collect(),
    });
    Ok(actual.key())
}

#[cfg(test)]
#[path = "../../../tests/admitted_composite_support/fixture.rs"]
mod composite_fixture;

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
            let wrong = WireNodeId::from_digest(*marker.key().as_bytes());
            let cause = context.composite_declaration(wrong).expect_err("own enum is wrong kind");
            assert_eq!(composite_lookup_refusal(cause), CompositeEqualityRefusal::WrongAdmittedKind {
                declaration: wrong.to_string(),
            });
            let unknown = WireNodeId::from_digest([7; 32]);
            let cause = context.composite_declaration(unknown).expect_err("not an admitted declaration");
            assert_eq!(composite_lookup_refusal(cause), CompositeEqualityRefusal::UnknownComposite {
                declaration: unknown.to_string(),
            });
            let leaf = &libraries.iter().find(|(identity,_)| identity == "test/composites").expect("original library").1;
            let owner = leaf.context().package_id().record();
            for actual in leaf.context().composites() {
                let wire = WireNodeId::from_digest(*actual.node().as_bytes());
                let cause = context.composite_declaration(wire).expect_err("foreign closure declaration");
                assert_eq!(composite_lookup_refusal(cause), CompositeEqualityRefusal::ForeignComposite {
                    declaration: wire.to_string(), package_domain: owner.domain().as_str().to_owned(),
                    package_digest: owner.hex(),
                });
            }
        }
    }
}
