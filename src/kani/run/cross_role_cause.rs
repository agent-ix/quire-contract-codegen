//! Opaque public identities for cross-role cause loss and cause-metadata integrity (FR-034).
//!
//! These values carry facts, not authority. The owner selects the actual role, operation and
//! predicate, authenticates its control and state, and applies the existing public mapping.
//! This module captures or projects no I/O error, serializes no source, and allocates no storage.
//! Original producer provenance and the independently known checking site are distinct.

use std::{error::Error, fmt};

use serde::{Deserialize, Serialize};

/// An original producer in the finite trusted-helper scope; neither C nor external M is one.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum RemoteCauseRole {
    Launcher,
    Outer,
    Inner,
    BackendInstaller,
}

/// The actual checking role, independently known rather than taken from a packet label.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum CauseCheckerRole {
    Caller,
    Launcher,
    Outer,
    Inner,
    BackendInstaller,
}

// The original finite enum declaration also supplies borrowed decoder labels. This does not
// authorize any role/site cross product or alter portable marker/source behavior.
macro_rules! wire_metadata_enum {
    ($(#[$attribute:meta])* $name:ident { $($variant:ident),+ $(,)? }) => {
        $(#[$attribute])*
        #[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
        pub(super) enum $name { $( $variant, )+ }

        impl $name {
            #[cfg(target_os = "linux")]
            pub(super) fn metadata_text(text: super::guardian_decode::Text<'_>) -> Option<Self> {
                $( if text.equals(stringify!($variant)) { return Some(Self::$variant); } )+
                None
            }
        }
    };
}

/// A finite operation name, not permission for every role/operation/phase combination.
///
/// The owner must enforce FR-034's actual role/site/state scope before constructing provenance.
wire_metadata_enum!(CauseOperation {
    RoleBootstrap,
    Identity,
    OwnerProtection,
    NamespaceSetup,
    ProcSetup,
    ControlPreparation,
    ControlEncoding,
    ControlReception,
    OuterSpawn,
    OuterControl,
    OuterWaitRetirement,
    LauncherObservation,
    TreeObservation,
    ResourceAccounting,
    ReportCreation,
    ReportCollection,
    ReportSealing,
    ReportDelivery,
    MonitorSpawn,
    MonitorClaim,
    MonitorStop,
    MonitorReap,
    ExclusiveLease,
    NativePolicyPreparation,
    NativePolicyInstallation,
    BackendSupervision,
    BackendCompletion,
    SettlementControl,
});

/// Authenticated original producer facts retained independently of its lost source object.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct RemoteCauseProvenance {
    pub(super) role: RemoteCauseRole,
    pub(super) operation: CauseOperation,
}

/// Only independently known actual checking facts; either field may genuinely be absent.
///
/// A caller's receive/projection check does not claim the caller performed remote installation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct CauseCheckerProvenance {
    pub(super) role: Option<CauseCheckerRole>,
    pub(super) operation: Option<CauseOperation>,
}

/// The single closed FR-034 cause-integrity predicate inventory.
wire_metadata_enum!(CauseIntegrityPredicate {
    RequiredRepresentationExceededBound,
    RequiredRepresentationFormattingFailed,
    UnnameableOriginalKind,
    OriginalOsKindMismatch,
    NonInstallationDependencyCause,
    PolicySiteCauseMismatch,
    UnknownKindMetadata,
    MalformedCauseMetadata,
    IncompleteCauseMetadata,
});

impl fmt::Display for CauseIntegrityPredicate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let diagnostic = match self {
            Self::RequiredRepresentationExceededBound => {
                "required representation exceeded its bound"
            }
            Self::RequiredRepresentationFormattingFailed => {
                "required representation formatting failed"
            }
            Self::UnnameableOriginalKind => "original kind has no admitted representation",
            Self::OriginalOsKindMismatch => "OS kind representation mismatch",
            Self::NonInstallationDependencyCause => {
                "dependency cause is outside installation scope"
            }
            Self::PolicySiteCauseMismatch => "cause does not match its policy site",
            Self::UnknownKindMetadata => "unknown kind metadata",
            Self::MalformedCauseMetadata => "malformed required cause metadata",
            Self::IncompleteCauseMetadata => "incomplete required cause custody",
        };
        formatter.write_str(diagnostic)
    }
}

