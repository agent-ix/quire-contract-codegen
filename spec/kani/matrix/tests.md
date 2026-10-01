---
id: TM-004
title: "Contract codegen Kani test matrix"
type: TestMatrix
---

# Contract codegen Kani test matrix

## Functional Requirement Coverage

| Functional Req | Acceptance Criteria | Test Cases | Status |
|---|---|---|---|
| FR-015 | FR-015-AC-1, FR-015-AC-2 | TC-025 | 🚧 Planned |
| FR-015 | FR-015-AC-3 through FR-015-AC-6 | TC-025 | ✅ Covered |
| FR-015 | FR-015-AC-7 through FR-015-AC-12 | TC-025 | ✅ Covered |
| FR-015 | FR-015-AC-13 | TC-025 | ✅ Covered |
| FR-015 | FR-015-AC-14 | TC-025 | ✅ Covered |
| FR-015 | FR-015-AC-15 | TC-033 | 🚧 Planned; the test's underivable claim is `integer.eq`, which FR-014-AC-35 makes derivable |
| FR-015 | FR-015-AC-16 through FR-015-AC-18 | TC-033 | ✅ Covered |
| FR-015 | FR-015-AC-19 through FR-015-AC-25 | TC-025 | 🚧 Planned |
| FR-015 | FR-015-AC-26 through FR-015-AC-36 | TC-025 | ✅ Covered |
| FR-015 | FR-015-AC-37 | TC-025 | ✅ Covered |
| FR-017 | FR-017-AC-2, FR-017-AC-4, FR-017-AC-5, FR-017-AC-12, FR-017-AC-13, FR-017-AC-14, FR-017-AC-15, FR-017-AC-16, FR-017-AC-17, FR-017-CON-2 | TC-027 | ✅ Covered |
| FR-017 | FR-017-AC-6, FR-017-AC-7, FR-017-AC-11, FR-017-CON-1 | TC-027 | 🚧 Planned |
| FR-025 | FR-025-AC-1 | TC-036 | 🚧 Planned; emission order is asserted only for the V1 `BoundClause` harness kinds, and the ascending order and the scalar-claim harness are unasserted |
| FR-025 | FR-025-AC-2 through FR-025-AC-8 | TC-036 | 🚧 Planned |
| FR-028 | FR-028-AC-1 through FR-028-AC-9 | TC-039 | 🚧 Planned |
| FR-029 | FR-029-AC-1 through FR-029-AC-6 | TC-040 | 🚧 Planned |
| FR-030 | FR-030-AC-1 through FR-030-AC-8 | TC-041 | 🚧 Planned |

## Test Case Summary

The coverage tables above -- Functional, Interface, Non-Functional and
Stakeholder -- are the authority for how much of any criterion they list is
backed. Where a row here reads `✅ Covered` and a criterion in its Traces To
column is marked `⚠️` or `🚧` in the table that owns it, that table governs.

| Test ID | Title | Type | Priority | Traces To | Status |
|---|---|---|---|---|---|
| TC-005 | Enforce Kani proof dependencies | Analysis | P0 | FR-015-AC-22, FR-015-AC-25 | 🚧 Planned |
| TC-014 | Verify bounded numeric and state Kani contracts | Analysis | P0 | FR-015-AC-11, FR-015-AC-19, FR-015-AC-24 | 🚧 Planned |
| TC-023 | Verify bounded Kani profile corpus parity | Integration | P0 | FR-015-AC-23 | 🚧 Planned |
| TC-025 | Verify separate bounded Kani obligations | Analysis | P0 | FR-015-AC-1, FR-015-AC-2, FR-015-AC-3, FR-015-AC-4, FR-015-AC-5, FR-015-AC-6, FR-015-AC-7, FR-015-AC-8, FR-015-AC-9, FR-015-AC-10, FR-015-AC-11, FR-015-AC-12, FR-015-AC-13, FR-015-AC-14, FR-015-AC-19, FR-015-AC-20, FR-015-AC-21, FR-015-AC-22, FR-015-AC-23, FR-015-AC-24, FR-015-AC-25, FR-015-AC-26, FR-015-AC-27, FR-015-AC-28, FR-015-AC-29, FR-015-AC-30, FR-015-AC-31, FR-015-AC-32, FR-015-AC-33, FR-015-AC-34, FR-015-AC-35, FR-015-AC-36, FR-015-AC-37 | 🚧 Planned |
| TC-027 | Verify Kani obligation execution and its evidence | Analysis | P0 | FR-017-AC-2, FR-017-AC-4, FR-017-AC-5, FR-017-AC-6, FR-017-AC-7, FR-017-AC-11, FR-017-AC-12, FR-017-AC-13, FR-017-AC-14, FR-017-AC-15, FR-017-AC-16, FR-017-AC-17, FR-017-CON-1, FR-017-CON-2 | 🚧 Planned |
| TC-036 | Verify the generated harness subject ABI | Integration | P0 | FR-025-AC-1, FR-025-AC-2, FR-025-AC-3, FR-025-AC-4, FR-025-AC-5, FR-025-AC-6, FR-025-AC-7, FR-025-AC-8 | 🚧 Planned |
| TC-039 | Verify bounded proof ceilings, their inconclusive reasons and the proof subject | Integration | P0 | FR-028-AC-1, FR-028-AC-2, FR-028-AC-3, FR-028-AC-4, FR-028-AC-5, FR-028-AC-6, FR-028-AC-7, FR-028-AC-8, FR-028-AC-9 | 🚧 Planned |
| TC-040 | Verify the total map from a Kani run outcome to QSL's terminal value | Integration | P0 | FR-029-AC-1, FR-029-AC-2, FR-029-AC-3, FR-029-AC-4, FR-029-AC-5, FR-029-AC-6 | 🚧 Planned |
| TC-041 | Verify the total map from a Contract IR Kani outcome to QSL's terminal value | Integration | P0 | FR-030-AC-1, FR-030-AC-2, FR-030-AC-3, FR-030-AC-4, FR-030-AC-5, FR-030-AC-6, FR-030-AC-7, FR-030-AC-8 | 🚧 Planned |
