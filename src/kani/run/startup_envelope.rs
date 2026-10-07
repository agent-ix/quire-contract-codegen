//! Exact, allocation-bounded private trusted-installer reply decoding.
//!
//! Context is original UTF-8 carried as byte values, not an escaped JSON String. Scalar/tag
//! decoding uses one supplied fixed scratch buffer and the owning finite schema inventories.
//! The actual owner still authenticates sender/run/build/phase/stamp and confirms cleanup.

use serde::{Deserialize, Serialize};

use super::{
    control::{ControlError, CONTROL_BYTES},
    protocol::{BuildIdentity, RunAuthority},
    role_deadline::StopStamp,
    startup_cause::{PreparedStartupContext, StartupCause},
};

/// Derived from the existing whole control limit, not another execution/capture budget. The
/// actual bounded serializer separately checks the complete metadata plus context envelope.
pub(super) const CONTEXT_BYTES: usize = CONTROL_BYTES / 6 - 1;

/// Actual policy failure discriminant, never selected by parsing its Display/context.
macro_rules! policy_causes {
    ($($variant:ident $({ $field:ident: $value:ty })?),+ $(,)?) => {
        #[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
        #[serde(deny_unknown_fields)]
        pub(super) enum PolicyFailureCause {
            $($variant $({ $field: $value })?),+
        }

        #[derive(Clone, Copy)]
        pub(super) enum PolicyFailureTag { $($variant),+ }

        impl PolicyFailureTag {
            pub(super) fn metadata_text(text: super::guardian_decode::Text<'_>) -> Option<Self> {
                $(if text.equals(stringify!($variant)) { return Some(Self::$variant); })+
                None
            }
        }
    };
}

policy_causes! {
    UnsupportedArchitecture,
    InvalidProgram,
    NotBackend,
    ProtectionUnverified,
    Preparation { cause: StartupCause },
    Privilege { cause: StartupCause },
    Filter { cause: StartupCause },
}

/// Output stays a typed enum. No arbitrary original loader environment or public cause appears
/// in this temporary channel, and a Refused record cannot authorize arbitrary backend execution.
#[derive(Serialize)]
#[serde(tag = "kind")]
pub(super) enum InstallerReply<'a> {
    PolicyReady {
        identity: BuildIdentity,
        authority: RunAuthority,
    },
    Refused {
        identity: BuildIdentity,
        authority: RunAuthority,
        stop: StopStamp,
        failure: PolicyFailureCause,
        context: &'a [u8],
    },
}

/// Decoded fixed-size metadata. Context remains in the same separately reserved owner buffer.
pub(super) enum InstallerReplyHeader {
    PolicyReady {
        identity: BuildIdentity,
        authority: RunAuthority,
    },
    Refused {
        identity: BuildIdentity,
        authority: RunAuthority,
        stop: StopStamp,
        failure: PolicyFailureCause,
    },
}

impl InstallerReplyHeader {
    pub(super) fn rights_count(&self) -> usize {
        0
    }

    pub(super) fn identity(&self) -> BuildIdentity {
        match self {
            Self::PolicyReady { identity, .. } | Self::Refused { identity, .. } => *identity,
        }
    }

    pub(super) fn authority(&self) -> RunAuthority {
        match self {
            Self::PolicyReady { authority, .. } | Self::Refused { authority, .. } => *authority,
        }
    }
}

macro_rules! installer_reply_kinds {
    ($($variant:ident),+ $(,)?) => {
        #[derive(Clone, Copy, Deserialize)]
        pub(super) enum ReplyKind { $($variant),+ }
        impl ReplyKind {
            pub(super) fn metadata_text(text: super::guardian_decode::Text<'_>) -> Option<Self> {
                $(if text.equals(stringify!($variant)) { return Some(Self::$variant); })+
                None
            }
        }
    };
}

installer_reply_kinds! { PolicyReady, Refused }

