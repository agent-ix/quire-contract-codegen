---
id: SR-576002
title: "IR-709 PR #576 spec-review/spec-integrity-analysis review"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen; PR #576 head 5188f6f45447ce2d33024e1c55c3813ddb692daf on base b60b18ef; spec/oracle/functional/FR-042-generated-checked-invariant-causes.md, spec/oracle/functional/FR-018-composite-equality-oracles.md, spec/oracle/matrix/TC-029-composite-equality-oracles.md (context: spec/oracle/matrix/tests.md); IR-709"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-042
    type: reviews
---

## Summary

This is the integrity sub-analysis (consistency, completeness and atomicity) of PR #576 at head 5188f6f4. The diff was compared word by word against main b60b18ef. Ticket: IR-709. The cross-repo contradiction with quire-exact #84 and the stale TC-057 procedure are recorded in SR-576001 (FND-001 and FND-003) and are not repeated here.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The oracle matrix trace row for FR-042-AC-1 was not updated. It still reads "seven of nine typed CG producers landed; two producers and TC-057 tagged verification remain open". That contradicts the amended FR-042 Status, which says there are seven producers and the two are not claimed. Update the row to match. | spec/oracle/matrix/tests.md:62 |
| FND-002 | low | FR-042 Status calls the two assignments "obsolete". Both variants remain in quire-exact FR-369's closed thirty-variant enum (FR-369-AC-1), so "obsolete" invites removing them, which neither the owner nor this PR has decided. The Status also says they have "no genuine failure path" without the reason. Current source has four exhaustive `Outcome` arms with no fallback. Boolean and function equality use the infallible `planned_equality` and `TypeEnvironment::default()`. Equality-environment admission returns `EnvironmentError::Declaration`. Say "unassigned in CG" rather than "obsolete", and state the reason in one clause so the claim can be re-measured. | spec/oracle/functional/FR-042-generated-checked-invariant-causes.md:41 |

## Verdict

Integrity alone: mergeable after FND-001. FND-002 is a nit. Checked clean:

- FR-042 Description, Inputs, Outputs and Behavior agree with the amended AC-1.
- The FR-018 Dependencies line correctly drops the "still name the former unit constructor" text, because the three unit spellings in FR-018 (lines 71, 431, 606) and AC-8 and AC-17 are now typed.
- No `Refusal::CheckedInvariant)` unit spelling remains in FR-018, TC-029 or FR-042.
- FR-018-AC-8's three environment cases are unchanged.
- TC-029 step 3 matches AC-8's new payload and step 8 matches AC-17's.
- No other AC row in FR-018 differs from main.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | The new Status reason gives Boolean/function oracles an environment construction that "uses infallible TypeEnvironment::default or fails with EnvironmentError::Declaration at preparation". The Boolean helper builds no environment; it calls planned_equality. EnvironmentError::Declaration is the composite equality oracle's preparation error, not a Boolean/function path. The conclusion is correct, but the attribution is wrong. Assign each reason to its own oracle. | spec/oracle/functional/FR-042-generated-checked-invariant-causes.md:41 |

## Dispositions

Round 1, checked against the repository at head f6ca73d22bc3b7b9d2569fc85819c90b1689ecfd.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The tests.md:62 row now reads 'seven typed CG producer paths are present, while two additional assignments remain planned in quire-exact FR-369 pending the direct owner decision ... TC-057 tagged verification of the seven named paths remains open'. |
| FND-002 | fixed | 'Obsolete' is removed. Status says the other two assignments 'remain PLANNED in quire-exact FR-369 while this correction awaits direct owner authorization' and states the reasons (exhaustive outcome matches, infallible default environment, EnvironmentError::Declaration). One attribution in that reason is imprecise; see FND-003. |

## Coverage

```yaml
scope:
- id: FR-042
  path: spec/oracle/functional/FR-042-generated-checked-invariant-causes.md
  role: examined
  excerpt: "PLANNED: Seven CG-owned typed cause producer paths are present in current source; ... The obsolete `GeneratedUnexpectedOutcome` and `GeneratedEnvironmentRejected` assignments have no genuine failure path in the checked generated Boolean/function oracles and are not claimed as current producers."
- id: FR-042-AC-1
  path: spec/oracle/functional/FR-042-generated-checked-invariant-causes.md
  role: examined
  excerpt: "use the exact seven CG-owned causes with genuine producer paths in the table ... The emitted sources and TC-057 controls enumerate every one of those seven cause paths"
- id: FR-018-AC-8
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: "yields `Outcome::Refused(Refusal::CheckedInvariant { cause: CheckedInvariantCause::GeneratedTypeCheckRejected { cause: error.cause } })` with the original `IllTypedCause` and no charge admitted"
- id: FR-018-AC-17
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: "a `match` whose `Err` arm yields `EnvironmentError::Reconstruction` or `Outcome::Refused(Refusal::CheckedInvariant { cause })`"
- id: FR-018
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: "an `IllTyped` from either `check_type` or `check_equality` becomes a typed `Outcome::Refused(Refusal::CheckedInvariant { cause })`, retaining the original `IllTypedCause` in `GeneratedTypeCheckRejected` or `GeneratedEqualityCheckRejected` respectively, before any charge."
- id: TC-029
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: "`match` whose `Err` arm yields `EnvironmentError::Reconstruction` or `Outcome::Refused(Refusal::CheckedInvariant { cause })`"
- id: spec/oracle/matrix/tests.md
  path: spec/oracle/matrix/tests.md
  role: examined
  excerpt: "| FR-042 | FR-042-AC-1 | TC-057 | PLANNED (IR-709): seven of nine typed CG producers landed; two producers and TC-057 tagged verification remain open. QSL owns replay-consumer verification |"
```
