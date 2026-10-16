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
| FR-002 | FR-002-AC-7, FR-002-AC-8, FR-002-AC-10 through FR-002-AC-18 | TC-004 | 🚧 V2 planned |
| FR-002 | FR-002-AC-9 | Inspection | 🚧 V2 planned |
| FR-008 | FR-008-AC-1 through FR-008-AC-5, FR-008-CON-2 | TC-017 | 🚧 V2 planned (V1 tests exist) |
| FR-008 | FR-008-AC-6 | TC-017 | 🚧 V2 planned (V1 tests exist) |
| FR-008 | FR-008-AC-7, FR-008-AC-9 | TC-017 | 🚧 V2 planned |
| FR-008 | FR-008-AC-8, FR-008-AC-10 | Inspection | 🚧 V2 planned |
| FR-008 | FR-008-CON-3 | TC-017 | 🚧 V2 planned (V1 tests exist) |
| FR-008 | FR-008-CON-4 | TC-017 | 🚧 V2 planned (V1 tests exist) |
| FR-008 | FR-008-CON-1 | Inspection | 🚧 V2 planned (V1 tests exist) |
| FR-009 | FR-009-AC-1 through FR-009-AC-7 | TC-018 | 🚧 V2 planned (V1 tests exist) |
| FR-010 | FR-010-AC-1 through FR-010-AC-5 | TC-019 | 🚧 V2 planned (V1 tests exist) |
| FR-011 | FR-011-AC-1 through FR-011-AC-5 | TC-020 | 🚧 V2 planned (V1 tests exist) |
| FR-012 | FR-012-AC-1 through FR-012-AC-3 | TC-021 | 🚧 V2 planned (V1 tests exist) |
| FR-012 | FR-012-AC-4 | TC-021 | 🚧 V2 planned (V1 tests exist) |
| FR-013 | FR-013-AC-1, FR-013-AC-2, FR-013-AC-4 | TC-022 | 🚧 V2 planned (V1 tests exist) |
| FR-013 | FR-013-AC-5 | Inspection | 🚧 V2 planned (V1 tests exist) |

## Non-Functional Requirement Coverage

| Non-Functional Req | Verification Method | Evidence/Test Cases | Status |
|---|---|---|---|
| NFR-004 | Test | TC-020 (NFR-004-AC-1) | 🚧 V2 planned (V1 tests exist) |
| NFR-004 | Test | TC-019 (NFR-004-AC-2) | 🚧 V2 planned (V1 tests exist) |

## Test Case Summary

The coverage tables above -- Functional, Interface, Non-Functional and
Stakeholder -- are the authority for how much of any criterion they list is
backed. The existing named tests exercise the V1 `BoundPackage` strategy. They do not discharge V2
criteria. The V2 rows remain planned until tagged tests run on the public checked-package path.

| Test ID | Title | Type | Priority | Traces To | Status |
|---|---|---|---|---|---|
| TC-004 | Preserve shaped proptest strategies | Property | P0 | FR-002-AC-1, FR-002-AC-2, FR-002-AC-3, FR-002-AC-4, FR-002-AC-5, FR-002-AC-6, FR-002-AC-7, FR-002-AC-8, FR-002-AC-10, FR-002-AC-11, FR-002-AC-12, FR-002-AC-13, FR-002-AC-14, FR-002-AC-15, FR-002-AC-16, FR-002-AC-17, FR-002-AC-18 | 🚧 Planned; AC-7 to AC-18 are V2 planned (IR-815) |
| TC-017 | Verify checked V2 clause strategy admission and refusal | Integration | P0 | FR-008-AC-1, FR-008-AC-2, FR-008-AC-3, FR-008-AC-4, FR-008-AC-5, FR-008-AC-6, FR-008-AC-7, FR-008-AC-9, FR-008-CON-2, FR-008-CON-3, FR-008-CON-4 | 🚧 V2 planned (V1 tests exist) |
| TC-018 | Verify constructive satisfying and violating populations | Property | P0 | FR-009-AC-1, FR-009-AC-2, FR-009-AC-3, FR-009-AC-4, FR-009-AC-5, FR-009-AC-6, FR-009-AC-7 | 🚧 V2 planned (V1 tests exist) |
| TC-019 | Verify domain and relation boundary censuses | Integration | P0 | FR-010-AC-1, FR-010-AC-2, FR-010-AC-3, FR-010-AC-4, FR-010-AC-5, NFR-004-AC-2 | 🚧 V2 planned (V1 tests exist) |
| TC-020 | Verify numeric conformance campaigns and rate reporting | Integration | P0 | FR-011-AC-1, FR-011-AC-2, FR-011-AC-3, FR-011-AC-4, FR-011-AC-5, NFR-004-AC-1 | 🚧 V2 planned (V1 tests exist) |
| TC-021 | Verify shrinking preserves numeric constraints | Property | P0 | FR-012-AC-1, FR-012-AC-2, FR-012-AC-3, FR-012-AC-4 | 🚧 V2 planned (V1 tests exist) |
| TC-022 | Verify strategy output is consumable without a local wire schema | Integration | P0 | FR-013-AC-1, FR-013-AC-2, FR-013-AC-4 | 🚧 V2 planned (V1 tests exist) |

The V1 form of TC-017 through TC-022 is backed by passing named tests in `tests/it/bound_strategy_generation.rs`,
`tests/it/bound_populations.rs`, and `tests/it/bound_census.rs`. Together they cover admission and ordered
refusals, constructive populations, boundary censuses, runtime accounting and replay, and generated
consumer compilation.

The FR-008 V2 scope depends on IR FR-038-AC-202 through AC-209 (IR-703), which are implemented in
Contract IR with tagged local controls (IR FR-038 and TC-445 on the measured IR main head); IR's
fresh QSL producer-origin case (AC-204) runs only under its conformance gate and no result of that
gate is claimed here. CG's ConfigVersion producer control still needs a CG-side real QSL V2
fixture. Prior V1 coverage is historical evidence only for the V2 contract.