/// Same exact installer schema on supplied resident scratch/context; no serde error is created.
pub(super) fn decode(
    payload: &[u8],
    context: &mut PreparedStartupContext,
    scratch: &mut super::guardian_decode::Scratch,
) -> Result<InstallerReplyHeader, ControlError> {
    super::installer_reply_decode::decode(payload, scratch, context)
        .map_err(|source| context.grammar_error(source))
}

/// I forwards only an authenticated pre-recipe installer refusal. The same bounded Refused
/// transaction retains the original producer stamp; I's receipt is not a new trigger.
pub(super) enum InnerStartupHeader {
    Ready(super::protocol::GuardianControl),
    Refused {
        identity: BuildIdentity,
        authority: RunAuthority,
        stop: StopStamp,
        failure: PolicyFailureCause,
    },
}

impl InnerStartupHeader {
    pub(super) fn rights_count(&self) -> usize {
        0
    }
}

macro_rules! inner_frame_kinds {
    ($($variant:ident),+ $(,)?) => {
        #[derive(Clone, Copy, Deserialize)]
        pub(super) enum InnerFrameKind { $($variant),+ }

        impl InnerFrameKind {
            pub(super) fn metadata_text(text: super::guardian_decode::Text<'_>) -> Option<Self> {
                $(if text.equals(stringify!($variant)) { return Some(Self::$variant); })+
                None
            }
        }
    };
}

inner_frame_kinds! { Ready, Dispatched, Completed, Refused }

/// Startup-only view of the one exact I reply grammar on the original owner-held cursor.
pub(super) fn decode_inner_startup(
    payload: &[u8],
    context: &mut PreparedStartupContext,
    scratch: &mut super::guardian_decode::Scratch,
) -> Result<InnerStartupHeader, ControlError> {
    match super::inner_reply_decode::decode(payload, scratch, context)
        .map_err(|source| context.grammar_error(source))?
    {
        super::inner_reply_decode::InnerReply::Ready(control) => {
            Ok(InnerStartupHeader::Ready(control))
        }
        super::inner_reply_decode::InnerReply::Refused {
            identity,
            authority,
            stop,
            failure,
        } => Ok(InnerStartupHeader::Refused {
            identity,
            authority,
            stop,
            failure,
        }),
        super::inner_reply_decode::InnerReply::Event(_) => Err(unexpected_inner_reply()),
    }
}

fn unexpected_inner_reply() -> ControlError {
    ControlError::InvalidGrammar(super::guardian_decode::DecodeError::new(
        super::guardian_decode::DecodeSite::Field,
        super::guardian_decode::DecodeCause::InvalidValue,
    ))
}

/// Post-Ready fixed-size events on the same original I lease. Refusal/EOF never stand for
/// acknowledgement or completion, and no report/cause context is allocated in this phase.
pub(super) enum InnerEventHeader {
    Dispatched {
        authority: RunAuthority,
    },
    Completed {
        authority: RunAuthority,
        outcome: super::protocol::BackendExit,
        stop: StopStamp,
    },
}

impl InnerEventHeader {
    pub(super) fn rights_count(&self) -> usize {
        0
    }
}

/// Event-only view; no Ready/refusal can authorize Dispatch acknowledgement or completion.
pub(super) fn decode_inner_event(
    payload: &[u8],
    context: &mut PreparedStartupContext,
    scratch: &mut super::guardian_decode::Scratch,
) -> Result<InnerEventHeader, ControlError> {
    match super::inner_reply_decode::decode(payload, scratch, context)
        .map_err(|source| context.grammar_error(source))?
    {
        super::inner_reply_decode::InnerReply::Event(event) => Ok(event),
        super::inner_reply_decode::InnerReply::Ready(_)
        | super::inner_reply_decode::InnerReply::Refused { .. } => Err(unexpected_inner_reply()),
    }
}

