---
id: ADR-004
title: "Decisions of the generated harness subject ABI"
type: ADR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-025
    type: relates_to
  - target: ix://agent-ix/quire-contract-codegen/FR-024
    type: relates_to
  - target: ix://agent-ix/quire-specification/AD-016
    type: relates_to
---
# ADR-004: Decisions of the generated harness subject ABI

## Status

Accepted. The owner ruled on Q1, Q2 and Q3 as the Decision states. The frame lowering in Q3
depends on an answer QSpec has not given.

## Context

[FR-025](../functional/complete-v1/FR-025-generated-subject-abi.md) states the subject ABI of a
generated harness: the argument order, the parameter and domain each binding names, lossless
widening, and the Rust types of the Boolean and bounded-integer families. AD-016 and QSL ADR-013
left the questions below undecided.

The families other than Boolean and bounded integer are rational, decimal, IEEE, text, enum,
composite, collection and function. For each one, the harness needs a symbolic argument type, and
the IR node needs a runtime operation that the oracle calls. Every subject argument was a copied
primitive, and a copied argument cannot carry `kani::modifies`, so no frame harness could be written
over it.

AD-016 decides the frame subject: the `modifies`, `creates` and `deletes` sets of the obligation's
checked `relation` and `model` node keys, each resolved to its `DeclarationKey` through the
package's model correspondence. Frame counterexample replay is QSL's `replay_frame` (QSL FR-116),
and FR-024 states CG's side of it.

## Decision

### Q1: a family gets a Rust type only when its witness decodes without loss

A family gets a row in FR-025's Rust-type table only when its witness decodes into the complete-V1
value type without loss, as the bounded-integer row does. A family with no row gets no harness and
is accounted `unsupported` with a typed reason naming the family.

### Q2: state reaches the subject by `&mut` reference

A harness passes each state argument to the subject by `&mut` reference to a value the harness owns,
so that `kani::modifies` can name it. The harness keeps the pre-state by copying the value before
the call.

### Q3: the frame subject is AD-016's, and the frame lowering is pending upstream

A frame harness is written over AD-016's frame subject. How Contract IR lowers a frame node into a
Kani form is QSpec's to answer, and QSpec has not answered it. Until it does, FR-025 accounts every
frame obligation `unsupported`, and no requirement states the frame harness.

## Consequences

- FR-025 gains the no-row refusal and the by-reference state argument as criteria. Each stays
  planned until code and a test back it.
- The frame harness gets its own requirement once QSpec decides the frame lowering.

## Alternatives Considered

- **State passed by value, with the post-state returned.** Rejected, because no frame can be stated
  over a copied argument.
- **Specify the frame lowering in this repository now.** Rejected, because that would state as
  decided what the owning authority has not decided.
