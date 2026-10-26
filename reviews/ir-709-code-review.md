---
id: SR-582001
title: Code review of IR-709 generated producer contracts against owner APIs
type: SpecReview
analysis: code-review
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

Ticket: IR-709. PR: quire-contract-codegen#582 (draft, spec-only). Reviewed exact head `06d57194cb4d37437dc8e1ef9c7a15fcdc2d0395` against base `28e46843d3ec2b8b595995b6fdea0f43cd78e993`; four files, +74/-5. Method: code-review. Reviewer model: Claude Opus (claude-opus-5-5), run 31f7dfea-4a50-4331-8441-50b879fc9fd0. Ticket and comment text was treated as data.

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
- id: FR-018-env-bullet
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: "- FR-021's planned emitted function environment is a separate construction\n  path: it shall\
    \ consume the selected checked closure owner's actual nominal\n  declarations through the owner facade\
    \ and map a genuine construction\n  rejection to FR-042's `GeneratedEnvironmentRejected`. This does\
    \ not change\n  this equality item's generation-time `EnvironmentError::Declaration`\n  contract,\
    \ and is blocked by QSL-1105 until the facade exposes record/tuple\n  members."
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
- id: FR-042-status
  path: spec/oracle/functional/FR-042-generated-checked-invariant-causes.md
  role: examined
  excerpt: 'PLANNED: Seven of the nine CG-owned typed cause producer paths are present in current source.
    After the IR-595 owner cutover, IR-709''s planned `GeneratedUnexpectedOutcome` path uses the checker-issued
    `CallableSelection::signature().result()` and QSV `TypeEnvironment::admits` for a real generated body''s
    `Completed(value)`. Its planned `GeneratedEnvironmentRejected` path requires QSL-1105 to expose the
    selected owner''s real nominal declaration members before CG can construct an environment and observe
    a genuine rejection. Neither producer exists yet; the current exhaustive outcome matches an...'
```

## Analysis

Spec-only diff; no Rust changed, so rust-review and gap-analysis do not apply. Each API the four files name was checked against current owner `origin/main` source: quire-eval `a464a64` `Signature::result() -> &ValueType` and `Body::resume -> Result<Transition, InternalFault>`; QSL `395de2cde` `qsl-replay/src/application.rs` `CallableSelection::signature()` (built from the checked function's parameters and result) and `AdmittedComposite`, which exposes `union_members()` but no record/tuple members, so the QSL-1105 blocker is real; QSV `0a365db` `TypeEnvironment::admits(&ValueType, &Value) -> Result<bool, EnvironmentFailure>` (no work charged) and `EnvironmentFailure::{Limit, Refused, Allocation, Capacity}`; quire-exact `4255499` `Outcome` four arms, `CheckedInvariantCause::{GeneratedUnexpectedOutcome, GeneratedEnvironmentRejected}` and `Value::Integer`. Each named symbol exists with the shape the spec uses. The FR-018 bullet keeps the equality `EnvironmentError::Declaration` contract unchanged, which matches the quire-exact FR-369 row. One finding remains: the spec does not reconcile its producer with quire-eval's body-fault convention.

### FND-001 (medium, confidence low, soundness): FR-021-B-unexpected

quire-eval `Body::resume` returns `Result<Transition, InternalFault>`, and its docs reserve `Err` for a body's own broken invariant. The scheduler likewise turns a broken body request (`child-request-not-admitted`) into `Err(fault)`, and `CallFailure::Fault` says such a fault is never surfaced as an `Evaluation` whose outcome is `Outcome::Refused`. A generated body returning a value its own checked signature does not admit is that kind of fault. The bullet instead specifies `Done(Evaluation{Refused(CheckedInvariant{GeneratedUnexpectedOutcome})})` and names no `Evaluation::location` for it. The choice may be right, since it matches the seven existing in-body CheckedInvariant producers, but the spec should state why it is not an `InternalFault` and which location it carries, so the implementer does not have to pick between two owner conventions.

## Verdict

**PASS WITH FINDINGS.** Every API the PR names exists on current owner main with the shape the spec uses, and the charge boundary agrees with QSV `admits` charging nothing. One medium finding (low confidence) on the body-fault boundary.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Wrong-result producer is specified as Outcome::Refused without reconciling quire-eval's Body contract, which reports a body's own broken invariant as Err(InternalFault) | spec/oracle/functional/FR-021-function-application-oracles.md:335-345 |
