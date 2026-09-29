---
id: ADR-003
title: "Kani tractability of generated obligations"
type: ADR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AD-001
    type: relates_to
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: relates_to
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: relates_to
  - target: ix://agent-ix/quire-contract-codegen/FR-021
    type: relates_to
  - target: ix://agent-ix/quire-contract-codegen/FR-028
    type: relates_to
  - target: ix://agent-ix/quire-spec-language/ADR-011
    type: relates_to
---
# ADR-003: Kani tractability of generated obligations

## Status

Accepted.

## Context

A generated Kani harness embeds its generated oracle. The oracle calls
`quire_contract_runtime::exact`, so the prover proves the production code through the production
data structures.

## Decision

The goal is the widest proof coverage that can be obtained, with every proof bounded so that it
can finish.

### Q1: a family proves production code where it verifies, and a bounded shadow elsewhere

Proof coverage is maximised per family:

- A family whose harness over the production `exact` code verifies within its ceilings proves the
  production code, as FR-015's scalar harnesses do.
- A family whose production harness cannot verify within its ceilings proves its claim over a
  bounded shadow model, together with a separate refinement obligation showing that the production
  code agrees with the shadow on the bounded domain. The shadow is derived independently of the
  code under proof (QSL ADR-011 §2.3). Contract Runtime co-owns the shadow and the refinement
  obligation for its kernel types.

A shadow proof is never reported without its refinement obligation.

### Q2: ceilings in the harness identity, with their own inconclusive reasons

- Every harness is bounded. Its symbolic arguments carry inclusive bounds from their declared model
  domains, and an item with no finite bound gets no harness, which settles `requires-bound`
  (FR-015, FR-025).
- Every harness identity records a memory ceiling and a wall-clock ceiling. A run is held to both.
- A run that exceeds its wall-clock ceiling settles `inconclusive` with its own timed-out reason. A
  run that exceeds its memory ceiling settles `inconclusive` with its own memory-exhausted reason.
  Neither is ever reported as verified or falsified.
- The evidence of every run records the bounds and the ceilings it ran under.
- Coverage is reportable per family. Every harness identity, refusal and execution evidence names
  its family, so a gap stays visible.
- A bound may be tightened inside the declared model domain to let a proof finish only when the
  harness identity and the evidence record both the declared domain and the tightened bound. A
  harness over a tightened bound covers that bound and never reads as a proof over the declared
  domain.

### Q3: the stubbing ban stays

FR-015-AC-9's ban stands: no option enabling stubbing is emitted. Every stub is an assumption the
proof does not discharge.

[FR-028](../functional/complete-v1/FR-028-bounded-proof-ceilings.md) states Q1 and Q2.

## Consequences

- FR-028 states the ceilings, their inconclusive reasons, the tightened-bound record, the
  per-family naming and the proof subject.
- FR-018 and FR-021 oracles get a Kani harness through FR-015 only over a bounded shadow with its
  refinement obligation.
