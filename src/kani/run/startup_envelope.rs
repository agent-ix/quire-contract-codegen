//! Exact, allocation-bounded private trusted-installer reply decoding.
//!
//! Context is original UTF-8 carried as byte values, not an escaped JSON String. Scalar/tag
//! preflight excludes serde_json scratch paths before the seeded schema decoder touches data.
//! The actual owner still authenticates sender/run/build/phase/stamp and confirms cleanup.

use std::fmt;

use serde::de::{DeserializeSeed, Error as _};
use serde::{de, Deserialize, Serialize};

use super::{
    control::{ControlError, CONTROL_BYTES},
    protocol::{BuildIdentity, RunAuthority},
    role_deadline::StopStamp,
    startup_cause::{check_scratch_free_json, PreparedStartupContext, StartupCause},
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

#[derive(Deserialize)]
#[serde(field_identifier, rename_all = "snake_case")]
enum Field {
    Kind,
    Identity,
    Authority,
    Stop,
    Failure,
    Context,
}

struct ReplySeed<'a>(&'a mut PreparedStartupContext);

impl<'de> DeserializeSeed<'de> for ReplySeed<'_> {
    type Value = InstallerReplyHeader;

    fn deserialize<D: de::Deserializer<'de>>(
        self,
        deserializer: D,
    ) -> Result<Self::Value, D::Error> {
        deserializer.deserialize_map(self)
    }
}

impl<'de> de::Visitor<'de> for ReplySeed<'_> {
    type Value = InstallerReplyHeader;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("one exact trusted installer reply")
    }

    fn visit_map<A: de::MapAccess<'de>>(self, mut fields: A) -> Result<Self::Value, A::Error> {
        let mut kind = None;
        let mut identity = None;
        let mut authority = None;
        let mut stop = None;
        let mut failure = None;
        let mut context = false;
        while let Some(field) = fields.next_key::<Field>()? {
            match field {
                Field::Kind => {
                    if kind.is_some() {
                        return Err(A::Error::duplicate_field("kind"));
                    }
                    kind = Some(fields.next_value::<ReplyKind>()?);
                }
                Field::Identity => {
                    if identity.is_some() {
                        return Err(A::Error::duplicate_field("identity"));
                    }
                    identity = Some(fields.next_value()?);
                }
                Field::Authority => {
                    if authority.is_some() {
                        return Err(A::Error::duplicate_field("authority"));
                    }
                    authority = Some(fields.next_value()?);
                }
                Field::Stop => {
                    if stop.is_some() {
                        return Err(A::Error::duplicate_field("stop"));
                    }
                    stop = Some(fields.next_value()?);
                }
                Field::Failure => {
                    if failure.is_some() {
                        return Err(A::Error::duplicate_field("failure"));
                    }
                    failure = Some(fields.next_value()?);
                }
                Field::Context => {
                    if context {
                        return Err(A::Error::duplicate_field("context"));
                    }
                    fields.next_value_seed(self.0.bytes_seed())?;
                    context = true;
                }
            }
        }
        let identity = identity.ok_or_else(|| A::Error::missing_field("identity"))?;
        let authority = authority.ok_or_else(|| A::Error::missing_field("authority"))?;
        match kind.ok_or_else(|| A::Error::missing_field("kind"))? {
            ReplyKind::PolicyReady => {
                if stop.is_some() || failure.is_some() || context {
                    return Err(A::Error::custom("PolicyReady carries refusal fields"));
                }
                Ok(InstallerReplyHeader::PolicyReady {
                    identity,
                    authority,
                })
            }
            ReplyKind::Refused => {
                if !context {
                    return Err(A::Error::missing_field("context"));
                }
                let failure = failure.ok_or_else(|| A::Error::missing_field("failure"))?;
                match failure {
                    PolicyFailureCause::Preparation {
                        cause: StartupCause::Io(_),
                    }
                    | PolicyFailureCause::Privilege {
                        cause: StartupCause::Io(_),
                    }
                    | PolicyFailureCause::Filter {
                        cause: StartupCause::Seccompiler(_),
                    }
                    | PolicyFailureCause::UnsupportedArchitecture
                    | PolicyFailureCause::InvalidProgram
                    | PolicyFailureCause::NotBackend
                    | PolicyFailureCause::ProtectionUnverified => {}
                    PolicyFailureCause::Preparation {
                        cause: StartupCause::Seccompiler(_),
                    }
                    | PolicyFailureCause::Privilege {
                        cause: StartupCause::Seccompiler(_),
                    }
                    | PolicyFailureCause::Filter {
                        cause: StartupCause::Io(_),
                    } => {
                        return Err(A::Error::custom(
                            "startup cause does not match actual failure site",
                        ));
                    }
                }
                Ok(InstallerReplyHeader::Refused {
                    identity,
                    authority,
                    stop: stop.ok_or_else(|| A::Error::missing_field("stop"))?,
                    failure,
                })
            }
        }
    }
}