/// The original I-to-O stream permits a policy-negative event only before Dispatched. The
/// receiving owner authenticates phase, build, run, actual I credentials and producer stamp.
pub(super) enum InnerOwnerHeader {
    Event(InnerEventHeader),
    Refused {
        identity: BuildIdentity,
        authority: RunAuthority,
        stop: StopStamp,
        failure: PolicyFailureCause,
    },
}

impl InnerOwnerHeader {
    pub(super) fn rights_count(&self) -> usize {
        0
    }
}

/// Owner-side events/negative view, with no Ready authorization in this phase.
pub(super) fn decode_inner_owner(
    payload: &[u8],
    context: &mut PreparedStartupContext,
    scratch: &mut super::guardian_decode::Scratch,
) -> Result<InnerOwnerHeader, ControlError> {
    match super::inner_reply_decode::decode(payload, scratch, context)
        .map_err(|source| context.grammar_error(source))?
    {
        super::inner_reply_decode::InnerReply::Event(event) => Ok(InnerOwnerHeader::Event(event)),
        super::inner_reply_decode::InnerReply::Refused {
            identity,
            authority,
            stop,
            failure,
        } => Ok(InnerOwnerHeader::Refused {
            identity,
            authority,
            stop,
            failure,
        }),
        super::inner_reply_decode::InnerReply::Ready(_) => Err(unexpected_inner_reply()),
    }
}

/// Fixed subset-view output values coexist with the shared parser's returned union.
/// These are separate from its schema delta and the resident scratch/context capacities;
/// declared layout sums do not establish supported-build native-stack highwater.
pub(super) fn inner_reply_view_bytes() -> Result<u64, ControlError> {
    use std::mem::{size_of, size_of_val};
    let terms = [
        size_of::<InnerStartupHeader>(),
        size_of::<InnerEventHeader>(),
        size_of::<InnerOwnerHeader>(),
        size_of::<Result<InnerStartupHeader, ControlError>>(),
        size_of::<Result<InnerEventHeader, ControlError>>(),
        size_of::<Result<InnerOwnerHeader, ControlError>>(),
    ];
    let bytes = terms.iter().try_fold(size_of_val(&terms), |bytes, term| {
        bytes
            .checked_add(*term)
            .ok_or(ControlError::EncodedBytesExceeded)
    })?;
    u64::try_from(bytes).map_err(|_| ControlError::EncodedBytesExceeded)
}

#[cfg(test)]
mod tests {
    macro_rules! context_parsed {
        ($name:ident, $result:ty) => {
            fn $name(
                payload: &[u8],
                context: &mut super::PreparedStartupContext,
            ) -> Result<$result, super::ControlError> {
                super::$name(
                    payload,
                    context,
                    &mut super::super::guardian_decode::Scratch::default(),
                )
            }
        };
    }
    context_parsed!(decode, super::InstallerReplyHeader);
    context_parsed!(decode_inner_startup, super::InnerStartupHeader);
    context_parsed!(decode_inner_owner, super::InnerOwnerHeader);
    fn decode_inner_event(payload: &[u8]) -> Result<super::InnerEventHeader, super::ControlError> {
        super::decode_inner_event(
            payload,
            &mut super::PreparedStartupContext::new(super::CONTEXT_BYTES).unwrap(),
            &mut super::super::guardian_decode::Scratch::default(),
        )
    }
    use super::super::startup_cause::check_scratch_free_json;
    use crate::kani::run::cross_role_cause::{
        CauseOperation, RemoteCauseProvenance, RemoteCauseRole,
    };

    fn projection_origin() -> RemoteCauseProvenance {
        RemoteCauseProvenance {
            role: RemoteCauseRole::BackendInstaller,
            operation: CauseOperation::NativePolicyPreparation,
        }
    }

    use super::*;
    use crate::kani::run::{
        protocol::current_build_identity,
        role_deadline::StopOrigin,
        startup_cause::{ProjectedStartupCause, ProjectionFidelity, StartupJsonError},
    };
    use std::io;

