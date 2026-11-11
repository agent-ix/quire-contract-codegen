---
id: SR-17940486709564847047695
title: "IR-1502 PR #797 code-review (rust-review lane)"
type: SpecReview
analysis: code-review
date: 2026-11-11
scope: "agent-ix/quire-contract-codegen PR #797 at 8c0b9508 (base main 6e62186e); IR-1502; src/oracle/scalar/{mod,derive,lower,render}.rs, src/oracle/scalar/derive/tests.rs, tests/it/ieee_sqrt_fma.rs, tests/it/scalar_source_forms.rs, tests/it/main.rs, tests/exact_scalar_support/package.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-024
    type: reviews
---

## Summary

Ticket: IR-1502. The review covers PR #797, one commit 8c0b9508 on main 6e62186e. It was done read-only in a detached worktree, scoped to `git diff origin/main...HEAD` (17 files, +914/-68). The rust-review lane is folded in here.

What the code does:
- `IeeeArithmeticOperator` gains `SquareRoot` and `FusedMultiplyAdd`.
- `family_of_identity` maps `quire.op.ieee.float32/64.sqrt` and `.fma` into `Family::IeeeArithmetic`. That reuses IEEE add's `float_rounding()` then `require_mode()` derivation (derive.rs:615-622) and add's `float_rounding` bound check (lower.rs:1012).
- The new `Shape::call` has form `call`, body operator `call` and 1 or 3 operands. Its result type is the identity's width. Add's `binary` shape is not reused.
- `catalogued_operation` and `operation_identity` get the four arms.
- render.rs renders `rt::IeeeOperation::SquareRoot(operand)` and `FusedMultiplyAdd(a, b, c)` in argument order under `rounding_path(descriptor rounding)`. Each operand has an `expect_width` prelude line, and the prelude runs before the `evaluate_ieee` call, so `OracleStop::OperandWidth` precedes any charge.
- The add, sub, mul and div rendering is byte-identical to main: same prelude strings, same call string.

Rust surface:
- There is no new `unwrap`, `expect`, `panic!` or `unsafe` in non-test src.
- `cargo fmt --check` is clean.
- Every new test uses `#[trace(...)]`, and no `/// Trace:` line was added.
- There is no new public path beyond the two enum variants, which are additive on the existing `pub enum`.
- No pin, hash, digest or Cargo.lock change.
- `scalar_source_forms` helpers became `pub(crate)` inside the `it` binary only.
- The layout tests pass: `l_1`, `l_2` (no crate-root imports, dependency direction, acyclic, no re-export outside the root) and the witness and qsl_replay scans.

Reviewer run: one focused job under the cargo slot, `cargo test --locked --lib --test it -- ieee_sqrt_fma layout scalar::derive`. Lib: 12 passed. it: 13 passed (5 ieee_sqrt_fma, 8 layout).

Independent IEEE reference: a throwaway rustc program using Rust `f32`/`f64` `sqrt` and `mul_add`, plus exact u128 squaring for the binary64 directed modes. It reproduced every TC-024 literal:
- binary32 sqrt(2): 0x3FB504F3 under RNE. That value lies below the true root, so toward-positive is 0x3FB504F4.
- sqrt(4.0) = 0x40000000 at both widths.
- binary64 sqrt(2) under RNE = 0x3FF6A09E667F3BCD. Its square exceeds 2, and 0x...BCC squared is below 2, so toward-negative and toward-zero are 0x...BCC.
- fma32(0x3F800001, 0x3F800001, 0xBF800002) = 0x28800000. The non-fused result is 0x0.
- fma64 analog = 0x3970000000000000. The non-fused result is 0x0.
- fma(2, 3, 1) = 7 at both widths. Swapping (a, c) or (b, c) gives 5, and at binary32 (a, c) on the single-rounding vector gives 0xB4800001.
- sqrt(-1) is NaN (invalid). sqrt(qNaN 0x7FC00001) keeps payload 0x7FC00001. Both are owner-agreement-only vectors, as TC-024 states.

Reviewer mutation controls, each run against the focused tests:
- M1, fma rendered as `FusedMultiplyAdd(c, b, a)`: FAILS at vector 9 (Float32 fma, single-rounding vector).
- M2, the sqrt oracle ignores the descriptor rounding (always NearestEven): FAILS at vector 1 (Float32 toward-positive 0x3FB504F4).
- M3, `quire.op.ieee.float64.sqrt` derives width Binary32: FAILS `tc_024_the_identity_inverse_matches_the_forward_catalogue_for_every_descriptor` and `tc_024_sqrt_and_fma_refuse_mode_and_bound_defects_like_add`.

The coder's mutants (a)-(f) were not re-run.

## Verdict

Code: correct, and mergeable as far as this method is concerned.
- The fma operand order is a, b, c, as the owner documents (`a * b + c`, one rounding).
- The width check precedes any charge for every operand position, and the test asserts both no admitted charge and all counters zero.
- The vectors are IEEE-correct.

One low test-quality nit follows. The spec and evidence gaps are in SR-17940486709564847047696 (gap-analysis) and SR-17940486709564847047697 (spec-review/base).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | In `defensive_seam_refuses_other_shapes_for_sqrt_and_fma`, the comment says a `call` node of the wrong result type (the Boolean `totalOrder` node) is a `ResultTypeMismatch`, but the assertion accepts `BodyMismatch` or `ResultTypeMismatch`. The node has 2 call arguments against 1 or 3, so generation always returns `BodyMismatch` (the arity check runs before the result check, lower.rs:166-180). The comment is wrong, and the disjunction hides which seam fires. The fixture function holding the sqrt call is named `neg32`. Assert `BodyMismatch { expected_operator: "call", .. }` exactly, fix the comment, and rename the fixture. | tests/it/ieee_sqrt_fma.rs:235, 261-270 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| src/oracle/scalar/derive.rs | examined | src/oracle/scalar/derive.rs:259-272, 615-622 | four identity arms into Family::IeeeArithmetic; float_rounding then require_mode |
| src/oracle/scalar/lower.rs | examined | src/oracle/scalar/lower.rs:1595-1606, 1688-1707, 1921-1932, 2214-2215 | Shape::call; sqrt/fma shapes; catalogued_operation and operation_identity arms |
| src/oracle/scalar/render.rs | examined | src/oracle/scalar/render.rs:459-490 | per-operator parameters, expect_width prelude per operand, SquareRoot(operand), FusedMultiplyAdd(a, b, c) |
| src/oracle/scalar/mod.rs | examined | src/oracle/scalar/mod.rs:195-211 | IeeeArithmeticOperator SquareRoot, FusedMultiplyAdd |
| src/oracle/scalar/derive/tests.rs | examined | src/oracle/scalar/derive/tests.rs:213, 519-540, 664-874 | round trip, refused list, AC-78 seam test, AC-79 operand test |
| tests/it/ieee_sqrt_fma.rs | examined | tests/it/ieee_sqrt_fma.rs:1-541 | five tests incl. native agreement and width stops |
| tests/it/scalar_source_forms.rs | examined | tests/it/scalar_source_forms.rs:19-231 | helpers pub(crate) |
| tests/exact_scalar_support/package.rs | examined | tests/exact_scalar_support/package.rs:2438-2463 | identity arms, call operator |
| src/routed/capability.rs | context_only | src/routed/capability.rs:612 | IeeeArithmetic maps to KaniFormFamily::IeeeFloat |
