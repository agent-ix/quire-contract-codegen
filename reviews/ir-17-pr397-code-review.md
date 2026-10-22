---
id: SR-11690
title: "IR-17 PR 397 code-review review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen@01c8b4f8e1f5c0066ef61ee3f0ea005777c71dfe (base 2897d5eb9cf30e294e3cb8c24b3a29f5d5c37f3b); git diff origin/main...01c8b4f, 35 files: Cargo.lock; src/core/profile.rs, src/kani/generate/frame.rs, src/kani/generate/scalar.rs, src/kani/run/scalar.rs, src/lib.rs, src/oracle/boolean_v1.rs, src/oracle/equality/mod.rs, src/oracle/function/mod.rs, src/oracle/operator_catalog.rs, src/oracle/scalar/mod.rs, src/replay/composite_builder.rs, src/replay/frame.rs, src/replay/function.rs; 17 test files under tests/; 4 spec files (read as context only)"
review_set: subset
---

## Summary

Ticket IR-17, PR quire-contract-codegen#397. This is a code review with the rust-review lane folded in. It covers `git diff origin/main...01c8b4f`: 35 files, +678/-517. The files split into 1 lockfile, 4 spec files, 13 source files and 17 test files.

The ticket's acceptance criteria are untrusted text, so each was measured again:

- CG takes the exact-kernel types from `quire-exact` and keeps no copy of the kernel.
- The only QSL crate CG depends on is `qsl-replay`.
- The change adds no compatibility layer.

Measured against the locked upstream sources (cargo git checkouts):

- `quire-exact` 37aa172
- `quire-contract-runtime` 288270a
- `qsl-replay` 20b3254 (QSL)

## Verdict

**FAIL**: one high finding, one medium and one low.

The library migration is sound:

- **One kernel copy.** `Cargo.lock` holds one `quire-exact` entry (37aa172). `cargo tree -d` lists no duplicated first-party crate. `scripts/check_one_copy.awk` passes and `cargo deny check` passes.
- **QSL dependency.** CG's only direct QSL dependency is `qsl-replay` 20b3254.
- **No kernel copy or compatibility layer.** CG defines no kernel type: no `Meter`, `Outcome`, `Refusal`, `Integer` or `ChargePoint`. It has no `pub use` of `quire_exact` or of the runtime's `exact` module, and no shim or alias module.
- **Ownership is enforced by the compiler.** RT 288270a deleted its kernel re-exports; `src/exact/mod.rs` carries a `compile_fail` doctest for `quire_contract_runtime::exact::Integer`. Any kernel name CG still spelled through `quire_contract_runtime::exact` would therefore fail to build.
- **Emitted source strings.** Every `rt::`/`residue::`/`exact::` name in the emitted source of the scalar, equality, function and boolean_v1 oracles was checked against the export lists of both crates:
  - The equality and function oracles keep `rt` = runtime only for runtime-owned items: `TypeEnvironment`, `Value`, `ValueType`, `NodeKey`, `Presence`, `FieldDeclaration`, `CollectionType`, `CheckedPackage`, `Frame`, `PackageDeclarations`. Their kernel names moved to `exact::`.
  - The scalar oracle aliases `quire_exact as rt` and `quire_contract_runtime::exact as residue`. Only enum and quantity residue go through `residue::`.