    /// Trace: FR-034-AC-15, FR-034-AC-35, FR-034-AC-38, FR-034-AC-39
    #[test]
    fn inner_owner_negative_keeps_original_metadata_and_cannot_be_a_positive_event() {
        let authority = RunAuthority::fresh().unwrap();
        let identity = current_build_identity();
        let stop = StopStamp::capture(StopOrigin::Backend).unwrap();
        let mut original = PreparedStartupContext::new(CONTEXT_BYTES).unwrap();
        let cause = original
            .capture_io(&io::Error::from(rustix::io::Errno::PERM))
            .unwrap();
        let failure = PolicyFailureCause::Privilege { cause };
        let encoded = serde_json::to_vec(&InstallerReply::Refused {
            identity,
            authority,
            stop,
            failure,
            context: original.context().as_bytes(),
        })
        .unwrap();
        let mut received = PreparedStartupContext::new(CONTEXT_BYTES).unwrap();
        let capacity = received.reserved_bytes();
        let InnerOwnerHeader::Refused {
            identity: actual_identity,
            authority: actual_authority,
            stop: actual_stop,
            failure: actual_failure,
        } = decode_inner_owner(&encoded, &mut received).unwrap()
        else {
            panic!("negative event became positive authorization");
        };
        assert_eq!(actual_identity, identity);
        assert_eq!(actual_authority, authority);
        assert_eq!(actual_stop, stop);
        assert_eq!(actual_failure, failure);
        assert_eq!(received.context(), original.context());
        assert_eq!(received.reserved_bytes(), capacity);
        assert!(decode_inner_event(&encoded).is_err());
        let original_value: serde_json::Value = serde_json::from_slice(&encoded).unwrap();
        for field in ["identity", "authority", "stop", "failure", "context"] {
            let mut malformed = original_value.clone();
            malformed.as_object_mut().unwrap().remove(field);
            assert!(
                decode_inner_owner(&serde_json::to_vec(&malformed).unwrap(), &mut received)
                    .is_err()
            );
        }
        let mut extra = original_value;
        extra
            .as_object_mut()
            .unwrap()
            .insert("outcome".to_owned(), serde_json::json!({"Code":0}));
        assert!(decode_inner_owner(&serde_json::to_vec(&extra).unwrap(), &mut received).is_err());
    }

    /// Trace: FR-034-AC-15, FR-034-AC-39
    #[test]
    fn context_free_inner_events_preserve_original_diagnostics_and_do_not_enter_startup() {
        use super::super::protocol::{BackendExit, GuardianControl};
        let authority = RunAuthority::fresh().unwrap();
        let stop = StopStamp::capture(StopOrigin::Inner).unwrap();
        let mut context = PreparedStartupContext::new(CONTEXT_BYTES).unwrap();
        context
            .capture_io(&io::Error::new(
                io::ErrorKind::PermissionDenied,
                "actual policy diagnostic",
            ))
            .unwrap();
        let original = context.context().to_owned();
        let capacity = context.reserved_bytes();
        let mut scratch = super::super::guardian_decode::Scratch::default();
        let dispatched = serde_json::to_vec(&GuardianControl::Dispatched { authority }).unwrap();
        let completed = serde_json::to_vec(&GuardianControl::Completed {
            authority,
            outcome: BackendExit::Code(0),
            stop,
        })
        .unwrap();
        for (payload, is_completion) in [(&dispatched, false), (&completed, true)] {
            match super::decode_inner_event(payload, &mut context, &mut scratch).unwrap() {
                InnerEventHeader::Dispatched { authority: actual } => {
                    assert!(!is_completion);
                    assert_eq!(actual, authority);
                }
                InnerEventHeader::Completed {
                    authority: actual,
                    outcome,
                    stop: actual_stop,
                } => {
                    assert!(is_completion);
                    assert_eq!(actual, authority);
                    assert_eq!(outcome, BackendExit::Code(0));
                    assert_eq!(actual_stop, stop);
                }
            }
            assert_eq!(context.context(), original);
            assert_eq!(context.reserved_bytes(), capacity);
            assert!(super::decode_inner_startup(payload, &mut context, &mut scratch).is_err());
            assert_eq!(context.context(), original);
            assert_eq!(context.reserved_bytes(), capacity);
        }
        let ready = serde_json::to_vec(&GuardianControl::Ready {
            identity: current_build_identity(),
            authority,
            mapped_uid: 0,
            creator_pid: 0,
        })
        .unwrap();
        assert!(matches!(
            super::decode_inner_startup(&ready, &mut context, &mut scratch).unwrap(),
            InnerStartupHeader::Ready(GuardianControl::Ready { authority: actual, .. }) if actual == authority
        ));
        assert!(super::decode_inner_owner(&ready, &mut context, &mut scratch).is_err());
        assert_eq!(context.context(), original);
        assert_eq!(context.reserved_bytes(), capacity);
    }

