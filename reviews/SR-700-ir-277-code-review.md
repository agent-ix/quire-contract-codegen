---
id: "SR-700"
title: "CG PR 210 code review (Rust lane): Kani exported-report verdicts and per-check view"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@363148d4e18e30c278b84c106a384503ed0ed47c; src/kani_transcript.rs, src/kani_execution.rs, src/lib.rs, tests/it/bounded_kani_corpus.rs, tests/it/kani_obligations.rs, tests/fixtures/kani-report/"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: references
  - target: ix://agent-ix/quire-contract-codegen/AD-003
    type: references
---

# SR-700: CG PR 210 code review (Rust lane)

## Summary

Ticket: IR-277 (also IR-288, IR-463; IR-465 deliberately not included). PR:
agent-ix/quire-contract-codegen#210 at 363148d, merge base fda2316 (main has since gained only
b2aa4e9, a spec-only AD-003 edit; `git merge-tree` against current main is clean). Methods:
code-review with the rust-review lane folded in (idioms, panic surface, error types, wire
boundaries, test intent), plus temporary probe tests and mutation runs in an uncommitted scratch
copy (deleted afterwards).

Checked and found as claimed:

- AD-003 (c) wire contract: `KaniCheckResult { id, class, location { file, line }, status }` is
  serialize-only (no `Deserialize` derive); Kani's spelling (`category`, line as a string,
  `unknown`) is read only by the private `RawCheck`/`RawLocation` and converted with `TryFrom`.
  The old `category`-in/`class`-out round-trip mismatch is gone.
  `tc_027_the_per_check_view_has_one_serialized_wire_shape` pins the serialized form.
- Report parse against the six verbatim 0.68-era captures in `tests/fixtures/kani-report/`
  (unknown members such as `property_details`, `error_details`, `cbmc`, `function`,
  `description`, `column` are ignored; `Undetermined`, `Unreachable`, `Satisfied`,
  `Unsatisfiable` and unwind `Failure` all present). The `make kani` lane exercises the real
  installed Kani's `--export-json` output at this head.
- Probes (temporary, never committed): missing `id`, negative or string `id`, a line of
  `4294967296`, an empty line, a numeric line, a null `metadata.version`, `checks: null`,
  trailing garbage, an empty body, invalid UTF-8 and deep nesting are each
  `KaniReportRefusal::Malformed`; none panics. Zero or two harness results are `HarnessCount`.
- SUCCESS-check count rule (`kani_execution.rs:736-744`) and the precondition exemption
  (`:775-787`): a satisfied precondition cover counts as one success; a precondition harness with
  an unsatisfiable or unreachable cover classifies `CoverUnsatisfied` with zero successes, never
  `Verified`; non-precondition kinds still route zero property successes through
  `KaniOutcome::proved_from_checks` to `VacuousProof`. Three mutations (drop the precondition
  addition; add covers for every kind; drop the exemption) are each killed by the unit tests.
- IR-277 single parse: the verdict comes from the report only; the console text is scanned once,
  by `counterexample_playback`, and only after the report names a failed property. No banner is
  read anywhere in `src/` (`tc_027_the_console_banner_never_decides_the_verdict`).
- No `unwrap`/`expect` in non-test paths; integer conversions saturate (`u32::try_from(..)
  .unwrap_or(u32::MAX)`); the report read is bounded (16 MiB, refused not truncated); errors are
  typed (`KaniReportRefusal`, wrapped as `KaniExecutionRefusal::Report`) with `Display` and
  `Error`. No pins, SHAs or version records added. The strip is clean: no `terminal_value`,
  `ir_outcome_terminal_value`, `proof_category` or `kani_terminal` remains in `src/` or `tests/`.
- `metadata.version == "1.0"` (`kani_transcript.rs:28,299`): judged load-bearing format
  identity, not a tool pin. It is Kani's report-schema discriminator, not the Kani release; what
  breaks without it is a schema change that keeps the five members this module reads but changes
  their meaning, which would be read silently. Its failure mode is a loud typed refusal. Keep it.
- Fixtures carry Kani/CBMC/rustc version strings as captured data; nothing asserts them and the
  paths are already scrubbed to `<CAPTURE_DIR>`. No scrub needed.
- Gates at this head (run by this reviewer): `make ci` exit 0 (108 unit, 245 integration, 9
  ignored, `make spec` with the 3 baseline warnings).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | `classify_success` never looks at non-success checks. A report whose harness `status` is `Success` but which lists a check with status `Failure`, `Error`, `Undetermined` or an unwind `Failure` classifies `Verified` (reproduced with probe reports: each gave `Ok((Verified, 1))`; a precondition harness with a satisfied cover plus a failed assertion is also `Verified`). The classifier trusts Kani's harness status over the per-check data it now has in hand. Kani derives harness status from `Failure` checks only, so an `Error` or `Undetermined` property without a `Failure` beside it is a real way to reach this. Fix: refuse a report whose harness status contradicts its checks (`Malformed`/a new cause), or never return `Verified` while any check is `Failure`, `Error`, `Undetermined` or `Unknown`; add a test | src/kani_execution.rs:752-797, src/kani_transcript.rs:340-346 |
| FND-002 | medium | The report path is one fixed name per target directory (`REPORT_FILE`). Two runs that share a `target_directory` concurrently race: each `remove_stale_report` can delete the other's report, and one run can read the other's report as its verdict. Reading stdout was per-process and had no such hazard. `KaniExecutionRequest::target_directory` documents no exclusivity. Fix: make the file name unique per run (for example the harness symbol plus the process id, or a temp file in the target dir), or state in the field doc and FR-017 that a target directory serves one run at a time | src/kani_execution.rs:243-244, src/kani_execution.rs:804-823 |
| FND-003 | low | The `success_checks` doc says "It is the SUCCESS-check count FR-029 carries into the terminal value." The terminal map was stripped from this PR and FR-029 is Planned, so the doc describes a consumer that does not exist; FR-017 now owns the rule | src/kani_execution.rs:373-378 |
| FND-004 | low | `KaniCheckClass::Other(String)` is a public variant that can hold `"cover"` or `"unwind"`: such a value serializes identically to `Cover`/`Unwind` but compares unequal and is not counted as a cover. Only the parser should build it; make the payload private (a newtype with a constructor that canonicalises) or document that `Other` never holds those two words | src/kani_transcript.rs:139-168 |
| FND-005 | low | `tests/it/kani_witness_join.rs:5` still says the lane "needs the real installed Kani 0.67.0 backend". Unchanged on main, but stale against the lane this PR moves onto the exported report, and the repo CLAUDE.md says to remove version records when found | tests/it/kani_witness_join.rs:5 |

## Verdict

Not mergeable as is: FND-001 lets a report listing a failed or errored property classify as
`Verified`, which is the one outcome FR-017 says is never defaulted; the fix is a few lines plus a
test. FND-002 should be fixed or the exclusivity documented in the same round. FND-003 to FND-005
are one-line cleanups. The wire shape, the refusal surface, the precondition rule and the strip
are sound.
