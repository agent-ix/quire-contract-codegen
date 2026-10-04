---
id: FR-031
title: "Generate Boolean oracles whose integer arithmetic and comparison take the runtime's meaning"
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
  - target: ix://agent-ix/quire-contract-ir/FR-013
    type: references
  - target: ix://agent-ix/quire-contract-ir/FR-015
    type: references
---
# FR-031: Generate Boolean oracles whose integer arithmetic and comparison take the runtime's meaning

## Description

The Boolean oracle of `interface-001`'s `oracle_slice` lowers one validated Boolean clause over
Boolean and bounded-integer values into a Rust function. Its clauses may hold integer add,
subtract, multiply, divide and remainder, and the six integer comparisons. Today the emitter
renders each of those five operators as the raw Rust infix operator on `i64`, and each comparison
as the raw infix comparison. It never reads the operand type's overflow policy. On overflow the
generated function panics in a debug build and wraps in a release build, and on a zero divisor it
panics. Contract Runtime's exact integer operations, which are the language's meaning of the same
nodes, return a typed outcome instead.

A generated oracle that panics or wraps disagrees with the language it claims to evaluate, and a
Kani proof or a conformance campaign over it proves or tests a different operation. This
requirement fixes what the Boolean oracle emits and means for integer arithmetic and comparison,
so that the meaning of an operator is stated once, in `quire-contract-runtime`, and the emitter
restates none of it, other than in the V1 Kani bundle. There the fixed-width infix arithmetic is a
second implementation of the rule, kept because QSL's exemplar mutates its text, and held to the
runtime's meaning by a differential test (see "Two consumers, two shapes of the same rule").

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

The requirement is split because the runtime and the language disagree about division, and the
disagreement is not CG's to settle (see "Held: integer divide and remainder").

- **IR-596's closing code change** implements FR-031-AC-1, AC-3, AC-4, AC-5, AC-7, AC-8, AC-9,
  AC-10, AC-11, AC-12, AC-13, AC-14, AC-15, AC-18, AC-19, AC-20 and AC-21. Divide and remainder
  are refused at generation by FR-031-AC-18 until the ruling below, so that no oracle emits a raw
  `/` or `%`. The closing change shall run QSL's Kani exemplar (FR-031-AC-11) against its branch,
  which needs a checkout of the integration repository, and shall record the run.
- **Held for a follow-up ticket after the IR-601 ruling:** FR-031-AC-2, AC-6, AC-16 and AC-17.
  They state the divide and remainder rendering, the zero-divisor outcomes, the minimum divided
  by negative one outcomes and the divide and remainder differential. The ruling decides their
  final text. Their vectors and constructions are recorded now so the follow-up inherits them.

### Scope

The Boolean oracle here is the output of `generate_boolean_oracle` and of the named generator
that the V1 Kani bundle, the V1 bound-oracle generator, the tri-state harness generator and the
Kani obligation clause lowering share, all in `src/oracle/boolean_v1.rs`. It is retired with that
file at AD-004 step 6, with the other V1 readers; step 4f deletes the V1 bundle and does not
delete this renderer, because the Kani obligation arm, the V1 bound generator and the tri-state
harness generator still call it. The exact V2 oracles of FR-014, FR-018 and FR-021 already call
`quire_contract_runtime::exact` and are unchanged.

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
- `interface-001`'s `oracle_slice` states that no checked result is unwrapped or defaulted into
  `bool`, and ADR-003 states that a generated oracle calls `quire_contract_runtime::exact` so a
  proof covers the production code.

### Two consumers, two shapes of the same rule

The renderer serves two kinds of consumer, and they need different arithmetic.

- **The native oracle**: `generate_boolean_oracle`, `generate_bound_oracles` and anything else
  that a Rust program evaluates. A wrapped or panicking result is the defect there, so the
  arithmetic is a runtime `exact` call returning a typed outcome.
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

