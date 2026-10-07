---
id: FR-031
title: "Generate Boolean oracles whose integer arithmetic and comparison take the exact kernel's meaning"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/interface-001
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: references
  - target: ix://agent-ix/quire-contract-runtime/FR-002
    type: references
  - target: ix://agent-ix/quire-contract-runtime/FR-006
    type: references
  - target: ix://agent-ix/quire-contract-runtime/FR-007
    type: references
  - target: ix://agent-ix/quire-exact/FR-357
    type: references
  - target: ix://agent-ix/quire-contract-ir/FR-013
    type: references
  - target: ix://agent-ix/quire-contract-ir/FR-015
    type: references
---
# FR-031: Generate Boolean oracles whose integer arithmetic and comparison take the exact kernel's meaning

## Description

The Boolean oracle of `interface-001`'s `oracle_slice` lowers one validated Boolean clause over
Boolean and bounded-integer values into a Rust function. Its clauses may hold integer add,
subtract, multiply, divide and remainder, and the six integer comparisons. Today the emitter
renders each of those five operators as the raw Rust infix operator on `i64`, and each comparison
as the raw infix comparison. It never reads the operand type's overflow policy. On overflow the
generated function panics in a debug build and wraps in a release build, and on a zero divisor it
panics. The exact kernel's integer operations (`quire-exact`, which Contract Runtime also carries a
copy of until IR-349 removes it), which are the language's meaning of the same nodes, return a typed
outcome instead.

A generated oracle that panics or wraps disagrees with the language it claims to evaluate, and a
Kani proof or a conformance campaign over it proves or tests a different operation. This
requirement fixes what the Boolean oracle emits and means for integer arithmetic and comparison,
so that the meaning of an operator is stated once, in the exact kernel, and the emitter
restates none of it, other than in the V1 Kani bundle. There the fixed-width arithmetic is a
second implementation of the rule, kept because QSL's exemplar mutates its text, and held to the
kernel's meaning by a differential test (see "Two consumers, two shapes of the same rule").

The defect is reachable in two ways. Contract IR discharges the definedness obligation of an
arithmetic node from declared bounds and dominating guards, so an operand inside its declared
domain cannot overflow or divide by zero in an expression IR admitted. But the generated function
is `pub` and takes any `i64`, and FR-015 and `interface-001`'s `kani_slice` assert a
subject-produced value's domain rather than assuming it, so the oracle is evaluated at operand
values the declared domain excludes. Add, subtract and multiply over an integer type whose
overflow policy is `saturate` raise no obligation at all, so that arithmetic is admitted for
every operand value, and the raw operator is wrong on in-domain values too: over `0..=10` it
computes `10 + 1` as `11`, where the policy requires the result to stay in range.

### Status

The requirement is in two parts, because the division semantics were decided after the first part
merged (see "Integer divide and remainder").

- **IR-596's closing code change** implemented FR-031-AC-1, AC-3, AC-4, AC-5, AC-7, AC-8, AC-9,
  AC-10, AC-11, AC-12, AC-13, AC-14, AC-15, AC-19, AC-20 and AC-21. It refused divide and
  remainder at generation, with the interim code `UnsupportedIntegerDivision` naming IR-601, so
  that no oracle emitted a raw `/` or `%`. The QSL exemplar run of FR-031-AC-11 is still pending:
  it needs a checkout of the integration repository, and the run shall be recorded.
- **The divide and remainder change (IR-602)** is planned: FR-031-AC-2, AC-6, AC-16 and AC-17
  (held until the division semantics were decided), the rewritten FR-031-AC-18, and the new
  FR-031-AC-22 to AC-27. It removes the interim refusal `UnsupportedIntegerDivision` and states
  what each consumer does with a divide or remainder node now.
