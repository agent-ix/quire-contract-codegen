---
id: SR-11691
title: "IR-17 PR 397 gap-analysis review"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen@01c8b4f8e1f5c0066ef61ee3f0ea005777c71dfe (base 2897d5eb9cf30e294e3cb8c24b3a29f5d5c37f3b); git diff origin/main...01c8b4f: spec/oracle/functional/FR-014-exact-scalar-oracles.md, spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md, spec/oracle/matrix/TC-024-exact-scalar-oracles.md, spec/oracle/matrix/TC-044-boolean-oracle-integer-arithmetic.md, the 13 changed source files and the 17 changed test files"
review_set: subset
---

## Summary

Ticket IR-17, PR quire-contract-codegen#397. This gap analysis includes test-oracle strength and is scoped to `git diff origin/main...01c8b4f`. Plan completion: not assessed.

It covers:

- the ticket's three acceptance criteria, measured again because ticket text is untrusted;
- the edited spec criteria FR-014-AC-2, AC-6, AC-8, AC-35 and AC-37, and FR-031-AC-22 and AC-26;
- the edited TC-024 and TC-044 procedure text;
- the computed Test Matrix (`quire matrix` 0.36.2), compared between the base and head trees.

## Verdict

**FAIL**: two medium findings and one low.

**Ticket criteria**

- **quire-exact dependency, no kernel copy: met for the CG library.** There is one `quire-exact` lock entry, and CG defines no kernel type. The generated Kani observer does not build; see SR-11690 FND-001.
- **QSL dependency limited to `qsl-replay`: met.**
- **No compatibility layer: met.** The change adds no shim, alias module, `pub use` forwarding or dual spelling. The test-only `rt_side` module (`tests/exact_scalar_support/agreement.rs:433`) now glob-imports `quire_exact::*` plus named RT residue items. It replaces the previous `quire_contract_runtime::exact::*` glob and is a test namespace, not a library compatibility layer.

**Test Matrix, base → head**

| Status | Base | Head |
| --- | --- | --- |
| tagged | 425 | 426 |
| untagged | 294 | 293 |
| untagged, statement not marked PLANNED | 185 | 183 |
| method-without-symbol | 41 | 41 |
| tagged-by-ignored-test | 14 | 14 |

The brief's "212 unbacked" baseline could not be reproduced with these status columns. The counts above are measured from the same tool on both trees.

- **No new unbacked row.**
- **FR-031-AC-26 moves from untagged to tagged.** It is now backed by the new `tc_044_native_connectives_retain_only_completed_results`.
- **FR-014-AC-35 and AC-37 moved from unmarked to PLANNED.** Both were untagged on main. The scalar module derives no Boolean-connective or `eq`/`ne` descriptor (`src/oracle/scalar/mod.rs` has no such family), so PLANNED is accurate. This relabel is not hiding implemented behaviour.

**Criteria carried in this diff**

- **FR-014-AC-2/AC-6 (quotient-only division).** These are backed by `tc_024_tc192_division_and_modulus_oracles_agree`. Its grid runs a, b over -20..=20 against `Bounded[-5, 5]`, which includes vectors whose quotient is in the domain and whose remainder is not, such as 13/7. A pair-admitting oracle would therefore disagree with the direct `divide(.., DivisionMember::Quotient, ..)` call.
- **FR-014-AC-8 (direct quire-exact dependency).** The pinned-text test `tc_024_oracle_manifest_names_the_runtime_by_branch_and_carries_no_pinned_revision` checks the `quire-exact` line. `tc_024_generated_crate_is_unpublished_charge_free_and_compiles` compiles the emitted library with `-Dwarnings`. Because that library passes `quire_exact` values into RT residue calls, the compile fails if two kernel copies resolve.
- **No test lost.** The base has 886 `#[test]` functions and the head has 890. The changes are:
  - Added: `runtime_boolean_outcome_has_the_kernel_type`, `tc_044_native_connectives_retain_only_completed_results`, and the generated `retain_*`/`retain_denied` cases.
  - Renamed: `tc_031_ac20_unknown_outcome_variant_refuses_checked_invariant` became `tc_031_ac20_closed_kernel_outcomes_are_exhaustive`. The kernel `Outcome` is closed, so the old wildcard refusal arm no longer exists. The new test asserts the four explicit arms plus `Err`, in order, for both function bodies.

