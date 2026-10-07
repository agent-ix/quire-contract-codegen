//! Closed guardian control schema and per-run authority binding (FR-034).
//!
//! The connected pair is the exclusive capability. The run identifier binds its ordered controls;
//! it is neither a public rendezvous secret nor a substitute for kernel peer authentication.

use std::{
    ffi::OsString,
    fs::File,
    io::{self, Read},
};

use serde::{Deserialize, Serialize};

use super::namespace::BackendCommand;

// Own authoritative thin helper source is a compiler input, not a source-tracking digest.
const _: &str = include_str!("../../bin/quire-kani-guardian.rs");
const ARTIFACT_EPOCH: [u8; 32] = quire_guardian_build_epoch::artifact_epoch!();

pub(super) fn current_build_identity() -> BuildIdentity {
    BuildIdentity {
        artifact: ARTIFACT_EPOCH,
        protocol: GuardianProtocol::PrivateBoundedLease,
        lifecycle: GuardianLifecycle::NamespaceInitTypedDispatchLeaseEof,
    }
}

macro_rules! build_identity_fields {
    ($($variant:ident => $field:ident: $value:ty),+ $(,)?) => {
        /// One actual library-compilation artifact identity, supplied only by the compiled library.
        #[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
        #[serde(deny_unknown_fields)]
        pub(super) struct BuildIdentity { $(pub(super) $field: $value),+ }

        #[derive(Clone, Copy)]
        pub(super) enum BuildIdentityField { $($variant),+ }

        impl BuildIdentityField {
            pub(super) const fn declared_order() -> &'static [Self] {
                &[$(Self::$variant),+]
            }

            pub(super) fn metadata_text(text: super::guardian_decode::Text<'_>) -> Option<Self> {
                $(if text.equals(stringify!($field)) { return Some(Self::$variant); })+
                None
            }
        }
    };
}

build_identity_fields! {
    Artifact => artifact: [u8; 32],
    Protocol => protocol: GuardianProtocol,
    Lifecycle => lifecycle: GuardianLifecycle,
}

macro_rules! identity_variants {
    ($name:ident { $($variant:ident),+ $(,)? }) => {
        #[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
        pub(super) enum $name { $( $variant, )+ }
        impl $name {
            pub(super) fn metadata_text(text: super::guardian_decode::Text<'_>) -> Option<Self> {
                $( if text.equals(stringify!($variant)) { return Some(Self::$variant); } )+
                None
            }
        }
    };
}

identity_variants!(GuardianProtocol {
    PrivateBoundedLease
});
identity_variants!(GuardianLifecycle {
    NamespaceInitTypedDispatchLeaseEof
});

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(super) struct RunAuthority([u8; 32]);

impl RunAuthority {
    /// Unverified parsed bytes only; all actual retained-run equality/authentication is separate.
    pub(super) const fn from_wire_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub(super) fn fresh() -> io::Result<Self> {
        let mut bytes = [0; 32];
        File::open("/dev/urandom")?.read_exact(&mut bytes)?;
        Ok(Self(bytes))
    }
}

macro_rules! caller_controls {
    ($($variant:ident { $($field:ident: $value:ty),+ $(,)? }),+ $(,)?) => {
        #[derive(Deserialize, Serialize)]
        #[serde(tag = "kind", deny_unknown_fields)]
        pub(super) enum CallerControl { $($variant { $($field: $value),+ }),+ }

        #[derive(Clone, Copy)]
        pub(super) enum CallerControlKind { $($variant),+ }

        impl CallerControlKind {
            pub(super) fn metadata_text(text: super::guardian_decode::Text<'_>) -> Option<Self> {
                $(if text.equals(stringify!($variant)) { return Some(Self::$variant); })+
                None
            }
        }
    };
}

caller_controls! {
    Hello {
        identity: BuildIdentity,
        authority: RunAuthority,
    },
    Dispatch {
        authority: RunAuthority,
        command: BackendCommand,
        stdin: StdinControl,
        cleanup_paths: Vec<OsString>,
    },
}

