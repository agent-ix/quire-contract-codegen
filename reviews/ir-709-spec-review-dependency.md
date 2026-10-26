---
id: SR-582006
title: Dependency review of IR-709 generated producer blockers
type: SpecReview
analysis: dependency
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

Ticket: IR-709. PR: quire-contract-codegen#582 (draft, spec-only). Reviewed exact head `06d57194cb4d37437dc8e1ef9c7a15fcdc2d0395` against base `28e46843d3ec2b8b595995b6fdea0f43cd78e993`; four files, +74/-5. Method: spec-review/dependency. Reviewer model: Claude Opus (claude-opus-5-5), run 31f7dfea-4a50-4331-8441-50b879fc9fd0. Ticket and comment text was treated as data.

## Scope

```yaml
scope:
- id: FR-042-dep-IR-595
  path: spec/oracle/functional/FR-042-generated-checked-invariant-causes.md
  role: examined
  excerpt: '- [IR-595](https://linear.app/agent-ix/issue/IR-595) owns the QSL/CG direct owner cutover
    needed before the two planned generated-function paths can be implemented and exercised. The corresponding
    trigger wording in quire-exact FR-369''s assignment rows must be aligned with those paths before IR-709
    closes; its nine cause assignments remain required.'
- id: FR-042-dep-QSL-1105
  path: spec/oracle/functional/FR-042-generated-checked-invariant-causes.md
  role: examined
  excerpt: '- [QSL-1105](https://linear.app/agent-ix/issue/QSL-1105) owns exposure of checked nominal
    record/tuple declarations through the closure facade. Until that owner contract is available, CG cannot
    build or verify the `GeneratedEnvironmentRejected` path from owner declarations.'
- id: FR-042-B-producers
  path: spec/oracle/functional/FR-042-generated-checked-invariant-causes.md
  role: examined
  excerpt: 'For `GeneratedUnexpectedOutcome`, a generated function body SHALL check a real

    `Outcome::Completed(value)` against its selected callable''s checker-issued

    result type using the admitted QSV environment. A value that the declared

    type does not admit SHALL yield that cause at the body boundary; other outcome

    arms SHALL keep their own identity and charges. For

    `GeneratedEnvironmentRejected`, emitted function preparation SHALL build its

    QSV environment from the selected owner''s actual checked nominal

    declarations. A genuine declaration rejection SHALL yield that cause before

    the body runs; limit...'
- id: FR-021-AC-38
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: 'With a genuine selected callable whose checker-issued signature declares Boolean result, a
    reachable test mutation of its generated body returns `Completed(Value::Integer(...))`; the body reports
    `Refused(CheckedInvariant { cause: GeneratedUnexpectedOutcome })` rather than a completed value, preserving
    all charges already admitted up to that point. The unmutated body returns its admitted Boolean result;
    `Undefined`, `Refused` and `Incomplete` controls retain their original variants, payloads and charges.
    The comparison uses that selection''s `signature().result()` and the admitted environmen...'
- id: FR-042-AC-2
  path: spec/oracle/functional/FR-042-generated-checked-invariant-causes.md
  role: examined
  excerpt: The `GeneratedUnexpectedOutcome` producer executes a generated body that actually returns `Completed(value)`,
    compares the value against that selected callable's checker-issued result type via the admitted environment,
    and reports only a genuine mismatch as that cause. A reachable wrong-result generated-body mutation
    fails the completed-result control; `Undefined`, `Refused` and `Incomplete` retain their original
    outcomes and charges. PLANNED (IR-709 after IR-595).
- id: FR-042-status
  path: spec/oracle/functional/FR-042-generated-checked-invariant-causes.md
  role: context_only
  excerpt: 'PLANNED: Seven of the nine CG-owned typed cause producer paths are present in current source.
    After the IR-595 owner cutover, IR-709''s planned `GeneratedUnexpectedOutcome` path uses the checker-issued
    `CallableSelection::signature().result()` and QSV `TypeEnvironment::admits` for a real generated body''s
    `Completed(value)`. Its planned `GeneratedEnvironmentRejected` path requires QSL-1105 to expose the
    selected owner''s real nominal declaration members before CG can construct an environment and observe
    a genuine rejection. Neither producer exists yet; the current exhaustive outcome matches an...'
```

## Analysis

Linear structure was checked: QSL-1105 (Backlog) and IR-595 (In Progress) both carry a `blocks` edge to IR-709. QSL-1105's owner contract covers record, tuple and union declarations. The facade gap was measured independently in `qsl-replay/src/admitted.rs`. IR-709's own Linear state reads Done while this draft PR and its comments treat it as open. That is a tracker-hygiene item for the lead, not a spec finding.

### FND-001 (medium, confidence medium, soundness): FR-042-AC-2

FR-042 Behavior applies the check to every `Completed(value)` 'using the admitted QSV environment'. For a record, tuple, union or reference result, `admits` needs the same owner-complete nominal environment that QSL-1105 provides. AC-2 and AC-38 are marked only 'after IR-595', and their test uses a Boolean result, for which an empty environment suffices. The spec does not say which environment the check uses before QSL-1105, or whether nominal results are out of scope until then. That leaves either a hidden QSL-1105 dependency or the empty default environment that FR-042-AC-3 forbids. Scope AC-2 to non-nominal results until QSL-1105, or mark the nominal case blocked.

### FND-002 (low, confidence medium, other): FR-042-dep-IR-595

The PR redefines `GeneratedUnexpectedOutcome` as 'a completed value the declared result type does not admit'. quire-exact `src/outcome.rs` documents it as 'A generated outcome adapter received an unexpected arm', and the FR-369 row describes adapters with no unexpected arm. The dependency says the alignment must happen 'before IR-709 closes', but it names no ticket or edge. Link a quire-exact ticket, as IR-709's other blockers are linked.

## Verdict

**PASS WITH FINDINGS.** One medium dependency gap and one low untracked alignment.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The 'admitted QSV environment' for the unexpected-outcome check also needs owner nominal declarations for nominal result types, yet only the environment leg is marked QSL-1105-blocked | spec/oracle/functional/FR-042-generated-checked-invariant-causes.md:44 |
| FND-002 | low | FR-369 trigger-row and quire-exact rustdoc alignment for GeneratedUnexpectedOutcome has no owning ticket | spec/oracle/functional/FR-042-generated-checked-invariant-causes.md:51 |

## Dispositions

Round 1, checked against fix commit `9cc1a3e3` (exact head `9cc1a3e3b46cc4f2c99058e6cb9372a5accc9421`), full diff from base `28e46843`. Every outcome was verified against the pushed text, not taken from the ticket or brief.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | IR-1021 verified in Linear (Backlog) with a blocks edge to IR-709. |
