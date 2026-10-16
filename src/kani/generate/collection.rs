//! Source-bound collection domains derived from admitted V2 model-field projections.
//!
//! This module deliberately does not infer a field from its display name or select a model
//! declaration by type. The projection's admitted declaration identity and member name select
//! the retained field table; unsupported shapes remain typed refusals for the negotiator.

use quire_contract_model::{
    CheckedCollectionKind, CheckedMemberType, CheckedNodeId, CheckedNodeTag, CheckedPackageV2,
    CheckedSemanticNodeV2,
};
use serde_json::Value;

use crate::{
    core::artifact::{Artifact, MAX_GENERATED_SOURCE_BYTES},
    kani::{
        abi::{adapter_options, readable_component, KaniSolver},
        generate::{
            outcome::{KaniObligationRequest, UnsupportedObligation},
            record::{harness_path, record},
        },
        identity::{
            CollectionElementIdentity, CollectionKindIdentity, CollectionObligationIdentity,
            CollectionPredicateIdentity, KaniCollectionObligationHarness,
        },
    },
};

/// A primitive element domain the current symbolic collection renderer can represent exactly.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CollectionElementDomain {
    /// Both Boolean values.
    Boolean,
    /// An inclusive bounded integer interval.
    Integer { minimum: i128, maximum: i128 },
}

/// The admitted selected model field and its inclusive sequence bounds.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CollectionFieldDomain {
    /// Selected function whose body contains the quantifier.
    pub(crate) function: CheckedNodeId,
    /// Quantifier node selected by the request.
    pub(crate) quantifier: CheckedNodeId,
    /// Model object declaration containing the selected field.
    pub(crate) declaration: CheckedNodeId,
    /// Exact admitted field name.
    pub(crate) field_name: String,
    /// Collection query/projection node in the checked graph.
    pub(crate) projection: CheckedNodeId,
    /// Inclusive collection cardinality bounds.
    pub(crate) minimum_length: u64,
    /// Inclusive collection cardinality bounds.
    pub(crate) maximum_length: u64,
    /// Domain of every element.
    pub(crate) element: CollectionElementDomain,
    /// Exact element type node reached through the admitted sequence type.
    pub(crate) element_type: CheckedNodeId,
    /// Exact QSL quantifier operation.
    pub(crate) quantifier_operation: String,
    /// Exact predicate node.
    pub(crate) predicate: CollectionPredicateIdentity,
    /// Predicate node identity.
    pub(crate) predicate_id: CheckedNodeId,
}

/// Why a checked collection projection cannot be assigned a symbolic slot layout.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum CollectionDomainRefusal {
    /// The graph node, operation, operand, or checked type did not have the required shape.
    UnsupportedShape,
    /// The admitted selected model field has no finite collection maximum.
    RequiresBound { unbounded_type: CheckedNodeId },
    /// The exact declaration/member pair does not resolve in the retained model field table.
    FieldNotAdmitted,
    /// The retained field form is not an ordered sequence with supported primitive elements.
    UnsupportedFieldType,
    /// The checked range has no inhabitants.
    UnsatisfiableElementDomain,
}

/// A completely checked collection claim ready for deterministic rendering.
pub(super) struct LoweredCollectionClaim {
    pub(super) package_digest: String,
    pub(super) field: CollectionFieldDomain,
}

/// Resolve one selected claim before rendering; no caller-provided field or domain is accepted.
pub(super) fn lower_collection_claim(
    package: &CheckedPackageV2,
    function_id: &CheckedNodeId,
    quantifier_id: &CheckedNodeId,
) -> Result<LoweredCollectionClaim, CollectionDomainRefusal> {
    let quantifier = package
        .graph()
        .nodes
        .iter()
        .find(|node| &node.node_id == quantifier_id)
        .ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    let field = resolve_collection_field(package, function_id, quantifier)?;
    Ok(LoweredCollectionClaim {
        package_digest: package.package_id().digest.to_string(),
        field,
    })
}

