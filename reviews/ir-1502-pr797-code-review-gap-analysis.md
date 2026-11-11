---
id: SR-17940486709564847047696
title: "IR-1502 PR #797 gap-analysis"
type: SpecReview
analysis: gap-analysis
date: 2026-11-11
scope: "agent-ix/quire-contract-codegen PR #797 at 8c0b9508 (base main 6e62186e); IR-1502; FR-014-AC-72..80, TC-024 IR-1502 section, tests/it/ieee_sqrt_fma.rs, src/oracle/scalar/derive/tests.rs, tests/exact_scalar_support/package.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-024
    type: reviews
---

## Summary

Ticket: IR-1502. This is a planless gap-analysis: Plan completion: not assessed. It runs over FR-014-AC-72..80 and the TC-024 IR-1502 section against the tagged tests and the src change.

`quire matrix` lists AC-72..80 as `tagged`:

| AC | Tagged test(s) |
| --- | --- |
| AC-72 | ieee_sqrt_fma.rs:95 |
| AC-73 | ieee_sqrt_fma.rs:95 |
| AC-74 | derive/tests.rs:215, ieee_sqrt_fma.rs:95 and :142 |
| AC-75 | ieee_sqrt_fma.rs:484 |
| AC-76 | ieee_sqrt_fma.rs:484 |
| AC-77 | ieee_sqrt_fma.rs:484 |
| AC-78 | derive/tests.rs:670 |
| AC-79 | derive/tests.rs:804, ieee_sqrt_fma.rs:142 and :231 |
| AC-80 | derive/tests.rs:526, ieee_sqrt_fma.rs:291 |

No stub or tautology was found. Each tagged test fails under a matching mutant. The reviewer ran M1 (fma operand order), M2 (sqrt ignores the rounding) and M3 (cross-width derivation), all detailed in SR-17940486709564847047695.

Rulings on the coder's three spec-gap notes:

1. AC-78 driven at the private derivation seam: ACCEPTABLE evidence.
   - IEEE add's mode refusal is tested the same way, through `Derivation::require_mode` at the seam (`tc_024_derivation_requires_the_mode_to_agree_with_the_bound_rounding`).
   - The sqrt/fma test is stronger. It runs the whole private `derive_operation` with bound nodes and asserts equality with IEEE add's result for missing, ambiguous and unreadable bounds.
   - The test does not drive the `IncompleteContext` refusal that AC-78 names, nor the "no oracle, every other item unchanged" clause. Both are operation-independent paths (the record context, and a refused derivation yields no item), so this is a low gap (FND-003), not an overclaim.
2. AC-72's "reader-admitted fixture with that call operator" covered by genuine QSL packages plus the package.rs identity arms: NOT ACCEPTABLE as evidence for that clause.
   - The package.rs arms exist only because the match must be exhaustive. No corpus or fixture builds a `SquareRoot` or `FusedMultiplyAdd` expression; a grep of tests/ for those operators finds only that match and ieee_sqrt_fma.rs.
   - So the fixture clause of AC-72 and TC-024 step 1 has no test, while tests.md marks the row "Delivered".
   - The genuine QSL path is the stronger evidence, so the right fix is a spec correction in this PR: drop the fixture clause from AC-72 and TC-024 step 1. The alternative is to add one non-corpus fixture test (FND-001).
