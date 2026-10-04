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
restates none of it.

The defect is reachable in two ways. Contract IR discharges the definedness obligation of an
arithmetic node from declared bounds and dominating guards, so an operand inside its declared
domain cannot overflow or divide by zero in an expression IR admitted. But the generated function
is `pub` and takes any `i64`, and FR-015 and `interface-001`'s `kani_slice` assert a
subject-produced value's domain rather than assuming it, so the oracle is evaluated at operand
values the declared domain excludes. Add, subtract and multiply over an integer type whose
overflow policy is `saturate` raise no obligation at all, so that arithmetic is admitted for every
operand value, and the raw operator is wrong on in-domain values too: over `0..=10` it computes
`10 + 1` as `11`, where the policy requires the result to stay in range.

### Scope

The Boolean oracle here is the output of `generate_boolean_oracle` and of the named generator
that the V1 Kani bundle, the V1 bound-oracle generator, the tri-state harness generator and the
Kani obligation clause lowering share, all in `src/oracle/boolean_v1.rs`. It is retired with that
file at AD-004 step 6, with the other V1 readers; step 4f deletes the V1 bundle and does not
delete this renderer, because the Kani obligation arm, the V1 bound generator and the tri-state
harness generator still call it. The exact V2 oracles of FR-014, FR-018 and FR-021 already call
`quire_contract_runtime::exact` and are unchanged.

This requirement states what the oracle emits and what it means. Whether the emitters of other
generators spell the same operators once, in a shared table, is a separate piece of work and is
not specified here.

### Basis

- Contract IR FR-013 declares each integer with an inclusive minimum and maximum and an overflow
  policy of `reject` or `saturate`. IR FR-015 makes saturating add, subtract, multiply and negate
  total, raises a non-zero-divisor obligation for every divide and remainder under both policies,
  and requires a range proof for `reject` arithmetic, including the minimum divided by negative
  one.
- Contract Runtime FR-006 and FR-007 give an exact integer operation one of four outcomes:
  completed, undefined, refused or incomplete. A zero divisor is `Undefined::DivisionByZero`. A
  result outside the supplied bound is `Refusal::IntegerOutOfDomain`. A quotient or remainder
  outside the supplied domain is `Refusal::DivisionPairOutOfDomain`. A false Boolean is a
  completed value, never a refusal. The runtime has no saturating integer operation.
- `interface-001`'s `oracle_slice` states that no checked result is unwrapped or defaulted into
  `bool`, and ADR-003 states that a generated oracle calls `quire_contract_runtime::exact` so a
  proof covers the production code.
- The truncating, floor and Euclidean laws agree on non-negative operands and differ on negative
  ones. The IR divide and remainder nodes name no law, and the Boolean oracle has no package to
  select one from. It takes the truncating law, which is what its emitted `/` and `%` mean on
  every in-domain operand today.

## Inputs

- A validated typed Boolean expression, as `OracleRequest` carries it, whose nodes are within
  `interface-001`'s supported grammar plus integer add, subtract, multiply, divide and remainder.
- For each arithmetic node, the checked `IntegerType` that types its operands and its result:
  minimum, maximum, signedness and overflow policy.

## Outputs

- A generated Rust function and source map, as before, or a typed refusal with no partial
  artifact.
- For an expression with no arithmetic node, the same `bool`-returning function as today.
- For an expression with at least one arithmetic node, a function that returns
  `quire_contract_runtime::exact::Outcome<bool>` and takes a trailing `&mut Meter`, as the
  FR-014 oracles do.

## Behavior

### Arithmetic

- The generator shall render integer add, subtract and multiply over a `reject` type as a call of
  `exact::evaluate_integer_arithmetic` with the matching `IntegerArithmetic` variant and the
  node's `[minimum, maximum]` interval as the result bound.
- The generator shall render integer divide and remainder over a `reject` type as a call of
  `exact::divide` with `DivisionProfile::Truncating` and the node's interval as a bounded
  domain. Divide takes the quotient of the pair and remainder takes its remainder. The generator
  shall not render either through `exact::modulo`, whose remainder is Euclidean.