This leaves two implementations of the rule: the runtime's exact operation for the native oracle
and the fixed-width operator for the bundle oracle (the second implementation IR-594 flagged). It
is justified by the exemplar's text-mutation contract and AD-004 L-5, which a call into the runtime
would break, and it is held in step by a differential
test (FR-031-AC-21), not by a copy of the arithmetic in a table. The two differ in one
documented way: on an operand outside the declared domain the native oracle returns the
runtime's refusal, and the bundle oracle fails Kani's overflow check, because under Kani the
harness assumes the declared domain on every drawn input and asserts it on every produced value
before the oracle runs (`ensures` is `(result bounds) && oracle`).

### Held: integer divide and remainder

`exact::divide` computes the quotient and the remainder together and admits the pair: when either
member is outside the supplied domain it returns `Refusal::DivisionPairOutOfDomain` for both
(Contract Runtime FR-007, from QSpec FR-147). IR admits a divide or remainder after proving only
the selected member in range. The two disagree on expressions IR admits:

- `10 / y <= 10` over a `reject` type of `1..=10`, at `y = 5`: the quotient 2 is in range and the
  remainder 0 is not, so `exact::divide` refuses where the raw operator, and IR's proof, give 2.
- `x % -1` over a `reject` type of `-10..=5`, at `x = -10`: the remainder 0 is in range and the
  quotient 10 is not, so `exact::divide` refuses where the raw operator gives 0.

Rendering divide and remainder as `exact::divide` over the node's interval would therefore refuse
in-domain expressions the language accepts. No existing kernel offers a division that checks one
member and returns typed outcomes: `exact::divide` admits the pair, and
`operators::checked_div` and `operators::checked_rem` select one member but return `Option`, so a
zero divisor and an overflow are indistinguishable. This requirement does not choose between them.
The question is IR-601: whether a V1 division takes IR's member-only domain semantics or QSpec
FR-147's pair semantics, and which runtime operation provides it. Until the ruling the Boolean
oracle refuses divide and remainder (FR-031-AC-18) and states no rendering for them.

The truncating, floor and Euclidean laws agree on non-negative operands and differ on negative
ones. The IR divide and remainder nodes name no law and the Boolean oracle has no package to
select one from. The follow-up takes the truncating law, which is what the emitted `/` and `%`
mean on every in-domain operand today, unless the ruling says otherwise.

## Inputs

- A validated typed Boolean expression, as `OracleRequest` carries it, whose nodes are within
  `interface-001`'s supported grammar plus integer add, subtract and multiply.
- For each arithmetic node, the checked `IntegerType` that types its operands and its result:
  minimum, maximum, signedness and overflow policy.

## Outputs

- A generated Rust function and source map, as before, or a typed refusal with no partial
  artifact.
- For an expression with no arithmetic node, the same `bool`-returning function as before.
- For an expression with at least one arithmetic node, a native oracle is a function that returns
  `quire_contract_runtime::exact::Outcome<bool>` and takes a trailing `&mut Meter`, as the
  FR-014 oracles do. A Kani bundle oracle is a function that returns `bool`, as before.

## Behavior

### Arithmetic in the native oracle

- The generator shall render integer add, subtract and multiply over a `reject` type as a call of
  `exact::evaluate_integer_arithmetic` with the matching `IntegerArithmetic` variant and the
  node's `[minimum, maximum]` interval as the result bound.
- The generator shall emit no Rust arithmetic operator between two operand expressions and no
  integer method such as `checked_add`, `wrapping_add`, `saturating_add` or `overflowing_add` for
  these operators. Every arithmetic operation in the emitted body shall be an `exact::` call, and
  no charge amount shall appear in the source.
- The generated oracle shall take the operands of an arithmetic call as the exact integers of the
  operand expressions, converted from the function's `i64` parameters and literals, and an operand
  that is itself an arithmetic node shall contribute its completed integer.

| `NumericOperator` | Emitted evaluation | Outcome when the result is outside the interval |
|---|---|---|
| Add | `evaluate_integer_arithmetic(Add(l, r), Some(interval), meter)` | `Refused(IntegerOutOfDomain)` |
| Subtract | `evaluate_integer_arithmetic(Subtract(l, r), Some(interval), meter)` | `Refused(IntegerOutOfDomain)` |
| Multiply | `evaluate_integer_arithmetic(Multiply(l, r), Some(interval), meter)` | `Refused(IntegerOutOfDomain)` |

