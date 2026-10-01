---
id: TM-001
title: "Contract codegen v0.1 test matrix"
type: TestMatrix
---

# Contract codegen v0.1 test matrix

## Functional Requirement Coverage

| Functional Req | Acceptance Criteria | Test Cases | Status |
|---|---|---|---|
| FR-002 | FR-002-AC-1 through FR-002-AC-6 | TC-004 | 🚧 Planned |
| FR-004 | FR-004-AC-1 through FR-004-AC-8 | TC-006 | 🚧 Planned |
| FR-004 | FR-004-AC-9 | TC-006 | 🚧 Planned |
| FR-005 | FR-005-AC-1 | TC-002 | 🚧 Planned |
| FR-005 | FR-005-AC-2 | TC-001 | 🚧 Planned |
| FR-005 | FR-005-AC-3 | Inspection | 🚧 Planned |
| FR-005 | FR-005-AC-4 | TC-007 | 🚧 Planned |
| FR-005 | FR-005-AC-5 | TC-002 | 🚧 Planned |
| FR-008 | FR-008-AC-1 through FR-008-AC-5, FR-008-CON-2 | TC-017 | ✅ Covered |
| FR-008 | FR-008-CON-1 | Inspection | ✅ Covered |
| FR-009 | FR-009-AC-1 through FR-009-AC-6 | TC-018 | ✅ Covered |
| FR-010 | FR-010-AC-1 through FR-010-AC-5 | TC-019 | ✅ Covered |
| FR-011 | FR-011-AC-1 through FR-011-AC-5 | TC-020 | ✅ Covered |
| FR-012 | FR-012-AC-1 through FR-012-AC-3 | TC-021 | ✅ Covered |
| FR-012 | FR-012-AC-4 | TC-021 | ✅ Covered |
| FR-013 | FR-013-AC-1, FR-013-AC-2, FR-013-AC-4 | TC-022 | ✅ Covered |
| FR-013 | FR-013-AC-5 | Inspection | ✅ Covered |
| FR-014 | FR-014-AC-1, FR-014-AC-3, FR-014-AC-5 through FR-014-AC-11 | TC-024 | ✅ Covered |
| FR-014 | FR-014-AC-2 | TC-024 | ⚠️ Partially covered; every family generates and agrees with the runtime except text admission, which is refused today and has no generated-crate evidence: Contract IR refuses the node because no catalogued `convert` identity takes a `text` operand (QSL lowers no text admission); unblocked by a catalogued text-admission operation. `tc_024_text_admission_corpus_is_refused_by_ir_today` pins each refusal |
| FR-014 | FR-014-AC-4 | TC-024 | ⚠️ Partially covered; byte identity across repeated runs and request permutations and the claim-map order are asserted by regeneration; the criterion's remaining clause has no test |
| FR-014 | FR-014-AC-12 | TC-024 | ⚠️ Partially covered; a descriptor naming a different catalogued operation is discharged, but the clause covering a descriptor naming an operation the catalogue has no entry for is a defensive branch no fixture reaches -- the only such state is a same-width IEEE conversion, which the package builder refuses to construct |
| FR-014 | FR-014-AC-13 | TC-024 | ✅ Covered |
| FR-014 | FR-014-AC-14 | TC-024 | ⚠️ Partially covered; the `caller_declared` provenance is asserted for every refused and every never-inspected claim, but no test asserts that the identity such a claim reports is the request item's own descriptor-derived one |
| FR-014 | FR-014-AC-15 | TC-024 | ⚠️ Partially covered; the typed refusal and the withheld generated function are asserted, but no fixture requests a work-exhausted item alongside healthy ones, so the per-item isolation clause is unasserted |
| FR-014 | FR-014-AC-16 | TC-024 | ✅ Covered |
| FR-014 | FR-014-AC-17 | TC-024 | ✅ Covered |
| FR-014 | FR-014-AC-18 | TC-024 | 🚧 Planned; the derivation test's refused-identity case is `integer.eq`, which FR-014-AC-35 makes derivable. The text-admission selector case (`numeric.convert` with a text result) is no longer asserted: Contract IR refuses every text-admission node, so the TextAdmission derivation and emission paths (`src/exact_scalar.rs`) are reachable from no admitted package and untested, pending the owner's decision on text admission |
| FR-014 | FR-014-AC-19 | TC-024 | ✅ Covered |
| FR-014 | FR-014-AC-20 through FR-014-AC-25 | TC-024 | ✅ Covered |
| FR-014 | FR-014-AC-26 through FR-014-AC-34 | TC-024 | ✅ Covered |
| FR-014 | FR-014-AC-35 through FR-014-AC-37 | TC-024 | 🚧 Planned; Boolean and integer `eq`/`ne` nodes are refused as `OperationNotDerivable` |
| FR-015 | FR-015-AC-1, FR-015-AC-2 | TC-025 | 🚧 Planned |
| FR-015 | FR-015-AC-3 through FR-015-AC-6 | TC-025 | ✅ Covered |
| FR-015 | FR-015-AC-7 through FR-015-AC-12 | TC-025 | ✅ Covered |
| FR-015 | FR-015-AC-13 | TC-025 | ✅ Covered |
| FR-015 | FR-015-AC-14 | TC-025 | ✅ Covered |
| FR-015 | FR-015-AC-15 | TC-033 | 🚧 Planned; the test's underivable claim is `integer.eq`, which FR-014-AC-35 makes derivable |
| FR-015 | FR-015-AC-16 through FR-015-AC-18 | TC-033 | ✅ Covered |
| FR-015 | FR-015-AC-19 through FR-015-AC-25 | TC-025 | 🚧 Planned |
| FR-015 | FR-015-AC-26 through FR-015-AC-36 | TC-025 | ✅ Covered |
| FR-015 | FR-015-AC-37 | TC-025 | ✅ Covered |
| FR-016 | FR-016-AC-1 through FR-016-AC-5, FR-016-AC-8 through FR-016-AC-11, FR-016-AC-13, FR-016-AC-14 through FR-016-AC-20 | TC-026 | ✅ Covered |
| FR-016 | FR-016-AC-6, FR-016-AC-7, FR-016-AC-12 | TC-026 | 🚧 Planned |
| FR-017 | FR-017-AC-2, FR-017-AC-4, FR-017-AC-5, FR-017-AC-12, FR-017-AC-13, FR-017-AC-14, FR-017-AC-15, FR-017-AC-16, FR-017-AC-17, FR-017-CON-2 | TC-027 | ✅ Covered |
| FR-017 | FR-017-AC-6, FR-017-AC-7, FR-017-AC-11, FR-017-CON-1 | TC-027 | 🚧 Planned |
| FR-018 | FR-018-AC-1, FR-018-AC-3, FR-018-AC-6, FR-018-AC-11 through FR-018-AC-15 | TC-029 | ✅ Covered |
| FR-018 | FR-018-AC-2 | TC-029 | ⚠️ Partially covered; every admitted shape agrees with the runtime, but no recursive composite is generated or agreement-tested: Contract IR 0a889f9 refuses an equality over any type that reaches itself (as the QSpec reference reader does), so `E_SELF` (`{ next: Option<R_SELF> }`, no text, for which QSL emits `leaves: []`) is outside the corpus. Pending STD-129 (a cyclic type with no text: operator-ineligible or 0 leaves). `tc_029_a_cyclic_compared_type_is_refused_by_ir_today` pins the refusal |
| FR-018 | FR-018-AC-10 | TC-029 | ⚠️ Partially covered; byte identity across repeated runs and request permutations, the descriptor-key order, and the generated crate compiled and executed at test time under AC-2 are asserted; the criterion's remaining clause has no test |
| FR-018 | FR-018-AC-4, FR-018-AC-5, FR-018-AC-7 through FR-018-AC-9 | TC-029 | 🚧 Planned |
| FR-019 | FR-019-AC-1 through FR-019-AC-4, FR-019-AC-7, FR-019-AC-8, FR-019-AC-10 | TC-030 | ✅ Covered |
| FR-019 | FR-019-AC-9 | Analysis | ✅ Covered |
| FR-021 | FR-021-AC-1 through FR-021-AC-3, FR-021-AC-5 through FR-021-AC-14, FR-021-AC-17 | TC-031 | ✅ Covered |
| FR-021 | FR-021-AC-4 | TC-031 | ⚠️ Partially covered; only `InputRefusal::WrongValueKind` is asserted -- `::DanglingReference` is structurally unreachable for any oracle this generator can produce, since `validate_arguments` checks `WrongValueKind` before it ever walks a value for a dangling reference, and a reference-typed parameter is refused at generation time (AC-10) |
| FR-021 | FR-021-AC-15 | TC-031 | 🚧 Planned; the `origin` half is implemented and tested, but under this V1's scoped one-node body vocabulary `path` can never be non-empty by construction, so the `path`-non-empty case this AC also describes is not implemented |
| FR-021 | FR-021-AC-16 | TC-031 (Inspection) | ✅ Covered |
| FR-021 | FR-021-AC-18 | TC-031 | 🚧 Planned; the `quire-spec-language` authority leg is written and unasserted |
| FR-022 | FR-022-AC-2 through FR-022-AC-5, FR-022-AC-7 through FR-022-AC-15 | TC-033 | ✅ Covered |
| FR-022 | FR-022-AC-6 | TC-033 | ⚠️ Partially covered; the out-of-range unwind and unparsable subject path refusals are asserted; the criterion's first example has no test |
| FR-022 | FR-022-AC-1 | Analysis | 🚧 Planned |
| FR-024 | FR-024-AC-1 through FR-024-AC-10 | TC-035 | 🚧 Planned |
| FR-025 | FR-025-AC-1 | TC-036 | 🚧 Planned; emission order is asserted only for the V1 `BoundClause` harness kinds, and the ascending order and the scalar-claim harness are unasserted |
| FR-025 | FR-025-AC-2 through FR-025-AC-8 | TC-036 | 🚧 Planned |
| FR-026 | FR-026-AC-1, FR-026-AC-4 | TC-037 | 🚧 Planned |
| FR-028 | FR-028-AC-1 through FR-028-AC-9 | TC-039 | 🚧 Planned |
| FR-029 | FR-029-AC-1 through FR-029-AC-6 | TC-040 | 🚧 Planned |
| FR-030 | FR-030-AC-1 through FR-030-AC-8 | TC-041 | 🚧 Planned |