    /// Trace: FR-034-AC-15, FR-034-AC-20, FR-034-AC-38
    #[test]
    fn inner_events_preserve_actual_typed_completion_and_refuse_wrong_phase_schema() {
        use super::super::protocol::{BackendExit, GuardianControl};
        let authority = RunAuthority::fresh().unwrap();
        let stop = StopStamp::capture(StopOrigin::Inner).unwrap();
        let dispatched = serde_json::to_vec(&GuardianControl::Dispatched { authority }).unwrap();
        assert!(matches!(decode_inner_event(&dispatched).unwrap(),
            InnerEventHeader::Dispatched { authority: actual } if actual == authority));
        let completed = serde_json::to_vec(&GuardianControl::Completed {
            authority,
            outcome: BackendExit::Signal(9),
            stop,
        })
        .unwrap();
        assert!(matches!(decode_inner_event(&completed).unwrap(),
            InnerEventHeader::Completed { authority: actual, outcome: BackendExit::Signal(9), stop: stamp }
                if actual == authority && stamp == stop));
        let original: serde_json::Value = serde_json::from_slice(&completed).unwrap();
        for field in ["authority", "outcome", "stop"] {
            let mut missing = original.clone();
            missing.as_object_mut().unwrap().remove(field);
            assert!(decode_inner_event(&serde_json::to_vec(&missing).unwrap()).is_err());
            let mut wrong = original.clone();
            wrong[field] = serde_json::Value::String("wrong-type".to_owned());
            assert!(decode_inner_event(&serde_json::to_vec(&wrong).unwrap()).is_err());
        }
        let mut extra = original.clone();
        extra["bytes"] = serde_json::Value::Number(1.into());
        assert!(decode_inner_event(&serde_json::to_vec(&extra).unwrap()).is_err());
        let mut wrong_phase = original;
        wrong_phase["kind"] = serde_json::Value::String("Dispatched".to_owned());
        assert!(decode_inner_event(&serde_json::to_vec(&wrong_phase).unwrap()).is_err());
        let ready = serde_json::to_vec(&GuardianControl::Ready {
            identity: current_build_identity(),
            authority,
            mapped_uid: 0,
            creator_pid: 0,
        })
        .unwrap();
        assert!(decode_inner_event(&ready).is_err());
        let mut trailing = completed;
        trailing.push(b'x');
        assert!(decode_inner_event(&trailing).is_err());
    }

