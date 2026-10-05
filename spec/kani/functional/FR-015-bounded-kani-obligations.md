---
id: FR-015
title: "Generate separate bounded Kani obligations for complete-V1 oracles"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-018
    type: references
  - target: ix://agent-ix/quire-specification/FR-196
    type: references
  - target: ix://agent-ix/quire-specification/TC-218
    type: references
---
# FR-015: Generate separate bounded Kani obligations for complete-V1 oracles

## Description

When a caller selects Kani generation for complete-V1 contracts whose scalar
oracles FR-014 generated, the code generator shall emit one bounded Kani
obligation per precondition, postcondition, invariant and frame condition,
each bounded by its model-domain bounds. It is the Kani
backend's one generator
([ADR-001](../../decisions/ADR-001-overlapping-generators-and-input-models.md)),
and every item it lowers is a claim over an admitted `quire.checked-package/v2`
package.

Planned (IR-489): the contract families (precondition, postcondition, invariant) take
their input from a V2 clause claim, defined under Inputs and by FR-015-AC-38 to
FR-015-AC-49, together with a V2 census input. FR-015-AC-22 and FR-015-AC-25 state
the census requirement on the request input; once implemented, the V2 census input
(FR-015-AC-44 and FR-015-AC-45) backs them in place of the V1 `ProofDependencyGraph`
([AD-004](../../assurance/AD-004-cg-crate-layout.md) steps 4c and 4d). The V2 clause
claim carries the clause requirement of FR-015-AC-8 on the V2 input. FR-015-AC-1 to
FR-015-AC-37 stay as they were; the new criteria add to them.

Implemented (IR-464): FR-015-AC-7 requires a cover in every generated harness. The V1
bundle generator (`generate_kani_bundle`) and the bounded Kani corpus generator once emitted
a harness with none, so a run of either could not classify `Verified` (FR-017-AC-4); both
now end their harness with one. FR-015-AC-53 to FR-015-AC-58 state what each of the seven
kinds' covers witnesses and guard against an emitted harness without one.

Planned (IR-264, IR-241): composite equality over records and tuples is the first family whose
production harness cannot verify within any ceiling this crate has measured (FR-028, Rationale), so
it proves its claim over a bounded shadow together with a refinement obligation, as ADR-003 Q1
decides. FR-015-AC-69 to FR-015-AC-76 state that family's item, drawn arguments, assertions,
source shape, refusals, real-Kani corpus case, seeded mutants and identity; FR-028-AC-13 to
FR-028-AC-24 state the family-agnostic contract they use. The scalar families (Boolean, bounded
integer) keep their production harnesses, and the shadow supports no other family yet (FR-015-AC-73
refuses each).

Rationale (IR-464): a corpus case draws no symbolic input and assumes nothing, so it has
no precondition for a cover to witness and no assumption that could make its cover
unsatisfiable. Its cover shows only that the harness runs to its end past its assertion,
which is what FR-017 needs to read the run as `Verified` rather than as a run with no
cover summary; no vacuous corpus run exists to classify. The generator has no single
emission seam today: seven templates in six files each format harness source, and
`HarnessSpec` (AD-004 step 4b) does not exist in `src/`. A constructor that refuses an
empty cover list cannot be built over those templates in this change, and AD-004
sequences the corpus behind QSL-353, so the guard of FR-015-AC-58 is an inspection of
emitted text. When a family renders through `HarnessSpec`, its constructor refusal
(AD-004 L-4) joins the inspection and does not replace it.

## Inputs

- The FR-014 oracle crate and claim map for the contract's expressions.
- Contract IR's lowered claims and bounds for each obligation, read from an
  admitted `CheckedPackageV2`. Bounds come only from the lowered IR
  `bounded_domain` nodes, never from a caller descriptor, and are tightened
  only as [FR-028](./FR-028-bounded-proof-ceilings.md) records.
- An optional declared proof-dependency census per obligation.
- A complete request. A postcondition or invariant is proved under the
  preconditions of its own anchor operation, so the request must name every
  package precondition sharing that anchor as an item of the same request. The
  caller does not choose a solver.
- The loop unwind bound, which is a request-level value in `1..=1024`.
- A V2 clause claim: one `state`/`state_clause` node id of an admitted
  `CheckedPackageV2` whose `operation.member.clause` is `precondition`, `postcondition`
  or `invariant` (the node shape Contract IR FR-040 admits), with the package it
  belongs to. The obligation kind is the node's `clause` value and is not chosen by the
  caller. The clause's parameters are `self` (a `Reference` to the anchor's object
  type), then for a postcondition the result, then the operation's parameters. The
  clause's leaf reads are the state fields of `self` the body reads, each a
  `project(deref(self), field)` term, written bare for the state the clause is judged on
  and as `pre(...)` for the pre-state, the result of a postcondition, and each operation
  parameter. A `Reference` parameter has no `bounded_domain` and is not itself an
  argument. The harness draws only the pre-state of every state field the body reads,
  bare or under `pre(...)` (it is the subject's input state, which FR-025 passes), and
  the operation parameters (for a
  precondition or invariant, the state the clause is judged on is the drawn state); a
  postcondition's result and its bare post-state reads are the values the customer subject
  produces, never drawn. A drawn value's declared domain is the object's `integer_range` for a
  state field (FR-015-AC-27, FR-015-AC-77) and the IR `bounded_domain` of its type for a
  parameter, never a caller descriptor; a produced value's domain is asserted, not
  assumed. The clause body is the node's Boolean condition, an inline term of the
  node, which may hold Boolean connectives, bounded-integer comparisons and the integer
  arithmetic expressions add, subtract, multiply and negate.
- The V2 census input: an optional declared proof-dependency census per V2
  obligation, carried by the same typed census request and readiness types the
  corpus uses, and not by `ProofDependencyGraph`.
- A composite-equality claim: one `binary` `expression` node of an admitted `CheckedPackageV2`
  that [FR-018](../../oracle/functional/FR-018-composite-equality-oracles.md) generated an oracle
  for, with that oracle's claim-map entry (its reconstructed declaration closure, in declaration
  order, with each member's presence and each leaf's type and bound) and its typed equality
  descriptor. The descriptor selects the operator and the operand types and never a bound; every
  bound comes from the IR `bounded_domain` nodes the closure reaches. Each operand is a parameter
  or a literal. The request also carries the refinement case cap of FR-028.

## Outputs

- One Kani harness per obligation kind and claim, with its bounds, solver,
  option vector and unwind bound recorded in its identity.
- Exactly one non-vacuity cover per harness, which is what distinguishes a
  proof from a harness whose assumptions are jointly unsatisfiable.
- For a postcondition or invariant, the assumed preconditions recorded in the
  harness identity's embedded oracles, so a reader can see the claim actually
  proved rather than the claim the clause states alone.
- A typed refusal for each obligation that has no finite encoding.

## Behavior

- When generating proofs, the generator shall emit precondition,
  postcondition, invariant and frame obligations as separate harnesses.
- When an obligation has a finite model domain, the generator shall bound
  every symbolic input by that domain without assuming away refused,
  undefined or incomplete outcomes.
- If an obligation's domain is unbounded or its family lacks a finite
  encoding, then the generator shall refuse it with a typed reason and emit no
  harness.
- If an obligation's bounds are unsatisfiable, so that the harness would hold
  vacuously, then the generator shall refuse it with a typed reason and emit no
  harness.
- If an obligation depends on an FR-014 oracle whose operation identity is
  `caller_declared`, then the generator shall refuse it with a typed reason
  naming that blocked item and emit no harness.
- The generator shall end every generated harness with exactly one
  non-vacuity cover: a precondition harness covers that the precondition holds
  within the IR bounds, and a contract harness covers, at a point after the
  contract call, that the requires and the IR bounds are jointly satisfiable. A
  backend run that reports success without satisfying that cover has proved
  nothing, and the cover is the only thing that says so.
- When lowering a postcondition or invariant, the generator shall assume every
  package precondition sharing the obligation's anchor operation, emitting each
  as a `requires` on the generated contract and recording it in the harness
  identity's embedded oracles. The claim proved is therefore the obligation
  under those preconditions, and never the obligation alone.
- If a precondition sharing that anchor is not a supported item of the same
  request, then the generator shall refuse the obligation with a typed reason
  naming that precondition and emit no harness, rather than proving the
  obligation under fewer assumptions than the contract states.
- The generator shall lower every obligation against the `cadical` solver and
  without stubbing, and shall record the solver and the complete ordered option
  vector in the harness identity. No option enabling stubbing is emitted,
  because a stub is an assumption this requirement does not admit.
- The generator shall bound every symbolic argument by an inclusive assumption
  taken from its IR `bounded_domain`, and shall require a bounded-integer
  post-state result to remain inside the same domain before the clause can
  hold.
- The generated source shall carry no `#[kani::unwind]`; the unwind bound
  reaches the backend only as an explicit option in the recorded vector, so the
  bound a harness was proved under is read from its identity rather than from
  its text.
- Regeneration from equal inputs shall be byte-identical, and the unwind bound
  and the customer subject shall each change the harness identity.
- If the request names no items, more than 256 items, or an unwind bound
  outside `1..=1024`, then the generator shall refuse the whole request and
  account no item.
- If an otherwise-supported obligation's generated harness source would exceed
  the crate's bounded-resource ceiling, then the generator shall refuse it with
  a distinct resource-limit ground naming the generated size and emit no
  harness; if it fits that ceiling but does not parse as Rust, then the
  generator shall refuse it with a distinct syntax ground naming the parse
  error and emit no harness; neither is reported as the internal-invariant
  fallback used for an otherwise-successful render whose harness could not be
  assembled.
- If a V2 scalar-claim item's graph node carries a tag/form pair the generator
  does not model as a contract role, then the generator shall refuse it with a
  typed reason naming that tag and form and emit no harness, rather than
  accounting it as supported with no contract role recorded; a claim naming a
  node id absent from the graph entirely is likewise refused, with no harness
  emitted.