/// Render one supported admitted collection claim as a symbolic Kani harness.
pub(super) fn render_collection(
    request: &KaniObligationRequest<'_>,
    lowered: &LoweredCollectionClaim,
) -> Result<KaniCollectionObligationHarness, UnsupportedObligation> {
    let field = &lowered.field;
    let minimum_length = i64::try_from(field.minimum_length).map_err(|_| {
        UnsupportedObligation::CollectionClaimNotRendered {
            node_id: field.quantifier.clone(),
            reason: crate::kani::generate::outcome::CollectionUnsupportedReason::DomainNotRepresentableInI64,
        }
    })?;
    let maximum_length = i64::try_from(field.maximum_length).map_err(|_| {
        UnsupportedObligation::CollectionClaimNotRendered {
            node_id: field.quantifier.clone(),
            reason: crate::kani::generate::outcome::CollectionUnsupportedReason::DomainNotRepresentableInI64,
        }
    })?;
    let array_len = usize::try_from(field.maximum_length).map_err(|_| {
        UnsupportedObligation::CollectionClaimNotRendered {
            node_id: field.quantifier.clone(),
            reason: crate::kani::generate::outcome::CollectionUnsupportedReason::DomainNotRepresentableInI64,
        }
    })?;
    if array_len > isize::MAX as usize {
        return Err(UnsupportedObligation::CollectionClaimNotRendered {
            node_id: field.quantifier.clone(),
            reason: crate::kani::generate::outcome::CollectionUnsupportedReason::DomainNotRepresentableInHarness,
        });
    }
    let (element_type, element_domain, element_assumption) = match field.element {
        CollectionElementDomain::Boolean => {
            ("bool", CollectionElementIdentity::Boolean, String::new())
        }
        CollectionElementDomain::Integer { minimum, maximum } => {
            let minimum = i64::try_from(minimum).map_err(|_| {
                UnsupportedObligation::CollectionClaimNotRendered {
                    node_id: field.quantifier.clone(),
                    reason: crate::kani::generate::outcome::CollectionUnsupportedReason::DomainNotRepresentableInI64,
                }
            })?;
            let maximum = i64::try_from(maximum).map_err(|_| {
                UnsupportedObligation::CollectionClaimNotRendered {
                    node_id: field.quantifier.clone(),
                    reason: crate::kani::generate::outcome::CollectionUnsupportedReason::DomainNotRepresentableInI64,
                }
            })?;
            (
                "i64",
                CollectionElementIdentity::Integer { minimum, maximum },
                format!("                kani::assume(item >= {minimum} && item <= {maximum});\n"),
            )
        }
    };
    let element_storage_bytes = match field.element {
        CollectionElementDomain::Boolean => 1u64,
        CollectionElementDomain::Integer { .. } => 8u64,
    };
    let minimum_required_bytes = field
        .maximum_length
        .checked_mul(element_storage_bytes)
        .and_then(|bytes| bytes.checked_add(8))
        .ok_or_else(|| UnsupportedObligation::CollectionClaimNotRendered {
            node_id: field.quantifier.clone(),
            reason: crate::kani::generate::outcome::CollectionUnsupportedReason::DomainNotRepresentableInHarness,
        })?;
    if minimum_required_bytes > request.ceilings.memory_bytes.get() {
        return Err(UnsupportedObligation::CollectionClaimNotRendered {
            node_id: field.quantifier.clone(),
            reason: crate::kani::generate::outcome::CollectionUnsupportedReason::SymbolicDomainExceedsMemoryCeiling {
                minimum_required_bytes,
                ceiling_bytes: request.ceilings.memory_bytes.get(),
            },
        });
    }
    let (quantifier, collection_call) = match field.quantifier_operation.as_str() {
        "quire.op.collection.forall" => ("forall", "all"),
        "quire.op.collection.exists" => ("exists", "any"),
        _ => {
            return Err(UnsupportedObligation::CollectionClaimNotRendered {
                node_id: field.quantifier.clone(),
                reason: crate::kani::generate::outcome::CollectionUnsupportedReason::UnsupportedClaimShape,
            });
        }
    };
    let predicate = match field.predicate {
        CollectionPredicateIdentity::IntegerGreaterOrEqual { threshold } => {
            format!("item >= {threshold}")
        }
        CollectionPredicateIdentity::IntegerEqual { value } => format!("item == {value}"),
        CollectionPredicateIdentity::BooleanNotEqual { value } => format!("item != {value}"),
    };
    let digest = field.quantifier.digest.as_ref();
    let module = format!("kob_collection_{digest}");
    let harness = format!("prove_collection_{digest}");
    let path = harness_path(&module, &harness)?;
    let options = adapter_options(
        &path.to_string(),
        request.unwind,
        KaniSolver::Cadical,
        false,
    );
    let item_name = format!("selected_{}", readable_component(&field.field_name));
    let length_name = format!("{item_name}_len");
    let identity = CollectionObligationIdentity {
        ceilings: request.ceilings,
        package_digest: lowered.package_digest.clone(),
        subject_path: request.subject_path.to_owned(),
        function_id: field.function.clone(),
        quantifier_id: field.quantifier.clone(),
        projection_id: field.projection.clone(),
        declaration_id: field.declaration.clone(),
        field_name: field.field_name.clone(),
        collection_kind: CollectionKindIdentity::Sequence,
        minimum_length,
        maximum_length,
        element: element_domain,
        element_type_id: field.element_type.clone(),
        quantifier_operation: field.quantifier_operation.clone(),
        predicate_id: field.predicate_id.clone(),
        predicate: field.predicate.clone(),
        solver: KaniSolver::Cadical,
        unwind: request.unwind,
        options,
        module_symbol: path.module.clone(),
        harness_symbol: path.harness.clone(),
    };
    let source = format!(
        "// SPDX-License-Identifier: MIT OR Apache-2.0\n\
         // Claim: {quantifier} over admitted model field `{}`\n\
         #[cfg(kani)]\n\
         mod {module} {{\n\
             #[kani::proof]\n\
             fn {harness}() {{\n\
                 let {length_name}: i64 = kani::any();\n\
                 kani::assume({length_name} >= {minimum_length} && {length_name} <= {maximum_length});\n\
                 let {item_name}: [{element_type}; {array_len}] = kani::any();\n\
                 for index in 0..{length_name} {{\n\
                     let item = {item_name}[index as usize];\n\
{element_assumption}\
                 }}\n\
                 let claim = (0..{length_name}).{collection_call}(|index| {{\n\
                     let item = {item_name}[index as usize];\n\
                     {predicate}\n\
                 }});\n\
                 assert!(claim, \"the symbolic sequence satisfies the selected QSL {quantifier} body\");\n\
                 kani::cover!({length_name} == {minimum_length} || {length_name} == {maximum_length}, \"a selected field cardinality endpoint is reachable\");\n\
             }}\n\
         }}\n",
        field.field_name,
    );
    if source.len() > MAX_GENERATED_SOURCE_BYTES {
        return Err(UnsupportedObligation::ResourceLimitExceeded {
            bytes: source.len(),
        });
    }
    if let Err(error) = syn::parse_file(&source) {
        return Err(UnsupportedObligation::InvalidGeneratedSyntax {
            error: error.to_string(),
        });
    }
    let rust = Artifact::new(format!("src/generated/{}.rs", path.module), source);
    let record = record(&path.module.to_string(), &identity, &rust)?;
    let harness = KaniCollectionObligationHarness::generated(identity, rust, record);
    if !harness.has_valid_provenance() {
        return Err(UnsupportedObligation::RenderFailed);
    }
    Ok(harness)
}

