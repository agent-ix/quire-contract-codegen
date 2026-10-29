---
id: SR-1121012
title: IR-1120 PR 630 gap analysis
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; src/routed/smt/refinement.rs; FR-045-AC-4 through FR-045-AC-7, TC-059 steps 5 through 8"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-045", type: references }
---

# SR-1121012: IR-1120 gap analysis

## Summary

Ticket: IR-1120. PR: quire-contract-codegen#630 at 2e84d64c. Plan completion: not assessed. I compared the computed Test Matrix, the new tagged tests, changed source and TC-059 steps 5 through 8. QSL origin/main faf9bdcb has no refinement `Initial`/`Step` encoder or `RefinementSmtCertificate` adapter. That integration belongs to QSL-1219 and is not CG end-to-end evidence.

## Verdict

**FAIL** for the claimed FR-045-AC-5 positive handoff path. The two new tests provide trace tags for AC-4..7, but the QSL-authored canonical row, genuine emitted proof and adapter paths remain unevidenced. This artifact assesses only the changed refinement slice; it makes no repository-wide PASS claim.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The AC-5-tagged test serializes a proof that it first appended to the query sent to `/usr/bin/cat`; it never observes an independent solver-emitted Alethe proof or feeds the handoff to QSL's FR-314 adapter. The tagged test therefore cannot establish AC-5's exact emitted proof or adapter construction. QSL-1219 owns the absent adapter integration, which must remain an explicit residual. | src/routed/smt/refinement.rs:216-253; src/routed/smt/refinement.rs:297-343; spec/routed/matrix/TC-059-hyper-smt-induction-map.md:74 |
| FND-002 | medium | The AC-4-tagged tests manufacture `Base` and `Step { row }` from a local `0..7` loop; they do not compile a QSpec FR-375 refinement, permute source rows, or obtain FR-315 query bytes. Thus a producer that labels source positions instead of canonical `steps` ordinals would pass these tests. | src/routed/smt/refinement.rs:266-280; spec/routed/matrix/TC-059-hyper-smt-induction-map.md:60 |
| FND-003 | medium | The AC-7-tagged test exercises `Unknown` and missing fields, but does not exercise an FR-315 encoding refusal, `Sat`, absent or malformed solver response, failed process or reached limit through this refinement handoff. Its trace tag overstates TC-059 step 8 evidence. | src/routed/smt/refinement.rs:345-439; spec/routed/matrix/TC-059-hyper-smt-induction-map.md:84 |

## Coverage

The computed matrix on the PR head reports FR-045-AC-4 through AC-7 `tagged` by `src/routed/smt/refinement.rs` tests. AC-1 remains `untagged` and predates this diff. AC-2 and AC-3 are tagged by existing pair/process tests. The changed code has FR-045 ownership, no new source stub and no reverse code-to-spec gap. No plan was supplied. Plan completion: not assessed. Optional semantic review was not requested; the concrete trace inflation above follows directly from the test bodies and TC-059 procedure.

## Examined scope

```yaml
scope:
  - {id: FR-045-AC-4, path: "spec/routed/functional/FR-045-hyper-smt-induction-map.md", role: examined, excerpt: "For a refinement with seven step rows, CG retains one `Base` and exactly seven typed `Step { row }` records numbered 0 through 6 from the recompiled, canonically ordered `steps` array, even when records arrive out of order or the source rows were written in another order."}
  - {id: FR-045-AC-5, path: "spec/routed/functional/FR-045-hyper-smt-induction-map.md", role: examined, excerpt: "Given seven completed `Unsat` step records and one completed `Unsat` base record with present Alethe proofs, CG emits one deterministic handoff sequence in `Base`, `Step { row: 0 }` through `Step { row: 6 }` order for either input permutation. Every entry has separate length-delimited query and proof byte fields equal to the original FR-315 query and emitted Alethe proof, including a present empty proof."}
  - {id: FR-045-AC-6, path: "spec/routed/functional/FR-045-hyper-smt-induction-map.md", role: examined, excerpt: "Starting from AC-5's collection, removing `Base`, removing row 3, duplicating `Base` or row 3, or adding row 7 yields a typed shape refusal and no certificate handoff."}
  - {id: FR-045-AC-7, path: "spec/routed/functional/FR-045-hyper-smt-induction-map.md", role: examined, excerpt: "Starting from AC-5's collection, making row 3's query or proof field absent yields a typed missing-field refusal and no handoff; making the proof present but empty preserves the empty bytes in the handoff."}
  - {id: TC-059, path: "spec/routed/matrix/TC-059-hyper-smt-induction-map.md", role: examined, excerpt: "Verify CG's solver invocation, exact query association, bounded capture and typed non-success over QSL-owned canonical HP-2 queries."}
```

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | still-open | At 3141a622 the public handoff refuses all otherwise valid records with `ProofProvenanceUnavailable`. The serializer is `#[cfg(test)]` and receives test-supplied proof bytes. No production positive AC-5 path or QSL adapter readback exists. |
| FND-002 | still-open | The updated tests still create roles in a local `0..7` loop and do not exercise QSpec FR-375 canonical step ordering or source-row permutation. |
| FND-003 | still-open | The updated tests add a provenance refusal but still do not drive the named FR-315 refusal or solver nonproof branches through this refinement collection. |