FR-018-AC-4, FR-018-AC-5, FR-018-AC-7 through FR-018-AC-9 are `🚧 Planned`: each names at least one
clause TC-029 carries no test for. AC-4 requires a schedule assertion for a top-level quantity pair;
the generator refuses `unit`/`dimension` scalars as `Unsupported { node_tag: "quantity" }`, so no
quantity item can reach a claim-map entry for that assertion to read. AC-5 lists six refused
conditions, each with its own `IllTypedCause`; only `convert<T>` outside `admits_equality_conversion`
is tested. Incompatible dimensions and distinct units are unreachable in the current corpus — both
need `ValueType::Quantity` operands, refused as `Unsupported` before `check_equality` runs — and
distinct text profiles, distinct enum declarations, no common type, and the "admits no charge on any
`Meter`" clause are untested but reachable. AC-7 names eight refused node forms across three distinct
blockers; the corpus exercises six (`reference`, `model`, `function`, `call`, `state`, `temporal`) and
confirms all three blocker values are distinct (`reference` is exercised as an operand reaching a
`reference` composite; the direct `reference` composite operand is refused by Contract IR at admission,
before the generator runs, and `tc_029_ac7_a_direct_reference_operand_is_refused_by_ir_today` pins that
refusal), but carries no `relation` or `protocol` node, so two
of the eight named forms are untested. AC-8 requires a declaration refusal for "both recursion passes
and a duplicate record field"; only the duplicate-field half is tested; no vector produces
`DeclarationCause::Recursion` in either pass. AC-9's conversion-charge clause ("each conversion charge
point in turn") is backed: `E_CONV_CHARGE` admits four conversion charge points —
`DecimalOperands`, `DecimalScaleExpansion`, `DecimalArithmetic`, `DecimalResultRetain` — and each is
denied in turn. Its counter clause ("every counter equals those of the same run stopped immediately
before that point") is not: `Outcome::Incomplete`'s `consumed` field and `Meter::consumed` read the
same array slot through the same accessor, so no comparison built from this repository's tests can
distinguish a runtime that snapshots a charge point before mutating it from one that mutates first and
snapshots the already-moved value; and `Meter::check_injected` reports `limit_kind: WorkUnits` for
every denial regardless of the charge's real kind, so only one of ten `LimitKind` counters is ever in
play.

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
| StR-001 | StR-001-VC-1, FR-014 | TC-024 | 🚧 Planned |
| StR-001 | StR-001-VC-2, FR-015, FR-004 | TC-007, TC-025 | 🚧 Planned |
| StR-001 | StR-001-VC-3, FR-016, FR-024 | TC-026, TC-035 | 🚧 Planned |
| StR-001 | StR-001-VC-4, FR-015, FR-017, FR-019 | TC-025, TC-027, TC-030 | 🚧 Planned |

## Test Case Summary

The coverage tables above -- Functional, Interface, Non-Functional and
Stakeholder -- are the authority for how much of any criterion they list is
backed. Where a row here reads `✅ Covered` and a criterion in its Traces To
column is marked `⚠️` or `🚧` in the table that owns it, that table governs.

| Test ID | Title | Type | Priority | Traces To | Status |
|---|---|---|---|---|---|
| TC-001 | Verify deterministic derivation | Integration | P0 | FR-005-AC-2, NFR-001-AC-1, NFR-002-AC-1, NFR-002-AC-2 | 🚧 Planned |
| TC-002 | Compile and publish atomically | Integration | P0 | FR-005-AC-1, FR-005-AC-5, NFR-001-AC-2, NFR-001-AC-3 | 🚧 Planned |
| TC-003 | Reject unsupported inputs explicitly | Integration | P0 | NFR-002-AC-3 | 🚧 Planned |
| TC-004 | Preserve shaped proptest strategies | Property | P0 | FR-002-AC-1, FR-002-AC-2, FR-002-AC-3, FR-002-AC-4, FR-002-AC-5, FR-002-AC-6 | 🚧 Planned |
| TC-005 | Enforce Kani proof dependencies | Analysis | P0 | FR-015-AC-22, FR-015-AC-25 | 🚧 Planned |
| TC-006 | Distinguish vacuity and unexecuted flow | Integration | P0 | FR-004-AC-1, FR-004-AC-2, FR-004-AC-3, FR-004-AC-5, FR-004-AC-6 | 🚧 Planned |
| TC-007 | Verify cross-backend semantic parity | Integration | P0 | FR-005-AC-4 | 🚧 Planned |
| TC-014 | Verify bounded numeric and state Kani contracts | Analysis | P0 | FR-015-AC-11, FR-015-AC-19, FR-015-AC-24 | 🚧 Planned |
| TC-017 | Verify bound-clause domain derivation and refusal | Integration | P0 | FR-008-AC-1, FR-008-AC-2, FR-008-AC-3, FR-008-AC-4, FR-008-AC-5, FR-008-CON-2 | ✅ Covered |
| TC-018 | Verify constructive satisfying and violating populations | Property | P0 | FR-009-AC-1, FR-009-AC-2, FR-009-AC-3, FR-009-AC-4, FR-009-AC-5, FR-009-AC-6 | ✅ Covered |
| TC-019 | Verify domain and relation boundary censuses | Integration | P0 | FR-010-AC-1, FR-010-AC-2, FR-010-AC-3, FR-010-AC-4, FR-010-AC-5, NFR-004-AC-2 | ✅ Covered |
| TC-020 | Verify numeric conformance campaigns and rate reporting | Integration | P0 | FR-011-AC-1, FR-011-AC-2, FR-011-AC-3, FR-011-AC-4, FR-011-AC-5, NFR-004-AC-1 | ✅ Covered |
| TC-021 | Verify shrinking preserves numeric constraints | Property | P0 | FR-012-AC-1, FR-012-AC-2, FR-012-AC-3, FR-012-AC-4 | ✅ Covered |
| TC-022 | Verify strategy output is consumable without a local wire schema | Integration | P0 | FR-013-AC-1, FR-013-AC-2, FR-013-AC-4 | ✅ Covered |
| TC-023 | Verify bounded Kani profile corpus parity | Integration | P0 | FR-015-AC-23 | 🚧 Planned |
| TC-024 | Verify exact complete-V1 scalar oracle generation and agreement | Integration | P0 | FR-014-AC-1, FR-014-AC-2, FR-014-AC-3, FR-014-AC-4, FR-014-AC-5, FR-014-AC-6, FR-014-AC-7, FR-014-AC-8, FR-014-AC-9, FR-014-AC-10, FR-014-AC-11, FR-014-AC-12, FR-014-AC-13, FR-014-AC-14, FR-014-AC-15, FR-014-AC-16, FR-014-AC-17, FR-014-AC-18, FR-014-AC-19, FR-014-AC-20, FR-014-AC-21, FR-014-AC-22, FR-014-AC-23, FR-014-AC-24, FR-014-AC-25, FR-014-AC-26, FR-014-AC-27, FR-014-AC-28, FR-014-AC-29, FR-014-AC-30, FR-014-AC-31, FR-014-AC-32, FR-014-AC-33, FR-014-AC-34, FR-014-AC-35, FR-014-AC-36, FR-014-AC-37 | 🚧 Planned |
| TC-025 | Verify separate bounded Kani obligations | Analysis | P0 | FR-015-AC-1, FR-015-AC-2, FR-015-AC-3, FR-015-AC-4, FR-015-AC-5, FR-015-AC-6, FR-015-AC-7, FR-015-AC-8, FR-015-AC-9, FR-015-AC-10, FR-015-AC-11, FR-015-AC-12, FR-015-AC-13, FR-015-AC-14, FR-015-AC-19, FR-015-AC-20, FR-015-AC-21, FR-015-AC-22, FR-015-AC-23, FR-015-AC-24, FR-015-AC-25, FR-015-AC-26, FR-015-AC-27, FR-015-AC-28, FR-015-AC-29, FR-015-AC-30, FR-015-AC-31, FR-015-AC-32, FR-015-AC-33, FR-015-AC-34, FR-015-AC-35, FR-015-AC-36, FR-015-AC-37 | 🚧 Planned |
| TC-026 | Verify witness decoding and native replay | Integration | P0 | FR-016-AC-1, FR-016-AC-2, FR-016-AC-3, FR-016-AC-4, FR-016-AC-5, FR-016-AC-6, FR-016-AC-7, FR-016-AC-8, FR-016-AC-9, FR-016-AC-10, FR-016-AC-11, FR-016-AC-12, FR-016-AC-13, FR-016-AC-14, FR-016-AC-15, FR-016-AC-16, FR-016-AC-17, FR-016-AC-18, FR-016-AC-19, FR-016-AC-20 | 🚧 Planned |
| TC-027 | Verify Kani obligation execution and its evidence | Analysis | P0 | FR-017-AC-2, FR-017-AC-4, FR-017-AC-5, FR-017-AC-6, FR-017-AC-7, FR-017-AC-11, FR-017-AC-12, FR-017-AC-13, FR-017-AC-14, FR-017-AC-15, FR-017-AC-16, FR-017-AC-17, FR-017-CON-1, FR-017-CON-2 | 🚧 Planned |
| TC-029 | Verify composite equality oracle generation and three-way agreement | Integration | P0 | FR-018-AC-1, FR-018-AC-2, FR-018-AC-3, FR-018-AC-4, FR-018-AC-5, FR-018-AC-6, FR-018-AC-7, FR-018-AC-8, FR-018-AC-9, FR-018-AC-10, FR-018-AC-11, FR-018-AC-12, FR-018-AC-13, FR-018-AC-14, FR-018-AC-15 | 🚧 Planned |
| TC-030 | Verify capability settlement at one negotiation point | Integration | P0 | FR-019-AC-1, FR-019-AC-2, FR-019-AC-3, FR-019-AC-4, FR-019-AC-7, FR-019-AC-8, FR-019-AC-10 | ✅ Covered |
| TC-031 | Verify function-application oracle generation, agreement, and static location tagging | Integration | P0 | FR-021-AC-1, FR-021-AC-2, FR-021-AC-3, FR-021-AC-4, FR-021-AC-5, FR-021-AC-6, FR-021-AC-7, FR-021-AC-8, FR-021-AC-9, FR-021-AC-10, FR-021-AC-11, FR-021-AC-12, FR-021-AC-13, FR-021-AC-14, FR-021-AC-15, FR-021-AC-16, FR-021-AC-17, FR-021-AC-18 | ✅ Covered |
| TC-033 | Verify routed generation per backend kind without re-negotiation | Integration | P0 | FR-022-AC-2, FR-022-AC-3, FR-022-AC-4, FR-022-AC-5, FR-022-AC-6, FR-022-AC-7, FR-022-AC-8, FR-022-AC-9, FR-022-AC-10, FR-022-AC-11, FR-022-AC-12, FR-022-AC-13, FR-022-AC-14, FR-022-AC-15, FR-015-AC-15, FR-015-AC-16, FR-015-AC-17, FR-015-AC-18 | ✅ Covered |
| TC-035 | Verify counterexample submission in QSL's counterexample envelope | Integration | P0 | FR-024-AC-1, FR-024-AC-2, FR-024-AC-3, FR-024-AC-4, FR-024-AC-5, FR-024-AC-6, FR-024-AC-7, FR-024-AC-8, FR-024-AC-9, FR-024-AC-10 | 🚧 Planned |
| TC-036 | Verify the generated harness subject ABI | Integration | P0 | FR-025-AC-1, FR-025-AC-2, FR-025-AC-3, FR-025-AC-4, FR-025-AC-5, FR-025-AC-6, FR-025-AC-7, FR-025-AC-8 | 🚧 Planned |
| TC-037 | Verify the backend adapter trait and its closed-enum dispatch | Integration | P0 | FR-026-AC-1, FR-026-AC-4 | 🚧 Planned |
| TC-039 | Verify bounded proof ceilings, their inconclusive reasons and the proof subject | Integration | P0 | FR-028-AC-1, FR-028-AC-2, FR-028-AC-3, FR-028-AC-4, FR-028-AC-5, FR-028-AC-6, FR-028-AC-7, FR-028-AC-8, FR-028-AC-9 | 🚧 Planned |
| TC-040 | Verify the total map from a Kani run outcome to QSL's terminal value | Integration | P0 | FR-029-AC-1, FR-029-AC-2, FR-029-AC-3, FR-029-AC-4, FR-029-AC-5, FR-029-AC-6 | 🚧 Planned |
| TC-041 | Verify the total map from a Contract IR Kani outcome to QSL's terminal value | Integration | P0 | FR-030-AC-1, FR-030-AC-2, FR-030-AC-3, FR-030-AC-4, FR-030-AC-5, FR-030-AC-6, FR-030-AC-7, FR-030-AC-8 | 🚧 Planned |

TC-017 through TC-022 are backed by passing named tests in `tests/bound_strategy_generation.rs`,
`tests/bound_populations.rs`, and `tests/bound_census.rs`. Together they cover admission and ordered
refusals, constructive populations, boundary censuses, runtime accounting and replay, and generated
consumer compilation.
