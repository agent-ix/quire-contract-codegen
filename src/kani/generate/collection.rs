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

/// Canonical digest of the complete semantic identity used to shorten generated Rust symbols.
/// The short symbol is only a path component; the persisted proof record retains the full fields.
fn collection_symbol_digest<T: serde::Serialize + ?Sized>(
    identity_preimage: &T,
) -> Result<String, UnsupportedObligation> {
    let value = crate::core::canonical::preimage_value(identity_preimage)
        .map_err(|_| UnsupportedObligation::RenderFailed)?;
    let digest = crate::core::canonical::content_digest(&value)
        .map_err(|_| UnsupportedObligation::RenderFailed)?;
    Ok(digest.iter().map(|byte| format!("{byte:02x}")).collect())
}

fn collection_harness_symbols(digest: &str) -> (String, String) {
    const SYMBOL_DIGEST_HEX_LEN: usize = 24;
    let symbol_digest = &digest[..SYMBOL_DIGEST_HEX_LEN];
    (
        format!("kob_collection_{symbol_digest}"),
        format!("prove_collection_{symbol_digest}"),
    )
}

/// Whether two generated collection harnesses would occupy the same Rust/Kani path while
/// representing different complete semantic identities.
pub(super) fn collection_symbols_conflict<I: PartialEq>(
    existing_module: &str,
    existing_harness: &str,
    existing_identity: &I,
    candidate_module: &str,
    candidate_harness: &str,
    candidate_identity: &I,
) -> bool {
    existing_module == candidate_module
        && existing_harness == candidate_harness
        && existing_identity != candidate_identity
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
    let minimum_length = collection_i64(i128::from(field.minimum_length), &field.quantifier)?;
    let maximum_length = collection_i64(i128::from(field.maximum_length), &field.quantifier)?;
    let array_len = collection_array_length(field.maximum_length, &field.quantifier)?;
    let required_unwind = field.maximum_length.checked_add(1);
    if required_unwind.is_none_or(|required| u64::from(request.unwind) < required) {
        return Err(UnsupportedObligation::CollectionClaimNotRendered {
            node_id: field.quantifier.clone(),
            reason: crate::kani::generate::outcome::CollectionUnsupportedReason::UnwindBelowCollectionBound {
                required: required_unwind.unwrap_or(u64::MAX),
                supplied: request.unwind,
            },
        });
    }
    let (element_type, element_domain, element_assumption) = match field.element {
        CollectionElementDomain::Boolean => {
            ("bool", CollectionElementIdentity::Boolean, String::new())
        }
        CollectionElementDomain::Integer { minimum, maximum } => {
            let minimum = collection_i64(minimum, &field.quantifier)?;
            let maximum = collection_i64(maximum, &field.quantifier)?;
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
    let symbol_material = serde_json::to_vec(&(
        (
            lowered.package_digest.as_str(),
            request.subject_path,
            &field.function,
            &field.quantifier,
            &field.projection,
            &field.declaration,
            &field.field_name,
            minimum_length,
            maximum_length,
        ),
        (
            &element_domain,
            &field.element_type,
            &field.quantifier_operation,
            &field.predicate_id,
            &field.predicate,
            KaniSolver::Cadical,
            request.unwind,
            request.ceilings,
        ),
    ))
    .map_err(|_| UnsupportedObligation::RenderFailed)?;
    let digest = collection_symbol_digest(&symbol_material)?;
    // Keep both Rust identifiers short enough for Kani's internal generated filenames. A
    // collision after truncation is detected during request negotiation before artifacts leave
    // this module; the full semantic identity remains in the proof record and artifact identity.
    let (module, harness) = collection_harness_symbols(&digest);
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

fn collection_i64(value: i128, node_id: &CheckedNodeId) -> Result<i64, UnsupportedObligation> {
    i64::try_from(value).map_err(|_| UnsupportedObligation::CollectionClaimNotRendered {
        node_id: node_id.clone(),
        reason: crate::kani::generate::outcome::CollectionUnsupportedReason::DomainNotRepresentableInI64,
    })
}

fn collection_array_length(
    value: u64,
    node_id: &CheckedNodeId,
) -> Result<usize, UnsupportedObligation> {
    usize::try_from(value).map_err(|_| UnsupportedObligation::CollectionClaimNotRendered {
        node_id: node_id.clone(),
        reason: crate::kani::generate::outcome::CollectionUnsupportedReason::DomainNotRepresentableInHarness,
    })
}

/// The selected field of a declaration's effective fields. A declaration whose fields cannot be
/// listed, and a listing without the selected field, are both `FieldNotAdmitted`: the field is
/// never taken from another declaration. Admitted input reaches neither branch (QSL refuses an
/// ambiguous field set at compile time and the reader validates the declaration), so the lookup is
/// generic over the table to be exercised directly.
fn declared_field<'a, F, E, T: ?Sized>(
    fields: Result<&'a F, E>,
    find: impl FnOnce(&'a F) -> Option<&'a T>,
) -> Result<&'a T, CollectionDomainRefusal> {
    fields
        .ok()
        .and_then(find)
        .ok_or(CollectionDomainRefusal::FieldNotAdmitted)
}

fn find_admitted_declaration<'a>(
    mut nodes: impl Iterator<Item = &'a CheckedSemanticNodeV2>,
    digest: &str,
) -> Result<CheckedNodeId, CollectionDomainRefusal> {
    nodes
        .find(|node| node.node_id.digest.as_ref() == digest)
        .map(|node| node.node_id.clone())
        .ok_or(CollectionDomainRefusal::FieldNotAdmitted)
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
    if !is_top_level_conjunct(package, &function_body, &quantifier.node_id) {
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
    let declaration = find_admitted_declaration(package.graph().nodes.iter(), declaration_digest)?;
    verify_function_receiver(package, function, collection, &declaration)?;
    let fields = package.model_object_fields(&declaration);
    let field = declared_field(fields.as_ref(), |fields| fields.field(&field_name))?;
    let Some(CheckedMemberType::Collection {
        kind: CheckedCollectionKind::Sequence,
        element,
        bounds,
    }) = field.member_type()
    else {
        return Err(CollectionDomainRefusal::UnsupportedFieldType);
    };
    let element_type = sequence_element_type(package, collection)?;
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
        CheckedMemberType::Integer
        | CheckedMemberType::String
        | CheckedMemberType::Uuid
        | CheckedMemberType::Timestamp => {
            return Err(CollectionDomainRefusal::RequiresBound {
                unbounded_type: element_type,
            });
        }
        _ => return Err(CollectionDomainRefusal::UnsupportedFieldType),
    };
    let Some((minimum_length, maximum_length)) = bounds else {
        return Err(CollectionDomainRefusal::RequiresBound {
            unbounded_type: collection.semantic_type.clone(),
        });
    };
    if minimum_length > maximum_length {
        return Err(CollectionDomainRefusal::UnsupportedFieldType);
    }
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

/// The operation identity of a `binary` expression predicate. Any other form, a quantity
/// conversion included, is an unsupported shape.
fn binary_predicate_operation(
    predicate: &CheckedSemanticNodeV2,
) -> Result<&str, CollectionDomainRefusal> {
    if CheckedNodeTag::from_wire(&predicate.node_tag) != Some(CheckedNodeTag::Expression)
        || predicate.semantic_form.as_ref() != "binary"
    {
        return Err(CollectionDomainRefusal::UnsupportedShape);
    }
    operation_identity(predicate).ok_or(CollectionDomainRefusal::UnsupportedShape)
}

fn resolve_predicate(
    package: &CheckedPackageV2,
    function: &CheckedSemanticNodeV2,
    binder_name: &str,
    predicate: &CheckedSemanticNodeV2,
    element: CollectionElementDomain,
    element_type: &CheckedNodeId,
) -> Result<CollectionPredicateIdentity, CollectionDomainRefusal> {
    let operation = binary_predicate_operation(predicate)?;
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
            let threshold = integer_literal(package, literal, element_type)?;
            Ok(CollectionPredicateIdentity::IntegerGreaterOrEqual { threshold })
        }
        (CollectionElementDomain::Integer { .. }, "quire.op.integer.eq", _) => {
            let value = integer_literal(package, literal, element_type)?;
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
    // Literals outside the admitted element interval remain representable in the claim. The
    // symbolic assumptions constrain slots, while this literal may make the claim falsifiable.
    Ok(value)
}

/// The selected quantifier may be the entire body or a conjunct reached only through `and`
/// nodes. A quantifier under negation, disjunction, implication, or another operation does not
/// represent an independent obligation and must not be asserted on its own.
fn is_top_level_conjunct(
    package: &CheckedPackageV2,
    root: &CheckedNodeId,
    selected: &CheckedNodeId,
) -> bool {
    if root == selected {
        return true;
    }
    let Some(node) = package
        .graph()
        .nodes
        .iter()
        .find(|node| &node.node_id == root)
    else {
        return false;
    };
    if operation_identity(node) != Some("quire.op.boolean.and") {
        return false;
    }
    node.body
        .get("arguments")
        .and_then(Value::as_array)
        .is_some_and(|arguments| {
            arguments
                .iter()
                .filter_map(term_target)
                .any(|child| is_top_level_conjunct(package, &child, selected))
        })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collection_symbols_are_bounded_and_stable_per_full_identity() {
        let identity = ("package-a", "function-a", "quantifier-a", "domain-a");
        let first = collection_symbol_digest(&identity).expect("canonical identity digest");
        let same = collection_symbol_digest(&identity).expect("same canonical identity");
        let changed_package =
            collection_symbol_digest(&("package-b", "function-a", "quantifier-a", "domain-a"))
                .expect("changed package identity");
        let changed_subject =
            collection_symbol_digest(&("package-a", "function-b", "quantifier-a", "domain-a"))
                .expect("changed subject identity");
        let changed_domain =
            collection_symbol_digest(&("package-a", "function-a", "quantifier-a", "domain-b"))
                .expect("changed domain identity");
        assert_eq!(first, same);
        assert_ne!(first, changed_package);
        assert_ne!(first, changed_subject);
        assert_ne!(first, changed_domain);
        assert_ne!(
            collection_harness_symbols(&first),
            collection_harness_symbols(&changed_package)
        );
        assert_ne!(
            collection_harness_symbols(&first),
            collection_harness_symbols(&changed_subject)
        );
        assert_ne!(
            collection_harness_symbols(&first),
            collection_harness_symbols(&changed_domain)
        );

        let (module, harness) = collection_harness_symbols(&first);
        assert!(module.len() <= 255, "module filename component: {module}");
        assert!(
            harness.len() <= 255,
            "harness filename component: {harness}"
        );
        assert!(format!("{module}::{harness}").len() <= 255);
    }

    fn node(label: &str) -> CheckedNodeId {
        CheckedNodeId {
            domain: "ix://test/domain".into(),
            digest: label.into(),
        }
    }

    fn base_claim() -> LoweredCollectionClaim {
        LoweredCollectionClaim {
            package_digest: "package-a".to_owned(),
            field: CollectionFieldDomain {
                function: node("function"),
                quantifier: node("quantifier"),
                declaration: node("declaration"),
                field_name: "items".to_owned(),
                projection: node("projection"),
                minimum_length: 0,
                maximum_length: 3,
                element: CollectionElementDomain::Integer {
                    minimum: 0,
                    maximum: 9,
                },
                element_type: node("element-type"),
                quantifier_operation: "quire.op.collection.forall".to_owned(),
                predicate: CollectionPredicateIdentity::IntegerGreaterOrEqual { threshold: 0 },
                predicate_id: node("predicate"),
            },
        }
    }

    fn rendered_module(
        claim: &LoweredCollectionClaim,
        subject_path: &str,
        unwind: u32,
        memory_bytes: u64,
    ) -> String {
        let request = KaniObligationRequest {
            ceilings: crate::kani::identity::ProofCeilings {
                memory_bytes: std::num::NonZeroU64::new(memory_bytes).expect("nonzero"),
                wall_clock: std::time::Duration::from_secs(60),
            },
            items: &[],
            subject_path,
            unwind,
        };
        render_collection(&request, claim)
            .expect("the base collection renders")
            .identity
            .module_symbol
            .to_string()
    }

    /// Every component of a collection's semantic identity changes the rendered Rust path, so two
    /// claims that differ only in one component never share a harness. Each variant below
    /// differs from the base in exactly one field of the digest preimage, holding the package
    /// digest, ids and ceilings constant where the field is not the one varied: a field dropped
    /// from the preimage makes its row fail.
    ///
    /// Trace: FR-015-AC-100, TC-025.
    #[test]
    fn every_identity_component_changes_the_rendered_harness_path() {
        const MEMORY: u64 = 1 << 30;
        let base = base_claim();
        let original = rendered_module(&base, "crate::subject", 8, MEMORY);
        assert_eq!(
            original,
            rendered_module(&base, "crate::subject", 8, MEMORY),
            "the path is deterministic"
        );
        type Change = fn(&mut LoweredCollectionClaim);
        let claim_changes: [(&str, Change); 13] = [
            ("package digest", |c| c.package_digest = "package-b".into()),
            ("function", |c| c.field.function = node("other-function")),
            ("quantifier", |c| {
                c.field.quantifier = node("other-quantifier")
            }),
            ("declaration", |c| {
                c.field.declaration = node("other-declaration")
            }),
            ("field name", |c| c.field.field_name = "other".into()),
            ("projection", |c| {
                c.field.projection = node("other-projection")
            }),
            ("minimum length", |c| c.field.minimum_length = 1),
            ("maximum length", |c| c.field.maximum_length = 4),
            ("element domain", |c| {
                c.field.element = CollectionElementDomain::Integer {
                    minimum: 0,
                    maximum: 10,
                }
            }),
            ("element type", |c| {
                c.field.element_type = node("other-element")
            }),
            ("quantifier operation", |c| {
                c.field.quantifier_operation = "quire.op.collection.exists".into()
            }),
            ("predicate", |c| {
                c.field.predicate =
                    CollectionPredicateIdentity::IntegerGreaterOrEqual { threshold: 1 }
            }),
            ("predicate id", |c| {
                c.field.predicate_id = node("other-predicate")
            }),
        ];
        for (name, change) in claim_changes {
            let mut changed = base_claim();
            change(&mut changed);
            assert_ne!(
                original,
                rendered_module(&changed, "crate::subject", 8, MEMORY),
                "{name} is part of the harness identity"
            );
        }
        assert_ne!(original, rendered_module(&base, "crate::other", 8, MEMORY));
        assert_ne!(
            original,
            rendered_module(&base, "crate::subject", 9, MEMORY)
        );
        assert_ne!(
            original,
            rendered_module(&base, "crate::subject", 8, MEMORY / 2)
        );
    }

    #[test]
    fn truncated_symbol_collision_is_detected_against_full_identity() {
        let shared_module = "kob_collection_0123456789abcdef0123";
        let shared_harness = "prove_collection_0123456789abcdef0123";
        assert!(collection_symbols_conflict(
            shared_module,
            shared_harness,
            &("package-a", "function-a", "quantifier-a"),
            shared_module,
            shared_harness,
            &("package-a", "function-b", "quantifier-a"),
        ));
        assert!(!collection_symbols_conflict(
            shared_module,
            shared_harness,
            &("package-a", "function-a", "quantifier-a"),
            shared_module,
            shared_harness,
            &("package-a", "function-a", "quantifier-a"),
        ));
        assert!(!collection_symbols_conflict(
            shared_module,
            shared_harness,
            &("package-a", "function-a", "quantifier-a"),
            "kob_collection_other",
            "prove_collection_other",
            &("package-a", "function-b", "quantifier-a"),
        ));
    }

    #[test]
    fn domain_representability_failures_keep_the_typed_reason() {
        let node_id = CheckedNodeId {
            domain: "ix://test/domain".into(),
            digest: "test-node".into(),
        };
        assert!(matches!(
            collection_i64(i128::from(i64::MAX) + 1, &node_id),
            Err(UnsupportedObligation::CollectionClaimNotRendered {
                node_id: rejected,
                reason: crate::kani::generate::outcome::CollectionUnsupportedReason::DomainNotRepresentableInI64,
            }) if rejected == node_id
        ));
        if usize::BITS < 64 {
            assert!(matches!(
                collection_array_length(u64::from(u32::MAX) + 1, &node_id),
                Err(UnsupportedObligation::CollectionClaimNotRendered {
                    node_id: rejected,
                    reason: crate::kani::generate::outcome::CollectionUnsupportedReason::DomainNotRepresentableInHarness,
                }) if rejected == node_id
            ));
        }
    }

    /// A quantity conversion is not a `binary` expression, so the collection predicate reader
    /// refuses it as an unsupported shape before reading an operand.
    ///
    /// Trace: FR-014-AC-2, TC-024.
    #[test]
    fn a_quantity_conversion_predicate_is_an_unsupported_shape() {
        let id = |digest: &str| {
            serde_json::json!({
                "domain": "quire.checked-semantic-node/v1",
                "digest": digest.repeat(64),
            })
        };
        let node = |form: &str, operator: &str, identity: &str| -> CheckedSemanticNodeV2 {
            serde_json::from_value(serde_json::json!({
                "node_id": id("1"),
                "schema_version": "quire.checked-semantic-graph/v2",
                "node_tag": "expression",
                "semantic_form": form,
                "semantic_type": id("2"),
                "dependencies": [],
                "occurrences": [],
                "body": {
                    "term": "application",
                    "operator": operator,
                    "operation": {"identity": identity, "laws": [], "mode": null,
                                  "member": null, "leaves": []},
                    "result_type": id("2"),
                    "arguments": [],
                },
            }))
            .expect("a checked expression node")
        };
        assert_eq!(
            binary_predicate_operation(&node("conversion", "convert", "quire.op.quantity.convert")),
            Err(CollectionDomainRefusal::UnsupportedShape)
        );
        assert_eq!(
            binary_predicate_operation(&node("binary", "binary", "quire.op.integer.ge")),
            Ok("quire.op.integer.ge")
        );
    }

    /// Trace: FR-015-AC-101, TC-025.
    #[test]
    fn a_declaration_without_listable_fields_or_the_selected_field_is_field_not_admitted() {
        use quire_contract_model::CheckedModelFieldsError;
        let table = vec!["items", "flags"];
        fn lookup<'a>(
            name: &'static str,
        ) -> impl FnOnce(&'a Vec<&'static str>) -> Option<&'a &'static str> {
            move |fields| fields.iter().find(|field| **field == name)
        }
        assert_eq!(
            declared_field(Ok::<_, ()>(&table), lookup("flags")),
            Ok(&"flags")
        );
        for error in [
            CheckedModelFieldsError::UnknownNode,
            CheckedModelFieldsError::NotModelObjectType,
            CheckedModelFieldsError::AmbiguousField("items".into()),
        ] {
            assert_eq!(
                declared_field(Err::<&Vec<&'static str>, _>(error), lookup("items")),
                Err(CollectionDomainRefusal::FieldNotAdmitted)
            );
        }
        assert_eq!(
            declared_field(Ok::<_, ()>(&table), lookup("missing")),
            Err(CollectionDomainRefusal::FieldNotAdmitted)
        );
    }

    #[test]
    fn unresolved_projection_declaration_is_field_not_admitted() {
        assert_eq!(
            find_admitted_declaration(std::iter::empty(), "missing-declaration"),
            Err(CollectionDomainRefusal::FieldNotAdmitted)
        );
    }
}
