---
id: SR-586009
title: "IR-348 CG #592 Rust code review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; PR #592 src/core/ir/frame.rs and src/replay/composite_builder.rs; AD-004 L-8, FR-033-AC-1"
review_set: subset
---
# IR-348 CG #592 Rust code review

## Summary

Ticket: IR-348. Independent Opus review of CG #592 at `d44a9420690969e743f04fa8099991c54a484174` against `origin/main`. The moved lookup returns the same `Option<&str>` and preserves Eq, Ne, mismatch and unsupported dispositions. The helper already existed in `core/ir/frame.rs`; no duplicate or compatibility layer was added. No applicable AssuranceProfile was found under `spec/`. Focused tests and Clippy were not run by this reviewer during concurrent final gating; `git diff --check` and `cargo fmt --all -- --check` passed.

## Scope

- Examined AD-004 L-8: - L-8. `serde_json::Value` and `.get("` appear for body terms only in `core/ir`. Test: a grep gate.
- Examined FR-033-AC-1's operator binding and typed mismatch refusal.
- Read FR-014-AC-2 for the unsupported quantity conversion context.
- Inspected the entire two-file diff, `selected_claim`, `node_equality_operator`, the existing tagged composite request tests and the new source guard.

## Verdict

CONDITIONAL: the behavior-preserving move is sound, with two low maintainability and test-quality findings.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The move deletes the only in-code IR-803 deferral marker while the separate raw operation lookup remains. | src/replay/composite_builder.rs:310 |
| FND-002 | low | The positive source assertion ties the L-8 guard to one expression spelling and can fail on an equivalent Rust rewrite. | src/core/ir/frame.rs:147 |

## Coverage

AD-004 L-8 remains open across the repository. The existing FR-033-AC-1 tagged tests cover operator mismatch and repeated request identity; FR-014-AC-2 tags the unsupported operation control. No new runtime test is required for this pure lookup relocation. No focused build was run by this reviewer while the host was under concurrent gate load. The prior IR-635 review's FND-001 explicitly deferred a typed operator on `CheckedCompositeOperands` to IR-803; this PR does not deliver it.