- **Duplicate names in RT.** RT still defines its own `Value`, `ValueType`, `NodeKey`, `Presence`, `FieldDeclaration`, `CollectionType`, `Location`, `Origin` and `Quantity`, next to same-named `quire_exact` items. RT's module doc lists this as its own open FR-275 residue (QSL-358). It is not a CG copy, and CG cannot swap them, because RT's `TypeEnvironment` and `CheckedEquality` consume RT's types.
- **Wildcard arms.** The removed `_`/`&_` arms over closed kernel enums are correct. The remaining `&_` arm over `QuantityTarget` covers a `#[non_exhaustive]` RT enum. `(Kind::Integer, _)` at `src/oracle/scalar/mod.rs:3095` cannot be reached because of the early return above it.
- **Panics, casts and lints.** The new `execute_native` parse of the observer's `integer_out_of_domain` bounds (`src/kani/run/scalar.rs:679-693`) maps every parse and interval error to a typed `NativeResult`; there is no `unwrap`, no indexing and no `as`. No `#[allow]` was added. New `unwrap`/`expect` calls appear only in tests.
- **Identity test.** `runtime_boolean_outcome_has_the_kernel_type` (`src/core/profile.rs:38`) is a real compile-time identity check. It passes a `quire_exact::Meter` and a closure returning `quire_exact::Outcome` into RT's `evaluate_boolean_short_circuit`, and hands the result to a `fn(quire_exact::Outcome<bool>)`.
  - Mutant: an out-of-tree probe crate declared `quire-exact` by `rev = d41a1da` beside RT 288270a. Cargo resolved two `quire-exact` entries, and the test body failed with three E0308 errors (`Outcome<bool>` ≠ `quire_exact::outcome::Outcome<bool>`, and the same for `Meter`). It cannot be satisfied vacuously.

The high finding is in the code CG emits for the Kani scalar native observer. It does not compile against the kernel that the canonical emitted manifest selects.

Gates run in a throwaway detached worktree with its own `CARGO_TARGET_DIR`:

- `cargo fmt --check`: clean.
- `cargo clippy --locked --all-targets -- -D warnings`: clean.
- `cargo test --locked --workspace --no-run`: builds.
- Focused tests, with no namespace or Kani tests:
  - lib `core::profile`: 2 passed.
  - lib `oracle::`/`replay::`/`kani::generate::`/`core::`: 140 passed.
  - `it` modules `oracle_arithmetic`, `exact_scalar_generation` and `tc_031_ac20`: 67 passed, with `QSPEC_DIR` at QSpec origin/main. The sibling QSpec checkout is 75 commits behind, which fails `tc_024_ac44` for environment reasons only.
  - `it` modules `capability_settlement`, `composite_equality_generation`, `composite_parity_converter`, `exact_function_agreement`, `exact_function_generation`, `exact_scalar_agreement` and `routed_generation`: 191 passed.

These are not the full gate. Kani and the namespace tests were not run here.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The emitted Kani scalar native observer calls `quire_exact::Meter::admitted_charges()`, which exists only under quire-exact's `test-support` feature. The canonical oracle manifest does not enable that feature, so `ir791_native` fails to compile (E0599), and every `execute_kani_scalar_native` run fails at the native step | src/kani/run/scalar.rs:595 |
| FND-002 | medium | `generate_lockfile` pins only `quire-contract-runtime` to CG's lock revision. The emitted crate's direct `quire-exact` dependency resolves to whatever `branch = main` head is in the offline git cache, so the snapshot can prove and run against a kernel other than the one CG was built and tested with | src/kani/run/scalar.rs:805 |
| FND-003 | low | A blanket `tuple`→`limit_tuple` rename damaged comment prose: "a vector is an operand limit_tuple and a limit limit_tuple", "DIV-08 exact limit_tuple", "T11 NFC limit_tuple" | tests/exact_scalar_support/agreement_cases.rs:10 |

## Checks

