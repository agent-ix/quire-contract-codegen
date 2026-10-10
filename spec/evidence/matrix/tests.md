---
id: TM-007
title: "Contract codegen evidence test matrix"
type: TestMatrix
---

# Contract codegen evidence test matrix

## Functional Requirement Coverage

| Functional Req | Acceptance Criteria | Test Cases | Status |
|---|---|---|---|
| FR-004 | FR-004-AC-1 through FR-004-AC-8 | TC-006 | 🚧 Planned |
| FR-004 | FR-004-AC-9 | TC-006 | 🚧 Planned |
| FR-004 | FR-004-AC-10, FR-004-AC-11 | TC-006 | 🚧 Planned; native run/source-map binding, LLVM producer identity and consuming obligation are not implemented |

## Test Case Summary

The coverage tables above -- Functional, Interface, Non-Functional and
Stakeholder -- are the authority for how much of any criterion they list is
backed. Where a row here reads `✅ Covered` and a criterion in its Traces To
column is marked `⚠️` or `🚧` in the table that owns it, that table governs.

| Test ID | Title | Type | Priority | Traces To | Status |
|---|---|---|---|---|---|
| TC-006 | Distinguish vacuity and unexecuted flow | Integration | P0 | FR-004-AC-1, FR-004-AC-2, FR-004-AC-3, FR-004-AC-4, FR-004-AC-5, FR-004-AC-6, FR-004-AC-7, FR-004-AC-8, FR-004-AC-9, FR-004-AC-10, FR-004-AC-11 | 🚧 Planned; bounded observation primitives are partial evidence only |