### Arithmetic in the Kani bundle oracle

- The generator shall render integer add, subtract and multiply over a `reject` type, in the
  oracle that `generate_kani_bundle` embeds, as the Rust infix operators `+`, `-` and `*` on the
  two `i64` operand expressions, in the parenthesized text shape the generator emits today, with
  one binary operator per arithmetic node. It shall not render them as an `exact::` call, a
  `checked_`, `wrapping_`, `saturating_` or `overflowing_` method, or any helper call, so that
  the one `+` of the exemplar clause stays one syntactic addition whose text can be extended.
- The bundle oracle shall return `bool`, shall take no meter, and shall contain no `unwrap`,
  `expect` or panic macro. Under Kani its arithmetic carries Kani's own overflow check, whose
  failure is a falsifiable property of the proof.
- The generated file of a bundle that holds an arithmetic node shall state in a comment that its
  arithmetic is checked by Kani and that the file is not a native evaluator. The bundle is
  consumed only by `cargo kani`.

### Interim refusals

- If the expression holds a divide or remainder node, whatever its overflow policy or consumer, then the
  generator shall refuse it with the code `UnsupportedIntegerDivision` at that node's source span,
  as the first unsupported node in authored preorder, and emit no artifact. The diagnostic's
  message shall name IR-601 as the open ruling, so that the refusal is not read as a final
  answer. The generator shall not emit a raw `/` or `%` and shall not render the node through
  `exact::divide` until that ruling.
- If an add, subtract or multiply node's integer type has overflow policy `saturate`, in either consumer, then the
  generator shall refuse the expression with the code `UnsupportedSaturatingArithmetic` at that
  node's source span, as the first unsupported node in authored preorder, and emit no artifact.
  The diagnostic's message shall name the missing Contract Runtime saturating integer operation.
  The generator shall not restate saturation inline: clamping a result to the interval would be a
  second implementation of the rule. A `saturate` integer that is only compared, never operated
  on, is unaffected.
- Both codes shall have terminal state `unsupported`, as `UnsupportedExpression` does, and their
  message text shall not be used as machine identity.

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
| Equal | `l == r` on the two `i64` values | `TypeEnvironment::check_equality` with `EqualityOperator::Equal`, then `CheckedEquality::evaluate`, over the two exact integers, as FR-018 does |
| NotEqual | `l != r` on the two `i64` values | the same with `EqualityOperator::NotEqual` |
| Less | `l < r` | `exact::order_numbers` with `OrderingOperator::Less` over `Integers` |
| LessEqual | `l <= r` | `exact::order_numbers` with `OrderingOperator::LessOrEqual` |
| Greater | `l > r` | `exact::order_numbers` with `OrderingOperator::Greater` |
| GreaterEqual | `l >= r` | `exact::order_numbers` with `OrderingOperator::GreaterOrEqual` |

- The generator shall keep the native operator and the `bool` result for a comparison of two
  arithmetic-free `i64` values, which has no failure mode. That comparison is held to the
  runtime's meaning by a differential test (FR-031-AC-4), not by a call.
- The generator, other than for the Kani bundle oracle, shall not use a Rust comparison operator
  between the two operands of a comparison that has an arithmetic operand. Its outcome is the runtime's, and an `Outcome::Completed`
  boolean continues as above.

### Consumers

- A generator other than the V1 Kani bundle that embeds the Boolean oracle where a plain `bool` is
  required shall refuse a clause that holds an arithmetic node, shall embed no raw arithmetic operator, and shall not read
  an `Outcome<bool>` as a `bool`. The tri-state harness generator shall refuse a clause whose
  first arithmetic node is an add, subtract or multiply over a `reject` type with
  `UnsupportedExpression` at that node, in addition to the `UnsupportedDependency` it already
  returns for an integer dependency; a divide, remainder or `saturate` node is refused by the
  interim refusals above, with `UnsupportedIntegerDivision` or
  `UnsupportedSaturatingArithmetic`, for every consumer. Bound oracle generation is
  all-or-nothing over a package, so one clause the interim refusals refuse refuses the bound
  strategy for every clause of that package, and the `UnsupportedClause` names the refused
  clause, not the requested one. The bound strategy
  generators shall keep the `UnsupportedRelation` they return for a clause whose oracle generates,
  such as a discharged add used as a comparison operand, and the `UnsupportedClause` carrying the
  oracle's code and span for a clause the oracle refuses, which includes every clause refused by
  the interim refusals above. The Kani obligation clause lowering shall keep its definedness
  refusal for a clause that carries an obligation, which is every `reject` arithmetic node, and
  shall take the oracle's refusal otherwise.
