---
id: TM-001
title: "Contract codegen core test matrix"
type: TestMatrix
---

# Contract codegen core test matrix

## Functional Requirement Coverage

| Functional Req | Acceptance Criteria | Test Cases | Status |
|---|---|---|---|
| FR-005 | FR-005-AC-1 | TC-002 | 🚧 Planned |
| FR-005 | FR-005-AC-2 | TC-001 | 🚧 Planned |
| FR-005 | FR-005-AC-3 | Inspection | 🚧 Planned |
| FR-005 | FR-005-AC-4 | TC-007 | 🚧 Planned |
| FR-005 | FR-005-AC-5 | TC-002 | 🚧 Planned |

## Non-Functional Requirement Coverage

| Non-Functional Req | Verification Method | Evidence/Test Cases | Status |
|---|---|---|---|
| NFR-001 | Test | TC-001 (NFR-001-AC-1) | 🚧 Planned |
| NFR-001 | Test | TC-002 (NFR-001-AC-2, NFR-001-AC-3) | 🚧 Planned |
| NFR-002 | Test | TC-001 (NFR-002-AC-1, NFR-002-AC-2) | 🚧 Planned |
| NFR-002 | Test | TC-003 (NFR-002-AC-3) | 🚧 Planned |
| NFR-002 | Inspection | NFR-002-AC-4 | 🚧 Planned |
| NFR-005 | Test | TC-042 (NFR-005-AC-1 through NFR-005-AC-5) | ✅ Covered; NFR-005-AC-1 allows the one dated IR-344 `expect` of `CaseIdentity::digest` until that code lands |
| NFR-005 | Test | TC-042 (NFR-005-AC-6 through NFR-005-AC-8) | 🚧 Planned (IR-577) |

## Stakeholder Requirement Coverage

| Stakeholder Req | Trace to US/FR | Test/Validation | Status |
|---|---|---|---|
| StR-001 | StR-001-VC-1, FR-014 | TC-024 | 🚧 Planned |
| StR-001 | StR-001-VC-2, FR-015, FR-004 | TC-007, TC-025 | 🚧 Planned |
| StR-001 | StR-001-VC-3, FR-016, FR-024 | TC-026, TC-035 | 🚧 Planned |
| StR-001 | StR-001-VC-4, FR-015, FR-017, FR-019 | TC-025, TC-027, TC-030 | 🚧 Planned |

## Test Case Summary

The coverage tables above -- Functional, Interface, Non-Functional and
Stakeholder -- are the authority for how much of any criterion they list is
backed. Where a row here reads `✅ Covered` and a criterion in its Traces To
column is marked `⚠️` or `🚧` in the table that owns it, that table governs.

| Test ID | Title | Type | Priority | Traces To | Status |
|---|---|---|---|---|---|
| TC-001 | Verify deterministic derivation | Integration | P0 | FR-005-AC-2, NFR-001-AC-1, NFR-002-AC-1, NFR-002-AC-2 | 🚧 Planned |
| TC-002 | Compile and publish atomically | Integration | P0 | FR-005-AC-1, FR-005-AC-5, NFR-001-AC-2, NFR-001-AC-3 | 🚧 Planned |
| TC-003 | Reject unsupported inputs explicitly | Integration | P0 | NFR-002-AC-3 | 🚧 Planned |
| TC-007 | Verify cross-backend semantic parity | Integration | P0 | FR-005-AC-4 | 🚧 Planned |
| TC-042 | Verify no panic on a generation or analysis path | Unit | P0 | NFR-005-AC-1, NFR-005-AC-2, NFR-005-AC-3, NFR-005-AC-4, NFR-005-AC-5, NFR-005-AC-6, NFR-005-AC-7, NFR-005-AC-8 | 🚧 NFR-005-AC-1 to AC-5 covered; AC-6 to AC-8 planned (IR-577) |
