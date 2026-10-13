---
id: SR-8666
title: "IR-460 PR 375 code-review review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; spec/kani/functional/FR-029-run-outcome-terminal-record.md, spec/kani/matrix/TC-040-run-outcome-terminal-record.md, spec/replay/functional/FR-024-counterexample-envelope-intake.md, tests/it/kani_obligations_state_clause_replay.rs, tests/state_frame_support/subject.rs"
review_set: subset
---

## Summary

Ticket: IR-460. Frozen head `d0572ffcac82e9ef487109f456c6d5f678bfb3f4`. Rust/code review of the two added tests, fixture comment, and changed spec-to-test bindings. Both focused tests and the in-range control passed; no changed production API or gate was found.

## Verdict

PASS. No diff-scoped defect found. QSL-634 merge commit `03d3f899` is an ancestor of locked QSL `b8981dc20a6c8c03166f996cd9ee5fbaf73bed65`.

## Examined scope

- `FR-029-AC-18` (examined) at `spec/kani/functional/FR-029-run-outcome-terminal-record.md`: For the `falsified` state-clause run of the wrapping debit subject `deposit_debiting`, whose post-state value (-1) lies outside `balance`'s declared range, this repository builds the post snapshot with that exact unclamped value and does not refuse it itself (not as `StateClauseReplayError::OutOfDomain` or any other CG defect), and the run maps to `Refuted`, the ordinary violated terminal, and not to `Inconclusive(ReplayRefused)` or `Failed`. QSL settles the exact post-state observation as a reproduced violation (QSL-634).
- `TC-040 step 17` (examined) at `spec/kani/matrix/TC-040-run-outcome-terminal-record.md`: Map the `falsified` state-clause run of `deposit_debiting`, whose post-state value lies outside its field's declared range, with the exact unclamped post snapshot (FR-029-AC-18, IR-460).
- `FR-024 post-state note` (examined) at `spec/replay/functional/FR-024-counterexample-envelope-intake.md`: Out-of-range post-state values are preserved as observations, while an out-of-range pre state or argument remains refused by QSL admission (QSL-634).
- `tc_040_out_of_range_debit_post_state_is_exact_and_refuted` (examined) at `tests/it/kani_obligations_state_clause_replay.rs`: The debit at the declared floor leaves an exact out-of-range post-state observation. QSL replays it as a violated witness and the ordinary falsified terminal is refuted.
- `tc_040_real_kani_out_of_range_debit_replays_as_refuted` (examined) at `tests/it/kani_obligations_state_clause_replay.rs`: Real Kani playback of the wrapping debit starts at the range floor. The native subject's -1 post state survives CG's snapshot construction and QSL settles the violation.
- `deposit_debiting` (examined) at `tests/state_frame_support/subject.rs`: account.balance = account.balance.wrapping_sub(1);

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Focused normal and real Kani FR-029-AC-18 tests passed. In-range real Kani control passed. Computed matrix lists both FR-029-AC-18 binders. Plan completion: not assessed. Full gates were reserved for the coder and were not run in this diff-scoped review.
