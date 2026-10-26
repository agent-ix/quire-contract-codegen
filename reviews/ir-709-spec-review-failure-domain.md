---
id: SR-582005
title: Failure-domain review of IR-709 generated producer paths
type: SpecReview
analysis: failure-domain
scope: 'agent-ix/quire-contract-codegen; PR #582 head 06d57194cb4d37437dc8e1ef9c7a15fcdc2d0395 vs base
  28e46843d3ec2b8b595995b6fdea0f43cd78e993; spec/oracle/functional/FR-018-composite-equality-oracles.md,
  spec/oracle/functional/FR-021-function-application-oracles.md, spec/oracle/functional/FR-042-generated-checked-invariant-causes.md,
  spec/oracle/matrix/TC-057-generated-checked-invariant-causes.md'
review_set: subset
date: '2026-10-26'
relationships:
- target: ix://agent-ix/quire-contract-codegen/FR-021
  type: references
- target: ix://agent-ix/quire-contract-codegen/FR-042
  type: references
- target: ix://agent-ix/quire-contract-codegen/FR-018
  type: references
- target: ix://agent-ix/quire-contract-codegen/TC-057
  type: references
---

## Summary

Ticket: IR-709. PR: quire-contract-codegen#582 (draft, spec-only). Reviewed exact head `06d57194cb4d37437dc8e1ef9c7a15fcdc2d0395` against base `28e46843d3ec2b8b595995b6fdea0f43cd78e993`; four files, +74/-5. Method: spec-review/failure-domain. Reviewer model: Claude Opus (claude-opus-5-5), run 31f7dfea-4a50-4331-8441-50b879fc9fd0. Ticket and comment text was treated as data.

## Scope

```yaml
scope:
- id: FR-021-B-unexpected
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: "- When an emitted function body returns `Outcome::Completed(value)`, the\n  generated body\
    \ shall compare that value with the same checker-issued\n  `CallableSelection::signature().result()`\
    \ supplied at registration, using\n  the admitted QSV type environment's `admits` operation. If the\
    \ value is not\n  admitted by the declared result type, it shall return\n  `Outcome::Refused(Refusal::CheckedInvariant\
    \ { cause:\n  CheckedInvariantCause::GeneratedUnexpectedOutcome })` at that body boundary.\n  It shall\
    \ forward `Undefined`, `Refused` and `Incomplete` without relabelling\n  them or changing admitted\
    \ ch..."
- id: FR-021-B-environment
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: "- Before executing a generated body that needs nominal types, its emitted\n  preparation shall\
    \ construct the QSV type environment from the actual\n  checker-owned nominal declarations of the\
    \ selected checked closure. If\n  that construction returns a declaration rejection, the generated\
    \ body shall\n  return `Outcome::Refused(Refusal::CheckedInvariant { cause:\n  CheckedInvariantCause::GeneratedEnvironmentRejected\
    \ })` before any body\n  operation or body charge, retaining the original construction rejection in\n\
    \  diagnostic evidence. Resource exhaustion and cancellation retain their own\n  typed fai..."
- id: FR-021-AC-38
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: 'With a genuine selected callable whose checker-issued signature declares Boolean result, a
    reachable test mutation of its generated body returns `Completed(Value::Integer(...))`; the body reports
    `Refused(CheckedInvariant { cause: GeneratedUnexpectedOutcome })` rather than a completed value, preserving
    all charges already admitted up to that point. The unmutated body returns its admitted Boolean result;
    `Undefined`, `Refused` and `Incomplete` controls retain their original variants, payloads and charges.
    The comparison uses that selection''s `signature().result()` and the admitted environmen...'
- id: FR-021-AC-39
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: 'For an emitted function with real checker-owned nominal declarations, the emitted preparation
    consumes the selected closure''s exact declaration keys, forms, ordered members, presence and types;
    a genuine QSV environment-construction rejection from that path yields `Refused(CheckedInvariant {
    cause: GeneratedEnvironmentRejected })` before body operation or body charge, with the original rejection
    observable in diagnostic evidence. A successful construction and a non-nominal control do not yield
    that cause; resource exhaustion and cancellation keep their distinct typed failures. The test uses...'
- id: FR-042-AC-3
  path: spec/oracle/functional/FR-042-generated-checked-invariant-causes.md
  role: examined
  excerpt: The `GeneratedEnvironmentRejected` producer uses only real nominal declarations supplied by
    the selected checked-closure owner; a genuine QSV construction rejection yields that cause before
    body execution while a successful construction does not. The rejection's original typed detail remains
    observable in diagnostics; limit and cancellation failures are not relabelled. An empty default environment,
    invented declaration or forced failure gives no acceptance credit. PLANNED (IR-709 after IR-595; blocked
    by QSL-1105).
```

