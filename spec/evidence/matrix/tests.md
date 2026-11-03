---
id: TM-007
title: "Contract codegen evidence test matrix"
type: TestMatrix
---

# Contract codegen evidence test matrix

## Functional Requirement Coverage

| Functional Req | Acceptance Criteria | Test Cases | Status |
|---|---|---|---|
| FR-004 | FR-004-AC-1 through FR-004-AC-7 | TC-006 | 🚧 Planned / UNRUN; primitive controls are partial evidence, while production live campaign binding and consuming-obligation evidence are unrun |
| FR-004 | FR-004-AC-8 | TC-006 | 🚧 Planned / UNRUN; adverse-run coverage discharge through the consuming obligation is unrun |
| FR-004 | FR-004-AC-9 | TC-006 | 🚧 Planned |
| FR-004 | FR-004-AC-10 | TC-006 | 🚧 Planned / UNRUN; production live campaign/profile/export binding and CG consumption are unrun |
| FR-004 | FR-004-AC-11 | TC-006 | 🚧 Planned / UNRUN; consuming-obligation evidence and proof-credit denial are unrun |
| FR-004 | FR-004-AC-16, FR-004-AC-18 through FR-004-AC-24 | TC-006 | 🚧 V2 planned (IR-815); the V2 population analyzer is not implemented |
| FR-004 | FR-004-AC-17 | Inspection | 🚧 Planned / UNRUN (IR-815; pending IR-1013); V2 analyzer source inspection is unrun |
| FR-004 | FR-004-AC-25 | TC-006, TC-020 | SUPERSEDED by AC-26; original same-node/different-point case infeasible; no implementation credit |
| FR-004 | FR-004-AC-26 | TC-006, TC-020 | 🚧 Planned (IR-1070); node-qualified point admission and generated-campaign evidence binding are not implemented |
| FR-004 | FR-004-AC-12 | TC-006 | 🚧 Planned / UNRUN; QSL-520 public `/2` producer and strict reader are delivered, but CG consumer evidence is unrun |
| FR-004 | FR-004-AC-13 | TC-006 | 🚧 Planned / UNRUN; QSL-520 public `/2` producer and strict reader are delivered, but CG disposition and coverage-gate evidence is unrun |
| FR-004 | FR-004-AC-14 | TC-006 | 🚧 Planned / UNRUN; CG live context and producer/artifact binding evidence is unrun |
| FR-004 | FR-004-AC-15 | TC-006 | 🚧 Planned / UNRUN; CG foreign-run refusal and replay evidence is unrun |
| FR-036 | FR-036-AC-1 through FR-036-AC-7 | [TC-051](TC-051-source-obligation-census.md) | 🚧 Planned; separate inventories/complete candidate tuples and typed artifact/run controls Gated on IR-759 reviewed neutral correspondence; IR-740 owns implementation |

## Test Case Summary

The coverage tables above -- Functional, Interface, Non-Functional and
Stakeholder -- are the authority for how much of any criterion they list is
backed. Where a row here reads `✅ Covered` and a criterion in its Traces To
column is marked `⚠️` or `🚧` in the table that owns it, that table governs.

| Test ID | Title | Type | Priority | Traces To | Status |
|---|---|---|---|---|---|
| TC-006 | Distinguish vacuity and unexecuted flow | Integration | P0 | FR-004-AC-1, FR-004-AC-2, FR-004-AC-3, FR-004-AC-4, FR-004-AC-5, FR-004-AC-6, FR-004-AC-7, FR-004-AC-8, FR-004-AC-9, FR-004-AC-10, FR-004-AC-11, FR-004-AC-12, FR-004-AC-13, FR-004-AC-14, FR-004-AC-15, FR-004-AC-16, FR-004-AC-18, FR-004-AC-19, FR-004-AC-20, FR-004-AC-21, FR-004-AC-22, FR-004-AC-23, FR-004-AC-24, FR-004-AC-25, FR-004-AC-26 | 🚧 Planned / UNRUN; AC-25 superseded with no credit; bounded observation primitives are partial evidence only; QSL-520 public `/2` reader is delivered, while CG consumer, production live campaign binding and consuming-obligation evidence are unrun |
| TC-020 | Verify numeric conformance campaigns and rate reporting | Integration | P0 | FR-004-AC-25, FR-004-AC-26 | 🚧 Planned; AC-25 superseded with no credit; the point-qualified V2 runner and its refusals are not implemented |
| TC-051 | Source census is independent of attempts and preserves producer facts | Integration | P0 | FR-036-AC-1, FR-036-AC-2, FR-036-AC-3, FR-036-AC-4, FR-036-AC-5, FR-036-AC-6, FR-036-AC-7 | 🚧 Planned; Gated on IR-759; no executed producer/consumer test claimed |
