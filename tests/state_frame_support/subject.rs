//! The operation subject of the state-frame fixture, compiled natively by the test binary and
//! embedded byte-for-byte in every Kani crate that proves a harness over it.
//!
//! Each variant acts at the all-zero valuation, and `deposit_debiting` at every valuation;
//! `deposit` and `deposit_touching_audit` act at every `balance` below 1000. Kani prints one
//! playback per distinct input valuation, so a defect that shows at a valuation the non-vacuity
//! cover also takes is the case a harness that places its cover before its assertion loses: the
//! cover's playback is printed and the failure reads as one with no counterexample (IR-451).
//! These subjects keep that case in the lane rather than seeding around it.

/// The operation's state: the two fields the domain package declares.
#[derive(Clone)]
pub struct Account {
    /// The field the operation's frame grants.
    pub balance: i64,
    /// The field no frame grants.
    pub audit: i64,
}

/// Credits one unit to `balance`, the field the frame grants.
pub fn deposit(account: &mut Account) {
    if account.balance < 1000 {
        account.balance += 1;
    }
}

/// Seeded defect: also rewrites `audit`, which the frame does not grant, at every valuation the
/// operation credits, the all-zero one among them.
pub fn deposit_touching_audit(account: &mut Account) {
    if account.balance < 1000 {
        account.balance += 1;
        account.audit = account.audit.wrapping_add(1);
    }
}

/// Seeded defect: debits instead of crediting, at every valuation.
pub fn deposit_debiting(account: &mut Account) {
    account.balance = account.balance.wrapping_sub(1);
}

/// Seeded defect: debits instead of crediting, but never below the floor of `balance`'s declared
/// range. It violates the postcondition at every valuation but the floor, and its post state
/// always lies inside the model's range, which QSL's snapshot admission requires: the playback
/// `deposit_debiting` is falsified by may be the floor, where it runs to a value outside the range
/// and QSL refuses the post snapshot.
pub fn deposit_debiting_within_range(account: &mut Account) {
    if account.balance > 0 {
        account.balance -= 1;
    }
}
