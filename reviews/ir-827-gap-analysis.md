---
id: SR-12001
title: "Gap analysis of IR-827 FR-017-AC-37 to AC-40 (PR 410)"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen@f05005ab2f135ef285bdc27fb9c08487b035c3fa (PR 410); spec/kani/functional/FR-017-kani-execution-evidence.md AC-37..AC-40, spec/kani/matrix/TC-027-kani-execution-evidence.md, spec/kani/matrix/TC-043-kani-batching-and-output-bounds.md, spec/kani/matrix/tests.md, src/kani/output/report.rs, src/kani/run/execute.rs, tests/it/kani_obligations.rs"
review_set: subset
---

## Summary

Ticket: IR-827. The audit was repository-driven over the PR diff, and plan completion was not
assessed. `quire matrix --format json` (quire 0.36.2, engine 0.50.2) reports FR-017-AC-37 to
FR-017-AC-40 all `tagged`, with 3, 6, 2 and 3 binders. The one ignored binder is AC-40's `make
kani` lane test. I ran it once against real Kani in a MemoryMax=16G scope and it was green: exit
code 64 became `failureCause {kind: exit_code, code: 64}`, and the stderr excerpt holds `Unknown
option: --no-such-option`. I checked each AC clause against an asserting test, and judged
test-oracle strength.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | In `tc_027_the_terminal_value_of_a_cbmc_failure_ignores_its_cause`, the half of AC-40 that says "equal to the terminal value of the same evidence with failureCause and diagnostics absent" cannot fail. `run_terminal_value(outcome, success_checks, settlement)` never receives the evidence, so `with_cause` and `without` are the same call on the same values. Only the `Failed` assertion has power. This repeats IR-809 SR-11908 FND-001 for `diagnostics`. The property holds by the function's signature, and TC-027 should cite the signature rather than this comparison. | tests/it/kani_obligations.rs:2376-2382; src/kani/terminal.rs:90-94 |

## Verdict

**CONDITIONAL**: one low finding. Coverage is real. Every flipped row in tests.md, TC-027 and
TC-043 has a test that asserts its clause:

- AC-37: the vocabulary, all 6 exit codes, the 7 non-canonical values, the 64-byte, 65-byte and
  100-byte cuts, the cut that straddles a 4-byte character, the crate directory, ANSI and
  pid/duration redaction, and a record on every case.
- AC-38: properties_failed with no record; the 5 silent shapes; the 6 other outcomes; timed-out
  and memory-exhausted runs; number and lone-surrogate refusals; the compile_fail doctest; and the
  autoref probe with its String control.
- AC-39: the five-member batch and the two-entry timeout case.
- AC-40: independence over full evidence equality, with only cause, record, memory and report
  path stripped; the Failed terminal value; and the real-Kani lane run.

The timed-out and memory-exhausted no-cause assertions hold by construction. `launch_evidence`
never reads the report for those launches. They are kept as a regression pin, not counted as an
oracle on `Record::of`.

## Coverage

- Reconciliation: quire matrix (quire 0.36.2) — no run evidence read
- Plan completion: not assessed
- Criteria: tagged 4 · untagged 0 · tagged-by-ignored-test 0 (AC-40 has non-ignored binders as well as its ignored lane test) · method-without-symbol 0
- Untraced behaviors / stubs: 0
- Semantic review: ran over FR-017-AC-37 to FR-017-AC-40 (the dispatching brief asked for test-oracle strength)

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1 (a84a772): the tautological with/without comparison is gone. The test now asserts only that the terminal value is `Failed` for each of the three causes. The test doc and TC-027 say that independence holds by the signature of `run_terminal_value`, which takes only the outcome and the SUCCESS-check count. AC-40 keeps 3 binders, and the real-Kani lane test was re-run green on a0280e8. |

## Follow-up (disposition pass 2)

Reviewed candidate: `4b1f2ad41924ff3532fbc488adc9d9b25b7bd251`; stacked parent: `1b2ab7462688d6e0709e0b0e5fcd56320da3d7a0`.

Gap follow-up: the #410 delta is clean. Repository-wide assurance remains FAIL
on existing baseline deficits; this is not a whole-repository PASS.

Fresh whole-repository computations: quoin matrix 0.28.4 and quire matrix 0.36.2 /
engine 0.50.2 report 821 criteria: 454 tagged, 308 untagged, 14 tagged only by ignored
tests, 45 method-without-symbol. All 821 have no run evidence in the evidence store.
The 812 criteria shared with SR-12730's baseline retain identical static status.
The only additional criteria are parent's AC-32..36 and #409's AC-37..40, all tagged.
AC-37..40 have respectively 3/6/2/3 binders, including active non-ignored tests for
AC-40 beside its documented make-kani test. The fresh static matrix equals both
coder before/after matrices. Five unresolved tags (normalized without source offsets)
and ten declaration diagnostics exactly match the published baseline. The existing
327 SR-12730 findings retain their routing through IR-726 / IR-863 / IR-865 and
are neither republished nor claimed fixed.

Read the prior whole-tree reverse-gap/stub inventory, then inspected each changed
production behavior and its tests. KaniFailureCause, reported_exit_status,
exit_code_of, cut_to_limit, single_exit_status and Record are owned by AC-37..40;
redact_text reuses AC-35's existing redactor. KaniExecutionEvidence.failure_cause
is the specified new public surface. No new unspecified behavior, concrete stub,
mock-only oracle or hollow assertion was found in this delta. All fourteen new
test binders were followed to their assertions. Existing stopped-run cause checks
remain construction-level regression pins, distinct from the six outcome-gate cases.
No optional new semantic audit was commissioned; this follows up the original
reviewed intent/test/code checks rather than expanding the review's requirements.
Plan completion: not assessed.

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
