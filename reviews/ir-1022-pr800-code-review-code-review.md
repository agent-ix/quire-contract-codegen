---
id: SR-17940486709564847047718
title: "IR-1022 PR #800 code-review (rust-review lane)"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen PR #800 at 2a45f2e8 (base main cd660b50); IR-1022; src/kani/generate/clause/v2.rs, src/kani/generate/render.rs, src/kani/generate/render/constrained.rs, src/kani/generate/spec.rs, src/oracle/clause.rs, src/oracle/scalar/render.rs, Cargo.lock, tests/it/native_boolean_equality_differential.rs, tests/it/kani_obligations_state_frame.rs, tests/it/main.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
---

## Summary

Ticket: IR-1022. The PR makes three generator changes for the native V2 clause proof: (A) the
clause-native Boolean `==`/`!=` renders as `rt::evaluate_native_boolean_equality(left, right, meter)`
with `!=` negating the completed relation; (B) `Statement::Forget`, which the top-level renderer
collects and emits after the cover, and which probes emit inline after their assertion; (C) the
native oracle returns `Result<bool, ClauseStop>` with a payload-free `Copy` `ClauseStop`, with
per-shape match arms in the V2 and constrained renderers.

Gates run in an own worktree at 2a45f2e8, each cargo job under `flock /tmp/cargo-slot.lock`:
`cargo fmt --check` clean; `cargo clippy --all-targets -- -D warnings` clean; `cargo test --lib`
540 passed, 7 ignored; the differential test passed; `--test it` with the filters native_boolean,
native_clause, state_frame, v2_clause, constrained and kani_obligations 242 passed, 21 ignored;
`--test it layout` 8/8.

Reviewer mutation controls (each reverted, tree clean afterwards):

- M1, `src/oracle/scalar/render.rs:306` `related == {equal}` to `related == true`, so `!=` is no
  longer negated: **survives**. 294 passed, 0 failed (filters above plus `oracle`), including
  `native_boolean_equality_and_inequality_match_the_typed_route` (FND-001).
- M2, `THIN_MEASURED_HELPER` maps every `Refused` to `ClauseStop::Refused`, so
  `IntegerOutOfDomain` is collapsed: killed only by the source-text assertion in
  `native_clause_oracle_stop_is_a_payload_free_copy_kind`. No executed host test observes the
  native out-of-domain stop (FND-004).
- M3, `forget_fat_values` no longer pushes `outcome` on the general (non-native) path:
  **survives**. 303 passed, 0 failed (FND-002).

Soundness checks with no defect found:

- (B) The top-level renderer (`render.rs` `render_statements`) puts every top-level `Forget` in a
  list and writes it after the `kani::cover!` line, so the forget follows every assertion and the
  cover whatever its position in `body`. In the rendered V2 harness of the healthy clause (scratch
  `ir1022-final/healthy/src/lib.rs:231-238`) the probe asserts and then forgets
  `arithmetic_outcome_0` and `arithmetic_meter_0`, the two top-level asserts and the cover observe
  `outcome`, and `core::mem::forget(meter)` is the last statement. The mutant crates have the same
  structure. Only drop glue is removed. Kani's assert acts as an assume after it, so the cover
  still needs every assertion to hold on some path. The forget does not change that: in the helper
  mutant run the cover is reported unreachable, which is honest. `forget(outcome)` is emitted only
  on the general path, since the native `Result<bool, ClauseStop>` is `Copy`. That matches "probes
  forget outcome+meter".
- (C) The general oracle maps `Err(Runtime(o))` to `Ok(o)`, so its `Err` arm only carries
  `Oracle`/`InvalidConstant`. Native mapping: `Undefined` to `Err(Undefined)`;
  `Refused(IntegerOutOfDomain)` to `Err(IntegerOutOfDomain)`; any other `Refused` to `Err(Refused)`;
  `Incomplete` to `Err(Incomplete)`; scalar `OracleStop` to `Err(Oracle)`. `OracleStop` is `Copy`,
  so dropping it in `|_|` loses no drop glue. In the V2 harness patterns, `OutOfDomain` is the only
  refusal kind the general harness singled out (`Refused(IntegerOutOfDomain { .. })`), and the
  native harness keeps it. In the constrained renderer, the arms give the same panic message per
  case as before: undefined, refused (both refusal kinds), incomplete, and owner call failure
  (`Oracle`/`InvalidConstant`). No variant merges two cases that a consumer of the general oracle
  tells apart.
