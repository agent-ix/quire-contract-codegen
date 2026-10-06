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
    StopNotStarted,
}

impl std::fmt::Display for DeadlineError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "original role deadline refused: {self:?}")
    }
}

impl std::error::Error for DeadlineError {}

impl RoleDeadline {
    /// Compare authenticated absolute bounds in their shared kernel monotonic domain. Two
    /// separate local() conversions conservatively round differently and cannot test equality.
    pub(super) fn no_later_than(self, bound: Self) -> Result<bool, DeadlineError> {
        if self.nanoseconds >= 1_000_000_000 || bound.nanoseconds >= 1_000_000_000 {
            return Err(DeadlineError::InvalidClock);
        }
        Ok((self.seconds, self.nanoseconds) <= (bound.seconds, bound.nanoseconds))
    }

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

/// The SPEC's one whole-run settlement reservation. It is not extra time after a finite T.
pub(super) const SETTLE_RESERVE: Duration = Duration::from_secs(1);

/// C owns the original clock and the first actual stop instant. Finite work and settlement
/// deadlines are derived once from that same original deadline, never from phase entry times.
pub(super) struct ExecutionClock {
    original: Option<Instant>,
    work: Option<Instant>,
    settlement: Option<Instant>,
}

impl ExecutionClock {
    /// `whole_bound` is the existing whole-run T (the outer bound for a batch). An overflowing
    /// original checked_add remains None: it neither creates a work cutoff nor refuses admission.
    pub(super) fn prepare(
        original: Option<Instant>,
        whole_bound: Duration,
    ) -> Result<Self, DeadlineError> {
        let work = original
            .map(|deadline| {
                let reserve = SETTLE_RESERVE.min(whole_bound / 2);
                deadline
                    .checked_sub(reserve)
                    .ok_or(DeadlineError::Unrepresentable)
            })
            .transpose()?;
        Ok(Self {
            original,
            work,
            settlement: original,
        })
    }

    pub(super) fn original_deadline(&self) -> Option<Instant> {
        self.original
    }

    pub(super) fn work_deadline(&self) -> Option<Instant> {
        self.work
    }

    pub(super) fn work_expired(&self, now: Instant) -> bool {
        self.work.is_some_and(|deadline| now >= deadline)
    }

    /// Call at the FIRST actual stop trigger, including completion. Repeated triggers retain
    /// exactly the same settlement deadline; they cannot restart a never-elapsing run's allowance.
    pub(super) fn begin_settlement(&mut self, trigger: Instant) -> Result<Instant, DeadlineError> {
        if let Some(deadline) = self.settlement {
            return Ok(deadline);
        }
        let deadline = trigger
            .checked_add(SETTLE_RESERVE)
            .ok_or(DeadlineError::Unrepresentable)?;
        self.settlement = Some(deadline);
        Ok(deadline)
    }

    pub(super) fn settlement_deadline(&self) -> Result<Instant, DeadlineError> {
        self.settlement.ok_or(DeadlineError::StopNotStarted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Trace: FR-034-AC-38.
    #[test]
    fn finite_reserve_is_inside_original_whole_bound_and_short_budgets_are_admitted() {
        let start = Instant::now();
        for bound in [
            Duration::ZERO,
            Duration::from_nanos(3),
            Duration::from_secs(10),
        ] {
            let original = start.checked_add(bound).unwrap();
            let mut clock = ExecutionClock::prepare(Some(original), bound).unwrap();
            let reserve = SETTLE_RESERVE.min(bound / 2);
            assert_eq!(clock.work_deadline(), original.checked_sub(reserve));
            assert_eq!(clock.original_deadline(), Some(original));
            assert_eq!(clock.begin_settlement(start).unwrap(), original);
            assert_eq!(clock.begin_settlement(original).unwrap(), original);
            assert!(clock.work_expired(original));
        }
    }

    /// Trace: FR-034-AC-38.
    #[test]
    fn never_elapsing_work_arms_only_one_first_stop_settlement_allowance() {
        let start = Instant::now();
        let mut clock = ExecutionClock::prepare(None, Duration::MAX).unwrap();
        assert_eq!(clock.work_deadline(), None);
        assert!(!clock.work_expired(start));
        assert_eq!(
            clock.settlement_deadline(),
            Err(DeadlineError::StopNotStarted)
        );
        let first = clock.begin_settlement(start).unwrap();
        assert_eq!(first, start.checked_add(SETTLE_RESERVE).unwrap());
        assert_eq!(clock.begin_settlement(first).unwrap(), first);
        assert_eq!(clock.settlement_deadline().unwrap(), first);
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
