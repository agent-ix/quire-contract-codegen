---
id: AA-001
title: Contract codegen v0.1 assurance argument
type: AssuranceArgument
status: proposed
owner: human-release-owner
profile: ix://agent-ix/quire-contract-codegen/AP-001
top_claim:
  id: claim-codegen-v01
  statement: the identified codegen source candidate and dependency set are acceptable for bounded v0.1 use
  subject: quire-contract-codegen v0.1 source candidate
  status: open
reasoning:
  - id: reasoning-derivation-conformance
    statement: evaluate reproducibility atomicity diagnostics and backend parity against the declared boundary
    supports: claim-codegen-v01
    sufficiency_criteria:
      - every native issue and protected CI gate is complete
      - upstream dependencies are released and reconciled
      - no blocking specification implementation or gap-review finding remains
assumptions:
  - id: assumption-consumer-validation
    statement: consuming projects validate the generator and outputs for their own intended use
    owner: human-release-owner
    status: open
    review_by: "2026-12-31T00:00:00Z"
participants:
  - id: human-release-owner
    role: decision owner
    authority: accept or reject the bounded source candidate
    independence: reviews agent-assisted implementation dependencies and evidence
challenges:
  - id: challenge-draft-dependencies
    target: claim-codegen-v01
    statement: PGM-01 is merged and reconciled, while runtime remains provisional and the IR corpus is unavailable
    status: open
    owner: human-release-owner
  - id: challenge-make-is-not-a-trust-root
    target: claim-codegen-v01
    statement: a green local gate run is evidence about the tree as committed and not about a tree whose Makefile has been edited
    status: open
    owner: human-release-owner
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AP-001
    type: references
---
# Contract codegen v0.1 assurance argument

## Claim

The bounded claim concerns one identified source revision, released dependency set, backends,
configuration, corpus, and platform profile. It remains open throughout foundation and implementation.

## Reasoning

Specification traceability, deterministic regeneration tests, atomic fault injection, compile tests,
shaped strategy tests, bounded proofs, source-mapped coverage, cross-backend parity, differential
fixtures, and dependency/license audits jointly address known failure scenarios. No single tool makes
the release decision.

## Sufficiency Decision

No automated sufficiency decision is recorded. The human release owner must review reconciled
dependencies, the local gates, gap analysis, open assumptions, and challenges.

## Challenges

Merged PGM-01 is reconciled. The runtime dependency remains provisional, and the IR corpus remains
unavailable. The runtime and IR contracts must be reconciled before semantic implementation leaves
draft or this claim can be considered.

A second challenge is recorded against the reasoning above rather than against the claim's subject.
The reasoning says "every native issue and protected CI gate is complete", and a reader may take a
green `make ci` as evidence for that. It is evidence about the tree as committed and about nothing
else: Make can be told to ignore a recipe's exit status (`.IGNORE:`, a `-` recipe prefix, or a
`SHELL` assignment), so the decision owner reviews the diff, not only the result. Tracked as
agent-ix/quire-contract-codegen#14.

