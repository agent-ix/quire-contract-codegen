---
id: FR-057
title: "Generate bounded integer division, remainder and modulo scalar harnesses"
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
# FR-057: Generate bounded integer division, remainder and modulo scalar harnesses

## Description

When an admitted request selects an IR-confirmed V2 exact-scalar claim of one
of the following kinds, CG SHALL generate a bounded Kani row:

- `quire.op.integer.div` or `quire.op.integer.rem`, each with its one
  law-selected `DivisionProfile` (truncating, floor or Euclidean);
- `quire.op.integer.mod`, which is always the Euclidean remainder and carries
  no law.

The row proves that the claim's generated oracle agrees with an independent
statement of the operation over every input in the claim's operand domains.

Today these claims get no harness:

- A `div` or `mod` claim settles `Unsupported(OperationNotRendered)`. Internally
  this is `ScalarLoweringRefusal::NoRenderer`, because `ScalarKaniOperator` in
  `src/oracle/operator_catalog.rs` and `ScalarOperation` in
  `src/kani/generate/scalar.rs` name only add, subtract, multiply and negate.
- A `rem` claim is `Underived`, because FR-014 refuses `rem` today.

This row replaces those dispositions for `div`, `rem` and `mod`.

What each upstream admits, as read on 2026-11-14:

- QSL main (`qsl-semantics/src/check/lowering.rs`, `NodeKind::IntegerDivision`)
  lowers `div` and `rem` with one `integer_division` law and `mod` with none.
- Contract IR admits all three identities.
- CG's V2 oracle ([FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md))
  derives two of them:
  - `div` (`ExactScalarOperation::IntegerDivision { profile, domain }`, calling
    `rt::divide(profile, DivisionMember::Quotient, ..)`);
  - `mod` (`ExactScalarOperation::IntegerModulo { domain }`, calling
    `rt::modulo`).
- FR-014 refuses `rem` as `OperationNotDerivable`.
  [FR-014-AC-81](../../oracle/functional/FR-014-exact-scalar-oracles.md),
  authored with this requirement under the same ticket, makes `rem` derivable:
  a remainder descriptor with its law-selected profile, an oracle calling
  `rt::divide(profile, DivisionMember::Remainder, ..)`, and inferred result
  intervals. This row renders `rem` once FR-014-AC-81 holds.

The kernel result bridge is quire-exact's single-member division API. The pair
type `QuotientRemainder` and `Refusal::DivisionPairOutOfDomain` no longer exist
on quire-exact main: they were removed under IR-601, as FR-031's Behavior and
quire-exact `reviews/ir-601-code-review.md` record. The bridge is therefore:

- general route: `divide(profile, member, &Integer, &Integer, &IntegerDomain,
  &mut Meter)` and `modulo(..)`;
- native route: `evaluate_native_integer_division(profile, member,
  NativeInteger, NativeInteger, Option<NativeIntegerInterval>, &mut Meter)` and
  `evaluate_native_integer_modulo(..)`.

The member is `DivisionMember::Quotient` for `div` and
`DivisionMember::Remainder` for `rem`. Each call returns `Outcome` over one
exposed member. CG does not reconstruct a quotient and remainder pair from the
kernel.

This row does not depend on IR-602, which was canceled: its V1 bundle work
retired under the IR-348 ruling. The V2 oracle behavior for `div` and `mod`
already exists in FR-014: derivation, the inferred result intervals, and the
kept `Undefined(DivisionByZero)` outcome. The native divide and remainder
agreements of FR-031 (AC-2, AC-6, AC-16, AC-17) are implemented by IR-1045 for
the Boolean-oracle path. FR-031-AC-27 (obsolete error-code cleanup) stays held
under IR-1316 and is not needed here.

IR-598 retires the hand-built corpus's ground `div` and `rem` literal check.
That check used Rust truncating `checked_div` and `checked_rem`, had no
mutation control, and its rendering did not match QSL (FR-015-AC-109). It
retires with no V2 replacement, and this row is that replacement. It restores
none of the retired checks. Until it lands, the arithmetic family of
FR-015-AC-57 excludes division, remainder and modulo.

## Inputs

- An admitted `div`, `rem` or `mod` claim, as FR-014 derives it and FR-015
  lowers scalar claims (operand bounds, literal singletons, inferred
  application ranges, path-condition guards). The claim supplies its operand
  ranges and result range, plus the law-selected profile for `div` and `rem`.
- The integer route the negotiation selects: `IntegerRoute::General` through
  `generate_routed` or `negotiate_kani_obligations`, or `IntegerRoute::Native`
  through `generate_routed_native` or `negotiate_kani_native_obligations`.
