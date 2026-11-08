// SPDX-License-Identifier: AGPL-3.0-or-later
//! FR-021: apply a generated body through QSL's retained checked closure.

use qsl_replay::{
    acquire_checked_closure, AdmittedRefusal, ApplicationBeginRefusal, ApplicationEnvironment,
    CallableLookupRefusal, CallableSelection, CheckedClosureCompilation, ClosureOwnerRefusal,
    CompileUnit, DependencyInput, LibraryName, NestedCallOccurrence, NestedCallSite,
    NestedCallSiteRefusal, NominalDeclaration, NominalDeclarationMember, NominalDeclarationRefusal,
    ReplayLimits, SelectionRefusal, SourceIdentity, StageLimits,
};
use quire_contract_model::{
    encode_checked_package_v2, CheckedNodeId, CheckedOccurrenceRole, CheckedPackageSidecarsRef,
    CheckedPackageSourceCorrespondenceRef, CheckedPackageV2, ContractSourceKeyV1,
};
use quire_eval::{Admission, BodyFactory, CallFailure, Evaluation};
use quire_exact::{Meter, NodeKey, Value};
use quire_semantic_value::object_closure::ObjectClosure;

use super::ExactFunctionRefusal;

/// Caller-owned limits and cancellation for the generated body's QSV work.
#[derive(Clone, Default)]
pub struct CheckedFunctionResources {
    /// Limits for assembling the selected owner's nominal environment.
    pub environment_limits: quire_semantic_value::declaration::TypeEnvironmentLimits,
    /// The original cancellation handle, retained for typed work-stop attribution.
    pub cancel: quire_exact::Cancel,
}

/// The caller-owned inputs to one checked-closure acquisition.
pub struct CheckedFunctionSource<'a> {
    /// Identity of the supplied unit source.
    pub source: SourceIdentity,
    /// Source path used by QSL diagnostics.
    pub path: &'a str,
    /// Exact unit source bytes.
    pub bytes: &'a [u8],
    /// Supplied package bytes, in caller order.
    pub packages: &'a [&'a [u8]],
    /// Exact linked dependency input.
    pub dependencies: &'a DependencyInput,
    /// Compilation-stage limits.
    pub limits: StageLimits,
    /// Replay-stage limits.
    pub replay_limits: ReplayLimits,
    /// Body-side QSV limits, cancellation and reservation policy.
    pub body_resources: CheckedFunctionResources,
}

/// Stable categories of a checked-function application failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CheckedFunctionErrorCode {
    /// Acquisition boundary.
    Acquisition,
    /// Checked nominal declaration boundary.
    NominalDeclaration,
    /// InvalidIdentity boundary.
    InvalidIdentity,
    /// PackageEncoding boundary.
    PackageEncoding,
    /// OwnerPackage boundary.
    OwnerPackage,
    /// OwnerIdentityMismatch boundary.
    OwnerIdentityMismatch,
    /// FacadeLookup boundary.
    FacadeLookup,
    /// SelectionRefused boundary.
    SelectionRefused,
    /// MissingCallOccurrence boundary.
    MissingCallOccurrence,
    /// AmbiguousCallOccurrence boundary.
    AmbiguousCallOccurrence,
    /// NestedCallSiteRefused boundary.
    NestedCallSiteRefused,
    /// RegistrationNotAccepted boundary.
    RegistrationNotAccepted,
    /// ApplicationBeginRefused boundary.
    ApplicationBeginRefused,
}

/// A typed failure before a checked function application can run.
#[derive(Debug, thiserror::Error)]
pub enum CheckedFunctionError {
    /// QSL did not admit the supplied closure; no handle was issued.
    #[error("checked closure acquisition refused: {0:?}")]
    Acquisition(AdmittedRefusal),
    /// The selected owner did not issue its reachable nominal declaration.
    #[error(transparent)]
    NominalDeclaration(NominalDeclarationRefusal),
    /// An admitted source identity could not be resolved in its own graph frame.
    #[error("node identity cannot be resolved in its admitted source frame")]
    InvalidIdentity,
    /// The admitted package could not be represented by its canonical writer.
    #[error("admitted package encoding failed")]
    PackageEncoding,
    /// QSL refused the retained owner payload without recompiling it.
    #[error(transparent)]
    OwnerPackage(ClosureOwnerRefusal),
    /// The acquired unit or checker-issued selection differs from the IR-admitted identities.
    #[error("checked unit or declaring owner contents differ from the acquisition")]
    OwnerIdentityMismatch,
    /// QSL did not issue a callable selection.
    #[error(transparent)]
    FacadeLookup(CallableLookupRefusal),
    /// The selection belongs to a different acquisition.
    #[error(transparent)]
    SelectionRefused(SelectionRefusal),
    /// The declaring package has no expression occurrence for the requested call node.
    #[error("call node has no expression occurrence")]
    MissingCallOccurrence,
    /// More than one source occurrence of this call belongs to the selected body.
    #[error("call node has multiple matching expression occurrences")]
    AmbiguousCallOccurrence,
    /// QSL did not issue a nested site inside the selected checked body.
    #[error(transparent)]
    NestedCallSiteRefused(NestedCallSiteRefusal),
    /// The leaf did not accept the generated body registration.
    #[error("generated body registration refused: {0:?}")]
    RegistrationNotAccepted(Admission),
    /// Begin refused before body invocation or application charge.
    #[error(transparent)]
    ApplicationBeginRefused(ApplicationBeginRefusal),
}

impl CheckedFunctionError {
    /// The stable category, independent of the original typed payload.
    pub fn code(&self) -> CheckedFunctionErrorCode {
        match self {
            Self::Acquisition(_) => CheckedFunctionErrorCode::Acquisition,
            Self::NominalDeclaration(_) => CheckedFunctionErrorCode::NominalDeclaration,
            Self::InvalidIdentity => CheckedFunctionErrorCode::InvalidIdentity,
            Self::PackageEncoding => CheckedFunctionErrorCode::PackageEncoding,
            Self::OwnerPackage(_) => CheckedFunctionErrorCode::OwnerPackage,
            Self::OwnerIdentityMismatch => CheckedFunctionErrorCode::OwnerIdentityMismatch,
            Self::FacadeLookup(_) => CheckedFunctionErrorCode::FacadeLookup,
            Self::SelectionRefused(_) => CheckedFunctionErrorCode::SelectionRefused,
            Self::MissingCallOccurrence => CheckedFunctionErrorCode::MissingCallOccurrence,
            Self::AmbiguousCallOccurrence => CheckedFunctionErrorCode::AmbiguousCallOccurrence,
            Self::NestedCallSiteRefused(_) => CheckedFunctionErrorCode::NestedCallSiteRefused,
            Self::RegistrationNotAccepted(_) => CheckedFunctionErrorCode::RegistrationNotAccepted,
            Self::ApplicationBeginRefused(_) => CheckedFunctionErrorCode::ApplicationBeginRefused,
        }
    }

