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
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) enum PolicyFailureCause {
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

#[derive(Deserialize)]
enum ReplyKind {
    PolicyReady,
    Refused,
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kani::run::{
        protocol::current_build_identity,
        role_deadline::StopOrigin,
        startup_cause::{ProjectedStartupCause, ProjectionFidelity, StartupJsonError},
    };
    use std::io;

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
        let ProjectedStartupCause::Io { error, fidelity } = cause.project().unwrap() else {
            panic!("original IO cause changed its typed domain");
        };
        assert_eq!(error.kind(), original.kind());
        assert_eq!(error.raw_os_error(), original.raw_os_error());
        assert_eq!(fidelity, ProjectionFidelity::KindOnly);
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
    fn malformed_failure_fields_context_and_scratch_paths_refuse() {
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
        let ProjectedStartupCause::Io { error, fidelity } = cause.project().unwrap() else {
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
        assert!(decode(&bytes, &mut tiny).is_err());
    }
}
