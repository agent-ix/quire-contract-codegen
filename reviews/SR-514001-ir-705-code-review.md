---
id: SR-514001
title: "IR-705 CG #514 code review (Rust lane)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen#514@27d2cfd086fa40a74d50af769f2542a908961bdf vs origin/main 4eccd682; docs/qualification/ir705-native-map.md, src/core/, src/kani/generate/, src/kani/identity.rs, src/kani/run/scalar.rs, src/kani/test_support.rs, src/lib.rs, src/oracle/clause.rs, src/oracle/clause/native.rs, src/replay/witness.rs, src/routed/generate.rs, tests/it/kani_obligations.rs, tests/it/kani_obligations_state_frame.rs, tests/it/kani_scalar_native_fixture.rs, tests/it/kani_scalar_native_private.rs, tests/state_frame_support/native_twin.rs"
review_set: subset
---
# SR-514001: IR-705 CG #514 code review (Rust lane)

## Summary

Ticket: IR-705. This is the code-review lane with the rust-review and rust-style checklist folded in. It covers `git diff origin/main...HEAD`: 22 files, +2285/−51, in two commits (`a25cea4f`, `27d2cfd0`). Exact head `27d2cfd086fa40a74d50af769f2542a908961bdf`, base `4eccd682`.

What was checked, and how:

- **Kernel consumption.** `Cargo.toml` and `Cargo.lock` are unchanged. CG calls the published quire-exact `NativeIntegerPlan` / `admit` / `evaluate_native_integer_arithmetic` / `evaluate_native_integer_ordering` API through the existing lock. No kernel code is copied. There is no vendored file, no compatibility layer, and no new hash, pin or digest catalog. The only digest use is the existing canonical identity mechanism.
- **Generated-source typecheck.** In a throwaway detached worktree (`/tmp/pr514-dump`, removed afterwards), the reviewer dumped the actual generated native scalar TC-027 harness, its bound variant, and the V2 precondition and postcondition harnesses. Each `#[cfg(kani)]` body was compiled against the locked kernel with `--cfg kani` and a stub `kani` module. All three typecheck.
- **Gates.** Fmt-check passes. `make lint` (locked clippy, all targets, `-D warnings`) passes. The focused native lib and `it` tests pass when run one at a time. No real Kani proof and no full aggregate gate was run, as the brief requires.
- **Old FND-005 misbinding.** It is not repeated. The positive helper-witness test traces FR-016-AC-1 (typed decoding). The inclusive-bounds decoder control traces FR-016-AC-2 (out-of-domain). Neither claims FR-016-AC-6 (byte/value limits). The remaining trace defects are in SR-514002.

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

## New findings (disposition pass 1)

Found at head `cb2c12e004002c2d08752d666c5c59486a4d9dc4` in the fix-round commits `dcfcebb9` and `cb2c12e0`.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-008 | low | The shared `SOURCE_HEADER` gains a `    /* native refusal */` line, and `SourceBuilder::finish` replaces it with an empty string when native admission is off. Every general-route scalar oracle and clause helper source now carries one more whitespace-only line (four spaces) inside `OracleStop`. So the ordinary `generate_routed` artifact bytes, their line numbers and the identities of general-route harnesses change in a PR whose AC says the general route is retained. The fix is to insert the variant without leaving a line, for example by replacing the whole line including its indentation and newline. | src/oracle/scalar/render.rs:61, src/oracle/scalar/render.rs:180 |
| FND-009 | low | The fix round adds lint suppressions, although the PR body states "no warning flags are removed or lint suppressed". The generated observer now emits `#[allow(dead_code)]` on `mod generated`. The new `-Dwarnings` typecheck controls prepend `#![allow(dead_code, unused_imports)]`, and the V2 control adds `unused_parens`. That last one exempts the generated V2 proof body from the lint that `-Dwarnings` is there to enforce. The claim and the code should agree: either drop the suppressions and fix the generated text, or record the reason for each one. | src/kani/run/scalar.rs:888, tests/it/kani_obligations_state_frame.rs:723, tests/it/kani_obligations.rs:3064 |
| FND-010 | low | `generate_routed_native` adds a `/// Trace: FR-015-AC-125` doc line. `Trace:` doc lines are a deprecated binding convention, and ix-trace-rs `#[trace]` on tests is the binding the matrix reads. The new line adds an unmeasured claim. | src/routed/generate.rs:315 |
| FND-011 | low | `oracle::scalar::native::render` chooses the native variant by matching identity strings (`"quire.op.integer.add"`, `".sub"`, `".mul"`, `".negate"`). `select` already receives the typed `IntegerOperator`, and the operator catalog owns that mapping. This is a second, string-keyed copy of the catalog in the renderer that both the routed artifact and the harness depend on. A catalog rename would make native publication refuse with `InvalidPlanState`, and nothing in the type system ties the two together. | src/oracle/scalar/native.rs:94, src/oracle/scalar/native.rs:104 |

## New findings (disposition pass 2)

