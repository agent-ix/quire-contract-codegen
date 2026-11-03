---
id: SR-2026110283824
title: "IR-1295 PR 721 scope and boundary analysis"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/quire-contract-codegen; PR #721; spec/core/functional/FR-005-cli-conformance.md; spec/core/matrix/TC-007-cross-backend-parity.md; FR-005-AC-3/4; TC-007"
review_set: subset
---

## Summary

Ticket: IR-1295. I checked the owner allocation and independent-result boundary against current CG source. CG owns the generated bound oracle, bound-strategy census and TC-007 register. The strategy relation computes tags independently; its runner calls the oracle. Current Kani corpus is graph-only and uses the same oracle, while coverage observes probes.

## Verdict

No scope-boundary findings. The executable bound oracle versus independent census tag is the qualified pair. The runner, Kani, coverage, and retired V1 bundle path receive no unsupported semantic-parity credit. Broad FR-005-AC-4 remains open.

## Examined

- FR-005-AC-3 and AC-4 and new CG ownership paragraph.
- TC-007 Description, Test Procedure, Register and Expected Results.
- Current CG bound oracle, bound-strategy census and runner, graph-only Kani corpus, and coverage observation source.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
