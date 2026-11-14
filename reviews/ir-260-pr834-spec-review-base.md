---
id: SR-17940486709564847047987
title: "IR-260 PR #834 spec-review base"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen PR #834 at 55457462, git diff origin/main...HEAD; IR-260 and IR-262; spec/kani/functional/FR-053-float-scalar-harnesses.md, spec/kani/matrix/TC-067-float-scalar-harnesses.md, spec/kani/matrix/tests.md, spec/kani/functional/FR-049-enum-scalar-harnesses.md, spec/kani/matrix/TC-063-enum-scalar-harnesses.md, docs/qualification/ir260-float-scalar-kani-measurements.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-053
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-067
    type: reviews
---

## Summary

Ticket: IR-260 (IR-262 for the FR-049 and TC-063 edits). This review covers the spec changes: the
FR-053-AC-12 amendment (no tool versions, QSL-406/EA-49), the new FR-053 and TC-067 PARTIAL
statuses, the FR-053 matrix row and the FR-049/TC-063 status edits.

Checked clean. The AC-12 and TC-067 step 12 amendment drops the Kani/Rust/CBMC/solver versions and
adds "every backend option", matching the ruling. The measurement doc records target, unwind, the
full option vector and every outcome, with no tool versions. Its Verdicts table matches the shipped
routed harness: the operator variants, the expected-side text replaced by the NaN-reflexive mutant,
the unwind of 2 and the cadical solver are all in the generator. The scope ruling (comparisons only,
arithmetic refused by name) is stated in FR-053 Status and agrees with quire-exact TC-953. The
FR-049 and TC-063 edits drop the closed AC-8 narrowing and add the float family to AC-9 correctly.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | medium | FR-053 contradicts itself on who refuses table-row combinations. The P1 paragraph says a combination whose quire-exact record is a table row "is refused by IR-1526's tractability seam (FR-056 ...), not generated ... FR-053 builds no refusal of its own for it". The new Status, the code (`float.<op>.not_kani_tractable` at generation) and the measurement doc ("the decider's width match does not select these rows; the refusal ... is made at generation by the float row") all say FR-053 refuses and the table rows are only a record. Amend P1 to the planner ruling: the float row refuses arithmetic with a named capability, and the five FR-056 table rows are the measured record, not the mechanism. | spec/kani/functional/FR-053-float-scalar-harnesses.md:69-74 |
| FND-002 | medium | The three status records disagree, and two of them overstate delivery. The matrix row narrows only AC-3, AC-9, AC-10, AC-13 and AC-14. The FR-053 Status also marks AC-2 (host enumeration not run), AC-4 (comparison charges and budgets not run) and AC-8 (four of thirteen mutants run) as partial. TC-067 Status says steps 4 to 8 and 12 "run", but step 4's budgeted charge comparison, step 6's carrier-removal, `OperationNotDerivable` and composite-leaf controls, step 7's forced failure controls and step 12's evidence inspection (gap-analysis FND-001) have no test. TC-067 also says "step 10's `requires_bound` half depends on IR-69", but `requires_bound` is step 3 and step 10 is the charge mutants. Make the matrix row, FR-053 Status and TC-067 Status list the same partial criteria and steps, worded the same way. | spec/kani/matrix/tests.md:17, spec/kani/matrix/TC-067-float-scalar-harnesses.md:144-150 |
| FND-003 | low | The planner ruling says the Kani f32 division finding (0x800000A9 / 0x9C873333, correct 0x1BA00000) is recorded in IR-260 and IR-1526 evidence. It is in the repo (measurement doc, table doc, `div` refusal detail), but no comment on the IR-260 or IR-1526 Linear ticket records it, although the measurement doc says "This is recorded against IR-260 and IR-1526". Post a one-paragraph evidence comment on both tickets, or reword the doc to name the repo files only. | docs/qualification/ir260-float-scalar-kani-measurements.md:69-75 |

## Verdict

Not mergeable until the three status records agree and stop overstating delivery (FND-002), and the P1 paragraph matches the ruling (FND-001). These are text fixes in the same PR. FND-003 is a ticket comment, no code change. The AC-12 amendment (no tool versions) is correct.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-004 | low | FR-056 now makes the Kani, CBMC and solver versions optional in a row, but FR-056-AC-4 still says the refusal names "the Kani, CBMC, solver, generator and kernel versions", and its mutant list fails a refusal "omitting ... versions". A row recorded without versions, as every float row now is, cannot satisfy AC-4 as written. Reword AC-4 to "the tool versions the row records, if any". | spec/kani/functional/FR-056-kani-tractability-refusal.md:104, spec/kani/functional/FR-056-kani-tractability-refusal.md:129 |

## Dispositions

Round 1, checked at f11eef79.

| FND | outcome | reason |
|---|---|---|
| FND-001 | fixed | P1 now says the float row refuses at generation with `float.<op>.not_kani_tractable`, and that the five FR-056 rows are the measured record, not the mechanism, which the decider does not select. |
| FND-002 | fixed | The tests.md row, FR-053 Status and TC-067 Status now list the same delivered and dispositioned items. TC-067 runs steps 1 to 5, 7, 8 and 11 to 14, and step 6 for refusals; the forced runner classifications and the unrun composite-leaf control are named. The step-10 mislabel is gone. AC-1, AC-3 and AC-6 were rewritten to the planner's scope ruling: the arithmetic result-bound refusals, P1-unavailable settlement and sqrt/fma `OperationNotDerivable` were dropped, because arithmetic is refused first. The lead should confirm the ruling covers these AC rewrites. |
| FND-003 | fixed | The finding is posted on IR-260 (comment 3824e56f) and IR-1526 (comment c04c79fb), both verified, and the measurement doc now names only the repo files. |

Round 2, checked at fe8ad242.

| FND | outcome | reason |
|---|---|---|
| FND-004 | fixed | FR-056-AC-4 now reads "the tool versions the row records, if any" and its mutant reads "a recorded version". The new test `tc_070_a_version_free_row_refuses_and_the_refusal_names_the_row_as_recorded` removes all three version keys, gets `V2ClauseNotTractable` and asserts the carried row equals the version-free row. |
