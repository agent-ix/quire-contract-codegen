---
id: SR-598004
title: "IR-348 PR #598 spec-review/ears-conformance (new hold requirement statements)"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-codegen; PR #598 head d9ce52c931dbcf0bad43a35d4ff0706705814842 against origin/main e45be9eb; spec/kani/functional/FR-041-claimed-module-proof-gate.md (new hold Behavior bullet), spec/kani/non-functional/NFR-006-real-kani-lane-gating.md (new hold bullet and AC-19 note); IR-348"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-041
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/NFR-006
    type: reviews
---

## Summary

Ticket: IR-348. The PR adds requirement statements in two places: the FR-041 hold Behavior bullet
and the NFR-006 hold bullet. This review checks both for EARS form, defined terms and testable
triggers. The Status sections and matrix prose are not requirement statements and are out of
scope here.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The FR-041 hold bullet uses a term nothing defines: "the declaration row and its merge-base identity remain visible". The text never says what a merge-base identity is, or where and how it "remains visible", so the clause cannot be tested. Bullet 1 already compares against "the declaration at the candidate's merge base", so the clause should say that comparison still includes the held row. | spec/kani/functional/FR-041-claimed-module-proof-gate.md:68 |
| FND-002 | low | The FR-041 hold bullet starts as a permission ("may be marked `held`") and mixes a dated event trigger with state conditions in one sentence. The NFR-006 bullet instead uses a state-driven form ("For the one ... held ... row, the ... lane shall ..."). Writing FR-041 the same way ("While the declaration marks the V1 row `held`, the gate SHALL ...") would make the trigger and its expiry testable. | spec/kani/functional/FR-041-claimed-module-proof-gate.md:61-72; spec/kani/non-functional/NFR-006-real-kani-lane-gating.md:44-51 |

## Verdict

Approve with low findings. The NFR-006 hold bullet is in conformant state-driven form. Its
invalid-hold cases are stated as fail conditions, and the reason and expiry strings are exact.

## Coverage

```yaml
scope:
- {id: FR-041:hold-bullet, path: spec/kani/functional/FR-041-claimed-module-proof-gate.md, role: examined, excerpt: "Under the 2026-10-27 IR-348 operator ruling, only the V1-sourced module claim row named by scripts/kani_claims.json may be marked held in that reviewed declaration. ... the declaration row and its merge-base identity remain visible."}
- {id: NFR-006:hold-bullet, path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md, role: examined, excerpt: "For the one IR-348 operator-held V1 module claim row in scripts/kani_claims.json, the one required lane shall print that row's identity, reason and expiry and count it as held separately from ran and expected."}
- {id: NFR-006:AC-19-note, path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md, role: examined, excerpt: "NFR-006-AC-19 remains Open during IR-348 V1 retirement. Only the V1-sourced claim row is HELD"}
```
