---
id: TC-020
title: "Verify numeric campaign verdict mapping and rate reporting"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-011
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/NFR-004
    type: verifies
---
# TC-020: Verify numeric campaign verdict mapping and rate reporting

## Description

Verify that numeric cases map to tri-state verdicts by anchor, that out-of-domain cases are kept out
of truth evaluation, and that the discard and rejection rates are exact.

## Test Procedure

1. In a generated-crate fixture, run a `Post`-anchored `VersionUnchanged` `Broad` campaign through the
   generated runner. Use a subject that preserves the version. Then use a subject that increments it.
2. Run a `Pre`-anchored `amount < 7` `Broad` campaign.
3. Run a `Boundary` campaign whose consumer domain check refuses `OutOfDomain` values. Then run it with
   a consumer that admits them. Count the adapter invocations.
4. Run a seeded mixed campaign with pinned counts, including one explicit discard supplied in the
   prior report. Read `discard_rate()` and `rejection_rate()`. Read both again on a fresh zero-attempt
   summary.
5. Run a 10,000-case constructive campaign with discard ceiling 0.
6. Request a campaign for an `Initialization`-anchored clause.

## Expected Results

- The preserving subject passes. The incrementing subject yields `FailedPostcondition` for `Holds`
  cases, which fails the campaign with the case values in the failure.
- `amount < 7` `Violated` cases return `RejectedPrecondition`.
- `OutOfDomain` cases never invoke the adapter. The refusing consumer increments only `out_of_domain`.
  The admitting consumer fails with `ExpectationMismatch`.
- The rates equal the pinned `(discarded, attempted)` and `(rejected, attempted)`, and both are `None`
  at zero attempts.
- The constructive run reports `Some((0, attempted))` with `attempted > 0` and passes the zero ceiling.
  Its global reject count and `Exhausted` count are 0.
- The `Initialization` anchor is refused with `UnsupportedAnchor`.
