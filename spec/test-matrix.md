---
id: TM-001
title: "Contract codegen v0.1 test matrix"
type: TestMatrix
---

# Contract codegen v0.1 test matrix

## Functional Requirement Coverage

| Functional Req | Acceptance Criteria | Test Cases | Status |
|---|---|---|---|
| FR-001 | FR-001-AC-1, FR-001-AC-3 | TC-001 | ✅ Covered |
| FR-001 | FR-001-AC-2 | TC-002 | ✅ Covered |
| FR-001 | FR-001-AC-4 | TC-003 | ✅ Covered |
| FR-001 | FR-001-AC-5 | TC-001, TC-006 | ✅ Covered |
| FR-001 | FR-001-AC-6 | TC-001, TC-002 | ✅ Covered |
| FR-001 | FR-001-AC-7 | TC-001 | ✅ Covered |
| FR-001 | FR-001-AC-8 | TC-002 | ✅ Covered |
| FR-002 | FR-002-AC-1 through FR-002-AC-6 | TC-004 | 🚧 Planned |
| FR-003 | FR-003-AC-1 | TC-005, TC-014 | 🚧 Planned |
| FR-003 | FR-003-AC-2 | TC-007, TC-014 | 🚧 Planned |
| FR-003 | FR-003-AC-3 | TC-003, TC-014 | 🚧 Planned |
| FR-003 | FR-003-AC-4 | TC-005, TC-014 | 🚧 Planned |
| FR-003 | FR-003-AC-5 | TC-014 | 🚧 Planned |
| FR-003 | FR-003-AC-6 | TC-014 | 🚧 Planned |
| FR-003 | FR-003-AC-7 | TC-014 | 🚧 Planned |
| FR-003 | FR-003-AC-8 | TC-014 | 🚧 Planned |
| FR-004 | FR-004-AC-1 through FR-004-AC-8 | TC-006 | 🚧 Planned |
| FR-004 | FR-004-AC-9 | TC-006 | 🚧 Planned |
| FR-005 | FR-005-AC-1 | TC-002 | 🚧 Planned |
| FR-005 | FR-005-AC-2 | TC-001 | 🚧 Planned |
| FR-005 | FR-005-AC-3 | Inspection | 🚧 Planned |
| FR-005 | FR-005-AC-4 | TC-007 | 🚧 Planned |
| FR-006 | FR-006-AC-1 | TC-008 | ✅ Covered |
| FR-006 | FR-006-AC-2 | TC-009 | ✅ Covered |
| FR-006 | FR-006-AC-3 | TC-010 | ✅ Covered |
| FR-006 | FR-006-AC-5 | TC-012 | ✅ Covered |
| FR-006 | FR-006-AC-6 | TC-013 | ✅ Covered |
| FR-006 | FR-006-AC-7 | TC-013 | ✅ Covered |

The current TestMatrix structure and coverage selector both consume the shared `Status` column. The
former `Coverage Status` conflict was tracked in upstream spec-artifacts-process #77; this repository
retains no local checker or copied traceability implementation.

Every FR-001 through FR-005, NFR, and StR row stays `🚧 Planned`. FR-001 through FR-003 have draft
implementations and local tests, but their complete ticket scopes have not been independently
reviewed at the current head. The shared-assurance migration did not perform that semantic review,
and promoting one of these rows remains a human-reviewed matrix change.

The FR-006 rows are the only ones this migration claims, and they are `✅ Covered` because TC-008
through TC-013 are backed by tests in `tests/shared_assurance.rs` that invoke the gates rather than
reimplementing them. FR-003 now has an implemented draft and SUITE-008, but remains planned pending
independent current-head review. FR-004 has no implementation or suite.

## Non-Functional Requirement Coverage

| Non-Functional Req | Verification Method | Evidence/Test Cases | Status |
|---|---|---|---|
| NFR-001 | Test | TC-001 (NFR-001-AC-1) | 🚧 Planned |
| NFR-001 | Test | TC-002 (NFR-001-AC-2, NFR-001-AC-3) | 🚧 Planned |
| NFR-002 | Test | TC-001 (NFR-002-AC-1, NFR-002-AC-2) | 🚧 Planned |
| NFR-002 | Test | TC-003 (NFR-002-AC-3) | 🚧 Planned |
| NFR-002 | Inspection | NFR-002-AC-4 | 🚧 Planned |

## Stakeholder Requirement Coverage

