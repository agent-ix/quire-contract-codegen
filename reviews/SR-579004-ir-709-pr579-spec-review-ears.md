---
id: SR-579004
title: "IR-709 PR #579 spec-review/spec-ears-analysis review"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-codegen; PR #579 head 3fc48f7e33e4a99c2dd738862f794d1cb45e74de on base c62487bf; changed Behavior statements of spec/oracle/functional/FR-018-composite-equality-oracles.md; IR-709"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-018
    type: reviews
---

## Summary

Ticket: IR-709. This EARS-conformance check covers the requirement statements the PR edits: two
FR-018 Behavior bullets and the FR-018 prose payload sentence. The FR-042 edits are Status and
Dependencies prose, not requirement statements. `make spec` reported no EARS warning in any
changed file.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Clean. The reconstruction bullet keeps its event form ("When a reconstruction fails, the emitted
environment constructor shall ..."). The IllTyped bullet keeps its subject-and-shall form. The
payload edits do not change either statement's pattern. Whether the reconstruction bullet's
content is correct is a separate question, covered by SR-579003 FND-001.

## Coverage

```yaml
scope:
- id: FR-018
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: "When a reconstruction fails, the emitted environment constructor shall return `EnvironmentError::Reconstruction` and the emitted oracle function shall return `Outcome::Refused(Refusal::CheckedInvariant { cause: CheckedInvariantCause::GeneratedDescriptorReconstructionFailed })`."
- id: FR-018 IllTyped bullet
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: "an `IllTyped` from either `check_type` or `check_equality` becomes a typed `Outcome::Refused(Refusal::CheckedInvariant { cause })`, retaining the original `IllTypedCause` in `GeneratedTypeCheckRejected` or `GeneratedEqualityCheckRejected` respectively, before any charge."
```
