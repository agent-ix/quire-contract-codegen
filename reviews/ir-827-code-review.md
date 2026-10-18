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

## Follow-up (disposition pass 2)

Reviewed candidate: `4b1f2ad41924ff3532fbc488adc9d9b25b7bd251`; stacked parent: `1b2ab7462688d6e0709e0b0e5fcd56320da3d7a0`.

Code/Rust follow-up: PASS for this maintenance delta, with original FND-001/002
fixed and FND-003 still deferred to the already-routed quire-driver owner.

Reviewed the parser, report selection, cause extraction and typed refusal paths;
KaniFailureCause remains Serialize-only with canonical i32 decimal recognition,
redaction before the 64-byte UTF-8-safe cut, and Unknown preservation of unrecognized
statuses. A cause is retained only after FailedWithoutCounterexample classification.
Malformed numeric/lone-surrogate values still return KaniReportRefusal::Malformed
inside KaniExecutionRefusal::Report, with no evidence. The unique-harness-entry rule
and independent batch timeout any-entry rule are preserved. No new refusal code,
stringly error envelope, panic path, unsafe code, asynchronous/lock mechanism,
dependency, duplicated upstream artifact or gate weakening is introduced.

The classifier and terminal files, Cargo.toml/lock, CI workflows, launcher/memory
mechanism and fixture files have no #410 diff from the refreshed parent. Inherited
main RSS and scoped-frame behavior therefore remains intact. Record::of preserves
the parent's NoVerdict diagnostics and extends records only to the CBMC failures
specified by AC-37..40. The redactor refactor keeps the same five operations in the
same order. The terminal test now asserts Failed and cites signature independence;
its removed vacuous comparison has not returned. IR-848 terminal reclassification
is later work and is not introduced here.

Follow-up of the frozen pushed candidate for IR-827 PR #410 against its refreshed
#409 parent. This review covers #410's own delta; inherited #406/#409 changes are
context. All seven original commits retain the reviewed implementation. The two
report.rs import joins retain the parent's ix_trace_rs::trace import and the later
removal of the obsolete Deserialize probe import. Independent full-file comparison
confirms the maintenance edit is exactly fourteen test trace doc lines converted to
imported #[trace] plus one production enum doc line changed to a prose citation.
All criterion IDs remain identical; no production or proof function gains an attribute.
The four spec patch payloads are byte-identical to the previously reviewed original
#410 delta. Their status updates and terminal-independence explanation add no new
normative requirement; no additional spec-review method is required.

No original finding or prior disposition was edited. Every existing finding already
has a terminal latest outcome. Round 2 appends no disposition rows, publishes
`dispositions: []`, and finds no new defect. Original review comments remain the
method records; this round posts only one disposition comment per existing artifact.
No applicable AssuranceProfile is present under spec/.

The coder's confined-checks.log was inspected: fmt/lint/spec pass (existing schema
and grammar warnings remain), 205 library tests plus two one-test child probes,
40 integration tests with 10 documented ignored Kani-lane tests, and four doctests
pass. These are direct execution logs, separate from the empty evidence store.
No Cargo, real Kani, or aggregate gate was run by this follow-up reviewer. Custody
of the latest artifacts and the leader's exact-final-head aggregate gate remain
required after #406 and #409 merge and the final rebase. PR #410 remains OPEN/DRAFT;
IR-827 is In Progress with has-review already present, consistent with pending work.

## Examined units (disposition pass 2)

