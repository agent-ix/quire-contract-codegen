---
id: TM-006
title: "Contract codegen replay test matrix"
type: TestMatrix
---

# Contract codegen replay test matrix

## Functional Requirement Coverage

| Functional Req | Acceptance Criteria | Test Cases | Status |
|---|---|---|---|
| FR-016 | FR-016-AC-1 through FR-016-AC-5, FR-016-AC-8 through FR-016-AC-11, FR-016-AC-13, FR-016-AC-14 through FR-016-AC-23 | TC-026 | ✅ Covered |
| FR-016 | FR-016-AC-6, FR-016-AC-7, FR-016-AC-12 | TC-026 | 🚧 Planned |
| FR-016 | FR-016-AC-24 | TC-026 | ✅ Covered (IR-611); `tc_026_a_lock_repeating_a_dependency_is_refused` asserts QSL's `DuplicateIdentity` refusal (`invalid_package`) arriving as `DependencyLockError::Input`, with no duplicate check of this crate's own |
| FR-024 | FR-024-AC-1 through FR-024-AC-10 | TC-035 | 🚧 Planned; no test of its own: `quire coverage --strict` stops flagging them only because TC-035 has tagged tests for AC-11 to AC-30 |
| FR-024 | FR-024-AC-11 through FR-024-AC-19 | TC-035 | ✅ Covered (IR-460); `StateClauseReplay` in `replay/state_clause.rs`, asserted over QSL's own `replay_state_clause` and `call_site` for the QSL twin of the state-frame fixture; FR-024-AC-18 is a real-Kani test of the `kani` lane (`make kani`) |
| FR-024 | FR-024-AC-20 through FR-024-AC-29 | TC-035 | ✅ Covered (IR-459); the frame path's one obligation identity (`replay/obligation.rs`), `state_fields` record, decoded witness, domain, pre-state and scope ties, asserted over QSL's own `call_site` and `replay_frame` for the QSL twin of the state-frame fixture; the former AC-30 real-Kani test is being migrated under IR-624 |
| FR-024 | FR-024-AC-30 through FR-024-AC-35 | TC-035 | 🚧 Planned (IR-624): emitted-package state field ranges come from `model_object_fields`, including unread declared fields, while input `state_fields` supplies draw order; replay validates names against the accessor and carries `TypeNotRange` for present fields without an `i64` range. IR-628 has merged; end-to-end replay, real-Kani cases and CG dependency update remain pending. Non-model and admitted unselected model/object_type bodies are refused without a range or replay; no body-member positive route remains. A selected/read nonempty-body tamper is rejected by IR admission as `StaleNodeKey` before CG. FR-024-AC-30 is restated over a QSL-emitted harness; AC-32 retires `Twin::aligned`. |
| FR-032 | FR-032-AC-1 through FR-032-AC-8 | TC-047 | 🚧 Planned (IR-631); admitted scalar route Gated on QSL-641 scalar arm, native-observation/proof-content tie, AD-002 R-6/R-7 contract fit and legal scalar settlement causes. No executable scalar replay coverage is claimed. |
| FR-033 | FR-033-AC-1 through FR-033-AC-13 | TC-048 | 🚧 Planned (IR-635), UNVERIFIED SOURCE and Gated on actual QSL-640 API delivery; canonical/node-selected parity, exact O-09 versus full-CG/content roles, shared Refinement and staged same-artifact observation; no executable coverage claimed |

## Test Case Summary

The coverage tables above -- Functional, Interface, Non-Functional and
Stakeholder -- are the authority for how much of any criterion they list is
backed. Where a row here reads `✅ Covered` and a criterion in its Traces To
column is marked `⚠️` or `🚧` in the table that owns it, that table governs.

| Test ID | Title | Type | Priority | Traces To | Status |
|---|---|---|---|---|---|
| TC-026 | Verify witness decoding and native replay | Integration | P0 | FR-016-AC-1, FR-016-AC-2, FR-016-AC-3, FR-016-AC-4, FR-016-AC-5, FR-016-AC-6, FR-016-AC-7, FR-016-AC-8, FR-016-AC-9, FR-016-AC-10, FR-016-AC-11, FR-016-AC-12, FR-016-AC-13, FR-016-AC-14, FR-016-AC-15, FR-016-AC-16, FR-016-AC-17, FR-016-AC-18, FR-016-AC-19, FR-016-AC-20, FR-016-AC-21, FR-016-AC-22, FR-016-AC-23, FR-016-AC-24 | 🚧 Planned |
| TC-035 | Verify counterexample submission in QSL's counterexample envelope | Integration | P0 | FR-024-AC-1, FR-024-AC-2, FR-024-AC-3, FR-024-AC-4, FR-024-AC-5, FR-024-AC-6, FR-024-AC-7, FR-024-AC-8, FR-024-AC-9, FR-024-AC-10, FR-024-AC-11, FR-024-AC-12, FR-024-AC-13, FR-024-AC-14, FR-024-AC-15, FR-024-AC-16, FR-024-AC-17, FR-024-AC-18, FR-024-AC-19, FR-024-AC-20, FR-024-AC-21, FR-024-AC-22, FR-024-AC-23, FR-024-AC-24, FR-024-AC-25, FR-024-AC-26, FR-024-AC-27, FR-024-AC-28, FR-024-AC-29, FR-024-AC-30, FR-024-AC-31, FR-024-AC-32, FR-024-AC-33, FR-024-AC-34, FR-024-AC-35 | 🚧 Planned |
| TC-047 | Check public routed scalar lowering replay and generated-content binding | Integration | P0 | FR-032-AC-1, FR-032-AC-2, FR-032-AC-3, FR-032-AC-4, FR-032-AC-5, FR-032-AC-6, FR-032-AC-7, FR-032-AC-8, FR-029-AC-15 | 🚧 Planned (IR-631); admitted route Gated on QSL-641; no implementation claimed |
| TC-048 | Check composite node parity replay, canonical values and shadow settlement | Integration | P0 | FR-033-AC-1, FR-033-AC-2, FR-033-AC-3, FR-033-AC-4, FR-033-AC-5, FR-033-AC-6, FR-033-AC-7, FR-033-AC-8, FR-033-AC-9, FR-033-AC-10, FR-033-AC-11, FR-033-AC-12, FR-033-AC-13, FR-025-AC-9, FR-028-AC-17, FR-028-AC-24, FR-029-AC-17, FR-029-AC-19, FR-029-AC-20, FR-029-AC-21, FR-029-AC-22, FR-029-AC-23, FR-029-AC-24, FR-029-AC-25, FR-029-AC-26, FR-029-AC-27 | 🚧 Planned (IR-635), Gated on actual QSL-640 delivery; scenario prose provides no executable coverage |