/// Resolve the first (collection) operand of an admitted quantifier to its exact model field.
///
/// Resolution uses the graph's projection declaration identity plus `model_object_fields`; it
/// never picks a declaration by matching a same-typed field elsewhere in the package.
pub(crate) fn resolve_collection_field(
    package: &CheckedPackageV2,
    function_id: &CheckedNodeId,
    quantifier: &CheckedSemanticNodeV2,
) -> Result<CollectionFieldDomain, CollectionDomainRefusal> {
    let function = package
        .graph()
        .nodes
        .iter()
        .find(|node| &node.node_id == function_id)
        .ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    if CheckedNodeTag::from_wire(&function.node_tag) != Some(CheckedNodeTag::Function) {
        return Err(CollectionDomainRefusal::UnsupportedShape);
    }
    let function_body = function
        .body
        .get("members")
        .and_then(Value::as_array)
        .and_then(|members| {
            members
                .iter()
                .find(|member| member.get("name").and_then(Value::as_str) == Some("body"))
        })
        .and_then(|member| member.get("value"))
        .and_then(term_target)
        .ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    if !depends_on(package, &function_body, &quantifier.node_id) {
        return Err(CollectionDomainRefusal::UnsupportedShape);
    }
    if CheckedNodeTag::from_wire(&quantifier.node_tag) != Some(CheckedNodeTag::Expression)
        || quantifier.semantic_form.as_ref() != "quantify"
    {
        return Err(CollectionDomainRefusal::UnsupportedShape);
    }
    let arguments = quantifier
        .body
        .get("arguments")
        .and_then(Value::as_array)
        .ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    let quantifier_operation = operation_identity(quantifier)
        .filter(|identity| {
            matches!(
                *identity,
                "quire.op.collection.forall" | "quire.op.collection.exists"
            )
        })
        .ok_or(CollectionDomainRefusal::UnsupportedShape)?
        .to_owned();
    let collection_id = term_target(
        arguments
            .first()
            .ok_or(CollectionDomainRefusal::UnsupportedShape)?,
    )
    .ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    let collection = package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_id == collection_id)
        .ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    if CheckedNodeTag::from_wire(&collection.node_tag) != Some(CheckedNodeTag::Expression)
        || collection.semantic_form.as_ref() != "query"
        || operation_identity(collection) != Some("quire.op.record.project")
    {
        return Err(CollectionDomainRefusal::UnsupportedShape);
    }
    let field_name = collection
        .body
        .get("operation")
        .and_then(|operation| operation.get("member"))
        .filter(|member| member.get("kind").and_then(Value::as_str) == Some("field"))
        .and_then(|member| member.get("name"))
        .and_then(Value::as_str)
        .ok_or(CollectionDomainRefusal::UnsupportedShape)?
        .to_owned();
    let declaration_digest = collection
        .body
        .get("operation")
        .and_then(|operation| operation.get("member"))
        .and_then(|member| member.get("declaration"))
        .and_then(|declaration| declaration.get("digest"))
        .and_then(Value::as_str)
        .ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    let declaration = package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_id.digest.as_ref() == declaration_digest)
        .map(|node| node.node_id.clone())
        .ok_or(CollectionDomainRefusal::FieldNotAdmitted)?;
    verify_function_receiver(package, function, collection, &declaration)?;
    let fields = package
        .model_object_fields(&declaration)
        .map_err(|_| CollectionDomainRefusal::FieldNotAdmitted)?;
    let field = fields
        .field(&field_name)
        .ok_or(CollectionDomainRefusal::FieldNotAdmitted)?;
    if field.member_type().is_none() {
        let element_type = sequence_element_type(package, collection)?;
        if supported_element_type(package, &element_type) {
            return Err(CollectionDomainRefusal::RequiresBound {
                unbounded_type: collection.semantic_type.clone(),
            });
        }
        return Err(CollectionDomainRefusal::UnsupportedFieldType);
    }
    let Some(CheckedMemberType::Collection {
        kind: CheckedCollectionKind::Sequence,
        element,
        bounds,
    }) = field.member_type()
    else {
        return Err(CollectionDomainRefusal::UnsupportedFieldType);
    };
    let Some((minimum_length, maximum_length)) = bounds else {
        return Err(CollectionDomainRefusal::RequiresBound {
            unbounded_type: collection.semantic_type.clone(),
        });
    };
    if minimum_length > maximum_length {
        return Err(CollectionDomainRefusal::UnsupportedFieldType);
    }
    let element_type = sequence_element_type(package, collection)?;
    let predicate_binding = arguments
        .get(1)
        .filter(|argument| argument.get("term").and_then(Value::as_str) == Some("binding"))
        .ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    let binder_name = predicate_binding
        .get("name")
        .and_then(Value::as_str)
        .ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    let predicate_id = term_target(
        predicate_binding
            .get("value")
            .ok_or(CollectionDomainRefusal::UnsupportedShape)?,
    )
    .ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    let predicate_node = package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_id == predicate_id)
        .ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    let element = match element.as_ref() {
        CheckedMemberType::Boolean => CollectionElementDomain::Boolean,
        CheckedMemberType::IntRange { lower, upper } if lower <= upper => {
            CollectionElementDomain::Integer {
                minimum: *lower,
                maximum: *upper,
            }
        }
        CheckedMemberType::IntRange { .. } => {
            return Err(CollectionDomainRefusal::UnsatisfiableElementDomain);
        }
        CheckedMemberType::Integer => {
            return Err(CollectionDomainRefusal::RequiresBound {
                unbounded_type: element_type,
            });
        }
        _ => return Err(CollectionDomainRefusal::UnsupportedFieldType),
    };
    let predicate = resolve_predicate(
        package,
        function,
        binder_name,
        predicate_node,
        element,
        &element_type,
    )?;
    Ok(CollectionFieldDomain {
        function: function_id.clone(),
        quantifier: quantifier.node_id.clone(),
        declaration,
        field_name,
        projection: collection.node_id.clone(),
        minimum_length: *minimum_length,
        maximum_length: *maximum_length,
        element,
        element_type,
        quantifier_operation,
        predicate,
        predicate_id,
    })
}

