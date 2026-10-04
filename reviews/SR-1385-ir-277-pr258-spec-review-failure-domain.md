---
id: "SR-1385"
title: "CG PR 258 spec review (failure-domain): batch verdicts, batch timeout and output cap against real Kani"
type: SpecReview
analysis: failure-domain
review_set: subset
scope: "agent-ix/quire-contract-codegen@1add57d77cfcd3a9e39e515f063fde12b81f4d70; spec/kani/functional/FR-017-kani-execution-evidence.md:41-43,90-122,187,194-198, spec/kani/functional/FR-028-bounded-proof-ceilings.md:55-66,96 (diff origin/main...HEAD); measured against Kani 0.68.0 / CBMC 6.11.0 installed locally"
---

# SR-1385: CG PR 258 spec review (failure-domain)

## Summary

Ticket: IR-277. The PR marked several Kani facts as unverified. I measured them with the installed
Kani 0.68.0 (`cargo kani --version`), on a scratch crate with two verifying harnesses, one
falsifying harness and one slow harness. I installed nothing.

- **Repeated flags.** One process accepts `--harness <path> --exact` repeated once per member. The
  pairs come before the shared options (`-Z function-contracts -Z concrete-playback --unwind 2
  --solver cadical --output-format regular --concrete-playback print`), followed by
  `-Z unstable-options --export-json`.
- **Sequential.** Harnesses run one after another ("Checking harness ..." sections), and not in
  request order: Kani used reverse source order. `--help` says it runs sequentially unless `-j` is
  given. Confirmed.
- **One report at the end.** I killed the batch while its third member was running, after two
  members had printed VERIFICATION results. No report file existed. Confirmed: a batch timeout loses
  every member's result.
- **Report shape.** `verification_results.results[]` holds one entry per harness, with
  `harness_id` equal to the `--harness` value (`proofs::ok_harness`), plus `status` and `checks`.
  The report contains no playback (grep finds 0 occurrences). Playbacks are on stdout under
  "Concrete playback unit test for `<harness>`:". See SR-1386.
- **Exit codes.** A mixed batch (one Failure, two Success) exits 1. An all-success batch exits 0.
  So the exit-code rule's main case matches real behaviour.
- **Kani's own per-harness timeout.** Kani 0.68 has `--harness-timeout` (unstable). With
  `--harness-timeout 5s`, the slow member is cut off and the others keep their results. The timed-out
  entry reads `status: Failure`, `checks: []`, and `error_details[].exit_status: "timeout"`.
- **Code check.** `src/kani/output/report.rs` reads only `status` and `checks` of a harness, and
  refuses any count other than one (l.319-330). It does not read `harness_id` or `error_details`.

## Verdict

