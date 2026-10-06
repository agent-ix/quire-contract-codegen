//! Transfer of the original identity deadline between roles sharing the kernel monotonic clock.
//!
//! Namespace setup never creates a time namespace. Sampling order rounds toward an earlier
//! deadline in both directions; an unrepresentable or expired deadline never becomes unbounded.

use std::time::{Duration, Instant};

use rustix::time::{clock_gettime, ClockId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
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
    StopBeforeRun,
    FutureStop,
    RegressingStop,
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

/// Absolute kernel-clock instant, distinct from an absolute deadline. Neither namespaces nor
/// receipt time may replace the actual producing role's event instant.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct MonotonicInstant {
    seconds: u64,
    nanoseconds: u32,
}

impl MonotonicInstant {
    pub(super) fn now() -> Result<Self, DeadlineError> {
        let time = monotonic()?;
        Ok(Self {
            seconds: time.as_secs(),
            nanoseconds: time.subsec_nanos(),
        })
    }

    fn duration(self) -> Result<Duration, DeadlineError> {
        if self.nanoseconds >= 1_000_000_000 {
            return Err(DeadlineError::InvalidClock);
        }
        Ok(Duration::new(self.seconds, self.nanoseconds))
    }

    fn deadline_after(self, reserve: Duration) -> Result<RoleDeadline, DeadlineError> {
        let deadline = self
            .duration()?
            .checked_add(reserve)
            .ok_or(DeadlineError::Unrepresentable)?;
        Ok(RoleDeadline {
            seconds: deadline.as_secs(),
            nanoseconds: deadline.subsec_nanos(),
        })
    }
}

/// The original event producer. A forwarded stamp retains its producer, not the receiver's role.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(super) enum StopOrigin {
    Caller,
    Launcher,
    Outer,
    Inner,
    /// Trusted same-PID installer can detect policy failure before I receives that event.
    Backend,
}

impl StopOrigin {
    fn slot(self) -> usize {
        match self {
            Self::Caller => 0,
            Self::Launcher => 1,
            Self::Outer => 2,
            Self::Inner => 3,
            Self::Backend => 4,
        }
    }
}

/// Mandatory typed payload on an authenticated existing production control. Missing payloads
/// refuse through ordinary schema decoding; this record itself grants no sender/run authority.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct StopStamp {
    pub(super) origin: StopOrigin,
    instant: MonotonicInstant,
}

impl StopStamp {
    /// Capture at the actual trigger, before later frame preparation or forwarding work.
    pub(super) fn capture(origin: StopOrigin) -> Result<Self, DeadlineError> {
        Ok(Self {
            origin,
            instant: MonotonicInstant::now()?,
        })
    }
}

/// Fixed storage for one run's earliest genuine trigger and per-origin monotonic validation.
/// Call only after existing credential, run, role and exact ancillary checks. A delayed valid
/// earlier producer can shorten the bound; no second event or delayed receipt can extend it.
pub(super) struct StopTimeline {
    started: MonotonicInstant,
    last: [Option<MonotonicInstant>; 5],
    earliest: Option<StopStamp>,
}

impl StopTimeline {
    pub(super) fn prepare(started: MonotonicInstant) -> Result<Self, DeadlineError> {
        started.duration()?;
        Ok(Self {
            started,
            last: [None; 5],
            earliest: None,
        })
    }

    pub(super) fn capture_once(&mut self, origin: StopOrigin) -> Result<StopStamp, DeadlineError> {
        if let Some(instant) = self.last.get(origin.slot()).copied().flatten() {
            return Ok(StopStamp { origin, instant });
        }
        let stamp = StopStamp::capture(origin)?;
        self.observe(stamp)?;
        Ok(stamp)
    }

    pub(super) fn earliest(&self) -> Result<StopStamp, DeadlineError> {
        self.earliest.ok_or(DeadlineError::StopNotStarted)
    }

    pub(super) fn observe(&mut self, stamp: StopStamp) -> Result<StopStamp, DeadlineError> {
        self.observe_at(stamp, MonotonicInstant::now()?)
    }

