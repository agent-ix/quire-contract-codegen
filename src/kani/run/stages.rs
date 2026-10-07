//! Caller-side authenticated lease stages. Monitor ownership stays with the separate RunOwner.

use std::{
    ffi::OsString,
    io,
    os::fd::AsFd,
    time::{Duration, Instant},
};

use super::{
    control::{
        private_pair, CallerLease, ControlError, GuardianEndpoint, IncrementalSend, PreparedFrame,
        Received, Transport,
    },
    namespace::{BackendCommand, GuardianIdentity, ReadyIdentityError},
    protocol::{
        current_build_identity, BackendExit, CallerControl, GuardianControl, GuardianRefusal,
        RunAuthority, StdinControl,
    },
    role_deadline::StopStamp,
    stdin::OriginalStdin,
};

const SETUP_CAP: Duration = Duration::from_secs(3);
const CONTROL_CAP: Duration = Duration::from_secs(3);

/// No INIT claim or backend authorization exists yet.
pub(super) struct Bootstrap {
    lease: CallerLease,
    authority: RunAuthority,
    setup_deadline: Instant,
    hello: Option<PreparedFrame>,
}

/// The independently retained RunOwner has claimed INIT and opened only trusted bootstrap.
pub(super) struct ClaimedBootstrap(Bootstrap);

/// Actual peer/build/session/UID identity and the namespace memory observer are ready.
pub(super) struct InitReady {
    bootstrap: Bootstrap,
    mapped_uid: u32,
}

/// Positive Dispatch was acknowledged by the actual retained INIT.
pub(super) struct Dispatched {
    ready: InitReady,
}

/// The bounded production Dispatch frame was sent; acknowledgement is still required.
pub(super) struct PendingDispatch {
    ready: InitReady,
}

/// C-owned authorization bytes may be allocated before any helper role is launched. Encoding
/// does not send or authorize anything; only the authenticated InitReady transition sends them.
pub(super) struct PreparedDispatch {
    authority: RunAuthority,
    stdin: StdinControl,
    frame: PreparedFrame,
    metadata_reservation: u64,
}

impl PreparedDispatch {
    pub(super) fn reserved_bytes(&self) -> Result<u64, StageError> {
        self.frame
            .reserved_bytes()
            .map_err(StageError::Control)?
            .checked_add(self.metadata_reservation)
            .ok_or(StageError::Control(ControlError::EncodedBytesExceeded))
    }
}

#[derive(Debug)]
pub(super) enum StageError {
    Control(ControlError),
    Identity(ReadyIdentityError),
    Authority(io::Error),
    BuildIdentityMismatch,
    AuthorityMismatch,
    UnexpectedControl,
    MissingSender,
    GuardianRefused(GuardianRefusal),
}

impl From<ControlError> for StageError {
    fn from(error: ControlError) -> Self {
        Self::Control(error)
    }
}

impl From<ReadyIdentityError> for StageError {
    fn from(error: ReadyIdentityError) -> Self {
        Self::Identity(error)
    }
}

impl std::fmt::Display for StageError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Control(error) => write!(formatter, "{error}"),
            Self::Identity(error) => write!(formatter, "{error}"),
            Self::Authority(error) => write!(formatter, "run authority acquisition: {error}"),
            other => write!(formatter, "{other:?}"),
        }
    }
}

impl std::error::Error for StageError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Control(error) => Some(error),
            Self::Identity(error) => Some(error),
            Self::Authority(error) => Some(error),
            _ => None,
        }
    }
}

impl Bootstrap {
    pub(super) fn new(deadline: Option<Instant>) -> Result<(Self, GuardianEndpoint), StageError> {
        let cap = Instant::now() + SETUP_CAP;
        let setup_deadline = deadline.map_or(cap, |deadline| deadline.min(cap));
        if Instant::now() >= setup_deadline {
            return Err(ControlError::Deadline.into());
        }
        let authority = RunAuthority::fresh().map_err(StageError::Authority)?;
        let (lease, endpoint) = private_pair()?;
        Ok((
            Self {
                lease,
                authority,
                setup_deadline,
                hello: Some(PreparedFrame::encode(&CallerControl::Hello {
                    identity: current_build_identity(),
                    authority,
                })?),
            },
            endpoint,
        ))
    }

    pub(super) fn reserved_bytes(&self) -> Result<u64, StageError> {
        self.hello
            .as_ref()
            .ok_or(StageError::UnexpectedControl)?
            .reserved_bytes()
            .map_err(StageError::Control)
    }

    pub(super) fn setup_deadline(&self) -> Instant {
        self.setup_deadline
    }

    pub(super) fn authority(&self) -> RunAuthority {
        self.authority
    }