**Test-oracle strength**

- The identity test is non-vacuous: a two-copy probe fails it with E0308 (see SR-11690).
- The retention test hard-codes independent expected retain counts per case and checks the denied-retain path (`Incomplete` at `BooleanResultRetain`, zero consumed, empty log). It kills both the drop-retain and the retain-after-stop mutants.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-031 and TC-044 say current QSL replay still passes `&Integer` to `Integer::div_mod_floor` and that CG cannot build with the current kernel. At this head both claims are false: qsl-replay 20b3254 passes `NonZeroInteger`, and CG builds and tests against quire-exact 37aa172 | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:73 |
| FND-002 | medium | The emitted Kani native observer (`src/bin/ir791_native.rs` in the staged package) is compiled only inside `#[ignore]` kani-lane tests. No plain `cargo test` builds it with the canonical manifest, so SR-11690 FND-001 passes fmt, clippy, `--no-run` and every non-Kani test | tests/it/kani_obligations.rs:2497 |
| FND-003 | low | Rewritten FR-014-AC-35 and FR-031-AC-26 render the short-circuit connective inline. RT 288270a documents `evaluate_boolean_short_circuit` as kept because "Generated oracles call this one (FR-014-AC-35 of the code generator)". The two repos now disagree on who owns that connective | spec/oracle/functional/FR-014-exact-scalar-oracles.md:464 |

## Checks

- FND-001: the false gate text appears in several places.
  - **Where.**
    - FR-031 lines 73-76, the "Dependency compatibility remains a gate" bullet.
    - FR-031 lines 258-266, "Prerequisite: the one quire-exact lock entry".
    - FR-031 line 274, "still required before this planned rendering can compile".
    - FR-031 OD-2, "AC-22 and AC-26 remain unverified until that code and the coherent CG lock land".
    - TC-044 lines 40-45.
  - **What was measured.** qsl-replay 20b3254 `src/certificate/zone.rs:78-80` builds `NonZeroInteger::try_from(&two)` and calls `div_mod_floor(divisor)`. quire-exact 37aa172 `Integer::div_mod_floor(&self, divisor: NonZeroInteger<'_>)` matches that call. `cargo build`, `cargo clippy` and `cargo test --no-run` all pass on this lock.
  - **Failure scenario.** A planner reading FR-031 holds FR-031-AC-2/6/16/17/22/25/27 as blocked on a QSL fix that has already landed. The same text also contradicts this PR, which lands the AC-26 code and its tagged test.
  - **Fix.** Restate the gate as met at the locked revisions, or delete the sentences.
- FND-002: FR-014-AC-8 now requires the emitted crate to "resolve one exact kernel and compile". FR-017-AC-27..30 bind a receipt to the native result.
  - **What exists now.** The only tests that build the observer are `tc_027_attested_scalar_receipt_binds_real_proof_and_native_result` and `tc_027_attested_scalar_subtraction_receipts_bind_both_operand_orders`, both `#[ignore = "kani lane"]` and both needing namespace support. `tc_024_generated_crate_is_unpublished_charge_free_and_compiles` builds the library only.
  - **Why it matters.** The observer is a generated source string. Rust type checking never reaches it until a scratch crate compiles it, so this oracle gap is exactly how a feature-gated kernel API got in.
  - **Fix.** Add a plain test that stages an emitted scalar package, writes the observer with `write_observer`, seeds the lock as `generate_lockfile` does, and runs `cargo build --offline --bin ir791_native`. This needs no Kani and no namespaces.
