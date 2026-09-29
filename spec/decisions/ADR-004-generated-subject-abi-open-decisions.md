---
id: ADR-004
title: "Open decisions of the generated harness subject ABI"
type: ADR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-025
    type: relates_to
  - target: ix://agent-ix/quire-contract-codegen/FR-024
    type: relates_to
  - target: ix://agent-ix/quire-specification/AD-016
    type: relates_to
---
# ADR-004: Open decisions of the generated harness subject ABI

## Status

Proposed. Each question is open until the owner rules. The recommendations decide nothing.

## Context

[FR-025](../functional/complete-v1/FR-025-generated-subject-abi.md) states the subject ABI of a
generated harness: the argument order, the parameter and domain each binding names, lossless
widening, and the Rust types of the Boolean and bounded-integer families. AD-016 and QSL ADR-013
leave the questions below undecided, and FR-025 specifies none of them.

## Decision

Nothing is decided. The owner rules on each question below.

### Q1: Which Rust type carries a symbolic argument of each other family?

The families are rational, decimal, IEEE, text, enum, composite, collection and function. For each
one, the harness needs a symbolic argument type, and the IR node needs a runtime operation that the
oracle calls. The bound representation of a function-family argument is also undecided.

Recommendation: a family gets a row in FR-025's table only when its witness decodes into the
complete-V1 value type without loss, as the integer row does.

### Q2: Does a harness pass state to the subject by value or by reference?

Today every subject argument is a copied primitive. A copied argument cannot carry
`kani::modifies`, so a frame harness cannot be written over it.

Options:

1. **By `&mut` reference to a harness-owned value.** `kani::modifies` can then name it, and the
   pre-state is kept by copying before the call.
2. **By value, with the post-state returned.** No frame can be stated.

Recommendation: option 1, decided together with Q3.

### Q3: What is the frame harness?

AD-016 decides the frame subject: the `modifies`, `creates` and `deletes` sets of the obligation's
checked `relation` and `model` node keys, each resolved to its `DeclarationKey` through the
package's model correspondence. It does not decide how Contract IR lowers a frame node into a Kani
form, or which generator files emit the frame harness. Frame counterexample replay is QSL's
`replay_frame` (QSL FR-116), and FR-024 states CG's side of it.

Recommendation: write a frame-harness requirement over that subject once Q2 is ruled on. Until
then, FR-025 accounts every frame obligation `unsupported`.

## Consequences

Each ruling becomes rows or criteria in FR-025, and for Q3 a new requirement.

## Alternatives Considered

- **Specify these in FR-025 now.** That would state as decided what no authority has decided.
