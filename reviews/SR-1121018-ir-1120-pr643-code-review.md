---
id: SR-1121018
title: "IR-1120 CG #643 code-review review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; PR #643 head 566a6629cb2e91e98a3a7cb785d32b51e8190cd5; code-review over changed SMT and FR-045 files"
review_set: subset
---

## Summary

Independent diff-scoped review of CG #643 against FR-045 AC4–7 and TC-059, with AC2 status behavior checked for the changed shared parser.

## Verdict

CONDITIONAL — parser regression requires a fix.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The generic solver status parser no longer accepts a standalone success notice after a status. Before this PR, unsat\nsuccess\n was Unsat; valid_solver_tail now returns false and run_solver_query reports MalformedResponse, changing ordinary HP-2 run evidence outside proof extraction. | src/routed/smt/mod.rs:314-338 |

## Coverage

Inspected the changed Rust, FR-045 status prose, matrix status, and Cargo.lock. The unchanged criteria retain their Test method and open live readback. No heavy build or full gate was run while the shared lock is held. `git diff --check` passed. Plan completion: not assessed.

## Examined scope and bindings

```yaml
scope:
- id: FR-045-AC-2
  path: spec/routed/functional/FR-045-hyper-smt-induction-map.md
  role: examined
  excerpt: A base `Sat` response remains only a candidate for QSL reconstruction and
    replay; base `Unsat` with step `Sat` is not induction; solver `Unknown`, QSL encoding
    refusal, resource stop, missing or malformed response, process failure and a response
    paired with another product, item, instance, depth, kind or query bytes remain
    distinct typed non-proof evidence. A resource stop is handed to FR-182 V-7 as
    incomplete. Every branch retains its available identities and depth, and no CG
    branch claims terminal refutation or certification.
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
| FND-001 | fixed | `7165831f` restores standalone `success` notices in the generic solver tail and asserts `parse_solver_status(b"unsat\nsuccess\n") == Unsat`; the stricter Alethe proof extractor still refuses that tail. `cargo fmt --check` and `git diff --check` pass. The focused build cannot reach CG until QSL adapts to IR #699/#700. |