    pub(super) fn prepare_dispatch(
        &self,
        command: BackendCommand,
        stdin: &OriginalStdin,
        cleanup_paths: Vec<OsString>,
    ) -> Result<PreparedDispatch, StageError> {
        let stdin = match stdin {
            OriginalStdin::Open(_) => StdinControl::Open,
            OriginalStdin::Closed => StdinControl::Closed,
        };
        // Charge the peak of actual named preparation metadata as well as the retained frame.
        // Serialization consumes these values before L exists; no late unbounded recipe clone
        // is introduced by the authenticated Dispatch transition.
        let mut metadata_reservation = command
            .reserved_bytes()
            .map_err(|error| StageError::Control(ControlError::Io(error)))?;
        let cleanup_storage = cleanup_paths
            .capacity()
            .checked_mul(std::mem::size_of::<OsString>())
            .ok_or(StageError::Control(ControlError::EncodedBytesExceeded))?;
        metadata_reservation = metadata_reservation
            .checked_add(
                u64::try_from(cleanup_storage)
                    .map_err(|_| StageError::Control(ControlError::EncodedBytesExceeded))?,
            )
            .ok_or(StageError::Control(ControlError::EncodedBytesExceeded))?;
        for path in &cleanup_paths {
            metadata_reservation = metadata_reservation
                .checked_add(
                    u64::try_from(path.capacity())
                        .map_err(|_| StageError::Control(ControlError::EncodedBytesExceeded))?,
                )
                .ok_or(StageError::Control(ControlError::EncodedBytesExceeded))?;
        }
        let frame = PreparedFrame::encode(&CallerControl::Dispatch {
            authority: self.authority,
            command,
            stdin,
            cleanup_paths,
        })?;
        Ok(PreparedDispatch {
            authority: self.authority,
            stdin,
            frame,
            metadata_reservation,
        })
    }

    /// Called only after the separate owner validates INIT and binds its memory observer.
    pub(super) fn claimed(self) -> Result<ClaimedBootstrap, StageError> {
        self.lease.transport().send_prepared(
            self.hello.as_ref().ok_or(StageError::UnexpectedControl)?,
            &[],
            self.setup_deadline,
        )?;
        Ok(ClaimedBootstrap(self))
    }

    /// The retained C owner moves these pre-L bytes once into its incremental send state.
    /// Taking the frame never sends Hello, opens a gate or authenticates INIT.
    pub(super) fn take_hello(&mut self) -> Result<PreparedFrame, StageError> {
        self.hello.take().ok_or(StageError::UnexpectedControl)
    }

    pub(super) fn transport(&self) -> super::control::Transport<'_> {
        self.lease.transport()
    }

    /// The token is minted only after the same actual INIT sender/run/build checks. C takes
    /// this stage after a completed incremental receive; pending/error paths retain the lease.
    pub(super) fn into_authenticated(
        self,
        ready: VerifiedInitReady,
    ) -> Result<InitReady, (Self, StageError)> {
        if ready.authority != self.authority {
            return Err((self, StageError::AuthorityMismatch));
        }
        Ok(InitReady {
            bootstrap: self,
            mapped_uid: ready.mapped_uid,
        })
    }

    pub(super) fn into_lease(self) -> CallerLease {
        self.lease
    }
}

impl ClaimedBootstrap {
    pub(super) fn authenticate(
        self,
        owner: &impl GuardianIdentity,
    ) -> Result<InitReady, StageError> {
        let received = self
            .0
            .lease
            .transport()
            .receive::<GuardianControl>(|_| 0, self.0.setup_deadline)?;
        if let GuardianControl::Refused { reason } = &received.control {
            return Err(StageError::GuardianRefused(*reason));
        }
        let ready = verify_init_ready(
            owner,
            received.control,
            received.credentials.ok_or(StageError::MissingSender)?,
            self.0.authority,
        )?;
        Ok(InitReady {
            bootstrap: self.0,
            mapped_uid: ready.mapped_uid,
        })
    }
}

/// Private positive receipt; no refusal, peer EOF or PID snapshot can construct it.
pub(super) struct VerifiedInitReady {
    mapped_uid: u32,
    authority: RunAuthority,
}

pub(super) fn verify_init_ready(
    owner: &impl GuardianIdentity,
    control: GuardianControl,
    sender: super::control::PeerCredentials,
    authority: RunAuthority,
) -> Result<VerifiedInitReady, StageError> {
    let GuardianControl::Ready {
        identity,
        authority: actual,
        mapped_uid,
        ..
    } = control
    else {
        // Typed startup failures use their separate authenticated cause path. An untyped
        // Refused cannot skip actual owner binding by being accepted as positive Ready.
        return Err(StageError::UnexpectedControl);
    };
    if identity != current_build_identity() {
        return Err(StageError::BuildIdentityMismatch);
    }
    if actual != authority {
        return Err(StageError::AuthorityMismatch);
    }
    owner.verify_ready(sender, mapped_uid)?;
    Ok(VerifiedInitReady {
        mapped_uid,
        authority,
    })
}

