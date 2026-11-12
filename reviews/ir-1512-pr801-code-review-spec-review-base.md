---
id: SR-17940486709564847047723
title: "IR-1512 PR #801 spec-review (status flips)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen PR #801 at 4ca035e6; IR-1512; spec/replay/functional/FR-024-counterexample-envelope-intake.md, spec/replay/matrix/TC-035-counterexample-envelope-intake.md, spec/replay/matrix/tests.md, spec/kani/functional/FR-029-run-outcome-terminal-record.md, spec/kani/matrix/tests.md, spec/core/functional/interface-001-codegen-api.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-024
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-029
    type: reviews
---

## Summary

Ticket: IR-1512. The spec diff flips status only: FR-024-AC-58/AC-69/AC-70 PLANNED to
Implemented, TC-035 steps 40/48/49 marked implemented and the Status paragraph rewritten, the
FR-024 and FR-029-AC-16 matrix rows to Covered, and "(planned, IR-1512)" removed from
interface-001 and FR-029-AC-16. No criterion text changed beyond the status words. `make spec`
exits 0 with the 6 pre-existing warnings and none new. The flips match what the tests assert
(see the gap-analysis SR), and the matrix tags in `quire matrix` agree with the rows.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | medium | FR-024-AC-70 (and interface-001) do not state which refusal wins when several apply at once (e.g. a drawn name repeated, or an undeclared name before a duplicate); the code checks per entry in supplied order: drawn/read, undeclared, duplicate, not-ranged, out-of-domain, then `MissingField`. No test pins it: the reviewer mutant reordering the checks (M2) passes all tests. AC-65 states its order explicitly; AC-70 should too, or say the order is unspecified | spec/replay/functional/FR-024-counterexample-envelope-intake.md:609 |
| FND-002 | low | FR-029-AC-16 now reads "`UnsupportedOperationShape` and `UnreadValueForReadField` and `UnreadFieldNotRanged` (FR-024-AC-70, IR-1512)": a doubled "and" left by removing "the planned" | spec/kani/functional/FR-029-run-outcome-terminal-record.md:393 |

## Verdict

Status flips are consistent across FR-024, TC-035, both matrices, FR-029-AC-16 and interface-001.
"Declared order" is handled correctly in the merged AC: AC-69 says the snapshot object is
name-keyed and asserts no order, so the reviewer's order-reversal mutant surviving is expected and
not a claim gap. One medium spec gap (refusal precedence).
