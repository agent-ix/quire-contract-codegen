---
id: SR-17940486709564847047603
title: "IR-1442 PR #769 FR-053 dependency analysis"
type: SpecReview
analysis: dependency
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

CLEAN for edges; prerequisite correctness is recorded under base (IR-1446 acceptance) and integrity (IR-1502). Frontmatter: satisfies StR-001; depends_on FR-014/015/016/017/025/028/029/039; references Contract IR FR-038, FR-046/049/050/051/052 and QSpec FR-196, matching the sibling rows' edge set. Enablement is separated from feature work: P1 (IR-1446) gates every supported harness, P2 (IR-69, Option A ruled 2026-11-10, not implemented on Contract IR 7add9a5) gates only the upstream half of AC-3, P3 gates only the sqrt/fma parts; the IR-1436 Meter ordering (IR-844, IR-241) is stated as context, not a criterion.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-053-frontmatter | examined | spec/kani/functional/FR-053-float-scalar-harnesses.md:5 | relationships: - target: ix://agent-ix/quire-contract-codegen/StR-001 type: satisfies - target: ix://agent-ix/quire-contract-codegen/FR-014 type: depends_on - target: ix://agent-ix/quire-contract-codegen/FR-015 type: depends_on - target: ix://agent-ix/quire-contract-codegen/FR-016 type: depends_on - target: ix://agent-ix/quire-contract-codegen/FR-017 type: depends_on - target: ix://agent-ix/quire-contract-codegen/FR-025 type: depends_on - target: ix://agent-ix/quire-contract-codegen/FR-028 type: depends_on - target: ix://agent-ix/quire-contract-codegen/FR-029 type: depends_on - target: ix:/... |
| FR-053-prerequisite-P1 | examined | spec/kani/functional/FR-053-float-scalar-harnesses.md:61 | - P1 (ticket IR-1446) is the quire-exact float carrier: a symbolic IEEE operand, and a native expression for the comparisons and for add, sub, mul, div, sqrt and fma at `binary32` and `binary64` under each of the six rounding modes, with the tractability contract of the IR-1436 design review (D4): `Copy`, no allocation, machine width, no iterator loop, and none of the owner's `BigInt` or `BigUint` arithmetic. Its carrier also supplies a host-callable replacement of the owner's `evaluate_ieee` and `compare_ieee`, which AC-13 needs; IR-1446's acceptance carries this requirement. |
| FR-053-prerequisite-P2 | examined | spec/kani/functional/FR-053-float-scalar-harnesses.md:68 | - P2 (ticket IR-69) is the Contract IR ruling that a `float32` or `float64` is unbounded unless a `float_rounding` domain applies (planner ruling, Option A). It adds both forms to the `requires_bound` unbounded set in `crates/quire-contract-model/src/checked_package/v2/lower.rs` and moves FR-038 Behavior and FR-038-AC-8 from eight forms to ten. It is not implemented: that predicate returns `false` for `ScalarTypeForm::Float32` and `ScalarTypeForm::Float64` on Contract IR main at the time of writing, so a package with an unbounded float lowers to `lowered` under a bounds-required profile tod... |
| FR-053-prerequisite-P3 | examined | spec/kani/functional/FR-053-float-scalar-harnesses.md:77 | - P3 (no ticket exists) is the FR-014 extension that derives and generates an oracle for `quire.op.ieee.float32.sqrt`, `.float64.sqrt`, `.float32.fma` and `.float64.fma`. FR-014 refuses these four identities as `OperationNotDerivable` today, so no oracle exists to prove. The owner supports both (`IeeeOperation::SquareRoot`, `IeeeOperation::FusedMultiplyAdd`), and Contract IR catalogues both identities, so the missing piece is CG's descriptor, derivation and renderer. Without P3 the `sqrt` and `fma` items keep FR-014's `OperationNotDerivable` and this row proves nothing about them. |
| FR-053-dependencies | examined | spec/kani/functional/FR-053-float-scalar-harnesses.md:318 | - [FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md) owns the IEEE oracle semantics and the derivable operation list (add, sub, mul, div, the three comparisons and the two width conversions); its oracle agreement is not this proof. It refuses `sqrt` and `fma` as `OperationNotDerivable`, which is why P3 is needed. - The float carrier (IR-1446) is prerequisite P1, the Contract IR float `requires_bound` ruling (IR-69) is P2, and the FR-014 `sqrt` and `fma` derivation (no ticket) is P3. The IR-1436 design review (D4) fixes the carrier contract and gates the memory evidence; its or... |
| TC-030-ieee-row | examined | spec/routed/matrix/TC-030-capability-settlement.md:398 | / IEEE-float scalar `expression` claim, IR-confirmed generated claim / Planned (IR-269) / IEEE-float scalar operation forms; the three comparisons and `float32`/`float64` `add`, `sub`, `mul` and `div` are planned under FR-053 (conditional on the quire-exact float carrier, IR-1446, and for unbounded operands the Contract IR ruling, IR-69; `sqrt` and `fma` also need an FR-014 derivation that has no ticket and stay `OperationNotDerivable` until then), which supersedes this row for them only when delivered. / `OperationNotRendered` (`classify_claim` maps `ScalarLoweringRefusal::NoRenderer`). / |
| tests.md-FR-053-row | examined | spec/kani/matrix/tests.md:13 | / FR-053 / FR-053-AC-1 through FR-053-AC-13 / TC-067 / 🚧 PLANNED (IR-1442): no float scalar Kani harness; needs the quire-exact float carrier (IR-1446) and the Contract IR float `requires_bound` ruling (IR-69), and `sqrt`/`fma` an FR-014 derivation that has no ticket; TC-067 controls have not run. / |