impl InitReady {
    pub(super) fn send_dispatch(
        self,
        command: BackendCommand,
        stdin: &OriginalStdin,
        cleanup_paths: Vec<OsString>,
    ) -> Result<PendingDispatch, StageError> {
        let prepared = self
            .bootstrap
            .prepare_dispatch(command, stdin, cleanup_paths)?;
        self.send_prepared_dispatch(prepared, stdin)
    }

    pub(super) fn send_prepared_dispatch(
        self,
        prepared: PreparedDispatch,
        stdin: &OriginalStdin,
    ) -> Result<PendingDispatch, StageError> {
        if prepared.authority != self.bootstrap.authority {
            return Err(StageError::AuthorityMismatch);
        }
        let (stdin_control, descriptor) = match stdin {
            OriginalStdin::Open(descriptor) => (StdinControl::Open, Some(descriptor.as_fd())),
            OriginalStdin::Closed => (StdinControl::Closed, None),
        };
        if prepared.stdin != stdin_control {
            return Err(StageError::UnexpectedControl);
        }
        self.bootstrap.lease.transport().send_prepared(
            &prepared.frame,
            descriptor.as_slice(),
            self.bootstrap.setup_deadline,
        )?;
        Ok(PendingDispatch { ready: self })
    }

    pub(super) fn into_lease(self) -> CallerLease {
        self.bootstrap.into_lease()
    }
}

impl PendingDispatch {
    #[cfg(feature = "guardian-test-support")]
    pub(super) fn into_lease(self) -> CallerLease {
        self.ready.into_lease()
    }

    pub(super) fn acknowledge(
        self,
        owner: &impl GuardianIdentity,
    ) -> Result<Dispatched, StageError> {
        let received = self
            .ready
            .bootstrap
            .lease
            .transport()
            .receive::<GuardianControl>(|_| 0, self.ready.bootstrap.setup_deadline)?;
        verify_sender(owner, &received, self.ready.mapped_uid)?;
        match received.control {
            GuardianControl::Dispatched { authority }
                if authority == self.ready.bootstrap.authority =>
            {
                Ok(Dispatched { ready: self.ready })
            }
            GuardianControl::Dispatched { .. } => Err(StageError::AuthorityMismatch),
            GuardianControl::Refused { reason } => Err(StageError::GuardianRefused(reason)),
            _ => Err(StageError::UnexpectedControl),
        }
    }
}

/// Actual C lease custody during nonblocking Dispatch/ACK/completion. No fallible operation
/// consumes the lease; even a partial send or bad reply retains it until explicit close.
pub(super) struct CallerLeaseClient {
    ready: InitReady,
    dispatch: Option<IncrementalSend>,
    stdin: StdinControl,
    prepared_authority: RunAuthority,
    state: CallerLeaseState,
    failed: bool,
}

enum CallerLeaseState {
    Sending,
    AwaitAcknowledgment,
    Running,
    Completed(BackendCompletion),
}

pub(super) enum CallerLeaseProgress {
    Pending,
    Dispatched,
    Completed,
}

impl CallerLeaseClient {
    /// Uses only the existing pre-L Dispatch frame. The owner stores actual lease custody
    /// before validating even its own prepared authority; failure never implicitly closes it.
    pub(super) fn new(ready: InitReady, prepared: PreparedDispatch) -> Self {
        Self {
            ready,
            dispatch: Some(IncrementalSend::new(prepared.frame)),
            stdin: prepared.stdin,
            prepared_authority: prepared.authority,
            state: CallerLeaseState::Sending,
            failed: false,
        }
    }

    pub(super) fn metadata_reservation() -> Result<u64, StageError> {
        u64::try_from(std::mem::size_of::<Self>())
            .map_err(|_| StageError::Control(ControlError::EncodedBytesExceeded))
    }

    /// At most one nonblocking send. C can sample captures/receive O's original stop clock
    /// between incomplete attempts; no local blocking control allowance replaces its cutoff.
    pub(super) fn send_step(
        &mut self,
        stdin: &OriginalStdin,
        cutoff: Instant,
    ) -> Result<bool, StageError> {
        if self.failed || !matches!(self.state, CallerLeaseState::Sending) {
            return Err(StageError::UnexpectedControl);
        }
        self.failed = true;
        if self.prepared_authority != self.ready.bootstrap.authority {
            return Err(StageError::AuthorityMismatch);
        }
        let (kind, descriptor) = match stdin {
            OriginalStdin::Open(descriptor) => (StdinControl::Open, Some(descriptor.as_fd())),
            OriginalStdin::Closed => (StdinControl::Closed, None),
        };
        if kind != self.stdin {
            return Err(StageError::UnexpectedControl);
        }
        let sent = self
            .dispatch
            .as_mut()
            .ok_or(StageError::UnexpectedControl)?
            .advance(
                &self.ready.bootstrap.lease.transport(),
                descriptor.as_slice(),
                cutoff,
            )?;
        if sent {
            self.dispatch = None;
            self.state = CallerLeaseState::AwaitAcknowledgment;
        }
        self.failed = false;
        Ok(sent)
    }

