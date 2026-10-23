---
id: SR-15713
title: "IR-974 CG PR 543 bounded launcher gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen@c73f8ffe6af69ec52148a62e1d0f3c7b308e34c3; FR-045-AC-3, TC-059, spec/routed/matrix/tests.md, src/process.rs, src/kani/run/launch.rs"
review_set: subset
---

## Summary

Ticket: IR-974. Compared the new process boundary and its eleven tagged tests with FR-045-AC-3 and TC-059. This PR is an extraction slice; the query and attempt identities, solver response parser and HP-2 outcome map remain explicitly planned. Plan completion: not assessed.

## Reviewed units

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-045-AC-3 | examined | CG invokes the selected solver executable directly, one process per QSL query, with exact query bytes on stdin and finite per-query input-byte, wall-clock, memory and captured-output limits. |
| TC-059 | examined | Run QSL-encoded product queries through the selected solver and check exact input, resource stops and typed run evidence. |
| spec/routed/matrix/tests.md | examined | FR-045-AC-1, FR-045-AC-2, FR-045-AC-3 are planned for IR-974; CG solver-run producer absent. |
| src/process.rs tests | examined | Exact stdin, input excess, missing tool, relative path, output excess, cancellation, nonzero exit, unread input, wall time, partial capture and memory ceiling. |
| src/kani/run/launch.rs | examined | Shared launcher extraction and existing Kani test paths. |

## Verdict

**CONDITIONAL** — the new tests cover the ordinary wall-clock limit but omit a legal extreme value that removes the limit (FND-001). The remaining HP-2 record and outcome work is explicitly planned and is not credited to this extraction PR.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-045-AC-3's finite wall-clock boundary is only tested with a one-second limit. No tagged control passes a `Duration` whose addition to `Instant::now()` overflows; `Duration::MAX` is accepted by the public request and produces no deadline. Add a focused refusal or stop control for this boundary. | src/process.rs:23; src/process.rs:470; src/kani/run/launch.rs:453 |

## Coverage

Plan completion: not assessed. Diff-scoped manual matrix and reverse-gap check; `quire matrix --scope . --format json` did not complete during this read-only review, so no full-repository matrix claim is made. The spec's TC-059 and matrix rows still mark the full IR-974 contract planned. No test or aggregate gate was run.

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | At 1995bccf, new `#[trace("FR-045-AC-3")]` test `overflowing_wall_clock_limit_refuses_before_spawn` passes `Duration::MAX`, requires the typed selection error, and checks that `/usr/bin/touch` did not create its marker. Focused run passed 1/1; the old code would return a completed child and fail this assertion. |
