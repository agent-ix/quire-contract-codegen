---
id: SR-15335
title: IR-705 quantity ownership integrity review
type: SpecReview
analysis: integrity
scope: agent-ix/quire-contract-codegen; IR-705; spec/oracle/functional/FR-014-exact-scalar-oracles.md;
  spec/oracle/matrix/TC-024-exact-scalar-oracles.md
review_set: subset
---

## Summary

Source-only independent review of CG PR505 for IR-705. FR-014 remains linked through satisfies to StR-001; TC-024 retains its verifies edge to FR-014. The new ABI connects one quantity semantic operation to the same existing direct-owner migration and does not alter supported QSL forms or Kani settlement. AC48 and AC52 consistently distinguish conversion (plain kernel Quantity plus caller-retained checked target) from arithmetic (owner-provided outcome/unit pair). Recording metadata occurs after the immutable operand borrow ends; no generated mutable-scope borrow is required. The new AC52 is in the real acceptance table and not a hand-maintained matrix.

## Verdict

PASS — No defect found in this method’s scoped specification obligations. Native implementation and execution remain outstanding under IR-705.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Examined units

```yaml
scope:
- id: FR-014-AC-2
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'Integer (add, subtract, multiply, negate, truncating/floor/Euclidean division,
    modulo), rational, ordering, decimal, IEEE arithmetic/comparison/width conversion,
    text comparison, enum comparison, quantity arithmetic/comparison and exact quantity
    conversion (FR-014-AC-48) descriptors for admitted checked-package expression
    nodes each generate a function calling the matching direct owner operator described
    above. Entry-point dispositions for a `quire.op.quantity.convert` node: derivation
    and generation produce the oracle (FR-014-AC-48); the routed path, `negotiate_kani_obligations`
    and'
- id: FR-014-AC-6
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: context_only
  excerpt: 'Executing generated oracles on the QSpec TC-185, TC-186, TC-192 and TC-193
    vectors, and on these QSpec TC-187 vectors, yields outcomes, charges and consumed
    counters equal to direct runtime execution: U02 (dimension normalization through
    a product and a quotient, and a zero power), U03 (the dimension mismatch), U05
    (arithmetic and a power on `degC` and `degF`, refused `ill_typed`), U09 (a base
    dimension of the same declaration and shape as `L` under a different node key,
    refused `ill_typed`), U12 (compound-unit products, quotients and powers, with
    the `s` family), U13 and U21 (powers'
- id: FR-014-AC-48
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'A `quire.op.quantity.convert` node whose `type_argument` names a quantity
    unit and whose `operation.mode` is `exact` derives and generates an `ir_confirmed`
    oracle, described by the public operation `quantity_conversion` (`ExactScalarOperation::QuantityConversion`,
    a unit-free variant: the target unit is a call-time argument `unit: &IdentifiedUnit`,
    and the generator does not check it against the `type_argument`, which the node
    carries only as a type node, Contract IR FR-038-AC-84 and FR-038-AC-213), with
    no checked bound, and the claim-map entry records the `type_argument` type node'
- id: FR-014-AC-49
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: 'When QSV refuses a `quire.op.quantity.convert` with `IllTyped` before
    any charge, the oracle stops as `ill_typed` with zero admitted charges and no
    change to any consumed counter, equal to direct `convert_quantity` execution (Contract
    Runtime FR-007-AC-21): `1 m` to `s` (U03-style `m` to `s`), U09 (a base dimension
    of the same declaration and shape as `L` under a different node key), `1 N_m`
    to `J` (U22) and compound `4 m^2` to `m`. An affine source or target converts
    and is not refused as `AffineUnitArithmetic`; only arithmetic on an affine unit
    is refused (FR-014-AC-6, U05).'
- id: FR-014-AC-50
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: context_only
  excerpt: 'No oracle generated for `quire.op.quantity.convert` returns a scalar:
    its result is always a `Quantity` in the supplied target unit, and a conversion
    node whose `operation.mode` is not `exact` is refused with `ClaimDerivationRefusal::ModeNotDerivable`,
    checked first, so a scalar `type_argument` with a non-`exact` mode is `ModeNotDerivable`;
    one whose mode is `exact` and whose `type_argument` is a scalar type (`Decimal`
    or `Integer`, QSpec TC-187 U07, U08, U16, U17, U18 and U29) or a compound unit
    (U25''s first step; IR-831) is refused with `OperandFormsNotDerivable`, each with
    no desc'
- id: FR-014-AC-51
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: context_only
  excerpt: A selected `quire.op.quantity.convert` claim settles through capability
    settlement like any other quantity operation, as `OperationNotRendered` (Planned,
    IR-269), because the Kani scalar renderer builds no quantity harness; the conversion
    is credited by its oracle only and is uncredited for Kani. The settlement is not
    a refusal by name, and a quantity harness is IR-269's work, not this requirement's.
    The owning settlement row is TC-030's Quantity scalar row, which names `quire.op.quantity.convert`
    and cites this criterion.
- id: FR-014-AC-52
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: Every generated quantity operand is a QSV `UnitQuantity<'_>` resolved through
    the kernel quantity's own id by `UnitTable::resolve` or `UnitScope::resolve`,
    with no CG-created unit ids or unrelated metadata pairing. Comparison calls QSV
    `compare_quantity` and preserves its typed `IllTyped`. Arithmetic calls QSV `evaluate_quantity_unit`
    and returns `Result<(Outcome<Quantity>, IdentifiedUnit), OracleStop>` with the
    exact owner-provided identified result unit on every `Ok`, including stopped outcomes,
    and preserves both `QuantityRefusal::IllTyped` and `QuantityRefusal::Identity`
    as disti
