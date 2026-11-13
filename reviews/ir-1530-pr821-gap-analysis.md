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

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-004 | medium | FR-056-AC-2's route half has no binder. The two AC-2 tests check widths only, and TC-070 step 2 ("Assert the exposed route is `general` for an `IntegerRoute::General` negotiation and `native` for `IntegerRoute::Native` on the same clause; a decider fed a fixed route fails") is not implemented: no spy decider records the `ClauseFeatures` the negotiation builds. Yet TC-070 Status says "the route additions (steps 2 to 4, 7a and 10) are implemented (IR-1530)". A fixed-route mutant of `native_route(native)` is killed only incidentally, by `tc_070_the_ac13_multiply_fixtures_split_on_the_row_bound`, which is tagged FR-056-AC-13 alone, so the matrix shows AC-2 bound by width tests while its route clause is untested under that id. Add a recording-decider test tagged FR-056-AC-2: negotiate `a * b + c` on both routes and assert `features.route` each time. | tests/it/kani_tractability.rs:188-270, src/kani/generate/negotiate.rs:336-342, spec/kani/matrix/TC-070-kani-tractability-refusal.md:22-25, spec/kani/matrix/TC-070-kani-tractability-refusal.md:82 |
| FND-005 | low | The negotiation-level evidence for FR-056-AC-14 (TC-070 step 7a: a general row refuses the general clause and not the native one, and the reverse, with the refusal reporting `IntegerRoute::Native`) lives only in `tc_070_the_ac13_multiply_fixtures_split_on_the_row_bound`, tagged FR-056-AC-13. The only AC-14 binder, `tc_070_rows_match_only_their_own_route`, is decider-level. Add `FR-056-AC-14` (and `FR-056-AC-2`, `FR-056-AC-4` for the route report) to that test's trace attribute. | tests/it/kani_tractability.rs:923-999, tests/it/kani_tractability.rs:1143 |

## Dispositions

Round 1, checked against 5443b94b.

| FND | outcome | reason |
|---|---|---|
| FND-001 | fixed | A mixed batch `[native_request, request]` (general second) is asserted `NotTractable` with the general harness path (tests/it/kani_tractability.rs:1021-1031). |
| FND-002 | fixed | A route-less `not_tractable` row for `quire.op.integer.add` at 4 bits leaves general-route features at 4 bits `Run` and refuses the V2 features (tests/it/kani_tractability.rs:1068-1077). |
| FND-003 | fixed | The qualification doc now says widths below the row's 4 bits are unmeasured, are not refused and are left to the wall-clock backstop; FR-056 Description says the same. |

The new test `tc_070_the_embedded_general_route_table_is_the_docs_file` compares the embedded table with the docs file loaded at run time. Both come from the same file, so they are equal by construction. What the test does pin is that the `include_bytes!` path points at the docs file, not at a copy, and that the file parses to exactly one row. The real coverage is that both refusal tests now get their table from `shipped_general_route_scalar()`, so the shipped path itself is exercised. That is enough for its purpose, and it is not a defect.

Round 2, checked against 7a7ceaa9. FND-001 and FND-003 keep `fixed`: the mixed batch and the
unmeasured-width note survive. FND-002's route-less-row test was replaced, as main's FR-056 now
requires: a row or features without `route` is malformed
(`tc_070_a_missing_or_unknown_route_is_malformed`, tagged FR-056-AC-3), so the round-1 fix is
superseded rather than regressed. Trace tags were renumbered: the three scalar tests are tagged
FR-056-AC-15, and FR-056-AC-14 now binds `tc_070_rows_match_only_their_own_route`.

| FND | outcome | reason |
|---|---|---|
| FND-004 | still-open | New in round 2; no recording-decider test for the AC-2 route feature, and TC-070 Status overclaims step 2. |
| FND-005 | still-open | New in round 2; the negotiation-level AC-14 evidence is not yet tagged AC-14. |

Round 3, checked against 5e6e2bbe.

| FND | outcome | reason |
|---|---|---|
| FND-004 | fixed | `tc_070_the_decider_receives_the_negotiation_route` (tagged FR-056-AC-2) negotiates `other * offset + 3 <= 100` through `negotiate_kani_obligations_with_decider` with a recording decider on each route and asserts one decision whose `features.route` is that route, with the same operations; a fixed-route mutant fails one of the two asserts. TC-070 step 2 now names the recording decider, so its Status claim holds. |
| FND-005 | fixed | `tc_070_the_ac13_multiply_fixtures_split_on_the_row_bound` is now tagged `FR-056-AC-13`, `FR-056-AC-14`, `FR-056-AC-4`, `TC-070`. |
