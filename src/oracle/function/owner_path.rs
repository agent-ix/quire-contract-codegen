// SPDX-License-Identifier: AGPL-3.0-or-later
//! FR-021: apply a generated body through QSL's retained checked closure.

use qsl_replay::{
    acquire_checked_closure, AdmittedRefusal, ApplicationBeginRefusal, ApplicationEnvironment,
    ByteDigest, CallableLookupRefusal, CallableSelection, CheckedClosureHandle, DependencyInput,
    DigestDomain, DigestRecord, NestedCallOccurrence, NestedCallSite, NestedCallSiteRefusal,
    ReplayLimits, SelectionRefusal, SourceIdentity, StageLimits, WireNodeId,
};
use quire_contract_model::{
    CheckedNodeId, CheckedOccurrenceRole, CheckedPackageV2, CheckedSemanticId,
};
use quire_eval::{Admission, BodyFactory, CallFailure, Evaluation};
use quire_exact::{Meter, Value};
use quire_semantic_value::object_closure::ObjectClosure;

use super::ExactFunctionRefusal;

/// The seven caller-owned inputs to one checked-closure acquisition.
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
}

/// A failure before the checked application can run.
#[derive(Debug)]
pub enum CheckedFunctionError {
    /// QSL did not admit the supplied closure; no handle was issued.
    Acquisition(AdmittedRefusal),
    /// An independently admitted IR identity did not have its required domain or digest shape.
    InvalidIdentity,
    /// The acquired unit or checker-issued selection differs from the IR-admitted identities.
    OwnerIdentityMismatch,
    /// QSL did not issue a callable selection.
    FacadeLookup(CallableLookupRefusal),
    /// The selection belongs to a different acquisition.
    SelectionRefused(SelectionRefusal),
    /// The declaring package has no expression occurrence for the requested call node.
    MissingCallOccurrence,
    /// More than one source occurrence of this call belongs to the selected body.
    AmbiguousCallOccurrence,
    /// QSL did not issue a nested site inside the selected checked body.
    NestedCallSiteRefused(NestedCallSiteRefusal),
    /// The leaf did not accept the generated body registration.
    RegistrationNotAccepted(Admission),
    /// Begin refused before body invocation or application charge.
    ApplicationBeginRefused(ApplicationBeginRefusal),
}

