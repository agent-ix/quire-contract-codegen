//! Private bounded Unix-stream framing and owned ancillary transport (FR-034).
//!
//! Only this module creates the anonymous pair. Fresh Linux AF_UNIX STREAM sockets have no
//! PASSSEC, PASSPIDFD, INQ or timestamp options; enabling only PASSCRED leaves credentials and
//! rights as the kernel-produced observable ancillary set. No caller-configured socket enters
//! this transport. Linux rejects unknown SOL_SOCKET send controls and erases other levels before
//! delivery. Erased controls are not received metadata. This restriction matters because rustix's
//! safe drain exposes owned rights and credentials, but skips unknown control types.

use std::{
    io::{self, IoSlice, IoSliceMut, Write},
    mem::MaybeUninit,
    os::fd::{AsFd, BorrowedFd, OwnedFd},
    time::{Duration, Instant},
};

use rustix::{
    event::{poll, PollFd, PollFlags},
    net::{
        recvmsg, sendmsg, socketpair, AddressFamily, RecvAncillaryBuffer, RecvAncillaryMessage,
        RecvFlags, ReturnFlags, SendAncillaryBuffer, SendAncillaryMessage, SendFlags, SocketFlags,
        SocketType,
    },
    time::Timespec,
};
use serde::{de::DeserializeOwned, Serialize};

const CONTROL_BYTES: usize = 65_536;
const RECEIVED_RIGHTS: usize = 4;
const ANCILLARY_BYTES: usize = rustix::cmsg_space!(ScmRights(RECEIVED_RIGHTS), ScmCredentials(1));

/// Exclusive original-caller liveness endpoint. Dropping it cannot drop monitor ownership.
pub(super) struct CallerLease(OwnedFd);

/// Guardian endpoint, mapped into only the intended child; never given to a backend.
pub(super) struct GuardianEndpoint(OwnedFd);

/// Separate role/control owner; it is never the original guardian lease writer.
pub(super) struct RoleCaller(OwnedFd);

/// Child side of an anonymous role pair, either mapped into entry stdin or safely received.
pub(super) struct RoleEndpoint(OwnedFd);

/// Bounded bootstrap reader with no authenticated-role authority yet.
pub(super) struct RoleEntry(OwnedFd);

/// A refusal at the bounded framing/descriptor boundary.
#[derive(Debug)]
pub(super) enum ControlError {
    Io(io::Error),
    Eof,
    Deadline,
    EncodedBytesExceeded,
    InvalidEncoding(serde_json::Error),
    Truncated,
    MissingCredentials,
    RepeatedCredentials,
    ChangedCredentials,
    UnknownAncillary,
    ExcessRights,
    RightsNotCloexec,
    UnexpectedCredentials,
    CreatorMismatch,
    ProgressPoisoned,
    PartialTerminalSend { written: usize, expected: usize },
    TrailingTerminalBytes,
    RightsCount { expected: usize, received: usize },
}

impl From<rustix::io::Errno> for ControlError {
    fn from(error: rustix::io::Errno) -> Self {
        Self::Io(error.into())
    }
}

impl From<io::Error> for ControlError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl std::fmt::Display for ControlError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "guardian control I/O: {error}"),
            Self::InvalidEncoding(error) => write!(formatter, "guardian control encoding: {error}"),
            Self::RightsCount { expected, received } => write!(
                formatter,
                "guardian control expected {expected} descriptor(s), received {received}"
            ),
            other => write!(formatter, "{other:?}"),
        }
    }
}

impl std::error::Error for ControlError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::InvalidEncoding(error) => Some(error),
            _ => None,
        }
    }
}

/// Creates both endpoints before spawning. No listener, address or option-setting escape exists.
pub(super) fn private_pair() -> Result<(CallerLease, GuardianEndpoint), ControlError> {
    let (caller, guardian) = fresh_pair()?;
    Ok((CallerLease(caller), GuardianEndpoint(guardian)))
}

/// Fresh role pairs have the same closed option authority, independently of the original lease.
pub(super) fn role_pair() -> Result<(RoleCaller, RoleEndpoint), ControlError> {
    let (caller, endpoint) = fresh_pair()?;
    Ok((RoleCaller(caller), RoleEndpoint(endpoint)))
}

fn fresh_pair() -> Result<(OwnedFd, OwnedFd), ControlError> {
    let (caller, guardian) = socketpair(
        AddressFamily::UNIX,
        SocketType::STREAM,
        SocketFlags::CLOEXEC | SocketFlags::NONBLOCK,
        None,
    )?;
    rustix::net::sockopt::set_socket_passcred(&caller, true)?;
    Ok((caller, guardian))
}

impl CallerLease {
    pub(super) fn transport(&self) -> Transport<'_> {
        Transport(self.0.as_fd(), CredentialsPolicy::ActualSender)
    }
}

impl GuardianEndpoint {
    pub(super) fn into_child_mapping(self) -> OwnedFd {
        self.0
    }

    pub(super) fn from_received(
        descriptor: OwnedFd,
        actual_creator: &OwnedFd,
    ) -> Result<Self, ControlError> {
        validate_endpoint(&descriptor)?;
        Transport(descriptor.as_fd(), CredentialsPolicy::ExclusiveCreator)
            .authenticate_creator(actual_creator)?;
        Ok(Self(descriptor))
    }

    pub(super) fn transport(&self) -> Transport<'_> {
        Transport(self.0.as_fd(), CredentialsPolicy::ExclusiveCreator)
    }
}

impl AsFd for GuardianEndpoint {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.0.as_fd()
    }
}

impl RoleCaller {
    pub(super) fn transport(&self) -> Transport<'_> {
        Transport(self.0.as_fd(), CredentialsPolicy::ActualSender)
    }
}

impl RoleEndpoint {
    pub(super) fn into_child_mapping(self) -> OwnedFd {
        self.0
    }

    pub(super) fn from_received(
        descriptor: OwnedFd,
        actual_creator: &OwnedFd,
    ) -> Result<Self, ControlError> {
        validate_endpoint(&descriptor)?;
        Transport(descriptor.as_fd(), CredentialsPolicy::ExclusiveCreator)
            .authenticate_creator(actual_creator)?;
        Ok(Self(descriptor))
    }

    pub(super) fn transport(&self) -> Transport<'_> {
        Transport(self.0.as_fd(), CredentialsPolicy::ExclusiveCreator)
    }
}

impl AsFd for RoleEndpoint {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.0.as_fd()
    }
}