- **Prerequisite, not yet met.** Everything in the IR-602 change that calls `quire_exact::divide`
  is gated on the lock reaching a `quire-exact` that has it (see "Prerequisite: the one
  `quire-exact` lock entry"). The criteria stay planned until then.

### Scope

The Boolean oracle here is the output of `generate_boolean_oracle` and of the named generator
that the V1 Kani bundle, the V1 bound-oracle generator, the tri-state harness generator and the
Kani obligation clause lowering share, all in `src/oracle/boolean_v1.rs`. It is retired with that
file at AD-004 step 6, with the other V1 readers; step 4f deletes the V1 bundle and does not
delete this renderer, because the Kani obligation arm, the V1 bound generator and the tri-state
harness generator still call it. The exact V2 oracles of FR-014, FR-018 and FR-021 call
`quire_contract_runtime::exact` today and are unchanged here; IR-349 moves them to `quire-exact`,
so until then a native Boolean oracle's `Meter` is not the type those oracles take. The divide
and remainder change is gated on the lock prerequisite stated below.

This requirement states what the oracle emits and what it means. Whether the emitters of other
generators spell the same operators once, in a shared table, is a separate piece of work (IR-597)
and is not specified here.

### Basis

- Contract IR FR-013 declares each integer with an inclusive minimum and maximum and an overflow
  policy of `reject` or `saturate`. IR FR-015 makes saturating add, subtract, multiply and negate
  total, raises a non-zero-divisor obligation for every divide and remainder under both policies,
  and requires a range proof for `reject` arithmetic, including the minimum divided by negative
  one. IR proves the range of the selected result only: a divide's quotient, a remainder's
  remainder.
- Contract Runtime FR-006 and FR-007 give an exact integer operation one of four outcomes:
  completed, undefined, refused or incomplete. A zero divisor is `Undefined::DivisionByZero`. A
  result outside the supplied bound is `Refusal::IntegerOutOfDomain`. A false Boolean is a
  completed value, never a refusal. The runtime has no saturating integer operation.
  `exact::evaluate_integer_arithmetic` refuses exactly when the one result it computes is outside
  the bound, so for add, subtract and multiply it agrees with IR's per-result range proof on
  every operand IR admitted.
- `quire-exact` FR-357 (merged in `agent-ix/quire-exact`, which holds the exact kernel) gives
  division a single-member operation: `divide(profile, member, dividend, divisor, domain, meter)`
  returns one member of the quotient and remainder pair, and only that member must be in the
  domain. A zero divisor is `Undefined(DivisionByZero)`; an exposed member outside a bounded
  domain is `Refused(DivisionOutOfDomain { domain, member })`, with code `division_out_of_domain`
  and cause `quotient-outside-domain` or `remainder-outside-domain`. This is IR's member-only
  domain semantics, which the earlier pair operation did not have.
- `interface-001`'s `oracle_slice` states that no checked result is unwrapped or defaulted into
  `bool`, and ADR-003 states that a generated oracle calls the exact kernel so a proof covers the
  production code. Contract Runtime FR-275 states that, at its end state, generated oracles name
  the owning crate's own paths rather than a runtime re-export.

### Two consumers, two shapes of the same rule

The renderer serves two kinds of consumer, and they need different arithmetic.

- **The native oracle**: `generate_boolean_oracle`, `generate_bound_oracles` and anything else
  that a Rust program evaluates. A wrapped or panicking result is the defect there, so the
  arithmetic is an exact-kernel call returning a typed outcome.
- **The Kani bundle oracle**: the oracle that `generate_kani_bundle` embeds in a
  `proof_for_contract` harness. It is evaluated only by `cargo kani`. QSL's Kani exemplar
  (IT-011 in the integration repository, and IT-010-SC-05) runs the clause `amount < 1000` implies
  `amount + 1 <= 1000` over `0..=1000` through that bundle, and its mutation control rewrites the
  generated oracle's text: it requires exactly one `+` in the postcondition oracle and appends
  ` + 1_i64` to it, so that the rule reads `amount + 2 <= 1000`.

Real Kani measurements, made on a scratch crate built from the generator's own output for the
exemplar clause (Kani 0.68.0, CBMC 6.11.0, the exemplar's option vector):

- A raw infix `+` on `i64` under Kani already carries Kani's own check, "attempt to add with
  overflow", and the check is a falsifiable property. On the unmutated exemplar it is discharged
  (`SUCCESS`) and the proof verifies, 0 of 85 checks failed. On unconstrained operands, raw `+`
  fails with "attempt to add with overflow", and raw `/` fails with two distinct checks, "attempt
  to divide by zero" and "attempt to divide with overflow", each with a counterexample.
- The exemplar's non-equivalent mutant (`left + right + 1`) is falsified, 1 of 86 checks failed,
  with the concrete counterexample `amount_current = 999`. The equivalent mutant (dropping the
  `+ 1`) verifies, 0 of 84 checks failed.
- A whole-clause exact rendering (`exact::evaluate_integer_arithmetic` then `exact::order_numbers`,
  a `Meter` with unlimited limits, `amount` in `0..=1000`) gave no verdict in 800 seconds at
  unwind 4 with the same solver (a second run gave none in 1000 seconds), against 9 to 55
  seconds for the infix rendering. The exact add alone over `0..=999` verifies in 28 seconds
  (0 of 4705 checks failed), and the FR-015-AC-37 scalar harness verifies an exact add, so the cost
  measured is in the clause rendering through `order_numbers`, not in exact arithmetic as such.
  This requirement does not claim that exact arithmetic is intractable under CBMC.

The Kani bundle keeps a checked fixed-width path. The decisive reason is the exemplar's contract,
not the tractability measurement: IT-011-SC-03 rewrites the generated oracle's text and requires
exactly one `+` in it, and AD-004 L-5 requires the exemplar to pass through `generate_kani_bundle`
until step 4e. Add, subtract and multiply therefore stay the infix `i64` operators, in the text
shape the exemplar's mutation targets, and Kani's arithmetic
check is what makes overflow a failing property. For a `reject` type the proof is provable, because
Contract IR discharges the range obligation from declared bounds and the harness assumes the
declared domain; if that discharge were wrong, Kani would fail the proof.

This leaves two implementations of the rule: the kernel's exact operation for the native oracle
and the fixed-width operator for the bundle oracle (the second implementation IR-594 flagged). It
is justified by the exemplar's text-mutation contract and AD-004 L-5, which a call into the kernel
would break, and it is held in step by a differential
test (FR-031-AC-21 for add, subtract and multiply, FR-031-AC-25 for divide and remainder), not by
a copy of the arithmetic in a table. The two differ in one
documented way: on an operand outside the declared domain the native oracle returns the
kernel's refusal, and the bundle oracle fails Kani's overflow check, because under Kani the
harness assumes the declared domain on every drawn input and asserts it on every produced value
before the oracle runs (`ensures` is `(result bounds) && oracle`).

The same holds for divide and remainder, and the same measurement bears on it. `quire_exact::divide`
computes over `quire_exact::Integer`, an arbitrary-precision integer backed by `num-bigint`, so a
call to it under CBMC unwinds the big-integer vector growth and the division routine's recursion.
A scratch crate that called `divide` over `x` in `0..=10` and `y` in `1..=10` (Kani 0.68.0,
CBMC 6.11.0, unwind 6) compiled to a goto program and reached bounded model checking, and gave
no verdict in 900 seconds, still unwinding the `num-bigint` division recursion. That measures this
call and this solver only. It does not show that exact division is intractable under CBMC in
general, and a fixed-width path in the kernel might change it (Open decisions).

### Integer divide and remainder

IR admits a divide or remainder after proving only the selected member in range (IR FR-015): a
divide's quotient, a remainder's remainder. The earlier pair operation, which Contract Runtime
still carries until IR-349 step 3 and which this requirement does not call, refused when either
member was outside the domain, so it refused expressions IR admits. IR-601 ruled for member-only
semantics, and `quire_exact::divide` provides them. The two reproductions that separated the
semantics are now vectors of FR-031-AC-17:

- R-1, `10 / y <= 10` over a `reject` type of `1..=10`, at `y = 5`: the quotient 2 is in range and
  the remainder 0 is not. It completes, with the comparison `2 <= 10` true.
- R-2, `x % -1 <= 5` over a `reject` type of `-10..=5`, at `x = -10`: the remainder 0 is in range
  and the quotient 10 is not. It completes, with the comparison `0 <= 5` true.

The semantics, in the native oracle:

- **The member and the domain.** A divide selects `DivisionMember::Quotient` and a remainder
  `DivisionMember::Remainder`. The domain is `IntegerDomain::Bounded` over the node's own
  `[minimum, maximum]` interval, as for add, so the member that IR proved in range is the one the
  oracle admits. The generator renders neither `modulo` nor a division of the pair.
- **The law.** The generator selects `DivisionProfile::Truncating`. The truncating, floor and
  Euclidean laws agree on non-negative operands and differ on negative ones, and the IR divide
  and remainder nodes name no law, and the Boolean oracle has no package to select one from.
  Truncating is what the earlier raw `/` and `%` computed on every in-domain operand, what Rust's
  `checked_div` and `checked_rem` compute in IR's own arithmetic preimage, and so what the bundle
  oracle's operators mean. The law is one constant of the generator. That IR and QSL want it is an
  open decision (Open decisions).
- **A zero divisor** is `Undefined(DivisionByZero)`, for both members, and `divide` stops before
  it computes anything.
- **A member outside the interval** is `Refused(DivisionOutOfDomain { domain, member })`, with the
  node's interval as `domain`. This is the typed outcome for the minimum divided by negative one:
  the quotient is the exact integer 2^63, which no `i64` interval holds, so `i64::MIN / -1` is
  `Refused` with member `Quotient`, code `division_out_of_domain` and cause
  `quotient-outside-domain`. It is not an overflow, because the kernel's integer is unbounded,
  and it never panics.
- **The remainder of the minimum by negative one** is the exact integer 0. It is not out of
  domain and not an overflow: `i64::MIN % -1` completes with 0 when the node's interval holds 0,
  where the raw Rust operator panics.
- **Charges.** `divide` charges `integer-division.operands`, `integer-division.arithmetic`,
  `integer-division.domain` and `integer-division.result-retain`, and an incomplete outcome is
  the first stop like any other. No charge amount appears in the generated source.

Divide and remainder over a `saturate` integer type are refused with
`UnsupportedSaturatingArithmetic`, as add, subtract and multiply are. IR FR-015 does not make
division total under `saturate` and asks no range proof for it, so a quotient or remainder outside
the interval has no value under that policy except by clamping, and the kernel defines no
saturating operation.

The interim refusal is removed. Until this change, a divide or remainder node under either
overflow policy was refused with the code `UnsupportedIntegerDivision`, whose message named
IR-601. That code, the variant `GenerationErrorCode::UnsupportedIntegerDivision` and its stable
code string are deleted. This is a breaking change to the public error vocabulary, made on a
prerelease crate with no alias, shim or deprecated variant: a caller that matched the variant stops
compiling, and an artifact that records the string no longer names a code the crate emits.

## Inputs

- A validated typed Boolean expression, as `OracleRequest` carries it, whose nodes are within
  `interface-001`'s supported grammar plus integer add, subtract, multiply, divide and remainder.
- For each arithmetic node, the checked `IntegerType` that types its operands and its result:
  minimum, maximum, signedness and overflow policy.

## Outputs

- A generated Rust function and source map, as before, or a typed refusal with no partial
  artifact.
- For an expression with no arithmetic node, the same `bool`-returning function as before.
- For an expression with at least one arithmetic node, a native oracle is a function that returns
  `quire_exact::Outcome<bool>` and takes a trailing `&mut quire_exact::Meter`, as the FR-014
  oracles do (the `quire_exact` types are the planned ones, contingent as "The kernel the native
  oracle calls" states; the oracle IR-596 implemented uses Contract Runtime's). A Kani bundle oracle is a function that
  returns `bool`, as before.

## Behavior

### Prerequisite: the one `quire-exact` lock entry

CG's `Cargo.lock` holds one `quire-exact` entry (at a270df3 when this was measured), shared with
`qsl-replay` and `qsl-semantics`, and the one-copy rule (`make deny`) forbids a second. The merged
single-member `divide` is in a newer `quire-exact` that deletes the pair `divide`,
`QuotientRemainder` and `DivisionPairOutOfDomain`, and QSL's `main` still calls all three
(`qsl-semantics/src/value/definition.rs`, `qsl-foundation/src/diagnostic.rs`, and tests in
`qsl-replay` and `qsl-semantics`, measured on QSL `main` c5fcc438). Bumping CG's one entry now
would break the build of CG's QSL dependency. The order is therefore: QSL moves to the
single-member `divide`; then CG bumps its QSL dependency and `quire-exact` together. Until then,
every criterion that needs `quire_exact::divide` is not buildable: FR-031-AC-2, AC-6, AC-16 and
AC-17, the migration criteria AC-22 and AC-26, and the divide and remainder differential
FR-031-AC-25 against the kernel, and the removal of the interim code, FR-031-AC-27.
FR-031-AC-18 (the `saturate` divide and remainder refusals), AC-23 and AC-24 (the bundle's own
operators), which call no kernel, could land first, with the native oracle still refusing a divide
or remainder over `reject` under the interim code, because removing that code before the native
division exists would leave the native oracle with no divide outcome at all. This requirement does
not require that split. Nothing that
calls `quire_exact::divide`, or that depends on the whole-oracle migration, is buildable before the
prerequisite.

### The kernel the native oracle calls

This subsection is contingent on the prerequisite above and on OD-2 (the owner and the IR-349
owner confirming that the native oracle takes the `quire_exact` family before IR-349 removes
Contract Runtime's copy). Until both hold it states the planned end state, not a settled rule.

- The native oracle shall take every exact operation from `quire_exact`, the one exact kernel, and
  from no other path: the generated source aliases it as `rt`, and `exact::` in this requirement
  names its items. Its `Outcome`, `Meter` and `Integer` are `quire_exact`'s. It shall contain no
  `quire_contract_runtime::exact` path.
- The short-circuit connectives cannot keep their current rendering. The oracle calls Contract
  Runtime's `operators::{and,or,implies}_short_circuit<R: From<bool>>`, which compile for
  `Outcome<bool>` only because Contract Runtime's `Outcome` implements `From<T>`.
  `quire_exact::Outcome` has no such impl, and the orphan rule stops a generated crate adding one.
  `quire_exact` exports `evaluate_boolean` and `retain_boolean`, which charge `boolean.result-retain`,
  and no short-circuit helper. The migrated oracle shall render a short-circuit connective as an
  inline `match` over the left operand's outcome that returns the first stop, skips the right
  operand when the left decides, and charges nothing (the runtime operators it replaces charge
  nothing), and a total connective as an inline evaluation of both operands in order that
  returns the first stop, with no `evaluate_boolean` call.
- The reason is Rust type identity. Contract Runtime carries its own `Integer`, `Meter` and
  `Outcome` until IR-349 removes them, and they are different types from `quire_exact`'s. One
  oracle body that adds, divides and compares passes one integer value and one `&mut Meter`
  through every call, so it cannot call both families, and a bridge between the two would be a
  compatibility layer. `quire_exact::divide` is the only division that takes IR's member-only
  semantics, so the whole native oracle takes the `quire_exact` family. This moves the add,
  subtract, multiply and comparison calls that IR-596 emitted against Contract Runtime's copy
  to the same calls in `quire_exact`, with the same meaning (Open decisions records the
  sequencing against IR-349).
- A generated crate that holds a native oracle with an arithmetic node shall depend on
  `quire-exact`, and CG shall depend on `quire-exact` directly, from its own repository
  `agent-ix/quire-exact`, spelled `branch = "main"` like the other first-party crates, so that
  the lock holds one `quire-exact` entry. Contract Runtime stays a dependency of the generated
  crate for the contract identity items the oracle declares.
- An oracle with no arithmetic node, which returns `bool` and takes no meter, calls no kernel item
  and is unchanged.

### Arithmetic in the native oracle

- The generator shall render integer add, subtract and multiply over a `reject` type as a call of
  `exact::evaluate_integer_arithmetic` with the matching `IntegerArithmetic` variant and the
  node's `[minimum, maximum]` interval as the result bound.
- The generator shall render integer divide and remainder over a `reject` type as a call of
  `exact::divide` with `DivisionProfile::Truncating`, `DivisionMember::Quotient` for a divide and
  `DivisionMember::Remainder` for a remainder, the two operand integers, `IntegerDomain::Bounded`
  over the node's `[minimum, maximum]` interval, and the meter (see "Integer divide and
  remainder").
- The generator shall emit no Rust arithmetic operator between two operand expressions and no
  integer method such as `checked_add`, `wrapping_add`, `saturating_add`, `overflowing_add`,
  `checked_div`, `checked_rem`, `wrapping_rem`, `div_euclid` or `rem_euclid` for these operators.
  Every arithmetic operation in the emitted body shall be an `exact::` call, and
  no charge amount shall appear in the source.
- The generated oracle shall take the operands of an arithmetic call as the exact integers of the
  operand expressions, converted from the function's `i64` parameters and literals, and an operand
  that is itself an arithmetic node shall contribute its completed integer.

| `NumericOperator` | Emitted evaluation | Outcome when the result is outside the interval |
|---|---|---|
| Add | `evaluate_integer_arithmetic(Add(l, r), Some(interval), meter)` | `Refused(IntegerOutOfDomain)` |
| Subtract | `evaluate_integer_arithmetic(Subtract(l, r), Some(interval), meter)` | `Refused(IntegerOutOfDomain)` |
| Multiply | `evaluate_integer_arithmetic(Multiply(l, r), Some(interval), meter)` | `Refused(IntegerOutOfDomain)` |
| Divide | `divide(Truncating, Quotient, l, r, Bounded(interval), meter)` | `Refused(DivisionOutOfDomain { member: Quotient })`; a zero divisor is `Undefined(DivisionByZero)` |
| Remainder | `divide(Truncating, Remainder, l, r, Bounded(interval), meter)` | `Refused(DivisionOutOfDomain { member: Remainder })`; a zero divisor is `Undefined(DivisionByZero)` |

### Arithmetic in the Kani bundle oracle

- The generator shall render integer add, subtract and multiply over a `reject` type, in the
  oracle that `generate_kani_bundle` embeds, as the Rust infix operators `+`, `-` and `*` on the
  two `i64` operand expressions, in the parenthesized text shape the generator emits today, with
  one binary operator per arithmetic node. It shall not render them as an `exact::` call, a
  `checked_`, `wrapping_`, `saturating_` or `overflowing_` method, or any helper call, so that
  the one `+` of the exemplar clause stays one syntactic addition whose text can be extended.
- The generator shall render integer divide over a `reject` type, in that oracle, as the Rust
  infix operator `/` on the two `i64` operand expressions, and integer remainder as the method call
  `wrapping_rem` on the left operand with the right operand as its argument, each in the
  parenthesized text shape of the other operators, with one operator or one call per arithmetic
  node. It shall not render either as an `exact::` call, as `modulo`, `div_euclid`, `rem_euclid`,
  a `checked_`, `saturating_` or `overflowing_` method, or any helper call, and `wrapping_rem` on
  a remainder node is the only method call it may emit for arithmetic. Both operators mean the
  truncating law, the law the native oracle selects.
- Why divide is infix and remainder is not. Kani 0.68.0 measures raw `/` on `i64` carrying two
  checks, "attempt to divide by zero" and "attempt to divide with overflow". IR FR-015 raises a
  non-zero-divisor obligation for every divide and, under `reject`, requires a range proof of the
  quotient against the type, which lies inside `i64`, so a divide whose quotient can reach the
  minimum divided by negative one (2^63) is refused. That guarantee is what makes raw `/` safe: on
  a declared-domain operand neither check can fire, and a failure is a falsifiable property
  exactly as for add (a `check_expression` run over a full-range guarded divide, which refuses it,
  is an observation of the guarantee, not its basis). Raw `%` carries a third check, "attempt to
  calculate the remainder with overflow", at the minimum by negative one, where the language and
  the kernel both give 0, and IR admits a remainder with that operand pair in its domain, because
  the remainder's magnitude is at most 2^63-1 (a remainder over the full `i64` type
  guarded by `y != 0` is admitted by `check_expression`). A raw `%` would therefore fail the proof
  of a correct clause at an operand the declared domain holds. `wrapping_rem` returns the truncating
  remainder for every non-zero divisor, 0 at that operand pair included, and keeps Kani's check on
  a zero divisor (measured: verified at the minimum by negative one, failed with "attempt to
  calculate the remainder with a divisor of zero" on unconstrained operands). It wraps no result,
  because a remainder always lies within the magnitude of its divisor.
- The bundle oracle shall return `bool`, shall take no meter, and shall contain no `unwrap`,
  `expect` or panic macro. Under Kani its arithmetic carries Kani's own overflow check, whose
  failure is a falsifiable property of the proof; for divide and remainder the falsifiable checks
  are the divide and remainder checks on a zero divisor and, for divide, on overflow.
- The generated file of a bundle that holds an arithmetic node shall state in a comment that its
  arithmetic is checked by Kani and that the file is not a native evaluator. The bundle is
  consumed only by `cargo kani`.

### Refusals

- The code `UnsupportedIntegerDivision` does not exist. The generator shall emit no diagnostic
  that names IR-601, and a divide or remainder node over a `reject` type is not refused for being
  a division (see "Integer divide and remainder" for the removal).
- If an add, subtract, multiply, divide or remainder node's integer type has overflow policy
  `saturate`, in either consumer, then the
  generator shall refuse the expression with the code `UnsupportedSaturatingArithmetic` at that
  node's source span, as the first unsupported node in authored preorder, and emit no artifact.
  The diagnostic's message shall name the missing Contract Runtime saturating integer operation.
  The generator shall not restate saturation inline: clamping a result to the interval would be a
  second implementation of the rule. A `saturate` integer that is only compared, never operated
  on, is unaffected.
- `UnsupportedSaturatingArithmetic` has terminal state `unsupported`, as `UnsupportedExpression`
  does, and its message text shall not be used as machine identity.

### Outcomes and their propagation in the native oracle

- The generated oracle shall return `Completed(false)` for a comparison that evaluates to false
  and `Completed(true)` for one that evaluates to true. Neither is a refusal.
- The generated oracle shall return as its outcome the first outcome that is not completed, in
  evaluation order: left operand before right operand, and the operands of a comparison before the
  comparison. It shall not unwrap, default, ignore or convert such an outcome to `true` or
  `false`, and its source shall contain no `unwrap`, `expect` or panic macro.
- A short-circuit connective in the generated oracle shall evaluate its right operand only when
  its left operand does not decide the result, so a stop its right operand could produce does not
  arise when the left decides. A total connective shall evaluate both operands from left to right
  and its outcome shall be the first stop.

### Comparison

The table is the native oracle's. The Kani bundle oracle keeps the native operator and its `bool`
result for every comparison, whether or not an operand holds arithmetic, since its arithmetic is
fixed-width and its comparison of two `i64` values has no failure mode.

| `ComparisonOperator` | Both operands arithmetic-free | An operand holds arithmetic |
|---|---|---|
| Equal | `l == r` on the two `i64` values | `exact::planned_equality` over `Value::Integer` of the two exact integers |
| NotEqual | `l != r` on the two `i64` values | the same, with a completed Boolean negated and any other outcome returned unchanged |
| Less | `l < r` | `exact::order_numbers` with `OrderingOperator::Less` over `Integers` |
| LessEqual | `l <= r` | `exact::order_numbers` with `OrderingOperator::LessOrEqual` |
| Greater | `l > r` | `exact::order_numbers` with `OrderingOperator::Greater` |
| GreaterEqual | `l >= r` | `exact::order_numbers` with `OrderingOperator::GreaterOrEqual` |

- The generator shall keep the native operator and the `bool` result for a comparison of two
  arithmetic-free `i64` values, which has no failure mode. That comparison is held to the
  kernel's meaning by a differential test (FR-031-AC-4), not by a call.
- The generator, other than for the Kani bundle oracle, shall not use a Rust comparison operator
  between the two operands of a comparison that has an arithmetic operand. Its outcome is the
  kernel's, and an `Outcome::Completed` boolean continues as above.
- `quire_exact` exports no type environment or checked equality, which the earlier rendering of
  equal and not-equal called through Contract Runtime's copy. `planned_equality` is the equality
  schedule the kernel exports over two completed values, and it is the call that the table names.
  For two integers it is the same value and the same charges as the checked equality (Open
  decisions, OD-3). The table is the migrated oracle's, and is contingent as "The kernel the
  native oracle calls" states. Until the migration, the oracle IR-596 implemented renders the
  Equal and NotEqual rows through Contract Runtime's `TypeEnvironment::check_equality` and
  `CheckedEquality::evaluate`, with the same value and charges.

### Consumers

- A generator other than the V1 Kani bundle that embeds the Boolean oracle where a plain `bool` is
  required shall refuse a clause that holds an arithmetic node, shall embed no raw arithmetic operator, and shall not read
  an `Outcome<bool>` as a `bool`. The tri-state harness generator shall refuse a clause whose
  first arithmetic node is an add, subtract, multiply, divide or remainder over a `reject` type
  with `UnsupportedExpression` at that node, in addition to the `UnsupportedDependency` it already
  returns for an integer dependency; a `saturate` node of any of the five operators is refused by
  the refusals above, with `UnsupportedSaturatingArithmetic`, for every consumer. Bound oracle
  generation is all-or-nothing over a package, so one clause the refusals above refuse refuses the
  bound strategy for every clause of that package, and the `UnsupportedClause` names the refused
  clause, not the requested one. The bound strategy
  generators shall keep the `UnsupportedRelation` they return for a clause whose oracle generates,
  such as a discharged add or a guarded divide used as a comparison operand, and the
  `UnsupportedClause` carrying the oracle's code and span for a clause the oracle refuses, which
  includes every clause refused by the refusals above. The Kani obligation clause lowering shall
  keep its definedness refusal for a clause that carries an obligation, which is every `reject`
  arithmetic node, and shall take the oracle's refusal otherwise.
- The V1 Kani bundle (`generate_kani_bundle`) is a consumer that needs a plain `bool`, because it
  places the oracle in a `requires` and an `ensures` clause. It shall use the Kani bundle oracle
  above for add, subtract, multiply, divide and remainder over a `reject` type, and it shall
  refuse a clause that holds a `saturate` arithmetic node of any of the five operators with
  `ClauseGenerationFailed`, retaining the oracle's own code (`UnsupportedSaturatingArithmetic`)
  and the node's span, and emit no harness. It shall not carry
  an `Outcome<bool>`. The exemplar's text-mutation contract and AD-004 L-5 require the infix
  shape (see the measurements above), and the bundle is deleted at AD-004 step 4f, where FR-014-AC-38 and FR-015-AC-40 and AC-41 own
  the `Outcome<bool>` oracle of a V2 clause body. The covers that FR-015-AC-53 to AC-58 add to V1
  bundle harnesses apply to a bundle with arithmetic as to any other.
- QSL's exemplar is the regression gate of this consumer. IT-011 in the integration repository
  runs the exemplar clause through `generate_kani_bundle` and real `cargo kani`, with its
  controls IT-011-SC-01 to SC-04 (the verifying run, the connective mutation and the addition
  mutation), and IT-010-SC-05 runs a comparison clause through the same bundle. AD-004 step 4a and
  L-5 require that control to pass through `generate_kani_bundle` until step 4e, so the exemplar's
  verifying run and both mutants (`left + right + 1` falsified at `amount_current = 999`, and the
  equivalent mutant verifying) are required results of the closing code change.

### Test obligations

- The overflow cases of FR-031-AC-5, the `saturate` cases of FR-031-AC-7, and the divide and
  remainder cases of FR-031-AC-6, AC-16 and AC-18 shall each exist as a test that fails on the
  tree before the closing code change, and that change shall record the failing run of each,
  before it records the passing run. A test that passes on the tree before the change does not
  establish the defect and does not count toward these criteria. For AC-6 and AC-16 the tree
  before the divide and remainder change refuses each construction with the interim code, so the
  assertion of the kernel's outcome fails there; the cases of AC-17 fail the same way. The
  held agreements, FR-031-AC-4, AC-12, AC-13 and AC-14, are exempt as before.
- The cases are constructed through `DeclarationEnvironment::check_expression`, so IR admits
  each. Operands the declared domain excludes are the only way to reach an overflow or a zero
  divisor through an expression IR admits under `reject`, and the proof of safety in each
  construction rests on a declared bound that the operand vector violates while the guards still
  pass. No case is replaced by a nearby vector.
- The closing code change shall run the exemplar tests of the integration repository
  (`tests/qsl_kani_exemplar.rs`: IT-011 and IT-010-SC-05) against its branch, before and after
  the change, and record both runs. That needs a checkout of the integration repository with its
  codegen dependency pointed at the branch, and the installed Kani backend. A run that cannot be
  made is reported as not run and is not read as a pass.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-031-AC-1 | IMPLEMENTED (IR-596). For each of Add, Subtract and Multiply over a `reject` integer type, the generated source of the native oracle calls `exact::evaluate_integer_arithmetic` with the matching `IntegerArithmetic` variant and the type's interval as the bound, and the oracle's body holds no Rust arithmetic operator between operand expressions and none of `checked_`, `wrapping_`, `saturating_` or `overflowing_` method calls. | Test (TC-044) |
| FR-031-AC-2 | PLANNED (IR-602), GATED on the lock prerequisite. For Divide and Remainder over a `reject` integer type, the generated source of the native oracle calls `exact::divide` with `DivisionProfile::Truncating`, with `DivisionMember::Quotient` for a divide and `DivisionMember::Remainder` for a remainder, and with `IntegerDomain::Bounded` over the node's interval as the domain, and the oracle's body holds no Rust `/` or `%` and none of `checked_div`, `checked_rem`, `wrapping_rem`, `div_euclid`, `rem_euclid` or `modulo`, and no division of a quotient and remainder pair. | Test (TC-044) |
| FR-031-AC-3 | IMPLEMENTED (IR-596). A native oracle with an arithmetic node returns `Outcome<bool>` and takes a trailing `&mut Meter`; an oracle with none returns `bool` and takes no meter. | Test (TC-044) |
| FR-031-AC-4 | IMPLEMENTED (IR-596). Each of the six comparisons over two arithmetic-free operands is emitted as the one native operator the table names, and over the seven values `i64::MIN`, `i64::MIN + 1`, `-1`, `0`, `1`, `i64::MAX - 1` and `i64::MAX`, with two interior values added, taken pairwise in both orders, agrees with the exact kernel's own evaluation, as the test takes it from Contract Runtime today (`order_numbers` for the four orderings, `check_equality` then `CheckedEquality::evaluate` for equal and not-equal; after the migration of FR-031-AC-22, `quire_exact`'s `order_numbers` and `planned_equality`). This criterion passes on the tree before the change and is a held agreement, not a defect proof. | Test (TC-044) |
| FR-031-AC-5 | IMPLEMENTED (IR-596). Under a `reject` integer type, an add, subtract or multiply result outside the type's interval evaluates to `Refused(IntegerOutOfDomain)`, and the oracle neither panics nor returns a wrapped, saturated or clamped value. The cases are O-1 `i64::MAX + 1`, O-2 `i64::MIN - 1` as a subtract, and O-4 `i64::MIN * 2`, each built as TC-044 step 4 gives. On the tree before the change each fails: a debug build panics with an overflow message, a release build returns a wrapped value, and no case returns the refusal. | Test (TC-044) |
| FR-031-AC-6 | PLANNED (IR-602), GATED on the lock prerequisite. Under a `reject` integer type, a zero divisor evaluates to `Undefined(DivisionByZero)` for both Divide and Remainder, for the cases Z-1 `x / 0`, Z-2 `0 / 0`, Z-3 `x % 0` and Z-4 `0 % 0`, each built as TC-044 step 5 gives (Z-3 and Z-4 reach the zero through the add `y + 11`, which completes with 0), and the oracle does not panic and does not return `Refused`. On the tree before the change each case is refused at generation with the interim code, and a raw operator would panic with a division-by-zero message. | Test (TC-044) |
| FR-031-AC-7 | IMPLEMENTED (IR-596). An add, subtract or multiply over a `saturate` integer type is refused with `UnsupportedSaturatingArithmetic` at the node's span, with no artifact, for the rows `x + 1` at `x = i64::MAX` and `x - 1` at `x = i64::MIN` over an integer type of `i64::MIN..=i64::MAX`, and `x * 2` at `x = i64::MAX` over `0..=10` (where an in-domain `x = 10` also gives the wrong value `20 <= 10` against the policy's `10 <= 10`); the refusal span is the first such node in authored preorder when a negation also occurs later, and the diagnostic message names the missing runtime saturating operation. On the tree before the change each expression generates, and the generated function panics in a debug build on the named operands. | Test (TC-044) |
| FR-031-AC-8 | IMPLEMENTED (IR-596). In a generated crate compiled and executed against `quire-contract-runtime`, the outcome of a native add, subtract or multiply oracle equals a plain-integer model of IR FR-015's member-range semantics, computed in the test with `i128` independently of the emitter and of any `exact::` call: `Completed` with the exact result when it lies in the type's interval and `Refused(IntegerOutOfDomain)` when it does not. The grid covers operand values inside the declared domain, at its edges and outside it, and the outcome kind is compared, so no vector returns `Completed` where the model refuses. | Test (TC-044) |
| FR-031-AC-9 | IMPLEMENTED (IR-596). The first non-completed outcome in evaluation order is the oracle's outcome: an arithmetic stop in the left operand of a short-circuit connective is returned, a stop in a right operand the left decided is never reached, a total connective returns the first stop of its two operands, and a stop is never returned as `Completed(true)` or `Completed(false)`. | Test (TC-044) |
| FR-031-AC-10 | IMPLEMENTED (IR-596). The tri-state harness generator, the bound strategy generators and the Kani obligation clause lowering each refuse a clause holding an arithmetic node with the refusal the Consumers section names for it, emit no raw arithmetic operator, and never read an `Outcome<bool>` as `bool`. | Test (TC-044) |
| FR-031-AC-11 | IMPLEMENTED IN CG (IR-596); THE QSL EXEMPLAR RUN IS PENDING. The clause that runs QSL's IT-011 and IT-010-SC-05 against this branch is pending the quire-integration run; CG's in-repo run of the same bundle shape under real Kani is the closest equivalent and does not stand for it. `generate_kani_bundle` over the postcondition `amount < 1000` implies `amount + 1 <= 1000`, with `amount` an integer of `0..=1000` under `reject`, generates a bundle whose postcondition oracle returns `bool`, holds exactly one syntactic addition (`syn::ExprBinary` with `BinOp::Add`) and no `exact::` call, and, run with real `cargo kani` as QSL's IT-011 and IT-010-SC-05 run it against this branch, verifies, while the mutant that appends ` + 1_i64` to the addition (`left + right + 1`) is falsified with the counterexample `amount_current = 999`, and the equivalent mutant that drops the `+ 1` verifies; over a `saturate` node of any of the five operators the bundle returns `ClauseGenerationFailed` retaining the oracle's own code (`UnsupportedSaturatingArithmetic`) and emits no harness. (IR-596 had this refusal retain `UnsupportedIntegerDivision` for a divide or remainder too; IR-602 removes that case, FR-031-AC-23.) | Test (TC-044), Test (quire-integration IT-011) |
| FR-031-AC-12 | IMPLEMENTED (IR-596). The oracle generated for an expression with no arithmetic node is byte-identical to the oracle the generator produced for it before this requirement. This criterion passes on the tree before the change and is a held agreement, not a defect proof. | Test (TC-044) |
| FR-031-AC-13 | IMPLEMENTED (IR-596). Generating the oracle of an expression with an arithmetic node twice, and from a permuted request, yields identical bytes. This criterion passes on the tree before the change and is a held agreement, not a defect proof. | Test (TC-044) |
| FR-031-AC-14 | IMPLEMENTED (IR-596). The generated source of an oracle with an arithmetic node contains no `unwrap`, `expect` or panic macro. This criterion passes on the tree before the change and is a held agreement, not a defect proof. | Test (TC-044) |
| FR-031-AC-15 | IMPLEMENTED (IR-596). In the native oracle, each of the six comparisons that has an arithmetic operand is emitted as the exact-kernel call the table names (for Equal and NotEqual, until the migration of FR-031-AC-22, the checked equality of Contract Runtime that the Comparison section records), with no Rust comparison operator between its two operands. | Test (TC-044) |
| FR-031-AC-16 | PLANNED (IR-602), GATED on the lock prerequisite. Under a `reject` integer type of `-5..=5`, the minimum divided by negative one is out of domain and the minimum remainder negative one is not, and neither panics: O-3 `i64::MIN / -1` evaluates to `Refused(DivisionOutOfDomain)` whose member is `Quotient` and whose domain is `-5..=5`, with the refusal's code `division_out_of_domain` and cause `quotient-outside-domain`; O-5 `i64::MIN % -1` evaluates to `Completed(true)`, the remainder being the exact 0 and `0 <= 5` true. Each is built as TC-044 step 4 gives. On the tree before the change each construction is refused at generation with the interim code, and a raw operator would panic with an overflow message. | Test (TC-044) |
| FR-031-AC-17 | PLANNED (IR-602), GATED on the lock prerequisite. In a generated crate, the outcome of a divide or remainder oracle equals a plain-integer model of IR's member-only semantics under the truncating law, computed with `i128` independently of the emitter and of any `exact::` call (zero divisor: `Undefined(DivisionByZero)`; the selected member in the type's interval: `Completed` with the comparison's Boolean over that member; otherwise `Refused(DivisionOutOfDomain)` with that member), on a grid of dividends and divisors inside the declared domain, at its edges and outside it, that includes the two reproductions as named vectors: R-1 `10 / y <= 10` over a `reject` type of `1..=10` at `y = 5`, which completes with `true` (the quotient 2 is in range and the remainder 0 is not), and R-2 `x % -1 <= 5` over `-10..=5` at `x = -10`, which completes with `true` (the remainder 0 is in range and the quotient 10 is not). The outcome kind is compared, so neither a `Refused` where the model completes, which is what a pair-admitting division returns on R-1 and R-2, nor a `Completed` where the model refuses, passes. The oracle returns `Outcome<bool>`, so the member's value is seen only through the comparison, and R-1 and R-2 complete `true` under every law; the law is pinned by AC-2 in the source and behaviourally by the grid vector `y != 0 && x / y <= -4` over a `reject` type of `-10..=10` (built through `check_expression`) at `x = -7`, `y = 2`, which is `Completed(false)` under truncating (quotient -3) and `Completed(true)` under floor and Euclidean (quotient -4). | Test (TC-044) |
| FR-031-AC-18 | PLANNED (IR-602). A divide or remainder node over a `saturate` integer type is refused with `UnsupportedSaturatingArithmetic` at the node's span, with no artifact and no raw `/` or `%` in any output, for the `saturate` rows S-1 to S-4 of TC-044 step 11, in both consumers; the diagnostic message names the missing saturating operation and the refusal has terminal state `unsupported`. On the tree before the change each row is refused with `UnsupportedIntegerDivision` instead. This criterion calls no kernel and can land before the lock prerequisite, while the interim code still exists for a divide or remainder over `reject`. | Test (TC-044) |
| FR-031-AC-27 | PLANNED (IR-602), GATED on the lock prerequisite (it cannot land before the native division of AC-2 exists). The code `UnsupportedIntegerDivision` does not exist: the crate's `GenerationErrorCode` has no such variant, nothing under `src/`, `tests/` or `schemas/` spells it, no stable code string of that name is emitted, and no diagnostic message that generation can emit names IR-601. On the tree before the change the variant exists and a divide or remainder over `reject` is refused with it. | Test (TC-044) |
| FR-031-AC-28 | The V1 Boolean oracle admits a typed IR integer domain or literal only when its value and both inclusive domain endpoints fit i64. A minimum of `i64::MIN - 1` or `i128::MIN`, or a maximum of `i64::MAX + 1` or `i128::MAX`, produces `UnsupportedExpression` at the first affected expression source span and no generated source; the same rule applies to an out-of-range literal. Values are never narrowed, clamped or wrapped to create an oracle. | Test (TC-044) |
| FR-031-AC-19 | IMPLEMENTED (IR-596). The Kani bundle oracle of an expression whose arithmetic nodes are add, subtract and multiply renders them as the infix `+`, `-` and `*` on `i64` with one binary operator per arithmetic node, contains no `exact::` call and no `checked_`, `wrapping_`, `saturating_` or `overflowing_` method (a remainder node's `wrapping_rem` is FR-031-AC-23's, not this criterion's), takes no meter, and its file states that its arithmetic is checked by Kani and is not a native evaluator. | Test (TC-044) |
| FR-031-AC-20 | IMPLEMENTED (IR-596). Under real Kani, a harness that calls a bundle oracle with unconstrained operands at which IR's discharge does not hold fails with Kani's arithmetic-overflow check and a counterexample: the oracle of `x < 5 && x * 2 <= 10` over `0..=10` under `reject`, called with `x` unconstrained, fails the "attempt to multiply with overflow" check, and the counterexample's playback value satisfies `x < 5` and overflows `x * 2` (any such value is valid; the assertion does not name one); a hand-written probe harness that assumes the declared domain on `x`, calls the same bundle oracle and discards its value has no failing arithmetic-overflow check, and verifies. (The bundle's own `proof_for_contract` harness is not that probe: its `ensures` asserts the clause, which is false for `x` in `5..=10`.) Overflow in the bundle oracle is a falsifiable property, never a wrapped value. | Test (TC-044) |
| FR-031-AC-21 | IMPLEMENTED (IR-596). For each of add, subtract and multiply, the bool the Kani bundle oracle returns equals the `bool` inside `Completed` of the native oracle of the same typed expression, on every vector of a grid inside the declared domain at which the expression's guards pass and IR's discharge holds, including the domain's edges, with the bundle oracle compiled and run natively in a generated crate against `quire-contract-runtime` in plain `cargo test`, with no Kani. This is the differential that holds the two implementations of the rule in step. | Test (TC-044) |
| FR-031-AC-22 | PLANNED (IR-602), CONTINGENT on OD-2 and on the lock prerequisite. The generated source of a native oracle with an arithmetic node aliases `quire_exact` as `rt`, takes `&mut rt::Meter`, returns `rt::Outcome<bool>`, and contains no `quire_contract_runtime::exact` path; a generated crate that compiles and runs it, with divide and remainder nodes and with add, subtract and multiply nodes in the same oracle (the construction Z-3, `y <= 0 && x % (y + 11) <= 10`, is one such oracle), resolves one `quire-exact`; CG's own `Cargo.toml` depends on `quire-exact` directly from `agent-ix/quire-exact`, and CG's lock holds one `quire-exact` entry (`make deny`'s one-copy check), at a revision that has the single-member `divide`, with `qsl-replay` and `qsl-semantics` resolving it. Once it holds, the generated crates of FR-031-AC-8, AC-21 and AC-25 depend on `quire-exact`, and AC-4's reference for equal and not-equal is `planned_equality`. | Test (TC-044) |
| FR-031-AC-26 | PLANNED (IR-602), CONTINGENT on OD-2 and on the lock prerequisite. The generated source of a native oracle holds no `quire_contract_runtime::operators` path and no `and_short_circuit`, `or_short_circuit` or `implies_short_circuit`; a short-circuit connective is an inline `match` over the left operand's `rt::Outcome` that returns the first stop and does not evaluate the right operand when the left decides, and a total connective evaluates both operands in order and returns the first stop, none charging any point and none calling `evaluate_boolean`; in a generated crate the outcomes of FR-031-AC-9's cases (a stop in a left operand, a stop in a right operand the left decided, each operand of a total connective) and of Z-3 are unchanged from the Contract Runtime rendering. | Test (TC-044) |
| FR-031-AC-23 | PLANNED (IR-602). The Kani bundle oracle renders Divide over a `reject` type as the infix `/` and Remainder as `wrapping_rem` on `i64`, one operator or call per arithmetic node, contains no `exact::` call and no `modulo`, `div_euclid`, `rem_euclid`, `checked_`, `saturating_` or `overflowing_` method, no `wrapping_` method other than `wrapping_rem` on a remainder node, takes no meter, returns `bool`, and its file states that its arithmetic is checked by Kani. `generate_kani_bundle` over a divide or a remainder over a `reject` type generates a bundle, and over a `saturate` divide or remainder it returns `ClauseGenerationFailed` retaining `UnsupportedSaturatingArithmetic` and emits no harness. | Test (TC-044) |
| FR-031-AC-24 | PLANNED (IR-602). Under real Kani, a harness that calls the bundle oracle of the construction Z-1 (a divide) with unconstrained operands fails the checks "attempt to divide by zero" and "attempt to divide with overflow", each with a counterexample, and one that calls the bundle oracle of the construction Z-3 (a remainder) with unconstrained operands fails the check "attempt to calculate the remainder with a divisor of zero" and no other arithmetic check; a probe harness that assumes the declared domain and the guards, calls the same oracle and discards its value verifies, for the divide `10 / y <= 10` over `1..=10` and the remainder `x % -1 <= 5` over `-10..=5`; and a probe of the remainder `y != 0 && x % y <= 0` over `i64::MIN..=i64::MAX` at `x = i64::MIN`, `y = -1` verifies, where a raw `%` fails "attempt to calculate the remainder with overflow". | Test (TC-044) |
| FR-031-AC-25 | PLANNED (IR-602), GATED on the lock prerequisite (its native side calls the kernel). For divide and for remainder, the bool the Kani bundle oracle returns equals the `bool` inside `Completed` of the native oracle of the same typed expression, on every vector of a grid inside the declared domain at which the expression's guards pass and the divisor is non-zero and IR's discharge holds, including the domain's edges and R-1 and R-2, and including the remainder `y != 0 && x % y <= 0` over `i64::MIN..=i64::MAX` at `x = i64::MIN`, `y = -1` (native `Completed(true)`, bundle `true`, where a raw `%` panics in a debug build), with the bundle oracle compiled and run natively in a generated crate against `quire-exact` in plain `cargo test`, with no Kani. | Test (TC-044) |

## Dependencies

- **Upstream**: `interface-001` (`oracle_slice`), [FR-014](./FR-014-exact-scalar-oracles.md) for the
  meter and `Outcome` convention of an exact oracle, [FR-018](./FR-018-composite-equality-oracles.md)
  for the equality evaluation, [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md)
  and [FR-028](../../kani/functional/FR-028-bounded-proof-ceilings.md) for the Kani bundle and its
  ceilings, [FR-008](../../strategy/functional/FR-008-bound-domain-strategy-admission.md) for the
  strategy admission, Contract IR FR-013 and FR-015, Contract Runtime FR-002, FR-006 and
  FR-007 (the identity items of a generated crate), and `quire-exact` FR-357 for the division
  operation. The IR-602 code change is blocked on QSL moving to the single-member `divide` and CG
  then bumping QSL and `quire-exact` together ("Prerequisite: the one `quire-exact` lock entry").
- **Downstream**: [TC-044](../matrix/TC-044-boolean-oracle-integer-arithmetic.md).

## Open decisions

OD-1, OD-2, OD-4 and OD-6 are not settled here. Each is stated as a position this requirement takes,
with who decides it, so that the code change does not hide a guess. OD-3 is closed.

- **OD-1, the law.** The generator selects `DivisionProfile::Truncating`. IR's divide and
  remainder nodes name no law, IR's own arithmetic preimage computes the truncating one, and
  so did the earlier raw operators, but no ruling names it for V1. To confirm with IR and QSL. If
  the language wants floor or Euclidean division here, the constant and the bundle oracle's
  `/` and `wrapping_rem` both change, because Rust's operators are truncating.
- **OD-2, the kernel family and its sequencing.** The planned end state is that the native oracle
  takes the whole `quire_exact` family (see "The kernel the native oracle calls"), which moves
  the add, subtract, multiply, comparison and connective renderings that IR-596 implemented off
  Contract Runtime's copy before IR-349 step 1 removes that copy. The migration criteria (AC-22
  and AC-26, and the retargeting noted in AC-8 and AC-21) are contingent on this decision and on
  the lock prerequisite. The alternative is to hold native divide and remainder until Contract
  Runtime's types are `quire_exact`'s, which leaves the native oracle with no division for that
  time. For the owner and the IR-349 owner to decide.
- **OD-3, the equality route (closed by measurement).** `quire_exact` exports `planned_equality`
  and no type environment or checked equality. For two integer operands, Contract Runtime's
  `CheckedEquality::evaluate` takes `EqualitySchedule::Plan`, calls its `planned_equality` and
  returns `equal == (operator == Equal)`, that is negated for not-equal, with no further charge
  (Contract Runtime `src/exact/equality.rs`). `quire_exact::planned_equality` over
  `Value::Integer` operands, negated for not-equal, is therefore the same value and the same
  charges.
- **OD-4, the Kani encoding of division.** The bundle takes fixed-width operators, with the
  measurements in "Two consumers, two shapes of the same rule". `quire_exact::Integer` is
  `num-bigint`-backed, and the call gave no CBMC verdict in 900 seconds. Contract Runtime's copy
  of the integer holds an `i64` inline whenever the value fits, and its source comment cites
  CBMC; `quire-exact`'s is a `BigInt` newtype. If QSL or IR want the bundle to prove the kernel's own division, the
  kernel needs a fixed-width path CBMC can fold, and that is QSL's decision. Not verified here:
  any divide under Kani through `quire_exact`.
- **OD-6, connective metering.** FR-031-AC-26 has the migrated oracle's connectives charge nothing,
  which preserves the behaviour of the Contract Runtime operators they replace.
  `quire_exact`'s documentation of `retain_boolean` says a caller's own short-circuit evaluation
  retains its decided result through that function, which charges `boolean.result-retain`. For QSL
  to decide whether native-oracle metering must match the kernel's, in which case the connectives
  charge that point and AC-26 and the unchanged-outcome clause change with it. This requirement
  does not pre-empt it.

## Out of Scope

- **Moving QSL's exemplar onto the V2 contract arm.** AD-004 step 4e and QSL's follow-up own it.
  Until then the exemplar runs through the V1 bundle and FR-031-AC-11 is its gate. Whether QSL
  makes the exemplar run a required gate on its side is an open question to QSL.
- **Numeric negation.** `interface-001` and the generator refuse it, and this requirement does
  not change that.
- **Rational, decimal, IEEE, quantity and text arithmetic.** The V1 Boolean oracle carries
  Boolean and bounded-integer values only.
- **A saturating integer operation.** Saturation has no runtime definition. The `saturate` refusal
  holds until Contract Runtime defines one.
- **A shared operator table for the emitters.** That is IR-597's work, and nothing here decides
  its shape.
- **The V2 inline clause oracle.** FR-014-AC-38 and FR-015-AC-40 and AC-41 own the `Outcome<bool>`
  oracle of a V2 clause body, which replaces this renderer's arithmetic at the V2 contract arm.
