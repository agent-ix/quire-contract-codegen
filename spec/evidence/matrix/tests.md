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
| FR-004 | FR-004-AC-16, FR-004-AC-18 through FR-004-AC-24 | TC-006 | 🚧 V2 planned (IR-815); the V2 population analyzer is not implemented |
| FR-004 | FR-004-AC-17 | Inspection | 🚧 V2 planned (IR-815) |
| FR-004 | FR-004-AC-12 through FR-004-AC-15 | TC-006 | 🚧 Planned; QSL `/2` strict reader and CG authenticated producer/artifact binding are not implemented |
| [FR-036](../functional/FR-036-source-obligation-census.md) | FR-036-AC-1 through FR-036-AC-7 | [TC-051](TC-051-source-obligation-census.md) | 🚧 Planned; separate inventories/complete candidate tuples and typed artifact/run controls Gated on IR-759 reviewed neutral correspondence; IR-740 owns implementation |

## Test Case Summary

The coverage tables above -- Functional, Interface, Non-Functional and
Stakeholder -- are the authority for how much of any criterion they list is
backed. Where a row here reads `✅ Covered` and a criterion in its Traces To
column is marked `⚠️` or `🚧` in the table that owns it, that table governs.

| Test ID | Title | Type | Priority | Traces To | Status |
|---|---|---|---|---|---|
| TC-006 | Distinguish vacuity and unexecuted flow | Integration | P0 | FR-004-AC-1, FR-004-AC-2, FR-004-AC-3, FR-004-AC-4, FR-004-AC-5, FR-004-AC-6, FR-004-AC-7, FR-004-AC-8, FR-004-AC-9, FR-004-AC-10, FR-004-AC-11, FR-004-AC-12, FR-004-AC-13, FR-004-AC-14, FR-004-AC-15, FR-004-AC-16, FR-004-AC-18, FR-004-AC-19, FR-004-AC-20, FR-004-AC-21, FR-004-AC-22, FR-004-AC-23, FR-004-AC-24 | 🚧 Planned; bounded observation primitives are partial evidence only; `/2` reader and authenticated CG binding remain pending |
| TC-051 | Source census is independent of attempts and preserves producer facts | Integration | P0 | FR-036-AC-1, FR-036-AC-2, FR-036-AC-3, FR-036-AC-4, FR-036-AC-5, FR-036-AC-6, FR-036-AC-7 | 🚧 Planned; Gated on IR-759; no executed producer/consumer test claimed |
