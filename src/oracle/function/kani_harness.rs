// SPDX-License-Identifier: AGPL-3.0-or-later
//! FR-021-AC-44 and AC-46: the Kani-harness entry (IR-1244).
//!
//! The strict reader, QSL's acquisition and callable selection run here, on the host, over the
//! unit's retained bytes. Their outputs are emitted as literal descriptors; the emitted crate's
//! `cfg(kani)` items rebuild the prepared function from them with the genuine constructors and
//! run the generated body, so the proof never links the reader or the QSL compiler.

use qsl_replay::NominalDeclaration;
use quire_contract_model::{
    ApplicationOperator, CheckedNodeId, CheckedPackageEvidence, CheckedPackageReadLimits,
    CheckedPackageV2, CheckedPackageV2ReadResult, CheckedTerm,
};
use quire_exact::Presence;

use super::kani_descriptor::{node_key, value_type};
use super::owner_path::{
    acquire_checked_source, checked_composite_declarations, checked_function_declarations,
    select_checked_callable, CheckedFunctionSource,
};
use super::owner_render::{self, HEADER};
use super::{
    application_arguments, binding_value, reference_in_graph, source_body_indices,
    BinaryIntegerOperator, EqualityOperatorKind, Graph, EXACT_FUNCTION_CRATE_NAME,
};
use crate::core::artifact::{Artifact, MAX_GENERATED_SOURCE_BYTES};
use crate::core::ir::frame::application_identity;
use crate::core::naming::bounded_readable_component;
use crate::core::profile::function_oracle_crate_manifest;
use crate::oracle::claim::OracleGenerationError;

/// Everything the Kani-harness entry needs to derive one function's pre-decoded data.
pub struct KaniFunctionRequest<'a> {
    /// The unit's retained checked-package bytes; the entry reads them itself.
    pub package_bytes: &'a [u8],
    /// Read ceilings for the strict reader.
    pub read_limits: CheckedPackageReadLimits,
    /// Feature and dependency evidence for the strict reader.
    pub evidence: &'a CheckedPackageEvidence,
    /// The caller's source, packages, dependency input, limits and body resources.
    pub source: CheckedFunctionSource<'a>,
    /// The `pure_function` node to prove.
    pub function: CheckedNodeId,
}

/// One emitted Kani function harness crate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KaniFunctionHarness {
    /// Symbol suffix of the emitted `body_{symbol}` module and `kani_factory_{symbol}`.
    pub symbol: String,
    /// `Cargo.toml` and `src/lib.rs`, in that order.
    pub artifacts: Vec<Artifact>,
}

enum SelectedBody {
    Scalar(BinaryIntegerOperator),
    Equality(EqualityOperatorKind),
    Call,
}

fn scalar_operator(identity: &str) -> Option<BinaryIntegerOperator> {
    match identity {
        "quire.op.integer.add" => Some(BinaryIntegerOperator::Add),
        "quire.op.integer.sub" => Some(BinaryIntegerOperator::Subtract),
        "quire.op.integer.mul" => Some(BinaryIntegerOperator::Multiply),
        _ => None,
    }
}

fn equality_operator(identity: &str) -> Option<EqualityOperatorKind> {
    match identity {
        "quire.op.integer.eq" | "quire.op.boolean.eq" | "quire.op.structural.eq" => {
            Some(EqualityOperatorKind::Equal)
        }
        "quire.op.integer.ne" | "quire.op.boolean.ne" | "quire.op.structural.ne" => {
            Some(EqualityOperatorKind::NotEqual)
        }
        _ => None,
    }
}

/// Classify the selected function's body from the reader's admitted graph alone.
fn classify_selected_body(
    package: &CheckedPackageV2,
    function: &CheckedNodeId,
) -> Result<SelectedBody, OracleGenerationError> {
    let graph = Graph::from_checked(package);
    let body = graph
        .term(function)
        .and_then(|term| binding_value(term, "body"))
        .and_then(|term| reference_in_graph(term, &graph))
        .and_then(|body| graph.term(body))
        .ok_or(OracleGenerationError::InvalidPlanState)?;
    if matches!(
        body,
        CheckedTerm::Application {
            operator: ApplicationOperator::Call,
            ..
        }
    ) {
        return Ok(SelectedBody::Call);
    }
    let identity = application_identity(body).ok_or(OracleGenerationError::InvalidPlanState)?;
    if !application_arguments(body).is_some_and(|arguments| arguments.len() == 2) {
        return Err(OracleGenerationError::InvalidPlanState);
    }
    scalar_operator(identity)
        .map(SelectedBody::Scalar)
        .or_else(|| equality_operator(identity).map(SelectedBody::Equality))
        .ok_or(OracleGenerationError::InvalidPlanState)
}

