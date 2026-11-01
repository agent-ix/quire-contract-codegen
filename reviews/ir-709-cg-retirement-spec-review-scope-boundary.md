---
id: SR-202610310139
title: IR-709 CG placeholder retirement spec-review/scope-boundary review
type: SpecReview
analysis: scope-boundary
scope: 'agent-ix/quire-contract-codegen; IR-709 PR #695 head 9a3f5ed09325aa8c7b33553e61518dfe1a681934;
  FR-042 and changed TC-031/TC-057/matrix rows'
review_set: subset
relationships:
- target: ix://agent-ix/quire-contract-codegen/FR-042
  type: references
---

## Summary

Ticket: IR-709. The four-file specification-only diff retires the removed placeholder allocation, preserves the other eight assignments, and claims no producer execution. No Cargo, Kani or mutation run was performed.

## Verdict

CONDITIONAL: one low scope wording finding.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-042 Description still includes the retired pre-check placeholder in the current constructor scope; remove it to agree with the retirement in Behavior and AC-1. | spec/oracle/functional/FR-042-generated-checked-invariant-causes.md:13 |

## Scope

- FR-042 (examined, spec/oracle/functional/FR-042-generated-checked-invariant-causes.md): The code generator SHALL assign the shared quire-exact typed checked-invariant cause at each generated Boolean, equality, function and pre-check placeholder constructor.
- FR-042-retirement (examined, spec/oracle/functional/FR-042-generated-checked-invariant-causes.md): `GeneratedBodyPlaceholderInvoked` is retired: IR-595 replaced the pre-check placeholder with concrete registered bodies, so there is no remaining producer to allocate. The generator SHALL NOT restore a placeholder or reallocate this name to another trigger. An unexpected body resume remains a genuine `InternalFault`; it SHALL NOT be relabelled as a checked-invariant refusal. The eight remaining cause assignments and their original triggers are unchanged.
- FR-042-AC-1 (examined, spec/oracle/functional/FR-042-generated-checked-invariant-causes.md): CG's native Boolean, equality, function constructors use the exact eight CG-owned causes in the table, retaining `IllTypedCause` from `check_type` and `check_equality`; downstream replay consumers preserve the typed payload or match it explicitly, retain charge/failure behavior, and accept neither the former unit constructor nor an invented default.
- FR-042-status (examined, spec/oracle/functional/FR-042-generated-checked-invariant-causes.md): PLANNED: Six of the eight CG-owned typed cause producer paths are present in current source. After the IR-595 owner cutover, IR-709's planned `GeneratedUnexpectedOutcome` path uses the checker-issued `CallableSelection::signature().result()` and QSV `TypeEnvironment::admits` for a real generated body's `Completed(value)`. Its scalar Boolean test can run without nominal declarations; nominal results also wait for QSL-1105. Its planned `GeneratedEnvironmentRejected` path requires QSL-1105 to expose the selected owner's real nominal declaration members. An unmutated healthy body submits each once
- TC-031-step-12 (examined, spec/oracle/matrix/TC-031-function-application-oracles.md): controls and all eight-cause comparison.
- TC-057-step-1 (examined, spec/oracle/matrix/TC-057-generated-checked-invariant-causes.md): 1. Confirm the retired placeholder is absent and unexpected body resumes retain their fault channel. Exercise every generated Boolean, equality, function checked-invariant constructor listed in the second cause table of quire-exact FR-369. Inspect the emitted source for each of the eight named paths; do not count an unreachable branch or a fabricated failure as evidence.
- TC-057-expected-results (examined, spec/oracle/matrix/TC-057-generated-checked-invariant-causes.md): All eight CG producer guards retain their table causes and original nested payloads without changing charge or failure behavior; test-only emitted-source mutations reach the shipped wrong-result and owner-declaration construction guards without injecting a failure. The environment-rejection leg remains PLANNED and blocked by QSL-1105 and QSL-1107 until its owner contracts and the healthy/mutant controls are verified.
- FR-021-AC-38..40-matrix (examined, spec/oracle/matrix/tests.md): | FR-021 | FR-021-AC-38 through FR-021-AC-40 | TC-031, TC-057 | 🚧 Planned (IR-709 after IR-595): generated result check and genuine nominal environment constructor; scalar Boolean mutation control is specified, while nominal declarations await QSL-1105 and body-resource failures await QSL-1107. No producer execution or eight-cause credit claimed |
- FR-042-AC-1-matrix (examined, spec/oracle/matrix/tests.md): | FR-042 | FR-042-AC-1 | TC-057 | 🚧 PLANNED (IR-709): six of eight typed CG producer paths are present; genuine paths for `GeneratedUnexpectedOutcome` and `GeneratedEnvironmentRejected` and TC-057 tagged verification remain open. QSL owns replay-consumer verification |
- FR-042-AC-2 (context_only, spec/oracle/functional/FR-042-generated-checked-invariant-causes.md): The `GeneratedUnexpectedOutcome` producer executes a generated body that returns `Completed(value)`, compares that value against its selected callable's checker-issued result type via QSV `admits`, and reports only `Ok(false)` as this cause at the result location. A test-only emitted-source mutation supplies a reachable wrong completed value to the unchanged shipped check; `Undefined`, `Refused` and `Incomplete` retain their original outcomes and charges, and `Err(EnvironmentFailure)` is never a mismatch. Scalar Boolean mismatch proof is PLANNED after IR-595; nominal result proof also waits fo
- FR-042-AC-3 (context_only, spec/oracle/functional/FR-042-generated-checked-invariant-causes.md): The `GeneratedEnvironmentRejected` producer uses only real nominal declarations supplied by the selected checked-closure owner, once each in unmutated production source. A test-only mutation of emitted pre-body assembly submits one unchanged owner declaration twice to the shipped QSV constructor; exactly its genuine duplicate-key `EnvironmentFailure::Refused(InvalidDeclaration)` yields this cause before body execution at the declaration occurrence, with the typed rejection retained in diagnostic evidence. Successful construction does not yield it; `Limit`, `Allocation` and `Capacity` are never

## Method and limitations

No applicable AssuranceProfile was found under spec/. The authorizing request selects a scoped direct review; no enforced workflow was required. The existing six production constructor identities and the retained unexpected-resume InternalFault channel were inspected as context only. Remaining producer implementation and runtime qualification are outside this specification-only diff. No production files change. Rust-review and gap-analysis are inapplicable to this Markdown-only diff; unrelated dependency, evidence, risk and failure-domain lenses were not selected.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Inspected 06b81b1988a36b93f6531190583ab4e1f780e0e6: Description now lists only generated Boolean, equality and function constructors. |

After excerpt: The code generator SHALL assign the shared quire-exact typed checked-invariant cause at each generated Boolean, equality and function constructor.

### Final disposition verdict

PASS at `06b81b1988a36b93f6531190583ab4e1f780e0e6`. The original finding is preserved above; no new findings.