- The requested unwind and ceilings ([FR-028](./FR-028-bounded-proof-ceilings.md)),
  and, at run time, the tractability table the caller passes
  ([FR-056](./FR-056-kani-tractability-refusal.md), FR-056-AC-15).

## Outputs

A supported item has:

- its embedded oracle;
- its independent expected side;
- a harness with one final cover;
- its scalar obligation identity and record.

A refused item has exactly one typed disposition and no Rust harness. Execution
and playback stay governed by [FR-017](./FR-017-kani-execution-evidence.md) and
[FR-029](./FR-029-run-outcome-terminal-record.md).

## Behavior

### Selection

CG SHALL add `Divide`, `Remainder` and `Modulo` to the scalar Kani projection of
the operator catalog ([FR-038](../../core/functional/FR-038-typed-operator-catalog.md)).
A `div` or `rem` harness SHALL take its profile from the claim's own descriptor,
the same value the embedded oracle was rendered from. The `mod` harness SHALL
take no profile.

### Operands and outcome mapping

Each operand SHALL be drawn as one symbolic `i64` constrained to its lowered
range, exactly as for the other integer scalar operations. The harness SHALL
NOT assume:

- that the divisor is nonzero;
- that the dividend is away from `i64::MIN`;
- any property of the outcome.

When the divisor's range contains zero, zero is a drawn input.

Let `a` be the dividend, `b` the divisor, `[L, U]` the lowered result range, and
"member" the quotient for `div` or the remainder for `rem` and `mod`. The
harness SHALL require exactly:

- `b == 0`: `Undefined(DivisionByZero)`;
- `b != 0` and the member in `[L, U]`: `Completed` with that member;
- `b != 0` and the member outside `[L, U]`: a refusal that depends on the
  operation:
  - `div`: `Refused(DivisionOutOfDomain { member: Quotient, .. })`;
  - `rem`: `Refused(DivisionOutOfDomain { member: Remainder, .. })`;
  - `mod`: `Refused(ModuloOutOfDomain { .. })`.

Any other outcome fails the assertion. That includes `Incomplete`, and a
refusal with another cause or member.

### Independent expected side

The expected side SHALL be generated per profile from the profile's defining
relation. It SHALL NOT call quire-exact or the embedded oracle, and SHALL NOT
restate the kernel's algorithm. In quire-exact, `native_div_rem` computes the
truncating `checked_div` and `wrapping_rem` and then applies sign corrections;
the expected side follows none of those steps. All expected-side arithmetic
SHALL be over `i128`, where every product of an `i64` operand and a value in
`i64` range is exact.

Each law has a remainder window `W`, a set of exactly `|b|` consecutive
integers:

| Profile | `W` |
| --- | --- |
| Truncating | `[0, |b| - 1]` when `a >= 0`, else `[-(|b| - 1), 0]` |
| Floor | `[0, b - 1]` when `b > 0`, else `[b + 1, 0]` |
| Euclidean (and `mod`) | `[0, |b| - 1]` |

For `div`, let `r(q) = a - q * b`. Exactly one quotient `q*` satisfies
`r(q*) ∈ W`. The `div` expected side SHALL work as follows:

- A completed quotient `q` SHALL be checked by `r(q) ∈ W`, with `r(q)` spelled
  `a - q * b`. Wherever the expected side forms a product of a quotient and the
  divisor, it SHALL keep that operand order: quotient, then divisor.
- Admission, meaning whether `q*` lies in `[L, U]`, SHALL be decided without
  division, from the products of the constant bounds with the divisor:
  - When `b > 0`: `q* >= L` iff `r(L) >= min W`, and `q* <= U` iff
    `r(U) <= max W`.
  - When `b < 0`: `q* >= L` iff `r(L) <= max W`, and `q* <= U` iff
    `r(U) >= min W`.
- The `div` expected side SHALL contain no Rust `/` or `%` and none of
  `checked_div`, `checked_rem`, `wrapping_div`, `wrapping_rem`, `div_euclid` or
  `rem_euclid`.

For `rem` and `mod`, the expected remainder SHALL be computed as follows:

1. Let `e` be the standard library's `i128::rem_euclid(a, b)`, which is defined
   for `b != 0`.
2. The expected remainder is the unique member of `W` congruent to `a` modulo
   `b`: `e` when `e ∈ W`, otherwise `e - |b|`.
3. For `mod` and the Euclidean `rem`, `W` always contains `e`.

