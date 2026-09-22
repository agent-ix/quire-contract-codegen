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
- The library publisher shall reject noncanonical artifact paths before mutation, distinguish
  missing ownership inputs from I/O inspection failures, and report `unchanged`, `published`, or
  `unknown` destination state according to the observed staging, commit, and rollback result.
- The library publisher shall refuse to construct an artifact bundle naming more than 4096
  artifacts, containing one artifact whose contents exceed 16,777,216 bytes, or whose complete
  artifact bytes exceed 134,217,728 bytes, returning a bounded-resource `InvalidBundle` diagnostic
  before any staging or destination I/O begins.
- Supported platforms shall produce reproducible generated files and attestations.
- Every differential discrepancy shall become a fixture or documented semantic difference.
- The Assurance Argument shall cite completed conformance evidence without closing the human claim.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-005-AC-1 | Failed generation leaves no partial published bundle or edited developer region. | Test (TC-002) |
| FR-005-AC-2 | Supported platforms reproduce byte-identical files and attestations. | Test (TC-001) |
| FR-005-AC-3 | Every differential discrepancy has a retained disposition. | Inspection |
| FR-005-AC-4 | Executable, proptest, Kani, and coverage semantics agree or retain an explicit difference. | Test (TC-007) |
| FR-005-AC-5 | A bundle naming more than 4096 artifacts, an artifact whose contents exceed 16,777,216 bytes, or a complete artifact byte total exceeding 134,217,728 bytes is refused at construction as a bounded-resource `InvalidBundle` diagnostic before any staging or destination I/O begins, so no partial bundle is ever published. | Test (TC-002) |

## Dependencies

- **Upstream**: [FR-002](./FR-002-tristate-proptest.md), [FR-003](./FR-003-kani-lowering.md), and [FR-004](./FR-004-vacuity-evidence.md).
