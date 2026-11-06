---
id: SR-2026110283882
title: "IR-1349 PR752 code-review review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; spec/oracle/functional/FR-014-exact-scalar-oracles.md"
review_set: subset
---

## Summary

Reviewed frozen CG PR #752 head 48975e418267ff916f16c26f7b2f1cfd814bed62 against main a881cec153860b27d964ef2c5f908ec3cc224e0d for IR-1349. PR #752 changes one FR-014 specification file. The diff contains no executable code, build configuration, copied vector, pin, or compatibility layer. No applicable AssuranceProfile was found in the scoped spec tree.

## Verdict

PASS. The review is limited to this spec diff and does not attest implementation or a full gate.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Scope checked

```yaml
{
  "scope": [
    {
      "id": "FR-014-AC-6",
      "path": "spec/oracle/functional/FR-014-exact-scalar-oracles.md",
      "role": "examined",
      "excerpt": "Executing generated oracles on the QSpec TC-185, TC-186, TC-192 and TC-193 vectors, and on these QSpec TC-187 vectors, yields outcomes, charges and consumed counters equal to direct runtime execution: U02 (dimension normalization through a product and a quotient, and a zero power), U03 (the dimension mismatch), U05 (arithmetic and a power on `degC` and `degF`, refused `ill_typed`), U09 (a base dimension of the same declaration and shape as `L` under a different node key, refused `ill_typed`), U12 (compound-unit products, quotients and powers, with the `s` family), U13 and U21 (powers), U14 (th"
    },
    {
      "id": "FR-014-AC-48",
      "path": "spec/oracle/functional/FR-014-exact-scalar-oracles.md",
      "role": "examined",
      "excerpt": "For an admitted V2 package or a V1 package with source-authenticated operation views, a `quire.op.quantity.convert` node whose `type_argument` names an admitted quantity `unit` or `compound_unit` and whose `operation.mode` is `exact` derives and generates an `ir_confirmed` oracle, described by the public operation `quantity_conversion` (`ExactScalarOperation::QuantityConversion`, a unit-free variant: the target unit is a call-time argument `unit: &IdentifiedUnit`, and the generator does not check it against the `type_argument`, which the node carries only as a type node, Contract IR FR-038-AC-"
    },
    {
      "id": "FR-014-AC-50",
      "path": "spec/oracle/functional/FR-014-exact-scalar-oracles.md",
      "role": "examined",
      "excerpt": "For a reader-admitted `quire.op.quantity.convert` whose target is a `unit` or `compound_unit`, the generated oracle returns a kernel `Quantity` in the supplied target unit and a mode other than `exact` refuses `ClaimDerivationRefusal::ModeNotDerivable` with no descriptor or oracle. At CG's defensive seam, an unsupported scalar target (plain `Decimal` or `Integer`, rather than `scalar_type`/`carried_quantity`) refuses `OperandFormsNotDerivable` without changing a sibling. Reader-admitted carried targets follow FR-014-AC-62..65, including a matching non-`exact` mode; they do not take either refu"
    },
    {
      "id": "FR-014-AC-61",
      "path": "spec/oracle/functional/FR-014-exact-scalar-oracles.md",
      "role": "examined",
      "excerpt": "A reader-admitted exact `quire.op.quantity.convert` from `1 N_m` to the `compound_unit` target `kg*m^2*s^-2` (QSpec TC-187 U25, first step) derives an `ir_confirmed` conversion claim, retains that exact target type node id, and generates an oracle. With the corresponding caller-supplied QSV `IdentifiedUnit`, execution completes as exact `1` in that compound unit; recording the owner-supplied unit through `UnitScope::form` lets the second admitted exact conversion complete as `1 J`. Both outcomes, admitted charges and consumed counters equal direct QSV execution under U25's stated limits; no ta"
    },
    {
      "id": "FR-014-AC-62",
      "path": "spec/oracle/functional/FR-014-exact-scalar-oracles.md",
      "role": "examined",
      "excerpt": "For each reader-admitted `quire.op.quantity.convert` into `scalar_type`/`carried_quantity`, derivation and generation produce an `ir_confirmed` conversion oracle and a claim retaining the exact target structural node id, admitted target unit, bounded Int, Decimal or Rational carrier domain, Decimal scale interval when applicable, and carrier rounding. The selected operation mode equals the target node's rounding; `nearest-even` is derivable for a Decimal or Int carrier and a Rational carrier uses `exact`. The oracle passes QSV the corresponding owner-constructed target unit and `QuantityTarget"
    },
    {
      "id": "FR-014-AC-63",
      "path": "spec/oracle/functional/FR-014-exact-scalar-oracles.md",
      "role": "examined",
      "excerpt": "Executing generated carried-conversion oracles on reader-admitted packages returns the same completed QSV conversion value and target unit as direct `convert_quantity`, preserving the distinct carried result type and any loss record: exact Decimal `25 cm` gives coefficient 25 at scale two in `m`; exact Int `2 m` gives 2; nearest-even Decimal `1 in` gives coefficient 3 at scale two with QSV's nonzero `DecimalLoss`; nearest-even Int `25 cm` gives 0 with its loss; exact Rational `25 cm` gives 1/4 and `-3 m` gives -3 within the stated domain. No Decimal or Integer completion is coerced into an exa"
    },
    {
      "id": "FR-014-AC-64",
      "path": "spec/oracle/functional/FR-014-exact-scalar-oracles.md",
      "role": "examined",
      "excerpt": "For a reader-admitted carried conversion, generated and direct QSV execution agree in typed stopped outcome, cause, absence of value, charge prefix and counters: under `exact`, `1 in` into the bounded Decimal scale-two carrier or bounded Int carrier refuses `inexact_conversion` at `convert` before target-domain even when it is also outside bounds; exact on-grid values outside Decimal or Int bounds refuse the respective `decimal_out_of_domain` or `integer_out_of_domain` after target-domain and before result-retain; nearest-even `13/5 m` to bounded Int rounds before its out-of-domain refusal; Ra"
    },
    {
      "id": "FR-014-AC-65",
      "path": "spec/oracle/functional/FR-014-exact-scalar-oracles.md",
      "role": "examined",
      "excerpt": "For each reader-admitted carried target, the generated oracle's complete, refused or incomplete outcome, ordered admitted charges, denied next charge and all consumed counters equal direct QSV `convert_quantity` at the same limits. Work limit four still gives U16's `inexact_conversion` after its four admitted work units; work limit one makes U17's out-of-domain Decimal case and U29's out-of-domain Int case incomplete at `unit.target-domain`; the huge Decimal scale bound gives U18's `decimal_digits` incomplete at that charge without materializing a retained coefficient; Rational `8 m` at work l"
    }
  ],
  "bindings": []
}
```
