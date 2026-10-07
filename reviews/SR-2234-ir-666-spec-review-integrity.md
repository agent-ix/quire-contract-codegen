---
id: "SR-2234"
title: "CG IR-666 spec-integrity-analysis: composite converter delivery and status amendments"
type: SpecReview
analysis: integrity
review_set: subset
scope: "agent-ix/quire-contract-codegen@code/ir-666-composite-converter (second frozen head, fix commit 'Bind composite reports and cover public parity rows'; reviewed revision recorded in the IR-666 Linear marker only, per this repository's no-SHA rule); spec/core/functional/interface-001-codegen-api.md (composite_parity_terminal_value, verified_shadow_terminal_value, Features), spec/kani/functional/FR-029-run-outcome-terminal-record.md (Description, Implementation Gate, AC-20, AC-21, AC-28), spec/kani/functional/FR-030-ir-outcome-terminal-map.md (Inputs, Outputs, Behavior, AC-15 to AC-17), spec/replay/functional/FR-033-composite-parity-replay-binding.md (Description, Outputs, Behavior binding-operations bullet, Falsified Settlement, Setup Refusal step 3, AC-7, AC-9, AC-12, AC-13, Dependencies), spec/kani/matrix/TC-041-ir-outcome-terminal-map.md (Status), spec/replay/matrix/TC-048-composite-parity-replay-binding.md (Description, step 8, step 8a, closing text), spec/kani/matrix/tests.md, spec/replay/matrix/tests.md, spec/tests.md"
---

# SR-2234: CG IR-666 spec-integrity analysis

## Summary

Ticket: IR-666. I checked the amendments for consistency within each document and across
FR-029, FR-030, FR-033, interface-001, TC-041, TC-048 and the matrix indexes. Each document now
states the same split, with IR-666 owning the public converter and IR-635 owning the production
original-artifact route. interface-001's operations list and Features table agree. The
merge-tree check with current `main` is clean: IR-682 touched only FR-034/TC-049.
Status-to-matrix contradictions are recorded in SR-2231 FND-005 and FND-006. The lost
`prepare`-with-Disagreed control behind FR-029-AC-28's "Covered" is in SR-2230 FND-004.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-029-AC-20 still reads "PLANNED CG CONSUMER (IR-666; QSL FR-358 delivered)" and is untagged. Its whole content is the identity-valid F-1 to F-7 consumption that this PR's tc_041 tests exercise through the converter: Disagreed, GeneratedFault with NativeCause, RefusedInput with code and index, the Admission, ExactEvaluation and RefinementCeiling stages, and Diverged/ScalarAgrees. The parallel FR-030-AC-15 to AC-17 and FR-033-AC-13 were amended to PARTIAL and tagged. AC-20 is the one IR-666 consumer criterion left PLANNED, and FR-029's Description says AC-28 alone is built. Failure scenario: IR-666 closes with an IR-666-labelled criterion still PLANNED and unowned. Fix: trace the tc_041 F-row tests to FR-029-AC-20 and amend its prefix like the others, or reassign it explicitly | spec/kani/functional/FR-029-run-outcome-terminal-record.md:328 |
| FND-002 | low | FR-033's Outputs still say "... missing report, wrong-claim report or unavailable CG consumer returns a typed refusal without a terminal value". Setup Refusal step 3 dropped the unavailable-capability refusal, FR-033-AC-9 dropped "unavailable CG consumer", and the delivered `CompositeReportError` has only MissingReport and ClaimMismatch. No type carries the remaining Outputs clause, and nothing now says when it would fire | spec/replay/functional/FR-033-composite-parity-replay-binding.md:111-114 |

## Verdict

The amendments are consistent across documents, with two gaps. The IR-666-labelled FR-029-AC-20
was left PLANNED and untraced (FND-001). A stale unavailable-consumer clause remains in FR-033
Outputs (FND-002).
