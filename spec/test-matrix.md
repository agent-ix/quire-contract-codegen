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
| FR-007 | FR-007-AC-1 through FR-007-AC-7 | TC-023 | ✅ Covered |
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
| FR-005 | FR-005-AC-5 | TC-002 | 🚧 Planned |
| FR-006 | FR-006-AC-1 | TC-008 | ✅ Covered |
| FR-006 | FR-006-AC-2 | TC-009 | ✅ Covered |
| FR-006 | FR-006-AC-3 | TC-010 | ✅ Covered |
| FR-006 | FR-006-AC-5 | TC-012 | ✅ Covered |
| FR-006 | FR-006-AC-6 | TC-013 | ✅ Covered |
| FR-006 | FR-006-AC-7 | TC-013 | ✅ Covered |
| FR-006 | FR-006-AC-8 through FR-006-AC-11 | TC-032 | ✅ Covered |
| FR-008 | FR-008-AC-1 through FR-008-AC-5, FR-008-CON-2 | TC-017 | ✅ Covered |
| FR-008 | FR-008-CON-1 | Inspection | ✅ Covered |
| FR-009 | FR-009-AC-1 through FR-009-AC-6 | TC-018 | ✅ Covered |
| FR-010 | FR-010-AC-1 through FR-010-AC-5 | TC-019 | ✅ Covered |
| FR-011 | FR-011-AC-1 through FR-011-AC-5 | TC-020 | ✅ Covered |
| FR-012 | FR-012-AC-1 through FR-012-AC-3 | TC-021 | ✅ Covered |
| FR-012 | FR-012-AC-4 | TC-021 | ✅ Covered |
| FR-013 | FR-013-AC-1 through FR-013-AC-4 | TC-022 | ✅ Covered |
| FR-013 | FR-013-AC-5 | Inspection | ✅ Covered |
| FR-014 | FR-014-AC-1 through FR-014-AC-11 | TC-024 | ✅ Covered |
| FR-014 | FR-014-AC-12 | TC-024 | ⚠️ Partially covered; a descriptor naming a different catalogued operation is discharged, but the clause covering a descriptor naming an operation the catalogue has no entry for is a defensive branch no fixture reaches -- the only such state is a same-width IEEE conversion, which the package builder refuses to construct (IR-225) |
| FR-014 | FR-014-AC-13 | TC-024 | ✅ Covered |
| FR-014 | FR-014-AC-14 | TC-024 | ⚠️ Partially covered; the `caller_declared` provenance is asserted for every refused and every never-inspected claim, but no test asserts that the identity such a claim reports is the request item's own descriptor-derived one |
| FR-014 | FR-014-AC-15 | TC-024 | ⚠️ Partially covered; the typed refusal and the withheld generated function are asserted, but no fixture requests a work-exhausted item alongside healthy ones, so the per-item isolation clause is unasserted |
| FR-014 | FR-014-AC-16 | TC-024 | ✅ Covered |
| FR-015 | FR-015-AC-1, FR-015-AC-2 | TC-025 | 🚧 Planned |
| FR-015 | FR-015-AC-3 through FR-015-AC-6 | TC-025 | ✅ Covered |
| FR-015 | FR-015-AC-7 through FR-015-AC-12 | TC-025 | ✅ Covered |
| FR-015 | FR-015-AC-13 | TC-025 | ✅ Covered |
| FR-015 | FR-015-AC-14 | TC-025 | ✅ Covered |
| FR-016 | FR-016-AC-8 | TC-026 | ✅ Covered |
| FR-016 | FR-016-AC-1, FR-016-AC-5 | TC-026 | ⚠️ Partially covered; the witness join (`witness_schema`, `decode_falsification`) decodes a matching transcript and refuses the harness-identity, arity, width, schema, Boolean-byte and comment cases, but reports every refusal as a `KaniOutcome` refusal code rather than FR-016's malformed-witness result, so neither criterion is backed and neither carries a trace tag; binding to the harness pins is unbuilt |
| FR-016 | FR-016-AC-2 through FR-016-AC-4, FR-016-AC-6, FR-016-AC-7 | TC-026 | 🚧 Planned |
| FR-017 | FR-017-AC-2 through FR-017-AC-5, FR-017-AC-8 through FR-017-AC-10, FR-017-CON-2 | TC-027 | ✅ Covered |
| FR-017 | FR-017-AC-1, FR-017-AC-6, FR-017-AC-7, FR-017-CON-1 | TC-027 | 🚧 Planned |
| FR-018 | FR-018-AC-1 through FR-018-AC-3, FR-018-AC-6, FR-018-AC-10 through FR-018-AC-13 | TC-029 | ✅ Covered |
| FR-018 | FR-018-AC-4, FR-018-AC-5, FR-018-AC-7 through FR-018-AC-9 | TC-029 | 🚧 Planned |
| FR-019 | FR-019-AC-1 through FR-019-AC-8, FR-019-AC-10 | TC-030 | ✅ Covered |
| FR-019 | FR-019-AC-9 | Analysis | ✅ Covered |
| FR-021 | FR-021-AC-1 through FR-021-AC-3, FR-021-AC-5 through FR-021-AC-14, FR-021-AC-17 | TC-031 | ✅ Covered |
| FR-021 | FR-021-AC-4 | TC-031 | ⚠️ Partially covered; only `InputRefusal::WrongValueKind` is asserted -- `::DanglingReference` is structurally unreachable for any oracle this generator can produce, since `validate_arguments` checks `WrongValueKind` before it ever walks a value for a dangling reference, and a reference-typed parameter is refused at generation time (AC-10, blocked on qsl#120) |
| FR-021 | FR-021-AC-15 | TC-031 | 🚧 Planned; the `origin` half is implemented and tested, but under this V1's scoped one-node body vocabulary `path` can never be non-empty by construction, so the `path`-non-empty case this AC also describes is not implemented |
| FR-021 | FR-021-AC-16 | TC-031 (Inspection) | ✅ Covered |
| FR-021 | FR-021-AC-18 | TC-031 | 🚧 Planned, pending the `quire-spec-language` re-pin named in FR-021's own Dependencies section |
| FR-022 | FR-022-AC-2 through FR-022-AC-9 | TC-033 | ✅ Covered |
| FR-022 | FR-022-AC-1 | Analysis | 🚧 Planned |

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
`kani::modifies`, so a frame needs a typed IR frame item first. Codegen#110 connected CheckedPackage
V2 claims to this negotiation, so "every V2 scalar obligation is caller-declared" is no longer true:
an IR-confirmed claim over one of the four `quire.op.integer.{add,sub,mul,negate}` identities is no
longer refused on operation identity at all, and reaches a real harness unless a ground independent
of the operation (an i64-unrepresentable endpoint, the source ceiling) displaces it. Every other
confirmed family (every family but `IntegerArithmetic`) is refused as `OperationNotRendered`, this
generator's own unbuilt renderer, not an upstream block; a claim this generator lowered but whose
operation it did not confirm against the node's own catalogued identity, mode or law definition is
refused as `CallerDeclaredOperation` (see `OperationProvenance::CallerDeclared`, the authoritative
enumeration of those cases). Model and graph bounds are blocked on agent-ix/quire-spec-language#120.

AC-1 remains planned for the frame harness; AC-2 remains planned for model-domain bounds on the
model and graph families. The V2 scalar item above is no longer among the reasons either stays
`🚧 Planned`.

FR-017 is the execution and evidence half of codegen#49, separated from FR-015 under codegen#55
because `src/kani_execution.rs` — pin measurement, the pre-run drift refusal, the seven-value
outcome vocabulary and the execution evidence document — had no owning requirement at all. AC-2
through AC-5, AC-8, AC-9 and CON-2 are `✅ Covered` by the default lane: the classification and
pin-comparison unit tests in `src/kani_execution.rs` run on every `cargo test` over the backend's own
recorded output; the absent-launcher refusal, the identity-pin-drift-before-measurement refusal, the
generation/execution boundary (CON-2), and the aggregate-verdict/retained-evidence census (AC-8,
AC-9) are all in `tests/kani_obligations.rs` and run without a Kani installation. AC-1, AC-6, AC-7 and
CON-1 are `🚧 Planned`: the parts of them that require a real installed backend — an installed-backend
pin difference, the full pinned-lane evidence shape, the library-containment refusal after a real
build, and never converting a non-verified outcome into a proof claim — are backed only by the
`#[ignore]`d `make kani` lane, which needs a real pinned installation and is not a `make ci` gate. The
run carries a caller-declared wall-clock budget and reports an elapsed budget as a timed-out
inconclusive result. FR-017-AC-4 and FR-017-AC-5 enumerate the inconclusive reasons they cover by
name; adding timed-out to that enumeration is codegen#55.

FR-018-AC-1 through FR-018-AC-3, FR-018-AC-6 and FR-018-AC-10 through FR-018-AC-13 are `✅ Covered`:
the composite/structural equality slice of codegen#48 that TC-029 backs with a passing test for every
clause those criteria name. Generation lives in `src/composite_equality.rs`; the committed golden
crate under `tests/fixtures/composite_equality/` is the crate TC-029 step 4 compiles and executes,
over all 11 of its oracles — both `EqualityOperator` variants on the record node, and the tuple,
text, enum, option, collection, self-recursive and nested-composite shapes — plus two
`converted`-operand vectors, one of which admits real `Decimal*` conversion charges — in
`tests/composite_equality_agreement.rs`'s `agree3!` macro. Each of these criteria's own
FR-018 mutation was applied, confirmed to turn its test red, and reverted, including AC-2's
conversion-ordering row (swapping which operand's `convert<T>` target the emitted code applies) and
all four of AC-10's: ordering claim-map entries by expression node id alone (which ties two descriptors
on one node and lets a permuted request permute them); the circularity check — blessing a golden whose
emitted operator was corrupted while the native leg reads its descriptor from that same golden; swapping
which operand's runtime value or conversion target is evaluated as left versus right
(`checked.evaluate(left, right, meter)` and `check_equality(op, left_operand, right_operand)`, each
swapped independently); and swapping the emitted `left_source`/`right_source` descriptor itself. The
circularity mutation is the one that tests whether the golden defence holds rather than merely exists:
with the corrupted golden re-blessed, the byte-comparison test passes by construction, but the AC-2
three-way agreement over the record node's `not_equal` oracle — the one the corruption reaches — still
goes red. The two operand-order/descriptor mutations are caught only because `E_CONV_CHARGE`
(FR-018-AC-9) is the corpus's first vector with an asymmetric operand pair — every other vector's left
and right share an identical descriptor, so equality over identical types is symmetric and a swap is
undetectable there; re-blessing under either swap with `E_CONV_CHARGE` in the corpus turns
`tc_029_ac2_a_converted_operand_agrees` and `tc_029_ac9_a_converted_operand_denies_its_own_conversion_charges`
red.

That coverage had a hard prerequisite, now satisfied: agent-ix/quire-contract-codegen#75, merged as
`e74d592`, re-pinned Contract Runtime `a04bd47`→`4e33052` and quire-spec-language
`d9d5273`→`21c507e`. Before it the equality surface FR-018 calls was not visible from this
repository and `quire_spec_language::value` published no `check_equality`, `CheckedEquality` or
`plan_equality`, so the third leg of the agreement had nothing to call. `quire_spec_language::value`
at `21c507e` mirrors the pinned Contract Runtime's equality surface under identical names, with one
signature difference: `InjectedDenial::occurrence` is a plain `u64` there against the runtime's
`NonZeroU64`; the agreement harness's `denials` helper abstracts over it.

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
confirms all three blocker values are distinct, but carries no `relation` or `protocol` node, so two
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
play. The property is verified in agent-ix/quire-contract-runtime#38.

`admits_equality_conversion`'s `converted` operand path is exercised at generation time (AC-3, AC-5)
and, for one `Int`-to-`Integer` vector, inside the three-way execution agreement (AC-2). The pinned
Contract Runtime's `admits_equality_conversion` (`src/exact/equality.rs`) has eight arms: identity
(`source == target`), `Int -> {Integer, Int, Rational, Decimal}`, `Rational -> Rational`,
`Rational -> {Integer, Int, Decimal}`, `Decimal -> Rational`, `Decimal -> Decimal`,
`Decimal -> {Integer, Int}`, and `Quantity -> Quantity`. Since codegen#83,
`tc_029_ac2_every_remaining_admits_equality_conversion_row_agrees` runs one `agree3!` vector per
remaining arm: `Rational -> Rational`, the `Rational -> {Integer, Int, Decimal}` arm exercised only
against `Integer`, `Decimal -> Rational`, `Decimal -> Decimal`, and the `Decimal -> {Integer, Int}`
arm exercised only against `Integer`. The identity arm and `Quantity -> Quantity` remain unexercised
by any test in this corpus; quantity operands are refused as `Unsupported { node_tag: "quantity" }`
before `check_equality` runs (FR-018-AC-5, above), so `Quantity -> Quantity` cannot be reached at all
here. The corpus now has one `converted` vector per exercised arm, not per row of every
target-type-pair the table names, and not the single `Int`-to-`Integer` vector it had before.

FR-018 does not claim the rest of codegen#48. Function application has no runtime surface to call
(agent-ix/quire-contract-runtime#34), the model graph awaits agent-ix/quire-spec-language#120, and
temporal and protocol await agent-ix/quire-spec-language#121; FR-018 refuses all three with distinct
typed blockers rather than specifying around them, and FR-020 remains unwritten.

`interface-001` declares FR-018's `generate_composite_equality_oracles` now that the code exists.
TC-028 reads every public function from the crate's own source, including root `pub fn`s and
everything reachable through a `pub mod`, and asserts it equals the declared operations in both
directions, so a public function without an entry, or an entry without one, fails.

FR-019-AC-1 through FR-019-AC-8 and FR-019-AC-10 are `✅ Covered`: TC-030 walks every row of
FR-290's ordered rules against the settlement point, and each assertion names the disposition and the
typed cause the row requires rather than only that a refusal occurred.

AC-5 is the seam itself, and it is backed by a scan of the source rather than by a behavioural test
because the property is about where code lives: no `Disposition` is constructed outside a
`negotiate_*` function in any Rust source this repository builds. The scan parses each file with
`syn` and inspects expressions only. A line-based scan was written first and measured wrong in both
directions under review — it did not recognise `pub(crate) fn`, so an injected settlement outside
every arm passed green, and it read a rustdoc link naming a variant as code, failing and blaming the
function above the comment. Three injections are now measured: a `pub(crate)` function and a
`Self::Supported` constructor added to `impl Disposition` each turn it red, and the rustdoc link
leaves it green. A paired test asserts the scan reads the three settlement functions by name and at
least twelve construction sites, so a refactor that moves settlement out of the scan's reach fails
even while the gate above stays green.

AC-6 owns the record an observation produces, not the measurement of the tool. Resolving a launcher
through `CARGO_HOME` and `PATH` and parsing what it prints is FR-017's, and TC-027 covers it. An
earlier version of this test wrote a shell script and executed it; it measured nothing the assertions
could catch, because `record_tool_probe` compares two strings, and it added a scratch-directory race
that failed once inside a full suite and never in isolation.

FR-019-AC-9 is `✅ Covered` by analysis, not by a test. The dispatch is an exhaustive `match` over
`BackendKind` with no catch-all, so a variant added without an arm is a compile error; the evidence
is the compiler, and a test asserting a compile failure would need a `trybuild` lane this repository
does not have.

`BackendKind` has one variant today. That is the measured state of the registry and not a
placeholder: Kani is the one backend descriptor registered under this contract. The ambiguous-backend
row is therefore reachable only through a candidate set the registry supplies, which is why TC-030
exercises it through `candidates` rather than by registering a second arm.

FR-022 (Linear IR-293) is the generation arm of the seam FR-019 settles, and it takes the routed
backend and kind as given. AC-2 through AC-9 are backed by TC-033's tests. AC-4's "converts to a kind other than the routed one" branch needs a second `BackendKind` variant
to be reachable, since with one variant the only disagreement is a backend with no kind
(`converted: None`); TC-033 exercises only that `converted: None` case, so the other branch is untested until a second kind exists.
AC-1 stays `🚧 Planned`
because no test backs it: its evidence is the compiler's exhaustiveness check over
`BackendKind` and the one field per kind in `GenerationContexts`, not a test, the same ground as FR-019-AC-9.

Two of codegen#86's eight asks are not in FR-019, and are deferred rather than dropped: the S6 enum
matches over Contract IR's decoded tag and form enums wait on agent-ix/quire-contract-ir#141, and the
`string-edge` scan waits on agent-ix/quire-spec-language#214, which ADR-012 §14.1 makes the owner of
both the `#[string_edge]` attribute and the `xtask string-edge` scan this repository is to run. No
requirement here claims either one.

## Interface Requirement Coverage

| Interface | Acceptance Criteria | Test Cases | Status |
|---|---|---|---|
| interface-001 | interface-001-AC-1 through interface-001-AC-5 | TC-028 | ✅ Covered |

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

The coverage tables above -- Functional, Interface, Non-Functional and
Stakeholder -- are the authority for how much of any criterion they list is
backed. Where a row here reads `✅ Covered` and a criterion in its Traces To
column is marked `⚠️` or `🚧` in the table that owns it, that table governs.

| Test ID | Title | Type | Priority | Traces To | Status |
|---|---|---|---|---|---|
| TC-001 | Reproduce artifacts and attestations | Integration | P0 | FR-001-AC-1, FR-001-AC-3, FR-005-AC-2, NFR-001-AC-1, NFR-002-AC-1, NFR-002-AC-2 | ✅ Covered |
| TC-002 | Compile and publish atomically | Integration | P0 | FR-001-AC-2, FR-001-AC-8, FR-005-AC-1, FR-005-AC-5, NFR-001-AC-2, NFR-001-AC-3 | ✅ Covered |
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
| TC-017 | Verify bound-clause domain derivation and refusal | Integration | P0 | FR-008-AC-1, FR-008-AC-2, FR-008-AC-3, FR-008-AC-4, FR-008-AC-5, FR-008-CON-2 | ✅ Covered |
| TC-018 | Verify constructive satisfying and violating populations | Property | P0 | FR-009-AC-1, FR-009-AC-2, FR-009-AC-3, FR-009-AC-4, FR-009-AC-5, FR-009-AC-6 | ✅ Covered |
| TC-019 | Verify domain and relation boundary censuses | Integration | P0 | FR-010-AC-1, FR-010-AC-2, FR-010-AC-3, FR-010-AC-4, FR-010-AC-5, NFR-004-AC-2 | ✅ Covered |
| TC-020 | Verify numeric conformance campaigns and rate reporting | Integration | P0 | FR-011-AC-1, FR-011-AC-2, FR-011-AC-3, FR-011-AC-4, FR-011-AC-5, NFR-004-AC-1 | ✅ Covered |
| TC-021 | Verify shrinking preserves numeric constraints | Property | P0 | FR-012-AC-1, FR-012-AC-2, FR-012-AC-3, FR-012-AC-4 | ✅ Covered |
| TC-022 | Verify strategy output is consumable without a local wire schema | Integration | P0 | FR-013-AC-1, FR-013-AC-2, FR-013-AC-3, FR-013-AC-4 | ✅ Covered |
| TC-023 | Verify bounded Kani profile corpus parity | Integration | P0 | FR-007-AC-1, FR-007-AC-2, FR-007-AC-3, FR-007-AC-4, FR-007-AC-5, FR-007-AC-6, FR-007-AC-7 | ✅ Covered |
| TC-024 | Verify exact complete-V1 scalar oracle generation and agreement | Integration | P0 | FR-014-AC-1, FR-014-AC-2, FR-014-AC-3, FR-014-AC-4, FR-014-AC-5, FR-014-AC-6, FR-014-AC-7, FR-014-AC-8, FR-014-AC-9, FR-014-AC-10, FR-014-AC-11, FR-014-AC-12, FR-014-AC-13, FR-014-AC-14, FR-014-AC-15, FR-014-AC-16 | ✅ Covered |
| TC-025 | Verify separate bounded Kani obligations | Analysis | P0 | FR-015-AC-1, FR-015-AC-2, FR-015-AC-3, FR-015-AC-4, FR-015-AC-5, FR-015-AC-6, FR-015-AC-7, FR-015-AC-8, FR-015-AC-9, FR-015-AC-10, FR-015-AC-11, FR-015-AC-12, FR-015-AC-14 | 🚧 Planned |
| TC-026 | Verify witness decoding and native replay | Integration | P0 | FR-016-AC-1, FR-016-AC-2, FR-016-AC-3, FR-016-AC-4, FR-016-AC-5, FR-016-AC-6, FR-016-AC-7, FR-016-AC-8 | 🚧 Planned; AC-8 is backed (`src/kani_witness_join.rs unit tests, `tests/it/kani_argument_order.rs`, and the ignored Kani lane `tests/it/kani_witness_join.rs`); AC-1 and AC-5 are planned because every join refusal (identity, arity, width, schema, Boolean-byte, comment) is reported as a `KaniOutcome` refusal code rather than FR-016's malformed-witness replay result; AC-2, AC-3, AC-4, AC-6 and AC-7 are planned |
| TC-027 | Verify pinned Kani obligation execution and its evidence | Analysis | P0 | FR-017-AC-1, FR-017-AC-2, FR-017-AC-3, FR-017-AC-4, FR-017-AC-5, FR-017-AC-6, FR-017-AC-7, FR-017-AC-8, FR-017-AC-9, FR-017-AC-10, FR-017-CON-1, FR-017-CON-2 | 🚧 Planned |
| TC-028 | Verify interface-001's declared API surface and identity envelope match the generator | Integration | P1 | interface-001-AC-1, interface-001-AC-2, interface-001-AC-3, interface-001-AC-4, interface-001-AC-5 | ✅ Covered |
| TC-029 | Verify composite equality oracle generation and three-way agreement | Integration | P0 | FR-018-AC-1, FR-018-AC-2, FR-018-AC-3, FR-018-AC-4, FR-018-AC-5, FR-018-AC-6, FR-018-AC-7, FR-018-AC-8, FR-018-AC-9, FR-018-AC-10, FR-018-AC-11, FR-018-AC-12, FR-018-AC-13 | 🚧 Planned |
| TC-030 | Verify capability settlement at one negotiation point | Integration | P0 | FR-019-AC-1, FR-019-AC-2, FR-019-AC-3, FR-019-AC-4, FR-019-AC-5, FR-019-AC-6, FR-019-AC-7, FR-019-AC-8, FR-019-AC-10 | ✅ Covered |
| TC-033 | Verify routed generation per backend kind without re-negotiation | Integration | P0 | FR-022-AC-2, FR-022-AC-3, FR-022-AC-4, FR-022-AC-5, FR-022-AC-6, FR-022-AC-7, FR-022-AC-8, FR-022-AC-9 | ✅ Covered |

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

FR-006-AC-8 through FR-006-AC-11 are `✅ Covered` by TC-032, backed by the eight tests in
`examples/generation_conformance.rs`'s own `#[cfg(test)]` module, which `[[example]] test = true`
makes `cargo test` build and run and which now carry `/// Trace:` comments like every other row in
this census. Two limits are stated rather than papered over. The census in FR-006-AC-10 is textual,
so an early `return` in `main` is outside it; closing that behaviourally needs a deliberately failing
corpus row, which would be a fault-injection affordance in an evidence producer. And the end-to-end
run in FR-006-AC-11 asserts exit 0 against a passing corpus, which a corpus that produced nothing
would also yield — the ten-row floor on the emitted JSONL under TC-009 is what refuses that, not
this row.

`cargo test --locked --no-fail-fast` and `cargo test --example generation_conformance` both reach
these eight. A stock fail-fast `cargo test` does not, because it aborts at `shared_assurance`
(agent-ix/quire-contract-codegen#121) or `kani_execution`
(agent-ix/quire-contract-codegen#128) first, both of which precede the example. That is pre-existing
drift in those two binaries rather than a gap in this row, and `make test`, `make msrv` and `make ci`
observe these tests once it clears.