    /// Trace: FR-034-AC-15, FR-034-AC-35, FR-034-AC-38, FR-034-AC-39
    #[test]
    fn inner_startup_preserves_forwarded_original_os_refusal_and_strict_ready_schema() {
        let original = io::Error::from_raw_os_error(nix::libc::EPERM);
        let mut producer = PreparedStartupContext::new(CONTEXT_BYTES).unwrap();
        let cause = producer.capture_io(&original).unwrap();
        let identity = current_build_identity();
        let authority = RunAuthority::fresh().unwrap();
        let stop = StopStamp::capture(StopOrigin::Backend).unwrap();
        let refusal = serde_json::to_vec(&InstallerReply::Refused {
            identity,
            authority,
            stop,
            failure: PolicyFailureCause::Privilege { cause },
            context: producer.context_bytes(),
        })
        .unwrap();
        let mut receiver = PreparedStartupContext::new(CONTEXT_BYTES).unwrap();
        let capacity = receiver.reserved_bytes();
        let InnerStartupHeader::Refused {
            identity: actual_identity,
            authority: actual_authority,
            stop: actual_stop,
            failure: PolicyFailureCause::Privilege { cause },
        } = decode_inner_startup(&refusal, &mut receiver).unwrap()
        else {
            panic!("I relay changed refusal into admission or lost its original site");
        };
        assert_eq!(actual_identity, identity);
        assert_eq!(actual_authority, authority);
        assert_eq!(actual_stop, stop);
        assert_eq!(receiver.context(), original.to_string());
        assert_eq!(receiver.reserved_bytes(), capacity);
        let ProjectedStartupCause::Io { error, fidelity } =
            cause.project(projection_origin()).unwrap()
        else {
            panic!("relayed OS error changed domains");
        };
        assert_eq!(error.raw_os_error(), original.raw_os_error());
        assert_eq!(error.kind(), original.kind());
        assert_eq!(fidelity, ProjectionFidelity::OsCodeAndKind);
        let ready = serde_json::to_vec(&super::super::protocol::GuardianControl::Ready {
            identity,
            authority,
            mapped_uid: 0,
            creator_pid: 0,
        })
        .unwrap();
        assert!(
            matches!(decode_inner_startup(&ready, &mut receiver).unwrap(),
            InnerStartupHeader::Ready(super::super::protocol::GuardianControl::Ready {
                identity: actual_identity, authority: actual_authority,
                mapped_uid: 0, creator_pid: 0,
            }) if actual_identity == identity && actual_authority == authority)
        );
        // Ready has no context field: preserve the pending original refusal diagnostic.
        assert_eq!(receiver.context(), original.to_string());
        assert_eq!(receiver.reserved_bytes(), capacity);
        let original_ready: serde_json::Value = serde_json::from_slice(&ready).unwrap();
        for field in ["identity", "authority", "mapped_uid", "creator_pid"] {
            let mut missing = original_ready.clone();
            missing.as_object_mut().unwrap().remove(field);
            assert!(
                decode_inner_startup(&serde_json::to_vec(&missing).unwrap(), &mut receiver)
                    .is_err()
            );
            let mut wrong = original_ready.clone();
            wrong[field] = serde_json::Value::String("wrong".to_owned());
            assert!(
                decode_inner_startup(&serde_json::to_vec(&wrong).unwrap(), &mut receiver).is_err()
            );
        }
        let mut extra = original_ready;
        extra["failure"] = serde_json::Value::String("not-admission".to_owned());
        assert!(decode_inner_startup(&serde_json::to_vec(&extra).unwrap(), &mut receiver).is_err());
        let positive_installer = serde_json::to_vec(&InstallerReply::PolicyReady {
            identity,
            authority,
        })
        .unwrap();
        assert!(decode_inner_startup(&positive_installer, &mut receiver).is_err());
        let mut refusal_without_stamp: serde_json::Value =
            serde_json::from_slice(&refusal).unwrap();
        refusal_without_stamp
            .as_object_mut()
            .unwrap()
            .remove("stop");
        assert!(decode_inner_startup(
            &serde_json::to_vec(&refusal_without_stamp).unwrap(),
            &mut receiver
        )
        .is_err());
    }

