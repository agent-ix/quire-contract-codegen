//! C-side projection of already authenticated, finite policy refusal provenance (FR-034 AC-39).
//!
//! Authentication, original context storage, stop timing and whole-chain settlement belong to the
//! caller owner. This module selects no capability from errno, ErrorKind or diagnostic text. OS
//! errors remain direct so their public raw errno survives. Transport cannot reconstruct arbitrary
//! custom error payloads/downcast chains; the exact finite policy origin remains separate custody.

use std::io;

use super::{
    backend_policy::BackendPolicyError,
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

/// Project only actual policy-site variants. The caller retains the separately bounded original
/// context and authenticates the pre-recipe producer before invoking this function. Representation
/// failure is not a substitute unavailable policy cause, Ready event or cleanup confirmation.
pub(super) fn project_policy(
    failure: PolicyFailureCause,
) -> Result<PolicyStartupRefusal, RepresentationError> {
    use KaniStartupCapability::{BackendIpcExclusion, TrustedOwnerProtection};

    let (capability, cause) = match failure {
        PolicyFailureCause::UnsupportedArchitecture => (
            BackendIpcExclusion,
            io::Error::new(
                io::ErrorKind::Unsupported,
                BackendPolicyError::UnsupportedArchitecture,
            ),
        ),
        PolicyFailureCause::InvalidProgram => (
            BackendIpcExclusion,
            io::Error::new(
                io::ErrorKind::Unsupported,
                BackendPolicyError::InvalidProgram,
            ),
        ),
        PolicyFailureCause::NotBackend => (
            BackendIpcExclusion,
            io::Error::new(io::ErrorKind::Unsupported, BackendPolicyError::NotBackend),
        ),
        PolicyFailureCause::ProtectionUnverified => (
            TrustedOwnerProtection,
            io::Error::new(
                io::ErrorKind::Unsupported,
                BackendPolicyError::ProtectionUnverified,
            ),
        ),
        PolicyFailureCause::Preparation {
            cause: StartupCause::Io(cause),
        } => (BackendIpcExclusion, project_io(cause)?),
        PolicyFailureCause::Privilege {
            cause: StartupCause::Io(cause),
        } => (TrustedOwnerProtection, project_io(cause)?),
        PolicyFailureCause::Filter {
            cause: StartupCause::Seccompiler(cause),
        } => match cause {
            // apply_filter's PR_SET_NO_NEW_PRIVS is an owner-protection operation, despite
            // the outer Filter wrapper. Return the nested original OS error without wrapping.
            StartupSeccompilerCause::Prctl(cause) => (TrustedOwnerProtection, project_io(cause)?),
            StartupSeccompilerCause::Seccomp(cause) => (BackendIpcExclusion, project_io(cause)?),
            StartupSeccompilerCause::EmptyFilter => (
                BackendIpcExclusion,
                io::Error::other(seccompiler::Error::EmptyFilter),
            ),
            StartupSeccompilerCause::ThreadSync { pid } => (
                BackendIpcExclusion,
                io::Error::other(seccompiler::Error::ThreadSync(pid)),
            ),
        },
        PolicyFailureCause::Preparation {
            cause: StartupCause::Seccompiler(_),
        }
        | PolicyFailureCause::Privilege {
            cause: StartupCause::Seccompiler(_),
        }
        | PolicyFailureCause::Filter {
            cause: StartupCause::Io(_),
        } => {
            return Err(RepresentationError::PolicyCauseMismatch);
        }
    };
    Ok(PolicyStartupRefusal {
        capability,
        cause,
        origin: failure,
    })
}

fn project_io(cause: StartupIoCause) -> Result<io::Error, RepresentationError> {
    match StartupCause::Io(cause).project()? {
        // KindOnly expressly loses custom payloads. Capability selection and finite origin
        // custody come from the actual PolicyFailureCause arm, never this projection alone.
        ProjectedStartupCause::Io { error, .. } => Ok(error),
        ProjectedStartupCause::Seccompiler { .. } => Err(RepresentationError::PolicyCauseMismatch),
    }
}

#[cfg(test)]
mod tests {
    use super::super::{startup_cause::PreparedStartupContext, startup_envelope::CONTEXT_BYTES};
    use super::*;

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
