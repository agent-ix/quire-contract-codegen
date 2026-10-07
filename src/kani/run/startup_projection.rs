//! C-side projection of already authenticated, finite policy refusal provenance (FR-034 AC-39).
//!
//! Authentication, original context storage, stop timing and whole-chain settlement belong to the
//! caller owner. This module selects no capability from errno, ErrorKind or diagnostic text. OS
//! errors remain direct so their public raw errno survives. Transport cannot reconstruct arbitrary
//! custom error payloads/downcast chains; the exact finite policy origin remains separate custody.

use std::io;

use super::{
    backend_policy::BackendPolicyError,
    cross_role_cause::{
        CauseCheckerProvenance, CauseCheckerRole, CauseIntegrityPredicate, CauseIntegritySource,
        CauseOperation, KaniCauseMetadataIntegrityError, KaniCrossRoleCauseLoss,
        RemoteCauseProvenance, RemoteCauseRole,
    },
    execute::KaniStartupCapability,
    startup_cause::{
        ProjectedStartupCause, RepresentationError, StartupCause, StartupIoCause,
        StartupSeccompilerCause,
    },
    startup_envelope::PolicyFailureCause,
};

/// Provisional refusal data, not authority to classify before actual owned settlement.
#[derive(Debug)]
pub(super) struct PolicyStartupRefusal {
    pub(super) capability: KaniStartupCapability,
    pub(super) cause: io::Error,
    /// Exact authenticated finite metadata, retained independently of the public OS error.
    pub(super) origin: PolicyFailureCause,
}

/// Finite CG-owned cause value/payload bound charged before L. The projection produces one
/// cause at a time. Opaque std io::Error/allocator internals remain outside named C terms; this
/// accounts the known CG/dependency source payload, independently of that opaque machinery.
pub(super) fn metadata_reservation() -> Result<u64, super::control::ControlError> {
    let payload = std::mem::size_of::<KaniCrossRoleCauseLoss>()
        .max(std::mem::size_of::<KaniCauseMetadataIntegrityError>())
        .max(std::mem::size_of::<BackendPolicyError>());
    #[cfg(any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "riscv64"
    ))]
    let payload = payload.max(std::mem::size_of::<seccompiler::Error>());
    let bytes = std::mem::size_of::<PolicyStartupRefusal>()
        .checked_add(payload)
        .ok_or(super::control::ControlError::EncodedBytesExceeded)?;
    u64::try_from(bytes).map_err(|_| super::control::ControlError::EncodedBytesExceeded)
}

/// Project only actual policy-site variants. The caller retains the separately bounded original
/// context and authenticates the pre-recipe producer before invoking this function. Representation
/// failure is not a substitute unavailable policy cause, Ready event or cleanup confirmation.
pub(super) fn project_policy(
    failure: PolicyFailureCause,
) -> Result<PolicyStartupRefusal, RepresentationError> {
    let result = project_policy_value(failure);
    match result {
        Ok(projected) => Ok(projected),
        Err(error) => {
            // These are actual C checking predicates, never an inferred sender fault.
            // Unrelated representation/preparation errors retain their original typed path.
            let predicate = match &error {
                RepresentationError::OsKindMismatch => {
                    CauseIntegrityPredicate::OriginalOsKindMismatch
                }
                RepresentationError::PolicyCauseMismatch => {
                    CauseIntegrityPredicate::PolicySiteCauseMismatch
                }
                RepresentationError::MissingOsCode | RepresentationError::OsPayloadMismatch => {
                    CauseIntegrityPredicate::MalformedCauseMetadata
                }
                RepresentationError::InvalidContextBound
                | RepresentationError::Reservation(_)
                | RepresentationError::ContextExceeded
                | RepresentationError::Formatting
                | RepresentationError::UnnamedIoKind
                | RepresentationError::NonInstallationBackendCause => return Err(error),
            };
            let (capability, _originating_operation) = policy_site(failure);
            let integrity = KaniCauseMetadataIntegrityError::new(
                predicate,
                CauseCheckerProvenance {
                    role: Some(CauseCheckerRole::Caller),
                    operation: Some(CauseOperation::ControlReception),
                },
                Some(CauseIntegritySource::Representation(error)),
            );
            Ok(PolicyStartupRefusal {
                capability,
                cause: io::Error::new(io::ErrorKind::InvalidData, integrity),
                origin: failure,
            })
        }
    }
}