fn presence(value: Presence) -> &'static str {
    match value {
        Presence::Required => "exact::Presence::Required",
        Presence::Optional => "exact::Presence::Optional",
    }
}

fn types(values: &[quire_exact::ValueType]) -> Result<String, OracleGenerationError> {
    Ok(values
        .iter()
        .map(value_type)
        .collect::<Result<Vec<_>, _>>()?
        .join(", "))
}

/// One reachable declaration as a literal descriptor: key, qualified name, form, members and
/// the final `node()` key `seal_union_verified` needs.
fn declaration_descriptor(view: &NominalDeclaration) -> Result<String, OracleGenerationError> {
    use qsl_replay::{NominalDeclarationForm as Form, NominalDeclarationMember as Member};
    let mut members = Vec::with_capacity(view.members().len());
    for member in view.members() {
        members.push(match (view.form(), member) {
            (
                Form::Record,
                Member::RecordField {
                    identifier,
                    value_type: field_type,
                    presence: field_presence,
                },
            ) => format!(
                "({:?}, {}, {})",
                identifier.as_str(),
                value_type(field_type)?,
                presence(*field_presence)
            ),
            (
                Form::Tuple,
                Member::TuplePosition {
                    value_type: position,
                    ..
                },
            ) => value_type(position)?,
            (
                Form::Union,
                Member::UnionMember {
                    member, payload, ..
                },
            ) => format!(
                "({:?}, {}, vec![{}])",
                member.identifier(),
                node_key(member.declaration())?,
                types(payload)?
            ),
            _ => return Err(OracleGenerationError::InvalidPlanState),
        });
    }
    let shape = match view.form() {
        Form::Record => "Record",
        Form::Tuple => "Tuple",
        Form::Union => "Union",
    };
    let name = view
        .qualified_name()
        .iter()
        .map(quire_exact::Identifier::as_str)
        .collect::<Vec<_>>()
        .join(".");
    Ok(format!(
        "DeclarationDescriptor {{ key: {}, name: {name:?}, node: {}, shape: ShapeDescriptor::{shape}(vec![{}]) }}",
        node_key(view.key())?,
        node_key(view.node())?,
        members.join(", ")
    ))
}

fn cancel_source(cancel: &quire_exact::Cancel) -> String {
    let cause = cancel.cause().map(|cause| match cause {
        quire_exact::CancelCause::Requested => "Requested",
        quire_exact::CancelCause::Deadline => "Deadline",
    });
    match cause {
        None => "exact::Cancel::new()".to_owned(),
        Some(cause) => format!(
            "{{ let cancel = exact::Cancel::new(); cancel.cancel(exact::CancelCause::{cause}); cancel }}"
        ),
    }
}

/// Proceed only on the reader's `Admitted` result; each other result is its own typed refusal.
fn admitted_package(
    result: CheckedPackageV2ReadResult,
) -> Result<Box<CheckedPackageV2>, OracleGenerationError> {
    match result {
        CheckedPackageV2ReadResult::Admitted(package) => Ok(package),
        CheckedPackageV2ReadResult::Refused(refusal) => {
            Err(OracleGenerationError::KaniReaderRefused(Box::new(refusal)))
        }
        CheckedPackageV2ReadResult::Incomplete(incomplete) => Err(
            OracleGenerationError::KaniReaderIncomplete(Box::new(incomplete)),
        ),
        CheckedPackageV2ReadResult::CatalogUnavailable(error) => Err(
            OracleGenerationError::KaniReaderCatalogUnavailable(Box::new(error)),
        ),
    }
}

