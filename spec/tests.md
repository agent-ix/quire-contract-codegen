---
id: TM-008
title: "quire-contract-codegen test matrix index"
type: TestMatrixIndex
---
# TM-008: quire-contract-codegen test matrix index

Each subsystem owns one matrix beside its requirements, per
`quire-contract-ir:ADR-0056`, whose registry is in [spec.md](./spec.md). This index declares no test case; the matrices are the coverage authority.

## Requirements Traceability

| Subsystem | Requirements | Local Matrix | Status |
|---|---|---|---|
| Core | StR-001, FR-005, interface-001, NFR-001, NFR-002, NFR-005 | [core](./core/matrix/tests.md) | 🚧 TC-001, TC-002, TC-003 and TC-007 rows are planned |
| Strategy | FR-002, FR-008, FR-009, FR-010, FR-011, FR-012, FR-013, NFR-004 | [strategy](./strategy/matrix/tests.md) | 🚧 FR-008 to FR-013 and NFR-004 are covered; FR-002 (TC-004) is planned |
| Oracle | FR-014, FR-018, FR-021, FR-031 | [oracle](./oracle/matrix/tests.md) | 🚧 several criteria are partial or planned in the matrix rows; FR-031 (TC-044) is implemented except FR-031-AC-11's QSL exemplar run (pending) and the divide and remainder criteria FR-031-AC-2, AC-6, AC-16, AC-17, AC-18 and AC-22 to AC-27, which are planned (IR-602) and, where they call `quire_exact::divide` or remove the interim code (all but AC-18, AC-23 and AC-24), gated on QSL moving to the single-member `divide` first |
| Kani | FR-015, FR-017, FR-025, FR-028, FR-029, FR-030, FR-034, NFR-006 | [kani](./kani/matrix/tests.md) | 🚧 several criteria are planned in the matrix rows; NFR-006 (TC-045) and FR-034 (TC-049, IR-639 original-caller ownership) are planned; guardian CODE and opt-in fixture support remain planned on merged containment (PR #295), independently of whole-parent IR-241 completion; AC-12 and AC-23/26 startup-cleanup/Analysis are Gated on IR-652 lifecycle delivery (Bootstrap INIT leak and inherited PDEATH artifact-unlink race); IR-639 process-only fixtures cannot mark these obligations Covered. |
| Routed | FR-019, FR-022, FR-026 | [routed](./routed/matrix/tests.md) | 🚧 Process cases of FR-019-AC-1, AC-11 to AC-13 and AC-16 to AC-23; FR-022-AC-17 to AC-22; and FR-026-AC-5 are planned for TC-046 or analysis. FR-026-AC-1 and AC-4 remain planned for TC-037; FR-022-AC-1, AC-6 and AC-16 remain open |
| Replay | FR-016, FR-024, FR-032, FR-033 | [replay](./replay/matrix/tests.md) | 🚧 FR-016-AC-6, AC-7, AC-12 and FR-024-AC-1 to AC-10 and FR-024-AC-31 to AC-35 are planned; FR-032-AC-1 to AC-14 (TC-047) are Planned (IR-631), with the admitted scalar route CODE-gated on owning vector merge/execution, typed scalar-family retention, IR-648 metadata/provenance, retained original context and driver/consumer conformance; FR-033-AC-1 to AC-13 and TC-048 are Planned (IR-635), UNVERIFIED SOURCE and Gated on actual QSL-640 API delivery |
| Evidence | FR-004 | [evidence](./evidence/matrix/tests.md) | 🚧 FR-004 (TC-006) is planned |
