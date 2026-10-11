---
id: TC-030
title: "Verify capability settlement at one negotiation point"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-019
    type: verifies
  - target: ix://agent-ix/quire-specification/TC-271
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-335
    type: references
---
# TC-030: Verify capability settlement at one negotiation point

## Description

Verify that every requested capability claim is settled by exactly one
`negotiate_*` arm over a closed backend kind, that each FR-290 rule settles its
own row, and that only an item settled `supported` routes a backend.

## Test Procedure

Settle envelopes whose items exercise each rule in FR-290's stated order: an
absent kind; an unknown kind; an absent extent classification; an unknown
backend; a candidate absent from the registered descriptors; a candidate not advertising the
item's kind; an empty candidate set; two candidates with no named backend; and
exactly one candidate under each row of the advertised-mode table, including an
unbounded extent against a `bounded`-only advertisement both with and without a
finite bound available.

Settle the two-candidate item again under the reverse registration order and
compare the disposition, the cause and the order of the named candidates.

Use the public negotiation API with typed IR forms and one Kani descriptor.
For every FR-019-AC-25 row, construct requests for each named family and
obligation role: Boolean, integer, rational, decimal, IEEE float, text, enum,
quantity, record/tuple equality, function application, precondition,
postcondition and invariant clauses, plus frame obligations represented by
typed V2 `StateFrame` items with `contract` or `frame` roles; also cover state
before/after, bounded collections and bounded graphs. Include the exact
operation identities, type/domain bounds and typed claim references required by
FR-015; do not replace them with display strings. Exercise every supported scalar operation in its family and the
boundary between a valid finite domain and a missing or non-representable
bound. For composite equality, vary record and tuple shape, optional members,
and recursive or over-budget shapes. For functions, exercise supported and
refused result outcomes. For V1 `BoundClause` items, cover its
precondition, postcondition and invariant roles, dependency roles and
Boolean/integer expression forms. V1
`ClauseKind` has no `Frame` variant; `Assertion`, `Case` and `Information`
clauses are not V1 obligation items. Cover frame obligations through typed V2
`StateFrame` items. For state and frame items, cover before and after reads,
granted and forbidden fields, and each typed role. For collections and graphs,
cover bounded and unbounded roots and every admitted finite shape.

Run every typed form selected by an Implemented subpath against a bounded Kani
candidate advertising the required mode and verify `supported`. For each
Planned subpath below, construct the exact tag/form/operation/role selector in
its row, confirm it does not match an Implemented subpath, and use a valid
bounded request. Verify AC-33: settlement is `unsupported`, carries only that
row's exact typed source refusal, and emits no artifact. Never accept
`supported` for a Planned subpath. Against a bounded-only candidate, run an
implemented unbounded form with and without a finite bound and verify
`requires-bound` or the typed unbounded refusal as AC-31 specifies. Run typed
forms outside the complete
V1/V2 set and malformed in-set forms as separate controls; the former settles
`unsupported`, warned, with `unsupported_projection`/`unsupported-requested-capability`
naming its typed form before generation, while the latter keeps its FR-015
validation cause. Verify no refused form emits an artifact or invokes a
generator.

Exercise the V2 clause path separately: use `state`/`state_clause` nodes with
`operation.member.clause` equal to `precondition`, `postcondition` or
`invariant`, and bodies and input domains within FR-015-AC-39 through AC-41.
Compare each with a typed role or body outside the supported V2 subset
(including an operator outside FR-015-AC-40 and AC-41, as listed in AC-42).
Verify typed form identity controls settlement and that V2 cases do not stand
in for any V1 family row.

Run distinct public API controls for an empty candidate set, multiple
candidates with no named backend, and a supported form whose sole candidate
cannot satisfy its extent/mode. The empty set retains its missing-capability
cause, the multiple candidates retain `invalid_capability`/`ambiguous-backend`,
and the mode/extent control follows the advertised-mode rules. With one
candidate, make both the typed form and its extent/mode unsupported and verify
the form refusal settles after candidate/manifest validation but before the
mode/extent refusal. Change only display/debug formatting for an unchanged
typed tag/form and verify its disposition, cause and named form identity do
not change.

For a supported unbounded form on bounded-only Kani, set
`finite_bound_available=true` and verify `requires-bound` without a substituted
bound or artifact. Supply a finite bound in a new bounded request and verify
`supported` retains its exact proof-bound value, domain kind and domain
identity. Settlement itself emits no artifact or Kani outcome.

Submit one envelope whose `capability_vocabulary` is absent and one whose value
is another identity, and inspect which kinds were read.

Enumerate the closed backend kind's variants and assert that settlement
dispatches an arm for each.

Ask each settlement that is not `supported`
whether it routes a backend at all, and ask a manifest that repeats one backend
identity the same question.

