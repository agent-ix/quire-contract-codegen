---
id: TM-002
title: "Contract codegen strategy test matrix"
type: TestMatrix
---

# Contract codegen strategy test matrix

## Functional Requirement Coverage

| Functional Req | Acceptance Criteria | Test Cases | Status |
|---|---|---|---|
| FR-002 | FR-002-AC-1 through FR-002-AC-6 | TC-004 | 🚧 Planned |
| FR-008 | FR-008-AC-1 through FR-008-AC-5, FR-008-CON-2 | TC-017 | ✅ Covered |
| FR-008 | FR-008-CON-1 | Inspection | ✅ Covered |
| FR-009 | FR-009-AC-1 through FR-009-AC-6 | TC-018 | ✅ Covered |
| FR-010 | FR-010-AC-1 through FR-010-AC-5 | TC-019 | ✅ Covered |
| FR-011 | FR-011-AC-1 through FR-011-AC-5 | TC-020 | ✅ Covered |
| FR-012 | FR-012-AC-1 through FR-012-AC-3 | TC-021 | ✅ Covered |
| FR-012 | FR-012-AC-4 | TC-021 | ✅ Covered |
| FR-013 | FR-013-AC-1, FR-013-AC-2, FR-013-AC-4 | TC-022 | ✅ Covered |
| FR-013 | FR-013-AC-5 | Inspection | ✅ Covered |

## Non-Functional Requirement Coverage

| Non-Functional Req | Verification Method | Evidence/Test Cases | Status |
|---|---|---|---|
| NFR-004 | Test | TC-020 (NFR-004-AC-1) | ✅ Covered |
| NFR-004 | Test | TC-019 (NFR-004-AC-2) | ✅ Covered |

## Test Case Summary

The coverage tables above -- Functional, Interface, Non-Functional and
Stakeholder -- are the authority for how much of any criterion they list is
backed. Where a row here reads `✅ Covered` and a criterion in its Traces To
column is marked `⚠️` or `🚧` in the table that owns it, that table governs.

| Test ID | Title | Type | Priority | Traces To | Status |
|---|---|---|---|---|---|
| TC-004 | Preserve shaped proptest strategies | Property | P0 | FR-002-AC-1, FR-002-AC-2, FR-002-AC-3, FR-002-AC-4, FR-002-AC-5, FR-002-AC-6 | 🚧 Planned |
| TC-017 | Verify bound-clause domain derivation and refusal | Integration | P0 | FR-008-AC-1, FR-008-AC-2, FR-008-AC-3, FR-008-AC-4, FR-008-AC-5, FR-008-CON-2 | ✅ Covered |
| TC-018 | Verify constructive satisfying and violating populations | Property | P0 | FR-009-AC-1, FR-009-AC-2, FR-009-AC-3, FR-009-AC-4, FR-009-AC-5, FR-009-AC-6 | ✅ Covered |
| TC-019 | Verify domain and relation boundary censuses | Integration | P0 | FR-010-AC-1, FR-010-AC-2, FR-010-AC-3, FR-010-AC-4, FR-010-AC-5, NFR-004-AC-2 | ✅ Covered |
| TC-020 | Verify numeric conformance campaigns and rate reporting | Integration | P0 | FR-011-AC-1, FR-011-AC-2, FR-011-AC-3, FR-011-AC-4, FR-011-AC-5, NFR-004-AC-1 | ✅ Covered |
| TC-021 | Verify shrinking preserves numeric constraints | Property | P0 | FR-012-AC-1, FR-012-AC-2, FR-012-AC-3, FR-012-AC-4 | ✅ Covered |
| TC-022 | Verify strategy output is consumable without a local wire schema | Integration | P0 | FR-013-AC-1, FR-013-AC-2, FR-013-AC-4 | ✅ Covered |

TC-017 through TC-022 are backed by passing named tests in `tests/bound_strategy_generation.rs`,
`tests/bound_populations.rs`, and `tests/bound_census.rs`. Together they cover admission and ordered
refusals, constructive populations, boundary censuses, runtime accounting and replay, and generated
consumer compilation.