fn resolve_predicate(
    package: &CheckedPackageV2,
    function: &CheckedSemanticNodeV2,
    binder_name: &str,
    predicate: &CheckedSemanticNodeV2,
    element: CollectionElementDomain,
    element_type: &CheckedNodeId,
) -> Result<CollectionPredicateIdentity, CollectionDomainRefusal> {
    if CheckedNodeTag::from_wire(&predicate.node_tag) != Some(CheckedNodeTag::Expression)
        || predicate.semantic_form.as_ref() != "binary"
    {
        return Err(CollectionDomainRefusal::UnsupportedShape);
    }
    let operation =
        operation_identity(predicate).ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    let arguments = predicate
        .body
        .get("arguments")
        .and_then(Value::as_array)
        .filter(|arguments| arguments.len() == 2)
        .ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    let parameter_count = function
        .body
        .get("members")
        .and_then(Value::as_array)
        .and_then(|members| members.first())
        .filter(|member| member.get("name").and_then(Value::as_str) == Some("parameters"))
        .and_then(|parameters| parameters.get("value"))
        .and_then(|value| value.get("members"))
        .and_then(Value::as_array)
        .map(Vec::len)
        .ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    let selected_parameter = arguments
        .iter()
        .filter_map(term_target)
        .filter_map(|id| package.graph().nodes.iter().find(|node| node.node_id == id))
        .find(|node| {
            node.semantic_form.as_ref() == "parameter"
                && parameter_name(node) == Some(binder_name)
                && parameter_level(node) == Some(parameter_count as u64)
                && &node.semantic_type == element_type
        })
        .ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    // For `>=`, the bound variable must be the left operand. Equality is symmetric.
    let selected_index = arguments
        .iter()
        .position(|argument| term_target(argument).as_ref() == Some(&selected_parameter.node_id))
        .ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    let literal = arguments
        .iter()
        .enumerate()
        .filter(|(index, _)| *index != selected_index)
        .map(|(_, term)| term)
        .next()
        .ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    match (element, operation, selected_index) {
        (CollectionElementDomain::Integer { .. }, "quire.op.integer.ge", 0) => {
            let threshold = integer_literal(package, literal, element_type, element)?;
            Ok(CollectionPredicateIdentity::IntegerGreaterOrEqual { threshold })
        }
        (CollectionElementDomain::Integer { .. }, "quire.op.integer.eq", _) => {
            let value = integer_literal(package, literal, element_type, element)?;
            Ok(CollectionPredicateIdentity::IntegerEqual { value })
        }
        (CollectionElementDomain::Boolean, "quire.op.boolean.ne", _) => {
            let value = boolean_literal(package, literal)?;
            Ok(CollectionPredicateIdentity::BooleanNotEqual { value })
        }
        _ => Err(CollectionDomainRefusal::UnsupportedShape),
    }
}

