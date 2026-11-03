---
id: SR-1121019
title: "IR-1120 CG #643 gap-analysis review"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; PR #643 head 566a6629cb2e91e98a3a7cb785d32b51e8190cd5; gap-analysis over changed SMT and FR-045 files"
review_set: subset
---

## Summary

Independent diff-scoped review of CG #643 against FR-045 AC4–7 and TC-059, with AC2 status behavior checked for the changed shared parser.

## Verdict

FAIL — acceptance evidence remains incomplete.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The seven-row test constructs roles from 0..7 directly. No source refinement with rows written in another order is recompiled and mapped through FR-315, so canonical source-row permutation remains unverified. | src/routed/smt/refinement.rs:397-417 |
| FND-002 | high | The test decodes the handoff locally and constructs RefinementSmtCertificate itself. It neither submits captured cvc5 proof bytes to QSL FR-314’s reader nor proves that reader accepts the exact unwrapped top-level commands; the required consumer readback is still absent. | src/routed/smt/refinement.rs:433-481 |

## Coverage

Inspected the changed Rust, FR-045 status prose, matrix status, and Cargo.lock. The unchanged criteria retain their Test method and open live readback. No heavy build or full gate was run while the shared lock is held. `git diff --check` passed. Plan completion: not assessed.

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
| FND-001 | deferred: QSL facade | CG's only permitted QSL dependency is `qsl-replay`; it does not expose the checked refinement's canonical `step_sources` or an owned FR-315 refinement request from compiled input. The AC4 source-row permutation remains open for the QSL-1219/QSL-908 facade chain; #643 does not claim it. |
| FND-002 | deferred: QSL proof rule | A temporary QSL-main `bceb4b2e1` test sent all eight FR-315 scripts directly to cvc5 1.4.2 with `--dump-proofs --proof-format-mode=alethe` and passed each exact outer-list interior to `check_refinement_smt_proof`. QSL returned `ProofStepInvalid` at canonical Step row 1, index 43, cvc5 `t36 :rule cong` (outer stdout line 46); the captured output was parsed, not `Malformed`. QSL FR-314/QSL-908 owns the `Cong` check. AC5 and TC-059 step 6 remain open. |