impl RoleEntry {
    /// The helper's known stdio mapping is safely duplicated; no raw inherited-fd adoption.
    /// The original slot is CLOEXEC before any role is allowed to create another process.
    pub(super) fn from_entry_stdin() -> Result<Self, ControlError> {
        let stdin = std::io::stdin();
        rustix::io::fcntl_setfd(&stdin, rustix::io::FdFlags::CLOEXEC)?;
        validate_endpoint(&stdin)?;
        let descriptor = rustix::io::fcntl_dupfd_cloexec(&stdin, 3)?;
        Ok(Self(descriptor))
    }

    /// Only bounded decoding is possible before independently checking actual creator authority.
    pub(super) fn receive<T: DeserializeOwned>(
        &self,
        expected_rights: fn(&T) -> usize,
        deadline: Instant,
    ) -> Result<Received<T>, ControlError> {
        Transport(self.0.as_fd(), CredentialsPolicy::ExclusiveCreator)
            .receive(expected_rights, deadline)
    }

    pub(super) fn authenticate(
        self,
        actual_creator: &OwnedFd,
    ) -> Result<RoleEndpoint, ControlError> {
        RoleEndpoint::from_received(self.0, actual_creator)
    }
}

fn validate_endpoint(descriptor: impl AsFd) -> Result<(), ControlError> {
    if rustix::net::sockopt::socket_domain(&descriptor)? != AddressFamily::UNIX
        || rustix::net::sockopt::socket_type(&descriptor)? != SocketType::STREAM
        || rustix::net::sockopt::socket_passcred(&descriptor)?
    {
        return Err(ControlError::UnexpectedCredentials);
    }
    if !rustix::io::fcntl_getfd(&descriptor)?.contains(rustix::io::FdFlags::CLOEXEC) {
        return Err(ControlError::RightsNotCloexec);
    }
    Ok(())
}

/// Safe borrowed helper control. Its only production inputs are the private pair's endpoints.
pub(super) struct Transport<'fd>(BorrowedFd<'fd>, CredentialsPolicy);

/// Only the final transaction may drain already-emitted bytes after O's expected normal exit.
/// Decoding under this policy proves neither that exit was normal nor whole-chain settlement.
#[derive(Clone, Copy)]
enum ReceiveEof {
    Refuse,
    DrainTerminal,
}

#[derive(Clone, Copy)]
enum CredentialsPolicy {
    ActualSender,
    ExclusiveCreator,
}

/// Kernel credentials without a nonzero-PID assumption. Outside creators can appear as PID0.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct PeerCredentials {
    pub(super) pid: i32,
    pub(super) uid: u32,
    pub(super) gid: u32,
}

impl<'fd> Transport<'fd> {
    #[cfg(feature = "guardian-test-support")]
    pub(super) fn fixture_reporter(descriptor: &'fd OwnedFd) -> Self {
        Self(descriptor.as_fd(), CredentialsPolicy::ExclusiveCreator)
    }

    /// The helper borrows its known-valid mapped stdin, never adopts an arbitrary raw number.
    pub(super) fn from_guardian_stdin(stdin: &'fd std::io::Stdin) -> Result<Self, ControlError> {
        rustix::io::fcntl_setfd(stdin, rustix::io::FdFlags::CLOEXEC)?;
        if rustix::net::sockopt::socket_passcred(stdin)? {
            return Err(ControlError::UnexpectedCredentials);
        }
        Ok(Self(stdin.as_fd(), CredentialsPolicy::ExclusiveCreator))
    }

    pub(super) fn creator_credentials(&self) -> Result<PeerCredentials, ControlError> {
        let credentials =
            nix::sys::socket::getsockopt(&self.0, nix::sys::socket::sockopt::PeerCredentials)
                .map_err(|error| ControlError::Io(io::Error::from(error)))?;
        Ok(PeerCredentials {
            pid: credentials.pid(),
            uid: credentials.uid(),
            gid: credentials.gid(),
        })
    }

    /// Kernel-minted peer authority works across namespaces even when peer PID is displayed as0.
    pub(super) fn authenticate_creator(
        &self,
        actual_creator: &OwnedFd,
    ) -> Result<(), ControlError> {
        let peer = nix::sys::socket::getsockopt(&self.0, nix::sys::socket::sockopt::PeerPidfd)
            .map_err(|error| ControlError::Io(io::Error::from(error)))?;
        let peer_identity = rustix::fs::fstat(&peer)?;
        let expected = rustix::fs::fstat(actual_creator)?;
        if peer_identity.st_dev != expected.st_dev || peer_identity.st_ino != expected.st_ino {
            return Err(ControlError::CreatorMismatch);
        }
        Ok(())
    }

    /// A finite supervision tick, with EOF checked before pending bytes.
    pub(super) fn pending_control(&self, interval: Duration) -> Result<bool, ControlError> {
        let mut events = [PollFd::from_borrowed_fd(
            self.0,
            PollFlags::IN | PollFlags::RDHUP,
        )];
        poll(&mut events, Some(&timespec(interval)?))?;
        self.refuse_observable_eof()?;
        Ok(events[0].revents().contains(PollFlags::IN))
    }

