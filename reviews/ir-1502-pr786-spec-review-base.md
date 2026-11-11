---
id: SR-17940486709564847047675
title: "IR-1502 PR #786 FR-014 IEEE sqrt and fma spec base review"
type: SpecReview
analysis: base
date: 2026-11-11
scope: "agent-ix/quire-contract-codegen; PR #786 @ 06dcdaf1; IR-1502; spec/oracle/functional/FR-014-exact-scalar-oracles.md, spec/oracle/matrix/TC-024-exact-scalar-oracles.md, spec/oracle/matrix/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-024
    type: reviews
---

## Summary

Ticket: IR-1502. This PR changes only the spec (head 06dcdaf1). It was reviewed in a separate detached worktree of that head, scoped to `git diff origin/main...HEAD` (3 files, +63/-3). The owner source was read without changes at the Cargo.lock pin: quire-exact 5db2d5e (`src/ieee.rs`: `IeeeOperation::SquareRoot(O)`, `FusedMultiplyAdd(O, O, O)` documented as `a * b + c` with one rounding, `evaluate_ieee(operation, rounding: RoundingMode, meter)`, arity 1 and 3 charged by `charge_operands`). Also read: Contract IR c8dca8c (strict reader, `operator-ineligible` arity and operand-family checks, `check_ieee_fma_result`), and the operation catalog quire-verification-contracts 1bf72e6. The catalog lists sqrt as `call` [float] -> float and fma as `call` [float, float, float] -> float, both with laws `ieee_profile` and mode `rounding`, the same as add except that add is `binary`. CG's IEEE add derivation and generation were read in `src/oracle/scalar/derive.rs` and `lower.rs`.

Every literal was recomputed with exact rationals in Python, without cargo:
- binary32 `sqrt(0x40000000)` is 0x3FB504F3 under nearest-even and 0x3FB504F4 under toward-positive.
- `sqrt(0x40800000)` is 0x40000000.
- `fma(0x3F800001, 0x3F800001, 0xBF800002)` is exactly 2^-46 = 0x28800000. The rounded product is 0x3F800002, and adding the addend then gives exactly 0.
- `fma(2, 3, 1)` is 0x40E00000. Both swapped forms, fma(2, 1, 3) and fma(1, 3, 2), give 5.

All PR literals are correct.

`make spec` and `quire matrix` report the same 6 warnings and give the same output on origin/main and on the head. That is because the new criteria are invisible to both tools (FND-001). AC ids 72..74 are free: no open PR adds an FR-014 AC row. The diff adds no memory-fit claims, hashes or pins.

## Verdict

