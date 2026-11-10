---
id: FR-053
title: "Generate bounded IEEE float scalar operation harnesses"
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
  - target: ix://agent-ix/quire-contract-codegen/FR-049
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-050
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-051
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-052
    type: references
  - target: ix://agent-ix/quire-specification/FR-196
    type: references
---
# FR-053: Generate bounded IEEE float scalar operation harnesses

## Description

When an admitted request selects a reader-admitted IEEE float scalar operation over
parameters typed `float32` or `float64`, and the planned prerequisites below are
available, CG SHALL generate a bounded Kani harness that proves the generated IEEE
oracle of [FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md) agrees
with an independent native expression of the same operation over every symbolic
operand bit pattern. The operations are the comparisons `quire.op.ieee.numeric_equal`,
`.total_order` and `.bit_identical`, the arithmetic `quire.op.ieee.float32.add`,
`.sub`, `.mul`, `.div` and the `float64` forms, and the planned `quire.op.ieee.float32.sqrt`,
`.fma` and the `float64` forms. This is the IEEE float row of the IR-269 scalar renderer
that TC-030's IEEE-float scalar row settles today as `OperationNotRendered`, and the
specification part of the row that IR-260 implements. It is a planned row, not a new
IR schema, a new oracle or a claim of current Rust support.

Three prerequisites are named, and none is delivered. Every criterion that describes a
supported harness is conditional on P1, the criteria about unbounded operands are
conditional on P2, and the `sqrt` and `fma` parts of every criterion are conditional on
P3.

- P1 (ticket IR-1446) is the quire-exact float carrier: a symbolic IEEE operand, and a
  native expression for the comparisons and for add, sub, mul, div, sqrt and fma at
  `binary32` and `binary64` under each of the six rounding modes, with the tractability
  contract of the IR-1436 design review (D4): `Copy`, no allocation, machine width, no
  iterator loop, and none of the owner's `BigInt` or `BigUint` arithmetic. IR-1446 is
  asked (by a Linear comment on IR-1446, not yet in its acceptance criteria) to also supply a
  host-callable replacement of the owner's `evaluate_ieee` and `compare_ieee`, which AC-13
  needs. Until IR-1446 carries it, AC-13 cannot run.
- P2 (ticket IR-69) is the Contract IR ruling that a `float32` or `float64` is
  unbounded unless a `float_rounding` domain applies (planner ruling, Option A). It adds
  both forms to the `requires_bound` unbounded set in
  `crates/quire-contract-model/src/checked_package/v2/lower.rs` and moves FR-038 Behavior
  and FR-038-AC-8 from eight forms to ten. It is not implemented: that predicate returns
  `false` for `ScalarTypeForm::Float32` and `ScalarTypeForm::Float64` on Contract IR main
  at the time of writing, so a package with an unbounded float lowers to `lowered` under
  a bounds-required profile today. Until P2 lands, AC-3's unpinned-operand refusal is
  CG's own defensive check and nothing upstream backs it.
- P3 (ticket IR-1502) is the FR-014 extension that derives and generates an oracle
  for `quire.op.ieee.float32.sqrt`, `.float64.sqrt`, `.float32.fma` and `.float64.fma`.
  FR-014 refuses these four identities as `OperationNotDerivable` today, so no oracle
  exists to prove. The owner supports both (`IeeeOperation::SquareRoot`,
  `IeeeOperation::FusedMultiplyAdd`), and Contract IR catalogues both identities, so
  the missing piece is CG's descriptor, derivation and renderer. Without P3 the `sqrt`
  and `fma` items keep FR-014's `OperationNotDerivable` and this row proves nothing
  about them.

The CG float ABI, the decoder and the replay grouping are IR-260's own implementation
work under this requirement, not prerequisites. The IR-1436 review orders the Meter
tractability work (IR-844, IR-241) before the carriers because every owner oracle
charges a Meter; that ordering is context and adds no criterion here.

Outside this requirement: `quire.op.ieee.to_float32` and `.to_float64` (width
conversions, a different operation kind), `quire.op.ieee.from_exact`,
`quire.op.ieee.to_rational`, `quire.op.ieee.float32.negate`, `quire.op.ieee.float64.negate` and every other identity FR-014
refuses or does not derive keep their FR-014 dispositions and, for a claim, TC-030's.
A float literal operand and an operand that is not a parameter are outside this row
(AC-1).

### Tractability and its ceiling

The owner's `compare_ieee` decodes a bit pattern and compares bits, with no
`BigInt` on that path. The owner's `evaluate_ieee` does not. It forms the exact
intermediate with `BigInt` and `BigUint`, aligns operands by shifts of up to about 2050
bits at `binary64` for add and sub and about 3120 bits for `fma` (an exact product down to
exponent -2148 against an addend up to 971), and takes `sqrt` through a Newton `integer_square_root` loop. None of
that is tractable as written, so the proof path for arithmetic rests entirely on the
carrier (P1). This requirement claims no operation, width or rounding mode is tractable.
The expected ordering of cost, which is unmeasured, is: the three comparisons and the
classification paths (bit operations, no multiplication or division), then `binary32`
add, sub and mul, then `binary32` div, sqrt and fma, then the `binary64` forms, where a
53-bit by 53-bit product, a 56-iteration quotient or root loop and a wide alignment are
the known cost drivers for the solver. A division or root loop of at most p + 3
iterations (27 at `binary32`, 56 at `binary64`) needs an unwind bound above that count.
A width, operation and rounding mode is supported only for the runs the carrier records
within the request's ceilings (FR-028), and each combination is a separate harness
because the rounding mode is a constant of the node, not a drawn input. Any run outside
the ceilings is `inconclusive` under FR-028 and never `Verified`. Whether the comparisons
need the carrier at all, or the owner's `compare_ieee` suffices on the proof path, is an
IR-1446 finding to record, not an assumption made here.