/// A lost original custom payload or source chain in a cross-role I/O cause projection.
///
/// Detect this marker with `io::Error::get_ref()` and `downcast_ref::<Self>()`. It is not the
/// original source and preserves no original object, value or downcast identity. Its `source()`
/// is always `None`. Private provenance grants no role/site authority or public stage query;
/// `Debug` and `Display` are diagnostic only and must not select a classification.
///
/// The owner attaches it only to an authenticated original no-errno custom cause whose payload
/// was actually lost. Loss-free OS and payload-free causes, local original sources and stronger
/// existing native-policy source duties must remain intact. It adds no carrier to detail-only
/// refusals. This type neither captures a cause nor constructs its surrounding I/O error.
#[derive(Debug)]
pub struct KaniCrossRoleCauseLoss {
    provenance: RemoteCauseProvenance,
}

impl KaniCrossRoleCauseLoss {
    pub(super) fn new(provenance: RemoteCauseProvenance) -> Self {
        Self { provenance }
    }
}

impl fmt::Display for KaniCrossRoleCauseLoss {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "cross-role original payload lost ({:?}/{:?})",
            self.provenance.role, self.provenance.operation
        )
    }
}

impl Error for KaniCrossRoleCauseLoss {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

/// An actual owned local error; none of these values is a reconstructed remote source object.
#[derive(Debug)]
pub(super) enum CauseIntegritySource {
    Encoding(serde_json::Error),
    #[cfg(target_os = "linux")]
    Control(super::control::ControlError),
    #[cfg(target_os = "linux")]
    Representation(super::startup_cause::RepresentationError),
}

impl CauseIntegritySource {
    fn as_error(&self) -> &(dyn Error + 'static) {
        match self {
            Self::Encoding(error) => error,
            #[cfg(target_os = "linux")]
            Self::Control(error) => error,
            #[cfg(target_os = "linux")]
            Self::Representation(error) => error,
        }
    }
}

/// An actual capture/check/required-metadata integrity failure, not an original producer cause.
///
/// Detect this error with `io::Error::get_ref()` and `downcast_ref::<Self>()`; neither I/O kind,
/// source presence nor diagnostic text distinguishes it from genuine original/local errors.
/// Its private predicate and independently known optional checker facts provide no public
/// role/site getter or stage query. Unknown facts remain absent, not inferred from packet labels.
/// `Debug` and `Display` supply diagnostic text only.
///
/// `source()` exposes the actual owned local decoder/control/representation error when present,
/// including that error's real nested chain. It does not fabricate a remote original payload.
/// An authenticated finite sender fault may have no local source object. Optional diagnostic
/// rendering failure alone is not required-metadata integrity; the owner enforces that boundary.
/// Public projection applies only to existing cause-bearing refusals; detail-only observation
/// results retain their existing shape and private facts without a new I/O carrier.
#[derive(Debug)]
pub struct KaniCauseMetadataIntegrityError {
    predicate: CauseIntegrityPredicate,
    provenance: CauseCheckerProvenance,
    local: Option<CauseIntegritySource>,
}

impl KaniCauseMetadataIntegrityError {
    pub(super) fn new(
        predicate: CauseIntegrityPredicate,
        provenance: CauseCheckerProvenance,
        local: Option<CauseIntegritySource>,
    ) -> Self {
        Self {
            predicate,
            provenance,
            local,
        }
    }
}

impl fmt::Display for KaniCauseMetadataIntegrityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "cause metadata integrity: {} ({:?}/{:?})",
            self.predicate, self.provenance.role, self.provenance.operation
        )
    }
}

impl Error for KaniCauseMetadataIntegrityError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.local.as_ref().map(CauseIntegritySource::as_error)
    }
}

#[cfg(test)]
mod tests {
    // Primitive identity/source checks only, not whole AC-40 producer/transport acceptance.
    use std::io;

    use super::*;

    fn assert_send_sync<T: Send + Sync>() {}

    fn caller_check() -> CauseCheckerProvenance {
        CauseCheckerProvenance {
            role: Some(CauseCheckerRole::Caller),
            operation: Some(CauseOperation::ControlReception),
        }
    }

    #[test]
    fn loss_marker_retains_private_origin_and_exposes_no_original_source() {
        assert_send_sync::<KaniCrossRoleCauseLoss>();
        let provenance = RemoteCauseProvenance {
            role: RemoteCauseRole::Outer,
            operation: CauseOperation::ReportCollection,
        };
        let cause = io::Error::new(
            io::ErrorKind::Other,
            KaniCrossRoleCauseLoss::new(provenance),
        );
        let marker = cause
            .get_ref()
            .unwrap()
            .downcast_ref::<KaniCrossRoleCauseLoss>()
            .unwrap();
        assert_eq!(marker.provenance, provenance);
        assert!(marker.source().is_none());
        assert!(cause
            .get_ref()
            .unwrap()
            .downcast_ref::<KaniCauseMetadataIntegrityError>()
            .is_none());
    }