- FND-001: `write_observer` emits `use quire_exact as rt;` and `for charge in meter.admitted_charges() { ... charge.point.as_str() }` (`src/kani/run/scalar.rs:561,595`).
  - **Why it does not compile.** In quire-exact 37aa172, `Meter::admitted_charges` and `AdmittedCharge` are `#[cfg(feature = "test-support")]` (`src/accounting.rs:570,722`; `src/lib.rs:101`). quire-exact's `Cargo.toml` says a normal or build dependency must never enable that feature.
    - The staged package must equal `oracle_crate_manifest` byte for byte (`src/kani/run/scalar.rs:402`).
    - That manifest declares `quire-exact = { git, branch = "main" }` with no features (`src/core/profile.rs:31`).
    - RT 288270a enables `test-support` only in its own `[dev-dependencies]`, which are not built for a dependent.
  - **Reproduced.** A probe crate used the canonical manifest text, `cargo generate-lockfile --offline`, then `cargo update --offline -p quire-contract-runtime --precise 288270a`, which is the same steps as `generate_lockfile`. It resolved one `quire-exact` at 37aa172, and `cargo build --bin ir791_native` failed with `E0599: no method named admitted_charges found for struct Meter`.
  - **Why this is new.** On main the observer used RT 3cc88c4's own `Meter`, whose `admitted_charges` had no feature gate (`src/exact/accounting.rs:525`).
  - **Consequence.** `execute_native` returns `KaniScalarNativeError::NativeLaunch` after the proof verifies, so no `KaniScalarNativeReceipt` can be produced. The kani-lane tests `tc_027_attested_scalar_receipt_binds_real_proof_and_native_result` and `tc_027_attested_scalar_subtraction_receipts_bind_both_operand_orders` (`tests/it/kani_obligations.rs:2497,2585`) should fail.
  - **Why the test crate passed.** The author's own scratch crate for `oracle_arithmetic` adds `features = ["test-support"]` explicitly (`tests/it/oracle_arithmetic.rs:1266`); the observer did not get the same treatment.
  - **Fix constraint.** Adding `test-support` to the emitted normal dependency would break quire-exact's stated rule. The fix needs a design choice:
    - build the observer as a test or example target, with a dev-dependency that enables the feature; or
    - drop the per-charge log from `NativeMeterObservation` (`src/kani/run/scalar.rs:102-115`) and from FR-017-AC-27..30, keeping the consumed counters.
- FND-002: `generate_lockfile` runs `cargo generate-lockfile --offline`, then `cargo update --offline -p quire-contract-runtime --precise <CG lock rev>`. It checks only the runtime revision (`src/kani/run/scalar.rs:805-890`).
  - **Why it matters now.** The emitted crate now depends on `quire-exact` directly, and the emitted source names kernel APIs directly: `Refusal::IntegerOutOfDomain { target }`, `CheckedInvariantCause::*`, `DivisionMember::Quotient`. Its kernel revision is whichever `main` head was last fetched into `~/.cargo/git/db`, not 37aa172.
  - **Failure scenario.** quire-exact `main` moves; 21 revisions are cached locally. Any other build then refreshes the cache. The snapshot then compiles against a different kernel. Depending on what changed, the emitted source stops compiling, or the observer's tab-separated output no longer matches `execute_native`'s parser. Either way the receipt is not for the kernel CG was tested with.
  - **Mitigation and current state.** The receipt's identity hashes the staged `Cargo.lock`, so provenance is recorded, but nothing prevents the drift. It is latent today: `gh api` shows `main` of both repos equal to the locked revisions (37aa172, 288270a).
  - **Fix.** Pin `quire-exact` with `--precise` to CG's lock revision and check it, the same way the runtime is pinned.
- FND-003: these lines are comments only, and test behaviour is unchanged.
- One kernel identity: `cargo tree --locked -i quire-exact` shows one `quire-exact` (37aa1721). It is reached by CG directly, by RT, by `qsl-replay`/`qsl-cst`/`qsl-eval`/`qsl-forms`/`qsl-foundation`/`qsl-package`/`qsl-semantics`, and by `quire-semantic-value`.
- Lock delta: besides the three locks named in the brief, the lock also moves `quire-contract-ir`/`quire-contract-model` (9be7387 → ef4ab49) and `quire-semantic-value` (b7d5531 → ad6e975). The new IR adds `CheckedMemberType::{String, Uuid, Timestamp}`, and the new qsl-replay adds `WitnessValue::{Uuid, Timestamp}` and `supplied_limits`. CG handles all of these with explicit refusal arms, not wildcards (`src/kani/generate/frame.rs:961`, `src/replay/frame.rs:380`, `src/replay/function.rs:239`), and sets `supplied_limits: None`, as QSL's own doc examples do.
- `CompositeBuildError::PackageRead` now boxes `CheckedPackageRefusal` (`src/replay/composite_builder.rs:58`). This is an API shape change made for clippy's `large_enum_variant`, and all uses are updated.
- Generated emitted-crate manifests: the only emitted `Cargo.toml` is `oracle_crate_manifest`, used by the scalar, equality and function oracles. It names `quire-exact` with the same spelling as CG's own `Cargo.toml` (`EXACT_DEPENDENCY_SOURCE`). The pinned-text test `tc_024_oracle_manifest_names_the_runtime_by_branch_and_carries_no_pinned_revision` asserts that line.
- Boolean V1 native oracle: the emitted source now uses `use quire_exact as rt;` and `rt::planned_equality`/`rt::retain_boolean`. Each connective retains through `completed_boolean` once, after the operands complete. The inline short-circuit table is correct for and, or and implies (`src/oracle/boolean_v1.rs:1228-1297`).
  - Mutant analysis by inspection: in-tree edits were refused, because this review is reviewer-only. Dropping the `retain_boolean` call would make `tc_044_native_connectives_retain_only_completed_results` see 0 retains against the expected 1, 2 or 3. Retaining after a child stop would break its `left_stops` expectation of 0. Either mutant is killed. The test and `tc_044_the_first_stop_in_evaluation_order_is_the_outcome` pass at head.