## Inputs

- A requested float node, its operation identity, its parameter operands and their float
  forms, and, for the arithmetic operations, the `float_rounding` result bound that the
  FR-014 descriptor equals (it supplies the rounding mode), as admitted by FR-014 and
  the strict Contract IR reader. A comparison returns `Outcome<bool>`, takes no rounding
  mode and has no result bound.
- The float carrier (P1) and, for `sqrt` and `fma`, the FR-014 oracle (P3).
- The proof subject, backend and ceilings of
  [FR-015](./FR-015-bounded-kani-obligations.md),
  [FR-025](./FR-025-generated-subject-abi.md) and
  [FR-028](./FR-028-bounded-proof-ceilings.md); per-item accounting is
  [FR-039](../../core/functional/FR-039-contract-package-consumption.md)'s.

## Outputs

A supported item has its harness, symbolic-input correspondence and source-associated
proof record. A refused item has its named disposition and no harness. Execution and
terminal outcomes remain [FR-017](./FR-017-kani-execution-evidence.md) and
[FR-029](./FR-029-run-outcome-terminal-record.md)'s; counterexample replay remains
[FR-016](../../replay/functional/FR-016-witness-native-replay.md)'s.

## Behavior

### Symbolic domain

When the operation is representable, the harness SHALL draw each operand as one `i64`
bit pattern. A `float32` operand is constrained to `0..=0xFFFF_FFFF` and built as
`IeeeValue::binary32(bits as u32)`. A `float64` operand is the same `i64` reinterpreted
as `u64` and built as `IeeeValue::binary64(bits as u64)`, so a pattern with the sign bit
set is a negative `i64`. Nothing else constrains the draw. Every pattern is an owner value:
both signs of zero, every subnormal, every normal, both infinities, and every quiet and
signaling NaN of every sign and payload. Each pattern is reachable exactly once. The
`float_rounding` pin is a rounding mode and not a value range, so it narrows no operand
and no bound is encoded into the bit pattern; there is no clipping and no named
unsupported domain. Two distinct parameters are drawn independently, and one parameter
referenced by both operand slots is drawn once. No constraint depends on the operation's
outcome.

### Evaluation

When evaluating, the harness SHALL compare the FR-014 generated oracle with a native
expression that is independent of that oracle (not a second call of the same function),
from matching entry Meter state and limits. For the comparisons the comparison covers the
`Outcome<bool>`, ordered charges, the denied next charge and consumed counters. For the
arithmetic operations it covers the typed outcome, the completed result's bit pattern,
its flag set (`invalid`, `divide_by_zero`, `overflow`, `underflow`, `inexact`) and its
provenance (width and rounding mode), ordered charges, the denied next charge and
consumed counters. The expected side of a comparison decides `bit_identical` by bit
equality, `numeric_equal` by "neither operand is NaN and either both are zero or the bits
are equal", and `total_order` by the totalOrder predicate (`a` precedes or equals `b`).
The expected side of an arithmetic operation is a second implementation in the carrier crate
(P1) under the node's rounding mode. It SHALL share no function with the replacement that
the oracle runs under `cfg(kani)`, apart from the decode of a bit pattern into
sign, exponent and fraction, and it neither calls nor reproduces the owner's `BigInt`
path. Under `cfg(kani)` the oracle is the owner's function with the carrier's replacement
substituted for its arithmetic, so the proof ties that replacement to the second
implementation and nothing else: it covers the `cfg(kani)` build, not the shipped oracle
that runs without it. A defect in the decode changes both
sides and is invisible to the proof. The shipped oracle is tied to the replacement only by
AC-13's host differential, and AC-13 is the only criterion that can catch such a
decode defect. AC-14 inspects that nothing but the decode is shared.

### Control points

The controls below name their points once, so that every mutant of AC-8 through AC-10 is
paired with an operator and an input that can make it fail. Bit patterns are given as
`binary32` / `binary64`. AC-13 asserts the expected result of every point against the
shipped owner on the host before a mutant is run against it.