A completed value SHALL equal the expected remainder and lie in `W`. Admission
is the expected remainder's membership in `[L, U]`. `rem_euclid` is the one
division-family operation these expected sides may contain.

### `i64::MIN / -1` and the result range

On both routes the operands are `i64`, so `i64::MIN / -1 = 2^63` is exact (in
`Integer`, or in the `i128` native carrier) and never wraps or panics.

- With an explicit `div` result range inside `i64`, `2^63` is outside it. The
  oracle returns `Refused(DivisionOutOfDomain { member: Quotient, .. })`, and
  the harness SHALL require that refusal at that input.
- With FR-014's inferred `div` interval `[-A, A]` and a dividend range that
  reaches `i64::MIN`, `A = 2^63` does not fit `i64`. The claim SHALL be refused
  at lowering as `ScalarLoweringRefusal::BoundNotI64` with its canonical
  decimal endpoints, with no harness. This is how the existing scalar lowering
  refuses any endpoint that is not `i64`.
- `i64::MIN rem -1` (under every profile) and `i64::MIN mod -1` are `0` and
  complete.

### Routes

On the general route, the embedded oracle is FR-014's `rt::divide` or
`rt::modulo` source.

On the native route, AD-006 requires a kernel-checked native plan before
evaluation. quire-exact main's `NativeIntegerPlan` / `NativeIntegerOperation`
has no division or modulo variant, although the plan-free evaluators
`evaluate_native_integer_division` and `evaluate_native_integer_modulo` exist.
IR-1541 owns that kernel plan and its Kani measurements, delivered as a
quire-exact PR; this requirement makes no AD-006 amendment. Until the plan is
available, native selection of a `div`, `rem` or `mod` claim SHALL be refused
with no harness and a typed reason that names the node and the operation. That
reason is distinct from `NativeIntegerCapacity`, which reports an interval the
kernel refused. When the plan exists, the native oracle SHALL call the native
evaluators through it, with the same expected side and outcome mapping.

### Measured widths and refusal

CG SHALL claim a verdict for a shape only where that shape was measured. On each
route the generator serves, the coder SHALL measure, under installed Kani with
the FR-028 wall-clock ceiling, `div` and `rem` under each profile, and `mod`, in
two shapes:

- constant divisor: a literal divisor, one positive and one negative, with a
  dividend range at 8, 16, 32 and 63 bits;
- symbolic divisor: a divisor range that contains zero and both signs, with
  both operands at 4, 8, 16, 32 and 63 bits, or up to the first width that does
  not finish.

Width is FR-056's: the largest operand bit width after range propagation.

Each measurement SHALL be recorded under `docs/qualification/` with:

- operation, profile, route and operand ranges;
- verdict, wall seconds and peak bytes (or none);
- Kani, CBMC and solver versions, generator and kernel versions, host and date.

A shape that ends `TimedOut` or `memory_exhausted` SHALL become a
`not_tractable` row in the shipped scalar table that FR-056-AC-15 consumes, at
its smallest failing width and route. A `div`, `rem` or `mod` harness at or
above that width is then refused at run time as `V2ClauseNotTractable` before
any Kani launch, through the existing decider seam. This requirement adds no
tractability code specific to an operation.

The FR-056 features carry no constant-divisor flag. CG SHALL NOT ship a row that
would refuse a shape measured `Verified` at that width and route. Only when the
measured rows show a split SHALL the coder fold in the following minimal
extension; otherwise the gap stays noted and nothing is added. A split means
the symbolic-divisor shape fails at a width where the constant-divisor shape
was measured `Verified`. The extension:

- a scalar feature `divisor_constant: bool`, true when the divisor's lowered
  range is a singleton;
- an optional row field `divisor_constant`. Absent means the row matches
  either value.

A row matches a harness only when the field is absent or equal to the
harness's feature. FR-056's other Lookup rules are unchanged.

