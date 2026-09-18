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
| FR-007 | FR-007-AC-1 through FR-007-AC-5 | TC-023 | ✅ Covered |
| FR-003 | FR-003-AC-1 | TC-005, TC-014 | ✅ Covered |
| FR-003 | FR-003-AC-2 | TC-007, TC-014 | ✅ Covered |
| FR-003 | FR-003-AC-3 | TC-003, TC-014 | ✅ Covered |
| FR-003 | FR-003-AC-4 | TC-005, TC-014 | ✅ Covered |
| FR-003 | FR-003-AC-5 | TC-014 | ✅ Covered |
| FR-003 | FR-003-AC-6 | TC-014 | ✅ Covered |
| FR-003 | FR-003-AC-7 | TC-014 | ✅ Covered |
| FR-003 | FR-003-AC-8 | TC-014 | ✅ Covered |
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
| FR-008 | FR-008-AC-1 through FR-008-AC-5, FR-008-CON-2 | TC-017 | ✅ Covered |
| FR-008 | FR-008-CON-1 | TC-017 | ✅ Covered |
| FR-009 | FR-009-AC-1 through FR-009-AC-6 | TC-018 | ✅ Covered |
| FR-010 | FR-010-AC-1 through FR-010-AC-5 | TC-019 | ✅ Covered |
| FR-011 | FR-011-AC-1 through FR-011-AC-5 | TC-020 | ✅ Covered |
| FR-012 | FR-012-AC-1 through FR-012-AC-3 | TC-021 | ✅ Covered |
| FR-012 | FR-012-AC-4 | TC-021 | ✅ Covered |
| FR-013 | FR-013-AC-1 through FR-013-AC-4 | TC-022 | ✅ Covered |
| FR-013 | FR-013-AC-5 | Inspection | ✅ Covered |
| FR-014 | FR-014-AC-1 through FR-014-AC-11 | TC-024 | ✅ Covered |
| FR-015 | FR-015-AC-1, FR-015-AC-2 | TC-025 | 🚧 Planned |
| FR-015 | FR-015-AC-3 through FR-015-AC-6 | TC-025 | ✅ Covered |
| FR-015 | FR-015-AC-7 through FR-015-AC-12 | TC-025 | ✅ Covered |
| FR-016 | FR-016-AC-1 through FR-016-AC-7 | TC-026 | 🚧 Planned |
| FR-017 | FR-017-AC-2 through FR-017-AC-5 | TC-027 | ✅ Covered |
| FR-017 | FR-017-AC-1, FR-017-AC-6, FR-017-AC-7, FR-017-CON-1, FR-017-CON-2 | TC-027 | 🚧 Planned |

The current TestMatrix structure and coverage selector both consume the shared `Status` column. The
former `Coverage Status` conflict was tracked in upstream spec-artifacts-process #77; this repository
retains no local checker or copied traceability implementation.

FR-001, FR-003, FR-006, and the numeric-strategy FR-008 through FR-013 slice are `✅ Covered` after
their ticket-scoped current-head reviews. FR-002, FR-004, FR-005, the remaining NFR rows, and StR
rows stay `🚧 Planned` until their complete ticket scopes are implemented and reviewed. The
shared-assurance migration did not promote those semantic rows.

The FR-006 rows are the only ones this migration claims, and they are `✅ Covered` because TC-008
through TC-013 are backed by tests in `tests/shared_assurance.rs` that invoke the gates rather than
reimplementing them. FR-003 is backed by the reviewed Boolean and numeric/state Kani implementation
and local SUITE-008. FR-004 has no complete implementation or suite.

FR-008 through FR-013 and NFR-004 are covered by TC-017 through TC-022 after the bounded-integer
oracle grammar landed in PR #29. The evidence includes exhaustive small-domain population and
shrink walks, exact boundary censuses, all supported clause-kind/population campaigns at 256 and
10,000 cases with zero global rejects, generated-consumer compilation, identity mutation, packaged
attestation validation/sealing, closing Rust review SR-016, and gap analysis SR-017.

