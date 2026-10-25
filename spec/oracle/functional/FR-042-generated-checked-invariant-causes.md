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

The code generator SHALL use these seven CG-owned causes at their owning constructors: `GeneratedBodyPlaceholderInvoked`, `GeneratedArgumentShapeMismatch`, `GeneratedOperandKindUnsupported`, `GeneratedIntervalInvalid`, `GeneratedTypeCheckRejected { cause }`, `GeneratedEqualityCheckRejected { cause }`, and `GeneratedDescriptorReconstructionFailed`. CG owns generated Boolean, equality, function and pre-check placeholder constructors and the payload they emit. QSL `qsl-replay/src/scalar.rs` owns the downstream `Refusal::CheckedInvariant { cause }` consumer; its preservation and matching work is tracked at that owner. The generated oracle SHALL preserve its payload and charge behavior.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-042-AC-1 | CG's native Boolean, equality, function and pre-check placeholder constructors use exactly the seven causes named in Behavior, retaining `IllTypedCause` from `check_type` and `check_equality`; downstream replay consumers preserve the typed payload or match it explicitly, retain charge/failure behavior, and accept neither the former unit constructor nor an invented default. | Test |

## Dependencies

- [quire-exact FR-369](ix://agent-ix/quire-exact/FR-369) owns the closed typed cause vocabulary and currently retains nine CG assignment rows. Its `GeneratedUnexpectedOutcome` and `GeneratedEnvironmentRejected` rows remain PLANNED under CG FR-042-AC-1/IR-709 in the merged owner spec. If the owner authorizes seven assignments, a new quire-exact PR correcting that table must merge together with this PR; both variants remain in the shared enum.
- [FR-018](./FR-018-composite-equality-oracles.md) AC-8 and AC-17 require the typed `Refusal::CheckedInvariant { cause }` payload at their producer paths.
- [IR-878](https://linear.app/agent-ix/issue/IR-878) tracks the allocation.

## Status

PLANNED: Seven CG-owned typed cause producer paths are present in current source; TC-057 tagged verification of all seven, nested `IllTypedCause`, and charge/failure behavior remains open under IR-709. The other two named assignments remain PLANNED in quire-exact FR-369 while this correction awaits direct owner authorization. The Boolean helper calls `planned_equality` without constructing an environment; the generated function uses `TypeEnvironment::default` and exhaustive outcome matches; composite equality environment admission fails with `EnvironmentError::Declaration` at preparation. None of these paths produces `GeneratedUnexpectedOutcome` or `GeneratedEnvironmentRejected`. QSL owns replay-consumer verification.