```yaml
scope:
- id: FR-017-AC-32
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: context_only
  excerpt: 'In the `make kani` lane, a crate whose library source holds the generated harness byte for
    byte and, beside it, an item that does not compile (a reference to an undefined name) is run through
    `execute_kani_obligation`: the run exits unsuccessfully, exports no report and is `Inconclusive` `NoVerdict`
    with its exit code, and its evidence carries a diagnostic record whose phase is `compile`, whose stdout
    excerpt holds the rustc error code and Kani''s `error: Failed to execute cargo` line and whose stderr
    excerpt holds Cargo''s `error: could not compile` line with the crate path as `<CRATE_DIR>`. A '
- id: FR-017-AC-33
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: context_only
  excerpt: 'Over single-run launcher stand-ins that exit 1: a report of success beside the unsuccessful
    exit is `unknown` even when its stdout holds `Solving with CaDiCaL 3.0.0`; with no report, stdout
    holding the Kani banner, `Checking harness proofs::check...` and `Solving with CaDiCaL 3.0.0` is `solver`;
    the banner and the `Checking harness` line alone is `backend`; the banner and `error: Failed to execute
    cargo (exit status: 101). Found 1 compilation errors.` is `compile`, as is stderr `error: could not
    compile` alone with the banner; a usage error on stderr and nothing on stdout is `launcher`; the ba'
- id: FR-017-AC-34
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: context_only
  excerpt: 'A redacted stream of exactly 8192 bytes is kept whole with `omittedBytes` 0; one of 8193 bytes
    whose final line starts within the window keeps the longest line-aligned suffix of at most 8192 bytes
    with `omittedBytes` the byte count before it; a stream whose final line alone exceeds 8192 bytes keeps
    at most 8192 bytes ending at its last byte, starting on a character boundary; a stream that is one
    10000-byte line followed by `\n` keeps the last 8192 bytes of it, ending in `\n`, never an empty excerpt;
    a stream of a 10000-byte line, `\n`, `error: last\n` keeps exactly `error: last\n`; a 4-byte ch'
- id: FR-017-AC-35
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: context_only
  excerpt: 'With crate directory `/h/u/ws/crate`, target directory `/h/u/ws/crate/target` and home directory
    `/h/u` supplied in the request, a stand-in whose streams print `(/h/u/ws/crate)`, `/h/u/ws/crate/src/lib.rs:6:21`,
    `/h/u/ws/crate/target/debug/x.out`, `/h/u/.kani/bin`, `/h/u/ws/crate2/src/lib.rs`, `/opt/tool/bin`,
    `https://host/x/y`, `and/or`, `/`, `2026-10-16T22:27:04.275975036Z`, `Runtime Symex: 0.00331318s`,
    `Runtime Solver: 1.385e-05s`, `in 0.02s`, `check_ttl_30s`, `u32s`, `pid 4242` and `thread ''main''
    (4242) panicked` yields, in order, `(<CRATE_DIR>)`, `<CRATE_DIR>/src/lib.rs:6:21`, `<TARGET_'
- id: FR-017-AC-36
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: context_only
  excerpt: 'The diagnostic record never feeds classification: two runs with the same report and exit and
    different stream text classify to the same outcome, checks, SUCCESS count and playback, and two `NoVerdict`
    runs with equal exit and no report whose streams differ have equal evidence in every field but `diagnostics`,
    `memory` (the observed peak resident bytes vary run to run) and the run''s report-path argument (the
    last argument, unique to each run); the FR-029 terminal value of a `NoVerdict` evidence carrying a
    record is `Failed`, the value FR-029-AC-4 gives the no-verdict reason, equal to the termin'
- id: FR-017-AC-37
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'Over single-run launcher stand-ins that exit 1, print `Checking harness proofs::check...`
    and `CBMC failed with status 64` on stdout and `Unknown option: --no-such-option` on stderr, and export
    a report whose one result `proofs::check` is `Failure` with no checks and whose one `error_details`
    entry for `proofs::check` has the exit status shown: `exit_code_64` is `Inconclusive` `FailedWithoutCounterexample`
    with `failureCause` `{ "kind": "exit_code", "code": 64 }` and a `diagnostics` record of phase `unknown`
    whose stderr excerpt holds `Unknown option: --no-such-option`; `exit_code_15` has `cod'
- id: FR-017-AC-38
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'A single-run stand-in that exits 1 and exports a `Failure` report listing a failed assertion
    check with no playback on the console, whose entry for the harness has exit status `properties_failed`,
    is `FailedWithoutCounterexample` with `failureCause` `{ "kind": "properties_failed" }` and no `diagnostics`
    member. The same report with no `error_details` member, with an entry that names another harness only,
    with two entries naming the harness, with an entry whose `exit_status` is null, or whose one result
    names no `harness_id`, is `FailedWithoutCounterexample` with neither a `failureCause` nor a '
