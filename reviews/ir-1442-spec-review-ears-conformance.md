---
id: SR-17940486709564847047602
title: "IR-1442 PR #769 FR-053 EARS conformance analysis"
type: SpecReview
analysis: ears-conformance
date: 2026-11-10
scope: "agent-ix/quire-contract-codegen; PR #769 @ 098be30e; IR-1442; spec/kani/functional/FR-053-float-scalar-harnesses.md, spec/kani/matrix/TC-067-float-scalar-harnesses.md, spec/kani/functional/FR-025-generated-subject-abi.md, spec/kani/functional/FR-028-bounded-proof-ceilings.md, spec/kani/matrix/TC-036-generated-subject-abi.md, spec/routed/matrix/TC-030-capability-settlement.md, spec/kani/matrix/tests.md, spec/spec.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-053
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-067
    type: reviews
---

## Summary

Ticket: IR-1442. Spec-only PR #769 (head 098be30e, one commit on origin/main 24a73c74) adding FR-053 and TC-067, the planned Kani row for the three IEEE comparisons and float32/float64 add, sub, mul and div (sqrt and fma conditional on P3), with edits to FR-025 (float ABI row, AC-7, new AC-17), FR-028, TC-036 step 17, TC-030 IEEE-float row, spec/kani/matrix/tests.md and spec/spec.md. Reviewed against quire-exact 5db2d5e3 (CG Cargo.lock pin) src/ieee.rs and src/accounting{,/units}.rs, CG src/oracle/scalar/{derive,derive/tests}.rs and src/kani/generate/negotiate.rs, Contract IR origin/main 7add9a5 (lower.rs requires_bound, FR-038-AC-8), QSpec proposals/checked-package-v2/schema.json operation catalogue, qsl-replay 434869e WitnessValue, Linear IR-69 (planner ruling comment, Option A), IR-1446, IR-1502.

## Verdict

CLEAN. The requirement statement (When an admitted request selects ..., CG SHALL generate ...) and each Behavior SHALL (When the operation is representable / When evaluating / When asserting / When expanding / When decoding / When submitting) are well-formed event-driven EARS sentences with one subject and one response; the refusal paragraph names its conditions and dispositions. quire validate reports no EARS diagnostic for FR-053.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-053 | examined | spec/kani/functional/FR-053-float-scalar-harnesses.md:43 | When an admitted request selects a reader-admitted IEEE float scalar operation over parameters typed `float32` or `float64`, and the planned prerequisites below are available, CG SHALL generate a bounded Kani harness that proves the generated IEEE oracle of [FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md) agrees with an independent native expression of the same operation over every symbolic operand bit pattern. The operations are the comparisons `quire.op.ieee.numeric_equal`, `.total_order` and `.bit_identical`, the arithmetic `quire.op.ieee.float32.add`, `.sub`, `.mul`, `.d... |
| FR-053-behavior-domain | examined | spec/kani/functional/FR-053-float-scalar-harnesses.md:145 | When the operation is representable, the harness SHALL draw each operand as one `i64` bit pattern. A `float32` operand is constrained to `0..=0xFFFF_FFFF` and built as `IeeeValue::binary32(bits as u32)`. A `float64` operand is the same `i64` reinterpreted as `u64` and built as `IeeeValue::binary64(bits as u64)`, so a pattern with the sign bit set is a negative `i64`. Nothing else constrains the draw. Every pattern is an owner value: both signs of zero, every subnormal, every normal, both infinities, and every quiet and signaling NaN of every sign and payload. Each pattern is reachable exact... |
| FR-053-behavior-evaluation | examined | spec/kani/functional/FR-053-float-scalar-harnesses.md:160 | When evaluating, the harness SHALL compare the FR-014 generated oracle with a native expression that is independent of that oracle (not a second call of the same function), from matching entry Meter state and limits. For the comparisons the comparison covers the `Outcome<bool>`, ordered charges, the denied next charge and consumed counters. For the arithmetic operations it covers the typed outcome, the completed result's bit pattern, its flag set (`invalid`, `divide_by_zero`, `overflow`, `underflow`, `inexact`) and its provenance (width and rounding mode), ordered charges, the denied next c... |
| FR-053-behavior-outcomes | examined | spec/kani/functional/FR-053-float-scalar-harnesses.md:244 | The typed outcomes the owner produces are these. `Completed` with the Boolean for a comparison, or with the result bits, flags and provenance for arithmetic. A division of a finite nonzero by a zero completes with a signed infinity and `divide_by_zero`, and an invalid operation completes with the canonical quiet NaN and `invalid`; neither is `Undefined`. A NaN operand completes with the leftmost NaN operand's sign and payload, quieted, and `invalid` raised when any NaN operand is signaling. `Incomplete` at `ieee.operands`, `ieee.exact-intermediate`, `ieee.round` or `ieee.result-retain`, wit... |
| FR-053-behavior-refusals | examined | spec/kani/functional/FR-053-float-scalar-harnesses.md:265 | When the carrier, the oracle (for `sqrt` and `fma`), a float operand form, the `float_rounding` result bound (arithmetic only) or the native expression is unavailable, the item is refused with the dispositions of AC-1, AC-3 and AC-6 and no harness. A float leaf inside a composite remains FR-015's `ShadowFamilyNotBuilt`. |
| FR-053-abi | examined | spec/kani/functional/FR-053-float-scalar-harnesses.md:272 | When expanding a logical float operand, CG SHALL persist exactly one physical `i64` binding, the bit pattern, for its one real parameter, grouped to the original parameter identity, role, width and any `float_rounding` pin. Logical parameter order remains ascending by parameter identifier; Boolean and integer ordering is unchanged. The carrier (P1) owns the exact binding facts; these are required conceptual facts, not a claim that a current Rust field carries them.  When decoding retained playback, CG SHALL validate each physical entry in persisted call order with the existing I64 width, li... |
