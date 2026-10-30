---
id: SR-1121026
title: "IR-1120 CG #643 gap-analysis disposition"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; PR #643 head f8b6b0573c71cb0476a67253c00c1cb3a34005e6; gap-analysis disposition of SR-1121022 and the fixes since 3f40b53ab057c52d72fdf49ca29faf2bcef89572"
review_set: subset
---

## Summary

Disposition of SR-1121022 at head f8b6b0573c71cb0476a67253c00c1cb3a34005e6. Row discrimination, the gap that blocked crediting source permutation, is closed. Three gaps remain, and each is listed honestly in FR-045 Status and the matrix. AC5 is PARTIAL, not Verified.

## Verdict

CONDITIONAL — acceptable as a partial merge with the residuals below deferred. AC4's seven-row source permutation is now backed by a discriminating test. AC4 as a whole stays open on its zero-step clause, and AC5 and AC7 also stay open.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Carried from SR-1121022 FND-002: no live cvc5 proof has been accepted by QSL's reader. The lead reports live proofs come back `Unverifiable` at unsupported `rare_rewrite`. The scratch logs show `Unverifiable { part: Base, step: 7 }` and `Unverifiable { part: Step { row: Some(1) }, step: 8 }`. AC5 and TC-059 step 6 stay open. | spec/routed/functional/FR-045-hyper-smt-induction-map.md:214-217; spec/routed/matrix/TC-059-hyper-smt-induction-map.md:71-88 |
| FND-002 | medium | Carried from SR-1121022 FND-003: no production code calls `encode_refinement_query` or `refinement_step_sources`, so the Base/Step-to-FR-315 mapping that FR-045 assigns to CG exists only in a test helper. | src/routed/smt/refinement_source_tests.rs:237; src/routed/smt/refinement_source_tests.rs:247-253; spec/routed/functional/FR-045-hyper-smt-induction-map.md:101-104 |
| FND-003 | medium | Carried from SR-1121022 FND-004: the zero-step case is still not a recompiled refinement, and step 8's FR-315 refusal, `Sat`, failed-process and reached-limit no-handoff controls are absent. `EncodingRefused` still does not appear in `src/`. | spec/routed/matrix/TC-059-hyper-smt-induction-map.md:68-69; spec/routed/matrix/TC-059-hyper-smt-induction-map.md:97-104 |

## Coverage

Original finding outcomes at this head:

| Original | Outcome | Evidence |
| --- | --- | --- |
| SR-1121022 FND-001 | fixed | Step queries are asserted pairwise distinct and all seven reversed source rows are asserted (src/routed/smt/refinement_source_tests.rs:304-318). The test passed in `cg643-ac4-review-fix.log`. This closes the row-discrimination gap only; see FND-003 for AC4's zero-step clause. |
| SR-1121022 FND-002 | deferred: IR-1139 | Local FND-001. Named in FR-045 Status and both matrix rows. |
| SR-1121022 FND-003 | deferred: stated in FR-045 Status | Local FND-002. Status line 212 says the production driver is absent. |
| SR-1121022 FND-004 | deferred: stated in FR-045 Status | Local FND-003. Status lines 212-214 name each remaining control. |
| SR-1121022 FND-005 | fixed | SR-1121022 is in the PR's `reviews/` and supersedes the stale SR-1121019 dispositions, whose text is kept as history. Note that SR-1121019 still reads `ProofStepInvalid` and "#643 does not claim it" (reviews/SR-1121019-ir-1120-pr643-gap-analysis.md:79-80), so a reader needs the later artifact. |

TC-059 steps 5-8 were compared with the local main checkout, which has no IR-1120 PR content. The head adds requirements to steps 6 and 8 and changes only the Description's status sentence. No step was weakened or removed. The AC4 and AC5 text matched by pattern and AC6-7 are identical, so FR-045's criteria are not narrowed.

Tracking gap: the Status names IR-1139 for AC5 but no ticket for FND-002 or FND-003 other than IR-1120 itself, so IR-1120 has to stay open after a partial merge unless those move to a new ticket. Linear is not authorized in this session, so IR-1120 and IR-1139 state was not read. The `rare_rewrite` steps were confirmed in local cvc5 output under `ir1139-cvc5-options*/`; the mapping from `rare_rewrite` to the reported `Unverifiable` step index was not traced, and this review relies on the lead's statement for it.

This reviewer ran no build, test, solver, spec validation or `git` command. The `/gap-analysis` skill was not loadable here, so its method was applied from the brief. Plan completion: not assessed.

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
| FND-001 | deferred: IR-1139 blocks AC5 and TC-059 step 6, which stay PARTIAL | FR-045 Status lines 214-217 keep the bar: CG must pass the exact interior bytes through the reader before claiming either. |
| FND-002 | deferred: production FR-315 driver, stated in Status, owner ticket to be confirmed by the lead | FR-045 Status line 212. |
| FND-003 | deferred: zero-step and step 8 controls, stated in Status, owner ticket to be confirmed by the lead | FR-045 Status lines 212-214; tests.md:34; tests.md:66. |
