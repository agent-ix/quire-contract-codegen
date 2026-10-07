---
id: "SR-2232"
title: "CG IR-666 spec-review base: composite converter delivery and status amendments"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-codegen@code/ir-666-composite-converter (second frozen head, fix commit 'Bind composite reports and cover public parity rows'; reviewed revision recorded in the IR-666 Linear marker only, per this repository's no-SHA rule); spec/core/functional/interface-001-codegen-api.md (composite_parity_terminal_value, verified_shadow_terminal_value, Features), spec/kani/functional/FR-029-run-outcome-terminal-record.md (Description, Implementation Gate, AC-20, AC-21, AC-28), spec/kani/functional/FR-030-ir-outcome-terminal-map.md (Inputs, Outputs, Behavior, AC-15 to AC-17), spec/replay/functional/FR-033-composite-parity-replay-binding.md (Description, Outputs, Behavior binding-operations bullet, Falsified Settlement, Setup Refusal step 3, AC-7, AC-9, AC-12, AC-13, Dependencies), spec/kani/matrix/TC-041-ir-outcome-terminal-map.md (Status), spec/replay/matrix/TC-048-composite-parity-replay-binding.md (Description, step 8, step 8a, closing text), spec/kani/matrix/tests.md, spec/replay/matrix/tests.md, spec/tests.md"
---

# SR-2232: CG IR-666 spec-review base

## Summary

Ticket: IR-666. This reviews the spec half of the round-1 fix commit. That commit records IR-666's
delivered public report converters, declares their API, and moves the IR-666 criteria from
PLANNED to PARTIAL or Covered, with IR-635 named for what remains. It is read against the staging
ruling on the ticket (data, re-measured): IR-666 owns the converter, the full claim check,
FR-029-AC-28, FR-033-AC-9 and FR-030-AC-15/16/17. IR-635 owns the builder, invocation and
original-artifact binding.

Correct as written:

- interface-001's two new operations match the exported functions and the error type.
- The FR-033 Behavior bullet keeps IR-635's precheck refusals separate from the two binding
  errors.
- FR-029-AC-21 now gates its interim refusals on the production path rather than on "CG consumes
  the facade", which keeps them true after this PR.
- No AC was deleted or renumbered. The changes are status prefixes and status prose.

The status-to-matrix contradictions are recorded in SR-2231 (FND-005, FND-006). The
FR-029-AC-20 and FR-033 Outputs inconsistencies are in SR-2234.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The new "PARTIAL (IR-666 direct public report controls; IR-635 original-artifact path planned)" prefixes on FR-030-AC-15, FR-030-AC-16 and FR-033-AC-13 do not say which clause is left open. Each criterion's text is about the identity-valid report result and its terminal value, which the new tc_041 tests exercise in full. None names an original-artifact clause. Two readers would disagree on when the criterion becomes Covered, and nothing in the text would move the prefix. Fix: name the remaining clause (for example "IR-635: the same rows reached from a CG-built request over the original proving context"), or mark the criterion Covered for its stated scope | spec/kani/functional/FR-030-ir-outcome-terminal-map.md:177-178, spec/replay/functional/FR-033-composite-parity-replay-binding.md:356 |

## Verdict

The amendments are accurate about what IR-666 delivered and what IR-635 still owns, and they
keep the QSL-owned terminal mapping intact. One ambiguity: the PARTIAL prefixes have no stated
remaining clause.

## Dispositions

Round 1. Reviewed on the rebased branch `code/ir-666-composite-converter-r2` at its frozen head
(fix commit "Close composite converter review findings"). The revision is recorded in the IR-666
Linear marker only.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | round-2 fix commit "Close composite converter review findings": FR-030-AC-15, FR-030-AC-16, FR-033-AC-13 and FR-029-AC-20 now read "IR-635 must reach the same rows from a CG-built request over the original proving context", which names the remaining clause and when it closes |
