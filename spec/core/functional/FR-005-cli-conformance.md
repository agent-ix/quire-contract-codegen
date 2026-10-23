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

### Third-party fixture provenance

This slice specifies IR-103's provenance dependency; acquisition and the comparison runner remain
unimplemented. [AA-001](../../assurance/AA-001-differential-fixture-provenance.md) records the current
research and open evidence claim under [AP-001](../../assurance/AP-001-differential-provenance.md).

- When a comparison cites a third-party result, its provenance record shall identify the exact tool
  release and source, acquisition date, invocation, inputs, retained outputs, normalization and
  permission applying to every retained file.
- The comparison evidence shall bind its exact input and raw output content through immutable
  content identities. These bind the evidence being compared; they are not a general tool-version
  or repository-file checksum catalog.
- If retention permission is unclear, then the provenance record shall classify the lane as
  unavailable without retaining third-party bytes or a derived fixture.
- The acquisition procedure shall require an explicit local invocation rather than downloading,
  generating or refreshing third-party results as a side effect of Quire, Quoin or a hosted workflow.
- The comparison record shall retain its raw observation separately from its normalized comparison
  view and identify each normalization rule that was applied.
- If a required input is absent, a version is unsupported, output is malformed, or source or content
  identity differs, then the comparison record shall report that non-success reason without
  counting the case as agreement.
- The assurance argument shall identify local independent evaluation and third-party differential
  comparison as separate evidence classes.

First-party acquisition, normalization, comparison and audit implementations follow the ticket's
Rust requirement after specification review. Foreign-language input samples and the external tool
are identified as such; they do not authorize a second first-party executable implementation.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-005-AC-1 | Failed generation leaves no partial published bundle or edited developer region. | Test (TC-002) |
| FR-005-AC-2 | Supported platforms reproduce byte-identical files. | Test (TC-001) |
| FR-005-AC-3 | For every shared-corpus case on which two backends' normalized outcomes differ, the TC-007 Differential Disposition Register holds exactly one row naming the case, the backends, and either a regression fixture that exists in the test tree or a documented profile/backend distinction; a differing case with no row, a row naming a fixture that does not exist, or a row with neither disposition fails the check. | Test (TC-007) |
| FR-005-AC-4 | Executable, proptest, Kani, and coverage semantics agree or retain an explicit difference. | Test (TC-007) |
| FR-005-AC-5 | A bundle naming more than 4096 artifacts, an artifact whose contents exceed 16,777,216 bytes, or a complete artifact byte total exceeding 134,217,728 bytes is refused at construction as a bounded-resource `InvalidBundle` diagnostic before any staging or destination I/O begins, so no partial bundle is ever published. | Test (TC-002) |
| FR-005-AC-6 | When an artifact path is absolute, or contains a `..` segment, a `.` segment, an empty segment, a trailing separator, a backslash or a control character, or is empty, bundle construction is refused with `UnsafeArtifactPath` naming the offending `bundle.artifacts[i].path`, before any destination or staging I/O begins. | Test (TC-002) |
| FR-005-AC-7 | When publication fails before the destination is replaced, or fails after replacement began and the prior destination is restored, the publisher reports destination state `unchanged`, the destination holds its prior bytes or remains absent, an adjacent developer-owned file is byte-identical, and no staging or backup sibling remains. | Test (TC-002) |
| FR-005-AC-8 | When removal of the replaced bundle's backup fails after the staged bundle has replaced the destination, the publisher reports destination state `published`, the destination holds every artifact of the new bundle, and the backup sibling remains as visible residue. | Test (TC-002) |
| FR-005-AC-9 | When replacement fails and restoring the prior destination also fails, the publisher reports destination state `unknown`, the destination path is absent, and the complete prior bundle and the complete staged bundle remain in their sibling directories for recovery. | Test (TC-002) |
| FR-005-AC-10 | When two artifacts of one bundle share the same path, bundle construction is refused with `DuplicateArtifactPath` naming the later `bundle.artifacts[i].path`, before any destination or staging I/O begins. | Test (TC-002) |
| FR-005-AC-11 | Each retained third-party fixture has an accountable record naming its tool release/source, source corpus, acquisition date, full argument vector and relevant environment, raw input/output content identities, normalization, and the license/permission for every retained file. Missing fields leave it ineligible as differential evidence. | Inspection |
| FR-005-AC-12 | The acquisition record identifies an explicit local procedure and its actual invocation. Quire, Quoin and hosted workflows neither acquire nor refresh the third-party outputs implicitly. An unexecuted procedure is recorded as unexecuted, with no invented acquisition date, output or content identity. | Inspection |
| FR-005-AC-13 | The record enumerates the compared semantics and unsupported or intentionally different cases. Normalization preserves the raw observation and never turns timeout, unknown, malformed output or unsupported semantics into agreement. | Inspection |
| FR-005-AC-14 | Each observed discrepancy has exactly one TC-007 disposition identifying a retained permitted regression fixture or an explicit semantic difference. A missing disposition, missing referenced fixture or ambiguous comparison leaves that case non-success. | Inspection |
| FR-005-AC-15 | The provenance contract includes adverse controls for absent input/tool/output, unsupported version, malformed output, source identity drift and content drift. Each leaves a visible non-success reason; unavailable cases are excluded from the agreement numerator and remain visible in the corpus inventory. | Inspection |
| FR-005-AC-16 | The assurance argument records zero third-party evidence while no permitted result exists. The existing 265-expression evaluator and generated compile/runtime controls remain local evidence and are never relabeled as Sireum/INSPECTA runs. Unclear retention permission retains neither third-party bytes nor a derived fixture. | Inspection |

## Dependencies

- **Upstream**: [FR-002](../../strategy/functional/FR-002-tristate-proptest.md), [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md), and [FR-004](../../evidence/functional/FR-004-vacuity-evidence.md).
- **Provenance boundary**: [NFR-002](../non-functional/NFR-002-provenance-boundary.md).
