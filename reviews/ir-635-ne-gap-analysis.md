---
id: "SR-11621"
title: "IR-635 Ne slice gap analysis: Ne constructor traceability and status text"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen branch code/ir-635-ne-builder (no PR yet; reviewed head recorded in the IR-635 Linear review marker, not here), diff against origin/main; spec/replay/functional/FR-033-composite-parity-replay-binding.md, spec/replay/matrix/TC-048-composite-parity-replay-binding.md, src/replay/composite_builder.rs, tests/it/composite_original_eq.rs; context spec/core/functional/interface-001-codegen-api.md"
---

# SR-11621: IR-635 Ne slice gap analysis

## Summary

Ticket: IR-635. Planless run over the criteria this diff touches. The computed matrix was read
with `quire matrix` on the reviewed head; `make spec` (quire validate over spec, plan and
reviews) exits 0. The `composite_original_eq` tests pass 16/16 on the reviewed head. Semantic
review (intent, test, code) was done by hand for the constructor criteria. The status-line edits
were reviewed here; no separate spec-review artifact was produced because the only spec changes
are PARTIAL status annotations and TC-048 scope prose.

Computed matrix for the touched criteria:

- FR-033-AC-1: tagged, now including the two Ne controls and the operator-mismatch control.
  Status stays PARTIAL; the falsified and generated-artifact routes stay planned, so no credit is
  taken for the falsified route.
- FR-033-AC-11: tagged, now including the two Ne controls. Status stays PARTIAL; inline operands
  and full proving-record/native integration stay planned.
- FR-033-AC-9 and AC-12 statuses still say Eq-only, which stays truthful: no Ne control reaches
  their prechecks or QSL refusal reports.

Test oracle strength: the Ne parameter control compares each O-09 argument with an independent
accessor projection and requires the two identities to differ; the Ne literal control fixes the
literal at position 1 with empty Bounds; both settle through QSL, so a wrong preimage or operator
fails the genuine identity tie. Every non-equivalent mutant tried is killed (see SR-11620).
QSL FR-358 and ADR-013 O-09 put no operator in the preimage, so leaving the operator out of O-09
is correct.

## Verdict

CONDITIONAL. The code and tests back the Ne claims, but the spec does not yet state the Ne
behaviour as a criterion (FND-001) and the public interface spec contradicts the new signature
(FND-002). Fix both, or disposition them, before merge. FND-003 and FND-004 are wording fixes.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-033-AC-1's criterion text never mentions the operator: no clause requires that a requested operator differing from the node's Eq/Ne operation refuses with typed `OperatorMismatch` before invocation, with no report or terminal value, nor that the claim's operator is the node's. Only the PARTIAL status annotation mentions it, and the FR body only says inequality preserves the claimed operation. `original_operator_mismatch_refuses_with_typed_operators` is tagged FR-033-AC-1, so AC-1 is credited with behaviour it does not require, and a mutant removing the check would break no stated criterion. Add the operator clause to AC-1 (or AC-11). | spec/replay/functional/FR-033-composite-parity-replay-binding.md:359; tests/it/composite_original_eq.rs:478-497 |
| FND-002 | medium | interface-001 still documents `OriginalCompositeEqContext::request` with inputs `[CheckedNodeId, CheckedOccurrence, harness ProofBound list, work ceiling]` and semantics "Eq-only graph-child route ... Ne and inline operands refuse". The code now takes an `EqualityOperator`, accepts Ne and refuses a mismatch with `OperatorMismatch`. The public API spec contradicts the source-breaking signature. The brief bars touching interface-001 in this slice, so the leader must route the fix (this PR or the interface-001 owner) before the stale text ships. | spec/core/functional/interface-001-codegen-api.md:174-177 |
| FND-003 | low | FR-033 body prose is stale against the new status lines and the locked IR: it still says IR-635's slice covers admitted graph-child `structural.eq`, that "The typed `structural.ne` accessor is still missing under IR-690", and that the converter binds reports from "the Eq-only verified constructor". | spec/replay/functional/FR-033-composite-parity-replay-binding.md:54-60, 346 |
| FND-004 | low | The AC-1 status claims "verified Eq/Ne graph-child constructor controls", which reads as Ne coverage of every shape AC-1 lists, but Ne has only parameter/parameter and parameter/literal controls (no literal/literal or self comparison). TC-048 states this precisely; the AC-1 status should too. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:359 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1 (commit 1ec4b90). FR-033-AC-1 now states that the verified request carries the node's operator (`structural.eq` is `Equal`, `structural.ne` is `NotEqual`), and that a different requested operator refuses with typed `OperatorMismatch` carrying both operators before any QSL invocation, with no report or terminal value. The FR-033-AC-1 tag on `original_operator_mismatch_refuses_with_typed_operators` is now truthful. Mutants M1, M2 and M7 are killed by that test. |
| FND-002 | fixed | Round 1 (commit 04b4e79). The interface-001 `request` inputs now list `EqualityOperator (Equal or NotEqual)` in code order (node, occurrence, operator, bounds, work ceiling), and the semantics now describe the Eq/Ne route and `OperatorMismatch`. The diff changes only lines 175 and 177. |
| FND-003 | fixed | Round 1 (commit 1ec4b90). The FR-033 prose now says the constructor accepts Equal for `structural.eq` and NotEqual for `structural.ne`, and that the locked IR revision admits `structural.ne`. The converter sentence now says "Eq/Ne verified constructor". |
| FND-004 | fixed | Round 1 (commit 1ec4b90). The AC-1 and AC-11 statuses now credit Eq controls plus only the Ne parameter/parameter and parameter/literal controls, and list Ne literal/literal and self-comparison as planned. |
