---
id: SR-17940486709564847047939
title: "IR-1526 PR #822 route feature spec review (base checklist)"
type: SpecReview
analysis: base
date: 2026-11-13
scope: "agent-ix/quire-contract-codegen; PR #822 @ a10eb3ca; IR-1526; spec/kani/functional/FR-056-kani-tractability-refusal.md, spec/kani/matrix/TC-070-kani-tractability-refusal.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-056
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-070
    type: references
---

## Summary

Ticket: IR-1526. Base checklist review of PR #822, scoped to `git diff origin/main...HEAD`. The
review covers the route additions to FR-056 (Terms, Inputs, Outputs, the decider seam, the table
schema, Lookup, AC-2/3/4/5/7/10, the new AC-14 and Status) and to TC-070 (steps 3, 4, 5, 7a and 10,
Expected Results and Status).

The spec was read against `src/kani/tractability.rs`, `src/kani/generate/negotiate.rs`
(`IntegerRoute`, `negotiate_integer_policy`), `src/kani/generate/clause/v2.rs`
(`refuse_if_not_tractable`), `tests/it/kani_tractability.rs`,
`tests/it/kani_obligations_v2_clause_controls.rs` and `tests/fixtures/tractability/{table,corpus}.json`.
The integrity sub-analysis is in its own file (SR-17940486709564847047940).

`QSPEC_DIR=/home/peter/dev/quire-specification quire validate` on both files exits 0. It reports
two `ac:non-canonical-shape` warnings. The AC-8 warning (given-when-then) is already on main. The
AC-10 warning (obligation-shaped) is new in this PR (FND-002). AC-14 raises no warning.

## Verdict

**Not mergeable as is: one high and two medium findings. All are small text fixes.**

What is right:

- The route is one feature and one key. `ClauseFeatures` and `TractabilityRow` both use
  `deny_unknown_fields`. A `route` field with no serde default makes the key required in the
  table, the corpus `features` and the decider input. No other mechanism is needed. The other
  changes are small: `classify_v2_clause` already holds the `native` flag and has to pass it to
  `refuse_if_not_tractable`, `V2ClauseNotTractable` gets a route field, and `IntegerRoute`
  gets snake_case serde. The PR adds no compatibility layer. "there is no default route" and AC-3
  make a row without `route` fail to load.
- Lookup is same-route only. It is stated in Behavior and tested in AC-14 in both directions, with
  the `tractable` row case, the reported route and the cross-route precedence case. The mutant is
  real.
- AC-4 adds the route to the refusal fields and the mutant. AC-5 holds the route fixed. AC-3 adds
  the missing and unknown route cases. TC-070 steps 3, 4, 5, 7a and 10 match.
- The author's open questions. (1) `route` as the corpus feature name: agreed, it matches the table
  key. (2) AC-13 narrow multiply stays on the general route: agreed. The test comment and the AC-13
  host test both record that the native route cannot render a two-parameter precondition. (3)
  The 15-bit rows are valid as general: agreed for the two table rows and the four `ir1526-*`
  corpus rows, whose sources say "General route". The other ten corpus rows are not general
  (FND-001).

## Findings

| ID      | Severity | Summary | Refs |
| ------- | -------- | ------- | ---- |
| FND-001 | high     | Status says the shipped corpus rows "are all general-route and must be given `route: general`". Ten of the fourteen are native-route measurements. These are the `ir1022-*` and `ir1509-*` rows, and both source docs say the harness was generated through `negotiate_kani_native_obligations`. Following the Status would mislabel ground-truth features, which is the confound the route exists to remove. Fix: the `ir1022-*` and `ir1509-*` rows get `native` and the table rows and `ir1526-*` rows get `general`. Those rows (integer add at 10 bits, native) already supply AC-10's below-15-bit native add case. | spec/kani/functional/FR-056-kani-tractability-refusal.md:141-144, tests/fixtures/tractability/corpus.json:37-370, docs/qualification/ir1022-kani-clause-measurements.md:8-9, docs/qualification/ir1509-kani-clause-controls.md:9-11 |
| FND-002 | medium   | AC-10 says "The corpus SHALL need native-route measurements …" and then "Whether these rows exist is the FR-056 Status; the criterion does not claim them". An Inspection criterion that disowns its own content cannot fail, and the SHALL triggers the new `ac:non-canonical-shape` (obligation-shaped) warning. Fix: state it as a direct assertion ("the corpus holds, for each of …, at least one measured native-route row below 15 bits and one at or above 15 bits"), and leave the PARTIAL state to Status. | spec/kani/functional/FR-056-kani-tractability-refusal.md:122 |
| FND-003 | medium   | AC-14 and TC-070 step 7a say the clause "runs on the `native` route" and "request the same multiply at 21 bits on `IntegerRoute::Native` (runs)". At the negotiation level, the multiply fixtures (`other * offset …`) end `RenderFailed` on the native route, because it renders no two-parameter precondition. So "runs" cannot mean a harness. Fix: say what is asserted. Either the decider returns `Decision::Run` for the features with route `native`, or the negotiated item is not refused as `V2ClauseNotTractable`. Related: the existing host test `tc_070_the_ac13_multiply_fixtures_split_on_the_row_bound` asserts that the native wide multiply is refused by a route-less row ("The decision does not depend on the route"). It must flip when the route lands. | spec/kani/functional/FR-056-kani-tractability-refusal.md:126, spec/kani/matrix/TC-070-kani-tractability-refusal.md:39-42, tests/it/kani_tractability.rs:901-938 |