| Name | `binary32` | `binary64` |
| --- | --- | --- |
| `Z+`, `Z-` | `0x00000000`, `0x80000000` | `0x0000000000000000`, `0x8000000000000000` |
| `ONE`, `MONE`, `TWO`, `MTWO` | `0x3F800000`, `0xBF800000`, `0x40000000`, `0xC0000000` | `0x3FF0000000000000`, `0xBFF0000000000000`, `0x4000000000000000`, `0xC000000000000000` |
| `HALF`, `THREE`, `FOUR`, `FIVE` | `0x3F000000`, `0x40400000`, `0x40800000`, `0x40A00000` | `0x3FE0000000000000`, `0x4008000000000000`, `0x4010000000000000`, `0x4014000000000000` |
| `INF+`, `INF-` | `0x7F800000`, `0xFF800000` | `0x7FF0000000000000`, `0xFFF0000000000000` |
| `QN0`, `QN1`, `QN2` (quiet NaNs, payloads 0, 1, 2) | `0x7FC00000`, `0x7FC00001`, `0x7FC00002` | `0x7FF8000000000000`, `0x7FF8000000000001`, `0x7FF8000000000002` |
| `SN1` (signaling NaN, payload 1) | `0x7F800001` | `0x7FF0000000000001` |
| `NQN0`, `NSN1` (negative quiet and signaling NaN) | `0xFFC00000`, `0xFF800001` | `0xFFF8000000000000`, `0xFFF0000000000001` |
| `SUBMIN`, `SUBMAX`, `NORMMIN`, `MAXF` | `0x00000001`, `0x007FFFFF`, `0x00800000`, `0x7F7FFFFF` | `0x0000000000000001`, `0x000FFFFFFFFFFFFF`, `0x0010000000000000`, `0x7FEFFFFFFFFFFFFF` |

Rounding-mode points. The six modes are `exact`, `toward-zero`, `toward-positive`,
`toward-negative`, `nearest-even` and `nearest-away`. `u` is one unit in the last place of
`ONE`. `T1` is `add(ONE, 2^-24)` (a tie; `2^-24` is `0x33800000`; at `binary64` it is
`add(ONE, 2^-53)` with `2^-53` as `0x3CA0000000000000`). `T2` is `add(ONE, 1.5 x 2^-24)` (`0x33C00000`; `binary64`
`0x3CA8000000000000`), three quarters of `u`. `T3` is `add(ONE, 2^-25)` (`0x33000000`;
`binary64` `2^-54`, `0x3C90000000000000`), a quarter of `u`. `T4` is `add(MONE, -2^-25)`
(`0xB3000000`; `binary64` `0xBC90000000000000`). With `u+` meaning `ONE + u`:

| Point | nearest-even | nearest-away | toward-zero | toward-positive | toward-negative |
| --- | --- | --- | --- | --- | --- |
| `T1` | `ONE` | `u+` | `ONE` | `u+` | `ONE` |
| `T2` | `u+` | `u+` | `ONE` | `u+` | `ONE` |
| `T3` | `ONE` | `ONE` | `ONE` | `u+` | `ONE` |
| `T4` | `MONE` | `MONE` | `MONE` | `MONE` | `-(u+)` |

Each of the ten pairs of the five non-`exact` modes differs at one of these points
(nearest-even and nearest-away at `T1`; nearest-even and toward-zero at `T2`; nearest-even
and toward-positive at `T1`; nearest-even and toward-negative at `T2`; nearest-away and
toward-zero at `T1`; nearest-away and toward-positive at `T3`; nearest-away and
toward-negative at `T1`; toward-zero and toward-positive at `T3`; toward-zero and
toward-negative at `T4`; toward-positive and toward-negative at `T1`). Under `exact` every
one of `T1` through `T4` is `Refused(IeeeNotExact)` and carries the would-be flag
`inexact`. `ONE + ONE` is exact and `Completed` under `exact`.

Per-operation rounding points, each verified against the owner's rounding rules
(`round_at` and `round` in `ieee.rs`):

- `add`: `T1` through `T4` as above.
- `sub`: `sub(a, b)` is the owner's addition of `a` and the negation of `b`, so `sub`
  has its own points with the second operand negated: `S1` is `sub(ONE, -2^-24)`
  (`0xB3800000`; `binary64` `-2^-53`, `0xBCA0000000000000`), `S2` is `sub(ONE, -1.5 x
  2^-24)` (`0xB3C00000`; `0xBCA8000000000000`), `S3` is `sub(ONE, -2^-25)` (`0xB3000000`;
  `0xBC90000000000000`) and `S4` is `sub(MONE, 2^-25)` (`0x33000000`; `0x3C90000000000000`).
  They give the results of `T1` through `T4` and separate the same pairs. Reusing `add`'s
  operands is wrong: `sub(ONE, 2^-24)` is `0x3F7FFFFF`, exactly representable, and separates
  no mode.
- `fma`: `fma(ONE, ONE, c)` equals `add(ONE, c)`, so `T1` through `T4` apply with `c` as the
  second addend.
- `mul`: `M1` is `mul(a, b)` with `a = b = 0x3F800800` (`1 + 2^-12`) at `binary32` and
  `a = 0x3FF0000004000000`, `b = 0x3FF0000002000000` at `binary64`, an exact tie whose
  lower neighbour has an even significand (`0x3F801000` and `0x3FF0000006000000`).
  Nearest-even, toward-zero and toward-negative give the lower neighbour; nearest-away and
  toward-positive the upper. `M1n` is the same with the first operand negated: nearest-even,
  toward-zero and toward-positive give the lower magnitude; nearest-away and toward-negative
  the upper. `M1` and `M1n` separate every pair except nearest-even against toward-zero,
  which is not claimed for `mul`.
