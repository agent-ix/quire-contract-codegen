---
id: SR-17940486709564847047677
title: "IR-1502 PR #786 FR-014 IEEE sqrt and fma spec failure-domain review"
type: SpecReview
analysis: failure-domain
date: 2026-11-11
scope: "agent-ix/quire-contract-codegen; PR #786 @ 06dcdaf1; IR-1502; spec/oracle/functional/FR-014-exact-scalar-oracles.md, spec/oracle/matrix/TC-024-exact-scalar-oracles.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-024
    type: reviews
---

## Summary

Ticket: IR-1502. This review covers failure modes and edge cases of the refusal criterion and the agreement vectors. It was checked against the Contract IR strict reader at c8dca8c, `checked_package/v2/operations.rs`. That reader refuses a wrong arity and a wrong operand family as `ill_typed`/`operator-ineligible`, and refuses an fma result of the wrong family through `check_ieee_fma_result`. It was also checked against quire-exact 5db2d5e `ieee.rs`: `plan_square_root` makes a negative input or -INF invalid and a zero input yield an exact signed zero; `plan_fma` makes INF x 0 and opposite infinities invalid; `classify` returns the leftmost NaN. The binary64 values were recomputed with exact rationals. sqrt(2) lies between 0x3FF6A09E667F3BCC (square below 2) and 0x3FF6A09E667F3BCD (square above 2). Nearest-even and toward-positive both give 0x3FF6A09E667F3BCD; toward-negative and toward-zero give 0x3FF6A09E667F3BCC. fma(0x3FF0000000000001, 0x3FF0000000000001, 0xBFF0000000000002) is exactly 2^-104 = 0x3970000000000000.

## Verdict

FAIL ON MEDIUMS. The NaN, invalid and charge-denial cases are delegated to the owner and compared directly, which is sound. Two edge cases are wrong as written. The refusal criterion's shape defects cannot reach CG from an admitted package. The binary64 vectors are left to 'the same construction', and for sqrt that construction does not separate the rounding modes it is supposed to separate.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-74 scopes its defects to 'a `sqrt` or `fma` node of FR-014-AC-72' (strict-reader-admitted), but the Contract IR strict reader refuses wrong arity and a wrong operand family (`float32` operand to a `float64` identity, non-float operand) as `ill_typed`/`operator-ineligible`, and a wrong fma result family, before CG receives the package. Those three defects cannot occur on an admitted node, so TC-024 step 4's wrong-count and wrong-width fixtures cannot be built as admitted packages. Split as FR-014-AC-10/AC-54 do: the reader refusal is tested at the owner boundary, and CG's generation refusals (`BodyMismatch`, `OperandTypeMismatch`, `ResultTypeMismatch`) only at its defensive seam with a descriptor that disagrees with the node. | spec/oracle/functional/FR-014-exact-scalar-oracles.md:727 |
| FND-002 | medium | TC-024 step 2 gives binary64 only as 'the same two shapes at `float64`, with operands chosen by the same construction'. At binary64, sqrt(2) under nearest-even and under toward-positive are both 0x3FF6A09E667F3BCD, so the same construction cannot separate those modes, and control (d) can survive at binary64. Pin the binary64 literals: sqrt(0x4000000000000000) is 0x3FF6A09E667F3BCD under nearest-even and 0x3FF6A09E667F3BCC under toward-negative or toward-zero; fma(0x3FF0000000000001, 0x3FF0000000000001, 0xBFF0000000000002) is 0x3970000000000000, and multiply-then-add gives 0. | spec/oracle/matrix/TC-024-exact-scalar-oracles.md:559 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-014-AC-74 | examined | spec/oracle/functional/FR-014-exact-scalar-oracles.md:727 | If a `sqrt` or `fma` node of FR-014-AC-72 has the wrong operand count, an operand of a different float width or of a non-float form, a mode unequal to its rounding bound, or no reachable `float_rounding` bound, then derivation shall return the same typed refusal IEEE `add` returns for that defect ... |
| FR-014-AC-73 | examined | spec/oracle/functional/FR-014-exact-scalar-oracles.md:726 | ... shall return the owner's result, ordered admitted charges, consumed counters and typed stops unchanged. |
| TC-024 IR-1502 step 2 | examined | spec/oracle/matrix/TC-024-exact-scalar-oracles.md:547-559 | Compare the complete owner outcome, flags, ordered admitted charges and consumed counters against a direct `evaluate_ieee` call, including a charge-denial case at every next-charge boundary. Vectors: ... the same two shapes at `float64`, with operands chosen by the same construction. |
| TC-024 IR-1502 step 4 | examined | spec/oracle/matrix/TC-024-exact-scalar-oracles.md:566-570 | Present wrong operand count, a `float32` operand to a `float64` identity, a mode unequal to the rounding bound, and a missing rounding bound. Assert the typed refusals of FR-014-AC-74 ... |
| FR-014-AC-10 | context_only | spec/oracle/functional/FR-014-exact-scalar-oracles.md:663 | Contract IR's FR-038 "Mode" reader rule also refuses a bare Float32 addition authored with the non-exact `nearest-even` mode and no `float_rounding` type pin ... before CG can classify a missing rounding bound. |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | medium | The closed set AC-79 gives for CG's defensive seam (`BodyMismatch`, `OperandTypeMismatch`, `ResultTypeMismatch`) leaves out `FormMismatch`. Generation checks `Shape::accepts_form` before the body operator (lower.rs:161-167), so a `sqrt`/`fma` descriptor presented against a node with another semantic form, such as add's `binary` node, is refused as `FormMismatch`, not `BodyMismatch`. Add `FormMismatch` to AC-79 and to TC-024 step 4. | spec/oracle/functional/FR-014-exact-scalar-oracles.md:736 |
| FND-004 | low | AC-72 puts the `float_rounding` bound `r` on the parameters ('over parameters typed `Float32[r]`... with a `float_rounding` bound `r` equal to `operation.mode`'). Derivation, however, reads the application's own result bound, and FR-014-AC-21 says a bound owned only by an operand is not a result bound (`MissingBound`). No genuine-source IEEE arithmetic derivation exists in CG yet. State that the bound is the application's result bound (its result type `Float32[r]`/`Float64[r]`, or a narrowing), so the genuine-source fixture is derivable as written. | spec/oracle/functional/FR-014-exact-scalar-oracles.md:729 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | verified at be0daf1b |
| FND-002 | fixed | verified at be0daf1b |
| FND-003 | still-open | New this round: AC-79 closed set omits FormMismatch, which generation reports before BodyMismatch for a node of another semantic form. |
| FND-004 | still-open | New this round: AC-72 places the float_rounding bound on the parameters; derivation reads the result bound (AC-21). |
