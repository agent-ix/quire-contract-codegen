---
id: SR-2026110283822
title: "IR-1295 PR 721 base specification review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; PR #721; spec/core/functional/FR-005-cli-conformance.md; spec/core/matrix/TC-007-cross-backend-parity.md; FR-005-AC-3/4; TC-007"
review_set: subset
---

## Summary

Ticket: IR-1295. I reviewed the frozen FR-005 and TC-007 edits against their acceptance criteria and current CG boundary. No applicable AssuranceProfile was found in the scoped specification. CG owns this internal pair; the independently computed bound-strategy census tag can be compared with the compiled executable bound oracle, while the runner is a consistency observation. Kani and coverage are not independently qualified by this slice. FR-005-AC-4 and the hand-maintained matrix remain open/planned. Quire validates both changed documents. The user-directed differential register is the required authority for this task.

## Verdict

Changes requested: TC-007 does not define whether an outside-domain census classification and an executable-oracle Boolean constitute a normalized discrepancy. That decision controls which register rows must exist.

## Examined

- FR-005-AC-3: exactly one disposition row for each differing shared-corpus case, with a real fixture or documented distinction.
- FR-005-AC-4: broad executable, proptest, Kani, and coverage agreement remains OPEN.
- TC-007 description and procedure steps 1 to 4: qualified pair, candidate census, runtime comparison, outside-domain handling, and adverse join controls.
- TC-007 Differential Disposition Register and Expected Results: exact key, disposition forms, and completion limits.
- spec/core/matrix/tests.md: AC-3, AC-4, and TC-007 remain Planned; no TC-007 trace-tagged test exists at this head.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-007 normalizes only admitted census tags and oracle Booleans, then asks whether an outside-domain classification and an oracle Boolean differ without defining their common normalization or saying they are always a profile distinction. One implementation could omit every outside-domain register row while another requires one per case. State the exact comparison and register rule for outside-domain observations. | spec/core/matrix/TC-007-cross-backend-parity.md:32-44 |
