---
id: SR-582002
title: Spec review of IR-709 two genuine generated cause paths
type: SpecReview
analysis: base
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

Ticket: IR-709. PR: quire-contract-codegen#582 (draft, spec-only). Reviewed exact head `06d57194cb4d37437dc8e1ef9c7a15fcdc2d0395` against base `28e46843d3ec2b8b595995b6fdea0f43cd78e993`; four files, +74/-5. Method: spec-review. Reviewer model: Claude Opus (claude-opus-5-5), run 31f7dfea-4a50-4331-8441-50b879fc9fd0. Ticket and comment text was treated as data.

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
- id: FR-042-AC-1
  path: spec/oracle/functional/FR-042-generated-checked-invariant-causes.md
  role: examined
  excerpt: CG's native Boolean, equality, function and pre-check placeholder constructors use the exact
    nine CG-owned causes in the table, retaining `IllTypedCause` from `check_type` and `check_equality`;
    downstream replay consumers preserve the typed payload or match it explicitly, retain charge/failure
    behavior, and accept neither the former unit constructor nor an invented default.
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
- id: FR-042-status
  path: spec/oracle/functional/FR-042-generated-checked-invariant-causes.md
  role: examined
  excerpt: 'PLANNED: Seven of the nine CG-owned typed cause producer paths are present in current source.
    After the IR-595 owner cutover, IR-709''s planned `GeneratedUnexpectedOutcome` path uses the checker-issued
    `CallableSelection::signature().result()` and QSV `TypeEnvironment::admits` for a real generated body''s
    `Completed(value)`. Its planned `GeneratedEnvironmentRejected` path requires QSL-1105 to expose the
    selected owner''s real nominal declaration members before CG can construct an environment and observe
    a genuine rejection. Neither producer exists yet; the current exhaustive outcome matches an...'
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
- id: TC-057-expected
  path: spec/oracle/matrix/TC-057-generated-checked-invariant-causes.md
  role: examined
  excerpt: All nine genuine CG producer paths retain their table causes and original nested payloads without
    changing charge or failure behavior; a reachable wrong-result body and a real owner-declaration construction
    rejection are shown, and no fabricated failure is counted as evidence. The environment-rejection leg
    remains PLANNED and blocked by QSL-1105 until its owner contract and a reachable source are verified.
- id: FR-042-B-producers
  path: spec/oracle/functional/FR-042-generated-checked-invariant-causes.md
  role: context_only
  excerpt: 'For `GeneratedUnexpectedOutcome`, a generated function body SHALL check a real

    `Outcome::Completed(value)` against its selected callable''s checker-issued

    result type using the admitted QSV environment. A value that the declared

    type does not admit SHALL yield that cause at the body boundary; other outcome

    arms SHALL keep their own identity and charges. For

    `GeneratedEnvironmentRejected`, emitted function preparation SHALL build its

    QSV environment from the selected owner''s actual checked nominal

    declarations. A genuine declaration rejection SHALL yield that cause before

    the body runs; limit...'
```

## Analysis

Checked against the brief's preservation requirements. FR-042-AC-1 is byte-identical to the base, and all nine FR-369 CG causes stay required. Every new AC and the TC-057 environment leg say PLANNED, and the TC-057 Expected Results and step 3 refuse acceptance credit without a reachable source, so no execution credit is claimed falsely and no AC is narrowed. The unexpected-outcome path uses the checker-issued `signature().result()` and QSV `admits`, with outcome-arm and charge preservation; it is coherent and implementable after IR-595. The environment-rejection path uses real owner declarations and carries the explicit QSL-1105 blocker. Reachability was then checked: QSL `qsl-semantics/src/check/assemble.rs` `admit_types` already runs `TypeEnvironment::bounded_with_cancel` over the package's declarations and refuses the package on `EnvironmentFailure::Refused`.

### FND-001 (high, confidence medium, untestable-ac): FR-021-AC-39

The QSL checker (`assemble.rs` `admit_types`, `TypeEnvironment::bounded_with_cancel`) refuses a package whose declarations QSV refuses. AC-39 requires CG to consume that owner's exact keys, forms, ordered members, presence and types, and forbids synthetic declarations and forced errors. QSV admission is deterministic, so the same declarations under the default limits cannot give `EnvironmentFailure::Refused`. Limit and cancellation failures are excluded from the cause. That leaves no conforming trigger. TC-057 step 3 admits the step may stay unverified, but FR-042 Status says an unreachable branch is not a producer path, and FR-042-AC-1 requires all nine. The cause would then be either fabricated or permanently blocked, and IR-709 could not close. The spec must either name a reachable genuine rejection (for example, own and imported owner declarations combined in a closure the checker never admitted as one environment) or record an owner decision on how this cause is satisfied.

### FND-002 (medium, confidence medium, ambiguous): FR-021-AC-38

A generated body from checked source cannot complete with a value its own signature rejects, so the trigger exists only under mutation. The AC and TC-057 step 2 do not say what a permitted mutation is: edited emitted source text recompiled in the test, a generator-side test hook, or a cfg-gated body. They also do not say why it is not the 'fabricated failure' that FR-042 Status and TC-057 step 1 exclude. Two implementers would differ, and a production-reachable test hook would weaken the generated body. State that the mutation is applied to the emitted source in the test only, that the unmutated production body contains the check, and that this counts as genuine because the executed check is the shipped one.

## Verdict

**FAIL (high).** The unexpected-outcome contract is sound apart from one ambiguity. The environment-rejection ACs require a genuine `Refused` rejection from exactly the declarations the checker already admitted, and the PR does not show that any conforming implementation can reach one. That needs an owner decision before merge, not a wording fix.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | A genuine declaration rejection from a checker-admitted source's exact owner declarations appears unreachable, so AC-39/FR-042-AC-3 and hence FR-042-AC-1 may never pass | spec/oracle/functional/FR-021-function-application-oracles.md:402 |
| FND-002 | medium | 'Reachable test mutation of its generated body' is undefined and sits against the 'fabricated failure gives no credit' exclusion | spec/oracle/functional/FR-021-function-application-oracles.md:401 |