- `div`: `D1` is `div(SUBMIN, TWO)`, an exact midpoint between `Z+` and `SUBMIN`: `Z+`
  under nearest-even, toward-zero and toward-negative, `SUBMIN` under nearest-away and
  toward-positive, with flags `underflow` and `inexact`. It separates nearest-even from
  nearest-away. `Q1` is `div(ONE, THREE)` at `binary32` (`0x3F800000` over `0x40400000`)
  and `div(FIVE, THREE)` at `binary64`: nearest-even, nearest-away and toward-positive give
  the upper neighbour (`0x3EAAAAAB`; `0x3FFAAAAAAAAAAAAB`), toward-zero and toward-negative
  the lower (`0x3EAAAAAA`; `0x3FFAAAAAAAAAAAAA`). `Q1n` negates the dividend: nearest-even,
  nearest-away and toward-negative give the larger magnitude, toward-zero and
  toward-positive the smaller. `D1`, `Q1` and `Q1n` together separate all ten pairs.
- `sqrt`: a square root never ties (an odd significand of p + 1 bits squares to at least
  2p + 1 bits, more than any operand holds), so nearest-even and nearest-away are
  equivalent for `sqrt`, and its result is never negative, so toward-zero and
  toward-negative are equivalent for `sqrt`. Neither pair is claimed. The classes claimed
  are nearest, toward-zero, toward-positive and `exact`. `SQ1` is `sqrt(TWO)`: at
  `binary32` it gives `0x3FB504F3` under nearest, toward-zero and toward-negative and
  `0x3FB504F4` under toward-positive; at `binary64` it gives `0x3FF6A09E667F3BCD` under
  nearest and toward-positive and `0x3FF6A09E667F3BCC` under toward-zero and
  toward-negative. `SQ2` is `sqrt(0x3F80000E)` at `binary32` (nearest and toward-positive
  `0x3F800007`, toward-zero `0x3F800006`) and `sqrt(0x3FF0000000000007)` at `binary64`
  (nearest and toward-zero `0x3FF0000000000003`, toward-positive `0x3FF0000000000004`).
  Nearest against toward-zero is separated by `SQ2` at `binary32` and `SQ1` at `binary64`;
  nearest against toward-positive by `SQ1` at `binary32` and `SQ2` at `binary64`;
  toward-zero against toward-positive by `SQ1`. `sqrt(FOUR)` is `TWO` exactly and separates
  no modes; it is the exactness control only.

### Outcomes and charges

The charges are those of the owner and are expected in this order. A comparison charges
`ieee.operands` (the width in bits against `integer_bits`, and two value occurrences),
then `ieee.result-retain` (one `result_units`). An arithmetic operation charges
`ieee.operands` (width bits, and the arity as value occurrences: two for add, sub, mul and
div, one for sqrt, three for fma). When any operand is a NaN, or the result is decided by
classification alone (an infinity operand, an invalid operation such as `INF+` minus
`INF+`, `0 x INF`, `0 / 0`, `INF / INF`, `sqrt` of a negative nonzero value or of `INF-`,
or a finite nonzero divided by a zero), it then charges only `ieee.result-retain`.
Otherwise (all operands finite, zeros of either sign included, and not a division of a
finite nonzero by zero) it charges `ieee.exact-intermediate` and `ieee.round` (each the
width in bits against `integer_bits`), then `ieee.result-retain`. `sqrt` of `Z+` or `Z-`
takes the finite path. A NaN operand decides the result before any invalid-operation
test.

The typed outcomes the owner produces are these. `Completed` with the Boolean for a
comparison, or with the result bits, flags and provenance for arithmetic. A division of a
finite nonzero by a zero completes with a signed infinity and `divide_by_zero`, and an
invalid operation completes with the canonical quiet NaN and `invalid`; neither is
`Undefined`. A NaN operand completes with the leftmost NaN operand's sign and payload,
quieted, and `invalid` raised when any NaN operand is signaling. `Incomplete` at
`ieee.operands`, `ieee.exact-intermediate`, `ieee.round` or `ieee.result-retain`, with its
limit kind (`integer_bits`, `value_occurrences` or `work_units` at `ieee.operands`, `integer_bits` or
`work_units` at the two arithmetic points and `result_units` or `work_units` at
`ieee.result-retain`; the owner charges one work unit at every charge point, so a short
work budget stops any of the four). `Refused(IeeeNotExact)`
carrying the result width and the would-be flags, only under the `exact` rounding mode,
only on the finite path, after `ieee.round` and before `ieee.result-retain`, so a refused
operation never charges `ieee.result-retain`. A comparison produces no `Refused` and no
`Undefined`, and arithmetic produces no `Undefined`, so none is a drawn control. The
owner's ill-typed cause `DistinctIeeeWidths` is unreachable in a harness because both
operand widths are fixed at generation; operands of different widths are refused before
generation by FR-014. None is assumed away, and a stopped outcome never becomes a
completed value.

When asserting, the harness SHALL emit exactly one final completion cover after all
assertions. No assumption depends on the operation's outcome.

When the carrier, the oracle (for `sqrt` and `fma`), a float operand form, the
`float_rounding` result bound (arithmetic only) or the native expression is unavailable,
the item is refused with the dispositions of AC-1, AC-3 and AC-6 and no harness. A float
leaf inside a composite remains FR-015's `ShadowFamilyNotBuilt`.

