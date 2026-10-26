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

For `GeneratedUnexpectedOutcome`, a generated function body SHALL check a real
`Outcome::Completed(value)` against its selected callable's checker-issued
result type using the admitted QSV environment. A value that the declared
type does not admit SHALL yield that cause at the body boundary; other outcome
arms SHALL keep their own identity and charges. For
`GeneratedEnvironmentRejected`, emitted function preparation SHALL build its
QSV environment from the selected owner's actual checked nominal
declarations. A genuine declaration rejection SHALL yield that cause before
the body runs; limit and cancellation failures SHALL retain their own typed
paths. The current FR-485 facade supplies the callable result type but does
not expose all nominal declaration members; QSL-1105 owns that missing input.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-042-AC-1 | CG's native Boolean, equality, function and pre-check placeholder constructors use the exact nine CG-owned causes in the table, retaining `IllTypedCause` from `check_type` and `check_equality`; downstream replay consumers preserve the typed payload or match it explicitly, retain charge/failure behavior, and accept neither the former unit constructor nor an invented default. | Test |
| FR-042-AC-2 | The `GeneratedUnexpectedOutcome` producer executes a generated body that actually returns `Completed(value)`, compares the value against that selected callable's checker-issued result type via the admitted environment, and reports only a genuine mismatch as that cause. A reachable wrong-result generated-body mutation fails the completed-result control; `Undefined`, `Refused` and `Incomplete` retain their original outcomes and charges. PLANNED (IR-709 after IR-595). | Test |
| FR-042-AC-3 | The `GeneratedEnvironmentRejected` producer uses only real nominal declarations supplied by the selected checked-closure owner; a genuine QSV construction rejection yields that cause before body execution while a successful construction does not. The rejection's original typed detail remains observable in diagnostics; limit and cancellation failures are not relabelled. An empty default environment, invented declaration or forced failure gives no acceptance credit. PLANNED (IR-709 after IR-595; blocked by QSL-1105). | Test |

## Dependencies

- [quire-exact FR-369](ix://agent-ix/quire-exact/FR-369) owns the shared typed cause vocabulary and nine CG cause assignments.
- [FR-018](./FR-018-composite-equality-oracles.md) AC-8 and AC-17 name the required typed `Refusal::CheckedInvariant { cause }` payload at their producer paths. Remaining unit-style or cause-family-only wording elsewhere in FR-018 and TC-029 still needs alignment; these edits do not claim it is complete.
- [IR-595](https://linear.app/agent-ix/issue/IR-595) owns the QSL/CG direct owner cutover needed before the two planned generated-function paths can be implemented and exercised. The corresponding trigger wording in quire-exact FR-369's assignment rows must be aligned with those paths before IR-709 closes; its nine cause assignments remain required.
- [QSL-1105](https://linear.app/agent-ix/issue/QSL-1105) owns exposure of checked nominal record/tuple declarations through the closure facade. Until that owner contract is available, CG cannot build or verify the `GeneratedEnvironmentRejected` path from owner declarations.
- [IR-878](https://linear.app/agent-ix/issue/IR-878) tracks the allocation.

## Status

PLANNED: Seven of the nine CG-owned typed cause producer paths are present in current source. After the IR-595 owner cutover, IR-709's planned `GeneratedUnexpectedOutcome` path uses the checker-issued `CallableSelection::signature().result()` and QSV `TypeEnvironment::admits` for a real generated body's `Completed(value)`. Its planned `GeneratedEnvironmentRejected` path requires QSL-1105 to expose the selected owner's real nominal declaration members before CG can construct an environment and observe a genuine rejection. Neither producer exists yet; the current exhaustive outcome matches and default environment do not supply them. IR-709 also owns TC-057 tagged verification of all nine constructors, nested `IllTypedCause`, and charge/failure behavior. An unreachable branch or a fabricated failure does not count as a producer path. QSL owns replay-consumer verification.
