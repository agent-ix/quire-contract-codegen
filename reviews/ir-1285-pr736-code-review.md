---
id: SR-2026110283849
title: "IR-1285 PR 736 native clause test build review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; tests/it/kani_obligations_state_frame.rs; IR-1285 PR #736"
review_set: subset
---

## Summary

Ticket: IR-1285; PR #736 at `ee52766fa529af2cb22e0309b03fa57d159b1cfb`. Reviewed the four changed native clause test commands, the existing shared target and mutex helper, and the unchanged generated assertions. No code or Rust defect was found in this diff.

## Scope

- examined: `tests/it/kani_obligations_state_frame.rs:137-145`, `:294-302`, `:385-393`, `:2000-2008`, the four changed child Cargo commands and their surrounding assertions.
- context_only: `tests/it/scratch_crate.rs:20-34`, the existing shared target and process-local mutex; FR-014-AC-38 and IR-1285 AC1-AC5.
- No applicable AssuranceProfile was found under `spec/`.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Evidence

The child package names are distinct. Each command retains `cargo test --offline --quiet`, `RUSTFLAGS=-Dwarnings`, its exact generated source, and its success assertion. The new guard spans child compilation and execution, then drops before the outer assertion. Existing other callers of `agreement_target()` either hold the same agreement mutex through child execution or perform a build-only Cargo command. Cargo's target lock protects build-only compilation. The diff adds no source copy, compatibility path, public API, unsafe code, dependency pin, CI workflow change, or gate narrowing.

`cargo fmt --all -- --check` and `git diff --check origin/main...HEAD` pass. The four affected focused tests passed on the candidate, with 7.82, 0.20, 0.21, and 1.01 second test-binary durations. The reviewer did not rerun Cargo or the full aggregate; the code lead owns the once-per-final-head gate. No real Kani/CBMC proof was run in this review.

## Verdict

**PASS for this scoped code and Rust review.** The final aggregate gate and whole-suite runtime measurement remain pending; this verdict does not accept IR-1285 AC3 or close the ticket.
