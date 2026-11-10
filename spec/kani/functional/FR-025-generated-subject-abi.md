---
id: FR-025
title: "Bind each generated harness argument to its parameter, domain and Rust type"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: depends_on
  - target: ix://agent-ix/quire-specification/AD-016
    type: references
  - target: ix://agent-ix/quire-spec-language/ADR-013
    type: references
  - target: ix://agent-ix/quire-contract-codegen/ADR-004
    type: references
---
# FR-025: Bind each generated harness argument to its parameter, domain and Rust type

## Description

When the code generator emits an [FR-015](./FR-015-bounded-kani-obligations.md) harness, it shall
emit the harness's symbolic bindings by expanding the obligation identity's logical
`arguments`, which ascend by parameter identifier, according to each family's row below.
Each logical argument names its original parameter node id and declared domain. Each physical
binding has the Rust type and component role its row states, and reconstruction reaches the
complete-V1 oracle without loss. Boolean and bounded-integer arguments retain their existing
one-argument/one-binding order; the planned bounded-Rational row has two ordered bindings for
one logical argument.

The scalar V2 postcondition result is separate from those drawn arguments. Until IR-948
implements the produced-result ABI below, the scalar V2 clause path refuses a postcondition
whose admitted operation declares a result (IR-1289). The existing collection postcondition
result path is outside that temporary scalar refusal.

The ascending order, the binding to a parameter node id and declared domain, and lossless widening
are what AD-016 arrow 5 and QSL ADR-013 O-25 decide. The Rust-type table, the by-reference state
argument and the frame disposition are
[ADR-004](../../decisions/ADR-004-generated-subject-abi-open-decisions.md)'s decisions. A family
gets a row in the table only when its witness decodes into the complete-V1 value type without loss.

## Inputs

- The obligation's argument bindings and their IR `bounded_domain` bounds (FR-015).
- The parameter node id of each argument in the admitted package.
- For the planned scalar V2 postcondition result path, the admitted operation anchor and its
  effective signature, including the optional result and ordered parameters (QSpec FR-341 and
  FR-342).

## Outputs

- A harness whose `kani::any()` calls equal its persisted physical binding order, obtained by
  row-specific expansion of the identity's ascending logical arguments.
- Argument bindings that each carry their parameter node id and declared domain.
- For the planned scalar V2 postcondition result path, a subject call whose produced Rust result
  has the declared supported type and whose domain is checked after the call.

## Behavior

- The generator shall order an identity's logical `arguments` ascending by parameter identifier.
- When emitting an argument, the generator shall emit the row's ordered primitive calls in that
  argument's position: one call for Boolean or bounded integer; numerator then denominator for
  the planned bounded-Rational row. Its persisted physical schema has exactly that call order.
- Each argument binding shall name the node id of the parameter it binds and that parameter's
  declared domain.
- If a binding names no parameter of the selected function, or a parameter of that function has no
  binding, then the generator shall refuse the obligation with a typed reason and emit no harness.
- The generator shall bind each argument by the Rust type its family's row states:

  | Family | Symbolic argument type | Value the oracle reads |
  |---|---|---|
  | Boolean | `bool` | the same `bool` |
  | Bounded integer | `i64`, assumed inside its declared domain | `rt::Integer::from(i64)`, passed to the oracle by reference |
  | Bounded Rational (planned [FR-046](./FR-046-rational-function-predicate-harnesses.md) predicate slice only) | two `i64` calls: numerator then denominator, inside the original inclusive component domains and constrained by owner canonical roundtrip | one owner `Rational::new(Integer::from(n), Integer::from(d))` value for the original parameter |
  | Bounded Decimal (planned [FR-051](./FR-051-decimal-scalar-harnesses.md) scalar operations only) | two `i64` calls: coefficient then scale, inside the original `decimal_range` intervals and constrained by owner `DecimalType::contains`; the pair is the retained representation | one owner `Decimal::new(Integer::from(c), s)` value for the original parameter |
  | Enum operand (planned [FR-049](./FR-049-enum-scalar-harnesses.md) comparison row only) | one `i64` call, assumed in `0..member_count` of the declaration | the owner checked member at that declared position |
  | Bounded quantity magnitude (planned [FR-050](./FR-050-quantity-scalar-harnesses.md) scalar operations only) | the physical magnitude bindings and roles defined by K-EXACT-quantity (IR-1443), inside the original inclusive domain; the unit is fixed, never a binding | one owner `Quantity` per parameter, paired with its fixed resolved `UnitQuantity` |

