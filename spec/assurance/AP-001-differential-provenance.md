---
id: AP-001
title: "Permitted differential fixture provenance scope"
type: AssuranceProfile
status: proposed
owner: quire-contract-codegen maintainers
schema_version: 0.2
profile_kind: general
scope: IR-103 provenance research and FR-005-AC-11 through FR-005-AC-16; no executed third-party comparison
impact_assessments:
  - id: evidence-misattribution
    scenario: local evaluation or unpermitted output is presented as an attributed third-party comparison
    severity: material
    verifiability:
      class: cheap-conclusive
      stochastic_dependency: none
review_policy:
  mode: require
  operations: [spec-review]
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-005
    type: governs
  - target: ix://agent-ix/quire-contract-codegen/NFR-002
    type: references
---
# AP-001: Permitted differential fixture provenance scope

## Decision Boundary

This proposed profile bounds the IR-103 specification and provenance record. It records the
ticket's existing specification-review requirement; it introduces no executable gate or hosted
workflow. The maintainer's recorded decision permits either Sireum or INSPECTA and deterministic
local invocation/normalization choices, subject to compatible provenance and retention permission.

## Impact Scenarios

Evidence misattribution occurs when an independent local evaluator is labeled as a third-party
run, when a proposed invocation is labeled as executed, or when unpermitted bytes become fixtures.
Inspection can establish the provenance omissions and the absence of an actual run. Metadata
inspection cannot establish semantic agreement.

## Evidence Policy

The scoped [argument and provenance record](AA-001-differential-fixture-provenance.md) keeps the
third-party claim open. Future retained evidence needs the exact producer, content, acquisition
conditions, permission and limitations described by
[FR-005](../core/functional/FR-005-cli-conformance.md). Published upstream metadata is research
input, not a comparison result. First-party executable work remains Rust and follows review.

## Measurement Policy

No measurement promotion or comparison run is part of this specification-only change. A future
runner must record actual observations and adverse cases before its evidence is considered.

## Exceptions

None recorded. Unclear retention permission leaves the lane unavailable under the existing
owner decision. Independent Luna IR specification review is requested; no review verdict is
asserted by this profile.
