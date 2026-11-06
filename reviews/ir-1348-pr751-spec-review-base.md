---
id: SR-2026110283879
title: "IR-1348 PR 751 FR-014 base specification review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; PR 751; spec/oracle/functional/FR-014-exact-scalar-oracles.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: reviews
---

## Summary

QSpec TC-187 U25 requires the compound target conversion and a subsequent conversion to J. The changed FR-014 rule admits the first step only for reader-admitted exact quantity conversion, retains the target node identity for caller-supplied QSV metadata, and keeps scalar-target and nonexact-mode refusals distinct. AC61 names value, unit, charge, counter and sibling controls. No applicable AssuranceProfile was found under `spec/`.

## Verdict

PASS. The consumer-specific requirement is testable, has an independently supplied target identity, and does not copy QSpec's dimension or conversion algebra.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-014 | examined | If a `quire.op.quantity.convert` node's `operation.mode` is `exact` |
| FR-014-AC-6 | examined | The TC-187 vectors that convert a quantity are part of this agreement set through the exact conversion oracle |
| FR-014-AC-48 | examined | both U25 steps and U26 agree under this criterion |
| FR-014-AC-50 | examined | one whose mode is `exact` and whose `type_argument` is a scalar type |
| FR-014-AC-61 | examined | A reader-admitted exact `quire.op.quantity.convert` from `1 N_m` to the `compound_unit` target `kg*m^2*s^-2` |
