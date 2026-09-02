---
id: FR-003
title: "Generate Kani obligations and proof dependencies"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-001
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/interface-001
    type: implements
---
# FR-003: Generate Kani obligations and proof dependencies

## Description

Where bounded proof lowering is supported, the generator shall emit Kani requires, ensures, proof
harnesses, framing, bindings, and a proof dependency graph from the same clauses as executable oracles.

## Inputs

- Bounded executable clauses, state frame, bindings, backend version, and declared proof dependencies.

## Outputs

- Version-adapted Kani source, proof graph, diagnostics, and evidence identity.

## Behavior

- Framing, binding, and contract portions shall remain separately inspectable.
- Every stubbed or assumed edge shall appear in the proof dependency graph.
- Missing or failed required dependencies shall prevent a complete-proof classification.
- Unstable Kani syntax shall remain isolated behind a pinned backend adapter.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-003-AC-1 | A proof is incomplete while any required dependency is missing or failed. | Test (TC-005) |
| FR-003-AC-2 | Kani and executable-oracle semantics agree on the shared bounded corpus. | Test (TC-007) |
| FR-003-AC-3 | Unsupported constructs produce explicit diagnostics. | Test (TC-003) |
| FR-003-AC-4 | Kani version, toolchain, options, assumptions, and output identity are retained. | Inspection |

## Dependencies

- **Upstream**: [FR-001](./FR-001-deterministic-oracles.md).
