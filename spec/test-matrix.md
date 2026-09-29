---
id: TM-001
title: "Contract codegen v0.1 test matrix"
type: TestMatrix
---

# Contract codegen v0.1 test matrix

## Functional Requirement Coverage

| Functional Req | Acceptance Criteria | Test Cases | Status |
|---|---|---|---|
| FR-001 | FR-001-AC-1 | TC-001 | ⛔ Retired; carried by FR-014-AC-4 |
| FR-001 | FR-001-AC-2, FR-001-AC-8 | TC-002 | ⛔ Retired; carried by FR-014-AC-35 and FR-014-AC-37 |
| FR-001 | FR-001-AC-3, FR-001-AC-7 | TC-001 | ⛔ Retired; carried by FR-014-AC-5 and FR-014-AC-38 |
| FR-001 | FR-001-AC-4 | TC-003 | ⛔ Retired; carried by FR-014-AC-1 |
| FR-001 | FR-001-AC-5 | TC-001, TC-006 | ⛔ Retired; carried by FR-014-AC-36 |
| FR-001 | FR-001-AC-6 | TC-001, TC-002 | ⛔ Retired with the V1 model; not carried |
| FR-002 | FR-002-AC-1 through FR-002-AC-6 | TC-004 | 🚧 Planned |
| FR-007 | FR-007-AC-1, FR-007-AC-3 | TC-023 | ⛔ Retired; carried by FR-015-AC-23, FR-028-AC-2 and FR-028-AC-3 |
| FR-007 | FR-007-AC-2, FR-007-AC-7 | TC-023 | ⛔ Retired; carried by FR-015-AC-10, FR-015-AC-22 and FR-015-AC-25 |
| FR-007 | FR-007-AC-4 | TC-023 | ⛔ Retired; carried by FR-024-AC-5 |
| FR-007 | FR-007-AC-5 | TC-023 | ⛔ Retired; not carried as a criterion, the dependency direction is AD-001's IR to CG seam |
| FR-007 | FR-007-AC-6 | TC-023 | ⛔ Retired; not carried |
| FR-003 | FR-003-AC-1 | TC-005, TC-014 | ⛔ Retired; carried by FR-015-AC-25 |
| FR-003 | FR-003-AC-2 | TC-014 | ⛔ Retired; carried by FR-015-AC-20 |
| FR-003 | FR-003-AC-7 | TC-014 | ⛔ Retired; carried by FR-015-AC-24 |
| FR-003 | FR-003-AC-3 | TC-003, TC-014 | ⛔ Retired; carried by FR-015-AC-3, FR-015-AC-21 and FR-015-AC-23 |
| FR-003 | FR-003-AC-4 | TC-005, TC-014 | ⛔ Retired; carried by FR-015-AC-2 and FR-015-AC-9 |
| FR-003 | FR-003-AC-5 | TC-014 | ⛔ Retired; carried by FR-015-AC-11 |
| FR-003 | FR-003-AC-6 | TC-014 | ⛔ Retired; carried by FR-015-AC-19 and FR-025-AC-8 |
| FR-003 | FR-003-AC-8 | TC-014 | ⛔ Retired; carried by FR-025-AC-3 and FR-015-AC-10 |
| FR-004 | FR-004-AC-1 through FR-004-AC-3, FR-004-AC-5 through FR-004-AC-8 | TC-006 | 🚧 Planned |
| FR-004 | FR-004-AC-4, FR-004-AC-9 | Inspection | ⛔ Retired; not carried |
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
| FR-014 | FR-014-AC-1 through FR-014-AC-3, FR-014-AC-5 through FR-014-AC-11 | TC-024 | ✅ Covered |
| FR-014 | FR-014-AC-4 | TC-024 | ⚠️ Partially covered; byte identity across repeated runs and request permutations and the claim-map order are asserted by regeneration; the criterion's remaining clause has no test |
| FR-014 | FR-014-AC-12 | TC-024 | ⚠️ Partially covered; a descriptor naming a different catalogued operation is discharged, but the clause covering a descriptor naming an operation the catalogue has no entry for is a defensive branch no fixture reaches -- the only such state is a same-width IEEE conversion, which the package builder refuses to construct (IR-225) |
| FR-014 | FR-014-AC-13 | TC-024 | ✅ Covered |
| FR-014 | FR-014-AC-14 | TC-024 | ⚠️ Partially covered; the `caller_declared` provenance is asserted for every refused and every never-inspected claim, but no test asserts that the identity such a claim reports is the request item's own descriptor-derived one |
| FR-014 | FR-014-AC-15 | TC-024 | ⚠️ Partially covered; the typed refusal and the withheld generated function are asserted, but no fixture requests a work-exhausted item alongside healthy ones, so the per-item isolation clause is unasserted |
| FR-014 | FR-014-AC-16 | TC-024 | ✅ Covered |
| FR-014 | FR-014-AC-17 | TC-024 | ✅ Covered |
| FR-014 | FR-014-AC-18 | TC-024 | 🚧 Planned; the derivation test's refused-identity case is `integer.eq`, which FR-014-AC-35 makes derivable |
| FR-014 | FR-014-AC-19 | TC-024 | ✅ Covered |
| FR-014 | FR-014-AC-20 through FR-014-AC-25 | TC-024 | ✅ Covered |
| FR-014 | FR-014-AC-26 through FR-014-AC-34 | TC-024 | ✅ Covered |
| FR-014 | FR-014-AC-35 through FR-014-AC-38 | TC-024 | 🚧 Planned; carried from FR-001, and Boolean and integer `eq`/`ne` nodes are refused as `OperationNotDerivable` at this revision |
| FR-015 | FR-015-AC-1, FR-015-AC-2 | TC-025 | 🚧 Planned |
| FR-015 | FR-015-AC-3 through FR-015-AC-6 | TC-025 | ✅ Covered |
| FR-015 | FR-015-AC-7 through FR-015-AC-12 | TC-025 | ✅ Covered |
| FR-015 | FR-015-AC-13 | TC-025 | ✅ Covered |
| FR-015 | FR-015-AC-14 | TC-025 | ✅ Covered |
| FR-015 | FR-015-AC-15 | TC-033 | 🚧 Planned; the test's underivable claim is `integer.eq`, which FR-014-AC-35 makes derivable |
| FR-015 | FR-015-AC-16 through FR-015-AC-18 | TC-033 | ✅ Covered |
| FR-015 | FR-015-AC-19 through FR-015-AC-25 | TC-025 | 🚧 Planned; carried from FR-003 and FR-007 |
| FR-016 | FR-016-AC-8 through FR-016-AC-11 | TC-026 | ✅ Covered |
| FR-016 | FR-016-AC-1, FR-016-AC-5 | TC-026 | ⚠️ Partially covered; the witness join (`witness_schema`, `decode_falsification`) decodes a matching transcript and refuses the harness-identity, arity, width, schema, Boolean-byte and comment cases, but reports every refusal as a `KaniOutcome` refusal code rather than FR-016's malformed-witness result, so neither criterion is backed and neither carries a trace tag |
| FR-016 | FR-016-AC-2 through FR-016-AC-4, FR-016-AC-6, FR-016-AC-7, FR-016-AC-12, FR-016-AC-13 | TC-026 | 🚧 Planned |
| FR-017 | FR-017-AC-2, FR-017-AC-4, FR-017-AC-5, FR-017-CON-2 | TC-027 | ✅ Covered |
| FR-017 | FR-017-AC-10 | TC-027 | ⚠️ Partially covered; each recorded Kani capture parses to its expected transcript and classifies to its expected outcome; the criterion's clause about other files under `src/` has no test |
| FR-017 | FR-017-AC-6, FR-017-AC-7, FR-017-AC-11, FR-017-CON-1 | TC-027 | 🚧 Planned |
| FR-018 | FR-018-AC-1 through FR-018-AC-3, FR-018-AC-6, FR-018-AC-11 through FR-018-AC-14 | TC-029 | ✅ Covered |
| FR-018 | FR-018-AC-10 | TC-029 | ⚠️ Partially covered; byte identity across repeated runs and request permutations, the descriptor-key order, and the generated crate compiled and executed at test time under AC-2 are asserted; the criterion's remaining clause has no test |
| FR-018 | FR-018-AC-4, FR-018-AC-5, FR-018-AC-7 through FR-018-AC-9 | TC-029 | 🚧 Planned |
| FR-019 | FR-019-AC-1 through FR-019-AC-4, FR-019-AC-7, FR-019-AC-8, FR-019-AC-10 | TC-030 | ✅ Covered |
| FR-019 | FR-019-AC-5 | TC-030 | 🚧 Planned; no behavioural test backs it |
| FR-019 | FR-019-AC-9 | Analysis | ✅ Covered |
| FR-021 | FR-021-AC-1 through FR-021-AC-3, FR-021-AC-5 through FR-021-AC-14, FR-021-AC-17 | TC-031 | ✅ Covered |
| FR-021 | FR-021-AC-4 | TC-031 | ⚠️ Partially covered; only `InputRefusal::WrongValueKind` is asserted -- `::DanglingReference` is structurally unreachable for any oracle this generator can produce, since `validate_arguments` checks `WrongValueKind` before it ever walks a value for a dangling reference, and a reference-typed parameter is refused at generation time (AC-10, blocked on qsl#120) |
| FR-021 | FR-021-AC-15 | TC-031 | 🚧 Planned; the `origin` half is implemented and tested, but under this V1's scoped one-node body vocabulary `path` can never be non-empty by construction, so the `path`-non-empty case this AC also describes is not implemented |
| FR-021 | FR-021-AC-16 | TC-031 (Inspection) | ✅ Covered |
| FR-021 | FR-021-AC-18 | TC-031 | 🚧 Planned; the `quire-spec-language` authority leg is written and unasserted |
| FR-022 | FR-022-AC-2 through FR-022-AC-5, FR-022-AC-7 through FR-022-AC-15 | TC-033 | ✅ Covered |
| FR-022 | FR-022-AC-6 | TC-033 | ⚠️ Partially covered; the out-of-range unwind and unparsable subject path refusals are asserted; the criterion's first example has no test |
| FR-022 | FR-022-AC-1 | Analysis | 🚧 Planned |
| FR-024 | FR-024-AC-1 through FR-024-AC-10 | TC-035 | 🚧 Planned |
| FR-025 | FR-025-AC-1 | TC-036 | 🚧 Planned; emission order is asserted only for the retired V1 `BoundClause` harness kinds, and the ascending order and the scalar-claim harness are unasserted |
| FR-025 | FR-025-AC-2 through FR-025-AC-8 | TC-036 | 🚧 Planned |
| FR-026 | FR-026-AC-1 through FR-026-AC-4 | TC-037 | 🚧 Planned |
| FR-028 | FR-028-AC-1 through FR-028-AC-9 | TC-039 | 🚧 Planned |
| FR-029 | FR-029-AC-1 through FR-029-AC-7 | TC-040 | 🚧 Planned |

The current TestMatrix structure and coverage selector both consume the shared `Status` column. The
former `Coverage Status` conflict was tracked in upstream spec-artifacts-process #77; this repository
retains no local checker or copied traceability implementation.

FR-001, FR-003 and FR-007 are retired (ADR-001). Each retired row is `⛔` and names the criterion
that carries it, and each carried criterion is `🚧 Planned` until code and a test back it. The tests
tagged to a retired criterion stay until the code retires its V1 path. The numeric-strategy FR-008
through FR-013 slice is `✅ Covered` after its ticket-scoped current-head reviews, against its current
text, which still states the V1 `BoundPackage` input that ADR-001 retires (see ADR-001's
Consequences). FR-002, FR-004, FR-005, the remaining NFR rows, and StR rows stay `🚧 Planned` until
their complete ticket scopes are implemented and reviewed. FR-004 has no complete implementation or
suite.

FR-008 through FR-013 and NFR-004 are covered by TC-017 through TC-022 after the bounded-integer
oracle grammar landed in PR #29. The evidence includes exhaustive small-domain population and
shrink walks, exact boundary censuses, all supported clause-kind/population campaigns at 256 and
10,000 cases with zero global rejects, generated-consumer compilation, identity mutation, closing
Rust review SR-016, and gap analysis SR-017.

`tests/it/kani_obligations.rs` backs FR-015-AC-3 through AC-6: unbounded, non-finite and
upstream-blocked items, unsatisfiable IR bounds and every `caller_declared` V2 scalar operation are
refused with a typed reason and no harness. AC-1 and AC-2 are `🚧 Planned`: the separate
precondition, postcondition and invariant harnesses with IR bounds in their identity are built
today only over the retired V1 `BoundClause` arm, which `make kani` verifies and falsifies.

AC-7 through AC-12 are added by codegen#56, which found the decisions that make an FR-015 harness
sound stated nowhere: the non-vacuity covers that are the only thing separating a proof from a
vacuous run, the sibling-precondition assumptions that make a postcondition harness prove a weaker
claim than the clause states, the hard-coded solver, the byte-identity of regeneration, the
IR-domain assumption on every symbolic argument, and the request-level ceilings. All six are backed
by the named tests in `tests/kani_obligations.rs` that already exercised them without a criterion to
bind to. Frame obligations are accounted `unsupported` until QSpec decides how a frame node lowers
into a Kani form (ADR-004), and FR-014 refuses V2 `state` nodes as `NoFiniteEncoding`. Codegen#110 connected CheckedPackage
V2 claims to this negotiation, so "every V2 scalar obligation is caller-declared" is no longer true:
an IR-confirmed claim over one of the four `quire.op.integer.{add,sub,mul,negate}` identities is no
longer refused on operation identity at all, and reaches a real harness unless a ground independent
of the operation (an i64-unrepresentable endpoint, the source ceiling) displaces it. Every other
confirmed family (every family but `IntegerArithmetic`) is refused as `OperationNotRendered`, this
generator's own unbuilt renderer, not an upstream block; a claim this generator lowered but whose
operation it did not confirm against the node's own catalogued identity, mode or law definition is
refused as `CallerDeclaredOperation` (see `OperationProvenance::CallerDeclared`, the authoritative
enumeration of those cases). Model and graph bounds are blocked on agent-ix/quire-spec-language#120.

FR-015-AC-19 through AC-25 carry FR-003's and FR-007's behaviours (ADR-001) and are `🚧 Planned`.

FR-017 is the execution and evidence half of codegen#49, separated from FR-015 under codegen#55
because `src/kani_execution.rs` — the launch, the outcome vocabulary and the execution evidence —
had no owning requirement. The outcome classification tests in `src/kani_execution.rs` run on every
`cargo test` over recorded backend output; the absent-launcher refusal and the generation/execution
boundary (CON-2) are in `tests/it/kani_obligations.rs` and run without a Kani installation. The parts that need a real
installed backend — the full evidence shape, the library-containment refusal after a real build,
and never converting a non-verified outcome into a proof claim — are backed only by the
`#[ignore]`d `make kani` lane, which needs a Kani installation and is not a `make ci` gate. The
run carries a caller-declared wall-clock budget and reports an elapsed budget as a timed-out
inconclusive result. The timed-out and memory-exhausted reasons, held to ceilings the harness
identity records, are FR-028's criteria and are `🚧 Planned`.

FR-018-AC-1 through FR-018-AC-3, FR-018-AC-6 and FR-018-AC-11 through FR-018-AC-14 are `✅ Covered`,
and FR-018-AC-10 is covered for the clauses its row names:
the composite/structural equality slice of codegen#48 that TC-029 backs with a passing test for every
clause those criteria name. Generation lives in `src/composite_equality.rs`; TC-029 step 4 generates
the corpus crate at test time and compiles and executes it, over all 11 of its oracles — both
`EqualityOperator` variants on the record node, and the tuple, text, enum, option, collection,
self-recursive and nested-composite shapes — plus two `converted`-operand vectors, one of which
admits real `Decimal*` conversion charges — in `tests/composite_equality_support/agreement_cases.rs`'s
`agree2!` macro, run by `tests/it/composite_equality_agreement.rs`. Each of these criteria's own
FR-018 mutation was applied, confirmed to turn its test red, and reverted, including AC-2's
conversion-ordering row (swapping which operand's `convert<T>` target the emitted code applies) and
all four of AC-10's: ordering claim-map entries by expression node id alone (which ties two descriptors
on one node and lets a permuted request permute them); corrupting the emitted operator of the record
node's `not_equal` oracle, which the AC-2 agreement over that oracle turns red; swapping
which operand's runtime value or conversion target is evaluated as left versus right
(`checked.evaluate(left, right, meter)` and `check_equality(op, left_operand, right_operand)`, each
swapped independently); and swapping the emitted `left_source`/`right_source` descriptor itself. The
two operand-order/descriptor mutations are caught only because `E_CONV_CHARGE`
(FR-018-AC-9) is the corpus's first vector with an asymmetric operand pair — every other vector's left
and right share an identical descriptor, so equality over identical types is symmetric and a swap is
undetectable there; under either swap, with `E_CONV_CHARGE` in the corpus,
`tc_029_ac2_a_converted_operand_agrees` and `tc_029_ac9_a_converted_operand_denies_its_own_conversion_charges`
go red.

`quire_spec_language::value` mirrors Contract Runtime's equality surface under identical names, with
one signature difference: `InjectedDenial::occurrence` is a plain `u64` there against the runtime's
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
and, for one `Int`-to-`Integer` vector, inside the three-way execution agreement (AC-2). Contract
Runtime's `admits_equality_conversion` (`src/exact/equality.rs`) has eight arms: identity
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

FR-018's entry point refuses function application, the model graph
(agent-ix/quire-spec-language#120) and temporal and protocol nodes
(agent-ix/quire-spec-language#121) with distinct typed blockers. FR-021 generates function
application through its own entry point, and FR-020 remains unwritten.

`interface-001` declares FR-018's `generate_composite_equality_oracles`.

FR-019-AC-1 through FR-019-AC-4, FR-019-AC-7, FR-019-AC-8 and FR-019-AC-10 are `✅ Covered`: TC-030 walks every row of
FR-290's ordered rules against the settlement point, and each assertion names the disposition and the
typed cause the row requires rather than only that a refusal occurred.

FR-019-AC-9 is `✅ Covered` by analysis, not by a test. The dispatch is an exhaustive `match` over
`BackendKind` with no catch-all, so a variant added without an arm is a compile error; the evidence
is the compiler, and a test asserting a compile failure would need a `trybuild` lane this repository
does not have.

`BackendKind` has one variant today. That is the measured state of the registry and not a
placeholder: Kani is the one backend descriptor registered under this contract. The ambiguous-backend
row is therefore reachable only through a candidate set the registry supplies, which is why TC-030
exercises it through `candidates` rather than by registering a second arm.

FR-022 (Linear IR-293) is the generation arm of the seam FR-019 settles, and it takes the routed
backend and kind as given. AC-2 through AC-5 and AC-7 through AC-15 are backed by TC-033's tests. AC-4's "converts to a kind other than the routed one" branch needs a second `BackendKind` variant
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
| TC-001 | Verify deterministic derivation | Integration | P0 | FR-001-AC-1, FR-001-AC-3, FR-005-AC-2, NFR-001-AC-1, NFR-002-AC-1, NFR-002-AC-2 | 🚧 Planned; its FR-001 criteria are retired, and its FR-005 and NFR criteria are planned |
| TC-002 | Compile and publish atomically | Integration | P0 | FR-001-AC-2, FR-001-AC-8, FR-005-AC-1, FR-005-AC-5, NFR-001-AC-2, NFR-001-AC-3 | 🚧 Planned; its FR-001 criteria are retired, and its FR-005 and NFR criteria are planned |
| TC-003 | Reject unsupported inputs explicitly | Integration | P0 | FR-001-AC-4, FR-003-AC-3, NFR-002-AC-3 | 🚧 Planned; its FR-001 and FR-003 criteria are retired, and NFR-002-AC-3 is planned |
| TC-004 | Preserve shaped proptest strategies | Property | P0 | FR-002-AC-1, FR-002-AC-2, FR-002-AC-3, FR-002-AC-4, FR-002-AC-5, FR-002-AC-6 | 🚧 Planned |
| TC-005 | Enforce Kani proof dependencies | Analysis | P0 | FR-003-AC-1 | ⛔ Retired with FR-003 |
| TC-006 | Distinguish vacuity and unexecuted flow | Integration | P0 | FR-004-AC-1, FR-004-AC-2, FR-004-AC-3, FR-004-AC-5, FR-004-AC-6 | 🚧 Planned |
| TC-007 | Verify cross-backend semantic parity | Integration | P0 | FR-005-AC-4 | 🚧 Planned |
| TC-014 | Verify bounded numeric and state Kani contracts | Analysis | P0 | FR-003-AC-2, FR-003-AC-3, FR-003-AC-4, FR-003-AC-5, FR-003-AC-6, FR-003-AC-7, FR-003-AC-8 | ⛔ Retired with FR-003 |
| TC-017 | Verify bound-clause domain derivation and refusal | Integration | P0 | FR-008-AC-1, FR-008-AC-2, FR-008-AC-3, FR-008-AC-4, FR-008-AC-5, FR-008-CON-2 | ✅ Covered |
| TC-018 | Verify constructive satisfying and violating populations | Property | P0 | FR-009-AC-1, FR-009-AC-2, FR-009-AC-3, FR-009-AC-4, FR-009-AC-5, FR-009-AC-6 | ✅ Covered |
| TC-019 | Verify domain and relation boundary censuses | Integration | P0 | FR-010-AC-1, FR-010-AC-2, FR-010-AC-3, FR-010-AC-4, FR-010-AC-5, NFR-004-AC-2 | ✅ Covered |
| TC-020 | Verify numeric conformance campaigns and rate reporting | Integration | P0 | FR-011-AC-1, FR-011-AC-2, FR-011-AC-3, FR-011-AC-4, FR-011-AC-5, NFR-004-AC-1 | ✅ Covered |
| TC-021 | Verify shrinking preserves numeric constraints | Property | P0 | FR-012-AC-1, FR-012-AC-2, FR-012-AC-3, FR-012-AC-4 | ✅ Covered |
| TC-022 | Verify strategy output is consumable without a local wire schema | Integration | P0 | FR-013-AC-1, FR-013-AC-2, FR-013-AC-4 | ✅ Covered |
| TC-023 | Verify bounded Kani profile corpus parity | Integration | P0 | FR-007-AC-1, FR-007-AC-2, FR-007-AC-3, FR-007-AC-4, FR-007-AC-5, FR-007-AC-6, FR-007-AC-7 | ⛔ Retired with FR-007 |
| TC-024 | Verify exact complete-V1 scalar oracle generation and agreement | Integration | P0 | FR-014-AC-1, FR-014-AC-2, FR-014-AC-3, FR-014-AC-4, FR-014-AC-5, FR-014-AC-6, FR-014-AC-7, FR-014-AC-8, FR-014-AC-9, FR-014-AC-10, FR-014-AC-11, FR-014-AC-12, FR-014-AC-13, FR-014-AC-14, FR-014-AC-15, FR-014-AC-16, FR-014-AC-17, FR-014-AC-18, FR-014-AC-19, FR-014-AC-20, FR-014-AC-21, FR-014-AC-22, FR-014-AC-23, FR-014-AC-24, FR-014-AC-25, FR-014-AC-26, FR-014-AC-27, FR-014-AC-28, FR-014-AC-29, FR-014-AC-30, FR-014-AC-31, FR-014-AC-32, FR-014-AC-33, FR-014-AC-34, FR-014-AC-35, FR-014-AC-36, FR-014-AC-37, FR-014-AC-38 | 🚧 Planned |
| TC-025 | Verify separate bounded Kani obligations | Analysis | P0 | FR-015-AC-1, FR-015-AC-2, FR-015-AC-3, FR-015-AC-4, FR-015-AC-5, FR-015-AC-6, FR-015-AC-7, FR-015-AC-8, FR-015-AC-9, FR-015-AC-10, FR-015-AC-11, FR-015-AC-12, FR-015-AC-14, FR-015-AC-19, FR-015-AC-20, FR-015-AC-21, FR-015-AC-22, FR-015-AC-23, FR-015-AC-24, FR-015-AC-25 | 🚧 Planned |
| TC-026 | Verify witness decoding and native replay | Integration | P0 | FR-016-AC-1, FR-016-AC-2, FR-016-AC-3, FR-016-AC-4, FR-016-AC-5, FR-016-AC-6, FR-016-AC-7, FR-016-AC-8, FR-016-AC-9, FR-016-AC-10, FR-016-AC-11, FR-016-AC-12, FR-016-AC-13 | 🚧 Planned |
| TC-027 | Verify Kani obligation execution and its evidence | Analysis | P0 | FR-017-AC-2, FR-017-AC-4, FR-017-AC-5, FR-017-AC-6, FR-017-AC-7, FR-017-AC-10, FR-017-AC-11, FR-017-CON-1, FR-017-CON-2 | 🚧 Planned |
| TC-029 | Verify composite equality oracle generation and three-way agreement | Integration | P0 | FR-018-AC-1, FR-018-AC-2, FR-018-AC-3, FR-018-AC-4, FR-018-AC-5, FR-018-AC-6, FR-018-AC-7, FR-018-AC-8, FR-018-AC-9, FR-018-AC-10, FR-018-AC-11, FR-018-AC-12, FR-018-AC-13, FR-018-AC-14 | 🚧 Planned |
| TC-030 | Verify capability settlement at one negotiation point | Integration | P0 | FR-019-AC-1, FR-019-AC-2, FR-019-AC-3, FR-019-AC-4, FR-019-AC-5, FR-019-AC-7, FR-019-AC-8, FR-019-AC-10 | ✅ Covered |
| TC-031 | Verify function-application oracle generation, agreement, and static location tagging | Integration | P0 | FR-021-AC-1, FR-021-AC-2, FR-021-AC-3, FR-021-AC-4, FR-021-AC-5, FR-021-AC-6, FR-021-AC-7, FR-021-AC-8, FR-021-AC-9, FR-021-AC-10, FR-021-AC-11, FR-021-AC-12, FR-021-AC-13, FR-021-AC-14, FR-021-AC-15, FR-021-AC-16, FR-021-AC-17, FR-021-AC-18 | ✅ Covered |
| TC-033 | Verify routed generation per backend kind without re-negotiation | Integration | P0 | FR-022-AC-2, FR-022-AC-3, FR-022-AC-4, FR-022-AC-5, FR-022-AC-6, FR-022-AC-7, FR-022-AC-8, FR-022-AC-9, FR-022-AC-10, FR-022-AC-11, FR-022-AC-12, FR-022-AC-13, FR-022-AC-14, FR-022-AC-15, FR-015-AC-15, FR-015-AC-16, FR-015-AC-17, FR-015-AC-18 | ✅ Covered |
| TC-035 | Verify counterexample submission in QSL's counterexample envelope | Integration | P0 | FR-024-AC-1, FR-024-AC-2, FR-024-AC-3, FR-024-AC-4, FR-024-AC-5, FR-024-AC-6, FR-024-AC-7, FR-024-AC-8, FR-024-AC-9, FR-024-AC-10 | 🚧 Planned |
| TC-036 | Verify the generated harness subject ABI | Integration | P0 | FR-025-AC-1, FR-025-AC-2, FR-025-AC-3, FR-025-AC-4, FR-025-AC-5, FR-025-AC-6, FR-025-AC-7, FR-025-AC-8 | 🚧 Planned |
| TC-037 | Verify the backend adapter trait and its closed-enum dispatch | Integration | P0 | FR-026-AC-1, FR-026-AC-2, FR-026-AC-3, FR-026-AC-4 | 🚧 Planned |
| TC-039 | Verify bounded proof ceilings, their inconclusive reasons and the proof subject | Integration | P0 | FR-028-AC-1, FR-028-AC-2, FR-028-AC-3, FR-028-AC-4, FR-028-AC-5, FR-028-AC-6, FR-028-AC-7, FR-028-AC-8, FR-028-AC-9 | 🚧 Planned |
| TC-040 | Verify the total map from a Kani run outcome to QSL's terminal value | Integration | P0 | FR-029-AC-1, FR-029-AC-2, FR-029-AC-3, FR-029-AC-4, FR-029-AC-5, FR-029-AC-6, FR-029-AC-7 | 🚧 Planned |

TC-005, TC-014 and TC-023 are retired with FR-003 and FR-007, and TC-025 verifies the criteria
FR-015 carries from them. TC-001 through TC-003 are planned: their FR-001 and FR-003 criteria are
retired, their tests exercise only the V1 path those criteria described, and their FR-005 and NFR
criteria are planned. TC-004, TC-006, and TC-007 remain planned until their complete backend/parity ticket scopes are independently
reviewed.

TC-017 through TC-022 are backed by passing named tests in `tests/bound_strategy_generation.rs`,
`tests/bound_populations.rs`, and `tests/bound_census.rs`. Together they cover admission and ordered
refusals, constructive populations, boundary censuses, runtime accounting and replay, and generated
consumer compilation.

TC-004's generated-crate fixtures include deterministic mixed-campaign counts and distinguish
framework exhaustion with a retained floor result from a completed below-floor campaign. Its row
remains planned pending independent review of the complete issue #3 scope.

## Evidence Locations

Each row is specified in the same-ID document under `spec/test/`. `spec/evidence/suites.md` is the
suite registry: it names the command, tool and evidence kind for each suite. SUITE-008 is the reviewed
local evidence producer for TC-003, TC-005, TC-014, and the FR-003 portion of TC-007; SUITE-010 is
local pre-review evidence for the publication portion of TC-002. SR-016 and SR-017 record the closing
code and gap reviews for TC-017 through TC-022.