/// Derive the pre-decoded prepared function for one function body through the genuine reader,
/// acquisition, selection, declaration and prepared-function path, and emit it as literal
/// descriptors beside the generated body module.
///
/// # Errors
/// Each of the five AC-46 causes is its own typed refusal and emits nothing: a refused,
/// incomplete or catalog-unavailable read; any owner-path failure (acquisition, selection,
/// declaration reading or prepared-function construction); and a nested-call body.
pub fn generate_kani_function_harness(
    request: &KaniFunctionRequest<'_>,
) -> Result<KaniFunctionHarness, OracleGenerationError> {
    let package = admitted_package(CheckedPackageV2::read(
        request.package_bytes,
        request.read_limits,
        request.evidence,
    ))?;
    let package: &CheckedPackageV2 = &package;
    let owner_path = |error: super::owner_path::CheckedFunctionError| {
        OracleGenerationError::KaniOwnerPathRefused { code: error.code() }
    };
    let acquired = acquire_checked_source(package, &request.source).map_err(owner_path)?;
    let selection = select_checked_callable(package, package, &request.function, &acquired)
        .map_err(owner_path)?;
    let body = classify_selected_body(package, &request.function)?;
    let owner =
        checked_function_declarations(package, &acquired, &selection).map_err(owner_path)?;
    checked_composite_declarations(&owner).map_err(owner_path)?;

    let graph = Graph::from_checked(package);
    let name = graph
        .node(&request.function)
        .and_then(|node| node.declaration.as_ref())
        .filter(|declaration| declaration.qualified_name.len() == 1)
        .map(|declaration| declaration.qualified_name[0].to_string())
        .ok_or(OracleGenerationError::InvalidPlanState)?;
    let index = *source_body_indices(package)
        .get(&request.function)
        .ok_or(OracleGenerationError::InvalidPlanState)?;
    let json_name =
        serde_json::to_string(&name).map_err(|_| OracleGenerationError::InvalidPlanState)?;
    let origin = format!(
        "sv::location::ExpressionOwner::Body {{ function: {json_name}.to_owned(), index: {index} }}"
    );
    let symbol = bounded_readable_component(&name);

    let item = match body {
        SelectedBody::Call => {
            return Err(OracleGenerationError::KaniNestedCallBody {
                function: request.function.clone(),
            });
        }
        SelectedBody::Scalar(operator) => {
            owner_render::scalar_item(&symbol, &request.function, operator, &origin)?
        }
        SelectedBody::Equality(operator) => {
            owner_render::equality_item(&symbol, &request.function, operator, &origin)?
        }
    };

    let declarations = owner
        .iter()
        .map(declaration_descriptor)
        .collect::<Result<Vec<_>, _>>()?
        .join(", ");
    let resources = &request.source.body_resources;
    let factory = format!(
        r#"
/// The prepared function rebuilt from host-emitted descriptors, for the Kani proof.
#[cfg(kani)]
pub(crate) fn kani_factory_{symbol}() -> body_{symbol}::Factory {{
    body_{symbol}::Factory(std::sync::Arc::new(PreparedFunction::from_descriptors(
        vec![{declarations}],
        vec![{parameters}],
        {result},
        sv::location::ExpressionLocation::root({origin}),
        sv::declaration::TypeEnvironmentLimits {{ ancestor_steps: {ancestor_steps}, work_units: {work_units} }},
        {cancel},
    )))
}}
"#,
        parameters = types(selection.signature().parameters())?,
        result = value_type(selection.signature().result())?,
        ancestor_steps = resources.environment_limits.ancestor_steps,
        work_units = resources.environment_limits.work_units,
        cancel = cancel_source(&resources.cancel),
    );

    let subject = crate::core::provenance::node_subject(&request.function);
    let lib = crate::core::provenance::with_rust_header(
        format!("{HEADER}{item}{factory}"),
        std::iter::once(subject.as_str()),
    );
    if lib.len() > MAX_GENERATED_SOURCE_BYTES {
        return Err(OracleGenerationError::SourceTooLarge {
            bytes: lib.len(),
            limit: MAX_GENERATED_SOURCE_BYTES,
            setting: "cg.function.source_bytes",
        });
    }
    Ok(KaniFunctionHarness {
        symbol,
        artifacts: vec![
            Artifact::new(
                "Cargo.toml",
                function_oracle_crate_manifest(EXACT_FUNCTION_CRATE_NAME),
            ),
            Artifact::new("src/lib.rs", lib),
        ],
    })
}

#[cfg(test)]
mod tests;
