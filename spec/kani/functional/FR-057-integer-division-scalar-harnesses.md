---
id: FR-057
title: "Generate bounded integer division and modulo scalar harnesses"
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
  - target: ix://agent-ix/quire-contract-codegen/FR-028
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-038
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-056
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/AD-006
    type: references
---
# FR-057: Generate bounded integer division and modulo scalar harnesses

## Description

When an admitted request selects an IR-confirmed V2 exact-scalar claim over
`quire.op.integer.div` (with its one law-selected `DivisionProfile`: truncating,
floor or Euclidean) or `quire.op.integer.mod` (always the Euclidean remainder,
no law), CG SHALL generate a bounded Kani row that proves the claim's generated
oracle agrees with an independent statement of the operation over every input
of the claim's operand domains. Today such a claim settles
`Unsupported(OperationNotRendered)` with no harness (the internal
`ScalarLoweringRefusal::NoRenderer`, because `ScalarKaniOperator` in
`src/oracle/operator_catalog.rs` and `ScalarOperation` in
`src/kani/generate/scalar.rs` name only add, subtract, multiply and negate).
This row replaces that disposition for `div` and `mod` only.

What each upstream admits, as read on 2026-11-14:

- QSL main (`qsl-semantics/src/check/lowering.rs`, `NodeKind::IntegerDivision`)
  lowers `div` and `rem` with one `integer_division` law and `mod` with none.
- Contract IR admits all three identities (`quire.op.integer.rem` is a
  catalogued identity; TC-033 routes a `rem` node).
- CG's V2 oracle ([FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md))
  derives `div` (`ExactScalarOperation::IntegerDivision { profile, domain }`,
  calling `rt::divide(profile, DivisionMember::Quotient, ..)`) and `mod`
  (`ExactScalarOperation::IntegerModulo { domain }`, calling `rt::modulo`). It
  refuses `quire.op.integer.rem` as `OperationNotDerivable`.

So `rem` has no V2 oracle to prove and this row does not render it: a `rem`
claim keeps its `Underived` disposition (FR-015-AC-109, TC-033). A `rem` row
needs FR-014 derivation first and is a separate criterion family.

The kernel result bridge is quire-exact's single-member division API. The pair
type `QuotientRemainder` and `Refusal::DivisionPairOutOfDomain` no longer exist
on quire-exact main: they were removed under IR-601, as FR-031's Behavior and
quire-exact `reviews/ir-601-code-review.md` record. The bridge is therefore:

- on the general route, `divide(profile, DivisionMember::Quotient, &Integer,
  &Integer, &IntegerDomain, &mut Meter)` and `modulo(..)`;
- on the native route, `evaluate_native_integer_division(profile,
  DivisionMember::Quotient, NativeInteger, NativeInteger,
  Option<NativeIntegerInterval>, &mut Meter)` and
  `evaluate_native_integer_modulo(..)`.

Each returns `Outcome` over one exposed member. CG does not reconstruct a
quotient and remainder pair from the kernel.

This row does not depend on IR-602, which was canceled: its V1 bundle work
retired under the IR-348 ruling. The V2 oracle behavior this row proves
already exists in FR-014 (derivation, the `[-A, A]` and `[0, B - 1]` inferred
result intervals, and the kept `Undefined(DivisionByZero)`). The native
divide and remainder agreements of FR-031 (AC-2, AC-6, AC-16, AC-17) are
implemented by IR-1045 for the Boolean-oracle path. FR-031-AC-27 (obsolete
error-code cleanup) stays held under IR-1316 and is not needed here.

This row restores none of the retired hand-built corpus division checks, which
were ground Rust `checked_div` and `checked_rem` literal checks with no mutation
control (FR-015-AC-109). Until it lands, the arithmetic family of FR-015-AC-57
excludes division and modulo.

## Inputs

- An admitted `div` or `mod` claim with its operand ranges, result range and,
  for `div`, its law-selected profile, as FR-014 derives them and FR-015 lowers
  scalar claims (operand bounds, literal singletons, inferred application
  ranges, path-condition guards).