- id: FR-014#quantity-operands
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: "- Generated quantity oracles shall receive each quantity operand as QSV\n\
    \  `UnitQuantity<'_>`, obtained by the caller through `UnitTable::resolve` or\n\
    \  `UnitScope::resolve` over the kernel `Quantity`'s own unit id. CG shall not\n\
    \  pair unrelated quantity and unit metadata or construct unit ids itself.\n \
    \ Quantity comparison shall call QSV `compare_quantity` with these resolved\n\
    \  operands and the caller's kernel `Meter`, preserving its `IllTyped` cause,\n\
    \  outcomes, charges and consumed counters (FR-014-AC-52)."
- id: FR-014#quantity-arithmetic
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: "- Quantity arithmetic shall call QSV `evaluate_quantity_unit` and return\n\
    \  `Result<(Outcome<Quantity>, IdentifiedUnit), OracleStop>`, using the kernel\n\
    \  quantity and outcome carriers. The returned unit shall be QSV's actual\n  identified\
    \ result unit, including when the paired outcome stops; a unit\n  alongside a\
    \ stopped outcome shall not imply a completed quantity. On an\n  error, `OracleStop`\
    \ shall preserve QSV `QuantityRefusal::IllTyped` and\n  `QuantityRefusal::Identity`\
    \ with their typed causes, rather than replacing\n  either with a semantic outcome,\
    \ Boolean value, string or `InvalidC"
- id: FR-014#quantity-conversion
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: examined
  excerpt: "- If a requested node's `operation.identity` is `quire.op.quantity.convert`,\n\
    \  then the generator shall derive or accept a descriptor for it whose oracle\n\
    \  calls QSV `convert_quantity(operand, unit.unit(), &QuantityTarget::Exact, meter)`.\n\
    \  Its call-time target argument shall be `unit: &IdentifiedUnit`: the caller\n\
    \  obtains this QSV carrier through its checked constructor before execution,\n\
    \  retaining both the target metadata and its owner-computed id. The admitted\n\
    \  node carries the target only as a type node, `operation.member`'s\n  `type_argument`\
    \ (Contract IR FR-038-AC-84 and FR-0"
- id: TC-024#quantity-agreement
  path: spec/oracle/matrix/TC-024-exact-scalar-oracles.md
  role: examined
  excerpt: "For quantity vectors, obtain every generated and direct operand through\n\
    \   actual QSV `UnitTable::resolve` or `UnitScope::resolve`. Compare generated\n\
    \   comparison calls with QSV `compare_quantity` and arithmetic calls with\n \
    \  QSV `evaluate_quantity_unit`, asserting the paired `IdentifiedUnit` on\n  \
    \ both completed and stopped outcomes. Record the result unit with\n   `UnitScope::form`\
    \ and resolve completed products, quotients and powers for\n   a subsequent operation;\
    \ assert the exact id and metadata. Exercise the\n   owner's reachable result-unit\
    \ identity-limit refusal and ill-typed\n  "
- id: TC-024#quantity-conversion
  path: spec/oracle/matrix/TC-024-exact-scalar-oracles.md
  role: examined
  excerpt: "with `QuantityTarget::Exact`, a resolved QSV `UnitQuantity` operand and\n\
    \   `unit.unit()`, and returns a kernel `Quantity` with `unit.id()` and the\n\
    \   `ConvertedValue::Exact` magnitude in the unit passed as the\n   call-time\
    \ argument `unit: &IdentifiedUnit`, never a scalar and never checked\n   against\
    \ the node's `type_argument` (FR-014-AC-48, FR-014-AC-50). Run the conversion\
    \ vectors\n   (U01, U04 `degC` to `K`, U09b, U10, U15, U22, U25's second step\
    \ and U26) and the `ill_typed` vectors (`1 m` to `s`, U09, `1 N_m` to `J`, compound\
    \ `4 m^2` to `m`),\n   and check each outcome, admitted cha"
- id: FR-014#owner-migration
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: context_only
  excerpt: 'The direct-owner migration is PLANNED until implementation and focused

    qualification. Kernel values, scalar types, outcomes and metering come from

    `quire-exact`; declarations/equality and enum semantics come from QSV

    (`quire-semantic-value`); functions still owned by Contract Runtime remain

    at that owner. Historical uses of the runtime''s `exact` facade below do not

    authorize re-exporting removed owner APIs or retaining a compatibility shim.

    Boolean/integer equality passes QSV an empty `EnumMemberIndex`, which is valid

    for those domains. These scalar oracle bodies and their proof mani'
```

## Coverage

The selection is the quantity ABI diff and its immediate compatibility context. No applicable AssuranceProfile or required review-selection run exists under spec/. Dependency analysis is inapplicable because no relationship edge changed. Object review is unavailable as a typed method in the installed SpecReview.analysis vocabulary; ownership/carrier checks are covered by integrity and scope-boundary. Criterion-strength/Jev and architecture-evaluation were not selected for this minimal ABI amendment. No Cargo, formatting, resolver, compiler, backend, provisioning, generated-crate execution or Kani command ran.

## Owner API evidence

QSV quantity.rs:101–130 defines checked IdentifiedUnit; 219–223 and 292–298 resolve kernel quantities by their own id; 268–274 records already-identified units; 300–320 keeps UnitQuantity fields private; 390–395 shows why Conversion::identified_quantity would recompute identity after conversion and is not the specified handoff; 436–453 shows actual paired arithmetic and distinct pre-charge errors; 458–485 defines conversion; 492–502 defines comparison. Runtime src/exact/mod.rs exports its remaining residue and explicitly retires scalar ownership. The actual owner repositories were read through their local Git objects. Archived IR-917 proof results are not qualification for this amended ABI or new native profile.