    /// Checks half-close before accepting a buffered authorization. Future death is not predicted.
    pub(super) fn refuse_observable_eof(&self) -> Result<(), ControlError> {
        let mut events = [PollFd::from_borrowed_fd(
            self.0,
            PollFlags::IN | PollFlags::RDHUP,
        )];
        poll(&mut events, Some(&Timespec::default()))?;
        if events[0]
            .revents()
            .intersects(PollFlags::HUP | PollFlags::RDHUP)
        {
            Err(ControlError::Eof)
        } else if events[0]
            .revents()
            .intersects(PollFlags::ERR | PollFlags::NVAL)
        {
            Err(ControlError::Io(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "guardian control socket failed",
            )))
        } else {
            Ok(())
        }
    }

    pub(super) fn send<T: Serialize>(
        &self,
        control: &T,
        rights: &[BorrowedFd<'_>],
        deadline: Instant,
    ) -> Result<(), ControlError> {
        if rights.len() > RECEIVED_RIGHTS {
            return Err(ControlError::ExcessRights);
        }
        let frame = PreparedFrame::encode(control)?;
        self.send_prepared(&frame, rights, deadline)
    }

    /// Send an already bounded/encoded frame without copying another whole control buffer.
    /// The run owner may prepare and charge this allocation before its first child is created.
    pub(super) fn send_prepared(
        &self,
        frame: &PreparedFrame,
        rights: &[BorrowedFd<'_>],
        deadline: Instant,
    ) -> Result<(), ControlError> {
        if rights.len() > RECEIVED_RIGHTS {
            return Err(ControlError::ExcessRights);
        }
        let bytes = &frame.bytes;
        let mut offset = 0;
        while offset < bytes.len() {
            self.wait(PollFlags::OUT, deadline)?;
            let current_rights = if offset == 0 { rights } else { &[] };
            if let Some(count) = self.send_chunk(&bytes[offset..], current_rights)? {
                offset = offset.checked_add(count).ok_or(ControlError::Truncated)?;
            }
        }
        Ok(())
    }

    /// Exactly one nonblocking syscall for the final metrics commit. No partial frame can resume
    /// with an earlier peak: the owner must refuse and exit abnormally after any partial write.
    /// A zero-progress AGAIN/INTR lets the owner run fresh accounting and refill the same storage.
    /// Checking a due tick and the original cutoff immediately before calling this method does
    /// not bound scheduler preemption or kernel execution time.
    pub(super) fn send_terminal_once(
        &self,
        frame: &PreparedFrame,
        deadline: Instant,
    ) -> Result<bool, ControlError> {
        if Instant::now() >= deadline {
            return Err(ControlError::Deadline);
        }
        match self.send_chunk(&frame.bytes, &[])? {
            None => Ok(false),
            Some(written) if written == frame.bytes.len() => Ok(true),
            Some(written) => Err(ControlError::PartialTerminalSend {
                written,
                expected: frame.bytes.len(),
            }),
        }
    }

    /// One actual nonblocking send. Both blocking and actor send modes share this ancillary
    /// encoding and syscall; rights accompany only the first positively written byte.
    fn send_chunk(
        &self,
        bytes: &[u8],
        rights: &[BorrowedFd<'_>],
    ) -> Result<Option<usize>, ControlError> {
        self.refuse_observable_eof()?;
        if rights.len() > RECEIVED_RIGHTS {
            return Err(ControlError::ExcessRights);
        }
        let mut storage = [MaybeUninit::uninit(); ANCILLARY_BYTES];
        let mut ancillary = SendAncillaryBuffer::new(&mut storage);
        if !rights.is_empty() && !ancillary.push(SendAncillaryMessage::ScmRights(rights)) {
            return Err(ControlError::ExcessRights);
        }
        let count = match sendmsg(
            self.0,
            &[IoSlice::new(bytes)],
            &mut ancillary,
            SendFlags::DONTWAIT | SendFlags::NOSIGNAL,
        ) {
            Ok(0) => return Err(ControlError::Eof),
            Ok(count) => count,
            Err(rustix::io::Errno::AGAIN | rustix::io::Errno::INTR) => return Ok(None),
            Err(error) => return Err(error.into()),
        };
        if count > bytes.len() {
            return Err(ControlError::Truncated);
        }
        self.refuse_observable_eof()?;
        Ok(Some(count))
    }

    pub(super) fn receive<T: DeserializeOwned>(
        &self,
        expected_rights: impl FnOnce(&T) -> usize,
        deadline: Instant,
    ) -> Result<Received<T>, ControlError> {
        let mut buffer = PreparedReceive::prepare()?;
        let received = self.receive_prepared(&mut buffer, expected_rights, deadline)?;
        Ok(Received {
            control: received.control,
            credentials: received.credentials,
            rights: std::mem::take(received.rights),
        })
    }

    /// C receives into storage whose actual capacity was reserved before L creation. Ancillary
    /// rights remain owned by that same storage until the caller explicitly takes each pin.
    pub(super) fn receive_prepared<'buffer, T: DeserializeOwned>(
        &self,
        buffer: &'buffer mut PreparedReceive,
        expected_rights: impl FnOnce(&T) -> usize,
        deadline: Instant,
    ) -> Result<PreparedReceived<'buffer, T>, ControlError> {
        buffer.rights.clear();
        let mut credentials = None;
        let mut header = [0; 4];
        self.read_exact(&mut header, &mut credentials, &mut buffer.rights, deadline)?;
        let length = usize::try_from(u32::from_be_bytes(header))
            .map_err(|_| ControlError::EncodedBytesExceeded)?;
        if length == 0 || length > CONTROL_BYTES {
            return Err(ControlError::EncodedBytesExceeded);
        }
        buffer.payload.resize(length, 0);
        self.read_exact(
            &mut buffer.payload,
            &mut credentials,
            &mut buffer.rights,
            deadline,
        )?;
        self.finish_receive(buffer, credentials, expected_rights)
    }

    fn finish_receive<'buffer, T: DeserializeOwned>(
        &self,
        buffer: &'buffer mut PreparedReceive,
        credentials: Option<PeerCredentials>,
        expected_rights: impl FnOnce(&T) -> usize,
    ) -> Result<PreparedReceived<'buffer, T>, ControlError> {
        self.finish_receive_mode(buffer, credentials, expected_rights, ReceiveEof::Refuse)
    }

    fn finish_receive_mode<'buffer, T: DeserializeOwned>(
        &self,
        buffer: &'buffer mut PreparedReceive,
        credentials: Option<PeerCredentials>,
        expected_rights: impl FnOnce(&T) -> usize,
        eof: ReceiveEof,
    ) -> Result<PreparedReceived<'buffer, T>, ControlError> {
        self.check_receive_state(eof)?;
        let control =
            serde_json::from_slice(&buffer.payload).map_err(ControlError::InvalidEncoding)?;
        let expected = expected_rights(&control);
        if expected > RECEIVED_RIGHTS {
            return Err(ControlError::ExcessRights);
        }
        if buffer.rights.len() != expected {
            return Err(ControlError::RightsCount {
                expected,
                received: buffer.rights.len(),
            });
        }
        if matches!(self.1, CredentialsPolicy::ActualSender) && credentials.is_none() {
            return Err(ControlError::MissingCredentials);
        }
        Ok(PreparedReceived {
            control,
            credentials,
            rights: &mut buffer.rights,
        })
    }

    fn read_exact(
        &self,
        mut bytes: &mut [u8],
        credentials: &mut Option<PeerCredentials>,
        rights: &mut Vec<OwnedFd>,
        deadline: Instant,
    ) -> Result<(), ControlError> {
        while !bytes.is_empty() {
            self.wait(PollFlags::IN, deadline)?;
            if let Some(count) = self.read_chunk(bytes, credentials, rights)? {
                bytes = bytes.get_mut(count..).ok_or(ControlError::Truncated)?;
            }
        }
        Ok(())
    }

    /// One nonblocking chunk with the same owned ancillary validation for both receive modes.
    /// EOF wins before and after the syscall, including queued but unauthorizable frame bytes.
    fn read_chunk(
        &self,
        bytes: &mut [u8],
        credentials: &mut Option<PeerCredentials>,
        rights: &mut Vec<OwnedFd>,
    ) -> Result<Option<usize>, ControlError> {
        self.read_chunk_mode(bytes, credentials, rights, ReceiveEof::Refuse)
    }

    fn read_chunk_mode(
        &self,
        bytes: &mut [u8],
        credentials: &mut Option<PeerCredentials>,
        rights: &mut Vec<OwnedFd>,
        eof: ReceiveEof,
    ) -> Result<Option<usize>, ControlError> {
        self.check_receive_state(eof)?;
        let mut storage = [MaybeUninit::uninit(); ANCILLARY_BYTES];
        let mut ancillary = RecvAncillaryBuffer::new(&mut storage);
        let received = match recvmsg(
            self.0,
            &mut [IoSliceMut::new(bytes)],
            &mut ancillary,
            RecvFlags::DONTWAIT | RecvFlags::CMSG_CLOEXEC,
        ) {
            Ok(received) => received,
            Err(rustix::io::Errno::AGAIN | rustix::io::Errno::INTR) => return Ok(None),
            Err(error) => return Err(error.into()),
        };
        // Even this early refusal drops/drains every safely owned delivered descriptor;
        // Linux closes any rights discarded because the ancillary buffer was too small.
        if received
            .flags
            .intersects(ReturnFlags::TRUNC | ReturnFlags::CTRUNC)
        {
            return Err(ControlError::Truncated);
        }
        if received.bytes == 0 {
            return Err(ControlError::Eof);
        }
        let mut chunk_credentials = None;
        for record in ancillary.drain() {
            match record {
                RecvAncillaryMessage::ScmCredentials(value) => {
                    if matches!(self.1, CredentialsPolicy::ExclusiveCreator) {
                        return Err(ControlError::UnexpectedCredentials);
                    }
                    let value = PeerCredentials {
                        pid: value.pid.as_raw_pid(),
                        uid: value.uid.as_raw(),
                        gid: value.gid.as_raw(),
                    };
                    if chunk_credentials.replace(value).is_some() {
                        return Err(ControlError::RepeatedCredentials);
                    }
                }
                RecvAncillaryMessage::ScmRights(descriptors) => {
                    for descriptor in descriptors {
                        if rights.len() == RECEIVED_RIGHTS {
                            return Err(ControlError::ExcessRights);
                        }
                        if !rustix::io::fcntl_getfd(&descriptor)?
                            .contains(rustix::io::FdFlags::CLOEXEC)
                        {
                            return Err(ControlError::RightsNotCloexec);
                        }
                        rights.push(descriptor);
                    }
                }
                _ => return Err(ControlError::UnknownAncillary),
            }
        }
        match self.1 {
            CredentialsPolicy::ActualSender => {
                let sender = chunk_credentials.ok_or(ControlError::MissingCredentials)?;
                if credentials.is_some_and(|previous| previous != sender) {
                    return Err(ControlError::ChangedCredentials);
                }
                *credentials = Some(sender);
            }
            CredentialsPolicy::ExclusiveCreator => {}
        }

        if received.bytes > bytes.len() {
            return Err(ControlError::Truncated);
        }
        self.check_receive_state(eof)?;
        Ok(Some(received.bytes))
    }

    fn check_receive_state(&self, eof: ReceiveEof) -> Result<(), ControlError> {
        if matches!(eof, ReceiveEof::Refuse) {
            return self.refuse_observable_eof();
        }
        let mut events = [PollFd::from_borrowed_fd(
            self.0,
            PollFlags::IN | PollFlags::RDHUP,
        )];
        poll(&mut events, Some(&Timespec::default()))?;
        if events[0]
            .revents()
            .intersects(PollFlags::ERR | PollFlags::NVAL)
        {
            return Err(ControlError::Io(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "terminal control socket failed",
            )));
        }
        // HUP/RDHUP is tolerated only to read a previously emitted final frame. recvmsg returning
        // zero before its declared length still refuses; normal O wait/reap is a separate gate.
        Ok(())
    }

    fn wait(&self, interest: PollFlags, deadline: Instant) -> Result<(), ControlError> {
        loop {
            let remaining = deadline
                .checked_duration_since(Instant::now())
                .ok_or(ControlError::Deadline)?;
            let timeout = timespec(remaining)?;
            let mut events = [PollFd::from_borrowed_fd(
                self.0,
                interest | PollFlags::RDHUP,
            )];
            match poll(&mut events, Some(&timeout)) {
                Ok(0) => return Err(ControlError::Deadline),
                Ok(_) => {
                    self.refuse_observable_eof()?;
                    return Ok(());
                }
                Err(rustix::io::Errno::INTR) => {}
                Err(error) => return Err(error.into()),
            }
        }
    }
}