- The integer route the negotiation selects (`IntegerRoute::General` through
  `generate_routed` or `negotiate_kani_obligations`, `IntegerRoute::Native`
  through `generate_routed_native` or `negotiate_kani_native_obligations`).
- The requested unwind and ceilings ([FR-028](./FR-028-bounded-proof-ceilings.md)),
  and, at run time, the caller-passed tractability table
  ([FR-056](./FR-056-kani-tractability-refusal.md), FR-056-AC-15).

## Outputs

A supported item has its embedded oracle, its independent expected side, a
harness with one final cover, and its scalar obligation identity and record. A
refused item has exactly one typed disposition and no Rust harness. Execution
and playback stay governed by [FR-017](./FR-017-kani-execution-evidence.md) and
[FR-029](./FR-029-run-outcome-terminal-record.md).

## Behavior

### Selection

CG SHALL add `Divide` and `Modulo` to the scalar Kani projection of the
operator catalog ([FR-038](../../core/functional/FR-038-typed-operator-catalog.md)).
The `div` harness SHALL take its profile from the claim's
`ExactScalarOperation::IntegerDivision { profile, .. }`, the same value the
embedded oracle was rendered from. The `mod` harness SHALL take no profile.
`quire.op.integer.rem` SHALL gain no scalar Kani projection.

### Operands and outcome mapping

Each operand SHALL be drawn as one symbolic `i64` constrained to its lowered
range, exactly as for the other integer scalar operations. The harness SHALL
NOT assume the divisor nonzero, the dividend away from `i64::MIN`, or any
property of the outcome. When the divisor's range contains zero, zero is a
drawn input.

With `a` the dividend, `b` the divisor and `[L, U]` the lowered result range,
the harness SHALL require exactly:

- `b == 0`: `Undefined(DivisionByZero)`;
- `b != 0` and the profile's member in `[L, U]`: `Completed` with that member;
- `b != 0` and the member outside `[L, U]`: `Refused(DivisionOutOfDomain {
  member: Quotient, .. })` for `div`, or `Refused(ModuloOutOfDomain { .. })`
  for `mod`.

Any other outcome, including `Incomplete` or a refusal with another cause,
fails the assertion.

### Independent expected side

The expected side SHALL be generated per profile from the profile's defining
relation. It SHALL NOT call quire-exact or the embedded oracle, and SHALL NOT
restate the kernel's algorithm: `native_div_rem` in quire-exact computes
`checked_div` and `wrapping_rem` and then applies sign corrections, and the
expected side follows none of those steps. All expected-side arithmetic SHALL
be over `i128`, where every product of an `i64` operand and an `i64`-range value
is exact.

For `div`, with remainder `r(q) = a - q * b`, the law's remainder window `W` is
a set of exactly `|b|` consecutive integers, so exactly one quotient `q*`
satisfies `r(q*) ∈ W`:

| Profile | `W` |
| --- | --- |
| Truncating | `[0, |b| - 1]` when `a >= 0`, else `[-(|b| - 1), 0]` |
| Floor | `[0, b - 1]` when `b > 0`, else `[b + 1, 0]` |
| Euclidean | `[0, |b| - 1]` |

- A completed quotient `q` SHALL be checked by `r(q) ∈ W`, with `r(q)` spelled
  `a - q * b`. Wherever the expected side forms a product of a quotient and the
  divisor it SHALL keep that operand order (quotient, then divisor).
- Admission (whether `q*` lies in `[L, U]`) SHALL be decided without division,
  from the products of the constant bounds with the divisor. When `b > 0`:
  `q* >= L` iff `r(L) >= min W`, and `q* <= U` iff `r(U) <= max W`. When
  `b < 0`: `q* >= L` iff `r(L) <= max W`, and `q* <= U` iff `r(U) >= min W`.
- The `div` expected side SHALL contain no Rust `/` or `%` and none of
  `checked_div`, `checked_rem`, `wrapping_div`, `wrapping_rem`, `div_euclid` or
  `rem_euclid`.

