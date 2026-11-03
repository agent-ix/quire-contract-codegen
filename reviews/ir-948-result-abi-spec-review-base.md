---
id: SR-2026110283820
title: "IR-948 scalar result ABI specification review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen PR #720 at a99dfe9430eb3375c323af5cf975b9472d1b038b; FR-025, ADR-004, TC-036 and authored matrix index"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-025
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-036
    type: reviews
---

## Summary

Ticket: IR-948. Reviewed the frozen spec-only diff against QSpec FR-341/FR-342 and IR's admitted `CheckedPackageV2::model_operations` API. No applicable AssuranceProfile was found in the scoped specification. The new criteria are explicitly planned, the existing IR-1289 scalar refusal and supported collection result path are separated, and the computed Quire matrix reports FR-025-AC-11 to AC-13 as untagged. Quire validation passes. No Rust or Kani gate applies to this review.

## Verdict

CHANGES REQUESTED: one medium finding in the Boolean result verification path. The Integer result, fixed signature-read provision, refusal fields and planned-status controls are coherent with their stated authorities. The new criteria are syntactically valid and testable when the missing Boolean identity control is added.

## Examined

- FR-025-AC-11 (examined): fixed `model_operations(context, 65_536)` signature read, QSpec binder order, typed no-harness refusal and preservation of IR read error fields.
- FR-025-AC-12 (examined): Boolean/Integer/absent scalar result return mapping and exclusion of result from symbolic parameter draws and subject arguments.
- FR-025-AC-13 (examined): produced Integer domain assertion, lossless oracle argument, final cover, and the Boolean result clause.
- ADR-004 Q4 and Q5 (examined): planned result ABI and bounded admitted operation read.
- TC-036 steps and expected results 11 to 13 (examined): signature, return mapping and produced-result checks.
- `spec/kani/matrix/tests.md` FR-025-AC-11 to AC-13 row (examined): planned-until status with no claimed executable coverage.
- QSpec FR-341 and FR-342 (context_only): binder order and admitted operation anchor resolution.
- IR `model_operations` API (context_only): complete census, ordered parameters, tri-state result and `CheckedOperationReadError` fields.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-13 and TC-036 step 13 require Integer result-to-oracle identity but leave the Boolean route at return-type checking. A generator could pass a different Boolean to the oracle while meeting the authored test. Require that the oracle receives the exact subject-returned Boolean, with an opposite-value control. | spec/kani/functional/FR-025-generated-subject-abi.md:177; spec/kani/matrix/TC-036-generated-subject-abi.md:61-63,99-102 |

## Review Method

Base specification checklist plus diff-scoped integrity and EARS checks. New AC identifiers are unique and the TC verifies FR-025. No new hash, digest, copied artifact or tracking-record authority was introduced. The requirement body names CG, an observable refusal or generated value, and the ordering of call, domain assertion, clause assertion and cover. The computed matrix is untagged for the three planned criteria, matching the authored status. The sole finding is a missing falsifiable Boolean value-identity check in the Test criterion and procedure.

## Dispositions

Round 1 at `259364b8826f31717bfd6b573bde7483cd78a8cb`: PASS after FND-001 fix. FR-025-AC-13 and ADR-004 Q4 now require the exact subject-returned Boolean to reach the clause oracle. TC-036 step 13 and expected result 13 check both truth values and an opposite-value negative control. No further findings arose from the fix.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The Boolean value-identity claim and positive/negative test controls are explicit in FR-025-AC-13 and TC-036 step 13. |
