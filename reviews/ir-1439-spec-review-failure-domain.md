---
id: SR-2026110283898
title: "IR-1439 PR #765 FR-050 failure-domain analysis"
type: SpecReview
analysis: failure-domain
date: 2026-11-10
scope: "agent-ix/quire-contract-codegen; PR #765 @ 51d9a3f9; IR-1439; spec/kani/functional/FR-050-quantity-scalar-harnesses.md, spec/kani/matrix/TC-064-quantity-scalar-harnesses.md, spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/functional/FR-025-generated-subject-abi.md, spec/kani/functional/FR-028-bounded-proof-ceilings.md, spec/kani/matrix/tests.md, spec/spec.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-050
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-064
    type: reviews
---

## Summary

Ticket: IR-1439. Failure modes of FR-050's quantity operations checked against QSV `src/quantity.rs` (`QuantityOperation`, `type_check`, `result_unit`, `check_comparable`, `check_undefined`, `evaluate`) and FR-014-AC-6/AC-52. `IllTyped` (incompatible dimensions, affine arithmetic) and `Identity` refuse before any charge; division by a zero quantity is a stopped `Undefined::DivisionByZero` after identity-read charges, which AC-4's 'unchanged charges where the owner refuses before charge' already allows.

## Verdict

CHANGES REQUIRED. The `pow` exponent is modelled as a bounded symbolic domain, which the fixed-unit rule and the owner API do not permit, and two owner failure modes are missing from AC-4.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | AC-4 asserts 'an exponent outside its bound' as an agreed typed outcome, and AC-5/AC-8/AC-9 add a `pow` exponent bound, `requires-bound` for a missing one, an 'altered exponent' mutant and recorded exponent domains. QSV `Power(UnitQuantity, &Integer)` fixes the result unit at type time (`canonical_compound().power(n)`), so a symbolic exponent would make the result unit symbolic, which FR-050's own 'a unit is never drawn symbolically' forbids. Contract IR FR-038-AC-471 admits only a literal exponent. QSV has no exponent-bound outcome. Treat the exponent as a fixed node value and drop the exponent-bound outcome and requires-bound case. | spec/kani/functional/FR-050-quantity-scalar-harnesses.md:96 |
| FND-002 | medium | AC-4 omits two owner failure modes: a zero base under a negative exponent (QSV `check_undefined` stops `Undefined::DivisionByZero` for `pow`, not only `div`), and `IllTyped` `DistinctUnits` (same dimension, different unit, e.g. `1 m + 1 cm` or comparing `m` with `cm`), which is a separate cause from dimension mismatch and affine arithmetic. Both should be asserted as agreed outcomes. | spec/kani/functional/FR-050-quantity-scalar-harnesses.md:96 |
| FND-003 | low | AC-3 requires agreement on 'identified result unit' for every operation, but `compare_quantity` returns `Outcome<bool>` with no unit; only `evaluate_quantity_unit` returns an `IdentifiedUnit`. Scope the unit check to arithmetic and `pow`. | spec/kani/functional/FR-050-quantity-scalar-harnesses.md:95 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-050-AC-3 | examined | spec/kani/functional/FR-050-quantity-scalar-harnesses.md:95 | The generated oracle and an independent native expression over public owner quantity operations agree, from matching entry state and limits, on completed value, identified result unit, typed outcome, ordered charges, denied next charge and consumed counters, including zero, exact and short budgets. The reference neither runs the generated oracle twice nor bridges through Runtime. |
| FR-050-AC-4 | examined | spec/kani/functional/FR-050-quantity-scalar-harnesses.md:96 | Dimension mismatch (`IllTyped`), identity refusal (`Identity`), arithmetic on an affine unit, division by a zero quantity and an exponent outside its bound are each asserted as the agreed typed outcome with unchanged charges where the owner refuses before charge. None is an assumption, none is relabelled a different cause and a stopped outcome is never proved as completed. |
| FR-050-AC-5 | examined | spec/kani/functional/FR-050-quantity-scalar-harnesses.md:97 | A missing, unbounded or repeated magnitude domain, or a missing `pow` exponent bound, yields source-associated `requires-bound` naming the operand and domain with no harness. A finite endpoint beyond the carrier capacity yields named `unsupported` retaining the endpoint with no harness; generation never clips or substitutes a smaller domain. |
| FR-050-AC-9 | examined | spec/kani/functional/FR-050-quantity-scalar-harnesses.md:101 | Run evidence records the operation and source association, magnitude and exponent domains with correspondence, fixed operand units, subject and full ordered backend arguments, unwind bounds, tool versions, target, dependency cohort including the K-EXACT-quantity carrier, limits, raw checks, cover and playback, exit and typed terminal outcome. No-run, refusal, cover failure, resource exhaustion and semantic falsification remain distinguishable. |
| FR-014-AC-52 | context_only | spec/oracle/functional/FR-014-exact-scalar-oracles.md:692 | Every generated quantity operand is a QSV `UnitQuantity<'_>` resolved through the kernel quantity's own id by `UnitTable::resolve` or `UnitScope::resolve`, with no CG-created unit ids or unrelated metadata pairing. Comparison calls QSV `compare_quantity` and preserves its typed `IllTyped`. Arithmetic calls QSV `evaluate_quantity_unit` and returns `Result<(Outcome<Quantity>, IdentifiedUnit), OracleStop>` with the exact owner-provided identified result unit on every `Ok`, including stopped outcomes, and preserves both `QuantityRefusal::IllTyped` and `QuantityRefusal::Identity` as distinct typed  |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | medium | AC-4 (and TC-064 step 4) still asserts 'a literal exponent outside the owner's admitted range' as a typed outcome. QSV has no exponent range: `Power(UnitQuantity, &Integer)` accepts any integer, and the private `power` charges `UnitRationalArithmetic` with `IntegerBits` = max(1, /n/ x maxparts), so a large literal exponent gives a limit-incomplete stop, not a range refusal. Replace it with the IntegerBits limit stop (incomplete record and denied charge) or drop it. | spec/kani/functional/FR-050-quantity-scalar-harnesses.md:118 |

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | fixed in f255c11d |
| FND-002 | fixed | fixed in f255c11d |
| FND-003 | fixed | fixed in f255c11d |
| FND-004 | fixed | fixed in e738e884/66c9558c |
