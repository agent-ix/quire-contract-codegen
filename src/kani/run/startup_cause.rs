//! Private typed startup-cause metadata and separately charged, reusable UTF-8 context.
//!
//! These values carry no site, capability, sender or admission authority. The role owner binds
//! those facts and budgets the complete control envelope. Serialization cannot preserve arbitrary
//! concrete I/O source chains or downcast identity. Projection states that loss explicitly.
//! OS-derived kinds and native seccompiler PID payloads are a same-host/toolchain transport,
//! not a portable persisted error format.

#![cfg(target_os = "linux")]

use std::{collections::TryReserveError, fmt, io};

use serde::{de, Deserialize, Serialize};

use super::{
    control::CONTROL_BYTES,
    cross_role_cause::{CauseIntegrityPredicate, KaniCrossRoleCauseLoss, RemoteCauseProvenance},
};

macro_rules! io_kinds {
    ($($kind:ident),+ $(,)?) => {
        /// Stable named kinds, or a kind positively reconstructed from its original OS code.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
        pub(super) enum StartupIoKind { $($kind),+, OsDerived }

        impl TryFrom<io::ErrorKind> for StartupIoKind {
            type Error = RepresentationError;

            fn try_from(kind: io::ErrorKind) -> Result<Self, Self::Error> {
                match kind {
                    $(io::ErrorKind::$kind => Ok(Self::$kind),)+
                    _ => Err(RepresentationError::UnnamedIoKind),
                }
            }
        }

        impl StartupIoKind {
            /// Recognize only an existing metadata name. Unknown metadata is a receiver fault,
            /// never a substitute original kind or permission to enlarge the producer domain.
            pub(super) fn metadata_name(name: &str) -> Option<Self> {
                match name {
                    $(stringify!($kind) => Some(Self::$kind),)+
                    "OsDerived" => Some(Self::OsDerived),
                    _ => None,
                }
            }

            /// Match a validated borrowed token against this same owning declaration.
            pub(super) fn metadata_text(text: super::guardian_decode::Text<'_>) -> Option<Self> {
                $( if text.equals(stringify!($kind)) { return Some(Self::$kind); } )+
                if text.equals("OsDerived") { Some(Self::OsDerived) } else { None }
            }

            fn named(self) -> Option<io::ErrorKind> {
                match self {
                    $(Self::$kind => Some(io::ErrorKind::$kind),)+
                    Self::OsDerived => None,
                }
            }
        }
    };
}

io_kinds!(
    NotFound,
    PermissionDenied,
    ConnectionRefused,
    ConnectionReset,
    HostUnreachable,
    NetworkUnreachable,
    ConnectionAborted,
    NotConnected,
    AddrInUse,
    AddrNotAvailable,
    NetworkDown,
    BrokenPipe,
    AlreadyExists,
    WouldBlock,
    NotADirectory,
    IsADirectory,
    DirectoryNotEmpty,
    ReadOnlyFilesystem,
    StaleNetworkFileHandle,
    InvalidInput,
    InvalidData,
    TimedOut,
    WriteZero,
    StorageFull,
    NotSeekable,
    QuotaExceeded,
    FileTooLarge,
    ResourceBusy,
    ExecutableFileBusy,
    Deadlock,
    CrossesDevices,
    TooManyLinks,
    InvalidFilename,
    ArgumentListTooLong,
    Interrupted,
    Unsupported,
    UnexpectedEof,
    OutOfMemory,
    Other,
);

macro_rules! payload_kinds {
    ($($payload:ident),+ $(,)?) => {
        /// Positively observed custom-payload facts, separate from ErrorKind and diagnostics.
        /// A direct reservation is identified through its stable concrete type only; no
        /// allocator-versus-capacity category, layout or equivalent payload is inferred.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
        pub(super) enum StartupPayload { $($payload),+ }

        impl StartupPayload {
            /// Recognize the existing payload fact without copying an unknown token into a
            /// dependency error message. This supplies no original payload/source authority.
            pub(super) fn metadata_name(name: &str) -> Option<Self> {
                match name {
                    $(stringify!($payload) => Some(Self::$payload),)+
                    _ => None,
                }
            }

            /// Decode without heap unescaping or a second payload inventory.
            pub(super) fn metadata_text(text: super::guardian_decode::Text<'_>) -> Option<Self> {
                $( if text.equals(stringify!($payload)) { return Some(Self::$payload); } )+
                None
            }
        }
    };
}

payload_kinds!(NoCustomPayload, DirectTryReserve, UnrepresentedCustom);

/// Original kind representation and optional original OS error; neither comes from Display.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct StartupIoCause {
    kind: StartupIoKind,
    #[serde(deserialize_with = "required_errno")]
    raw_os_error: Option<i32>,
    payload: StartupPayload,
}