FAIL (not mergeable). The direction is right: the four identities exist in the owner and the catalog with the argument order the spec states, and they take the same rounding mode and `ieee_profile` law as add. The vectors and the multiply-then-add and swap controls are correct and can fail at binary32. `from_exact`, `to_rational` and `negate` do stay `OperationNotDerivable`, since `family_of_identity` has no arm for them. Two blockers remain. The new criteria are not parsed at all, and once parsed they fail validation. The refusal claim of AC-74 does not match what IEEE add returns.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-014-AC-72..74 are appended after a blank line (FR-014:722) that splits the AC table, so `quire validate` and `quire matrix` never see them; the matrix lists FR-014 only through AC-69 on both main and head, and the blank line (pre-existing from #757) also hides the tagged AC-70/71. With the blank line removed the matrix lists AC-70..74 and `quire validate` FAILS structurally: `ac:non-singular` on AC-72 and AC-73, plus `ac:non-canonical-shape` warnings on AC-72..74. The green `make spec` is an artifact of the hidden rows. Remove the blank line and split AC-72/AC-73 into singular assertions. | spec/oracle/functional/FR-014-exact-scalar-oracles.md:722-727 |
| FND-002 | high | AC-74 says derivation returns 'the same typed refusal IEEE `add` returns for that defect (`operand_forms_not_derivable`, `mode_not_derivable` or `RequiresBound`)'. Source contradicts both halves: IEEE add derivation (derive.rs:601-609) reads only `float_rounding` and `require_mode`, so it never refuses operand count, width or form (those are generation-side `BodyMismatch`, `OperandTypeMismatch`, `ResultTypeMismatch`, `FormMismatch`); a missing float_rounding bound is `MissingBound` (or `AmbiguousBound`/`IncompleteContext`/`UnreadableBound`), not `RequiresBound`, which is the lowering record's unbounded-operand refusal. Name the actual closed set per defect and per stage (derivation vs generation), or drop the 'same as add' relation. | spec/oracle/functional/FR-014-exact-scalar-oracles.md:727 |
| FND-003 | low | The derivable-list prose (FR-014:292) and AC-72..74 state sqrt/fma derivation as current behavior with no PLANNED marker, while the code still refuses them; FR-014's own precedent (AC-54 'PLANNED (IR-917).') marks undelivered criteria. Only tests.md says planned. Prefix AC-72..74 with 'PLANNED (IR-1502).' and mark the prose list entry planned. | spec/oracle/functional/FR-014-exact-scalar-oracles.md:292 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-014-AC-72 | examined | spec/oracle/functional/FR-014-exact-scalar-oracles.md:725 | When a strict-reader-admitted application node carries `operation.identity` `quire.op.ieee.float32.sqrt` or `quire.op.ieee.float64.sqrt` over one operand, or `quire.op.ieee.float32.fma` or `quire.op.ieee.float64.fma` over three operands, each operand and the result of the identity's width, and a `float_rounding` bound whose rounding equals `operation.mode`, then `derive_exact_scalar_items` shall return an item for it and its descriptor shall name the identity's operation and width, and `generate_exact_scalar_oracles` shall mark its operation `ir_confirmed`. ... |
| FR-014-AC-73 | examined | spec/oracle/functional/FR-014-exact-scalar-oracles.md:726 | The generated oracle for each of the four identities of FR-014-AC-72 shall call `evaluate_ieee` with `IeeeOperation::SquareRoot(a)` or `IeeeOperation::FusedMultiplyAdd(a, b, c)` in argument order under the descriptor's rounding and the caller's `Meter`, and shall return the owner's result, ordered admitted charges, consumed counters and typed stops unchanged. |
| FR-014-AC-74 | examined | spec/oracle/functional/FR-014-exact-scalar-oracles.md:727 | If a `sqrt` or `fma` node of FR-014-AC-72 has the wrong operand count, an operand of a different float width or of a non-float form, a mode unequal to its rounding bound, or no reachable `float_rounding` bound, then derivation shall return the same typed refusal IEEE `add` returns for that defect (`operand_forms_not_derivable`, `mode_not_derivable` or `RequiresBound`), ... |
| FR-014 derivable-list prose | examined | spec/oracle/functional/FR-014-exact-scalar-oracles.md:289-300 | IEEE `float32` and `float64` `add`, `sub`, `mul`, `div`, `sqrt` and `fma`, `numeric_equal`, `total_order`, `bit_identical`, `to_float32` and `to_float64`; ... The catalogued identities refused this way are `rational.eq`, `rational.ne`, `decimal.eq`, `decimal.ne`, `integer.rem`, `ieee.from_exact`, `ieee.to_rational`, `numeric.narrow`, `rational.narrow` and `text.size` |
| FR-014 Behavior sqrt/fma bullet | examined | spec/oracle/functional/FR-014-exact-scalar-oracles.md:351-363 | For `quire.op.ieee.float32.sqrt`, ... the generator shall derive the descriptor and generate the oracle exactly as for IEEE `add`, with these differences. `sqrt` takes one operand and `fma` takes three, ... any other operand count, operand form or width is `operand_forms_not_derivable`. ... |
| TC-024 IR-1502 section | examined | spec/oracle/matrix/TC-024-exact-scalar-oracles.md:535-577 | IEEE square root and fused multiply-add (IR-1502, planned): steps 1-5, vectors, mutation controls (a)-(e) |
| TM FR-014-AC-72..74 row | examined | spec/oracle/matrix/tests.md:24 | FR-014-AC-72 through FR-014-AC-74 / TC-024 / Planned (IR-1502); `ieee.float32/64.sqrt` and `.fma` are `OperationNotDerivable` today; the TC-024 section has not run |
| FR-014-AC-12 | context_only | spec/oracle/functional/FR-014-exact-scalar-oracles.md:665 | A descriptor naming a different catalogued operation is not ir_confirmed |
| FR-014-AC-70 | context_only | spec/oracle/functional/FR-014-exact-scalar-oracles.md:723 | Each generated lazy And, Or and Implies scalar oracle accepts its right operand as a function of the caller's mutable exact Meter. ... |
