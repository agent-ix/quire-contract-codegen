---
id: SR-15402
title: "Code review of IR-850 Runtime lock and root cohort in CG PR 513"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen PR #513; Cargo.lock Runtime a60 hunk at 9b2c69f22f8840930fde7a7a29abe1ff1859eab5 and root first-party cohort"
review_set: subset
---

# SR-15402: IR-850 lock and root cohort review

## Summary

Ticket: IR-850. Reviewed PR #513 exact head `9b2c69f22f8840930fde7a7a29abe1ff1859eab5` against pre-lock head `a3893118f8563518482a2f418a5c26fe157a4c7a` and base main `724d6bfba77154dde44623655e57d3d7c92d8301`. This review is limited to the Runtime lock hunk and supported root cohort. The source additions in `src/core/profile.rs`, `src/lib.rs`, and `tests/it/v2_llvm_coverage.rs` are already in the independent source review lane; they were inspected for the lock interface and are not re-reviewed here. No workflow, gate script, or deny policy changes appear in the PR diff.

## Scope

- `Cargo.lock`: Runtime `3cc88c480d4efe5179fe91acf4634def5d2d82fd` changes to `a60c508f3a2af230aeb51af97bdf549cd5fffa01`, and Runtime's dependency list gains `quire-semantic-value`. The diff contains no other lock change.
- Root cohort: exactly one Runtime a60, one `quire-exact` 72b25421, and one QSV 1a38c06f revision; FCD 3/3 ac62202a, quire-rs 2/2 fd1cdd9d, QSL 8/8 485ccaa3. The package counts are separate from the count of Git revisions.
- `src/core/profile.rs`, `src/lib.rs`, `tests/it/v2_llvm_coverage.rs`: inspected the adjacent three-file integration delta to confirm the new QSV manifest constant is exported and the LLVM native test imports the kernel type owner. No compatibility layer or copied owner implementation was added in this delta.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - | correct-requirement-no-evidence |

## Verdict

**PASS for the Runtime lock hunk and supported root cohort.** The committed `Cargo.lock` SHA-256 is `43dd9ef84f43ea430aa0e3408e5a77063a732adda9a3d294b903642b7a0f9582`, byte-identical to the earlier independently measured combined lock. On this exact PR head, `git diff --check`, `cargo +1.98.1 metadata --locked --offline --no-deps`, the built IR-736 `quire check-lock Cargo.lock`, `scripts/check_one_copy.awk`, `cargo deny check`, and `cargo +1.98.1 fmt --all -- --check` all exited zero. `cargo deny` reported only its existing warnings. A prior local combined-head focused test generated admitted quantity multiply, comparison, and exact conversion claims and compiled the emitted scratch crate against this same lock; it did not establish full behavioral parity.

I downloaded the six raw focused-control transcripts privately attached to IR-850 comment 117a283f and compared them with its `final-summary.json` for this exact head and root lock SHA-256. Each transcript names the expected test and ends with `1 passed; 0 failed`; the summary records exit zero for all six commands. This independently certifies the bounded-parameter, claim-descriptor, function-native, clause-native, quantity-route, and LLVM-native focused controls. The final combined-head `make ci` and real Kani gate remain with the author after review custody. The test-generated scratch lock is a separate observed cohort tracked by IR-962; this verdict applies to the committed root lock only.