// Explicit null records a positively captured absence. An omitted member is a required
// metadata fault; deserialize_with makes serde reject omission rather than default Option.
fn required_errno<'de, D: de::Deserializer<'de>>(deserializer: D) -> Result<Option<i32>, D::Error> {
    Option::<i32>::deserialize(deserializer)
}

impl StartupIoCause {
    /// Construct only parsed typed metadata. This grants no producer/site authority and does
    /// not verify errno/kind/payload consistency; the existing projector performs that check.
    pub(super) fn from_metadata(
        kind: StartupIoKind,
        raw_os_error: Option<i32>,
        payload: StartupPayload,
    ) -> Self {
        Self {
            kind,
            raw_os_error,
            payload,
        }
    }

    fn capture(error: &io::Error) -> Result<Self, RepresentationError> {
        let raw_os_error = error.raw_os_error();
        let kind = match error.kind().try_into() {
            Ok(kind) => kind,
            Err(RepresentationError::UnnamedIoKind) => {
                let errno = raw_os_error.ok_or(RepresentationError::UnnamedIoKind)?;
                if io::Error::from_raw_os_error(errno).kind() != error.kind() {
                    return Err(RepresentationError::OsKindMismatch);
                }
                StartupIoKind::OsDerived
            }
            Err(error) => return Err(error),
        };
        if let Some(errno) = raw_os_error {
            if io::Error::from_raw_os_error(errno).kind() != error.kind() {
                return Err(RepresentationError::OsKindMismatch);
            }
        }
        let payload = match error.get_ref() {
            None => StartupPayload::NoCustomPayload,
            Some(source) if source.is::<TryReserveError>() => StartupPayload::DirectTryReserve,
            Some(_) => StartupPayload::UnrepresentedCustom,
        };
        Ok(Self {
            kind,
            raw_os_error,
            payload,
        })
    }

    /// These actual capture facts survive private transport. They are not site authority or
    /// permission to replace a local/AC39 representable source with a generic loss marker.
    pub(super) fn payload(self) -> StartupPayload {
        self.payload
    }

    /// Validate required captured facts without constructing an I/O carrier or loss marker.
    /// Detail-only observation failures retain the actual result privately after settlement.
    pub(super) fn validate_metadata(self) -> Result<(), RepresentationError> {
        match self.raw_os_error {
            Some(errno) => {
                if self.payload != StartupPayload::NoCustomPayload {
                    return Err(RepresentationError::OsPayloadMismatch);
                }
                if self
                    .kind
                    .named()
                    .is_some_and(|kind| io::Error::from_raw_os_error(errno).kind() != kind)
                {
                    return Err(RepresentationError::OsKindMismatch);
                }
            }
            None => {
                self.kind
                    .named()
                    .ok_or(RepresentationError::MissingOsCode)?;
            }
        }
        Ok(())
    }

    fn project(
        self,
        provenance: RemoteCauseProvenance,
    ) -> Result<(io::Error, ProjectionFidelity), RepresentationError> {
        self.validate_metadata()?;
        let kind = self.kind.named();
        match self.raw_os_error {
            Some(errno) => {
                let error = io::Error::from_raw_os_error(errno);
                Ok((error, ProjectionFidelity::OsCodeAndKind))
            }
            None => {
                let kind = kind.ok_or(RepresentationError::MissingOsCode)?;
                match self.payload {
                    StartupPayload::NoCustomPayload => {
                        Ok((io::Error::from(kind), ProjectionFidelity::PayloadFree))
                    }
                    StartupPayload::DirectTryReserve | StartupPayload::UnrepresentedCustom => Ok((
                        io::Error::new(kind, KaniCrossRoleCauseLoss::new(provenance)),
                        ProjectionFidelity::OpaqueCustomLoss,
                    )),
                }
            }
        }
    }
}

// One declaration supplies each external enum's serialized variants and decoder labels.
// Tags are private parsing facts only: they grant no installation, site or sender authority.
macro_rules! cause_variants {
    ($name:ident, $tag:ident; $(
        $variant:ident $(($payload:ty))? $({ $($field:ident: $field_type:ty),+ $(,)? })?
    ),+ $(,)?) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
        #[serde(deny_unknown_fields)]
        pub(super) enum $name {
            $( $variant $(($payload))? $({ $($field: $field_type),+ })?, )+
        }

        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub(super) enum $tag { $( $variant, )+ }

        impl $tag {
            /// Exact owning variant-label authority, without allocating unknown input text.
            pub(super) fn metadata_name(name: &str) -> Option<Self> {
                match name {
                    $( stringify!($variant) => Some(Self::$variant), )+
                    _ => None,
                }
            }

            /// Match decoded characters using the same original variant declaration.
            pub(super) fn metadata_text(text: super::guardian_decode::Text<'_>) -> Option<Self> {
                $( if text.equals(stringify!($variant)) { return Some(Self::$variant); } )+
                None
            }
        }
    };
}

