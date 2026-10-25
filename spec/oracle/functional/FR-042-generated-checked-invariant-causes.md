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
- [FR-018](./FR-018-composite-equality-oracles.md) AC-8 and AC-17 name the required typed `Refusal::CheckedInvariant { cause }` payload at their producer paths. Remaining unit-style or cause-family-only wording elsewhere in FR-018 and TC-029 still needs alignment; these edits do not claim it is complete.
- [IR-595](https://linear.app/agent-ix/issue/IR-595) owns the QSL/CG direct owner cutover needed before the two planned generated-function paths can be implemented and exercised. The corresponding trigger wording in quire-exact FR-369's assignment rows must be aligned with those paths before IR-709 closes; its nine cause assignments remain required.
- [IR-878](https://linear.app/agent-ix/issue/IR-878) tracks the allocation.

## Status

PLANNED: Seven of the nine CG-owned typed cause producer paths are present in current source. After the IR-595 owner cutover, IR-709's planned `GeneratedUnexpectedOutcome` path checks an actual generated operation's `Outcome::Completed(value)` against its declared result type and refuses a genuine mismatch. Its planned `GeneratedEnvironmentRejected` path constructs the emitted function's type environment from real admitted nominal declarations and preserves a genuine rejection from that construction. Neither path exists yet; the current exhaustive outcome matches and default environment do not supply them. IR-709 also owns TC-057 tagged verification of all nine constructors, nested `IllTypedCause`, and charge/failure behavior. An unreachable branch or a fabricated failure does not count as a producer path. QSL owns replay-consumer verification.
