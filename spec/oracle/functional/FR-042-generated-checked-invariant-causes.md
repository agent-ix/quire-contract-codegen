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

The code generator SHALL use the seven CG-owned causes with genuine producer paths in the second cause table of [quire-exact FR-369](ix://agent-ix/quire-exact/FR-369) at their owning constructors. CG owns generated Boolean, equality, function and pre-check placeholder constructors and the payload they emit. QSL `qsl-replay/src/scalar.rs` owns the downstream `Refusal::CheckedInvariant { cause }` consumer; its preservation and matching work is tracked at that owner. The generated oracle SHALL preserve its payload and charge behavior.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-042-AC-1 | CG's native Boolean, equality, function and pre-check placeholder constructors use the exact seven CG-owned causes with genuine producer paths in the table, retaining `IllTypedCause` from `check_type` and `check_equality`; downstream replay consumers preserve the typed payload or match it explicitly, retain charge/failure behavior, and accept neither the former unit constructor nor an invented default. The emitted sources and TC-057 controls enumerate every one of those seven cause paths; no cause is counted from an unreachable branch or a fabricated failure. | Test |

## Dependencies

- [quire-exact FR-369](ix://agent-ix/quire-exact/FR-369) owns the shared typed cause vocabulary and CG cause assignments; its table correction is tracked with IR-709 at that owner.
- [FR-018](./FR-018-composite-equality-oracles.md) AC-8 and AC-17 require the typed `Refusal::CheckedInvariant { cause }` payload at their producer paths.
- [IR-878](https://linear.app/agent-ix/issue/IR-878) tracks the allocation.

## Status

PLANNED: Seven CG-owned typed cause producer paths are present in current source; TC-057 tagged verification of all seven, nested `IllTypedCause`, and charge/failure behavior remains open under IR-709. The obsolete `GeneratedUnexpectedOutcome` and `GeneratedEnvironmentRejected` assignments have no genuine failure path in the checked generated Boolean/function oracles and are not claimed as current producers. QSL owns replay-consumer verification.