    fn observe_at(
        &mut self,
        stamp: StopStamp,
        now: MonotonicInstant,
    ) -> Result<StopStamp, DeadlineError> {
        stamp.instant.duration()?;
        now.duration()?;
        if stamp.instant < self.started {
            return Err(DeadlineError::StopBeforeRun);
        }
        if stamp.instant > now {
            return Err(DeadlineError::FutureStop);
        }
        let last = self
            .last
            .get_mut(stamp.origin.slot())
            .ok_or(DeadlineError::InvalidClock)?;
        if last.is_some_and(|previous| stamp.instant < previous) {
            return Err(DeadlineError::RegressingStop);
        }
        *last = Some(stamp.instant);
        let earliest = self.earliest.map_or(stamp, |previous| {
            if stamp.instant < previous.instant {
                stamp
            } else {
                previous
            }
        });
        self.earliest = Some(earliest);
        Ok(earliest)
    }

    /// Return the absolute earliest-trigger bound, clipped to the original finite T. No local
    /// conversion or receiving role's current time contributes a new allowance.
    pub(super) fn active_deadline(
        &self,
        reserve: Duration,
        original: IdentityDeadline,
    ) -> Result<Option<RoleDeadline>, DeadlineError> {
        if self.earliest.is_none() {
            return Ok(None);
        }
        self.deadline(reserve, original).map(Some)
    }

    pub(super) fn deadline(
        &self,
        reserve: Duration,
        original: IdentityDeadline,
    ) -> Result<RoleDeadline, DeadlineError> {
        let trigger = self.earliest.ok_or(DeadlineError::StopNotStarted)?;
        let stop = trigger.instant.deadline_after(reserve)?;
        match original {
            IdentityDeadline::NeverElapses => Ok(stop),
            IdentityDeadline::Finite { deadline } => {
                if stop.no_later_than(deadline)? {
                    Ok(stop)
                } else {
                    Ok(deadline)
                }
            }
        }
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
    stops: StopTimeline,
    reserve: Duration,
}

impl ExecutionClock {
    /// `whole_bound` is the existing whole-run T (the outer bound for a batch). An overflowing
    /// original checked_add remains None: it neither creates a work cutoff nor refuses admission.
    pub(super) fn prepare(
        original: Option<Instant>,
        whole_bound: Duration,
    ) -> Result<Self, DeadlineError> {
        let reserve = SETTLE_RESERVE.min(whole_bound / 2);
        let started = MonotonicInstant::now()?;
        let work = original
            .map(|deadline| {
                deadline
                    .checked_sub(reserve)
                    .ok_or(DeadlineError::Unrepresentable)
            })
            .transpose()?;
        Ok(Self {
            original,
            work,
            settlement: original,
            stops: StopTimeline::prepare(started)?,
            reserve: if original.is_some() {
                reserve
            } else {
                SETTLE_RESERVE
            },
        })
    }

    pub(super) fn started(&self) -> MonotonicInstant {
        self.stops.started
    }

    /// Adopt only an already authenticated origin record. Validation precedes local conversion;
    /// a late O/I frame retains its producer's absolute trigger and may only shorten C's cutoff.
    pub(super) fn adopt_stop(
        &mut self,
        stamp: StopStamp,
        original: IdentityDeadline,
    ) -> Result<Instant, DeadlineError> {
        let previous = self.stops.earliest;
        let earliest = self.stops.observe(stamp)?;
        if previous == Some(earliest) {
            return self.settlement_deadline();
        }
        let deadline = self.stops.deadline(self.reserve, original)?.local()?;
        let deadline = self
            .settlement
            .map_or(deadline, |previous| previous.min(deadline));
        self.settlement = Some(deadline);
        Ok(deadline)
    }

    /// Called synchronously at C's genuine first stop boundary. Preserve the first C stamp if
    /// another local condition follows; the shared timeline can still adopt an earlier O/I event.
    pub(super) fn capture_caller_stop(
        &mut self,
        original: IdentityDeadline,
    ) -> Result<Instant, DeadlineError> {
        let stamp = match self.stops.last.first().copied().flatten() {
            Some(instant) => StopStamp {
                origin: StopOrigin::Caller,
                instant,
            },
            None => StopStamp::capture(StopOrigin::Caller)?,
        };
        self.adopt_stop(stamp, original)
    }

    pub(super) fn reserve(&self) -> Duration {
        self.reserve
    }

    pub(super) fn stop_deadline(
        &self,
        original: IdentityDeadline,
    ) -> Result<RoleDeadline, DeadlineError> {
        self.stops.deadline(self.reserve, original)
    }

