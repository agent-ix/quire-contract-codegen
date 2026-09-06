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

Issue #3 is being reconciled directly from the shared-assurance `main` revision. The harness
generator owns the pre-state snapshot, subject invocation ordering, post-state evaluation, runtime
`Verdict`, proptest execution loop, explicit-discard path, and complete campaign accounting boundary.
Its accepted-case floor and discard ceiling are request inputs bound into generation identity.
Strategy generation directly shapes bounded ranges, finite memberships, enums, and supported
correlated relations; only explicitly residual constraints may use rejection.

## PR #22 round 8 repair delta

Seed the mixed positive fixture and pin its exact counters. Preserve every framework abort as
`Exhausted`, retaining the reason, accounting, and optional policy failure. Preserve the
policy-level discard check because supplied reports may contain prior discards, and prove that
path with a prepopulated-report fixture. Preserve the loop/failure checks and every generated-source
size guard. Correct the accounting unit and add a source-limit conformance case.
The campaign-outcome producer integration remains a separate completion item; generated-crate
execution supplies the focused campaign controls in this revision.

Issue #5's reviewed spec is being repaired in a bounded coordinator-authorized slice: generated
entry probes, a typed-IR implication census, strict LLVM reading, and measured classification
primitives. Native fixtures use actual generator outputs. Aggregate analysis and coverage
attestations await IR #50's bound population and a native producer/run-result contract; no private
binding or counter model is introduced. REV-014 and REV-015 record the repair and residual work.

## Guards

- The source candidates share `0f4df413b21dba39ef62aca89ededc707aab7056`; the local derivative
  integrates reviewed PR #22, #26 and #23 heads without changing those branches or reviving
  superseded bespoke-assurance PRs.
- Unsupported state, constraint, or shrinking semantics fail with a structured diagnostic rather
  than falling back to an unreported filter.
- Kani is outside this recovery slice. Vacuity is partial and closes no full FR-004 row.
