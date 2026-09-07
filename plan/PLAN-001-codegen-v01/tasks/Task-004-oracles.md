---
id: Task-004
title: "Deterministic oracles and proof attestations"
type: Task
status: in_progress
track: B
priority: P0
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-001
    type: references
---
# Task-004: Deterministic oracles and proof attestations

## Scope

Implement issue #4 against the exact accepted IR PR #19 merge: deterministic per-clause Rust
oracles, source maps, and one Quoin ProofAttestationV1 body per output.

## Guard

Task-003 is complete. PR #15 migrated the implementation on `main` to the shared assurance contract
against IR revision `5c49ebfd1c87415f74420ad047392bd03b1bd202`; this task remains in progress
until its semantic acceptance criteria and every current-head review finding are closed.