- (A, type) `EqualityOperandKind::Boolean` comes from the operation identity
  (`quire.op.boolean.eq`/`ne`). The clause plan checks each child step's `ValueKind` against
  `[Boolean; 2]` before rendering (`src/oracle/clause.rs` ~612). The kernel call's parameters are
  `bool`, so a non-Boolean operand would be refused or fail to compile. It cannot be picked
  silently.
- Panic/unsafe: no new `unwrap`/`expect`/`unsafe`/`as` in non-test `src`. The `panic!` strings
  are in generated harness text, as before.
- Trace tags: `#[trace(..)]` only. Pins: `Cargo.lock` moves quire-exact on its existing
  `branch=main` source (5db2d5e3 to e82db487, QE #151/#152). That is a lock refresh, not a new pin,
  hash or digest, and no test pins a content hash that changed.

## Verdict

**Not mergeable as is.** FND-001 (high): the differential test that AC-146 names does not exercise
the rendered route, and a `!=` regression in the generator passes every test. FND-002 (medium):
the general-path half of AC-143 is untested. The soundness of B and C holds, and fmt, clippy, lib
and focused tests and layout are green.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The differential test compares `TypeEnvironment::check_equality(..).evaluate(..)` with a hand-written copy (`fn native`) of the rendered body, not with the code the generator renders. Reviewer mutant M1 (`related == {equal}` to `related == true` in `src/oracle/scalar/render.rs:306`, so `!=` is never negated) passes all 294 focused tests, the differential included. The only renderer check is the substring `related == true` for the fixture's `==`, and no native fixture uses `!=`. A wrong `!=` oracle would make Kani prove the wrong property. Fix: drive the differential through the generated helper source (render the clause-native helper for `quire.op.boolean.eq` and `quire.op.boolean.ne` and compile or `include!` it), or at minimum assert the rendered `related == false` for `!=` and run M1 | tests/it/native_boolean_equality_differential.rs:54-66, src/oracle/scalar/render.rs:293-308 |
| FND-002 | medium | AC-143's general-path branch (`forget(outcome)` when the oracle outcome is not `Copy`) has no test. `native_clause_harness_forgets_fat_values_after_the_cover` checks only the native harness, and no test asserts `core::mem::forget(outcome);` in a general V2 harness. Reviewer mutant M3 (drop `names.push("outcome")` in `forget_fat_values`) survives 303 tests | src/kani/generate/clause/v2.rs:910-922 (forget_fat_values at :913), tests/it/kani_obligations_state_frame.rs:1413-1454 |
| FND-003 | low | The differential tightens only `ValueOccurrences`, `WorkUnits` and `ResultUnits`. The other seven `KINDS` hit `_ => continue` without asserting their spend is zero, so "at each tightened limit" is complete only on an unstated premise. Assert `spend == 0` for the skipped kinds or tighten them too | tests/it/native_boolean_equality_differential.rs:104-118 |
| FND-004 | low | The native `IntegerOutOfDomain`/`Refused` split in `THIN_MEASURED_HELPER` is pinned only by source-text `contains` checks (M2 is killed by a `contains` assertion). No host test executes a native oracle to a stop. The native out-of-domain behaviour rests on the installed-Kani lane alone | src/oracle/clause.rs:684-707, tests/it/kani_obligations_state_frame.rs:1370-1410 |
| FND-005 | low | `HarnessSpec` validation returns `HarnessSpecError::AssertBeforeCall` for a top-level `Forget` of an unbound name, and `InvalidExpression` for one inside a probe. Neither names the defect, which is an unbound or unknown forget target | src/kani/generate/spec.rs:559-563, src/kani/generate/spec.rs:575 |

## Dispositions

Round 1, fix head 4a2849b9 (one commit on 2a45f2e8). origin/main is still cd660b50 and
`git merge-tree` is clean. Checked against `git diff 2a45f2e8..4a2849b9` and the code. The only
`src/` changes are `HarnessSpecError::ForgetBeforeCall`/`ForgetUnbound` in
`src/kani/generate/spec.rs` and their mapping in `collection.rs`. Both are validation-only and
change no rendered statement. The twin fixture refactor keeps the same condition string, so the
generated harness text, and with it the earlier Kani numbers, is unchanged.

One batched job in an own worktree under the cargo lock: fmt clean; clippy -D warnings clean;
`--lib` 541 passed; `--test it` with the focused filters plus the new tests 252 passed, 21 ignored;
the new scratch-crate test passes in 5.2 s (5.3 s on rerun, `--locked --offline` with a seeded
lock). Reviewer M1 re-run now FAILS `generated_native_boolean_equality_matches_the_typed_route`.
Reviewer M3 re-run now FAILS
`general_clause_harness_forgets_the_outcome_and_the_meter_after_the_cover`. Both reverted.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | The hand-written differential is deleted. `generated_native_boolean_equality_matches_the_typed_route` compiles the generator's real native oracle source for a `result = ..` and a `result != ..` clause in a scratch crate. It runs the `quire.op.boolean.eq`/`ne` helpers against `check_equality(..).evaluate(..)` and fails by name under M1. |
| FND-002 | fixed | `general_clause_harness_forgets_the_outcome_and_the_meter_after_the_cover` asserts `forget(outcome)` and `forget(meter)` once each, after the cover, in the general harness. It fails under M3. |
| FND-003 | fixed | The scratch-crate test tightens all ten kinds (`tighten` covers every `LimitKind`, bounds 0..=spend) and asserts that the seven kinds outside value_occurrences/work_units/result_units spend 0. |
| FND-004 | fixed | The same scratch crate calls the generated `measured` with a constructed `Refusal::IntegerOutOfDomain` and asserts `Err(IntegerOutOfDomain)`. An out-of-interval operand asserts `Err(Oracle)`. Owned by the new FR-015-AC-150. |
| FND-005 | fixed | New `ForgetBeforeCall` and `ForgetUnbound(name)`, with the probe-local unbound case also `ForgetUnbound`. Unit test `forget_statement_refusals_name_their_own_errors` covers all three cases and the accepted case. |

Round 2, head 9bbc7d89 (fix commits 1f071146 and 9bbc7d89 on 4a2849b9). origin/main is now
04148fc7 and `git merge-tree` is clean. This round re-reviews the render change in
`git diff 1f071146..9bbc7d89`. When a harness has forgets, the renderer now emits
`let cover_reached: bool = <cover condition>;`, then the forgets, and ends with
`kani::cover!(cover_reached, "<message>")`. Harnesses with no forgets keep the inline cover.

- The cover is the single, last statement.
- Every assertion comes before the `let` (top-level asserts are emitted in body order; the `let`,
  forgets and cover come after the body loop).
- The condition is evaluated at the same point as before: after every assertion and probe. Only
  `forget` calls sit between the read and the cover, and they change neither control flow nor any
  value the condition reads. So the cover's reachability-with-condition is unchanged, and the
  `bool` carries no fat value.
- The rendered r2 crates (scratchpad `r2/*/src/lib.rs`) have exactly this tail. Their first 236
  lines are byte-identical to the round-1 crates.

Gates, one batched job under the cargo lock: fmt clean, clippy `-D warnings` clean,
`--lib kani::generate` 87 passed, `--test it` with the focused filters plus `cover_last`, `qsl_`
and `layout` 273 passed, 22 ignored.

Reviewer mutants on the pinned-text tests:

- M7 restores the old order (inline cover, then the forgets). It fails five tests:
  `cover_last::tc_025_every_emitted_harness_ends_with_exactly_one_cover`, the native and general
  `*_before_the_final_cover` tests, `qsl_state_invariant_is_a_separate_v2_assertion_obligation`
  and `qsl_constant_invariants_need_no_symbolic_input_or_subject`.
- M8 puts the forgets before the cover-condition read. It fails
  `native_clause_harness_forgets_fat_values_before_the_final_cover` and
  `general_clause_harness_forgets_the_outcome_and_the_meter_before_the_final_cover`.

Both reverted. No new findings; FND-001 to FND-005 stay fixed.
