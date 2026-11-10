---
id: FR-051
title: "Generate bounded decimal scalar operation harnesses"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-016
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
  - target: ix://agent-ix/quire-contract-ir/FR-038
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-046
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-050
    type: references
  - target: ix://agent-ix/quire-specification/FR-196
    type: references
---
# FR-051: Generate bounded decimal scalar operation harnesses

## Description

When an admitted request selects a reader-admitted `decimal` scalar operation
(`quire.op.decimal.add`, `.sub`, `.mul`, `.div`, `.negate`,
`quire.op.numeric.convert_rounding` into a decimal, or an ordering comparison
`lt`, `le`, `gt`, `ge` over decimals) and the planned prerequisite below is
available, CG SHALL generate a bounded Kani harness that proves the generated
decimal oracle of
[FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md) agrees with an
independent native expression of the same operation over every admitted symbolic
decimal operand. This is the decimal row of the IR-269 scalar renderer that
TC-030's Decimal scalar row settles today as `OperationNotRendered`, and the
row that IR-259 waits on. It is a planned row, not a new IR schema, a new
oracle or a claim of current Rust support.

Prerequisite P1 is the quire-exact decimal carrier (ticket IR-1444): a symbolic
exact decimal operand and a native expression over it. It is not delivered.
Every criterion that describes a supported harness is conditional on P1. The
operand bound mechanism needs no further prerequisite: a bounded Decimal is
already the admitted `decimal_range` form (`coefficient_min`, `coefficient_max`,
`scale_min`, `scale_max`, `rounding`). Under a bounds-required profile (Contract IR
FR-038) an unbounded decimal parameter is `requires_bound` upstream, and FR-014
reports `RequiresBound` for an unbounded operand or arithmetic result bound; a
profile that does not require bounds is outside this row, and CG reports the
missing bound defensively (AC-5).

Decimal equality and inequality (`quire.op.decimal.eq`, `.ne`) are outside this
requirement: FR-014 refuses them as `OperationNotDerivable`, so no oracle exists
to prove. Quantity carried-Decimal conversions are a quantity operation: FR-014-AC-62
through AC-65 own their oracle, and FR-014-AC-51 and TC-030's Quantity scalar row
own their Kani settlement. FR-050 excludes conversions, so no row here or there
proves them.

## Inputs

- A requested decimal operation node, its operand forms and the `decimal_range`
  bound of each operand, as admitted by FR-014. For `add`, `sub`, `mul`, `div`,
  `negate` and `convert_rounding` also the `decimal_range` result bound, which
  supplies the owner `DecimalType` target and its rounding. The ordering
  comparisons return `Outcome<bool>`, take no `DecimalType` and have no result
  bound.
- The K-EXACT-decimal carrier (P1).
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

When the operation is representable, the harness SHALL draw each operand as a
retained representation, a coefficient and a scale, from the node's `decimal_range`
through the carrier. The owner's operations charge and place by the retained
representation (`Decimal::representation()`), and `DecimalResult` identity is the
retained representation plus the loss record, so two numerically equal operands
with different scales (`1.0` and `1`) are distinct inputs. The owner retains every
result at the target's maximum scale `T = scale_max`, as `(c x 10^(T-s), T)`; for
`Decimal[0,100;0,2]` the value 50 is retained as `(5000, 2)`, whose raw coefficient
lies outside `[coefficient_min, coefficient_max]`. The bound therefore applies to
the owner's value-only membership, not to the raw coefficient. The representation
constraint is `DecimalType::contains` over the drawn pair at scales `0..=scale_max`,
within a finite coefficient envelope that the carrier (P1) states and that contains
every such member, including the owner's own result form. The constraint applies
before evaluation and does not depend on the outcome. Representations whose scale
exceeds `scale_max` are outside this encoding, and no proof claims them.

When evaluating, the harness SHALL compare the FR-014 generated oracle with a
native expression that is independent of that oracle (not a second call of the same
function), from matching entry Meter state and limits. For `add`, `sub`, `mul`,
`div`, `negate` and `convert_rounding` the comparison covers the full typed
outcome, the completed value's retained representation and its loss record,
ordered charges and consumed counters. For `lt`, `le`, `gt` and `ge` it covers the
`Outcome<bool>`, charges and counters. Ordering is by value, not by retained
representation.

