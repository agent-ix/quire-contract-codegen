---
id: SR-010
title: "Numeric and state oracle Rust review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen#4 numeric/state slice at 3bc7e60; origin/main..3bc7e60"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-001
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/Task-004
    type: references
---

# SR-010: Numeric and state oracle Rust review

## Summary

The exact issue #4 delta was reviewed against the repository Rust conventions, public wire
compatibility, fail-closed lowering boundary, test quality, resource bounds, and local gates. Four
findings were repaired before this closing verdict; no open Rust finding remains.

## Verdict

**PASS** — all review findings are closed and the post-remediation stable all-target suite passes.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | medium | Closed: an older serialized diagnostic without `sourceSpan` now has an explicit decode control, preventing an optional additive wire field from becoming an accidental compatibility break. | src/oracle.rs:145, tests/oracle_generation.rs:1583 |
| FND-002 | low | Closed: numeric/refusal acceptance tags now use Quire-bound test annotations; the former semicolon form left the tokens unmatched and could not back the criterion rows. | tests/oracle_generation.rs:1299, tests/oracle_generation.rs:1523 |
| FND-003 | low | Closed: the shared Boolean-only dependency adapter now names both harness and Kani, so a numeric Kani refusal cannot be misreported as harness-only. | src/oracle.rs:732 |
| FND-004 | low | Closed: the source census excludes the mandated isolated Cargo output directory; otherwise compiled metadata was treated as authored wide-encoded source and failed the local gate. | tests/shared_assurance.rs:810 |

## Review Notes

- The supported grammar is exhaustive over Boolean literals/references/operators, bounded `i64`
  literals/references, and all six integer comparisons. Arithmetic, negation, obligations and every
  other expression kind return structured diagnostics before artifact construction.
- Parameter types come from QCIR typed nodes rather than names. Dependency and observation identity
  determine deterministic names; public bound generation preserves full ClauseRef identity.
- Non-Boolean roots, unsupported nodes/dependencies, and obligations carry their specified exact
  `SourceSpan`. The negative corpus proves deterministic first-locus selection and no approximation.
- The generated corpus compiles against the pinned runtime, denies warnings, and compares all six
  operators over endpoint and immediately-outside values with an independent evaluator.
- No workflow, dependency, unsafe surface, publish setting, or hosted-CI behavior changed.

## Gate Results

- `cargo fmt --all -- --check`: pass.
- `cargo clippy --locked --all-targets --target-dir target-codex-backends -- -D warnings`: pass.
- Stable `cargo test --locked --all-targets --target-dir target-codex-backends`: 68 passed, 0 failed.
- Rust 1.75.0 full suite before the closing documentation/trace remediation: 68 passed, 0 failed;
  affected post-remediation tests were repeated on the final implementation candidate.
- `cargo deny check`: advisories, bans, licenses and sources pass; only unmatched allow-list warnings.
- Rustdoc with denied warnings, release build, unsafe audit, upstream pin check and Quire validation:
  pass.