    /// Trace: FR-034-AC-15, FR-034-AC-35, FR-034-AC-38
    #[test]
    fn actual_error_context_and_typed_metadata_survive_bounded_byte_reply() {
        let original = io::Error::new(io::ErrorKind::PermissionDenied, "policy α\n\"\\ setup");
        let mut producer = PreparedStartupContext::new(CONTEXT_BYTES).unwrap();
        let cause = producer.capture_io(&original).unwrap();
        let identity = current_build_identity();
        let authority = RunAuthority::fresh().unwrap();
        let stop = StopStamp::capture(StopOrigin::Backend).unwrap();
        let bytes = serde_json::to_vec(&InstallerReply::Refused {
            identity,
            authority,
            stop,
            failure: PolicyFailureCause::Privilege { cause },
            context: producer.context_bytes(),
        })
        .unwrap();
        assert!(bytes.len() <= CONTROL_BYTES);
        let mut receiver = PreparedStartupContext::new(CONTEXT_BYTES).unwrap();
        let reserved = receiver.reserved_bytes();
        let decoded = decode(&bytes, &mut receiver).unwrap();
        assert_eq!(receiver.context(), original.to_string());
        assert_eq!(receiver.reserved_bytes(), reserved);
        assert_eq!(decoded.identity(), identity);
        assert_eq!(decoded.authority(), authority);
        let InstallerReplyHeader::Refused {
            stop: observed,
            failure: PolicyFailureCause::Privilege { cause },
            ..
        } = decoded
        else {
            panic!("actual privilege failure was not preserved");
        };
        assert_eq!(observed, stop);
        let ProjectedStartupCause::Io { error, fidelity } =
            cause.project(projection_origin()).unwrap()
        else {
            panic!("original IO cause changed its typed domain");
        };
        assert_eq!(error.kind(), original.kind());
        assert_eq!(error.raw_os_error(), original.raw_os_error());
        assert_eq!(fidelity, ProjectionFidelity::OpaqueCustomLoss);
        let ready = serde_json::to_vec(&InstallerReply::PolicyReady {
            identity,
            authority,
        })
        .unwrap();
        assert!(matches!(
            decode(&ready, &mut receiver).unwrap(),
            InstallerReplyHeader::PolicyReady { .. }
        ));
        assert!(receiver.context().is_empty());
        assert_eq!(receiver.reserved_bytes(), reserved);
        // Three-digit byte values exercise the actual worst sequence expansion at this bound.
        let largest = io::Error::new(io::ErrorKind::Other, "x".repeat(CONTEXT_BYTES));
        let cause = producer.capture_io(&largest).unwrap();
        let largest_frame = serde_json::to_vec(&InstallerReply::Refused {
            identity,
            authority,
            stop,
            failure: PolicyFailureCause::Preparation { cause },
            context: producer.context_bytes(),
        })
        .unwrap();
        assert!(largest_frame.len() <= CONTROL_BYTES);
        assert!(matches!(
            decode(&largest_frame, &mut receiver).unwrap(),
            InstallerReplyHeader::Refused { .. }
        ));
        assert_eq!(receiver.context(), largest.to_string());
        assert_eq!(receiver.reserved_bytes(), reserved);
    }

