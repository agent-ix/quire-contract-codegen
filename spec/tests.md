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
| Kani | FR-015, FR-017, FR-025, FR-028, FR-029, FR-030, NFR-006 | [kani](./kani/matrix/tests.md) | 🚧 several criteria are planned in the matrix rows; NFR-006 (TC-045) is planned |
| Routed | FR-019, FR-022, FR-026 | [routed](./routed/matrix/tests.md) | 🚧 FR-019 and FR-022 are covered except FR-019-AC-11 to AC-14 (TC-046 and analysis, planned), FR-022-AC-1, FR-022-AC-6 and FR-022-AC-16; FR-026 is planned |
| Replay | FR-016, FR-024, FR-032, FR-033 | [replay](./replay/matrix/tests.md) | 🚧 FR-016-AC-6, AC-7, AC-12 and FR-024-AC-1 to AC-10 and FR-024-AC-31 to AC-35 are planned; FR-032-AC-1 to AC-8 (TC-047) are Planned (IR-631), with the admitted scalar route Gated on QSL-641; FR-033-AC-1 to AC-10 and TC-048 are Planned (IR-635), UNVERIFIED SOURCE and Gated on actual QSL-640 API delivery |
| Evidence | FR-004 | [evidence](./evidence/matrix/tests.md) | 🚧 FR-004 (TC-006) is planned |
