---
id: SR-2026110283897
title: "IR-1439 PR #765 FR-050 dependency analysis"
type: SpecReview
analysis: dependency
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

Ticket: IR-1439. Dependency analysis of FR-050's `relationships` edges and stated prerequisites. IR-1443 ("quire-exact: Kani-tractable quantity carrier", Backlog) is the carrier ticket and FR-050, TC-064, tests.md and the FR-015 row all name it consistently as K-EXACT-quantity (IR-1443). Upstream bound admission was checked in quire-contract-ir FR-038-AC-73 and FR-038-AC-471.

## Verdict

CHANGES REQUIRED. The IR-1443 carrier gate is stated correctly, but it is not the only prerequisite: the bounded magnitude domains FR-050 draws from do not exist upstream for exact quantity operands. Two cited requirements are missing from `relationships`.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-050 requires 'operands whose unit and magnitude domains are finite' 'from the admitted input', but upstream gives none for the operands FR-014's arithmetic oracle takes. Contract IR FR-038-AC-73/AC-471 return `requires_bound` for every parameter or field declared at an exact `unit`/`compound_unit`, and FR-014-AC-60 keeps that terminal before CG. Only `carried_quantity` positions are bounded, and FR-014 accepts carried operands only for conversion (AC-66). As written every non-literal operand is `requires_bound` (AC-5), so the supported path (AC-1..3, AC-7) cannot be reached with IR-1443 alone. Name the upstream bound mechanism or carried-operand arithmetic as a dependency, with its ticket. | spec/kani/functional/FR-050-quantity-scalar-harnesses.md:23 |
| FND-002 | low | FR-050 cites FR-029 (Outputs, AC-7) and FR-039 (AC-6 per-item accounting) as owners of behaviour it relies on, but its `relationships` list only StR-001, FR-014, FR-015, FR-017, FR-025 and FR-028. Add `depends_on` edges for FR-029 and FR-039. | spec/kani/functional/FR-050-quantity-scalar-harnesses.md:23 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-050 | examined | spec/kani/functional/FR-050-quantity-scalar-harnesses.md:23 | When an admitted request selects a reader-admitted `quantity` scalar operation (`add`, `sub`, `mul`, `div`, `pow` or a comparison) over operands whose unit and magnitude domains are finite, and the quire-exact quantity carrier dependency K-EXACT-quantity (ticket IR-1443) is available, CG SHALL generate a bounded Kani harness that proves the generated quantity oracle of [FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md) agrees with the native quantity expression over every admitted symbolic magnitude tuple. This is the quantity row of the IR-269 scalar renderer that FR-014-AC-51 l |
| FR-050-AC-2 | examined | spec/kani/functional/FR-050-quantity-scalar-harnesses.md:94 | Symbolic magnitudes are drawn from the exact inclusive domain of each operand through K-EXACT-quantity, each declared magnitude has exactly one accepted representation, and a clipped endpoint, duplicate encoding or extra value fails the correspondence check. Units are fixed and resolved through `UnitTable::resolve` or `UnitScope::resolve`; no symbolic unit, CG-created unit id or unrelated metadata pairing is generated. |
| FR-050-AC-5 | examined | spec/kani/functional/FR-050-quantity-scalar-harnesses.md:97 | A missing, unbounded or repeated magnitude domain, or a missing `pow` exponent bound, yields source-associated `requires-bound` naming the operand and domain with no harness. A finite endpoint beyond the carrier capacity yields named `unsupported` retaining the endpoint with no harness; generation never clips or substitutes a smaller domain. |
| FR-050-AC-6 | examined | spec/kani/functional/FR-050-quantity-scalar-harnesses.md:98 | When K-EXACT-quantity (IR-1443) is unavailable, the item is `unsupported` naming that carrier dependency with no harness and no Verified claim. Unsupported operand form, absent native expression and a quantity leaf of a composite each yield their own named disposition, with total per-item accounting under FR-039 and an unchanged supported sibling. |
| FR-050-deps | examined | spec/kani/functional/FR-050-quantity-scalar-harnesses.md:105 | - [FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md) owns quantity oracle semantics (AC-6, AC-48..AC-52, AC-59..AC-65); its native or oracle agreement is not this proof, and AC-51 is superseded for non-conversion quantity operations only when this row is delivered. |
| FR-014-AC-60 | context_only | spec/oracle/functional/FR-014-exact-scalar-oracles.md:700 | A multiplication with an unrelated Integer result reaching CG's defensive form check refuses `ResultTypeMismatch` and emits no symbol. A plain quantity-typed parameter remains `RequiresBound` in Contract IR's bounds-required profile and generates no CG oracle; accepting a `compound_unit` result does not relax that refusal. |
| FR-014-AC-52 | context_only | spec/oracle/functional/FR-014-exact-scalar-oracles.md:692 | Every generated quantity operand is a QSV `UnitQuantity<'_>` resolved through the kernel quantity's own id by `UnitTable::resolve` or `UnitScope::resolve`, with no CG-created unit ids or unrelated metadata pairing. Comparison calls QSV `compare_quantity` and preserves its typed `IllTyped`. Arithmetic calls QSV `evaluate_quantity_unit` and returns `Result<(Outcome<Quantity>, IdentifiedUnit), OracleStop>` with the exact owner-provided identified result unit on every `Ok`, including stopped outcomes, and preserves both `QuantityRefusal::IllTyped` and `QuantityRefusal::Identity` as distinct typed  |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | P2 (the upstream quantity magnitude bound) is named but 'has no ticket named here'. IR-1439's own AC says the row ticket is unblocked by this spec, which cannot hold while an unowned prerequisite gates every supported criterion. File a P2 ticket (Contract IR bound on exact quantity parameters, or FR-014 carried-operand arithmetic) and cite it. | spec/kani/functional/FR-050-quantity-scalar-harnesses.md:131 |
| FND-004 | low | The fix makes counterexample replay FR-016's ('counterexample replay remains FR-016's') and AC-9 relies on it, but FR-050's `relationships` has no `depends_on` edge to FR-016. | spec/kani/functional/FR-050-quantity-scalar-harnesses.md:72 |

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | fixed in f255c11d |
| FND-002 | fixed | fixed in f255c11d |
| FND-003 | fixed | fixed in e738e884/66c9558c |
| FND-004 | fixed | fixed in e738e884/66c9558c |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | low | The IR-1497 citation was applied in FR-050, TC-064, tests.md and spec.md but not in the TC-030 Quantity scalar row this PR amended, which still reads 'conditional on IR-1443 and an upstream quantity bound mechanism' with no ticket. Add IR-1497 there too. | spec/routed/matrix/TC-030-capability-settlement.md:401 |
