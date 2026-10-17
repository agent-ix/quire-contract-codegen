---
id: SR-11999
title: "Evidence review of IR-827 CBMC failure cause"
type: SpecReview
analysis: evidence
scope: "agent-ix/quire-contract-codegen@1ed68d0931d45988fb46b5d7a03c000f8c4657f8 (PR 409); spec/kani/functional/FR-017-kani-execution-evidence.md (AC-37 to AC-40), spec/kani/matrix/TC-027-kani-execution-evidence.md, spec/kani/matrix/TC-043-kani-batching-and-output-bounds.md, spec/kani/matrix/tests.md"
review_set: subset
---

## Summary

Ticket: IR-827. Evidence lens, with the integrity and EARS lenses folded in (only three SR ids were
available). Every new AC is `Test` with a named TC and a PLANNED/UNRUN status that matches the
tests.md rows. The real-lane case was re-measured on Kani 0.68.0 / CBMC 6.11.0, in a scratch crate,
under `systemd-run --scope -p MemoryMax=16G`.

Measured, using CG's argv order (`kani <options> -Z unstable-options --export-json <path>`,
execute.rs:959-965):

- `--cbmc-args --no-such-option` as the options gives exit 2, the error "The `--cbmc-args` option is
  unstable and requires `-Z unstable-options`", and no report.
- `-Z unstable-options --cbmc-args --no-such-option` as the options gives exit 1 and no report:
  `--cbmc-args` takes every argument after it, including `--export-json <path>`.
- `--cbmc-args` placed after `--export-json` (not CG's order) gives a report with `exit_code_64`.
- `[package.metadata.kani] unstable = { unstable-options = true }`,
  `flags = { cbmc-args = ["--no-such-option"] }` with CG's order gives exit 1, a report with
  `exit_code_64` and `Unknown option: --no-such-option` on stderr.

`QSPEC_DIR=/home/peter/dev/quire-specification make spec` exited 0. The one EARS warning in FR-017 is
at line 259, which this PR does not touch.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | AC-40's real `make kani` case cannot pass as written. CG appends `-Z unstable-options --export-json <path>` after the harness options (FR-017-AC-19; execute.rs:959-965). With `--cbmc-args --no-such-option` appended to the options, Kani exits 2 with a usage error and exports no report, so the run is `NoVerdict` (phase `launcher`), not `FailedWithoutCounterexample`. Adding `-Z unstable-options` first still exports no report, because `--cbmc-args` takes `--export-json <path>` as CBMC arguments. The PR body's `exit_code_64` measurement holds only when `--cbmc-args` follows `--export-json`, which CG never does. Use the crate's `[package.metadata.kani] flags = { cbmc-args = ["--no-such-option"] }` (the route src/core/profile.rs:18 already uses, re-measured to give `exit_code_64`), or use a slow harness with `--harness-timeout`, and update TC-027 to match. | spec/kani/functional/FR-017-kani-execution-evidence.md:620; spec/kani/matrix/TC-027-kani-execution-evidence.md:124-125, :235-237; src/kani/run/execute.rs:959-965 | wrong-requirement |
| FND-002 | medium | AC-37 defines the `exit_code` edges and the 64-byte cut but tests only one side of each limit. The rejection side is tested (`exit_code_2147483648`, a 100-byte value, a value split inside a character). The acceptance side is not: there is no `exit_code_2147483647`, no `exit_code_-2147483648`, and no value of exactly 64 or 65 bytes. An implementation that parses into `i16`, or that keeps values only below 64 bytes (`< 64` instead of `<= 64`), passes every case listed. Add the i32 MAX and MIN acceptance cases and the 64/65-byte ASCII pair to AC-37 and TC-027. | spec/kani/functional/FR-017-kani-execution-evidence.md:533-539, :617; spec/kani/matrix/TC-027-kani-execution-evidence.md:179-182 | correct-requirement-no-evidence |

## Scope examined

Evidence: AC-37 to AC-40 verification column, TC-027 procedure (173-192), expected results (204-237)
and implementation note (369-372), TC-043 step 12, its expected result and implementation note,
tests.md rows 41 and 46-47 and 104/106. Integrity: AC ids contiguous (37-40), each AC owned by exactly
one TC and that TC's matrix row, interface-001 consistent with the section, Outputs bullet added.
EARS: the eight new `shall` statements (522-564) are ubiquitous or event (`When ...`) form with the
generator as subject. No grammar warning was raised on them. Falsifiability: AC-37, AC-38 and AC-39
state concrete inputs and wire values that an implementation can fail. AC-40's stand-in half is
falsifiable. Its real-lane half is FND-001. The AC-19 matrix correction was checked and is accurate
(see SR-11997).

## Verdict

Needs changes: one high (the real-lane AC is infeasible as written) and one medium (untested
acceptance boundaries).
