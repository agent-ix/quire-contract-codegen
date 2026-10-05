---
id: TC-025
title: "Verify separate bounded Kani obligations"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: verifies
  - target: ix://agent-ix/quire-specification/TC-218
    type: references
---
# TC-025: Verify separate bounded Kani obligations

## Description

Verify that complete-V1 contracts produce one bounded harness per obligation
kind and refuse non-finite obligations.

## Test Procedure

Generate harnesses for a contract with a precondition, postcondition,
invariant and frame condition over bounded scalar domains, and for a contract
with an unbounded domain, one with unsatisfiable bounds, and one over a
caller-declared oracle operation. Inspect harness identities, bounds and
assumptions, and run the installed Kani backend on the bounded harnesses.

Additionally: count the covers in each generated harness and locate the
contract cover relative to the contract call; request a postcondition without
the precondition sharing its anchor; read the solver and the option vector out
of each identity; regenerate from equal inputs and compare bytes; regenerate
with a changed unwind bound and a changed subject and compare identities;
search each harness source for a loop bound; and submit a request with no
items, with more items than the ceiling, with an unparsable subject path, and
with an unwind bound on each side of the admissible range.

State clauses: generate the operation-contract and frame-effect harnesses of one
`postcondition` clause over one integer field; read each identity's scope, field
ranges, granted and forbidden fields and cover count; and submit a clause that is
a precondition, one comparing two reads of the same side, one comparing two
fields, a frame that creates or deletes an object or grants a relationship or a foreign field, a condition that is a negation, compares to a literal or reads through a second parameter, a clause field with no integer range,
a state without the clause's field, a frame granting every field, and an
unparsable path and an out-of-range unwind bound. Run the installed Kani backend
over a healthy subject, a subject mutated to debit, a subject writing a granted
field, a subject also writing an ungranted field, and the allowed subject against
a frame regenerated from a package whose `modifies` is emptied; then execute the
forbidden counterexample natively and replay it, and the allowed run, through
QSL's `replay_frame`.

## Expected Results

Four distinct harnesses carry their bounds; no assumption
excludes an undefined, refused or incomplete outcome; the unbounded, the
unsatisfiable and the caller-declared obligations are refused with no harness.

Each harness contains exactly one cover, and the contract harness's cover
follows its contract call. Each contract harness carries a `requires` for every
package precondition sharing its anchor and embeds no other obligation's
oracle; the postcondition requested without its sibling precondition is refused
naming that precondition, with no harness. Every identity records solver
`cadical` and the ordered option vector, and no option enables stubbing.
Regeneration is byte-identical, while the changed unwind bound and the changed
subject each produce a different identity. Every symbolic argument
carries an inclusive assumption equal to its IR domain, the post-state result
is required to lie in the same domain, and no harness source contains a loop
bound. Each of the four malformed requests is refused whole, with no item
accounted.

## Implementation

`tests/it/kani_obligations.rs`, and for state clauses `tests/it/kani_obligations_state_frame.rs`,
whose module name `make kani`'s `kani_obligations` filter selects. The default lane negotiates every item before
any harness is exposed and checks separate harnesses, IR-derived bounds,
assumptions and every refusal. `make kani` runs the ignored lane serially under
a host lock against the installed Kani backend: it verifies the precondition,
postcondition and invariant harnesses, falsifies a seeded postcondition defect
with a concrete counterexample, and reports a contract harness with jointly
unsatisfiable requires as `cover_unsatisfied`.

## State transitions, censuses and dispositions

1. Generate a zero-input transition over one bounded-integer state value, run it for an identity
   subject and for a subject that changes the value. The identity subject verifies, and the
   changing subject is falsified with a concrete counterexample (FR-015-AC-19).
2. Generate a harness for each Boolean connective and each integer comparison claim, and compare
   the embedded oracle with the FR-014 crate's function byte for byte. They are identical
   (FR-015-AC-20).
3. Request a precondition that reads a post-state value. It is refused naming the node, with no
   harness (FR-015-AC-21).
