//! Source-bound collection domains derived from admitted V2 model-field projections.
//!
//! This module deliberately does not infer a field from its display name or select a model
//! declaration by type. The projection's admitted declaration identity and member name select
//! the retained field table; unsupported shapes remain typed refusals for the negotiator.

use quire_contract_model::{
    CheckedCollectionKind, CheckedMemberType, CheckedNodeId, CheckedNodeTag, CheckedPackageV2,
    CheckedSemanticNodeV2, CheckedTerm, LiteralKind,
};
use std::{borrow::Borrow, collections::BTreeMap};

use crate::{
    core::artifact::MAX_GENERATED_SOURCE_BYTES,
    kani::{
        abi::{adapter_options, readable_component, KaniSolver},
        generate::{
            outcome::{CollectionUnsupportedReason, KaniObligationRequest, UnsupportedObligation},
            record::{harness_path, record},
            render,
            spec::{
                CollectionElement, CollectionQuantifier, CollectionSpec, HarnessSpec,
                HarnessSpecError,
            },
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

/// First admitted graph entries, retained in source order for collection lowering.
pub(super) struct CollectionGraph<'a> {
    package: &'a CheckedPackageV2,
    index: CollectionNodeIndex<'a>,
}

struct CollectionNodeIndex<'a> {
    package: Option<&'a CheckedPackageV2>,
    by_id: FirstMatchIndex<&'a CheckedNodeId, &'a CheckedSemanticNodeV2>,
    by_digest: FirstMatchIndex<&'a str, &'a CheckedSemanticNodeV2>,
}

#[cfg(test)]
thread_local! {
    static COLLECTION_GRAPH_BUILD_COUNT: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static COLLECTION_NODE_INDEX_BUILD_COUNT: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// Read and reset the number of real admitted collection graph constructions on this test thread.
#[cfg(test)]
pub(crate) fn take_collection_graph_builds() -> usize {
    COLLECTION_GRAPH_BUILD_COUNT.with(|count| count.replace(0))
}

struct FirstMatchIndex<K, V> {
    entries: BTreeMap<K, V>,
}

impl<K: Ord, V> FirstMatchIndex<K, V> {
    fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
        }
    }

    fn insert_first(&mut self, key: K, value: V) {
        self.entries.entry(key).or_insert(value);
    }

    fn get<Q: Ord + ?Sized>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
    {
        self.entries.get(key)
    }
}

impl<'a> CollectionGraph<'a> {
    pub(super) fn new(package: &'a CheckedPackageV2) -> Self {
        #[cfg(test)]
        COLLECTION_GRAPH_BUILD_COUNT.with(|count| count.set(count.get() + 1));
        let mut index = CollectionNodeIndex::from_nodes(package.graph().nodes.iter());
        index.package = Some(package);
        Self { package, index }
    }
}

impl<'a> CollectionNodeIndex<'a> {
    fn from_nodes(nodes: impl IntoIterator<Item = &'a CheckedSemanticNodeV2>) -> Self {
        #[cfg(test)]
        COLLECTION_NODE_INDEX_BUILD_COUNT.with(|count| count.set(count.get() + 1));
        let mut by_id = FirstMatchIndex::new();
        let mut by_digest = FirstMatchIndex::new();
        for node in nodes {
            by_id.insert_first(&node.node_id, node);
            by_digest.insert_first(node.node_id.digest.as_ref(), node);
        }
        Self {
            package: None,
            by_id,
            by_digest,
        }
    }

