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

[FR-025](../kani/functional/FR-025-generated-subject-abi.md) states the subject ABI of a
generated harness: the argument order, the parameter and domain each binding names, lossless
widening, and the Rust types of the Boolean and bounded-integer families.

The originally unrepresented families are rational, decimal, IEEE, text, enum,
composite, collection and function. FR-025 now specifies a planned bounded-Rational
row for the FR-046 whole-function predicate slice; all other family boundaries remain.
For each one, the harness needs a symbolic argument type, and the IR node needs a runtime
operation that the oracle calls. A copied argument cannot carry
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

The planned [FR-046](../kani/functional/FR-046-rational-function-predicate-harnesses.md)
row expands one logical Rational parameter into two physical `i64` bindings, numerator then
denominator, in ascending logical parameter order. Both bindings retain their original parameter,
component role and inclusive domain. Their canonical owner constructor/accessor roundtrip gives
one-to-one mathematical representation; neither normalization duplicates nor extra parameter ids
are admitted. Existing Boolean/integer expansion and order are unchanged.

Q1's losslessness prerequisite applies to the entire roundtrip: physical playback width/comment
and byte/value-count validation, canonical owner Rational reconstruction and one typed value per
original parameter through the public owner replay route. Until that implementation and owner
capability are available, the item remains `unsupported` with no harness. The planned row is no
claim of a delivered ABI/decoder API and no authority to bypass Q1 or mint owner identities.

### Q2: state reaches the subject by `&mut` reference

A harness passes each state argument to the subject by `&mut` reference to a value the harness owns,
so that `kani::modifies` can name it. The harness keeps the pre-state by copying the value before
the call.

### Q3: the frame subject is AD-016's

A frame harness is written over AD-016's frame subject. FR-025 accounts every frame obligation
`unsupported`.

## Consequences

- FR-025 states the no-applicable-row refusal, planned Rational grouping/lossless witness contract
  and the by-reference state argument as criteria.
