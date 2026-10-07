//! Private typed startup-cause metadata and separately charged, reusable UTF-8 context.
//!
//! These values carry no site, capability, sender or admission authority. The role owner binds
//! those facts and budgets the complete control envelope. Serialization cannot preserve arbitrary
//! concrete I/O source chains or downcast identity. Projection states that loss explicitly.
//! OS-derived kinds and native seccompiler PID payloads are a same-host/toolchain transport,
//! not a portable persisted error format.

#![cfg(target_os = "linux")]

use std::{collections::TryReserveError, fmt, io};

use serde::de::Error as _;
use serde::{de, Deserialize, Serialize};

use super::control::CONTROL_BYTES;

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

/// Original kind representation and optional original OS error; neither comes from Display.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct StartupIoCause {
    kind: StartupIoKind,
    raw_os_error: Option<i32>,
}

impl StartupIoCause {
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
        Ok(Self { kind, raw_os_error })
    }

    fn project(self) -> Result<(io::Error, ProjectionFidelity), RepresentationError> {
        let kind = self.kind.named();
        match self.raw_os_error {
            Some(errno) => {
                let error = io::Error::from_raw_os_error(errno);
                if kind.is_some_and(|kind| error.kind() != kind) {
                    return Err(RepresentationError::OsKindMismatch);
                }
                Ok((error, ProjectionFidelity::OsCodeAndKind))
            }
            None => Ok((
                io::Error::from(kind.ok_or(RepresentationError::UnnamedIoKind)?),
                ProjectionFidelity::KindOnly,
            )),
        }
    }
}

/// Actual apply_filter installation causes, distinct from context and role/site selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) enum StartupSeccompilerCause {
    EmptyFilter,
    Prctl(StartupIoCause),
    Seccomp(StartupIoCause),
    ThreadSync { pid: std::os::raw::c_long },
}

/// Allocation-free metadata. The original Display is retained separately in prepared storage.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) enum StartupCause {
    Io(StartupIoCause),
    Seccompiler(StartupSeccompilerCause),
}

/// Projection fidelity is never a claim that an arbitrary original source chain survived.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ProjectionFidelity {
    /// Original errno reconstructs the original kind on this same host/toolchain.
    OsCodeAndKind,
    /// Original kind survives; its custom payload and concrete source chain do not.
    KindOnly,
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
    /// Reconstruct only public, typed error shapes, without inventing an errno or source chain.
    pub(super) fn project(self) -> Result<ProjectedStartupCause, RepresentationError> {
        match self {
            Self::Io(cause) => {
                let (error, fidelity) = cause.project()?;
                Ok(ProjectedStartupCause::Io { error, fidelity })
            }
            Self::Seccompiler(cause) => {
                let (error, fidelity) = match cause {
                    StartupSeccompilerCause::Prctl(cause) => {
                        let (error, fidelity) = cause.project()?;
                        (seccompiler::Error::Prctl(error), fidelity)
                    }
                    StartupSeccompilerCause::Seccomp(cause) => {
                        let (error, fidelity) = cause.project()?;
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
    }

    /// Capture the original error without allocating its Display into an intermediate String.
    pub(super) fn capture_io(
        &mut self,
        error: &io::Error,
    ) -> Result<StartupCause, RepresentationError> {
        self.text.clear();
        let cause = StartupIoCause::capture(error)?;
        self.capture_optional_display(error);
        Ok(StartupCause::Io(cause))
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
/// Error-formatting/runtime internals remain opaque incidental allocations, not caller buffers.
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

    /// Trace: FR-034-AC-15, FR-034-AC-39
    #[test]
    fn retained_os_cause_survives_optional_display_bound_and_formatting_failure() {
        let original = io::Error::from_raw_os_error(nix::libc::EPERM);
        let mut context = PreparedStartupContext::new(0).unwrap();
        let reserved = context.reserved_bytes();
        let cause = context.capture_io(&original).unwrap();
        assert!(context.context().is_empty());
        let ProjectedStartupCause::Io { error, fidelity } = cause.project().unwrap() else {
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
        let ProjectedStartupCause::Io { error, .. } = cause.project().unwrap() else {
            panic!("diagnostic formatting changed retained cause metadata");
        };
        assert_eq!(error.raw_os_error(), Some(nix::libc::EPERM));
        let original = io::Error::new(io::ErrorKind::PermissionDenied, FormattingFailure);
        let cause = context.capture_io(&original).unwrap();
        assert!(context.context().is_empty());
        let ProjectedStartupCause::Io { error, fidelity } = cause.project().unwrap() else {
            panic!("optional formatter failure changed required metadata");
        };
        assert_eq!(error.kind(), original.kind());
        assert_eq!(error.raw_os_error(), None);
        assert_eq!(fidelity, ProjectionFidelity::KindOnly);
        assert_eq!(context.reserved_bytes(), reserved);
    }
}