For `mod`, the expected remainder SHALL be the standard library's
`i128::rem_euclid(a, b)` (defined for `b != 0`). A completed value SHALL equal
it and lie in `[0, |b| - 1]`, and admission is that value's membership in
`[L, U]`. `rem_euclid` is the one division-family operation the expected side
may contain, and only in the `mod` harness.

### `i64::MIN / -1` and the result range

On both routes the operands are `i64`, so `i64::MIN / -1 = 2^63` is exact (in
`Integer`, or in the `i128` native carrier) and never wraps or panics.

- With an explicit result range inside `i64`, `2^63` is outside it and the
  oracle returns `Refused(DivisionOutOfDomain { member: Quotient, .. })`. The
  harness SHALL require that refusal at that input.
- With the inferred `div` interval `[-A, A]` of FR-014 and a dividend range
  reaching `i64::MIN`, `A = 2^63` does not fit `i64`. The claim SHALL be
  refused at lowering as `ScalarLoweringRefusal::BoundNotI64` with its
  canonical decimal endpoints, with no harness, as the existing scalar lowering
  refuses any non-`i64` endpoint.
- `i64::MIN mod -1` and `i64::MIN mod 1` are `0` and complete.

### Routes

On the general route the embedded oracle is FR-014's existing `rt::divide` or
`rt::modulo` source, unchanged.

On the native route, AD-006 requires a kernel-checked native plan before
evaluation. quire-exact main's `NativeIntegerPlan` / `NativeIntegerOperation`
has no division or modulo variant, although the plan-free evaluators
`evaluate_native_integer_division` and `evaluate_native_integer_modulo` exist.
Until a kernel-checked native division plan is available, native selection of a
`div` or `mod` claim SHALL be refused with a typed reason naming the node and
the operation (distinct from `NativeIntegerCapacity`, which reports an interval
the kernel refused), with no harness. That native plan is an open dependency
(see Status). When it exists, the native oracle SHALL call the native
evaluators through it, with the same expected side and outcome mapping.

### Measured widths and refusal

CG SHALL claim a verdict for a shape only where it was measured. The coder
SHALL measure, under installed Kani with the FR-028 wall-clock ceiling, on each
route the generator serves:

- `div` under each profile, and `mod`, with a constant (literal) divisor: one
  positive and one negative, and a dividend range at 8, 16, 32 and 63 bits;
- `div` under each profile, and `mod`, with a symbolic divisor whose range
  contains zero and both signs: both operands at 4, 8, 16, 32 and 63 bits, or
  up to the first width that does not finish.

Width is FR-056's: the largest operand bit width after range propagation.

Each measurement SHALL be recorded under `docs/qualification/` with:

- operation, profile, route and operand ranges;
- verdict, wall seconds and peak bytes (or none);
- Kani, CBMC and solver versions, generator and kernel versions, host and date.

A shape that ends `TimedOut` or `memory_exhausted` SHALL become a
`not_tractable` row in the shipped scalar table that FR-056-AC-15 consumes, at
its smallest failing width and route. A `div` or `mod` harness at or above that
width is then refused at run time as `V2ClauseNotTractable` before any Kani
launch, through the existing decider seam. This requirement adds no
per-operation tractability code.

The FR-056 features carry no constant-divisor flag. If a symbolic-divisor shape
fails at a width where the constant-divisor shape was measured `Verified`, CG
SHALL NOT ship a row that would refuse the measured-verified shape. The split
SHALL be recorded as a seam gap for FR-056, and the wall-clock backstop governs
that width.

