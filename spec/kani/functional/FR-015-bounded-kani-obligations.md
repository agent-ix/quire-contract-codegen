---
id: FR-015
title: "Generate separate bounded Kani obligations for complete-V1 oracles"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: depends_on
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
  produces, never drawn. A drawn value's declared domain is the object member's
  `integer_range` for a state field and the IR `bounded_domain` of its type for a
  parameter, never a caller descriptor; a produced value's domain is asserted, not
  assumed. The clause body is the node's Boolean condition, an inline term of the
  node, which may hold Boolean connectives, bounded-integer comparisons and the integer
  arithmetic expressions add, subtract, multiply and negate.
- The V2 census input: an optional declared proof-dependency census per V2
  obligation, carried by the same typed census request and readiness types the
  corpus uses, and not by `ProofDependencyGraph`.

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

## Dependencies

- **Upstream**: [FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md),
  [FR-025](./FR-025-generated-subject-abi.md), [FR-028](./FR-028-bounded-proof-ceilings.md).
- **Downstream**: [TC-025](../matrix/TC-025-bounded-kani-obligations.md),
  [FR-016](../../replay/functional/FR-016-witness-native-replay.md),
  [FR-022](../../routed/functional/FR-022-routed-generation.md), whose Kani generation arm calls this generator.