4. Request obligations with a declared census that is empty-identity, duplicate-identity,
   kind/state/path-inconsistent or non-`Required`. Each is refused with no harness (FR-015-AC-22).
5. Request items that settle `supported`, `requires-bound` and `unsupported` together. Each has
   exactly one `ObligationDisposition`, and each item that is not `supported` keeps its source
   identity and reason with no harness and no assumption (FR-015-AC-23).
6. Run a falsifiable plain bounded-integer comparison harness. The falsified run prints a concrete
   playback (FR-015-AC-24).
7. Request obligations with valid censuses whose dependencies are all passed, one missing and one
   failed. Each folds into the identity with readiness `ready`, `incomplete` and `incomplete`, and
   every harness records execution `not_run` (FR-015-AC-25).
8. Generate the operation-contract and frame-effect harnesses of one `postcondition` state clause
   over one integer field. Each carries one cover, an identity scoped to the operation, anchor,
   frame and object, and the IR range of each state field (FR-015-AC-26 to FR-015-AC-28). Each clause or frame
   shape outside that encoding is refused by name with no harness (FR-015-AC-29). The installed
   backend verifies the healthy subject, falsifies the debiting subject, verifies the granted write,
   falsifies the ungranted write naming its field, falsifies the allowed subject against a frame
   regenerated with nothing granted, and QSL's `replay_frame` reproduces the forbidden write and
   finds the allowed run inside its frame (FR-015-AC-30 to FR-015-AC-32). The default suite builds that
   replay's request and envelope through `qsl_replay::call_site` and settles a forbidden and a
   granted write without Kani, and refuses an operation with no frame, declared or not (FR-015-AC-33 to FR-015-AC-36).
9. Generate the scalar harness of each integer operation (add, subtract, multiply, negate). Each
   asserts the oracle's outcome against the operation evaluated natively in `i128`, with a `Completed`
   arm, a `Refused` arm and a failing catch-all. The installed backend verifies the healthy `x + 1`
   harness and falsifies it with the oracle's `Add` replaced by `Subtract` (FR-015-AC-37).

## V2 clause claims and the V2 census (planned, IR-489)

10. Request one V2 clause claim each over a `precondition`, a `postcondition` and an
    `invariant` node of an admitted `CheckedPackageV2`. Each yields one harness of its own
    kind with one drawn value per drawn input (the pre-state of every field the body reads, bare or `pre(...)`, and the
    operation parameters, bounded by their declared domains; the postcondition's result
    and post-state come from the subject call and their domain is asserted), and none takes a
    bound from a caller (FR-015-AC-38, FR-015-AC-39). The invariant harness asserts the
    clause with no subject call (FR-015-AC-49).
11. Generate a clause of connectives and comparisons and compare the embedded oracle with
    the FR-014 crate's function byte for byte (needs FR-014-AC-38); generate the
    arithmetic postcondition `amount < 1000` implies `amount + 1 <= 1000`, check the
    native `i128` assertion of each arithmetic subterm, and run the installed backend over
    the unmutated clause and over one whose `+` returns `left + right + 1` (expected
    playback `amount_current = 999`), and one whose `+` returns `left`, which is
    falsified on the native assertion (FR-015-AC-40,
    FR-015-AC-41). The package comes from QSL's facade on source, never a copied fixture
    (AD-004 step 4a); if the facade cannot build it, the real-Kani control is the
    quire-integration exemplar of AD-004 L-5, a test in a repository above both.
12. Request a clause with an unsupported operator (division, modulo, absolute value), a
    node that is not a modelled clause role and an absent node id beside a supported item.
    The operator is `unsupported`, the other two refused as FR-015-AC-14 states, each
    keeps its reason and gets no harness (FR-015-AC-42). A clause value outside the three
    kinds and a non-Boolean body cannot reach this arm on an admitted package (Contract IR
    FR-040-AC-8 and FR-040-AC-10 refuse them at admission), so that guard is unreachable
    and not tested here. Request a postcondition without its sibling precondition
    (FR-015-AC-43).
