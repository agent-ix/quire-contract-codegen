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

The scalar V2 clause path currently refuses an operation with a declared result (IR-1289).
QSpec FR-341 binds `result` between `self` and the operation's parameters, whereas the result
is produced by the subject call, not drawn as a parameter. FR-025 needs an explicit Rust
return type and a bounded admitted-signature read before that scalar path can support it.
The collection postcondition result path has its own supported lowering and is unaffected.

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

### Q4: a scalar V2 postcondition result is a typed subject return (planned until IR-948 lands)

The generated customer subject returns `bool` for a declared Boolean result and `i64` for a
declared bounded Integer result whose inclusive domain fits signed `i64`; an operation without
a declared result returns `()`. The result is never a symbolic parameter draw or a subject
argument. After the call, the harness asserts an Integer result's admitted inclusive domain
before the clause assertion and gives that same result to the clause oracle, widening it
losslessly as `rt::Integer::from(i64)`. A Boolean return's Rust type enforces its domain.
An unsupported, unavailable or out-of-i64 result domain produces a typed refusal and no scalar
harness. The existing collection result path remains governed by its own supported ABI.

### Q5: the admitted operation signature read has a fixed work provision (planned until IR-948 lands)

CG resolves the QSpec FR-342 operation anchor's context and name, then calls IR
`CheckedPackageV2::model_operations(context, 65_536)` for a complete effective signature.
The fixed work limit is charged per read and is not taken from a caller or a proof bound.
CG matches QSpec FR-341's `self`, optional `result` and declared parameter order against that
signature before lowering. An ambiguous, unavailable, work-exhausted or mismatched read
refuses the scalar item with a typed reason and no harness. A refusal from IR retains its
code, context, work and limit; partial signatures are never used. IR-1289's scalar typed
result-binder refusal stays in effect until this planned route is implemented.

## Consequences

- FR-025 states the no-applicable-row refusal, planned Rational grouping/lossless witness contract
  and the by-reference state argument as criteria. Its planned scalar V2 result criteria state
  the return mapping, fixed signature-read work provision and post-call result-domain assertion.