### Symbolic ABI and witness roundtrip (planned)

When expanding a logical float operand, CG SHALL persist exactly one physical `i64`
binding, the bit pattern, for its one real parameter, grouped to the original
parameter identity, role, width and any `float_rounding` pin. Logical parameter order
remains ascending by parameter identifier; Boolean and integer ordering is unchanged. The
carrier (P1) owns the exact binding facts; these are required conceptual facts, not a
claim that a current Rust field carries them.

When decoding retained playback, CG SHALL validate each physical entry in persisted call
order with the existing I64 width, little-endian byte and comment checks and the byte and
value-count limits before grouping. For a `float32` parameter it SHALL then check that the
value is within `0..=0xFFFF_FFFF`. A `float64` parameter has no further check, because
every `i64` is a pattern, and the decoder SHALL reinterpret it as `u64` without a checked
conversion that would refuse a negative `i64`. A missing, duplicate, foreign, role-swapped
or width-conflicting schema refuses with a typed cause, malformed entries retain typed
decode causes, and no replay is attempted for these evidence failures.

When submitting a validated parameter to public replay, CG SHALL construct one
`qsl_replay::WitnessValue::Float32(u32)` or `WitnessValue::Float64(u64)` holding the exact
bit pattern per parameter, so that a NaN keeps its sign and payload. If the public route is
unavailable the item is unsupported with no harness. CG introduces no transcript grammar
or occurrence key.

## Acceptance Criteria