- The V1 Kani bundle (`generate_kani_bundle`) is a consumer that needs a plain `bool`, because it
  places the oracle in a `requires` and an `ensures` clause. It shall use the Kani bundle oracle
  above for add, subtract and multiply over a `reject` type, and it shall refuse a clause that
  holds a divide, remainder or `saturate` arithmetic node with `ClauseGenerationFailed`,
  retaining the oracle's own code (`UnsupportedIntegerDivision` or
  `UnsupportedSaturatingArithmetic`) and the node's span, and emit no harness. It shall not carry
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
  remainder cases of FR-031-AC-18 shall each exist as a test that fails on the tree before the
  closing code change, and that change shall record the failing run of each, before it records
  the passing run. A test that passes on the tree before the change does not establish the defect
  and does not count toward these criteria. The same holds for the held cases of FR-031-AC-6 and
  AC-16 when the follow-up lands.
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
| FR-031-AC-2 | HELD (IR-601; follow-up ticket). Divide and Remainder over a `reject` integer type are rendered as the runtime operation the IR-601 ruling names, with the domain semantics it names, with no Rust `/` or `%`. The criterion's final text is written when the ruling lands. | Test (TC-044) |
| FR-031-AC-3 | IMPLEMENTED (IR-596). A native oracle with an arithmetic node returns `Outcome<bool>` and takes a trailing `&mut Meter`; an oracle with none returns `bool` and takes no meter. | Test (TC-044) |
| FR-031-AC-4 | IMPLEMENTED (IR-596). Each of the six comparisons over two arithmetic-free operands is emitted as the one native operator the table names, and over the seven values `i64::MIN`, `i64::MIN + 1`, `-1`, `0`, `1`, `i64::MAX - 1` and `i64::MAX`, with two interior values added, taken pairwise in both orders, agrees with the runtime (`exact::order_numbers` for the four orderings, `check_equality` then `CheckedEquality::evaluate` for equal and not-equal). This criterion passes on the tree before the change and is a held agreement, not a defect proof. | Test (TC-044) |
| FR-031-AC-5 | IMPLEMENTED (IR-596). Under a `reject` integer type, an add, subtract or multiply result outside the type's interval evaluates to `Refused(IntegerOutOfDomain)`, and the oracle neither panics nor returns a wrapped, saturated or clamped value. The cases are O-1 `i64::MAX + 1`, O-2 `i64::MIN - 1` as a subtract, and O-4 `i64::MIN * 2`, each built as TC-044 step 4 gives. On the tree before the change each fails: a debug build panics with an overflow message, a release build returns a wrapped value, and no case returns the refusal. | Test (TC-044) |
| FR-031-AC-6 | HELD (IR-601; follow-up ticket). Under a `reject` integer type, a zero divisor evaluates to the runtime's undefined outcome for both Divide and Remainder, for the cases Z-1 `x / 0`, Z-2 `0 / 0`, Z-3 `x % 0` and Z-4 `0 % 0`, each built as TC-044 step 5 gives, and the oracle does not panic. On the tree before the change each case panics with a division-by-zero message. | Test (TC-044) |
| FR-031-AC-7 | IMPLEMENTED (IR-596). An add, subtract or multiply over a `saturate` integer type is refused with `UnsupportedSaturatingArithmetic` at the node's span, with no artifact, for the rows `x + 1` at `x = i64::MAX` and `x - 1` at `x = i64::MIN` over an integer type of `i64::MIN..=i64::MAX`, and `x * 2` at `x = i64::MAX` over `0..=10` (where an in-domain `x = 10` also gives the wrong value `20 <= 10` against the policy's `10 <= 10`); the refusal span is the first such node in authored preorder when a negation also occurs later, and the diagnostic message names the missing runtime saturating operation. On the tree before the change each expression generates, and the generated function panics in a debug build on the named operands. | Test (TC-044) |
| FR-031-AC-8 | IMPLEMENTED (IR-596). In a generated crate compiled and executed against `quire-contract-runtime`, the outcome of a native add, subtract or multiply oracle equals a plain-integer model of IR FR-015's member-range semantics, computed in the test with `i128` independently of the emitter and of any `exact::` call: `Completed` with the exact result when it lies in the type's interval and `Refused(IntegerOutOfDomain)` when it does not. The grid covers operand values inside the declared domain, at its edges and outside it, and the outcome kind is compared, so no vector returns `Completed` where the model refuses. | Test (TC-044) |
| FR-031-AC-9 | IMPLEMENTED (IR-596). The first non-completed outcome in evaluation order is the oracle's outcome: an arithmetic stop in the left operand of a short-circuit connective is returned, a stop in a right operand the left decided is never reached, a total connective returns the first stop of its two operands, and a stop is never returned as `Completed(true)` or `Completed(false)`. | Test (TC-044) |
| FR-031-AC-10 | IMPLEMENTED (IR-596). The tri-state harness generator, the bound strategy generators and the Kani obligation clause lowering each refuse a clause holding an arithmetic node with the refusal the Consumers section names for it, emit no raw arithmetic operator, and never read an `Outcome<bool>` as `bool`. | Test (TC-044) |
| FR-031-AC-11 | IMPLEMENTED IN CG (IR-596); THE QSL EXEMPLAR RUN IS PENDING. The clause that runs QSL's IT-011 and IT-010-SC-05 against this branch is pending the quire-integration run; CG's in-repo run of the same bundle shape under real Kani is the closest equivalent and does not stand for it. `generate_kani_bundle` over the postcondition `amount < 1000` implies `amount + 1 <= 1000`, with `amount` an integer of `0..=1000` under `reject`, generates a bundle whose postcondition oracle returns `bool`, holds exactly one syntactic addition (`syn::ExprBinary` with `BinOp::Add`) and no `exact::` call, and, run with real `cargo kani` as QSL's IT-011 and IT-010-SC-05 run it against this branch, verifies, while the mutant that appends ` + 1_i64` to the addition (`left + right + 1`) is falsified with the counterexample `amount_current = 999`, and the equivalent mutant that drops the `+ 1` verifies; over a divide, a remainder or a `saturate` node the bundle returns `ClauseGenerationFailed` retaining the oracle's own code and emits no harness. | Test (TC-044), Test (quire-integration IT-011) |
| FR-031-AC-12 | IMPLEMENTED (IR-596). The oracle generated for an expression with no arithmetic node is byte-identical to the oracle the generator produced for it before this requirement. This criterion passes on the tree before the change and is a held agreement, not a defect proof. | Test (TC-044) |
| FR-031-AC-13 | IMPLEMENTED (IR-596). Generating the oracle of an expression with an arithmetic node twice, and from a permuted request, yields identical bytes. This criterion passes on the tree before the change and is a held agreement, not a defect proof. | Test (TC-044) |
| FR-031-AC-14 | IMPLEMENTED (IR-596). The generated source of an oracle with an arithmetic node contains no `unwrap`, `expect` or panic macro. This criterion passes on the tree before the change and is a held agreement, not a defect proof. | Test (TC-044) |
| FR-031-AC-15 | IMPLEMENTED (IR-596). In the native oracle, each of the six comparisons that has an arithmetic operand is emitted as the runtime call the table names, with no Rust comparison operator between its two operands. | Test (TC-044) |
| FR-031-AC-16 | HELD (IR-601; follow-up ticket). Under a `reject` integer type, a divide or remainder of the minimum by negative one evaluates to the runtime outcome the IR-601 ruling names, and does not panic, for O-3 `i64::MIN / -1` and O-5 `i64::MIN % -1`, each built as TC-044 step 4 gives. On the tree before the change each case panics with an overflow message. | Test (TC-044) |
| FR-031-AC-17 | HELD (IR-601; follow-up ticket). In a generated crate, the outcome of a divide or remainder oracle equals a plain-integer model of the semantics the IR-601 ruling names, computed independently of the emitter, on a grid that includes the two reproductions of the Held section: `10 / y <= 10` over a `reject` type of `1..=10` at `y = 5`, and `x % -1` over `-10..=5` at `x = -10`. Under member-only semantics both complete, with 2 and 0; the reproductions are the vectors that distinguish the member-only from the pair semantics. | Test (TC-044) |
| FR-031-AC-18 | IMPLEMENTED (IR-596). An expression holding a divide or a remainder node, over a `reject` or a `saturate` integer type, is refused with `UnsupportedIntegerDivision` at the node's span, with no artifact and no raw `/` or `%` in any output, for each of the constructions O-3, O-5, Z-1, Z-2, Z-3 and Z-4 of TC-044 and the `saturate` rows S-1 to S-4 of TC-044 step 11, in both consumers; the diagnostic message names IR-601 and the refusal has terminal state `unsupported`. On the tree before the change each construction generates and its generated function panics in a debug build on the vector TC-044 names for its row. | Test (TC-044) |
| FR-031-AC-19 | IMPLEMENTED (IR-596). The Kani bundle oracle renders add, subtract and multiply as the infix `+`, `-` and `*` on `i64` with one binary operator per arithmetic node, contains no `exact::` call and no `checked_`, `wrapping_`, `saturating_` or `overflowing_` method, takes no meter, and its file states that its arithmetic is checked by Kani and is not a native evaluator. | Test (TC-044) |
| FR-031-AC-20 | IMPLEMENTED (IR-596). Under real Kani, a harness that calls a bundle oracle with unconstrained operands at which IR's discharge does not hold fails with Kani's arithmetic-overflow check and a counterexample: the oracle of `x < 5 && x * 2 <= 10` over `0..=10` under `reject`, called with `x` unconstrained, fails the "attempt to multiply with overflow" check, and the counterexample's playback value satisfies `x < 5` and overflows `x * 2` (any such value is valid; the assertion does not name one); a hand-written probe harness that assumes the declared domain on `x`, calls the same bundle oracle and discards its value has no failing arithmetic-overflow check, and verifies. (The bundle's own `proof_for_contract` harness is not that probe: its `ensures` asserts the clause, which is false for `x` in `5..=10`.) Overflow in the bundle oracle is a falsifiable property, never a wrapped value. | Test (TC-044) |
| FR-031-AC-21 | IMPLEMENTED (IR-596). For each of add, subtract and multiply, the bool the Kani bundle oracle returns equals the `bool` inside `Completed` of the native oracle of the same typed expression, on every vector of a grid inside the declared domain at which the expression's guards pass and IR's discharge holds, including the domain's edges, with the bundle oracle compiled and run natively in a generated crate against `quire-contract-runtime` in plain `cargo test`, with no Kani. This is the differential that holds the two implementations of the rule in step. | Test (TC-044) |

## Dependencies

- **Upstream**: `interface-001` (`oracle_slice`), [FR-014](./FR-014-exact-scalar-oracles.md) for the
  meter and `Outcome` convention of an exact oracle, [FR-018](./FR-018-composite-equality-oracles.md)
  for the equality evaluation, [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md)
  and [FR-028](../../kani/functional/FR-028-bounded-proof-ceilings.md) for the Kani bundle and its
  ceilings, [FR-008](../../strategy/functional/FR-008-bound-domain-strategy-admission.md) for the
  strategy admission, Contract IR FR-013 and FR-015, and Contract Runtime FR-002, FR-006 and
  FR-007.
- **Downstream**: [TC-044](../matrix/TC-044-boolean-oracle-integer-arithmetic.md).

## Out of Scope

- **The IR-601 ruling.** Which domain semantics a V1 division uses and which runtime operation
  provides it are decided by QSL, QSpec, IR and the runtime. FR-031-AC-2, AC-6, AC-16 and AC-17
  wait on it.
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
