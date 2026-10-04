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
| Core | StR-001, FR-005, interface-001, NFR-001, NFR-002, NFR-005 | [core](./core/matrix/tests.md) | 🚧 TC-001, TC-002, TC-003, TC-007 and TC-042 rows are planned |
| Strategy | FR-002, FR-008, FR-009, FR-010, FR-011, FR-012, FR-013, NFR-004 | [strategy](./strategy/matrix/tests.md) | 🚧 FR-008 to FR-013 and NFR-004 are covered; FR-002 (TC-004) is planned |
| Oracle | FR-014, FR-018, FR-021 | [oracle](./oracle/matrix/tests.md) | 🚧 several criteria are partial or planned in the matrix rows |
| Kani | FR-015, FR-017, FR-025, FR-028, FR-029, FR-030 | [kani](./kani/matrix/tests.md) | 🚧 several criteria are planned in the matrix rows |
| Routed | FR-019, FR-022, FR-026 | [routed](./routed/matrix/tests.md) | 🚧 FR-019 and FR-022 are covered except FR-022-AC-1, FR-022-AC-6 and FR-022-AC-16; FR-026 is planned |
| Replay | FR-016, FR-024 | [replay](./replay/matrix/tests.md) | 🚧 FR-016-AC-6, AC-7, AC-12 and FR-024 are planned |
| Evidence | FR-004 | [evidence](./evidence/matrix/tests.md) | 🚧 FR-004 (TC-006) is planned |