    pub(super) fn transport(&self) -> Transport<'_> {
        self.ready.bootstrap.lease.transport()
    }

    /// Called only after the SINGLE retained I framer produced one complete strictly live
    /// frame. State plus actual pinned sender/run authentication precede either transition.
    pub(super) fn accept_event(
        &mut self,
        owner: &impl GuardianIdentity,
        control: super::startup_envelope::InnerEventHeader,
        sender: super::control::PeerCredentials,
    ) -> Result<CallerLeaseProgress, StageError> {
        if self.failed {
            return Err(StageError::UnexpectedControl);
        }
        self.failed = true;
        self.transport().refuse_observable_eof()?;
        owner.verify_ready(sender, self.ready.mapped_uid)?;
        let progress = match (&self.state, control) {
            (
                CallerLeaseState::AwaitAcknowledgment,
                super::startup_envelope::InnerEventHeader::Dispatched { authority },
            ) => {
                if authority != self.ready.bootstrap.authority {
                    return Err(StageError::AuthorityMismatch);
                }
                self.state = CallerLeaseState::Running;
                CallerLeaseProgress::Dispatched
            }
            (
                CallerLeaseState::Running,
                super::startup_envelope::InnerEventHeader::Completed {
                    authority,
                    outcome,
                    stop,
                },
            ) => {
                if authority != self.ready.bootstrap.authority {
                    return Err(StageError::AuthorityMismatch);
                }
                if stop.origin != super::role_deadline::StopOrigin::Inner {
                    return Err(StageError::UnexpectedControl);
                }
                self.state = CallerLeaseState::Completed(BackendCompletion { outcome, stop });
                CallerLeaseProgress::Completed
            }
            (
                CallerLeaseState::Sending
                | CallerLeaseState::AwaitAcknowledgment
                | CallerLeaseState::Running
                | CallerLeaseState::Completed(_),
                _,
            ) => return Err(StageError::UnexpectedControl),
        };
        self.failed = false;
        Ok(progress)
    }

    pub(super) fn completion(&self) -> Option<&BackendCompletion> {
        match &self.state {
            CallerLeaseState::Completed(completion) => Some(completion),
            CallerLeaseState::Sending
            | CallerLeaseState::AwaitAcknowledgment
            | CallerLeaseState::Running => None,
        }
    }

    /// Actual close remains the original owner operation, which publishes close-completion
    /// before LeaseClosing and observes I before any independent escalation.
    pub(super) fn into_lease(self) -> CallerLease {
        self.ready.into_lease()
    }
}

/// Authenticated actual I completion retains its original trigger for C's whole-owner clock.
/// This record is private and provisional; it grants neither report nor settlement acceptance.
pub(super) struct BackendCompletion {
    pub(super) outcome: BackendExit,
    pub(super) stop: StopStamp,
}

impl Dispatched {
    /// Bounded authenticated completion; control EOF never stands for backend completion.
    pub(super) fn completion(
        &self,
        owner: &impl GuardianIdentity,
        deadline: Option<Instant>,
    ) -> Result<Option<BackendCompletion>, StageError> {
        let transport = self.ready.bootstrap.lease.transport();
        if !transport.pending_control(Duration::ZERO)? {
            return Ok(None);
        }
        let cap = Instant::now() + CONTROL_CAP;
        let deadline = deadline.map_or(cap, |deadline| deadline.min(cap));
        let received = transport.receive::<GuardianControl>(|_| 0, deadline)?;
        verify_sender(owner, &received, self.ready.mapped_uid)?;
        match received.control {
            GuardianControl::Completed {
                authority,
                outcome,
                stop,
            } if authority == self.ready.bootstrap.authority => {
                Ok(Some(BackendCompletion { outcome, stop }))
            }
            GuardianControl::Completed { .. } => Err(StageError::AuthorityMismatch),
            GuardianControl::Refused { reason } => Err(StageError::GuardianRefused(reason)),
            _ => Err(StageError::UnexpectedControl),
        }
    }

    pub(super) fn into_lease(self) -> CallerLease {
        self.ready.into_lease()
    }
}

fn verify_sender(
    owner: &impl GuardianIdentity,
    received: &Received<GuardianControl>,
    mapped_uid: u32,
) -> Result<(), StageError> {
    owner.verify_ready(
        received.credentials.ok_or(StageError::MissingSender)?,
        mapped_uid,
    )?;
    Ok(())
}