- If an argument's family has no row in that table, then the generator shall account the
  obligation `unsupported` with a typed reason naming the family and emit no harness.
- When an obligation reads state, the generator shall pass each state value to the subject by
  `&mut` reference to a value the harness owns, so that `kani::modifies` can name it, and shall
  keep the pre-state by copying that value before the call.
- The generator shall widen an `i64` argument into `rt::Integer` without narrowing, wrapping or
  saturating, including at `i64::MIN` and `i64::MAX`.
- If an argument's domain has no declared finite bound, then the generator shall emit no harness
  for the obligation, rather than narrowing the domain implicitly. That disposition is
  `requires-bound` at settlement ([FR-019](../../routed/functional/FR-019-capability-settlement.md)).
- If an obligation is a frame obligation, then the generator shall account it `unsupported` with a
  typed reason and emit no harness.

### Scalar V2 postcondition produced result (planned until IR-948 lands)

Before lowering a scalar V2 postcondition, CG SHALL resolve the admitted operation anchor's
context and operation name and read the complete effective signature with
`CheckedPackageV2::model_operations(context, 65_536)`. The fixed `65_536` work-unit provision
applies to each complete signature read; it is not a caller-supplied narrowing or proof bound.
CG SHALL select exactly the operation named by that anchor and pair QSpec FR-341's aggregate
binders in order: `self`, a `result` binder exactly when the signature declares a result, then
the signature's parameters in their declared order. An absent, ambiguous, unavailable or
work-exhausted signature, or a mismatched binder role, type or order, SHALL refuse the scalar
item with a typed reason and no harness. A refused `model_operations` read SHALL retain its
`CheckedOperationReadError` code, context, charged work and limit; no partial signature is used.

For a supported scalar result, the generated customer subject SHALL return `bool` for a declared
Boolean result or `i64` for a declared bounded Integer result whose inclusive endpoints fit
`i64`. A subject with no declared result returns `()`. A result with no available finite domain,
an Integer result whose endpoint lies outside `i64`, or a result family with no supported row
SHALL be accounted with a typed refusal and no scalar harness. These result types do not add a
logical `arguments` entry or a `kani::any()` draw: the `result` binder is the value produced by
the subject call, never an argument passed to that call.
The FR-015-AC-119 implication control declares a Boolean operation result and uses the `bool`
row; its subject result remains observable by an independent expectation assertion. It does
not use the no-result `()` row.

After the subject call and before the clause assertion, the harness SHALL assert that a produced
Integer result lies within its admitted inclusive result domain; it SHALL NOT assume that fact.
The `bool` return type itself enforces the declared Boolean domain. The clause oracle SHALL
receive that same produced value, with an Integer widened to `rt::Integer::from(i64)` without
loss. A subject-produced Integer outside the admitted domain therefore falsifies the harness's
assertion rather than disappearing behind an assumption. For a Boolean result, the oracle
SHALL receive the exact `bool` returned by the subject for both `true` and `false`; changing
only that oracle argument to the opposite Boolean fails the argument-identity control.
FR-015's final cover follows the result-domain and clause assertions. Until this planned path
lands, IR-1289's typed scalar result-binder refusal remains in force; the collection result path
retains its own ABI.

### Bounded-Rational grouping and lossless witness contract (planned IR-258)

For the FR-046 row, a logical Rational argument is one real admitted function parameter,
not two invented integer parameters. When emitting this argument, the generator SHALL persist
its original parameter/source identity, complete Rational domain and both physical bindings,
with distinct identifiers, primitive `i64` types, original component bounds and explicit
numerator/denominator roles and positions. Logical arguments retain ascending parameter-id
order; expansion within each Rational argument is numerator then denominator. Existing
Boolean and integer argument ordering, widths and values are unchanged.

