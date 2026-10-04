---
id: TM-003
title: "Contract codegen oracle test matrix"
type: TestMatrix
---

# Contract codegen oracle test matrix

## Functional Requirement Coverage

| Functional Req | Acceptance Criteria | Test Cases | Status |
|---|---|---|---|
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
| FR-014 | FR-014-AC-38 | TC-024 | 🚧 Planned; the oracle of an inline V2 clause term (IR-489, FR-015-AC-40) |
| FR-014 | FR-014-AC-35 through FR-014-AC-37 | TC-024 | 🚧 Planned; Boolean and integer `eq`/`ne` nodes are refused as `OperationNotDerivable` |
| FR-018 | FR-018-AC-1, FR-018-AC-3, FR-018-AC-6, FR-018-AC-11 through FR-018-AC-15 | TC-029 | ✅ Covered |
| FR-018 | FR-018-AC-2 | TC-029 | ⚠️ Partially covered; every admitted shape agrees with `quire_contract_runtime`'s own check and evaluation, the recursive composite `E_SELF` (`{ next: Option<R_SELF> }`, no text, for which QSL emits `leaves: []`) included: Contract IR now admits an equality over a type that reaches itself, so it is in the corpus with its generated-crate agreement vectors. The agreement with `quire_spec_language::value` has not run in this repository since `agree3!` was deleted; it is delegated to Contract Runtime's conformance lane, and nothing here shows that lane covers a recursive record such as `E_SELF` |
| FR-018 | FR-018-AC-16 | TC-029 | ⚠️ Partially covered; the text, integer, decimal and rational bases (read as their bounded scalars), the never-a-sibling-bound rule, the form-mismatch refusal, the boolean, enum and record base refusals, and a `float_rounding` member's OperatorIneligible refusal are each asserted. The form-mismatch refusal is exercised over an integer base only: a mismatched bound over a rational, decimal, text, `float32` or `float64` base shares the one form table but has no test of its own |
| FR-018 | FR-018-AC-10 | TC-029 | ⚠️ Partially covered; byte identity across repeated runs and request permutations, the descriptor-key order, and the generated crate compiled and executed at test time under AC-2 are asserted; the criterion's remaining clause has no test |
| FR-018 | FR-018-AC-4, FR-018-AC-5, FR-018-AC-7 through FR-018-AC-9 | TC-029 | 🚧 Planned |
| FR-018 | FR-018-AC-17 through FR-018-AC-19 | TC-029 | ✅ Covered |
| FR-018 | FR-018-AC-20 | TC-029 | ✅ Covered; the equality generator's refusals of a byte-ceiling and of an unrecognised lowering failure (IR-547) |
| FR-014 | FR-014-AC-39 | TC-024 | ✅ Covered |
| FR-014 | FR-014-AC-40 through FR-014-AC-42 | TC-024 | ✅ Covered; a byte-ceiling lowering failure is its own refusal, `LoweringByteLimitExceeded`, and `LoweringWorkExhausted` stays the work ceiling's alone (IR-547) |
| FR-021 | FR-021-AC-1 through FR-021-AC-3, FR-021-AC-5 through FR-021-AC-14, FR-021-AC-17 | TC-031 | ✅ Covered |
| FR-021 | FR-021-AC-4 | TC-031 | ⚠️ Partially covered; only `InputRefusal::WrongValueKind` is asserted -- `::DanglingReference` is structurally unreachable for any oracle this generator can produce, since `validate_arguments` checks `WrongValueKind` before it ever walks a value for a dangling reference, and a reference-typed parameter is refused at generation time (AC-10) |
| FR-021 | FR-021-AC-15 | TC-031 | 🚧 Planned; the `origin` half is implemented and tested, but under this V1's scoped one-node body vocabulary `path` can never be non-empty by construction, so the `path`-non-empty case this AC also describes is not implemented |
| FR-021 | FR-021-AC-16 | TC-031 (Inspection) | ✅ Covered |
| FR-021 | FR-021-AC-18 | TC-031 | 🚧 Planned; the `quire-spec-language` authority leg is written and unasserted |
| FR-021 | FR-021-AC-19 through FR-021-AC-22 | TC-031 | ✅ Covered |
| FR-021 | FR-021-AC-23 | TC-031 | ✅ Covered; the function generator's refusals of a byte-ceiling and of an unrecognised lowering failure (IR-547) |
| FR-021 | FR-021-AC-24 | TC-031 | ✅ Covered; one `UnknownFunction` entry per unknown name, and one entry per duplicate-node pair member, on one call node, in byte order of the name when equal on the earlier key fields and by declaring node id otherwise (IR-545) |

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

## Test Case Summary

The coverage tables above -- Functional, Interface, Non-Functional and
Stakeholder -- are the authority for how much of any criterion they list is
backed. Where a row here reads `✅ Covered` and a criterion in its Traces To
column is marked `⚠️` or `🚧` in the table that owns it, that table governs.

| Test ID | Title | Type | Priority | Traces To | Status |
|---|---|---|---|---|---|
| TC-024 | Verify exact complete-V1 scalar oracle generation and agreement | Integration | P0 | FR-014-AC-1, FR-014-AC-2, FR-014-AC-3, FR-014-AC-4, FR-014-AC-5, FR-014-AC-6, FR-014-AC-7, FR-014-AC-8, FR-014-AC-9, FR-014-AC-10, FR-014-AC-11, FR-014-AC-12, FR-014-AC-13, FR-014-AC-14, FR-014-AC-15, FR-014-AC-16, FR-014-AC-17, FR-014-AC-18, FR-014-AC-19, FR-014-AC-20, FR-014-AC-21, FR-014-AC-22, FR-014-AC-23, FR-014-AC-24, FR-014-AC-25, FR-014-AC-26, FR-014-AC-27, FR-014-AC-28, FR-014-AC-29, FR-014-AC-30, FR-014-AC-31, FR-014-AC-32, FR-014-AC-33, FR-014-AC-34, FR-014-AC-35, FR-014-AC-36, FR-014-AC-37, FR-014-AC-38, FR-014-AC-39, FR-014-AC-40, FR-014-AC-41, FR-014-AC-42 | 🚧 Planned |
| TC-029 | Verify composite equality oracle generation and three-way agreement | Integration | P0 | FR-018-AC-1, FR-018-AC-2, FR-018-AC-3, FR-018-AC-4, FR-018-AC-5, FR-018-AC-6, FR-018-AC-7, FR-018-AC-8, FR-018-AC-9, FR-018-AC-10, FR-018-AC-11, FR-018-AC-12, FR-018-AC-13, FR-018-AC-14, FR-018-AC-15, FR-018-AC-16, FR-018-AC-17, FR-018-AC-18, FR-018-AC-19, FR-018-AC-20 | 🚧 Planned |
| TC-031 | Verify function-application oracle generation, agreement, and static location tagging | Integration | P0 | FR-021-AC-1, FR-021-AC-2, FR-021-AC-3, FR-021-AC-4, FR-021-AC-5, FR-021-AC-6, FR-021-AC-7, FR-021-AC-8, FR-021-AC-9, FR-021-AC-10, FR-021-AC-11, FR-021-AC-12, FR-021-AC-13, FR-021-AC-14, FR-021-AC-15, FR-021-AC-16, FR-021-AC-17, FR-021-AC-18, FR-021-AC-19, FR-021-AC-20, FR-021-AC-21, FR-021-AC-22, FR-021-AC-23, FR-021-AC-24 | ✅ Covered; FR-021-AC-15, FR-021-AC-18 and FR-021-AC-24 are 🚧 Planned |
