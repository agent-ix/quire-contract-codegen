---
id: SR-17940486709564847047749
title: "IR-1437 PR #810 kani-mutations target failure-domain review"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-contract-codegen PR #810; IR-1437; spec/kani/functional/FR-055-kani-mutations-target.md; spec/kani/matrix/TC-069-kani-mutations-target.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-055
    type: reviews
---

## Summary

Ticket: IR-1437. Failure-domain lens over FR-055's outcome model and TC-069, compared with the
sibling quire-contract-runtime `scripts/check_kani_mutations.py` (reference only, read at
7caefac). FR-055 says it "uses the vocabulary and contract shape" of that target. The four outcome
words match. Their meanings do not fully match, and the non-`pass` failure modes FR-055 names
(timeout, inconclusive, Kani crash) have no outcome assigned.

## Verdict

**FAIL.** Two medium findings and one low finding. None is a wrong result on the main path. Each
one leaves two implementers free to print different outcomes for the same run.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Timeout, inconclusive and a non-zero exit that never reached a failed check are "not `pass`" but map to none of the four outcomes, although Behavior requires exactly one of the four and AC-2 prints only those four | FR-055-AC-5, FR-055, FR-055-AC-2 |
| FND-002 | medium | Outcome meanings diverge from the runtime target FR-055 claims to match: runtime `malformed` is anchor-count only and a non-compiling or non-verifying-failure run is `fail`; FR-055 makes non-compiling `malformed`. Runtime also refuses below a 3-mutation floor (exit 2), which FR-055 lacks | FR-055 |
| FND-003 | low | AC-2 says a mutant not run does not appear as run, but under AC-6 every mutant is `unavailable` (not run); whether those rows are printed and counted in the total is unspecified | FR-055-AC-2, FR-055-AC-6 |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-055 | examined | Each mutant has exactly one outcome: `pass`, `fail` (a surviving mutant), `malformed` (the anchor text no longer occurs exactly once, or the mutant does not compile) or `unavailable` (no `cargo-kani`). A mutant that times out, is inconclusive, is skipped, or fails only a dirty-tree or other process guard is not `pass`. |
| FR-055-AC-2 | examined | one line per mutant run ... outcome (`pass`, `fail`, `malformed` or `unavailable`), followed by a total. A mutant that was not run does not appear as run. |
| FR-055-AC-5 | examined | anchor absent or duplicated, does not compile, times out or is inconclusive each give an outcome other than `pass` and a non-zero exit. |
| FR-055-AC-6 | examined | With no `cargo-kani` on `PATH`, every mutant is `unavailable` and the target exits non-zero. |
| TC-069 steps 4-5 | examined | stand-in verifies one mutant -> `fail`; anchor removed/duplicated, uncompilable mutant, timing-out stand-in -> non-`pass`, non-zero; no `cargo-kani` -> all `unavailable`. |
| quire-contract-runtime check_kani_mutations.py run_mutation | context_only | returncode 0 -> `fail`; non-zero without `Checking harness ...` and `VERIFICATION:- FAILED` -> `fail` ("did not reach a proof failure"); anchor count != 1 -> `malformed`; below MINIMUM_KANI_MUTATIONS -> exit 2. |