// Actual apply_filter installation causes, distinct from context and role/site selection.
cause_variants!(StartupSeccompilerCause, StartupSeccompilerTag;
    EmptyFilter,
    Prctl(StartupIoCause),
    Seccomp(StartupIoCause),
    ThreadSync { pid: std::os::raw::c_long },
);

// Allocation-free metadata; original Display is retained in separate prepared storage.
cause_variants!(StartupCause, StartupCauseTag;
    Io(StartupIoCause),
    Seccompiler(StartupSeccompilerCause),
);

/// Projection fidelity is never a claim that an arbitrary original source chain survived.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ProjectionFidelity {
    /// Original errno reconstructs the original kind on this same host/toolchain.
    OsCodeAndKind,
    /// Original kind and positively observed absence of custom payload survive.
    PayloadFree,
    /// Original kind survives with an explicit typed marker for lost custom payload/chain.
    OpaqueCustomLoss,
    /// The dependency's public non-I/O variant and typed payload are reconstructed.
    DependencyVariant,
}

/// A safe public-constructor projection, accompanied by its explicit preservation limit.
#[derive(Debug)]
pub(super) enum ProjectedStartupCause {
    Io {
        error: io::Error,
        fidelity: ProjectionFidelity,
    },
    Seccompiler {
        error: seccompiler::Error,
        fidelity: ProjectionFidelity,
    },
}

/// Representation failures are typed; callers must not classify them from diagnostic messages.
#[derive(Debug)]
pub(super) enum RepresentationError {
    InvalidContextBound,
    Reservation(TryReserveError),
    ContextExceeded,
    Formatting,
    UnnamedIoKind,
    OsKindMismatch,
    /// Required OS-derived metadata lacks the actual code it claims to preserve.
    MissingOsCode,
    /// Raw-OS and custom-payload metadata contradict the actual std I/O representation.
    OsPayloadMismatch,
    /// Backend compilation errors are not emitted by the actual apply_filter producer.
    NonInstallationBackendCause,
    /// Authenticated metadata does not describe the original error domain at its typed site.
    PolicyCauseMismatch,
}

impl fmt::Display for RepresentationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reservation(error) => write!(formatter, "startup context reservation: {error}"),
            other => write!(formatter, "startup cause representation: {other:?}"),
        }
    }
}

impl std::error::Error for RepresentationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Reservation(error) => Some(error),
            _ => None,
        }
    }
}

impl StartupCause {
    /// Capture required original I/O metadata even when optional context storage is absent.
    /// This operation allocates no diagnostic or source clone and grants no producer authority.
    pub(super) fn capture_io(error: &io::Error) -> Result<Self, RepresentationError> {
        StartupIoCause::capture(error).map(Self::Io)
    }

    /// Reconstruct only public, typed error shapes, without inventing an errno or source chain.
    pub(super) fn project(
        self,
        provenance: RemoteCauseProvenance,
    ) -> Result<ProjectedStartupCause, RepresentationError> {
        match self {
            Self::Io(cause) => {
                let (error, fidelity) = cause.project(provenance)?;
                Ok(ProjectedStartupCause::Io { error, fidelity })
            }
            Self::Seccompiler(cause) => {
                let (error, fidelity) = match cause {
                    StartupSeccompilerCause::Prctl(cause) => {
                        let (error, fidelity) = cause.project(provenance)?;
                        (seccompiler::Error::Prctl(error), fidelity)
                    }
                    StartupSeccompilerCause::Seccomp(cause) => {
                        let (error, fidelity) = cause.project(provenance)?;
                        (seccompiler::Error::Seccomp(error), fidelity)
                    }
                    StartupSeccompilerCause::EmptyFilter => (
                        seccompiler::Error::EmptyFilter,
                        ProjectionFidelity::DependencyVariant,
                    ),
                    StartupSeccompilerCause::ThreadSync { pid } => (
                        seccompiler::Error::ThreadSync(pid),
                        ProjectionFidelity::DependencyVariant,
                    ),
                };
                Ok(ProjectedStartupCause::Seccompiler { error, fidelity })
            }
        }
    }
}

/// Retained C-side context capacity is allocated fallibly before startup and reused by decoding.
/// The owner charges actual capacity and includes metadata/envelope bytes in its frame budget.
pub(super) struct PreparedStartupContext {
    text: String,
    limit: usize,
    encoded_max: usize,
    metadata_fault: Option<CauseIntegrityPredicate>,
}

