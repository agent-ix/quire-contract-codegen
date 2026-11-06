---
id: SR-2026110283887
title: "IR-1348 PR 753 scoped Rust code review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; IR-1348; PR 753; src/oracle/scalar/{derive,lower,mod}.rs; tests/exact_scalar_support; tests/it/exact_scalar_{agreement,generation}.rs; Cargo.lock"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: references
---

## Summary

Reviewed PR 753 at head 7107aa912872fc4dececb3be06158b27c43363b8 against FR-014-AC-48 and AC-58 through AC-61, the local Rust conventions, and QSpec FR-142's compound-result rule. No applicable AssuranceProfile was found under `spec/`.

## Verdict

**PASS for the production-code diff.** Multiplication, division and power require a compound result; addition and subtraction follow the first quantity operand's unit form; conversion checks its admitted target form. The caller still supplies the identified conversion target, and an unrelated Integer result reaches a typed refusal. The scalar corpus fixture migration keeps literal dependencies empty and places bound edges on value-shaped expression nodes. The lock refresh selects normal `main` branches. Test trace and status gaps are recorded in the separate gap-analysis artifact.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

- FR-014-AC-48 (context only): admitted exact conversion target and caller-supplied `IdentifiedUnit`.
- FR-014-AC-58 (examined): four U12 compound results and generated QSV agreement.
- FR-014-AC-59 (examined): compound result retained through `UnitScope::form` and reused.
- FR-014-AC-60 (examined): wrong Integer result refusal and bare quantity bound preservation.
- FR-014-AC-61 (examined): U25 first compound target, retained target type, two-step conversion, and defensive scalar-target refusal.
- Read the changed production and test files and checked their references into the unchanged shape renderer, IR lowering refusal mapping, and QSV's `result_unit` and power typing. No copied cross-repo source, compatibility path, new dependency source, unsafe block, or gate change was found.
- The coder's scoped fmt, Clippy and value-test results were inspected as evidence; the reviewer ran no additional Cargo gate under the session's one-gate cadence.
