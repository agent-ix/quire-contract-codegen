---
id: SR-8669
title: "IR-460 PR 375 spec-review/ears review"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-codegen; spec/kani/functional/FR-029-run-outcome-terminal-record.md, spec/replay/functional/FR-024-counterexample-envelope-intake.md"
review_set: subset
---

## Summary

Ticket: IR-460. Frozen head `d0572ffcac82e9ef487109f456c6d5f678bfb3f4`. The edited requirement statement names the adapter and a measurable exact-output response. The FR-024 explanatory note is context rather than a new shall statement.

## Verdict

PASS. No diff-scoped defect found. QSL-634 merge commit `03d3f899` is an ancestor of locked QSL `b8981dc20a6c8c03166f996cd9ee5fbaf73bed65`.

## Examined scope

- `FR-029 Behavior` (examined) at `spec/kani/functional/FR-029-run-outcome-terminal-record.md`: For IR-460, the Kani adapter shall read a post-state value outside its field's declared range as the subject's output and so as the witness, not as a refused input.
- `FR-024 post-state note` (context_only) at `spec/replay/functional/FR-024-counterexample-envelope-intake.md`: Out-of-range post-state values are preserved as observations, while an out-of-range pre state or argument remains refused by QSL admission (QSL-634).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Focused normal and real Kani FR-029-AC-18 tests passed. In-range real Kani control passed. Computed matrix lists both FR-029-AC-18 binders. Plan completion: not assessed. Full gates were reserved for the coder and were not run in this diff-scoped review.