impl PreparedStartupContext {
    /// Reserve a caller-budgeted context bound. JSON escaping uses at most six bytes per UTF-8
    /// byte, plus two quotes; this bound alone never permits a complete envelope above the cap.
    pub(super) fn new(limit: usize) -> Result<Self, RepresentationError> {
        let encoded_max = limit
            .checked_mul(6)
            .and_then(|bytes| bytes.checked_add(2))
            .filter(|bytes| *bytes <= CONTROL_BYTES)
            .ok_or(RepresentationError::InvalidContextBound)?;
        let mut text = String::new();
        text.try_reserve_exact(limit)
            .map_err(RepresentationError::Reservation)?;
        Ok(Self {
            text,
            limit,
            encoded_max,
            metadata_fault: None,
        })
    }

    /// Actual retained allocation, which may exceed the requested reservation.
    pub(super) fn reserved_bytes(&self) -> usize {
        self.text.capacity()
    }

    /// Worst-case JSON bytes for this context, excluding the owner's complete envelope.
    pub(super) fn encoded_context_max(&self) -> usize {
        self.encoded_max
    }

    /// Borrow the original bounded Display retained separately from typed metadata.
    pub(super) fn context(&self) -> &str {
        &self.text
    }

    /// Exact original UTF-8 as a borrowed JSON byte sequence. Dynamic context then requires no
    /// JSON string-unescaping scratch; the whole envelope still needs separate lexical bounds.
    pub(super) fn context_bytes(&self) -> &[u8] {
        self.text.as_bytes()
    }

    pub(super) fn clear(&mut self) {
        self.text.clear();
        self.metadata_fault = None;
    }

    /// Start a new required-metadata check without discarding a retained original diagnostic.
    /// A terminal/phase receipt has no context field and must not erase the pending I refusal.
    /// An actual context field clears/reuses the text through decode_context instead.
    pub(super) fn begin_metadata_frame(&mut self) {
        self.metadata_fault = None;
    }

    /// Borrow the first required-metadata checking fault slot. Optional diagnostic retention
    /// never writes or clears this fact; a new whole decode resets it explicitly.
    pub(super) fn metadata_fault_slot(&mut self) -> &mut Option<CauseIntegrityPredicate> {
        &mut self.metadata_fault
    }

    /// Carry the actual selected fixed decoder source without constructing a serde error.
    /// Only a fact recorded by its required-metadata schema supplies an integrity predicate;
    /// frame/input/reservation owners separately handle errors with no such fact.
    pub(super) fn grammar_error(
        &self,
        source: super::guardian_decode::DecodeError,
    ) -> super::control::ControlError {
        match self.metadata_fault {
            Some(predicate) => {
                super::control::ControlError::CauseMetadataGrammar { predicate, source }
            }
            None => super::control::ControlError::InvalidGrammar(source),
        }
    }

    /// Preserve the actual moved decoder error and observed predicate. No error text is parsed,
    /// and an error outside the required-cause seed remains an ordinary encoding refusal.
    pub(super) fn metadata_error(&self, source: serde_json::Error) -> super::control::ControlError {
        match self.metadata_fault {
            Some(predicate) => super::control::ControlError::CauseMetadata {
                // Only a source EOF encountered inside the required-cause seed supplies this
                // category. Ordinary framed transport/other envelope errors stay separate.
                predicate: if source.is_eof() {
                    CauseIntegrityPredicate::IncompleteCauseMetadata
                } else {
                    predicate
                },
                source,
            },
            None => super::control::ControlError::InvalidEncoding(source),
        }
    }

    /// Required byte-array grammar is independent of optional UTF-8 diagnostic rendering.
    /// Consume every typed byte after diagnostic omission, using only the original reservation.
    pub(super) fn decode_context(
        &mut self,
        decoder: &mut super::guardian_decode::Decoder<'_, '_>,
    ) -> Result<(), super::guardian_decode::DecodeError> {
        self.text.clear();
        let mut array = decoder.begin_array()?;
        let mut staged = [0_u8; 4];
        let mut length = 0_usize;
        let mut omitted = false;
        while decoder.next_element(&mut array)? {
            // Width/type/array structure remains required even after text is discarded.
            let byte = decoder.byte()?;
            if omitted {
                continue;
            }
            let Some(slot) = staged.get_mut(length) else {
                self.text.clear();
                omitted = true;
                continue;
            };
            *slot = byte;
            length = length.checked_add(1).ok_or_else(|| {
                super::guardian_decode::DecodeError::new(
                    super::guardian_decode::DecodeSite::Storage,
                    super::guardian_decode::DecodeCause::StorageBound,
                )
            })?;
            let Some(current) = staged.get(..length) else {
                self.text.clear();
                omitted = true;
                continue;
            };
            match std::str::from_utf8(current) {
                Ok(text)
                    if self
                        .text
                        .len()
                        .checked_add(text.len())
                        .is_some_and(|final_length| {
                            final_length <= self.limit && final_length <= self.text.capacity()
                        }) =>
                {
                    self.text.push_str(text);
                    length = 0;
                }
                Err(error) if error.error_len().is_none() && length < staged.len() => {}
                Ok(_) | Err(_) => {
                    self.text.clear();
                    omitted = true;
                }
            }
        }
        if length != 0 || omitted {
            self.text.clear();
        }
        Ok(())
    }

