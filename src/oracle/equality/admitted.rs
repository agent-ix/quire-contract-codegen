//! Consume the public replay facade's checker-issued enum identities.

use super::{CompositeEqualityRefusal, ItemCheckError, OracleGenerationError, TypeClosure};
use qsl_replay::{AdmittedContext, AdmittedLookupRefusal, WireNodeId};
use quire_contract_model::{CheckedNodeId, CheckedSemanticNodeV2, NominalIdentityPreimage};
use quire_exact::{EnumShape, ValueType};
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