All supported-harness criteria are conditional on P1 (IR-1446). The unpinned-operand
refusal of AC-3 is conditional on P2 (IR-69) for its upstream half. The `sqrt` and `fma`
parts of AC-1, AC-4, AC-5, AC-9, AC-10, AC-13 and AC-14 are conditional on P3 (IR-1502).

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-053-AC-1 | Each admitted `quire.op.ieee.numeric_equal`, `.total_order` and `.bit_identical` node over two parameters of one float width, and each `quire.op.ieee.float32.add`, `.sub`, `.mul`, `.div` and `float64` node over parameters, selects a harness naming the operation identity, width, operand forms and, for arithmetic, the `float_rounding` result bound from the admitted node; the oracle is FR-014's and the expected side is an independent native expression, not a second call of the oracle. The `sqrt` and `fma` identities select a harness only after P3 delivers their oracle, and before that keep FR-014's `OperationNotDerivable`. `quire.op.ieee.to_float32`, `.to_float64`, `.from_exact`, `.to_rational`, `.float32.negate`, `.float64.negate` and every other identity select no harness and keep their existing dispositions. A float literal operand is accounted `unsupported` naming `FloatLiteralOperand` and the operand node, and an operand that is neither a parameter nor a literal is accounted `unsupported` naming FR-015's `OperandNotSupported` and the node, each with no harness. | Test |
| FR-053-AC-2 | Each operand is drawn as one `i64` bit pattern, `0..=0xFFFF_FFFF` for `float32` and every `i64` for `float64`, built through `IeeeValue::binary32(u32)` or `binary64(u64)`. Every pattern is reachable exactly once, including both zeros, subnormals, both infinities and every NaN sign and payload, and a host enumeration of the stated class-boundary set (both signs; exponent field 0, 1, the bias, the largest finite and all ones; fraction 0, 1, the largest below the quiet bit, the quiet bit, the quiet bit plus one and all ones) at each width, plus the `i64` values 2^32, -1, `i64::MIN` and `i64::MAX` at `float32`, matches the encoding: an in-range pattern is admitted once and decodes to the same bits, and an out-of-range `float32` value is excluded. Each of these fails the correspondence check (QSpec FR-196-AC-5): an assumption of non-negative `float64` bits (point `0x8000000000000000`), an assumption excluding NaN (`QN0`, `SN1`), excluding infinities (`INF+`), excluding subnormals or zero (`SUBMIN`, `Z+`), a `float32` draw left unconstrained above `u32::MAX` (the value 2^32 aliases `Z+`), a `float32` draw decoded as a signed 32-bit value (`0x80000000`), and a decode that canonicalizes a NaN (`SN1` must keep its payload). | Test |
| FR-053-AC-3 | An arithmetic node whose result is not typed by exactly one reachable `float_rounding` equal to the descriptor's rounding yields source-associated `requires-bound` (`RequiresBound` per FR-014-AC-25) or `MissingBound` (FR-014-AC-21, AC-23), `AmbiguousBound` (AC-22) or `BoundMismatch`, with no harness. A comparison or arithmetic operand whose type reaches no `float_rounding` yields source-associated `requires-bound` naming the operand's type node with no harness; upstream this is the `requires_bound` Contract IR raises once IR-69 lands, and until then it is CG's own check. A `float_rounding` that is unreadable yields named `unsupported` retaining its cause (FR-014-AC-16). No result bound is required, read or reported for a comparison, because a comparison has none. A `float_rounding` pin narrows no operand domain. | Test |
| FR-053-AC-4 | For every supported operation the oracle and native expression agree on the typed outcome, the completed value's bits, flags and provenance (arithmetic) or Boolean (comparison), ordered charges, the denied next charge and consumed counters, at zero, exact and short budgets at each charge point the operation reaches on that input: `ieee.operands`, `ieee.exact-intermediate`, `ieee.round` and `ieee.result-retain` as the Behavior list orders them, with the arity of `sqrt` and `fma`. Classification-decided inputs (NaN, infinity, invalid operation, division by zero) charge only `ieee.operands` and `ieee.result-retain`. The reference neither runs the generated oracle twice nor bridges through Runtime. | Test |
| FR-053-AC-5 | The reachable noncompletions are asserted as the agreed typed outcome and never assumed away: `Incomplete` at each charge point an input reaches, with a short budget for each limit kind that point charges (including `work_units`), and under `exact` `Refused(IeeeNotExact)` with the would-be flags for an inexact, overflowing or tiny-and-inexact finite result (`T1`, `MAXF + MAXF`, `SUBMIN x HALF`, `SQ1`), charging no `ieee.result-retain`. A division by zero and an invalid operation under `exact` are `Completed` with their flag and are not refused. No `Undefined` or comparison `Refused` control is drawn, because the owner produces none, and `DistinctIeeeWidths` is not a harness control. No cause is relabelled and a stopped outcome is never proved completed. The generated harness contains no assumption that depends on the outcome (inspected, as FR-046-AC-4). | Test |
| FR-053-AC-6 | When P1 (IR-1446) is unavailable the item remains settled as TC-030's IEEE-float scalar row states, `OperationNotRendered` (`NoRenderer`), carrying no prerequisite name, with no harness and no Verified claim. A `sqrt` or `fma` item without P3 keeps `OperationNotDerivable`. An unsupported operand form, an absent native expression and a float leaf of a composite each keep their own named disposition, with total per-item accounting under FR-039 and an unchanged supported sibling. | Test |
| FR-053-AC-7 | A healthy float operation receives `Verified` only from an actual successful proof with all checks and exactly one final completion cover satisfied. Unsatisfied cover, failed unwind (including an unwind at or below the carrier's division or root loop bound), timeout, memory exhaustion or missing verdict cannot yield `Verified`, and FR-017 and FR-029 classifications are retained. A width, operation and rounding mode whose recorded run exceeds the request's memory or wall-clock ceiling settles `inconclusive` under FR-028, and none is reported verified by a run at a larger ceiling than the request's. | Test |
| FR-053-AC-8 | Each comparison mutant below fails with actual solver playback of an in-domain input, caught by the listed operators at a point stated in the Behavior control points. (a) `numeric_equal` as bit equality: `numeric_equal`, over (`Z+`, `Z-`), expected true. (b) NaN reflexive in `numeric_equal`: `numeric_equal`, over (`QN0`, `QN0`), expected false. (c) `numeric_equal` ignoring the sign of a nonzero value: `numeric_equal`, over (`ONE`, `MONE`), expected false. (d) `bit_identical` as numeric equality: `bit_identical`, over (`Z+`, `Z-`), expected false, and over (`QN0`, `QN0`), expected true. (e) `bit_identical` canonicalizing a NaN: `bit_identical`, over (`QN1`, `QN2`). (f) `total_order` as signed integer comparison of the raw bits: `total_order`, over (`MONE`, `MTWO`), expected false. (g) `total_order` as unsigned comparison of the raw bits: `total_order`, over (`MONE`, `ONE`), expected true. (h) `total_order` equating the zeros: `total_order`, over (`Z+`, `Z-`), expected false. (i) `total_order` strict instead of precedes-or-equal: `total_order`, over (`ONE`, `ONE`) and (`QN0`, `QN0`), expected true. (j) `total_order` leaving NaN unordered: `total_order`, over (`INF+`, `QN0`) and (`NQN0`, `INF-`), expected true. (k) `total_order` equating all NaNs: `total_order`, over (`QN0`, `SN1`) expected false, and (`NSN1`, `NQN0`) expected false. (l) Operand swap: `total_order` over (`ONE`, `TWO`); a swap in `numeric_equal` and `bit_identical` is equivalent for a symmetric operation and is not claimed. (m) `total_order` key built with the `binary64` sign position at `binary32`: `total_order`, `float32`, over (`MONE`, `ONE`), expected true. | Test |
| FR-053-AC-9 | Each arithmetic mutant below fails with actual solver playback of an in-domain input, caught by the listed operators at the stated points. (a) One of the four operations in place of another: `add`, `sub`, `mul`, `div`, over (`THREE`, `TWO`), whose four results are 5, 1, 6 and 1.5. (b) Operand swap: `sub` and `div` over (`THREE`, `TWO`); `add` and `mul` only over (`QN1`, `QN2`), where the leftmost NaN payload decides, because their values are symmetric; for `fma`, `(a, c)` and `(b, c)` swaps over (`THREE`, `TWO`, `ONE`) and the `(a, b)` swap only over (`QN1`, `QN2`, `ONE`). (c) A rounding mode in place of another, at the per-operation points of the Behavior list and only for the pairs each separates: `add` over `T1` through `T4` and `sub` over `S1` through `S4` (and `fma` over `T1` through `T4` with the addend as `c`) for every pair of the five modes; `mul` over `M1` and `M1n` for every pair except nearest-even against toward-zero; `div` over `D1`, `Q1` and `Q1n` for every pair; `sqrt` over `SQ1` and `SQ2` for the pairs nearest, toward-zero and toward-positive, with nearest-even against nearest-away and toward-zero against toward-negative not claimed for `sqrt` because each pair is equivalent on every input; and `exact` rounding like a nearest mode instead of refusing, over the operator's inexact point. (d) The `exact` refusal reported without its would-be flags, or reported as a rounded value: over `T1` for `add` and `fma`, `S1` for `sub`, `M1` for `mul`, `Q1` for `div` and `SQ1` for `sqrt`. (e) Zero sign: a zero sum fixed to `Z+`, over `add(ONE, MONE)` and `sub(ONE, ONE)` under toward-negative where `Z-` is expected; the sign of a zero product, quotient or root dropped, over `mul(Z+, MONE)`, `div(Z+, MONE)` and `sqrt(Z-)`. (f) Division by zero reported as `Undefined`, as a NaN, with the wrong sign or without `divide_by_zero`: `div`, over (`ONE`, `Z+`), (`ONE`, `Z-`) and (`Z+`, `Z+`). (g) An invalid operation yielding a payload-preserving or infinite result or no `invalid`: `add(INF+, INF-)`, `mul(INF+, Z+)`, `div(INF+, INF+)`, `sqrt(MONE)`, `sqrt(INF-)`, `fma(INF+, Z+, ONE)`, `fma(INF+, ONE, INF-)`. (h) NaN propagation, for `add`, `sub`, `mul`, `div` and `fma` (with the NaN in each operand position) and for `sqrt` over `SN1` and `NQN0`: the canonical NaN returned instead of the operand's payload, over (`SN1`, `QN2`), expected `0x7FC00001` with `invalid`; `invalid` raised only when the leftmost NaN is signaling, over (`QN2`, `SN1`), expected `QN2` with `invalid`; quieting dropped, over (`SN1`, `ONE`), expected `0x7FC00001`; sign dropped, over (`NQN0`, `ONE`), expected `NQN0` with no flag; the invalid-operation test run before the NaN test, over `fma(INF+, Z+, QN2)`, expected `QN2` with no flag. (i) Overflow: always rounding to infinity, over `add(MAXF, MAXF)` under toward-zero and toward-negative (expected `MAXF`) and over its negated operands under toward-zero and toward-positive (expected the negated `MAXF`); the `overflow` flag missing, over `add(MAXF, MAXF)` in any mode; a completed value under `exact`, over the same input. (j) Underflow and subnormals: flush-to-zero, over `add(SUBMIN, Z+)` (expected `SUBMIN`, no flags); the `underflow` flag missing, over `mul(SUBMIN, HALF)` (an exact tie between `Z+` and `SUBMIN`: `Z+` under nearest-even, toward-zero and toward-negative, `SUBMIN` under nearest-away and toward-positive, flags `underflow` and `inexact` in each); the subnormal-to-normal carry mis-encoded, over `add(SUBMAX, SUBMIN)` (expected `NORMMIN`, no flags). (k) `fma` computed as a `mul` followed by an `add` (two roundings): `fma`, over (`0x3F800800`, `0x3F800800`, `0xBF801000`) at `float32` and (`0x3FF0000002000000`, `0x3FF0000002000000`, `0xBFF0000004000000`) at `float64`, whose single-rounding result is exactly 2^-24 (`0x33800000`) and 2^-54 (`0x3C90000000000000`) in every mode, with the two-rounding result differing in every mode. (l) `sqrt` reporting every result inexact, or none: `sqrt(FOUR)` (expected `TWO`, no flags, `Completed` under `exact`) and `SQ1`. (m) The wrong width or rounding mode in the result provenance: every operator. | Test |
| FR-053-AC-10 | Each charge and outcome mutant fails with actual solver playback of an in-domain input. (a) A dropped or reordered charge: every operator, with `ieee.exact-intermediate` and `ieee.round` dropped over a finite input. (b) `ieee.exact-intermediate` or `ieee.round` charged on a classification-decided input: every arithmetic operator, over a NaN, infinity or division-by-zero input. (c) A wrong charge amount: the width taken as 64 at `float32` or 32 at `float64`, every operator; the arity taken as two for `sqrt` or `fma`, over those operators. (d) `ieee.result-retain` charged on an `exact` refusal: every arithmetic operator, over its inexact point named in AC-9 (d). (e) A stopped outcome reported completed: every operator. | Test |
| FR-053-AC-11 | Counterexample decoding validates the one physical I64 entry of a float parameter (width, little-endian bytes, comment, order, schema role, byte and value-count limits) before grouping, then, for `float32`, the range `0..=0xFFFF_FFFF`. Grouping yields one `WitnessValue::Float32(u32)` or `WitnessValue::Float64(u64)` per parameter through the public replay route for that exact declaration, retaining the bits exactly. Round-trip controls cover `Z-`, `SN1`, `NQN0` and `0x8000000000000000`; a decode through a host float value, or through a checked `i64` to `u64` conversion, fails the control. Missing, duplicate, foreign, role-swapped, width-conflicting and malformed schemas or entries, and a `float32` value above `u32::MAX` or negative, are refused with typed causes and no replay. An unavailable replay capability is reported without claiming acceptance. | Test |
| FR-053-AC-12 | Run evidence records the operation and source association, operand forms and width, the `float_rounding` result bound and rounding mode (arithmetic only), correspondence, ordered physical bindings and their logical grouping, subject and full ordered backend arguments, the unwind bound and the carrier's loop bounds, Kani/Rust/CBMC/solver versions, target, dependency cohort including the carrier, limits, the observed peak memory per width, operation and rounding mode, raw checks, cover and playback, exit and typed terminal outcome. No-run, refusal, cover failure, resource exhaustion and semantic falsification remain distinguishable; preparation alone earns no proof result. | Test |
| FR-053-AC-13 | On the host, the carrier's `cfg(kani)` replacement of the owner's `evaluate_ieee` and `compare_ieee` and the shipped owner functions agree for every supported operation, width and rounding mode on the typed outcome, the result bits, flags and provenance, ordered charges, the denied next charge and consumed counters at zero, exact and short budgets. The inputs are the AC-2 class-boundary set and a stated boundary family: every control point of the Behavior list (whose expected results are first asserted against the shipped owner), the largest and smallest subnormal and normal magnitudes, the exact overflow and underflow thresholds and one unit either side, the rounding ties at the subnormal-to-normal boundary, operand pairs with the largest exponent difference for add and sub, an `fma` whose product overflows and whose addend brings it back in range, an `fma` that cancels exactly, and perfect and imperfect squares including the subnormal range. A replacement that differs from the owner on any of these inputs fails, and the check covers these inputs and not every operand pair. For `nearest-even`, the `float32` and `float64` results for non-NaN outcomes of `add`, `sub`, `mul`, `div`, `sqrt` and `fma` are also compared with Rust's `f32` and `f64` operators, `sqrt` and `mul_add` on the same inputs, as an independent reference that shares no code with the owner or the carrier; NaN payloads are excluded from that comparison because the host does not fix them. The only code the replacement and the expected side share is the bit-pattern decode (AC-14). A defect injected into the decode changes both sides, so the Kani proof cannot catch it, and it must fail this differential against the unmodified shipped owner. A defect injected into the replacement's rounding step, sticky shift or operand classification is not shared with the expected side, so the proof against the expected side catches it, and this differential catches it too. Until IR-1446 supplies the host-callable replacement this criterion cannot run. | Test |
| FR-053-AC-14 | The expected side of every supported operation reaches no function of the carrier's `cfg(kani)` replacement other than the bit-pattern decode (inspected: its call graph from the harness shows the decode as the only shared function, and the rounding step, sticky shift, operand classification, flag computation and charge sequence as separate implementations). A mutant that makes the expected side call the replacement's rounding step, or its classification, fails the inspection even though it still passes FR-053-AC-1's "not a second call of the oracle". A defect injected into the decode is recorded as detectable by FR-053-AC-13 only. | Inspection |

## Dependencies

- [FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md) owns the IEEE
  oracle semantics and the derivable operation list (add, sub, mul, div, the three
  comparisons and the two width conversions); its oracle agreement is not this proof.
  It refuses `sqrt` and `fma` as `OperationNotDerivable`, which is why P3 is needed.
- The float carrier (IR-1446) is prerequisite P1, the Contract IR float
  `requires_bound` ruling (IR-69) is P2, and the FR-014 `sqrt` and `fma` derivation
  (IR-1502) is P3. The IR-1436 design review (D4) fixes the carrier contract and gates the
  memory evidence; its ordering puts the Meter work (IR-844, IR-241) first.
- [FR-052](./FR-052-text-scalar-harnesses.md), [FR-051](./FR-051-decimal-scalar-harnesses.md),
  [FR-050](./FR-050-quantity-scalar-harnesses.md), [FR-049](./FR-049-enum-scalar-harnesses.md),
  [FR-046](./FR-046-rational-function-predicate-harnesses.md) and QSpec FR-196 are the
  precedent and the finite-encoding obligation this row follows.
- TC-030's IEEE-float scalar row owns today's `OperationNotRendered` settlement; this
  row supersedes it for the listed operations only when delivered.
- [FR-015](./FR-015-bounded-kani-obligations.md), [FR-025](./FR-025-generated-subject-abi.md)
  and [FR-028](./FR-028-bounded-proof-ceilings.md) own the shared refusals and ABI row.
- [TC-067](../matrix/TC-067-float-scalar-harnesses.md) defines the planned verification.

## Status

PLANNED (IR-1442). This specification supplies no implementation or generated-proof
credit. All fourteen criteria remain planned and untagged until P1 is delivered with the
IR-269 float harness. P2 (IR-69) and P3 (IR-1502) are undelivered. The public Float32
and Float64 replay values exist in `qsl_replay`; the CG decoder grouping and the carrier's
physical bindings do not. Oracle agreement under FR-014 is not Kani credit. No width,
operation or rounding mode is known to verify within the request's ceilings (an open
IR-1446 obligation), and the `sqrt` and `fma` items have no CG oracle at all; these are
open obligations, not waived gaps.
