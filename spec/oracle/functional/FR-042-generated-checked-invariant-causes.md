---
id: FR-042
title: "Emit typed checked-invariant causes from generated oracles"
type: FR
relationships:
  - target: ix://agent-ix/quire-exact/FR-369
    type: depends_on
---
# FR-042: Emit typed checked-invariant causes from generated oracles

## Description

The code generator SHALL assign the shared quire-exact typed checked-invariant cause at each generated Boolean, equality, function and pre-check placeholder constructor.

## Inputs

- Checked native oracle requests and RT type/equality check results.

## Outputs

- Generated oracle refusals retaining the precise shared cause and nested `IllTypedCause` where supplied.

## Behavior

The code generator SHALL use the nine CG-owned causes in the second cause table of [quire-exact FR-369](ix://agent-ix/quire-exact/FR-369) at their owning constructors. CG owns generated Boolean, equality, function and pre-check placeholder constructors and the payload they emit. QSL `qsl-replay/src/scalar.rs` owns the downstream `Refusal::CheckedInvariant { cause }` consumer; its preservation and matching work is tracked at that owner. The generated oracle SHALL preserve its payload and charge behavior.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-042-AC-1 | CG's native Boolean, equality, function and pre-check placeholder constructors use the exact nine CG-owned causes in the table, retaining `IllTypedCause` from `check_type` and `check_equality`; downstream replay consumers preserve the typed payload or match it explicitly, retain charge/failure behavior, and accept neither the former unit constructor nor an invented default. | Test |

## Dependencies

- [quire-exact FR-369](ix://agent-ix/quire-exact/FR-369) owns the shared typed cause vocabulary and nine CG cause assignments.
- [FR-018](./FR-018-composite-equality-oracles.md) AC-8 and AC-17 still name the former unit `Refusal::CheckedInvariant` constructor; their producer/consumer updates must align with the required payload in FR-042-AC-1.
- [IR-878](https://linear.app/agent-ix/issue/IR-878) tracks the allocation.

## Status

PLANNED: IR-709 owns generated producer and consumer changes and tests. The former unit-variant removal is gated on the IR-497 coordinated cutover and the dependency bump that supplies the typed payload.