- If a V2 scalar-claim item's claim is a refusal that FR-022 recorded because the
  node carries no derivable FR-014 descriptor (`NoDerivableClaim`), then the
  generator shall account it `unsupported` with the `no_derivable_claim`
  reason naming the node and the derivation refusal, emit no harness, and not
  reject the request.
- Where an operand of a V2 scalar claim is a literal, inline or a reference to a
  `value` node whose body is a literal, the generator shall constrain that
  symbolic argument to exactly the literal's own value.
- If a literal operand's value does not fit `i64`, then the generator shall
  refuse the item as `domain_not_representable_in_i64` naming the value as both
  endpoints and emit no harness.
- If the exact results of the operation over every operand range lie outside
  the result range, then the generator shall refuse the item as
  `result_bound_unreachable` naming the result range and the reachable result
  range, and emit no harness, since no assumed input could meet its
  non-vacuity cover.
- When a transition obligation reads state, the generator shall emit a harness
  that asserts the obligation over the pre-state and the post-state the
  subject ABI binds ([FR-025](./FR-025-generated-subject-abi.md) owns how state
  reaches the subject).
- When a claim is a Boolean connective or a bounded-integer comparison, the
  generator shall embed its FR-014 oracle byte-identical to the oracle crate's
  function, so that the harness decides the same verdict the oracle decides.
- If a precondition reads a post-state value, then the generator shall refuse
  the obligation with a typed reason naming the node and emit no harness.
- When a request declares a proof-dependency census for an obligation, the
  generator shall admit only `Required` dependencies, fold the census into the
  harness identity, and record generation-time readiness `ready` when every
  dependency passed and `incomplete` while any is missing or failed, with proof
  execution recorded `not_run`.
- If a declared census has an empty or duplicate dependency identity, an
  inconsistent kind, state and path combination, or a kind other than
  `Required`, then the generator shall refuse the obligation as a typed invalid
  input and emit no harness.
- The generator shall give every requested item exactly one
  `ObligationDisposition`: `supported`, `requires-bound`, `unsupported` or
  `invalid-request`, the FR-331 disposition set.
- If an item's disposition is `requires-bound`, `unsupported` or
  `invalid-request`, then the generator shall keep its source identity and typed
  reason and emit no harness and no assumption for it.
- When a request names a V2 clause claim, the generator shall read the claimed
  `state`/`state_clause` node of the admitted `CheckedPackageV2`, take its obligation
  kind from the node's `clause` value, and emit one harness for it, separate from the
  harness of every other clause (FR-015-AC-38).
- The generator shall draw one nondeterministic value per drawn input of a V2 clause
  claim (the pre-state of every state field the body reads, bare or `pre(...)`, and the
  operation parameters; for a precondition or
  invariant, the state the clause is judged on), as the Inputs bullet defines them,
  under an inclusive assumption equal to that input's declared domain; shall take a
  postcondition's result and post-state reads from the subject call, asserting their
  domains and never assuming them; and shall take no bound from a caller descriptor
  (FR-015-AC-39).
- When a V2 clause body holds Boolean connectives, bounded-integer comparisons or
  integer arithmetic expressions, the generator shall embed an FR-014 oracle of that
  body byte-identical to the oracle crate's function, and shall not assume away a
  `Refused` arithmetic outcome (FR-015-AC-40, FR-015-AC-41). The oracle of an inline
  clause term depends on the planned FR-014-AC-38.
- If a V2 clause body holds an operator the V2 arm does not support (division, modulo,
  absolute value and every other operator outside the Inputs bullet's grammar), then the
  generator shall account the item `unsupported` with a typed reason naming the node and
  the operator and emit no harness; a node that is not a modelled clause role, or a node
  id absent from the package, is refused as FR-015-AC-14 states. No item is skipped
  silently (FR-015-AC-42).
- When lowering a V2 postcondition, the generator shall assume every `precondition`
  clause node of the same anchor node, as for the V1 clause under FR-015-AC-8
  (FR-015-AC-43).
- When a request declares a census on a V2 obligation, the generator shall validate it
  by the rules of FR-015-AC-22, and shall fold it into that obligation's identity and
  record its readiness by the rules of FR-015-AC-25 (FR-015-AC-44, FR-015-AC-45).
- The generator shall end every V2 clause harness with exactly one non-vacuity cover,
  placed after every assumption and after the subject call, that states the family's own
  reachability: a precondition holds, a postcondition's subject call completes with its
  requires satisfied, and an invariant's drawn state satisfies the domain assumptions
  (FR-015-AC-46; IR-464).
- Regeneration of a V2 clause harness from equal inputs shall be byte-identical
  (FR-015-AC-47).
- The generator shall form the obligation identity of a V2 clause claim from the members
  AD-003 E-1 names, with the source span excluded, and shall keep it distinct from the
  harness identity record (FR-015-AC-48). Whether E-1's member list covers a state field
  read, which is not a parameter, is an open question for the AD-003 owner.
- A V2 invariant harness shall assert that the invariant clause holds for every state its
  arguments' declared domains admit; preservation of the invariant under an operation is
  not specified here (FR-015-AC-49).
- When the exact-scalar oracle refuses an obligation's node as `LoweringByteLimitExceeded` or
  `LoweringLimitUnrecognised` (FR-014; IR-547), the negotiation shall report the obligation as
  `OracleRefused` carrying that refusal unchanged, as it does for `LoweringWorkExhausted`
  (FR-015-AC-50).
- When the bounded Kani corpus generator is given a finite input validated under a different
  profile selection than the profile offered with it, it shall refuse the case with a typed
  `InvalidInput` result, record no case identity and emit no artifact (FR-015-AC-51).
- The bounded Kani corpus generator shall return the revision of the profile selection as the
  context of every typed outcome and refusal it returns (FR-015-AC-52).
- The generator shall end a V1 bundle harness (`generate_kani_bundle`) with one cover, after
  its contract call, that witnesses the bundle's requires clause and bounds are jointly
  satisfiable (FR-015-AC-54).
- The generator shall end a bounded-corpus harness, of the arithmetic, graph and collection
  families alike, with one cover after its assertion (FR-015-AC-55).
- When the installed backend runs a V1 bundle harness whose requires clause some bounded
  argument satisfies and whose ensures holds for every such argument, the generator shall
  classify the run `Verified` (FR-015-AC-56).
- If the installed backend runs a V1 bundle harness whose requires clause no bounded argument
  satisfies, then the generator shall classify the run `CoverUnsatisfied`, never `Verified`
  (FR-015-AC-56).
- When the installed backend runs a bounded-corpus harness whose oracle is true, the
  generator shall classify the run `Verified` (FR-015-AC-57).
- If the installed backend runs a graph or collection corpus harness whose oracle is false, then the
  generator shall classify the run `Falsified` (FR-015-AC-57).
- The crate shall carry a gate that fails when a harness it emits has other than exactly one
  cover as the last statement of its body, and when a source file spells a proof attribute
  the gate does not drive, or a driven file spells more of them than the gate counts
  (FR-015-AC-58).

IR-461 (both code changes have landed; FR-015-AC-60 and FR-015-AC-68 stay planned for the one comparison
their rows name): the state and frame lane gives every state-clause obligation of a package
a disposition through `negotiate_kani_obligations`, as the scalar lane does, so the
per-construct accounting QSL-20 needs comes from the one machinery of FR-015-AC-23. It adds
one `ObligationItem` arm, `StateFrame`, which is the frame arm AD-004 step 4d plans, and no
public entry and no disposition type: the records are `ObligationRecord`s, the dispositions
are `ObligationDisposition`'s four, and the reasons are `UnsupportedObligation`'s and
`InvalidObligationItem`'s, plus the five new reasons named below. A `StateFrame` item names
one clause (the `state`/`state_clause` node of a `postcondition`), one role (`contract` for
the operation-contract harness, `frame` for the frame-effect harness), the state struct path,
its fields and the operation's subject path, the inputs of `StateFrameRequest`; the request's
unwind bound applies. The item's subject path is the one its harness calls; the request's own
`subject_path` serves the other arms, is still validated first, and is not read by a
`StateFrame` item. A clause asked in both roles has two items and two records
(`kind` `postcondition` and `frame`). The role split of the engine adds
`generate_state_frame_role(request: &StateFrameRequest, role: StateFrameRole) ->
Result<StateFrameHarness, StateFrameRefusal>`, one role in and its harness or its first refusal
out (crate-internal, not a second public entry); the arm calls it once per item, and it is the comparison target of FR-015-AC-60 and
FR-015-AC-68. `generate_state_frame_obligations` stays the single-clause entry: it returns both
roles' harnesses when both succeed and otherwise the first refusal of its one clause, so for a
clause whose roles differ it returns the refusal and the arm records one item `supported`;
AD-004 step 4d retires it as a public entry. This arm does not change FR-025's rule that a V1 frame item is `unsupported`
(`FrameNotClauseRendered`); that rule is about `BoundClause` items.

Relation to the V2 clause arm: planned FR-015-AC-38 (IR-489) also yields a `postcondition`
harness for the same `state_clause` node, a different harness (any clause body, drawn inputs)
from the `contract` role's (one integer comparison over the fixed single-struct ABI of
`StateFrameRequest`). The arms coexist. `DuplicateItem` spans only items of one arm and role, so a
clause may be named once in each arm. Whether the `contract` role retires when AC-38 lands is
IR-489's decision and is not settled here.

A refusal of the single-clause engine becomes a record by one total mapping, a `match` over
every `StateFrameRefusal` variant with no wildcard arm, so a new variant fails to compile.
`NotLowered` carries the six refusal arms of `CompleteLoweringRecordV2` and not its `Lowered`
arm: the payload is a type that cannot hold a lowered node, so the mapping has no row, no
`unreachable!` and no invented reason for one.

The roles are independent. A ground common to both roles refuses both items alike: a malformed
request, the lowering, a node that is not a `state_clause`, a clause that is not a postcondition
and a malformed clause. A ground of the contract harness refuses the `contract` item only: the
condition shapes, the clause field's bound and the clause field missing from the state. A ground
of the frame harness refuses the `frame` item only: a frame effect outside the encoding, a frame
granting every field and a granted field missing from the state. A render ground (source over
the ceiling, source that does not parse, identity that does not serialize) refuses the item
whose harness it is. So a frame granting every field leaves the `contract` item `supported`.

