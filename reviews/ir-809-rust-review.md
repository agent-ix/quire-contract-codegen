---
id: SR-11906
title: "Rust review of IR-809 NoVerdict diagnostics (PR 406)"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; src/kani/output/diagnostics.rs, src/kani/run/execute.rs, src/kani/run/launch.rs (diff origin/peter/ir-809-run-diagnostics-spec...3c4386a)"
review_set: subset
---

## Summary

Ticket: IR-809. Rust lane of the code review. Checks: idioms, panics and slicing in
`diagnostics.rs` (every index is guarded: `bytes[least - 1]` runs with `least > 0`, every byte
slice follows a successful `get`, and every `str` slice falls on an ASCII position), integer
conversions (`u64::try_from(start).unwrap_or(u64::MAX)`), no `unsafe`, serialize-only derives (the
`compile_fail` doctest and its control both pass), and the `Trace:` tags on every new test.
`cargo clippy --all-targets -- -D warnings` is clean, 27 focused lib tests pass, and both
doctests pass. Judged the coder's ambiguities (2), (3), (4) and (5).

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | low | The parallel rule matches only `-j`, `--jobs` and `--jobs=`, so the attached forms `-j2` and `-j=2`, which clap accepts for Kani's short `-j`, do not count as parallel. On Kani 0.68 this cannot over-claim: Kani refuses `--jobs` without `--output-format=terse` (kprobe e4.txt), and terse output prefixes the heading `Thread 0: Checking harness ...`, which starts no segment (o5.txt). A later Kani that prints plain headings under `-j2` would give a member a `solver` phase taken from interleaved output. The spec rule and `runs_in_parallel` should also cover `-j<digits>` and `-j=`. | src/kani/output/diagnostics.rs:150-154; spec/kani/functional/FR-017-kani-execution-evidence.md:410-412 | wrong-requirement |
| FND-002 | low | The spec says only "an RFC 3339 date-time", and the recognizer's choices are unstated, so two implementers would redact differently. It accepts `2026-02-31T...Z` and offset hour `+99:00`. It requires no left boundary, so `x2026-10-16T22:27:04Z` becomes `x<TIMESTAMP>`. It accepts lowercase `t`/`z`, and it rejects the space separator that RFC 3339 section 5.6's NOTE allows. The record stays deterministic, so no stability property breaks. The spec should state the grammar: the section 5.6 ABNF with T/t only, no calendar check, no boundary rule. | src/kani/output/diagnostics.rs:385-427, :458-465; spec/kani/functional/FR-017-kani-execution-evidence.md:452 | wrong-requirement |

## Verdict

Low findings only. Ambiguity (4): a `Solving with ` line before any segment heading is ignored.
That is what rule 2 requires (the line must sit in the member's own segment), and
`tc_027_a_solver_line_outside_the_members_segment_does_not_make_it_solver` pins it, so no change.
Ambiguity (5): ANSI is not stripped before markers are matched. The spec reads markers from the
captured streams, as lines beginning with literal text, and strips ANSI only for excerpts. A
colored Cargo `error: could not compile` line (CARGO_TERM_COLOR=always) therefore falls through
to `unknown`, an under-claim the spec allows. Kani 0.68's own piped output is uncolored, so no
change. Ambiguities (2) and (3) are FND-001 and FND-002.
