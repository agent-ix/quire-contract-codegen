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

Issue #3 starts in stacked draft `wave2-agent-b-harnesses`. The harness generator owns the pre-state
snapshot, subject invocation ordering, post-state evaluation, runtime `Verdict`, and complete campaign
accounting boundary. Strategy generation directly shapes bounded ranges, finite memberships, and
supported correlated relations; only explicitly residual constraints may use rejection.

Issue #5 is in specification review from the shared-assurance `main` revision. Its proposed analyzer
consumes full LLVM JSON, requires a dedicated oracle-evaluation source region, and keeps unexecuted,
vacuous, partially exercised, and exercised classifications distinct from campaign counts and test
outcome. No vacuity implementation begins until the exact specification head receives independent
review.

## Guards

- PR #10 remains the provisional Task-004 base; this branch must rebase to its accepted revision.
- Unsupported state, constraint, or shrinking semantics fail with a structured diagnostic rather
  than falling back to an unreported filter.
- Kani and vacuity backends remain not started within this task.
