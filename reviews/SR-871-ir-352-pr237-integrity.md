---
id: "SR-871"
title: "CG PR 237 spec review (integrity): FR-021, TC-031 and matrix consistency"
type: SpecReview
analysis: integrity
review_set: subset
scope: "agent-ix/quire-contract-codegen@6d0e02a4770c6b77bc4b08ea46a3978015c6c1bc; spec/oracle/functional/FR-021-function-application-oracles.md, spec/oracle/matrix/TC-031-function-application-oracles.md, spec/oracle/matrix/tests.md (diff origin/main...HEAD); tests/it/exact_scalar_generation.rs read as context"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/TC-031
    type: references
---

# SR-871: CG PR 237 spec review, integrity

## Summary

Ticket: IR-352. Checked: id continuity (AC-19 to AC-21 follow AC-18, no gaps or duplicates), one
mutation row per new AC, TC-031 step renumbering (no spec, test or source cites "TC-031 step 8" or
"step 9"; the only step cite in tests is "step 1(a)"), the matrix row's status (Planned, no backed
count changed) and the TC-031 relationships (`verifies` FR-021 already present).

## Verdict

Structurally sound. Three low consistency findings, none blocking on its own.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | TC-031's Description and Expected Results were not updated for the new step 8. Expected Results lists what a pass looks like for every other step but says nothing about zero panicking macros in emitted source, the `CheckedInvariant` catch-all, or the `Negate` refusal. Add one clause to each. | spec/oracle/matrix/TC-031-function-application-oracles.md:13-21, :97-108 |
| FND-002 | low | TC-031 step 8 says to "count the same four macros in `src/oracle/function/mod.rs` and assert zero", while FR-021-AC-21 says only that the file has no such arm "over the operator". The step is the stronger claim and is what the code PR should test (it also covers the two template arms). Align AC-21 to "contains no ... invocation" or the step to the AC. The existing `oracle_generators_have_no_unexcused_panicking_arms` already scans this file; in the code PR it should drop the three function/mod.rs excusals and be tagged FR-021-AC-21. | spec/oracle/matrix/TC-031-function-application-oracles.md:87-88; spec/oracle/functional/FR-021-function-application-oracles.md:239 |
| FND-003 | low | The new matrix row for AC-19 to AC-21 is inserted above the FR-021-AC-18 row, so FR-021's rows are no longer in id order. Move it below AC-18. | spec/oracle/matrix/tests.md:37-38 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | low | The new statement bullet bans only an `unreachable!`, `panic!`, `todo!` or `unimplemented!` "arm over an operator or over a runtime enum" in `src/oracle/function/mod.rs`, while FR-021-AC-21 and TC-031 step 8 require zero occurrences anywhere in that file. The criterion is stronger than its requirement, and a literal count would also hit a doc comment naming a macro (the existing scanner skips `//` lines). Make the bullet "contain no invocation of" the four macros, or scope the AC to arms. Not blocking. | spec/oracle/functional/FR-021-function-application-oracles.md:194-195, :247 |

## Dispositions

Round 1, reviewed at cac5002cc137ad6297def98b225b9394b0673fef.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | cac5002cc137ad6297def98b225b9394b0673fef (TC-031 Description and Expected Results now state the no-panic, CheckedInvariant and UnsupportedOperator checks as Planned) |
| FND-002 | fixed | cac5002cc137ad6297def98b225b9394b0673fef (step 8 and AC-21 both count the four macros in src/oracle/function/mod.rs and assert zero) |
| FND-003 | fixed | cac5002cc137ad6297def98b225b9394b0673fef (the AC-19 to AC-21 row now follows the AC-18 row) |

Round 2, reviewed at c7fbbc3d0e37f92768515611c8517257f1c42537 (one commit over cac5002cc137ad6297def98b225b9394b0673fef; merge base still origin/main 81a9c69fb1e55066204a2e8cbe94232169cacdcd). `make spec` exit 0 (8 loader diagnostics, unchanged); strict coverage 66 unbacked, 211 backed of 323, status lies 0 (unchanged).

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-004 | fixed | c7fbbc3d0e37f92768515611c8517257f1c42537 (the FR-021 bullet, FR-021-AC-21 and TC-031 step 8 all say zero invocations of the four macros anywhere in src/oracle/function/mod.rs, comment lines not counted) |
