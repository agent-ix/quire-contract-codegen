---
id: FR-005
title: "Provide atomic CLI and cross-backend conformance"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-002
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-004
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/interface-001
    type: implements
---
# FR-005: Provide atomic CLI and cross-backend conformance

## Description

When invoked through the library or CLI, the generator shall write a complete artifact bundle
atomically and shall retain differential and cross-backend semantic-parity results.

## Inputs

- Serialized contract package, output directory, and backend selection.

## Outputs

- Atomic artifact directory or explicit diagnostic with no partial developer-owned edits.

## Behavior

- The CLI shall never edit developer-owned source regions.
- The library publisher shall reject noncanonical artifact paths before mutation, and report `unchanged`, `published`, or
  `unknown` destination state according to the observed staging, commit, and rollback result.
- The library publisher shall refuse to construct an artifact bundle naming more than 4096
  artifacts, containing one artifact whose contents exceed 16,777,216 bytes, or whose complete
  artifact bytes exceed 134,217,728 bytes, returning a bounded-resource `InvalidBundle` diagnostic
  before any staging or destination I/O begins.
- Supported platforms shall produce reproducible generated files.
- Every differential discrepancy shall become a fixture or documented semantic difference, retained as
  one row of the TC-007 Differential Disposition Register.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-005-AC-1 | Failed generation leaves no partial published bundle or edited developer region. | Test (TC-002) |
| FR-005-AC-2 | Supported platforms reproduce byte-identical files. | Test (TC-001) |
| FR-005-AC-3 | For every shared-corpus case on which two backends' normalized outcomes differ, the TC-007 Differential Disposition Register holds exactly one row naming the case, the backends, and either a regression fixture that exists in the test tree or a documented profile/backend distinction; a differing case with no row, a row naming a fixture that does not exist, or a row with neither disposition fails the check. | Test (TC-007) |
| FR-005-AC-4 | Executable, proptest, Kani, and coverage semantics agree or retain an explicit difference. | Test (TC-007) |
| FR-005-AC-5 | A bundle naming more than 4096 artifacts, an artifact whose contents exceed 16,777,216 bytes, or a complete artifact byte total exceeding 134,217,728 bytes is refused at construction as a bounded-resource `InvalidBundle` diagnostic before any staging or destination I/O begins, so no partial bundle is ever published. | Test (TC-002) |
| FR-005-AC-6 | When an artifact path is absolute, or contains a `..` segment, a `.` segment, an empty segment, a trailing separator or a backslash, or aliases an earlier path in the same bundle, bundle construction is refused with `UnsafeArtifactPath` naming the offending `bundle.artifacts[i].path`, before any destination or staging I/O begins. | Test (TC-002) |
| FR-005-AC-7 | When publication fails before the destination is replaced, or fails after replacement began and the prior destination is restored, the publisher reports destination state `unchanged`, the destination holds its prior bytes or remains absent, an adjacent developer-owned file is byte-identical, and no staging or backup sibling remains. | Test (TC-002) |
| FR-005-AC-8 | A failure to remove the replaced bundle's backup after the staged bundle has replaced the destination is reported with destination state `published`; the destination holds the complete new bundle and the backup sibling remains as visible residue. | Test (TC-002) |
| FR-005-AC-9 | When replacement fails and restoring the prior destination also fails, the publisher reports destination state `unknown`, the destination path is absent, and the complete prior bundle and the complete staged bundle remain in their sibling directories for recovery. | Test (TC-002) |

## Dependencies

- **Upstream**: [FR-002](../../strategy/functional/FR-002-tristate-proptest.md), [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md), and [FR-004](../../evidence/functional/FR-004-vacuity-evidence.md).
