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

## New findings (disposition pass 1)

Re-checked at head ad57b91cb39b7ad610ad191af92c15537eecdf9a against the previous head d9ce52c9.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | The rewritten FR-041 hold bullet gives the wrong actor: "the gate SHALL mark only the `kani_fr_003_4_proof_boolean_transition_module` claim row in `scripts/kani_claims.json` as `held` in the reviewed declaration". The reviewed declaration is a checked-in input that the gate compares, not one it writes. As worded, it obliges the gate to edit a reviewed file. It should say the reviewed declaration SHALL mark only that row `held`, and that the gate SHALL accept a hold only on that row. | spec/kani/functional/FR-041-claimed-module-proof-gate.md:61-63 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1, head ad57b91c. The undefined "merge-base identity remain visible" clause is gone. The bullet now says "The gate SHALL retain that exact row in the reviewed declaration and compare it with the merge-base declaration, even after the V1 module is absent from the compiled build", and a removed claim relative to the merge-base declaration SHALL fail the gate. |
| FND-002 | fixed | Round 1, head ad57b91c. The bullet now opens in state-driven form ("While the 2026-10-27 IR-348 V1 retirement hold is active, the gate SHALL ...") with SHALL reason and expiry, and names its expiry under IR-1047. The "may be marked" permission wording is gone. |
| FND-003 | fixed | Round 2, head bc82713c. The actor is corrected. FR-041 now reads "the checked-in reviewed declaration SHALL mark only the `kani_fr_003_4_proof_boolean_transition_module` claim row in `scripts/kani_claims.json` as `held`. The gate SHALL accept a hold only on that row and validate it. While this hold is active, the reviewed declaration SHALL record reason ... and expiry ...". The declaration is the subject of the mark, reason and expiry obligations, and the gate's obligation is acceptance and validation. The surviving "The gate SHALL retain that exact row in the reviewed declaration and compare it with the merge-base declaration" is enforced by the next clause ("a removed claim relative to the merge-base declaration ... SHALL fail the gate"). It reads as a comparison obligation, not an edit, so it is not raised as a new finding. |
