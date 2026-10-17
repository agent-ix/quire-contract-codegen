---
id: SR-12083
title: "integrity review of IR-818 CG quantity-conversion spec (PR #415)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen; spec/oracle/functional/FR-014-exact-scalar-oracles.md, spec/oracle/functional/FR-021-function-application-oracles.md, spec/oracle/matrix/TC-024-exact-scalar-oracles.md, spec/oracle/matrix/tests.md"
review_set: subset
---

## Summary

Ticket: IR-818. PR agent-ix/quire-contract-codegen#415, frozen head 5abb404, diff `origin/main...HEAD`
(base 0e0f92e). Consistency check of the reworded FR-014 conversion text, the new FR-014-AC-48..51,
FR-021-AC-21, TC-024 step 6 and the oracle test-matrix rows against each other, against Contract
Runtime FR-007-AC-20..25 / TC-201 at RT 561691e, against the RT code at CG's locked RT 3cc88c4, and
against QSpec TC-187 (quire-specification origin/main 9a1d3cf).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-014-AC-6 puts U07, U08, U16, U17, U18 and U29 in the exact quantity-to-quantity conversion agreement set, but in QSpec TC-187 each of them converts to a Decimal or Integer target (with `exact`/`nearest-even` rounding). That is a scalar result, which FR-014-AC-50 excludes and RT TC-201 puts out of scope. tests.md:14 and TC-024:53-54 repeat the claim | spec/oracle/functional/FR-014-exact-scalar-oracles.md:446, spec/oracle/functional/FR-014-exact-scalar-oracles.md:489, spec/oracle/matrix/tests.md:14, spec/oracle/matrix/TC-024-exact-scalar-oracles.md:53 |
| FND-002 | medium | tests.md marks FR-014-AC-2 `✅ Covered`, including "a quantity.convert node is an admitted quantity operation at every entry point". No test asserts that. The tests still tagged FR-014-AC-2 (`tc_024_quantity_conversion_is_refused_with_its_named_capability`, `tc_025_a_quantity_conversion_claim_is_unsupported_...`, `tc_033_a_quantity_conversion_is_unsupported_...`, `a_selected_quantity_conversion_settles_as_unsupported_...`) assert the opposite. The clause should be marked Planned with the code slice | spec/oracle/matrix/tests.md:15 |
| FND-003 | low | FR-014-AC-48 cites RT FR-007-AC-22 to AC-25 for "the affine, charge-order and denied-charge behaviour". RT FR-007-AC-24 covers `evaluate_quantity` affine-arithmetic refusals (U05/U20), not conversion. The conversion criteria are AC-22 (affine), AC-23 (charge order) and AC-25 (denial) | spec/oracle/functional/FR-014-exact-scalar-oracles.md:487 |
| FND-004 | low | FR-014-AC-51's evidence is a capability-settlement disposition. It is verified only as `Test (TC-024)`, and the tests.md row cites TC-024 only. The owning settlement row (TC-030:308, `Quantity scalar expression claim ... OperationNotRendered`, Planned IR-269) does not name `quire.op.quantity.convert`, and neither document cross-references the other | spec/oracle/functional/FR-014-exact-scalar-oracles.md:490, spec/oracle/matrix/tests.md:16, spec/routed/matrix/TC-030-capability-settlement.md:308 |

## Verdict

These rulings hold in the text:
- CG calls the existing `convert_quantity` with `QuantityTarget::Exact` and adds no new RT API (FR-014:270-276, AC-48).
- The capability text is reworded in the same change that removes the refusal, and `quantity_conversion_unsupported` is gone from Outputs.
- `ill_typed` stops with zero charge and no counter change (AC-49).
- Compound targets are out (IR-831).
- Kani option (i) applies: `OperationNotRendered` (IR-269), uncredited for Kani (AC-51).

The RT citations check out:
- FR-007-AC-20, AC-21, AC-22, AC-23 and AC-25, and TC-201, exist at RT 561691e and say what FR-014 cites them for. AC-24 is the exception (FND-003).
- At the locked RT 3cc88c4, the behaviour is in the code even though those ACs are not yet written there:
  - `convert_quantity(&Quantity,&QuantityUnit,&QuantityTarget,&mut Meter) -> Result<Outcome<Conversion>,IllTyped>` returns `IncompatibleDimensions` before `read_identities`, so it refuses before any charge.
  - `QuantityTarget::Exact` gives `ConvertedValue::Exact`.
  - An affine source or target is admitted.
  - The `charge_bits` formula is the one TC-187 states.
- The spec correctly carries no revision pin.

No main-branch spec text still describes `quantity_conversion_unsupported`. TC-025 and TC-033 contain no refusal text; only the code-side tests named in FND-002 still assert the refusal. The new tests.md row uses the repo's `🚧 Planned (...)` marker convention. Not mergeable until FND-001 is fixed.

## Scope

Examined: FR-014 Description (:213-216), Outputs (:239-243), the Behavior conversion bullet (:269-290), FR-014-AC-2, AC-6, AC-18, AC-48, AC-49, AC-50, AC-51; FR-021-AC-21; TC-024 step 6 (:44-66); tests.md rows :14-16.
Context only: RT FR-007-AC-20..25, RT TC-201, QSpec TC-187 U01-U29, TC-030:308, FR-019-AC-44, FR-015-AC-15, FR-022-AC-11, TC-025, TC-033.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | d685fc5: AC-6, TC-024 step 6 and tests.md:14 drop U07, U08, U16, U17, U18 and U29 from the agreement set and place them under AC-50 refusals |
| FND-002 | fixed | d685fc5: the tests.md AC-2 row marks the conversion entry-point clause `🚧 Planned (IR-818 code slice)` and names the four refusal tests that flip |
| FND-003 | fixed | d685fc5: AC-48 cites FR-007-AC-22, AC-23 and AC-25; AC-24 is gone |
| FND-004 | fixed | d685fc5: the TC-030:308 row names `quire.op.quantity.convert` and FR-014-AC-51; AC-51 and the tests.md row name the owning TC-030 row |

## New findings (disposition pass 3)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | high | FR-014-AC-50 and TC-024 step 6 get the IR reachability of the two refusal cases backwards. Contract IR FR-038-AC-248 (TC-048:360-366, quire-contract-ir fabdbb4) refuses a `quantity.convert` whose resolved declared type is not a quantity at `result_type` by its `member_family`. So a Decimal or Integer `type_argument` never reaches an admitted package, yet the spec says the scalar-target refusal "runs on an admitted-package fixture". The same AC, and FR-038 at line 3649, admit `quire.op.quantity.convert` into a `compound_unit`, so a compound target is reachable from an admitted package, yet the spec says IR "defines no compound-unit form" and covers it only by a hand-built module test. AC-213 does not support the "no compound-unit form" claim. The `ModeNotDerivable` unreachability claim is correct | spec/oracle/functional/FR-014-exact-scalar-oracles.md:519, spec/oracle/matrix/TC-024-exact-scalar-oracles.md:71-76, spec/oracle/matrix/tests.md:14 |