    fn node(&self, id: &CheckedNodeId) -> Option<&'a CheckedSemanticNodeV2> {
        self.by_id.get(id).copied()
    }

    fn declaration(&self, digest: &str) -> Option<&'a CheckedSemanticNodeV2> {
        self.by_digest.get(digest).copied()
    }

    fn term(&self, node: &CheckedSemanticNodeV2) -> Option<&'a CheckedTerm> {
        self.package?.term(&node.node_id)
    }
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
    graph: &CollectionGraph<'_>,
    function_id: &CheckedNodeId,
    quantifier_id: &CheckedNodeId,
) -> Result<LoweredCollectionClaim, CollectionDomainRefusal> {
    let quantifier = graph
        .index
        .node(quantifier_id)
        .ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    let field = resolve_collection_field(graph.package, &graph.index, function_id, quantifier)?;
    Ok(LoweredCollectionClaim {
        package_digest: graph.package.package_id().digest.to_string(),
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
    let (element, element_domain) = match field.element {
        CollectionElementDomain::Boolean => (
            CollectionElement::Boolean,
            CollectionElementIdentity::Boolean,
        ),
        CollectionElementDomain::Integer { minimum, maximum } => {
            let minimum = collection_i64(minimum, &field.quantifier)?;
            let maximum = collection_i64(maximum, &field.quantifier)?;
            (
                CollectionElement::Integer { minimum, maximum },
                CollectionElementIdentity::Integer { minimum, maximum },
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
    let quantifier = match field.quantifier_operation.as_str() {
        "quire.op.collection.forall" => CollectionQuantifier::ForAll,
        "quire.op.collection.exists" => CollectionQuantifier::Exists,
        _ => {
            return Err(UnsupportedObligation::CollectionClaimNotRendered {
                node_id: field.quantifier.clone(),
                reason: crate::kani::generate::outcome::CollectionUnsupportedReason::UnsupportedClaimShape,
            });
        }
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
    let spec = HarnessSpec::new_collection(
        path.clone(),
        CollectionSpec {
            field_name: field.field_name.clone(),
            item_name,
            length_name,
            minimum_length,
            maximum_length,
            array_len,
            element,
            quantifier,
            predicate: field.predicate.clone(),
        },
        request.unwind,
        KaniSolver::Cadical,
    )
    .map_err(|error| match error {
        HarnessSpecError::InvalidDomain => UnsupportedObligation::CollectionClaimNotRendered {
            node_id: field.quantifier.clone(),
            reason: CollectionUnsupportedReason::DomainNotRepresentableInHarness,
        },
        HarnessSpecError::InvalidUnwind(_)
        | HarnessSpecError::CoverCount(_)
        | HarnessSpecError::LiteralTrueCover
        | HarnessSpecError::CallCount(_)
        | HarnessSpecError::InputAfterCall
        | HarnessSpecError::AssumeAfterCall
        | HarnessSpecError::AssertBeforeCall
        | HarnessSpecError::DuplicateBinding(_) => UnsupportedObligation::RenderFailed,
        HarnessSpecError::InvalidIdentifier(_)
        | HarnessSpecError::InvalidMessage
        | HarnessSpecError::InvalidExpression
        | HarnessSpecError::ForbiddenExpression => UnsupportedObligation::InvalidGeneratedSyntax {
            error: error.to_string(),
        },
    })?;
    let rendered = render::render(&spec)?;
    let subjects = [
        crate::core::provenance::node_subject(&identity.function_id),
        crate::core::provenance::node_subject(&identity.quantifier_id),
        crate::core::provenance::node_subject(&identity.projection_id),
        crate::core::provenance::node_subject(&identity.declaration_id),
        crate::core::provenance::node_subject(&identity.element_type_id),
        crate::core::provenance::node_subject(&identity.predicate_id),
    ];
    let source = crate::core::provenance::with_rust_header(
        rendered.contents,
        subjects.iter().map(String::as_str),
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
    let rust = crate::core::artifact::Artifact::new(rendered.path, source);
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

fn admitted_declaration(
    graph: &CollectionNodeIndex<'_>,
    digest: &str,
) -> Result<CheckedNodeId, CollectionDomainRefusal> {
    graph
        .declaration(digest)
        .map(|node| node.node_id.clone())
        .ok_or(CollectionDomainRefusal::FieldNotAdmitted)
}

/// Resolve the first (collection) operand of an admitted quantifier to its exact model field.
///
/// Resolution uses the graph's projection declaration identity plus `model_object_fields`; it
/// never picks a declaration by matching a same-typed field elsewhere in the package.
fn resolve_collection_field(
    package: &CheckedPackageV2,
    graph: &CollectionNodeIndex<'_>,
    function_id: &CheckedNodeId,
    quantifier: &CheckedSemanticNodeV2,
) -> Result<CollectionFieldDomain, CollectionDomainRefusal> {
    let function = graph
        .node(function_id)
        .ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    if CheckedNodeTag::from_wire(&function.node_tag) != Some(CheckedNodeTag::Function) {
        return Err(CollectionDomainRefusal::UnsupportedShape);
    }
    let function_body = graph
        .term(function)
        .and_then(aggregate_members)
        .and_then(|members| binding(members, "body"))
        .and_then(term_target)
        .ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    if !is_top_level_conjunct(graph, &function_body, &quantifier.node_id) {
        return Err(CollectionDomainRefusal::UnsupportedShape);
    }
    if CheckedNodeTag::from_wire(&quantifier.node_tag) != Some(CheckedNodeTag::Expression)
        || quantifier.semantic_form.as_ref() != "quantify"
    {
        return Err(CollectionDomainRefusal::UnsupportedShape);
    }
    let arguments = graph
        .term(quantifier)
        .and_then(application_arguments)
        .ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    let quantifier_operation = operation_identity(graph, quantifier)
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
    let collection = graph
        .node(&collection_id)
        .ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    if CheckedNodeTag::from_wire(&collection.node_tag) != Some(CheckedNodeTag::Expression)
        || collection.semantic_form.as_ref() != "query"
        || operation_identity(graph, collection) != Some("quire.op.record.project")
    {
        return Err(CollectionDomainRefusal::UnsupportedShape);
    }
    let CheckedTerm::Application { operation, .. } = graph
        .term(collection)
        .ok_or(CollectionDomainRefusal::UnsupportedShape)?
    else {
        return Err(CollectionDomainRefusal::UnsupportedShape);
    };
    let (field_declaration, field_name) = operation
        .projection_field()
        .map_err(|_| CollectionDomainRefusal::UnsupportedShape)?
        .ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    let field_name = field_name.to_owned();
    let declaration = admitted_declaration(graph, &field_declaration.digest)?;
    verify_function_receiver(graph, function, collection, &declaration)?;
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
    let element_type = sequence_element_type(graph, collection)?;
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
    let Some(CheckedTerm::Binding {
        name: binder_name,
        value: predicate_value,
    }) = arguments.get(1)
    else {
        return Err(CollectionDomainRefusal::UnsupportedShape);
    };
    let predicate_id =
        term_target(predicate_value).ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    let predicate_node = graph
        .node(&predicate_id)
        .ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    let predicate = resolve_predicate(
        graph,
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
fn binary_predicate_operation<'a>(
    predicate: &CheckedSemanticNodeV2,
    term: Option<&'a CheckedTerm>,
) -> Result<&'a str, CollectionDomainRefusal> {
    if CheckedNodeTag::from_wire(&predicate.node_tag) != Some(CheckedNodeTag::Expression)
        || predicate.semantic_form.as_ref() != "binary"
    {
        return Err(CollectionDomainRefusal::UnsupportedShape);
    }
    let Some(CheckedTerm::Application { operation, .. }) = term else {
        return Err(CollectionDomainRefusal::UnsupportedShape);
    };
    operation
        .identity()
        .ok_or(CollectionDomainRefusal::UnsupportedShape)
}

fn resolve_predicate(
    graph: &CollectionNodeIndex<'_>,
    function: &CheckedSemanticNodeV2,
    binder_name: &str,
    predicate: &CheckedSemanticNodeV2,
    element: CollectionElementDomain,
    element_type: &CheckedNodeId,
) -> Result<CollectionPredicateIdentity, CollectionDomainRefusal> {
    let operation = binary_predicate_operation(predicate, graph.term(predicate))?;
    let arguments = graph
        .term(predicate)
        .and_then(application_arguments)
        .filter(|arguments| arguments.len() == 2)
        .ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    let parameter_count = graph
        .term(function)
        .and_then(aggregate_members)
        .and_then(|members| members.first())
        .and_then(|member| match member {
            CheckedTerm::Binding { name, value } if name.as_ref() == "parameters" => {
                aggregate_members(value)
            }
            _ => None,
        })
        .map(|members| members.len())
        .ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    let selected_parameter = arguments
        .iter()
        .filter_map(term_target)
        .filter_map(|id| graph.node(&id))
        .find(|node| {
            node.semantic_form.as_ref() == "parameter"
                && parameter_name(graph, node) == Some(binder_name)
                && parameter_level(graph, node) == Some(parameter_count as u64)
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
            let threshold = integer_literal(graph, literal, element_type)?;
            Ok(CollectionPredicateIdentity::IntegerGreaterOrEqual { threshold })
        }
        (CollectionElementDomain::Integer { .. }, "quire.op.integer.eq", _) => {
            let value = integer_literal(graph, literal, element_type)?;
            Ok(CollectionPredicateIdentity::IntegerEqual { value })
        }
        (CollectionElementDomain::Boolean, "quire.op.boolean.ne", _) => {
            let value = boolean_literal(graph, literal)?;
            Ok(CollectionPredicateIdentity::BooleanNotEqual { value })
        }
        _ => Err(CollectionDomainRefusal::UnsupportedShape),
    }
}

fn sequence_element_type(
    graph: &CollectionNodeIndex<'_>,
    collection: &CheckedSemanticNodeV2,
) -> Result<CheckedNodeId, CollectionDomainRefusal> {
    let type_node = graph
        .node(&collection.semantic_type)
        .ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    let sequence = match type_node.semantic_form.as_ref() {
        "collection_bounds" => graph
            .node(&type_node.semantic_type)
            .filter(|node| node.semantic_form.as_ref() == "sequence"),
        "sequence" => Some(type_node),
        _ => None,
    }
    .ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    let members = graph
        .term(sequence)
        .and_then(aggregate_members)
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
    graph: &CollectionNodeIndex<'_>,
    function: &CheckedSemanticNodeV2,
    projection: &CheckedSemanticNodeV2,
    declaration: &CheckedNodeId,
) -> Result<(), CollectionDomainRefusal> {
    let parameters = graph
        .term(function)
        .and_then(aggregate_members)
        .and_then(|members| members.first())
        .and_then(|member| match member {
            CheckedTerm::Binding { name, value } if name.as_ref() == "parameters" => {
                aggregate_members(value)
            }
            _ => None,
        })
        .ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    let parameter_ids = parameters
        .iter()
        .filter_map(term_target)
        .collect::<std::collections::BTreeSet<_>>();
    let receiver = graph
        .term(projection)
        .and_then(application_arguments)
        .and_then(|arguments| arguments.first())
        .and_then(term_target)
        .and_then(|id| graph.node(&id))
        .filter(|node| operation_identity(graph, node) == Some("quire.op.model.deref"))
        .ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    let parameter_id = graph
        .term(receiver)
        .and_then(application_arguments)
        .and_then(|arguments| arguments.first())
        .and_then(term_target)
        .ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    let parameter = graph
        .node(&parameter_id)
        .filter(|node| {
            parameter_ids.contains(&node.node_id) && node.semantic_form.as_ref() == "parameter"
        })
        .ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    let parameter_type = graph
        .node(&parameter.semantic_type)
        .ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    if parameter_type.semantic_form.as_ref() != "reference"
        || !depends_on(graph, &parameter_type.node_id, declaration)
    {
        return Err(CollectionDomainRefusal::UnsupportedShape);
    }
    Ok(())
}

fn parameter_name<'a>(
    graph: &CollectionNodeIndex<'a>,
    node: &CheckedSemanticNodeV2,
) -> Option<&'a str> {
    let members = aggregate_members(graph.term(node)?)?;
    let CheckedTerm::Literal { value, .. } = binding(members, "name")? else {
        return None;
    };
    value.as_str()
}

fn parameter_level(graph: &CollectionNodeIndex<'_>, node: &CheckedSemanticNodeV2) -> Option<u64> {
    let members = aggregate_members(graph.term(node)?)?;
    let CheckedTerm::Literal { value, .. } = binding(members, "level")? else {
        return None;
    };
    value.as_str()?.parse().ok()
}

fn integer_literal(
    graph: &CollectionNodeIndex<'_>,
    term: &CheckedTerm,
    element_type: &CheckedNodeId,
) -> Result<i64, CollectionDomainRefusal> {
    let id = term_target(term).ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    let node = graph
        .node(&id)
        .ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    let literal = if node.semantic_form.as_ref() == "literal" {
        node
    } else if operation_identity(graph, node) == Some("quire.op.numeric.narrow")
        && node.semantic_form.as_ref() == "conversion"
        && &node.semantic_type == element_type
    {
        let source = graph
            .term(node)
            .and_then(application_arguments)
            .and_then(|arguments| arguments.first())
            .and_then(term_target)
            .ok_or(CollectionDomainRefusal::UnsupportedShape)?;
        graph
            .node(&source)
            .filter(|candidate| candidate.semantic_form.as_ref() == "literal")
            .ok_or(CollectionDomainRefusal::UnsupportedShape)?
    } else {
        return Err(CollectionDomainRefusal::UnsupportedShape);
    };
    let Some(CheckedTerm::Literal {
        value_kind: LiteralKind::Integer,
        value,
        ..
    }) = graph.term(literal)
    else {
        return Err(CollectionDomainRefusal::UnsupportedShape);
    };
    let value = value
        .as_str()
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
    graph: &CollectionNodeIndex<'_>,
    root: &CheckedNodeId,
    selected: &CheckedNodeId,
) -> bool {
    if root == selected {
        return true;
    }
    let Some(node) = graph.node(root) else {
        return false;
    };
    if operation_identity(graph, node) != Some("quire.op.boolean.and") {
        return false;
    }
    graph
        .term(node)
        .and_then(application_arguments)
        .is_some_and(|arguments| {
            arguments
                .iter()
                .filter_map(term_target)
                .any(|child| is_top_level_conjunct(graph, &child, selected))
        })
}

fn boolean_literal(
    graph: &CollectionNodeIndex<'_>,
    term: &CheckedTerm,
) -> Result<bool, CollectionDomainRefusal> {
    let id = term_target(term).ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    let literal = graph
        .node(&id)
        .filter(|node| node.semantic_form.as_ref() == "literal")
        .ok_or(CollectionDomainRefusal::UnsupportedShape)?;
    let Some(CheckedTerm::Literal {
        value_kind: LiteralKind::Boolean,
        value,
        ..
    }) = graph.term(literal)
    else {
        return Err(CollectionDomainRefusal::UnsupportedShape);
    };
    value
        .as_bool()
        .ok_or(CollectionDomainRefusal::UnsupportedShape)
}

fn depends_on(
    graph: &CollectionNodeIndex<'_>,
    root: &CheckedNodeId,
    selected: &CheckedNodeId,
) -> bool {
    let mut pending = vec![root.clone()];
    let mut seen = std::collections::BTreeSet::new();
    while let Some(current) = pending.pop() {
        if &current == selected {
            return true;
        }
        if !seen.insert(current.clone()) {
            continue;
        }
        if let Some(node) = graph.node(&current) {
            pending.extend(node.dependencies.iter().cloned());
        }
    }
    false
}

fn aggregate_members(term: &CheckedTerm) -> Option<&[CheckedTerm]> {
    match term {
        CheckedTerm::Aggregate { members } => Some(members),
        _ => None,
    }
}

fn binding<'a>(members: &'a [CheckedTerm], name: &str) -> Option<&'a CheckedTerm> {
    members.iter().find_map(|member| match member {
        CheckedTerm::Binding {
            name: actual,
            value,
        } if actual.as_ref() == name => Some(value.as_ref()),
        _ => None,
    })
}

fn application_arguments(term: &CheckedTerm) -> Option<&[CheckedTerm]> {
    match term {
        CheckedTerm::Application { arguments, .. } => Some(arguments),
        _ => None,
    }
}

fn operation_identity<'a>(
    graph: &CollectionNodeIndex<'a>,
    node: &CheckedSemanticNodeV2,
) -> Option<&'a str> {
    let CheckedTerm::Application { operation, .. } = graph.term(node)? else {
        return None;
    };
    operation.identity()
}

fn term_target(term: &CheckedTerm) -> Option<CheckedNodeId> {
    match term {
        CheckedTerm::Reference { target } => Some(target.clone()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ix_trace_rs::trace;
    use quire_contract_model::ApplicationOperator;
    use std::{cell::Cell, cmp::Ordering, rc::Rc};

    #[derive(Clone)]
    struct CountedKey {
        value: usize,
        comparisons: Rc<Cell<usize>>,
    }

    impl PartialEq for CountedKey {
        fn eq(&self, other: &Self) -> bool {
            self.cmp(other) == Ordering::Equal
        }
    }

    impl Eq for CountedKey {}

    impl PartialOrd for CountedKey {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for CountedKey {
        fn cmp(&self, other: &Self) -> Ordering {
            self.comparisons.set(self.comparisons.get() + 1);
            self.value.cmp(&other.value)
        }
    }

    fn assert_collection_index_lookup_avoids_full_graph_walk() {
        let comparisons = Rc::new(Cell::new(0));
        let key = |value| CountedKey {
            value,
            comparisons: Rc::clone(&comparisons),
        };
        let mut index = FirstMatchIndex::new();
        for value in 0..128 {
            index.insert_first(key(value), value);
        }
        index.insert_first(key(127), 999);
        comparisons.set(0);
        for _ in 0..3 {
            assert_eq!(index.get(&key(127)), Some(&127), "first entry wins");
        }
        assert!(
            comparisons.get() < 128,
            "three lookups must avoid K*R comparisons; saw {} for K=3 R=128",
            comparisons.get(),
        );
    }

    fn assert_collection_graph_reuses_one_ordered_walk() {
        COLLECTION_NODE_INDEX_BUILD_COUNT.with(|count| count.set(0));
        let checked_node = |digit: &str| -> CheckedSemanticNodeV2 {
            let id = |digit: &str| {
                serde_json::json!({
                    "domain": "quire.checked-semantic-node/v1", "digest": digit.repeat(64),
                })
            };
            serde_json::from_value(serde_json::json!({
                "node_id": id(digit),
                "schema_version": "quire.checked-semantic-graph/v2",
                "node_tag": "expression", "semantic_form": "literal",
                "semantic_type": id("3"), "dependencies": [], "occurrences": [],
                "body": {"term": "literal", "type": id("3"),
                         "value_kind": "boolean", "value": true},
            }))
            .expect("checked node")
        };
        let preceding = checked_node("1");
        let selected = checked_node("2");
        let duplicate = checked_node("2");
        let visited = Cell::new(0usize);
        let nodes = std::iter::repeat_n(&preceding, 126).chain([&selected, &duplicate]);
        let graph =
            CollectionNodeIndex::from_nodes(nodes.inspect(|_| visited.set(visited.get() + 1)));
        COLLECTION_NODE_INDEX_BUILD_COUNT.with(|count| {
            assert_eq!(count.get(), 1, "the graph index was built once");
        });
        for _ in 0..3 {
            let found = graph.node(&selected.node_id);
            assert!(std::ptr::eq(found.expect("selected node"), &selected));
        }
        assert_eq!(visited.get(), 128, "one request graph construction");
        COLLECTION_NODE_INDEX_BUILD_COUNT.with(|count| {
            assert_eq!(count.get(), 1, "K lookups must reuse the same graph index");
        });
        assert!(
            std::ptr::eq(
                graph
                    .declaration(selected.node_id.digest.as_ref())
                    .expect("declaration"),
                &selected,
            ),
            "declaration lookup retains the first graph entry"
        );
        assert!(
            graph.node(&node("absent")).is_none(),
            "missing node remains missing"
        );
    }

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
    #[trace("FR-015-AC-100", "TC-025")]
    #[test]
    fn every_identity_component_changes_the_rendered_harness_path() {
        // The same collection implementation keeps first-match graph identity and bounded
        // lookup work while rendering the harness identities asserted below.
        assert_collection_index_lookup_avoids_full_graph_walk();
        assert_collection_graph_reuses_one_ordered_walk();

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

    #[trace("FR-015-AC-101", "TC-025")]
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

    #[trace("FR-015-AC-101", "TC-025")]
    #[test]
    fn rendering_refuses_an_unrepresentable_element_domain_without_a_harness() {
        let mut claim = base_claim();
        claim.field.element = CollectionElementDomain::Integer {
            minimum: 0,
            maximum: i128::from(i64::MAX) + 1,
        };
        let request = KaniObligationRequest {
            ceilings: crate::kani::identity::ProofCeilings {
                memory_bytes: std::num::NonZeroU64::new(1 << 30).unwrap(),
                wall_clock: std::time::Duration::from_secs(60),
            },
            items: &[],
            subject_path: "crate::subject",
            unwind: 8,
        };
        assert!(matches!(
            render_collection(&request, &claim),
            Err(UnsupportedObligation::CollectionClaimNotRendered {
                node_id,
                reason: CollectionUnsupportedReason::DomainNotRepresentableInI64,
            }) if node_id == claim.field.quantifier
        ));
    }

    /// A quantity conversion is not a `binary` expression, so the collection predicate reader
    /// refuses it as an unsupported shape before reading an operand.
    #[trace("FR-014-AC-2", "TC-024")]
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
        let term = |operator, identity| CheckedTerm::Application {
            operator,
            operation: serde_json::from_value(serde_json::json!({"identity": identity}))
                .expect("fixture operation"),
            result_type: CheckedNodeId {
                domain: "quire.checked-semantic-node/v1".into(),
                digest: "2".repeat(64).into(),
            },
            arguments: Vec::new(),
        };
        let conversion = term(ApplicationOperator::Convert, "quire.op.quantity.convert");
        let binary = term(ApplicationOperator::Binary, "quire.op.integer.ge");
        assert_eq!(
            binary_predicate_operation(
                &node("conversion", "convert", "quire.op.quantity.convert"),
                Some(&conversion),
            ),
            Err(CollectionDomainRefusal::UnsupportedShape)
        );
        assert_eq!(
            binary_predicate_operation(
                &node("binary", "binary", "quire.op.integer.ge"),
                Some(&binary),
            ),
            Ok("quire.op.integer.ge")
        );
    }

    #[trace("FR-015-AC-101", "TC-025")]
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
        let graph = CollectionNodeIndex::from_nodes(std::iter::empty());
        assert_eq!(
            admitted_declaration(&graph, "missing-declaration"),
            Err(CollectionDomainRefusal::FieldNotAdmitted),
        );
    }
}