- FND-003: the RT doc is at `src/exact/boolean.rs:4-7` in RT 288270a. FR-031-AC-22 forbids any `quire_contract_runtime::exact` path in the native oracle, so CG cannot call RT's helper under its own spec.
  - **The conflict.** RT keeps an exported item whose stated consumer no longer intends to use it, and CG re-renders the same and/or/implies decision table in emitted code. The behaviour matches: one retain on a completed result and none after a stop.
  - **Fix.** Resolve ownership with RT, either by RT dropping or retargeting that doc and item, or by the CG spec naming the helper. Do not keep both.
- Spec carry-through:
  - FR-014-AC-2, AC-6 and AC-8 are strengthened, not weakened.
  - FR-031-AC-26 replaces "charging nothing" with exactly one `boolean.result-retain` per completed connective. This follows OD-6, which is now closed, and TC-044 step 14 matches it.
  - TC-024 steps 4-5 add the quotient-in/remainder-out vector and the one-`quire-exact` compile.
  - No criterion was dropped. `quire validate` over the four files reports only the `ears:non-canonical-trigger` warning at FR-014 line 255, which is also on base.
- Bindings checked: `runtime_boolean_outcome_has_the_kernel_type` → FR-014-AC-8 (correct, identity part); `tc_044_native_connectives_retain_only_completed_results` → FR-031-AC-26 (correct); `tc_024_tc192_division_and_modulus_oracles_agree` → FR-014-AC-6 (correct); `tc_031_ac20_closed_kernel_outcomes_are_exhaustive` → FR-021-AC-20 (correct).

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | low | FR-031 Outputs now says "(the oracle IR-596 implemented uses Contract Runtime's types)", directly after saying the native oracle returns `quire_exact::Outcome<bool>`. At this head the native Boolean oracle emits `use quire_exact as rt;`, so readers can take the parenthetical as describing current state | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:250 |

- FND-004: `src/oracle/boolean_v1.rs` emits `use quire_exact as rt;` for every arithmetic-bearing native oracle. The parenthetical is accurate only as history from before IR-17. Fix: write "the oracle IR-596 originally implemented used Contract Runtime's types", or delete the parenthetical. No criterion depends on it.

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | f315205: FR-031 line 73 reads "Dependency compatibility is met at CG's locked revisions. QSL replay passes `NonZeroInteger` to `Integer::div_mod_floor`, and CG builds with the current kernel." The prerequisite subsection, line 274, OD-2 and TC-044 lines 40-41 are rewritten the same way. Measured again: qsl-replay 20b3254 zone.rs:78-80 builds `NonZeroInteger`, and CG builds at quire-exact 37aa172. Only the "GATED on the lock prerequisite" qualifiers were dropped from FR-031-AC-2/6/16/17/18/22/25/26/27. Every criterion body is unchanged and every one still reads PLANNED (IR-602), so none is weakened |
| FND-002 | fixed | f315205: the new non-ignored lib test `tc_027_staged_native_observer_compiles_with_locked_kernel_and_charge_log` (src/kani/run/scalar.rs:1074) stages `scalar_oracle_crate_manifest`, a stub lib and the real `write_observer` output. It generates the lock offline, pins both dependencies, and runs `cargo build --offline --locked --example ir791_native`. It passes in 8.7s with no Kani or namespaces. Mutants: dropping the dev-dependency feature, or a call to a feature-gated `admitted_charges`, reproduce E0599 in an out-of-tree probe, so the test can fail. It tags FR-017-AC-27/30, which moves tagged-by-ignored from 14 to 12 and tagged from 426 to 428 |
| FND-003 | deferred | Not a CG defect. CG's spec is internally consistent: FR-014-AC-35 and FR-031-AC-26 render the connective inline, and FR-031-AC-22 forbids any `quire_contract_runtime::exact` path in the native oracle. The stale text is RT 288270a `src/exact/boolean.rs:4-7`, which claims CG FR-014-AC-35 calls `evaluate_boolean_short_circuit`. This is an upstream note for the leader to route to quire-contract-runtime, and no CG change is needed |
