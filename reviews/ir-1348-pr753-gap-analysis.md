---
id: SR-2026110283888
title: "IR-1348 PR 753 scalar quantity trace gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; IR-1348; PR 753; FR-014-AC-48,58-61; tests/exact_scalar_support; tests/it/exact_scalar_{agreement,generation}.rs; spec/oracle/matrix/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: reviews
---

## Summary

Compared the PR's changed trace tags, generated-native quantity assertions, the computed Quire matrix, and the FR-014 status row at head 7107aa912872fc4dececb3be06158b27c43363b8. The executable positive behavior is exercised, but several trace claims and one status row do not describe the evidence truthfully.

## Verdict

**CONDITIONAL for the PR subset.** Correct the trace bindings, add the missing bare quantity refusal control and full-U12 binding, and update the directly contradicted status row before merge.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The U22 compound-source test claims AC-61, which specifies U25's compound target; bind this test to AC-48. | tests/exact_scalar_support/compound_conversion_agreement_cases.rs:89 |
| FND-002 | medium | The U25 compound-target test claims AC-60's wrong-result/bare-parameter criterion; bind this test to AC-61. | tests/exact_scalar_support/compound_conversion_agreement_cases.rs:124 |
| FND-003 | medium | The admitted compound wrapper claims AC-60 but executes only positive generation and agreement, not either AC-60 refusal; remove that unsupported tag. | tests/it/exact_scalar_agreement.rs:633 |
| FND-004 | medium | The computed matrix binds AC-58 only to the new square test. The existing generated-native U12 test asserts all four product, quotient, square and cancel values, but lacks the AC-58 trace tag, so the criterion's four-node coverage is not credited. | tests/exact_scalar_support/agreement_cases.rs:872; spec/oracle/functional/FR-014-exact-scalar-oracles.md:667 |
| FND-005 | medium | AC-60's bare quantity parameter `RequiresBound` and no-oracle clause has no CG test; its only valid tag checks the unrelated Integer result through a private seam. Add an admitted bare-quantity parameter control that asserts the typed refusal and no generated claim. | spec/oracle/functional/FR-014-exact-scalar-oracles.md:669; src/oracle/scalar/lower/tests.rs:914 |
| FND-006 | medium | The FR-014-AC-6 status row still says a compound target has a refusal control and U25's first compound-target step is uncovered, although this PR now generates and executes that step. Keep QSL value-authority agreement unclaimed, but update this local status. | spec/oracle/matrix/tests.md:18 |

## Coverage

Plan completion: not assessed. The Quire computed matrix was read for FR-014-AC-58 through AC-61. This is a PR-diff subset assessment; no optional repository-wide semantic sweep or Cargo gate was run by the reviewer.

### Examined units

- FR-014-AC-48 (examined): an admitted exact conversion to a `unit` or `compound_unit` target derives and generates an oracle, retaining target type identity and matching direct QSV conversion.
- FR-014-AC-58 (examined): four U12 nodes must generate, execute to magnitudes 6/3/4/1, and retain exact compound terms and QSV charge agreement.
- FR-014-AC-59 (examined): a completed compound product must be formable in `UnitScope` and reusable to produce exact `12 m^2*s`.
- FR-014-AC-60 (examined): an unrelated Integer result refuses without a symbol, and a bare quantity parameter remains `RequiresBound` without a CG oracle.
- FR-014-AC-61 (examined): U25's compound target first step derives and generates, retains type identity and value, then converts to J; scalar target private controls still refuse.

## Dispositions

Round 1 checked PR head `467f3332eb50234a75a6b4f49fa508c7d0ddc14a`.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | U22 compound-source test now traces AC-48. |
| FND-002 | fixed | U25 compound-target test now traces AC-61. |
| FND-003 | fixed | Positive admitted compound wrapper no longer claims AC-60. |
| FND-004 | fixed | Four-case U12 generated/native test now traces AC-58. |
| FND-005 | still-open | New control selects a bare parameter as the oracle node and gives it a Multiply descriptor; no admitted multiply expression uses that parameter. It could pass even if a multiply over a bare quantity parameter incorrectly generated. |
| FND-006 | fixed | Local matrix row now credits both U25 steps and the measured conversion coverage. |

Round 2 checked PR head `79e163bf47775626b8c53db817f9010fad6aaad9`.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-005 | fixed | Reader-admitted `quantity.mul` expression 9909 uses bare compound-typed parameter 9908, and the test requests expression 9909; it asserts the compound type's exact `RequiresBound` refusal and absence of the expression symbol. |