pub(super) struct Received<T> {
    pub(super) control: T,
    pub(super) credentials: Option<PeerCredentials>,
    pub(super) rights: Vec<OwnedFd>,
}

/// Encoded controls have one finite owned buffer, including their existing four-byte framing.
/// No caller can mutate the encoded bytes or manufacture an unbounded prepared frame.
pub(super) struct PreparedFrame {
    bytes: Vec<u8>,
}

/// Unencoded bounded storage. It cannot be sent: serialization consumes it into PreparedFrame.
/// C can measure this actual allocation before filling the run's own charge in its bootstrap.
pub(super) struct FrameStorage {
    bytes: Vec<u8>,
}

impl FrameStorage {
    pub(super) fn prepare() -> Result<Self, ControlError> {
        let limit = CONTROL_BYTES
            .checked_add(4)
            .ok_or(ControlError::EncodedBytesExceeded)?;
        let mut bytes = Vec::new();
        bytes
            .try_reserve_exact(limit)
            .map_err(|error| ControlError::Io(io::Error::other(error)))?;
        Ok(Self { bytes })
    }

    pub(super) fn reserved_bytes(&self) -> Result<u64, ControlError> {
        u64::try_from(self.bytes.capacity()).map_err(|_| ControlError::EncodedBytesExceeded)
    }

    pub(super) fn encode<T: Serialize>(
        mut self,
        control: &T,
    ) -> Result<PreparedFrame, ControlError> {
        let limit = CONTROL_BYTES
            .checked_add(4)
            .ok_or(ControlError::EncodedBytesExceeded)?;
        self.bytes.extend_from_slice(&[0; 4]);
        let mut encoded = BoundedEncoding {
            bytes: self.bytes,
            limit,
        };
        if let Err(error) = serde_json::to_writer(&mut encoded, control) {
            return if error.is_io() && encoded.bytes.len() == limit {
                Err(ControlError::EncodedBytesExceeded)
            } else {
                Err(ControlError::InvalidEncoding(error))
            };
        }
        let length = encoded
            .bytes
            .len()
            .checked_sub(4)
            .and_then(|length| u32::try_from(length).ok())
            .filter(|length| *length != 0)
            .ok_or(ControlError::EncodedBytesExceeded)?;
        encoded
            .bytes
            .get_mut(..4)
            .ok_or(ControlError::EncodedBytesExceeded)?
            .copy_from_slice(&length.to_be_bytes());
        Ok(PreparedFrame {
            bytes: encoded.bytes,
        })
    }
}

