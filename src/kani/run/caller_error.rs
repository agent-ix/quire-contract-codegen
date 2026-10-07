//! Consuming projection of original local caller errors after caller-owned settlement.
//!
//! Known local I/O leaves move directly into the returned io::Error, preserving its kind, errno
//! and concrete custom payload. Required-cause checking faults expose the actual local integrity
//! marker with the moved control/decoder source; other non-I/O failures retain the owned caller
//! error as source. This performs no remote cause replay or public classification decision.

use std::io;

use super::{
    caller_bootstrap::CallerBootstrapError,
    caller_driver::CallerDriveError,
    caller_execution::CallerExecutionError,
    caller_prepare::PreparationError,
    control::ControlError,
    cross_role_cause::{
        CauseCheckerProvenance, CauseCheckerRole, CauseIntegrityPredicate, CauseIntegritySource,
        CauseOperation, KaniCauseMetadataIntegrityError,
    },
    launch::BoundedLaunchError,
    namespace::ReadyIdentityError,
    report_storage::ReportError,
    stages::StageError,
};

/// Recognize only an actual local required-cause checking error. Other failures retain their
/// original path; neither an encoding message nor a packet label supplies this predicate.
pub(super) fn metadata_predicate(error: &CallerExecutionError) -> Option<CauseIntegrityPredicate> {
    use CallerBootstrapError as Bootstrap;
    use CallerDriveError as Progress;
    use CallerExecutionError as Execution;
    use PreparationError as Preparation;
    use StageError as Stage;

    match error {
        Execution::Preparation(Preparation::Stage(Stage::Control(
            ControlError::CauseMetadata { predicate, .. },
        )))
        | Execution::Progress(Progress::Stage(Stage::Control(ControlError::CauseMetadata {
            predicate,
            ..
        })))
        | Execution::Bootstrap(
            Bootstrap::Control(ControlError::CauseMetadata { predicate, .. })
            | Bootstrap::Stage(Stage::Control(ControlError::CauseMetadata { predicate, .. })),
        )
        | Execution::Preparation(Preparation::CallerBootstrap(
            Bootstrap::Control(ControlError::CauseMetadata { predicate, .. })
            | Bootstrap::Stage(Stage::Control(ControlError::CauseMetadata { predicate, .. })),
        ))
        | Execution::Progress(Progress::Bootstrap(
            Bootstrap::Control(ControlError::CauseMetadata { predicate, .. })
            | Bootstrap::Stage(Stage::Control(ControlError::CauseMetadata { predicate, .. })),
        )) => Some(*predicate),
        _ => None,
    }
}

/// Move the original reachable local I/O cause, or expose the actual local metadata checking
/// source through its integrity marker; otherwise retain the exact typed owned error.
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
        Execution::Preparation(Preparation::Stage(Stage::Control(
            control @ ControlError::CauseMetadata { predicate, .. },
        )))
        | Execution::Progress(Progress::Stage(Stage::Control(
            control @ ControlError::CauseMetadata { predicate, .. },
        )))
        | Execution::Bootstrap(
            Bootstrap::Control(control @ ControlError::CauseMetadata { predicate, .. })
            | Bootstrap::Stage(Stage::Control(
                control @ ControlError::CauseMetadata { predicate, .. },
            )),
        )
        | Execution::Preparation(Preparation::CallerBootstrap(
            Bootstrap::Control(control @ ControlError::CauseMetadata { predicate, .. })
            | Bootstrap::Stage(Stage::Control(
                control @ ControlError::CauseMetadata { predicate, .. },
            )),
        ))
        | Execution::Progress(Progress::Bootstrap(
            Bootstrap::Control(control @ ControlError::CauseMetadata { predicate, .. })
            | Bootstrap::Stage(Stage::Control(
                control @ ControlError::CauseMetadata { predicate, .. },
            )),
        )) => io::Error::new(
            io::ErrorKind::InvalidData,
            KaniCauseMetadataIntegrityError::new(
                predicate,
                CauseCheckerProvenance {
                    role: Some(CauseCheckerRole::Caller),
                    operation: Some(CauseOperation::ControlReception),
                },
                Some(CauseIntegritySource::Control(control)),
            ),
        ),
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
        | Execution::Assembly(
            BoundedLaunchError::Guardian { .. }
            | BoundedLaunchError::MemoryObservationFailed { .. },
        )
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

    /// Trace: FR-034-AC-15
    #[test]
    fn actual_kind_decoder_fault_projects_integrity_with_its_owned_control_source() {
        use super::super::{cause_metadata::CauseSeed, startup_cause::PreparedStartupContext};
        use serde::de::DeserializeSeed;

        let mut context = PreparedStartupContext::new(0).unwrap();
        let mut decoder = serde_json::Deserializer::from_slice(
            br#"{"Io":{"kind":"UnknownKind","raw_os_error":null,"payload":"NoCustomPayload"}}"#,
        );
        let source = CauseSeed::new(context.metadata_fault_slot())
            .deserialize(&mut decoder)
            .unwrap_err();
        let original = CallerExecutionError::Progress(CallerDriveError::Bootstrap(
            CallerBootstrapError::Control(context.metadata_error(source)),
        ));
        assert_eq!(
            metadata_predicate(&original),
            Some(CauseIntegrityPredicate::UnknownKindMetadata)
        );
        let returned = into_original_io(original);
        assert_eq!(returned.kind(), io::ErrorKind::InvalidData);
        assert_eq!(returned.raw_os_error(), None);
        let marker = returned
            .get_ref()
            .unwrap()
            .downcast_ref::<KaniCauseMetadataIntegrityError>()
            .unwrap();
        let control = std::error::Error::source(marker)
            .unwrap()
            .downcast_ref::<ControlError>()
            .unwrap();
        let ControlError::CauseMetadata { predicate, source } = control else {
            panic!("actual decoder source was replaced")
        };
        assert_eq!(*predicate, CauseIntegrityPredicate::UnknownKindMetadata);
        assert!(source.is_data());
        assert!(std::ptr::eq(
            source,
            std::error::Error::source(control)
                .unwrap()
                .downcast_ref::<serde_json::Error>()
                .unwrap()
        ));
    }
}
