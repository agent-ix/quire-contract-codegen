---
id: "SR-1411"
title: "CG PR 260 code review and Rust review: state and frame Kani harnesses end with their cover"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen@a85f1cf666589eb5599328705df7ff737ca2f849; src/kani/generate/frame.rs, tests/it/kani_obligations_state_frame.rs, tests/state_frame_support/subject.rs (diff origin/main...HEAD, one commit)"
---

# SR-1411: CG PR 260 code review and Rust review

## Summary

Ticket: IR-451. PR: agent-ix/quire-contract-codegen#260 at a85f1cf. This file holds the
code-review method with its rust-review lane.

What the PR does. `postcondition_body` and `frame_body` in `src/kani/generate/frame.rs` now emit
`kani::cover!` after the `assert!` calls instead of before them. A default-lane assertion checks
that the last `assert!(` comes before the first `kani::cover!(` in both harnesses. The IR-412
fixture subjects no longer act only on funded accounts. The classifier is unchanged.

How I checked it, with real Kani 0.68.0 and CBMC 6.11.0, on a scratch crate whose harnesses
have the same shape as `frame.rs`. The flags were the production ones: `-Z concrete-playback
--concrete-playback print --output-format regular --solver cadical`.

- The defect reproduces with the cover before the assertion, for both the postcondition harness
  (`deposit_debiting`) and the frame harness (`deposit_touching_audit`). The run reports
  `1 of N failed` and the cover as satisfied, but the console prints one playback block, headed
  ``Check for `cover` `` at (0, 0). No assertion block is printed, so `counterexample_playback`
  finds nothing and the run classifies `FailedWithoutCounterexample`.
- Why. Kani names each playback test `kani_concrete_playback_check_<hash>`. The hash covers the
  concrete values only: all-zero hashes to `14500938541994739385` in every harness I ran. Kani
  keeps one block per name. So one block per distinct valuation is how Kani behaves, not a side
  effect of this setup. When the cover and the failing check land on the same valuation, the
  check listed first keeps its block. With the cover first, that is the cover.
- With the cover after the assertion, the same subjects print the assertion block at (0, 0)
  first. Where a cover block is printed at all, it is at a different, passing valuation
  ((1000, 0) and (512, 512)). The run classifies `Falsified`.
- An assertion that fails does not return. With a one-valuation domain that fails, the cover
  after the assertion is reported `0 of 1 cover properties satisfied (1 unreachable)`. The run
  still fails with the assertion's playback. `classify_report` goes to the failure arm and never
  reads the covers, so the result is `Falsified`, not `CoverUnsatisfied`. With the cover before
  the assertion, the same harness loses its counterexample.
- The cover still guards against vacuity. When every assertion holds everywhere, a cover after
  them is reached by exactly the valuations that reach a cover before them. On the `Verified`
  path the guard is therefore unchanged, and on the failure path covers are not consulted. An
  unsatisfiable-assumption harness with the cover last reports `VERIFICATION:- SUCCESSFUL` with
  `0 of 1 cover properties satisfied`. That is not `Verified` under FR-017. The contract harness
  (`contract.rs`, whose check is inside the `proof_for_contract` call) and the scalar harness
  (`scalar.rs`) already put the cover after the check.
- Mutation. I ran a throwaway worktree with only `frame.rs` reverted to `origin/main`, using its
  own target dir. `tc_025_a_postcondition_yields_a_contract_harness_and_a_scoped_frame_harness`
  fails. All three real-Kani tests fail with `Inconclusive { reason:
  FailedWithoutCounterexample }`. At the head commit all three real-Kani tests pass (3 of 3).
- Other emitters. `precondition.rs` contains only the cover. `contract.rs` and `scalar.rs` put
  the cover after the check. The corpus harness (`corpus/bounded_kani_corpus.rs`) and the V1
  bundle harness (`v1_bundle.rs`) emit no cover at all, which is pre-existing and outside this
  diff (see SR-1412). The `lower/` graph, collection and arithmetic modules render no harness
  text. No other emitter puts a cover before an assertion.
- NFR-005. The production change only reorders string literals: no panic token, index or
  unchecked subtraction. The fixture uses `wrapping_sub` and `wrapping_add`.
- Rust review. The new test assertion uses `Option::zip` and `is_some_and`, with no `unwrap`, and
  is traced FR-015-AC-26. Formatting and clippy are clean (`make ci`, below).

## Verdict

Correct, and the fix is the right one: it changes the harness shape and leaves the classifier
alone, as FR-017 requires. I measured both the mechanism the PR describes and the fix. Three
low findings, none blocking. Mergeable.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The module doc of `subject.rs` says "Each variant acts at every valuation, the all-zero one included". `deposit` and `deposit_touching_audit` do nothing at `balance == 1000`. Only `deposit_debiting` acts at every valuation. The per-function doc of `deposit_touching_audit` ("at every valuation the operation credits") is the accurate wording. | tests/state_frame_support/subject.rs:4 |
| FND-002 | low | The real-Kani lane catches the regression only because CBMC happens to pick (0, 0) for both the cover and the failing check: measured, the mutant fails 3 of 3 today. Nothing forces the two valuations to coincide. A different solver model could pick a different valuation for the cover, and the lane would then pass with the cover before the assertion. The default-lane text-order assertion is deterministic and does pin the order, so this is a robustness note on the real-Kani control only. A one-valuation domain fixture would make the collision certain. | tests/it/kani_obligations_state_frame.rs:999-1045 |
| FND-003 | low | The new rationale sits in the doc comment of `const HARNESS: &str = "check"`, which is the harness function's name. The rule it describes (every body ends with the cover, after the assertions) belongs to the body emitters `postcondition_body` and `frame_body`. A reader of either function does not see it. | src/kani/generate/frame.rs:952-961 |

## Dispositions

Round 1, reviewed at b2598f8f468629fd2ebba5eaa2d9193f0b403850 (fix commit b2598f8 over a85f1cf).

How I re-checked:

- I ran the new real-Kani test
  `tc_025_real_kani_a_violation_at_the_only_valuation_is_a_counterexample_not_the_covers_playback`
  at the head. Both state fields are bounded to (0, 0), and those bounds flow into the
  `kani::assume` lines, as `tc_025_each_field_is_assumed_in_its_own_declared_range` shows. So the
  harness can draw exactly one valuation.
- In a throwaway worktree with its own target dir, I put the cover back before the assertion in
  `frame.rs` (main's copy) and in `scalar.rs` (edited by hand). The default-lane order tests for
  the state/frame and scalar harnesses both fail. The real-Kani lane fails 4 of 4 with
  `Inconclusive { FailedWithoutCounterexample }` on two consecutive runs. That includes the new
  one-valuation test, which can only fail this way.
- At the head, the state/frame real lane passes 4 of 4.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed b2598f8 | The module doc now reads "Each variant acts at the all-zero valuation, and `deposit_debiting` at every valuation; `deposit` and `deposit_touching_audit` act at every `balance` below 1000", which matches the code. |
| FND-002 | fixed b2598f8 | The new one-valuation real-Kani test makes the cover and the failing check share their valuation whatever model the solver picks. The mutant fails it deterministically (measured twice). |
| FND-003 | fixed b2598f8 | The rationale moved to `postcondition_body`. `frame_body` refers to it. `const HARNESS` keeps only its one-line doc. |