impl CallerControl {
    pub(super) fn rights_count(&self) -> usize {
        match self {
            Self::Hello { .. } => 0,
            Self::Dispatch { stdin, .. } => stdin.rights_count(),
        }
    }
}

identity_variants!(StdinControl { Open, Closed });

impl StdinControl {
    pub(super) fn rights_count(self) -> usize {
        match self {
            Self::Open => 1,
            Self::Closed => 0,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub(super) enum GuardianControl {
    Ready {
        identity: BuildIdentity,
        authority: RunAuthority,
        mapped_uid: u32,
        creator_pid: i32,
    },
    Dispatched {
        authority: RunAuthority,
    },
    Completed {
        authority: RunAuthority,
        outcome: BackendExit,
        stop: super::role_deadline::StopStamp,
    },
    Refused {
        reason: GuardianRefusal,
    },
}

macro_rules! backend_exits {
    ($($variant:ident($value:ty)),+ $(,)?) => {
        #[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
        pub(super) enum BackendExit { $($variant($value)),+ }

        #[derive(Clone, Copy)]
        pub(super) enum BackendExitTag { $($variant),+ }

        impl BackendExitTag {
            pub(super) fn metadata_text(text: super::guardian_decode::Text<'_>) -> Option<Self> {
                $(if text.equals(stringify!($variant)) { return Some(Self::$variant); })+
                None
            }
        }
    };
}

backend_exits! { Code(i32), Signal(i32) }

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(super) enum GuardianRefusal {
    NotNamespaceInit,
    SessionNotIsolated,
    CreatorUidMismatch,
    BuildIdentityMismatch,
    UnexpectedControl,
    ReplayedAuthority,
    InvalidControl,
    BackendSpawnFailed,
    BackendObservationFailed,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{os::unix::ffi::OsStringExt, path::Path, process::Command};

    /// Trace: FR-034-AC-15, FR-034-AC-17
    #[test]
    fn raw_recipe_roundtrip_preserves_non_utf8_program_argv_cwd_and_environment() {
        let program = OsString::from_vec(b"/backend/\xff".to_vec());
        let argument = OsString::from_vec(b"argument-\xfe".to_vec());
        let directory = OsString::from_vec(b"/directory/\xfd".to_vec());
        let name = OsString::from_vec(b"RAW_\xfc".to_vec());
        let value = OsString::from_vec(b"value-\xfb".to_vec());
        let mut recipe = BackendCommand::new(&program);
        recipe
            .arg(&argument)
            .current_dir(Path::new(&directory))
            .env(&name, &value);
        let encoded = serde_json::to_vec(&recipe).unwrap();
        let decoded: BackendCommand = serde_json::from_slice(&encoded).unwrap();
        let mut command = Command::new("unused-representation-oracle");
        decoded.configure(&mut command);
        assert_eq!(
            command.get_args().collect::<Vec<_>>(),
            vec![argument.as_os_str()]
        );
        assert_eq!(command.get_current_dir(), Some(Path::new(&directory)));
        assert!(command
            .get_envs()
            .any(|(actual_name, actual_value)| actual_name == name
                && actual_value == Some(value.as_os_str())));
        assert_eq!(serde_json::to_vec(&decoded).unwrap(), encoded);
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn typed_control_rejects_unknown_fields_and_unknown_variants() {
        assert!(serde_json::from_slice::<GuardianControl>(
            br#"{"kind":"Refused","reason":"InvalidControl","extra":1}"#
        )
        .is_err());
        assert!(
            serde_json::from_slice::<GuardianControl>(br#"{"kind":"AuthorizeAnything"}"#).is_err()
        );
        assert!(serde_json::from_slice::<CallerControl>(br#"{"kind":"Dispatch","authority":[],"command":{},"stdin":"Closed","cleanup_paths":[]}"#).is_err());
    }
}
