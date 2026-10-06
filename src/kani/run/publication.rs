//! Unconditional private monotonic stage facts; publication performs no I/O or handoff.

use std::sync::{
    atomic::{AtomicU64, AtomicU8, Ordering},
    OnceLock,
};

/// Actual shared executor boundaries, in production transition order.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub(super) enum Stage {
    BeforeMonitor = 1,
    Bootstrap,
    ClaimedGated,
    ClaimedBootstrap,
    InitReady,
    Dispatched,
    LeaseClosing,
}

/// Sealed at publication, so a subsequent close cannot repair an early publication.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct CloseOrder {
    pub(super) completed_close: Option<u64>,
    pub(super) publication: u64,
}

/// One producer records completed work; fixture readers cannot modify any observation.
#[derive(Default)]
pub(super) struct Publication {
    stage: AtomicU8,
    ordinal: AtomicU64,
    completed_close: AtomicU64,
    close_order: OnceLock<CloseOrder>,
}

impl Publication {
    pub(super) fn publish(&self, stage: Stage) {
        let ordinal = self.ordinal.fetch_add(1, Ordering::SeqCst) + 1;
        if stage == Stage::LeaseClosing {
            let closed = self.completed_close.load(Ordering::SeqCst);
            let _ = self.close_order.set(CloseOrder {
                completed_close: (closed != 0).then_some(closed),
                publication: ordinal,
            });
        }
        self.stage.fetch_max(stage as u8, Ordering::Release);
    }

    /// Called only after the owned caller endpoint's close returns.
    pub(super) fn lease_closed(&self) {
        let ordinal = self.ordinal.fetch_add(1, Ordering::SeqCst) + 1;
        self.completed_close.store(ordinal, Ordering::SeqCst);
    }

    pub(super) fn stage(&self) -> Option<Stage> {
        match self.stage.load(Ordering::Acquire) {
            1 => Some(Stage::BeforeMonitor),
            2 => Some(Stage::Bootstrap),
            3 => Some(Stage::ClaimedGated),
            4 => Some(Stage::ClaimedBootstrap),
            5 => Some(Stage::InitReady),
            6 => Some(Stage::Dispatched),
            7 => Some(Stage::LeaseClosing),
            _ => None,
        }
    }

    #[cfg(any(feature = "guardian-test-support", test))]
    pub(super) fn close_order(&self) -> Option<CloseOrder> {
        self.close_order.get().copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Trace: FR-034-AC-24, FR-034-AC-28
    #[test]
    fn publication_seals_actual_close_order_without_late_fill() {
        let normal = Publication::default();
        normal.publish(Stage::InitReady);
        normal.lease_closed();
        normal.publish(Stage::LeaseClosing);
        let order = normal.close_order().unwrap();
        assert!(order.completed_close.unwrap() < order.publication);
        assert_eq!(normal.stage(), Some(Stage::LeaseClosing));

        let early = Publication::default();
        early.publish(Stage::LeaseClosing);
        let sealed = early.close_order().unwrap();
        assert_eq!(sealed.completed_close, None);
        early.lease_closed();
        assert_eq!(early.close_order(), Some(sealed));
        early.publish(Stage::InitReady);
        assert_eq!(early.stage(), Some(Stage::LeaseClosing));
    }
}
