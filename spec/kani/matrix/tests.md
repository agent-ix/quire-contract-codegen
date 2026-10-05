---
id: TM-004
title: "Contract codegen Kani test matrix"
type: TestMatrix
---

# Contract codegen Kani test matrix

## Functional Requirement Coverage

| Functional Req | Acceptance Criteria | Test Cases | Status |
|---|---|---|---|
| FR-015 | FR-015-AC-1, FR-015-AC-2 | TC-025 | 🚧 Planned |
| FR-015 | FR-015-AC-3 through FR-015-AC-6 | TC-025 | ✅ Covered |
| FR-015 | FR-015-AC-7 | TC-025 | ✅ Covered; every emitted harness kind (precondition, V1 contract, scalar, state-clause, frame-effect, V1 bundle, corpus) ends with its one cover, which FR-015-AC-53 to FR-015-AC-58 (IR-464) state and guard |
| FR-015 | FR-015-AC-8 through FR-015-AC-12 | TC-025 | ✅ Covered |
| FR-015 | FR-015-AC-13 | TC-025 | ✅ Covered |
| FR-015 | FR-015-AC-14 | TC-025 | ✅ Covered |
| FR-015 | FR-015-AC-15 | TC-033 | 🚧 Planned; the test's underivable claim is `integer.eq`, which FR-014-AC-35 makes derivable |
| FR-015 | FR-015-AC-16 through FR-015-AC-18 | TC-033 | ✅ Covered |
| FR-015 | FR-015-AC-19 through FR-015-AC-25 | TC-025 | 🚧 Planned |
| FR-015 | FR-015-AC-50 | TC-025 | ✅ Covered; negotiate reports a byte-ceiling and an unrecognised lowering refusal as `OracleRefused`, unchanged (IR-547) |
| FR-015 | FR-015-AC-26 through FR-015-AC-36 | TC-025 | ✅ Covered |
| FR-015 | FR-015-AC-37 | TC-025 | ✅ Covered |
| FR-015 | FR-015-AC-53, FR-015-AC-54, FR-015-AC-56, FR-015-AC-58 | TC-025 | ✅ Covered (IR-464); a cover as the last statement of every harness kind (`syn` inspection of each emitting entry point, and a scan of the non-test string literals of `src/` that fails a file the inspection does not drive), and the V1 bundle's real-Kani outcomes (`Verified` for a bundle whose requires some bounded argument satisfies and whose `ensures` holds for every such argument, `CoverUnsatisfied` for one whose requires no bounded argument satisfies, `Falsified` for a broken `ensures`), run in the `kani` lane |
| FR-015 | FR-015-AC-55, FR-015-AC-57 | TC-023 | ✅ Covered (IR-464); the corpus harness's cover after its assertion, a true case of each corpus family classifying `Verified` and a false graph and collection case `Falsified` with an empty-valued playback, run in the `kani` lane |
| FR-015 | FR-015-AC-51, FR-015-AC-52 | TC-023 | ✅ Covered; the profile mismatch refusal (AC-51) and the revision as the context of every corpus outcome and refusal (AC-52) |
| FR-015 | FR-015-AC-38 through FR-015-AC-49 | TC-025 | 🚧 Planned; the V2 clause claim and V2 census inputs (IR-489, AD-004 steps 4c and 4d). FR-015-AC-44 and FR-015-AC-45 back FR-015-AC-22 and FR-015-AC-25 once `ProofDependencyGraph` retires at step 4f; if they are not implemented by then, the row above holding AC-22 and AC-25 (FR-015-AC-19 through FR-015-AC-25) stays planned and unbacked, and no criterion is deleted or rewritten |
| FR-017 | FR-017-AC-2, FR-017-AC-4, FR-017-AC-5, FR-017-AC-12, FR-017-AC-13, FR-017-AC-15, FR-017-AC-16, FR-017-AC-17, FR-017-AC-18, FR-017-AC-19, FR-017-AC-20, FR-017-CON-2 | TC-027 | ✅ Covered |
| FR-017 | FR-017-AC-6, FR-017-AC-7, FR-017-AC-11, FR-017-CON-1 | TC-027 | 🚧 Planned |
| FR-017 | FR-017-AC-14, FR-017-AC-21 through FR-017-AC-25 | TC-043 | ✅ Covered (IR-277); the output-cap refusal (AC-14), harness batching (AC-21 to AC-23), group cleanup on a normal exit (AC-24) and the capture-failure refusal (AC-25), asserted against launcher stand-ins and in the `make kani` lane against real Kani |
| FR-025 | FR-025-AC-1 | TC-036 | 🚧 Planned; emission order is asserted only for the V1 `BoundClause` harness kinds, and the ascending order and the scalar-claim harness are unasserted |
| FR-025 | FR-025-AC-2 through FR-025-AC-8 | TC-036 | 🚧 Planned |
| FR-028 | FR-028-AC-1 through FR-028-AC-9 | TC-039 | 🚧 Planned |
| FR-028 | FR-028-AC-12 | TC-039 | ✅ Covered (IR-277); the batch wall-clock rule, asserted by the `tc_043_*` batch tests, which are tagged to both TC-039 and TC-043 |
| FR-029 | FR-029-AC-1, FR-029-AC-2, FR-029-AC-4 through FR-029-AC-6, FR-029-AC-8, FR-029-AC-9, FR-029-AC-11 through FR-029-AC-14 | TC-040 | ✅ Covered (IR-465); `run_terminal_value` in `kani/terminal.rs`, with the `From` conversions in `replay/` from each error the replay path returns, asserted over QSL's own refusal values; the `Inconclusive` terminal value is merged in QSL |
| FR-029 | FR-029-AC-3 | TC-040 | 🚧 Planned; the timed-out and exhausted-unwind-bound reasons are asserted, but `KaniInconclusiveReason` has no memory-exhausted reason until FR-028-AC-3 adds it, so the criterion is not backed as a whole |
| FR-029 | FR-029-AC-10 | TC-040 | 🚧 Planned; the fault readings (`ReplayRefusal::Fault`, `AdmissionFailure::Fault`, `CallSiteRefusal::Fault`, bare and wrapped) are built in the map and its conversions, but no test constructs a QSL `InternalFault`: `qsl-replay` does not re-export the type and this crate may depend on `qsl-replay` alone, so a fault value is unreachable from a test here |
| FR-030 | FR-030-AC-1 through FR-030-AC-13 | TC-041 | 🚧 Planned; the map is one `match` over every `KaniOutcomeKind`, and `Refused`, `InvalidInput` and `IncompleteInput` must map to `Declined`, whose QSL value now carries a `DeclineCode` with only a QSL catalog arm until IR-605 and QSL-351 add IR's arm; IR's `kani_*` cause is not a QSL catalog code, so no total map is buildable, and none is written |

