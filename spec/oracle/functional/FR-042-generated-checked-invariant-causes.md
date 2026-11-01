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

The code generator SHALL assign the shared quire-exact typed checked-invariant cause at each generated Boolean, equality and function constructor.

## Inputs

- Checked native oracle requests and RT type/equality check results.

## Outputs

- Generated oracle refusals retaining the precise shared cause and nested `IllTypedCause` where supplied.

## Behavior

The code generator SHALL use the eight CG-owned causes in the second cause table of [quire-exact FR-369](ix://agent-ix/quire-exact/FR-369) at their owning constructors. CG owns generated Boolean, equality and function constructors and the payload they emit. QSL `qsl-replay/src/scalar.rs` owns the downstream `Refusal::CheckedInvariant { cause }` consumer; its preservation and matching work is tracked at that owner. The generated oracle SHALL preserve its payload and charge behavior.

`GeneratedBodyPlaceholderInvoked` is retired: IR-595 replaced the pre-check placeholder with concrete registered bodies, so there is no remaining producer to allocate. The generator SHALL NOT restore a placeholder or reallocate this name to another trigger. An unexpected body resume remains a genuine `InternalFault`; it SHALL NOT be relabelled as a checked-invariant refusal. The eight remaining cause assignments and their original triggers are unchanged.

When a generated function body obtains a real `Outcome::Completed(value)`, the body SHALL check that value against its selected callable's checker-issued result type using the admitted QSV environment.
When QSV `admits` returns `Ok(false)`, the body SHALL yield `GeneratedUnexpectedOutcome` at the result occurrence.
When QSV `admits` returns `Ok(true)`, the body SHALL complete with the original value.
When QSV `admits` returns `Err(EnvironmentFailure)`, the body SHALL NOT relabel undecided membership as `GeneratedUnexpectedOutcome`.
When the generated body returns `Undefined`, `Refused` or `Incomplete`, the body SHALL preserve that arm and its admitted charges.
A scalar Boolean result uses a complete scalar environment; nominal membership uses the selected owner's declarations, retaining each field's exact presence as well as its type.
When emitted function preparation receives the selected owner's actual checked nominal declarations, the preparation SHALL build its QSV environment from them.
When that constructor returns exactly `EnvironmentFailure::Refused(InvalidDeclaration)`, the body SHALL yield `GeneratedEnvironmentRejected` at the declaration-construction occurrence before body operations or charges.
For the unmutated production body, the preparation SHALL submit each authentic owner declaration once.
For the test-only emitted-source mutation, the mutated preparation SHALL resubmit one unchanged owner declaration to that same QSV constructor, so its duplicate-key refusal exercises the shipped cause guard without an invented declaration, injected failure or production hook.
When QSV environment construction returns a limit, allocation or capacity failure, the body SHALL NOT relabel it as `GeneratedEnvironmentRejected`.
The owner-issued views supply each declaration's key, form, member order and types; a record view SHALL also retain each field's presence. CG SHALL NOT substitute Required for unavailable presence. Body-side limits retain the original caller cancellation handle and configured QSV environment limits. Resource failures retain BodyFailure::QsvEnvironment through CallFailure::Body, with their complete owner payload and observed cancellation. Allocation and capacity controls select QSV's public reservation-boundary constructor/membership entry in the test-only emitted copy, using unchanged owner inputs and cause guards; production gains no failure-policy hook.

For the current Boolean/Integer result families, the primitive QSV membership
walk allocates no storage. Allocation/Capacity handling at this body-result
site is therefore verified by inspection with **no execution credit**. The
reservation-boundary control must observe zero requests and retain the original
primitive completion; it must not fabricate a reservation or failure.
TC-057 gains Allocation/Capacity execution credit only when the first shipped
body family has a checker-issued result type and genuine completed value whose
actual result-membership walk requests storage. That family's delivery must
observe the actual request, separately deny it as Allocation and Capacity
through QSV's public seam, and assert the unchanged membership/cause guards'
complete typed leaf failure, charge prefix and location behavior. A heap-backed
result whose membership is allocation-free does not satisfy this condition.
[IR-1235](https://linear.app/agent-ix/issue/IR-1235) retains this activation
obligation; no unrelated body family is added solely to manufacture coverage.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-042-AC-1 | CG's native Boolean, equality and function constructors use the exact eight CG-owned causes in the table, retaining `IllTypedCause` from `check_type` and `check_equality`; downstream replay consumers preserve the typed payload or match it explicitly, retain charge/failure behavior, and accept neither the former unit constructor nor an invented default. | Test |
| FR-042-AC-2 | The `GeneratedUnexpectedOutcome` producer executes a generated body that returns `Completed(value)`, compares that value against its selected callable's checker-issued result type via QSV `admits`, and reports only `Ok(false)` as this cause at the result location. A test-only emitted-source mutation supplies a reachable wrong completed value to the unchanged shipped check; `Undefined`, `Refused` and `Incomplete` retain their original outcomes and charges, and `Err(EnvironmentFailure)` is never a mismatch. For current Boolean/Integer results, Allocation/Capacity mapping is inspected with no execution credit because membership allocates no storage; the first shipped allocating-result membership family activates the explicit TC-057 / IR-1235 execution condition. Genuine resource failures must retain the typed leaf failure instead of a false verdict. | Test and Inspection |
| FR-042-AC-3 | The `GeneratedEnvironmentRejected` producer uses only real nominal declarations supplied by the selected checked-closure owner, once each in unmutated production source. A test-only mutation of emitted pre-body assembly submits one unchanged owner declaration twice to the shipped QSV constructor; exactly its genuine duplicate-key `EnvironmentFailure::Refused(InvalidDeclaration)` yields this cause before body execution at the declaration occurrence, with the typed rejection retained in diagnostic evidence. Successful construction does not yield it; `Limit`, `Allocation` and `Capacity` are never relabelled. An empty default environment, invented declaration, injected failure or production hook gives no credit. Public Limit/cancel and actual QSV reservation-boundary controls exercise their distinct typed failures; no failure is injected into a cause guard. | Test and Inspection |

## Dependencies

- [quire-exact FR-369](ix://agent-ix/quire-exact/FR-369) owns the shared typed cause vocabulary and eight CG cause assignments.
- [FR-018](./FR-018-composite-equality-oracles.md) AC-8 and AC-17 name the required typed `Refusal::CheckedInvariant { cause }` payload at their producer paths. Remaining unit-style or cause-family-only wording elsewhere in FR-018 and TC-029 still needs alignment; these edits do not claim it is complete.
- [IR-595](https://linear.app/agent-ix/issue/IR-595) owns the QSL/CG direct owner cutover needed before the two generated-function paths can be implemented and exercised. The corresponding trigger wording in quire-exact FR-369's assignment rows must be aligned with those paths before IR-709 closes; its eight cause assignments remain required.
- [QSL-1105](https://linear.app/agent-ix/issue/QSL-1105) owns the checked nominal declaration facade. QSL FR-485-AC-18 additionally retains each record field's exact presence; CG consumes that field without a default.
- [QSL-1107](https://linear.app/agent-ix/issue/QSL-1107) owns the typed `BodyFailure::QsvEnvironment` channel and public QSV reservation seam. Genuine constructor storage controls use it now; primitive result membership remains allocation-free, as the execution-credit condition above states.
- [IR-1021](https://linear.app/agent-ix/issue/IR-1021) owns quire-exact FR-369 and `GeneratedUnexpectedOutcome` rustdoc trigger alignment with the final CG producer contract.
- [IR-878](https://linear.app/agent-ix/issue/IR-878) tracks the cause allocation. [IR-1235](https://linear.app/agent-ix/issue/IR-1235) tracks storage-failure execution when the first allocating-result membership family ships.

## Status

The eight live constructors have TC-057-tagged native controls, including exact
nested IllTypedCause payloads and charge/failure boundaries. The real selected
signature drives result membership. Authentic record, tuple and union views,
including required/optional field presence and sealed identities, drive the
healthy and duplicate-declaration controls. Genuine constructor work/cancel and
storage stops are executed. Primitive result-membership storage failure remains
inspection-only with zero execution credit until the explicit IR-1235 activation
condition is met. QSL owns replay-consumer verification. The retired placeholder
supplies no execution credit.
