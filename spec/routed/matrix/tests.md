---
id: TM-005
title: "Contract codegen routed generation test matrix"
type: TestMatrix
---

# Contract codegen routed generation test matrix

## Functional Requirement Coverage

| Functional Req | Acceptance Criteria | Test Cases | Status |
|---|---|---|---|
| FR-019 | FR-019-AC-4, FR-019-AC-7 | TC-030 | ✅ Covered; tagged settlement tests assert both registration orders and whole-envelope vocabulary refusal before kind settlement |
| FR-019 | FR-019-AC-45, FR-019-AC-46 | TC-030 | ✅ Covered; tagged tests assert whole-envelope repeated-identity refusal, descriptor and order cases, bytewise-first selection, case distinction, and carrier-refusal precedence |
| FR-019 | FR-019-AC-48 through FR-019-AC-52 | TC-030 | ✅ Covered; tagged TC-030 tests assert typed whole-request reader projections, first-failing-item order, admitted support and item-level backend refusal |
| FR-019 | FR-019-AC-53 | TC-030 | ✅ Covered; tagged tests assert bound-only envelope count, outer and nested driver indexes, no excluded settlement, indexed classifier and preflight observations, context-only root invariance, typed count refusals, and function-body lookup after exclusion |
| FR-019 | FR-019-AC-1 (Kani arm), FR-019-AC-2, FR-019-AC-3, FR-019-AC-8 | TC-030 | 🚧 Planned; generic TC-030 tests exist, but these criteria have no complete criterion-level test; AC-1 process arm remains planned in TC-046 |
| FR-019 | FR-019-AC-9, FR-019-AC-47 | Analysis | ✅ Covered |
| FR-019 | FR-019-AC-10 | Analysis | Analysis (CG); owner evidence in quire-driver FR-001-AC-3 (TC-003) and QSL FR-057-AC-6 (TC-155) |
| FR-022 | FR-022-AC-2, FR-022-AC-3, FR-022-AC-5, FR-022-AC-7 through FR-022-AC-15 | TC-033 | ✅ Existing Kani behavior covered; test literals need constructor migration in IR-629 code |
| FR-022 | FR-022-AC-4 | TC-033, TC-046 | ⚠️ Existing direct-item mismatch test is tagged; a crate-internal test is planned when `backend` and `kind` become private |
| FR-022 | FR-022-AC-16 | TC-033 | ⚠️ Partially covered; asserted on the harness-pairing function (`index_harnesses`), not through `generate_routed`, because unique name assignment makes the duplicate unreachable from the public entry |
| FR-022 | FR-022-AC-6 | TC-033 | ⚠️ Partially covered; the out-of-range unwind and unparsable subject path refusals are asserted; the criterion's first example has no test |
| FR-022 | FR-022-AC-1 | Analysis | 🚧 Planned |
| FR-022 | FR-022-AC-26 through FR-022-AC-28 | TC-033 | 🚧 Planned (IR-304): QSL per-occurrence records are available; CG typed record intake, guarded proof conditions and missing-record refusal await code |
| FR-022 | FR-022-AC-29 | TC-033 | ✅ Covered; tagged tests assert typed excluded driver-index refusal and valid bound routes at indexes 0 and 2 |
| FR-019 | FR-019-AC-1 (process arm), FR-019-AC-11 through FR-019-AC-13, FR-019-AC-16 through FR-019-AC-23 | TC-046 | 🚧 Planned; QSpec requires `ProofBound.kind`, while admitted-descriptor rows await QSL-654's producer and admit-side `invalid-domains` enforcement |
| FR-022 | FR-022-AC-17 through FR-022-AC-19 | TC-046 | 🚧 Planned |
| FR-022 | FR-022-AC-20 | Analysis | 🚧 Planned |
| FR-022 | FR-022-AC-21, FR-022-AC-22 | TC-046 | 🚧 Planned |
| FR-026 | FR-026-AC-5 | Analysis | 🚧 Planned |
| FR-045 | FR-045-AC-1, FR-045-AC-2 | TC-059 | 🚧 Planned (IR-974): waits for QSL-owned FR-315 HP-2 product query form; CG solver/map producer is not implemented |
| FR-019 | FR-019-AC-14 | Analysis | 🚧 Planned |
| FR-019 | FR-019-AC-24 | TC-030 | 🚧 Planned; generic extent rows do not verify QSL FR-335's concrete collection claim or composed driver outcome |
| FR-019 | FR-019-AC-25 through FR-019-AC-33 | TC-030 | 🚧 Planned; TC-030 specifies typed-form, planned-family refusal, candidate, mode, display-text and bound-identity controls |
| FR-019 | FR-019-AC-34 through FR-019-AC-41 | TC-030 | 🚧 Planned (IR-516); the model-checking negotiation arms and their criterion-level tests are not implemented |
| FR-026 | FR-026-AC-1, FR-026-AC-4 | TC-037 | 🚧 Planned |

