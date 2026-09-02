---
id: FR-005
title: "Provide atomic CLI and cross-backend conformance"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-002
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-003
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-004
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/interface-001
    type: implements
---
# FR-005: Provide atomic CLI and cross-backend conformance

## Description

When invoked through the library or CLI, the generator shall write a complete artifact bundle
atomically and shall retain golden, differential, and cross-backend semantic-parity results.

## Inputs

- Serialized contract package, output directory, backend selection, and pinned fixture identities.

## Outputs

- Atomic artifact directory or explicit diagnostic with no partial developer-owned edits.

## Behavior

- The CLI shall never edit developer-owned source regions.
- Supported platforms shall produce reproducible generated files and manifests.
- Every differential discrepancy shall become a fixture or documented semantic difference.
- The Assurance Argument shall cite completed conformance evidence without closing the human claim.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-005-AC-1 | Failed generation leaves no partial published bundle or edited developer region. | Test (TC-002) |
| FR-005-AC-2 | Supported platforms reproduce byte-identical files and manifests. | Test (TC-001) |
| FR-005-AC-3 | Every differential discrepancy has a retained disposition. | Inspection |
| FR-005-AC-4 | Executable, proptest, Kani, and coverage semantics agree or retain an explicit difference. | Test (TC-007) |

## Dependencies

- **Upstream**: [FR-002](./FR-002-tristate-proptest.md), [FR-003](./FR-003-kani-lowering.md), and [FR-004](./FR-004-vacuity-evidence.md).
