---
id: SR-1121022
title: "IR-1120 CG #643 gap-analysis re-review"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; PR #643 head 3f40b53ab057c52d72fdf49ca29faf2bcef89572; gap-analysis over changed SMT and FR-045 files"
review_set: subset
---

## Summary

Independent static re-review of CG #643 against FR-045 AC4–7 and TC-059 steps 5–8. The new test really does recompile two authored orders through QSL's FR-492 facade and gives a nontrivial source mapping. It does not yet show that each row keeps its own query. AC5 stays partial.

## Verdict

FAIL for crediting AC4 at this head. The gap is small and test-only. AC5 and TC-059 step 6 remain open and are not credited.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Prior finding, partly closed: source permutation evidence was absent. The source compile and mapping now exist, but all seven Step queries are the same bytes by static reading, so "each row's FR-315 query bytes remain equal" passes for any row assignment. Only op0's mapping is checked across the two compilations. | src/routed/smt/refinement_source_tests.rs:63-66; src/routed/smt/refinement_source_tests.rs:293; src/routed/smt/refinement_source_tests.rs:299-309 |
| FND-002 | high | Prior finding: no live cvc5 proof has passed through QSL's reader. | spec/routed/matrix/TC-059-hyper-smt-induction-map.md:69-86 |
| FND-003 | medium | No production code calls `encode_refinement_query` or `refinement_step_sources`. The role-to-FR-315 mapping exists only in the test helper, so the test checks bindings its own helper authored. FR-045 assigns that identification to CG. | src/routed/smt/refinement_source_tests.rs:228-237; spec/routed/functional/FR-045-hyper-smt-induction-map.md:101-104 |
| FND-004 | medium | TC-059 work remains outside step 6. Step 5's zero-step case is synthetic rather than a recompiled refinement. Step 8's FR-315 refusal has no CG type or test: `EncodingRefused` does not appear in `src/`. Handoff-level `Sat`, failed-process and reached-limit cases are not exercised; only `Unknown` is. | src/routed/smt/refinement.rs:500-509; src/routed/smt/refinement.rs:566-570; spec/routed/matrix/TC-059-hyper-smt-induction-map.md:66-68; spec/routed/matrix/TC-059-hyper-smt-induction-map.md:95-97 |
| FND-005 | low | The SR-1121019 dispositions are stale at this head: one says #643 does not claim source-row permutation, and the other records `ProofStepInvalid` at `t36`, while the latest scratch evidence is `Unverifiable`. | reviews/SR-1121019-ir-1120-pr643-gap-analysis.md:79-80 |

## Coverage

Verified from source:

- Canonical `steps` order sorts by `source_node` digest, independent of authored order (`qsl-semantics/src/check/refinement.rs:652-664`), so the sources differ between the two orders.
- `Step { row }` resolves through `step_sources` (`qsl-eval/src/smt/refinement.rs:157-166`).
- Stdin capture, attempt binding, out-of-order records, source-position relabel, query-byte mutation and base/step exchange are all present in the test.

The relabel control refuses through a duplicate row (`Shape`), which TC-059 permits as typed refusal. The base/step exchange refuses through `Association`. The test's solver is `/bin/sh`, which is acceptable for AC4 because it observes stdin; it is not cvc5 evidence.

The AC5 evidence is the lead's scratch log `qsl1873-isolated-row1-detail.log`, read but not reproduced: `Unverifiable { part: Base, step: 7 }` and `Unverifiable { part: Step { row: Some(1) }, step: 8 }`, with `hole` steps visible in the output.

This reviewer ran no build, test, solver, Quire validation or `git` command, and Linear was not consulted, so IR-1139's state is unverified. `pr-diff.patch` contains no `reviews/` paths, so whether SR-1121019 changes in this PR was not established. The `/gap-analysis` skill was not loadable here, so its method was applied from the brief. Plan completion: not assessed.

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
- id: FR-045-AC-6
  path: spec/routed/functional/FR-045-hyper-smt-induction-map.md
  role: examined
  excerpt: Starting from AC-5's collection, removing `Base`, removing row 3, duplicating
    `Base` or row 3, or adding row 7 yields a typed shape refusal and no certificate
    handoff. Replacing row 3 with a record from another item, subject or attempt,
    or swapping row 3's record with row 4's while retaining the old row labels, yields
    a typed association refusal and no handoff.
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
| FND-001 | still-open: source compile and nontrivial mapping are delivered; row discrimination is not | Use distinct per-row queries, assert them pairwise distinct, and check the mapping for all seven rows (SR-1121021 FND-002). |
| FND-002 | still-open: AC5 and TC-059 step 6 stay PARTIAL on the QSL readback blocker | The latest scratch run returned `Unverifiable`, so it gives AC5 no credit. |
| FND-003 | still-open: record the missing production driver in FR-045 Status or add it in a tracked follow-up | Lead decides whether it belongs in this slice. A partial merge is acceptable only if Status says so (SR-1121024 FND-002). |
| FND-004 | still-open: add the recompiled zero-step case and the FR-315 refusal path, or keep steps 5 and 8 listed as open | Cases listed in the finding. |
| FND-005 | still-open: supersede SR-1121019's dispositions with this artifact | No change to the PR branch was made by this review. |