    /// Capture the original error without allocating its Display into an intermediate String.
    pub(super) fn capture_io(
        &mut self,
        error: &io::Error,
    ) -> Result<StartupCause, RepresentationError> {
        self.text.clear();
        let cause = StartupCause::capture_io(error)?;
        self.capture_optional_display(error);
        Ok(cause)
    }

    /// Capture optional diagnostics only after the owner has retained its typed cause.
    /// Formatting or retention failure omits text and never replaces that cause.
    pub(super) fn capture_context(&mut self, error: &impl fmt::Display) {
        self.text.clear();
        self.capture_optional_display(error);
    }

    /// Capture actual public installation variants exhaustively. Backend compilation errors
    /// are explicitly outside apply_filter, rather than reconstructed from their Display.
    pub(super) fn capture_seccompiler(
        &mut self,
        error: &seccompiler::Error,
    ) -> Result<StartupCause, RepresentationError> {
        self.text.clear();
        let cause = match error {
            seccompiler::Error::EmptyFilter => StartupSeccompilerCause::EmptyFilter,
            seccompiler::Error::Prctl(error) => {
                StartupSeccompilerCause::Prctl(StartupIoCause::capture(error)?)
            }
            seccompiler::Error::Seccomp(error) => {
                StartupSeccompilerCause::Seccomp(StartupIoCause::capture(error)?)
            }
            seccompiler::Error::ThreadSync(pid) => {
                StartupSeccompilerCause::ThreadSync { pid: *pid }
            }
            seccompiler::Error::Backend(_) => {
                return Err(RepresentationError::NonInstallationBackendCause)
            }
        };
        self.capture_optional_display(error);
        Ok(StartupCause::Seccompiler(cause))
    }

    fn capture_optional_display(&mut self, error: &impl fmt::Display) {
        // FR-034 cross-role representation: diagnostics are optional only AFTER required
        // metadata has been captured. capture_display clears partial text on either failure.
        if self.capture_display(error).is_err() {
            self.text.clear();
        }
    }

    fn capture_display(&mut self, error: &impl fmt::Display) -> Result<(), RepresentationError> {
        use fmt::Write;
        let mut writer = BoundedWriter {
            context: self,
            exceeded: false,
        };
        if write!(writer, "{error}").is_err() {
            let exceeded = writer.exceeded;
            writer.context.text.clear();
            return Err(if exceeded {
                RepresentationError::ContextExceeded
            } else {
                RepresentationError::Formatting
            });
        }
        Ok(())
    }

    /// Integrate this seed into the owner's authenticated envelope decoder. It reuses retained
    /// storage; a JSON decoder may still need separately charged transient unescaping scratch.
    pub(super) fn seed(&mut self) -> StartupContextSeed<'_> {
        self.text.clear();
        StartupContextSeed(self)
    }

    /// Decode required byte-array structure into the retained optional diagnostic. At most four
    /// UTF-8 bytes are staged on the stack. Invalid/incomplete UTF-8 or retention excess drops
    /// the whole diagnostic; every remaining member must still decode as u8.
    pub(super) fn bytes_seed(&mut self) -> StartupBytesSeed<'_> {
        self.text.clear();
        StartupBytesSeed(self)
    }
}

struct BoundedWriter<'a> {
    context: &'a mut PreparedStartupContext,
    exceeded: bool,
}

impl fmt::Write for BoundedWriter<'_> {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        if text.len() > self.context.limit.saturating_sub(self.context.text.len()) {
            self.exceeded = true;
            return Err(fmt::Error);
        }
        self.context.text.push_str(text);
        Ok(())
    }
}

/// A context-only seed; the role owner validates metadata, framing and provenance separately.
pub(super) struct StartupContextSeed<'a>(&'a mut PreparedStartupContext);

impl<'de> de::DeserializeSeed<'de> for StartupContextSeed<'_> {
    type Value = ();

    fn deserialize<D: de::Deserializer<'de>>(self, deserializer: D) -> Result<(), D::Error> {
        deserializer.deserialize_str(self)
    }
}

impl<'de> de::Visitor<'de> for StartupContextSeed<'_> {
    type Value = ();

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("UTF-8 startup context within its pre-reserved bound")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<(), E> {
        if value.len() > self.0.limit {
            return Err(E::custom(RepresentationError::ContextExceeded));
        }
        self.0.text.clear();
        self.0.text.push_str(value);
        Ok(())
    }
}

pub(super) struct StartupBytesSeed<'a>(&'a mut PreparedStartupContext);

