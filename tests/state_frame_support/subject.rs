//! The operation subject of the state-frame fixture, compiled natively by the test binary and
//! embedded byte-for-byte in every Kani crate that proves a harness over it.
//!
//! Each variant acts only on an account that already holds funds. Kani prints one playback per
//! distinct input valuation, and when the failing check and the non-vacuity cover share a
//! valuation it prints the cover's; a defect that also showed at the all-zero valuation would
//! then read as a failure without a counterexample instead of as the counterexample it is.

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
    if account.balance > 0 && account.balance < 1000 {
        account.balance += 1;
    }
}

/// Seeded defect: also rewrites `audit`, which the frame does not grant.
pub fn deposit_touching_audit(account: &mut Account) {
    if account.balance > 0 && account.balance < 1000 {
        account.balance += 1;
        account.audit = account.audit.wrapping_add(1);
    }
}

/// Seeded defect: debits instead of crediting.
pub fn deposit_debiting(account: &mut Account) {
    if account.balance > 0 {
        account.balance -= 1;
    }
}