3. AC-79 "wrong body operator" and "non-float operand" not reader-admittable: PARTLY ACCEPTABLE.
   - Non-float and other-width operands are covered by `tc_024_sqrt_and_fma_operands_are_checked_for_float_type_and_width`. It calls the free `check_operand` with `Shape::of(..).operands[position]`, and `Shape::check_operand` delegates to it unchanged for a non-carried shape. That is acceptable.
   - Wrong form (FormMismatch over add's binary node), wrong arity (BodyMismatch) and wrong result width (ResultTypeMismatch) are covered.
   - The wrong-body-operator case is a separate guard: `application_arguments`' `operator.as_wire() == expected` (lower.rs:336). It returns the same `BodyMismatch` variant, but no sqrt/fma test reaches it, so a mutant that drops the operator guard survives for these shapes. A one-node seam test with `v2_node` would cover it, or AC-79 could stop listing it separately (FND-002).

## Verdict

NOT MERGEABLE as is, on one medium finding. FR-014-AC-72's fixture clause is claimed Delivered with no test. The fix is a one-sentence spec correction in this PR, or one fixture test. Everything else in AC-72..80 has a genuine test that fails under the relevant mutant. The low findings can land in the same fix round.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-014-AC-72 ends 'and also accepting the same operations in a reader-admitted fixture with that call operator', and TC-024 step 1 says 'also derive reader-admitted fixtures with the catalogued call operator'. No test does this: the package.rs SquareRoot/FusedMultiplyAdd arms are exhaustiveness-only and no corpus or fixture expression uses them. Yet spec/oracle/matrix/tests.md marks AC-72..80 '✅ Delivered'. Drop the clause from AC-72 and TC-024 step 1 (the genuine QSL path is the stronger evidence), or add one non-corpus reader-admitted fixture test tagged FR-014-AC-72. | spec/oracle/functional/FR-014-exact-scalar-oracles.md:729, spec/oracle/matrix/TC-024-exact-scalar-oracles.md:542-546, tests/exact_scalar_support/package.rs:2438-2463 |
| FND-002 | low | FR-014-AC-79 and TC-024 step 4 list a 'wrong body operator' refusal for sqrt/fma descriptors. That is the operator guard in `application_arguments` (lower.rs:336), separate from the arity filter, and no sqrt/fma test reaches it. The tagged BodyMismatch cases are arity-only, so a mutant that ignores the body operator survives. Add a seam test with a `call`-form node whose body operator is not `call`, or remove the clause from AC-79/TC-024. | spec/oracle/functional/FR-014-exact-scalar-oracles.md:736, src/oracle/scalar/lower.rs:327-339, tests/it/ieee_sqrt_fma.rs:142-197 |
| FND-003 | low | FR-014-AC-78 names `IncompleteContext` among the add-equivalent refusals and requires 'generation emits no oracle for either item and leaves every other item unchanged'. `tc_024_sqrt_and_fma_refuse_mode_and_bound_defects_like_add` covers missing, ambiguous and unreadable bounds and the mode mismatch only, and never runs generation or a sibling. Both omitted clauses are operation-independent, so either drop `IncompleteContext` and the generation clause from the sqrt/fma AC (they belong to the generic FR-014 ACs) or add those two assertions. | spec/oracle/functional/FR-014-exact-scalar-oracles.md:735, src/oracle/scalar/derive/tests.rs:664-790 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-014-AC-72 | examined | spec/oracle/functional/FR-014-exact-scalar-oracles.md | genuine QSL sqrt/fma calls derive; also reader-admitted fixture with that call operator |
| FR-014-AC-73 | examined | spec/oracle/functional/FR-014-exact-scalar-oracles.md | generation marks ir_confirmed with identity, laws and mode |
| FR-014-AC-74 | examined | spec/oracle/functional/FR-014-exact-scalar-oracles.md | identity to descriptor to identity; no cross operation or width |
| FR-014-AC-75 | examined | spec/oracle/functional/FR-014-exact-scalar-oracles.md | value and flags of evaluate_ieee SquareRoot / FusedMultiplyAdd(a, b, c) |
| FR-014-AC-76 | examined | spec/oracle/functional/FR-014-exact-scalar-oracles.md | ordered charges, counters, stops at every denial boundary |
| FR-014-AC-77 | examined | spec/oracle/functional/FR-014-exact-scalar-oracles.md | other width stops as OperandWidth before any charge |
| FR-014-AC-78 | examined | spec/oracle/functional/FR-014-exact-scalar-oracles.md | mode_not_derivable; Missing/Ambiguous/IncompleteContext/Unreadable as add |
| FR-014-AC-79 | examined | spec/oracle/functional/FR-014-exact-scalar-oracles.md | defensive seam FormMismatch, BodyMismatch, OperandTypeMismatch, ResultTypeMismatch |
| FR-014-AC-80 | examined | spec/oracle/functional/FR-014-exact-scalar-oracles.md | from_exact, to_rational, both negate stay OperationNotDerivable |
| TC-024 IR-1502 section | examined | spec/oracle/matrix/TC-024-exact-scalar-oracles.md:535-602 | steps 1-6 and mutation controls (a)-(f) |
