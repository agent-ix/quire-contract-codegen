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
    statement: evaluate reproducibility atomicity diagnostics backend parity and provenance against the declared boundary
    supports: claim-codegen-v01
    sufficiency_criteria:
      - every native issue and protected CI gate is complete
      - upstream pins are released and reconciled
      - no blocking specification implementation or gap-review finding remains
assumptions:
  - id: assumption-consumer-validation
    statement: consuming projects validate the pinned generator and outputs for their own intended use
    owner: human-release-owner
    status: open
    review_by: "2026-12-31T00:00:00Z"
participants:
  - id: human-release-owner
    role: decision owner
    authority: accept or reject the bounded source candidate
    independence: reviews agent-assisted implementation dependency pins and evidence
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

The bounded claim concerns one identified source revision, released dependency set, backend versions,
configuration, corpus, and platform profile. It remains open throughout foundation and implementation.

## Reasoning

Specification traceability, deterministic/golden tests, atomic fault injection, compile tests, shaped
strategy tests, bounded proofs, source-mapped coverage, cross-backend parity, differential fixtures,
dependency/license audits, and complete evidence jointly address known failure scenarios. No single
tool or generated manifest makes the release decision.

## Sufficiency Decision

No automated sufficiency decision is recorded. The human release owner must review reconciled
dependencies, protected CI, retained measurements, gap analysis, open assumptions, and challenges.

## Challenges

Merged PGM-01 is pinned and reconciled. Runtime helpers remain provisionally pinned, and the IR
corpus remains unavailable. The runtime and IR identities and contracts must be reconciled before
semantic implementation leaves draft or this claim can be considered.

A second challenge is recorded against the reasoning above rather than against the claim's subject.
The reasoning says "every native issue and protected CI gate is complete", and a reader may take a
green `make ci` as evidence for that. It is evidence about the tree as committed and about nothing
else.

Measured here with three injected defects: the control tree exits 2 at `fmt-check`, the first of
eleven `ci` prerequisites; the same tree with `.IGNORE:` prepended exits 0, runs all eleven, fails
seven of them — `fmt-check`, `spec`, `lint`, `msrv`, `upstream-identity`, `test` and
`assurance-chain` — prints every diagnostic, and fails the build for none of them. The chain itself
detected the defect and returned 1; Make discarded that.

Anything that feeds the change-assurance chain is protected differently and better: Quoin binds
retained inputs by digest and every attested result is read out of the producer's own bytes, so a
suppressed producer yields an absent or unreadable input and the chain errors. The gates that feed
nothing into the chain are simply neutered, and the sufficiency criterion "every protected CI gate is
complete" cannot be discharged by an exit code alone. The decision owner reviews the diff, not only
the result. Tracked as agent-ix/quire-contract-codegen#14; the policer that used to make this claim
locally was itself a self-attestation and was not re-added.
