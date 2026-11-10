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
| FR-002 | FR-002-AC-7, FR-002-AC-8, FR-002-AC-10 through FR-002-AC-20, FR-002-AC-22, FR-002-AC-24 | TC-004 | 🚧 Planned (IR-1316; IR-815/IR-364): selected V2 harness and non-completed terminals are absent |
| FR-002 | FR-002-AC-9 | Inspection | 🚧 Planned (IR-1316; IR-815): harness generator still uses V1 typed bodies outside the V2 access path |
| FR-002 | FR-002-AC-25, FR-002-AC-26 | TC-004 | 🚧 Planned (IR-1316; IR-815): V2 harness structural identity propagation and text refusals are absent |
| FR-008 | FR-008-AC-1 through FR-008-AC-5, FR-008-CON-2 | TC-017 | 🚧 V2 planned (V1 tests exist) |
| FR-008 | FR-008-AC-6 | TC-017 | 🚧 V2 planned (V1 tests exist) |
| FR-008 | FR-008-AC-7 | TC-017 | 🚧 Planned (IR-1316; IR-364): exhaustive shared-oracle agreement is absent |
| FR-008 | FR-008-AC-9 | TC-017 | ✅ Covered by `v2_accessor_population_and_refusal_fields_agree`: six operators, both read provenances, all observations, census and typed refusal retention |
| FR-008 | FR-008-AC-8 | Inspection | 🚧 Planned (IR-1316; IR-364): FR-014 does not call the shared lowerer |
| FR-008 | FR-008-AC-10 | Inspection | ✅ Covered by source inspection recorded in FR-008; no runtime credit |
| FR-008 | FR-008-CON-3 | TC-017 | 🚧 V2 planned (V1 tests exist) |
| FR-008 | FR-008-CON-4 | TC-017 | 🚧 V2 planned (V1 tests exist) |
| FR-008 | FR-008-CON-1 | Inspection | 🚧 V2 planned (V1 tests exist) |
| FR-009 | FR-009-AC-1 through FR-009-AC-6 | TC-018 | 🚧 V2 planned (V1 tests exist) |
| FR-009 | FR-009-AC-7 | TC-018 | ✅ Covered by QSL-produced mirrored comparison populations and compiled exhaustive small-domain controls |
| FR-010 | FR-010-AC-1 through FR-010-AC-5 | TC-019 | 🚧 V2 planned (V1 tests exist) |
| FR-011 | FR-011-AC-1 through FR-011-AC-5 | TC-020 | 🚧 V2 planned (V1 tests exist) |
| FR-011 | FR-011-AC-10 | TC-020 | 🚧 Planned (IR-1316; IR-1124): no V2 unbound-model validation or complete driver-export controls |
| FR-011 | FR-011-AC-12 | TC-020 | ✅ Covered by repeated V2 source/map artifacts and generated-consumer execution |
| FR-011 | FR-011-AC-19 through FR-011-AC-21 | TC-020 | 🚧 Planned (IR-1316; IR-1079): identity mutation/body comparison, complete owner handoff and canonical-text refusal controls are absent |
| FR-011 | FR-011-AC-13 | TC-020 | 🚧 V2 selected-claim entry planned (IR-1072) |
| FR-011 | FR-011-AC-16, FR-011-AC-17 | TC-020 | ⚠️ Partial V2 runner/publication controls exist; full refusal census remains Planned (IR-1124) |
| FR-011 | FR-011-AC-18 | TC-020, TC-006 | 🚧 Planned (IR-1316; IR-1124): detached summaries lack FR-004 live evidence binding |
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
backed. The historical named tests exercise the V1 `BoundPackage` strategy and do not discharge V2
criteria. The explicitly covered V2 rows above use the checked-package path or their declared
inspection method; the other V2 rows remain Planned or partial.

| Test ID | Title | Type | Priority | Traces To | Status |
|---|---|---|---|---|---|
| TC-004 | Preserve shaped proptest strategies | Property | P0 | FR-002-AC-1, FR-002-AC-2, FR-002-AC-3, FR-002-AC-4, FR-002-AC-5, FR-002-AC-6, FR-002-AC-7, FR-002-AC-8, FR-002-AC-10, FR-002-AC-11, FR-002-AC-12, FR-002-AC-13, FR-002-AC-14, FR-002-AC-15, FR-002-AC-16, FR-002-AC-17, FR-002-AC-18, FR-002-AC-19, FR-002-AC-20, FR-002-AC-22, FR-002-AC-24, FR-002-AC-25, FR-002-AC-26 | 🚧 Planned; AC-7 to AC-20, AC-22 and replacement AC-25/26 are Planned (IR-1316; IR-815), AC-16/24 are Planned (IR-1316; IR-364); selected V2 harness and non-completed terminals are absent |
| TC-017 | Verify checked V2 clause strategy admission and refusal | Integration | P0 | FR-008-AC-1, FR-008-AC-2, FR-008-AC-3, FR-008-AC-4, FR-008-AC-5, FR-008-AC-6, FR-008-AC-7, FR-008-AC-9, FR-008-CON-2, FR-008-CON-3, FR-008-CON-4 | 🚧 Partial; AC-9 covered by public-accessor/census/refusal integration test; AC-10 covered by inspection; AC-7/8 Planned (IR-1316; IR-364) |
| TC-018 | Verify constructive satisfying and violating populations | Property | P0 | FR-009-AC-1, FR-009-AC-2, FR-009-AC-3, FR-009-AC-4, FR-009-AC-5, FR-009-AC-6, FR-009-AC-7 | 🚧 Partial; AC-7 covered on V2; AC-1 through AC-6 remain Planned |
| TC-019 | Verify domain and relation boundary censuses | Integration | P0 | FR-010-AC-1, FR-010-AC-2, FR-010-AC-3, FR-010-AC-4, FR-010-AC-5, NFR-004-AC-2 | 🚧 V2 planned (V1 tests exist) |
| TC-020 | Verify numeric conformance campaigns and rate reporting | Integration | P0 | FR-011-AC-1, FR-011-AC-2, FR-011-AC-3, FR-011-AC-4, FR-011-AC-5, FR-011-AC-10, FR-011-AC-12, FR-011-AC-13, FR-011-AC-14, FR-011-AC-15, FR-011-AC-16, FR-011-AC-17, FR-011-AC-18, FR-011-AC-19, FR-011-AC-20, FR-011-AC-21, NFR-004-AC-1 | 🚧 Partial; AC-12 covered; AC-10/18/19/20/21 Planned (IR-1316; IR-1124/IR-1079); other point and runner controls retain their scoped status |
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
