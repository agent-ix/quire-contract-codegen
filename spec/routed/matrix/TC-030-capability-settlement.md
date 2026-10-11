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
postcondition, invariant, frame, state before/after, bounded collection and
bounded graph. Include the exact operation identities, type/domain bounds and
typed claim references required by FR-015; do not replace them with display
strings. Exercise every supported scalar operation in its family and the
boundary between a valid finite domain and a missing or non-representable
bound. For composite equality, vary record and tuple shape, optional members,
and recursive or over-budget shapes. For functions, exercise supported and
refused result outcomes. For clauses, cover each kind, dependency role and
Boolean/integer expression form. For state and frame items, cover before and
after reads, granted and forbidden fields, and each typed role. For collections
and graphs, cover bounded and unbounded roots and every admitted finite shape.

Run every currently implemented in-set form against a bounded Kani candidate
advertising the required mode and verify `supported`. For each in-set form
whose family status is Planned, use a valid bounded request and verify AC-33:
settlement is `unsupported`, carries the exact typed source refusal in the
status register below, and emits no artifact. Never accept `supported` for a
planned family. Against a bounded-only candidate, run an implemented unbounded
form with and without a finite bound and verify `requires-bound` or the typed
unbounded refusal as AC-31 specifies. Run typed forms outside the complete
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

Under FR-019-AC-25 through AC-33, every implemented complete-V1 family and
role has a valid bounded case that settles `supported` under the required
advertisement; each Planned family settles `unsupported` with the exact cause
in the status register and never `supported` until implemented. The typed
out-of-set controls refuse before generation. V2 clause forms
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

The status register describes implementation state against the complete normative set in FR-019-AC-25. A subset marked implemented does not reduce the criterion's scope.

| Form family | STATUS | Implemented subset or planned owner | Current typed refusal for a valid bounded planned form |
|---|---|---|---|
| Boolean scalar | Implemented (subset) | V1 Boolean clause expressions. | — |
| Boolean scalar | Planned | Remaining typed Boolean scalar forms; IR-269. | `OperationNotRendered` for an IR-confirmed generated scalar claim; `NoFiniteEncoding` for the source's `ExactScalarRefusal::Unsupported` path (FR-015 `classify_claim`). |
| Integer scalar | Implemented (subset) | V1 bounded-integer clauses and V2 confirmed add/subtract/multiply/negate. | — |
| Integer scalar | Planned | Remaining integer operation forms; IR-269. | `OperationNotRendered` for a generated claim without a `ScalarOperation` renderer (`scalar::lower_scalar_claim`). |
| Rational scalar | Planned | IR-269. | `OperationNotRendered` for a generated confirmed claim; `NoFiniteEncoding` for an unsupported-node claim (`classify_claim`). |
| Decimal scalar | Planned | IR-269. | `OperationNotRendered` for a generated confirmed claim; `NoFiniteEncoding` for an unsupported-node claim (`classify_claim`). |
| IEEE-float scalar | Planned | IR-269. | `OperationNotRendered` for a generated confirmed claim; `NoFiniteEncoding` for an unsupported-node claim (`classify_claim`). |
| Text scalar | Planned | IR-269. | `OperationNotRendered` for a generated confirmed claim; `NoFiniteEncoding` for an unsupported-node claim (`classify_claim`). |
| Enum scalar | Planned | IR-269. | `OperationNotRendered` for a generated confirmed claim; `NoFiniteEncoding` for an unsupported-node claim (`classify_claim`). |
| Quantity scalar | Planned | IR-269. | `OperationNotRendered` for a generated confirmed claim; `NoFiniteEncoding` for an unsupported-node claim (`classify_claim`). |
| Record and tuple composite equality | Planned | IR-264 and IR-241; FR-015-AC-69 through AC-76 define bounded-shadow work. | `OperationNotRendered` for an IR-confirmed generated claim without a scalar renderer; `NoDerivableClaim` when claim derivation returns that typed refusal (`classify_claim`). |
| Function application | Implemented (subset) | Integer scalar function application under FR-015-AC-37. | — |
| Function application | Planned | Remaining function families; IR-269. | `OperationNotRendered` for an IR-confirmed generated claim without a renderer (`classify_claim`). |
| Precondition, postcondition and invariant clauses | Implemented (subset) | Frozen V1 `BoundClause` Boolean/bounded-integer subset. | — |
| Precondition, postcondition and invariant clauses | Planned | Complete V1 family coverage and V2 clause claims; IR-269 and IR-489. | `ClauseLowering { generation_code }` when the Boolean clause lowerer rejects the typed form; `DefinednessNotEncoded { obligations }` when its expression contains definedness obligations (`lower_clause`). |
| V1 frame clauses | Planned | IR-269. | `FrameNotClauseRendered` for a V1 `BoundClause` frame item (`negotiate::render`). |
| V2 state before/after and frame roles | Implemented (subset) | Restricted postcondition state/frame roles under FR-015-AC-26 through AC-29 and AC-66 through AC-68. | — |
| Remaining state/frame forms | Planned | IR-269. | `StateFrameRefused { refusal }` with the concrete typed `StateFrameRefusal` from `generate_state_frame_role` (FR-015 refusal table). |
| Bounded collections and graphs | Planned | IR-269; FR-019-AC-24 is negotiation-only and does not claim generation support. | `OperationNotRendered` for a generated confirmed claim without a scalar renderer; `NoFiniteEncoding` for a claim rejected as `ExactScalarRefusal::Unsupported` (`classify_claim`). |