/// Uses only the shared bounded frame's borrowed payload and pre-reserved context. Exact schema
/// and JSON EOF are checked; no ambient syscall/error message selects the failure discriminant.
pub(super) fn decode(
    payload: &[u8],
    context: &mut PreparedStartupContext,
) -> Result<InstallerReplyHeader, ControlError> {
    context.clear();
    check_scratch_free_json(payload).map_err(|error| {
        ControlError::InvalidEncoding(<serde_json::Error as de::Error>::custom(error))
    })?;
    let mut decoder = serde_json::Deserializer::from_slice(payload);
    let header = ReplySeed(context)
        .deserialize(&mut decoder)
        .map_err(ControlError::InvalidEncoding)?;
    decoder.end().map_err(ControlError::InvalidEncoding)?;
    Ok(header)
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

#[derive(Deserialize)]
enum InnerReplyKind {
    Ready,
    Refused,
}

#[derive(Deserialize)]
struct InnerReplySelector {
    kind: InnerReplyKind,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct InnerReadyReply {
    kind: InnerReplyKind,
    identity: BuildIdentity,
    authority: RunAuthority,
    mapped_uid: u32,
    creator_pid: i32,
}

/// Flat schemas avoid serde's internally tagged Content accumulator. The context decoder
/// borrows the original frame and copies only validated UTF-8 into C's pre-L reservation.
pub(super) fn decode_inner_startup(
    payload: &[u8],
    context: &mut PreparedStartupContext,
) -> Result<InnerStartupHeader, ControlError> {
    context.clear();
    check_scratch_free_json(payload).map_err(|error| {
        ControlError::InvalidEncoding(<serde_json::Error as de::Error>::custom(error))
    })?;
    let selector: InnerReplySelector =
        serde_json::from_slice(payload).map_err(ControlError::InvalidEncoding)?;
    match selector.kind {
        InnerReplyKind::Ready => {
            let ready: InnerReadyReply =
                serde_json::from_slice(payload).map_err(ControlError::InvalidEncoding)?;
            if !matches!(ready.kind, InnerReplyKind::Ready) {
                return Err(ControlError::InvalidEncoding(
                    <serde_json::Error as de::Error>::custom("expected exact I Ready"),
                ));
            }
            Ok(InnerStartupHeader::Ready(
                super::protocol::GuardianControl::Ready {
                    identity: ready.identity,
                    authority: ready.authority,
                    mapped_uid: ready.mapped_uid,
                    creator_pid: ready.creator_pid,
                },
            ))
        }
        InnerReplyKind::Refused => match decode(payload, context)? {
            InstallerReplyHeader::Refused {
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
            InstallerReplyHeader::PolicyReady { .. } => Err(ControlError::InvalidEncoding(
                <serde_json::Error as de::Error>::custom("expected exact I refusal"),
            )),
        },
    }
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

#[derive(Deserialize)]
enum InnerEventKind {
    Dispatched,
    Completed,
}

#[derive(Deserialize)]
struct InnerEventSelector {
    kind: InnerEventKind,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct DispatchedReply {
    kind: InnerEventKind,
    authority: RunAuthority,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CompletedReply {
    kind: InnerEventKind,
    authority: RunAuthority,
    outcome: super::protocol::BackendExit,
    stop: StopStamp,
}

pub(super) fn decode_inner_event(payload: &[u8]) -> Result<InnerEventHeader, ControlError> {
    check_scratch_free_json(payload).map_err(|error| {
        ControlError::InvalidEncoding(<serde_json::Error as de::Error>::custom(error))
    })?;
    let selector: InnerEventSelector =
        serde_json::from_slice(payload).map_err(ControlError::InvalidEncoding)?;
    match selector.kind {
        InnerEventKind::Dispatched => {
            let reply: DispatchedReply =
                serde_json::from_slice(payload).map_err(ControlError::InvalidEncoding)?;
            if !matches!(reply.kind, InnerEventKind::Dispatched) {
                return Err(ControlError::InvalidEncoding(
                    <serde_json::Error as de::Error>::custom("expected exact I Dispatched"),
                ));
            }
            Ok(InnerEventHeader::Dispatched {
                authority: reply.authority,
            })
        }
        InnerEventKind::Completed => {
            let reply: CompletedReply =
                serde_json::from_slice(payload).map_err(ControlError::InvalidEncoding)?;
            if !matches!(reply.kind, InnerEventKind::Completed) {
                return Err(ControlError::InvalidEncoding(
                    <serde_json::Error as de::Error>::custom("expected exact I Completed"),
                ));
            }
            Ok(InnerEventHeader::Completed {
                authority: reply.authority,
                outcome: reply.outcome,
                stop: reply.stop,
            })
        }
    }
}

pub(super) fn inner_event_decode_bytes() -> Result<u64, ControlError> {
    let bytes = std::mem::size_of::<InnerEventSelector>()
        .checked_add(
            std::mem::size_of::<DispatchedReply>().max(std::mem::size_of::<CompletedReply>()),
        )
        .and_then(|bytes| bytes.checked_add(std::mem::size_of::<InnerEventHeader>()))
        .ok_or(ControlError::EncodedBytesExceeded)?;
    u64::try_from(bytes).map_err(|_| ControlError::EncodedBytesExceeded)
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

#[derive(Deserialize)]
enum InnerOwnerKind {
    Dispatched,
    Completed,
    Refused,
}

#[derive(Deserialize)]
struct InnerOwnerSelector {
    kind: InnerOwnerKind,
}

pub(super) fn decode_inner_owner(
    payload: &[u8],
    context: &mut PreparedStartupContext,
) -> Result<InnerOwnerHeader, ControlError> {
    check_scratch_free_json(payload).map_err(|error| {
        ControlError::InvalidEncoding(<serde_json::Error as de::Error>::custom(error))
    })?;
    let selector: InnerOwnerSelector =
        serde_json::from_slice(payload).map_err(ControlError::InvalidEncoding)?;
    match selector.kind {
        InnerOwnerKind::Dispatched | InnerOwnerKind::Completed => {
            decode_inner_event(payload).map(InnerOwnerHeader::Event)
        }
        InnerOwnerKind::Refused => match decode(payload, context)? {
            InstallerReplyHeader::Refused {
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
            InstallerReplyHeader::PolicyReady { .. } => Err(ControlError::InvalidEncoding(
                <serde_json::Error as de::Error>::custom("expected exact I owner refusal"),
            )),
        },
    }
}

/// Fixed decoder-owned values coexist during selection and return; the context capacity is
/// charged separately from these owning metadata values by the retained caller before L.
pub(super) fn inner_startup_decode_bytes() -> Result<u64, ControlError> {
    let bytes = std::mem::size_of::<InnerReplySelector>()
        .checked_add(
            std::mem::size_of::<InnerReadyReply>().max(std::mem::size_of::<InstallerReplyHeader>()),
        )
        .and_then(|bytes| bytes.checked_add(std::mem::size_of::<InnerStartupHeader>()))
        .ok_or(ControlError::EncodedBytesExceeded)?;
    u64::try_from(bytes).map_err(|_| ControlError::EncodedBytesExceeded)
}

#[cfg(test)]
mod tests {
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
        assert!(receiver.context().is_empty());
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