fn sequence_element_type(
    package: &CheckedPackageV2,
    collection: &CheckedSemanticNodeV2,
) -> Result<CheckedNodeId, CollectionDomainRefusal> {
    let type_node = package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_id == collection.semantic_type)
        .ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    let sequence = match type_node.semantic_form.as_ref() {
        "collection_bounds" => package
            .graph()
            .nodes
            .iter()
            .find(|node| node.node_id == type_node.semantic_type)
            .filter(|node| node.semantic_form.as_ref() == "sequence"),
        "sequence" => Some(type_node),
        _ => None,
    }
    .ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    let members = sequence
        .body
        .get("members")
        .and_then(Value::as_array)
        .filter(|members| members.len() == 1)
        .ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    term_target(
        members
            .first()
            .ok_or(CollectionDomainRefusal::UnsupportedShape)?,
    )
    .ok_or(CollectionDomainRefusal::UnsupportedShape)
}

fn supported_element_type(package: &CheckedPackageV2, element: &CheckedNodeId) -> bool {
    package
        .graph()
        .nodes
        .iter()
        .find(|node| &node.node_id == element)
        .is_some_and(|node| {
            (node.node_tag.as_ref() == "scalar_type" && node.semantic_form.as_ref() == "boolean")
                || (node.node_tag.as_ref() == "scalar_type"
                    && node.semantic_form.as_ref() == "integer")
                || (node.node_tag.as_ref() == "bounded_domain"
                    && node.semantic_form.as_ref() == "integer_range"
                    && integer_member_i128(node, "min").is_some()
                    && integer_member_i128(node, "max").is_some())
        })
}

