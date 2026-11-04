---
id: SR-2026110283855
title: IR-1120 production SMT driver code-review
type: SpecReview
analysis: code-review
scope: agent-ix/quire-contract-codegen; IR-1120; draft PR 742 production retained-request
  refinement driver
review_set: subset
relationships:
- target: ix://agent-ix/quire-contract-codegen/FR-045
  type: references
---

## Summary

The production driver borrows the retained request, pre-encodes the complete selected query set, runs exact bytes, retains typed outcomes and delegates the existing handoff validator. No production correctness defect was established; the PR description is stale. No applicable AssuranceProfile is declared under spec/.

## Verdict

PASS for the reviewed CG-owned slice after disposition round 1. Original findings remain below; this is not native proof qualification or merge authorization.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The PR description still describes the superseded one-field control as current and reports all focused refinement controls passing. The frozen source now uses seven fields and pairwise-distinct actual queries/proofs, and its intact strict Verified assertion fails at the known native Unverifiable result. Update the description to these current measured facts, including the draft/publication/final-gate blockers, without claiming broader proof support. | PR 742 description:3 |

## Coverage

Plan completion: not assessed. Reviewed every changed file, existing solver/record boundaries and the retained-request adapter contract. The computed matrix marks AC-4 through AC-7 tagged; that does not establish a passing native reader or whole-ticket completion. The semantic binding review was explicitly requested.

No Cargo, solver or proof run was performed by this reviewer. Independently ran the computed matrix and scoped spec validation and read the existing focused/live receipts. The real distinct-query control reaches native Unverifiable at Step 0 because the QSL checked-rule table lacks the emitted symm rule. The strict Verified assertion remains intact and failing; no larger proof kernel, output rewrite, query edit or acceptance narrowing is part of this review. The draft is not merge-ready. QSL facade publication and the final unpatched consumer gate remain external prerequisites.

Existing synthetic source/provenance controls retain association/framing credit only. New error enums follow this module’s existing typed enum idiom. The prepared collection owns query bytes and only borrows the actual request; no parallel subject schema or internal QSL request appears. Fallible outer collection reservations occur before process launch. Cancellation is distinct from native encoding refusal. Existing process support remains the execution owner; no backend proof or CI workflow is changed.

### Examined units

- FR-045-AC-4 (examined), spec/routed/functional/FR-045-hyper-smt-induction-map.md: For a refinement with seven step rows, CG retains one `Base` and exactly seven typed `Step { row }` records numbered 0 through 6 from the recompiled `steps` array in authored declaration order (QSpec FR-375-AC-22), even when run records arrive out of order. Permuting the authored source rows correspondingly permutes the recompiled step rows: each named row's `Step { row }` label and exact FR-315 query association follow its new position, and a record bound to the wrong row refuses handoff. Each record binds the refinement item, subjects, distinct invocation attempt, role and row, and
- FR-045-AC-5 (examined), spec/routed/functional/FR-045-hyper-smt-induction-map.md: Given seven completed `Unsat` step records and one completed `Unsat` base record with present Alethe proofs, CG emits one deterministic handoff sequence in `Base`, `Step { row: 0 }` through `Step { row: 6 }` order for either input permutation. Every entry has separate length-delimited query and proof byte fields equal to the original FR-315 query and emitted Alethe proof, including a present empty proof. Repeating serialization of the same typed fields gives identical bytes; changing a proof byte changes only that entry's proof field. The QSL adapter can construct `RefinementSmtCerti
- FR-045-AC-6 (examined), spec/routed/functional/FR-045-hyper-smt-induction-map.md: Starting from AC-5's collection, removing `Base`, removing row 3, duplicating `Base` or row 3, or adding row 7 yields a typed shape refusal and no certificate handoff. Replacing row 3 with a record from another item, subject or attempt, or swapping row 3's record with row 4's while retaining the old row labels, yields a typed association refusal and no handoff.
- FR-045-AC-7 (examined), spec/routed/functional/FR-045-hyper-smt-induction-map.md: Starting from AC-5's collection, making row 3's query or proof field absent yields a typed missing-field refusal and no handoff; making the proof present but empty preserves the empty bytes in the handoff. An FR-315 refusal, a `Sat` or `Unknown` answer, a missing or malformed response, a failed solver invocation or a reached limit remains typed non-proof evidence and produces no proof-bearing handoff.
- FR-045 (examined), spec/routed/functional/FR-045-hyper-smt-induction-map.md: Completed records SHALL remain available even if a later process mechanism fails.
- TC-059 (examined), spec/routed/matrix/TC-059-hyper-smt-induction-map.md: Through the production driver, take the selected refinement key and row count from its replay-owned request. Pre-encode every query before any solver launch; preserve a real any-row encoding refusal and pre-encoding cancellation as distinct typed failures, with no executable collection.

### Files examined

- spec/routed/functional/FR-045-hyper-smt-induction-map.md
- spec/routed/matrix/TC-059-hyper-smt-induction-map.md
- src/routed/smt/mod.rs
- src/routed/smt/refinement_driver.rs
- src/routed/smt/refinement_source_tests.rs
- PR 742 description

### Method selection

Code review includes Rust. Gap analysis includes the requested semantic binding check. Base, EARS, integrity and scope-boundary cover the changed requirements and public interface. No new spec object or relationship edge requires object/dependency analysis. No second reviewer or heavy build was used.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The PR description now names the seven-field distinct live control, its unchanged failing strict Verified assertion, the native QSL unsupported-rule seam, and the pending public-facade/final consumer gates. |

### Disposition verification

Inspected the frozen fix diff and current PR description. No production driver, live strict Verified assertion, query/proof interpretation or acceptance boundary was weakened. The new subprocess controls use the actual production preparation/execution path; the second failure comes from the native selected-executable check, not an injected semantic result or public fault API. No new findings. The QSL checked-rule and public facade/final unpatched gate blockers remain; draft PR 742 is not merge-ready. Reviewer ran no Cargo.