- The generator shall emit no Rust arithmetic operator between two operand expressions and no
  integer method such as `checked_add`, `wrapping_add`, `saturating_add` or `overflowing_add` for
  any of the five operators. Every arithmetic operation in the emitted body is an `exact::` call,
  and no charge amount appears in the source.
- The operands of an arithmetic call are the exact integers of the operand expressions, converted
  from the function's `i64` parameters and literals. An operand that is itself an arithmetic node
  contributes its completed integer.

| `NumericOperator` | Emitted evaluation | Operand-independent outcomes |
|---|---|---|
| Add | `evaluate_integer_arithmetic(Add(l, r), Some(interval), meter)` | `Refused(IntegerOutOfDomain)` when the sum is outside the interval |
| Subtract | `evaluate_integer_arithmetic(Subtract(l, r), Some(interval), meter)` | `Refused(IntegerOutOfDomain)` when the difference is outside the interval |
| Multiply | `evaluate_integer_arithmetic(Multiply(l, r), Some(interval), meter)` | `Refused(IntegerOutOfDomain)` when the product is outside the interval |
| Divide | quotient of `divide(Truncating, l, r, Bounded(interval), meter)` | `Undefined(DivisionByZero)` for a zero divisor; `Refused(DivisionPairOutOfDomain)` when the quotient or the remainder is outside the interval, which includes the minimum divided by negative one |
| Remainder | remainder of `divide(Truncating, l, r, Bounded(interval), meter)` | the same two outcomes as Divide |

### Overflow policy

- If an arithmetic node's integer type has overflow policy `saturate`, then the generator shall
  refuse the expression with `UnsupportedExpression` at that node's source span, as the first
  unsupported node in authored preorder, and emit no artifact. The runtime has no saturating
  integer operation, and the generator shall not restate saturation inline: clamping a result to
  the interval would be a second implementation of the rule. A `saturate` integer that is only
  compared, never operated on, is unaffected.

### Outcomes and their propagation

- A comparison that evaluates to false is `Completed(false)`, and one that evaluates to true is
  `Completed(true)`. Neither is a refusal.
- The outcome of the oracle is the first outcome that is not completed, in evaluation order:
  left operand before right operand, and the operands of a comparison before the comparison. The
  oracle shall not unwrap, default, ignore or convert such an outcome to `true` or `false`, and its
  source shall contain no `unwrap`, `expect` or panic macro.
- A short-circuit connective evaluates its right operand only when its left operand does not
  decide the result, so a stop its right operand could produce does not arise when the left
  decides. A total connective evaluates both operands from left to right and its outcome is the
  first stop.

### Comparison

| `ComparisonOperator` | Both operands arithmetic-free | An operand holds arithmetic |
|---|---|---|
| Equal | `l == r` on the two `i64` values | the runtime's equality evaluation over the two exact integers |
| NotEqual | `l != r` on the two `i64` values | the same evaluation with the not-equal operator |
| Less | `l < r` | `exact::order_numbers` with `OrderingOperator::Less` over `Integers` |
| LessEqual | `l <= r` | `exact::order_numbers` with `OrderingOperator::LessOrEqual` |
| Greater | `l > r` | `exact::order_numbers` with `OrderingOperator::Greater` |
| GreaterEqual | `l >= r` | `exact::order_numbers` with `OrderingOperator::GreaterOrEqual` |

- A comparison of two `i64` values has no failure mode, so an arithmetic-free comparison keeps the
  native operator and keeps its `bool` result. It is held to the runtime's meaning by a
  differential test (FR-031-AC-4), not by a call.
- A comparison with an arithmetic operand shall not use a Rust comparison operator between the
  two operands. Its outcome is the runtime's, and an `Outcome::Completed` boolean continues as
  above.

### Consumers

- A generator that embeds the Boolean oracle where a plain `bool` is required shall refuse a
  clause that holds an arithmetic node, shall embed no raw arithmetic operator, and shall not read
  an `Outcome<bool>` as a `bool`. The tri-state harness generator refuses with
  `UnsupportedExpression` at the first arithmetic node, in addition to the
  `UnsupportedDependency` it already returns for an integer dependency. The bound strategy
  generators keep the `UnsupportedRelation` they return today for a clause whose oracle generates,
  such as a guarded division, and the `UnsupportedClause` carrying the oracle's code and span for
  a clause the oracle refuses, such as a `saturate` one. The Kani obligation clause lowering keeps
  its definedness refusal for a clause that carries an obligation, which is every `reject`
  arithmetic node, and takes the oracle's refusal otherwise. FR-008-AC-3's refusals of negation
  and of an oracle-refused clause are unchanged.
