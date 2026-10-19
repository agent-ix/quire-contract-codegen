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
| FR-005 | FR-005-AC-3 | TC-007 | 🚧 Planned; the Differential Disposition Register has no rows and no join check exists |
| FR-005 | FR-005-AC-4 | TC-007 | 🚧 Planned |
| FR-005 | FR-005-AC-5 | TC-002 | 🚧 Planned |
| FR-005 | FR-005-AC-6 through FR-005-AC-10 | TC-002 | ✅ Covered |
| FR-037 | FR-037-AC-1 | TC-052 | 🚧 Planned |
| FR-037 | FR-037-AC-2 | TC-052 | 🚧 Planned |
| FR-037 | FR-037-AC-3 | TC-052 | 🚧 Planned |
| FR-037 | FR-037-AC-4 | TC-052 | 🚧 Planned |
| FR-037 | FR-037-AC-5 | TC-052 | 🚧 Planned |
| FR-039 | FR-039-AC-1 through FR-039-AC-3, FR-039-AC-6 through FR-039-AC-8 | TC-053 | 🚧 Planned; production FR-330 consumer and criterion-tagged integration tests are not implemented |
| FR-039 | FR-039-AC-5 | Analysis | 🚧 Planned; CG's admitted-package type boundary is checkable in source, but result preservation has a known deviation (IR-892; planned FR-019-AC-48 to AC-52 in TC-030). Contract IR has tagged TC-047 reader tests for wrong version (AC-10), Incomplete (AC-11), invalid source map (AC-5), and stale identities (AC-5-tagged tests; AC-2 text, TC-044 tests planned/unrun for FR-330). Malformed and noncanonical wires and invalid accounting have AC-5-tagged tests but no FR-035 criterion stating those reader obligations (IR-893). No CG implementation credit |
| FR-039 | FR-039-AC-4 | TC-054 | 🚧 Planned; FR-322 worked-carrier positive and recomputed adverse strict-reader controls are feasible now, while genuine QSL/IR source-to-CG positive and changed-binding controls depend on QSL-764; no CG implementation credit |

## Non-Functional Requirement Coverage

| Non-Functional Req | Verification Method | Evidence/Test Cases | Status |
|---|---|---|---|
| NFR-001 | Test | TC-001 (NFR-001-AC-1) | 🚧 Planned |
| NFR-001 | Test | TC-002 (NFR-001-AC-2, NFR-001-AC-3) | 🚧 Planned |
| NFR-002 | Test | TC-001 (NFR-002-AC-1, NFR-002-AC-2) | 🚧 Planned; header identity and SPDX checks span every generated Rust family named in NFR-002 Scope |
| NFR-002 | Test | TC-003 (NFR-002-AC-3) | 🚧 Planned |
| NFR-002 | Inspection | NFR-002-AC-4 | 🚧 Planned |
| NFR-005 | Test | TC-042 (NFR-005-AC-1 through NFR-005-AC-8) | ✅ Covered; NFR-005-AC-1 scans the canonical `CaseIdentity::digest` with no exception |

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
| TC-001 | Verify deterministic derivation and generated headers | Integration | P0 | FR-005-AC-2, NFR-001-AC-1, NFR-002-AC-1, NFR-002-AC-2 | 🚧 Planned; all generated Rust families require header checks |
| TC-002 | Compile and publish atomically | Integration | P0 | FR-005-AC-1, FR-005-AC-5, FR-005-AC-6, FR-005-AC-7, FR-005-AC-8, FR-005-AC-9, FR-005-AC-10, NFR-001-AC-2, NFR-001-AC-3 | 🚧 Planned |
| TC-003 | Reject unsupported inputs explicitly | Integration | P0 | NFR-002-AC-3 | 🚧 Planned |
| TC-007 | Verify cross-backend semantic parity | Integration | P0 | FR-005-AC-3, FR-005-AC-4 | 🚧 Planned |
| TC-042 | Verify no panic on a generation or analysis path | Unit | P0 | NFR-005-AC-1, NFR-005-AC-2, NFR-005-AC-3, NFR-005-AC-4, NFR-005-AC-5, NFR-005-AC-6, NFR-005-AC-7, NFR-005-AC-8 | ✅ Covered; NFR-005-AC-1 scans the canonical `CaseIdentity::digest` with no exception |
| TC-052 | Verify checked-package fixture identity ownership | Integration | P0 | FR-037-AC-1, FR-037-AC-2, FR-037-AC-3, FR-037-AC-4, FR-037-AC-5 | 🚧 Planned |
| TC-053 | Verify production decisions from one admitted ContractPackage | Integration | P0 | FR-039-AC-1, FR-039-AC-2, FR-039-AC-3, FR-039-AC-6, FR-039-AC-7, FR-039-AC-8 | 🚧 Planned |
| TC-054 | Verify exact local protocol-target consumption | Integration | P0 | FR-039-AC-4 | 🚧 Planned; reader controls feasible now, QSL-764 prerequisite for source-to-CG controls |
