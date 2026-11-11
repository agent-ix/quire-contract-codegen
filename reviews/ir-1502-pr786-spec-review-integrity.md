---
id: SR-17940486709564847047676
title: "IR-1502 PR #786 FR-014 IEEE sqrt and fma spec integrity review"
type: SpecReview
analysis: integrity
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

Ticket: IR-1502. This review checks that the FR-014 Behavior bullet, the derivable-list prose and AC-72..74 are consistent with each other, with the rest of FR-014 and with the source they delegate to: "exactly as for IEEE add". Sources read without changes: CG `src/oracle/scalar/derive.rs` (Family::IeeeArithmetic arm at 601-609, `family_of_identity`), `lower.rs` (Shape::of IeeeArithmetic is `binary`; generation check `BodyMismatch` on `body_operator`), `render.rs` (IEEE add call), and FR-014-AC-67 (IEEE comparisons accept the QSL `expression`/`call` form with the catalogued `call` body operator). Also read: the operation catalog quire-verification-contracts 1bf72e6, where sqrt and fma are `operator: call` and add is `binary`, and quire-exact 5db2d5e `ieee.rs`.

## Verdict

FAIL (not mergeable). The rule that delegates to IEEE add contradicts itself. It also leaves out the one difference an implementer must get right for any genuine node to be accepted. The descriptor's internal shape can stay open: the implementer may add new `IeeeArithmeticOperator` variants or a separate operation, since `IeeeArithmetic` is binary-only today. The observable acceptance shape, however, has to be pinned by an AC.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The Behavior bullet says sqrt/fma derive 'exactly as for IEEE `add`' and then 'any other operand count, operand form or width is `operand_forms_not_derivable`'. IEEE add derivation has no operand count/form/width check at all (derive.rs:601-609 reads `float_rounding` then `require_mode`); those defects surface only in generation as `BodyMismatch`/`OperandTypeMismatch`/`ResultTypeMismatch`. The bullet therefore states two incompatible rules; an implementer following 'exactly as add' derives a wrong-width node, one following the second sentence adds a new derivation check. Decide which and state it once. | spec/oracle/functional/FR-014-exact-scalar-oracles.md:351-363 |
| FND-002 | medium | 'Exactly as for IEEE add, with these differences' omits that the catalog gives sqrt and fma the `call` operator, whereas add is `binary`. QSL emits them as the `quire::value::ieee::sqrt`/`fma` intrinsic calls, the same case FR-014-AC-67 handles for the comparisons. If add's binary shape is reused, every genuine node is refused with `BodyMismatch`/`FormMismatch`. The descriptor variant shape can stay with the implementer. AC-72, however, must pin the accepted semantic form and the `call` body operator, and should require genuine QSL-emitted source as AC-67/AC-69 do. | spec/oracle/functional/FR-014-exact-scalar-oracles.md:725 |
| FND-003 | low | The prose list of catalogued identities refused as `OperationNotDerivable` omits `ieee.float32.negate` and `ieee.float64.negate`. Both are catalogued, both are refused (no `family_of_identity` arm), and AC-74 now names them as remaining refused. Add them to the list so that the prose and AC-74 agree. | spec/oracle/functional/FR-014-exact-scalar-oracles.md:297-300 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-014 Behavior sqrt/fma bullet | examined | spec/oracle/functional/FR-014-exact-scalar-oracles.md:351-363 | the generator shall derive the descriptor and generate the oracle exactly as for IEEE `add`, with these differences. `sqrt` takes one operand and `fma` takes three, and every operand and the result shall have the identity's float width; any other operand count, operand form or width is `operand_forms_not_derivable`. The rounding bound, mode and result-bound rules are those of IEEE `add`. ... |
| FR-014-AC-72 | examined | spec/oracle/functional/FR-014-exact-scalar-oracles.md:725 | When a strict-reader-admitted application node carries `operation.identity` `quire.op.ieee.float32.sqrt` ... then `derive_exact_scalar_items` shall return an item for it and its descriptor shall name the identity's operation and width, and `generate_exact_scalar_oracles` shall mark its operation `ir_confirmed`. ... |
| FR-014-AC-73 | examined | spec/oracle/functional/FR-014-exact-scalar-oracles.md:726 | The generated oracle ... shall call `evaluate_ieee` with `IeeeOperation::SquareRoot(a)` or `IeeeOperation::FusedMultiplyAdd(a, b, c)` in argument order under the descriptor's rounding and the caller's `Meter` ... |
| FR-014-AC-74 | examined | spec/oracle/functional/FR-014-exact-scalar-oracles.md:727 | ... `quire.op.ieee.from_exact`, `quire.op.ieee.to_rational`, `quire.op.ieee.float32.negate` and `quire.op.ieee.float64.negate` remain `OperationNotDerivable`. |
| FR-014 derivable-list prose | examined | spec/oracle/functional/FR-014-exact-scalar-oracles.md:289-300 | The catalogued identities refused this way are `rational.eq`, `rational.ne`, `decimal.eq`, `decimal.ne`, `integer.rem`, `ieee.from_exact`, `ieee.to_rational`, `numeric.narrow`, `rational.narrow` and `text.size` |
| FR-014-AC-67 | context_only | spec/oracle/functional/FR-014-exact-scalar-oracles.md:719 | For strict-reader-admitted `quire.op.ieee.numeric_equal`, ... CG accepts the genuine QSL-emitted `expression`/`call` semantic form with the catalogued `call` body operator ... |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | verified at be0daf1b |
| FND-002 | fixed | verified at be0daf1b |
| FND-003 | fixed | verified at be0daf1b |