## Analysis

Failure modes were enumerated against the owner APIs: QSV `admits` returns `Result<bool, EnvironmentFailure>`. `TypeEnvironment::bounded_with_cancel` reports cancellation as `EnvironmentFailure::Limit(WorkUnits)` ('which the caller that owns the handle reads as its cancellation'), and allocation and capacity stops are separate variants. The charge boundary is fine: `admits` charges no work, and construction uses its own work budget rather than the session `Meter`.

### FND-001 (medium, confidence high, other): FR-021-B-unexpected

QSV `TypeEnvironment::admits` returns `Result<bool, EnvironmentFailure>`, and a refused worklist reservation is an `Err`, never `false`. The bullet defines only the admitted and not-admitted cases. An implementer could map `Err` to `GeneratedUnexpectedOutcome`, which would falsely blame the body, or swallow it. quire-eval reports the parallel argument-admission storage failure as `CallFailure::Storage`. State the mapping and keep it distinct from the cause.

### FND-002 (medium, confidence high, untestable-ac): FR-021-AC-39

The AC and the Behavior bullet say only that limit, cancellation and resource failures 'retain their own typed failure paths'. They never name the result: `Incomplete`, `Err(InternalFault)` or a CG diagnostic. Any non-`GeneratedEnvironmentRejected` result therefore passes. QSV reports cancellation as `EnvironmentFailure::Limit(WorkUnits)`, so cancellation can be told apart from a real work limit only by reading the cancel handle. `Allocation` and `Capacity` are not mentioned. Name the trigger as exactly `EnvironmentFailure::Refused(InvalidDeclaration)`, and give the mapping for `Limit` (cancelled or not), `Allocation` and `Capacity`.

## Verdict

**PASS WITH FINDINGS.** Two medium unstated-failure-mode findings.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | No disposition for admits() returning Err(EnvironmentFailure) on the completed-result check | spec/oracle/functional/FR-021-function-application-oracles.md:335-345 |
| FND-002 | medium | 'Resource exhaustion and cancellation keep their distinct typed failures' names no target, and QSV reports cancellation as a WorkUnits limit | spec/oracle/functional/FR-021-function-application-oracles.md:402 |

## New findings (disposition pass 1)

Checked at head `9cc1a3e3b46cc4f2c99058e6cb9372a5accc9421`.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | medium | TC-057 requires Allocation, Capacity and admits-reservation-failure controls that no public QSV API can produce without injection | spec/oracle/matrix/TC-057-generated-checked-invariant-causes.md:35-37, 52-55 |

### FND-003 (medium, confidence high, untestable-ac): TC-057-step-3

TC-057 step 2 says 'Probe QSV `admits` reservation failure', and step 3 says 'Exercise `Limit(WorkUnits)` ... plus `Allocation` and `Capacity`'. On QSV origin/main 0a365db, `TypeEnvironment::admits` and `new`/`bounded`/`bounded_with_cancel` always use the private `GlobalReservations` policy. `bounded_with_reservations` and `admits_checked` are not public. The only public reservation-injection entry point is `admit_supplied_with_reservation`, which is supplied-argument admission, not either call this TC names. So `Allocation` and `Capacity` arise only from genuine allocator denial or size overflow. The steps therefore cannot be run without an injection hook, which is the kind of 'injected constructor error' the same TC forbids, or without CG entering QSV internals. `Limit(WorkUnits)`, tripped or untripped, is reachable through `TypeEnvironmentLimits` and `Cancel`. Fix: scope the storage controls to an owner-provided public injection seam (add it to QSL-1107 or a QSV ticket), or mark them inspection-only until one exists.

## Dispositions

Round 1, checked against fix commit `9cc1a3e3` (exact head `9cc1a3e3b46cc4f2c99058e6cb9372a5accc9421`), full diff from base `28e46843`. Every outcome was verified against the pushed text, not taken from the ticket or brief.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Non-conflation is now normative; exact leaf mapping is explicitly PLANNED on filed blocker QSL-1107 (Linear: blocks IR-709). |
| FND-002 | fixed | - |
