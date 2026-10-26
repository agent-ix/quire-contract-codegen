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

When a generated function body obtains a real `Outcome::Completed(value)`, the body SHALL check that value against its selected callable's checker-issued result type using the admitted QSV environment.
When QSV `admits` returns `Ok(false)`, the body SHALL yield `GeneratedUnexpectedOutcome` at the result occurrence.
When QSV `admits` returns `Ok(true)`, the body SHALL complete with the original value.
When QSV `admits` returns `Err(EnvironmentFailure)`, the body SHALL NOT relabel undecided membership as `GeneratedUnexpectedOutcome`.
When the generated body returns `Undefined`, `Refused` or `Incomplete`, the body SHALL preserve that arm and its admitted charges.
A scalar Boolean result can use a complete environment after IR-595; a nominal result also needs QSL-1105's owner-complete declarations.
When emitted function preparation receives the selected owner's actual checked nominal declarations, the preparation SHALL build its QSV environment from them.
When that constructor returns exactly `EnvironmentFailure::Refused(InvalidDeclaration)`, the body SHALL yield `GeneratedEnvironmentRejected` at the declaration-construction occurrence before body operations or charges.
For the unmutated production body, the preparation SHALL submit each authentic owner declaration once.
For the test-only emitted-source mutation, the mutated preparation SHALL resubmit one unchanged owner declaration to that same QSV constructor, so its duplicate-key refusal exercises the shipped cause guard without an invented declaration, injected failure or production hook.
When QSV environment construction returns a limit, allocation or capacity failure, the body SHALL NOT relabel it as `GeneratedEnvironmentRejected`.
The current FR-485 facade supplies the callable result type but does not expose all nominal declaration members; QSL-1105 owns that missing input. Exact non-conflating leaf mapping for body-side QSV resource failures remains a QSL-1107 owner contract gap.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-042-AC-1 | CG's native Boolean, equality, function and pre-check placeholder constructors use the exact nine CG-owned causes in the table, retaining `IllTypedCause` from `check_type` and `check_equality`; downstream replay consumers preserve the typed payload or match it explicitly, retain charge/failure behavior, and accept neither the former unit constructor nor an invented default. | Test |
| FR-042-AC-2 | The `GeneratedUnexpectedOutcome` producer executes a generated body that returns `Completed(value)`, compares that value against its selected callable's checker-issued result type via QSV `admits`, and reports only `Ok(false)` as this cause at the result location. A test-only emitted-source mutation supplies a reachable wrong completed value to the unchanged shipped check; `Undefined`, `Refused` and `Incomplete` retain their original outcomes and charges, and `Err(EnvironmentFailure)` is never a mismatch. Scalar Boolean proof is PLANNED after IR-595; nominal result proof additionally waits for QSL-1105 and a typed body-resource mapping. | Test |
| FR-042-AC-3 | The `GeneratedEnvironmentRejected` producer uses only real nominal declarations supplied by the selected checked-closure owner, once each in unmutated production source. A test-only mutation of emitted pre-body assembly submits one unchanged owner declaration twice to the shipped QSV constructor; exactly its genuine duplicate-key `EnvironmentFailure::Refused(InvalidDeclaration)` yields this cause before body execution at the declaration occurrence, with the typed rejection retained in diagnostic evidence. Successful construction does not yield it; `Limit`, `Allocation` and `Capacity` are never relabelled. An empty default environment, invented declaration, injected failure or production hook gives no credit. PLANNED after IR-595; blocked by QSL-1105 and QSL-1107. | Test |

## Dependencies

- [quire-exact FR-369](ix://agent-ix/quire-exact/FR-369) owns the shared typed cause vocabulary and nine CG cause assignments.
- [FR-018](./FR-018-composite-equality-oracles.md) AC-8 and AC-17 name the required typed `Refusal::CheckedInvariant { cause }` payload at their producer paths. Remaining unit-style or cause-family-only wording elsewhere in FR-018 and TC-029 still needs alignment; these edits do not claim it is complete.
- [IR-595](https://linear.app/agent-ix/issue/IR-595) owns the QSL/CG direct owner cutover needed before the two planned generated-function paths can be implemented and exercised. The corresponding trigger wording in quire-exact FR-369's assignment rows must be aligned with those paths before IR-709 closes; its nine cause assignments remain required.
- [QSL-1105](https://linear.app/agent-ix/issue/QSL-1105) owns exposure of checked nominal record/tuple declarations through the closure facade. Until that owner contract is available, CG cannot build or verify the `GeneratedEnvironmentRejected` path from owner declarations.
- [QSL-1107](https://linear.app/agent-ix/issue/QSL-1107) owns a typed body-side channel for QSV `EnvironmentFailure` from result membership and environment construction; `Body::resume` currently has only `InternalFault` in its error branch.
- [IR-1021](https://linear.app/agent-ix/issue/IR-1021) owns quire-exact FR-369 and `GeneratedUnexpectedOutcome` rustdoc trigger alignment with the final CG producer contract.
- [IR-878](https://linear.app/agent-ix/issue/IR-878) tracks the allocation.

## Status

PLANNED: Seven of the nine CG-owned typed cause producer paths are present in current source. After the IR-595 owner cutover, IR-709's planned `GeneratedUnexpectedOutcome` path uses the checker-issued `CallableSelection::signature().result()` and QSV `TypeEnvironment::admits` for a real generated body's `Completed(value)`. Its scalar Boolean test can run without nominal declarations; nominal results also wait for QSL-1105. Its planned `GeneratedEnvironmentRejected` path requires QSL-1105 to expose the selected owner's real nominal declaration members. An unmutated healthy body submits each once; a test-only emitted-source mutation submits one unchanged owner declaration twice to the real QSV constructor and reaches its duplicate-key rejection without injecting an error. Both paths need QSL-1107's non-conflating body-resource result for QSV `EnvironmentFailure`; quire-eval currently exposes only `InternalFault` from `Body::resume`. Neither producer exists yet; the current exhaustive outcome matches and default environment do not supply them. IR-1021 owns upstream cause wording. IR-709 also owns TC-057 tagged verification of all nine constructors, nested `IllTypedCause`, and charge/failure behavior. An unreachable branch or an injected failure does not count as a producer path. QSL owns replay-consumer verification.
