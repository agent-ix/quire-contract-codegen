---
id: FR-011
title: "Run numeric campaigns through the tri-state runner and report rates"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-009
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-010
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-002
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/interface-001
    type: implements
---
# FR-011: Run numeric campaigns through the tri-state runner and report rates

## Description

When numeric populations from [FR-009](./FR-009-constructive-correlated-populations.md) or
[FR-010](./FR-010-domain-boundary-campaigns.md) run, the generated campaign runner shall map each
case's expectation onto the tri-state verdict for the clause's anchor. It shall keep out-of-domain
cases out of truth evaluation, and its summary shall report the discard rate and the rejection rate
exactly.

## Inputs

- A generated numeric population and the clause's `ExecutionPoint` anchor.
- The existing campaign policy: minimum accepted, minimum rejected, and maximum discarded counts.
- A consumer-supplied domain-admission result for each `OutOfDomain` case.

## Outputs

- The existing generated campaign summary and error types, extended with two rate accessors.

## Behavior

- For a `Pre`-anchored clause:
  - a `Holds` case shall expect an accepted verdict, `Passed` or `FailedPostcondition`;
  - a `Violated` case shall expect `RejectedPrecondition`.
- For a `Post`-anchored clause:
  - a `Holds` case shall expect `Passed`;
  - a `Violated` case shall expect `FailedPostcondition`.
- For any other anchor, the generator shall refuse with `UnsupportedAnchor` until that anchor's
  verdict mapping is specified.
- The runner shall not evaluate the oracle for an `OutOfDomain` case. It shall require the consumer's
  domain-admission result for that case to be a refusal. It shall fail the campaign with
  `ExpectationMismatch` if the consumer admits the value.
- The runner shall count each consumer-refused `OutOfDomain` case in a separate `out_of_domain`
  summary counter, outside accepted, rejected, and discarded.
- The runner shall not convert an `OutOfDomain` case into a `false` verdict.
- The generated summary shall expose `discard_rate()` and `rejection_rate()`:
  - each returns the exact pair `(numerator, attempted)`, discarded or rejected over attempted, using
    the existing `attempted` denominator from interface-001's `accounting_unit`;
  - each returns `None` when `attempted` is zero, never a zero rate.
- Constructive numeric populations shall never invoke the explicit-discard channel.
- If a constructive population's campaign observes a non-zero `discarded` count, then the runner shall
  report that count unchanged and fail a policy whose discard ceiling is zero.
- Existing floor, ceiling, `Exhausted`, and `Failed` conclusions from
  [FR-002](../FR-002-tristate-proptest.md) shall apply unchanged to numeric campaigns.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-011-AC-1 | A `Post`-anchored `VersionUnchanged` campaign maps `Holds` to `Passed` and `Violated` to `FailedPostcondition`; a `Pre`-anchored integer clause maps `Violated` to `RejectedPrecondition`; a mismatch fails the campaign with the case values in the failure. | Test (TC-020) |
| FR-011-AC-2 | An `OutOfDomain` case never reaches the oracle adapter; a consumer that admits it fails the campaign with `ExpectationMismatch`; a consumer that refuses it increments `out_of_domain` and no other counter. | Test (TC-020) |
| FR-011-AC-3 | A seeded campaign with known counts returns `discard_rate() == Some((discarded, attempted))` and `rejection_rate() == Some((rejected, attempted))` with the exact pinned values; a zero-attempt summary returns `None` for both. | Test (TC-020) |
| FR-011-AC-4 | A constructive numeric campaign reports `discard_rate() == Some((0, attempted))` with `attempted > 0`, and passes a zero discard ceiling. | Test (TC-020) |
| FR-011-AC-5 | A clause anchored at `Initialization` or `Handler` is refused with `UnsupportedAnchor`. | Test (TC-020) |

## Dependencies

- **Upstream**: [FR-002](../FR-002-tristate-proptest.md) campaign runner,
  [FR-009](./FR-009-constructive-correlated-populations.md),
  [FR-010](./FR-010-domain-boundary-campaigns.md).
- **Downstream**: [FR-013](./FR-013-it010-consumable-output.md),
  [TC-020](../../test/strategies/TC-020-numeric-harness-campaigns.md).
