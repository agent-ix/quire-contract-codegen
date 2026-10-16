---
id: SR-11905
title: "Code review of IR-809 NoVerdict diagnostics (PR 406)"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; src/kani/output/diagnostics.rs, src/kani/output/mod.rs, src/kani/run/execute.rs, src/kani/run/launch.rs, src/kani/run/scalar.rs, src/lib.rs, tests/it/*.rs (diff origin/peter/ir-809-run-diagnostics-spec...3c4386a)"
review_set: subset
---

## Summary

Ticket: IR-809. PR quire-contract-codegen#406 at 3c4386a, stacked on spec PR #403 (de9648b).
Reviewed the code diff only. Checked the spec boundary (Kani output is read only in
`src/kani/output/`: the phase markers, the `-j` rule and the excerpts are all in
`output/diagnostics.rs`, and `execute.rs` only routes streams), that classification text is
byte-unchanged (`launch.rs` builds the merged `text` exactly as before and only then moves the raw
bytes into `CapturedStreams`), that the over-limit and unread refusals are unchanged (no diff
touches those paths; the tests assert the literal codes), the public API additions (three types
re-exported from `lib.rs` only, per AD-004 L-2; `tests/it/layout.rs` MODULE_FILES gains
`kani/output/diagnostics.rs`), and all 28 `KaniExecutionRequest` literal sites (every one sets
`home_directory` or spreads a request that does; `cargo clippy --all-targets -D warnings` is
clean). Searched the sibling repos read-only for constructions that a CG pin bump would break.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | A batch group recomputes both excerpts and the stdout segment scan once per member, although the spec makes every member's excerpts identical. Each `diagnostics_of` call decodes, strips, redacts (four full-text passes, each allocating a full copy) and tails both streams, which may hold up to N x 8 MiB each. The work is O(N^2) in group size and runs after the process ends, outside the deadline. Measured with the PR's excerpt code under rustc -O: 170 MiB/s (8 MiB in 49 ms, 100 MiB in 583 ms). A 50-member NoVerdict group whose streams are near their limit costs about 50 x 800 MiB, or 4 minutes of CPU, after the launcher has exited; 10 members cost about 10 s. Fix: compute the two excerpts and the segment table once per group and clone the record per member. | src/kani/run/execute.rs:810-818; src/kani/output/diagnostics.rs:103-118 | implementation-bug-despite-evidence |
| FND-002 | medium | Downstream routing. `KaniExecutionRequest` gains the public field `home_directory` and `KaniExecutionEvidence` gains `diagnostics`; neither struct is `#[non_exhaustive]`. quire-driver builds both as struct literals: src/provider.rs:308 (the production `execute_kani_obligation` call) and src/provider.rs:395 (a test `KaniExecutionEvidence` literal). It pins CG `branch = "main"` (lock f1c8fa9), so the first `cargo update` after #406 merges fails with E0063 (missing field). quire-integration tests/qsl_kani_exemplar.rs:246 already fails to compile against current CG (it passes `timeout` and no `cancel`); #406 adds a third missing field. These need a quire-driver follow-up ticket, landed with or before its next CG bump. | src/kani/run/execute.rs:68-71, src/kani/run/execute.rs:290-293; quire-driver/src/provider.rs:308, :395; quire-integration/tests/qsl_kani_exemplar.rs:246 | missing-requirement |
| FND-003 | low | `CapturedStreams` keeps the raw stdout and stderr bytes beside the lossy merged `text` for every completed run, not only `NoVerdict` ones, so the parent holds two copies of the captured output until evidence is built. At the cap this is 2 x 8 MiB x N per stream for a group: 50 members add about 800 MiB of resident memory over the previous peak. Fix: keep the raw bytes only until the outcome is known, or build `text` lazily from them. | src/kani/run/launch.rs:386-395 | implementation-bug-despite-evidence |
| FND-004 | low | AD-004's target tree lists `output/` as `report.rs` and `playback.rs` only. The PR adds `output/diagnostics.rs`, and `tests/it/layout.rs` MODULE_FILES now names it, but the architecture record does not, so AD-004 and the layout test have drifted apart. | spec/assurance/AD-004-cg-crate-layout.md:240-244; tests/it/layout.rs:75 | correct-requirement-no-evidence |

## Verdict

Two medium findings and two low. The diagnostics are correct on the main path. The spec boundary
holds, classification text and the refusals are unchanged, the public API additions follow AD-004
L-2, and every construction site compiles. FND-001 must be fixed before merge. FND-002 is a
cross-repo routing item and does not block the CG merge, but it needs a quire-driver ticket.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1 (818d8c6): d5c6bd6: ProcessOutput::analyse reads both streams once; finish_group memoises it in a OnceCell shared by all members, finish_single calls it once. |
| FND-002 | deferred | Routed downstream by the team leader (IR-809 comment 8ac125de): quire-driver src/provider.rs:308/:395 literals handed via the Kani planner to the quire-driver owner; #[non_exhaustive] rejected because it would also break CG integration-test literals. |
| FND-003 | fixed | Round 1 (818d8c6): d5c6bd6: LaunchOutcome::Completed keeps only CapturedStreams; the merged text is rebuilt transiently by merged_text() at its three readers (launch_evidence, finish_group playback, scalar.rs), byte-identical format. |
| FND-004 | fixed | Round 1 (818d8c6): d5c6bd6: AD-004 output/ tree names diagnostics.rs. |
