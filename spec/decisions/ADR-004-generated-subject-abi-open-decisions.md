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

Accepted.

## Context

[FR-025](../functional/complete-v1/FR-025-generated-subject-abi.md) states the subject ABI of a
generated harness: the argument order, the parameter and domain each binding names, lossless
widening, and the Rust types of the Boolean and bounded-integer families.

The families other than Boolean and bounded integer are rational, decimal, IEEE, text, enum,
composite, collection and function. For each one, the harness needs a symbolic argument type, and
the IR node needs a runtime operation that the oracle calls. A copied argument cannot carry
`kani::modifies`.

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

### Q3: the frame subject is AD-016's

A frame harness is written over AD-016's frame subject. FR-025 accounts every frame obligation
`unsupported`.

## Consequences

- FR-025 states the no-row refusal and the by-reference state argument as criteria.
