---
id: "SR-1450"
title: "CG PR 265 spec review (criterion strength): changed ACs of FR-029 and FR-030"
type: SpecReview
analysis: criterion-strength
review_set: subset
date: "2026-10-04"
scope: "agent-ix/quire-contract-codegen@fd54a90fd3066845aea60064cbe96d433097222d; spec/kani/functional/FR-029-run-outcome-terminal-record.md (AC-11, AC-13, AC-14), spec/kani/functional/FR-030-ir-outcome-terminal-map.md (AC-1, AC-12, AC-13), spec/kani/matrix/TC-040-run-outcome-terminal-record.md, spec/kani/matrix/TC-041-ir-outcome-terminal-map.md (diff origin/main...HEAD)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-029
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-030
    type: references
---

# SR-1450: CG PR 265 spec review (criterion strength)

## Summary

Ticket: IR-465. Each changed AC was judged on whether a wrong implementation could fail it.
Spec-text judgment only.

- FR-029-AC-11 can fail: it gives a closed list, each item must map to `Failed`, plus a negative
  check on `ReplayRefused` codes.
- FR-029-AC-13 can fail: a named input set, a named value and code source, and a negative
  `Declined` check. TC-040 step 12 tests it bare and wrapped.
- FR-029-AC-14 and FR-030-AC-13 are held placeholders. They are honest as held rows, and they
  cannot fail until QSL rules.
- FR-030-AC-1 is strong: it states the exclusion explicitly.

## Verdict

The criteria are strong. There is one low finding: FR-030-AC-12 bundles two independent
outcomes.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-030-AC-12 is compound. It asserts two independent outcomes: the call-site and lock-input refusals map to `Inconclusive(ReplayRefused)`, never `Declined`, and `InvalidFunction` and `Name` map to `Failed`. The `Failed` half is already covered by FR-030-AC-10, which refers to "each CG-raised failure FR-029-AC-11 lists", and AC-11 now lists `InvalidFunction` and `Name`. The two halves also differ in buildability: the `Failed` half needs no pending QSL type, yet TC-041 Status says all of step 11 waits on `Inconclusive`. A test failing AC-12 does not say which half broke. Fix: drop the `Failed` half (AC-10 covers it), or split it into its own AC. | spec/kani/functional/FR-030-ir-outcome-terminal-map.md:125; spec/kani/matrix/TC-041-ir-outcome-terminal-map.md:33-35,60-62 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | low | The new TC-041 Status sentence misdescribes step 10. It says "steps 9 and 10, which include `InvalidFunction` and `Name` as `Failed`, do not [wait on the unmerged `Inconclusive`]". Step 9 does include them, through the FR-029-AC-11 list. Step 10 does not include them: it maps `Counterexample` under every replay settlement other than reproduced, and expects that "No value is `Refuted`" (FR-030-AC-11). That set includes the disagreement and refused settlements, whose values are `Inconclusive`, so step 10 cannot be fully built before those types merge. Wording only; not blocking. Fix: "step 9, which includes `InvalidFunction` and `Name` as `Failed`, does not". | spec/kani/matrix/TC-041-ir-outcome-terminal-map.md:57-60 |

## Dispositions

Round 1, reviewed at 808a3322ce2abf78ae32b00178a13cca51f59e29 (fix commit 808a332 on top of fd54a90).

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 808a3322ce2abf78ae32b00178a13cca51f59e29: FR-030-AC-12 is now the single `ReplayRefused`, never `Declined`, obligation. Its `Failed` half was dropped because FR-030-AC-10 covers it through the FR-029-AC-11 list, and no new id was minted. TC-041 step 11 and expected result 11 no longer include `InvalidFunction` or `Name`. Steps 1 to 12 and expected results 1 to 12 still align one to one. The new Status wording about step 10 is FND-002. |

Round 2, reviewed at 7e50ae15a1e2de6f9bef1b06e9e9303cb17291b9 (fix commit 7e50ae1 on top of 808a332; it touches only the TC-040 and TC-041 Status paragraphs).

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-002 | fixed | 7e50ae15a1e2de6f9bef1b06e9e9303cb17291b9: TC-041 Status now says that steps 8, 10 and 11 wait on the unmerged `Inconclusive`, "because they include settlements whose value is `Inconclusive`", and that step 9 (`InvalidFunction` and `Name` as `Failed`) does not. Each step was checked against what it asserts: step 8 is ParityReplay/ReplayRefused, step 10 is every non-reproduced settlement, step 11 is ReplayRefused, step 9 is `Failed` only, and step 12 is held. TC-040 Status was also checked: steps 7 and 8 (Inconclusive), 11 (every non-reproduced settlement) and 12 (ReplayRefused), plus step 10's ReplayRefused-code inspection, wait. Step 9 (faults to `Failed`) and the `Failed` half of step 10 do not. Step 13 is held. All claims are accurate. Not a finding: TC-041 step 1 and TC-040 step 6 (every outcome) also span `Inconclusive` pairs and are not listed as waiting. That omission predates this PR, and neither Status says those steps are buildable. |
