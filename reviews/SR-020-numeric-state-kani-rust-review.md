---
id: SR-020
title: "Numeric and state Kani Rust review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen#2 numeric/state slice at b614cc9; origin/main..b614cc9"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-003
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/Task-010
    type: references
---

# SR-020: Numeric and state Kani Rust review

## Summary

The issue #2 delta was reviewed against the repository's Rust conventions, typed ABI and wire
contracts, fail-closed lowering boundary, source-span preservation, test quality, resource bounds,
and local gates. Three findings were repaired during the PR-time pass; no open Rust finding remains.

## Verdict

**PASS** — every Rust-review finding is closed and the stable plus exact Rust 1.75 local suites pass
on the committed candidate.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | medium | Closed: the additive `KaniDiagnostic.sourceSpan` wire field now defaults when absent and has an explicit legacy decode control, while mapped clause failures preserve the exact IR-owned span. | src/kani.rs:252, src/kani.rs:1133, tests/kani_generation.rs:1427 |
| FND-002 | medium | Closed: multi-ID slash comments were split into Quire-bound annotations and the implemented FR-003 rows were promoted; coverage now reports FR-003 8/8, matrix tests 13/13, zero status lies, and zero unmatched tags. | tests/kani_generation.rs:455, tests/kani_generation.rs:1123, spec/test-matrix.md:21 |
| FND-003 | low | Closed: the Kani-specific generated-source ceiling now has an adverse control that requires `ResourceLimitExceeded`, `unsupported`, and the stable `generated.rust` path without a partial bundle. | tests/kani_generation.rs:1439, FR-003-AC-3 |

## Review Notes

- `generate_kani_bundle` reuses the executable-oracle analyzer and exact rendered predicates; it does
  not introduce a second expression interpreter or approximate an unsupported clause.
- Full `DependencyIdentity` ordering drives deterministic argument/result positions. Repeated
  occurrences unify, while type/domain conflicts, generated-name collisions, post-state
  preconditions, unsupported observations, and malformed dependency identities refuse explicitly.
- Boolean and bounded `i64` bindings are exhaustive. Integer assumptions and result guarantees use
  the checked IR minimum and maximum; no caller range or strategy range participates.
- Generated result shapes cover `()`, one primitive, and ordered tuples. Actual cargo-kani execution
  proves both the legacy Boolean ABI and a mixed Boolean/integer multi-result ABI.
- Public wire structures deny unknown fields; the additive diagnostic field is backward compatible;
  v1 schemas remain historical and v2 outputs are separately identified and schema validated.
- No production panic, unsafe block, unchecked integer conversion, unbounded recursion, new
  dependency, hosted workflow change, or `publish` change was introduced. Codegen #3 files were not
  edited.

## Gate Results

- `cargo fmt --all -- --check`: pass.
- `cargo clippy --locked --all-targets -- -D warnings` with
  `CARGO_TARGET_DIR=target-codex-backends`: pass.
- Stable `cargo test --locked` with prescribed assurance inputs: 73 passed, 0 failed.
- Exact Rust 1.75.0 `cargo +1.75.0 test --locked`: 73 passed, 0 failed.
- SUITE-008 actual backend: cargo-kani 0.67.0; executable SHA-256
  `7f143a251d11c7e6e232bbf2cbccf56f9ce66a5f0107eeb3008698e6715f55d9`; healthy mixed and
  ConfigVersion-style identity proofs pass; changed-state and strict-comparison subjects print
  concrete counterexamples.
- Generated option identity: `-Z function-contracts`, optional `-Z stubbing`,
  `-Z concrete-playback`, exact harness, unwind, `cadical`, regular output, and printed playback.
- `cargo deny check`: advisories, bans, licenses, and sources pass; only the existing unmatched
  allow-list warnings remain.
- Unsafe audit, warning-denied rustdoc, upstream pin check, ten-row generation conformance, release
  build, shared pin classification, and the full assurance chain: pass.
- The aggregate `make spec` remains independently red only on the pre-existing installed-catalog
  `Status` versus `Coverage Status` TestMatrix column conflict; this review does not report it green.
