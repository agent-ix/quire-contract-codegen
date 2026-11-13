---
id: SR-17940486709564847047942
title: "IR-1526 PR #823 route as a tractability decider feature gap analysis"
type: SpecReview
analysis: gap-analysis
date: 2026-11-13
scope: "agent-ix/quire-contract-codegen; PR #823 @ 73e4d64a; IR-1526; FR-056-AC-2, FR-056-AC-3, FR-056-AC-4, FR-056-AC-5, FR-056-AC-7, FR-056-AC-10, FR-056-AC-13, FR-056-AC-14, TC-070; src/kani/tractability.rs, src/kani/generate/, tests/it/kani_tractability.rs, tests/it/kani_obligations_v2_clause_controls.rs, tests/fixtures/tractability/"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-056
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-070
    type: references
---

## Summary

Ticket: IR-1526. Planless gap analysis of PR #823, scoped to `git diff origin/main...HEAD`.
Plan completion: not assessed. It covers the route additions of FR-056 (Terms, Lookup,
AC-2 to AC-5, AC-7, AC-10, AC-14) and TC-070 steps 2 to 4, 7a and 10 against the changed source
and tests.

## Verdict

**CONDITIONAL** (one low finding). Every route criterion has a test or an inspectable property
that the PR changed: AC-3 (missing and unknown route fail the load at the row index), AC-4
(refusal carries `route`, asserted by full equality), AC-14 (two tagged tests, decider level and
generation level, mutant-killed), AC-10 (every corpus row's route asserted by id family; required
by the type), AC-13 (installed-Kani fixture rows carry `route: general`). No reverse gap: the
only new code (`IntegerRoute` serde, `from_native`, the route field and check) is owned by
FR-056 Terms and Lookup. No stubs or placeholder returns.

## Findings

| ID      | Severity | Summary | Refs |
| ------- | -------- | ------- | ---- |
| FND-001 | low      | FR-056-AC-2's route half ("CG also exposes the route of the negotiation as a clause feature ... reporting a fixed route fails the route check") has no test tagged FR-056-AC-2. The behaviour is tested: the fixed-route mutant is killed by `tc_070_the_negotiated_route_decides_the_refusal`, but that test is tagged only `FR-056-AC-14`, and both AC-2-tagged tests still exercise widths only. Add `FR-056-AC-2` to that test's `#[trace]`. | tests/it/kani_tractability.rs:1020-1023, tests/it/kani_tractability.rs:192-194 |

## Coverage

- Criteria examined: FR-056-AC-2, AC-3, AC-4, AC-5, AC-7, AC-10, AC-13, AC-14; TC-070 steps 2-4,
  7a, 10. AC-5 and AC-7 changes are wording only (route held fixed / route is not a version).
- `quire trace --id FR-056-AC-14` resolves two structural claims
  (`tc_070_lookup_matches_rows_only_for_the_clause_route`,
  `tc_070_the_negotiated_route_decides_the_refusal`).
- Tool discrepancy (not a PR finding): `quire matrix --scope . --format json` (quire 0.44.0,
  engine 0.59.1) reports every FR-056 test criterion `untagged` while `quire trace` resolves
  their `#[trace]` claims; FR-055 shows the same. Pre-existing, independent of this diff.
- Semantic review: skipped (not requested).
- Plan completion: not assessed
