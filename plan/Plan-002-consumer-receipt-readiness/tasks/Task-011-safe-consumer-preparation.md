---
id: Task-011
title: "IR-1114 — audit safe consumer/profile preparation"
type: Task
status: not_started
track: A
priority: P1
relationships:
  - { target: ix://agent-ix/quire-contract-codegen/FR-015, type: references }
  - { target: ix://agent-ix/quire-contract-codegen/FR-017, type: references }
  - { target: ix://agent-ix/quire-exact/FR-359, type: references }
  - { target: ix://agent-ix/quire-exact/FR-371, type: references }
  - { target: ix://agent-ix/quire-contract-codegen/TC-025, type: verifies }
  - { target: ix://agent-ix/quire-contract-codegen/TC-027, type: verifies }
---
# Task-011: Safe consumer/profile preparation

## Scope

Prepare execution without launching a backend or changing the authentic consumer tests. Read accepted AD-006, the current Exact obligations, CG generation helpers and all call paths of proposed host tests. Reuse existing native controls; do not write another harness or test API.

## Subtasks

- [ ] Audit and run only exact qualified selectors in the existing `it` binary: `kani_obligations::native_scalar_increment_keeps_original_profile_and_checked_capacity`, `kani_obligations::native_scalar_false_bound_control_keeps_original_draws`, `kani_obligations::tc_027_attested_scalar_subtraction_fixture_has_two_bounded_operands`, and `kani_obligations::tc_027_attested_scalar_rejects_preflight_mismatches_without_launch`. Establish that each calls generation/inspection or refuses before backend dispatch. The last test's fake launcher must remain uncalled and its build target absent.
- [ ] Compare original/native semantic domains, checked interval capacity, solver, unwind, configured ceilings and oracle embedding. A configured resource ceiling is not a measured peak. Preserve Add input `[0,9]` and singleton one, subtraction's two `[0,9]` operands, and the original false-bound control's unchanged draws.
- [ ] Reconcile the five normative receipt names with current CG entries using Task-013. The historical healthy test name is absent; the present wrapper delegates `verify_routed_scalar_harness_real_kani`. Merged CG PR #601 moved the same proof assertions into that helper and its ignored wrapper; the routed scalar fixture and original profile remain the proposition. Use the current exact selector without an AC amendment, and never report a nonexistent selector as passing.
- [ ] Inspect the current dependency/adoption relationship against the reviewed Exact kernel. Existing generated native proofs are not new receipts merely because the lock can resolve them. Do not edit the lock as a documentation exercise.

## Deliverables

An operational preparation report with application version, exact selectors, actual exit codes, preserved profile observations, unrun checks and unresolved adoption/mapping prerequisites. Keep observations outside this authored plan definition. Record no Kani verdict for host-only checks.

## Notes

`native_scalar_generated_proof_body_typechecks` creates scratch Cargo metadata/check subprocesses and substitutes Kani syntax with compile stubs. It is optional non-qualification evidence, deferred unless child Cargo can meet the thermal procedure without deadlock; even a pass is no proof. No broad `cargo test`, `--ignored`, backend discovery/dispatch, or 16 GiB run is part of this task.
