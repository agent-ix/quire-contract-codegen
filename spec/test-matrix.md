---
id: TM-001
title: "Contract codegen v0.1 test matrix"
type: TestMatrix
---

# Contract codegen v0.1 test matrix

## Functional Requirement Coverage

| Functional Req | Acceptance Criteria | Test Cases | Coverage Status |
|---|---|---|---|
| FR-001 | FR-001-AC-1, FR-001-AC-3 | TC-001 | 🚧 Planned |
| FR-001 | FR-001-AC-2 | TC-002 | 🚧 Planned |
| FR-001 | FR-001-AC-4 | TC-003 | 🚧 Planned |
| FR-002 | FR-002-AC-1 through FR-002-AC-4 | TC-004 | 🚧 Planned |
| FR-003 | FR-003-AC-1 | TC-005 | 🚧 Planned |
| FR-003 | FR-003-AC-2 | TC-007 | 🚧 Planned |
| FR-003 | FR-003-AC-3 | TC-003 | 🚧 Planned |
| FR-003 | FR-003-AC-4 | Inspection | 🚧 Planned |
| FR-004 | FR-004-AC-1 through FR-004-AC-3 | TC-006 | 🚧 Planned |
| FR-004 | FR-004-AC-4 | Inspection | 🚧 Planned |
| FR-005 | FR-005-AC-1 | TC-002 | 🚧 Planned |
| FR-005 | FR-005-AC-2 | TC-001 | 🚧 Planned |
| FR-005 | FR-005-AC-3 | Inspection | 🚧 Planned |
| FR-005 | FR-005-AC-4 | TC-007 | 🚧 Planned |

The current coverage selector expects a `Status` column while the TestMatrix structure requires
`Coverage Status` (upstream spec-artifacts-process #77). Until that conflict is resolved, the
foundation evidence test requires every functional, stakeholder, and test-case row to remain
`🚧 Planned`; any transition is a human-reviewed matrix change and must replace that temporary
control with backed-test verification.

## Nonfunctional Requirement Coverage

| Nonfunctional Req | Acceptance Criteria | Test/Inspection | Coverage Status |
|---|---|---|---|
| NFR-001 | NFR-001-AC-1 | TC-001 | 🚧 Planned |
| NFR-001 | NFR-001-AC-2, NFR-001-AC-3 | TC-002 | 🚧 Planned |
| NFR-002 | NFR-002-AC-1, NFR-002-AC-2 | TC-001 | 🚧 Planned |
| NFR-002 | NFR-002-AC-3 | TC-003 | 🚧 Planned |
| NFR-002 | NFR-002-AC-4 | Inspection | 🚧 Planned |

## Stakeholder Requirement Coverage

| Stakeholder Req | Trace to US/FR | Test/Validation | Coverage Status |
|---|---|---|---|
| StR-001 | StR-001-VC-1, FR-001 | TC-001 | 🚧 Planned |
| StR-001 | StR-001-VC-2, FR-003, FR-004 | TC-007 | 🚧 Planned |

## Test Case Summary

| Test ID | Title | Type | Priority | Traces To | Status |
|---|---|---|---|---|---|
| TC-001 | Reproduce artifacts and manifests | Integration | P0 | FR-001-AC-1, FR-001-AC-3, FR-005-AC-2, NFR-001-AC-1, NFR-002-AC-1, NFR-002-AC-2 | 🚧 Planned |
| TC-002 | Compile and publish atomically | Integration | P0 | FR-001-AC-2, FR-005-AC-1, NFR-001-AC-2, NFR-001-AC-3 | 🚧 Planned |
| TC-003 | Reject unsupported inputs explicitly | Integration | P0 | FR-001-AC-4, FR-003-AC-3, NFR-002-AC-3 | 🚧 Planned |
| TC-004 | Preserve shaped proptest strategies | Property | P0 | FR-002-AC-1, FR-002-AC-2, FR-002-AC-3, FR-002-AC-4 | 🚧 Planned |
| TC-005 | Enforce Kani proof dependencies | Analysis | P0 | FR-003-AC-1 | 🚧 Planned |
| TC-006 | Distinguish vacuity and unexecuted flow | Integration | P0 | FR-004-AC-1, FR-004-AC-2, FR-004-AC-3 | 🚧 Planned |
| TC-007 | Verify cross-backend semantic parity | Integration | P0 | FR-003-AC-2, FR-005-AC-4 | 🚧 Planned |

All rows are deliberately planned. The placeholder scaffold tests carry no TC identifier and do not
back any semantic requirement.

## Evidence Locations

Each row is specified in the same-ID document under `spec/test/`; implementation evidence is pending.