When decoding this row, CG SHALL validate the physical schema and both original I64 playback
entries before reconstructing one semantic Rational per parameter, as specified in FR-046's
physical ABI and witness contract. A missing, repeated, foreign, role-swapped or domain-conflicting
component binding is a typed schema refusal with no replayed failure. A malformed entry or
noncanonical pair is refused without normalization into a replacement witness. Witness byte
limits apply to the retained transcript; value-count limits count the physical entries before
reconstruction, including both entries of every Rational argument. No semantic grouping hides
an entry from either limit.

When this row's complete generation, lossless decoding or public owner replay capability is
unavailable, CG SHALL account that item as typed `unsupported` with no harness. This planned
row is a required implementation contract, not evidence that these capabilities already exist.
The row adds no support for other Rational body shapes, unbounded domains or finite component
bounds outside signed-i64 capacity; missing bounds retain `requires-bound`.

### Composite leaf bindings (planned IR-806)

For a composite shadow, the drawn primitive leaves are arguments; the composite is not itself a
machine argument. This preserves AC-7 and FR-015 AC-70. When such a harness is emitted, the
generator shall persist each drawn leaf's original operand/parameter node identity, typed path,
declared leaf domain and primitive draw type, together with the owning composite declaration and
presence/member/slot controls needed to reconstruct it. Paths distinguish record fields, tuple
positions, union members, option presence/payload and collection element occurrences. Binding
order is original parameter node id then leaf path, matching actual symbolic calls. Literal
operands retain their original node/value and singleton domain and have no unconstrained draw.
No fabricated parameter binding stands in for a literal-only claim.