| Refusal (arm) | Disposition and reason |
|---|---|
| `NotLowered` (`RequiresBound`) | `requires_bound`, `unbounded_type` the record's |
| `BoundNotResolved`, where the clause field's member `value.target` is an unbounded type (a node that is not a bound) | `requires_bound`, `unbounded_type` that target node |
| `BoundNotResolved`, where that target is a bound that is not an `integer_range`, or an `integer_range` with an endpoint that does not fit `i64`, or the object has no member of that name, or the member's value is not a reference and so has no `value.target` | `unsupported`, `StateFrameRefused` carrying the refusal, which names the field and the bound node |
| `BoundNotResolved`, planned (IR-624, FR-015-AC-78): where a model declaration node's field has no read (`FieldNotRead`), its reads name two types (`ConflictingFieldReads`), or an object's member and a read of the field name two types (`MemberDisagreesWithRead`) | `unsupported`, `StateFrameRefused` carrying the refusal, which names the field and the nodes it has |
| `NotLowered` (`Unsupported`) | `unsupported`, `NoFiniteEncoding` naming the record's `unsupported_node_id` and its family, the mapping negotiate gives a scalar `ExactScalarRefusal::Unsupported` |
| `NotAStateClause` | `unsupported`, `UnknownNodeKind` naming the node, family and form |
| `ConditionNotSupported` (a shape other than one integer comparison of one pre and one post read of one field through `self`: negation, a literal operand, an operator other than the six integer comparisons, a read through another parameter), `ObservationsDiffer`, `ObservationsSameSide` | `unsupported`, `StateFrameRefused` carrying the refusal. Not `NoFiniteEncoding`: these shapes have a finite encoding (planned FR-015-AC-40 encodes `not` and the six comparisons for the same node), this arm does not render them |
| `FrameEffectUnsupported` | `unsupported`, `StateFrameRefused` carrying the refusal and the effect (`Creates`, `Deletes`, `Relationship`, `ForeignField`). Not `NoFiniteEncoding`, which in this crate says a node family has no finite encoding; the `state` family of a frame is what this arm encodes, and `ForeignField` is encodable outside this single-struct ABI |
| `NotAPostcondition`, `MalformedClause`, `NothingForbidden`, `NotLowered` (`InvalidBody`, `BodyIncomplete`, `Failed`) | `unsupported`, `StateFrameRefused` carrying the refusal |
| `ResourceLimitExceeded` | `unsupported`, `ResourceLimitExceeded` with the same size |
| `InvalidGeneratedSyntax` | `unsupported`, `InvalidGeneratedSyntax` with the same error |
| `RecordSerialization` | `unsupported`, `RenderFailed` |
| `NotLowered` (`InvalidInput`) | `invalid_request`, `UnknownNode` |
| `InvalidPath` (the state path or the item's subject path) | `invalid_request`, `InvalidStatePath` naming the path |
| `InvalidField`, `UnknownStateField` | `invalid_request`, `InvalidStateField` naming the field |
| `UnwindOutOfRange` | `invalid_request`, `InvalidStateUnwind` naming the bound; a request reaches it only through the mapping, since the request-level `InvalidUnwind` error precedes it |

The engine's `BoundNotResolved` is split to carry what the table keys on: the field, and a
`cause` (`BoundNotResolvedCause`) that separates an unbounded type from a bound that is not an
`integer_range`, an endpoint outside `i64`, an absent member and a value that is not a
reference, each carrying the member's `value.target` node when it has one. The new reasons are
`UnsupportedObligation::StateFrameRefused { refusal }`, serialized as
`code: state_frame_refused` with the refusal's snake_case code and the node, field and
effect it names (never the IR record), and `InvalidObligationItem::InvalidStatePath { path }`,
`InvalidStateField { name }`, `InvalidStateUnwind { unwind }` and `MixedStatePackages`. A field
name read from the graph that is not a Rust identifier is a malformed clause, `MalformedClause`
at the node, not `InvalidField`, which names only a name the caller supplied; an operand node
that cannot be followed in the graph is likewise `MalformedClause`, not `ConditionNotSupported`;
an admitted package never holds an unresolved reference (the IR reader refuses it), so that case
has no criterion and no test here, and only the identifier case is asserted (FR-015-AC-67). Both
the engine and the arm report them so.

A supported item's harness is returned in a new `state_frame_harnesses: Vec<StateFrameHarness>`
field of `KaniObligationOutcome::Emitted`, one per `supported` `StateFrame` record, in request
order. A `Rejected` outcome has none.

The code lands in two changes that trace to these criteria. The first adds the mapping, the five
reasons, the `NotLowered` payload and `BoundNotResolved` changes and the engine's
`MalformedClause` reclassification, and backs FR-015-AC-66 and FR-015-AC-67 against the mapping
and the engine. The second adds the `StateFrame` arm, the `state_frame_harnesses` field and the
role split of the engine, and backs FR-015-AC-59 to FR-015-AC-65 and FR-015-AC-68, which are
about items in a negotiated request.

- When a request holds `StateFrame` items, the generator shall return one `ObligationRecord`
  per item, in request order, with `kind` `postcondition` for the `contract` role and `frame`
  for the `frame` role (FR-015-AC-59).
- If a `StateFrame` item is refused, then the generator shall settle every other item of the
  request by the rule it has in a request holding that item only (FR-015-AC-59).
- When a `StateFrame` item yields its harness, the generator shall record it `supported`
  carrying the harness symbol (FR-015-AC-60).
- When a `StateFrame` item is recorded `supported`, the generator shall return its harness in
  `state_frame_harnesses`, byte-identical to the harness `generate_state_frame_role` returns
  for that clause and role (FR-015-AC-60).
- If a `StateFrame` item's refusal is `requires_bound` by the table above, then
  the generator shall record it `requires_bound` (FR-015-AC-61).
- If a `StateFrame` item is recorded other than `supported`, then the generator shall emit no
  harness for it (FR-015-AC-61 to FR-015-AC-63).
- If a `StateFrame` item's refusal is `unsupported` by the table above, then
  the generator shall record it `unsupported` with the reason the table gives (FR-015-AC-62,
  FR-015-AC-63).
- If a `StateFrame` item's refusal is `invalid_request` by the table above, then
  the generator shall record it `invalid_request` (FR-015-AC-64).
- If a `StateFrame` item repeats an earlier item (the same clause and role) or names a package
  other than the first `StateFrame` item's, then the generator shall record it `invalid_request`
  (FR-015-AC-64).
- If a request holds an item recorded `invalid_request`, then the generator shall return every
  item's record with no harness bytes, as negotiate does for any invalid item (FR-015-AC-64).
- If a request holding `StateFrame` items names no items, more than `MAX_OBLIGATION_ITEMS`,
  an unparsable request subject path or an out-of-range unwind bound, then the generator shall
  refuse the whole call with the existing `KaniObligationError` and account no item
  (FR-015-AC-65).
- The refusal-to-record mapping shall be a `match` with no wildcard arm over every
  `StateFrameRefusal` variant (FR-015-AC-66).
- When the single-clause engine reads a graph field name that is not a Rust identifier, the
  engine shall refuse with `MalformedClause` at the node (FR-015-AC-67).
- If a refusal concerns only one role of a clause, then the generator shall record the other
  role's item by that role's own result (FR-015-AC-68).

IR-624 (planned): where a state field's declared range is read from. Measured on the package QSL
emits (`qsl_replay::call_site`'s `CallSite::package`, admitted by `CheckedPackageV2::read` with the
domain package document and `quire.value.complete/v1` as evidence; the twin of
`tests/state_frame_support/native_twin.rs`, `Twin::emitted_package`): the framed object's
`model`/`object_type` node has body `{"members": [], "term": "aggregate"}`, and QSpec FR-322
("Model-owned members") says it stays so: a model declaration node's semantic type is its own id, it carries no
declaration and its body is empty, and its fields are resolved against the selected domain package
document, not against the node. The `bounded_domain`/`integer_range` node (0 to 1000 for `balance`
and for `audit`, one node since the bounds are equal) is the `result_type` of each
`expression`/`query` node that reads a field: an application of `quire.op.record.project` over a
`quire.op.model.deref` whose `operation.member` is `{kind: "field", name, declaration}`, with
`declaration` the framed object's node. The Contract IR reader admits such an application only when
its `result_type` names the node of the field's member type, which it derives from the selected
domain document by the model-owned member resolution (QSpec FR-322 step 4, FR-322-AC-31 and
FR-322-AC-34; Contract IR FR-038 and FR-040 "Model forms"; `check_model_member` in the reader's
`operations.rs`). That check is partial, and the limit is stated here and not buried. (1) It
compares only the digest of `result_type` with the key of the derived member type. It never
re-derives the key of an anonymous structural node such as a `bounded_domain` from that node's
body, so a package whose `integer_range` node carries the key of `Int[0, 1000]` and the bounds 0 to
10 or 0 to 5000 is admitted (measured in the review of CG #285, reproduced with a recomputed
`package_id`; IR-627). (2) It runs only when the object is a model declaration node (QSpec FR-322
step 2); for any other object the reader checks only that the field's name is declared
(`check_field_member`), so a read's type is unchecked. The package therefore carries a range in one
place, and the range a harness assumes from it is only as trustworthy as IR's admission, which today
does not bind the bounds. CG cannot close that itself: the preimage of a structural node key is
not published by QSpec ("QSpec does not publish", `structural.rs`) and IR exports no function for
it, so re-deriving the key in CG would be a copy of an unpublished derivation, which this
repository does not make. The dependency is on IR: either IR re-derives anonymous structural node
keys at admission (IR-627) or it exports a typed accessor that returns the bounds as values
(IR-628, Q-5 of FR-024). FR-015-AC-77 to FR-015-AC-81 and FR-024-AC-31 to FR-024-AC-35, every
criterion that reads a range from an emitted package, are GATED on IR-627 or IR-628; they stay
planned. Both are specified in Contract IR's merged FR-038 and neither is landed: IR-627's decided stage
(IR #295, amended by #298: FR-038-AC-123 to AC-127 and AC-145 to AC-150) and IR-628's accessor
`CheckedPackageV2::model_object_fields` (IR #296: FR-038-AC-136 to AC-144, TC-227) are specification
only, and their code is open as IR #297 (IR-627) and IR #299 (IR-628), not merged at the time of writing.
The gate lifts, and the word "landed" applies, only when the IR code merges. The IR-624 code shall not emit
a harness from the model declaration path until IR-627's stage has landed or IR-628's accessor is in use,
so the unsound range of the stated limit is never accepted; the code change is gated on IR-627 or IR-628,
either one. IR-628's accessor is independent of IR-627's stage (FR-038 "What the accessor guarantees"), so
neither order is required.

What each IR route covers, as measured in the merged FR-038 text (an IR claim, re-read there, and
unlanded):

- IR-628's accessor reads a model declaration object's effective fields and their member types from the
  field table the reader retains from the selected domain document, not from any node body: for a
  model declaration node it returns each field with its type, `IntRange` bounds as values, `None` for a
  type the tables give no type, and `UnknownNode`, `NotModelObjectType` and `AmbiguousField` as typed
  errors. It is the route for a model declaration object's declared fields and ranges. It does not make a
  node body trustworthy, closes no body route, and does not extend to an object that is not a model
  declaration node. When it lands this crate adapts: FR-015-AC-77, AC-78 and AC-81 (`result_type` route,
  `FieldNotRead`, `ConflictingFieldReads`, `MemberDisagreesWithRead`, `NoRead`) and the FR-024 criteria
  that restate them (FR-024-AC-31 to AC-35) are amended to the accessor route and not only ungated,
  retiring the `result_type` route, its three causes and the `NoRead` reason for that path, keeping the body
  path for objects that are not model declaration nodes, and keeping the IR-627 gate on every range read
  from a node body. That adaptation is not made here: those criteria are PLANNED and read consistently
  with the code that has not landed (each names "IR-627 or IR-628" and its route), so amending them
  before the accessor exists would describe an interface that does not exist.
- IR-627's decided stage always re-derives and checks the key of `bounded_domain` `integer_range`,
  `scalar_type` `boolean` and `integer`, and `composite_type` `reference`, whatever `recursion_group` a node
  carries (FR-038-AC-146). A range read through an `integer_range` node is therefore verified in every
  package once IR-627 lands, which is the route FR-015-AC-77 reads.
- IR-627's stage does not verify an `option`, `set`, `bag`, `sequence`, `ordered_set` or
  `collection_bounds` node that carries a `recursion_group`, lies on a cycle of the names graph and sits in a
  component whose every node carries that label: it skips it, and a changed `max` of the in-group
  `collection_bounds` node of `Sequence<Tree>[0, 3]` admits (the stated limit, FR-038-AC-148). A forged group
  extends the limit to a node a tamperer places on a cycle. Verifying them waits on IR-630 (in-group
  re-derivation, FR-038-AC-150), which is blocked by QSL-638 (the owner on the v2 wire).

The in-group limit does not touch IR-624's reads and is not an IR-624 requirement. IR-624's reads (a state
field's range, FR-015-AC-77 to AC-81, and FR-024's) read an `integer_range` node or the accessor, which
IR-627 verifies whatever label a node carries or which reads no node body, so they stay governed by the
gate above; a node retagged to a skippable shape is not an `integer_range` and gives no range. The reads
the limit does affect exist today and belong to FR-018 (the composite equality oracle), whose behavior
this change specifies as FR-018-AC-24, planned and not gated on IR-630. IR-630 is what later lifts the limit,
and then retires FR-018-AC-24 by an amendment that names the shapes IR verifies; it is not a precondition
of the rule. Measured in this repository's source:

- Affected: FR-018's composite equality oracle reads the element and payload type nodes of an `option`,
  `sequence`, `set`, `bag` and `ordered_set` composite and the `collection_bounds` `min` and `max` it names
  (FR-018-AC-14; `resolve_type` in `src/oracle/equality/mod.rs`). A refused FR-018 item reaches the Kani
  route as FR-018's own refusal (FR-015-AC-73), so the Kani composite-equality path inherits the behavior.
- Not affected: the `collection_bounds` read of `unsatisfiable` (`src/kani/generate/scalar.rs`; its callers
  in `negotiate.rs` use it only to refuse a bound whose minimum exceeds its maximum, so a forged count can
  cost an item and can never prove one), the `integer_range` operand bounds of FR-014 and FR-015's scalar
  route, and the state field ranges above.
- Precedence with FR-015-AC-73: the unsupported-shape row for a declaration that reaches itself (a recursive
  record or tuple) settles `requires_bound`, `unbounded_type` (planned, FR-015-AC-73), and it shall run first on the Kani path, so a
  `Tree` or `List` package of IR's FR-038-AC-145 settles `requires_bound` there and never reaches
  FR-018-AC-24. FR-018-AC-24 is what the direct FR-018 entry returns for such a package, and what any path
  returns for an in-group node that no recursive declaration reaches, for example IR's forged group
  (FR-038-AC-148).
- IR's stated open routes that are not in-group (a `value` or `parameter` node's `semantic_type`, an alias,
  record, tuple or union composite over a range, and a declared `bounded_domain`) stay open until IR's
  FR-038-AC-131 gate lifts; this change places no new obligation for them and the existing GATED criteria
  keep that gate.

The shared reader `field_range` separates two kinds of framed object:

- A model declaration node is a `model`/`object_type` node whose body is the empty aggregate, with
  no `declaration`, as QSL emits. A read of `field` on `object` is an `expression`/`query` node
  whose application operation is `quire.op.record.project` with member kind `field`, member name
  `field` and member declaration `object`; a `pre_read` node wraps such a read and names no member.
- Where the object is a model declaration node and the package holds reads of `field` on it that
  all name one `result_type`, `field_range` shall take that node as the field's declared type, and
  as its range when the node is a `bounded_domain`/`integer_range` with `i64` endpoints
  (FR-015-AC-77).
- Where the object is any other node, `field_range` shall read the field's type from the object's
  body member exactly as it does today, and shall trust no read's `result_type` in its place
  (FR-015-AC-77).
- If the object is not a model declaration node and a read of `field` names a `result_type` other
  than the member's `value.target`, then `field_range` shall return `MemberDisagreesWithRead` naming
  both nodes (FR-015-AC-78).
- If the object is a model declaration node and no read of `field` names it, then `field_range` shall
  return `FieldNotRead`; for the clause's own field that cannot happen, because the clause reads it
  (FR-015-AC-78).
- If two reads of `field` on a model declaration node name two different `result_type` nodes, then
  `field_range` shall return `ConflictingFieldReads` naming both (FR-015-AC-78).
- `field_range` shall pick neither of two reads that disagree (FR-015-AC-78).
- The existing causes (`MemberAbsent`, `ValueNotReference`, `UnboundedType`, `NotIntegerRange`,
  `EndpointOutsideI64`) are kept, with the names and payloads they have; no cause is renamed or
  removed, so the Covered criteria that name them stand. `FieldNotRead`, `ConflictingFieldReads`
  and `MemberDisagreesWithRead` are added, mapped to `unsupported`, `StateFrameRefused` by the
  table above (FR-015-AC-78).
- If a state field other than the clause's has no range, then the generator shall list it in
  `state_fields`, as FR-024-AC-23 settled for the record (FR-015-AC-78).
- If a state field other than the clause's has no range, then the generator shall give it no entry
  in `domains` and draw it without an assumption (FR-015-AC-78).
- If the clause's own field has no range, then the generator shall refuse the item (FR-015-AC-27,
  FR-015-AC-61, FR-015-AC-63).
- Where the generator draws a state field without a range, it shall record in the harness identity
  that field and the reason it has none: `NoRead` (the package carries no read of it, so whether
  the model declares a range is unknown to the package) or `TypeNotRange` (its type is known and is
  no `i64` `integer_range`). `domains` alone cannot tell the two apart (FR-015-AC-81).
- The generator shall take the field set and the draw order from the request's `state_fields`, the
  fields of the subject's state struct (FR-025) in the caller's order, because the package carries
  no field list: the object body is empty and the frame names only the granted fields
  (FR-015-AC-79).
- The generator shall keep its refusals that the clause's field and every granted field are among
  `state_fields` (FR-015-AC-29, FR-015-AC-79).

An unranged draw is the main case of a frame harness, since the fields a frame does not grant are
usually read by no clause. It is not refused, because refusing would refuse nearly every frame
harness generated from an emitted package until IR-628 lands, and the unconstrained draw is sound
for the proof: it is wider than the model. Its cost is at replay. A falsified run may bind a value
outside the model's range for such a field, and QSL refuses an out-of-range pre state (QSL ruling,
QSL-634), so that counterexample cannot reproduce. FR-024-AC-35 states how it settles.

