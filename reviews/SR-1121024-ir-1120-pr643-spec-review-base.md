---
id: SR-1121024
title: "IR-1120 CG #643 spec-review/base re-review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; PR #643 head 3f40b53ab057c52d72fdf49ca29faf2bcef89572; spec-review/base over changed SMT and FR-045 files"
review_set: subset
---

## Summary

Independent static re-review of the FR-045 Status text, the two matrix rows, TC-059 and the AC4–7 wording against the code at this head. AC5 is correctly held open, but the status wording claims more than the tests show.

## Verdict

CONDITIONAL — the matrix and Status wording must be narrowed before merge. The criteria themselves are unchanged and need no rewrite.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Prior placeholder: no substantive finding. | - |
| FND-002 | medium | The status text is overbroad. The matrix calls source permutation "covered" although row discrimination is unproven (SR-1121022 FND-001), and calls proof-frame extraction "covered" although TC-059 step 6 requires real cvc5 output and CG's tests use a shell stub. The Status narrows "steps 5-8" to step 6 alone although parts of steps 5 and 8 remain open (SR-1121022 FND-004). It also does not say that no production driver exists (SR-1121022 FND-003). | spec/routed/matrix/tests.md:34; spec/routed/matrix/tests.md:66; spec/routed/functional/FR-045-hyper-smt-induction-map.md:207-214 |
| FND-003 | low | FR-045 says the outer list's direct children are "complete SMT-LIB expressions", which admits bare atoms. The code and tests refuse them, including `unsat\n(step s0 (cl) :rule resolution)`. The behavior fails closed and is right, so the prose is what should change, to "parenthesized commands". | spec/routed/functional/FR-045-hyper-smt-induction-map.md:124-125; src/routed/smt/mod.rs:289-312 |
| FND-004 | low | Other spec text still says planned: TC-059's description and the `spec.md` registry row both disagree with the Partial matrix status. | spec/routed/matrix/TC-059-hyper-smt-induction-map.md:18-19; spec/spec.md:96 |
| FND-005 | low | The Status keeps AC5 open but names no owning ticket for the QSL readback blocker, so a partial merge leaves the remaining scope tracked only outside the spec. | spec/routed/functional/FR-045-hyper-smt-induction-map.md:211-214 |

## Coverage

AC4–7 and TC-059's procedure are unchanged in this PR and remain testable. The Status correctly keeps the AC5 and step 6 bar: CG must pass the exact interior bytes through the QSL reader before claiming either. The matrix rows remain Partial, and nothing credits the live reader. The `success` change matches FR-045:120-122 and the trailing-token rule at FR-045:134-136.

This reviewer ran no `make spec` or Quire validation; the lead will run that on this artifact. The `/spec-review` skill was not loadable here, so the base method was applied from the brief. Only the base analysis was performed. Plan completion: not assessed.

## Examined scope and bindings

```yaml
scope:
- id: FR-045-AC-4
  path: spec/routed/functional/FR-045-hyper-smt-induction-map.md
  role: examined
  excerpt: For a refinement with seven step rows, CG retains one `Base` and exactly
    seven typed `Step { row }` records numbered 0 through 6 from the recompiled, canonically
    ordered `steps` array, even when records arrive out of order or the source rows
    were written in another order. Each record binds the refinement item, subjects,
    distinct invocation attempt, role and row, and exact FR-315 `Initial` or `Step
    { row }` query bytes sent on stdin; changing a query byte or exchanging the base
    and a step role refuses the handoff. With zero step rows, the complete shape has
    only `Base`.
- id: FR-045-AC-5
  path: spec/routed/functional/FR-045-hyper-smt-induction-map.md
  role: examined
  excerpt: 'Given seven completed `Unsat` step records and one completed `Unsat` base
    record with present Alethe proofs, CG emits one deterministic handoff sequence
    in `Base`, `Step { row: 0 }` through `Step { row: 6 }` order for either input
    permutation. Every entry has separate length-delimited query and proof byte fields
    equal to the original FR-315 query and emitted Alethe proof, including a present
    empty proof. Repeating serialization of the same typed fields gives identical
    bytes; changing a proof byte changes only that entry''s proof field. The QSL adapter
    can construct `RefinementSmtCertificate` wi'
- id: FR-045-AC-7
  path: spec/routed/functional/FR-045-hyper-smt-induction-map.md
  role: examined
  excerpt: Starting from AC-5's collection, making row 3's query or proof field absent
    yields a typed missing-field refusal and no handoff; making the proof present
    but empty preserves the empty bytes in the handoff. An FR-315 refusal, a `Sat`
    or `Unknown` answer, a missing or malformed response, a failed solver invocation
    or a reached limit remains typed non-proof evidence and produces no proof-bearing
    handoff.
bindings: []
```

## Dispositions

| Finding | Outcome | Evidence |
| --- | --- | --- |
| FND-001 | accepted-no-change | Carried from SR-1121020; a placeholder with nothing to fix. |
| FND-002 | still-open: reword so only delivered controls are claimed and steps 5 and 8 residuals are listed | Claim source permutation only after the test fix; describe proof framing as synthetic until real cvc5 capture exists. |
| FND-003 | still-open: replace "complete SMT-LIB expressions" with parenthesized commands | Spec wording only; no code change. |
| FND-004 | still-open: update both lines to Partial | Spec wording only. |
| FND-005 | still-open: name the owning ticket in Status | The lead should confirm the ticket in Linear; this reviewer did not read it. |
