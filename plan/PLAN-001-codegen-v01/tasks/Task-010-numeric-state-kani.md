---
id: Task-010
title: "Numeric and state Kani ticket increment"
type: Task
status: done
track: B
priority: P0
relationships:
  - target: ix://agent-ix/quire-contract-codegen/Task-004
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-003
    type: references
  - target: ix://agent-ix/quire-contract-codegen/TC-003
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/TC-005
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/TC-007
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/TC-014
    type: verifies
---
# Task-010: Numeric and state Kani ticket increment

## Scope

Complete issue #2's numeric/state increment after Task-004: derive deterministic Boolean and bounded
`i64` subject arguments and zero/one/multiple post-state result ABIs from the checked executable
oracle dependencies. Emit exact inclusive model-domain assumptions, v2 graph/schema/attestation
identity, and source-spanned explicit refusals without consuming issue #3's strategy interface.

## Completion Evidence

SUITE-008 passes on stable and exact Rust 1.75.0 with cargo-kani 0.67.0 and the recorded executable
digest/options. Healthy mixed and ConfigVersion-style identity subjects prove; changed-state and
strict-comparison subjects print bounded concrete counterexamples. All eight FR-003 criteria are
backed, and the integrated Test Matrix reports 19 / 19 backed rows with no status lie. Native replay
is the downstream quire-spec-language IT-010 boundary.