- id: FR-017-AC-39
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A batch stand-in of members `a::check`, `b::check`, `c::check`, `d::check` and `e::check` whose
    process exits 1 and whose report has `a::check` `Success` with a passing assertion, every cover satisfied
    and no exit status, `b::check` `Failure` with no checks and exit status `exit_code_64`, `c::check`
    `Failure` with no checks and exit status `timeout`, `d::check` `Failure` listing a failed assertion
    with no playback block headed for its path and exit status `timeout`, and `e::check` `Failure` listing
    a failed assertion with no playback block headed for its path and exit status `properties_failed
- id: FR-017-AC-40
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'The cause never feeds classification: three single-run stand-ins with the same `Failure` report,
    no checks and the same exit, whose entries give `exit_code_64`, `out_of_memory` and `cbmc_exploded`
    and whose streams differ, classify to the same outcome, checks, SUCCESS count and playback, and their
    evidence is equal in every field but `failureCause`, `diagnostics`, `memory` and the report-path argument;
    the FR-029 terminal value of each is `Failed`, equal to the terminal value of the same evidence with
    `failureCause` and `diagnostics` absent. In the `make kani` lane, a generated harness in a cr'
- id: FR-029-AC-4
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: context_only
  excerpt: The no-verdict reason maps to `Failed`.
bindings:
- test_id: tests::tc_027_the_exit_status_vocabulary_maps_to_its_causes
  ac_id: FR-017-AC-37
  trace: correct
- test_id: tests::tc_027_an_unknown_exit_status_is_redacted_and_cut_to_64_bytes_on_a_character
  ac_id: FR-017-AC-37
  trace: correct
- test_id: batch_tests::no_verdict_diagnostics::failure_cause::tc_027_every_exit_status_gives_its_cause_and_a_diagnostic_record
  ac_id: FR-017-AC-37
  trace: correct
- test_id: tests::tc_027_the_exit_status_is_read_only_from_the_one_entry_naming_the_harness
  ac_id: FR-017-AC-38
  trace: correct
- test_id: tests::tc_027_a_number_or_lone_surrogate_exit_status_is_still_malformed
  ac_id: FR-017-AC-38
  trace: correct
- test_id: tests::tc_027_the_cause_implements_no_deserialize
  ac_id: FR-017-AC-38
  trace: correct
- test_id: batch_tests::no_verdict_diagnostics::failure_cause::tc_027_a_failed_property_has_no_record_and_an_unnamed_entry_has_no_cause
  ac_id: FR-017-AC-38
  trace: correct
- test_id: batch_tests::no_verdict_diagnostics::failure_cause::tc_027_the_cause_is_kept_for_no_other_outcome
  ac_id: FR-017-AC-38
  trace: correct
- test_id: batch_tests::no_verdict_diagnostics::failure_cause::tc_027_a_malformed_exit_status_still_refuses_the_report
  ac_id: FR-017-AC-38
  trace: correct
- test_id: batch_tests::no_verdict_diagnostics::failure_cause::tc_043_batch_members_keep_their_cause_beside_the_timeout_rule
  ac_id: FR-017-AC-39
  trace: correct
- test_id: batch_tests::no_verdict_diagnostics::failure_cause::tc_043_two_entries_for_a_member_leave_the_timeout_rule_unchanged
  ac_id: FR-017-AC-39
  trace: correct
- test_id: batch_tests::no_verdict_diagnostics::failure_cause::tc_027_the_cause_never_feeds_classification
  ac_id: FR-017-AC-40
  trace: correct
- test_id: tc_027_the_terminal_value_of_a_cbmc_failure_ignores_its_cause
  ac_id: FR-017-AC-40
  trace: correct
- test_id: tc_027_real_kani_unknown_cbmc_option_keeps_exit_code_64_as_the_failure_cause
  ac_id: FR-017-AC-40
  trace: correct
```
