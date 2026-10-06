//! Transfer of the original identity deadline between roles sharing the kernel monotonic clock.
//!
//! Namespace setup never creates a time namespace. Sampling order rounds toward an earlier
//! deadline in both directions; an unrepresentable or expired deadline never becomes unbounded.

use std::time::{Duration, Instant};

use rustix::time::{clock_gettime, ClockId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct RoleDeadline {
    seconds: u64,
    nanoseconds: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum DeadlineError {
    Expired,
    Unrepresentable,
    InvalidClock,
}

impl std::fmt::Display for DeadlineError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "original role deadline refused: {self:?}")
    }
}

impl std::error::Error for DeadlineError {}

impl RoleDeadline {
    /// Kernel clock is sampled first: any intervening work subtracts rather than adds time.
    pub(super) fn from_original(original: Instant) -> Result<Self, DeadlineError> {
        let kernel = monotonic()?;
        let remaining = original
            .checked_duration_since(Instant::now())
            .filter(|remaining| !remaining.is_zero())
            .ok_or(DeadlineError::Expired)?;
        let deadline = kernel
            .checked_add(remaining)
            .ok_or(DeadlineError::Unrepresentable)?;
        Ok(Self {
            seconds: deadline.as_secs(),
            nanoseconds: deadline.subsec_nanos(),
        })
    }

    /// Local Instant is sampled first, again rounding toward an earlier original deadline.
    pub(super) fn local(self) -> Result<Instant, DeadlineError> {
        if self.nanoseconds >= 1_000_000_000 {
            return Err(DeadlineError::InvalidClock);
        }
        let local = Instant::now();
        let remaining = Duration::new(self.seconds, self.nanoseconds)
            .checked_sub(monotonic()?)
            .filter(|remaining| !remaining.is_zero())
            .ok_or(DeadlineError::Expired)?;
        local
            .checked_add(remaining)
            .ok_or(DeadlineError::Unrepresentable)
    }
}

/// Explicit mandatory wire state for the original admission. Missing JSON cannot silently
/// become never-elapsing through serde's implicit missing Option-field behavior.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub(super) enum IdentityDeadline {
    Finite { deadline: RoleDeadline },
    NeverElapses,
}

impl IdentityDeadline {
    pub(super) fn from_original(original: Option<Instant>) -> Result<Self, DeadlineError> {
        match original {
            Some(original) => {
                RoleDeadline::from_original(original).map(|deadline| Self::Finite { deadline })
            }
            None => Ok(Self::NeverElapses),
        }
    }

    pub(super) fn local(self) -> Result<Option<Instant>, DeadlineError> {
        match self {
            Self::Finite { deadline } => deadline.local().map(Some),
            Self::NeverElapses => Ok(None),
        }
    }
}

fn monotonic() -> Result<Duration, DeadlineError> {
    let time = clock_gettime(ClockId::Monotonic);
    let seconds = u64::try_from(time.tv_sec).map_err(|_| DeadlineError::InvalidClock)?;
    let nanoseconds = u32::try_from(time.tv_nsec).map_err(|_| DeadlineError::InvalidClock)?;
    if nanoseconds >= 1_000_000_000 {
        return Err(DeadlineError::InvalidClock);
    }
    Ok(Duration::new(seconds, nanoseconds))
}
