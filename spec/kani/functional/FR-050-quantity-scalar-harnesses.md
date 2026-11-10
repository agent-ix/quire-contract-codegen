---
id: FR-050
title: "Generate bounded quantity scalar operation harnesses"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-025
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-028
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-029
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-039
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-016
    type: depends_on
---
# FR-050: Generate bounded quantity scalar operation harnesses

## Description

When an admitted request selects a reader-admitted `quantity` scalar operation
(`add`, `sub`, `mul`, `div`, `pow` with a literal exponent, or a comparison) and
both planned prerequisites below are available, CG SHALL generate a bounded Kani
harness that proves the generated quantity oracle of
[FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md) agrees with the
native quantity expression over every admitted symbolic magnitude tuple. This is
the quantity row of the IR-269 scalar renderer that TC-030's Quantity scalar row
settles today as `OperationNotRendered`. It is a planned row, not a new IR schema,
unit-id constructor or claim of current Rust support.

Two prerequisites are named and neither is delivered. (P1) K-EXACT-quantity
(ticket IR-1443) is the quire-exact carrier giving a symbolic exact magnitude and a
native expression over it. (P2, ticket IR-1497) An upstream magnitude-bound mechanism for quantity
operands does not exist: Contract IR FR-038-AC-73 and FR-038-AC-471 and
FR-014-AC-60 make every exact `unit` or `compound_unit` parameter `requires_bound`
before CG, `carried_quantity` is the only bounded quantity type, and FR-014 admits
carried operands only for conversion (FR-014-AC-66). A supported item therefore
needs either an upstream bound on exact quantity parameters or FR-014 arithmetic
over carried operands. Every criterion that describes a supported harness is
conditional on P1 and P2; absent either, the item stays settled as AC-6 states.

