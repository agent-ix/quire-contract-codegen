---
id: TM-005
title: "Contract codegen routed generation test matrix"
type: TestMatrix
---

# Contract codegen routed generation test matrix

## Functional Requirement Coverage

| Functional Req | Acceptance Criteria | Test Cases | Status |
|---|---|---|---|
| FR-019 | FR-019-AC-1 (Kani arm), FR-019-AC-2 through FR-019-AC-4, FR-019-AC-7, FR-019-AC-8, FR-019-AC-10 | TC-030 | ⚠️ AC-1 process arm planned in TC-046; remaining listed criteria covered |
| FR-019 | FR-019-AC-9 | Analysis | ✅ Covered |
| FR-022 | FR-022-AC-2, FR-022-AC-3, FR-022-AC-5, FR-022-AC-7 through FR-022-AC-15 | TC-033 | ✅ Existing Kani behavior covered; test literals need constructor migration in IR-629 code |
| FR-022 | FR-022-AC-4 | TC-033, TC-046 | ⚠️ Existing direct-item mismatch test is tagged; a crate-internal test is planned when `backend` and `kind` become private |
| FR-022 | FR-022-AC-16 | TC-033 | ⚠️ Partially covered; asserted on the harness-pairing function (`index_harnesses`), not through `generate_routed`, because unique name assignment makes the duplicate unreachable from the public entry |
| FR-022 | FR-022-AC-6 | TC-033 | ⚠️ Partially covered; the out-of-range unwind and unparsable subject path refusals are asserted; the criterion's first example has no test |
| FR-022 | FR-022-AC-1 | Analysis | 🚧 Planned |
| FR-019 | FR-019-AC-1 (process arm), FR-019-AC-11 through FR-019-AC-13, FR-019-AC-16 through FR-019-AC-23 | TC-046 | 🚧 Planned; QSpec requires `ProofBound.kind`, while admitted-descriptor rows await QSL-654's producer and admit-side `invalid-domains` enforcement |
| FR-022 | FR-022-AC-17 through FR-022-AC-19 | TC-046 | 🚧 Planned |
| FR-022 | FR-022-AC-20 | Analysis | 🚧 Planned |
| FR-022 | FR-022-AC-21, FR-022-AC-22 | TC-046 | 🚧 Planned |
| FR-026 | FR-026-AC-5 | Analysis | 🚧 Planned |
| FR-019 | FR-019-AC-14 | Analysis | 🚧 Planned |
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
| TC-030 | Verify capability settlement at one negotiation point | Integration | P0 | FR-019-AC-1, FR-019-AC-2, FR-019-AC-3, FR-019-AC-4, FR-019-AC-7, FR-019-AC-8, FR-019-AC-10 | ✅ Covered |
| TC-033 | Verify routed generation per backend kind without re-negotiation | Integration | P0 | FR-022-AC-2, FR-022-AC-3, FR-022-AC-4, FR-022-AC-5, FR-022-AC-6, FR-022-AC-7, FR-022-AC-8, FR-022-AC-9, FR-022-AC-10, FR-022-AC-11, FR-022-AC-12, FR-022-AC-13, FR-022-AC-14, FR-022-AC-15, FR-022-AC-16, FR-015-AC-15, FR-015-AC-16, FR-015-AC-17, FR-015-AC-18 | ✅ Covered |
| TC-037 | Verify the backend adapter trait and its closed-enum dispatch | Integration | P0 | FR-026-AC-1, FR-026-AC-4 | 🚧 Planned |
| TC-046 | Verify process-provider settlement and empty generation | Integration | P0 | FR-019-AC-1 (process arm), FR-019-AC-11, FR-019-AC-12, FR-019-AC-13, FR-019-AC-16, FR-019-AC-17, FR-019-AC-18, FR-019-AC-19, FR-019-AC-20, FR-019-AC-21, FR-019-AC-22, FR-019-AC-23, FR-022-AC-17, FR-022-AC-18, FR-022-AC-19, FR-022-AC-21, FR-022-AC-22 | 🚧 Planned |
