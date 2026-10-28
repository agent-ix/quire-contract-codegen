---
id: SR-616005
title: "IR-1092 PR #618 base review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; spec/kani/functional/FR-015-bounded-kani-obligations.md:851; spec/kani/matrix/TC-025-bounded-kani-obligations.md:66-90; spec/kani/matrix/tests.md:39,113"
review_set: subset
---

## Summary

Reviewed AC-101 ownership correction, TC-025 test procedure, and the Kani matrix index. QSL and IR now own the upstream malformed-input refusals; the CG fallback remains explicitly conditional.

## Verdict

**CONDITIONAL** — the edited spec and test procedure leave stale coverage claims and an unconstructible CG test step.

## Scope

Examined FR-015-AC-101, TC-025 selected-field procedure, and Kani matrix status rows. Read spec/core/functional/interface-001-codegen-api.md and spec/tests.md as context; neither contains an AC-101-specific status claim. No applicable AssuranceProfile was found under spec/.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Kani matrix rows still call AC-101’s old malformed-declaration clause unbacked, even though AC-101 now assigns missing fields to QSL, malformed projections to Contract IR, and only an unresolved admitted claim to CG. Update both status rows to describe the new owner-boundary evidence gaps. | spec/kani/matrix/tests.md:39,113 |
| FND-002 | medium | TC-025 requires checking CG FieldNotAdmitted for an admitted selected claim with an unresolved checked declaration or field, but strict admission rejects that defect before CG and the procedure gives no constructible admitted case. State how to test the defensive CG path without claiming an authentic public admitted projection, or make this a conditional inspection control. | spec/kani/matrix/TC-025-bounded-kani-obligations.md:75-79; spec/kani/functional/FR-015-bounded-kani-obligations.md:851 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Both matrix rows now name missing QSL fields, malformed IR projections, and CG’s helper-only defensive control as distinct, still partial evidence. |
| FND-002 | fixed | TC-025 now specifies CG helper testing and inspection, explicitly excludes a public admitted malformed-projection claim, and asks for a public control only if strict admission permits one. |

## Disposition Verdict

**PASS at fb2ec40ad3a5f1cfcebda14a7c4ba291295366ee** — both review findings are fixed. The FR-015-AC-101 normative row is byte-identical to the original reviewed head; no new defect was found in the fix diff.