impl PreparedFrame {
    pub(super) fn encode<T: Serialize>(control: &T) -> Result<Self, ControlError> {
        FrameStorage::prepare()?.encode(control)
    }

    /// Actual heap capacity, even if the frame's payload is small. The owner additionally charges
    /// its fixed control/ancillary/descriptor state and any decoded command metadata.
    pub(super) fn reserved_bytes(&self) -> Result<u64, ControlError> {
        u64::try_from(self.bytes.capacity()).map_err(|_| ControlError::EncodedBytesExceeded)
    }

    /// Refill after zero-progress terminal send without allocating a second metrics frame.
    pub(super) fn into_storage(mut self) -> FrameStorage {
        self.bytes.clear();
        FrameStorage { bytes: self.bytes }
    }
}

/// An O-owned encoded reply with finite send progress. The actual role owner retains the
/// associated process capability until all bytes are sent; retries attach rights only once.
pub(super) struct IncrementalSend {
    frame: PreparedFrame,
    offset: usize,
    poisoned: bool,
}

impl IncrementalSend {
    pub(super) fn new(frame: PreparedFrame) -> Self {
        Self {
            frame,
            offset: 0,
            poisoned: false,
        }
    }

    pub(super) fn advance(
        &mut self,
        transport: &Transport<'_>,
        rights: &[BorrowedFd<'_>],
        deadline: Instant,
    ) -> Result<bool, ControlError> {
        if self.poisoned {
            return Err(ControlError::ProgressPoisoned);
        }
        self.poisoned = true;
        if Instant::now() >= deadline {
            return Err(ControlError::Deadline);
        }
        transport.refuse_observable_eof()?;
        if self.offset < self.frame.bytes.len() {
            let current_rights = if self.offset == 0 { rights } else { &[] };
            if let Some(count) =
                transport.send_chunk(&self.frame.bytes[self.offset..], current_rights)?
            {
                self.offset = self
                    .offset
                    .checked_add(count)
                    .ok_or(ControlError::Truncated)?;
            }
        }
        self.poisoned = false;
        Ok(self.offset == self.frame.bytes.len())
    }
}

/// Named C receive payload/right storage retained across the closed, ordered role controls.
pub(super) struct PreparedReceive {
    payload: Vec<u8>,
    rights: Vec<OwnedFd>,
}

/// O's incremental frame owner. Partial frames retain owned rights and credentials between
/// normal accounting ticks; no partial read is restarted or promoted to an authorization.
pub(super) struct IncrementalReceive {
    buffer: PreparedReceive,
    header: [u8; 4],
    header_read: usize,
    length: Option<usize>,
    payload_read: usize,
    credentials: Option<PeerCredentials>,
    active: bool,
    poisoned: bool,
}

/// State-specific decoder for the one final metrics frame. Startup, Dispatch and descriptor
/// delivery continue using the strict decoder. The caller still authenticates the actual O
/// sender/run and requires its normal owned child exit before classification.
pub(super) struct TerminalReceive {
    receive: IncrementalReceive,
    completed: bool,
}

impl TerminalReceive {
    pub(super) fn prepare() -> Result<Self, ControlError> {
        Ok(Self {
            receive: IncrementalReceive::prepare()?,
            completed: false,
        })
    }

    pub(super) fn reserved_bytes(&self) -> Result<u64, ControlError> {
        let fixed = std::mem::size_of::<Self>()
            .checked_sub(std::mem::size_of::<PreparedReceive>())
            .and_then(|bytes| u64::try_from(bytes).ok())
            .ok_or(ControlError::EncodedBytesExceeded)?;
        self.receive
            .buffer
            .reserved_bytes()?
            .checked_add(fixed)
            .ok_or(ControlError::EncodedBytesExceeded)
    }

    pub(super) fn advance<'buffer, T: DeserializeOwned>(
        &'buffer mut self,
        transport: &Transport<'_>,
        deadline: Instant,
    ) -> Result<Option<PreparedReceived<'buffer, T>>, ControlError> {
        if self.completed {
            return Err(ControlError::ProgressPoisoned);
        }
        let result =
            self.receive
                .advance_mode(transport, |_| 0, deadline, ReceiveEof::DrainTerminal)?;
        self.completed = result.is_some();
        Ok(result)
    }

    /// After separate actual normal O settlement, require stream EOF with no second frame/tail.
    /// This socket observation alone never proves O exited or that an owned child was reaped.
    pub(super) fn confirm_end(
        &mut self,
        transport: &Transport<'_>,
        deadline: Instant,
    ) -> Result<bool, ControlError> {
        if !self.completed || Instant::now() >= deadline {
            return Err(if self.completed {
                ControlError::Deadline
            } else {
                ControlError::ProgressPoisoned
            });
        }
        let mut byte = [0];
        match transport.read_chunk_mode(
            &mut byte,
            &mut self.receive.credentials,
            &mut self.receive.buffer.rights,
            ReceiveEof::DrainTerminal,
        ) {
            Err(ControlError::Eof) => Ok(true),
            Ok(None) => Ok(false),
            Ok(Some(_)) => Err(ControlError::TrailingTerminalBytes),
            Err(error) => Err(error),
        }
    }
}

impl IncrementalReceive {
    pub(super) fn prepare() -> Result<Self, ControlError> {
        Ok(Self {
            buffer: PreparedReceive::prepare()?,
            header: [0; 4],
            header_read: 0,
            length: None,
            payload_read: 0,
            credentials: None,
            active: false,
            poisoned: false,
        })
    }

    pub(super) fn has_partial_frame(&self) -> bool {
        self.active && (self.header_read != 0 || self.payload_read != 0)
    }