- CLAUDE.md rules:
  - No new hash, digest or pin record is introduced. The lockfile pin in `generate_lockfile` is CG's own lock, used as the build's source of truth.
  - No new `=` exact pin of a shared ecosystem crate in `Cargo.toml`, which is unchanged.
  - No new top-level `tests/*.rs` file.
  - No new `unsafe`.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | low | The new lock-pin test reads the same `LOCKED_DEPENDENCIES` constant that `generate_lockfile` reads, then repeats the pin commands itself. Deleting `quire-exact` from that constant still passes the test, so FND-002 could regress without any test failing | src/kani/run/scalar.rs:1117 |

- FND-004:
  - The test's assertions loop `for (package, repository) in LOCKED_DEPENDENCIES`, the constant the production loop also uses (`src/kani/run/scalar.rs:41,858`). A mutant that drops the `quire-exact` entry would stop production from pinning the kernel, and the test would still pass. It would only pin and check the runtime, and today `main` still equals the lock (37aa172).
  - The test also copies CG's `Cargo.lock` into the snapshot, but `cargo generate-lockfile` overwrites that copy straight away, so the copy does nothing.
  - Fix: name `quire-exact` and `quire-contract-runtime` literally in the test's assertion. Alternatively, assert `LOCKED_DEPENDENCIES` contains both names.

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | f315205: the staged scalar manifest (`scalar_oracle_crate_manifest`, `src/core/profile.rs:31-49`) adds `[[example]] ir791_native` and `[dev-dependencies] quire-exact` with `features = ["test-support"]`. The normal dependency and the emitted `oracle_crate_manifest` carry no features: `tc_024_generated_crate_is_unpublished_charge_free_and_compiles` asserts no `test-support` and no `[[example]]`, and the emitted manifest must still equal `oracle_crate_manifest` (scalar.rs:402). The observer moved to `examples/ir791_native.rs` with `#[path = "../src/lib.rs"]` and is launched with `cargo run --locked --example ir791_native`. Its output format is byte-identical. Probe: the staged manifest plus runtime 288270a and quire-exact 37aa172 pinned gives `cargo build --offline --locked --example ir791_native` Finished. Dropping the dev-dependency feature gives E0599 again. `cargo tree -e features --edges normal` shows no `test-support` |
| FND-002 | fixed | f315205: `generate_lockfile` loops `LOCKED_DEPENDENCIES` = [quire-contract-runtime, quire-exact] (scalar.rs:41,858). It reads each revision from `include_str!("../../../Cargo.lock")` at build time, not from a literal, then runs `cargo update --offline -p <pkg> --precise <rev>` and checks the result. `tc_027_staged_native_observer_compiles_with_locked_kernel_and_charge_log` asserts both pins in a staged lock |
| FND-003 | fixed | f315205: `agreement_cases.rs:10-11` reads "an operand tuple and a limit tuple", line 217 "DIV-08 exact limits", line 414 "T11 NFC tuple" |