fn verify_function_receiver(
    package: &CheckedPackageV2,
    function: &CheckedSemanticNodeV2,
    projection: &CheckedSemanticNodeV2,
    declaration: &CheckedNodeId,
) -> Result<(), CollectionDomainRefusal> {
    let parameters = function
        .body
        .get("members")
        .and_then(Value::as_array)
        .and_then(|members| members.first())
        .filter(|member| member.get("name").and_then(Value::as_str) == Some("parameters"))
        .and_then(|member| member.get("value"))
        .and_then(|value| value.get("members"))
        .and_then(Value::as_array)
        .ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    let parameter_ids = parameters
        .iter()
        .filter_map(term_target)
        .collect::<std::collections::BTreeSet<_>>();
    let receiver = projection
        .body
        .get("arguments")
        .and_then(Value::as_array)
        .and_then(|arguments| arguments.first())
        .and_then(term_target)
        .and_then(|id| package.graph().nodes.iter().find(|node| node.node_id == id))
        .filter(|node| operation_identity(node) == Some("quire.op.model.deref"))
        .ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    let parameter_id = receiver
        .body
        .get("arguments")
        .and_then(Value::as_array)
        .and_then(|arguments| arguments.first())
        .and_then(term_target)
        .ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    let parameter = package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_id == parameter_id)
        .filter(|node| {
            parameter_ids.contains(&node.node_id) && node.semantic_form.as_ref() == "parameter"
        })
        .ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    let parameter_type = package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_id == parameter.semantic_type)
        .ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    if parameter_type.semantic_form.as_ref() != "reference"
        || !depends_on(package, &parameter_type.node_id, declaration)
    {
        return Err(CollectionDomainRefusal::UnsupportedShape);
    }
    Ok(())
}

fn parameter_name(node: &CheckedSemanticNodeV2) -> Option<&str> {
    node.body
        .get("members")?
        .as_array()?
        .iter()
        .find(|member| member.get("name").and_then(Value::as_str) == Some("name"))?
        .get("value")?
        .get("value")?
        .as_str()
}