    pub(super) fn stop_stamp(&self) -> Result<StopStamp, DeadlineError> {
        self.stops.earliest()
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

    pub(super) fn settlement_deadline(&self) -> Result<Instant, DeadlineError> {
        self.settlement.ok_or(DeadlineError::StopNotStarted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stamp(origin: StopOrigin, seconds: u64) -> StopStamp {
        StopStamp {
            origin,
            instant: MonotonicInstant {
                seconds,
                nanoseconds: 0,
            },
        }
    }

    /// Trace: FR-034-AC-38.
    #[test]
    fn delayed_producer_trigger_shortens_caller_bound_without_restart() {
        for origin in [
            StopOrigin::Launcher,
            StopOrigin::Outer,
            StopOrigin::Inner,
            StopOrigin::Backend,
        ] {
            let mut timeline =
                StopTimeline::prepare(stamp(StopOrigin::Caller, 10).instant).unwrap();
            timeline
                .observe_at(
                    stamp(StopOrigin::Caller, 20),
                    stamp(StopOrigin::Caller, 22).instant,
                )
                .unwrap();
            let original = IdentityDeadline::NeverElapses;
            assert_eq!(
                timeline.deadline(SETTLE_RESERVE, original).unwrap().seconds,
                21
            );
            timeline
                .observe_at(stamp(origin, 12), stamp(StopOrigin::Caller, 22).instant)
                .unwrap();
            assert_eq!(
                timeline.deadline(SETTLE_RESERVE, original).unwrap().seconds,
                13
            );
            timeline
                .observe_at(
                    stamp(StopOrigin::Caller, 23),
                    stamp(StopOrigin::Caller, 24).instant,
                )
                .unwrap();
            assert_eq!(
                timeline.deadline(SETTLE_RESERVE, original).unwrap().seconds,
                13
            );
        }
    }

    /// Trace: FR-034-AC-15, FR-034-AC-38.
    #[test]
    fn invalid_stamps_cannot_poison_the_retained_earliest_trigger() {
        let mut timeline = StopTimeline::prepare(stamp(StopOrigin::Caller, 10).instant).unwrap();
        let now = stamp(StopOrigin::Caller, 30).instant;
        assert_eq!(
            timeline.observe_at(stamp(StopOrigin::Outer, 9), now),
            Err(DeadlineError::StopBeforeRun)
        );
        assert_eq!(
            timeline.observe_at(stamp(StopOrigin::Outer, 31), now),
            Err(DeadlineError::FutureStop)
        );
        timeline
            .observe_at(stamp(StopOrigin::Outer, 20), now)
            .unwrap();
        assert_eq!(
            timeline.observe_at(stamp(StopOrigin::Outer, 19), now),
            Err(DeadlineError::RegressingStop)
        );
        let mut invalid = stamp(StopOrigin::Inner, 12);
        invalid.instant.nanoseconds = 1_000_000_000;
        assert_eq!(
            timeline.observe_at(invalid, now),
            Err(DeadlineError::InvalidClock)
        );
        assert_eq!(
            timeline
                .deadline(SETTLE_RESERVE, IdentityDeadline::NeverElapses)
                .unwrap()
                .seconds,
            21
        );
        let clipped = RoleDeadline {
            seconds: 20,
            nanoseconds: 500_000_000,
        };
        assert_eq!(
            timeline
                .deadline(
                    SETTLE_RESERVE,
                    IdentityDeadline::Finite { deadline: clipped }
                )
                .unwrap()
                .nanoseconds,
            clipped.nanoseconds
        );
        assert!(serde_json::from_str::<StopStamp>(r#"{"origin":"Outer"}"#).is_err());
    }

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
            let clock = ExecutionClock::prepare(Some(original), bound).unwrap();
            let reserve = SETTLE_RESERVE.min(bound / 2);
            assert_eq!(clock.work_deadline(), original.checked_sub(reserve));
            assert_eq!(clock.original_deadline(), Some(original));
            assert_eq!(clock.settlement_deadline().unwrap(), original);
            assert_eq!(clock.reserve(), reserve);
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
        let trigger = StopStamp::capture(StopOrigin::Caller).unwrap();
        let first = clock
            .adopt_stop(trigger, IdentityDeadline::NeverElapses)
            .unwrap();
        let absolute = trigger.instant.deadline_after(SETTLE_RESERVE).unwrap();
        assert_eq!(
            clock.stop_deadline(IdentityDeadline::NeverElapses).unwrap(),
            absolute
        );
        assert_eq!(
            clock
                .adopt_stop(trigger, IdentityDeadline::NeverElapses)
                .unwrap(),
            first
        );
        assert_eq!(
            clock
                .capture_caller_stop(IdentityDeadline::NeverElapses)
                .unwrap(),
            first
        );
        assert_eq!(clock.stop_stamp().unwrap(), trigger);
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