13. Declare the four invalid censuses of FR-015-AC-22 and the three valid censuses of
    FR-015-AC-25 on V2 clause claims, with the dependency list reordered. The invalid ones
    are refused; the valid ones fold into the identity and settle `ready`, `incomplete` and
    `incomplete`, and every harness records execution `not_run` (FR-015-AC-44,
    FR-015-AC-45).
14. Count the covers and locate each relative to the assumptions and the subject call; run
    a clause with jointly unsatisfiable assumptions; regenerate from equal inputs; vary the
    unwind bound, the subject, a parameter's domain and the source span: the harness
    identity record changes with the first two, the obligation identity with the domain
    and not with the span, unwind bound or subject (FR-015-AC-46 to FR-015-AC-48).
15. Feed `negotiate` a scalar refusal of each of `LoweringByteLimitExceeded` and
    `LoweringLimitUnrecognised` and read `OracleRefused` carrying it unchanged, field for field
    (FR-015-AC-50, IR-547).

## A cover in every harness kind (IR-464)

16. Generate a harness of each kind: precondition, V1 contract, scalar, state-clause,
    frame-effect and V1 bundle (the corpus kind is TC-023's). Parse each emitted source
    and read the last statement of every function attributed `#[kani::proof]` or
    `#[kani::proof_for_contract]`: it is the only `kani::cover!` of the body, and no
    assertion follows it (FR-015-AC-53, FR-015-AC-54).
17. Run the installed backend over a V1 bundle harness whose requires clause some
    bounded argument satisfies and whose `ensures` holds for every such argument, which classifies
    `Verified`; over the same bundle with its cover removed, which classifies
    `Inconclusive` as `MissingCoverSummary`; over the same bundle with a requires clause no
    bounded argument satisfies, which classifies `CoverUnsatisfied` (0 of 1 covers) and is
    neither `Verified` nor `Falsified`; and over the same bundle with a subject that breaks
    its `ensures`, which classifies `Falsified` (FR-015-AC-56).
18. The guard: drive every emitting entry point (precondition, V1 contract, scalar,
    state-clause and frame-effect families, `generate_kani_bundle` and the corpus generator for each family) through the
    inspection of item 16, and scan the non-test string literals of `src/` for spellings of
    a proof attribute: `#[kani::` other than `requires`, `ensures`, `stub` and `unwind`,
    `kani::proof`, and `proof_for_contract`, with `\` line continuations joined, so a
    `format!` template, a split `concat!` and a continued literal are seen. The files that
    spell one are exactly the files the inspection drives, each with exactly the number of
    templates the guard counts, so a new emitter file and a new template in a driven file
    fail it until driven. A spelling built from fragments none of which holds `#[kani::` or
    `kani::proof` is not seen (FR-015-AC-58).

## A disposition for every state clause (IR-461)

19. Negotiate one request of `StateFrame` items over one admitted QSL-emitted package with
    selected model declarations: first a `precondition` clause (refused), then one clause in
    both roles that yields, one whose lowering is a `RequiresBound` record, one whose selected
    model field has no representable `i64` range, one comparing two fields, one whose frame
    grants a relationship and one whose frame grants every field. The `RequiresBound`
    case must come from lowering in that same admitted package; a separate request or
    a body-member target does not satisfy this mixed-disposition check. Read one record
    per item in request order, `kind`
    `postcondition` or `frame`, and each later record equal to the record the same item has in
    a request of that item only; repeat with three refused items and read three records. An
    implementation that stops at the first refusal reads one record and fails
    (FR-015-AC-59).
20. Compare the `supported` records' harnesses, read from `state_frame_harnesses` in request
    order, with `generate_state_frame_role` for the same clause and role (and, for a clause
    whose roles both succeed, `generate_state_frame_obligations`), source byte
    for byte (FR-015-AC-60).
21. Read `requires_bound` and the record's `unbounded_type` for a request whose lowering is
    a requires-bound record. For a selected model declaration's clause read of a native
    `Integer` field, verify lowering's `RequiresBound` wins before accessor resolution.
    For a present listed `Integer` field not read by that clause, verify value-based
    `NonRangeType` only if the item reaches accessor resolution. For a non-model or
    admitted unselected model/object_type body, read `ModelFieldsUnavailable` without a
    fabricated `unbounded_type` or a body range (FR-015-AC-61, FR-015-AC-78).
22. Read `unsupported` `NoFiniteEncoding` for a lowering unsupported-family record and
    `UnknownNodeKind` for a node that is not a `state_clause` (FR-015-AC-62).
23. Read `unsupported` `StateFrameRefused`, never `NoFiniteEncoding`, for a negation, a literal
    comparison, an operator outside the six comparisons, a read through another parameter, two
    fields, two reads of one side, a frame that creates, deletes or grants a relationship or a
    foreign field (naming the effect), an invariant clause, a clause with a malformed shape,
    a frame granting every field and an over-budget lowering. For a selected model
    declaration, check an accessor-absent field, a non-range member type, and an endpoint
    outside `i64`; for a non-model object and an admitted unselected model/object_type
    with a nonempty body, check `NotModelObjectType` and no harness. These replace the old body's
    `NotIntegerRange`, `EndpointOutsideI64`, `MemberAbsent` and `ValueNotReference`
    probes (FR-015-AC-63, FR-015-AC-78).
24. Read `invalid_request` for an unparsable state path, an unparsable item subject path, an
    invalid state field name, a state lacking the clause's field, an absent node, a repeated
    item and an item of another package, and `Rejected` with every record and no harness bytes
    (FR-015-AC-64).
25. Negotiate no items, `MAX_OBLIGATION_ITEMS` plus one, an unparsable request subject path and
    an unwind bound outside the range, each with `StateFrame` items present or absent as the
    case allows, and read the existing `KaniObligationError` variant with no record
    (FR-015-AC-65).
26. Build every retained `StateFrameRefusal` variant and call the mapping; read the table's
    record for each. Include `ModelFieldsUnavailable` for `UnknownNode`,
    `NotModelObjectType` and `AmbiguousField`, `MemberAbsent`, and
    `ModelMemberNotI64Range` with `NoMemberType`, `NonRangeType` and
    `EndpointOutsideI64`, plus each of the six `NotLowered` refusal arms. Do not construct
    retired body-member `BoundNotResolvedCause` cases. Build `UnwindOutOfRange`,
    `InvalidGeneratedSyntax` and `RecordSerialization` directly, and reach
    `ResourceLimitExceeded` with a state-field list that passes the 1 MiB ceiling or build
    it directly. Inspect that the mapping is a `match` with no wildcard arm (FR-015-AC-66).
27. Call the single-clause engine on a clause whose graph field name is not a Rust identifier
    and on a request whose caller-supplied field name is not an identifier: read
    `MalformedClause` and `InvalidField` (FR-015-AC-67).
28. Negotiate a clause whose frame grants every field in both roles, one whose condition is a
    negation in both roles, and one whose lowering is refused in both roles: read `frame`
    `unsupported` with `contract` `supported`, `contract` `unsupported` with `frame`
    `supported`, and both alike, comparing each `supported` harness with
    `generate_state_frame_role` and reading `generate_state_frame_obligations` return the
    refusal of the failing role (FR-015-AC-68).

The first code change backs steps 26 and 27, and the second steps 19, 21 to 25 and, apart from
one comparison, steps 20 and 28; both have landed. The comparison against
`generate_state_frame_role` is not testable from `tests/it` (crate-private) and no V2 package
builder exists under `src/`; byte identity holds by construction. Steps 20 and 28 therefore
compare the harnesses with `generate_state_frame_obligations` where a clause's roles both succeed
and read the refusal it returns where one fails, and their tests carry no trace tag of
FR-015-AC-60 or FR-015-AC-68, which stay planned until that comparison has a test. The tests are in
`tests/it/kani_obligations_state_frame.rs`; the package they negotiate holds twenty-four shapes,
each over nodes of its own and read under a byte ceiling above the default, which the corpus
package alone nearly fills.

## Composite equality over a bounded shadow (planned, IR-264, IR-241)

29. Request a composite-equality claim over parameters of one record type, one tuple type and one
    option type (Boolean and bounded-integer leaves, an optional field, a nested option), under
    `Equal` and `NotEqual`: read one harness per item with family `composite_equality`, proof subject
    `bounded_shadow` and a refinement obligation in the same result, distinct from every other
    item's, with the operator and operand types from the descriptor and no bound from it
    (FR-015-AC-69).
30. Read the harness's drawn values (a `bool` or `i64` per leaf, the presence draws, the
    `present`/`absent`/`null` draw of an optional field), the assumption on each integer leaf, the
    pinned leaves of a literal operand, the order of the arguments by parameter node id then leaf
    path with each bound to both, and that no composite is an argument (FR-015-AC-70).
