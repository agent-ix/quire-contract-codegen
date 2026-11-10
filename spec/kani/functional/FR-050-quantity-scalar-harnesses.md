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
  - target: ix://agent-ix/quire-contract-codegen/FR-046
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-051
    type: depends_on
---
# FR-050: Generate bounded quantity scalar operation harnesses

## Description

When an admitted request selects a reader-admitted `quantity` scalar operation
(`add`, `sub`, `mul`, `div`, `pow` with a literal exponent, or a comparison) whose
every operand is a conversion of a carried-quantity leaf parameter (defined below)
and the planned prerequisite below is available, CG SHALL generate a bounded Kani
harness that proves the generated quantity oracle of
[FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md) agrees with the
native quantity expression over every admitted symbolic leaf tuple. This is
the quantity row of the IR-269 scalar renderer that TC-030's Quantity scalar row
settles today as `OperationNotRendered`. It is a planned row, not a new IR schema,
unit-id constructor or claim of current Rust support.

One prerequisite is named and is not delivered: (P1) K-EXACT-quantity
(ticket IR-1443) is the quire-exact carrier giving a symbolic carried-quantity leaf
built from its carrier's physical bindings and a native expression over it. The
operand form is decided, not awaited (ticket IR-1497): the finite magnitude domain of an operand comes from a carried leaf,
not from the operand itself, because no other bounded form is finite: Contract IR
FR-038-AC-73, FR-038-AC-471 and FR-014-AC-60 make every bare exact `unit` or
`compound_unit` parameter `requires_bound` before CG; the `bounded_quantity` form
(Contract IR FR-038-AC-485 and AC-486) bounds only a rational interval
`[min, max]`, whose members have unbounded denominators and so form no finite
domain; and the language refuses a carried quantity as an operand of quantity
arithmetic or comparison (QSpec FR-142-AC-27, QSL FR-497-AC-2). The one finite form
the language does admit is a `scalar_type`/`carried_quantity` parameter (carrier
`Int[..]`, `Decimal[..]` with a scale interval, or `Rational[..]` with a denominator
range) converted to an exact `Quantity<u>` by `quire.op.quantity.convert`, which
FR-014-AC-66 already accepts. This requirement therefore draws the finite domain
from that leaf and composes the conversion, with its charges, into the proof of the
operation. Every criterion that describes a supported harness is conditional on P1;
absent it, the item stays settled as AC-6 states, and an operand that is not
composable is refused as AC-5 states.

Whether the composed proof (leaf construction, conversion and operation in one
harness) fits the proof ceilings of FR-028, including the per-request memory
ceiling (16 GiB in the current proof profile), is not known and is measured later
under IR-1443. If it does not fit, this requirement does not narrow the domain to
make it fit: a new finite-grid bound form for exact quantity parameters is then the
fallback. QSpec FR-154 "Unit bounds" does not give one, because it bounds only a
rational interval, so the fallback is a QSpec change specified under its own
ticket, which does not exist yet.

