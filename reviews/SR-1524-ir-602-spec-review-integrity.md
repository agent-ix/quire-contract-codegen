---
id: "SR-1524"
title: "CG PR 281 integrity analysis: FR-031 statuses, AC cross-consistency, matrix"
type: SpecReview
analysis: integrity
review_set: subset
scope: "agent-ix/quire-contract-codegen@607a975f0ab00aab0730539b75ac506a1380dbaa; spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md (AC-1 to AC-25), spec/oracle/matrix/TC-044-boolean-oracle-integer-arithmetic.md, spec/oracle/matrix/tests.md, spec/tests.md, interface-001, FR-008-AC-3, TC-003, NFR-006"
---

# SR-1524: CG PR 281 integrity analysis

## Summary

Ticket: IR-602. I checked consistency across FR-031's criteria, TC-044's steps and the matrix
rows.

Consistent:

- The matrix rows list AC-1 to AC-25 once each. The AC-18 row moved from Covered to Planned.
- TC-044's steps 14 to 17 map onto AC-22 to AC-25.
- The step 11 and tc_023/TC-017 rewrite agrees with FR-008-AC-3 and the Consumers section.
- interface-001's oracle_slice agrees with FR-031.
- Strict coverage is unchanged at 44, and FR-031 goes from 17/21 to 17/25.

Inconsistent: three implemented criteria now describe a state that is only planned (FND-001),
or still name the earlier runtime API (FND-002, FND-003).

## Verdict

Changes requested on FND-001. FND-002 and FND-003 are wording fixes.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-21 keeps IMPLEMENTED and sits in the "✅ Covered" matrix row, but its text now requires the differential to run "against `quire-exact`". That is planned IR-602 work, and today's test runs against Contract Runtime, so the status contradicts the text. AC-8 has the same defect in softer words ("the kernel the oracle calls", with a note saying from IR-602). AC-18 likewise stays counted as backed (17/25) by tests that assert the code the criterion now deletes. Keep the IMPLEMENTED texts as they were and carry the quire-exact retarget only in AC-22, or mark the changed criteria Planned. | FR-031:463,476,473; spec/oracle/matrix/tests.md:45 |
| FND-002 | low | AC-19 (IMPLEMENTED) says the bundle oracle "contains no ... `wrapping_` method". A bundle oracle that holds an add and a remainder now contains `wrapping_rem` (AC-23), so read literally the two criteria contradict. Scope AC-19 to add, subtract and multiply nodes, or add the AC-23 exception. | FR-031:474,478 |
| FND-003 | low | Stale runtime wording. AC-4 still names "`check_equality` then `CheckedEquality::evaluate`", which `quire_exact` does not export, while the prose says the comparisons are held to the kernel. AC-15 says "the runtime call the table names", and the table now names `planned_equality`. TC-044's title in tests.md still says "take the runtime's meaning". | FR-031:459,470; spec/oracle/matrix/tests.md:91 |

## Dispositions

Round 1, reviewed at f27977083e292d1334aa7a92c63f570af305b532 (fix commit f279770 plus a merge of main 735e704).

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | f279770: AC-8 and AC-21 are back to their IMPLEMENTED text against `quire-contract-runtime`, and TC-044 step 13 is reverted (the quire-exact case is a later note). The quire-exact retarget is carried only in AC-22. The AC-18 matrix cell says Planned and states that the tests asserting the IR-596 text change with the code; the tool still counts AC-18 as backed, and the cell says why. |
| FND-002 | fixed | f279770: AC-19 is scoped to an expression whose arithmetic nodes are add, subtract and multiply, and it carves out AC-23's `wrapping_rem`. |
| FND-003 | fixed | f279770: AC-4 names the kernel's evaluation as the test takes it from RT today, and planned_equality after AC-22. AC-15 names the exact-kernel call with the RT checked equality until the migration. The FR-031, TC-044 and tests.md titles now say "the exact kernel's meaning". |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | medium | AC-18 is not gated, although its deletion half depends on the gate. The new Prerequisite paragraph says removing `UnsupportedIntegerDivision` before the native division exists "would leave the native oracle with no divide outcome at all", so the code stays until AC-2 lands. But AC-18 ("The code `UnsupportedIntegerDivision` does not exist ...") is marked only PLANNED. The prerequisite's not-buildable list, the AC-18 matrix row, the TC-044 row's gated list (AC-2, 6, 16, 17, 22, 25, 26), TC-044's gated-step list (which leaves out step 11's "the interim code is gone" assertion) and spec/tests.md all omit it. Only AC-18's saturate half (S-1 to S-4 refused with `UnsupportedSaturatingArithmetic`) can land early, and the "could land first" list does not name it either. Split AC-18, or mark its deletion half GATED in every one of those places. | FR-031:76-83 (Prerequisite), FR-031 AC-18; spec/oracle/matrix/tests.md AC-18 row and TC-044 row; TC-044 Description and step 11; spec/tests.md Oracle row |

Round 2, reviewed at 41526f7be8c13e6b835dbef253b508297de6f6a5 (fix commit 41526f7).

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-004 | fixed | 41526f7: AC-18 now holds only the `saturate` rows S-1 to S-4, refused with `UnsupportedSaturatingArithmetic`, buildable before the prerequisite. The new AC-27 removes `UnsupportedIntegerDivision` (the variant, nothing under src/tests/schemas spelling it, the code string, IR-601 messages) and is GATED. The gate is stated consistently in the Prerequisite's not-buildable list (AC-27) and could-land-first list (AC-18, AC-23, AC-24, with the native oracle refusing a `reject` divide or remainder under the interim code until then), the AC-18 and AC-27 matrix rows, the TC-044 trace row (AC-27 added and in the gated list), the TC-044 Description (step 11 split between its gated and buildable halves), step 11 itself and spec/tests.md (all but AC-18, AC-23 and AC-24 gated). No remaining text lists the deletion under AC-18. The unmarked end-state sentences ("Integer divide and remainder" removal paragraph, the Refusals bullet, TC-044 Description and Expected Results) describe the completed IR-602 change, in the same way as the unmarked divide Behavior bullets, and the Prerequisite governs them all. No regression. |
