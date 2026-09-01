---
id: Task-005
title: "Harness proof and vacuity backends"
type: Task
status: in_progress
track: B
priority: P0
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-002
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-003
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-004
    type: references
---
# Task-005: Harness, proof, and vacuity backends

## Scope

Implement tri-state harness/proptest generation, Kani lowering and proof dependencies, and LLVM-based
vacuity evidence after deterministic oracle semantics exist.

## Current slice

Issue #2 starts on parallel draft `wave2-agent-b-kani`, based on the exact current deterministic
oracle head. The first slice is limited to Boolean pre/post clauses, explicit customer subject
bindings, Kani `0.67.0` function contracts/proof harnesses, and a fail-closed proof dependency graph.

## Guards

- The oracle branch remains the provisional Task-004 base; this branch must rebase to its accepted revision.
- No generated harness may be classified complete from its own exit status alone.
- Assumption/stub source sites and graph edges must form a bijection.
- Vacuity backend work remains not started on this branch.