31. Read the harness's assertions against the expectation computed from the drawn values (verdict
    and pair count, `absent` against `null`, two `none`, `none` against present, a `some`/`some`
    option, no early exit, the negation under `NotEqual`) and that it ends with one cover after
    them (FR-015-AC-71).
32. Parse the shadow and the harness for loops, recursion and heap types; generate a closure one
    pair node under, at and one over `SHADOW_PAIR_NODE_BUDGET` and read the identity's budget and
    count (FR-015-AC-72).
33. Request one item reaching each row of the unsupported-shape table, beside one supported item,
    and read each disposition and reason, that no harness exists for the refused ones, and that
    the supported item settles as it does alone (FR-015-AC-73).
34. With the installed backend, run the corpus case's harness and its refinement run: read
    `Verified` with its cover satisfied inside the identity's ceilings, the versions and options in
    the evidence, and the refinement class `exhaustive` with its case count (the case's domain fits
    the cap) and agreement; run a second case whose domain exceeds the cap and read `sampled`
    (FR-015-AC-74).
35. With the installed backend, run each of the six shadow mutants, and natively each of the five
    production mutants (opposite operator, `absent` equal to `null`, one pair too many, an optional
    field declared required, a legal pair `Refused`) and the unmutated case: read `Falsified` with
    a playback for each shadow mutant, `refinement_failed` naming the first disagreeing case for
    each production mutant, and `Verified` with an agreeing refinement for the unmutated case. Run
    the closure-reader mutants (a dropped member, a bound narrowed by one, an optional member read
    as required) against FR-018-AC-23's independent check, which they fail, and permute the member
    order and read that no check is required to fail (FR-015-AC-75).
