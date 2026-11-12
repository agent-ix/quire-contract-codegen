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