Shapes that are not measured carry no claim. The general route is expected to
be not tractable, because IR-1530 measured an add over `rt::Integer` not
tractable from 2 bits. That expectation is not a verdict: one smallest-width
general-route probe per operation is measured before any general-route row
ships.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-057-AC-1 | QSL-emitted, IR-admitted claims are generated through `generate_routed` and `generate_routed_native`: `div` and `rem` claims under each of the truncating, floor and Euclidean laws, and a `mod` claim. Each claim's operand ranges include a negative dividend and both divisor signs. None of these claims settles `OperationNotRendered` or `Underived` (the `rem` claim needs FR-014-AC-81). Each settles `Supported` with one harness, or with exactly one typed refusal that this requirement names: the native-plan refusal, `BoundNotI64`, `ResultUnreachable`, a guard refusal or `V2ClauseNotTractable`. Each `div` and `rem` harness renders its profile from the claim's own descriptor. The embedded oracle and the expected side derive from the same node and keep its source association. The catalog's scalar Kani projection holds `Divide`, `Remainder` and `Modulo`. | Test |
| FR-057-AC-2 | The harness draws each operand as one `i64` over its lowered range, with no assumption on the divisor, the dividend or the outcome. The assertion maps `b == 0` to `Undefined(DivisionByZero)` and an in-range member to `Completed` with that member. It maps an out-of-range member to `Refused(DivisionOutOfDomain { member: Quotient })` for `div`, `Refused(DivisionOutOfDomain { member: Remainder })` for `rem` and `Refused(ModuloOutOfDomain)` for `mod`. Every other outcome fails. A host control evaluates the rendered assertion over the real oracle at the input that distinguishes each mutant. These mutants each fail it: accepting any `Refused`, accepting the other `DivisionOutOfDomain` member, accepting `Incomplete`, and accepting `Completed` at `b == 0`. | Test |
| FR-057-AC-3 | The `div` expected side is the relational form of the Behavior section. A completed `q` is checked by `a - q * b` lying in the profile's window, admission comes from `r(L)` and `r(U)` against the window by divisor sign, and every quotient-divisor product keeps the order quotient, then divisor. The `rem` and `mod` expected sides shift `rem_euclid` into the window. Inspection of each emitted expected side finds no `quire_exact`, `rt::` or `nx::` path and no oracle symbol. It finds none of `/`, `%`, `checked_div`, `checked_rem`, `wrapping_div`, `wrapping_rem`, `div_euclid` or `rem_euclid`, except `rem_euclid` in a `rem` or `mod` expected side. | Inspection |
| FR-057-AC-4 | A host differential compiles the rendered expected side (the same text the harness embeds) as host Rust. It compares that side with quire-exact `divide` (both members) and `modulo` over `Integer` for every profile. The grid of dividends and divisors includes `0`, `±1`, `±2`, `±7`, `i64::MIN`, `i64::MIN + 1`, `i64::MAX` and `-1`, with result ranges both inside and excluding the member. Outcome kind, value and refusal member agree at every point. The grid includes these named vectors (truncating, floor, Euclidean): `-7 div 2` is `-3`, `-4`, `-4`; `-7 div -2` is `3`, `3`, `4`; `-7 rem 2` is `-1`, `1`, `1`; `-7 rem -2` is `-1`, `-1`, `1`. Both `-7 mod 2` and `-7 mod -2` are `1`. A window table with two profile rows swapped fails the differential. | Test |
| FR-057-AC-5 | At a zero divisor, over a divisor range containing zero (for example `y: Int[-3, 3]`), the oracle, quire-exact over `Integer` and the harness's assertion all give `Undefined(DivisionByZero)`, with no panic and no `Refused`. This holds for `div` and `rem` under each profile and for `mod`. For `MIN / -1` with an explicit result range inside `i64`, `i64::MIN div -1` gives `Refused(DivisionOutOfDomain { member: Quotient })` from the oracle and from quire-exact, and the harness requires that refusal. With the inferred `[-A, A]` range and a dividend range reaching `i64::MIN`, lowering refuses `BoundNotI64` naming the canonical endpoints `-9223372036854775808` and `9223372036854775808`, with no harness. `i64::MIN rem -1` (every profile) and `i64::MIN mod -1` complete `0`. | Test |
| FR-057-AC-6 | On the native route, while quire-exact exposes no kernel-checked native division or modulo plan (owned by IR-1541), a `div`, `rem` or `mod` claim settles with a typed native-plan refusal naming the node and the operation identity. It has no harness and no `NativeIntegerCapacity` reason. A supported sibling in the same request is generated byte-identically to how it is generated alone. | Test |
| FR-057-AC-7 | Under installed Kani, on each route and shape measured tractable, the healthy `div` and `rem` harnesses of each profile and the healthy `mod` harness classify `Verified`, with every check and the one completion cover satisfied. Each run records wall seconds and peak bytes. No other shape is claimed `Verified`. The native-route verdict is blocked by IR-1541 (the kernel's native division plan). | Test (installed Kani) |
| FR-057-AC-8 | Under installed Kani, each seeded mutant classifies `Falsified` with solver playback of in-domain operands, over a fixture whose operand ranges include a negative dividend, both divisor signs and zero, with an explicit result range that excludes some members. The mutants fall into two groups. Wrong profile: the embedded oracle's `DivisionProfile` swapped for each of the other two laws (six swaps for `div` and six for `rem`); the expected side's window table rendered for the wrong profile; `rem` rendered as `modulo` (for the Euclidean law this is caught only by the refusal cause); `rem` rendered with `DivisionMember::Quotient`; `mod` rendered as `divide(Truncating, Remainder, ..)` and as `divide(Floor, Remainder, ..)`. Missing zero guard: the oracle returning a completed value at a zero divisor, and the expected side's zero arm removed. The decoded playback is replayed through the routed scalar replay ([FR-032](../../replay/functional/FR-032-routed-scalar-replay-binding.md)) and through quire-exact over `Integer`, and the kernel value or outcome differs from the mutant's. | Test (installed Kani) |
| FR-057-AC-9 | The Behavior section's measurement record exists for every listed operation, shape, route and width, with each field filled. Every `TimedOut` or `memory_exhausted` shape is a `not_tractable` row in the shipped scalar table at its smallest failing width and route. A harness at or above that width is refused at run time as `V2ClauseNotTractable` with no Kani launch, and a harness below it runs. No shipped row refuses a shape measured `Verified` at that width and route. If the rows show a split between a constant and a symbolic divisor, the `divisor_constant` extension is present: a row carrying it refuses only harnesses whose feature matches, and a row without it matches both. If the rows show no split, no extension is added. The spec and the Status claim no verdict for an unmeasured shape. | Test and Inspection |
| FR-057-AC-10 | Each emitted `div`, `rem` and `mod` harness has exactly one completion cover after all assertions. An unsatisfied cover, a failed unwind, a timeout, memory exhaustion or a missing verdict cannot yield `Verified`, and classifications keep their FR-017 and FR-029 distinctions. | Test |
| FR-057-AC-11 | The routed outputs of these claims are byte-identical before and after this change: integer add, subtract, multiply and negate, the integer comparisons, Boolean implication, enum comparison and float comparison. FR-015-AC-115's reference-copy equivalence for the four existing integer operations still holds. | Test |
| FR-057-AC-12 | Run evidence records the selected node and source association, the operation identity, the profile (for `div` and `rem`), the route, and the operand and result ranges. It also records the full ordered backend arguments, every unwind and option, the target, limits, raw checks and cover, playback, exit and the typed terminal outcome. Replay uses the retained generated artifact without hand editing. | Test |

## Dependencies

- [FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md) owns the
  `div`, `rem` (FR-014-AC-81) and `mod` oracles, the law-selected profile and
  the inferred result intervals.
- [FR-015](./FR-015-bounded-kani-obligations.md) owns scalar lowering, guards,
  the cover rule and the retirement of the arithmetic family (FR-015-AC-57,
  FR-015-AC-109).
- [FR-038](../../core/functional/FR-038-typed-operator-catalog.md) owns the
  catalog that gains the new scalar Kani entries.
- [FR-056](./FR-056-kani-tractability-refusal.md) owns the decider seam and the
  shipped scalar table that measured rows are added to.
- [AD-006](../../assurance/AD-006-admitted-native-integer-oracles.md) owns
  native-route admission. IR-1541 owns the native division and modulo plan that
  quire-exact must provide.
- quire-exact main owns `divide`, `modulo`, `evaluate_native_integer_division`,
  `evaluate_native_integer_modulo`, `DivisionProfile`, `DivisionMember`,
  `Undefined::DivisionByZero`, `Refusal::DivisionOutOfDomain` and
  `Refusal::ModuloOutOfDomain`. IR-17 (kernel adoption) is Done.
- [TC-071](../matrix/TC-071-integer-division-scalar-harnesses.md) defines the
  planned verification.

## Status

PLANNED (IR-854). No `div`, `rem` or `mod` scalar Kani harness exists yet. A
`div` claim settles `Unsupported(OperationNotRendered)`, and a `rem` claim is
`Underived` until FR-014-AC-81. No verdict is claimed for any shape.

IR-854 is an IR-269 MVP row. IR-598 retires the corpus's ground `div` and `rem`
check, and IR-854 owns its V2 replacement (this requirement).

IR-854 is blocked by IR-1541 (quire-exact native `Integer` Divide and Modulo
plan, with Kani measurements) only for FR-057-AC-7's native-route `Verified`
verdict. The spec, the renderer, the expected side, the refusals and the host
criteria proceed without it, and FR-057-AC-6 governs the native route until the
plan lands.