    /// Convert a post-acquisition failure into the claim-map's stable serializable refusal.
    /// Acquisition and malformed admitted IR identities retain their own typed boundaries.
    pub fn into_claim_refusal(self) -> Result<ExactFunctionRefusal, Self> {
        match self {
            Self::OwnerIdentityMismatch => Ok(ExactFunctionRefusal::OwnerIdentityMismatch),
            Self::OwnerPackage(cause) => {
                let cause_code = match cause {
                    ClosureOwnerRefusal::WrongAcquisition => "owner.wrong_acquisition",
                    ClosureOwnerRefusal::Unknown => "owner.unknown",
                    ClosureOwnerRefusal::OwnUnit => "owner.own_unit",
                };
                Ok(ExactFunctionRefusal::OwnerPackageRefused {
                    cause_code,
                    detail: format!("{cause:?}"),
                })
            }
            Self::FacadeLookup(cause) => Ok(map_lookup_refusal(cause)),
            Self::SelectionRefused(cause) => Ok(map_selection_refusal(cause)),
            Self::MissingCallOccurrence => Ok(ExactFunctionRefusal::NestedCallSite {
                cause_code: "site.missing_occurrence",
                detail: "the declaring V2 source map has no expression occurrence for this call"
                    .to_owned(),
            }),
            Self::AmbiguousCallOccurrence => Ok(ExactFunctionRefusal::NestedCallSite {
                cause_code: "site.ambiguous_occurrence",
                detail: "multiple call occurrences belong to the selected body".to_owned(),
            }),
            Self::NestedCallSiteRefused(cause) => Ok(map_nested_site_refusal(cause)),
            Self::RegistrationNotAccepted(admission) => {
                map_registration(admission).ok_or(Self::RegistrationNotAccepted(admission))
            }
            Self::ApplicationBeginRefused(cause) => Ok(map_begin_refusal(cause)),
            Self::Acquisition(_)
            | Self::NominalDeclaration(_)
            | Self::InvalidIdentity
            | Self::PackageEncoding => Err(self),
        }
    }
}

fn map_lookup_refusal(cause: CallableLookupRefusal) -> ExactFunctionRefusal {
    let cause_code = match cause {
        CallableLookupRefusal::Unknown => "lookup.unknown",
        CallableLookupRefusal::Foreign(_) => "lookup.foreign",
        CallableLookupRefusal::WrongKind => "lookup.wrong_kind",
    };
    ExactFunctionRefusal::FacadeLookup {
        cause_code,
        detail: format!("{cause:?}"),
    }
}

fn map_registration(admission: Admission) -> Option<ExactFunctionRefusal> {
    let cause_code = match admission {
        Admission::Accepted => return None,
        Admission::RegistrySealed => "registration.registry_sealed",
        Admission::CallableNotAdmitted => "registration.callable_not_admitted",
        Admission::SignatureMismatch => "registration.signature_mismatch",
        Admission::DuplicateBinding => "registration.duplicate_binding",
    };
    Some(ExactFunctionRefusal::RegistrationNotAccepted {
        cause_code,
        detail: format!("{admission:?}"),
    })
}

fn map_selection_refusal(cause: SelectionRefusal) -> ExactFunctionRefusal {
    let cause_code = match cause {
        SelectionRefusal::WrongAcquisition => "selection.wrong_acquisition",
    };
    ExactFunctionRefusal::SelectionRefused {
        cause_code,
        detail: format!("{cause:?}"),
    }
}

fn map_nested_site_refusal(cause: NestedCallSiteRefusal) -> ExactFunctionRefusal {
    let cause_code = match cause {
        NestedCallSiteRefusal::WrongAcquisition => "site.wrong_acquisition",
        NestedCallSiteRefusal::UnknownCall => "site.unknown_call",
        NestedCallSiteRefusal::ForeignCall(_) => "site.foreign_call",
        NestedCallSiteRefusal::WrongKind => "site.wrong_kind",
        NestedCallSiteRefusal::UnknownOccurrence => "site.unknown_occurrence",
        NestedCallSiteRefusal::Unreachable => "site.unreachable",
    };
    ExactFunctionRefusal::NestedCallSite {
        cause_code,
        detail: format!("{cause:?}"),
    }
}

fn map_begin_refusal(cause: ApplicationBeginRefusal) -> ExactFunctionRefusal {
    let cause_code = match &cause {
        ApplicationBeginRefusal::WrongAcquisition => "begin.wrong_acquisition",
        ApplicationBeginRefusal::Call(failure) => match failure {
            CallFailure::Input(_) => "begin.call.input",
            CallFailure::Admission(_) => "begin.call.admission",
            CallFailure::Body(_) => "begin.call.body",
            CallFailure::Fault(_) => "begin.call.fault",
            CallFailure::Cancelled(_) => "begin.call.cancelled",
            CallFailure::Storage { .. } => "begin.call.storage",
            CallFailure::Supplied(_) => "begin.call.supplied",
            CallFailure::UnsupportedHelper(_) => "begin.call.unsupported_helper",
        },
    };
    ExactFunctionRefusal::ApplicationBeginRefused {
        cause_code,
        detail: format!("{cause:?}"),
    }
}

/// Resolve an IR node in its admitted source graph, without inventing an identity.
pub fn checked_function_key(
    package: &CheckedPackageV2,
    node: &CheckedNodeId,
) -> Result<NodeKey, CheckedFunctionError> {
    let key = ContractSourceKeyV1::from_v2_request(package, node)
        .ok_or(CheckedFunctionError::InvalidIdentity)?;
    crate::core::ir::node_key::checked_node_key(key.structural_key())
        .ok_or(CheckedFunctionError::InvalidIdentity)
}

