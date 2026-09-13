---
id: TC-020
title: "Verify numeric conformance campaigns and rate reporting"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-011
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/NFR-004
    type: verifies
---
# TC-020: Verify numeric conformance campaigns and rate reporting

## Description

Verify that the generated conformance runner maps oracle results to verdicts by clause kind, records
them without global rejects, runs the census exhaustively, detects oracle disagreement, and reports
exact rates.

## Test Procedure

1. In a generated-crate fixture, run `Satisfying`, `Violating`, and `Broad` campaigns through the
   generated population runner for `VersionUnchanged` (`Postcondition`), `amount < 7` (`Invariant`),
   and a `Precondition` fixture `amount <= 500`, each with a fresh report, at 256 cases under the
   default configuration and at 10,000 cases.
2. Repeat step 1's 256-case `Broad` campaigns with each generated oracle replaced by its negation.
3. Run the census runner for `VersionUnchanged` with an instrumented oracle that records every
   valuation it receives.
4. Run a campaign over a report seeded with pinned prior accepted, rejected, and discarded counts,
   then read both rates; read both rates on a fresh zero-attempt summary and on a report driven to
   `at_limit`.
5. Read the runner's proptest global reject count and each campaign conclusion from steps 1 and 4.

## Expected Results

- `VersionUnchanged` records `Holds` as `Passed` and `Violated` as `FailedPostcondition`; the
  `Precondition` fixture records `Violated` as `RejectedPrecondition` in `rejected`; `amount < 7`
  records `Violated` as `FailedPostcondition` with an `Invariant` observation; every campaign passes.
- Each negated-oracle campaign fails with `ConformanceMismatch` carrying the case values, expected
  tag, and observed verdict kind.
- The instrumented oracle receives exactly the 10 in-domain census cases, once each, in census order,
  and no out-of-domain case.
- The rates equal the pinned `(discarded, attempted)` and `(rejected, attempted)`, and both are `None`
  at zero attempts and at `at_limit`.
- Every fresh campaign in step 1 reports `discard_rate() == Some((0, attempted))` with
  `attempted > 0`, passes a zero discard ceiling, records zero global rejects, and does not end
  `Exhausted`.
