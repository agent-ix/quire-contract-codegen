---
id: SR-585005
title: 'spec-review/failure-domain review of IR-917 CG PR #585'
type: SpecReview
analysis: failure-domain
scope: agent-ix/quire-contract-codegen; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md
review_set: subset
---

## Summary

Reviewed CG PR #585 exact head 52a4dfde26116ea81cd2c313f6c8df14c75f057f against base 1175b995bc4314798cafe839a85a1ca2a745408c, limited to the FR-015-AC-41 row and TC-025 step 11 changes. FR-015-AC-7, FR-015-AC-37, FR-015-AC-119 and SR-583002 FND-007 were read as context. Ticket: IR-917. Reviewer model: Claude Opus 5.5 (claude-opus-5-5). Criterion-strength was skipped because no Jev client is available. Spec-only: there was no Cargo, Kani or full-gate run.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-015-AC-41 and TC-025 step 11 require the deterministic `amount_current = 999` playback to fail at the generated harness's independent arithmetic assertion. But 999 is the one admitted value at which the plus-one mutant also makes the clause itself false (`1001 <= 1000`). A natively executed playback stops at the first generated check that panics, for example the postcondition check inside the contract call (FR-015-AC-7). Neither text requires the arithmetic assertion to be reached before the clause check, nor says how a playback failure is attributed to that assertion. A test that accepts any playback panic would therefore pass on the wrong check, and an honest test may find the arithmetic assertion unreachable at 999. Fix: in FR-015-AC-41 and TC-025 step 11, require the playback to identify the failing check as the generated independent arithmetic assertion (by its generated check message or source location). State that a panic at an assumption, the clause/postcondition check or the cover does not satisfy the control. Then either require the generated harness to evaluate each arithmetic-subterm assertion before the clause verdict is checked, or allow the playback to record every failed check. Neither change narrows the AC. | spec/kani/functional/FR-015-bounded-kani-obligations.md:789, spec/kani/matrix/TC-025-bounded-kani-obligations.md:305-321 |

## Verdict

CHANGES REQUESTED (medium). At 999 the clause check and the arithmetic assertion both fail, and the spec does not say how the playback's failure is attributed to the arithmetic assertion.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1 at e6902279401586508a42d0194ce8352cd2a3ad5f. Fix commit e6902279 adds the same rule to FR-015-AC-41 and TC-025 step 11. At 999 the selected subject returns true normally, and the generated harness runs the arithmetic assertion before it checks the clause verdict. The playback failure must name the generated arithmetic assertion by its check message or source location. A failure at an assumption, the clause/postcondition check or the cover gets no credit. Both mutants and the unconstrained runs are still required, so nothing is narrowed and no regression was found. |
