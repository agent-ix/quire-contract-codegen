---
id: SR-17015
title: "IR-705 CG #514 code review (Rust lane)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen#514@27d2cfd086fa40a74d50af769f2542a908961bdf vs origin/main 4eccd682; docs/qualification/ir705-native-map.md, src/core/, src/kani/generate/, src/kani/identity.rs, src/kani/run/scalar.rs, src/kani/test_support.rs, src/lib.rs, src/oracle/clause.rs, src/oracle/clause/native.rs, src/replay/witness.rs, src/routed/generate.rs, tests/it/kani_obligations.rs, tests/it/kani_obligations_state_frame.rs, tests/it/kani_scalar_native_fixture.rs, tests/it/kani_scalar_native_private.rs, tests/state_frame_support/native_twin.rs"
review_set: subset
---
# SR-17015: IR-705 CG #514 code review (Rust lane)

## Summary

Ticket: IR-705. This is the code-review lane with the rust-review and rust-style checklist folded in. It covers `git diff origin/main...HEAD`: 22 files, +2285/−51, in two commits (`a25cea4f`, `27d2cfd0`). Exact head `27d2cfd086fa40a74d50af769f2542a908961bdf`, base `4eccd682`.

What was checked, and how:

- **Kernel consumption.** `Cargo.toml` and `Cargo.lock` are unchanged. CG calls the published quire-exact `NativeIntegerPlan` / `admit` / `evaluate_native_integer_arithmetic` / `evaluate_native_integer_ordering` API through the existing lock. No kernel code is copied. There is no vendored file, no compatibility layer, and no new hash, pin or digest catalog. The only digest use is the existing canonical identity mechanism.
- **Generated-source typecheck.** In a throwaway detached worktree (`/tmp/pr514-dump`, removed afterwards), the reviewer dumped the actual generated native scalar TC-027 harness, its bound variant, and the V2 precondition and postcondition harnesses. Each `#[cfg(kani)]` body was compiled against the locked kernel with `--cfg kani` and a stub `kani` module. All three typecheck.
- **Gates.** Fmt-check passes. `make lint` (locked clippy, all targets, `-D warnings`) passes. The focused native lib and `it` tests pass when run one at a time. No real Kani proof and no full aggregate gate was run, as the brief requires.
- **Old FND-005 misbinding.** It is not repeated. The positive helper-witness test traces FR-016-AC-1 (typed decoding). The inclusive-bounds decoder control traces FR-016-AC-2 (out-of-domain). Neither claims FR-016-AC-6 (byte/value limits). The remaining trace defects are in SR-17016.

Findings by area:

- **Scalar route (FND-001).** The native scalar harness proves an oracle that the routed production path never emits.
- **Bound control (FND-002).** The native TC-027 bound control changes the oracle's semantic domain together with the harness bound, so it cannot falsify.
- **Fixture codes (FND-003).** Two new non-ignored tests register conflicting fixture codes, which fails `make test`.
- **Weaker items (FND-004 to FND-007).** A duplicate interval derivation in the V2 postcondition probes, weaker mutant assertions, a string-patched enum, and no in-repo typecheck of generated harness or observer bodies.