fn owner_name(package: &CheckedPackageV2) -> Result<LibraryName, CheckedFunctionError> {
    LibraryName::new(package.name()).map_err(|_| CheckedFunctionError::InvalidIdentity)
}

fn package_bytes(package: &CheckedPackageV2) -> Result<Vec<u8>, CheckedFunctionError> {
    encode_checked_package_v2(
        package.name(),
        package.lock(),
        package.graph(),
        CheckedPackageSourceCorrespondenceRef {
            source_map: package.source_map(),
            refinement_step_rows: package.refinement_step_rows(),
        },
        CheckedPackageSidecarsRef {
            checked_term_data: package.checked_term_data(),
            temporal_bindings: package.temporal_bindings(),
            temporal_clause_names: package.temporal_clause_names(),
            schedulability_claims: package.schedulability_claim_rows(),
            operation_effects: package.operation_effect_rows(),
        },
        package.capability_report(),
        package.diagnostics(),
        quire_canonical::Limits::new(u64::MAX),
    )
    .map_err(|_| CheckedFunctionError::PackageEncoding)
}

/// Select a callable only after binding complete unit and owner contents to this acquisition.
pub fn select_checked_callable(
    admitted_unit: &CheckedPackageV2,
    declaring_package: &CheckedPackageV2,
    function: &CheckedNodeId,
    acquired: &CheckedClosureCompilation,
) -> Result<CallableSelection, CheckedFunctionError> {
    let expected_unit = owner_name(admitted_unit)?;
    let expected_owner = owner_name(declaring_package)?;
    let unit_bytes = package_bytes(admitted_unit)?;
    let handle = acquired.handle();
    if handle.package_name() != &expected_unit || acquired.package().bytes() != unit_bytes {
        return Err(CheckedFunctionError::OwnerIdentityMismatch);
    }
    if !std::ptr::eq(admitted_unit, declaring_package) {
        let actual = if expected_owner == expected_unit {
            acquired.package().bytes()
        } else {
            acquired
                .owner_package_bytes(&expected_owner)
                .map_err(CheckedFunctionError::OwnerPackage)?
        };
        if actual != package_bytes(declaring_package)? {
            return Err(CheckedFunctionError::OwnerIdentityMismatch);
        }
    }
    let expected_function = checked_function_key(declaring_package, function)?;
    let selection = handle
        .lookup_callable(&expected_owner, &expected_function)
        .map_err(CheckedFunctionError::FacadeLookup)?;
    validate_selected_callable(&expected_owner, &expected_function, &selection)?;
    Ok(selection)
}

fn validate_selected_callable(
    owner: &LibraryName,
    function: &quire_exact::NodeKey,
    selection: &CallableSelection,
) -> Result<(), CheckedFunctionError> {
    if selection.owner_package_name() != owner || selection.key() != function {
        return Err(CheckedFunctionError::OwnerIdentityMismatch);
    }
    Ok(())
}

/// Issue the checked call location for one body-root call in a selected callable.
/// Only expression ordinals recorded by the declaring V2 package are considered;
/// QSL then verifies which of them belongs to this caller's retained body.
///
/// # Errors
///
/// Refuses an invalid owner or node identity, a missing or ambiguous source
/// occurrence, or QSL's typed refusal for a call outside the selected body.
pub fn selected_nested_call_site(
    declaring_package: &CheckedPackageV2,
    call: &CheckedNodeId,
    acquired: &CheckedClosureCompilation,
    caller: &CallableSelection,
) -> Result<NestedCallSite, CheckedFunctionError> {
    let handle = acquired.handle();
    let actual = match handle.selection_owner_package_bytes(caller) {
        Ok(bytes) => bytes,
        Err(ClosureOwnerRefusal::OwnUnit) => acquired.package().bytes(),
        Err(cause) => return Err(CheckedFunctionError::OwnerPackage(cause)),
    };
    if caller.owner_package_name() != &owner_name(declaring_package)?
        || actual != package_bytes(declaring_package)?
    {
        return Err(CheckedFunctionError::OwnerIdentityMismatch);
    }
    let call_wire = checked_function_key(declaring_package, call)?;
    let mut found = None;
    let mut first_refusal = None;
    let mut had_occurrence = false;
    for occurrence in declaring_package
        .source_map()
        .iter()
        .filter(|entry| entry.node_id == *call && entry.role == CheckedOccurrenceRole::Expression)
    {
        had_occurrence = true;
        match handle.nested_call_site(
            caller,
            NestedCallOccurrence::new(call_wire.clone(), occurrence.ordinal),
        ) {
            Ok(_) if found.is_some() => return Err(CheckedFunctionError::AmbiguousCallOccurrence),
            Ok(site) => found = Some(site),
            Err(refusal) => {
                first_refusal.get_or_insert(refusal);
            }
        }
    }
    if let Some(site) = found {
        return Ok(site);
    }
    if !had_occurrence {
        return Err(CheckedFunctionError::MissingCallOccurrence);
    }
    match first_refusal {
        Some(refusal) => Err(CheckedFunctionError::NestedCallSiteRefused(refusal)),
        None => Err(CheckedFunctionError::MissingCallOccurrence),
    }
}

