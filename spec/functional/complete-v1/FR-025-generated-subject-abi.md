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
---
# FR-025: Bind each generated harness argument to its parameter, domain and Rust type

## Description

When the code generator emits an [FR-015](./FR-015-bounded-kani-obligations.md) harness, it shall
emit the harness's symbolic arguments in the order of the obligation identity's `arguments`, which
ascend by identifier. Each argument binding names its parameter node id and declared domain, each
argument has the Rust type its family's row below states, and each argument reaches the generated
oracle widened into the complete-V1 value type without loss.

This is the subject ABI that AD-016 arrow 5 and QSL ADR-013 O-25 decide. Where they leave a
decision open, it is listed under Open items and is not specified here.

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
- The generator shall bind each argument by the Rust type its family's row states:

  | Family | Symbolic argument type | Value the oracle reads |
  |---|---|---|
  | Boolean | `bool` | the same `bool` |
  | Bounded integer | `i64`, assumed inside its declared domain | `rt::Integer::from(i64)`, passed to the oracle by reference |

- The generator shall widen an `i64` argument into `rt::Integer` without narrowing, wrapping or
  saturating, including at `i64::MIN` and `i64::MAX`.
- If an argument's domain has no declared finite bound, then the generator shall emit no harness
  for the obligation, rather than narrowing the domain implicitly. That disposition is
  `requires-bound` at settlement ([FR-019](./FR-019-capability-settlement.md)).
- The subject of a frame obligation shall be the `modifies`, `creates` and `deletes` sets of the
  obligation's checked `relation` and `model` node keys, each resolved to its `DeclarationKey`
  through the package's model correspondence.
- While no Contract IR frame lowering exists, the generator shall account a frame obligation
  `unsupported` with a typed reason and emit no harness.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-025-AC-1 | For every harness kind, the identifiers of the emitted `kani::any()` bindings, in source order, equal the identity's `arguments` identifiers in order, and those ascend by identifier. | Test (TC-036) |
| FR-025-AC-2 | Every argument binding in a persisted identity names its parameter node id and declared domain. A binding with no parameter, or a parameter with no binding, is refused at generation with a typed reason and no harness. | Test (TC-036) |
| FR-025-AC-3 | A Boolean argument is a `bool` that reaches the oracle unchanged, and a bounded-integer argument is an `i64` that reaches the oracle as `rt::Integer::from` of that value. The values `i64::MIN`, `-1`, `0`, `1` and `i64::MAX` each reach the oracle equal to their mathematical value. | Test (TC-036) |
| FR-025-AC-4 | An argument whose domain has no declared finite bound yields no harness and is never narrowed to a machine range. | Test (TC-036) |
| FR-025-AC-5 | A frame obligation is accounted `unsupported` with a typed reason and no harness while no Contract IR frame lowering exists. | Test (TC-036) |

## Open items

Each item is left open by AD-016 or QSL ADR-013, and this requirement specifies none of them. Each
has a recommendation for the owner.

- **Rust types of the other families.** The Rust type of a symbolic argument for rational, decimal,
  IEEE, text, enum, composite, collection and function-family values is not decided. The IR-node →
  runtime-operation selection table is AD-016 `OPEN — decided in WP8`, and so is the
  `SemanticFamily::Function` bound representation. Recommendation: a family gets a row in the
  table above only when its witness decodes losslessly into the complete-V1 value type, as the
  integer row does.
- **By-reference state arguments.** Neither AD-016 nor QSL ADR-013 decides whether a harness
  passes state to the subject by value or by reference. Today every subject argument is a copied
  primitive, and a copied argument cannot carry `kani::modifies`, so a frame harness cannot be
  written over it. Recommendation: pass state by `&mut` reference to a harness-owned value, so that
  `kani::modifies` can name it and the pre-state can be kept by copying before the call. Decide
  this together with the frame harness.
- **The frame harness.** Contract IR's Kani frame lowering and CG's frame-harness file list are
  AD-016 `OPEN — decided in WP10`. Frame counterexample replay is already QSL's `replay_frame`
  (QSL FR-116), and [FR-024](./FR-024-counterexample-envelope-intake.md) states CG's side of it.

## Dependencies

- **Upstream**: [FR-015](./FR-015-bounded-kani-obligations.md), whose harnesses this ABI binds.
- **Downstream**: [FR-024](./FR-024-counterexample-envelope-intake.md),
  [FR-016](./FR-016-witness-native-replay.md),
  [TC-036](../../test/complete-v1/TC-036-generated-subject-abi.md).
