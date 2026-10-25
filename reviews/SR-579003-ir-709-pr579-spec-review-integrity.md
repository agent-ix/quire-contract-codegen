---
id: SR-579003
title: "IR-709 PR #579 spec-review/spec-integrity-analysis review"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen; PR #579 head 3fc48f7e33e4a99c2dd738862f794d1cb45e74de on base c62487bf; spec/oracle/functional/FR-018-composite-equality-oracles.md, spec/oracle/functional/FR-042-generated-checked-invariant-causes.md, spec/oracle/matrix/TC-029-composite-equality-oracles.md; cross-repo owner agent-ix/quire-exact FR-369 (origin/main); IR-709"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-018
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-042
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-369
    type: references
---

## Summary

Ticket: IR-709. This integrity sub-analysis checks that the FR-018, FR-042 and TC-029 cause
assignments agree with each other and with the owning cause table in quire-exact FR-369's second
table, which assigns CG's nine causes.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-018 assigns `GeneratedDescriptorReconstructionFailed` to the wrong trigger. The new Behavior bullet says that "when a reconstruction fails", the emitted oracle shall return this cause. FR-369 gives the cause a different trigger: a prepared descriptor with the wrong binding, expected package identity or context requirement, or a context with the wrong package identity. FR-369 also says that "descriptor reconstruction errors while preparing that environment retain their separate typed `EnvironmentError` path". The FR-018 paragraph that does specify FR-369's trigger (wrong private binding, mismatched retained `Some`, missing context for a nominal recipe) still says unit-style `Refused(CheckedInvariant)` and names no cause. The cause is therefore attached to a path the owner excludes, and missing from the path the owner assigns. | spec/oracle/functional/FR-018-composite-equality-oracles.md:604-606; spec/oracle/functional/FR-018-composite-equality-oracles.md:326-329 |
| FND-002 | low | The FR-042 Dependencies edit removes the former open item "alignment of the former unit-constructor text in FR-018 AC-8 and AC-17" and now says those two ACs name the typed payload. The alignment is not complete across FR-018 and TC-029. Besides FND-001, unit-style or family-only `CheckedInvariant` references remain at FR-018:328, FR-018:792 and TC-029:557 and 579, and no open item tracks them. | spec/oracle/functional/FR-042-generated-checked-invariant-causes.md:36; spec/oracle/functional/FR-018-composite-equality-oracles.md:792; spec/oracle/matrix/TC-029-composite-equality-oracles.md:557; spec/oracle/matrix/TC-029-composite-equality-oracles.md:579 |

## Verdict

Mergeable after FND-001 is fixed. The FR-042, TC-057 and `tests.md` owner split matches FR-369:
CG owns nine producers, QSL owns the replay consumer, and IR-709 owns the two open
assignments. The FR-369 table also says the two absent rows "remain required target
assignments". The FR-018 payload names for `GeneratedTypeCheckRejected` and
`GeneratedEqualityCheckRejected` agree with FR-369, including the nested `IllTypedCause`.

## Coverage

```yaml
scope:
- id: FR-018
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: "When a reconstruction fails, the emitted environment constructor shall return `EnvironmentError::Reconstruction` and the emitted oracle function shall return `Outcome::Refused(Refusal::CheckedInvariant { cause: CheckedInvariantCause::GeneratedDescriptorReconstructionFailed })`."
- id: FR-018 prepared-wrapper paragraph
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: "The oracle SHALL refuse a wrong private binding, a mismatched retained `Some`, or missing context for a nominal recipe as `Refused(CheckedInvariant)` before any meter charge."
- id: FR-042
  path: spec/oracle/functional/FR-042-generated-checked-invariant-causes.md
  role: examined
  excerpt: "[FR-018](./FR-018-composite-equality-oracles.md) AC-8 and AC-17 name the required typed `Refusal::CheckedInvariant { cause }` payload at their producer paths."
- id: TC-029
  path: spec/oracle/matrix/TC-029-composite-equality-oracles.md
  role: examined
  excerpt: "a foreign environment refuses as `CheckedInvariant` without panicking and without charging, decided by the emitted `check_type` calls"
- id: quire-exact FR-369
  path: "agent-ix/quire-exact spec FR-369 (origin/main)"
  role: context_only
  excerpt: "`GeneratedDescriptorReconstructionFailed` | A generated equality oracle receives a prepared descriptor with the wrong binding, expected package identity or context requirement, or an admitted context with the wrong package identity ... Descriptor reconstruction errors while preparing that environment retain their separate typed `EnvironmentError` path."
```