    #[test]
    fn integrity_downcast_keeps_the_actual_owned_decoder_error() {
        assert_send_sync::<KaniCauseMetadataIntegrityError>();
        let decoder = serde_json::from_str::<u8>("{").unwrap_err();
        let provenance = caller_check();
        let cause = io::Error::new(
            io::ErrorKind::InvalidData,
            KaniCauseMetadataIntegrityError::new(
                CauseIntegrityPredicate::MalformedCauseMetadata,
                provenance,
                Some(CauseIntegritySource::Encoding(decoder)),
            ),
        );
        let marker = cause
            .get_ref()
            .unwrap()
            .downcast_ref::<KaniCauseMetadataIntegrityError>()
            .unwrap();
        assert_eq!(marker.provenance, provenance);
        assert_eq!(
            marker.predicate,
            CauseIntegrityPredicate::MalformedCauseMetadata
        );
        let Some(CauseIntegritySource::Encoding(owned)) = marker.local.as_ref() else {
            panic!("actual decoder source was not retained");
        };
        let exposed = marker
            .source()
            .unwrap()
            .downcast_ref::<serde_json::Error>()
            .unwrap();
        assert!(std::ptr::eq(owned, exposed));
        assert!(cause
            .get_ref()
            .unwrap()
            .downcast_ref::<KaniCrossRoleCauseLoss>()
            .is_none());

        // Direct loss-free representations are controls, not executed producer observations.
        for original in [
            io::Error::from_raw_os_error(22),
            io::ErrorKind::InvalidData.into(),
        ] {
            assert!(original
                .get_ref()
                .and_then(|source| source.downcast_ref::<KaniCauseMetadataIntegrityError>())
                .is_none());
            assert!(original
                .get_ref()
                .and_then(|source| source.downcast_ref::<KaniCrossRoleCauseLoss>())
                .is_none());
        }
    }

    #[test]
    fn unknown_checker_fields_remain_absent_without_an_invented_source() {
        for provenance in [
            CauseCheckerProvenance {
                role: None,
                operation: None,
            },
            CauseCheckerProvenance {
                role: Some(CauseCheckerRole::Caller),
                operation: None,
            },
            CauseCheckerProvenance {
                role: None,
                operation: Some(CauseOperation::ControlReception),
            },
        ] {
            let marker = KaniCauseMetadataIntegrityError::new(
                CauseIntegrityPredicate::IncompleteCauseMetadata,
                provenance,
                None,
            );
            assert_eq!(marker.provenance, provenance);
            assert!(marker.source().is_none());
        }
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn control_source_exposes_its_actual_error_and_nested_decoder_chain() {
        use super::super::control::ControlError;

        let decoder = serde_json::from_str::<u8>("{").unwrap_err();
        let marker = KaniCauseMetadataIntegrityError::new(
            CauseIntegrityPredicate::MalformedCauseMetadata,
            caller_check(),
            Some(CauseIntegritySource::Control(
                ControlError::InvalidEncoding(decoder),
            )),
        );
        let Some(CauseIntegritySource::Control(owned)) = marker.local.as_ref() else {
            panic!("actual control source was not retained");
        };
        let exposed = marker
            .source()
            .unwrap()
            .downcast_ref::<ControlError>()
            .unwrap();
        assert!(std::ptr::eq(owned, exposed));
        let ControlError::InvalidEncoding(decoder) = owned else {
            panic!("actual nested decoder error was not retained");
        };
        let nested = exposed
            .source()
            .unwrap()
            .downcast_ref::<serde_json::Error>()
            .unwrap();
        assert!(std::ptr::eq(decoder, nested));
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn representation_source_is_not_skipped_when_its_own_source_is_absent() {
        use super::super::startup_cause::RepresentationError;

        let marker = KaniCauseMetadataIntegrityError::new(
            CauseIntegrityPredicate::PolicySiteCauseMismatch,
            caller_check(),
            Some(CauseIntegritySource::Representation(
                RepresentationError::PolicyCauseMismatch,
            )),
        );
        let Some(CauseIntegritySource::Representation(owned)) = marker.local.as_ref() else {
            panic!("actual representation source was not retained");
        };
        let exposed = marker
            .source()
            .unwrap()
            .downcast_ref::<RepresentationError>()
            .unwrap();
        assert!(std::ptr::eq(owned, exposed));
        assert!(exposed.source().is_none());
    }
}