impl<'de> de::DeserializeSeed<'de> for StartupBytesSeed<'_> {
    type Value = ();

    fn deserialize<D: de::Deserializer<'de>>(self, deserializer: D) -> Result<(), D::Error> {
        deserializer.deserialize_seq(self)
    }
}

impl<'de> de::Visitor<'de> for StartupBytesSeed<'_> {
    type Value = ();

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("bounded original UTF-8 startup context as bytes")
    }

    fn visit_seq<A: de::SeqAccess<'de>>(self, mut sequence: A) -> Result<(), A::Error> {
        let mut staged = [0_u8; 4];
        let mut length = 0_usize;
        let mut omitted = false;
        while let Some(byte) = sequence.next_element::<u8>()? {
            // Even after rendering fails, SeqAccess continues to enforce every required u8
            // and enclosing-array boundary. A malformed element never becomes omitted text.
            if omitted {
                continue;
            }
            let Some(slot) = staged.get_mut(length) else {
                self.0.text.clear();
                omitted = true;
                continue;
            };
            *slot = byte;
            length += 1; // At most the four-byte staging array, established by get_mut above.
            let Some(current) = staged.get(..length) else {
                self.0.text.clear();
                omitted = true;
                continue;
            };
            match std::str::from_utf8(current) {
                Ok(text) if text.len() <= self.0.limit.saturating_sub(self.0.text.len()) => {
                    self.0.text.push_str(text);
                    length = 0;
                }
                Err(error) if error.error_len().is_none() && length < staged.len() => {}
                Ok(_) | Err(_) => {
                    self.0.text.clear();
                    omitted = true;
                }
            }
        }
        if length != 0 || omitted {
            self.0.text.clear();
        }
        Ok(())
    }
}

/// Scratch-excluding admission of the private startup envelope's canonical producer subset.
/// The producer emits only static unescaped string labels, integral metadata and UTF-8 context
/// as a byte sequence. This is not a replacement JSON/schema decoder: serde still validates
/// exact fields/types and trailing input after this finite allocation-free preflight.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum StartupJsonError {
    EncodedBound,
    EscapedString,
    NonIntegralNumber,
    IntegerOverflow,
    UnclosedString,
}

impl fmt::Display for StartupJsonError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "private startup JSON refused: {self:?}")
    }
}