The code that follows this specification changes `field_range`, `state_domains`, the harness
identity, the state-clause replay's field reader and its fixture in one change, so the generator
and the replay cannot disagree on one package. It adds three causes to `BoundNotResolvedCause` (a
new variant on a public enum, re-exported from `lib.rs`, so an exhaustive `match` of a consumer
breaks) and three mapping arms (`outcome.rs`, the `StateFrameRefusal` mapping and its AC-66 test
rows), and a fixture whose emitted-package case reads from the model declaration path. It retires
`tc_035_the_generator_reads_no_field_range_from_the_object_shape_qsl_emits`, which measures the
refusal this change removes. Until it lands, `field_range` reads the object body for every object,
and the criteria below are planned.

IR-264 (planned): the composite-equality family. Its shadow is the equality walk of QSpec FR-149's
occurrence-pair plan over a fixed record, tuple and option shape, written as straight-line Rust over
`bool`, `i64`, `Option` and plain structs. It is generated from the item's declaration closure, so
the closure's shape is the only thing it knows about the operands, and it imports nothing from the
evaluator the production oracle calls. The assertion's expectation is computed from the same drawn
values by FR-149's own rules and shares no function with the shadow where one can be written
without; the helper they cannot avoid sharing, the closure reader, is listed in the identity
(FR-028-AC-18). Its outputs are of two kinds. What it returns that the comparison reads (which
members exist, their presence, each leaf's bound) is `value_bearing`: the production oracle is built
through the same reader (FR-018-AC-21), so refinement cannot see a fault in it, and FR-018-AC-23
checks it against an independent read of the package. The order of the members it returns is
`order_only`: FR-149's verdict is a conjunction over fields and its pair count a sum with no early
exit, so a consistent permutation changes neither, and no mutation of the order is a mutant any
check can or need catch.

- When a request names a composite-equality claim that the shadow supports, the generator shall
  emit one harness for it with family `composite_equality`, proof subject `bounded_shadow` and its
  refinement obligation in the same result (FR-015-AC-69).
- The generator shall draw one nondeterministic value per leaf of each parameter operand and
  constrain a literal operand's leaves to exactly its value (FR-015-AC-70).
- The generator shall assert, over every drawn pair, that the shadow's verdict and pair count equal
  the expectation's, and shall end the harness with one cover that the drawn operands compare equal
  (FR-015-AC-71).
- The generator shall emit the shadow and the harness with no loop, no recursion and no heap
  allocation, and shall hold the closure's pair-node count to the crate's shadow size budget
  (FR-015-AC-72).
- If a composite-equality claim reaches a shape the shadow does not support, then the generator shall refuse it as the table above states and emit no harness (FR-015-AC-73).
- The generator shall emit, for the corpus case, a harness whose run the backend classifies
  `Verified` within the ceilings its identity records, with a refinement run classified
  `exhaustive` (FR-015-AC-74).
- The generator shall emit a shadow harness that the backend classifies `Falsified` for each
  seeded mutant of the shadow (FR-015-AC-75).
- The generator shall record in the identity the closure, the operator, the size budget, the
  abstractions, the behaviours the proof does not exercise and, for each, the backing state of the
  criterion that covers it (FR-015-AC-76).

Shapes the composite-equality shadow does not support, and the disposition each takes. Every row
is an interim refusal that names a capability; none is the end state of this family or of the
families it names, each of which is owed a shadow or a production harness of its own:

| Shape reached | Disposition and reason |
|---|---|
| an integer leaf whose type is a plain `integer` with no bound | `requires_bound`, `unbounded_type` naming that node |
| an integer leaf whose bound is not an `integer_range`, or has an endpoint outside `i64` | `unsupported`, `domain_not_representable_in_i64` naming the bound node, as FR-015-AC-17 states for a scalar |
| a declaration that reaches itself (a recursive record or tuple) | `requires_bound`, `unbounded_type` naming the declaration's node: its depth has no declared bound, and none is invented |
| a leaf or member of family text, enum, rational, decimal, quantity, float, sequence, set, bag or ordered set | `unsupported`, `ShadowFamilyNotBuilt` naming the family and the node; the equality oracle of FR-018 still generates for it |
| a `reference` composite, or a model or relation node | `unsupported`, blocked on `agent-ix/quire-spec-language#120`, as FR-018-AC-7 states |
| an operand that is neither a parameter nor a literal, or an operand that is a `convert<T>` | `unsupported`, `OperandNotSupported` naming the operand's node |
| a closure whose pair-node count exceeds the shadow size budget | `unsupported`, `ShadowShapeOverBudget` naming the count and the budget |
| a descriptor `check_equality` refuses, or an item FR-018 refused | the refusal FR-018 recorded, unchanged, and no harness |

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-015-AC-1 | Pre, post, invariant and frame obligations of one contract produce separate harnesses with distinct identities. | Test (TC-025) |
| FR-015-AC-2 | Every harness identity records its model-domain bounds, taken only from IR `bounded_domain` nodes, and its solver and options and unwind bound. | Test (TC-025) |
| FR-015-AC-3 | An unbounded or non-finite obligation is refused with a typed reason and no harness. | Test (TC-025) |
| FR-015-AC-4 | No harness assumption excludes an undefined, refused or incomplete runtime outcome. | Test (TC-025) |
| FR-015-AC-5 | An obligation whose bounds are unsatisfiable is refused with a typed reason and no harness. | Test (TC-025) |
| FR-015-AC-6 | An obligation over an oracle whose operation identity is `caller_declared` is refused with a typed reason and no harness. | Test (TC-025) |
| FR-015-AC-7 | Every generated harness contains exactly one non-vacuity cover; a precondition harness covers that the precondition holds within the IR bounds, and a contract harness's cover stands after the contract call, so a run that satisfies every check without satisfying the cover is not a proof. The cover is the last statement of the harness, after every assertion the harness carries: the state-clause and frame-effect harnesses and the scalar harness place it after their assertions, and a contract harness's checks are inside the contract call its cover follows. A cover that preceded an assertion would be satisfied by the very valuation that falsifies the assertion, and Kani prints one concrete playback per distinct valuation, so it would print the cover's and the run would classify as a failure with no counterexample (IR-451). Tested for the state-clause and frame-effect harnesses (source order) and for the scalar harness (source order); the contract harness is tested for the cover following the contract call; a precondition harness asserts nothing. The V1 bundle and corpus harnesses are under the same rule (FR-015-AC-53), and the guard of FR-015-AC-58 inspects every kind. | Test (TC-025) |
| FR-015-AC-8 | A postcondition or invariant harness emits every package precondition sharing its anchor operation as a `requires` on its generated contract and records each in its identity's embedded oracles; it embeds no other obligation's oracle; and an obligation whose sibling precondition is not a supported item of the same request is refused with a typed reason naming that precondition and no harness. | Test (TC-025) |
| FR-015-AC-9 | Every harness identity records solver `cadical` and the complete ordered option vector — function contracts, concrete playback, the exact fully qualified harness, `--exact`, the explicit unwind, the explicit solver, `--output-format regular`, and `--concrete-playback print` — and no option enabling stubbing is emitted. | Test (TC-025) |
| FR-015-AC-10 | Regeneration from equal inputs is byte-identical, and changing the unwind bound or the customer subject changes the harness identity. | Test (TC-025) |
| FR-015-AC-11 | Every symbolic argument carries an inclusive assumption equal to its IR `bounded_domain` (a literal operand's, to its own value: FR-015-AC-16), and a bounded-integer post-state result is required to lie in the same domain; no generated source carries a `#[kani::unwind]`. | Test (TC-025) |
| FR-015-AC-12 | A request naming no items, more than 256 items, an unparsable subject path, or an unwind bound outside `1..=1024` is refused whole, with no item accounted and no harness exposed. | Test (TC-025) |
| FR-015-AC-13 | An otherwise-supported obligation whose generated harness source exceeds the bounded-resource ceiling is refused with a distinct resource-limit reason naming the generated size, and one that fits the ceiling but fails to parse as Rust is refused with a distinct syntax reason naming the parse error; neither is reported as the internal-invariant render-assembly fallback. | Test (TC-025) |
| FR-015-AC-14 | A scalar-claim item whose graph node carries a tag/form pair this generator does not model as a contract role is refused with a typed reason naming that tag and form, and no harness is emitted; a claim naming a node id absent from the graph entirely is refused rather than accounted as supported with no contract role recorded. | Test (TC-025) |
| FR-015-AC-15 | A scalar-claim item whose claim is a `NoDerivableClaim` refusal yields `unsupported` with the `no_derivable_claim` reason naming the node and the derivation refusal, emits no harness, and does not reject the group. | Test (TC-033) |
| FR-015-AC-16 | A scalar harness constrains a literal operand, inline or a reference to a `value` node whose body is a literal, to exactly its own value: `x + 1` over `x: Int[0, 9]` into `Int[0, 10]` has arguments `[0, 9]` and `[1, 1]`. | Test (TC-033) |
| FR-015-AC-17 | A scalar claim whose literal operand does not fit `i64` (`x + 10^23`) is `unsupported` as `domain_not_representable_in_i64` naming the literal as both endpoints, with no harness. | Test (TC-033) |
| FR-015-AC-18 | A scalar claim whose operation over its operand ranges yields no result inside the result range (`x + 100` over `x: Int[0, 9]` into `Int[0, 10]`, reachable `[100, 109]`) is `unsupported` as `result_bound_unreachable` naming both ranges, with no harness. | Test (TC-033) |
| FR-015-AC-19 | A zero-input transition over one bounded-integer state value verifies for an identity subject and is falsified with a concrete counterexample for a subject that changes the value. | Test (TC-025) |
| FR-015-AC-20 | A harness for a Boolean-connective or bounded-integer comparison claim (`and`, `or`, `not`, `implies`, `eq`, `ne` and the six integer comparisons) embeds its FR-014 oracle byte-identical to the oracle crate's function. | Test (TC-025) |
| FR-015-AC-21 | A precondition that reads a post-state value is refused with a typed reason naming the node, and no harness is emitted. | Test (TC-025) |
| FR-015-AC-22 | A declared proof-dependency census with an empty or duplicate identity, an inconsistent kind, state and path combination, or a non-`Required` kind is refused as a typed invalid input with no harness. | Test (TC-025) |
| FR-015-AC-23 | Every requested item receives exactly one `ObligationDisposition` (`supported`, `requires-bound`, `unsupported` or `invalid-request`), and an item that is not `supported` keeps its source identity and typed reason with no harness and no assumption emitted for it. | Test (TC-025) |
| FR-015-AC-24 | A falsifying in-domain assignment of a plain bounded-integer comparison claim is reported through Kani's concrete playback. | Test (TC-025) |
| FR-015-AC-25 | A valid declared census is folded into the harness identity, readiness is `ready` only when every dependency passed and `incomplete` while any is missing or failed, and proof execution is recorded `not_run`. | Test (TC-025) |
| FR-015-AC-26 | One `postcondition` `state_clause` of an admitted package, whose condition is one integer comparison of the pre and post reads of one field through its first parameter `self`, yields a separate operation-contract harness and frame-effect harness for the operation its anchor names. Each has exactly one non-vacuity cover, and its identity is scoped to the operation, anchor, frame and framed object; the two harnesses of two clauses of one operation are written to different paths. | Test (TC-025) |
| FR-015-AC-27 | The operation-contract harness assumes each state field in the integer range that the framed object's body member of the same name declares, and asserts the clause's comparison between the pre-state snapshot and the state after the subject runs; a clause whose field has no such range is refused. | Test (TC-025) |
| FR-015-AC-28 | The frame-effect harness takes the granted fields from the frame node's `modifies` entries and asserts every other state field the caller names unchanged. | Test (TC-025) |
| FR-015-AC-29 | A state clause that is not a postcondition, whose condition is not one comparison of the pre and post reads of one field through `self` (a literal operand, a negation, two fields, two reads of one side, or a read through another parameter), whose frame creates, deletes or grants a relationship or a foreign field, whose frame grants every state field, whose field the caller's state lacks, or whose clause field declares no integer range, is refused with a typed reason and no harness; a malformed request is refused first. | Test (TC-025) |
| FR-015-AC-30 | With the installed backend, the operation contract verifies for a healthy subject and is falsified, naming the postcondition, for a subject mutated to debit. | Test (TC-025) |
| FR-015-AC-31 | With the installed backend, a frame-allowed effect verifies, a frame-forbidden effect is falsified naming the forbidden field, and regenerating the frame from a package whose `modifies` is mutated to grant nothing falsifies the allowed subject. | Test (TC-025) |
| FR-015-AC-32 | The forbidden frame counterexample, executed natively, reproduces as a frame violation of that field through QSL's `replay_frame`, and the allowed run replays as a respected frame. | Test (TC-025) |
| FR-015-AC-33 | The frame-replay payload's anchor, frame and frame occurrence are the values `qsl_replay::call_site` names for the operation's name. | Test (TC-025) |
| FR-015-AC-34 | The frame-replay envelope's `clause_node` is the payload's frame node and its `occurrence_key` is the payload's frame occurrence. | Test (TC-025) |
| FR-015-AC-35 | An operation the unit names no frame for, whether the domain package declares it or not, is refused by the call site when the frame-replay request is built. | Test (TC-025) |
| FR-015-AC-36 | `FrameReplay::replay` returns QSL's `replay_frame` result: a forbidden write settles a reproduced violation naming the written field, and a write the frame grants settles `inconclusive` with no frame witness. | Test (TC-025) |
| FR-015-AC-37 | A scalar function-application harness asserts the oracle's outcome against the clause's operation evaluated natively in `i128` over the same symbolic operands, independently of the oracle: `Completed` with exactly that value when it lies in the result bound, `Refused` when it does not, and any other outcome fails. With the installed backend, the healthy oracle verifies and an oracle whose arithmetic is mutated is falsified on that assertion. | Test (TC-025) |
| FR-015-AC-38 | A V2 clause claim naming a `state`/`state_clause` node of an admitted `CheckedPackageV2` whose `operation.member.clause` is `precondition`, `postcondition` or `invariant` yields one harness of that obligation kind, separate from the harness of every other clause of the package; the kind is read from the node and no caller value changes it. PLANNED (IR-489). | Test (TC-025) |
| FR-015-AC-39 | A V2 clause harness draws one nondeterministic value per drawn input: the pre-state of each state field of `self` the body reads, whether bare or under `pre(...)` (the subject's input state; how it reaches the subject is FR-025's), and each operation parameter (for a precondition or invariant, each bare state field read, since the state is the one judged). Each carries an inclusive assumption equal to its declared domain: the object member's `integer_range` for a state field, the IR `bounded_domain` of its type for a parameter. A `Reference` parameter such as `self` is not an argument and binds through its object's declared fields; no bound comes from a caller. A postcondition's result and bare post-state reads are not drawn: they are what the subject call produces, their domain is asserted and never assumed (FR-015-AC-4), and a produced value outside its domain fails the harness. An input with no finite domain refuses the claim as FR-015-AC-3 states. PLANNED (IR-489). | Test (TC-025) |
| FR-015-AC-40 | A harness for a V2 clause whose body is Boolean connectives and bounded-integer comparisons (`and`, `or`, `not`, `implies`, `eq`, `ne` and the six integer comparisons) embeds an FR-014 oracle of that body byte-identical to the oracle crate's function, as FR-015-AC-20 states for a scalar claim; the oracle of an inline clause term needs FR-014-AC-38. PLANNED (IR-489). | Test (TC-025) |
| FR-015-AC-41 | A V2 clause whose body holds integer arithmetic (add, subtract, multiply, negate) as an operand of a comparison lowers, and its harness keeps the independent native assertion of FR-015-AC-37 for each arithmetic subterm (the oracle's outcome against the operation evaluated natively in `i128` over the same operands, `Refused` assumed away nowhere): a postcondition `amount < 1000 implies amount + 1 <= 1000` over `amount: Int[0, 1000]` yields one contract harness. With the installed backend the unmutated clause verifies with its cover satisfied, and the same clause with `+` mutated to return `left + right + 1` is falsified with the concrete playback `amount_current = 999`. The real-Kani control is verified by the quire-integration exemplar of AD-004 L-5 when QSL's facade offers no way to build the package from CG. PLANNED (IR-489). | Test (TC-025) |
| FR-015-AC-42 | A V2 clause body holding an operator the V2 arm does not support (division, modulo, absolute value, and every operator outside connectives, the six integer comparisons and add, subtract, multiply, negate; those operators are planned scope for a later criterion) is `unsupported`, with a typed reason naming the node and the operator and no harness; a node that is not a modelled clause role, or a node id absent from the package, is refused as FR-015-AC-14 states; no item is skipped silently, and a supported item of the same request is unaffected. PLANNED (IR-489). | Test (TC-025) |
| FR-015-AC-43 | A V2 postcondition harness emits every `precondition` clause node sharing its anchor node as a `requires` and records each in its identity's embedded oracles, embeds no other obligation's oracle, and a postcondition whose sibling precondition is not a supported item of the same request is refused naming that precondition with no harness; a V2 invariant is anchored at an object type, binds `self` alone, and so has no operation precondition to assume. PLANNED (IR-489). | Test (TC-025) |
| FR-015-AC-44 | A census declared on a V2 clause claim with an empty or duplicate dependency identity, an inconsistent kind, state and path combination, or a non-`Required` kind is refused as a typed invalid input with no harness, by the same rules as FR-015-AC-22; the census request and readiness types it uses are not `ProofDependencyGraph`. PLANNED (IR-489). | Test (TC-025) |
| FR-015-AC-45 | A valid census declared on a V2 clause claim is folded into that harness's identity record, readiness is `ready` only when every dependency passed and `incomplete` while any is missing or failed, and proof execution is recorded `not_run`, by the same rules as FR-015-AC-25; in addition, the folded census is independent of the order the caller lists its dependencies. PLANNED (IR-489). | Test (TC-025) |
| FR-015-AC-46 | Every V2 clause harness contains exactly one non-vacuity cover, placed after every assumption, after the subject call and after every assertion the harness carries (the last statement, as FR-015-AC-7 states), that states the family's reachability, as FR-015-AC-7 does: a precondition harness covers that the precondition holds within the bounds; a postcondition harness covers, after the subject call returns, that the requires and the bounds are jointly satisfiable; an invariant harness covers that the drawn state satisfies its domain assumptions. The subject call is the call of the customer subject at the request's subject path in a postcondition harness, which supplies the result and post-state; a precondition harness and an invariant harness have none, and their cover follows the last assumption. A V2 clause whose assumptions and bounds are jointly unsatisfiable is refused with a typed reason and no harness, or, if it reaches a backend run, does not classify `Verified`. PLANNED (IR-489). | Test (TC-025) |
| FR-015-AC-47 | Regeneration of a V2 clause harness from equal inputs is byte-identical, and changing the unwind bound or the customer subject changes the harness identity record (as FR-015-AC-10 states). PLANNED (IR-489). | Test (TC-025) |
| FR-015-AC-48 | The obligation identity of a V2 clause claim is formed by AD-003 E-1 from the clause node id, its occurrence key, the obligation kind, and the arguments each as parameter node id and declared domain for an operation parameter, as E-1 states (the domain FR-015-AC-39 defines). A state field read is named by its declaring node id and field name, never by a node of its own; that E-1 lists parameters only is an open question for the AD-003 owner and E-1 is not widened here. The source span is excluded; changing any included member changes it, and changing the span, the unwind bound or the subject does not. It is a different value from the harness identity record of FR-015-AC-47. PLANNED (IR-489). | Test (TC-025) |
| FR-015-AC-49 | A V2 invariant claim yields a harness that draws its state and parameters within their declared domains (FR-015-AC-39) and asserts the invariant clause; its subject is the clause itself, with no subject call. Preservation of the invariant under an operation is not specified by this criterion. PLANNED (IR-489). | Test (TC-025) |
| FR-015-AC-50 | `negotiate` reports an obligation whose scalar refusal is `ExactScalarRefusal::LoweringByteLimitExceeded` or `LoweringLimitUnrecognised` as `Outcome::Unsupported(UnsupportedObligation::OracleRefused { refusal })` with `refusal` equal to the scalar refusal, field for field, and neither is mapped to another `UnsupportedObligation` variant nor to `LoweringWorkExhausted`. | Test (TC-025) |
| FR-015-AC-51 | A finite input validated under a profile selection other than the offered profile's is refused as a typed `InvalidInput` `kani_profile_input_mismatch` result naming the request's source id, with no artifact and no case identity recorded. | Test (TC-023) |
| FR-015-AC-52 | The context of every outcome and refusal the corpus generator returns (proved, counterexample, `kani_corpus_dependency_invalid`, `kani_corpus_identity_collision`, a lowering refusal and `kani_profile_input_mismatch`) is the profile selection's revision. | Test (TC-023) |
| FR-015-AC-53 | Each of the seven harness kinds the generator emits (precondition, V1 contract, scalar, state-clause, frame-effect, V1 bundle and corpus) is subject to the cover-last rule of FR-015-AC-7: exactly one `kani::cover!`, the last statement of the body. What the cover witnesses, by kind: a precondition harness, that the precondition holds within the IR bounds (it asserts nothing); a V1 contract harness (postcondition or invariant), that the requires clause and the IR bounds are jointly satisfiable, after the contract call whose `ensures` is the harness's check; a scalar harness, that the oracle's `Completed` branch is reached; a state-clause harness and a frame-effect harness, that the bounded state is reached and the subject returns; a V1 bundle harness, as FR-015-AC-54 states; a corpus harness, as FR-015-AC-55 states. | Test (TC-025) |
| FR-015-AC-54 | A V1 bundle harness (`generate_kani_bundle`, the `proof_for_contract` harness it emits for a requires clause, an ensures clause and the bundle's bounded arguments) ends with exactly one `kani::cover!` after the call of its contract, which witnesses that the requires clause and the argument and result bounds are jointly satisfiable, so a run whose requires clause no bounded argument satisfies never reaches it; the bundle's `ensures` is checked at the contract call, so the cover follows every check the harness carries. | Test (TC-025) |
| FR-015-AC-55 | A bounded-corpus harness, whichever of the arithmetic, graph and collection families the case belongs to, ends with exactly one `kani::cover!` after its `assert!` of the case's oracle, which witnesses that the harness runs to its end past that assertion. | Test (TC-023) |
| FR-015-AC-56 | With the installed backend, a V1 bundle harness whose requires clause some bounded argument satisfies and whose `ensures` holds for every such argument classifies `Verified` (FR-017-AC-4); the same bundle with a requires clause no bounded argument satisfies classifies `CoverUnsatisfied` with its satisfied and total cover counts, never `Verified` and never `Falsified`. | Test (TC-025) |
| FR-015-AC-57 | With the installed backend, a bounded-corpus harness of each of the arithmetic, graph and collection families whose oracle is true classifies `Verified`, and a graph or collection harness whose oracle is false (an arithmetic case's oracle is always true) classifies `Falsified` carrying the assertion's playback (empty-valued, since a corpus case draws no input), never `Inconclusive` for lack of a counterexample. | Test (TC-023) |
| FR-015-AC-58 | A test generates a harness through every emitting entry point (the precondition, V1 contract, scalar, state-clause and frame-effect families, `generate_kani_bundle`, and the corpus generator for each of its three families), parses each emitted source, and fails for any function attributed `#[kani::proof]` or `#[kani::proof_for_contract]` whose body does not contain exactly one `kani::cover!`, which must be the last statement, with no assertion after it and no other cover; and a scan of the non-test string literals of `src/` fails when a file other than the ones the test drives spells a proof attribute, or a driven file spells more proof attributes than the test counts; a spelling is `#[kani::` other than `requires`, `ensures`, `stub` and `unwind`, `kani::proof` or `proof_for_contract` in one literal, with `\` line continuations joined, which includes a `format!` template and a split `concat!` whose first fragment holds `#[kani::`, and the scan does not see a spelling assembled from fragments none of which holds `#[kani::` or `kani::proof`. | Test (TC-025) |
| FR-015-AC-59 | A request of `StateFrame` items in which the first item is refused and later items are supported, requires-bound, unsupported or invalid yields exactly one `ObligationRecord` per item in request order, `kind` `postcondition` for a `contract` item and `frame` for a `frame` item; and each later item's record (disposition and reason) equals the record the same item has in a request holding that item only; a request of three refused items yields three records. | Test (TC-025) |
| FR-015-AC-60 | A `StateFrame` item that yields is recorded `supported` carrying its harness symbol, and its harness, returned in the `state_frame_harnesses` field of `KaniObligationOutcome::Emitted` in request order, is byte-identical to the harness `generate_state_frame_role` returns for the same clause and role, and for a clause both of whose roles succeed to the corresponding harness `generate_state_frame_obligations` returns. PLANNED (IR-461): the comparison against `generate_state_frame_role` is not testable from `tests/it` (crate-private) and no V2 package builder exists under `src/`; byte identity holds by construction; the test compares against the public entry for a clause whose roles both succeed, and carries no trace tag of this criterion. | Test (TC-025) |
| FR-015-AC-61 | A `StateFrame` item whose clause field's member references an unbounded type (a plain integer type, no bound) is recorded `requires_bound` with that member's `value.target` node as `unbounded_type`, and one whose lowering is a requires-bound record is recorded `requires_bound` with that record's `unbounded_type`, each with no harness. | Test (TC-025) |
| FR-015-AC-62 | A `StateFrame` item whose lowering is an unsupported-family record is recorded `unsupported` with reason `NoFiniteEncoding` naming the record's node and family, and an item whose node is not a `state_clause` is recorded `unsupported` with `UnknownNodeKind`, each with no harness. | Test (TC-025) |
| FR-015-AC-63 | A `StateFrame` item whose condition is a negation, compares to a literal, uses an operator other than the six integer comparisons, reads through a parameter other than `self`, compares two fields or compares two reads of one side, an item whose frame creates or deletes an object or grants a relationship or a foreign field (naming the effect), an item whose clause field's member references a bound that is not an `integer_range` or one with an endpoint outside `i64`, one whose member is absent from the object, one whose member's value is not a reference, and an item whose clause is not a postcondition, is malformed, whose frame grants every field, or whose lowering is an invalid-body, incomplete or over-budget record, is recorded `unsupported` with reason `StateFrameRefused` carrying that refusal and never `NoFiniteEncoding`, with no harness. Covered through a negotiated request except for a member absent from the object and the invalid-body, incomplete and over-budget lowering records, which no fixture reaches and which are covered only through the mapping test of FR-015-AC-66. | Test (TC-025) |
| FR-015-AC-64 | A `StateFrame` item with an unparsable state path or an unparsable item subject path, an invalid or lacking state field, an absent node, a repeat of an earlier item or a package other than the first item's is recorded `invalid_request` (`InvalidStatePath`, `InvalidStateField`, `UnknownNode`, `DuplicateItem`, `MixedStatePackages`), and the outcome is `Rejected` with every item's record and no harness bytes. | Test (TC-025) |
| FR-015-AC-65 | A request holding `StateFrame` items and no items, more than `MAX_OBLIGATION_ITEMS`, an unparsable request subject path or an unwind bound outside `1..=MAX_OBLIGATION_UNWIND` is refused whole with the existing `KaniObligationError` variant and no record. | Test (TC-025) |
| FR-015-AC-66 | A test builds every `StateFrameRefusal` variant (each `BoundNotResolved` case the table separates, and each of the six refusal arms of `NotLowered`) and calls the mapping on it, and each yields the disposition and reason of the table; the variants a request reaches only with a defective generator or a source over 1 MiB (`UnwindOutOfRange`, `InvalidGeneratedSyntax`, `RecordSerialization`) are built directly, and `ResourceLimitExceeded` is built directly or reached with a state-field list large enough to pass the ceiling; an inspection confirms the mapping is a `match` with no wildcard arm. | Test (TC-025) |
| FR-015-AC-67 | `generate_state_frame_obligations` refuses with `MalformedClause` at the node, not `InvalidField` or `ConditionNotSupported`, for a clause whose graph field name is not a Rust identifier; a field name the caller supplies that is not an identifier is still `InvalidField`. | Test (TC-025) |
| FR-015-AC-68 | A `StateFrame` request whose frame grants every state field records its `frame` item `unsupported` (`StateFrameRefused`) and its `contract` item `supported` with the harness `generate_state_frame_role` returns for the `contract` role; a request whose condition is a negation records its `contract` item `unsupported` and its `frame` item `supported` with the harness `generate_state_frame_role` returns for the `frame` role; in both, `generate_state_frame_obligations` returns the refusal of the failing role; a request whose lowering is refused records both items alike. PLANNED (IR-461): the comparison against `generate_state_frame_role` is not testable from `tests/it` (crate-private) and no V2 package builder exists under `src/`; byte identity holds by construction; the test checks the independence, the records and the engine's refusal, and carries no trace tag of this criterion. | Test (TC-025) |
| FR-015-AC-69 | A composite-equality claim over operands that are parameters or literals of one record, tuple or option type whose leaves are Boolean or bounded integers, with the descriptor FR-018 generated, yields one harness whose identity records family `composite_equality`, proof subject `bounded_shadow` and a refinement obligation in the same result (FR-028-AC-7), separate from the harness of every other item; both operators (`Equal`, `NotEqual`) yield one; and the operator and operand types are read from the descriptor while no bound is. PLANNED (IR-264). | Test (TC-025) |
| FR-015-AC-70 | The harness draws one nondeterministic value per leaf of each parameter operand and no other value: a `bool` per Boolean leaf, an `i64` per bounded-integer leaf assumed inside its `integer_range` (FR-015-AC-11), a presence `bool` per option and a payload only for a present option, and for a record field of optional presence one of `present`, `absent` or `null`; a literal operand's leaves are constrained to exactly its value (FR-015-AC-16); the drawn arguments ascend by parameter node id then by leaf path, each bound to its node id and path (FR-025-AC-1, FR-025-AC-2), and no composite is itself an argument (FR-025-AC-7). PLANNED (IR-264). | Test (TC-025) |
| FR-015-AC-71 | Over every drawn pair the harness asserts that the shadow's verdict equals the expectation's and that the shadow's pair count equals the expectation's, where the expectation is computed from the drawn values by QSpec FR-149's rules (same declaration and pairwise-equal fields or positions in order; two `none` equal; `none` and present unequal; `absent` equals `absent`, `null` equals `null`, `absent` and `null` unequal; the pair count is the node count of the occurrence-pair tree with no early exit after an unequal pair), the verdict of `NotEqual` is the negation of `Equal`'s with the same count, and the harness ends with exactly one `kani::cover!` that the drawn operands compare equal, after every assertion (FR-015-AC-7). PLANNED (IR-264). | Test (TC-025) |
| FR-015-AC-72 | The emitted shadow and harness, parsed, contain no loop (`while`, `loop`, `for`), no recursive call, no `Box`, `Vec`, `Rc`, `String` or map, and no integer type wider than `i64`; and a closure whose pair-node count (the node count of the largest occurrence-pair tree its type admits) exceeds `SHADOW_PAIR_NODE_BUDGET` is refused as `ShadowShapeOverBudget` naming the count and the budget, one at the budget generates, and the budget and the count are recorded in the identity and the evidence. The value of the budget is set by the measurement of the code change and recorded in TC-025's status. PLANNED (IR-264). | Test (TC-025) |
| FR-015-AC-73 | Each row of the unsupported-shape table above is produced by an item reaching that shape and yields its disposition and reason with no harness: an unbounded integer leaf `requires_bound`; a bound outside `i64` `domain_not_representable_in_i64`; a recursive declaration `requires_bound` naming the declaration; one leaf of each family the table lists `ShadowFamilyNotBuilt` naming that family; a `reference` composite, as FR-018-AC-7; an operand that is neither a parameter nor a literal, and a `convert<T>` operand, `OperandNotSupported`; an over-budget closure `ShadowShapeOverBudget`; an item FR-018 refused carries FR-018's refusal; and a supported item in the same request settles as it does alone. PLANNED (IR-264). | Test (TC-025) |
| FR-015-AC-74 | With the installed backend, the harness of the corpus case (two record-typed and two tuple-typed values compared with `=` and `!=`, a record with an optional field and a nested option, Boolean and bounded-integer leaves) is classified `Verified` with its cover satisfied within the memory and wall-clock ceilings its identity records, and its execution evidence records the versions and options of FR-028-AC-23; the refinement run of the same case agrees on every case it runs, and the case's refinement domain is at most the case cap, so the run is classified `exhaustive` and records its case count; a second case whose domain exceeds the cap (two unconstrained `i64`-range leaves) is classified `sampled`, which shows both classes. PLANNED (IR-264). | Test (TC-025) |
| FR-015-AC-75 | With the installed backend each seeded mutant of the shadow is `Falsified` with a concrete playback: an `absent` slot read equal to a `null` slot; one field of the left operand compared with the next field of the right; the pair count omitting the payload pair of two present options; the count stopping after the first unequal field; the operator `NotEqual` rendered as `Equal`; and a field left out of the comparison. Run natively, each seeded mutant of the production oracle settles `refinement_failed` naming the first disagreeing case: the opposite operator emitted; an `absent` slot compared equal to a `null` slot; one pair charged too many for two present options; an optional field's declaration emitted as required (caught as the construction refusal `MissingField` or `NullForRequiredField` of the oracle's own environment, FR-028-AC-14, since the verdict and count do not change); and an oracle that completes a legal pair as `Refused`. A seeded mutant of the closure reader that the shadow, its expectation and the production oracle share is caught by FR-018-AC-23's independent read, not by refinement: one that drops a member, narrows an integer bound by one, or reads an optional member as required. A mutation of the order of the members the reader returns is not a mutant of this set (the `order_only` exemption of FR-028-AC-18). The unmutated corpus case verifies and its refinement run agrees. PLANNED (IR-264). | Test (TC-025) |
| FR-015-AC-76 | The identity of a composite-equality harness records family `composite_equality`, proof subject `bounded_shadow`, the closure's declaration keys each with its leaves' declared domains, the operator, the size budget and the closure's pair-node count, the abstractions `representation`, `integer_widening`, `absent_payload` and `unrolled_walk` each with its discharging evidence, and the behaviours the proof does not exercise (`meter_limits`, `ill_typed_operands`, `charge_amounts_other_than_pairs`, `foreign_reference`, `declaration_reconstruction`) each with the native criterion that covers it (FR-018-AC-9, FR-018-AC-8, FR-018-AC-2, FR-018-AC-7, FR-018-AC-23) and that criterion's backing state, `backed` where the test matrix lists it covered and `unbacked` otherwise, which a test asserts against the matrix rows (today FR-018-AC-7, AC-8 and AC-9 are planned and FR-018-AC-2 is partly covered, so each of those four reads `unbacked`); a result whose covering criterion is `unbacked` is reported with that state and is not offered as closing a family row; changing the closure, a bound, the operator or the budget changes the identity; and regeneration from equal inputs is byte-identical. PLANNED (IR-264). | Test (TC-025) |
| FR-015-AC-77 | PLANNED (IR-624), GATED on IR-627 or IR-628. Over the package QSL emits for the twin's unit (`Twin::emitted_package`: a model declaration node, empty body), `field_range` returns 0 to 1000 for `balance` and for `audit` from the reads' `result_type`; over the hand-built fixture package, whose object is not a model declaration node, it returns the same range for each field from the object's body member, trusting no read's type; the harness generator's `domains` and the state-clause replay's declared ranges hold the same range for each field of one package. | Test (TC-025) |
| FR-015-AC-78 | PLANNED (IR-624), GATED on IR-627 or IR-628. For a field of the framed object, `field_range` returns `FieldNotRead` for a model declaration node no read of the field names, `ConflictingFieldReads` naming both nodes for one whose reads name two types, `MemberDisagreesWithRead` naming both nodes for any other object whose member and read name two types (the fixture with `Member::RationalBound`, `Member::WideRange` or `Member::Literal` and reads typed `Int[0, 1000]`), and keeps `MemberAbsent`, `ValueNotReference`, `UnboundedType`, `NotIntegerRange` and `EndpointOutsideI64` unchanged; each cause of the clause's own field refuses the item, and each of the three new causes maps to `unsupported`, `StateFrameRefused` (FR-015-AC-66). For the package QSL emits from a unit whose clauses read only `balance`, a harness is generated whose `state_fields` lists `audit`, whose `domains` has no entry for it and whose draw of it is unconstrained. | Test (TC-025) |
| FR-015-AC-79 | PLANNED (IR-624), GATED on IR-627 or IR-628. From the package QSL emits for the twin's unit, both roles of each clause (`BalanceNeverDrops`, `AuditNeverDrops`) are generated; each identity's `state_fields` equals the request's list in the request's order, its `domains` are the ranges of FR-015-AC-77, and its `scope.anchor` and `scope.frame` equal the ids `qsl_replay::call_site` names for the operation, asserted on the generated identity before any replay; a request whose list omits the clause's field or a granted field is refused as FR-015-AC-29 states. | Test (TC-025) |
| FR-015-AC-80 | PLANNED (IR-624), GATED on IR-627 or IR-628. With the installed backend, the cases of FR-015-AC-30 and FR-015-AC-31 run over harnesses generated from the package QSL emits: the healthy subject verifies, the subject mutated to debit is falsified naming the postcondition, a granted write verifies and a write to an ungranted field is falsified naming that field. | Test (TC-025) |
| FR-015-AC-81 | PLANNED (IR-624), GATED on IR-627 or IR-628. A harness whose state field has no range records that field and its reason in `StateFrameIdentity` and its persisted record: `NoRead` for a model declaration node's field no read names, and `TypeNotRange` for a field whose type is known and is no `i64` `integer_range`; the two reasons give different records, a regenerated harness from equal inputs has a byte-identical record, and a record naming a field twice is not read as an identity. | Test (TC-025) |

### Mutations FR-015-AC-69 to FR-015-AC-76 detect

| ID | Mutation that breaks it |
|----|-------------------------|
| FR-015-AC-69 | Emit only the `Equal` harness for a `NotEqual` item, emit a `production` harness for the family, or read a bound from the descriptor. |
| FR-015-AC-70 | Draw a payload for an absent option, collapse `absent` and `null` into one draw, take a bound from the whole `i64`, or leave a literal operand's leaf unpinned. |
| FR-015-AC-71 | Compute the expectation by calling the shadow, so the assertion is `x == x`; stop counting after the first unequal field; or place the cover before the assertions. |
| FR-015-AC-72 | Emit a loop or a `Vec` in the shadow, or raise `SHADOW_PAIR_NODE_BUDGET` to fit a closure instead of refusing it. |
| FR-015-AC-73 | Refuse every shape as one reason, treat an unbounded integer as the whole `i64`, a recursive declaration as depth one, or a `convert<T>` operand as its source type, or let a refused item drop a supported sibling. |
| FR-015-AC-74 | Pass a corpus case whose cover is unsatisfiable, whose refinement ran over a narrower domain than the shadow drew, or whose refinement was `sampled` where its domain fits the cap. |
| FR-015-AC-75 | Mutate only the production oracle or only the shadow; or accept an equivalent mutant as the test of a check (a permutation of member order, or swapping two descriptors of one type). |
| FR-015-AC-76 | Record `backed` for a covering criterion the matrix lists planned, omit `declaration_reconstruction`, or leave the identity unchanged when a bound changes. |

### Mutations FR-015-AC-77 to FR-015-AC-81 detect

| ID | Mutation that breaks it |
|----|-------------------------|
| FR-015-AC-77 | Read the range from the object body's members for a model declaration node, which the emitted package leaves empty; trust a read's `result_type` for an object with body members (the fixture's, caught by AC-78's `MemberDisagreesWithRead`); or let the generator and the state-clause replay carry two readers that disagree on one package. |
| FR-015-AC-78 | Pick the first of two disagreeing reads, let a read override a body member, treat a field no read names as the whole `i64` or as an error for a field that is not the clause's, return the `result_type` of another field's read, or collapse the causes into one. |
| FR-015-AC-79 | Emit the hand-built fixture's anchor or frame id in the identity, reorder `state_fields` by name or by body position, or give an unread field a range of the whole `i64` in `domains`. |
| FR-015-AC-80 | Generate the harness from the hand-built fixture and not from the emitted package, so the test never reads the emitted shape. |
| FR-015-AC-81 | Record no reason, record `NoRead` for a field whose type is known and not a range, or record both reasons alike. |

## Dependencies

- **Upstream**: [FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md),
  [FR-025](./FR-025-generated-subject-abi.md), [FR-028](./FR-028-bounded-proof-ceilings.md).
- **Downstream**: [TC-025](../matrix/TC-025-bounded-kani-obligations.md),
  [FR-016](../../replay/functional/FR-016-witness-native-replay.md),
  [FR-022](../../routed/functional/FR-022-routed-generation.md), whose Kani generation arm calls this generator.
