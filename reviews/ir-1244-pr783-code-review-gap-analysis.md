---
id: SR-17940486709564847047660
title: "IR-1244 PR #783 S1 pre-decoded Kani function harness gap analysis"
type: SpecReview
analysis: gap-analysis
date: 2026-11-11
scope: "agent-ix/quire-contract-codegen; PR #783 @ 2d3ebcf5 (slice S1); FR-021-AC-42, FR-021-AC-44 through FR-021-AC-47, TC-031; src/oracle/function/kani_harness.rs, src/oracle/function/kani_descriptor.rs, src/oracle/function/owner_render.rs, their tests, tests/it/kani_function_harness.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-021
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-031
    type: reviews
---

## Summary

Ticket: IR-1244, slice S1. Plan completion: not assessed (planless). Inputs: `quoin matrix --json` (quoin 0.29.1) at 2d3ebcf5, the tagged tests, and the source.

The matrix shows:
- FR-021-AC-44 is tagged by 6 binders: 2 in kani_descriptor tests, 2 in kani_harness tests and 2 in tests/it/kani_function_harness.rs.
- FR-021-AC-46 is tagged by 7 kani_harness tests.
- FR-021-AC-45 and AC-47 are untagged, as expected for S2 (PLANNED).
- FR-021-AC-42 keeps its existing `kani_build_keeps_body_modules_and_drops_host_entries` binder.
- All of these are "no run evidence" in the matrix. The AC-44 and AC-46 method strings `Inspection, Test` and `Test, Inspection` are reported as unknown-method by quoin. That is pre-existing spec wording from PR #770 and is not counted against this PR.

The tagged tests were executed by the reviewer and passed: 12 `it` tests in the focused set and the 13 new lib tests. One mutation control (record presence forced to Required) was killed by both the lib inspection test and the integration Debug differential. Production code with no owning requirement: none. The two untagged helper tests (`function_origin_matches_the_existing_generator_location`, `node_keys_render_without_recursion_at_depth`) back AC-43 location parity and the iterative rendering, and are not coverage inflation. No stubs and no tautological assertions were found: each injected-violation control in `kani_items_rebuild_from_descriptors_through_genuine_constructors` changes the inspected text and asserts that a violation is flagged.

## Verdict

**CHANGES REQUIRED (test strength).** AC-46's reader causes, nested-call cause, acquisition, selection and owner-identity causes all have value tests that can fail. Catalog-unavailable is covered by a sound inspection, because `CatalogReadError` cannot be constructed. AC-44's no-reader, no-prebuilt-environment, no-verdict and seal checks have injected-violation controls. The gaps are one AC-44 mutation row with no failing control, one AC-46 cause with no test, and a differential that omits two descriptor fields.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-44 mutation row "drop ready/finish or the admits guard from the cfg(kani) items" has no failing control. The inspection only asserts that `admits(` is absent from the `cfg(kani)` items and never checks positively that the proof-driven `body_{symbol}` `Invocation::resume` calls `ready()` and `finish()`, or that `finish` calls `admits`. The scalar probe asserts `Completed(Boolean(true))`, which a body with `finish` removed still returns. | src/oracle/function/kani_harness/tests.rs:179-205, tests/it/kani_function_harness.rs:244-268 |
| FND-002 | low | AC-46's owner-path cause is value-tested only for Acquisition, selection and OwnerIdentityMismatch. No test, and no documented inspection like the catalog one, covers a `checked_function_declarations` or `checked_composite_declarations` (prepared-function construction) failure. The selection assertion also accepts either `FacadeLookup` or `InvalidIdentity` instead of the one code the genuine path returns. | src/oracle/function/kani_harness/tests.rs:376-412 |
| FND-003 | low | The integration differential prints and compares only environment, parameters and result. The result location and `TypeEnvironmentLimits` are in the Behavior descriptor set but are not compared. Its "genuine" side also re-implements `PreparedFunction::new`'s environment assembly in the test instead of running the emitted constructor; that part is S2/AC-45, so it should be stated as such in the test doc. | tests/it/kani_function_harness.rs:96-133, tests/it/kani_function_harness.rs:253 |
| FND-004 | low | The kani_descriptor round-trip scratch crate runs `cargo run --offline --quiet` without `--locked`, so drift from the seeded Cargo.lock is not caught. The `it` scratch crates use `--locked --offline`. | src/oracle/function/kani_descriptor/tests.rs:199-203 |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-021-AC-42 | examined | The emitted function-oracle crate, compiled with cfg(kani), declares every body_{symbol} module pub(crate) with a pub(crate) Factory and omits every oracle_{symbol} host entry |
| FR-021-AC-44 | examined | the cfg(kani) items of the emitted crate rebuild the prepared function from the literal descriptors Behavior lists, call TypeEnvironment::bounded_with_cancel and, for each union declaration, seal_union_verified, and run PreparedFunction::ready and finish with their admits guard. |
| FR-021-AC-45 | context_only | embedded cfg(not(kani)) host differential (S2, PLANNED) |
| FR-021-AC-46 | examined | The Kani-harness entry refuses at generation time, with a typed refusal and no harness, const or partial bundle, each of five causes kept apart |
| FR-021-AC-47 | context_only | fixture adverse controls (S2, PLANNED) |
| TC-031 | context_only | function-application oracle test case; step 16 is S2 |