For planned FR-019-AC-15, inspect the driver projection from each QSL
`Registry::descriptors()` value to CG's descriptor. Build two registries separately, each with
an otherwise equal descriptor, one with `Linked` and one with `Process` origin. Project each,
and confirm identity and advertised pairs
are equal in the CG values while origin alone differs. Verify that a linked
Kani descriptor remains `Linked`, the driver conversion exhaustively matches both QSL origin
variants with no wildcard, no identity or manifest byte test chooses
the origin, and CG has no direct `qsl-route` dependency or new FR-331 origin
wire member. Run this row when IR-633 implements the projection.

For planned FR-019-AC-24, use the QSL FR-335 request for the `+` value-validity
claim in `f using v(s: Set<Int[0,9999]>): Integer pure { size(s) + 1 }`.
Supply the already-classified claim item to CG settlement with: (1) an empty
registry and candidate set; (2) one linked Kani candidate advertising only bounded
`value-validity` with `finite_bound_available=true`; (3) a separate bounded
item requested with `ProofBound::Cardinality{maximum: 8}` against that same
sole candidate; and (4) an added unboundable quantity root with
`finite_bound_available=false` against that bounded-only candidate. Preserve
each item's request index across settlement and routing. Inspect warning
cause, artifact absence and Kani outcome absence for unsupported and
requires-bound items; settlement of the bounded supported item makes no
execution-outcome claim.
QSL TC-845 owns compiling the source into the claim and request fixture;
quire-integration owns a composed source-to-claim, CG and driver execution.

## Expected Results

Each item settles exactly one of `supported`, `requires-bound`, `unsupported`
and `invalid-request`, with the cause FR-290 names for its row and naming the
backend or candidates that row requires.

The unbounded extent against a `bounded`-only advertisement settles
`requires-bound` where a finite bound is available and `unsupported`, warned,
with `unsupported_projection`/`unbounded-extent` where none is; it never
settles `supported`.

The two-candidate item settles `invalid-request` with
`invalid_capability`/`ambiguous-backend`, naming both candidates in candidate
order, identically under both registration orders.

Both malformed-vocabulary envelopes refuse as
`invalid_capability`/`unsupported-version` with no kind read.

Every variant of the closed backend kind has a dispatched arm.

No settlement other than `supported` routes a backend, and a manifest repeating an
identity routes none.

Under FR-019-AC-25 through AC-33, every Implemented typed subpath has a valid
bounded case that settles `supported` under the required advertisement. For
each Planned typed selector, the result is `unsupported` with exactly the
single cause in that selector's status-register row; no family-level result
substitutes for checking the exact selector, and no Planned selector settles
`supported` until its renderer is implemented. In particular, the negated
StateFrame contract selector returns `ConditionNotSupported`, while the
all-fields-granted StateFrame frame selector returns `NothingForbidden`; each
is wrapped in `StateFrameRefused` and neither changes the other role's result.
The typed out-of-set controls refuse before generation. V2 clause forms
defined by FR-015-AC-38 through AC-42 are separately admitted or refused
deterministically. A form refusal names the typed form and precedes extent/mode
settlement after candidate validation. Empty, ambiguous and mode/extent cases
retain their own outcomes. Display/debug formatting does not affect form
settlement. An unbounded `requires-bound` item receives no substituted bound,
while a new bounded request retains the exact caller-supplied proof-bound value
and domain identity and settles `supported`.

Under FR-019-AC-15, the driver preserves the QSL registry origin as a typed
CG value without altering identity or advertised pairs; the CG crate boundary
and FR-331 wire remain as specified.

The planned collection controls settle respectively `unsupported`, warned with
`value-validity` and no artifact or Kani outcome; `requires-bound`, without a
substituted bound, artifact or Kani outcome; `supported` for the independently
indexed bounded request on that same sole candidate, with any later result
joining only the bounded request's index and no Kani execution outcome implied
by settlement; and `unsupported`, warned with
`unsupported_projection`/`unbounded-extent`, without an artifact or Kani
outcome. The unbounded item's disposition is unchanged by the bounded request.
Passing generic mode-table controls alone does not complete this collection case.

## FR-019-AC-25 implementation status

The V1 `BoundClause` path derives the obligation role from `ClauseKind`:
`Precondition`, `Postcondition` and `Invariant` map to obligation roles;
`Assertion`, `Case` and `Information` map to `ClauseKindNotObligation`. V1
`ClauseKind` has no `Frame` variant, so there is no V1 `BoundClause` frame
selector. Frame obligations in this test use typed V2 `StateFrame` items and
their `contract` or `frame` roles. The status register describes typed
subpaths against the complete normative set in FR-019-AC-25. A Planned
subpath is disjoint from every Implemented subpath; its selector and one
current refusal cause are explicit. A subset marked Implemented does not
reduce AC-25's scope.

