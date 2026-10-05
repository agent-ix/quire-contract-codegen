---
id: TM-006
title: "Contract codegen replay test matrix"
type: TestMatrix
---

# Contract codegen replay test matrix

## Functional Requirement Coverage

| Functional Req | Acceptance Criteria | Test Cases | Status |
|---|---|---|---|
| FR-016 | FR-016-AC-1 through FR-016-AC-5, FR-016-AC-8 through FR-016-AC-11, FR-016-AC-13, FR-016-AC-14 through FR-016-AC-23 | TC-026 | ✅ Covered |
| FR-016 | FR-016-AC-6, FR-016-AC-7, FR-016-AC-12 | TC-026 | 🚧 Planned |
| FR-016 | FR-016-AC-24 | TC-026 | ✅ Covered (IR-611); `tc_026_a_lock_repeating_a_dependency_is_refused` asserts QSL's `DuplicateIdentity` refusal (`invalid_package`) arriving as `DependencyLockError::Input`, with no duplicate check of this crate's own |
| FR-024 | FR-024-AC-1 through FR-024-AC-10 | TC-035 | 🚧 Planned |
| FR-024 | FR-024-AC-11 through FR-024-AC-17 | TC-035 | 🚧 Planned (IR-460) |

## Test Case Summary

The coverage tables above -- Functional, Interface, Non-Functional and
Stakeholder -- are the authority for how much of any criterion they list is
backed. Where a row here reads `✅ Covered` and a criterion in its Traces To
column is marked `⚠️` or `🚧` in the table that owns it, that table governs.

| Test ID | Title | Type | Priority | Traces To | Status |
|---|---|---|---|---|---|
| TC-026 | Verify witness decoding and native replay | Integration | P0 | FR-016-AC-1, FR-016-AC-2, FR-016-AC-3, FR-016-AC-4, FR-016-AC-5, FR-016-AC-6, FR-016-AC-7, FR-016-AC-8, FR-016-AC-9, FR-016-AC-10, FR-016-AC-11, FR-016-AC-12, FR-016-AC-13, FR-016-AC-14, FR-016-AC-15, FR-016-AC-16, FR-016-AC-17, FR-016-AC-18, FR-016-AC-19, FR-016-AC-20, FR-016-AC-21, FR-016-AC-22, FR-016-AC-23, FR-016-AC-24 | 🚧 Planned |
| TC-035 | Verify counterexample submission in QSL's counterexample envelope | Integration | P0 | FR-024-AC-1, FR-024-AC-2, FR-024-AC-3, FR-024-AC-4, FR-024-AC-5, FR-024-AC-6, FR-024-AC-7, FR-024-AC-8, FR-024-AC-9, FR-024-AC-10, FR-024-AC-11, FR-024-AC-12, FR-024-AC-13, FR-024-AC-14, FR-024-AC-15, FR-024-AC-16, FR-024-AC-17 | 🚧 Planned |
