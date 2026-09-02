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

The harness generator owns the pre-state snapshot, subject invocation ordering, post-state evaluation,
runtime `Verdict`, and an owned campaign runner with accepted/rejected/failed/discarded accounting.
Strategy generation directly shapes bounded ranges, finite memberships, enum memberships, and supported
correlated relations, with each case carrying an expected domain outcome. The bounded Kani adapter
reuses the exact oracle predicates, emits distinct framing/binding/contract/harness regions, derives a
complete dependency graph, and records proof execution as `not_run`.

## Guards

- The current implementation is based directly on migrated `main`; no superseded stacked evidence
  envelope or repository-local assurance framework may return.
- Unsupported state, constraint, or shrinking semantics fail with a structured diagnostic rather
  than falling back to an unreported filter.
- Kani remains a draft until independent current-head review; LLVM vacuity analysis is not started.