Found at head `6d405d29c7255654da4e96376bfaebcda9b0e7ee`. The defect was introduced in `dcfcebb9`, was missed in pass 1, and is still present.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-012 | high | `SourceBuilder::finish` (non-test code in `src/oracle/scalar/render.rs`) calls `.expect("the scalar source header declares its native refusal insertion point")`. FR-014-AC-39 requires zero panic tokens in that file's non-test code, and origin/main `dd59aa14` has none. The suite's guard for it fails at this head. Reproduced: `tc_024_ac39_scalar_generator_and_generated_crates_hold_no_panic_site` fails with `src/oracle/scalar/render.rs left: ["expect"] right: []`, so `make test` fails. The fix is to insert the native variant without a panic site, for example by building the enum body from the typed flag instead of splitting the header text. | src/oracle/scalar/render.rs:182, tests/it/exact_scalar_generation.rs:3579 |

## New findings (disposition pass 4)

These defects were present at `ff42c8496613f5f297e5ca8e5bb9bf90898d7713`. The pass-3 review missed them, because it did not run the `layout`, `no_generation_panics` or `kani_gate` suites. The author's aggregate gate on `af1b6ccb` caught them. They are recorded here so the record is complete. All three are fixed at `8c7b3598`.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-013 | high | `src/oracle/scalar/mod.rs` added `pub(crate) use render::publish_native_scalar_artifacts;` and `pub(crate) use render::render_clause_native_operation_helpers;`. Layout rule L-2 says no file except the crate root re-exports an item, and `layout::l_2_no_file_but_the_crate_root_re_exports_an_item` fails. | src/oracle/scalar/mod.rs:843 |
| FND-014 | high | Non-test Kani generation code added panic tokens: `kind.expect("guarded clause kind")` in `classify_v2_clause`, and `unreachable!` in the `ConditionalProbe` renderer. `no_generation_panics::tc_042_ac1_src_holds_no_panic_token` fails. | src/kani/generate/negotiate.rs:666, src/kani/generate/render.rs:144 |
| FND-015 | medium | `kani_obligations_state_frame::native_v2_helper_mutant_playback_decodes_typed_original_witness` is ignored without a `kani lane:` tag. Because its full name contains the `kani_obligations` lane filter, `make kani --ignored` would select it, and it panics without its external artifact env vars. `kani_obligations::native_original_tc027_increment_verifies` is a real lane proof whose ignore reason lacked the tag. `kani_gate::ignored_lane_selection_and_transitive_include_targets_match_scope` fails. | tests/it/kani_obligations_state_frame.rs:546, tests/it/kani_obligations.rs:3137 |

## Dispositions