// Call only with independently authenticated actual policy-site custody. This is not a parser
// or authority factory: a caller cannot use an unverified packet label to select an admission.
fn policy_site(failure: PolicyFailureCause) -> (KaniStartupCapability, CauseOperation) {
    use KaniStartupCapability::{BackendIpcExclusion, TrustedOwnerProtection};
    match failure {
        PolicyFailureCause::UnsupportedArchitecture
        | PolicyFailureCause::InvalidProgram
        | PolicyFailureCause::NotBackend
        | PolicyFailureCause::Preparation { .. } => {
            (BackendIpcExclusion, CauseOperation::NativePolicyPreparation)
        }
        PolicyFailureCause::ProtectionUnverified
        | PolicyFailureCause::Privilege { .. }
        | PolicyFailureCause::Filter {
            cause: StartupCause::Seccompiler(StartupSeccompilerCause::Prctl(_)),
        } => (TrustedOwnerProtection, CauseOperation::OwnerProtection),
        PolicyFailureCause::Filter {
            cause: StartupCause::Io(_),
        }
        | PolicyFailureCause::Filter {
            cause:
                StartupCause::Seccompiler(
                    StartupSeccompilerCause::Seccomp(_)
                    | StartupSeccompilerCause::EmptyFilter
                    | StartupSeccompilerCause::ThreadSync { .. },
                ),
        } => (
            BackendIpcExclusion,
            CauseOperation::NativePolicyInstallation,
        ),
    }
}

fn project_policy_value(
    failure: PolicyFailureCause,
) -> Result<PolicyStartupRefusal, RepresentationError> {
    let (capability, operation) = policy_site(failure);
    let cause = match failure {
        PolicyFailureCause::UnsupportedArchitecture => io::Error::new(
            io::ErrorKind::Unsupported,
            BackendPolicyError::UnsupportedArchitecture,
        ),
        PolicyFailureCause::InvalidProgram => io::Error::new(
            io::ErrorKind::Unsupported,
            BackendPolicyError::InvalidProgram,
        ),
        PolicyFailureCause::NotBackend => {
            io::Error::new(io::ErrorKind::Unsupported, BackendPolicyError::NotBackend)
        }
        PolicyFailureCause::ProtectionUnverified => io::Error::new(
            io::ErrorKind::Unsupported,
            BackendPolicyError::ProtectionUnverified,
        ),
        PolicyFailureCause::Preparation {
            cause: StartupCause::Io(cause),
        }
        | PolicyFailureCause::Privilege {
            cause: StartupCause::Io(cause),
        } => project_io(cause, operation)?,
        PolicyFailureCause::Filter {
            cause: StartupCause::Seccompiler(cause),
        } => match cause {
            // Return the nested actual OS error directly; finite non-I/O dependency variants
            // retain their real public typed source instead of a generic custom-loss marker.
            StartupSeccompilerCause::Prctl(cause) | StartupSeccompilerCause::Seccomp(cause) => {
                project_io(cause, operation)?
            }
            StartupSeccompilerCause::EmptyFilter => {
                io::Error::other(seccompiler::Error::EmptyFilter)
            }
            StartupSeccompilerCause::ThreadSync { pid } => {
                io::Error::other(seccompiler::Error::ThreadSync(pid))
            }
        },
        PolicyFailureCause::Preparation {
            cause: StartupCause::Seccompiler(_),
        }
        | PolicyFailureCause::Privilege {
            cause: StartupCause::Seccompiler(_),
        }
        | PolicyFailureCause::Filter {
            cause: StartupCause::Io(_),
        } => return Err(RepresentationError::PolicyCauseMismatch),
    };
    Ok(PolicyStartupRefusal {
        capability,
        cause,
        origin: failure,
    })
}

fn project_io(
    cause: StartupIoCause,
    operation: CauseOperation,
) -> Result<io::Error, RepresentationError> {
    match StartupCause::Io(cause).project(RemoteCauseProvenance {
        role: RemoteCauseRole::BackendInstaller,
        operation,
    })? {
        // Payload-free causes remain payload-free; actual custom loss is explicit. Capability
        // and finite origin come from the authenticated PolicyFailureCause, not ErrorKind.
        ProjectedStartupCause::Io { error, .. } => Ok(error),
        ProjectedStartupCause::Seccompiler { .. } => Err(RepresentationError::PolicyCauseMismatch),
    }
}

#[cfg(test)]
mod tests {
    use super::super::{startup_cause::PreparedStartupContext, startup_envelope::CONTEXT_BYTES};
    use super::*;