If a path names no declared leaf, repeats a draw position, omits a required drawn leaf or conflicts
with its original domain/type/presence control, then the generator shall refuse with a distinct
typed reason and emit no harness. Reconstruction and canonical admission are owned by
[FR-033](../../replay/functional/FR-033-composite-parity-replay-binding.md). This binding contract
adds no unsupported-family machine ABI or implicit integer narrowing.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-025-AC-1 | For every harness kind, emitted `kani::any()` bindings in source order equal the persisted physical schema obtained by row-specific expansion of the identity's logical `arguments`, which ascend by parameter identifier. Existing Boolean/integer one-binding expansion is unchanged; the planned Rational row expands each original parameter as numerator then denominator. | Test |
| FR-025-AC-2 | Every argument binding in a persisted identity names its original parameter node id and declared domain. Each planned Rational component also retains its role, position and own original bounds within that one parameter's complete Rational domain. | Test |
| FR-025-AC-3 | A Boolean argument is a `bool` that reaches the oracle unchanged, and a bounded-integer argument is an `i64` that reaches the oracle as `rt::Integer::from` of that value. The values `i64::MIN`, `-1`, `0`, `1` and `i64::MAX` each reach the oracle equal to their mathematical value. | Test (TC-036) |
| FR-025-AC-4 | An argument whose domain has no declared finite bound yields no harness and is never narrowed to a machine range. | Test (TC-036) |
| FR-025-AC-5 | A frame obligation is accounted `unsupported` with a typed reason and no harness. | Test (TC-036) |
| FR-025-AC-6 | A binding that names no parameter of the selected function, and a parameter of that function with no binding, are each refused at generation with a typed reason and no harness. | Test (TC-036) |
| FR-025-AC-7 | An argument with no applicable row (decimal outside the planned FR-051 row, IEEE, text, enum outside the planned FR-049 row, composite, collection, function, quantity outside the planned FR-050 row, or Rational outside the planned FR-046 row) is accounted `unsupported` with a typed reason naming its family/capability, and no harness is emitted. The planned Rational row does not authorize a harness without its lossless decoding and public replay prerequisites. | Test |
| FR-025-AC-8 | A harness over a state-reading obligation passes each state value to the subject as `&mut` to a harness-owned value and copies it before the call; the pre-state the harness asserts over is the copy, and the post-state is the value after the call. | Test (TC-036) |
| FR-025-AC-9 | PLANNED (IR-806). Every emitted composite shadow leaf binding retains its original operand/parameter node, typed path, declared domain and primitive draw type in symbolic call order; controls distinguish absent/null/present slots and option/union members. A missing, duplicate, unknown or domain/type-conflicting leaf refuses with a typed reason and no harness; parameter/literal and literal/literal operands retain their literal singleton identity/value with no unconstrained draw. Public reconstruction preserves these facts through FR-033. | Test |
| FR-025-AC-10 | PLANNED (IR-258). Every supported FR-046 Rational argument has exactly two ordered I64 physical bindings grouped to its one original parameter, complete domain and canonical value. Actual playback decodes losslessly with existing I64 width/comment checks and physical byte/value-count caps, then reconstructs and submits exactly one canonical owner Rational per parameter. Missing, duplicate, foreign, role-swapped or bound-conflicting schema, malformed entries, noncanonical pairs and over-limit witnesses refuse at their responsible typed boundary; no alternate normalized witness is replayed. Existing Boolean/integer controls remain unchanged. | Test |
| FR-025-AC-11 | PLANNED until IR-948 lands. A scalar V2 postcondition's operation anchor selects one complete effective `model_operations(context, 65_536)` signature; its binders match `self`, optional `result`, then declared parameters in order. An unavailable, ambiguous, work-exhausted or mismatched read gives a typed refusal with no harness, retaining the read's code, context, work and limit when the read itself refuses. No caller value changes the fixed work provision or supplies a partial signature. | Test (TC-036) |
| FR-025-AC-12 | PLANNED until IR-948 lands. A scalar V2 postcondition subject with a declared Boolean result returns `bool`, one with a finite bounded Integer result within signed-i64 capacity returns `i64`, and one with no result returns `()`; an unsupported or unavailable result domain gives a typed refusal with no harness. A declared result is neither drawn with `kani::any()` nor recorded as a parameter argument nor passed to the subject as one. | Test (TC-036) |
| FR-025-AC-13 | PLANNED until IR-948 lands. After a scalar V2 postcondition subject returns, the harness asserts a produced Integer result's admitted inclusive domain before asserting the clause and passes the same losslessly widened value to its oracle; it never assumes the result domain or substitutes a draw. A produced out-of-domain Integer falsifies the assertion, and the family cover remains after both assertions. For both `true` and `false` subject returns, the oracle receives that exact returned `bool`; a control changing only the oracle-bound Boolean to its opposite fails the argument-identity check. | Test (TC-036) |
| FR-025-AC-14 | PLANNED (IR-1438). Every supported FR-049 enum operand has exactly one `i64` physical binding grouped to its original parameter, declaration, ordered member list and index bounds. Playback decodes with the existing I64 width/comment checks and caps, then maps an in-range index to the declaration's member; an out-of-range, missing, duplicate or foreign binding refuses with a typed reason and no replay. Existing Boolean/integer controls remain unchanged. | Test (TC-036) |
| FR-025-AC-15 | PLANNED (IR-1440). Every supported FR-051 decimal operand has exactly two ordered `i64` physical bindings (coefficient, scale) grouped to its original parameter, `decimal_range` domain and retained representation. Playback decodes with the existing I64 width/comment checks and entry and byte caps, validates the scale and `DecimalType::contains`, then submits one unnormalized `WitnessValue::Decimal` per parameter. Missing, duplicate, foreign, role-swapped or domain-conflicting schema, malformed entries and out-of-domain pairs refuse with a typed reason and no replay. Existing Boolean/integer controls remain unchanged. | Test (TC-036) |

## Dependencies

- **Upstream**: [FR-015](./FR-015-bounded-kani-obligations.md), whose harnesses this ABI binds.
- **Downstream**: [FR-024](../../replay/functional/FR-024-counterexample-envelope-intake.md),
  [FR-016](../../replay/functional/FR-016-witness-native-replay.md),
  [TC-036](../matrix/TC-036-generated-subject-abi.md).
