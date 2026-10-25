---
id: TC-020
title: "Verify numeric conformance campaigns and rate reporting"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-011
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/FR-004
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

1. In a generated-crate fixture built from IR-admitted `CheckedPackageV2` selected claims, run `Satisfying`, `Violating`, and `Broad` campaigns through the
   generated population runner for `VersionUnchanged` (`Postcondition`), `amount < 7` (`Invariant`),
   and a `Precondition` fixture `amount <= 500`, each with a fresh report, at 256 cases under the
   deterministic configuration with `max_global_rejects: 0` and at 10,000 cases.
2. Repeat step 1's 256-case `Broad` campaigns with each selected claim's V2 comparison oracle
   replaced by its negation.
3. Run the census runner for `VersionUnchanged` with an instrumented oracle that records every
   valuation it receives.
4. Run a campaign over a report seeded with fixed prior accepted, rejected, and discarded counts,
   then read both rates; read both rates on a fresh zero-attempt summary and on a runtime snapshot
   decoded with counters at `u64::MAX`. The runtime deliberately exposes no conversion from
   an imported snapshot back to a mutable report, so the generated snapshot-summary operation is
   the public boundary for this saturated-rate check.
5. Read the runner's proptest global reject count and each campaign conclusion from steps 1 and 4.
6. Run a generated campaign with a report whose requirement identity is wrong and whose prior
   discarded count already exceeds the configured ceiling.
7. Generate from accepted QSL bindings at distinct typed points: one invariant at an initialization
   anchor, two otherwise equivalent invariants at different handler anchors, and precondition and
   postcondition bindings. Instrument `construct_verdict` and compare each runtime string with the
   source binding's variant, UTF-8 anchor name and exact byte-length rendering. Keep clause kind
   fixed while changing only the invariant binding's anchor, then compare the runtime strings.
   Use one admitted package with one invariant checked clause node and two authentic `claim`
   occurrences bound to distinct initialization and handler points. Select each exact `(checked
   clause id, claim occurrence)` in turn and compare its constructed verdict point with that
   occurrence's accepted binding; also vary the handler name on a third authentic claim.
8. Independently select a wrong-role, wrong-ordinal, and other-node occurrence against a checked
   clause with another authentic claim that has a point; verify that the sibling point cannot satisfy
   the lookup. Supply an authentic claim lacking its accepted point beside another authentic claim
   of the same node that has a point; inspect the generation refusal with no sibling substitution.
   Also select an absent clause id and a present non-state-clause id to inspect the mapped Contract IR
   FR-058 refusals before occurrence checking. Independently apply a renderer mutant that changes only the generated
   runner's point after binding selection; inspect the refusal before artifact publication.

## Expected Results

- `VersionUnchanged` records `Holds` as `Passed` and `Violated` as `FailedPostcondition`; the
  `Precondition` fixture records `Violated` as `RejectedPrecondition` in `rejected`; `amount < 7`
  records `Violated` as `FailedPostcondition` with an `Invariant` observation; every campaign passes.
- Each negated-oracle campaign fails with `ConformanceMismatch` carrying the case values, expected
  tag, and observed verdict kind.
- The instrumented oracle receives exactly the 10 in-domain census cases, once each, in census order,
  and no out-of-domain case.
- The rates equal the seeded `(discarded, attempted)` and `(rejected, attempted)`, and both are `None`
  at zero attempts and at `at_limit`.
- Every fresh campaign in step 1 reports `discard_rate() == Some((0, attempted))` with
  `attempted > 0`, passes a zero discard ceiling, records zero global rejects, and does not end
  `Exhausted`.
- The report in step 6 returns `IdentityMismatch`; its prior discard does not mask the identity
  failure as `AboveDiscardCeiling`.
- Step 7 preserves each accepted binding point through injective runtime rendering, including both
  invariant variants and both distinct handler names. Changing only the invariant's bound anchor
  changes the runtime point while its clause kind remains `Invariant`. The same checked node's two
  authentic claim occurrences produce distinct points with no node-level reuse.
- Each wrong occurrence in step 8 returns `StrategyErrorCode::MissingClaimOccurrence` with no
  artifact even when a sibling claim has a point. The authentic claim lacking a point returns
  `StrategyErrorCode::MissingExecutionPoint` despite its pointed sibling. An absent clause id returns
  `StrategyErrorCode::UnknownClause` and a non-state-clause returns
  `StrategyErrorCode::UnsupportedClauseKind`, each before occurrence checking and without an
  artifact. The renderer mutant returns `StrategyErrorCode::ExecutionPointMismatch` before
  publication and produces no artifact. Clause kind supplies no default point.