Batching is feasible: one process with repeated pairs and a per-harness report. But three of the
batch rules fail on real edges (FND-001 to FND-003), and the capture-failure criterion needs a
concrete failure source (FND-004). None is high on its own. Together with SR-1386 the PR is not
mergeable.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-028-AC-12 (N times T, then kill and mark every member timed out) throws away the finished results of N-1 members when one member is slow. The Basis says "a timeout loses every member's result", as if that were unavoidable. Kani 0.68's `--harness-timeout` cuts off only the slow member and keeps the others: measured, the timed-out entry is `status: Failure`, `checks: []`, `error_details.exit_status: timeout`. The spec should consider a per-member Kani timeout, with N times T plus slack as the outer limit. If it does, it must also say that such an entry is a timed-out member and not a failure. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:55-66,96 |
| FND-002 | medium | The 8 MiB cap is per stream per process, whatever the batch size, and no batch-size limit is stated. Measured: about 3 KB of regular-format stdout per trivial harness, and real contract harnesses print more. A large batch can go over the cap when every member alone would not, and the whole batch is then refused with `kani_output_over_limit`. Retrying "singly" is only a "may". State a member limit, a cap that scales with N, or a required split and retry. | spec/kani/functional/FR-017-kani-execution-evidence.md:93-98,105-110,187 |
| FND-003 | medium | "another member's entry states a failed check that accounts for the exit" is undefined. Kani exits 1 for any failed harness and for its own errors alike, so nothing can be "accounted" beyond "some member has a failed check". Two implementers would differ. Also undefined: whether a `Failure` entry with no checks (Kani's own timeout, a CBMC error) counts. State the rule as that predicate, and give the reason for the inconclusive case. | spec/kani/functional/FR-017-kani-execution-evidence.md:113-117,195 |
| FND-004 | low | FR-017-AC-25 says a capture thread that "fails to read" is refused. Today `capture_tail` stops on a poll or read error and returns partial bytes as if it had succeeded (`Err(_) => break`). The AC should say that read and poll errors, not only panics, become `kani_output_unread`. TC-027 should say how a test causes a read failure, since its procedure only drives a panic. | src/kani/run/launch.rs:183,194, spec/kani/functional/FR-017-kani-execution-evidence.md:198, spec/kani/matrix/TC-027-kani-execution-evidence.md:49-52 |

## New findings (disposition pass 1)

Reviewed at a0eba86e850e08f0e19edfa99ce804b166dfda53, re-measured on Kani 0.68.0 / CBMC 6.11.0:

- `--harness-timeout` without `-Z unstable-options` exits 2 with "requires `-Z unstable-options`".
- A bare number is taken as seconds.
- A member that times out reads Failure, `checks: []`, `error_details.exit_status: "timeout"`, `error_type: "unknown_failure"`.
- A batch of one timed-out member and one Success member exits 1.
- `--harness-timeout 0` times every member out at once.
- Playback headings read "Concrete playback unit test for `<module::harness>`:".

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | medium | Kani 0.68 accepts `--harness-timeout` only up to 4294967295 seconds. Measured: 4294967296, 9223372036854775807 and 18446744073709551615 each exit 2 with "Invalid timeout value" and write no report. The spec passes T "whole seconds, rounded up" with no clamp. Yet FR-017-AC-15 supports a `Duration::MAX` timeout, and the outer bound is told never to elapse when N times T does not fit. So any batch with T above about 136 years fails to launch at all. State a clamp: cap the flag at 4294967295 s, or leave the flag out when T does not fit. | spec/kani/functional/FR-017-kani-execution-evidence.md:117-127,232 |
| FND-006 | medium | No rule covers a batch process that exits on its own without writing a report. Measured: a rejected flag gives exit 2 and no file, and a build failure does the same. Only a batch killed at N times T is refused. The single-run rules (refused after exit 0, `NoVerdict` after a non-zero exit) are scoped to single runs, and FR-017-AC-23 covers only a report that lacks a harness. State the batch counterpart and test it. | spec/kani/functional/FR-017-kani-execution-evidence.md:122-127,156-158,187-191,234 |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed a0eba86 | Each member is now held to T through Kani's `--harness-timeout`, and a timed-out entry (Failure, no checks, exit status `timeout`) is inconclusive timed-out while the other members keep their results. N times T is an outer bound for a wedged backend only. Re-measured: the flag needs `-Z unstable-options`, a bare number is seconds, and the entry shape is as the spec states. |
| FND-002 | fixed a0eba86 | The cap is now 8 MiB times the member count per stream, so each member gets the same bound a single run gets, and a refused batch is not split or retried. This is honest, not a limit raised to fit: the bound is the sum of N single-run bounds. What remains is that peak memory grows with N, since no batch-size limit is set. The Rationale leaves that choice to the owner. |
| FND-003 | fixed a0eba86 | The rule is now an explicit predicate: a member's success is verified on exit 0, or on a non-zero exit when at least one member's entry states failure. A Failure entry with no checks is split into the timeout case and the no-counterexample case. Measured: a batch with one timed-out member and one Success member exits 1, and the Success member is verified because the timeout entry states failure, which is right. A Kani error exit with one Success entry and no Failure entry is inconclusive, which is conservative. An error exit with no report is FND-006. |
| FND-004 | fixed a0eba86 | FR-017-AC-25 and its Behavior bullet now cover a panic and a failed poll or read, and TC-043 step 2 drives both. |
| FND-005 | fixed ce7e0f0 | If T rounded up exceeds 4294967295 s, the batch is launched without `--harness-timeout` (FR-017 l.122-124, FR-028 l.55-59, AC-21, AC-12, TC-043 step 5). Re-measured on Kani 0.68.0: a batch at 4294967295 runs and writes a report; 4294967296 exits 2 with "Invalid timeout value" and no report; a batch with the flag left out runs and reports per member. |
| FND-006 | fixed ce7e0f0 | Exit 0 with no report refuses the batch with the single-run refusal. An unsuccessful exit of its own with no report gives every member inconclusive `NoVerdict` (FR-017 l.165-171, AC-21, TC-043 step 6). This mirrors the single-run rule (FR-017 l.207, AC-18) and covers every shape of no-report exit. A process that exits by itself either succeeded or did not; an unsuccessful exit includes a signal from outside, where the exit code is None. A process the generator kills, at N times T or at the output cap, has its own refusal. Re-measured: a failed build exits 1 with no report and an argument error exits 2 with no report, and both fall under the NoVerdict rule. |

## New findings (disposition pass 2)

Reviewed at ce7e0f0d455710b4b3a742665de01627f6f397e2.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-007 | low | Wording only. FR-028 says that when T rounded up exceeds 4294967295 s, "The outer bound below then does not elapse either, as in FR-017-AC-15". That holds only when N times T does not fit, as with a `Duration::MAX` request. For T between 2^32 s and `Duration::MAX`/N, the product does fit, and it elapses after more than 136 times N years. There is no practical effect. Saying "does not elapse when the product does not fit" would make it exact. FR-017 l.124 ("no per-member bound") and TC-043 result 5 (about `Duration::MAX`) are already correct. | spec/kani/functional/FR-028-bounded-proof-ceilings.md:55-59 |

## Dispositions (round 3)

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-007 | fixed 5ea8438 | The FR-028 per-member bullet now says "The outer bound below still applies, and does not elapse only when the product does not fit". FR-028-AC-12 and TC-039 step 10 and result 10 cover both cases, product fits and product does not fit. A sweep of spec/ for elapse statements finds them consistent: FR-017 l.124 and l.126, FR-017-AC-15, FR-017-AC-21, TC-043 result 5 (the `Duration::MAX` case), TC-039, FR-028 l.59, l.65 and AC-12, AD-004 l.1258, and interface-001. At 5ea8438, `make spec` exits 0 and `quire coverage --strict` is 83, identical to ce7e0f0. |
