---
id: SR-12000
title: "Code review of IR-827 CBMC failure cause (PR 410)"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen@f05005ab2f135ef285bdc27fb9c08487b035c3fa (PR 410); src/kani/classify.rs, src/kani/mod.rs, src/kani/output/diagnostics.rs, src/kani/output/report.rs, src/kani/run/execute.rs, src/kani/test_support.rs, src/lib.rs, tests/it/kani_obligations.rs (diff origin/peter/ir-827-cbmc-failure-cause-spec...f05005a)"
review_set: subset
---

## Summary

Ticket: IR-827. PR quire-contract-codegen#410 at f05005a, stacked on spec PR #409 (902b867),
which stacks on #406 and #403. Reviewed the code diff only, with the rust-review lane folded in.

Checked:

- Classification is unchanged. `ClassifiedRun.exit_status` is only written in classify.rs (lines
  101, 144 and 205). `classify_report` never reads it, and terminal.rs is not in the diff.
- `exit_code_of` handles its edges: an empty value, `-`, `-0`, a leading zero, `+`, non-ASCII
  digits, i32 overflow and the i32 minimum.
- `cut_to_limit` cuts on a character boundary. It runs after redaction.
- The `excerpt` refactor into `redact_text` is byte-identical: the same five passes in the same
  order over the same `&str`. IR-809's diagnostics tests pass.
- The single-entry rule (`reported_exit_status`) needs a `harness_id`. A duplicate entry gives
  `None`. The batch timeout rule still reads every entry through `any`.
- A lone surrogate is still refused as `Malformed`. `RawErrorDetail.exit_status` was already
  `Option<String>`, and there are tests at both the report level and the execute level.
- The probe macro is `#[cfg(test)]`, and `test_support` is `#[cfg(test)]`. `$crate::` paths are
  outside the layout.rs scan by design, and `test_support` is exempt in `kani_order_violation`.
  `report.rs` imports only `super::diagnostics`, inside `output`.

Gates run on f05005a:

- Passed: `cargo fmt --check`, `cargo clippy --all-targets -D warnings`, `cargo test --lib kani::`,
  `cargo test --test it -- kani_obligations layout diagnostics` (132 passed) and
  `cargo test --doc` for `KaniFailureCause` and `KaniNoVerdictDiagnostics`.
- Real-Kani AC-40 test, run once in a MemoryMax=16G scope: green in 2.66 s. Its assertion
  `failureCause == {kind: exit_code, code: 64}` held.
- `identity_ceilings_govern_execution_and_successful_evidence_records_observed_memory`
  failed in 2 of 6 parallel `kani::` runs on this head and 2 of 6 on base 902b867. It passed 5 of
  5 when run alone. This PR does not make it worse, so it stays IR-808's item.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The public rustdoc of `KaniExecutionEvidence::diagnostics` still says "Present exactly when the outcome is NoVerdict". The type doc of `KaniNoVerdictDiagnostics` still says "Why a NoVerdict run has no verdict". This PR attaches the record to `FailedWithoutCounterexample` evidence whose cause is a CBMC failure, so the stated invariant is now false. Scenario: a consumer that follows the rustdoc and treats `diagnostics.is_some()` as a NoVerdict test misfiles an `exit_code_64` CBMC failure as no-verdict. The spec states the exception (FR-017 line 366), but the API doc does not. | src/kani/run/execute.rs:290-293; src/kani/output/diagnostics.rs:45 |
| FND-002 | low | `ClassifiedRun` is re-exported from lib.rs:133, has all fields `pub` and is not `#[non_exhaustive]`. It gains the `pub(crate)` field `exit_status`, so code outside the crate can no longer build or exhaustively destructure it. The hidden field also takes part in the derived `PartialEq` and `Debug`. Scenario: an external test that asserts `classify_kani_run(..) == ClassifiedRun { outcome, success_checks, checks }` no longer compiles, and the expected value cannot be built at all. Two runs whose public fields are equal can also compare unequal on a field the caller cannot see. No consumer breaks today: tests/it and quire-integration only call `classify_kani_run` and read its fields, and the crate has `publish = false`. A side channel would avoid the break, for example returning the exit status beside the `ClassifiedRun` from an internal classify entry point. | src/kani/classify.rs:88-102; src/lib.rs:132-133 |
| FND-003 | low | Downstream routing. `KaniExecutionEvidence` gains another public field, `failure_cause`. quire-driver origin/main (5be46b3) builds the struct as a literal in its src/provider.rs test fixture, around line 482, and pins CG `branch = "main"`. The next CG bump fails with E0063 on a missing `failure_cause`, as well as on IR-809's `diagnostics`. IR-809 FND-002 handed the quire-driver literals to that repo's owner, and that handoff now also needs `failure_cause: None`. | src/kani/run/execute.rs:294-298; quire-driver/src/provider.rs:482 |

## Verdict

**CONDITIONAL**: one medium and two low findings. Nothing changes behavior. FND-001 is a public
doc contract that the code now violates, and should be fixed in this PR. Classification and the
FR-029 value are unchanged, the parse and cut edges are correct, the single-entry rule holds and
IR-809's excerpts are byte-identical. On items (7) and (9) of the brief: the timed-out and
memory-exhausted no-cause assertions hold by construction, because `launch_evidence` hard-codes
`exit_status: None` for those launches. That is acceptable. They pin the contract that a stopped
run never reads its report, and the gate in `Record::of` is covered separately by the six
other-outcome cases. The flaky identity_ceilings test is not made worse.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1 (a84a772): the `KaniExecutionEvidence::diagnostics` rustdoc now says the field is present for a `NoVerdict` outcome and for a `FailedWithoutCounterexample` outcome whose cause is `timeout`, `out_of_memory`, `exit_code` or `unknown`, and absent otherwise. The `KaniNoVerdictDiagnostics` doc and the module doc say the same. Checked against `Record::of`: the record is kept only when the cause is a CBMC failure or the outcome is NoVerdict. The phase is `unknown` because a report was exported. `cargo doc -D warnings` is clean. |
| FND-002 | fixed | Round 1 (a84a772): `src/kani/classify.rs` is byte-identical to base peter/ir-827-cbmc-failure-cause-spec, with an empty `git diff`, so `ClassifiedRun` is unchanged. The exit status now travels beside the run. In a batch it comes from `member.report.exit_status`. In a single run, `single_exit_status` re-parses the report only after the outcome is FWC. classify.rs never names `exit_status`. |
| FND-003 | deferred | Routed downstream by the team leader (IR-809 comment 93cf3a2c) to the quire-driver owner, together with IR-809's `diagnostics` literal fix. It is out of this repo's scope. |