/// Read each distinct nominal declaration reachable from the selected signature
/// through the owning checked closure. Keys, member order and sealed union
/// witnesses are returned unchanged; no declaration is invented or recompiled.
///
/// # Errors
/// Returns QSL's typed owner refusal if a reachable declaration is unavailable.
pub fn checked_function_declarations(
    declaring_package: &CheckedPackageV2,
    acquired: &CheckedClosureCompilation,
    selection: &CallableSelection,
) -> Result<Vec<NominalDeclaration>, CheckedFunctionError> {
    let handle = acquired.handle();
    let actual = match handle.selection_owner_package_bytes(selection) {
        Ok(bytes) => bytes,
        Err(ClosureOwnerRefusal::OwnUnit) => acquired.package().bytes(),
        Err(cause) => return Err(CheckedFunctionError::OwnerPackage(cause)),
    };
    if selection.owner_package_name() != &owner_name(declaring_package)?
        || actual != package_bytes(declaring_package)?
    {
        return Err(CheckedFunctionError::OwnerIdentityMismatch);
    }
    let mut declarations = Vec::new();
    let mut keys = std::collections::BTreeSet::new();
    for node in declaring_package
        .resolved_graph()
        .nodes
        .iter()
        .filter(|node| {
            node.node_tag.as_ref() == "composite_type"
                && matches!(node.semantic_form.as_ref(), "record" | "tuple" | "union")
        })
    {
        // The facade takes wire node identities. Union ValueType identities
        // instead carry kernel handles; retain the owner's explicit mapping.
        let node_key = checked_function_key(declaring_package, &node.node_id)?;
        match handle.nominal_declaration(selection, &node_key) {
            Ok(declaration) => {
                keys.insert(declaration.key().clone());
                keys.insert(declaration.node().clone());
                declarations.push(declaration);
            }
            // This authentic owner declaration is not reachable from this
            // selected signature. QSL owns and checks that reachability.
            Err(NominalDeclarationRefusal::Unknown) => {}
            Err(cause) => return Err(CheckedFunctionError::NominalDeclaration(cause)),
        }
    }
    let mut pending = selection
        .signature()
        .parameters()
        .iter()
        .chain(std::iter::once(selection.signature().result()))
        .collect::<Vec<_>>();
    for declaration in &declarations {
        for member in declaration.members() {
            match member {
                NominalDeclarationMember::RecordField { value_type, .. }
                | NominalDeclarationMember::TuplePosition { value_type, .. } => {
                    pending.push(value_type)
                }
                NominalDeclarationMember::UnionMember { payload, .. } => pending.extend(payload),
            }
        }
    }
    while let Some(value_type) = pending.pop() {
        match value_type {
            quire_exact::ValueType::Option(payload) => pending.push(payload),
            quire_exact::ValueType::Collection(collection) => pending.push(collection.element()),
            quire_exact::ValueType::Map(map) => pending.push(map.values()),
            quire_exact::ValueType::Composite(key) if !keys.contains(key) => {
                return Err(CheckedFunctionError::NominalDeclaration(
                    NominalDeclarationRefusal::Unknown,
                ));
            }
            quire_exact::ValueType::Composite(_)
            | quire_exact::ValueType::Boolean
            | quire_exact::ValueType::Integer
            | quire_exact::ValueType::Int(_)
            | quire_exact::ValueType::Rational(_)
            | quire_exact::ValueType::Decimal(_)
            | quire_exact::ValueType::Float(_)
            | quire_exact::ValueType::Quantity(_)
            | quire_exact::ValueType::BoundedQuantity(_)
            | quire_exact::ValueType::Text(_)
            | quire_exact::ValueType::String
            | quire_exact::ValueType::Uuid
            | quire_exact::ValueType::Timestamp
            | quire_exact::ValueType::Enum(_)
            | quire_exact::ValueType::Reference(_)
            | quire_exact::ValueType::Population(_)
            | quire_exact::ValueType::CarriedQuantity(_) => {}
        }
    }
    Ok(declarations)
}

