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
emit the harness's symbolic arguments in the order of the obligation identity's `arguments`, which
ascend by identifier. Each argument binding names its parameter node id and declared domain, each
argument has the Rust type its family's row below states, and each argument reaches the generated
oracle widened into the complete-V1 value type without loss.

The ascending order, the binding to a parameter node id and declared domain, and lossless widening
are what AD-016 arrow 5 and QSL ADR-013 O-25 decide. The Rust-type table, the by-reference state
argument and the frame disposition are
[ADR-004](../../decisions/ADR-004-generated-subject-abi-open-decisions.md)'s decisions. A family
gets a row in the table only when its witness decodes into the complete-V1 value type without loss.

## Inputs

- The obligation's argument bindings and their IR `bounded_domain` bounds (FR-015).
- The parameter node id of each argument in the admitted package.

## Outputs

- A harness whose `kani::any()` calls, and the `arguments` of its persisted identity, share one
  order.
- Argument bindings that each carry their parameter node id and declared domain.

## Behavior

- The generator shall order an identity's `arguments` ascending by identifier.
- The generator shall emit one `kani::any()` call per argument, in the order of `arguments`.
- Each argument binding shall name the node id of the parameter it binds and that parameter's
  declared domain.
- If a binding names no parameter of the selected function, or a parameter of that function has no
  binding, then the generator shall refuse the obligation with a typed reason and emit no harness.
- The generator shall bind each argument by the Rust type its family's row states:

  | Family | Symbolic argument type | Value the oracle reads |
  |---|---|---|
  | Boolean | `bool` | the same `bool` |
  | Bounded integer | `i64`, assumed inside its declared domain | `rt::Integer::from(i64)`, passed to the oracle by reference |

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

### Composite leaf bindings (planned IR-635)

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
| FR-025-AC-1 | For every harness kind, the identifiers of the emitted `kani::any()` bindings, in source order, equal the identity's `arguments` identifiers in order, and those ascend by identifier. | Test (TC-036) |
| FR-025-AC-2 | Every argument binding in a persisted identity names its parameter node id and declared domain. | Test (TC-036) |
| FR-025-AC-3 | A Boolean argument is a `bool` that reaches the oracle unchanged, and a bounded-integer argument is an `i64` that reaches the oracle as `rt::Integer::from` of that value. The values `i64::MIN`, `-1`, `0`, `1` and `i64::MAX` each reach the oracle equal to their mathematical value. | Test (TC-036) |
| FR-025-AC-4 | An argument whose domain has no declared finite bound yields no harness and is never narrowed to a machine range. | Test (TC-036) |
| FR-025-AC-5 | A frame obligation is accounted `unsupported` with a typed reason and no harness. | Test (TC-036) |
| FR-025-AC-6 | A binding that names no parameter of the selected function, and a parameter of that function with no binding, are each refused at generation with a typed reason and no harness. | Test (TC-036) |
| FR-025-AC-7 | An argument of a family with no row in the Rust-type table (rational, decimal, IEEE, text, enum, composite, collection or function) is accounted `unsupported` with a typed reason naming the family, and no harness is emitted. | Test (TC-036) |
| FR-025-AC-8 | A harness over a state-reading obligation passes each state value to the subject as `&mut` to a harness-owned value and copies it before the call; the pre-state the harness asserts over is the copy, and the post-state is the value after the call. | Test (TC-036) |

| FR-025-AC-9 | PLANNED (IR-635). Every emitted composite shadow leaf binding retains its original operand/parameter node, typed path, declared domain and primitive draw type in symbolic call order; controls distinguish absent/null/present slots and option/union members. A missing, duplicate, unknown or domain/type-conflicting leaf refuses with a typed reason and no harness; parameter/literal and literal/literal operands retain their literal singleton identity/value with no unconstrained draw. Public reconstruction preserves these facts through FR-033. | Test |

## Dependencies

- **Upstream**: [FR-015](./FR-015-bounded-kani-obligations.md), whose harnesses this ABI binds.
- **Downstream**: [FR-024](../../replay/functional/FR-024-counterexample-envelope-intake.md),
  [FR-016](../../replay/functional/FR-016-witness-native-replay.md),
  [TC-036](../matrix/TC-036-generated-subject-abi.md).