| Typed form subpath selector | STATUS | Implemented subset or planned owner | Exact current refusal cause for this subpath |
|---|---|---|---|
| V1 `BoundClause`, Boolean clause root, Boolean/bounded-integer dependencies, supported connectives/comparisons | Implemented | FR-015-AC-14, AC-20 and AC-21. | — |
| V1 `BoundClause`, precondition/postcondition/invariant, Boolean root with a rational, decimal, IEEE-float, text, enum or quantity dependency, and zero definedness obligations | Planned (IR-269) | Remaining V1 scalar clause domains that reach Boolean oracle lowering. | `ClauseLowering { generation_code: UnsupportedDependency }` (`lower_clause` maps `typed_dependency_parameters`' diagnostic after the zero-obligations check). |
| V2 `expression` claim, `OperationProvenance::IrConfirmed`, identity `quire.op.integer.add`, `.sub`, `.mul` or `.negate`, valid `i64` bounds | Implemented | `scalar::lower_scalar_claim`. | — |
| Boolean scalar `expression` claim, IR-confirmed generated claim, operation identity other than the four implemented integer identities above | Planned (IR-269) | Remaining typed Boolean scalar operations. | `OperationNotRendered` (`classify_claim` maps `ScalarLoweringRefusal::NoRenderer`). |
| Integer scalar `expression` claim, IR-confirmed generated claim, operation identity other than the four implemented integer identities above | Planned (IR-269) | Remaining typed integer operations. | `OperationNotRendered` (`classify_claim` maps `ScalarLoweringRefusal::NoRenderer`). |
| Rational scalar `expression` claim, IR-confirmed generated claim | Planned (IR-269) | Rational scalar operation forms. | `OperationNotRendered` (`classify_claim` maps `ScalarLoweringRefusal::NoRenderer`). |
| Decimal scalar `expression` claim, IR-confirmed generated claim | Planned (IR-269) | Decimal scalar operation forms. | `OperationNotRendered` (`classify_claim` maps `ScalarLoweringRefusal::NoRenderer`). |
| IEEE-float scalar `expression` claim, IR-confirmed generated claim | Planned (IR-269) | IEEE-float scalar operation forms. | `OperationNotRendered` (`classify_claim` maps `ScalarLoweringRefusal::NoRenderer`). |
| Text scalar `expression` claim, IR-confirmed generated claim | Planned (IR-269) | Text scalar operation forms. | `OperationNotRendered` (`classify_claim` maps `ScalarLoweringRefusal::NoRenderer`). |
| Enum scalar `expression` claim, IR-confirmed generated claim | Planned (IR-269) | Enum scalar operation forms. | `OperationNotRendered` (`classify_claim` maps `ScalarLoweringRefusal::NoRenderer`). |
| Quantity scalar `expression` claim, IR-confirmed generated claim | Planned (IR-269) | Quantity scalar operation forms. | `OperationNotRendered` (`classify_claim` maps `ScalarLoweringRefusal::NoRenderer`). |
| Record/tuple equality claim with a derivable IR-confirmed operation and bounded shadow shape | Planned (IR-264, IR-241) | FR-015-AC-69 through AC-76. | `OperationNotRendered` (`classify_claim` maps `ScalarLoweringRefusal::NoRenderer` for the non-integer renderer). |
| Record/tuple equality claim whose derivation returns `ExactScalarRefusal::NoDerivableClaim` | Planned (IR-264, IR-241) | Refusal subpath for composite equality. | `NoDerivableClaim` (`classify_claim`). |
| Integer scalar function application with FR-015-AC-37's operation/result contract | Implemented | FR-015-AC-37. | — |
| Function claim with IR-confirmed generated operation identity outside the four `ScalarOperation` identities above | Planned (IR-269) | Remaining function families. | `OperationNotRendered` (`classify_claim` maps `ScalarLoweringRefusal::NoRenderer`). |
| V1 `BoundClause` with one or more expression definedness obligations, regardless of operand type | Planned (IR-269) | Definedness-bearing V1 clause forms. | `DefinednessNotEncoded { obligations }` (`lower_clause` checks this first and reports the exact obligation count). |
| V2 `state`/`state_clause` claim sent through the scalar-claim path and refused by exact-scalar lowering as unsupported node family | Planned (IR-489) | V2 contract clause claim. | `NoFiniteEncoding` (`classify_claim` maps `ExactScalarRefusal::Unsupported`). |
| V2 `StateFrame` contract role for a `postcondition` whose Boolean condition is a negation of a supported pre/post integer comparison | Planned (IR-269) | Negated state-clause comparison form; FR-015-AC-63 names this refusal shape. | `StateFrameRefused { ConditionNotSupported }` (`generate_state_frame_role`, FR-015-AC-63). |
| V2 `StateFrame` frame role for a `postcondition` whose frame grants every state field | Planned (IR-269) | All-fields-granted frame form; FR-015-AC-63 and AC-68 name this refusal shape. | `StateFrameRefused { NothingForbidden }` (`generate_state_frame_role`, FR-015-AC-63 and AC-68). |
| Collection/graph claim whose IR-confirmed generated operation identity has no `ScalarOperation` renderer | Planned (IR-269) | Bounded collection and graph operations outside the integer renderer. | `OperationNotRendered` (`classify_claim` maps `ScalarLoweringRefusal::NoRenderer`). |
| Collection/graph claim node refused by exact-scalar lowering as an unsupported node family | Planned (IR-269) | No-finite-encoding subpath for collection/graph nodes. | `NoFiniteEncoding` (`classify_claim` maps `ExactScalarRefusal::Unsupported`). |
