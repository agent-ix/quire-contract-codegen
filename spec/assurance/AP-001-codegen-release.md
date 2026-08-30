---
id: AP-001
title: Contract codegen v0.1 decision profile
type: AssuranceProfile
status: proposed
owner: human-release-owner
profile_version: 0.2
profile_kind: general
scope: one identified quire-contract-codegen v0.1 source candidate and pinned dependency set
impact_assessments:
  - id: impact-semantic-drift
    scenario: a generated backend artifact differs from the authoritative contract or another backend
    severity: material
    verifiability:
      class: cheap-conclusive
      stochastic_dependency: none
    detect_before_harm:
      expected: true
      control_ref: ix://agent-ix/quire-contract-codegen/CAC-001
review_policy:
  mode: require
  operations: [code-review, gap-analysis]
relationships:
  - target: ix://agent-ix/quire-contract-ir/PGM-01
    type: references
---
# Contract codegen v0.1 decision profile

## Decision Boundary

This profile covers one codegen source revision, pinned IR/schema/corpus, runtime, backend versions,
configuration, and platform profile. It supplies evidence only and confers no consuming-project
validation, accreditation, certification, or release authority.

## Impact Scenarios

Material scenarios include silent construct loss, nondeterministic derivation, pass/rejection
conflation, proof completion with missing dependencies, vacuous satisfaction presented as exercised,
partial output publication, backend drift, and incomplete provenance or licensing.

## Evidence Policy

Evidence identifies source, input, schema, corpus, runtime, tool, backend, options, platform, outputs,
and digests. Unsupported, rejected, discarded, failed, inconclusive, unavailable, and differential
states remain visible. A human release owner alone judges sufficiency.

## Exceptions

No standing exceptions exist. Any exception requires an owner, rationale, affected requirements and
backends, expiry, evidence effect, and explicit human acceptance under PGM-01.
