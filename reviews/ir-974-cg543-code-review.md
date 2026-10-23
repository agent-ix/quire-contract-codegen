---
id: SR-15712
title: "IR-974 CG PR 543 bounded launcher code and Rust review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen@c73f8ffe6af69ec52148a62e1d0f3c7b308e34c3; Cargo.toml, src/kani/run/launch.rs, src/kani/run/namespace.rs, src/lib.rs, src/process.rs; FR-045-AC-3"
review_set: subset
---

## Summary

Ticket: IR-974. Reviewed the exact PR diff at c73f8ffe. The new process entry point uses the existing owned namespace monitor, direct executable invocation, bounded stdin and per-stream capture. The Kani path retains its prior capture ceiling and outcome mapping. No copied launcher or shell/prlimit shim was introduced.

## Reviewed units

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-045-AC-3 | examined | CG invokes the selected solver executable directly, one process per QSL query, with exact query bytes on stdin and finite per-query input-byte, wall-clock, memory and captured-output limits. |
| src/process.rs | examined | Public process request, typed run/outcome/error, input/path checks and process tests. |
| src/kani/run/launch.rs | examined | Shared launch extraction, stdin writer, capture, deadline, memory observation and Kani wrappers. |
| src/kani/run/namespace.rs | examined | Existing owned namespace, one argument visibility change. |
| Cargo.toml and src/lib.rs | examined | rustix fs feature and public process module. |

## Verdict

**FAIL** — a legal public wall-clock value can disable the deadline and leave a child without a finite time bound.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Reject or otherwise bound an overflowing wall-clock value: `Instant::checked_add(Duration::MAX)` returns `None`, then `wait_until_at` treats the absent deadline as no timeout. A caller can run an endless selected child under a supposedly finite `ProcessLimits`; the writer and capture loops do not supply a time ceiling. | src/kani/run/launch.rs:453; src/kani/run/launch.rs:796; src/process.rs:23 |

## Coverage

Diff-scoped review of code and Rust idioms. CI workflows are unchanged. Existing Kani launcher tests were inspected for changed calls; no test or limit constant changed in this PR. No aggregate gates were run in this reviewer pass. A local Rust check confirmed `Instant::now().checked_add(Duration::MAX).is_none()` is true on this host.

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | At 1995bccf, `run_bounded_process` computes `Instant::now().checked_add(limits.wall_clock)` before calling the launcher and returns `ProcessError::Io { phase: Selection, source: InvalidInput }` when it cannot be represented. The shared launcher receives the already validated `Instant`; no child is spawned on overflow. The tagged `Duration::MAX` control passed 1/1. |