Shapes that are not measured carry no claim. The general route is expected to
be not tractable, because IR-1530 measured an add over `rt::Integer` not
tractable from 2 bits, but that expectation is not a verdict: one smallest-width
general-route probe per operation is measured before any general-route row
ships.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-057-AC-1 | Through `generate_routed` and `generate_routed_native`, a QSL-emitted, IR-admitted `div` claim under each of the truncating, floor and Euclidean laws, and a `mod` claim, each over operand ranges that include a negative dividend and both divisor signs, no longer settle `OperationNotRendered`. Each settles `Supported` with one harness, or with exactly one typed refusal that this requirement names (the native-plan refusal, `BoundNotI64`, `ResultUnreachable` or a guard refusal). Each `div` harness renders its profile from the claim's own `IntegerDivision { profile }`, and the embedded oracle and the expected side derive from the same node with its source association. A `quire.op.integer.rem` claim keeps its `Underived` disposition with no harness. The catalog's scalar Kani projection holds `Divide` and `Modulo` and no remainder entry. | Test |
| FR-057-AC-2 | The harness draws each operand as one `i64` over its lowered range, with no assumption on the divisor, the dividend or the outcome. The assertion maps `b == 0` to `Undefined(DivisionByZero)`, an in-range member to `Completed` with that member, and an out-of-range member to `Refused(DivisionOutOfDomain { member: Quotient })` (for `div`) or `Refused(ModuloOutOfDomain)` (for `mod`). Every other outcome fails. Mutants: accepting any `Refused`, accepting `Incomplete`, or accepting `Completed` at `b == 0` each fail a host control that evaluates the rendered assertion over the real oracle at the distinguishing input. | Test |
| FR-057-AC-3 | The `div` expected side is the relational form of the Behavior section: a completed `q` is checked by `a - q * b` lying in the profile's window, and admission comes from `r(L)` and `r(U)` against the window by divisor sign. Inspection of each emitted `div` harness's expected side finds no `quire_exact`, `rt::` or `nx::` path, no oracle symbol, no `/`, `%`, `checked_div`, `checked_rem`, `wrapping_div`, `wrapping_rem`, `div_euclid` or `rem_euclid`. Inspection of the `mod` expected side finds only `rem_euclid` among those. Every quotient-divisor product keeps the order quotient, then divisor. | Inspection |
| FR-057-AC-4 | A host differential evaluates the rendered expected side (the same text the harness embeds, compiled as host Rust) against quire-exact `divide` and `modulo` over `Integer` for every profile. It uses a grid of dividends and divisors that includes `0`, `±1`, `±2`, `±7`, `i64::MIN`, `i64::MIN + 1`, `i64::MAX` and `-1`, and result ranges both inside and excluding the member. Outcome kind and value agree at every point, including the named vectors from quire-exact's IR-601 review: `-7 div 2` is `-3` (truncating), `-4` (floor) and `-4` (Euclidean); `-7 div -2` is `3`, `3` and `4`; `-7 mod 2` and `-7 mod -2` are both `1`. A window table with two profile rows swapped fails the differential. | Test |
| FR-057-AC-5 | Divisor zero: over a divisor range containing zero (for example `y: Int[-3, 3]` for `x div y`), the oracle, quire-exact `divide`/`modulo` over `Integer`, and the harness's assertion all give `Undefined(DivisionByZero)` at `b == 0` for each profile and for `mod`, with no panic and no `Refused`. `MIN / -1`: with an explicit result range inside `i64`, `i64::MIN div -1` gives `Refused(DivisionOutOfDomain { member: Quotient })` from the oracle and from quire-exact, and the harness requires it. With the inferred `[-A, A]` range and a dividend range reaching `i64::MIN`, lowering refuses `BoundNotI64` naming the canonical endpoints `-9223372036854775808` and `9223372036854775808`, with no harness. `i64::MIN mod -1` completes `0`. | Test |
| FR-057-AC-6 | On the native route, while quire-exact exposes no kernel-checked native division or modulo plan, a `div` or `mod` claim settles with a typed native-plan refusal naming the node and the operation identity, with no harness and no `NativeIntegerCapacity` reason. A supported sibling in the same request is generated byte-identically to how it is generated alone. When the kernel plan lands, this criterion is replaced by native harnesses meeting FR-057-AC-2 to FR-057-AC-5 through it. | Test |
| FR-057-AC-7 | Under installed Kani, on each route and shape measured tractable, the healthy `div` harness of each profile and the healthy `mod` harness classify `Verified`, with every check and the one completion cover satisfied. Each run records wall seconds and peak bytes. No other shape is claimed `Verified`. | Test (installed Kani) |
| FR-057-AC-8 | Under installed Kani, over a fixture whose operand ranges include a negative dividend, both divisor signs and zero, each seeded mutant classifies `Falsified` with solver playback of in-domain operands. The mutants are: (wrong profile) the embedded oracle's `DivisionProfile` swapped for each of the other two laws, six swaps in all; the expected side's window table rendered for the wrong profile; `mod` rendered as `divide(Truncating, Remainder, ..)` and as `divide(Floor, Remainder, ..)`; (missing zero guard) the oracle returning a completed value at a zero divisor; and the expected side's zero arm removed. The decoded playback is replayed through the routed scalar replay ([FR-032](../../replay/functional/FR-032-routed-scalar-replay-binding.md)) and through quire-exact over `Integer`, and the kernel value there differs from the mutant's. | Test (installed Kani) |
| FR-057-AC-9 | The measurement record of the Behavior section exists for every listed shape, route and width, with each field filled. Every `TimedOut` or `memory_exhausted` shape is a `not_tractable` row in the shipped scalar table at its smallest failing width and route. A harness at or above that width is refused at run time as `V2ClauseNotTractable` with no Kani launch, and a harness below it runs. No shipped row refuses a shape measured `Verified` at that width and route: a constant-versus-symbolic divisor split is recorded as an FR-056 seam gap instead. The spec and the Status claim no verdict for an unmeasured shape. | Test and Inspection |
| FR-057-AC-10 | Each emitted `div` and `mod` harness has exactly one completion cover after all assertions. An unsatisfied cover, a failed unwind, a timeout, memory exhaustion or a missing verdict cannot yield `Verified`, and classifications keep their FR-017 and FR-029 distinctions. | Test |
| FR-057-AC-11 | The routed outputs of integer add, subtract, multiply, negate, the integer comparisons, Boolean implication, enum comparison and float comparison claims are byte-identical before and after this change. FR-015-AC-115's reference-copy equivalence for the four existing integer operations still holds. A `rem` claim's record is unchanged. | Test |
| FR-057-AC-12 | Run evidence records the selected node and source association, operation identity, profile (for `div`), route, operand and result ranges, the full ordered backend arguments, every unwind and option, target, limits, raw checks and cover, playback, exit and the typed terminal outcome. Replay uses the retained generated artifact without hand editing. | Test |