## Non-Functional Requirement Coverage

| Non-Functional Req | Verification Method | Evidence/Test Cases | Status |
|---|---|---|---|
| NFR-006 | Test | TC-045 (NFR-006-AC-1 through NFR-006-AC-12, NFR-006-AC-18) | 🚧 Planned; the `kani-scope` and `kani-gate` targets, their scripts and `tests/it/kani_gate.rs` do not exist yet |
| NFR-006 | Inspection | NFR-006-AC-13 through NFR-006-AC-17 | 🚧 Planned; the pull-request and release-ticket evidence line is read by the merger, and no workflow enforces it |

## Test Case Summary

The coverage tables above -- Functional, Interface, Non-Functional and
Stakeholder -- are the authority for how much of any criterion they list is
backed. Where a row here reads `✅ Covered` and a criterion in its Traces To
column is marked `⚠️` or `🚧` in the table that owns it, that table governs.

| Test ID | Title | Type | Priority | Traces To | Status |
|---|---|---|---|---|---|
| TC-005 | Enforce Kani proof dependencies | Analysis | P0 | FR-015-AC-22, FR-015-AC-25, FR-015-AC-44, FR-015-AC-45 | 🚧 Planned |
| TC-014 | Verify bounded numeric and state Kani contracts | Analysis | P0 | FR-015-AC-11, FR-015-AC-19, FR-015-AC-24 | 🚧 Planned |
| TC-023 | Verify bounded Kani profile corpus parity | Integration | P0 | FR-015-AC-23, FR-015-AC-51, FR-015-AC-52, FR-015-AC-55, FR-015-AC-57 | 🚧 Planned |
| TC-025 | Verify separate bounded Kani obligations | Analysis | P0 | FR-015-AC-1, FR-015-AC-2, FR-015-AC-3, FR-015-AC-4, FR-015-AC-5, FR-015-AC-6, FR-015-AC-7, FR-015-AC-8, FR-015-AC-9, FR-015-AC-10, FR-015-AC-11, FR-015-AC-12, FR-015-AC-13, FR-015-AC-14, FR-015-AC-19, FR-015-AC-20, FR-015-AC-21, FR-015-AC-22, FR-015-AC-23, FR-015-AC-24, FR-015-AC-25, FR-015-AC-26, FR-015-AC-27, FR-015-AC-28, FR-015-AC-29, FR-015-AC-30, FR-015-AC-31, FR-015-AC-32, FR-015-AC-33, FR-015-AC-34, FR-015-AC-35, FR-015-AC-36, FR-015-AC-37, FR-015-AC-38, FR-015-AC-39, FR-015-AC-40, FR-015-AC-41, FR-015-AC-42, FR-015-AC-43, FR-015-AC-44, FR-015-AC-45, FR-015-AC-46, FR-015-AC-47, FR-015-AC-48, FR-015-AC-49, FR-015-AC-50, FR-015-AC-53, FR-015-AC-54, FR-015-AC-56, FR-015-AC-58 | 🚧 Planned |
| TC-027 | Verify Kani obligation execution and its evidence | Analysis | P0 | FR-017-AC-2, FR-017-AC-4, FR-017-AC-5, FR-017-AC-6, FR-017-AC-7, FR-017-AC-11, FR-017-AC-12, FR-017-AC-13, FR-017-AC-15, FR-017-AC-16, FR-017-AC-17, FR-017-AC-18, FR-017-AC-19, FR-017-AC-20, FR-017-CON-1, FR-017-CON-2 | 🚧 Planned |
| TC-043 | Verify Kani harness batching, the output cap and launcher cleanup | Integration | P0 | FR-017-AC-14, FR-017-AC-21, FR-017-AC-22, FR-017-AC-23, FR-017-AC-24, FR-017-AC-25 | ✅ Covered |
| TC-045 | Verify the real-Kani lane gate targets | Integration | P0 | NFR-006-AC-1, NFR-006-AC-2, NFR-006-AC-3, NFR-006-AC-4, NFR-006-AC-5, NFR-006-AC-6, NFR-006-AC-7, NFR-006-AC-8, NFR-006-AC-9, NFR-006-AC-10, NFR-006-AC-11, NFR-006-AC-12, NFR-006-AC-18 | 🚧 Planned |
| TC-036 | Verify the generated harness subject ABI | Integration | P0 | FR-025-AC-1, FR-025-AC-2, FR-025-AC-3, FR-025-AC-4, FR-025-AC-5, FR-025-AC-6, FR-025-AC-7, FR-025-AC-8 | 🚧 Planned |
| TC-039 | Verify bounded proof ceilings, their inconclusive reasons and the proof subject | Integration | P0 | FR-028-AC-1, FR-028-AC-2, FR-028-AC-3, FR-028-AC-4, FR-028-AC-5, FR-028-AC-6, FR-028-AC-7, FR-028-AC-8, FR-028-AC-9, FR-028-AC-12 | 🚧 Planned |
| TC-040 | Verify the total map from a Kani run outcome to QSL's terminal value | Integration | P0 | FR-029-AC-1, FR-029-AC-2, FR-029-AC-3, FR-029-AC-4, FR-029-AC-5, FR-029-AC-6, FR-029-AC-8, FR-029-AC-9, FR-029-AC-10, FR-029-AC-11, FR-029-AC-12, FR-029-AC-13, FR-029-AC-14 | 🚧 Planned; every criterion but FR-029-AC-3 and FR-029-AC-10 is covered in the Functional Requirement Coverage table above |
| TC-041 | Verify the total map from a Contract IR Kani outcome to QSL's terminal value | Integration | P0 | FR-030-AC-1, FR-030-AC-2, FR-030-AC-3, FR-030-AC-4, FR-030-AC-5, FR-030-AC-6, FR-030-AC-7, FR-030-AC-8, FR-030-AC-9, FR-030-AC-10, FR-030-AC-11, FR-030-AC-12, FR-030-AC-13 | 🚧 Planned |
