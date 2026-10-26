---
id: SR-35021
title: "code-review and rust-review of IR-349 CG scalar consumer (quire-contract-codegen PR 568)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen@b4f398d7; Cargo.lock; src/core/profile.rs; src/oracle/scalar/render.rs; tests/exact_scalar_support/agreement.rs; tests/it/exact_scalar_generation.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: references
---

# Code review and Rust review of the IR-349 CG scalar consumer (PR 568)

## Summary

Ticket: IR-349. Draft PR 568, frozen head b4f398d7, two commits on origin/main dd59aa14. This is the paired consumer of quire-contract-runtime PR 160 (merged as 95a27602), which moved 11 runtime-owned scalar items from `quire_contract_runtime::exact` to `quire_contract_runtime::scalar` with no alias. The Rust-review lane is folded in here.

Net diff, 10 lines across 5 files:

- `Cargo.lock`: only the `quire-contract-runtime` source revision changes, a60c508f to 95a27602. That is RT `main` HEAD now (checked against GitHub and the local RT clone). Commit 1 also moved `quire-exact` and `quire-semantic-value`; commit 2 reverts both, so their net diff is zero. The lock still holds one entry each for `quire-exact`, `quire-semantic-value` and `quire-contract-runtime`. `--locked` builds pass.
- `src/core/profile.rs`: the unit test now calls `quire_contract_runtime::scalar::{evaluate_boolean_short_circuit, ShortCircuitConnective}`.
- `src/oracle/scalar/render.rs`: the emitted alias `use quire_contract_runtime::exact as residue;` becomes `use quire_contract_runtime::scalar as scalar_owner;`. The trigger (`functions.contains(...)`) and the emitted short-circuit call change with it, so they stay consistent. `scalar_owner` matches the existing `enum_owner` and `quantity_owner` owner aliases. It is a local `use ... as` name for the real owning module, not an alias, shim or copy of a moved item.
- `tests/exact_scalar_support/agreement.rs`: the direct-runtime side re-exports the two items from `scalar`.
- `tests/it/exact_scalar_generation.rs`: the TC-024 source assertion now expects `scalar_owner::evaluate_boolean_short_circuit(`. That is the real emitted text, so the assertion is no weaker than before.

Repo-wide grep (excluding `reviews/` and `target/`): no Rust source, test or emitted string still names any of the 11 moved items under `exact::`. The remaining `quire_contract_runtime::exact` uses (`src/oracle/function/mod.rs`, `tests/exact_function_support/agreement_cases.rs`, `tests/it/exact_function_agreement.rs`, equality doc comments) name items that are still in `exact` (`PackageDeclarations`, `Origin`, `Location`, `TypeEnvironment`, `EqualityOperator`, `RecursionEdges`). They compile against 95a27602. The two mentions of `IeeeItemRequirement`/`IntegerDivisionConsumer` in `src/oracle/function/mod.rs` are doc prose with no path. RT's `scalar` module is behind `#[cfg(feature = "exact")]`. CG's own manifest and the generated oracle manifests both enable `exact`, so the new path resolves. The Kani scalar runner includes the generated `lib.rs` by `#[path]`, so it inherits the new `use` line.

Rust-review checklist: no new arithmetic, indexing, `unwrap`/`expect`, unsafe or panic site. No new public API. No trait seam or async code. No `#[trace]` tag added, removed or changed.

Measured at head, with one target dir, `RUST_TEST_THREADS=4` and `QSPEC_DIR` set to a read-only export of quire-specification origin/main 17a304c0:

- `cargo test --locked --lib runtime_boolean_outcome_has_the_kernel_type`: passed.
- `cargo test --locked --test it` for `tc_024_boolean_and_integer_equality_nodes_generate_confirmed_runtime_oracles`, `tc_024_v2_boolean_and_equality_oracles_agree_with_direct_runtime` and `tc_024_generated_scalar_crate_agrees_with_direct_runtime`: passed. The V2 agreement test compiles and runs the generated crate against the direct runtime calls, which proves that the emitted `scalar_owner` import resolves and agrees.

The aggregate gate and Kani were not run (the lead runs them once on the final head).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Clean. The repointing in code, emitted source and tests is complete and minimal. No compatibility layer was added. The lock edit matches RT main, and the generated-source assertion checks real output. Spec-prose drift is recorded separately in the gap analysis (SR-35022).