FR-015 is split (codegen#49 slice A). `tests/kani_obligations.rs` backs AC-3 through AC-6: unbounded,
non-finite and upstream-blocked items, unsatisfiable IR bounds and every `caller_declared` V2 scalar
operation are refused with a typed reason and no harness, and the V1 harness assumptions constrain
only arguments to their IR `bounded_domain` bounds. The same file backs the V1 half of AC-1 and AC-2:
precondition, postcondition and invariant each lower to a separate harness whose identity carries IR
bounds and every Kani pin, and `make kani` verifies all three and falsifies a seeded defect with a
concrete counterexample under the committed backend pins, and reports jointly unsatisfiable
`requires` as `cover_unsatisfied` rather than verified.

AC-7 through AC-12 are added by codegen#56, which found the decisions that make an FR-015 harness
sound stated nowhere: the non-vacuity covers that are the only thing separating a proof from a
vacuous run, the sibling-precondition assumptions that make a postcondition harness prove a weaker
claim than the clause states, the hard-coded solver, the byte-identity of regeneration, the
IR-domain assumption on every symbolic argument, and the request-level ceilings. All six are backed
by the named tests in `tests/kani_obligations.rs` that already exercised them without a criterion to
bind to. AC-1 and AC-2 stay `🚧 Planned`. Frame
obligations cannot be generated from the merged IR: V1 `ClauseKind` has no frame kind, FR-014
refuses V2 `state` nodes as `NoFiniteEncoding`, and the by-value harness arguments cannot express
`kani::modifies`, so a frame needs a typed IR frame item first. Every V2 scalar obligation carries
only a caller-declared operation identity and is blocked on agent-ix/quire-specification#76. Model
and graph bounds are blocked on agent-ix/quire-spec-language#120.

FR-017 is the execution and evidence half of codegen#49, separated from FR-015 under codegen#55
because `src/kani_execution.rs` — pin measurement, the pre-run drift refusal, the seven-value
outcome vocabulary and the execution evidence document — had no owning requirement at all. AC-2
through AC-5 are `✅ Covered` by the default lane: the classification and pin-comparison unit tests
in `src/kani_execution.rs` run on every `cargo test` over the backend's own recorded output, and the
absent-launcher refusal is in `tests/kani_obligations.rs`. AC-1, AC-6, AC-7 and CON-1 are
`🚧 Planned`: they are backed only by the `#[ignore]`d `make kani` lane, which needs a real pinned
installation and is not a `make ci` gate. No timed-out criterion is written, because the run has no
wall-clock budget to fail one — that is codegen#58, and TC-027 records it as blocked rather than
specifying around it.

## Non-Functional Requirement Coverage

| Non-Functional Req | Verification Method | Evidence/Test Cases | Status |
|---|---|---|---|
| NFR-001 | Test | TC-001 (NFR-001-AC-1) | 🚧 Planned |
| NFR-001 | Test | TC-002 (NFR-001-AC-2, NFR-001-AC-3) | 🚧 Planned |
| NFR-002 | Test | TC-001 (NFR-002-AC-1, NFR-002-AC-2) | 🚧 Planned |
| NFR-002 | Test | TC-003 (NFR-002-AC-3) | 🚧 Planned |
| NFR-002 | Inspection | NFR-002-AC-4 | 🚧 Planned |
| NFR-004 | Test | TC-020 (NFR-004-AC-1) | ✅ Covered |
| NFR-004 | Test | TC-019 (NFR-004-AC-2) | ✅ Covered |

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
| TC-005 | Enforce Kani proof dependencies | Analysis | P0 | FR-003-AC-1 | ✅ Covered |
| TC-006 | Distinguish vacuity and unexecuted flow | Integration | P0 | FR-004-AC-1, FR-004-AC-2, FR-004-AC-3, FR-004-AC-5, FR-004-AC-6 | 🚧 Planned |
| TC-007 | Verify cross-backend semantic parity | Integration | P0 | FR-003-AC-2, FR-005-AC-4 | 🚧 Planned |
| TC-008 | Verify the shared component pins through the packaged matrix | Integration | P0 | FR-006-AC-1 | ✅ Covered |
| TC-009 | Verify Quoin intake without Quoin or Quire executing a producer | Integration | P0 | FR-006-AC-2 | ✅ Covered |
| TC-010 | Verify the sealed impact snapshot is the Quire export | Integration | P0 | FR-006-AC-3 | ✅ Covered |
| TC-012 | Verify the demonstrable verification outcomes stay distinguishable | Integration | P0 | FR-006-AC-5, NFR-002-AC-3 | ✅ Covered |
| TC-013 | Verify no local evidence framework remains | Integration | P0 | FR-006-AC-6, FR-006-AC-7 | ✅ Covered |
| TC-014 | Verify bounded numeric and state Kani contracts | Analysis | P0 | FR-003-AC-2, FR-003-AC-3, FR-003-AC-4, FR-003-AC-5, FR-003-AC-6, FR-003-AC-7, FR-003-AC-8 | ✅ Covered |
| TC-017 | Verify bound-clause domain derivation and refusal | Integration | P0 | FR-008-AC-1, FR-008-AC-2, FR-008-AC-3, FR-008-AC-4, FR-008-AC-5, FR-008-CON-1, FR-008-CON-2 | ✅ Covered |
| TC-018 | Verify constructive satisfying and violating populations | Property | P0 | FR-009-AC-1, FR-009-AC-2, FR-009-AC-3, FR-009-AC-4, FR-009-AC-5, FR-009-AC-6 | ✅ Covered |
| TC-019 | Verify domain and relation boundary censuses | Integration | P0 | FR-010-AC-1, FR-010-AC-2, FR-010-AC-3, FR-010-AC-4, FR-010-AC-5, NFR-004-AC-2 | ✅ Covered |
| TC-020 | Verify numeric conformance campaigns and rate reporting | Integration | P0 | FR-011-AC-1, FR-011-AC-2, FR-011-AC-3, FR-011-AC-4, FR-011-AC-5, NFR-004-AC-1 | ✅ Covered |
| TC-021 | Verify shrinking preserves numeric constraints | Property | P0 | FR-012-AC-1, FR-012-AC-2, FR-012-AC-3, FR-012-AC-4 | ✅ Covered |
| TC-022 | Verify strategy output is consumable without a local wire schema | Integration | P0 | FR-013-AC-1, FR-013-AC-2, FR-013-AC-3, FR-013-AC-4 | ✅ Covered |
| TC-023 | Verify bounded Kani profile corpus parity | Integration | P0 | FR-007-AC-1, FR-007-AC-2, FR-007-AC-3, FR-007-AC-4, FR-007-AC-5 | ✅ Covered |
| TC-024 | Verify exact complete-V1 scalar oracle generation and agreement | Integration | P0 | FR-014-AC-1, FR-014-AC-2, FR-014-AC-3, FR-014-AC-4, FR-014-AC-5, FR-014-AC-6, FR-014-AC-7, FR-014-AC-8, FR-014-AC-9, FR-014-AC-10, FR-014-AC-11 | ✅ Covered |
| TC-025 | Verify separate bounded Kani obligations | Analysis | P0 | FR-015-AC-1, FR-015-AC-2, FR-015-AC-3, FR-015-AC-4, FR-015-AC-5, FR-015-AC-6, FR-015-AC-7, FR-015-AC-8, FR-015-AC-9, FR-015-AC-10, FR-015-AC-11, FR-015-AC-12 | 🚧 Planned |
| TC-026 | Verify witness decoding and native replay | Integration | P0 | FR-016-AC-1, FR-016-AC-2, FR-016-AC-3, FR-016-AC-4, FR-016-AC-5, FR-016-AC-6, FR-016-AC-7 | 🚧 Planned |
| TC-027 | Verify pinned Kani obligation execution and its evidence | Analysis | P0 | FR-017-AC-1, FR-017-AC-2, FR-017-AC-3, FR-017-AC-4, FR-017-AC-5, FR-017-AC-6, FR-017-AC-7, FR-017-CON-1, FR-017-CON-2 | 🚧 Planned |

TC-001 through TC-003, TC-005, and TC-014 are covered after ticket-scoped current-head Rust review
and gap analysis. Together they establish deterministic identity-bearing artifacts, compilation and
independent evaluation for the supported grammar, proof-dependency closure, exact IR-domain Kani
bounds, healthy and falsifying pinned-backend executions, and exact fail-closed diagnostics. TC-004,
TC-006, and TC-007 remain planned until their complete backend/parity ticket scopes are independently
reviewed; the FR-003 portion of TC-007 is already covered by TC-014 without promoting TC-007 overall.

TC-008 through TC-013 are the shared-assurance migration's own rows and are covered by named tests.

TC-017 through TC-022 are backed by passing named tests in `tests/bound_strategy_generation.rs`,
`tests/bound_populations.rs`, and `tests/bound_census.rs`. Together they cover admission and ordered
refusals, constructive populations, boundary censuses, runtime accounting and replay, generated
consumer compilation, and Quoin attestation integration.

TC-004's generated-crate fixtures include deterministic mixed-campaign counts and distinguish
framework exhaustion with a retained floor result from a completed below-floor campaign. Its row
remains planned pending independent review of the complete issue #3 scope.

## Evidence Locations

Each row is specified in the same-ID document under `spec/test/`. `spec/evidence/suites.md` is the
suite registry: it names the command, tool and evidence kind for each suite. SUITE-008 is the reviewed
local evidence producer for TC-003, TC-005, TC-014, and the FR-003 portion of TC-007; SUITE-010 is
local pre-review evidence for the publication portion of TC-002. TC-008 through TC-013 are backed by
`tests/shared_assurance.rs`, whose `/// Trace:` comments are what Quire's census reads. SR-016 and
SR-017 record the closing code and gap reviews for TC-017 through TC-022.