    /// Trace: FR-034-AC-39
    /// Actual primitive checking failures only, not sender/authentication or public-launch proof.
    #[test]
    fn projection_integrity_preserves_the_authenticated_site_and_actual_local_source() {
        let mut context = PreparedStartupContext::new(CONTEXT_BYTES).unwrap();
        let cause = context
            .capture_io(&io::Error::from_raw_os_error(nix::libc::EPERM))
            .unwrap();
        let original = serde_json::to_value(cause).unwrap();
        for (field, malformed) in [
            ("kind", serde_json::json!("Other")),
            ("payload", serde_json::json!("UnrepresentedCustom")),
        ] {
            let mut metadata = original.clone();
            metadata["Io"][field] = malformed;
            let cause = serde_json::from_value(metadata).unwrap();
            let refusal = project_policy(PolicyFailureCause::Preparation { cause }).unwrap();
            assert_eq!(
                refusal.capability,
                KaniStartupCapability::BackendIpcExclusion
            );
            assert_eq!(refusal.cause.kind(), io::ErrorKind::InvalidData);
            assert_eq!(refusal.cause.raw_os_error(), None);
            let source = refusal.cause.get_ref().unwrap();
            let integrity = source
                .downcast_ref::<KaniCauseMetadataIntegrityError>()
                .unwrap();
            assert!(source.downcast_ref::<KaniCrossRoleCauseLoss>().is_none());
            let local = std::error::Error::source(integrity).unwrap();
            let actual = local.downcast_ref::<RepresentationError>().unwrap();
            match field {
                "kind" => assert!(matches!(actual, RepresentationError::OsKindMismatch)),
                "payload" => assert!(matches!(actual, RepresentationError::OsPayloadMismatch)),
                _ => panic!("unlisted test mutation"),
            }
        }
        let mismatch = project_policy(PolicyFailureCause::Filter { cause }).unwrap();
        assert_eq!(
            mismatch.capability,
            KaniStartupCapability::BackendIpcExclusion
        );
        let integrity = mismatch
            .cause
            .get_ref()
            .unwrap()
            .downcast_ref::<KaniCauseMetadataIntegrityError>()
            .unwrap();
        assert!(matches!(
            std::error::Error::source(integrity)
                .unwrap()
                .downcast_ref::<RepresentationError>()
                .unwrap(),
            RepresentationError::PolicyCauseMismatch
        ));
    }

    /// Trace: FR-034-AC-39
    /// Primitive projection only; no helper transport, actual admission or AC40 completion credit.
    #[test]
    fn preparation_projection_distinguishes_actual_custom_loss_from_payload_free_cause() {
        let mut context = PreparedStartupContext::new(CONTEXT_BYTES).unwrap();
        let free = io::Error::from(io::ErrorKind::InvalidData);
        let custom = io::Error::new(io::ErrorKind::InvalidData, "actual custom diagnostic");
        for (original, expected_loss) in [(&free, false), (&custom, true)] {
            let failure = PolicyFailureCause::Preparation {
                cause: context.capture_io(original).unwrap(),
            };
            let refusal = project_policy(failure).unwrap();
            assert_eq!(
                refusal.capability,
                KaniStartupCapability::BackendIpcExclusion
            );
            assert_eq!(refusal.origin, failure);
            assert_eq!(refusal.cause.kind(), original.kind());
            assert_eq!(refusal.cause.raw_os_error(), None);
            let marker = refusal
                .cause
                .get_ref()
                .and_then(|source| source.downcast_ref::<KaniCrossRoleCauseLoss>());
            assert_eq!(marker.is_some(), expected_loss);
            if let Some(marker) = marker {
                assert!(std::error::Error::source(marker).is_none());
            }
            assert!(refusal
                .cause
                .get_ref()
                .and_then(|source| source.downcast_ref::<KaniCauseMetadataIntegrityError>())
                .is_none());
        }
    }

    /// Trace: FR-034-AC-39
    /// Projection-only control; this does not execute or prove policy installation failure.
    #[test]
    fn prctl_projection_preserves_direct_errno_and_selects_owner_protection() {
        let original = io::Error::from(rustix::io::Errno::PERM);
        let mut context = PreparedStartupContext::new(CONTEXT_BYTES).unwrap();
        let StartupCause::Io(cause) = context.capture_io(&original).unwrap() else {
            panic!("I/O capture changed cause family");
        };
        let refusal = project_policy(PolicyFailureCause::Filter {
            cause: StartupCause::Seccompiler(StartupSeccompilerCause::Prctl(cause)),
        })
        .unwrap();
        assert_eq!(
            refusal.capability,
            KaniStartupCapability::TrustedOwnerProtection
        );
        assert_eq!(refusal.cause.raw_os_error(), original.raw_os_error());
        assert_eq!(refusal.cause.kind(), original.kind());
        assert!(matches!(
            refusal.origin,
            PolicyFailureCause::Filter {
                cause: StartupCause::Seccompiler(StartupSeccompilerCause::Prctl(_)),
            }
        ));
    }

    /// Trace: FR-034-AC-39
    /// Finite-source projection only; no claim that the actual flags-zero installer emits TSYNC.
    #[test]
    fn thread_sync_projection_retains_the_dependency_pid_without_inventing_errno() {
        let pid = 37;
        let refusal = project_policy(PolicyFailureCause::Filter {
            cause: StartupCause::Seccompiler(StartupSeccompilerCause::ThreadSync { pid }),
        })
        .unwrap();
        assert_eq!(
            refusal.capability,
            KaniStartupCapability::BackendIpcExclusion
        );
        assert_eq!(refusal.cause.raw_os_error(), None);
        let source = refusal
            .cause
            .get_ref()
            .unwrap()
            .downcast_ref::<seccompiler::Error>()
            .unwrap();
        assert!(matches!(source, seccompiler::Error::ThreadSync(actual) if *actual == pid));
    }
}