FR-019-AC-9 is `✅ Covered` by analysis, not by a test. The dispatch is an exhaustive `match` over
`BackendKind` with no catch-all, so a variant added without an arm is a compile error; the evidence
is the compiler, and a test asserting a compile failure would need a `trybuild` lane this repository
does not have.

FR-022 is the generation arm of the seam FR-019 settles, and it takes the routed
backend and kind as given. AC-2 through AC-5 and AC-7 through AC-15 are backed by TC-033's tests. AC-4's "converts to a kind other than the routed one" branch needs a second `BackendKind` variant
to be reachable, since with one variant the only disagreement is a backend with no kind
(`converted: None`); TC-033 exercises only that `converted: None` case, so the other branch is untested until a second kind exists.
AC-1 stays `🚧 Planned`
because no test backs it: its evidence is the compiler's exhaustiveness check over
`BackendKind` and the one field per kind in `GenerationContexts`, not a test, the same ground as FR-019-AC-9.

## Test Case Summary

The coverage tables above -- Functional, Interface, Non-Functional and
Stakeholder -- are the authority for how much of any criterion they list is
backed. Where a row here reads `✅ Covered` and a criterion in its Traces To
column is marked `⚠️` or `🚧` in the table that owns it, that table governs.

| Test ID | Title | Type | Priority | Traces To | Status |
|---|---|---|---|---|---|
| TC-030 | Verify capability settlement at one negotiation point | Integration | P0 | FR-019-AC-1, FR-019-AC-2, FR-019-AC-3, FR-019-AC-4, FR-019-AC-7, FR-019-AC-8, FR-019-AC-45, FR-019-AC-46, FR-019-AC-48, FR-019-AC-49, FR-019-AC-50, FR-019-AC-51, FR-019-AC-52, FR-019-AC-53, FR-019-AC-24, FR-019-AC-25, FR-019-AC-26, FR-019-AC-27, FR-019-AC-28, FR-019-AC-29, FR-019-AC-30, FR-019-AC-31, FR-019-AC-32, FR-019-AC-33, FR-019-AC-34, FR-019-AC-35, FR-019-AC-36, FR-019-AC-37, FR-019-AC-38, FR-019-AC-39, FR-019-AC-40, FR-019-AC-41 | 🚧 Planned; FR-019-AC-4, FR-019-AC-7, FR-019-AC-45, FR-019-AC-46, and FR-019-AC-48 through FR-019-AC-53 have criterion-level tags and implementation; other criteria remain planned or only suite-tagged; QSL FR-335 collection composition is pending in quire-integration, and IR-516 arms remain future work |
| TC-033 | Verify routed generation per backend kind without re-negotiation | Integration | P0 | FR-022-AC-2, FR-022-AC-3, FR-022-AC-4, FR-022-AC-5, FR-022-AC-6, FR-022-AC-7, FR-022-AC-8, FR-022-AC-9, FR-022-AC-10, FR-022-AC-11, FR-022-AC-12, FR-022-AC-13, FR-022-AC-14, FR-022-AC-15, FR-022-AC-16, FR-022-AC-26, FR-022-AC-27, FR-022-AC-28, FR-022-AC-29, FR-015-AC-15, FR-015-AC-16, FR-015-AC-17, FR-015-AC-18 | 🚧 Existing cases and IR-915 covered; IR-304 cases planned |
| TC-037 | Verify the backend adapter trait and its closed-enum dispatch | Integration | P0 | FR-026-AC-1, FR-026-AC-4 | 🚧 Planned |
| TC-059 | Map QSL-encoded HP-2 induction solver results | Integration | P0 | FR-045-AC-1, FR-045-AC-2 | 🚧 Planned (IR-974): QSL hyper-product query form and CG SMT producer absent |
| TC-046 | Verify process-provider settlement and empty generation | Integration | P0 | FR-019-AC-1 (process arm), FR-019-AC-11, FR-019-AC-12, FR-019-AC-13, FR-019-AC-16, FR-019-AC-17, FR-019-AC-18, FR-019-AC-19, FR-019-AC-20, FR-019-AC-21, FR-019-AC-22, FR-019-AC-23, FR-022-AC-17, FR-022-AC-18, FR-022-AC-19, FR-022-AC-21, FR-022-AC-22 | 🚧 Planned |
