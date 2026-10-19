---
id: SR-13033
title: "IR-878 CG PR 449 gap analysis: FR-042-AC-1 against CG oracle constructors"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen@f69874b0a8d63bf6f2384656961ee15c105de642 vs origin/main 4fb3402; FR-042-AC-1 vs src/oracle/boolean_v1.rs, src/oracle/function/mod.rs, src/oracle/equality/mod.rs, src/replay/"
review_set: subset
---

## Summary

Ticket: IR-878. Plan completion: not assessed. This analysis compared the new criterion FR-042-AC-1 with the CG source it names. Excluding the Kani lane (`src/kani`), the CG production sites that construct `rt::Refusal::CheckedInvariant` are:

- `src/oracle/boolean_v1.rs:895,904,921,929` (4 sites)
- `src/oracle/function/mod.rs:2384` (the pre-check placeholder), `2933,2941,2953,2957,2961,2965,2973` (8 sites in all)
- `src/oracle/equality/mod.rs:1731,1734,1739,1742,1745,1750,1759` (7 sites)

These 19 sites match one-to-one the 19 site citations in quire-exact FR-369's CG table. That table pins CG `1b225f4`, so its line numbers have drifted, but they are informational only. FR-042 cites no line numbers. All 19 sites are still the unit form, as expected while the criterion is PLANNED. No test is tagged FR-042-AC-1, and the matrix credits none. The diff touches no code, so the startup-signal test timeout the author reports under load is environmental.

## Examined criteria

- FR-042-AC-1 (examined): "CG's native Boolean, equality, function and pre-check placeholder constructors use the exact nine CG-owned causes in the table, retaining `IllTypedCause` from `check_type` and `check_equality`; downstream replay consumers preserve the typed payload or match it explicitly, retain charge/failure behavior, and accept neither the former unit constructor nor an invented default."
- FR-042 Status: "PLANNED: owner tests are pending." (examined)

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-042's Status reads only "PLANNED: owner tests are pending" and names no implementing ticket or gate. FR-369's Status assigns CG producers and consumers to IR-709, and the unit-variant removal to the IR-497 cutover. CG cannot test the "accept neither the former unit constructor" clause until a quire-exact/RT bump carries the typed payload. The Status should name IR-709 and that gate so the unbacked row has a visible owner | spec/oracle/functional/FR-042-generated-checked-invariant-causes.md:38-40 |

## Verdict

**PASS with a low finding**: CG owns all 19 producer sites the criterion names, and the PLANNED status with no credited test is honest.