36. Read the identity's closure, domains, operator, budget, count, abstractions and unexercised
    behaviours with each covering criterion's backing state against the matrix rows, change each
    of the closure, a bound, the operator and the budget in turn, and regenerate from equal inputs
    (FR-015-AC-76).

Steps 29 to 36 are planned. Nothing in `src/` renders a composite harness: the corpus families
(arithmetic, graph, collection) draw no input, FR-025's argument table has rows for Boolean and
bounded integer only, and composite equality has an oracle (FR-018) and a native agreement corpus
(TC-029) but no Kani harness. The value of `SHADOW_PAIR_NODE_BUDGET` and the case cap's default are
set by the code change's measurement and recorded here.

## State field ranges from the package QSL emits (planned, IR-624)

37. Call `model_object_fields(&object_id)` for the QSL-emitted twin's model declaration
    object and read both `balance` and unread `audit` as `IntRange { lower: 0, upper: 1000 }`;
    convert their `i128` endpoints to `i64`, and compare the resulting `domains` with the
    state-clause replay's declared ranges. Admit a second unit with the same declaration but
    no read of `audit` and observe its range unchanged; cite IR FR-038-AC-140 for the
    independent post-admission body mutation (FR-015-AC-77; emitted path awaits CG's dependency
    update and implementation).