The compound-unit and carried result forms admitted by FR-014 (PRs #750 through
#754) and resolved `UnitQuantity` operands are reused unchanged: this requirement
adds no operand form, no unit construction and no oracle of its own. Quantity
conversion (`quire.op.quantity.convert`) is outside this requirement.

## Inputs

- A requested quantity operation node and the operand forms, units and identified
  result unit supplied by the admitted package and FR-014's generated oracle.
- The finite magnitude domain of each quantity operand (P2) and, for `pow`, the
  literal integer exponent carried by the node.
- The K-EXACT-quantity carrier (P1).
- The proof subject, backend and ceilings of
  [FR-015](./FR-015-bounded-kani-obligations.md),
  [FR-025](./FR-025-generated-subject-abi.md) and
  [FR-028](./FR-028-bounded-proof-ceilings.md); per-item accounting is
  [FR-039](../../core/functional/FR-039-contract-package-consumption.md)'s.

## Outputs

A supported item has its harness, symbolic-input correspondence and
source-associated proof record. A refused item has its named disposition and no
harness. Execution and terminal outcomes remain
[FR-017](./FR-017-kani-execution-evidence.md) and
[FR-029](./FR-029-run-outcome-terminal-record.md)'s; counterexample replay remains
[FR-016](../../replay/functional/FR-016-witness-native-replay.md)'s.

## Behavior

When the operation is representable, CG SHALL draw each operand magnitude
symbolically from its exact inclusive domain through the carrier. Each operand's
unit is fixed by the admitted node and resolved to a QSV `UnitQuantity<'_>` as
FR-014-AC-52 states; a unit is never drawn symbolically and CG creates no unit id.
A `pow` exponent is the node's literal integer, never a symbolic value, because the
QSV power operation fixes the result unit at type time. Representation constraints
apply before evaluation, admit every declared magnitude exactly once and do not
depend on the outcome.

When evaluating an arithmetic operation, the harness SHALL compare the FR-014
generated oracle with a native expression that calls the owner's public
`evaluate_quantity_unit` independently, from matching entry Meter state and limits,
over the full typed outcome, the identified result unit on every `Ok`, ordered
charges and consumed counters. A comparison calls `compare_quantity`, returns an
`Outcome<bool>` with no unit, and is compared on outcome, charges and counters only.
Typed refusals are expected observations, preserved as distinct causes and never
assumed away. A stopped outcome never becomes a completed value.

When asserting, the harness SHALL emit exactly one final completion cover after all
assertions. No assumption depends on the operation's outcome.

When decoding retained playback, CG SHALL decode the physical bindings that the
carrier's FR-025 row defines, validate them against the original domains before
reconstruction and submit one owner quantity per parameter through the public
replay route, as FR-025 states for Rational. An unavailable route is reported as
unsupported, not as an accepted replay.

When the carrier, a finite magnitude bound, the literal exponent, the resolved
operand form or the native expression is unavailable, the item is refused with the
dispositions below and no harness. A quantity leaf inside a composite remains
FR-015's `ShadowFamilyNotBuilt`.

## Acceptance Criteria

All supported-harness criteria are conditional on P1 and P2.

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-050-AC-1 | Each admitted quantity `add`, `sub`, `mul`, `div`, literal-exponent `pow` and comparison node selects a harness that names the operation identity, operand forms and units from the admitted node; compound-unit and carried-result forms admitted by FR-014 (#750-#754) are accepted without a new operand form. A conversion node is not selected here. | Test |
| FR-050-AC-2 | Symbolic magnitudes are drawn from the exact inclusive domain of each operand through the carrier, each declared magnitude has exactly one accepted representation, and a clipped endpoint, duplicate encoding or extra value fails the correspondence check (as FR-046-AC-2). Units are fixed and resolved through `UnitTable::resolve` or `UnitScope::resolve`; no symbolic unit, CG-created unit id or unrelated metadata pairing is generated. | Test |
| FR-050-AC-3 | For arithmetic operations the generated oracle and an independent native expression over public `evaluate_quantity_unit` agree, from matching entry state and limits, on completed value, identified result unit, typed outcome, ordered charges, denied next charge and consumed counters, including zero, exact and short budgets. Comparisons agree on `Outcome<bool>`, charges and counters, with no unit asserted. The reference neither runs the generated oracle twice nor bridges through Runtime. | Test |
| FR-050-AC-4 | Each of these is asserted as the agreed typed outcome with the owner's charges, not assumed away: `IllTyped` dimension mismatch, `IllTyped` distinct units of one dimension (`1 m + 1 cm`), `Identity` refusal, arithmetic on an affine unit, `DivisionByZero` for division by a zero quantity, `DivisionByZero` for a zero base with a negative literal exponent, and a large literal exponent, which has no range refusal and yields the owner's `IntegerBits` limit-incomplete stop at `UnitRationalArithmetic`. No cause is relabelled and a stopped outcome is never proved completed. The generated harness contains no assumption that depends on the operation's outcome (inspected, as FR-046-AC-4). | Test |
| FR-050-AC-5 | A missing, unbounded or repeated magnitude domain (the upstream `requires_bound` of P2 reaching CG defensively), or a `pow` whose exponent is not a literal integer, yields source-associated `requires-bound` or `unsupported` naming the operand or exponent with no harness. A finite endpoint beyond the carrier capacity yields named `unsupported` retaining the endpoint; generation never clips or substitutes a smaller domain. | Test |
| FR-050-AC-6 | When P1 (K-EXACT-quantity, IR-1443) or P2 is unavailable the item remains settled as TC-030's Quantity scalar row states, `OperationNotRendered`, with the `NoRenderer` settlement carrying no prerequisite name (a typed reason names it only where one is carried), with no harness and no Verified claim. Unsupported operand form, absent native expression and a quantity leaf of a composite each keep their own named disposition, with total per-item accounting under FR-039 and an unchanged supported sibling. | Test |
| FR-050-AC-7 | A healthy quantity operation receives `Verified` only from an actual successful proof with all checks and exactly one final completion cover satisfied. Unsatisfied cover, failed unwind, timeout, memory exhaustion or missing verdict cannot yield `Verified`, and FR-017 and FR-029 classifications are retained. | Test |
| FR-050-AC-8 | Mutants fail with actual solver playback of an in-domain input, over control domains that distinguish them: wrong operator (`add` for `sub`) with a nonzero second operand; swapped operands in `sub`, `div`, `lt`, `le`, `gt` and `ge` (not `eq` or `ne`, where the swap is equivalent) with distinct nonzero operands; `lt` for `le` or `gt` for `ge` with equal operands; a result unit differing from the owner's identified unit; a `pow` result for a different literal exponent over a base whose magnitude is not 0, 1 or -1; a dropped or reordered charge; and a stopped outcome reported completed. The control domain for each must contain its distinguishing point. | Test |
| FR-050-AC-9 | Counterexample decoding validates every physical binding of the carrier's FR-025 row (width, order, comment, schema role, byte and entry-count limits, canonical form and original domain) before reconstructing one owner quantity per parameter, each paired with its fixed unit, and submits it through the public replay route for that exact declaration. Missing, duplicate, foreign, role-swapped, out-of-domain or noncanonical evidence is refused with a typed cause and no replay; an unavailable replay capability is reported without claiming acceptance. | Test |
| FR-050-AC-10 | Run evidence records the operation and source association, magnitude domains with correspondence, the literal exponent, fixed operand units, subject and full ordered backend arguments, unwind bounds, tool versions, target, dependency cohort including the carrier, limits, raw checks, cover and playback, exit and typed terminal outcome. No-run, refusal, cover failure, resource exhaustion and semantic falsification remain distinguishable. | Test |

## Dependencies

- [FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md) owns quantity
  oracle semantics (AC-6, AC-48..AC-52, AC-59..AC-66); its oracle agreement is not
  this proof.
- K-EXACT-quantity (IR-1443) is prerequisite P1. P2, the upstream quantity bound
  mechanism, is IR-1497.
- TC-030's Quantity scalar row owns today's `OperationNotRendered` settlement; this
  row supersedes it for non-conversion quantity operations only when delivered.
- [FR-015](./FR-015-bounded-kani-obligations.md), [FR-025](./FR-025-generated-subject-abi.md)
  and [FR-028](./FR-028-bounded-proof-ceilings.md) own the shared refusals and ABI row.
- [TC-064](../matrix/TC-064-quantity-scalar-harnesses.md) defines the planned
  verification.

## Status

PLANNED (IR-1439). This specification supplies no implementation or
generated-proof credit. All ten criteria remain planned and untagged until P1 and
P2 are delivered with the IR-269 quantity harness. Oracle agreement under FR-014
is not Kani credit.