    /// At most one recvmsg per call. A successful incomplete result requires another ordinary
    /// actor/accounting tick; errors poison the decoder and can never resume a half-accepted frame.
    pub(super) fn advance<'buffer, T: DeserializeOwned>(
        &'buffer mut self,
        transport: &Transport<'_>,
        expected_rights: impl FnOnce(&T) -> usize,
        deadline: Instant,
    ) -> Result<Option<PreparedReceived<'buffer, T>>, ControlError> {
        self.advance_mode(transport, expected_rights, deadline, ReceiveEof::Refuse)
    }

    fn advance_mode<'buffer, T: DeserializeOwned>(
        &'buffer mut self,
        transport: &Transport<'_>,
        expected_rights: impl FnOnce(&T) -> usize,
        deadline: Instant,
        eof: ReceiveEof,
    ) -> Result<Option<PreparedReceived<'buffer, T>>, ControlError> {
        if self.poisoned {
            return Err(ControlError::ProgressPoisoned);
        }
        self.poisoned = true;
        if Instant::now() >= deadline {
            return Err(ControlError::Deadline);
        }
        if !self.active {
            self.buffer.rights.clear();
            self.header_read = 0;
            self.length = None;
            self.payload_read = 0;
            self.credentials = None;
            self.active = true;
        }
        if self.header_read < self.header.len() {
            if let Some(count) = transport.read_chunk_mode(
                &mut self.header[self.header_read..],
                &mut self.credentials,
                &mut self.buffer.rights,
                eof,
            )? {
                self.header_read = self
                    .header_read
                    .checked_add(count)
                    .ok_or(ControlError::Truncated)?;
            }
            if self.header_read == self.header.len() {
                let length = usize::try_from(u32::from_be_bytes(self.header))
                    .map_err(|_| ControlError::EncodedBytesExceeded)?;
                if length == 0 || length > CONTROL_BYTES {
                    return Err(ControlError::EncodedBytesExceeded);
                }
                self.buffer.payload.resize(length, 0);
                self.length = Some(length);
            }
            self.poisoned = false;
            return Ok(None);
        }
        let length = self.length.ok_or(ControlError::ProgressPoisoned)?;
        if let Some(count) = transport.read_chunk_mode(
            &mut self.buffer.payload[self.payload_read..],
            &mut self.credentials,
            &mut self.buffer.rights,
            eof,
        )? {
            self.payload_read = self
                .payload_read
                .checked_add(count)
                .ok_or(ControlError::Truncated)?;
        }
        if self.payload_read != length {
            self.poisoned = false;
            return Ok(None);
        }
        // Set successful progress before lending the owned rights. A decoding/validation failure
        // permanently poisons this owner, while a caller may explicitly take valid rights.
        let received = transport.finish_receive_mode(
            &mut self.buffer,
            self.credentials,
            expected_rights,
            eof,
        )?;
        self.active = false;
        self.poisoned = false;
        Ok(Some(received))
    }
}

pub(super) struct PreparedReceived<'buffer, T> {
    pub(super) control: T,
    pub(super) credentials: Option<PeerCredentials>,
    pub(super) rights: &'buffer mut Vec<OwnedFd>,
}

impl PreparedReceive {
    pub(super) fn prepare() -> Result<Self, ControlError> {
        let mut payload = Vec::new();
        payload
            .try_reserve_exact(CONTROL_BYTES)
            .map_err(|error| ControlError::Io(io::Error::other(error)))?;
        let mut rights = Vec::new();
        rights
            .try_reserve_exact(RECEIVED_RIGHTS)
            .map_err(|error| ControlError::Io(io::Error::other(error)))?;
        Ok(Self { payload, rights })
    }

