//! Caller-side authenticated lease stages. Monitor ownership stays with the separate RunOwner.

use std::{
    ffi::OsString,
    io,
    os::fd::AsFd,
    time::{Duration, Instant},
};

use super::{
    control::{private_pair, CallerLease, ControlError, GuardianEndpoint, PreparedFrame, Received},
    namespace::{BackendCommand, NamespaceOwner, ReadyIdentityError},
    protocol::{
        current_build_identity, BackendExit, CallerControl, GuardianControl, GuardianRefusal,
        RunAuthority, StdinControl,
    },
    stdin::OriginalStdin,
};

const SETUP_CAP: Duration = Duration::from_secs(3);
const CONTROL_CAP: Duration = Duration::from_secs(3);

/// No INIT claim or backend authorization exists yet.
pub(super) struct Bootstrap {
    lease: CallerLease,
    authority: RunAuthority,
    setup_deadline: Instant,
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
}

impl PreparedDispatch {
    pub(super) fn reserved_bytes(&self) -> Result<u64, StageError> {
        self.frame.reserved_bytes().map_err(StageError::Control)
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
            },
            endpoint,
        ))
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
        })
    }

    /// Called only after the separate owner validates INIT and binds its memory observer.
    pub(super) fn claimed(self) -> Result<ClaimedBootstrap, StageError> {
        self.lease.transport().send(
            &CallerControl::Hello {
                identity: current_build_identity(),
                authority: self.authority,
            },
            &[],
            self.setup_deadline,
        )?;
        Ok(ClaimedBootstrap(self))
    }

    pub(super) fn into_lease(self) -> CallerLease {
        self.lease
    }
}

impl ClaimedBootstrap {
    pub(super) fn authenticate(self, owner: &NamespaceOwner) -> Result<InitReady, StageError> {
        let received = self
            .0
            .lease
            .transport()
            .receive::<GuardianControl>(|_| 0, self.0.setup_deadline)?;
        let (identity, authority, mapped_uid) = match &received.control {
            GuardianControl::Ready {
                identity,
                authority,
                mapped_uid,
                ..
            } => (*identity, *authority, *mapped_uid),
            GuardianControl::Refused { reason } => {
                return Err(StageError::GuardianRefused(*reason))
            }
            _ => return Err(StageError::UnexpectedControl),
        };
        if identity != current_build_identity() {
            return Err(StageError::BuildIdentityMismatch);
        }
        if authority != self.0.authority {
            return Err(StageError::AuthorityMismatch);
        }
        verify_sender(owner, &received, mapped_uid)?;
        Ok(InitReady {
            bootstrap: self.0,
            mapped_uid,
        })
    }
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

    pub(super) fn acknowledge(self, owner: &NamespaceOwner) -> Result<Dispatched, StageError> {
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

impl Dispatched {
    /// Bounded authenticated completion; control EOF never stands for backend completion.
    pub(super) fn completion(
        &self,
        owner: &NamespaceOwner,
        deadline: Option<Instant>,
    ) -> Result<Option<BackendExit>, StageError> {
        let transport = self.ready.bootstrap.lease.transport();
        if !transport.pending_control(Duration::ZERO)? {
            return Ok(None);
        }
        let cap = Instant::now() + CONTROL_CAP;
        let deadline = deadline.map_or(cap, |deadline| deadline.min(cap));
        let received = transport.receive::<GuardianControl>(|_| 0, deadline)?;
        verify_sender(owner, &received, self.ready.mapped_uid)?;
        match received.control {
            GuardianControl::Completed { authority, outcome }
                if authority == self.ready.bootstrap.authority =>
            {
                Ok(Some(outcome))
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
    owner: &NamespaceOwner,
    received: &Received<GuardianControl>,
    mapped_uid: u32,
) -> Result<(), StageError> {
    owner.verify_ready(
        received.credentials.ok_or(StageError::MissingSender)?,
        mapped_uid,
    )?;
    Ok(())
}
