---
id: SR-576003
title: "IR-709 PR #576 spec-review/spec-ears-analysis review"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-codegen; PR #576 head 5188f6f45447ce2d33024e1c55c3813ddb692daf on base b60b18ef; Behavior statement and FR-042-AC-1 of spec/oracle/functional/FR-042-generated-checked-invariant-causes.md, FR-018-AC-8 and FR-018-AC-17 of spec/oracle/functional/FR-018-composite-equality-oracles.md; IR-709"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-042
    type: reviews
---

## Summary

This is the EARS conformance sub-analysis of the SHALL statements and acceptance criteria that PR #576 edits. Ticket: IR-709. FR-018's edits change only the payload spelling inside statements whose EARS shape is unchanged. The FR-042 Behavior statement is a ubiquitous SHALL and is unchanged in form.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-042-AC-1 was already compound: producer use, downstream consumer preservation, and charge/failure behaviour. The PR appends a further obligation on test artefacts ("The emitted sources and TC-057 controls enumerate every one of those seven cause paths"), which is a verification-plan statement rather than an observable system response. The qualifier "with genuine producer paths" in both Behavior and AC-1 is also an unbounded state condition with no defined test. Naming the seven causes (SR-576001 FND-002) removes the qualifier. Move the TC-057 enumeration to TC-057 or the Verification column. | spec/oracle/functional/FR-042-generated-checked-invariant-causes.md:25,31 |

## Verdict

EARS alone: mergeable, with one low finding. Checked clean: the FR-018 Behavior "becomes a typed ... before any charge" response stays a single event-driven response. The AC-8 and AC-17 edits change only the payload spelling and introduce no new trigger or response.

## Dispositions

Round 1, checked against the repository at head f6ca73d22bc3b7b9d2569fc85819c90b1689ecfd.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The test-artefact obligation and the 'with genuine producer paths' qualifier are removed from FR-042 Behavior and AC-1. The enumeration and the no-fabrication rule moved to TC-057 step 1 and Expected Results. AC-1's remaining producer/consumer/charge compound is the text already on main and is unchanged. |

## Coverage

```yaml
scope:
- id: FR-042
  path: spec/oracle/functional/FR-042-generated-checked-invariant-causes.md
  role: examined
  excerpt: "The code generator SHALL use the seven CG-owned causes with genuine producer paths in the second cause table of quire-exact FR-369 at their owning constructors."
- id: FR-042-AC-1
  path: spec/oracle/functional/FR-042-generated-checked-invariant-causes.md
  role: examined
  excerpt: "The emitted sources and TC-057 controls enumerate every one of those seven cause paths; no cause is counted from an unreachable branch or a fabricated failure."
- id: FR-018-AC-8
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: "yields `Outcome::Refused(Refusal::CheckedInvariant { cause: CheckedInvariantCause::GeneratedTypeCheckRejected { cause: error.cause } })` with the original `IllTypedCause` and no charge admitted, rather than a panic or a completed Boolean."
- id: FR-018-AC-17
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: "every call of one is followed by `?` or a `match` whose `Err` arm yields `EnvironmentError::Reconstruction` or `Outcome::Refused(Refusal::CheckedInvariant { cause })`"
```