The typed outcomes the owner produces are these. `Completed` (with a loss record
when a rounding step ran). `Undefined(DivisionByZero)` for a divisor whose
normalized coefficient is zero, produced after the `decimal.operands` charge.
`Refused(InexactDecimal)` under `exact` rounding only, before any
`decimal.rounding` charge or loss record, whenever the exact result is not a
multiple of `10^-T`: a `div` quotient that does not terminate by `T`, a `mul` whose
operand scales sum above `T`, and `add`, `sub`, `negate` or `convert_rounding` over
an operand whose scale exceeds `T` (the result bound is independent of each
operand's own bound). `Refused(DecimalOutOfDomain)` when the placed result is
outside the result `DecimalType`, reachable for `add`, `sub`, `mul`, `div`,
`negate` and `convert_rounding`, and after rounding under a nearest or directed
mode. Comparisons produce only `Completed` or `Incomplete`. `Incomplete` at a charge point: `decimal.operands`,
`decimal.scale-expansion`, `decimal.arithmetic`, `decimal.rounding` (only on a
rounding step) or `decimal.result-retain`, and for comparisons `ordering.operands`,
`ordering.arithmetic` or `ordering.result-retain`, with its limit kind. A malformed
`DecimalType` is `IllTyped` before any charge and is refused by the reader, so it
is not a drawn control. None is assumed away, and a stopped outcome never becomes
a completed value.

When asserting, the harness SHALL emit exactly one final completion cover after all
assertions. No assumption depends on the operation's outcome.

When the carrier, a finite `decimal_range` bound, a result bound, the resolved
operand form or the native expression is unavailable, the item is refused with the
dispositions below and no harness. A decimal leaf inside a composite remains
FR-015's `ShadowFamilyNotBuilt`.

### Symbolic ABI and witness roundtrip (planned)

When expanding a logical decimal operand, CG SHALL persist exactly two physical
`i64` bindings in coefficient-then-scale order for its one real parameter,
grouped to the original parameter identity, role, inclusive component bounds and
complete `decimal_range` domain. Logical parameter order remains ascending by
parameter identifier; Boolean and integer ordering is unchanged. The carrier
(P1) owns the exact binding facts; these are required conceptual facts, not a
claim that a current Rust field carries them.

When decoding retained playback, CG SHALL validate each physical entry in
persisted call order with the existing I64 width, little-endian byte and comment
checks and the byte and value-count limits (two entries per decimal parameter)
before grouping, then check the scale is non-negative and within `u32`, and that
the pair satisfies the original domain and `DecimalType::contains`. A missing,
duplicate, foreign, role-swapped or domain-conflicting schema refuses with a typed
cause, malformed entries retain typed decode causes, and no replay is attempted
for these evidence failures.

When submitting a validated parameter to public replay, CG SHALL construct one
`qsl_replay::WitnessValue::Decimal { coefficient, scale }` with owner Integer
values, which retains the pair unnormalized, per parameter. If the public route is
unavailable the item is unsupported with no harness. CG introduces no transcript
grammar or occurrence key.

## Acceptance Criteria

All supported-harness criteria are conditional on P1 (IR-1444).

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-051-AC-1 | Each admitted decimal `add`, `sub`, `mul`, `div`, `negate`, `convert_rounding` and each of `lt`, `le`, `gt`, `ge` node selects a harness naming the operation identity and operand forms from the admitted node, and for the six arithmetic operations the `decimal_range` result bound (orderings have none); the oracle is FR-014's and the expected side is an independent native expression, not a second call of the oracle. `quire.op.decimal.eq` and `.ne` select no harness and keep FR-014's `OperationNotDerivable`. | Test |
| FR-051-AC-2 | Each operand is drawn as a (coefficient, scale) pair at scales `0..=scale_max` that satisfies the owner `DecimalType::contains`, within the carrier's stated coefficient envelope. Every such member is reachable exactly once, including numerically equal pairs with different scales and the owner's own result form (`(5000, 2)` for 50 in `Decimal[0,100;0,2]`), and an independent host enumeration of the `contains`-admitted set matches the encoding; clipping an endpoint, rejecting a raw coefficient outside `[coefficient_min, coefficient_max]` that `contains` admits, admitting a pair `contains` rejects, collapsing equal-valued pairs or drawing only the target scale fails the correspondence check (QSpec FR-196-AC-5). | Test |
| FR-051-AC-3 | For `add`, `sub`, `mul`, `div`, `negate` and `convert_rounding` the oracle and native expression agree on the typed outcome, the completed value's retained representation (not only its normalized value), the loss record, ordered charges, the denied next charge and consumed counters, at zero, exact and short budgets at each charge point in the Behavior list. For the four ordering operators they agree on `Outcome<bool>`, charges and counters, with ordering by value. The reference neither runs the generated oracle twice nor bridges through Runtime. | Test |
| FR-051-AC-4 | Each reachable noncompletion is asserted as the agreed typed outcome and never assumed away: `Undefined(DivisionByZero)` for a zero divisor (and `Incomplete` at `decimal.operands` instead at a zero budget, since the zero check follows that charge), `Refused(InexactDecimal)` under `exact` rounding for an inexact `div` quotient, a `mul` whose operand scales sum above the result scale, and `add`, `sub`, `negate` and `convert_rounding` over an operand whose scale exceeds the result scale, `Refused(DecimalOutOfDomain)` for an `add`, `sub`, `mul`, `div`, `negate` or `convert_rounding` result outside the result domain (including after rounding), and `Incomplete` at each charge point named in the Behavior list. No cause is relabelled and a stopped outcome is never proved completed. The generated harness contains no assumption that depends on the outcome (inspected, as FR-046-AC-4). | Test |
| FR-051-AC-5 | A missing, unbounded or repeated `decimal_range` for an operand, or, for the six arithmetic operations only, a missing or ambiguous result bound, yields source-associated `requires-bound` naming the operand or result with no harness. A coefficient endpoint beyond signed `i64`, or a scale endpoint beyond the carrier or `u32`, yields named `unsupported` retaining the endpoint; generation never clips or substitutes a smaller domain. | Test |
| FR-051-AC-6 | When P1 (IR-1444) is unavailable the item remains settled as TC-030's Decimal scalar row states, `OperationNotRendered` (`NoRenderer`), carrying no prerequisite name, with no harness and no Verified claim. An unsupported operand form, an absent native expression and a decimal leaf of a composite each keep their own named disposition, with total per-item accounting under FR-039 and an unchanged supported sibling. | Test |
| FR-051-AC-7 | A healthy decimal operation receives `Verified` only from an actual successful proof with all checks and exactly one final completion cover satisfied. Unsatisfied cover, failed unwind, timeout, memory exhaustion or missing verdict cannot yield `Verified`, and FR-017 and FR-029 classifications are retained. | Test |
| FR-051-AC-8 | Each mutant below fails with actual solver playback of an in-domain input, caught by the listed operators over a control domain that contains its distinguishing point. (a) `add` for `sub` (and the converse): `add` and `sub`, nonzero second operand. (b) Operand swap: `sub`, `div`, `lt`, `le`, `gt`, `ge`, over numerically distinct operands (for `div`, nonzero with `a` not equal to `-b`; for `sub`, `a - b` differs from `b - a`); `add` and `mul` swaps are equivalent and are not claimed. (c) `lt` for `le`, `gt` for `ge` (and converses): the matching ordering operator, equal-valued operands. (d) Ordering by raw coefficient only, ignoring scale: the four ordering operators over operands whose scales differ and whose raw coefficient order disagrees with value order, for example `1.0` (10, scale 1) against `2` (2, scale 0), where the coefficients order 10 above 2 but the values order 1.0 below 2. (e) A different rounding mode (`nearest-even` for `nearest-away`): `div`, `mul` and `convert_rounding` over a tie point such as 0.125 to scale two. (f) A result reported by normalized value instead of retained representation, or a wrong target scale: every arithmetic operator, over operands with trailing-zero or lower-scale results, for example `0.5 + 0.5` at scale two. (g) A dropped loss record: `div`, `mul` and `convert_rounding` where a rounding step runs. (h) A dropped or reordered charge: every operator. (i) An `InexactDecimal` refusal reported as a rounded value: `div` under `exact`, for example one third at scale two. (j) A zero divisor reported as a value: `div`. (k) A stopped outcome reported completed: every operator. | Test |
| FR-051-AC-9 | Counterexample decoding validates two physical I64 entries per decimal parameter (width, little-endian bytes, comment, order, schema role, byte and value-count limits), then the scale range, the original domain and `DecimalType::contains`, before grouping. Grouping yields one `WitnessValue::Decimal { coefficient, scale }` per parameter through the public replay route for that exact declaration, retaining the unnormalized pair. Missing, duplicate, foreign, role-swapped, domain-conflicting and malformed schemas or entries, a negative or oversize scale and an out-of-domain pair are refused with typed causes and no replay; a pair is never normalized into a different witness. An unavailable replay capability is reported without claiming acceptance. | Test |
| FR-051-AC-10 | Run evidence records the operation and source association, operand and result `decimal_range` bounds and correspondence, ordered physical bindings and their logical grouping, subject and full ordered backend arguments, unwind bounds, Kani/Rust/CBMC/solver versions, target, dependency cohort including the carrier, limits, raw checks, cover and playback, exit and typed terminal outcome. No-run, refusal, cover failure, resource exhaustion and semantic falsification remain distinguishable; preparation alone earns no proof result. | Test |

## Dependencies

- [FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md) owns decimal
  oracle semantics and the derivable operation list; its oracle agreement is not
  this proof.
- K-EXACT-decimal (IR-1444) is prerequisite P1. No further Contract IR bound
  requirement is outstanding for decimals: `decimal_range` is admitted today, and
  under a bounds-required profile an unbounded decimal is `requires_bound` upstream
  (Contract IR FR-038).
- [FR-046](./FR-046-rational-function-predicate-harnesses.md),
  [FR-050](./FR-050-quantity-scalar-harnesses.md) and QSpec FR-196 are the
  precedent and the finite-encoding obligation this row follows.
- TC-030's Decimal scalar row owns today's `OperationNotRendered` settlement; this
  row supersedes it for the listed decimal operations only when delivered.
- [FR-015](./FR-015-bounded-kani-obligations.md), [FR-025](./FR-025-generated-subject-abi.md)
  and [FR-028](./FR-028-bounded-proof-ceilings.md) own the shared refusals and ABI row.
- [TC-065](../matrix/TC-065-decimal-scalar-harnesses.md) defines the planned
  verification.

## Status

PLANNED (IR-1440). This specification supplies no implementation or
generated-proof credit. All ten criteria remain planned and untagged until P1 is
delivered with the IR-269 decimal harness. The public Decimal replay value exists
in `qsl_replay`; the CG decoder grouping and the carrier's physical bindings do
not. Oracle agreement under FR-014 is not Kani credit. The encoding covers retained
scales `0..=scale_max` only.