impl CheckedFunctionError {
    /// Convert a post-acquisition failure into the claim-map's stable serializable refusal.
    /// Acquisition and malformed admitted IR identities retain their own typed boundaries.
    pub fn into_claim_refusal(self) -> Result<ExactFunctionRefusal, Self> {
        match self {
            Self::OwnerIdentityMismatch => Ok(ExactFunctionRefusal::OwnerIdentityMismatch),
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
            Self::Acquisition(_) | Self::InvalidIdentity => Err(self),
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

/// Decode an IR-admitted package identity in the QSL facade's exact package domain.
pub fn checked_package_digest(
    id: &CheckedSemanticId,
) -> Result<DigestRecord, CheckedFunctionError> {
    if id.domain.as_ref() != "quire.package.semantic/v2" || id.algorithm.as_ref() != "sha256" {
        return Err(CheckedFunctionError::InvalidIdentity);
    }
    let bytes = ByteDigest::from_hex(&id.digest)
        .map_err(|_| CheckedFunctionError::InvalidIdentity)?
        .as_bytes();
    Ok(DigestRecord::mint(DigestDomain::PackageSemanticV2, bytes))
}

/// Decode an IR-admitted function node identity for QSL callable selection.
pub fn checked_function_wire_id(id: &CheckedNodeId) -> Result<WireNodeId, CheckedFunctionError> {
    if id.domain.as_ref() != "quire.checked-semantic-node/v1" {
        return Err(CheckedFunctionError::InvalidIdentity);
    }
    WireNodeId::from_hex(&id.digest).ok_or(CheckedFunctionError::InvalidIdentity)
}

fn validate_selected_callable(
    expected_unit: DigestRecord,
    expected_owner: DigestRecord,
    expected_function: WireNodeId,
    handle: &CheckedClosureHandle,
    selection: &CallableSelection,
) -> Result<(), CheckedFunctionError> {
    if handle.package_id() != expected_unit
        || selection.owner_package_id() != expected_owner
        || selection.key().as_bytes() != expected_function.as_bytes()
    {
        return Err(CheckedFunctionError::OwnerIdentityMismatch);
    }
    Ok(())
}

/// Select one callable from a separately admitted unit or locked dependency package.
/// The facade remains the authority for the callable key and signature.
pub fn select_checked_callable(
    admitted_unit: &CheckedPackageV2,
    declaring_package: &CheckedPackageV2,
    function: &CheckedNodeId,
    handle: &CheckedClosureHandle,
) -> Result<CallableSelection, CheckedFunctionError> {
    let expected_unit = checked_package_digest(admitted_unit.package_id())?;
    let expected_owner = checked_package_digest(declaring_package.package_id())?;
    let expected_function = checked_function_wire_id(function)?;
    if handle.package_id() != expected_unit
        || (admitted_unit.package_id() != declaring_package.package_id()
            && !admitted_unit
                .lock()
                .dependency_selections
                .iter()
                .any(|selection| selection.package_id == *declaring_package.package_id()))
    {
        return Err(CheckedFunctionError::OwnerIdentityMismatch);
    }
    let selection = handle
        .lookup_callable(expected_owner, expected_function)
        .map_err(CheckedFunctionError::FacadeLookup)?;
    if !declaring_package
        .graph()
        .nodes
        .iter()
        .any(|node| node.node_id == *function && node.node_tag.as_ref() == "function")
    {
        return Err(CheckedFunctionError::OwnerIdentityMismatch);
    }
    validate_selected_callable(
        expected_unit,
        expected_owner,
        expected_function,
        handle,
        &selection,
    )?;
    Ok(selection)
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
    handle: &CheckedClosureHandle,
    caller: &CallableSelection,
) -> Result<NestedCallSite, CheckedFunctionError> {
    if caller.owner_package_id() != checked_package_digest(declaring_package.package_id())? {
        return Err(CheckedFunctionError::OwnerIdentityMismatch);
    }
    let call_wire = checked_function_wire_id(call)?;
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
            NestedCallOccurrence::new(call_wire, occurrence.ordinal),
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
    Build:
        FnOnce(&CheckedClosureHandle, &CallableSelection) -> Result<Factory, CheckedFunctionError>,
{
    let acquired = acquire_checked_closure(
        source.source,
        source.path,
        source.bytes,
        source.packages.iter().copied(),
        source.dependencies,
        source.limits,
        source.replay_limits,
    )
    .map_err(CheckedFunctionError::Acquisition)?;
    let handle = acquired.handle();
    let selection = select_checked_callable(admitted_unit, declaring_package, function, handle)?;
    let factory = build_factory(handle, &selection)?;
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
        SourceIdentity::new("agent-ix", "function-owner", "git", "1")
    }

    fn admitted() -> (CheckedPackageV2, CheckedNodeId, CheckedNodeId) {
        let compiled = acquire_checked_closure(
            identity(),
            "function-owner.native",
            SOURCE.as_bytes(),
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
                .graph()
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
            identity(),
            "function-owner.native",
            OTHER_SOURCE.as_bytes(),
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
        CheckedPackageV2,
        CheckedNodeId,
        DependencyInput,
        String,
    ) {
        let library_source = SourceIdentity::new("agent-ix", "function-library", "git", "1");
        let library_compiled = acquire_checked_closure(
            library_source.clone(),
            "function-library.native",
            LIBRARY_SOURCE.as_bytes(),
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
        let library = *library;
        let function = library
            .graph()
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
            identity(),
            "function-importer.native",
            unit_source.as_bytes(),
            [],
            &dependencies,
            StageLimits::default(),
            ReplayLimits::default(),
        )
        .expect("unit imports the supplied library");
        let mut unit_evidence = CheckedPackageEvidence::new();
        unit_evidence.support_feature("quire.value.complete/v1");
        unit_evidence
            .insert_dependency_package("test/function-library", Arc::new(library.clone()))
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
        ) -> Result<Transition, InternalFault> {
            match input {
                Resume::Start(mut arguments) if arguments.len() == 1 => {
                    Ok(Transition::Done(Evaluation::completed(arguments.remove(0))))
                }
                _ => Err(InternalFault::new(
                    "generated_function",
                    "unexpected_resume",
                )),
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
                callee: self.callee,
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
        ) -> Result<Transition, InternalFault> {
            match input {
                Resume::Start(arguments) => Ok(Transition::Call(ChildRequest {
                    callee: self.callee,
                    arguments,
                    site: self.site.clone(),
                })),
                Resume::Child(result) => Ok(Transition::Done(result)),
                Resume::Again => Err(InternalFault::new(
                    "generated_function",
                    "unexpected_resume",
                )),
            }
        }
    }

    /// Trace: FR-021-AC-35, FR-021-AC-36, TC-031.
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
            },
            &package,
            &package,
            &f,
            vec![value],
            ObjectClosure::default(),
            &mut meter,
            |_, _| Ok(EchoFactory),
        )
        .expect("same source and admitted identities")
        .expect("leaf terminal result");
        assert!(matches!(
            result.outcome,
            FamilyOutcome::Evaluated(Outcome::Completed(Value::Integer(ref number)))
                if *number == Integer::from(4_u64)
        ));
    }

    /// Trace: FR-021-AC-35, FR-021-AC-36, TC-031.
    #[test]
    fn checked_owner_path_applies_a_locked_imported_callable() {
        let (unit, library, function, dependencies, source_bytes) = imported();
        assert_ne!(unit.package_id(), library.package_id());
        assert!(unit
            .lock()
            .dependency_selections
            .iter()
            .any(|selection| selection.package_id == *library.package_id()));
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
            },
            &unit,
            &library,
            &function,
            vec![Value::Integer(Integer::from(6_u64))],
            ObjectClosure::default(),
            &mut meter,
            |handle, selection| {
                assert_eq!(
                    selection.owner_package_id(),
                    checked_package_digest(library.package_id()).expect("IR library identity")
                );
                assert_eq!(
                    selection.key().as_bytes(),
                    checked_function_wire_id(&function)
                        .expect("IR declaration identity")
                        .as_bytes()
                );
                assert_eq!(
                    handle.package_id(),
                    checked_package_digest(unit.package_id()).expect("IR unit identity")
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

    /// Trace: FR-021-AC-17, FR-021-AC-36, TC-031.
    #[test]
    fn nested_call_site_comes_from_the_selected_checked_body() {
        let (unit, library, imported_function, dependencies, source_bytes) = imported();
        let acquired = acquire_checked_closure(
            identity(),
            "function-importer.native",
            source_bytes.as_bytes(),
            [],
            &dependencies,
            StageLimits::default(),
            ReplayLimits::default(),
        )
        .expect("QSL retains the imported closure");
        let handle = acquired.handle();
        let caller = unit
            .graph()
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
            .graph()
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
        assert_eq!(
            site.owner_package_id(),
            checked_package_digest(unit.package_id()).unwrap()
        );
        let issued = handle
            .nested_call_site(
                &caller_selection,
                NestedCallOccurrence::new(
                    checked_function_wire_id(&call).expect("admitted call identity"),
                    occurrence.ordinal,
                ),
            )
            .expect("QSL issues the IR-selected occurrence");
        assert_eq!(site.location(), issued.location());
        assert!(matches!(
            &site.location().origin,
            quire_semantic_value::location::Origin::Body { function, .. } if function == "caller"
        ));

        let wrong_ordinal = handle.nested_call_site(
            &caller_selection,
            NestedCallOccurrence::new(
                checked_function_wire_id(&call).expect("admitted call identity"),
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
            .graph()
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
            .graph()
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
            imported_site.owner_package_id(),
            checked_package_digest(library.package_id()).expect("IR library identity")
        );
        assert!(matches!(
            &imported_site.location().origin,
            quire_semantic_value::location::Origin::Body { function, .. } if function == "outer"
        ));
    }

    /// Trace: FR-021-AC-17, FR-021-AC-35, FR-021-AC-36, TC-031.
    #[test]
    fn own_and_imported_callers_charge_their_checked_nested_site() {
        let (unit, library, _, dependencies, source_bytes) = imported();
        for (caller_package, caller_name, callee_name) in [
            (&unit, "caller", "imported"),
            (&library, "outer", "imported"),
        ] {
            let function = |package: &CheckedPackageV2, name: &str| {
                package
                    .graph()
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
                .graph()
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
                |_, _| Ok(EchoFactory),
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
                |handle, caller_selection| {
                    let callee_selection =
                        select_checked_callable(&unit, &library, &callee, handle)?;
                    let site =
                        selected_nested_call_site(caller_package, &call, handle, caller_selection)?;
                    Ok(SitedCallFactory {
                        callee: callee_selection.key(),
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
                |handle, caller_selection| {
                    let callee_selection =
                        select_checked_callable(&unit, &library, &callee, handle)?;
                    let site =
                        selected_nested_call_site(caller_package, &call, handle, caller_selection)?;
                    issued_site = Some(site.location().clone());
                    Ok(SitedCallFactory {
                        callee: callee_selection.key(),
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

    /// Trace: FR-021-AC-35, TC-031.
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
            },
            &package,
            &package,
            &f,
            vec![Value::Integer(Integer::from(4_u64))],
            ObjectClosure::default(),
            &mut meter,
            |_, _| Ok(EchoFactory),
        )
        .expect_err("malformed source must issue no checked handle");
        assert!(matches!(refusal, CheckedFunctionError::Acquisition(_)));
    }

    /// Trace: FR-021-AC-37, TC-031.
    #[test]
    fn genuine_other_unit_identity_stops_before_lookup_or_registration() {
        let (package, f, _) = admitted();
        let other = separately_admitted_unit();
        assert_ne!(package.package_id(), other.package_id());
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
            },
            &other,
            &package,
            &f,
            vec![Value::Integer(Integer::from(4_u64))],
            ObjectClosure::default(),
            &mut meter,
            |_, _| Ok(EchoFactory),
        )
        .expect_err("another admitted unit cannot authorize this source");
        assert!(matches!(
            refusal,
            CheckedFunctionError::OwnerIdentityMismatch
        ));
    }

    /// Trace: FR-021-AC-35, FR-021-AC-37, TC-031.
    #[test]
    fn unrelated_declaring_package_cannot_authorize_a_callable() {
        let (unit, f, _) = admitted();
        let unrelated = separately_admitted_unit();
        assert_ne!(unit.package_id(), unrelated.package_id());
        let acquisition = acquire_checked_closure(
            identity(),
            "function-owner.native",
            SOURCE.as_bytes(),
            [],
            &DependencyInput::default(),
            StageLimits::default(),
            ReplayLimits::default(),
        )
        .expect("genuine checked source");
        assert!(matches!(
            select_checked_callable(&unit, &unrelated, &f, acquisition.handle()),
            Err(CheckedFunctionError::OwnerIdentityMismatch)
        ));
    }

    /// Trace: FR-021-AC-37, TC-031.
    #[test]
    fn post_lookup_guard_refuses_a_genuine_second_selection() {
        let (package, f, g) = admitted();
        let acquisition = acquire_checked_closure(
            identity(),
            "function-owner.native",
            SOURCE.as_bytes(),
            [],
            &DependencyInput::default(),
            StageLimits::default(),
            ReplayLimits::default(),
        )
        .expect("same source");
        let owner = checked_package_digest(package.package_id()).expect("IR package id");
        let second = acquisition
            .handle()
            .lookup_callable(owner, checked_function_wire_id(&g).expect("IR node id"))
            .expect("genuine second callable");
        assert!(matches!(
            validate_selected_callable(
                owner,
                owner,
                checked_function_wire_id(&f).expect("IR node id"),
                acquisition.handle(),
                &second,
            ),
            Err(CheckedFunctionError::OwnerIdentityMismatch)
        ));
    }

    /// Trace: FR-021-AC-37, TC-031.
    #[test]
    fn typed_facade_outcomes_keep_distinct_serializable_cause_codes() {
        let (package, _, _) = admitted();
        let owner = checked_package_digest(package.package_id()).expect("IR package id");
        let cases = [
            map_lookup_refusal(CallableLookupRefusal::Unknown),
            map_lookup_refusal(CallableLookupRefusal::Foreign(owner)),
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
            .is_some_and(|s| s.contains(&owner.hex())));
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
