//! Consuming projection of original local caller errors after caller-owned settlement.
//!
//! Known local I/O leaves move directly into the returned io::Error, preserving its kind, errno
//! and concrete custom payload. Non-I/O failures retain the actual owned typed caller error as
//! source. This is neither a remote cause reconstruction nor a public classification decision.

use std::io;

use super::{
    caller_bootstrap::CallerBootstrapError, caller_driver::CallerDriveError,
    caller_execution::CallerExecutionError, caller_prepare::PreparationError,
    control::ControlError, launch::BoundedLaunchError, namespace::ReadyIdentityError,
    report_storage::ReportError, stages::StageError,
};

/// Move the original reachable local I/O cause; otherwise retain the exact typed owned error.
/// Direct extraction does not preserve every intermediate wrapper in the returned I/O envelope:
/// the caller separately retains meaningful stage/context and selects the public boundary only
/// after cleanup. No error payload supplies role ownership or confirmed-cleanup authority.
pub(super) fn into_original_io(error: CallerExecutionError) -> io::Error {
    use CallerBootstrapError as Bootstrap;
    use CallerDriveError as Progress;
    use CallerExecutionError as Execution;
    use PreparationError as Preparation;
    use StageError as Stage;

    match error {
        Execution::Io(error)
        | Execution::Assembly(BoundedLaunchError::Io(error))
        | Execution::Assembly(BoundedLaunchError::BoundaryIo { cause: error, .. })
        | Execution::Assembly(BoundedLaunchError::Unavailable { cause: error, .. }) => error,
        Execution::Preparation(Preparation::Stage(
            Stage::Authority(error)
            | Stage::Control(ControlError::Io(error))
            | Stage::Identity(ReadyIdentityError::Observation(error)),
        ))
        | Execution::Progress(Progress::Stage(
            Stage::Authority(error)
            | Stage::Control(ControlError::Io(error))
            | Stage::Identity(ReadyIdentityError::Observation(error)),
        )) => error,
        Execution::Bootstrap(
            Bootstrap::Io(error)
            | Bootstrap::Control(ControlError::Io(error))
            | Bootstrap::Report(ReportError::Io(error))
            | Bootstrap::Stage(
                Stage::Authority(error)
                | Stage::Control(ControlError::Io(error))
                | Stage::Identity(ReadyIdentityError::Observation(error)),
            ),
        )
        | Execution::Preparation(Preparation::CallerBootstrap(
            Bootstrap::Io(error)
            | Bootstrap::Control(ControlError::Io(error))
            | Bootstrap::Report(ReportError::Io(error))
            | Bootstrap::Stage(
                Stage::Authority(error)
                | Stage::Control(ControlError::Io(error))
                | Stage::Identity(ReadyIdentityError::Observation(error)),
            ),
        ))
        | Execution::Progress(Progress::Bootstrap(
            Bootstrap::Io(error)
            | Bootstrap::Control(ControlError::Io(error))
            | Bootstrap::Report(ReportError::Io(error))
            | Bootstrap::Stage(
                Stage::Authority(error)
                | Stage::Control(ControlError::Io(error))
                | Stage::Identity(ReadyIdentityError::Observation(error)),
            ),
        )) => error,
        Execution::Bootstrap(Bootstrap::Spawn(error))
        | Execution::Preparation(Preparation::CallerBootstrap(Bootstrap::Spawn(error)))
        | Execution::Progress(Progress::Bootstrap(Bootstrap::Spawn(error))) => error.into_io(),
        original @ (Execution::Preparation(_)
        | Execution::Progress(_)
        | Execution::Bootstrap(_)
        | Execution::Deadline(_)
        | Execution::Assembly(BoundedLaunchError::Guardian { .. })
        | Execution::PolicyProjection(_)) => io::Error::other(original),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct OriginalPayload(u64);

    impl std::fmt::Display for OriginalPayload {
        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(formatter, "local payload {}", self.0)
        }
    }

    impl std::error::Error for OriginalPayload {}

    /// Trace: FR-034-AC-15, FR-034-AC-38, FR-034-AC-39.
    #[test]
    fn nested_local_custom_io_preserves_original_kind_and_concrete_payload_identity() {
        let original = io::Error::new(io::ErrorKind::PermissionDenied, OriginalPayload(41));
        let address = original
            .get_ref()
            .unwrap()
            .downcast_ref::<OriginalPayload>()
            .unwrap() as *const OriginalPayload;
        let returned = into_original_io(CallerExecutionError::Preparation(
            PreparationError::CallerBootstrap(CallerBootstrapError::Stage(StageError::Authority(
                original,
            ))),
        ));
        assert_eq!(returned.kind(), io::ErrorKind::PermissionDenied);
        assert_eq!(returned.raw_os_error(), None);
        let payload = returned
            .get_ref()
            .unwrap()
            .downcast_ref::<OriginalPayload>()
            .unwrap();
        assert_eq!(payload.0, 41);
        assert!(std::ptr::eq(payload, address));
    }

    /// Trace: FR-034-AC-15, FR-034-AC-38, FR-034-AC-39.
    #[test]
    fn nested_local_os_io_keeps_raw_errno_and_kind_without_outer_wrapping() {
        // Actual local std I/O values in production enum shapes; no remote syscall/site claim.
        let wrappers: [fn(io::Error) -> CallerExecutionError; 7] = [
            CallerExecutionError::Io,
            |error| CallerExecutionError::Assembly(BoundedLaunchError::Io(error)),
            |error| {
                CallerExecutionError::Bootstrap(CallerBootstrapError::Control(ControlError::Io(
                    error,
                )))
            },
            |error| {
                CallerExecutionError::Bootstrap(CallerBootstrapError::Report(ReportError::Io(
                    error,
                )))
            },
            |error| {
                CallerExecutionError::Preparation(PreparationError::Stage(StageError::Authority(
                    error,
                )))
            },
            |error| {
                CallerExecutionError::Progress(CallerDriveError::Stage(StageError::Identity(
                    ReadyIdentityError::Observation(error),
                )))
            },
            |error| {
                CallerExecutionError::Progress(CallerDriveError::Bootstrap(
                    CallerBootstrapError::Stage(StageError::Control(ControlError::Io(error))),
                ))
            },
        ];
        for wrap in wrappers {
            let original = io::Error::from_raw_os_error(rustix::io::Errno::ACCESS.raw_os_error());
            let kind = original.kind();
            let errno = original.raw_os_error();
            let returned = into_original_io(wrap(original));
            assert_eq!(returned.kind(), kind);
            assert_eq!(returned.raw_os_error(), errno);
            assert!(returned.get_ref().is_none());
        }
    }

    /// Trace: FR-034-AC-15, FR-034-AC-38.
    #[test]
    fn non_io_local_failure_retains_exact_typed_owned_source() {
        let returned = into_original_io(CallerExecutionError::Preparation(
            PreparationError::CaptureSizeOverflow,
        ));
        assert_eq!(returned.raw_os_error(), None);
        assert!(matches!(
            returned
                .get_ref()
                .unwrap()
                .downcast_ref::<CallerExecutionError>(),
            Some(CallerExecutionError::Preparation(
                PreparationError::CaptureSizeOverflow
            ))
        ));
    }
}
