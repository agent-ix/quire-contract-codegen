---
id: "SR-2244"
title: "IR-635 composite claim-shape spec diff: failure-domain analysis"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@cc3a78b0df85c6d3fde86af0b5a13a6eccc02d08; spec/replay/functional/FR-033-composite-parity-replay-binding.md, spec/replay/matrix/TC-048-composite-parity-replay-binding.md, spec/assurance/AD-003-evidence-chain.md"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-033
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-048
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/AD-003
    type: reviews
---

# SR-2244: IR-635 composite claim-shape spec diff: failure-domain analysis

## Summary

Ticket: IR-635. PR: not yet opened. Diff reviewed: ca5d61adbb62959097bfdd06a3374f2f890f2e3b...cc3a78b0df85c6d3fde86af0b5a13a6eccc02d08.
Method: spec-review/failure-domain. Reviewer model claude-opus-5-5, session cd5dfa92-5d1b-442d-b3fe-30c6c25b005e, run 09c0daf0-442e-4959-9cd9-2346807cdb0d.

## Method

Unstated failure modes and identity confusion in the new positional O-09, sent-claim and artifact-authentication text, checked against QSL source.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Population-keyed harness bounds partition out of O-09 arguments is unstated | spec/replay/functional/FR-033-composite-parity-replay-binding.md:76-85 |

### Finding detail

- FND-001 (severity low, confidence medium, check other, unit FR-033 Inputs O-09 bullet): When QSL recomputes the preimage (composite_parity::parity_preimage), each parameter's Bounds uses only the `harness_bounds` entries keyed `DomainKey::Node` with that parameter's node. `DomainKey::Population` entries stay in `CompositeIdentity.harness_bounds` but never enter O-09. FR-033 says only 'its drawn harness bounds', and AC-11/TC-048 step 9 never exercise a population key. A CG builder that puts a Population-keyed bound, or the whole bound list, into an argument mints an O-09 that QSL's identity tie refuses as an obligation mismatch. Every claim whose harness carries a population bound would then refuse.

## Verdict

Self-comparison, literal and duplicate-key identity rules match QSL. Echo equality is correctly kept separate from artifact authentication. Refusals stay typed, with no sentinel identity.