The compound-unit result forms admitted by FR-014 (PRs #750 through #754) and
resolved `UnitQuantity` operands are reused unchanged: this requirement
adds no operand form, no unit construction and no oracle of its own. A conversion
into a carried target is not a composable operand (AC-5). A
`quire.op.quantity.convert` node as the requested root, with its carried-conversion
forms of FR-014-AC-62 through AC-65, remains outside this requirement; the same
conversion as an operand is composed here and is not proved on its own.

## Inputs

- A requested quantity operation node and the operand forms, units and identified
  result unit supplied by the admitted package and FR-014's generated oracle.
- For each operand, the carried-quantity leaf parameter of the
  `quire.op.quantity.convert` application the operand is: its fixed source unit, its
  carrier domain, the fixed exact target unit of the conversion and, for `pow`, the
  literal integer exponent carried by the operation node.
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

A quantity operand is composable when it is a reader-admitted application of
`quire.op.quantity.convert` whose mode is `exact`, whose result type is an exact
`unit` or `compound_unit`, and whose source is a `value`/`parameter` typed at a
`scalar_type`/`carried_quantity`. When the operation is representable, CG SHALL
draw only that leaf parameter symbolically, from its carrier's inclusive domain
through the carrier, and SHALL NOT draw the converted magnitude: the converted
magnitude is computed by the owner's conversion over the drawn leaf. When evaluating the composed operation, the harness SHALL evaluate the operands in the node's operand order, each conversion
completely before the next, and then the operation, on one Meter; the charges
are the conversions' in operand order followed by the operation's. Each converted
operand's unit is the conversion's fixed target unit, recorded through
`UnitScope::form` and resolved to a QSV `UnitQuantity<'_>` as FR-014-AC-48,
AC-52 and AC-59 state; a unit is never drawn symbolically and CG creates no unit
id.
A `pow` exponent is the node's literal integer, never a symbolic value, because the
QSV power operation fixes the result unit at type time. Representation constraints
apply before evaluation, admit every declared leaf value exactly once and do not
depend on the outcome.

When evaluating an arithmetic operation, the harness SHALL compare the FR-014
generated oracle with a native expression that calls the owner's public
`evaluate_quantity_unit` independently, from matching entry Meter state and limits,
over the full typed outcome, the identified result unit on every `Ok`, ordered
charges and consumed counters. The native expression converts each leaf by calling
the owner's public `convert_quantity` with `QuantityTarget::Exact` independently,
and never reuses the generated conversion oracle. A comparison calls
`compare_quantity`, returns an `Outcome<bool>` with no unit, and is compared on outcome, charges and counters only.
Typed refusals are expected observations, preserved as distinct causes and never
assumed away. A stopped outcome never becomes a completed value.

When asserting, the harness SHALL emit exactly one final completion cover after all
assertions. No assumption depends on the operation's outcome.

When decoding retained playback, CG SHALL decode the physical bindings that the
leaf carrier's FR-025 row defines (FR-025-AC-18), validate them against the
original carrier domain before reconstruction and submit one owner carried quantity
per leaf parameter, not per converted operand, through the public replay route, as
FR-025 states for Rational. An unavailable route is reported as
unsupported, not as an accepted replay.

When the carrier, a leaf carrier domain, the literal exponent, the resolved
operand form or the native expression is unavailable, the item is refused with the
dispositions below and no harness. A quantity leaf inside a composite remains
FR-015's `ShadowFamilyNotBuilt`.

## Acceptance Criteria

All supported-harness criteria are conditional on P1.

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-050-AC-1 | Each admitted quantity `add`, `sub`, `mul`, `div`, literal-exponent `pow` and comparison node selects a harness that names the operation identity, operand forms and units from the admitted node; compound-unit result forms admitted by FR-014 (#750-#754) are accepted without a new operand form, and a conversion into a carried target is not a composable operand (AC-5). A conversion node as the requested root is not selected here; as an operand it is composed (AC-11). | Test |
| FR-050-AC-2 | Symbolic leaf values are drawn from the exact inclusive carrier domain of each operand's leaf parameter through the carrier, each declared leaf value has exactly one accepted representation, and a clipped endpoint, duplicate encoding or extra value fails the correspondence check (as FR-046-AC-2); the converted magnitude is never drawn (AC-11). Units are fixed and resolved through `UnitTable::resolve` or `UnitScope::resolve`; no symbolic unit, CG-created unit id or unrelated metadata pairing is generated. | Test |
| FR-050-AC-3 | For arithmetic operations, at the operation stage of AC-11's pipeline, the generated oracle and an independent native expression over public `evaluate_quantity_unit` agree, from matching entry state and limits, on completed value, identified result unit, typed outcome, ordered charges, denied next charge and consumed counters, including zero, exact and short budgets. Comparisons agree on `Outcome<bool>`, charges and counters, with no unit asserted. The reference neither runs the generated oracle twice nor bridges through Runtime. | Test |
| FR-050-AC-4 | Each of these is asserted as the agreed typed outcome with the owner's charges, not assumed away: `IllTyped` dimension mismatch (two leaves converted to targets of different dimensions), `IllTyped` distinct units of one dimension (two leaves converted to an `m` and a `cm` target and added), `Identity` refusal, arithmetic on an affine unit, `DivisionByZero` for division by a zero quantity, `DivisionByZero` for a zero base with a negative literal exponent, and a large literal exponent, which has no range refusal and yields the owner's `IntegerBits` limit-incomplete stop at `UnitRationalArithmetic`. No cause is relabelled and a stopped outcome is never proved completed. The generated harness contains no assumption that depends on the operation's outcome (inspected, as FR-046-AC-4). | Test |
| FR-050-AC-5 | With P1 available, a bare exact `unit` or `compound_unit` parameter operand (Contract IR's `requires_bound` of FR-038-AC-73 reaching CG; FR-014's `RequiresBound`, AC-60 and AC-25) yields source-associated `requires-bound` naming the operand with no harness. A `bounded_quantity` operand, which the reader admits to `add`, `convert` and `lt` (Contract IR FR-038-AC-486) and which is bounded but forms no finite domain, yields `unsupported` with reason `NoFiniteEncoding` naming the operand's node identity and tag (the reason carries no interval; it stays in the diagnostic evidence). A conversion into a carried target, a conversion of a non-carried source, a quantity-literal operand and a nested operation operand each yield `unsupported` as an unsupported operand form naming the operand. One leaf parameter reaching two operand positions and a `pow` whose exponent is not a literal integer each yield `unsupported` naming the parameter or exponent. These cases are reachable only at CG's private defensive seam, because the reader refuses them: a carried operand that is not under a conversion (Contract IR rejects it for every quantity operation but `convert`) and a leaf whose carrier domain is missing or unbounded, which yields `requires-bound` (FR-014's `MissingBound`, AC-23). A leaf endpoint beyond `i64` capacity per AC-13 yields named `unsupported` retaining the endpoint; generation never clips or substitutes a smaller domain. | Test |
| FR-050-AC-6 | When P1 (K-EXACT-quantity, IR-1443) is unavailable the item remains settled as TC-030's Quantity scalar row states, `OperationNotRendered`, with the `NoRenderer` settlement carrying no prerequisite name (a typed reason names it only where one is carried), with no harness and no Verified claim. Unsupported operand form (including a quantity-literal operand and a nested operation operand, neither of which this row composes), absent native expression and a quantity leaf of a composite each keep their own named disposition, with total per-item accounting under FR-039 and an unchanged supported sibling. | Test |
| FR-050-AC-7 | A healthy quantity operation receives `Verified` only from an actual successful proof with all checks and exactly one final completion cover satisfied. Unsatisfied cover, failed unwind, timeout, memory exhaustion or missing verdict cannot yield `Verified`, and FR-017 and FR-029 classifications are retained. | Test |
| FR-050-AC-8 | Mutants fail with actual solver playback of an in-domain input, over control domains that distinguish them: wrong operator (`add` for `sub`) with a nonzero second operand; swapped operands in `sub`, `div`, `lt`, `le`, `gt` and `ge` (not `eq` or `ne`, where the swap is equivalent) with distinct nonzero operands; `lt` for `le` or `gt` for `ge` with equal operands; a result unit differing from the owner's identified unit; a `pow` result for a different literal exponent over a base whose magnitude is not 0, 1 or -1; a dropped or reordered charge; and a stopped outcome reported completed. The control domain for each must contain its distinguishing point. | Test |
| FR-050-AC-9 | Counterexample decoding validates every physical binding of the leaf carrier's FR-025 row (FR-025-AC-18: width, order, comment, schema role, byte and entry-count limits, canonical form and original carrier domain) before reconstructing one owner carried quantity per leaf parameter, paired with its fixed source unit, and submits it through the public replay route for that exact declaration. Missing, duplicate, foreign, role-swapped, out-of-domain or noncanonical evidence is refused with a typed cause and no replay; an unavailable replay capability is reported without claiming acceptance. | Test |
| FR-050-AC-10 | Run evidence records the operation and source association, leaf carrier domains with correspondence, the literal exponent, fixed leaf source units and conversion target units, subject and full ordered backend arguments, unwind bounds, tool versions, target, dependency cohort including the carrier, limits, raw checks, cover and playback, exit and typed terminal outcome. No-run, refusal, cover failure, resource exhaustion and semantic falsification remain distinguishable. | Test |
| FR-050-AC-11 | Each supported operation node whose operands are composable (a reader-admitted exact `quire.op.quantity.convert` of a `carried_quantity` leaf parameter) selects a harness that draws only the leaf parameters and evaluates the operand conversions in operand order, then the operation, on one Meter. The generated oracle and an independent native expression over the public `convert_quantity` (`QuantityTarget::Exact`) and `evaluate_quantity_unit` or `compare_quantity` agree on completed value, identified result unit, typed outcome, ordered charges (each conversion's in operand order, then the operation's), denied next charge and consumed counters at zero, exact and short budgets, with each converted operand's unit equal to the conversion's fixed target unit. The reference neither reuses the generated conversion oracle nor bridges through Runtime. | Test |
| FR-050-AC-12 | A conversion-stage stop is the whole outcome and is asserted equal to the native one: an `IllTyped` conversion with zero charges for that conversion (FR-014-AC-49) and every earlier operand conversion's admitted charges and counters unchanged, or an incomplete outcome at a conversion charge under a short budget, leaves the operation unrun with no operation charge and no later operand converted, and is never proved completed. The `IllTyped` case is reachable from a hand-built package the reader admits, because the reader does not check dimensions for a carried-to-exact conversion; real QSL source with such a conversion is refused upstream. Independently of the native comparison, the harness asserts that no conversion-stage outcome is a refusal for an inexact or out-of-domain conversion, because the target is exact: a control substituting a carried `exact` `Int[..]` target for one conversion, over an off-grid leaf (`1 in`, FR-014-AC-64), makes that assertion fail with playback. | Test |
| FR-050-AC-13 | A leaf's physical bindings are those of its carrier's FR-025 row (FR-025-AC-18): an `Int[..]` carrier has one `i64` inside `[lo, hi]`; a `Decimal[..]` carrier has the coefficient then scale pair of the Decimal row (the raw coefficient may fall outside `[lo, hi]` for the owner's retained form; the scale is within `0..=scale_max` and `u32`), whose lifted bound `max(\|lo\|,\|hi\|) x 10^(scale_max - scale_min)` must fit `i64`; a `Rational[..]` carrier has the numerator then denominator pair of the Rational row, the numerator inside `[n1, n2]`, the denominator inside `[d1, d2]` and the pair canonical. An `Int[..]` endpoint, a Decimal lifted bound or scale endpoint, or a Rational numerator or denominator endpoint that does not fit `i64` (or, for the scale, `u32`) yields `unsupported` retaining it (AC-5), checked per component and before any harness is emitted. The converted magnitude, computed by the owner in unbounded integers, is not subject to the `i64` fit. | Test |
| FR-050-AC-14 | These composition mutants fail with actual solver playback of an in-domain leaf input, each over a control domain stating its distinguishing point. One operand's conversion skipped (its leaf read as already in the target unit), over a leaf whose source unit scale differs from the target's and a nonzero leaf magnitude, with a nonzero other operand for `mul` and `div`, a literal exponent other than 0 for `pow`, and for a comparison a `cm` leaf of 60 against an `m` leaf of 7/10 (`lt`, `le`, `gt`, `ge`) or a `cm` leaf of 70 against the same `m` leaf (`eq`, `ne`, which are equal only after conversion). The conversion's charges dropped, and separately moved after the operation's charges, under a budget that charges the conversion. A conversion target replaced by a different unit of the same dimension, over a nonzero leaf with a literal exponent other than 0 for `pow`: an arithmetic operation differs in result unit or value, and a comparison whose two operands share one target unit refuses `IllTyped` for distinct units where the unmutated pipeline completes. The assertions of AC-11 are the operators that catch them. | Test |

## Dependencies

- [FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md) owns quantity
  oracle semantics (AC-6, AC-48..AC-52, AC-59..AC-66); its oracle agreement is not
  this proof.
- K-EXACT-quantity (IR-1443) is prerequisite P1: it must build an owner carried
  quantity from the leaf carrier's physical bindings (AC-13). IR-1497 decided this
  carried-leaf composition; it needs no Contract IR or QSpec change, as the language
  and FR-038 already admit the conversion of a carried parameter.
  IR-1439 is the row's specification ticket and IR-263 the MVP row it serves.
- The Rational and Decimal leaf encodings are those of
  [FR-046](./FR-046-rational-function-predicate-harnesses.md) and
  [FR-051](./FR-051-decimal-scalar-harnesses.md), reused through FR-025-AC-18.
- TC-030's Quantity scalar row owns today's `OperationNotRendered` settlement; this
  row supersedes it for non-conversion quantity operations only when delivered.
- [FR-015](./FR-015-bounded-kani-obligations.md), [FR-025](./FR-025-generated-subject-abi.md)
  and [FR-028](./FR-028-bounded-proof-ceilings.md) own the shared refusals and ABI row.
- [TC-064](../matrix/TC-064-quantity-scalar-harnesses.md) defines the planned
  verification.

## Status

PLANNED (IR-1439). This specification supplies no implementation or
generated-proof credit. All fourteen criteria remain planned and untagged until P1 is delivered with the IR-269 quantity harness. Oracle agreement under FR-014
is not Kani credit.
