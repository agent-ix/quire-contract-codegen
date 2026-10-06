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

fn validate_endpoint(descriptor: &OwnedFd) -> Result<(), ControlError> {
    if rustix::net::sockopt::socket_domain(descriptor)? != AddressFamily::UNIX
        || rustix::net::sockopt::socket_type(descriptor)? != SocketType::STREAM
        || rustix::net::sockopt::socket_passcred(descriptor)?
    {
        return Err(ControlError::UnexpectedCredentials);
    }
    if !rustix::io::fcntl_getfd(descriptor)?.contains(rustix::io::FdFlags::CLOEXEC) {
        return Err(ControlError::RightsNotCloexec);
    }
    Ok(())
}

/// Safe borrowed helper control. Its only production inputs are the private pair's endpoints.
pub(super) struct Transport<'fd>(BorrowedFd<'fd>, CredentialsPolicy);

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
        let mut encoded = BoundedEncoding(Vec::new());
        if let Err(error) = serde_json::to_writer(&mut encoded, control) {
            return if error.is_io() && encoded.0.len() == CONTROL_BYTES {
                Err(ControlError::EncodedBytesExceeded)
            } else {
                Err(ControlError::InvalidEncoding(error))
            };
        }
        let length =
            u32::try_from(encoded.0.len()).map_err(|_| ControlError::EncodedBytesExceeded)?;
        let mut frame = Vec::with_capacity(encoded.0.len() + 4);
        frame.extend_from_slice(&length.to_be_bytes());
        frame.extend_from_slice(&encoded.0);
        let mut offset = 0;
        let mut storage = [MaybeUninit::uninit(); ANCILLARY_BYTES];
        let mut ancillary = SendAncillaryBuffer::new(&mut storage);
        if !rights.is_empty() && !ancillary.push(SendAncillaryMessage::ScmRights(rights)) {
            return Err(ControlError::ExcessRights);
        }
        while offset < frame.len() {
            self.wait(PollFlags::OUT, deadline)?;
            match sendmsg(
                self.0,
                &[IoSlice::new(&frame[offset..])],
                &mut ancillary,
                SendFlags::DONTWAIT | SendFlags::NOSIGNAL,
            ) {
                Ok(0) => return Err(ControlError::Eof),
                Ok(count) => {
                    offset += count;
                    // SCM_RIGHTS accompanies only the first successfully written byte.
                    ancillary.clear();
                }
                Err(rustix::io::Errno::AGAIN | rustix::io::Errno::INTR) => {}
                Err(error) => return Err(error.into()),
            }
        }
        Ok(())
    }

    pub(super) fn receive<T: DeserializeOwned>(
        &self,
        expected_rights: impl FnOnce(&T) -> usize,
        deadline: Instant,
    ) -> Result<Received<T>, ControlError> {
        let mut credentials = None;
        let mut rights = Vec::new();
        let mut header = [0; 4];
        self.read_exact(&mut header, &mut credentials, &mut rights, deadline)?;
        let length = usize::try_from(u32::from_be_bytes(header))
            .map_err(|_| ControlError::EncodedBytesExceeded)?;
        if length == 0 || length > CONTROL_BYTES {
            return Err(ControlError::EncodedBytesExceeded);
        }
        let mut payload = vec![0; length];
        self.read_exact(&mut payload, &mut credentials, &mut rights, deadline)?;
        self.refuse_observable_eof()?;
        let control = serde_json::from_slice(&payload).map_err(ControlError::InvalidEncoding)?;
        let expected_rights = expected_rights(&control);
        if expected_rights > RECEIVED_RIGHTS {
            return Err(ControlError::ExcessRights);
        }
        if rights.len() != expected_rights {
            return Err(ControlError::RightsCount {
                expected: expected_rights,
                received: rights.len(),
            });
        }
        if matches!(self.1, CredentialsPolicy::ActualSender) && credentials.is_none() {
            return Err(ControlError::MissingCredentials);
        }
        Ok(Received {
            control,
            credentials,
            rights,
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
            let mut storage = [MaybeUninit::uninit(); ANCILLARY_BYTES];
            let mut ancillary = RecvAncillaryBuffer::new(&mut storage);
            let received = match recvmsg(
                self.0,
                &mut [IoSliceMut::new(bytes)],
                &mut ancillary,
                RecvFlags::DONTWAIT | RecvFlags::CMSG_CLOEXEC,
            ) {
                Ok(received) => received,
                Err(rustix::io::Errno::AGAIN | rustix::io::Errno::INTR) => continue,
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
            bytes = bytes
                .get_mut(received.bytes..)
                .ok_or(ControlError::Truncated)?;
        }
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

struct BoundedEncoding(Vec<u8>);

impl Write for BoundedEncoding {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let remaining = CONTROL_BYTES.saturating_sub(self.0.len());
        let count = remaining.min(bytes.len());
        self.0.extend_from_slice(&bytes[..count]);
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