- The V1 Kani bundle shall carry the outcome. A postcondition is asserted as `Completed(true)`. A
  precondition is assumed only where its oracle is `Completed(true)`, and an input for which it is
  `Completed(false)` is excluded as before. A precondition or postcondition oracle that is
  `Undefined`, `Refused` or `Incomplete` is a failed obligation, never an assumption, never
  satisfied and never silently excluded. The bundle supplies the meter as the FR-015 scalar
  harness does, and its ceilings are FR-028's: a run that exhausts a ceiling is inconclusive and
  never verified.

### Test obligations

- The overflow and zero-divisor cases of FR-031-AC-5 and FR-031-AC-6 and the `saturate` cases of
  FR-031-AC-7 shall each exist as a test that fails on the tree before this requirement is
  implemented, and the code change shall record that failing run for each, before it records the
  passing run. A test that passes on the tree before the change does not establish the defect and
  does not count toward these criteria.
- Operands the declared domain excludes are the only way to reach an overflow or a zero divisor
  through an expression Contract IR admits under `reject`. The test constructs each expression
  through `DeclarationEnvironment::check_expression`, so IR admits it, and chooses declarations
  and guards so that the proof of safety rests on a declared bound that the operand vector
  violates. A vector that no admitted `reject` expression can reach is exercised with the
  nearest reachable vector of the same operation, and the code change names the substitution.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-031-AC-1 | PLANNED (IR-596). For each of Add, Subtract and Multiply over a `reject` integer type, the generated source calls `exact::evaluate_integer_arithmetic` with the matching `IntegerArithmetic` variant and the type's interval as the bound, and the oracle's body holds no Rust arithmetic operator between operand expressions and none of `checked_`, `wrapping_`, `saturating_` or `overflowing_` method calls. | Test (TC-044) |