/// Prevent serde_json's private scratch Vec from allocating for escaped strings or malformed
/// numeric overflow/float paths, including dependency feature unification with float_roundtrip.
/// Ordinary unescaped slice strings and checked integral tokens use its borrowed/stack paths.
/// This preflight does not bound serde's private error-message allocation. Retained decoder
/// errors are named control storage; their replacement/bound remains an integration obligation.
pub(super) fn check_scratch_free_json(bytes: &[u8]) -> Result<(), StartupJsonError> {
    if bytes.is_empty() || bytes.len() > CONTROL_BYTES {
        return Err(StartupJsonError::EncodedBound);
    }
    let mut string = false;
    let mut integer: Option<u64> = None;
    for byte in bytes {
        if string {
            match byte {
                b'\\' => return Err(StartupJsonError::EscapedString),
                b'"' => string = false,
                _ => {}
            }
            continue;
        }
        match byte {
            b'"' => {
                string = true;
                integer = None;
            }
            b'0'..=b'9' => {
                integer = Some(
                    integer
                        .unwrap_or(0)
                        .checked_mul(10)
                        .and_then(|value| value.checked_add(u64::from(*byte - b'0')))
                        .ok_or(StartupJsonError::IntegerOverflow)?,
                );
            }
            b'.' | b'e' | b'E' => return Err(StartupJsonError::NonIntegralNumber),
            _ => integer = None,
        }
    }
    if string {
        return Err(StartupJsonError::UnclosedString);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kani::run::cross_role_cause::{CauseOperation, RemoteCauseRole};

    // The criterion's finite list is independent of the production macro: removing or
    // coercing a production kind must fail this real std-I/O capture/project check. This does
    // not prove the separate exhaustive source-flow gate for every actual sender operation.
    /// Trace: FR-034-AC-15, FR-034-AC-40.
    #[test]
    fn fixed_context_omission_preserves_required_byte_grammar_and_actual_capacity() {
        use super::super::guardian_decode::{DecodeCause, Decoder, Scratch};
        let mut scratch = Scratch::default();
        let mut context = PreparedStartupContext::new(2).unwrap();
        let capacity = context.reserved_bytes();
        let mut decoder = Decoder::new(b"[195,169]", &mut scratch).unwrap();
        context.decode_context(&mut decoder).unwrap();
        decoder.finish().unwrap();
        assert_eq!(context.context(), "é");
        assert_eq!(context.reserved_bytes(), capacity);
        for input in [b"[226,130,172]".as_slice(), b"[226,130]", b"[255,97]"] {
            let mut decoder = Decoder::new(input, &mut scratch).unwrap();
            context.decode_context(&mut decoder).unwrap();
            decoder.finish().unwrap();
            assert_eq!(context.context(), "");
            assert!(context.metadata_fault_slot().is_none());
            assert_eq!(context.reserved_bytes(), capacity);
        }
        for (input, cause) in [
            (b"[255,300]".as_slice(), DecodeCause::IntegerOverflow),
            (b"[255,".as_slice(), DecodeCause::UnexpectedEnd),
            (b"[255,null]".as_slice(), DecodeCause::InvalidNumber),
        ] {
            let mut decoder = Decoder::new(input, &mut scratch).unwrap();
            assert_eq!(
                context.decode_context(&mut decoder).unwrap_err().cause(),
                cause
            );
            assert_eq!(context.reserved_bytes(), capacity);
        }
        // This is the required-array/optional-rendering primitive only, not an authenticated
        // original cause, actual producer-kind gate or whole error/settlement acceptance.
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn all_declared_no_errno_kinds_capture_actual_std_values_without_payload_or_normalization() {
        let kinds = [
            io::ErrorKind::NotFound,
            io::ErrorKind::PermissionDenied,
            io::ErrorKind::ConnectionRefused,
            io::ErrorKind::ConnectionReset,
            io::ErrorKind::HostUnreachable,
            io::ErrorKind::NetworkUnreachable,
            io::ErrorKind::ConnectionAborted,
            io::ErrorKind::NotConnected,
            io::ErrorKind::AddrInUse,
            io::ErrorKind::AddrNotAvailable,
            io::ErrorKind::NetworkDown,
            io::ErrorKind::BrokenPipe,
            io::ErrorKind::AlreadyExists,
            io::ErrorKind::WouldBlock,
            io::ErrorKind::NotADirectory,
            io::ErrorKind::IsADirectory,
            io::ErrorKind::DirectoryNotEmpty,
            io::ErrorKind::ReadOnlyFilesystem,
            io::ErrorKind::StaleNetworkFileHandle,
            io::ErrorKind::InvalidInput,
            io::ErrorKind::InvalidData,
            io::ErrorKind::TimedOut,
            io::ErrorKind::WriteZero,
            io::ErrorKind::StorageFull,
            io::ErrorKind::NotSeekable,
            io::ErrorKind::QuotaExceeded,
            io::ErrorKind::FileTooLarge,
            io::ErrorKind::ResourceBusy,
            io::ErrorKind::ExecutableFileBusy,
            io::ErrorKind::Deadlock,
            io::ErrorKind::CrossesDevices,
            io::ErrorKind::TooManyLinks,
            io::ErrorKind::InvalidFilename,
            io::ErrorKind::ArgumentListTooLong,
            io::ErrorKind::Interrupted,
            io::ErrorKind::Unsupported,
            io::ErrorKind::UnexpectedEof,
            io::ErrorKind::OutOfMemory,
            io::ErrorKind::Other,
        ];
        for kind in kinds {
            let original = io::Error::from(kind);
            assert_eq!(original.raw_os_error(), None);
            assert!(original.get_ref().is_none());
            let captured = StartupCause::capture_io(&original).unwrap();
            let encoded = serde_json::to_vec(&captured).unwrap();
            let decoded: StartupCause = serde_json::from_slice(&encoded).unwrap();
            let ProjectedStartupCause::Io { error, fidelity } =
                decoded.project(projection_origin()).unwrap()
            else {
                panic!("actual IO changed producer domain")
            };
            assert_eq!(error.kind(), kind);
            assert_eq!(error.raw_os_error(), None);
            assert!(error.get_ref().is_none());
            assert_eq!(fidelity, ProjectionFidelity::PayloadFree);
        }
    }

    fn projection_origin() -> RemoteCauseProvenance {
        RemoteCauseProvenance {
            role: RemoteCauseRole::BackendInstaller,
            operation: CauseOperation::NativePolicyPreparation,
        }
    }

    /// Trace: FR-034-AC-15, FR-034-AC-39
    #[test]
    fn original_payload_presence_is_required_and_independent_from_kind_and_display() {
        let mut context = PreparedStartupContext::new(0).unwrap();
        let payload_free = io::Error::from(io::ErrorKind::InvalidData);
        let custom = io::Error::new(io::ErrorKind::InvalidData, "actual custom payload");
        let os = io::Error::from_raw_os_error(nix::libc::EPERM);
        let free = context.capture_io(&payload_free).unwrap();
        let lost = context.capture_io(&custom).unwrap();
        let original_os = context.capture_io(&os).unwrap();
        let StartupCause::Io(free_facts) = free else {
            panic!("payload-free cause changed domain")
        };
        let StartupCause::Io(lost_facts) = lost else {
            panic!("custom cause changed domain")
        };
        let StartupCause::Io(os_facts) = original_os else {
            panic!("OS cause changed domain")
        };
        assert_eq!(free_facts.payload(), StartupPayload::NoCustomPayload);
        assert_eq!(lost_facts.payload(), StartupPayload::UnrepresentedCustom);
        assert_eq!(os_facts.payload(), StartupPayload::NoCustomPayload);
        assert_eq!(free_facts.kind, lost_facts.kind);
        assert!(context.context().is_empty());
        let encoded = serde_json::to_value(lost).unwrap();
        assert_eq!(
            serde_json::from_value::<StartupCause>(encoded.clone()).unwrap(),
            lost
        );
        for malformed in [
            serde_json::Value::Null,
            serde_json::json!("unknown"),
            serde_json::json!(1),
        ] {
            let mut value = encoded.clone();
            value["Io"]["payload"] = malformed;
            assert!(serde_json::from_value::<StartupCause>(value).is_err());
        }
        assert!(encoded["Io"]["raw_os_error"].is_null());
        for malformed in [
            serde_json::json!("1"),
            serde_json::json!(true),
            serde_json::json!([]),
            serde_json::json!(i64::from(i32::MAX) + 1),
        ] {
            let mut value = encoded.clone();
            value["Io"]["raw_os_error"] = malformed;
            assert!(serde_json::from_value::<StartupCause>(value).is_err());
        }
        let mut missing_errno = encoded.clone();
        missing_errno["Io"]
            .as_object_mut()
            .unwrap()
            .remove("raw_os_error");
        assert!(serde_json::from_value::<StartupCause>(missing_errno).is_err());
        let mut missing = encoded;
        missing["Io"].as_object_mut().unwrap().remove("payload");
        assert!(serde_json::from_value::<StartupCause>(missing).is_err());
    }

    /// Trace: FR-034-AC-15, FR-034-AC-39
    #[test]
    fn retained_os_cause_survives_optional_display_bound_and_formatting_failure() {
        let original = io::Error::from_raw_os_error(nix::libc::EPERM);
        let mut context = PreparedStartupContext::new(0).unwrap();
        let reserved = context.reserved_bytes();
        let cause = context.capture_io(&original).unwrap();
        assert!(context.context().is_empty());
        let ProjectedStartupCause::Io { error, fidelity } =
            cause.project(projection_origin()).unwrap()
        else {
            panic!("diagnostic retention changed the original domain");
        };
        assert_eq!(error.raw_os_error(), original.raw_os_error());
        assert_eq!(error.kind(), original.kind());
        assert_eq!(fidelity, ProjectionFidelity::OsCodeAndKind);
        assert_eq!(context.reserved_bytes(), reserved);

        #[derive(Debug)]
        struct FormattingFailure;
        impl fmt::Display for FormattingFailure {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("partial diagnostic")?;
                Err(fmt::Error)
            }
        }
        impl std::error::Error for FormattingFailure {}
        let mut context = PreparedStartupContext::new(64).unwrap();
        let reserved = context.reserved_bytes();
        let cause = context.capture_io(&original).unwrap();
        assert!(!context.context().is_empty());
        context.capture_context(&FormattingFailure);
        assert!(context.context().is_empty());
        assert_eq!(context.reserved_bytes(), reserved);
        let ProjectedStartupCause::Io { error, .. } = cause.project(projection_origin()).unwrap()
        else {
            panic!("diagnostic formatting changed retained cause metadata");
        };
        assert_eq!(error.raw_os_error(), Some(nix::libc::EPERM));
        let original = io::Error::new(io::ErrorKind::PermissionDenied, FormattingFailure);
        let cause = context.capture_io(&original).unwrap();
        assert!(context.context().is_empty());
        let ProjectedStartupCause::Io { error, fidelity } =
            cause.project(projection_origin()).unwrap()
        else {
            panic!("optional formatter failure changed required metadata");
        };
        assert_eq!(error.kind(), original.kind());
        assert_eq!(error.raw_os_error(), None);
        assert_eq!(fidelity, ProjectionFidelity::OpaqueCustomLoss);
        let StartupCause::Io(metadata) = cause else {
            panic!("formatter failure changed captured IO metadata");
        };
        assert_eq!(metadata.payload(), StartupPayload::UnrepresentedCustom);
        let payload_free = io::Error::from(io::ErrorKind::InvalidData);
        let StartupCause::Io(metadata) = context.capture_io(&payload_free).unwrap() else {
            panic!("payload-free capture changed IO metadata");
        };
        assert_eq!(metadata.payload(), StartupPayload::NoCustomPayload);
        assert_eq!(context.reserved_bytes(), reserved);
    }
}