    pub(super) fn reserved_bytes(&self) -> Result<u64, ControlError> {
        // Include the explicit temporary control/ancillary arrays as named C control work.
        // Opaque serde/libc/runtime allocations are not estimated by this capacity calculation.
        let bytes = self
            .rights
            .capacity()
            .checked_mul(std::mem::size_of::<OwnedFd>())
            .and_then(|bytes| bytes.checked_add(self.payload.capacity()))
            .and_then(|bytes| bytes.checked_add(std::mem::size_of::<Self>()))
            .and_then(|bytes| bytes.checked_add(ANCILLARY_BYTES))
            .and_then(|bytes| bytes.checked_add(std::mem::size_of::<[u8; 4]>()))
            .and_then(|bytes| bytes.checked_add(std::mem::size_of::<Option<PeerCredentials>>()))
            .and_then(|bytes| bytes.checked_add(std::mem::size_of::<[PollFd<'_>; 1]>()))
            .and_then(|bytes| bytes.checked_add(std::mem::size_of::<[IoSliceMut<'_>; 1]>()))
            .ok_or(ControlError::EncodedBytesExceeded)?;
        u64::try_from(bytes).map_err(|_| ControlError::EncodedBytesExceeded)
    }
}

struct BoundedEncoding {
    bytes: Vec<u8>,
    limit: usize,
}

impl Write for BoundedEncoding {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let remaining = self.limit.saturating_sub(self.bytes.len());
        let count = remaining.min(bytes.len());
        self.bytes.extend_from_slice(&bytes[..count]);
        if count == 0 && !bytes.is_empty() {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "guardian control exceeds byte limit",
            ))
        } else {
            Ok(count)
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn timespec(duration: Duration) -> Result<Timespec, ControlError> {
    Ok(Timespec {
        tv_sec: i64::try_from(duration.as_secs()).map_err(|_| ControlError::Deadline)?,
        tv_nsec: i64::from(duration.subsec_nanos()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Debug, Deserialize, Serialize, PartialEq)]
    #[serde(deny_unknown_fields)]
    struct Message {
        authorized: bool,
    }

    /// Trace: FR-034-AC-11, FR-034-AC-15, FR-034-AC-33.
    #[test]
    fn only_terminal_receiver_drains_a_complete_frame_after_peer_close() {
        let (caller, endpoint) = private_pair().unwrap();
        guardian(&endpoint)
            .send(&Message { authorized: true }, &[], deadline())
            .unwrap();
        drop(endpoint);
        let mut strict = IncrementalReceive::prepare().unwrap();
        assert!(matches!(
            strict.advance::<Message>(&caller.transport(), |_| 0, deadline()),
            Err(ControlError::Eof)
        ));
        let mut terminal = TerminalReceive::prepare().unwrap();
        assert!(terminal
            .advance::<Message>(&caller.transport(), deadline())
            .unwrap()
            .is_none());
        let received = terminal
            .advance::<Message>(&caller.transport(), deadline())
            .unwrap()
            .expect("positively emitted complete terminal frame");
        assert_eq!(received.control, Message { authorized: true });
        assert!(received.credentials.is_some());
        assert!(received.rights.is_empty());
        assert!(terminal
            .confirm_end(&caller.transport(), deadline())
            .unwrap());
        assert!(matches!(
            terminal.advance::<Message>(&caller.transport(), deadline()),
            Err(ControlError::ProgressPoisoned)
        ));
        // This is transport coverage only: no normal O Child exit/reap is inferred from EOF.
    }

    /// Trace: FR-034-AC-11, FR-034-AC-15, FR-034-AC-33.
    #[test]
    fn terminal_receiver_refuses_peer_exit_during_an_incomplete_frame() {
        let (caller, endpoint) = private_pair().unwrap();
        let frame = PreparedFrame::encode(&Message { authorized: true }).unwrap();
        assert_eq!(
            guardian(&endpoint)
                .send_chunk(&frame.bytes[..4], &[])
                .unwrap(),
            Some(4)
        );
        drop(endpoint);
        let mut terminal = TerminalReceive::prepare().unwrap();
        assert!(terminal
            .advance::<Message>(&caller.transport(), deadline())
            .unwrap()
            .is_none());
        assert!(matches!(
            terminal.advance::<Message>(&caller.transport(), deadline()),
            Err(ControlError::Eof)
        ));
    }

    /// Trace: FR-034-AC-11, FR-034-AC-15, FR-034-AC-33.
    #[test]
    fn terminal_receiver_refuses_a_second_frame_after_the_commit() {
        let (caller, endpoint) = private_pair().unwrap();
        for authorized in [true, false] {
            guardian(&endpoint)
                .send(&Message { authorized }, &[], deadline())
                .unwrap();
        }
        drop(endpoint);
        let mut terminal = TerminalReceive::prepare().unwrap();
        assert!(terminal
            .advance::<Message>(&caller.transport(), deadline())
            .unwrap()
            .is_none());
        assert_eq!(
            terminal
                .advance::<Message>(&caller.transport(), deadline())
                .unwrap()
                .expect("first complete frame")
                .control,
            Message { authorized: true }
        );
        assert!(matches!(
            terminal.confirm_end(&caller.transport(), deadline()),
            Err(ControlError::TrailingTerminalBytes)
        ));
    }

    /// Trace: FR-034-AC-4, FR-034-AC-15, FR-034-AC-16.
    #[test]
    fn prepared_receiver_reuses_storage_and_retains_actual_cloexec_rights() {
        let (caller, endpoint) = private_pair().unwrap();
        let source = std::fs::File::open("/dev/null").unwrap();
        let mut receive = PreparedReceive::prepare().unwrap();
        let reservation = receive.reserved_bytes().unwrap();
        guardian(&endpoint)
            .send(&Message { authorized: true }, &[source.as_fd()], deadline())
            .unwrap();
        {
            let received = caller
                .transport()
                .receive_prepared::<Message>(
                    &mut receive,
                    |message| usize::from(message.authorized),
                    deadline(),
                )
                .unwrap();
            assert_eq!(received.control, Message { authorized: true });
            let right = received.rights.pop().unwrap();
            assert!(rustix::io::fcntl_getfd(&right)
                .unwrap()
                .contains(rustix::io::FdFlags::CLOEXEC));
            let original = rustix::fs::fstat(&source).unwrap();
            let delivered = rustix::fs::fstat(&right).unwrap();
            assert_eq!(
                (delivered.st_dev, delivered.st_ino),
                (original.st_dev, original.st_ino)
            );
        }
        guardian(&endpoint)
            .send(&Message { authorized: false }, &[], deadline())
            .unwrap();
        let received = caller
            .transport()
            .receive_prepared::<Message>(
                &mut receive,
                |message| usize::from(message.authorized),
                deadline(),
            )
            .unwrap();
        assert_eq!(received.control, Message { authorized: false });
        assert!(received.rights.is_empty());
        assert_eq!(receive.reserved_bytes().unwrap(), reservation);
    }

    /// Trace: FR-034-AC-4, FR-034-AC-15, FR-034-AC-16.
    #[test]
    fn incremental_receiver_retains_partial_header_and_actual_owned_rights() {
        let (caller, endpoint) = private_pair().unwrap();
        let source = std::fs::File::open("/dev/null").unwrap();
        let frame = PreparedFrame::encode(&Message { authorized: true }).unwrap();
        let mut receive = IncrementalReceive::prepare().unwrap();
        let mut storage = [MaybeUninit::uninit(); ANCILLARY_BYTES];
        let mut ancillary = SendAncillaryBuffer::new(&mut storage);
        let rights = [source.as_fd()];
        assert!(ancillary.push(SendAncillaryMessage::ScmRights(&rights)));
        assert_eq!(
            sendmsg(
                &endpoint.0,
                &[IoSlice::new(&frame.bytes[..1])],
                &mut ancillary,
                SendFlags::NOSIGNAL
            )
            .unwrap(),
            1
        );
        assert!(receive
            .advance::<Message>(&caller.transport(), |_| 1, deadline())
            .unwrap()
            .is_none());
        // No bytes are available: a finite tick does not restart or forget the earlier right.
        assert!(receive
            .advance::<Message>(&caller.transport(), |_| 1, deadline())
            .unwrap()
            .is_none());
        assert_eq!(
            rustix::io::write(&endpoint.0, &frame.bytes[1..]).unwrap(),
            frame.bytes.len() - 1
        );
        // Header completion is its own tick; payload must await the next normal actor tick.
        assert!(receive
            .advance::<Message>(&caller.transport(), |_| 1, deadline())
            .unwrap()
            .is_none());
        let received = receive
            .advance::<Message>(&caller.transport(), |_| 1, deadline())
            .unwrap()
            .unwrap();
        assert_eq!(received.control, Message { authorized: true });
        assert_eq!(
            received.credentials.unwrap().pid,
            rustix::process::getpid().as_raw_pid()
        );
        let delivered = received.rights.pop().unwrap();
        let original = rustix::fs::fstat(&source).unwrap();
        let actual = rustix::fs::fstat(&delivered).unwrap();
        assert_eq!(
            (original.st_dev, original.st_ino),
            (actual.st_dev, actual.st_ino)
        );
        assert!(rustix::io::fcntl_getfd(&delivered)
            .unwrap()
            .contains(rustix::io::FdFlags::CLOEXEC));
    }

    /// Trace: FR-034-AC-4, FR-034-AC-15.
    #[test]
    fn incremental_receiver_rejects_eof_over_queued_authorization_and_cannot_resume() {
        let (caller, endpoint) = private_pair().unwrap();
        let frame = PreparedFrame::encode(&Message { authorized: false }).unwrap();
        assert_eq!(
            rustix::io::write(&endpoint.0, &frame.bytes[..1]).unwrap(),
            1
        );
        let mut receive = IncrementalReceive::prepare().unwrap();
        assert!(receive
            .advance::<Message>(&caller.transport(), |_| 0, deadline())
            .unwrap()
            .is_none());
        assert_eq!(
            rustix::io::write(&endpoint.0, &frame.bytes[1..]).unwrap(),
            frame.bytes.len() - 1
        );
        drop(endpoint);
        assert!(matches!(
            receive.advance::<Message>(&caller.transport(), |_| 0, deadline()),
            Err(ControlError::Eof)
        ));
        assert!(matches!(
            receive.advance::<Message>(&caller.transport(), |_| 0, deadline()),
            Err(ControlError::ProgressPoisoned)
        ));
    }

    fn deadline() -> Instant {
        Instant::now() + Duration::from_secs(2)
    }

    fn guardian(endpoint: &GuardianEndpoint) -> Transport<'_> {
        Transport(endpoint.0.as_fd(), CredentialsPolicy::ExclusiveCreator)
    }

    fn send_raw(endpoint: &GuardianEndpoint, payload: &[u8], rights: &[BorrowedFd<'_>]) {
        let length = u32::try_from(payload.len()).unwrap().to_be_bytes();
        let mut storage = [MaybeUninit::uninit(); rustix::cmsg_space!(ScmRights(8))];
        let mut ancillary = SendAncillaryBuffer::new(&mut storage);
        if !rights.is_empty() {
            assert!(ancillary.push(SendAncillaryMessage::ScmRights(rights)));
        }
        let count = sendmsg(
            &endpoint.0,
            &[IoSlice::new(&length), IoSlice::new(payload)],
            &mut ancillary,
            SendFlags::NOSIGNAL,
        )
        .unwrap();
        assert_eq!(count, payload.len() + 4);
    }

    /// Trace: FR-034-AC-4, FR-034-AC-15, FR-034-AC-16
    #[test]
    fn fresh_private_pair_receives_actual_sender_and_cloexec_owned_stdin() {
        let (caller, endpoint) = private_pair().unwrap();
        let (stdin, writer) = rustix::pipe::pipe_with(rustix::pipe::PipeFlags::CLOEXEC).unwrap();
        let control = Message { authorized: true };
        guardian(&endpoint).send(&control, &[], deadline()).unwrap();
        let ready = caller
            .transport()
            .receive::<Message>(|_| 0, deadline())
            .unwrap();
        let credentials = ready
            .credentials
            .expect("caller receives actual sender credentials");
        assert_eq!(credentials.pid, rustix::process::getpid().as_raw_pid());
        assert_eq!(credentials.uid, rustix::process::getuid().as_raw());
        assert_eq!(credentials.gid, rustix::process::getgid().as_raw());
        assert!(!rustix::net::sockopt::socket_passcred(&endpoint.0).unwrap());
        caller
            .transport()
            .send(&control, &[stdin.as_fd()], deadline())
            .unwrap();
        let received = guardian(&endpoint)
            .receive::<Message>(|_| 1, deadline())
            .unwrap();
        assert_eq!(received.control, control);
        assert_eq!(
            received.credentials, None,
            "guardian never decodes an outside sender's possibly zero PID as nonzero UCred"
        );
        assert_eq!(received.rights.len(), 1);
        let descriptor = &received.rights[0];
        assert!(rustix::io::fcntl_getfd(descriptor)
            .unwrap()
            .contains(rustix::io::FdFlags::CLOEXEC));
        rustix::io::write(&writer, b"stdin").unwrap();
        let mut actual = [0; 5];
        assert_eq!(rustix::io::read(descriptor, &mut actual).unwrap(), 5);
        assert_eq!(&actual, b"stdin");
        assert!(rustix::io::fcntl_getfd(&caller.0)
            .unwrap()
            .contains(rustix::io::FdFlags::CLOEXEC));
        assert!(rustix::io::fcntl_getfd(&endpoint.0)
            .unwrap()
            .contains(rustix::io::FdFlags::CLOEXEC));
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn unknown_fields_refuse_and_close_received_rights() {
        let (caller, endpoint) = private_pair().unwrap();
        let (reader, writer) = rustix::pipe::pipe_with(
            rustix::pipe::PipeFlags::CLOEXEC | rustix::pipe::PipeFlags::NONBLOCK,
        )
        .unwrap();
        send_raw(
            &endpoint,
            br#"{"authorized":true,"unknown":1}"#,
            &[writer.as_fd()],
        );
        drop(writer);
        assert!(matches!(
            caller.transport().receive::<Message>(|_| 1, deadline()),
            Err(ControlError::InvalidEncoding(_))
        ));
        let mut bytes = [0];
        assert_eq!(
            rustix::io::read(&reader, &mut bytes).unwrap(),
            0,
            "refusal must close its owned received writer"
        );
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn extra_rights_refuse_and_close_every_received_writer() {
        let (caller, endpoint) = private_pair().unwrap();
        let (reader, writer) = rustix::pipe::pipe_with(
            rustix::pipe::PipeFlags::CLOEXEC | rustix::pipe::PipeFlags::NONBLOCK,
        )
        .unwrap();
        send_raw(
            &endpoint,
            br#"{"authorized":true}"#,
            &[writer.as_fd(), writer.as_fd()],
        );
        drop(writer);
        assert!(matches!(
            caller.transport().receive::<Message>(|_| 1, deadline()),
            Err(ControlError::RightsCount {
                expected: 1,
                received: 2
            })
        ));
        let mut bytes = [0];
        assert_eq!(rustix::io::read(&reader, &mut bytes).unwrap(), 0);
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn truncated_ancillary_refuses_with_no_delivered_or_discarded_right_leak() {
        let (caller, endpoint) = private_pair().unwrap();
        let (reader, writer) = rustix::pipe::pipe_with(
            rustix::pipe::PipeFlags::CLOEXEC | rustix::pipe::PipeFlags::NONBLOCK,
        )
        .unwrap();
        let rights = [writer.as_fd(); 8];
        send_raw(&endpoint, br#"{"authorized":true}"#, &rights);
        drop(writer);
        assert!(matches!(
            caller.transport().receive::<Message>(|_| 1, deadline()),
            Err(ControlError::Truncated)
        ));
        let mut bytes = [0];
        assert_eq!(
            rustix::io::read(&reader, &mut bytes).unwrap(),
            0,
            "both rustix-owned delivered rights and kernel-discarded excess rights must close"
        );
    }

    /// Trace: FR-034-AC-1, FR-034-AC-15
    #[test]
    fn observable_eof_refuses_an_already_buffered_authorization() {
        let (caller, endpoint) = private_pair().unwrap();
        caller
            .transport()
            .send(&Message { authorized: true }, &[], deadline())
            .unwrap();
        drop(caller);
        assert!(matches!(
            guardian(&endpoint).receive::<Message>(|_| 0, deadline()),
            Err(ControlError::Eof)
        ));
    }

    /// Trace: FR-034-AC-15
    #[test]
    fn missing_right_and_excess_encoded_bytes_refuse() {
        let (caller, endpoint) = private_pair().unwrap();
        caller
            .transport()
            .send(&Message { authorized: true }, &[], deadline())
            .unwrap();
        assert!(matches!(
            guardian(&endpoint).receive::<Message>(|_| 1, deadline()),
            Err(ControlError::RightsCount {
                expected: 1,
                received: 0
            })
        ));
        assert!(matches!(
            caller
                .transport()
                .send(&vec![b'a'; CONTROL_BYTES], &[], deadline()),
            Err(ControlError::EncodedBytesExceeded)
        ));
    }
}