    /// Trace: FR-034-AC-15, FR-034-AC-35, FR-034-AC-38
    #[test]
    fn required_metadata_refuses_but_optional_diagnostics_preserve_original_cause() {
        let mut producer = PreparedStartupContext::new(CONTEXT_BYTES).unwrap();
        let cause = producer
            .capture_io(&io::Error::from_raw_os_error(nix::libc::EPERM))
            .unwrap();
        let bytes = serde_json::to_vec(&InstallerReply::Refused {
            identity: current_build_identity(),
            authority: RunAuthority::fresh().unwrap(),
            stop: StopStamp::capture(StopOrigin::Backend).unwrap(),
            failure: PolicyFailureCause::Privilege { cause },
            context: producer.context_bytes(),
        })
        .unwrap();
        let original: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        let mut receiver = PreparedStartupContext::new(CONTEXT_BYTES).unwrap();
        let InstallerReplyHeader::Refused {
            failure: PolicyFailureCause::Privilege { cause },
            ..
        } = decode(&bytes, &mut receiver).unwrap()
        else {
            panic!("original OS privilege cause changed its typed site");
        };
        let ProjectedStartupCause::Io { error, fidelity } =
            cause.project(projection_origin()).unwrap()
        else {
            panic!("original OS cause changed its typed domain");
        };
        assert_eq!(error.raw_os_error(), Some(nix::libc::EPERM));
        assert_eq!(error.kind(), io::ErrorKind::PermissionDenied);
        assert_eq!(fidelity, ProjectionFidelity::OsCodeAndKind);
        for field in [
            "kind",
            "identity",
            "authority",
            "stop",
            "failure",
            "context",
        ] {
            let mut wrong = original.clone();
            wrong[field] = serde_json::Value::String("wrong-type".to_owned());
            assert!(
                decode(&serde_json::to_vec(&wrong).unwrap(), &mut receiver).is_err(),
                "wrong {field} type accepted"
            );
            let mut missing = original.clone();
            missing.as_object_mut().unwrap().remove(field);
            assert!(
                decode(&serde_json::to_vec(&missing).unwrap(), &mut receiver).is_err(),
                "missing {field} accepted"
            );
        }
        for context in [
            serde_json::json!([255]),
            serde_json::json!([240, 159]),
            serde_json::json!([192, 128]),
        ] {
            let mut wrong = original.clone();
            wrong["context"] = context;
            let InstallerReplyHeader::Refused {
                failure: PolicyFailureCause::Privilege { cause },
                ..
            } = decode(&serde_json::to_vec(&wrong).unwrap(), &mut receiver).unwrap()
            else {
                panic!("optional diagnostics changed the original refusal");
            };
            assert!(receiver.context().is_empty());
            let ProjectedStartupCause::Io { error, .. } =
                cause.project(projection_origin()).unwrap()
            else {
                panic!("optional diagnostics changed the original cause domain");
            };
            assert_eq!(error.raw_os_error(), Some(nix::libc::EPERM));
            assert_eq!(error.kind(), io::ErrorKind::PermissionDenied);
        }
        for context in [
            serde_json::json!([255, "bad"]),
            serde_json::json!([255, 256]),
            serde_json::json!([null]),
        ] {
            let mut wrong = original.clone();
            wrong["context"] = context;
            assert!(decode(&serde_json::to_vec(&wrong).unwrap(), &mut receiver).is_err());
        }
        let mut extra = original.clone();
        extra["unknown"] = serde_json::json!(0);
        assert!(decode(&serde_json::to_vec(&extra).unwrap(), &mut receiver).is_err());
        let duplicate = format!(
            "{{\"kind\":\"Refused\",{}",
            std::str::from_utf8(&bytes)
                .unwrap()
                .strip_prefix('{')
                .unwrap()
        );
        assert!(decode(duplicate.as_bytes(), &mut receiver).is_err());
        let escaped = std::str::from_utf8(&bytes)
            .unwrap()
            .replace("\"context\"", "\"con\\u0074ext\"");
        assert_eq!(
            check_scratch_free_json(escaped.as_bytes()),
            Err(StartupJsonError::EscapedString)
        );
        for integer in [b"[18446744073709551616]".as_slice(), b"[1e100]", b"[0.5]"] {
            assert!(check_scratch_free_json(integer).is_err());
        }
        let mut trailing = bytes.clone();
        trailing.extend_from_slice(b"{}");
        assert!(decode(&trailing, &mut receiver).is_err());
        let mut tiny = PreparedStartupContext::new(1).unwrap();
        let InstallerReplyHeader::Refused {
            failure: PolicyFailureCause::Privilege { cause },
            ..
        } = decode(&bytes, &mut tiny).unwrap()
        else {
            panic!("diagnostic retention excess changed the refusal");
        };
        assert!(tiny.context().is_empty());
        let ProjectedStartupCause::Io { error, .. } = cause.project(projection_origin()).unwrap()
        else {
            panic!("diagnostic retention excess changed the cause domain");
        };
        assert_eq!(error.raw_os_error(), Some(nix::libc::EPERM));
    }
}