/// Acquire one genuine checked closure, bind a generated body, and run its selected callable.
/// The returned inner `CallFailure` is the leaf's terminal result, never a CG reinterpretation.
pub fn apply_checked_function<Factory, Build>(
    source: CheckedFunctionSource<'_>,
    admitted_unit: &CheckedPackageV2,
    declaring_package: &CheckedPackageV2,
    function: &CheckedNodeId,
    arguments: Vec<Value>,
    objects: ObjectClosure,
    meter: &mut Meter,
    build_factory: Build,
) -> Result<Result<Evaluation, CallFailure>, CheckedFunctionError>
where
    Factory: BodyFactory + 'static,
    Build: FnOnce(
        &CheckedClosureCompilation,
        &CallableSelection,
        &CheckedFunctionResources,
    ) -> Result<Factory, CheckedFunctionError>,
{
    let acquired = acquire_checked_closure(
        CompileUnit {
            source: source.source,
            package_name: admitted_unit.name().to_owned(),
            path: source.path,
            bytes: source.bytes,
        },
        source.packages.iter().copied(),
        source.dependencies,
        source.limits,
        source.replay_limits,
    )
    .map_err(CheckedFunctionError::Acquisition)?;
    let handle = acquired.handle();
    let selection = select_checked_callable(admitted_unit, declaring_package, function, &acquired)?;
    let factory = build_factory(&acquired, &selection, &source.body_resources)?;
    let mut registry = handle.registry();
    let admission = registry
        .register(&selection, selection.signature(), factory)
        .map_err(CheckedFunctionError::SelectionRefused)?;
    if admission != Admission::Accepted {
        return Err(CheckedFunctionError::RegistrationNotAccepted(admission));
    }
    let environment = ApplicationEnvironment::new(objects);
    let session = registry
        .begin_call(&selection, arguments, &environment, meter)
        .map_err(CheckedFunctionError::ApplicationBeginRefused)?;
    Ok(session.run())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ix_trace_rs::trace;
    use quire_contract_model::{
        CheckedPackageEvidence, CheckedPackageReadLimits, CheckedPackageV2,
        CheckedPackageV2ReadResult,
    };
    use quire_eval::{Body, ChildRequest, FamilyOutcome, Resume, Transition};
    use quire_exact::{Integer, LimitKind, NodeKey, Outcome, ScalarLimits};
    use quire_semantic_value::catalog::InternalFault;
    use quire_semantic_value::location::Location;
    use std::sync::Arc;

    const SOURCE: &str = concat!(
        "language \"ix:native\" edition \"1-draft\";\n",
        "profile v = \"quire.value.complete/v1\";\n",
        "function f using v(x: Integer): Integer pure { x }\n",
        "function g using v(x: Integer): Integer pure { x }\n",
    );
    const OTHER_SOURCE: &str = concat!(
        "language \"ix:native\" edition \"1-draft\";\n",
        "profile v = \"quire.value.complete/v1\";\n",
        "function f using v(x: Integer): Integer pure { x }\n",
    );
    const LIBRARY_SOURCE: &str = concat!(
        "language \"ix:native\" edition \"1-draft\";\n",
        "profile v = \"quire.value.complete/v1\";\n",
        "function imported using v(x: Integer): Integer pure { x }\n",
        "function outer using v(x: Integer): Integer pure { imported(x) }\n",
    );

    fn identity() -> SourceIdentity {
        SourceIdentity::new("agent-ix", "function-owner")
    }

    fn admitted() -> (CheckedPackageV2, CheckedNodeId, CheckedNodeId) {
        let compiled = acquire_checked_closure(
            CompileUnit {
                source: identity(),
                package_name: "function-owner".to_owned(),
                path: "function-owner.native",
                bytes: SOURCE.as_bytes(),
            },
            [],
            &DependencyInput::default(),
            StageLimits::default(),
            ReplayLimits::default(),
        )
        .expect("genuine checked source");
        let mut evidence = CheckedPackageEvidence::new();
        evidence.support_feature("quire.value.complete/v1");
        let read = CheckedPackageV2::read(
            compiled.package().bytes(),
            CheckedPackageReadLimits::bounded(),
            &evidence,
        );
        let CheckedPackageV2ReadResult::Admitted(package) = read else {
            panic!("the same QSL emission must be admitted by Contract IR: {read:?}");
        };
        let function = |name: &str| {
            package
                .resolved_graph()
                .nodes
                .iter()
                .find(|node| {
                    node.declaration
                        .as_ref()
                        .is_some_and(|decl| decl.qualified_name == [Box::<str>::from(name)])
                })
                .expect("named source function")
                .node_id
                .clone()
        };
        let f = function("f");
        let g = function("g");
        (*package, f, g)
    }

    fn separately_admitted_unit() -> CheckedPackageV2 {
        let compiled = acquire_checked_closure(
            CompileUnit {
                source: identity(),
                package_name: "function-owner".to_owned(),
                path: "function-owner.native",
                bytes: OTHER_SOURCE.as_bytes(),
            },
            [],
            &DependencyInput::default(),
            StageLimits::default(),
            ReplayLimits::default(),
        )
        .expect("second genuine checked source");
        let mut evidence = CheckedPackageEvidence::new();
        evidence.support_feature("quire.value.complete/v1");
        let CheckedPackageV2ReadResult::Admitted(package) = CheckedPackageV2::read(
            compiled.package().bytes(),
            CheckedPackageReadLimits::bounded(),
            &evidence,
        ) else {
            panic!("the second QSL emission must be admitted by Contract IR");
        };
        *package
    }

    fn imported() -> (
        CheckedPackageV2,
        Arc<CheckedPackageV2>,
        CheckedNodeId,
        DependencyInput,
        String,
    ) {
        let library_source = SourceIdentity::new("agent-ix", "function-library");
        let library_compiled = acquire_checked_closure(
            CompileUnit {
                source: library_source.clone(),
                package_name: "test/function-library".to_owned(),
                path: "function-library.native",
                bytes: LIBRARY_SOURCE.as_bytes(),
            },
            [],
            &DependencyInput::default(),
            StageLimits::default(),
            ReplayLimits::default(),
        )
        .expect("genuine library source");
        let mut library_evidence = CheckedPackageEvidence::new();
        library_evidence.support_feature("quire.value.complete/v1");
        let library_read = CheckedPackageV2::read(
            library_compiled.package().bytes(),
            CheckedPackageReadLimits::bounded(),
            &library_evidence,
        );
        let CheckedPackageV2ReadResult::Admitted(library) = library_read else {
            panic!("Contract IR must admit the library: {library_read:?}");
        };
        let library = Arc::new(*library);
        let function = library
            .resolved_graph()
            .nodes
            .iter()
            .find(|node| {
                node.declaration.as_ref().is_some_and(|declaration| {
                    declaration.qualified_name == [Box::<str>::from("imported")]
                })
            })
            .expect("library function declaration")
            .node_id
            .clone();
        let unit_source = concat!(
            "language \"ix:native\" edition \"1-draft\";\n",
            "profile v = \"quire.value.complete/v1\";\n",
            "import \"test/function-library\" as lib;\n",
            "function caller using v(x: Integer): Integer pure { lib::imported(x) }\n",
        );
        let dependencies = DependencyInput::new([qsl_replay::SuppliedLibrary {
            identity: "test/function-library".to_owned(),
            source: library_source,
            path: "function-library.native".to_owned(),
            bytes: LIBRARY_SOURCE.as_bytes().to_vec(),
        }])
        .expect("one authentic supplied library");
        let unit_compiled = acquire_checked_closure(
            CompileUnit {
                source: identity(),
                package_name: "function-importer".to_owned(),
                path: "function-importer.native",
                bytes: unit_source.as_bytes(),
            },
            [],
            &dependencies,
            StageLimits::default(),
            ReplayLimits::default(),
        )
        .expect("unit imports the supplied library");
        let mut unit_evidence = CheckedPackageEvidence::new();
        unit_evidence.support_feature("quire.value.complete/v1");
        unit_evidence
            .insert_dependency_package("test/function-library", Arc::clone(&library))
            .expect("retain the admitted declaring package");
        let CheckedPackageV2ReadResult::Admitted(unit) = CheckedPackageV2::read(
            unit_compiled.package().bytes(),
            CheckedPackageReadLimits::bounded(),
            &unit_evidence,
        ) else {
            panic!("Contract IR must admit the importing unit");
        };
        (
            *unit,
            library,
            function,
            dependencies,
            unit_source.to_owned(),
        )
    }

    fn meter_with_work(work_units: u64) -> Meter {
        let large = u64::from(u32::MAX);
        Meter::new(ScalarLimits {
            integer_bits: large,
            decimal_digits: large,
            scale_expansion: large,
            text_input_bytes: large,
            text_scalars: large,
            normalized_scalars: large,
            unit_edges: large,
            value_occurrences: large,
            work_units,
            result_units: large,
        })
    }

    fn meter() -> Meter {
        meter_with_work(u64::from(u32::MAX))
    }

    struct EchoFactory;
    impl BodyFactory for EchoFactory {
        fn begin(&self) -> Box<dyn Body> {
            Box::new(EchoBody)
        }
    }
    struct EchoBody;
    impl Body for EchoBody {
        fn resume(
            &mut self,
            input: Resume,
            _meter: &mut Meter,
        ) -> Result<Transition, quire_eval::BodyFailure> {
            match input {
                Resume::Start(mut arguments) if arguments.len() == 1 => {
                    Ok(Transition::Done(Evaluation::completed(arguments.remove(0))))
                }
                _ => Err(InternalFault::new("generated_function", "unexpected_resume").into()),
            }
        }
    }

    struct SitedCallFactory {
        callee: NodeKey,
        site: Location,
    }

    impl BodyFactory for SitedCallFactory {
        fn begin(&self) -> Box<dyn Body> {
            Box::new(SitedCallBody {
                callee: self.callee.clone(),
                site: self.site.clone(),
            })
        }
    }

    struct SitedCallBody {
        callee: NodeKey,
        site: Location,
    }

    impl Body for SitedCallBody {
        fn resume(
            &mut self,
            input: Resume,
            _meter: &mut Meter,
        ) -> Result<Transition, quire_eval::BodyFailure> {
            match input {
                Resume::Start(arguments) => Ok(Transition::Call(ChildRequest {
                    callee: self.callee.clone(),
                    arguments,
                    site: self.site.clone(),
                })),
                Resume::Child(result) => Ok(Transition::Done(*result)),
                Resume::Again => {
                    Err(InternalFault::new("generated_function", "unexpected_resume").into())
                }
            }
        }
    }

    #[trace("TC-031", "FR-021-AC-35", "FR-021-AC-36")]
    #[test]
    fn checked_owner_path_applies_a_genuine_source_callable() {
        let (package, f, _) = admitted();
        let dependencies = DependencyInput::default();
        let mut meter = meter();
        let value = Value::Integer(Integer::from(4_u64));
        let result = apply_checked_function(
            CheckedFunctionSource {
                source: identity(),
                path: "function-owner.native",
                bytes: SOURCE.as_bytes(),
                packages: &[],
                dependencies: &dependencies,
                limits: StageLimits::default(),
                replay_limits: ReplayLimits::default(),
                body_resources: CheckedFunctionResources::default(),
            },
            &package,
            &package,
            &f,
            vec![value],
            ObjectClosure::default(),
            &mut meter,
            |_, _, _| Ok(EchoFactory),
        )
        .expect("same source and admitted identities")
        .expect("leaf terminal result");
        assert!(matches!(
            result.outcome,
            FamilyOutcome::Evaluated(Outcome::Completed(Value::Integer(ref number)))
                if *number == Integer::from(4_u64)
        ));
    }

    #[trace("TC-031", "FR-021-AC-35", "FR-021-AC-36")]
    #[test]
    fn checked_owner_path_applies_a_locked_imported_callable() {
        let (unit, library, function, dependencies, source_bytes) = imported();
        assert_ne!(unit.name(), library.name());
        assert!(unit
            .lock()
            .dependency_selections
            .iter()
            .any(|selection| selection.identity.as_ref() == library.name()));
        let mut meter = meter();
        let result = apply_checked_function(
            CheckedFunctionSource {
                source: identity(),
                path: "function-importer.native",
                bytes: source_bytes.as_bytes(),
                packages: &[],
                dependencies: &dependencies,
                limits: StageLimits::default(),
                replay_limits: ReplayLimits::default(),
                body_resources: CheckedFunctionResources::default(),
            },
            &unit,
            &library,
            &function,
            vec![Value::Integer(Integer::from(6_u64))],
            ObjectClosure::default(),
            &mut meter,
            |handle, selection, _| {
                assert_eq!(
                    selection.owner_package_name(),
                    &owner_name(&library).expect("IR library identity")
                );
                assert_eq!(
                    selection.key(),
                    &checked_function_key(&library, &function).expect("IR declaration identity")
                );
                assert_eq!(
                    handle.handle().package_name(),
                    &owner_name(&unit).expect("IR unit identity")
                );
                Ok(EchoFactory)
            },
        )
        .expect("QSL selected the locked imported declaration")
        .expect("the leaf completed the imported callable");
        assert!(matches!(
            result.outcome,
            FamilyOutcome::Evaluated(Outcome::Completed(Value::Integer(ref number)))
                if *number == Integer::from(6_u64)
        ));
    }

    #[trace("TC-031", "FR-021-AC-17", "FR-021-AC-36")]
    #[test]
    fn nested_call_site_comes_from_the_selected_checked_body() {
        let (unit, library, imported_function, dependencies, source_bytes) = imported();
        let acquired = acquire_checked_closure(
            CompileUnit {
                source: identity(),
                package_name: "function-importer".to_owned(),
                path: "function-importer.native",
                bytes: source_bytes.as_bytes(),
            },
            [],
            &dependencies,
            StageLimits::default(),
            ReplayLimits::default(),
        )
        .expect("QSL retains the imported closure");
        let handle = &acquired;
        let caller = unit
            .resolved_graph()
            .nodes
            .iter()
            .find(|node| {
                node.declaration.as_ref().is_some_and(|declaration| {
                    declaration.qualified_name == [Box::<str>::from("caller")]
                })
            })
            .expect("admitted caller")
            .node_id
            .clone();
        let call = unit
            .resolved_graph()
            .nodes
            .iter()
            .find(|node| {
                node.node_tag.as_ref() == "expression" && node.semantic_form.as_ref() == "call"
            })
            .expect("checked nested call")
            .node_id
            .clone();
        let occurrence = unit
            .source_map()
            .iter()
            .find(|entry| entry.node_id == call && entry.role == CheckedOccurrenceRole::Expression)
            .expect("exact expression occurrence in the IR-admitted source map");
        let caller_selection = select_checked_callable(&unit, &unit, &caller, handle)
            .expect("QSL selects the own caller");
        let site = selected_nested_call_site(&unit, &call, handle, &caller_selection)
            .expect("QSL issues this caller's nested site");
        assert_eq!(site.owner_package_name(), &owner_name(&unit).unwrap());
        let issued = handle
            .handle()
            .nested_call_site(
                &caller_selection,
                NestedCallOccurrence::new(
                    checked_function_key(&unit, &call).expect("admitted call identity"),
                    occurrence.ordinal,
                ),
            )
            .expect("QSL issues the IR-selected occurrence");
        assert_eq!(site.location(), issued.location());
        assert!(matches!(
            &site.location().origin,
            quire_semantic_value::location::Origin::Body { function, .. } if function == "caller"
        ));

        let wrong_ordinal = handle.handle().nested_call_site(
            &caller_selection,
            NestedCallOccurrence::new(
                checked_function_key(&unit, &call).expect("admitted call identity"),
                occurrence.ordinal + 100,
            ),
        );
        assert!(matches!(
            wrong_ordinal,
            Err(NestedCallSiteRefusal::UnknownOccurrence)
        ));

        let imported_selection =
            select_checked_callable(&unit, &library, &imported_function, handle)
                .expect("QSL selects the imported declaration");
        assert!(matches!(
            selected_nested_call_site(&unit, &call, handle, &imported_selection),
            Err(CheckedFunctionError::OwnerIdentityMismatch)
        ));

        let outer = library
            .resolved_graph()
            .nodes
            .iter()
            .find(|node| {
                node.declaration.as_ref().is_some_and(|declaration| {
                    declaration.qualified_name == [Box::<str>::from("outer")]
                })
            })
            .expect("admitted imported caller")
            .node_id
            .clone();
        let imported_call = library
            .resolved_graph()
            .nodes
            .iter()
            .find(|node| {
                node.node_tag.as_ref() == "expression" && node.semantic_form.as_ref() == "call"
            })
            .expect("admitted imported caller's call")
            .node_id
            .clone();
        let outer_selection = select_checked_callable(&unit, &library, &outer, handle)
            .expect("QSL selects the imported caller");
        let imported_site =
            selected_nested_call_site(&library, &imported_call, handle, &outer_selection)
                .expect("QSL issues the imported caller's site");
        assert_eq!(
            imported_site.owner_package_name(),
            &owner_name(&library).expect("IR library identity")
        );
        assert!(matches!(
            &imported_site.location().origin,
            quire_semantic_value::location::Origin::Body { function, .. } if function == "outer"
        ));
    }

    #[trace("TC-031", "FR-021-AC-17", "FR-021-AC-35", "FR-021-AC-36")]
    #[test]
    fn own_and_imported_callers_charge_their_checked_nested_site() {
        let (unit, library, _, dependencies, source_bytes) = imported();
        for (caller_package, caller_name, callee_name) in [
            (&unit, "caller", "imported"),
            (&library, "outer", "imported"),
        ] {
            let function = |package: &CheckedPackageV2, name: &str| {
                package
                    .resolved_graph()
                    .nodes
                    .iter()
                    .find(|node| {
                        node.declaration.as_ref().is_some_and(|declaration| {
                            declaration.qualified_name == [Box::<str>::from(name)]
                        })
                    })
                    .expect("admitted callable")
                    .node_id
                    .clone()
            };
            let caller = function(caller_package, caller_name);
            let callee = function(&library, callee_name);
            let call = caller_package
                .resolved_graph()
                .nodes
                .iter()
                .find(|node| {
                    node.node_tag.as_ref() == "expression" && node.semantic_form.as_ref() == "call"
                })
                .expect("admitted nested call")
                .node_id
                .clone();
            let source = || CheckedFunctionSource {
                source: identity(),
                path: "function-importer.native",
                bytes: source_bytes.as_bytes(),
                packages: &[],
                dependencies: &dependencies,
                limits: StageLimits::default(),
                replay_limits: ReplayLimits::default(),
                body_resources: CheckedFunctionResources::default(),
            };
            let argument = || vec![Value::Integer(Integer::from(6_u64))];

            let mut baseline_meter = meter();
            apply_checked_function(
                source(),
                &unit,
                caller_package,
                &caller,
                argument(),
                ObjectClosure::default(),
                &mut baseline_meter,
                |_, _, _| Ok(EchoFactory),
            )
            .expect("root selection and registration")
            .expect("root completes without a child");
            let root_work = baseline_meter.consumed(LimitKind::WorkUnits);
            assert!(root_work > 0, "the root call must charge before its body");

            let mut full_meter = meter();
            let completed = apply_checked_function(
                source(),
                &unit,
                caller_package,
                &caller,
                argument(),
                ObjectClosure::default(),
                &mut full_meter,
                |handle, caller_selection, _| {
                    let callee_selection =
                        select_checked_callable(&unit, &library, &callee, handle)?;
                    let site =
                        selected_nested_call_site(caller_package, &call, handle, caller_selection)?;
                    Ok(SitedCallFactory {
                        callee: callee_selection.key().clone(),
                        site: site.location().clone(),
                    })
                },
            )
            .expect("QSL selects both callables and the checked child site")
            .expect("the leaf applies the interpreted child");
            assert!(matches!(
                completed.outcome,
                FamilyOutcome::Evaluated(Outcome::Completed(Value::Integer(ref number)))
                    if *number == Integer::from(6_u64)
            ));
            assert!(full_meter.admission_count() >= 2, "root and child charges");
            assert!(
                full_meter.consumed(LimitKind::WorkUnits) > root_work,
                "the nested call follows the admitted root charge"
            );

            let mut limited_meter = meter_with_work(root_work);
            let mut issued_site = None;
            let denied = apply_checked_function(
                source(),
                &unit,
                caller_package,
                &caller,
                argument(),
                ObjectClosure::default(),
                &mut limited_meter,
                |handle, caller_selection, _| {
                    let callee_selection =
                        select_checked_callable(&unit, &library, &callee, handle)?;
                    let site =
                        selected_nested_call_site(caller_package, &call, handle, caller_selection)?;
                    issued_site = Some(site.location().clone());
                    Ok(SitedCallFactory {
                        callee: callee_selection.key().clone(),
                        site: site.location().clone(),
                    })
                },
            )
            .expect("the same checked child site is issued before Begin")
            .expect("denied child charge is a located evaluation");
            assert!(matches!(
                denied.outcome,
                FamilyOutcome::Evaluated(Outcome::Incomplete(_))
            ));
            assert_eq!(denied.location, issued_site);
            assert_eq!(limited_meter.consumed(LimitKind::WorkUnits), root_work);
        }
    }

    #[trace("TC-031", "FR-021-AC-35")]
    #[test]
    fn malformed_source_stops_at_acquisition() {
        let (package, f, _) = admitted();
        let dependencies = DependencyInput::default();
        let mut meter = meter();
        let refusal = apply_checked_function(
            CheckedFunctionSource {
                source: identity(),
                path: "function-owner.native",
                bytes: b"not a QSL unit",
                packages: &[],
                dependencies: &dependencies,
                limits: StageLimits::default(),
                replay_limits: ReplayLimits::default(),
                body_resources: CheckedFunctionResources::default(),
            },
            &package,
            &package,
            &f,
            vec![Value::Integer(Integer::from(4_u64))],
            ObjectClosure::default(),
            &mut meter,
            |_, _, _| Ok(EchoFactory),
        )
        .expect_err("malformed source must issue no checked handle");
        assert!(matches!(refusal, CheckedFunctionError::Acquisition(_)));
    }

    #[trace("TC-031", "FR-021-AC-37")]
    #[test]
    fn genuine_other_unit_identity_stops_before_lookup_or_registration() {
        let (package, f, _) = admitted();
        let other = separately_admitted_unit();
        assert_eq!(package.name(), other.name());
        assert_ne!(
            package_bytes(&package).unwrap(),
            package_bytes(&other).unwrap()
        );
        let dependencies = DependencyInput::default();
        let mut meter = meter();
        let refusal = apply_checked_function(
            CheckedFunctionSource {
                source: identity(),
                path: "function-owner.native",
                bytes: SOURCE.as_bytes(),
                packages: &[],
                dependencies: &dependencies,
                limits: StageLimits::default(),
                replay_limits: ReplayLimits::default(),
                body_resources: CheckedFunctionResources::default(),
            },
            &other,
            &package,
            &f,
            vec![Value::Integer(Integer::from(4_u64))],
            ObjectClosure::default(),
            &mut meter,
            |_, _, _| Ok(EchoFactory),
        )
        .expect_err("another admitted unit cannot authorize this source");
        assert!(matches!(
            refusal,
            CheckedFunctionError::OwnerIdentityMismatch
        ));
    }

    #[trace("TC-031", "FR-021-AC-35", "FR-021-AC-37")]
    #[test]
    fn unrelated_declaring_package_cannot_authorize_a_callable() {
        let (unit, f, _) = admitted();
        let unrelated = separately_admitted_unit();
        assert_eq!(unit.name(), unrelated.name());
        assert_ne!(
            package_bytes(&unit).unwrap(),
            package_bytes(&unrelated).unwrap()
        );
        let acquisition = acquire_checked_closure(
            CompileUnit {
                source: identity(),
                package_name: "function-owner".to_owned(),
                path: "function-owner.native",
                bytes: SOURCE.as_bytes(),
            },
            [],
            &DependencyInput::default(),
            StageLimits::default(),
            ReplayLimits::default(),
        )
        .expect("genuine checked source");
        assert!(matches!(
            select_checked_callable(&unit, &unrelated, &f, &acquisition),
            Err(CheckedFunctionError::OwnerIdentityMismatch)
        ));
    }

    #[trace("TC-031", "FR-021-AC-37")]
    #[test]
    fn post_lookup_guard_refuses_a_genuine_second_selection() {
        let (package, f, g) = admitted();
        let acquisition = acquire_checked_closure(
            CompileUnit {
                source: identity(),
                package_name: "function-owner".to_owned(),
                path: "function-owner.native",
                bytes: SOURCE.as_bytes(),
            },
            [],
            &DependencyInput::default(),
            StageLimits::default(),
            ReplayLimits::default(),
        )
        .expect("same source");
        let owner = owner_name(&package).expect("IR package id");
        let second = acquisition
            .handle()
            .lookup_callable(
                &owner,
                &checked_function_key(&package, &g).expect("IR node id"),
            )
            .expect("genuine second callable");
        assert!(matches!(
            validate_selected_callable(
                &owner,
                &checked_function_key(&package, &f).expect("IR node id"),
                &second,
            ),
            Err(CheckedFunctionError::OwnerIdentityMismatch)
        ));
    }

    #[trace("TC-031", "FR-021-AC-37")]
    #[test]
    fn typed_facade_outcomes_keep_distinct_serializable_cause_codes() {
        let (package, _, _) = admitted();
        let owner = owner_name(&package).expect("IR package id");
        let cases = [
            map_lookup_refusal(CallableLookupRefusal::Unknown),
            map_lookup_refusal(CallableLookupRefusal::Foreign(owner.clone())),
            map_lookup_refusal(CallableLookupRefusal::WrongKind),
            map_selection_refusal(SelectionRefusal::WrongAcquisition),
            map_begin_refusal(ApplicationBeginRefusal::WrongAcquisition),
        ];
        let encoded = cases
            .iter()
            .map(|refusal| serde_json::to_value(refusal).expect("serializable claim refusal"))
            .collect::<Vec<_>>();
        let pairs = encoded
            .iter()
            .map(|record| {
                (
                    record["code"].as_str().expect("outer code"),
                    record["cause_code"].as_str().expect("cause code"),
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(
            pairs,
            [
                ("facade_lookup", "lookup.unknown"),
                ("facade_lookup", "lookup.foreign"),
                ("facade_lookup", "lookup.wrong_kind"),
                ("selection_refused", "selection.wrong_acquisition"),
                ("application_begin_refused", "begin.wrong_acquisition"),
            ]
        );
        assert!(encoded[1]["detail"]
            .as_str()
            .is_some_and(|s| s.contains(owner.as_str())));
        assert!(map_registration(Admission::Accepted).is_none());
        for (admission, code) in [
            (Admission::RegistrySealed, "registration.registry_sealed"),
            (
                Admission::CallableNotAdmitted,
                "registration.callable_not_admitted",
            ),
            (
                Admission::SignatureMismatch,
                "registration.signature_mismatch",
            ),
            (
                Admission::DuplicateBinding,
                "registration.duplicate_binding",
            ),
        ] {
            let refusal = map_registration(admission).expect("non-accepted admission");
            let record = serde_json::to_value(refusal).expect("serializable claim refusal");
            assert_eq!(record["code"], "registration_not_accepted");
            assert_eq!(record["cause_code"], code);
        }
    }
}