38. For a model declaration, request an accessor-absent field; exercise each accessor error
    (`UnknownNode`, `NotModelObjectType`, `AmbiguousField`) and check a typed refusal without
    falling back to a body or read. Exercise a present `None`, unread `Integer`,
    `Option`, and an `IntRange` with one endpoint outside `i64` in items whose
    lowering succeeds; read `ModelMemberNotI64Range` with the object,
    field and `NoMemberType`, `NonRangeType` or `EndpointOutsideI64` (exact `i128` endpoints)
    rather than a fabricated bound node. Check that only a present range with two `i64`
    endpoints enters `domains`, an unranged non-clause field is still drawn, and an unranged
    clause field refuses its item as `unsupported`, `StateFrameRefused`. For a non-model
    object and an admitted unselected model/object_type with a nonempty body, verify
    `NotModelObjectType` refuses without any assumption, `domains` entry or harness, even
    when the body contains an apparent range. Use an admitted QSL-emitted graph with
    selected-model-document override and recomputed digests for `None` and out-of-`i64`
    type cases QSL cannot emit directly. Separately submit a selected model/object_type
    whose nonempty body is read by the state clause and verify IR admission refuses
    `StaleNodeKey` before CG receives a package; do not expect a CG `NotModelObjectType`
    result from that rejected input (FR-015-AC-78; IR TC-227).
39. Generate both roles of each twin clause from the emitted package with no `Twin::aligned`;
    assert request order rather than accessor name order for `state_fields`, accessor ranges
    for `domains`, and scope ids equal to `call_site`'s. An absent accessor field, an omitted
    clause field and an omitted granted field each refuse (FR-015-AC-79).
40. With installed Kani, run the healthy, debiting, granted-write and forbidden-write subjects
    against harnesses generated from the emitted package (FR-015-AC-80).
41. Generate a harness with an unread present ranged field, and another with a present
    non-range field; check the first has a `domains` entry and no unranged reason, the second
    has no range assumption and records `TypeNotRange` in identity and persisted record.
    Regenerate for byte identity, and reject a record naming a field twice (FR-015-AC-81).

Steps 37 to 41 are planned. The IR-628 accessor has merged; the QSL-emitted model declaration
path awaits CG's dependency update and implementation. The hand-built non-declaration
body-member range path is retired; an agreeing read does not make that body trusted. The code change
replaces `tc_035_the_generator_reads_no_field_range_from_the_object_shape_qsl_emits`, updates
the typed accessor refusal and AC-66 mapping tests, and edits or supersedes the test of
FR-024-AC-30 that calls `Twin::aligned`; this PR restates AC-30 over the emitted-package
harness, so its previous test evidence remains planned until that migration lands.

## Blocked

- Frame harnesses in the clause negotiation: FR-025 accounts every frame obligation
  `unsupported` there until QSpec decides how a frame node lowers into a Kani form (ADR-004). The
  frame of a `postcondition` state clause over one integer field is instead generated with that
  clause by `generate_state_frame_obligations` (FR-015-AC-26 to FR-015-AC-32).
- V2 scalar harnesses outside `IntegerArithmetic`: an IR-confirmed claim over
  one of the four `quire.op.integer.{add,sub,mul,negate}` identities reaches a
  real harness unless a ground independent of the operation (an
  i64-unrepresentable endpoint, the source ceiling) displaces it. Every other
  confirmed family is refused as `OperationNotRendered`, an unbuilt renderer in
  this generator, not an upstream block, and so is a claim of a rendered family
  from a claim map this generator did not produce (NFR-005-AC-2). A claim this generator lowered but
  whose operation it did not confirm against the node's own catalogued
  identity, mode or law definition is refused as `CallerDeclaredOperation`.
- Model and graph bounds: refused as blocked on `agent-ix/quire-spec-language#120`.