| FR-031-AC-2 | PLANNED (IR-596). For each of Divide and Remainder over a `reject` integer type, the generated source calls `exact::divide` with `DivisionProfile::Truncating` and the type's interval as a bounded domain, reads the quotient for Divide and the remainder for Remainder, and calls neither `exact::modulo` nor a Rust `/` or `%`. | Test (TC-044) |
| FR-031-AC-3 | PLANNED (IR-596). An oracle with an arithmetic node returns `Outcome<bool>` and takes a trailing `&mut Meter`; an oracle with none returns `bool`, takes no meter and is byte-identical to its output before this requirement; both regenerate byte-identically across repeated runs; and no generated oracle source contains `unwrap`, `expect` or a panic macro. | Test (TC-044) |
| FR-031-AC-4 | PLANNED (IR-596). Each of the six comparisons over two arithmetic-free operands is emitted as the one native operator the table names, and over the seven values `i64::MIN`, `i64::MIN + 1`, `-1`, `0`, `1`, `i64::MAX - 1` and `i64::MAX`, with two interior values added, taken pairwise in both orders, agrees with the runtime (`exact::order_numbers` for the four orderings, the runtime's equality evaluation for equal and not-equal). Each of the six comparisons with an arithmetic operand is emitted as the runtime call the table names and has no Rust comparison operator between its operands. This criterion's native half passes before the change and is a held agreement, not a defect proof. | Test (TC-044) |
| FR-031-AC-5 | PLANNED (IR-596). Under a `reject` integer type, an arithmetic result outside the type's interval evaluates to `Refused(IntegerOutOfDomain)`, and a quotient or remainder outside it to `Refused(DivisionPairOutOfDomain)`, and the oracle neither panics nor returns a wrapped, saturated or clamped value. The cases are `i64::MAX + 1`, `i64::MIN - 1` and `i64::MIN / -1`, each with its operands outside the declared domain, a multiply whose product leaves `i64` (`i64::MIN * 2`), and `i64::MIN % -1`. On the tree before the change each fails: a debug build panics with an overflow message for every case, a release build wraps the add, subtract and multiply cases and still panics for the divide and remainder cases, and no case returns the refusal. | Test (TC-044) |
| FR-031-AC-6 | PLANNED (IR-596). Under a `reject` integer type, a zero divisor evaluates to `Undefined(DivisionByZero)` for both Divide (`x / 0`) and Remainder (`x % 0`), for a non-zero `x` and for `x = 0`, and the oracle does not panic. On the tree before the change each case panics with a division-by-zero message. | Test (TC-044) |
| FR-031-AC-7 | PLANNED (IR-596). An expression with an arithmetic node over a `saturate` integer type is refused with `UnsupportedExpression` at that node's span, with no artifact, for each of Add, Subtract, Multiply, Divide and Remainder, including the `saturate` form of `i64::MAX + 1`, `i64::MIN - 1`, `i64::MIN / -1`, `x / 0` and `x % 0` over an integer type of `i64::MIN..=i64::MAX` and over a type whose declaration excludes zero; the refusal span is the first arithmetic node in authored preorder when a negation also occurs later. On the tree before the change each expression generates, and the generated function panics on the named operands. | Test (TC-044) |
| FR-031-AC-8 | PLANNED (IR-596). In a generated crate compiled and executed against `quire-contract-runtime`, an arithmetic-bearing oracle's outcome equals the outcome of the direct runtime evaluation of the same expression, on every vector of a grid that covers both the declared domain and operand values it excludes, including the outcome kind, so no vector returns `Completed` where the runtime refuses or is undefined. | Test (TC-044) |
| FR-031-AC-9 | PLANNED (IR-596). The first non-completed outcome in evaluation order is the oracle's outcome: an arithmetic stop in the left operand of a short-circuit connective is returned, a stop in a right operand the left decided is never reached, a total connective returns the first stop of its two operands, and a stop is never returned as `Completed(true)` or `Completed(false)`. | Test (TC-044) |
| FR-031-AC-10 | PLANNED (IR-596). A clause with an arithmetic node is refused by the tri-state harness generator, by the bound strategy generators and by the Kani obligation clause lowering, with the typed refusal each already returns for an unsupported clause, and none of them emits a raw arithmetic operator or reads an `Outcome<bool>` as `bool`; FR-008-AC-3 is unchanged. | Test (TC-044) |
| FR-031-AC-11 | PLANNED (IR-596). The V1 Kani bundle over the postcondition `amount < 1000` implies `amount + 1 <= 1000` with `amount` an integer of `0..=1000` under `reject` generates; its harness asserts `Completed(true)` for the postcondition and holds no `unwrap`, `expect` or raw arithmetic operator in the oracle; with the installed backend the unmutated clause verifies; and a subject that produces a post-state outside its declared domain fails the obligation with the runtime's refusal visible in the generated assertion, not with an arithmetic-overflow property of the oracle. | Test (TC-044) |

## Dependencies

- **Upstream**: `interface-001` (`oracle_slice`), [FR-014](./FR-014-exact-scalar-oracles.md) for the
  meter and `Outcome` convention of an exact oracle, [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md)
  and [FR-028](../../kani/functional/FR-028-bounded-proof-ceilings.md) for the Kani bundle and its
  ceilings, [FR-008](../../strategy/functional/FR-008-bound-domain-strategy-admission.md) for the
  strategy admission it leaves unchanged, Contract IR FR-013 and FR-015, and Contract Runtime
  FR-002, FR-006 and FR-007.
- **Downstream**: [TC-044](../matrix/TC-044-boolean-oracle-integer-arithmetic.md).

## Out of Scope

- **Numeric negation.** `interface-001` and the generator refuse it, and this requirement does
  not change that.
- **Rational, decimal, IEEE, quantity and text arithmetic.** The V1 Boolean oracle carries
  Boolean and bounded-integer values only.
- **A saturating integer operation.** Saturation has no runtime definition. The `saturate` refusal
  above holds until Contract Runtime defines one.
- **A shared operator table for the emitters.** That is IR-597's work, and nothing here decides
  its shape.
- **The V2 inline clause oracle.** FR-014-AC-38 and FR-015-AC-40 and AC-41 own the `Outcome<bool>`
  oracle of a V2 clause body, which replaces this renderer's arithmetic at the V2 contract arm.
