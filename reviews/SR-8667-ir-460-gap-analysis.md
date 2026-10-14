---
id: SR-8667
title: "IR-460 PR 375 gap-analysis review"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; spec/kani/functional/FR-029-run-outcome-terminal-record.md, spec/kani/matrix/TC-040-run-outcome-terminal-record.md, spec/replay/functional/FR-024-counterexample-envelope-intake.md, tests/it/kani_obligations_state_clause_replay.rs"
review_set: subset
---

## Summary

Ticket: IR-460. Frozen head `d0572ffcac82e9ef487109f456c6d5f678bfb3f4`. Computed matrix tags both tests to FR-029-AC-18. The normal replay and real Kani test exercise QSL and the CG terminal mapper. Plan completion: not assessed.

## Verdict

PASS. No diff-scoped defect found. QSL-634 merge commit `03d3f899` is an ancestor of locked QSL `b8981dc20a6c8c03166f996cd9ee5fbaf73bed65`.

## Examined scope

- `FR-029-AC-18` (examined) at `spec/kani/functional/FR-029-run-outcome-terminal-record.md`: For the `falsified` state-clause run of the wrapping debit subject `deposit_debiting`, whose post-state value (-1) lies outside `balance`'s declared range, this repository builds the post snapshot with that exact unclamped value and does not refuse it itself (not as `StateClauseReplayError::OutOfDomain` or any other CG defect), and the run maps to `Refuted`, the ordinary violated terminal, and not to `Inconclusive(ReplayRefused)` or `Failed`. QSL settles the exact post-state observation as a reproduced violation (QSL-634).
- `TC-040 step 17` (examined) at `spec/kani/matrix/TC-040-run-outcome-terminal-record.md`: Map the `falsified` state-clause run of `deposit_debiting`, whose post-state value lies outside its field's declared range, with the exact unclamped post snapshot (FR-029-AC-18, IR-460).
- `FR-024 post-state note` (examined) at `spec/replay/functional/FR-024-counterexample-envelope-intake.md`: Out-of-range post-state values are preserved as observations, while an out-of-range pre state or argument remains refused by QSL admission (QSL-634).
- `FR-029-AC-18 trace` (examined) at `tests/it/kani_obligations_state_clause_replay.rs`: Trace: FR-029-AC-18, TC-040 appears on both the normal and ignored Kani tests.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Focused normal and real Kani FR-029-AC-18 tests passed. In-range real Kani control passed. Computed matrix lists both FR-029-AC-18 binders. Plan completion: not assessed. Full gates were reserved for the coder and were not run in this diff-scoped review.