| Stakeholder Req | Trace to US/FR | Test/Validation | Status |
|---|---|---|---|
| StR-001 | StR-001-VC-1, FR-001 | TC-001 | 🚧 Planned |
| StR-001 | StR-001-VC-2, FR-003, FR-004 | TC-007 | 🚧 Planned |

## Test Case Summary

| Test ID | Title | Type | Priority | Traces To | Status |
|---|---|---|---|---|---|
| TC-001 | Reproduce artifacts and attestations | Integration | P0 | FR-001-AC-1, FR-001-AC-3, FR-005-AC-2, NFR-001-AC-1, NFR-002-AC-1, NFR-002-AC-2 | ✅ Covered |
| TC-002 | Compile and publish atomically | Integration | P0 | FR-001-AC-2, FR-001-AC-8, FR-005-AC-1, NFR-001-AC-2, NFR-001-AC-3 | ✅ Covered |
| TC-003 | Reject unsupported inputs explicitly | Integration | P0 | FR-001-AC-4, FR-003-AC-3, NFR-002-AC-3 | ✅ Covered |
| TC-004 | Preserve shaped proptest strategies | Property | P0 | FR-002-AC-1, FR-002-AC-2, FR-002-AC-3, FR-002-AC-4, FR-002-AC-5, FR-002-AC-6 | 🚧 Planned |
| TC-005 | Enforce Kani proof dependencies | Analysis | P0 | FR-003-AC-1 | 🚧 Planned |
| TC-006 | Distinguish vacuity and unexecuted flow | Integration | P0 | FR-004-AC-1, FR-004-AC-2, FR-004-AC-3, FR-004-AC-5, FR-004-AC-6 | 🚧 Planned |
| TC-007 | Verify cross-backend semantic parity | Integration | P0 | FR-003-AC-2, FR-005-AC-4 | 🚧 Planned |
| TC-008 | Verify the shared component pins through the packaged matrix | Integration | P0 | FR-006-AC-1 | ✅ Covered |
| TC-009 | Verify Quoin intake without Quoin or Quire executing a producer | Integration | P0 | FR-006-AC-2 | ✅ Covered |
| TC-010 | Verify the sealed impact snapshot is the Quire export | Integration | P0 | FR-006-AC-3 | ✅ Covered |
| TC-012 | Verify the demonstrable verification outcomes stay distinguishable | Integration | P0 | FR-006-AC-5, NFR-002-AC-3 | ✅ Covered |
| TC-013 | Verify no local evidence framework remains | Integration | P0 | FR-006-AC-6, FR-006-AC-7 | ✅ Covered |
| TC-014 | Verify bounded numeric and state Kani contracts | Analysis | P0 | FR-003-AC-2, FR-003-AC-3, FR-003-AC-4, FR-003-AC-5, FR-003-AC-6, FR-003-AC-7, FR-003-AC-8 | 🚧 Planned |

TC-001 through TC-003 are covered after issue #4's exact-head Rust review and gap analysis. Together
they establish deterministic identity-bearing artifacts, compilation and independent evaluation for
the Boolean and obligation-free bounded-integer grammar, public bound-package generation, atomic
publication controls, and exact fail-closed diagnostics. TC-004 through TC-007 remain planned until
their complete backend/parity ticket scopes are independently reviewed. TC-014 specifies the
numeric/state Kani increment, including exact IR-domain bounds, proof success, falsifying concrete
playback, unsupported bindings and deterministic Boolean compatibility; it remains planned until the
reviewed implementation and local SUITE-008 evidence exist at the same revision.

TC-008 through TC-013 are the shared-assurance migration's own rows and are covered by named tests.

TC-004's generated-crate fixtures include deterministic mixed-campaign counts and distinguish
framework exhaustion with a retained floor result from a completed below-floor campaign. Its row
remains planned pending independent review of the complete issue #3 scope.

## Evidence Locations

Each row is specified in the same-ID document under `spec/test/`. `spec/evidence/suites.md` is the
suite registry: it names the command, tool and evidence kind for each suite. SUITE-008 is the planned
local evidence producer for TC-003, TC-005, TC-014, and the FR-003 portion of TC-007; SUITE-010 is
local pre-review evidence for the publication portion of TC-002. Transcribed current-head evidence
and matrix promotion remain pending for the numeric/state increment. TC-008 through TC-013 are backed by
`tests/shared_assurance.rs`, whose `/// Trace:` comments are what Quire's census reads.