AC3's three real proofs have not been run, and this review does not claim AC3 complete. FND-001 and FND-002 must be resolved before those runs can count as the AC3 evidence.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The native scalar route replaces the routed oracle with a second oracle that `LoweredScalarClaim::select_native_integer` renders in the Kani layer. That oracle is proof-only. Routed generation (`generate_routed`, unchanged) still ships the general `rt::Integer` / `evaluate_integer_arithmetic` oracle in `oracle_artifacts` `src/lib.rs`. The native fixture discards that `src/lib.rs` and returns only `Cargo.toml`. So the "healthy routed TC-027" native proof would verify bytes that no production artifact contains, against FR-015-AC-125 ("Native production and proof artifacts use the same kernel implementation") and AD-006 ("Generated production native and Kani clients call the same kernel-owned admitted operations"). The general route embeds the routed oracle verbatim, but the native route does not. Oracle rendering also moves out of `src/oracle` into `src/kani/generate`. Fix: render the native scalar oracle in the scalar oracle generator owned by `src/oracle`, select it in routed generation, and embed that same artifact in the harness. | src/kani/generate/scalar.rs:339, src/kani/generate/scalar.rs:447, tests/it/kani_scalar_native_fixture.rs:193 |
| FND-002 | high | The native TC-027 violating-bound control cannot falsify. `native_original_tc027_false_bound_falsifies` (and its static twin) regenerates the package with result bound `Int[0,5]`. That changes the oracle's semantic domain (`NativeIntegerPlan::new(..., Some([0,5]))`, recorded `semanticDomain` 0..5) and the harness's `admitted` bound together. The oracle then refuses exactly when `exact > 5` and the harness expects that refusal, so the `sound` assertion holds for every draw. Diffing the reviewer-dumped healthy and bound sources confirms the only changes are those two places. The original control (`tc_027_a_routed_scalar_harness_violating_its_bound_is_falsified`) edits only the harness's independent bound literal. The PR's own map (docs/qualification/ir705-native-map.md:11) also says "only its independent result bound changes". As written, the run should end `Verified`, the ignored test would panic, and AC3's falsifying control is not delivered. | tests/it/kani_obligations.rs:3040, tests/it/kani_obligations.rs:3073, docs/qualification/ir705-native-map.md:11 |
| FND-003 | high | `native_scalar_increment_keeps_original_profile_and_checked_capacity` (bound 0..9) and `native_scalar_false_bound_control_keeps_original_draws` (bound 0..5) both register fixture code `49_100` for different nodes in the process-global `application_registry`. They run in the same `it` binary. Reproduced: `cargo test --locked --test it native_scalar_` fails both tests, one with "code 49100 is already registered as … two distinct fixture nodes share one code" and the other with `PoisonError`. Each passes alone. `make test` / `make ci` therefore fails at this head. The two ignored real-Kani tests (`native_original_tc027_increment_verifies` and `native_original_tc027_false_bound_falsifies`) collide the same way when run together. | tests/it/kani_obligations.rs:2973, tests/it/kani_obligations.rs:3040, tests/exact_scalar_support/package.rs:121 |
| FND-004 | medium | The V2 postcondition harness re-derives operand intervals with `native_term`, which pattern-matches operator identity strings. This is a second source of truth beside `oracle::clause::native::helpers`, which works from the typed `ExactScalarOperation` plan. The two disagree. `native.rs` narrows each completed intermediate by its semantic domain (`completed_interval`). `native_term` ignores `IntegerDomain::Bounded`, plans each probe with `None`, and has no signed32 check. Take a clause whose bounded Add feeds a Multiply. The oracle admits the narrowed operand, but the probe's unnarrowed operand can exceed signed32. The probe's `NativeIntegerPlan::new` then hits `panic!("checked native capacity")` and the proof fails on a healthy clause. Its i128 `expected` also ignores intermediate semantic-domain refusal. Fix: derive the probes and the expectation domains from the same typed plan the oracle uses. | src/kani/generate/clause/v2.rs:355, src/kani/generate/clause/v2.rs:479, src/oracle/clause/native.rs:222 |
| FND-005 | medium | `native_tc025_arithmetic_mutant_falsifies` never checks which check failed. The original `tc_025_scalar_harness_falsifies_a_mutated_oracle_arithmetic` asserts the counterexample names the generated `sound` assertion message. The native test does not, and its final `assert_ne!(i128::from(*left) + 1, i128::from(*left) - 1)` is always true. Any falsification passes, including a native-admission `?` stop, an overflow or another panic. The native bound control likewise asserts no failing message. | tests/it/kani_scalar_native_private.rs:68, tests/it/kani_scalar_native_private.rs:143, tests/it/kani_obligations.rs:3073 |
| FND-006 | low | `oracle::clause::native::helpers` adds the `NativeAdmission` variant by text substitution, `source.replace("    OperandWidth,", …)`, on the rendered `OracleStop` enum. If the shared helper renderer's spelling or indentation changes, the replace silently does nothing. The result is uncompilable generated source (`OracleStop::NativeAdmission` unresolved), not a typed refusal. The rust-style rule asks for auditable construction over text patching. | src/oracle/clause/native.rs:64 |
| FND-007 | low | No non-ignored test typechecks the generated `#[cfg(kani)]` harness bodies or the native observer binary. `native_observer_preserves_native_operands_result_and_admission_stop` checks substrings plus `syn::parse_file` only. `native_v2_postcondition_oracle_typechecks_with_direct_dependencies` runs `cargo check --lib` without `--cfg kani` and without the `-Dwarnings` its general sibling sets. A type error in the native harness or observer would first appear during the costly real-Kani run. The reviewer's stub typecheck passed at this head, but nothing in the repo keeps it from regressing. | src/kani/run/scalar.rs:1699, tests/it/kani_obligations_state_frame.rs:679 |

## Verdict

**FAIL — not mergeable at this head.** The kernel is consumed directly and not copied. Identity binds the native selection and policy. Typed capacity refusal is separate from semantic-domain refusal. The V2 precondition and postcondition oracles come from the production `generate_v2_clause_native_oracle`. The generated sources typecheck. The old FND-005 trace misbinding is corrected.

Three high findings block merge:

- The native scalar proof subject is a proof-only oracle (FND-001).
- The TC-027 falsifying control cannot falsify (FND-002).
- The suite fails on a fixture-code collision (FND-003).

Fix FND-001 to FND-003 before the coordinated 16 GiB / unwind 3 / Cadical runs. Otherwise those runs cannot serve as AC3 evidence. Observation outside this diff: the untouched `bounded_single_run_preserves_cancelled_with_start_signal_and_timeout_without_signal` failed once under a parallel build load and passed alone, so it is load-sensitive.