## Dependencies

- [FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md) owns the
  `div` and `mod` oracles, the law-selected profile and the inferred result
  intervals; this row proves them and changes none of them.
- [FR-015](./FR-015-bounded-kani-obligations.md) owns scalar lowering, guards,
  the cover rule and the arithmetic-family retirement (FR-015-AC-57,
  FR-015-AC-109).
- [FR-038](../../core/functional/FR-038-typed-operator-catalog.md) owns the
  catalog that gains the `Divide` and `Modulo` scalar Kani entries.
- [FR-056](./FR-056-kani-tractability-refusal.md) owns the decider seam and the
  shipped scalar table that measured rows are added to.
- [AD-006](../../assurance/AD-006-admitted-native-integer-oracles.md) owns the
  native-route admission that a native division plan must satisfy.
- quire-exact main owns `divide`, `modulo`, `evaluate_native_integer_division`,
  `evaluate_native_integer_modulo`, `DivisionProfile`, `DivisionMember`,
  `Undefined::DivisionByZero`, `Refusal::DivisionOutOfDomain` and
  `Refusal::ModuloOutOfDomain`. IR-17 (kernel adoption) is Done.
- [TC-071](../matrix/TC-071-integer-division-scalar-harnesses.md) defines the
  planned verification.

## Status

PLANNED (IR-854). No `div` or `mod` scalar Kani harness exists yet; a `div`
claim settles `Unsupported(OperationNotRendered)`. No verdict is claimed for
any shape. The native route depends on a kernel-checked native division and
modulo plan that quire-exact main does not provide (open dependency). Until it
lands, FR-057-AC-6 governs the native route and FR-057-AC-7 and FR-057-AC-8
can be met only on the general route, where they are expected to measure not
tractable. Whether this row is an IR-269 MVP row is the planner's call.
