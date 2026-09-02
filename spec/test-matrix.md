---
id: TM-001
title: "Contract codegen v0.1 test matrix"
type: TestMatrix
---

# Contract codegen v0.1 test matrix

## Functional Requirement Coverage

| Functional Req | Acceptance Criteria | Test Cases | Coverage Status |
|---|---|---|---|
| FR-001 | FR-001-AC-1, FR-001-AC-3 | TC-001 | 🚧 Planned |
| FR-001 | FR-001-AC-2 | TC-002 | 🚧 Planned |
| FR-001 | FR-001-AC-4 | TC-003 | 🚧 Planned |
| FR-002 | FR-002-AC-1 through FR-002-AC-4 | TC-004 | 🚧 Planned |
| FR-003 | FR-003-AC-1 | TC-005 | 🚧 Planned |
| FR-003 | FR-003-AC-2 | TC-007 | 🚧 Planned |
| FR-003 | FR-003-AC-3 | TC-003 | 🚧 Planned |
| FR-003 | FR-003-AC-4 | Inspection | 🚧 Planned |
| FR-004 | FR-004-AC-1 through FR-004-AC-3 | TC-006 | 🚧 Planned |
| FR-004 | FR-004-AC-4 | Inspection | 🚧 Planned |
| FR-005 | FR-005-AC-1 | TC-002 | 🚧 Planned |
| FR-005 | FR-005-AC-2 | TC-001 | 🚧 Planned |
| FR-005 | FR-005-AC-3 | Inspection | 🚧 Planned |
| FR-005 | FR-005-AC-4 | TC-007 | 🚧 Planned |
| FR-006 | FR-006-AC-1 | TC-008 | ✅ Covered |
| FR-006 | FR-006-AC-2 | TC-009 | ✅ Covered |
| FR-006 | FR-006-AC-3 | TC-010 | ✅ Covered |
| FR-006 | FR-006-AC-4 | TC-011 | ✅ Covered |
| FR-006 | FR-006-AC-5 | TC-012 | ✅ Covered |
| FR-006 | FR-006-AC-6 | TC-013 | ✅ Covered |

The current coverage selector expects a `Status` column while the TestMatrix structure requires
`Coverage Status` (upstream spec-artifacts-process #77). The local checker that used to compensate
for that conflict was a second traceability implementation carrying a hand-copied matrix, and it went
with the rest of the generic evidence machinery; the conflict itself is unresolved and is carried as
an open unknown in `assurance/change-assurance.json`.

Every FR-001 through FR-005, NFR, and StR row stays `🚧 Planned`. Those requirements describe
semantic generation behaviour whose complete ticket scope has not been independently reviewed, and
the shared-assurance migration does not review it. Promoting one of those rows is a human-reviewed
matrix change.

The FR-006 rows are the only ones this migration claims, and they are `✅ Covered` because TC-008
through TC-013 are backed by tests in `tests/shared_assurance.rs` that invoke the gates rather than
reimplementing them. FR-003 and FR-004 have no implementation at this revision at all, which is why
they have no evidence suite and no proof obligation; their rows are planned in the strict sense that
nothing has been built.

## Nonfunctional Requirement Coverage

| Nonfunctional Req | Acceptance Criteria | Test/Inspection | Coverage Status |
|---|---|---|---|
| NFR-001 | NFR-001-AC-1 | TC-001 | 🚧 Planned |
| NFR-001 | NFR-001-AC-2, NFR-001-AC-3 | TC-002 | 🚧 Planned |
| NFR-002 | NFR-002-AC-1, NFR-002-AC-2 | TC-001 | 🚧 Planned |
| NFR-002 | NFR-002-AC-3 | TC-003 | 🚧 Planned |
| NFR-002 | NFR-002-AC-4 | Inspection | 🚧 Planned |

## Stakeholder Requirement Coverage

| Stakeholder Req | Trace to US/FR | Test/Validation | Coverage Status |
|---|---|---|---|
| StR-001 | StR-001-VC-1, FR-001 | TC-001 | 🚧 Planned |
| StR-001 | StR-001-VC-2, FR-003, FR-004 | TC-007 | 🚧 Planned |

## Test Case Summary

| Test ID | Title | Type | Priority | Traces To | Status |
|---|---|---|---|---|---|
| TC-001 | Reproduce artifacts and manifests | Integration | P0 | FR-001-AC-1, FR-001-AC-3, FR-005-AC-2, NFR-001-AC-1, NFR-002-AC-1, NFR-002-AC-2 | 🚧 Planned |
| TC-002 | Compile and publish atomically | Integration | P0 | FR-001-AC-2, FR-005-AC-1, NFR-001-AC-2, NFR-001-AC-3 | 🚧 Planned |
| TC-003 | Reject unsupported inputs explicitly | Integration | P0 | FR-001-AC-4, FR-003-AC-3, NFR-002-AC-3 | 🚧 Planned |
| TC-004 | Preserve shaped proptest strategies | Property | P0 | FR-002-AC-1, FR-002-AC-2, FR-002-AC-3, FR-002-AC-4 | 🚧 Planned |
| TC-005 | Enforce Kani proof dependencies | Analysis | P0 | FR-003-AC-1 | 🚧 Planned |
| TC-006 | Distinguish vacuity and unexecuted flow | Integration | P0 | FR-004-AC-1, FR-004-AC-2, FR-004-AC-3 | 🚧 Planned |
| TC-007 | Verify cross-backend semantic parity | Integration | P0 | FR-003-AC-2, FR-005-AC-4 | 🚧 Planned |
| TC-008 | Verify the shared component pins through the packaged matrix | Integration | P0 | FR-006-AC-1 | ✅ Covered |
| TC-009 | Verify Quoin intake without Quoin or Quire executing a producer | Integration | P0 | FR-006-AC-2 | ✅ Covered |
| TC-010 | Verify the sealed impact snapshot is the Quire export | Integration | P0 | FR-006-AC-3 | ✅ Covered |
| TC-011 | Verify retained evidence is readable and unmoved | Integration | P0 | FR-006-AC-4, NFR-002-AC-1 | ✅ Covered |
| TC-012 | Verify twelve verification outcomes stay distinguishable | Integration | P0 | FR-006-AC-5, NFR-002-AC-3 | ✅ Covered |
| TC-013 | Verify no local evidence framework remains | Integration | P0 | FR-006-AC-6 | ✅ Covered |

TC-001 through TC-007 remain deliberately planned until their complete ticket scope is independently
reviewed. The oracle draft carries bound implementation symbols for TC-001 through TC-003; TC-002
remains partial because atomic publication is not implemented, and no row is promoted by that draft.

TC-008 through TC-013 are the shared-assurance migration's own rows and are covered by named tests.

## Evidence Locations

Each row is specified in the same-ID document under `spec/test/`. `spec/evidence/suites.md` is the
suite registry: it names the command, tool and evidence kind for each suite whose results are
transcribed. Implementation evidence for TC-001 through TC-007 is pending. TC-008 through TC-013 are
backed by `tests/shared_assurance.rs`, whose `/// Trace:` comments are what Quire's census reads.