fn parameter_level(node: &CheckedSemanticNodeV2) -> Option<u64> {
    node.body
        .get("members")?
        .as_array()?
        .iter()
        .find(|member| member.get("name").and_then(Value::as_str) == Some("level"))?
        .get("value")?
        .get("value")?
        .as_str()?
        .parse()
        .ok()
}

fn integer_literal(
    package: &CheckedPackageV2,
    term: &Value,
    element_type: &CheckedNodeId,
    element: CollectionElementDomain,
) -> Result<i64, CollectionDomainRefusal> {
    let id = term_target(term).ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    let node = package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_id == id)
        .ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    let literal = if node.semantic_form.as_ref() == "literal" {
        node
    } else if operation_identity(node) == Some("quire.op.numeric.narrow")
        && node.semantic_form.as_ref() == "conversion"
        && &node.semantic_type == element_type
    {
        let source = node
            .body
            .get("arguments")
            .and_then(Value::as_array)
            .and_then(|arguments| arguments.first())
            .and_then(term_target)
            .ok_or(CollectionDomainRefusal::UnsupportedShape)?;
        package
            .graph()
            .nodes
            .iter()
            .find(|candidate| {
                candidate.node_id == source && candidate.semantic_form.as_ref() == "literal"
            })
            .ok_or(CollectionDomainRefusal::UnsupportedShape)?
    } else {
        return Err(CollectionDomainRefusal::UnsupportedShape);
    };
    if literal.body.get("value_kind").and_then(Value::as_str) != Some("integer") {
        return Err(CollectionDomainRefusal::UnsupportedShape);
    }
    let value = literal
        .body
        .get("value")
        .and_then(Value::as_str)
        .and_then(|value| value.parse().ok())
        .ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    if let CollectionElementDomain::Integer { minimum, maximum } = element {
        if i128::from(value) < minimum || i128::from(value) > maximum {
            return Err(CollectionDomainRefusal::UnsupportedShape);
        }
    }
    Ok(value)
}

fn boolean_literal(
    package: &CheckedPackageV2,
    term: &Value,
) -> Result<bool, CollectionDomainRefusal> {
    let id = term_target(term).ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    let literal = package
        .graph()
        .nodes
        .iter()
        .find(|node| node.node_id == id && node.semantic_form.as_ref() == "literal")
        .ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    if literal.body.get("value_kind").and_then(Value::as_str) != Some("boolean") {
        return Err(CollectionDomainRefusal::UnsupportedShape);
    }
    literal
        .body
        .get("value")
        .and_then(Value::as_bool)
        .ok_or(CollectionDomainRefusal::UnsupportedShape)
}

fn depends_on(package: &CheckedPackageV2, root: &CheckedNodeId, selected: &CheckedNodeId) -> bool {
    let mut pending = vec![root.clone()];
    let mut seen = std::collections::BTreeSet::new();
    while let Some(current) = pending.pop() {
        if &current == selected {
            return true;
        }
        if !seen.insert(current.clone()) {
            continue;
        }
        if let Some(node) = package
            .graph()
            .nodes
            .iter()
            .find(|node| node.node_id == current)
        {
            pending.extend(node.dependencies.iter().cloned());
        }
    }
    false
}

fn operation_identity(node: &CheckedSemanticNodeV2) -> Option<&str> {
    node.body
        .get("operation")
        .and_then(|operation| operation.get("identity"))
        .and_then(Value::as_str)
}

fn term_target(term: &Value) -> Option<CheckedNodeId> {
    let target = term.get("target")?;
    Some(CheckedNodeId {
        domain: target.get("domain")?.as_str()?.into(),
        digest: target.get("digest")?.as_str()?.into(),
    })
}

fn integer_member_i128(node: &CheckedSemanticNodeV2, name: &str) -> Option<i128> {
    node.body
        .get("members")?
        .as_array()?
        .iter()
        .find(|member| member.get("name").and_then(Value::as_str) == Some(name))?
        .get("value")?
        .get("value")?
        .as_str()?
        .parse()
        .ok()
}
