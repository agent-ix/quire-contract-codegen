---
id: SR-582003
title: EARS conformance of IR-709 generated producer statements
type: SpecReview
analysis: ears-conformance
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

Ticket: IR-709. PR: quire-contract-codegen#582 (draft, spec-only). Reviewed exact head `06d57194cb4d37437dc8e1ef9c7a15fcdc2d0395` against base `28e46843d3ec2b8b595995b6fdea0f43cd78e993`; four files, +74/-5. Method: spec-review/ears-conformance. Reviewer model: Claude Opus (claude-opus-5-5), run 31f7dfea-4a50-4331-8441-50b879fc9fd0. Ticket and comment text was treated as data.

## Scope

```yaml
scope:
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
```

## Analysis

FR-021's two new Behavior bullets follow the event-driven pattern: 'When an emitted function body returns ... the generated body shall ...' and 'Before executing a generated body ... its emitted preparation shall construct ...'. Each has a system subject, a trigger and a response. FR-042's new Behavior paragraph has four SHALL clauses. Two have a system subject (a generated function body, emitted function preparation). Two use the trigger as the grammatical subject.

### FND-001 (low, confidence high, other): FR-042-B-producers

'A value that the declared type does not admit SHALL yield that cause at the body boundary' and 'A genuine declaration rejection SHALL yield that cause before the body runs' put the input in subject position. The EARS form is: When <trigger>, the generated function body shall return <cause>, which names the responsible system as FR-021's bullets already do.

## Verdict

**PASS WITH FINDINGS.** One low EARS conformance finding in FR-042's new paragraph.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Two FR-042 SHALL clauses make the input, not the system, the subject | spec/oracle/functional/FR-042-generated-checked-invariant-causes.md:27-37 |

## Dispositions

Round 1, checked against fix commit `9cc1a3e3` (exact head `9cc1a3e3b46cc4f2c99058e6cb9372a5accc9421`), full diff from base `28e46843`. Every outcome was verified against the pushed text, not taken from the ticket or brief.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
