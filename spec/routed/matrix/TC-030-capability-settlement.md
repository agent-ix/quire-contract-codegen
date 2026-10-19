---
id: TC-030
title: "Verify capability settlement at one negotiation point"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-019
    type: verifies
  - target: ix://agent-ix/quire-specification/TC-271
    type: references
  - target: ix://agent-ix/quire-spec-language/ADR-018
    type: references
  - target: ix://agent-ix/quire-spec-language/ADR-020
    type: references
  - target: ix://agent-ix/quire-spec-language/ADR-021
    type: references
  - target: ix://agent-ix/quire-spec-language/ADR-022
    type: references
  - target: ix://agent-ix/quire-spec-language/ADR-023
    type: references
  - target: ix://agent-ix/quire-spec-language/ADR-025
    type: references
  - target: ix://agent-ix/quire-spec-language/ADR-027
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-335
    type: references
---
# TC-030: Verify capability settlement at one negotiation point

## Description

Verify that every requested capability claim is settled by exactly one
`negotiate_*` arm over a closed backend kind, that each FR-290 rule settles its
own row, including whole-envelope refusal for a repeated backend identity.

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
For FR-019-AC-43 and AC-44, build an admitted checked source and its matching
ContractPackage with one `integer.add` node used by two driver positions.
Select distinct `expression` application occurrences and different direct
`numeric.narrow` conversions through typed `OwnResult` selections and
context roots. Settle both against the same Kani advertisement, then change
only one selection. Separately remove `OwnResult`, mispair the application,
substitute a foreign or unrelated conversion, omit its context root, and
mismatch the ContractPackage source identity. Check each typed cause and
absence of artifact. Pair one selection defect with an extent/mode defect,
then with an earlier candidate defect, to establish the stated precedence.
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

For FR-019-AC-45, submit [A, A], [A, B, A], [A(Linked), A(Process)]
and [A, A'] where A' changes another CG descriptor field. In each case,
assert `negotiate_backend_provider` returns only
`EnvelopeRefusal::ManifestRepeatsBackendIdentity { identity: "A" }` through
its existing `Result<Vec<ItemSettlement>, EnvelopeRefusal>` signature, with
no string cause, manifest or digest payload. Reverse the differing cases.

For FR-019-AC-46, submit [B, A, B, A] in multiple orders and assert the
bytewise-first repeated identity A is reported. Submit [A, a] and assert
these case-distinct identities do not trigger this refusal. Pair a repeat
with an invalid contract version, then with a foreign or absent capability
vocabulary; assert `InvalidCapability` wins and no kind is read.

For planned IR-516 arms, use admitted typed items with one registered candidate
at a time. Derive the form, subject class, selected reduction and resolved
memory model from the admitted package; vary only the candidate advertisement
for a paired positive and negative case. Assert the exact disposition, warning
cause, routed backend and the identities required by FR-019, and that each refused
item emits no artifact or engine request. Do not use display text to select an
arm or count a current unregistered backend as implemented support.

1. For AC-34, enumerate EN-1's TP-1 through TP-4, TP-5, SG-1 through SG-3,
   HP-1 model, HP-2, HP-3 and HP-6 forms, under both admitted model and
   protocol subjects where each form admits the subject. Set the corresponding
   advertisement, then remove it. Include HP-1 code and HP-5 as controls that
   cannot turn into EN-1 every-behaviour claims.
2. For AC-35, pair TP-1, TP-2 `on origin`, TP-2 `on each`, TP-3 and TP-4
   with an SMT candidate advertising each form; verify the selected EN-2 or
   EN-3 method is one ADR-018 assigns. Remove the form advertisement and
   verify TP-4 never selects EN-3.
   Check that negotiation emits no terminal verdict.
3. For AC-36, compare a TP-5 whole-refinement item with and without abstract
   fairness. Verify the EN-1 route retains its safety half and the optional
   liveness half. Route the same whole item to an advertising SMT candidate
   for RE-3 refutation, and request an RE-2 `operation-contract` item
   separately for per-step checking. Assert that only the TP-5 item carries
   the whole-refinement identity and that RE-2 cannot settle it. Remove the
   form advertisement for a refusal control.
4. For AC-37, exercise HP-1 model, HP-2, HP-3, HP-4, HP-5 and HP-6 with the
   candidate and subject classes each form permits. Exercise EN-2 and EN-3
   on HP-2 only where its safety shape permits induction. Verify HP-4 and an
   unadvertised form refuse before engine invocation; verify HP-5 takes its
   possible-property route and HP-1 code is a distinct `operation-contract`
   request.
5. For AC-38, route SG-1, SG-2 and SG-3 through EN-1, then submit each to an
   SMT-only candidate. Verify only SG-1 can route to SMT under advertisement.
6. For AC-39, request no reduction, advertised symmetry, advertised POR,
   their selected combination, and an advertised state constraint, comparing
   the selection in each routed request. Compare symmetry-only, POR-only and
   combined obligation identities with the unreduced identity for the same
   claim and assert they differ. Changing only the state constraint leaves
   that claim's obligation identity unchanged but changes its request
   identity; a result for one constraint must not join the other request.
   Remove one advertisement and supply a POR subject without footprints as
   refusal controls. Verify neither control runs unreduced.
7. For AC-40, repeat one otherwise equal request over a weak `parallel` with
   resolved model `tso`, then `ra`. Advertise and then remove that exact
   model. Verify the accepted route carries it and the other route refuses
   without substituting a weaker model. Use a separate `sc` strong-model
   subject as the control; it must not enter the weak-memory branch.
8. For AC-41, use one form valid on both a model and a protocol subject.
   Hold its kind and form advertisements fixed, then route it through an arm
   that discharges only one subject class and an arm that discharges both.
   Compare dispositions and routed subject identities. A protocol acceptance retains its protocol
   transition-system identity.

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

Every second descriptor of one identity, identical or differing and adjacent
or not, refuses the whole CG envelope with
`ManifestRepeatsBackendIdentity { identity }`. The call returns no settlements,
including for an item using another identity.
The bytewise-first repeated identity is reported regardless of manifest
order; case-distinct identities remain distinct, and contract-version or
vocabulary refusal wins before repeat detection or kind reading
(FR-019-AC-45 and FR-019-AC-46).

For AC-34 through AC-41, every positive case uses a validated candidate and
only a form, subject class and modifier its arm discharges. After candidate
validation, a missing form or modifier advertisement, an excluded form, a
selected reduction without a capable candidate, a POR
subject without footprints, and a resolved memory model the candidate does not
advertise each settle `unsupported`, warned, with
`unsupported_projection`/`unsupported-requested-capability`, emit no artifact
and start no engine. The accepted route preserves its typed form, subject,
reduction and memory model, with symmetry and POR in obligation identity and
state constraint in request identity only. No per-step refinement result replaces the whole
TP-5 result, SG-2/SG-3 never reach SMT, and the protocol subject remains a
protocol transition system. These are planned cases, not current support
claims.

For the kind-advertisement control, distinguish an empty candidate set from a
supplied candidate lacking the item's kind. The empty set settles `unsupported`
under the existing candidate table; the supplied inconsistent candidate
settles `invalid-request` with `invalid_capability`/`inconsistent-candidates`
before a model-checking arm runs. Neither produces an artifact.

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
| Quantity scalar `expression` claim, IR-confirmed generated claim, including identity `quire.op.quantity.convert` | Planned (IR-269) | Quantity scalar operation forms; the conversion's settlement is FR-014-AC-51. | `OperationNotRendered` (`classify_claim` maps `ScalarLoweringRefusal::NoRenderer`). |
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
