---
id: "SR-1546"
title: "CG PR 286 code review: fault halves of the terminal-map criteria"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen@d16a37d59d8501751749eb1ff8918df66589daaa; tests/it/terminal_map.rs; diff origin/main...HEAD, base 13fc2d0"
---

# SR-1546: CG PR 286 code review: fault halves of the terminal-map criteria

## Summary

Ticket: IR-465 (also IR-460). PR: agent-ix/quire-contract-codegen#286 at d16a37d. This file is the code-review method with the rust-review lane folded in. The only code in the diff is Rust test code in `tests/it/terminal_map.rs`. The PR text and the author's report were treated as claims and measured.

What was measured:

- **src is untouched.** `git diff origin/main...HEAD -- src` is empty. `Cargo.lock` is unchanged, so qsl-replay stays at bcca433. No `#[ignore]` was added. The diff replaces one test function per file section: `tc_040_a_fault_settlement_is_failed` becomes `tc_040_a_fault_in_any_replay_wrapper_is_failed`, and `tc_041_a_counterexample_with_a_cg_defect_is_failed` becomes `tc_041_a_counterexample_with_a_fault_or_a_cg_defect_is_failed`. The helper `for_each_fault` is added. Net test count is unchanged.
- **The wrapper list is complete.** Every `From<&_> for ReplaySettlement` impl was enumerated (src/replay/function.rs:557-633, frame.rs:270, state_clause.rs:284 and 306), along with the fault-bearing variants of the locked QSL:
  - `ReplayRefusal::Fault` (execute.rs:231)
  - `AdmissionFailure::Fault`, reached as `ReplayRefusal::Admission`
  - `CallSiteRefusal::Fault` (call_site.rs:335)
  - `DependencyInputRefusal` has no fault variant, so `DependencyLockError` carries none.
  - `EvidenceFailureCause` and `StateClauseReplayResult` carry none.
  - QSL turns a `CheckCause::InternalFault` into a stage fault before any `CompileRefusal` is built (`fault_of`, spine/lifecycle.rs:306).

  So the fault positions that can reach the map are exactly 13, and `for_each_fault` builds all of them:
  - `ReplaySettlement::Fault`
  - the 2 `ReplayRefusal` faults × 4 positions: bare `Refused`, `SpineReplayError::Refused`, `FrameReplayError::Refused` and `StateClauseReplayError::Refused`
  - `CallSiteRefusal::Fault` × 4 positions: bare, `ReplayPackageError::CallSite`, `FrameReplayError::CallSite` and `StateClauseReplayError::CallSite`

  No wrapper or variant is missing.
- **Failed is the right value.** FR-029 reads "fault: an `InternalFault` anywhere in the error ... Each maps to `Failed`". QSL's `TerminalValue::from_replay_refusal` (proof_result.rs:253) returns `Failed` for both `ReplayRefusal` faults.
- **The oracle is strong. Hand mutations were run on a scratch copy of the head and reverted, never pushed.**
  - (M1) `FrameReplayError::Refused` read as `SetupRefused(code)`: killed by both tests.
  - (M2) `CallSiteRefusal::Fault` read as `Reproduced`: killed.
  - (M3) `StateClauseReplayError::CallSite` read as `SetupRefused(code)`: killed.
  - (M4) `ReplaySettlement::Fault` mapped to `Declined` in `settled`: killed.
  - (M5) one `check(...)` dropped from `for_each_fault`: killed by the count assertion in both tests.
  - (M6) count literal changed to 12: killed.
  - (M7) `SpineReplayError::Refused` read as `SetupRefused(code)`: killed.
  - (equivalent) `CallSiteRefusal::Fault` read as `CgDefect` survives. That is correct, because both read as `Failed` and FR-029 asks for nothing finer.
- **Rust idiom is fine.**
  - `ReplayRefusal` is not `Clone`, so fresh values come from `fn` pointers, as the comment says.
  - The counting wrapper closure shadows `check` legally.
  - The count assertion catches a wrapper silently dropped from the helper.
  - The doc comment's claim "neither `Refuted` nor a refusal carrying a code" is implied by the `assert_eq!(.., Failed)`.
- **`make ci` passes on the head**, run by the reviewer with its own target dir: 173 lib tests, 389 it, 25 ignored, exit 0. `make kani` was not run.

## Verdict

Sound. Approve after the low finding below, or accept it as is. The tests are real, tagged and failable, and they cover every fault position the code can produce. The one defect is a trace-tag artefact in a doc comment.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | tc_041 doc comment names `FR-029-AC-10` in prose; quire reads it as a trace tag and reports it in `unmatched_tags` (tc_041 tagged FR-029-AC-10 outside its TC-041 row) | tests/it/terminal_map.rs:1131 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | low | The fix round's rewording of the tc_041 doc comment left one line 110 characters wide, past the repo's 100-character width (rustfmt.toml `max_width = 100`); stable rustfmt does not wrap comments, so `make fmt-check` does not catch it | tests/it/terminal_map.rs:1132 |

## Dispositions

Round 1 was reviewed at 87918933f8ff9be4d3d4fa0b2003089565ff61aa (delta d16a37d..8791893, one commit).

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 8791893: the tc_041 doc comment now reads "every wrapper the run-outcome map's fault criterion lists". The only id-shaped tokens left are on its `Trace: FR-030-AC-10, TC-041` line. `quire coverage --json` at 8791893 has no FR-029 or FR-030 entry in `unmatched_tags`. |

Round 2 was reviewed at 10046f16869fdf40d75e5c3d1fcbc9147bf154b7 (delta 8791893..10046f1, one commit). The delta touches doc-comment lines only (`///`), and no code token changed.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-002 | fixed | 10046f1: the tc_041 doc comment is rewrapped. No line in tests/it/terminal_map.rs is wider than 100 characters. |
