---
id: SR-586007
title: "IR-348 CG591 Rust code review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; PR #591 head 5ab576da; src/core/ir/frame.rs, src/oracle/function/mod.rs; AD-004 L-8"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/AD-004", type: references }
---

## Summary

Ticket: IR-348. Reviewed the five changed function operation-identity reads and the new source guard at PR #591 head `5ab576da`. The accessors preserve the old `Option<&str>` behavior and refusal paths. The guard misses an alternate raw JSON identity read.

## Verdict

**CONDITIONAL** — one medium test-intent finding needs a fix. Full gates were not run in this review; the coder's focused evidence is pending a reported guard result.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The new function source guard accepts a raw `op.get("identity")` read, so it cannot enforce the scoped AD-004 L-8 body-access boundary. | src/core/ir/frame.rs:133 |

## Coverage

Examined AD-004 L-8 and step 3; both changed files; the existing `core/ir/checked_term::application` and `core/ir/frame::application_identity` helpers; the five function call sites; and FR-021 schedule/refusal behavior as context. The five identity replacements preserve `None` on non-application or missing identity and preserve the existing typed refusals. No new vendoring, compatibility layer, digest, public API, or CI workflow change was found. An optional simplification would move the general identity accessor out of `frame.rs` and reuse it inside `checked_term::application`; it is not necessary for correctness in this PR.

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | At `5a6ecec4`, the production-source guard checks `.get("` for every receiver, and the appended `op.get("identity")` control must be detected by the same scanner. The exact cited bypass now fails the guard. |
