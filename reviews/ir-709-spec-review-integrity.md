---
id: SR-582004
title: Integrity review of IR-709 generated producer ACs and verification trace
type: SpecReview
analysis: integrity
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

Ticket: IR-709. PR: quire-contract-codegen#582 (draft, spec-only). Reviewed exact head `06d57194cb4d37437dc8e1ef9c7a15fcdc2d0395` against base `28e46843d3ec2b8b595995b6fdea0f43cd78e993`; four files, +74/-5. Method: spec-review/integrity. Reviewer model: Claude Opus (claude-opus-5-5), run 31f7dfea-4a50-4331-8441-50b879fc9fd0. Ticket and comment text was treated as data.

## Scope

```yaml
scope:
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
- id: FR-042-AC-2
  path: spec/oracle/functional/FR-042-generated-checked-invariant-causes.md
  role: examined
  excerpt: The `GeneratedUnexpectedOutcome` producer executes a generated body that actually returns `Completed(value)`,
    compares the value against that selected callable's checker-issued result type via the admitted environment,
    and reports only a genuine mismatch as that cause. A reachable wrong-result generated-body mutation
    fails the completed-result control; `Undefined`, `Refused` and `Incomplete` retain their original
    outcomes and charges. PLANNED (IR-709 after IR-595).
- id: FR-042-AC-3
  path: spec/oracle/functional/FR-042-generated-checked-invariant-causes.md
  role: examined
  excerpt: The `GeneratedEnvironmentRejected` producer uses only real nominal declarations supplied by
    the selected checked-closure owner; a genuine QSV construction rejection yields that cause before
    body execution while a successful construction does not. The rejection's original typed detail remains
    observable in diagnostics; limit and cancellation failures are not relabelled. An empty default environment,
    invented declaration or forced failure gives no acceptance credit. PLANNED (IR-709 after IR-595; blocked
    by QSL-1105).
- id: TC-057-description
  path: spec/oracle/matrix/TC-057-generated-checked-invariant-causes.md
  role: examined
  excerpt: Planned owner control for FR-042-AC-1's IR-709 generated producer migration.
- id: TC-057-step-2
  path: spec/oracle/matrix/TC-057-generated-checked-invariant-causes.md
  role: examined
  excerpt: "2. After IR-595, select an authentic admitted callable with a Boolean result\n   from the\
    \ FR-485 facade. Register a reachable generated body with that\n   selection and use a test mutation\
    \ of the emitted body to return\n   `Completed(Value::Integer(...))`. Assert that the body compares\
    \ against\n   that selection's `signature().result()` via the admitted QSV environment\n   and reports\
    \ `GeneratedUnexpectedOutcome`. Run the unmutated Boolean body\n   and separate `Undefined`, `Refused`\
    \ and `Incomplete` controls; assert that\n   only the wrong completed value changes cause and that\
    \ admitted charges a..."
- id: TC-057-step-3
  path: spec/oracle/matrix/TC-057-generated-checked-invariant-causes.md
  role: examined
  excerpt: "3. Once QSL-1105 exposes the selected owner's record/tuple members, use a\n   checker-admitted\
    \ source with real nominal declarations to exercise the\n   emitted function's QSV environment construction.\
    \ Record the declaration\n   keys, forms, ordered members, presence and types read from that checked\n\
    \   owner, and verify that the emitted constructor consumes those exact\n   declarations. Exercise\
    \ a genuine construction rejection through the\n   emitted path and assert `GeneratedEnvironmentRejected`,\
    \ the original typed\n   diagnostic and zero body operation/charge. A successful construction and\n\
    \   ..."
- id: tests.md-TC-057
  path: spec/oracle/matrix/tests.md
  role: examined
  excerpt: '| TC-057 | Check typed checked-invariant causes from generated oracles | Integration | P0
    | FR-042-AC-1 | 🚧 Planned (IR-709): generated-oracle producer controls remain open |'
- id: FR-042-AC-1
  path: spec/oracle/functional/FR-042-generated-checked-invariant-causes.md
  role: context_only
  excerpt: CG's native Boolean, equality, function and pre-check placeholder constructors use the exact
    nine CG-owned causes in the table, retaining `IllTypedCause` from `check_type` and `check_equality`;
    downstream replay consumers preserve the typed payload or match it explicitly, retain charge/failure
    behavior, and accept neither the former unit constructor nor an invented default.
```

## Analysis

`quire validate` passes on all four changed files at the reviewed head. FR-021 Behavior, FR-021 AC-38/39, FR-042 Behavior, FR-042 AC-2/3, TC-057 steps 2-3 and FR-018's cross-reference agree with each other on mechanism, ordering and blockers. Trace completeness was checked against the unchanged matrix index and TC frontmatter.

### FND-001 (medium, confidence high, trace): TC-057-description

FR-021-AC-38/39 cite 'Test (TC-031, TC-057)', but TC-057's `relationships` verify only FR-042, and its Description still reads 'Planned owner control for FR-042-AC-1'. TC-031 has no step for AC-38/39. In `spec/oracle/matrix/tests.md`, the FR-021 rows stop at AC-37, the FR-042 row lists only AC-1, and the TC-031 and TC-057 rows list neither the new FR-021 nor the new FR-042 ACs. The hand index and the TC trace now under-report four planned obligations. Add FR-021 as a verified target of TC-057 (or TC-031 steps), and add the AC rows to the index.

### FND-002 (low, confidence medium, compound): FR-021-AC-38

One AC row asserts the wrong-result refusal, charge preservation, the unmutated success, the Undefined, Refused and Incomplete pass-through, and that the type comes from `signature().result()`. A partial pass cannot be recorded. Split pass-through and provenance into their own ACs, or accept it knowingly.

## Verdict

**PASS WITH FINDINGS.** The content is consistent across files. The four new ACs are missing from the verification trace (medium), and one AC is compound (low).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | New FR-021-AC-38/39 and FR-042-AC-2/3 are absent from TC-057's frontmatter/description, TC-031 and the matrix index | spec/oracle/matrix/TC-057-generated-checked-invariant-causes.md:1-13 |
| FND-002 | low | FR-021-AC-38 bundles mutated, unmutated, three outcome-arm controls and result-type provenance in one AC | spec/oracle/functional/FR-021-function-application-oracles.md:401 |
