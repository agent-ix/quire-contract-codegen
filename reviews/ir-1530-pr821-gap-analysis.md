---
id: SR-17940486709564847047956
title: "IR-1530 PR #821 gap-analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen PR #821 at c3a58c73, git diff origin/main...HEAD; IR-1530; FR-056-AC-14, FR-056-AC-9, FR-017-AC-27, TC-070 step 14; src/kani/terminal/tractable.rs, src/kani/tractability.rs, tests/it/kani_tractability.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-056
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-070
    type: reviews
---

## Summary

Ticket: IR-1530. Plan completion: not assessed. Computed matrix (`quoin matrix --json` at
c3a58c73, quoin 0.29.1): FR-056-AC-14 is bound by two default-lane tests,
`tc_070_a_general_route_scalar_harness_is_refused_before_kani_with_its_row` and
`tc_070_native_route_and_v2_clause_features_are_not_refused_by_the_general_row` (status "no run
evidence", static tagged). Both assert real values: the refusal carries node id, operation id, width
4 and the full shipped row with versions, and a missing launcher and crate prove no launch. The
native harness reaches the runner's own refusal; V2-route features at the same width run; a
narrower general-route width runs. No new production code lacks an owning criterion
(`tractable.rs` and the `route` field are owned by FR-056-AC-14). No stubs, no tautologies.

Examined: FR-056-AC-14 (bound, FND-001 and FND-002), FR-056-AC-9 (scan narrowed, see code-review
FND-002), FR-017-AC-27 (text-only edit, no new binder needed), TC-070 step 14 (matches the tests).

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | low | FR-056-AC-14 says "a batch is refused whole", but the batch assertion uses a one-member batch, so a mutant that refuses only the offending member, or skips the check after the first member, is not distinguished. Add a mixed batch (native harness plus general harness, general second) that is refused `NotTractable` with no launch. | tests/it/kani_tractability.rs:1004-1006, src/kani/terminal/tractable.rs:121-129 |
| FND-002 | low | AC-14 says "a row without a `route` key applies to V2 clauses only", but no test loads a route-less row against general-route features; only the reverse direction (general row vs V2 features) is exercised. A mutant that treats a route-less row as matching every route passes. Add a route-less `not_tractable` row for `quire.op.integer.add` at 4 bits and assert general-route features at 4 bits run on it. | tests/it/kani_tractability.rs:1011-1036, src/kani/tractability.rs:208-212 |
| FND-003 | low | The row is width-thresholded at 4 bits, while the qualification doc attributes the cost to the `rt::Integer` heap shape, not to width. A general-route harness over an operand bound of 3 bits or fewer (for example `Int[0, 7]`) is not refused and is expected to hit the 600 s ceiling instead. This matches the measurement (nothing narrower was probed), but the doc should state that narrower widths are unmeasured and left to the backstop. | docs/qualification/ir1530-general-route-scalar-table.json:5, docs/qualification/ir1530-general-route-scalar-not-tractable.md:49-50 |

## Verdict

AC-14 is substantively covered and the assertions would fail on the main mutants the AC names
(route-blind matching, refusing the native harness). The three findings are hardening of edges
the AC text claims; none blocks merge.
