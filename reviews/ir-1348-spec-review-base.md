---
id: SR-2026110283876
title: "IR-1348 PR 750 FR-014 base specification review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; PR 750; spec/oracle/functional/FR-014-exact-scalar-oracles.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: reviews
---

## Summary

FR-014's new operand and result rule is specific to admitted quantity operations. QSpec FR-142-AC-6 requires products, quotients and integer powers in canonical compound units, including dimensionless cancellation; FR-146 retains the division nonzero obligation. AC58 covers the four U12 values and unit identities, AC59 covers a compound result reused as an operand, and AC60 preserves a genuine wrong-result refusal and Contract IR's bare-quantity bound policy. The PR does not assert implementation or test coverage. No applicable AssuranceProfile was found under `spec/`.

## Verdict

PASS. The changed requirement is testable, has distinct positive and adverse cases, and does not duplicate QSpec's unit algebra or introduce a hash, digest, pin or record.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-014 | examined | For a reader-admitted quantity operation, the generator SHALL classify a checked `scalar_type`/`unit` or `scalar_type`/`compound_unit` as a quantity operand form when the operation takes a quantity. |
| FR-014-AC-58 | examined | The four reader-admitted TC-187 U12 operations derive and generate oracles whose exact magnitudes are 6, 3, 4 and 1 with QSV-identified compound units. |
| FR-014-AC-59 | examined | A completed product recorded through `UnitScope::form` resolves as a compound operand for later multiplication and returns 12 in `m^2*s`. |
| FR-014-AC-60 | examined | An unrelated Integer result refuses `ResultTypeMismatch`, and a bare quantity parameter remains `RequiresBound`. |
