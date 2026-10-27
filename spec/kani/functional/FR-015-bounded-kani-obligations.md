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

For production requests after the FR-330 consumer transition,
[FR-039](../../core/functional/FR-039-contract-package-consumption.md) requires
this generator to read represented meaning and typed per-item accounting from
the one admitted ContractPackage. References below to `CheckedPackageV2` name
the current direct input and upstream native authority; they do not permit
production to decide the same item again from that original graph.

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

Current interim ownership (IR-329): the bounded-profile classifier, the two
retained finite-input semantic-family lowerers (collection and graph) and the
bounded-corpus emitter are auxiliary operations of this Kani generator, governed
today by FR-015-AC-82 to FR-015-AC-94. Implemented (IR-615): the owning IR
profile reader rejects the legacy `Inconclusive` disposition, and no family
admission arm turns a profile code into an inconclusive run. The census preserves
admitted Supported/Refused entries in request order. Profile selection cannot
establish the run-only vacuity classification governed by
[Contract IR FR-030](ix://agent-ix/quire-contract-ir/FR-030). This cleanup does
not complete the `unsupported` matrix entry, capability-kind warning and
no-artifact negotiation behavior required by
[Contract IR FR-029](ix://agent-ix/quire-contract-ir/FR-029), AC-2;
that integration remains planned. The arithmetic retirement slice
FR-015-AC-109 is implemented; the remaining step 4g retirement slices
FR-015-AC-110 (collection) and FR-015-AC-111 (graph and the whole corpus API)
remain planned, with FR-015-AC-95 as their end state.
They consume Contract IR's validated finite-input/profile interfaces, not a V2
clause claim. The corpus emits oracle, strategy, Kani harness and proof graph
from one admitted case; it does not retain a replay packet or claim a native
replay result. ADR-001 Q1 and Q3 remain the target: FR-015 is the one Kani
generation requirement and `CheckedPackageV2` is its one input model. AD-004
steps 4c through 4g retire the V1 bundle, V1 clause arm and hand-built corpus
path only after their V2 replacements serve the required families. Step 4g is
staged per family with no compatibility wrapper (IR-598): each slice deletes
its family's arm, lowerer and tests once V2 serves the family, and every slice
is gated on IR-858 (step 4b). Until the final slice deletes the hand-built
corpus path, its `BoundedCorpusExpectation` predicts whether the generated
finite case's one assertion passes (one expected SUCCESS check) or fails (zero
expected SUCCESS checks). It is not a `KaniOutcome` and cannot enter the
terminal proof mapper. The final slice removes that generation-time
expectation and the path's typed refusal `KaniOutcome`; FR-017 supplies the
verdict from a run. Corpus playback and
replay are dropped with the final slice (FR-015-AC-111), not re-homed: their only
caller replays graph and collection cases, and the real-Kani replays of
FR-015-AC-99 and FR-015-AC-106 replace them. V2 does not serve divide or
remainder until IR-854, and the arithmetic slice records that coverage loss.
FR-015-AC-96 to FR-015-AC-102 retain the collection implementation and partial
statuses recorded below; graph clauses remain PLANNED. This retirement plan
adds no proof credit or current backend results.
FR-015-AC-109 to FR-015-AC-111 state the slices and FR-015-AC-95 their end
state. QSL-353 establishes that
QSL-emitted packages read through Contract IR; it does not supply the symbolic
corpus or the graph population input. The graph slice also waits for IR-805: CG integration of the merged IR-851/IR-852
accessors and QSL-820 real-package population emission (after QSL-819). Retirement of each family waits for its V2
path, among arithmetic, collection and graph, to serve the corpus through the one
generator, with the symbolic domains of AC-96 to AC-102 and AD-004's shared
renderer. The other criteria below own live interim behavior and do not mark
that retirement implemented.

Partly implemented (IR-268, collection half; CG #396 fe20ecc): FR-015-AC-97 is
implemented and -102 is implemented for the collection family. FR-015-AC-96, -99,
-100 and -101 are partial: -96 (uniqueness clause), -99 (symbolic-draw,
cover-after-assertions and no-`Vec<i128>` clauses) and -101 (representational-ceiling
and malformed-declaration clauses) have no test carrying that AC's tag for the clause,
and -100 lacks the FR-017 run-evidence version clause. Graph clauses stay PLANNED
under IR-805. They specify bounded collection claims in the V2 Kani item path. A claim
is an admitted QSL V2 clause/function body whose collection operation reads a
selected bounded model collection field. QSL expresses collection properties
as ordinary clauses or function bodies using `forall` or `exists`; the emitted
V2 body carries the claim and field path, rather than a separate collection
claim kind. Symbolic lengths and elements come from the selected field's typed
inclusive cardinality and element domain admitted by Contract IR. The independent
oracle replays the same authored function with `qsl_replay::replay`. These are
not the concrete `CollectionQuery` or corpus lowerings of AC-86 to AC-95, and
caller `max_items` cannot stand in for the model domain. The test fixture uses
QSL emission and Contract IR admission; it does not construct a second package
wire format.

Planned (IR-805, graph half of IR-268): FR-015-AC-98, -105, -106 and -107, with
the graph clauses of -99 to -102, specify bounded reference-graph reachability
claims in the same V2 item path. QSL-804 is Done: QSL (`quire-spec-language`) lowers a checked `reaches` to a V2 expression node, and
Contract IR (`quire-contract-ir`, IR #422) admits it, including
a finite multi-valued destination end. IR-268 earns no graph implementation or
proof credit. The finite-input `GraphRequest` and corpus are not evidence for
symbolic graph proofs, and caller `max_expansions` is not a domain input.

The claim is one reachability application inside an admitted `state` /
`state_clause` node. The producer-owned relationship-end form is defined by
[QSL FR-365](ix://agent-ix/quire-spec-language/FR-365), and the field-edge
emission by [QSL FR-105](ix://agent-ix/quire-spec-language/FR-105).
CG consumes those admitted forms rather than defining their wire schema.
A claim is identified by the pair of the `state_clause` node id and the path
of argument positions from the clause's condition term to the reachability
node, as defined by FR-365's "What a clause claim carries for a proof",
never by the node id alone, because one node can sit under several clauses
and on several paths. Population bounds and observations are supplied by the
authorities below, never inferred from a reachability node's operands.

Typed Contract IR inputs: reachability is homogeneous. Both operands conform to
the receiver endpoint's type `E` (IR FR-038-AC-252), so one object type and one
population cover the start and the target (a subtype shares that population's
domain key). CG reads field-edge multiplicity through `model_object_fields`
and its typed member domain; for a relationship-end claim CG consumes the typed
`relationship_end` accessor defined by [Contract IR FR-038](ix://agent-ix/quire-contract-ir/FR-038),
AC-262 through AC-268. That upstream requirement owns the accessor record and
errors; CG uses its selected destination end's finite multiplicity without
redefining the record. IR-851's accessor code is merged (IR #438).
The claim-to-population accessor `reaches_population` is also merged (IR-852,
IR #442; FR-038-AC-269 through AC-280). CG dependency integration and graph
harness generation remain outstanding, and QSL-820 still owns real-package
population-node emission and the claim's population link.
The finite population of `E` is the inclusive `[0, N]` that `select_population`
([Contract IR FR-053](ix://agent-ix/quire-contract-ir/FR-053)) reads from the
claim's admitted package. A bound chosen by the caller, like `max_expansions`,
is forbidden, so the item does not carry a caller-named binding or maximum.

Disposition sequence for a graph claim:
1. If the admitted claim has no available population selection, its selection
   is `Unavailable` and it settles `unsupported` naming that input. The merged
   IR accessor does not make a missing package population link available.
2. Given an available selection, a claim whose population has no finite
   authored bound has an `Unbounded` selection and settles `requires_bound`.
3. A claim whose population has a finite authored bound has a `Finite`
   selection and runs the `Verified` and `Falsified` path below when the
   selected edge's finite multiplicity is available. A relationship-end claim
   for which the typed end cannot be consumed settles `unsupported`.
These are conditional CG obligations, not claims that every current package
has the same selection. Real-package graph proofs remain Planned pending CG
integration and QSL-820 emission; neither the landed IR accessors nor unit
fixtures earn end-to-end proof credit.
The V1-draft wording of IR-268 is interpreted through the current admitted
`quire.checked-package/v2` boundary.

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
  assumed. The clause body's third argument is its Boolean condition: it may be an inline
  term or a reference to an admitted Boolean expression node, whose expression supplies
  the condition without replacing the clause node as the claim identity. The condition may
  hold Boolean connectives, bounded-integer comparisons and the integer
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
- A symbolic collection item (IR-268): one admitted QSL V2 clause/function
  body claim and its selected model declaration in a `CheckedPackageV2`, plus
  the customer subject and request unwind/resource ceilings. The body contains
  a QSL collection `forall` or `exists` operation over the selected field; the
  item is the admitted body claim, not a separate query kind. Its symbolic
  cardinality and element domain come from that field's typed
  `CheckedMemberType` and reachable V2 bounds. The native oracle replays the
  same authored function through `qsl_replay::replay`. A symbolic graph item
  (IR-805) is one admitted `state_clause` node id, the argument path to one
  reachability application (`quire.op.model.reaches` or
  `quire.op.model.reaches_field`), the customer subject and request ceilings;
  its population is read from the admitted package, never named or sized by
  the caller; the selected edge's multiplicity comes from
  typed `CheckedMemberType` (`Reference`, `Option` or bounded `Collection`) or,
  for a relationship end, from IR's typed end multiplicity. Concrete values and caller
  `max_items` or `max_expansions` are not domain inputs.

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
- Where FR-014 establishes an inferred integer operand interval for a
  referenced application, the generator shall take that conservative interval for
  the corresponding symbolic argument and record its source node and endpoints
  in the harness identity. It shall apply the same `i64` representability
  check as it applies to an IR operand bound or literal; it shall not substitute
  the outer result bound, a consuming narrowing conversion's bound, or the
  whole `i64` range for the inferred interval. An FR-014 inference refusal
  remains a refused claim with no harness. `RequiresBound` maps to routed
  `requires_bound` with its `unbounded_type`; `OperandRangeLimitExceeded`,
  `OperandUnsupported`, `OperandTypeMismatch` and `InvalidBody` map to routed
  `unsupported` with `oracle_refused` carrying the exact FR-014 refusal.
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
- When a V2 Boolean implication claim is generated, the code generator shall
  assert the result of a compiled `operators::implies_short_circuit` subject
  call and the FR-014 oracle's result separately against the independent native
  implication over the same symbolic Boolean operands (FR-015-AC-118).
- When the installed-Kani mutation control runs for that claim, the test shall
  change only the generated subject call (FR-015-AC-118).
- When the installed-Kani control runs for a QSL-authored V2 implication
  postcondition, the test shall keep the admitted clause, its FR-014 oracle,
  generated harness and non-helper subject source unchanged between runs and change
  only the parsed body of the authoritative runtime implication helper in an
  isolated checkout (FR-015-AC-119).
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
  clause term depends on the planned FR-014-AC-38; a QSL-checked literal
  `numeric.narrow` operand uses FR-014-AC-54.
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
- Until AD-004 step 4g retires the hand-built bounded Kani corpus path, when
  that path is given a finite input validated under a different profile
  selection than the profile offered with it, it shall refuse the case with a
  typed `InvalidInput` result, record no case identity and emit no artifact
  (FR-015-AC-51).
- Until AD-004 step 4g retires the hand-built bounded Kani corpus path, that
  generator shall retain the revision of the profile selection as the context
  of every successful expectation and typed refusal it returns (FR-015-AC-52).
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

IR-461 (both code changes and the crate-internal role comparison have landed): the state and frame
lane gives every state-clause obligation of a package
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
| Planned (IR-624, FR-015-AC-78): `ModelFieldsUnavailable` for an accessor error, `MemberAbsent` for a requested field absent from its returned table, or value-based `ModelMemberNotI64Range` for a present field with no `i64` range | `unsupported`, `StateFrameRefused` carrying the object, field where applicable, and exact cause; no node id is invented for an accessor value |
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

The former `BoundNotResolvedCause` body-member cases (`UnboundedType`, `NotIntegerRange`,
`EndpointOutsideI64`, `MemberAbsent`, and `ValueNotReference`) do not authorize a range read.
An object refused by `model_object_fields`, including a non-model object or a model object
whose admitted body is not the selected declaration shape, maps to `ModelFieldsUnavailable`
before any assumption or domain entry. Lowering's independent `RequiresBound` record still
carries its actual `unbounded_type` node. The new reasons are
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
about items in a negotiated request. Crate-internal tests back the direct role comparisons of
FR-015-AC-60 and FR-015-AC-68.

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

IR-624 (planned): read state fields of a model declaration object through Contract IR's
`CheckedPackageV2::model_object_fields(&object_id)` (FR-038-AC-136 to AC-144, IR-628).
The package QSL emits for the twin has an empty object body, as QSpec FR-322 requires, but
IR resolves the effective fields from the selected domain document at admission. The accessor
returns the own and inherited fields once each, sorted by name, with the most-derived member
type. `fields.field(name)` returns `None` for an absent name; `field.member_type()` returns
`Option<&CheckedMemberType>`. For `CheckedMemberType::IntRange { lower, upper }`, both endpoints
are inclusive `i128` values. CG shall convert each endpoint to `i64` without narrowing and use
them as the field's range; conversion failure is `EndpointOutsideI64`. An `Integer`, any other
variant, or `None` is not an `i64` range. The reader shall take neither a model declaration's
body member nor a field read's `result_type` as its range or field set. In particular, a model
field with no read can still have a declared range. A model field absent from the accessor is
`MemberAbsent`, and an accessor error (`UnknownNode`, `NotModelObjectType`, or
`AmbiguousField(name)`) is a typed `ModelFieldsUnavailable` refusal naming the object and error;
CG shall not fall back to a read or body on that error (FR-015-AC-77 and AC-78).

The old hand-built non-declaration object fixture supplied ranges through body-member
`value.target` nodes. That range source is removed: a non-model object and an admitted,
unselected model/object_type with a nonempty body make
`model_object_fields` return `NotModelObjectType`. CG refuses the item with
`ModelFieldsUnavailable` and emits no assumption, `domains` entry or harness. A matching
field read, body member, or grouped derived-shape node cannot turn that refusal into a
range. Positive state/frame cases use QSL-emitted selected model declarations only
(FR-015-AC-78).

IR TC-227 admits a selected model/object_type with a tampered nonempty body only when
that node is unread. When a state clause or frame reads that selected object, IR admission
refuses `StaleNodeKey` at the body member before CG runs. CG therefore does not promise
to observe a selected/read tamper as `NotModelObjectType`; its reachable negative case is
the admitted unselected object above (FR-015-AC-78).

QSL does not currently emit the `None` member type or an integer-range endpoint outside
`i64`. Those negative accessor cases use a QSL-emitted graph with a selected model document
override and recomputed content digests, admitted by IR before CG receives it. They do not
claim that QSL emits those declarations directly (FR-015-AC-78 and AC-81).

The request's `state_fields` remains the field set and draw order of the subject's state struct
(FR-025), in caller order. CG shall check that every listed name occurs in the accessor's
returned field list for a model declaration; it shall not silently drop an absent name or insert
an unrequested inherited name. The clause field and every granted field must still occur in
`state_fields` (FR-015-AC-29). A listed, present field whose member type is `None`, `Integer`,
another non-range variant, or an `IntRange` outside `i64` has no usable range.
Lowering precedes accessor range resolution: when a clause read of an `Integer` field
yields `NotLowered(RequiresBound)`, that record settles the item before CG asks the
accessor for a range. Value-based `NonRangeType` applies only when lowering permits
the accessor lookup, such as an unread listed field or a direct `field_range` lookup.
`ModelMemberNotI64Range` records the object id, field name and one reason (`NoMemberType`,
`NonRangeType`, or `EndpointOutsideI64` with the exact `i128` endpoints); it carries no invented
bound-node id and maps to `unsupported`, `StateFrameRefused` when the field is the clause's.
If the field is the clause's, the item is refused. Otherwise it remains in `state_fields`, has
no `domains` entry, and is drawn without a range assumption. Its identity and persisted record
name the field with `TypeNotRange`, deterministically. `NoRead` is retired for the model declaration
route: whether a read exists cannot decide its range. Duplicate unranged names make the record
invalid (FR-015-AC-78 and AC-81).

The generator and state-clause replay shall share this range resolution so that their domains
for one package agree. Both paths shall refuse an absent name or accessor error. The generator
and replay shall use no alternative source after an accessor failure. The emitted-package path
awaits CG's dependency update and implementation of the merged IR-628 accessor. There is no
body-member fallback.
The change replaces the prior emitted-package refusal test with a positive accessor test, and
retires body-member `BoundNotResolvedCause` cases and updates the `StateFrameRefusal` mapping
and AC-66 mapping tests for
`ModelFieldsUnavailable`. It does not introduce `FieldNotRead`, `ConflictingFieldReads`, or
`MemberDisagreesWithRead`: no `result_type` is authoritative. Until the IR-624
code lands, `field_range` still reads the object body for every object and these criteria remain
planned.

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

## Constraints

| ID | Constraint | Type | Validation |
|----|------------|------|------------|
| FR-015-CON-1 | Clause ABI construction SHALL check both endpoints of a bounded integer dependency before conversion to i64 and refuse one outside i64 as `UnsupportedDependency` in the clause-lowering detail, without producing a binding. | Integrity | Test (TC-025) |
| FR-015-CON-2 | Subject binding SHALL check both endpoints of a bounded integer dependency before conversion to i64 and refuse one outside i64 as `UnsupportedBinding`, without producing a binding or harness. | Integrity | Test (TC-025) |

### Direct family/corpus non-success outcomes (IR-621)

While the finite-input family/corpus APIs remain live, their non-success paths
shall use the matching infallible IR `KaniOutcome::refused`, `invalid_input` or
`resource_exhausted` constructor and return the direct `KaniOutcome` error side
specified by `interface-001`. This adoption retires only the impossible public
outcome-construction error surface. The existing profile census, family/corpus
success, refusal ordering and identity contracts remain in force. Canonical
proof-graph serialization failure remains `Refused` with the exact
`kani_corpus_serialization_failed` cause under NFR-005. Terminal cancellation
pairing and the independently gated AC-111 generator retirement are excluded.
No new installed-backend evidence or IR constructor/proof-count acceptance is
claimed by this consumer change.

## Admitted native integer capability (IR-705 option 2)

When selecting the additive kernel facility of [quire-exact FR-384](ix://agent-ix/quire-exact/FR-384), CG SHALL obtain finite operand domains and operation structure from the authentic CheckedPackageV2 and existing IR-owned accessors, preserve every original input valuation, and request kernel capacity admission for every reachable intermediate. CG SHALL render the same production kernel API in native and proof clients, retain the real Meter and semantic result-domain refusals, and return a typed source-associated capability/generation refusal if capacity or value admission fails. It SHALL NOT add a domain-narrowing assumption, copy evaluator logic, fabricate an admitted key/certificate or present admission failure as a proof verdict. [AD-006](../../assurance/AD-006-admitted-native-integer-oracles.md) owns the selection and evidence decision. Existing general exact generation remains available under its existing capability policy.

CG SHALL use FR-384's operator-specific capacity policy through the actual checked kernel plan: native i128 carriers and interval endpoints, full admitted signed-i128 Add/Subtract/Negate and ordering capacities, and Multiply only when both complete operand intervals lie within signed32 `[-2147483648, 2147483647]`, with the exact four-corner signed64 product image retained in i128. Semantic result-domain endpoints remain i128 and are distinct from the operation capacity. Eligibility SHALL derive from the original complete operand and intermediate intervals; a result-domain restriction or a fitting sampled payload/product cannot repair an unsupported Multiply interval. An unsupported Multiply makes its entire requested arithmetic component ineligible for native generation; the caller's existing policy selects the honest general route or a typed native-capability refusal identifying the source operation and failing interval. No input draw, declared bound, proposition, cover, unwind or solver/time setting may be narrowed to force native admission. The general route receives its own actual qualification; fallback selection is not Verified.

Generated artifacts and canonical identity inputs SHALL bind the actual selected operator-capacity policy and its revision together with the selected ABI and interval decisions, under the owning identity schema's compatibility rules. A policy revision changes that identity even when the original fixture domains remain admitted and the rendered arithmetic for them would otherwise be identical. New qualification uses fresh evidence for that policy and actual production kernel implementation, not an old unrestricted-i128 multiplication proof attempt.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-015-AC-1 | Pre, post, invariant and frame obligations of one contract produce separate harnesses with distinct identities. | Test (TC-025) |
| FR-015-AC-2 | OPEN after the 2026-10-27 IR-348 V1 retirement: the only tagged `BoundClause` test retires; surviving V2 behavior and replacement evidence remain required. Original criterion retained: Every harness identity records its model-domain bounds, taken only from IR `bounded_domain` nodes, and its solver and options and unwind bound. | Test (TC-025) |
| FR-015-AC-3 | An unbounded or non-finite obligation is refused with a typed reason and no harness. | Test (TC-025) |
| FR-015-AC-4 | No harness assumption excludes an undefined, refused or incomplete runtime outcome. | Test (TC-025) |
| FR-015-AC-5 | An obligation whose bounds are unsatisfiable is refused with a typed reason and no harness. | Test (TC-025) |
| FR-015-AC-6 | An obligation over an oracle whose operation identity is `caller_declared` is refused with a typed reason and no harness. | Test (TC-025) |
| FR-015-AC-7 | OPEN for surviving harness families; the V1 bundle portion is retired by the 2026-10-27 owner ruling on IR-348. Original criterion retained: Every generated harness contains exactly one non-vacuity cover; a precondition harness covers that the precondition holds within the IR bounds, and a contract harness's cover stands after the contract call, so a run that satisfies every check without satisfying the cover is not a proof. The cover is the last statement of the harness, after every assertion the harness carries: the state-clause and frame-effect harnesses and the scalar harness place it after their assertions, and a contract harness's checks are inside the contract call its cover follows. A cover that preceded an assertion would be satisfied by the very valuation that falsifies the assertion, and Kani prints one concrete playback per distinct valuation, so it would print the cover's and the run would classify as a failure with no counterexample (IR-451). Tested for the state-clause and frame-effect harnesses (source order) and for the scalar harness (source order); the contract harness is tested for the cover following the contract call; a precondition harness asserts nothing. The V1 bundle and corpus harnesses are under the same rule (FR-015-AC-53), and the guard of FR-015-AC-58 inspects every kind. | Test (TC-025) |
| FR-015-AC-8 | OPEN after the 2026-10-27 IR-348 V1 retirement: the only tagged `BoundClause` test retires; surviving V2 behavior and replacement evidence remain required. Original criterion retained: A postcondition or invariant harness emits every package precondition sharing its anchor operation as a `requires` on its generated contract and records each in its identity's embedded oracles; it embeds no other obligation's oracle; and an obligation whose sibling precondition is not a supported item of the same request is refused with a typed reason naming that precondition and no harness. | Test (TC-025) |
| FR-015-AC-9 | OPEN after the 2026-10-27 IR-348 V1 retirement: the only tagged `BoundClause` test retires; surviving V2 behavior and replacement evidence remain required. Original criterion retained: Every harness identity records solver `cadical` and the complete ordered option vector — function contracts, concrete playback, the exact fully qualified harness, `--exact`, the explicit unwind, the explicit solver, `--output-format regular`, and `--concrete-playback print` — and no option enabling stubbing is emitted. | Test (TC-025) |
| FR-015-AC-10 | OPEN after the 2026-10-27 IR-348 V1 retirement: the only tagged `BoundClause` test retires; surviving V2 behavior and replacement evidence remain required. Original criterion retained: Regeneration from equal inputs is byte-identical, and changing the unwind bound or the customer subject changes the harness identity. | Test (TC-025) |
| FR-015-AC-11 | OPEN after the 2026-10-27 IR-348 V1 retirement: the only tagged `BoundClause` test retires; surviving V2 behavior and replacement evidence remain required. Original criterion retained: Every symbolic argument carries an inclusive assumption equal to its IR `bounded_domain` or a literal operand's own value (FR-015-AC-16); for a V2 scalar claim, an operand with the FR-014 inferred application interval uses that interval (FR-015-AC-103). A bounded-integer post-state result is required to lie in the same domain; no generated source carries a `#[kani::unwind]`. | Test (TC-025) |
| FR-015-AC-12 | OPEN after the 2026-10-27 IR-348 V1 retirement: the only tagged `BoundClause` test retires; surviving V2 behavior and replacement evidence remain required. Original criterion retained: A request naming no items, more than 256 items, an unparsable subject path, or an unwind bound outside `1..=1024` is refused whole, with no item accounted and no harness exposed. | Test (TC-025) |
| FR-015-AC-13 | OPEN after the 2026-10-27 IR-348 V1 retirement: the only tagged `BoundClause` test retires; surviving V2 behavior and replacement evidence remain required. Original criterion retained: An otherwise-supported obligation whose generated harness source exceeds the bounded-resource ceiling is refused with a distinct resource-limit reason naming the generated size, and one that fits the ceiling but fails to parse as Rust is refused with a distinct syntax reason naming the parse error; neither is reported as the internal-invariant render-assembly fallback. | Test (TC-025) |
| FR-015-AC-14 | A scalar-claim item whose graph node carries a tag/form pair this generator does not model as a contract role is refused with a typed reason naming that tag and form, and no harness is emitted; a claim naming a node id absent from the graph entirely is refused rather than accounted as supported with no contract role recorded. | Test (TC-025) |
| FR-015-AC-15 | A scalar-claim item whose claim is a `NoDerivableClaim` refusal yields `unsupported` with the `no_derivable_claim` reason naming the node and the derivation refusal, emits no harness, and does not reject the group. | Test (TC-033) |
| FR-015-AC-16 | A scalar harness constrains a literal operand, inline or a reference to a `value` node whose body is a literal, to exactly its own value: `x + 1` over `x: Int[0, 9]` into `Int[0, 10]` has arguments `[0, 9]` and `[1, 1]`. | Test (TC-033) |
| FR-015-AC-17 | A scalar claim whose literal operand does not fit `i64` (`x + 10^23`) is `unsupported` as `domain_not_representable_in_i64` naming the literal as both endpoints, with no harness. | Test (TC-033) |
| FR-015-AC-18 | A scalar claim whose operation over its operand ranges yields no result inside the result range (`x + 100` over `x: Int[0, 9]` into `Int[0, 10]`, reachable `[100, 109]`) is `unsupported` as `result_bound_unreachable` naming both ranges, with no harness. | Test (TC-033) |
| FR-015-AC-19 | OPEN: V1 contract-arm evidence retires under the 2026-10-27 owner ruling on IR-348; a supported V2 transition control remains to be measured. Original criterion retained: A zero-input transition over one bounded-integer state value verifies for an identity subject and is falsified with a concrete counterexample for a subject that changes the value. | Test (TC-025) |
| FR-015-AC-20 | OPEN: V1 clause evidence retires under the 2026-10-27 owner ruling on IR-348; V2 clause behavior remains planned under AC-40. Original criterion retained: A harness for a Boolean-connective or bounded-integer comparison claim (`and`, `or`, `not`, `implies`, `eq`, `ne` and the six integer comparisons) embeds its FR-014 oracle byte-identical to the oracle crate's function. | Test (TC-025) |
| FR-015-AC-21 | A precondition that reads a post-state value is refused with a typed reason naming the node, and no harness is emitted. | Test (TC-025) |
| FR-015-AC-22 | A declared proof-dependency census with an empty or duplicate identity, an inconsistent kind, state and path combination, or a non-`Required` kind is refused as a typed invalid input with no harness. | Test (TC-025) |
| FR-015-AC-23 | Every requested item receives exactly one `ObligationDisposition` (`supported`, `requires-bound`, `unsupported` or `invalid-request`), and an item that is not `supported` keeps its source identity and typed reason with no harness and no assumption emitted for it. | Test (TC-025) |
| FR-015-AC-24 | OPEN: V1 comparison-claim evidence retires under the 2026-10-27 owner ruling on IR-348; supported V2 playback remains to be measured. Original criterion retained: A falsifying in-domain assignment of a plain bounded-integer comparison claim is reported through Kani's concrete playback. | Test (TC-025) |
| FR-015-AC-25 | A valid declared census is folded into the harness identity, readiness is `ready` only when every dependency passed and `incomplete` while any is missing or failed, and proof execution is recorded `not_run`. | Test (TC-025) |
| FR-015-AC-26 | One `postcondition` `state_clause` of an admitted package, whose condition is one integer comparison of the pre and post reads of one field through its first parameter `self`, yields a separate operation-contract harness and frame-effect harness for the operation its anchor names. Each has exactly one non-vacuity cover, and its identity is scoped to the operation, anchor, frame and framed object; the two harnesses of two clauses of one operation are written to different paths. | Test (TC-025) |
| FR-015-AC-27 | The operation-contract harness assumes each ranged state field within the inclusive `i64` endpoints returned by `model_object_fields` for the QSL-emitted selected model declaration (FR-015-AC-77). It draws a present non-clause field with no usable range without an assumption (FR-015-AC-78), asserts the clause's comparison between pre-state and post-state, and refuses a clause whose own field has no usable range. A non-model object or admitted unselected model/object_type body is refused rather than used as a range source (FR-015-AC-78). PLANNED (IR-624). | Test (TC-025) |
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
| FR-015-AC-39 | A V2 clause harness draws one nondeterministic value per drawn input: the pre-state of each state field of `self` the body reads, whether bare or under `pre(...)` (the subject's input state; how it reaches the subject is FR-025's), and each operation parameter (for a precondition or invariant, each bare state field read, since the state is the one judged). Each carries an inclusive assumption equal to its declared domain: the accessor's `IntRange` values for a selected model declaration state field (FR-015-AC-77), and the IR `bounded_domain` of a parameter's type. A non-model or admitted unselected model/object_type body supplies no field domain (FR-015-AC-78). A `Reference` parameter such as `self` is not an argument and binds through its object's declared fields; no bound comes from a caller. A postcondition's result and bare post-state reads are not drawn: they are what the subject call produces, their domain is asserted and never assumed (FR-015-AC-4), and a produced value outside its domain fails the harness. An input with no finite domain refuses the claim as FR-015-AC-3 states. PLANNED (IR-489). | Test (TC-025) |
| FR-015-AC-40 | A harness for a V2 clause whose condition, inline or referenced from an admitted Boolean expression node, contains Boolean connectives and bounded-integer comparisons (`and`, `or`, `not`, `implies`, `eq`, `ne` and the six integer comparisons) embeds an FR-014 oracle of that condition byte-identical to the oracle crate's clause-identified function, as FR-015-AC-20 states for a scalar claim; the clause-identified oracle needs FR-014-AC-38. PLANNED (IR-489). | Test (TC-025) |
| FR-015-AC-41 | A V2 clause whose body holds integer arithmetic (add, subtract, multiply, negate) as an operand of a comparison lowers, and its harness keeps the independent native assertion of FR-015-AC-37 for each arithmetic subterm (the oracle's outcome against the operation evaluated natively in `i128` over the same operands, `Refused` assumed away nowhere): a postcondition `amount < 1000 implies amount + 1 <= 1000` over `amount: Int[0, 1000]` yields one contract harness. With the installed backend the unmutated clause verifies with its cover satisfied. In separate unconstrained installed-Kani runs of that same clause, mutate the `+` arithmetic actually evaluated by the embedded clause oracle or its authoritative kernel operation to return `left + right + 1`, then `left`; each run is falsified by the independent native assertion with concrete in-domain playback. For the plus-one mutant, a separate deterministic concrete playback at `amount_current = 999` against the same mutated authoritative owner and the actual generated harness's independent arithmetic assertion must fail. In this selected fixture the subject returns true normally at 999, and the generated harness executes that arithmetic assertion before checking the clause verdict; this ordering does not constrain unrelated result-dependent probes. The playback failure must identify the generated arithmetic assertion by its check message or source location: an assumption, clause/postcondition check or cover failure does not satisfy it. The playback-only test is appended inside the generated module immediately before its closing brace, outside the proof function; removing exactly those appended bytes restores the unique full sealed rust artifact, including its header, oracle source and module, byte for byte. That restored artifact, its identity record, oracle bytes, clause identity and native expectation are unchanged from the mutant's unconstrained Kani run. The backend may select any in-domain counterexample for the unconstrained run; the deterministic playback establishes the exact `999` control. Mutating only a separately compiled subject expression or a harness probe, or substituting a hand-restated assertion for the generated one, is insufficient: the oracle-side arithmetic must execute in the installed proof and in the concrete playback, with the native expectation and clause identity unchanged. The real-Kani control is verified by the quire-integration exemplar of AD-004 L-5 when QSL's facade offers no way to build the package from CG. PLANNED (IR-489; literal `numeric.narrow` operands depend on FR-014-AC-54). | Test (TC-025, TC-027) |
| FR-015-AC-42 | A V2 clause body holding an operator the V2 arm does not support (division, modulo, absolute value, and every operator outside connectives, the six integer comparisons and add, subtract, multiply, negate; those operators are planned scope for a later criterion) is `unsupported`, with a typed reason naming the node and the operator and no harness; a node that is not a modelled clause role, or a node id absent from the package, is refused as FR-015-AC-14 states; no item is skipped silently, and a supported item of the same request is unaffected. PLANNED (IR-489). | Test (TC-025) |
| FR-015-AC-43 | A V2 postcondition harness emits every `precondition` clause node sharing its anchor node as a `requires` and records each in its identity's embedded oracles, embeds no other obligation's oracle, and a postcondition whose sibling precondition is not a supported item of the same request is refused naming that precondition with no harness; a V2 invariant is anchored at an object type, binds `self` alone, and so has no operation precondition to assume. PLANNED (IR-489). | Test (TC-025) |
| FR-015-AC-44 | A census declared on a V2 clause claim with an empty or duplicate dependency identity, an inconsistent kind, state and path combination, or a non-`Required` kind is refused as a typed invalid input with no harness, by the same rules as FR-015-AC-22; the census request and readiness types it uses are not `ProofDependencyGraph`. PLANNED (IR-489). | Test (TC-025) |
| FR-015-AC-45 | A valid census declared on a V2 clause claim is folded into that harness's identity record, readiness is `ready` only when every dependency passed and `incomplete` while any is missing or failed, and proof execution is recorded `not_run`, by the same rules as FR-015-AC-25; in addition, the folded census is independent of the order the caller lists its dependencies. PLANNED (IR-489). | Test (TC-025) |
| FR-015-AC-46 | Every V2 clause harness contains exactly one non-vacuity cover, placed after every assumption, after the subject call and after every assertion the harness carries (the last statement, as FR-015-AC-7 states), that states the family's reachability, as FR-015-AC-7 does: a precondition harness covers that the precondition holds within the bounds; a postcondition harness covers, after the subject call returns, that the requires and the bounds are jointly satisfiable; an invariant harness covers that the drawn state satisfies its domain assumptions. The subject call is the call of the customer subject at the request's subject path in a postcondition harness, which supplies the result and post-state; a precondition harness and an invariant harness have none, and their cover follows the last assumption. A V2 clause whose assumptions and bounds are jointly unsatisfiable is refused with a typed reason and no harness, or, if it reaches a backend run, does not classify `Verified`. PLANNED (IR-489). | Test (TC-025) |
| FR-015-AC-47 | Regeneration of a V2 clause harness from equal inputs is byte-identical, and changing the unwind bound or the customer subject changes the harness identity record (as FR-015-AC-10 states). PLANNED (IR-489). | Test (TC-025) |
| FR-015-AC-48 | The obligation identity of a V2 clause claim is formed by AD-003 E-1 from the clause node id, its occurrence key, the obligation kind, and the arguments each as parameter node id and declared domain for an operation parameter, as E-1 states (the domain FR-015-AC-39 defines). A state field read is named by its declaring node id and field name, never by a node of its own; that E-1 lists parameters only is an open question for the AD-003 owner and E-1 is not widened here. The source span is excluded; changing any included member changes it, and changing the span, the unwind bound or the subject does not. It is a different value from the harness identity record of FR-015-AC-47. PLANNED (IR-489). | Test (TC-025) |
| FR-015-AC-49 | A V2 invariant claim yields a harness that draws its state and parameters within their declared domains (FR-015-AC-39) and asserts the invariant clause; its subject is the clause itself, with no subject call. Preservation of the invariant under an operation is not specified by this criterion. PLANNED (IR-489). | Test (TC-025) |
| FR-015-AC-50 | `negotiate` reports an obligation whose scalar refusal is `ExactScalarRefusal::LoweringByteLimitExceeded` or `LoweringLimitUnrecognised` as `Outcome::Unsupported(UnsupportedObligation::OracleRefused { refusal })` with `refusal` equal to the scalar refusal, field for field, and neither is mapped to another `UnsupportedObligation` variant nor to `LoweringWorkExhausted`. | Test (TC-025) |
| FR-015-AC-51 | Until the final retirement slice (FR-015-AC-111, AD-004 step 4g) deletes the hand-built bounded-corpus path, a finite input validated under a profile selection other than the offered profile's is refused as a typed `InvalidInput` `kani_profile_input_mismatch` result naming the request's source id, with no artifact and no case identity recorded. | Test (TC-023) |
| FR-015-AC-52 | Until the final retirement slice (FR-015-AC-111, AD-004 step 4g) deletes the hand-built bounded-corpus path, the context of every successful `BoundedCorpusExpectation` and typed refusal that path returns (`kani_corpus_dependency_invalid`, `kani_corpus_identity_collision`, a lowering refusal and `kani_profile_input_mismatch`) is the profile selection's revision. The expectation is not a Kani run verdict. | Test (TC-023) |
| FR-015-AC-53 | OPEN for surviving V2 precondition, scalar, state-clause, frame-effect and corpus harnesses; the V1 contract and bundle portions retire under the 2026-10-27 owner ruling on IR-348. Original criterion retained: Each of the seven harness kinds the generator emits (precondition, V1 contract, scalar, state-clause, frame-effect, V1 bundle and corpus) is subject to the cover-last rule of FR-015-AC-7: exactly one `kani::cover!`, the last statement of the body. What the cover witnesses, by kind: a precondition harness, that the precondition holds within the IR bounds (it asserts nothing); a V1 contract harness (postcondition or invariant), that the requires clause and the IR bounds are jointly satisfiable, after the contract call whose `ensures` is the harness's check; a scalar harness, that the oracle's `Completed` branch is reached; a state-clause harness and a frame-effect harness, that the bounded state is reached and the subject returns; a V1 bundle harness, as FR-015-AC-54 states; a corpus harness, as FR-015-AC-55 states. | Test (TC-025) |
| FR-015-AC-54 | RETIRED (2026-10-27 owner ruling, IR-348; V1 bundle-only, original criterion retained): A V1 bundle harness (`generate_kani_bundle`, the `proof_for_contract` harness it emits for a requires clause, an ensures clause and the bundle's bounded arguments) ends with exactly one `kani::cover!` after the call of its contract, which witnesses that the requires clause and the argument and result bounds are jointly satisfiable, so a run whose requires clause no bounded argument satisfies never reaches it; the bundle's `ensures` is checked at the contract call, so the cover follows every check the harness carries. | Test (TC-025) |
| FR-015-AC-55 | While a family's corpus arm remains (deleted by FR-015-AC-109 for arithmetic, FR-015-AC-110 for collection, FR-015-AC-111 for graph), a bounded-corpus harness of that family ends with exactly one `kani::cover!` after its `assert!` of the case's oracle, which witnesses that the harness runs to its end past that assertion. | Test (TC-023) |
| FR-015-AC-56 | RETIRED (2026-10-27 owner ruling, IR-348; V1 bundle-only, original criterion retained): With the installed backend, a V1 bundle harness whose requires clause some bounded argument satisfies and whose `ensures` holds for every such argument classifies `Verified` (FR-017-AC-4); the same bundle with a requires clause no bounded argument satisfies classifies `CoverUnsatisfied` with its satisfied and total cover counts, never `Verified` and never `Falsified`. | Test (TC-025) |
| FR-015-AC-57 | While its arm remains (FR-015-AC-109 to FR-015-AC-111), with the installed backend, a bounded-corpus harness of each of the arithmetic, graph and collection families whose oracle is true classifies `Verified`, and a graph or collection harness whose oracle is false (an arithmetic case's oracle is always true) classifies `Falsified` carrying the assertion's playback, which until FR-015-AC-111 deletes the corpus replay is replayed through the corpus case's QSL source (`tc_023_real_corpus_playback_replays_through_its_qsl_source`), never `Inconclusive` for lack of a counterexample. | Test (TC-023) |
| FR-015-AC-58 | OPEN for surviving emitting entries; V1 contract and bundle entries retire under the 2026-10-27 owner ruling on IR-348. Original criterion retained: A test generates a harness through every emitting entry point (the precondition, V1 contract, scalar, state-clause and frame-effect families, `generate_kani_bundle`, and the corpus generator for each of its families whose arm remains, per FR-015-AC-109 to FR-015-AC-111), parses each emitted source, and fails for any function attributed `#[kani::proof]` or `#[kani::proof_for_contract]` whose body does not contain exactly one `kani::cover!`, which must be the last statement, with no assertion after it and no other cover; and a scan of the non-test string literals of `src/` fails when a file other than the ones the test drives spells a proof attribute, or a driven file spells more proof attributes than the test counts; a spelling is `#[kani::` other than `requires`, `ensures`, `stub` and `unwind`, `kani::proof` or `proof_for_contract` in one literal, with `\` line continuations joined, which includes a `format!` template and a split `concat!` whose first fragment holds `#[kani::`, and the scan does not see a spelling assembled from fragments none of which holds `#[kani::` or `kani::proof`. | Test (TC-025) |
| FR-015-AC-59 | A request of `StateFrame` items in which the first item is refused and later items are supported, requires-bound, unsupported or invalid yields exactly one `ObligationRecord` per item in request order, `kind` `postcondition` for a `contract` item and `frame` for a `frame` item; and each later item's record (disposition and reason) equals the record the same item has in a request holding that item only; a request of three refused items yields three records. The requires-bound item comes from a lowering `RequiresBound` record in that same admitted package, not an object-body member target; removing the body route does not weaken mixed-disposition independence. | Test (TC-025) |
| FR-015-AC-60 | A `StateFrame` item that yields is recorded `supported` carrying its harness symbol, and its harness, returned in the `state_frame_harnesses` field of `KaniObligationOutcome::Emitted` in request order, is byte-identical to the harness `generate_state_frame_role` returns for the same clause and role, and for a clause both of whose roles succeed to the corresponding harness `generate_state_frame_obligations` returns. COVERED (IR-461): a crate-internal test uses an admitted V2 package to compare direct role generation with each negotiated supported item, its symbol and exact harness bytes, and the both-role entry. | Test (TC-025) |
| FR-015-AC-61 | A `StateFrame` item whose lowering is a requires-bound record is recorded `requires_bound` with that record's `unbounded_type` and no harness. A selected model declaration's clause read of a native `Integer` that makes lowering return `RequiresBound` keeps that lowering refusal; it is not remapped to value-based `ModelMemberNotI64Range`. The latter applies to a present listed `Integer` field only if range resolution is reached, such as an unread field. A non-model or admitted unselected model/object_type body is refused as `ModelFieldsUnavailable`, without inventing an `unbounded_type` node or reading its body. The former body-member unbounded target is covered by this fail-closed refusal rather than `requires_bound`. | Test (TC-025) |
| FR-015-AC-62 | A `StateFrame` item whose lowering is an unsupported-family record is recorded `unsupported` with reason `NoFiniteEncoding` naming the record's node and family, and an item whose node is not a `state_clause` is recorded `unsupported` with `UnknownNodeKind`, each with no harness. | Test (TC-025) |
| FR-015-AC-63 | A `StateFrame` item whose condition is a negation, compares to a literal, uses an operator other than the six integer comparisons, reads through a parameter other than `self`, compares two fields or two reads of one side, or whose frame creates or deletes an object or grants a relationship or foreign field (naming the effect), is recorded `unsupported` with `StateFrameRefused` and no harness. The same result holds for a clause that is not a postcondition, is malformed, whose frame grants every field, whose lowering is an invalid-body, incomplete or over-budget record, or whose object is non-model or an admitted unselected model/object_type with a nonempty body (`ModelFieldsUnavailable`, FR-015-AC-78). A selected model field absent from the accessor or with no representable `i64` range is refused as FR-015-AC-78 states. None maps to `NoFiniteEncoding`. The former body-member `NotIntegerRange`, `EndpointOutsideI64`, `MemberAbsent` and `ValueNotReference` probes are replaced by accessor and object-shape refusals. The invalid-body, incomplete and over-budget lowering records are asserted through FR-015-AC-66's mapping test. | Test (TC-025) |
| FR-015-AC-64 | A `StateFrame` item with an unparsable state path or an unparsable item subject path, an invalid or lacking state field, an absent node, a repeat of an earlier item or a package other than the first item's is recorded `invalid_request` (`InvalidStatePath`, `InvalidStateField`, `UnknownNode`, `DuplicateItem`, `MixedStatePackages`), and the outcome is `Rejected` with every item's record and no harness bytes. | Test (TC-025) |
| FR-015-AC-65 | A request holding `StateFrame` items and no items, more than `MAX_OBLIGATION_ITEMS`, an unparsable request subject path or an unwind bound outside `1..=MAX_OBLIGATION_UNWIND` is refused whole with the existing `KaniObligationError` variant and no record. | Test (TC-025) |
| FR-015-AC-66 | A test builds every retained `StateFrameRefusal` variant (including `ModelFieldsUnavailable`, `MemberAbsent`, `ModelMemberNotI64Range` with each cause, and each of the six refusal arms of `NotLowered`) and calls the mapping on it; each yields the disposition and reason of the table. It does not construct retired body-member `BoundNotResolvedCause` values. The variants a request reaches only with a defective generator or a source over 1 MiB (`UnwindOutOfRange`, `InvalidGeneratedSyntax`, `RecordSerialization`) are built directly, and `ResourceLimitExceeded` is built directly or reached with a sufficiently large state-field list. An inspection confirms the mapping is a `match` with no wildcard arm. | Test (TC-025) |
| FR-015-AC-67 | `generate_state_frame_obligations` refuses with `MalformedClause` at the node, not `InvalidField` or `ConditionNotSupported`, for a clause whose graph field name is not a Rust identifier; a field name the caller supplies that is not an identifier is still `InvalidField`. | Test (TC-025) |
| FR-015-AC-68 | A `StateFrame` request whose frame grants every state field records its `frame` item `unsupported` (`StateFrameRefused`) and its `contract` item `supported` with the harness `generate_state_frame_role` returns for the `contract` role; a request whose condition is a negation records its `contract` item `unsupported` and its `frame` item `supported` with the harness `generate_state_frame_role` returns for the `frame` role; in both, `generate_state_frame_obligations` returns the refusal of the failing role; a request whose lowering is refused records both items alike. COVERED (IR-461): a crate-internal test compares each direct role with its negotiated item for both one-role refusals and a refused lowering, including exact supported harnesses, both `RequiresBound` records, and the pair entry's refusal. | Test (TC-025) |
| FR-015-AC-69 | A composite-equality claim over operands that are parameters or literals of one record, tuple or option type whose leaves are Boolean or bounded integers, with the descriptor FR-018 generated, yields one harness whose identity records family `composite_equality`, proof subject `bounded_shadow` and a refinement obligation in the same result (FR-028-AC-7), separate from the harness of every other item; both operators (`Equal`, `NotEqual`) yield one; and the operator and operand types are read from the descriptor while no bound is. PLANNED (IR-264). | Test (TC-025) |
| FR-015-AC-70 | The harness draws one nondeterministic value per leaf of each parameter operand and no other value: a `bool` per Boolean leaf, an `i64` per bounded-integer leaf assumed inside its `integer_range` (FR-015-AC-11), a presence `bool` per option and a payload only for a present option, and for a record field of optional presence one of `present`, `absent` or `null`; a literal operand's leaves are constrained to exactly its value (FR-015-AC-16); the drawn arguments ascend by parameter node id then by leaf path, each bound to its node id and path (FR-025-AC-1, FR-025-AC-2), and no composite is itself an argument (FR-025-AC-7). PLANNED (IR-264). | Test (TC-025) |
| FR-015-AC-71 | Over every drawn pair the harness asserts that the shadow's verdict equals the expectation's and that the shadow's pair count equals the expectation's, where the expectation is computed from the drawn values by QSpec FR-149's rules (same declaration and pairwise-equal fields or positions in order; two `none` equal; `none` and present unequal; `absent` equals `absent`, `null` equals `null`, `absent` and `null` unequal; the pair count is the node count of the occurrence-pair tree with no early exit after an unequal pair), the verdict of `NotEqual` is the negation of `Equal`'s with the same count, and the harness ends with exactly one `kani::cover!` that the drawn operands compare equal, after every assertion (FR-015-AC-7). PLANNED (IR-264). | Test (TC-025) |
| FR-015-AC-72 | The emitted shadow and harness, parsed, contain no loop (`while`, `loop`, `for`), no recursive call, no `Box`, `Vec`, `Rc`, `String` or map, and no integer type wider than `i64`; and a closure whose pair-node count (the node count of the largest occurrence-pair tree its type admits) exceeds `SHADOW_PAIR_NODE_BUDGET` is refused as `ShadowShapeOverBudget` naming the count and the budget, one at the budget generates, and the budget and the count are recorded in the identity and the evidence. The value of the budget is set by the measurement of the code change and recorded in TC-025's status. PLANNED (IR-264). | Test (TC-025) |
| FR-015-AC-73 | Each row of the unsupported-shape table above is produced by an item reaching that shape and yields its disposition and reason with no harness: an unbounded integer leaf `requires_bound`; a bound outside `i64` `domain_not_representable_in_i64`; a recursive declaration `requires_bound` naming the declaration; one leaf of each family the table lists `ShadowFamilyNotBuilt` naming that family; a `reference` composite, as FR-018-AC-7; an operand that is neither a parameter nor a literal, and a `convert<T>` operand, `OperandNotSupported`; an over-budget closure `ShadowShapeOverBudget`; an item FR-018 refused carries FR-018's refusal; and a supported item in the same request settles as it does alone. PLANNED (IR-264). | Test (TC-025) |
| FR-015-AC-74 | With the installed backend, the harness of the corpus case (two record-typed and two tuple-typed values compared with `=` and `!=`, a record with an optional field and a nested option, Boolean and bounded-integer leaves) is classified `Verified` with its cover satisfied within the memory and wall-clock ceilings its identity records, and its execution evidence records the versions and options of FR-028-AC-23; the refinement run of the same case agrees on every case it runs, and the case's refinement domain is at most the case cap, so the run is classified `exhaustive` and records its case count; a second case whose domain exceeds the cap (two unconstrained `i64`-range leaves) is classified `sampled`, which shows both classes. PLANNED (IR-264). | Test (TC-025) |
| FR-015-AC-75 | With the installed backend each seeded mutant of the shadow is `Falsified` with a concrete playback: an `absent` slot read equal to a `null` slot; one field of the left operand compared with the next field of the right; the pair count omitting the payload pair of two present options; the count stopping after the first unequal field; the operator `NotEqual` rendered as `Equal`; and a field left out of the comparison. Run natively, each seeded mutant of the production oracle settles `refinement_failed` naming the first disagreeing case: the opposite operator emitted; an `absent` slot compared equal to a `null` slot; one pair charged too many for two present options; an optional field's declaration emitted as required (caught as the construction refusal `MissingField` or `NullForRequiredField` of the oracle's own environment, FR-028-AC-14, since the verdict and count do not change); and an oracle that completes a legal pair as `Refused`. A seeded mutant of the closure reader that the shadow, its expectation and the production oracle share is caught by FR-018-AC-23's independent read, not by refinement: one that drops a member, narrows an integer bound by one, or reads an optional member as required. A mutation of the order of the members the reader returns is not a mutant of this set (the `order_only` exemption of FR-028-AC-18). The unmutated corpus case verifies and its refinement run agrees. PLANNED (IR-264). | Test (TC-025) |
| FR-015-AC-76 | The identity of a composite-equality harness records family `composite_equality`, proof subject `bounded_shadow`, the closure's declaration keys each with its leaves' declared domains, the operator, the size budget and the closure's pair-node count, the abstractions `representation`, `integer_widening`, `absent_payload` and `unrolled_walk` each with its discharging evidence, and the behaviours the proof does not exercise (`meter_limits`, `ill_typed_operands`, `charge_amounts_other_than_pairs`, `foreign_reference`, `declaration_reconstruction`) each with the native criterion that covers it (FR-018-AC-9, FR-018-AC-8, FR-018-AC-2, FR-018-AC-7, FR-018-AC-23) and that criterion's backing state, `backed` where the test matrix lists it covered and `unbacked` otherwise, which a test asserts against the matrix rows (today FR-018-AC-7, AC-8 and AC-9 are planned and FR-018-AC-2 is partly covered, so each of those four reads `unbacked`); a result whose covering criterion is `unbacked` is reported with that state and is not offered as closing a family row; changing the closure, a bound, the operator or the budget changes the identity; and regeneration from equal inputs is byte-identical. PLANNED (IR-264). | Test (TC-025) |
| FR-015-AC-77 | PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation pending. For the QSL-emitted twin package, `field_range` calls `model_object_fields` on the framed object's selected model declaration and obtains both `balance` and unread `audit` with `IntRange { lower: 0, upper: 1000 }`; it converts their `i128` endpoints to `i64` and returns 0 to 1000 for each. A second admitted unit with the same declaration but no read of `audit` returns the same range for `audit`; IR FR-038-AC-140 independently shows a read's range-node body does not change the accessor's bound. The generator's `domains` and state-clause replay's declared ranges agree for one package. No range is read from an object body. | Test (TC-025) |
| FR-015-AC-78 | PLANNED (IR-624), IR-628 accessor merged; CG implementation pending. `field_range` returns `MemberAbsent` for a requested name absent from the selected model declaration's accessor, including a clause's own field; an accessor `UnknownNode`, `NotModelObjectType` or `AmbiguousField(name)` returns `ModelFieldsUnavailable` with that error and object id, with no body or read fallback. A non-model object and an admitted unselected model/object_type with a nonempty body both yield `NotModelObjectType` and refuse even if body members claim valid ranges. A selected model/object_type whose nonempty body is read by a state clause or frame is refused by IR admission as `StaleNodeKey` before CG sees a package; TC-227's admitted selected tamper is unread and is not a CG generator/replay fixture. If lowering succeeds and range resolution is reached, a present model field with `member_type()` of `None` returns `ModelMemberNotI64Range` with `NoMemberType`; `Integer`, `Boolean`, `Option`, `Collection` or `Reference` returns it with `NonRangeType`; `IntRange` whose lower or upper cannot fit `i64` returns it with `EndpointOutsideI64` and the exact `i128` endpoints. Each cause names the model object and field, not a body-bound node; only two inclusive `i64` endpoints yield a `domains` entry. A clause's own field without a range refuses its item if lowering has not already refused it; another listed field without a range remains drawn without an assumption. `ModelFieldsUnavailable` and `ModelMemberNotI64Range` map to `unsupported`, `StateFrameRefused` (FR-015-AC-66). No body-member `UnboundedType`, `NotIntegerRange`, `ValueNotReference` or node-based `EndpointOutsideI64` cause is consulted. | Test (TC-025) |
| FR-015-AC-79 | PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation pending. From the package QSL emits for the twin's unit, both roles of each clause (`BalanceNeverDrops`, `AuditNeverDrops`) are generated; each identity's `state_fields` equals the request's list in its order, its `domains` equal the accessor ranges of FR-015-AC-77, and its `scope.anchor` and `scope.frame` equal the ids `qsl_replay::call_site` names, asserted before replay. The accessor's ascending-name order does not reorder `state_fields`. A request naming a field absent from the accessor is refused; one omitting the clause's or a granted field is refused as FR-015-AC-29 states. | Test (TC-025) |
| FR-015-AC-80 | PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation pending. With the installed backend, the cases of FR-015-AC-30 and FR-015-AC-31 run over harnesses generated from the package QSL emits: the healthy subject verifies, the subject mutated to debit is falsified naming the postcondition, a granted write verifies and a write to an ungranted field is falsified naming that field. | Test (TC-025) |
| FR-015-AC-81 | PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation pending. A present listed model field with `member_type()` of `None`, a non-`IntRange` variant, or `IntRange` outside `i64` is recorded as unranged with `TypeNotRange` in `StateFrameIdentity` and its persisted record, even when no read names it; a present unread `IntRange` within `i64` is recorded in `domains` and is absent from the unranged list. Equal inputs produce byte-identical records; a record naming a field twice is invalid. The model declaration path records no `NoRead` reason. QSL-emission gaps for `None` and out-of-`i64` types are exercised by an admitted QSL-emitted graph with selected-model-document override and recomputed digests. | Test (TC-025) |
| FR-015-AC-82 | Until FR-015-AC-111 deletes the bounded-profile classifier, a request for distinct named constructs returns one `CapabilityEntry` per requested construct in request order, preserving each admitted `Supported` module or `Refused` cause code exactly. The Contract IR profile reader rejects the removed `inconclusive` disposition rather than admitting it; no profile disposition or profile-supplied code becomes an `Inconclusive` run outcome or asserts proof vacuity. | Test |
| FR-015-AC-83 | Until FR-015-AC-111 deletes the bounded-profile classifier, an empty, duplicate or absent construct is a typed `KaniOutcome` refusal rather than a partial disposition census. | Test (TC-023) |
| FR-015-AC-84 | Until the arithmetic slice (FR-015-AC-109) deletes the finite-input arithmetic lowerer, a profile/dispatch-admitted add, subtract, multiply, divide or remainder returns the exact checked `i128` result only inside the request's inclusive result range. | Test (TC-023) |
| FR-015-AC-85 | Until the arithmetic slice (FR-015-AC-109) deletes the finite-input arithmetic lowerer, an inverted result range is `InvalidInput`; zero division, checked overflow and a result outside the range are typed refusals, with no arithmetic result or generated artifact. | Test (TC-023) |
| FR-015-AC-86 | Until the graph slice (FR-015-AC-111) deletes the finite-graph lowerer, a profile/dispatch-admitted request follows only the selected reference field, finds a positive-length path in sorted depth-first order, and checks the expansion bound before entering each new object; a target edge is found without expanding its target. | Test (TC-023) |
| FR-015-AC-87 | Until the graph slice (FR-015-AC-111) deletes the finite-graph lowerer, a zero expansion bound or unknown endpoint is `InvalidInput`, and exhausting the bound is `ResourceExhausted`, with no graph result or generated artifact. | Test (TC-023) |
| FR-015-AC-88 | Until the collection slice (FR-015-AC-110) deletes the finite-collection lowerer, a profile/dispatch-admitted `ForAllNonNegative` or `ExistsEqual` query retains input order and duplicates and returns its truth value and examined-item count; a decisive item ends that count, otherwise every item counts. | Test (TC-023) |
| FR-015-AC-89 | Until the collection slice (FR-015-AC-110) deletes the finite-collection lowerer, an input longer than `max_items` is `ResourceExhausted`, with no query result or generated artifact. | Test (TC-023) |
| FR-015-AC-90 | Until FR-015-AC-111 deletes the bounded-corpus emitter (a family's arm ends at its own slice, FR-015-AC-109 to FR-015-AC-111), one admitted arithmetic, graph or collection case emits all four artifacts (oracle, finite strategy, Kani harness and proof-dependency graph) from the same selected profile, validated finite input and family lowering. | Test (TC-023) |
| FR-015-AC-91 | Until the final retirement slice (FR-015-AC-111, AD-004 step 4g) deletes the hand-built bounded-corpus path, that path returns a `BoundedCorpusExpectation`: the family lowering's true Boolean oracle predicts that the one generated assertion holds with one expected SUCCESS check, and false predicts that it fails with zero expected SUCCESS checks. This value is neither an observed `KaniOutcome` nor a native replay verdict and cannot enter the FR-030 terminal mapper. This interim expectation ends when FR-015-AC-111 deletes the path. | Test (TC-023) |
| FR-015-AC-92 | Until FR-015-AC-111 deletes the bounded-corpus emitter (a family's arm ends at its own slice, FR-015-AC-109 to FR-015-AC-111), a declared dependency census with an empty or repeated identity, inconsistent kind/state/path or a kind other than `Required` refuses as typed `InvalidInput` with no artifact or case-identity claim; a valid census is normalized into the emitted proof graph. | Test (TC-023) |
| FR-015-AC-93 | Until the final retirement slice (FR-015-AC-111, AD-004 step 4g) deletes the hand-built bounded-corpus path, a profile/input mismatch, family-lowering refusal, proof-graph serialization refusal or duplicate case identity returns its typed refusal with the selected profile revision, emits no artifact and claims no new case identity. | Test (TC-023) |
| FR-015-AC-94 | Until FR-015-AC-111 deletes the bounded-corpus emitter (a family's arm ends at its own slice, FR-015-AC-109 to FR-015-AC-111), equal case inputs produce equal artifact paths and contents regardless of emission order, and reordering the same finite graph's objects or references does not change them; changing a request field, selected profile, finite input, ceilings or normalized dependency census changes the case identity and artifact paths. | Test (TC-023) |
| FR-015-AC-95 | AD-004 step 4g retires the hand-built bounded-corpus path in the three per-family slices FR-015-AC-109 (arithmetic), FR-015-AC-110 (collection) and FR-015-AC-111 (graph, which deletes the whole public corpus API, its generation-time expectation and typed refusal outcome); this criterion is the end state they reach together and adds no deletion of its own. The end state is: QSL-emitted packages admitted through Contract IR feed the one V2 generator for arithmetic, collection and graph cases; the collection and graph cases satisfy FR-015-AC-96 to FR-015-AC-102, including typed model-derived symbolic bounds, and each proof-eligible admitted case renders through `HarnessSpec` and the shared `render.rs` with the cover after every assertion. A case without symbolic input emits no proof harness. Neither successful nor refused generation returns a `KaniOutcome` or a proof verdict; a successful case returns artifacts, and only an FR-017 backend run supplies the verdict. The retirement is staged with no compatibility wrapper and no second public generator, and every slice is gated on IR-858 (AD-004 step 4b, FR-015-AC-112 to FR-015-AC-117). Until FR-015-AC-109 to FR-015-AC-111 all hold, FR-015-AC-90 to FR-015-AC-94 remain interim and this criterion remains PLANNED. | Test (TC-023) |
| FR-015-AC-96 | An admitted QSL V2 clause/function body that contains a `quire.op.collection.forall` or `quire.op.collection.exists` operation over a selected model collection field is a collection claim. For a field with typed inclusive cardinality `[minimum, maximum]` and supported finite elements, the generator emits a harness with symbolic length and symbolic elements. Its assumptions admit both cardinality endpoints and every length between them; for `IntRange` elements they admit both inclusive integer endpoints and every integer between them, and for `Boolean` elements they admit both `false` and `true`. They preserve the field's declared order and uniqueness rules and use no caller-supplied `max_items` as a semantic bound. The V2 claim is the admitted clause/function body and field path, not a separate query kind or a finite-input `CollectionQuery`. PARTIAL (IR-268, CG #396 fe20ecc); the uniqueness clause is unbacked by an AC-96-tagged test. Backed by `tests/it/kani_obligations_symbolic_domains.rs`: `qsl_emits_and_ir_admits_the_authored_collection_and_reference_shapes` (:705), `negotiation_renders_the_selected_sequence_claims_deterministically` (:975), `out_of_range_integer_literal_is_rendered_as_a_falsifiable_claim` (:1250), `boolean_sequence_uses_symbolic_slots_for_both_boolean_values` (:1277, both Boolean values), `nonzero_sequence_bounds_are_inclusive_in_the_symbolic_domain` (:1346, both cardinality and element endpoints), `collection_claims_nested_under_not_or_disjunction_are_refused` (:1192). The uniqueness clause is backed only by refusal: ordered and unique shapes are refused, not modelled, by `ordered_set_shape_is_refused_without_erasing_its_uniqueness_semantics` (:1461), which is tagged FR-015-AC-97 and FR-015-AC-102, not AC-96; no AC-96-tagged test backs the uniqueness clause. | Test |
| FR-015-AC-97 | For an admitted collection claim whose selected field has multiplicity lower bound `0` and no finite upper bound, QSL admits an unbounded collection type; when its element type is supported, this generator settles `requires_bound` naming the selected field or unbounded collection type and emits no harness. An admitted field with no finite element domain likewise settles `requires_bound` naming that element type and emits no harness. A field with multiplicity lower bound greater than `0` and no finite upper bound has no QSL kernel type ([QSL FR-056-AC-10](ix://agent-ix/quire-spec-language/FR-056)); QSL refuses it at assembly as `unknown_required_feature`/`unsupported-feature` (`UnsupportedModelMember`), before a V2 package or IR collection item exists, so it is not an IR `requires_bound` result. A finite element type, collection form or `forall`/`exists` body whose semantics this generator cannot encode settles `unsupported` naming that capability and emits no harness. Neither disposition substitutes a finite ceiling. IMPLEMENTED (IR-268, CG #396 fe20ecc). Backed by `tests/it/kani_obligations_symbolic_domains.rs`: `qsl_admitted_collection_without_an_upper_bound_requires_bound_without_harness` (:1699), `qsl_rejects_a_missing_upper_bound_with_a_positive_minimum` (:1847), `finite_sequence_of_unbounded_integer_elements_requires_bound_when_qsl_admits_it` (:1879), `bounded_sequences_of_string_uuid_and_timestamp_require_element_bounds` (:1992), `unbounded_sequence_of_reference_elements_is_unsupported_not_requires_bound` (:1795), `unsupported_collection_predicate_is_refused_without_harness` (:1544). | Test |
| FR-015-AC-98 | A graph claim (one reachability application of either form above, at its `state_clause` node id and argument path) generates a harness when the admitted package gives its covering object type `E` a finite population `[0, N]` through `select_population` and the selected edge has finite multiplicity (for `quire.op.model.reaches_field` the field's `Reference`, `Option` or bounded `Collection` member; for `quire.op.model.reaches` the destination end's finite multiplicity). Every symbolic object and edge is bounded inclusively by those declarations, including zero and maximum where admitted. Reachability follows only the selected field or end and requires a positive-length path: with `start = target`, an isolated object is unreachable and a self-loop is reachable. No caller `max_expansions`, population or multiplicity is a bound. PLANNED (IR-805; success path needs CG integration of the merged IR-851/IR-852 accessors and QSL-820 real-package population emission). | Test |
| FR-015-AC-99 | The collection harness asserts the admitted clause/function body over symbolic inputs drawn from the selected model field's typed domain, ends with the FR-015-AC-7 cover after its assertions, and never replaces those inputs with the finite-input corpus's concrete `Vec<i128>`. The independent native oracle replays the same authored function through `qsl_replay::replay`; it does not substitute a separately authored predicate. A healthy QSL-emitted admitted V2 case yields installed-Kani `Verified` with its cover satisfied; a seeded claim or subject mutant yields `Falsified` with a concrete in-domain playback. A generation-time `BoundedCorpusExpectation` is not proof evidence. PARTIAL (IR-268, CG #396 fe20ecc); implemented for the controls below, while the symbolic-draw, cover-after-assertions and no-`Vec<i128>` clauses are backed only by tests tagged AC-96/AC-100; graph proof controls are AC-106 (PLANNED, IR-805). Backed by `tests/it/kani_obligations_symbolic_domains.rs`: `native_collection_replay_evaluates_the_authored_healthy_and_mutated_functions` (:2064, native oracle through `qsl_replay::replay`) and the `#[ignore]`d Kani-lane test `tc_025_real_kani_collection_claim_verifies_and_mutated_claim_falsifies` (:2080, run by `make kani` and `make kani-gate`). Recorded run (IR-268 close-out, 2026-10-16T22:32): PR head f43adca (code 9c5f291, the same tree as fe20ecc), 1 passed, exit 0; healthy claim `Verified` with 94 SUCCESS; mutant `Falsified` with 93 SUCCESS and 1 FAILURE, in-domain playback; Kani 0.68.0, CBMC 6.11.0, CaDiCaL 3.0.0, unwind 8. The out-of-range literal test (:1250) backs the falsifiable-claim rendering. The clauses that the harness draws symbolic inputs from the field's typed domain, ends with the FR-015-AC-7 cover after its assertions, and never uses the finite-input `Vec<i128>` are asserted by `negotiation_renders_the_selected_sequence_claims_deterministically` (:975) and `nonzero_sequence_bounds_are_inclusive_in_the_symbolic_domain` (:1346), which are tagged FR-015-AC-96, FR-015-AC-100, not AC-99; no AC-99-tagged test asserts them. | Test |
| FR-015-AC-100 | For the collection family, changing the selected field's authored cardinality or element bound, admitted body claim, customer subject or unwind changes the harness identity; equal admitted inputs regenerate byte-identically. The identity records every symbolic collection domain and the solver and complete ordered option vector, and FR-017 run evidence records the actual Kani, Rust and solver versions, unwind and every option used by the installed backend. For the graph family (AC-98), changing the claim's clause id or argument path, the reachability form, relationship or field, its role or name, the population extent, the edge multiplicity, the customer subject or unwind changes the harness identity, and the identity records the population extent, the edge multiplicity and the ordered options; equal inputs regenerate byte-identically. PARTIAL. Collection-family harness identity is implemented (IR-268, CG #396 fe20ecc): `tests/it/kani_obligations_symbolic_domains.rs` `collection_identity_tracks_domains_claim_subject_and_unwind` (:1397), `negotiation_renders_the_selected_sequence_claims_deterministically` (:975, solver and ordered option vector in the identity), and `src/kani/generate/collection.rs` `every_identity_component_changes_the_rendered_harness_path` (:974, each identity component and byte-identical regeneration). The clause that FR-017 run evidence records the actual Kani, Rust and solver versions and every option used by the installed backend is PLANNED: `KaniExecutionEvidence` (`src/kani/run/execute.rs:246-283`) has no Kani, Rust or solver version field (it records arguments, unwind and solver kind), and the Kani-lane test (:2080) is tagged AC-99 and asserts only the solver kind and unwind. The graph family stays PLANNED (IR-805). | Test |
| FR-015-AC-101 | A request whose symbolic collection domain exceeds the generator's representational or resource ceiling settles `unsupported` with a typed reason naming the failing domain or ceiling and no harness; it never tightens the model domain to make a proof tractable. A missing or malformed selected declaration likewise refuses with its typed reason and no harness. For a graph claim, a population or edge domain whose symbolic object or edge count exceeds the ceiling likewise settles `unsupported` naming the domain, with no harness and no reduced population or multiplicity. PARTIAL (IR-268, CG #396 fe20ecc). Backed by `tests/it/kani_obligations_symbolic_domains.rs`: `collection_unwind_must_cover_maximum_length_and_loop_exit` (:1222, unwind ceiling), `collection_domain_larger_than_the_request_memory_ceiling_is_refused_without_harness` (:1608, request memory ceiling) and `collection_claims_nested_under_not_or_disjunction_are_refused` (:1192). Not backed by a tagged test: (1) the representational-ceiling clause: `canonical_model_wire_rejects_cardinality_above_i64` (:1675) tests the upstream wire and does not assert the generator disposition or that no harness is emitted; the test that does, `src/kani/generate/collection.rs:1070` `domain_representability_failures_keep_the_typed_reason`, carries no AC tag. (2) The missing-or-malformed-declaration clause: `src/kani/generate/collection.rs` `a_declaration_without_listable_fields_or_the_selected_field_is_field_not_admitted` (:1137) asserts helper-internal behaviour only, so this clause is unbacked. Tagging :1070 is a follow-up. The graph clause stays PLANNED (IR-805). | Test |
| FR-015-AC-102 | An admitted collection body claim is settled through the FR-015 V2 obligation path, one disposition per requested item; supported items alone emit harnesses. The finite-input lowerer and hand-built corpus of AC-86 to AC-95 neither supply the symbolic bounds nor classify the proof verdict. Graph items take the same V2 path (IR-805), one disposition per requested item, and are not implementation or proof evidence for IR-268. IMPLEMENTED for the collection family (IR-268, CG #396 fe20ecc): `tests/it/kani_obligations_symbolic_domains.rs` `collection_negotiation_accounts_for_each_supported_or_refused_item` (:2117, one disposition per item in a mixed request), `ordered_set_shape_is_refused_without_erasing_its_uniqueness_semantics` (:1461) and `unsupported_collection_predicate_is_refused_without_harness` (:1544, only supported items emit harnesses). The graph clause stays PLANNED (IR-805). | Test |
| FR-015-AC-103 | Through `generate_routed`, `(x + 1) + 1` over `x: Int[0, 9]` into `Int[0, 20]`, `x * -1` over `x: Int[1, 9]` into `Int[-9, -1]`, and `-(x + 1)` over `x: Int[0, 9]` into `Int[-10, -1]` each settle `Supported` with an IR-confirmed outer oracle and a harness. Their respective argument ranges are `[1, 10]` and `[1, 1]`, `[1, 9]` and `[-1, -1]`, and `[1, 10]`; the result assertions use only the outer declared result bounds. Each inferred range carries its source application id in the harness identity. | Test (TC-033) |
| FR-015-AC-104 | A plain-Integer parameter as any nested leaf keeps `requires_bound` with the Integer `unbounded_type` and no harness; a direct reference to a raw application never borrows a separate narrowing conversion's domain, and an inferred endpoint outside `i64` settles `unsupported` as `domain_not_representable_in_i64` naming both exact endpoints. `OperandRangeLimitExceeded`, `OperandUnsupported`, `OperandTypeMismatch` and `InvalidBody` settle `unsupported` with `oracle_refused` carrying the exact scalar refusal, including its node, position or limit fields, and no harness. Refused items leave a supported sibling's harness and claim unchanged. | Test (TC-033) |
| FR-015-AC-105 | For a graph claim the harness draws an object count in `[0, N]` of the one covering population and symbolic objects within it; for every object, the selected edge as its shape states: a `Reference` field has exactly one target, an `Option` field an absent or one target, a `Collection` field or finite multi-valued relationship end a count in `[lower, upper]` (a unique end admits no repeated target, a non-unique end repeats, and `lower > 0`, such as `{2,3}`, forces at least `lower` targets) and every target is an object of the population. Start and target are drawn from that population and may be equal; a subtype operand is drawn from it too. The harness asserts the admitted clause condition with the reachability evaluated over only the selected edge by positive-length search, and ends with the FR-015-AC-7 cover after its assertions. `N = 0`, the maximum, and edge counts `lower` and `upper` are each drawn. PLANNED (IR-805; needs CG integration of the merged IR-851/IR-852 accessors and QSL-820 real-package population emission). | Test |
| FR-015-AC-106 | The independent native oracle replays the same authored clause over each original-domain concrete graph through QSL's replay (`qsl_replay`), and does not substitute a separately authored reachability. With the installed backend, a QSL-emitted, IR-admitted healthy case yields `Verified` with its cover satisfied for one field-edge claim and one relationship-end claim over a `{0,3}` unique and a `{0,3}` non-unique end. Each semantic mutant yields `Falsified` at the clause assertion with an original-domain playback and native replay of that same authored clause: reachability that allows a zero-length path (so an isolated `start = target` reads reachable), one that rejects a self-loop, one following another field or role, and a negated claim. Separately, the domain-conformance/completeness oracle defined below verifies the healthy generated domain and falsifies each pure authored-bound mutant: population extent raised or lowered by one and edge upper bound raised by one. Those controls identify a domain mismatch, with a typed boundary witness, rather than requiring a change in clause truth. A generation-time `BoundedCorpusExpectation` is not proof evidence. PLANNED (IR-805; IR-939 clarifies the domain oracle; needs CG integration of the merged IR-851/IR-852 accessors and QSL-820 real-package population emission). | Test |
| FR-015-AC-107 | Graph claims are located by `state_clause` node id and argument path: one reachability written twice in one condition yields two items at positions 0 and 1; one node under two clauses yields two items with different identities; a field-edge and a relationship-end reachability yield separate items; a collection claim, a clause with no reachability and a path that does not name a reachability application each settle `unsupported` naming the missing claim with no harness. A mixed request settles one disposition per item and a supported item's harness is unchanged by a refused sibling. PLANNED (IR-805). | Test |
| FR-015-AC-108 | A graph claim settles by the disposition sequence in the Description (the paragraph "Disposition sequence for a graph claim"), with no harness, domain reduction or caller `max_expansions` substitute. While the population selection for the claim is `Unavailable` (no available population selection for this admitted claim), the claim settles `unsupported` naming it. Given a selection, `Unbounded` or `UnknownClosureOpenExtent`, that is a claim whose population has no finite authored bound (once a selection is available), settles `requires_bound` naming the population; an unbounded `Collection` edge settles `requires_bound` naming the edge. A selection of `StaleBinding`, `ForeignPackage`, `NotPopulation`, `MissingOccurrence`, `UnknownClosureUnresolvedObjectType` or `UnknownClosureUnresolvedSubtypeClosure`, a relationship-end claim whose typed end is unavailable to CG, and an accessor refusal, each `unsupported` naming the refusal and returning no partial end: `UnknownNode` (the id of the reaches application found at the claim's path, which is the id passed to `relationship_end` and is not the `state_clause` id, is absent from the graph), `NotReachesApplication` (that application id names a node that is not a `quire.op.model.reaches` application) or `UnresolvedEnd` (the member no longer resolves to one retained relationship end). AC-107's location refusal is checked first, so a path that is missing or names no reachability application gets AC-107's refusal and never `NotReachesApplication`. A field member of another shape, and a claim form the encoding cannot express, each settle `unsupported` naming the input. PLANNED (IR-805). | Test, Inspection |
| FR-015-AC-109 | Implemented (IR-598 arithmetic slice only; merged IR-858 satisfies AD-004 step 4b). After this slice no source under `src/` contains the corpus `Arithmetic` request variant and arm, `BoundedCorpusFamily::DefinednessArithmetic`, `prepare_checked_arithmetic`, `checked_method`, `render_arithmetic_*` or `lower/definedness_arithmetic.rs` and their re-exports, and no test in `tests/` drives them. The slice deletes only what the arithmetic family owns: the corpus replay (`BoundedCorpusCase::replay_counterexample`, `compile_corpus_qsl_package`, `BoundedCorpusReplayError` and the replay-harness identity helpers) stays until FR-015-AC-111, because its only caller replays graph and collection cases (an arithmetic corpus case never falsifies, FR-015-AC-57), and the arithmetic slice adds no replay test. V2 add, subtract, multiply and negate serve the family (FR-015-AC-103, FR-015-AC-115, FR-015-AC-117). V2 does not serve divide or remainder, tested: a claim over `quire.op.integer.div` settles `Unsupported(OperationNotRendered)` with no harness (the internal `ScalarLoweringRefusal::NoRenderer`), and `quire.op.integer.rem` stays the refused catalogue identity with no family. IR-854 owns V2 `div` and `mod`. Recorded coverage loss: FR-015-AC-84 and FR-015-AC-85 covered divide and remainder only as a Rust `checked_div` and `checked_rem` literal check (exact result, zero-division and range refusals) with no mutation control, whose retained QSL text uses `/` and `%`; the original IR-598 review inferred from its then-resolved `qsl-forms` that `%` was not a QSL operator and that the remainder case's QSL probably did not compile (not executed then and not rechecked by this spec refresh), hence the loss is not QSL division coverage and IR-854 inherits none of it. Inspection of the no-wrapper rule: the slice adds no `interface-001` operation, and no `src/` item converts a finite-input request type (`CheckedArithmeticRequest`, `CollectionQuery`, `GraphRequest`, `BoundedCorpusRequest`) into a V2 generator call. | Test (TC-023) and Inspection |
| FR-015-AC-110 | PLANNED (IR-598 collection slice; gated on IR-858, AD-004 step 4b, and on AD-004 step 4h, because the collection family renders through `HarnessSpec` and `render.rs` only after 4h). The gate opens when the AC-99 installed-backend controls are green, with their tagged tests present (`tc_025_real_kani_collection_claim_verifies_and_mutated_claim_falsifies` and the symbolic-domain tests tagged FR-015-AC-96 and FR-015-AC-99 in `tests/it/kani_obligations_symbolic_domains.rs`). Evidence for a real-Kani precondition is a recorded `make kani` or `make kani-gate` run on the slice PR's head that is green, cited in the PR (the real-Kani tests are ignored kani-lane tests, so default CI does not run them). After this slice no source under `src/` contains the corpus `Collection` variant and arm, `prepare_bounded_collection_query`, `render_collection_qsl` or `lower/bounded_collections.rs` and their re-exports, and a collection claim settles only through FR-015-AC-96 to FR-015-AC-102, with no corpus fallback. The AC-99 real-Kani collection test is the replacement for the corpus collection replay. Recorded coverage loss: FR-015-AC-88 and FR-015-AC-89 (finite query truth value, examined-item count and the `max_items` `ResourceExhausted` bound) have no V2 equivalent by design, since a caller `max_items` cannot stand in for the model domain (FR-015-AC-102). Inspection of the no-wrapper rule: the slice adds no `interface-001` operation, and no `src/` item converts a finite-input request type (`CheckedArithmeticRequest`, `CollectionQuery`, `GraphRequest`, `BoundedCorpusRequest`) into a V2 generator call. | Test (TC-023) and Inspection |
| FR-015-AC-111 | PLANNED (IR-598 graph and final slice; gated on IR-858, AD-004 step 4b, on FR-015-AC-109 and FR-015-AC-110 having landed, and on IR-805, whose graph claims need CG integration of the merged IR-851/IR-852 accessors and QSL-820 real-package population emission (after QSL-819)). Graph claims are served by FR-015-AC-98 and FR-015-AC-105 to FR-015-AC-108, and FR-015-AC-106's real-Kani graph replay is the replacement for the corpus graph replay; Evidence for a real-Kani precondition is a recorded `make kani` or `make kani-gate` run on the slice PR's head that is green, cited in the PR (the real-Kani tests are ignored kani-lane tests, so default CI does not run them). After this slice no source or test under `src/` or `tests/` contains the corpus `Graph` variant and arm, `prepare_finite_graph_reaches`, `lower/finite_reference_graphs.rs`, `lower/bounded_kani_profile.rs` with `classify_bounded_kani_profile` and `BoundedKaniProfile` (deleted unconditionally; nothing in `src/` uses them except their `lib.rs` re-export), `lower/mod.rs`, `generate_bounded_kani_corpus_case`, the `BoundedCorpusRequest`, `BoundedCorpusCase`, `BoundedCorpusError`, `BoundedCorpusArtifacts`, `BoundedCorpusFamily`, `CorpusProofDependencyGraph` and `EmittedCorpusIdentities` types, `CORPUS_PROOF_GRAPH_SCHEMA` and `schemas/kani-corpus-proof-graph-v1.schema.json`, the corpus's use of the census validation (its calls to `validate_dependencies` in the deleted corpus module; `census_validation.rs` itself is not deleted by this slice, because `validate_dependencies`, `validate_plain_identity`, `validate_path`, `single_diagnostic`, `KaniDiagnostic` and `KaniErrorCode` are also used by `v1_bundle.rs`, so no item in that file is corpus-only, and its items go once their last caller is gone, per AD-004 step 4f), `kani/generate/corpus/`, `tests/it/bounded_kani_corpus.rs` and the corpus replay (`BoundedCorpusCase::replay_counterexample`, `compile_corpus_qsl_package`, `BoundedCorpusReplayError`, and the replay-harness identity helpers `ReplayHarnessIdentity`, `ReplayPackage::harness_identity` and `replay_harness_counterexample` with its `_through` form, which only the corpus uses), together with their only caller `tests/it/bounded_kani_corpus.rs`; the crate exports no corpus API. Generation, successful or refused, returns no `KaniOutcome` (FR-015-AC-95), and a graph claim settles with no caller `max_expansions`. Recorded coverage loss: FR-015-AC-86 and FR-015-AC-87 (finite sorted depth-first expansion bound) have no V2 equivalent by design. A search of `src/` and `tests/` for each deleted name finds nothing. Inspection of the no-wrapper rule: the slice adds no `interface-001` operation, and no `src/` item converts a finite-input request type (`CheckedArithmeticRequest`, `CollectionQuery`, `GraphRequest`, `BoundedCorpusRequest`) into a V2 generator call. | Test (TC-023) and Inspection |
| FR-015-AC-112 | Implemented (IR-858, AD-004 step 4b). `HarnessSpec::new` is the only constructor and refuses with a typed `HarnessSpecError`, never a panic and never a harness: a spec with no cover or more than one cover; a cover whose condition is the literal `true`; no `Call` or more than one; an `Input` or `Assume` after the `Call`; an `Assert` before it; a binding name bound twice (other than an input's own rebinding); a name, `result` or `callee` that is not a valid Rust identifier; an input domain with `minimum` above `maximum` or outside its type; an `unwind` outside `1..=MAX_OBLIGATION_UNWIND`; a message or label containing `"`, `\`, `{`, `}` or a line break; and an `ExprText` that does not parse as a Rust expression, spells `kani::` or `#[`, contains a block, `unsafe`, `async`, closure, loop or `let` expression, or contains a macro invocation other than `matches!`, a `matches!` with a guard, or a forbidden form nested inside a `matches!` argument (AD-004 "The HarnessSpec and render contract (step 4b)", H-2 and H-3). | Test (TC-025) |
| FR-015-AC-113 | Implemented (IR-858). For every spec that constructs, `render` emits the header, the oracle source, the module wrapper and the proof attribute, then the spec's statements in spec order, then its one cover as the last statement of the function: after every assumption, the subject call and every assertion, whatever order a family listed them in. A hand-built spec in each of several statement orders renders its cover last; a cover is not a field of `body` and no statement order moves it (AD-004 "The HarnessSpec and render contract (step 4b)", R-1; FR-015-AC-7). | Test (TC-025) |
| FR-015-AC-114 | Implemented (IR-858). `render` of one spec returns equal bytes on repeated calls and in a second process, and a spec whose `Input` list is reordered renders the reordered source and not the same bytes (AD-004 "The HarnessSpec and render contract (step 4b)", R-1). | Test (TC-025) |
| FR-015-AC-115 | Implemented (IR-858). After the scalar port, for each claim of a fixed set (each of `quire.op.integer.add`, `sub`, `mul` and `negate`; a claim with a literal operand; a claim with an inferred operand; a claim with each operand at an `i64` extreme; claims at two unwind values including `MAX_OBLIGATION_UNWIND`; claims at two ceilings values) the ported path renders the same artifact (path and source bytes), `ScalarObligationIdentity`, `options` and `ObligationRecord`, and the same digests derived from them, as a test-only reference copy of the pre-port `render_scalar` computes in the same test. No stored expectation file and no regenerate mode is involved. The port has no explicitly specified difference. The refusals `ResourceLimitExceeded` and `InvalidGeneratedSyntax` keep their values and their order, now raised by `render` (AD-004 "The HarnessSpec and render contract (step 4b)", P-2 and P-3). | Test (TC-025) |
| FR-015-AC-116 | Implemented (IR-858). `render` holds no family-specific template text: for hand-built specs whose label, expressions and messages are not the scalar family's, the rendered source consists of the fixed header, the spec's `oracle_source`, the module wrapper, the proof attribute, the spec's statements rendered from their typed fields, and the one cover, and contains no other text; and the ported scalar harness is emitted by `render`, so that the FR-015-AC-58 inspection drives `render.rs` for it (AD-004 "The HarnessSpec and render contract (step 4b)", R-1 item 3). | Test (TC-025) |
| FR-015-AC-117 | Implemented (IR-858). With the installed backend, after the scalar port, the unmutated scalar harness classifies `Verified` with its cover satisfied and the existing scalar mutation control (the oracle's `Add` swapped for `Subtract`, FR-015-AC-37) classifies `Falsified`; and a hand-built `HarnessSpec` with an unsatisfiable `Assume`, rendered by `render`, does not classify `Verified` (L-4). This run is a smoke over the ported source and not the evidence of equivalence, which FR-015-AC-115 is (AD-004 "The HarnessSpec and render contract (step 4b)", P-3). | Test (TC-025) |
| FR-015-AC-118 | For an admitted V2 scalar `implies` claim over two Boolean operands, the generated harness embeds its FR-014 oracle as FR-015-AC-20 requires and draws both operands symbolically. It calls `quire_contract_runtime::operators::implies_short_circuit` with `left` and a closure returning `right` as the subject expression, asserts that subject result equals an independent native `!left \|\| right` expectation over the same operands, and separately asserts that the oracle's `Outcome<bool>` is `Completed` with that expectation; neither subject nor oracle computes the native expectation. With installed Kani, the healthy harness classifies `Verified` with its final cover satisfied. In a separate run of the same claim, replacing only the parsed generated subject call expression with `false` classifies `Falsified` with a concrete in-domain playback whose `left` is `false` (the correct implication is `true`). The control fails if the compiled subject call is absent, if mutation changes the oracle or native expectation, or if the result is merely a generation-time Boolean. A V2 clause body containing the same implication remains governed by planned FR-015-AC-40 and its inline-oracle prerequisite FR-014-AC-38; this scalar control does not claim that clause path implemented. | Test (TC-025) |
| FR-015-AC-119 | PLANNED (IR-917; depends on FR-015-AC-40 and FR-015-AC-41). QSL source authors an operation with one parameter `amount: Int[0, 1000]` and its V2 postcondition `amount < 1000 implies amount + 1 <= 1000`; both occurrences of `amount` read that operation parameter, not a state field or subject-produced post-state value. The admitted `CheckedPackageV2` yields a clause-identified FR-014-AC-38 oracle and an FR-015 postcondition harness for that same clause, not a scalar implication harness. Under FR-015-AC-39 the harness draws the parameter once within its declared finite domain and passes that same value to the compiled subject; the postcondition, its oracle and an independent native `!left \|\| right` expectation all use that value, with `left = amount < 1000` and `right = amount + 1 <= 1000`. The harness embeds the oracle byte-identically as FR-015-AC-40 requires; neither the native expectation nor the oracle calls the runtime implication helper. The compiled subject calls `quire_contract_runtime::operators::implies_short_circuit` for that postcondition. With installed Kani, the healthy package and subject classify `Verified` with a satisfied final cover. In a separate run, replacing only the parsed body of `quire_contract_runtime::operators::implies_short_circuit` with a body returning `false` in an isolated checkout of its authoritative source classifies the same clause `Falsified` with concrete in-domain playback for which the implication is true. The admitted package, clause identity, oracle bytes, generated harness source and non-helper subject source are unchanged between runs. The control fails if the compiled subject's helper call or the parsed helper body is absent, if another source component changes, if the two runs share a wrong helper-derived expectation, or if either verdict is only a generation-time Boolean. No runtime source or binary is copied into this repository. This parameter fixture requires no selected-model state-field range path and makes no claim that the separate IR-624 state-field path is implemented. | Test (TC-025) |
| FR-015-AC-120 | PLANNED (IR-942; depends on QSL ADR-013 O-07/O-09 and QSV FR-123). After the selected owner and occurrence validate under AC-122, CG forms the per-step record content identity from an RFC 8785 object whose only members are `subject_kind: "protocol-refinement-step"` and `record: StepSimulationRecord` encoded by QSV. Two separately valid selected refinements with the same obligation and hypotheses but different record `refinement` values, each with its own matching occurrence, yield different content identities; a record with a mismatched owner has no identity to compare. Within one admitted refinement, changing between two valid obligations with their matching occurrences, or replacing one hypothesis while retaining FR-146's ascending node-key digest order, changes the record content identity. A separate raw QSV encoder vector with two supplied hypotheses in opposite order changes the canonical bytes and digest; it does not represent a second admitted QSL emission. The O-07 occurrence and `source_span` are absent from its preimage; changing only source regions or span for a fixed admitted record and its valid occurrence leaves that identity unchanged. The test compares hand-built canonical byte vectors and digests with CG's results, rather than deriving expected digests through CG's encoder. | Test (TC-025) |
| FR-015-AC-121 | PLANNED (IR-942). A per-step `KaniObligationIdentity` uses the QSV-owned `StepSimulationRecord` as a typed subject, carries its content identity and the owning refinement's O-07 `generated` occurrence key separately, and keeps any CG proof or obligation kind in the harness record rather than the record content preimage. CG publishes a persisted harness-record digest over every `KaniObligationIdentity` member except `source_span`; changing any included member changes this digest, while changing only `source_span` does not. This digest is distinct from the per-step record content identity. The record is not reconstructed from a V1 `ClauseRef` or a copied QSV schema. | Test (TC-025) and Inspection |
| FR-015-AC-122 | PLANNED (IR-942). A selected proof-eligible refinement under QSpec FR-322-AC-88 through AC-90 with an `Initial` record and `Step { row: i }` records for its admitted FR-375 `steps` order yields generated occurrences on the owner node at ordinals 0 and `i + 1`, respectively. An ineligible refinement under FR-322-AC-91 (hidden field, history field, or `any` step target) carries only its `declaration` occurrence and yields no per-step record, harness, or proof credit; the IR reader refuses a generated occurrence on it as `invalid_package`/`invalid-value` before CG setup. Two step rows tied on node-key digest and member name use the FR-375-AC-19 alias-free class spelling order, unchanged by source permutation. A different record owner, out-of-range row, wrong occurrence role or ordinal, or a second record with the same `(refinement, obligation)` pair refuses with a typed setup cause and produces no harness or proof credit, including when only the duplicate's hypotheses differ. | Test (TC-025) |
| FR-015-AC-123 | External public-consumer calls to `prepare_bounded_collection_query`, `prepare_finite_graph_reaches` and `generate_bounded_kani_corpus_case` have the direct result signatures specified by `interface-001`, with `KaniOutcome` as their error type and no public `FamilyLoweringError`, `BoundedCorpusError`, `OutcomeConstruction` branch, or replacement alias or wrapper. | Test |
| FR-015-AC-124 | After infallible-constructor adoption, each retained family/corpus adverse control returns the original exact non-success kind, typed cause, source identity and selected profile/bound context, with no Boolean claim, partial artifact or premature identity claim; successful outputs and failure ordering remain unchanged. | Test |
| FR-015-AC-125 | Authentic QSL-emitted, IR-admitted bounded integer scalar and clause inputs generate kernel-owned native carriers and checked operation plans for all intermediates, with original graph identities/domains retained. Add/Subtract/Negate/ordering retain their admitted signed-i128 capacities; Multiply is eligible exactly when both whole operand intervals fit signed32 and uses the exact four-corner signed64 image retained in i128. A Multiply operand interval outside signed32 makes the whole requested arithmetic component native-ineligible, even with a zero opposite operand, fitting actual payload or narrower semantic result domain; the existing policy returns an honest general route or typed source-associated native refusal, with original domains and i64 symbolic draws unchanged. Generated artifact and canonical identity inputs bind the actual operator-capacity policy/revision; changing that policy changes identity. Neither fallback nor a prior proof attempt supplies a Verified result. Reversed/oversized capacities and operands outside a selected plan yield typed source-associated refusal, never narrowed assumptions or a completed value. Calling the explicit generate_routed_native API selects native scalar rendering during routed Kani generation; the ordinary generate_routed API retains its general scalar route. For one generated scalar claim, routed src/lib.rs and the oracle source embedded in its Kani harness are byte-identical; for a group, each embedded oracle source is a byte-identical contiguous module body in routed src/lib.rs. Native production and proof artifacts use the same kernel implementation and real Meter; inspection finds no copied arithmetic or verification-only evaluator. | Test |
| FR-015-AC-126 | Genuine TC-027 healthy and bound mutant, TC-025 arithmetic mutant, IR-917 clause implication and IR-948 V2 pre/post/invariant profiles regenerate through public CG with the reviewed admitted kernel API. Original semantic domains, selected propositions/clauses, independent expected values, subject controls, assertions, final completion covers and existing solver/unwind/time settings remain. Static plan-admission checks establish that the original TC-027, IR-917 and IR-948 fixture operand/intermediate intervals remain admitted under this policy, including their original 0..1000 domains; those checks are not backend proof verdicts. Each healthy profile verifies at 16 GiB; each required substantive mutant falsifies with original-domain native playback. The IR-948 impossible-precondition control must complete as CoverUnsatisfied at the same ceiling, not resource exhaustion or a vacuous Verified. Recorded source/ABI/identity changes explain the new route without reusing stale proof evidence. MemoryExhausted, timeout, unknown, missing cover/mutant or missing teardown is inconclusive. | Test |
| FR-015-AC-127 | End-to-end denied/cancelled admitted-native controls retain all real Meter counters, charge order, complete incomplete/refusal fields and admitted prefixes at every reached charge point, compared with independent normative expectations and the general exact route. A semantic result-domain failure remains after arithmetic charge and before result-retain, separately from generation-time native-capacity failure. | Test |

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
| FR-015-AC-77 | Read a model field from an empty object body or a read's `result_type`, omit unread `audit`, narrow an `i128` endpoint, or let generation and replay use different domains. |
| FR-015-AC-78 | Fall back after an accessor error; treat an absent accessor field as an unranged present field; use an `Integer`, `Option` or out-of-`i64` range as a range; or trust a grouped derived-shape body bound. |
| FR-015-AC-79 | Reorder request fields to the accessor's name order, insert an unrequested inherited field, or emit the hand-built fixture's scope ids. |
| FR-015-AC-80 | Generate the harness from the hand-built fixture rather than the emitted package. |
| FR-015-AC-81 | Mark an unread `IntRange` as `NoRead`, give an untyped member a range, omit its reason, or accept a duplicate unranged name. |

### FR-015-AC-106

The graph verification controls distinguish clause semantics from domain
conformance and completeness. This clarification defines a test oracle, not an
additional production generator, public input model or proof of delivery. The
IR-805 symbolic-graph criteria remain planned until their required evidence
exists.

For a selected claim, let `D(g)` mean that the concrete active graph `g`
belongs to the original authored domain: its population count is in `[0,N]`,
every active row has degree in the selected edge's inclusive `[lower,U]`,
every active target belongs to that population, and its uniqueness and operand
constraints hold. Required and optional fields retain their respective exact
one and zero-or-one multiplicities. A zero population has no live operands;
any representation sentinel is not an object or a path. Only active rows and
active edge slots describe the graph; padding is not an extra object or edge.

The expected predicate `D` is independently implemented from the retained
original QSL model declarations and their admitted typed owner projections:
`reaches_population` / `select_population`, and `relationship_end` or
`model_object_fields` / `CheckedMemberType`, as applicable. The fixture checks
these original projections against its authored population, selected
role/field, multiplicity and uniqueness declarations before mutation. Neither
`D` nor its expected endpoints may be obtained from the generated harness, the
mutated bound, or the generator's mutated lowering result. Mutating a reader
shared with the expected predicate does not qualify as an independent control.
The original package and claim identity stay unchanged.

Let `G(g)` mean that the actual generated sampler can represent and admit that
same active graph, before evaluating the clause. Its observation includes its
real storage extents, input ranges, active-count interpretation and all
sampler assumptions, not just identity metadata or the intended bound. For the
fixed-array encoding, population capacity, row capacity and degree capacity
must cover every active row and slot of `g`; absent capacity means `G(g)` is
false, without indexing outside an array. Dormant padding may be normalized
only after checking that this preserves admission of every active graph.
Observing the emitted sampler's typed statements or parsed source is allowed;
reconstructing `G` solely from an expected or mutated endpoint is not. The
observer is checked against the actual emitted ranges, array shapes and
assumptions, including rejection, so that a domain error in emission is
observable.

With installed Kani, the separate domain oracle asserts both `G(g) implies
D(g)` (conformance) and `D(g) implies G(g)` (completeness) over a common
bounded candidate carrier. The healthy oracle classifies `Verified` with a
satisfied final cover witnessing `D(g) and G(g) and count = N`.
Separate boundary-probe runs each retain exactly one final cover and witness
`D(g) and G(g)` with the selected edge's lower and upper degrees where
feasible; no harness gains an additional cover. These candidates are drawn independently of `G` and `D`:
neither predicate, their conjunction nor the original sampler's assumptions
may filter out a candidate before those assertions. Carrier-only
representation constraints may be assumed, with evidence that every graph in
the compared domains has a representation. A native evaluation of `D` and `G`
agrees with each recorded backend boundary witness.

The candidate carrier is verification storage, distinct from the production
sampler's arrays. It represents the union of the healthy domain and all three
mutated domains, including `N+1` active objects and `U+1` active edges per
row, without pretending the production `[N][U]` arrays can hold them. The
observer reports unrepresentable candidates as rejected rather than truncating
them. All original inclusive ranges remain quantified in full; ceiling or
integer/array representability failures remain refusals, never a smaller
replacement domain. A control whose union carrier cannot be represented or run
within the declared ceilings remains unqualified. This test carrier does not
relax the production representational ceilings of AC-101.

Each pure-bound mutant changes only the generated domain construction for the
selected population maximum or selected edge upper bound, including any
storage extents derived from that bound, while keeping the original package,
expected predicate, clause evaluation rules and candidate carrier unchanged.
Storage and unrolling dimensions may follow the changed bound, but active-
count interpretation and positive-length search semantics do not change. The
population-minus-one control uses `N > 0`. Fixtures make the changed boundary
satisfiable; in particular a unique `U+1` edge witness needs at least `U+1`
active objects. Every run retains generated source, the observed domain
definition, backend property identity and FR-017 execution evidence. The
healthy run verifies both implications. Each mutant classifies `Falsified` at
the appropriate domain assertion and yields a typed witness containing the
original declaration/claim/edge identity, active population, per-row degrees
and targets, operands, original and generated bounds/capacities, and
independently checked `D` and `G` results. A separate carrier boundary-probe
run for each mutant has one final cover witnessing its required mismatch
below, without assuming either membership predicate; each of those covers is
satisfied with the installed backend:

- Population `N+1`: a graph with `N+1` active objects is admitted by `G` and
  rejected by `D`; this is a mutant-only conformance witness.
- Population `N-1`: an original-valid graph with `N` active objects satisfies
  `D` and is rejected by `G`; this is an omitted original-domain completeness
  witness, even when the smaller production array cannot encode it.
- Edge upper `U+1`: an otherwise valid graph with one active row of degree
  `U+1` is admitted by `G` and rejected by `D`; this is a mutant-only
  conformance witness, preserving original lower bound and uniqueness.

An array bounds fault, changed traversal limit, added dormant row/edge,
constant-false predicate, changed claim or an expectation copied from the
mutant does not qualify. Pure assumption narrowing need not change clause
truth on any retained graph. Mutant-only witnesses are not original-domain
clause counterexamples and earn no original-domain replay credit. The separate
semantic controls above still require original-domain playback and same-clause
`qsl_replay` agreement. Missing real-package population links or typed field
accessors retain AC-108's refusal; a test-only carrier or a relationship-end
success does not supply the missing field-edge producer/API evidence.

## Dependencies

- **Upstream**: [FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md),
  [FR-025](./FR-025-generated-subject-abi.md), [FR-028](./FR-028-bounded-proof-ceilings.md).
- **Downstream**: [TC-025](../matrix/TC-025-bounded-kani-obligations.md),
  [FR-016](../../replay/functional/FR-016-witness-native-replay.md),
  [FR-022](../../routed/functional/FR-022-routed-generation.md), whose Kani generation arm calls this generator.
