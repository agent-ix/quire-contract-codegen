---
id: "SR-3163"
title: "IR-635 PR 326 spec integrity: Eq constructor status consistency"
type: SpecReview
analysis: integrity
review_set: subset
scope: "agent-ix/quire-contract-codegen PR 326, branch code/ir-635-original-eq-builder (frozen head recorded in the IR-635 Linear review marker, not here); spec/core/functional/interface-001-codegen-api.md, spec/kani/functional/FR-029-run-outcome-terminal-record.md, spec/kani/matrix/tests.md, spec/replay/functional/FR-033-composite-parity-replay-binding.md, spec/replay/matrix/TC-048-composite-parity-replay-binding.md, spec/replay/matrix/tests.md"
---

# SR-3163: IR-635 PR 326 spec integrity

## Summary

Ticket: IR-635. Integrity lens (completeness, consistency, atomicity) over the changed spec text.

Completeness: every new interface-001 operation maps to FR-033 criteria, and every FR-033 row the
PR relabels has a matrix row carrying the same status. The replay matrix splits the old planned
row into an AC-1/AC-11 partial row and an AC-2 to AC-6, AC-8, AC-10 planned row with no criterion
dropped (checked against the computed matrix record set).

Consistency: interface-001 `request` semantics, the FR-033 description, TC-048 and the code agree
on Eq-only graph-child operands, Node-only bounds, empty literal Bounds, decoded stage limits,
exact recompiled-context equality and the pre-invocation imported refusal. The FR-029 paragraph
agrees with AC-17 and AC-19 to AC-27 remaining planned.

Hidden-assumption probe (FR depends on an unimplemented service): the Ne accessor (IR-690),
bounded-field literals (IR-691) and dependency-package retention are each named with their
interim behaviour (typed refusal), so no hidden fallback exists.

## Verdict

CONDITIONAL. One consistency gap: interface-001 and FR-033 describe membership as an original
function/node/occurrence requirement but do not say which component's definition of "belongs to
the function" is authoritative, which is how the CG-local heuristic in SR-3160 FND-001 arose.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Ambiguous ownership of the membership rule: interface-001 `request` says "require original function/node/occurrence membership" and FR-033-AC-1 says the node/occurrence "must belong to the actual retained proving context", without naming the authority. QSL's `locate` already defines it (checked-body membership plus the occurrence's `Body { function }` origin). Two implementers would read this differently (CG source-region containment versus QSL's origin), and the code took the CG-local reading. The spec should state that membership is QSL's (or a named IR accessor's) definition and that a CG precheck, if kept, must not refuse what that owner admits. | spec/core/functional/interface-001-codegen-api.md (OriginalCompositeEqContext::request); spec/replay/functional/FR-033-composite-parity-replay-binding.md (FR-033-AC-1 row) |