Disposition pass 1 at head `cb2c12e004002c2d08752d666c5c59486a4d9dc4` (rebased onto `dd59aa14`, the #564 scalar split). `git range-diff 4eccd682..27d2cfd0 dd59aa14..cb2c12e0` shows the two reviewed commits are patch-identical (`a25cea4f = 0bed25e6`, `27d2cfd0 = c13bb455`), so the review-pass verdict carries for them. The fix commits `dcfcebb9` and `cb2c12e0` were reviewed in full. `cb2c12e0` only moves the `native_probe_tests` module below the production items. The #564 split is preserved: `src/oracle/scalar/{mod,derive,lower,render}.rs` name `limit_kind` 1/2/0/0 times, and `tc_024_ac42` passes.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | `generate_routed_native` now runs native negotiation and `publish_native_scalar_artifacts`. That function rewrites the routed `src/lib.rs` from the same `oracle::scalar::native::render` that the harness embeds. The fixture asserts the harness contains the routed production oracle byte for byte, and the oracle rendering now lives in `src/oracle/scalar/native.rs`. |
| FND-002 | fixed | The bound control keeps `Bound::Integer(0, 9)` (semantic domain and routed oracle unchanged) and edits only the harness literal `exact <= i128::from(9_i64)` to 5. The static twin asserts that the mutant still carries `NativeIntegerInterval::new(0_i128, 9_i128)`. |
| FND-003 | fixed | Every native routed fixture now registers code 49_100 for the same `Bound::Integer(0, 9)` node. The focused `it` and lib native tests pass together under RUST_TEST_THREADS=4. |
| FND-004 | fixed | `bind_native_arithmetic_probes` overwrites each probe's operand bounds and semantic domain from the oracle's own typed `NativeIntegerSelection` decisions, matched by operation identity and count. Short-circuit guards and a `completed_prefix` carry intermediate refusal into the final assertions and cover. New tests check the Add-to-Multiply intermediate interval. |
| FND-005 | fixed | `run_mutant` now requires the counterexample to contain `Check for \`assertion\`` and the generated sound-assertion message. The tautological `assert_ne!` is removed, and the bound control shares the same check. |
| FND-006 | fixed | The text `replace` is gone. `SourceBuilder` takes a typed `needs_native_admission` flag and splices the variant at a declared header insertion point. The byte side-effect on the general route is recorded separately as FND-008. |
| FND-007 | fixed | New non-ignored `-Dwarnings` typecheck controls compile the native scalar proof body, the native V2 proof body (with stub `kani`) and the native observer example against the locked kernel. All three pass. |

Disposition pass 2 at head `6d405d29c7255654da4e96376bfaebcda9b0e7ee`, which covers repair commit `8dca8c01` and the review-ID rekey commit `6d405d29`. Checked against the code at that head.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-008 | fixed | `SOURCE_HEADER` is byte-identical to origin/main `dd59aa14` again, with no native placeholder. The native variant is spliced in only when `needs_native_admission` is set. The general route therefore emits main's exact `OracleStop` bytes, and general harness identities are unchanged. |
| FND-009 | fixed | The observer's `#[allow(dead_code)]` is removed, and the observer reads the field (`NativeAdmission(reason) => { let _ = reason; ... }`). The typecheck stubs no longer carry `allow(dead_code, unused_imports, unused_parens)`. Proof functions become `#[test]` under `cargo check --tests` with `-Dwarnings`, and the V2 `expected` binding goes through `native_expected_expression`. Both controls pass. The one remaining `#[allow(dead_code)]`, on the shared fixture, predates pass 1 and carries a stated reason. |
| FND-010 | fixed | The `/// Trace: FR-015-AC-125` doc line is removed. The binding is carried by the `#[trace]`-tagged tests. |
| FND-011 | fixed | `native::render` now takes a typed `IntegerOperator`. The operation variant is chosen by matching on the operator. The decision identity is checked through `Operator::from_identity(..).entry().scalar_operator`, and `publish_native_scalar_artifacts` resolves the operator through the catalog. No identity-string selection remains. |

Disposition pass 3 at head `ff42c8496613f5f297e5ca8e5bb9bf90898d7713`, rebased onto CG main `1944268f` (#567/#570). `git range-diff dd59aa14..6d405d29 1944268f..ff42c849` shows commits 3 to 6 are patch-identical. Commits 1 and 2 differ only by adapting to #570's typed Kani output reader. `decode_v2_clause_falsification` now calls `parse_assertion_playback` with the caller's byte and value limits, then passes `&Playback` to `decode_falsification`. That is the pattern every main replay caller uses, and the bounds check is unchanged. The re-export moved `DecodeFailure` to `replay::witness`. Those adaptations were reviewed and carry no defect. The new commit `ff42c849` was reviewed in full. The #564 split is intact: `limit_kind` counts are 1/2/0/0, plus 0 in `native.rs`. Spec text is unchanged from pass 2 and no AC is narrowed.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-012 | fixed | `SourceBuilder::finish` now appends the optional native variant, the quantity variant and the closing brace with `push_str`, and the header no longer carries the placeholder. No `expect` or panic token remains in non-test `render.rs`, and no allow was added. `tc_024_ac39_scalar_generator_and_generated_crates_hold_no_panic_site` and `tc_024_ac42` pass. The general-route header bytes still equal main's, and the native header still compiles (`native_scalar_generated_proof_body_typechecks`, `native_v2_postcondition_oracle_typechecks_with_direct_dependencies`). |

Disposition pass 4 at head `8c7b3598134328fafdbf5d8d4307a24d1c8252e9`, rebased onto main `b60b18ef`. `git range-diff 1944268f..ff42c849 b60b18ef..8c7b3598` shows commits 1 to 7 are patch-identical, so their verdicts carry. `9a152e5d` only records the pass-3 rows. Repair commit `8c7b3598` was reviewed in full. The #564 split still holds: `limit_kind` counts are 1/2/0/0, plus 0 in `native.rs`. The spec diff against main is unchanged, with only the appended FR-015-AC-125 and AD-006 text, so no AC is narrowed.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-013 | fixed | Both `pub(crate) use` lines are deleted, and no other forwarding path replaces them. `routed/generate.rs` and `oracle/clause/native.rs` now call `crate::oracle::scalar::render::{publish_native_scalar_artifacts, render_clause_native_operation_helpers}` directly through the existing `pub(crate) mod render`. All `layout::l_2_*` tests pass. |
| FND-014 | fixed | `classify_v2_clause` now matches `kind` and returns the typed `Unsupported(V2ClauseConditionNotRendered)` for `None`. The probe renderer returns `Err(InvalidGeneratedSyntax)` instead of `unreachable!`. No panic token is added to non-test production code. The remaining `expect`/`panic!` text is emitted into the cfg(kani) harness as literals, which the scan excludes by rule. `tc_042_ac1_src_holds_no_panic_token` and `tc_024_ac39` pass. |
| FND-015 | fixed | The helper-mutant decoder becomes `pub(super) fn decode_native_v2_helper_mutant_playback`, and its ignored, traced entry point moves to the new module `native_v2_helper_playback`, which no lane filter selects. It is a manual artifact-decoding control, not a Kani run, so leaving it out of the lane is correct, and it still runs explicitly. `native_original_tc027_increment_verifies` is retagged `kani lane:`, so `make kani` now selects the real TC-027 proof. Neither the filter list nor the gate was widened, and no ignore was added. `ignored_lane_selection_and_transitive_include_targets_match_scope` passes. |
