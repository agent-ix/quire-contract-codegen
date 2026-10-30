---
id: FR-024
title: "Submit every counterexample to QSL replay in QSL's counterexample envelope"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-016
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-025
    type: depends_on
  - target: ix://agent-ix/quire-spec-language/ADR-013
    type: references
  - target: ix://agent-ix/quire-spec-language/ADR-015
    type: references
  - target: ix://agent-ix/quire-spec-language/ADR-017
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-116
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-071
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-098
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-122
    type: references
  - target: ix://agent-ix/quire-specification/FR-197
    type: references
  - target: ix://agent-ix/quire-specification/FR-312
    type: references
  - target: ix://agent-ix/quire-specification/FR-323
    type: references
---
# FR-024: Submit every counterexample to QSL replay in QSL's counterexample envelope

## Description

When a counterexample is to be replayed, the code generator shall carry it to QSL as QSL's
counterexample envelope (`qsl_replay::WitnessEnvelope`), keyed by QSL's `ObligationIdentity` and
holding exactly one `qsl_replay::ReplaySource` arm. It shall validate every value against its
declared domain before any replay, and shall replay only through `qsl_replay::replay`, or through
`qsl_replay::replay_frame` for a frame counterexample, or through
`qsl_replay::replay_state_clause` for a postcondition state-clause counterexample.

The envelope, the witness, the replay source, the FR-331 terminal record and the obligation
identity type are QSL's, as the `qsl-replay` API defines them.
[AD-001](../../assurance/AD-001-codegen-architecture.md) states that ownership and the upstream
records that still differ from it.

This requirement states what the generator puts into those types, and does not restate their shape.
[FR-016](./FR-016-witness-native-replay.md) owns decoding a Kani playback, the adapter refusals and
the partition of replay outcomes. This requirement owns what reaches QSL.

For every counterexample, the packet's replay target follows QSpec FR-323:
exactly one of `function`, `frame_operation` or `state_clause`, with the other two absent.
`function` holds the selected function's
`QualifiedName`. `frame_operation` holds `{object, operation}`: `object` is the
`QualifiedName` of the object type, process or state machine that declares the
operation, and `operation` is its `Identifier`. CG obtains that pair from QSL's
typed call-site result after it resolves the operation against the checked package,
including an inherited operation's declaring owner. The caller's `OperationName`
selects the site but is not authority for the declaring owner. A frame packet carries
`frame_operation` and no `function` or `state_clause`. A state-clause packet carries
`state_clause` as the clause's own declared `QualifiedName`, including for an operation's
postcondition, and no `function` or `frame_operation`. Its payload carries the observation
and no clause selection. The old `selected_function` member has no alias or fallback.
The frame packet still
carries the frame and generated-occurrence identities that ADR-017 PF-4 assigns
to the envelope and payload; the *replay request* carries no `arguments` or
`occurrence_key` for a frame.

For a `function` replay request, CG carries the admitted packet's complete
`occurrence_key` into the request unchanged. A function-contract predicate packet carries
the selected function's `declaration` occurrence. An operator-parity packet carries its
own scalar application's occurrence, while a value-parity packet carries its own
function `declaration` occurrence. The key selects the claim under QSL FR-098; the
`function` target selects its owner, and `obligation_identity` remains independent
provenance. CG does not infer the key from the function name, node alone or obligation
identity, and does not mint a substitute or default its role or ordinal. This rule adds
no `occurrence_key` or `arguments` to a frame or state-clause replay request.

The function-contract and parity entry points receive a completed
`WitnessPacket<NoPayload>` from the producer of the replay evidence, before
request construction. For a falsified function contract the backend adapter
completes that packet from the retained proving-run members and its decoded
witness. For an input-arm parity claim the claim producer completes the packet
from the original checked claim and its input source; no backend witness is
implied. These packets carry QSL's `function` target, their own occurrence key,
subject node, package identity, obligation identity and source. CG admits each packet through
`WitnessEnvelope::reconstruct`, then checks the admitted target, full key, subject node, package
identity, source arm and obligation identity against the independently selected function or
checked claim.
The expected claim is taken from the admitted package and retained proving-run
identity for a function, or from the original checked parity claim; it is not
read back from the packet being checked. The request builder
receives the admitted envelope and request-only members. It cannot accept a
bare `OccurrenceKey`, `FunctionSite` or caller-supplied parity claim as a substitute for
the packet. A packet with a different occurrence of the same node is a
different claim: CG shall accept it only when that occurrence belongs to the
selected claim, and shall otherwise refuse it before request construction.

The common intake takes a typed outcome from any backend adapter: a successful submission or a
decode refusal. It owns admission and lineage across adapters; each adapter owns its backend-native
text and decoding. A CG-owned
immutable lineage record refers to, but is not a member or copy of, QSL's envelope.

## Scope

The state-clause path covers the `postcondition` state clause of one operation over one state
object (the operation-contract harness of FR-015-AC-26), replayed as QSL's `Invocation`
observation. State preconditions (`PreCall`) and invariants (`Current`) have no CG harness
obligation of this kind and are not specified here.

The frame path (FR-024-AC-6) and the state-clause path fill the same `WitnessPacket` members from
the same proving-run members. The state-clause code keeps its packet assembly inside its own
module and does not edit `src/replay/frame.rs`. Whichever of the frame-envelope code and the
state-clause code lands second extracts the shared packet-assembly piece, in its own change; the
one that lands first leaves it local.

The frame path's obligation identity, witness and pre-state tie are specified by FR-024-AC-47 to
FR-024-AC-30 (IR-459). That slice is the single-`self` frame harness only: its only symbolic
quantities are fields of the one state struct and it has no parameters, so the `arguments` of its
identity are empty, and its `declared_domains` stay empty (FR-024-AC-29). The postcondition state-clause path's
caller-supplied `obligation_identity` is outside that slice; the one identity function accepts a
`postcondition` harness's `StateFrameIdentity` only once its field list is settled (Open
questions, Q-1).

The state-clause path supports an operation that declares no parameter and no result, the shape
of the single-`self` operation-contract harness. An operation that declares either is refused up
front (FR-024-AC-19), because QSL checks the invocation document's `parameters` and `result`
members against the operation's declaration in the domain package.

## Inputs

- A typed adapter submission containing parameter-node-keyed assignments, the producing backend's
  identity, QSL's opaque `ObligationIdentity`, each binding's declared domain, the source arm and
  its provenance, and the replay packet members supplied by the proving run. A
  backend witness submission carries decoded values and the adapter's rendered transcript.
  Backend-native text is consumed only inside its adapter and never crosses this boundary.
- The adapter outcome is either that successful typed submission or a typed decode refusal
  containing the producing backend identity, opaque obligation identity and adapter-owned decode
  cause. The refusal arm contains no assignments, replay packet or backend-native text. FR-016
  defines Kani's decode causes and its mapping into this arm.
- A backend counterexample: an [FR-017](../../kani/functional/FR-017-kani-execution-evidence.md)
  `falsified` run's playback, decoded by FR-016 into values named by their argument bindings.
- The obligation's `KaniObligationIdentity` and its argument bindings, each naming its parameter
  node id and declared domain ([FR-025](../../kani/functional/FR-025-generated-subject-abi.md)).
- For a backend falsification, the proving run's envelope members, supplied by the adapter:
  every `WitnessPacket` member QSL defines, including the trace position (present in every
  packet, with the value none for a family that has no trace position). For input-arm parity,
  the claim producer supplies those members from the original checked claim and its retained
  input/source context; there is no backend proving run or witness transcript for that arm.
- For a function-contract or parity request, the producer's completed
  `WitnessPacket<NoPayload>` is a single input to CG's packet-admission
  boundary. A function adapter supplies its decoded witness arm; a parity
  claim producer supplies its input arm. The independently selected claim and
  request-only members are separate inputs to that boundary. Neither request
  builder accepts the occurrence key separately.
- The request-only members from the proving run for a falsification, or from the original checked
  claim context for input-arm parity, which belong to the replay request and not to the envelope:
  the dependency entries (QSL ADR-015 D-4), the state environment, the accounting limits, the S1 to
  S4 stage limits and the byte provision.
- For a postcondition state-clause counterexample, the proving run's byte provision is the
  caller's domain package documents (as `FrameReplayInputs::packages`) and nothing else. The
  invocation and snapshot documents are not supplied as bytes; the generator builds them from:
  - the decoded playback of the falsified operation-contract run: one `i64` per state field, the
    symbolic pre state the harness drew;
  - the post state: one `i64` per state field, which the caller obtains by running the customer's
    subject natively over that pre state. The generator links no customer code, as on the frame
    path where the caller supplies the post snapshot;
  - the state object's address (population, object key and object type), the `OperationName`, and
    one identity label (authority, identity, revision namespace and revision) for each of the
    invocation and its two snapshots, all supplied by the caller;
  - each state field's declared integer range (FR-015-AC-27; for the package QSL emits,
    FR-015-AC-77, planned), and the operation's declared parameters and result, read from the
    admitted package and the clause's node, which the caller supplies.
- For a frame counterexample (IR-459; typed intake implemented in IR-1086): the falsified harness's
  `StateFrameIdentity` and parsed `Playback` from the falsified outcome's complete counterexample
  text, both read by the generator, and the requested decode limits (bounded by the proving run's
  `run.decode_limits`), the proving run's members, the
  operation, the invocation document reference, the pre, post and invocation documents and the
  claimed change, which the caller supplies. The caller supplies
  no obligation identity and no witness transcript.

## Outputs

- A `WitnessEnvelope` admitted through `WitnessEnvelope::reconstruct`, whose `ReplaySource` is
  `Witness` for a backend counterexample.
- A typed evidence failure naming decode, out-of-domain, QSL admission or replay-verdict
  disagreement when that stage refuses; none is a contract success or contract failure.
- For each retained minimization revision, a CG-owned immutable lineage record referring to its
  exact QSL envelope, its obligation and its parent revision (except the original root).
- The replay request built from that envelope and the request members.
- The QSL replay result for the envelope's arm, or a typed refusal with no replay.
- For a postcondition state-clause counterexample: a `StateClauseReplay` holding the request and
  the envelope packet, whose `replay` returns QSL's `StateClauseReplayResult`, or a typed
  `StateClauseReplayError` with no replay.

## Behavior

- The common intake shall accept only a typed adapter outcome.
- When an adapter submits success, it shall include decoded, parameter-bound assignments, backend
  identity, opaque obligation identity, declared domains, source/provenance and replay packet
  members.
- When an adapter submits a decode refusal, it shall include only backend identity, obligation
  identity and its typed decode cause.
- The adapter shall keep native counterexample text inside its own boundary.
- The intake shall preserve a successful submission's backend and source identities in the
  completed QSL packet. It shall not parse backend-native text or recompute an obligation identity.
- When an adapter outcome is a decode refusal, the common intake shall return a typed decode
  evidence failure retaining the adapter's cause, without building an envelope or calling native
  replay. FR-016 determines Kani's decode cause and refusal category.
- When preparing native replay, the intake shall check every assignment against its corresponding
  declared domain and admit the completed packet through QSL's `Witness::parse` (for a Witness
  source) and `WitnessEnvelope::reconstruct`. A failed check shall stop before replay.
- The intake shall report decode, out-of-domain, QSL admission and replay-verdict mismatch as
  distinct typed evidence failures, with the original cause retained. The intake shall not rewrite
  any of them as contract success or contract failure. FR-016 owns the Kani-specific decode and
  refusal mapping; this common rule applies to submissions from every backend adapter.
- The generator shall carry QSL's producer-issued structural `ObligationIdentity`
  (`NodeKey`, optional owning `ModelIdentity`, producer-assigned ordinal) unchanged into the
  envelope and request, without deriving or hashing an identity from the transcript,
  occurrence key, obligation kind or arguments. It shall compare those latter record members,
  including ordered argument domains, against the selected claim before replay (AD-003 E-1;
  QSL ADR-013 O-09 corrected at 878381034d5ddda662def7cffe0096d0e3c5f0cb).
- The generator shall build QSL's backend-witness transcript from FR-016's decoded values, never
  from backend-native text, in exactly one function of the backend's adapter.
- The generator shall admit that transcript only through `qsl_replay::Witness::parse`.
- When a counterexample is to be replayed, the generator shall check every decoded or canonical
  value against the declared domain of the parameter it binds before it builds a replay request.
  For a state-clause counterexample the value is a state field, bound through `self`. Its
  declared domain is the shared `field_range` result from the selected model declaration's
  accessor (FR-015-AC-77, planned IR-624). A non-model object or admitted unselected
  model/object_type body is refused before a range or replay request is built
  (FR-015-AC-78). A selected/read nonempty-body tamper is refused by IR admission as
  `StaleNodeKey` before replay receives a package.
- If a value lies outside its declared domain, then the generator shall report an out-of-domain
  counterexample and shall not call `replay`, `replay_frame` or `replay_state_clause`.
- For a backend falsification, the generator shall fill every common required `WitnessPacket`
  member from the proving run's envelope members and the obligation identity. For input-arm
  parity, it shall fill them from the original checked claim and retained input/source context.
  It shall invent none. It shall fill exactly one
  replay-target member: `function` on a function packet, `frame_operation` on a frame packet,
  using the validated declaring object and operation returned for that frame by QSL's typed
  call-site result, or `state_clause` on a state-clause packet, using the clause's declared
  `QualifiedName` after QSL's clause call-site validates that selection.
- If QSL cannot return a frame operation's validated declaring owner and identifier, then the
  generator shall refuse that frame packet rather than derive the owner from the caller's
  selection. If QSL cannot validate a state clause's declared name, then the generator shall
  refuse that state-clause packet rather than infer the name from its payload.
- For every packet, the generator shall refuse a packet with none of the three replay-target
  members or with more than one as distinct typed packet-admission failures before any replay.
  It shall
  admit neither an empty placeholder nor an old `selected_function` alias as a selection.
- The generator shall admit the envelope only through `WitnessEnvelope::reconstruct`.
- If `WitnessEnvelope::reconstruct` refuses the packet, then the generator shall return QSL's
  `WitnessRefusal` and shall not call `replay`, `replay_frame` or `replay_state_clause`.
- The generator shall build the replay request from the admitted envelope and the request-only
  members of the proving run or original checked parity claim context, and shall invent no
  request member. The state-clause path's invocation and
  snapshot documents are the one addition to the byte provision, built as the state-clause
  bullets below state, and no other document is added.
- When a function-contract or parity replay is requested, the generator shall
  require the producer's completed `WitnessPacket<NoPayload>` at the intake
  boundary, admit it through `WitnessEnvelope::reconstruct`, and pass the
  admitted envelope to the request builder. It shall refuse packet omission or
  QSL packet admission failure before building a request or invoking replay.
  It shall not complete a packet inside the request builder from a call site,
  claim preimage or caller-supplied bare occurrence key.
- When a function-contract or parity packet is admitted, the generator shall compare the admitted
  packet's `function` target, complete occurrence key, subject `clause_node`, `package_id`,
  source arm and opaque obligation
  identity with the independently selected function-contract, operator-parity or
  value-parity claim, whose expected identity is established before reading the
  packet. A mismatch shall return a typed claim-binding refusal that identifies
  the mismatched member and shall invoke neither request construction nor
  replay. A falsified function contract requires QSL's `Witness` source arm; operator and value
  parity require its `Input` arm. For operator parity, the selected occurrence is the checked scalar
  application; for function contracts and value parity, it is the selected
  function declaration. Distinct admitted occurrences of one node remain
  distinct claims.
- For a function-contract or parity replay request, the generator shall copy the admitted
  envelope's complete occurrence key, replay target and obligation identity
  into the request without changing the key's node, role or ordinal. When the
  packet key is missing, QSL packet admission shall refuse. When the request key
  is missing, QSL's request reader shall refuse at decode before replay. The
  generator shall not supply a declaration key as a default for an absent or different key.
- When a counterexample is a frame counterexample, the generator shall submit a
  `WitnessEnvelope<FrameCounterexample>` through `qsl_replay::replay_frame`.
- The generator shall carry the QSL producer's structural obligation identity for function
  and frame paths without minting another value. A frame's node is the `OperationSite` frame
  node, its optional model owner is the producer's, and its ordinal is assigned by the producer
  for that node/model pair. The envelope's `clause_node` and `occurrence_key` remain the
  `OperationSite` frame and frame occurrence. The harness property must be `frame`, and the
  frame obligation record has an empty argument list; a mismatching property, occurrence,
  kind, field set or declared domain refuses by its existing typed cause before replay, even
  when structural identity is unchanged. State fields and ranges are compared to the admitted
  record and replay context, not placed inside `ObligationIdentity`.
- The generator shall derive no node id and accept no caller-supplied obligation identity
  for the frame path (FR-024-AC-24, replaced by AC-51).
- If the harness's property is not `frame`, then the generator shall return
  `FrameReplayError::NotAFrame` and shall not call `call_site` or `replay_frame`.
- If the harness's granted and checked fields are not exactly its state fields, then the
  generator shall return `FrameReplayError::FieldSetMismatch`, before `call_site` is called
  (FR-024-AC-22).
- The generator shall record in `StateFrameIdentity` and its persisted record every state field
  the harness draws, in draw order, as `state_fields` (FR-024-AC-23).
- The generator shall decode the falsified run's playback only through the one decoder of
  `src/replay/witness.rs`, against the harness's state fields in the order the harness draws them
  (FR-024-AC-25).
- The generator shall render the frame witness transcript only through the one adapter rendering
  function, from the decoded values (FR-024-AC-25).
- If the playback does not decode against the harness, then the generator shall return a typed
  refusal and shall not call `call_site` or `replay_frame` (FR-024-AC-25).
- If a decoded value lies outside its field's declared domain, then the generator shall return
  `FrameReplayError::OutOfDomain` and shall not call `call_site` or `replay_frame`
  (FR-024-AC-26).
- If the pre snapshot that the invocation document names does not hold, for every state field,
  the value the playback decoded for it, then the generator shall return
  `FrameReplayError::PreState` naming the field and both values, and shall not call
  `replay_frame` (FR-024-AC-27).

  A document that is absent, or not of the shape the state-clause path writes, is the same
  refusal, and FR-029 reads it as `Failed`: the playback comes from this generator's own harness
  and the documents from the driver that builds the request, so a pre state the tie cannot read
  is a defect on this side of the seam, not a QSL refusal on data, and no QSL code is supplied for
  it. The tie compares document values after request decode and adds no document digest check.
- If the harness's operation, anchor or frame is not the operation, anchor and frame that
  `call_site` names, then the generator shall return `FrameReplayError::ScopeMismatch` naming the
  member (FR-024-AC-28).
- The generator shall leave the frame envelope's `declared_domains` empty and build no
  `DeclaredDomain` for it until QSL-345 settles the key and refuses an empty declaration
  (FR-024-AC-29).
- When a counterexample is a postcondition state-clause counterexample, the generator shall
  submit a `WitnessEnvelope<StateClauseCounterexample>` through
  `qsl_replay::replay_state_clause`. It is the falsification of the operation-contract harness of
  one `postcondition` state clause (FR-015-AC-26).
- The generator shall expose the call as `StateClauseReplay::replay_through`, which hands the
  built request and envelope to a caller-supplied executor, and `StateClauseReplay::replay`, which
  is `replay_through` with `qsl_replay::replay_state_clause` as the executor, as
  `replay_counterexample_through` does for the function path.
- The generator shall return the executor's result unchanged.
- The generator shall set the state-clause packet and request's `state_clause` replay target to
  the clause's declared `QualifiedName` that QSL's `ClauseSite` validates. It shall set no
  payload `clause` member or duplicate selection.
- The generator shall set the payload's `observation` to `ClauseSelectionInput::Invocation`.
- The generator shall submit the state-clause envelope on the `Witness` arm, with a transcript
  rendered by the one adapter rendering function (FR-024-AC-2) from the decoded playback values,
  and shall set the payload's `witness` record to none. The counterexample is a backend
  counterexample, so the `Input` arm is not used. QSL reads neither the transcript nor the
  `Input` assignments for this payload (QSL FR-122), so the arm selects only the result's arm.
  A clause whose settlement basis is decisive makes QSL re-derive a witness record the payload
  does not carry, and the result is then QSL's `inconclusive` with cause `Witness`, which the
  generator returns as it is.
- The generator shall set the envelope's `clause_node` to the `ClauseSite` node that
  `qsl_replay::call_site` names for the clause.
- The generator shall set the envelope's `occurrence_key` to the `ClauseSite` occurrence that
  `qsl_replay::call_site` names for the clause. The payload carries neither identity (QSL
  FR-122), so there is no second copy to keep equal.
- The generator shall set the envelope's `declared_domains` to one `DeclaredDomain` per state
  field, from the field's declared integer range.
- The generator shall build the pre snapshot from the decoded playback.
- The generator shall build the post snapshot from the post-state values the caller supplies.
- The generator shall give each snapshot one population marked `complete` that holds the one
  state object, which is the whole of the population the single-`self` harness draws, with every
  declared field of that object as an integer value.
- The generator shall build the invocation document with `format` `quire.state.invocation/v1`,
  the caller's identity label, the model header of the supplied domain package, `context` the
  object type, `operation` the operation's name, `self` the object's population and key, `pre` and
  `post` each the snapshot's identity and `sha256-jcs` digest, `parameters` empty, `result` null
  and `created` and `deleted` empty. `parameters` and `result` are empty and null because the
  supported operation shape declares neither (the Scope paragraph); `created` and `deleted` are
  empty because the frame of a supported clause creates and deletes nothing (FR-015-AC-29).
- If the operation the clause anchors declares a parameter or a result, then the generator shall
  return `StateClauseReplayError::UnsupportedOperationShape` carrying the operation and what it
  declares, before it builds any document. The shape is read from the caller's admitted package,
  while QSL's admission reads the documents provided in the byte provision.
- If the generator returns `StateClauseReplayError::UnsupportedOperationShape`, then it shall not
  call `replay_state_clause`.
- The generator shall encode each document and compute its `sha256-jcs` digest only through
  `core::canonical` (AD-004), the one RFC 8785 encoder, which gains the function that returns the
  encoded bytes beside `content_digest`.
- The generator shall add no second encoder, no hand-written key sorting or string escaping and no
  hashing outside `core::canonical` to the state-clause module, and no hashing dependency to
  `[dependencies]`.
- The generator shall take the state fields in input order and validate each against the
  selected model declaration's accessor. It shall refuse an object the accessor rejects,
  including a non-model object or an admitted unselected model/object_type with a nonempty
  body, without reading that body's members or calling replay (FR-024-AC-31). IR refuses
  a selected/read nonempty-body tamper as `StaleNodeKey` before this path runs.
- Planned (IR-624, FR-024-AC-31): for a QSL-emitted model declaration object with an empty body,
  the generator shall take its state field names and draw order from the `state_fields` member of
  `StateClauseReplayInputs` and check each name against
  `CheckedPackageV2::model_object_fields(&object_id)`. It shall read each present field's range
  from that field's `CheckedMemberType::IntRange` values through the shared reader of
  FR-015-AC-77, never from a read's `result_type`. A present field with no representable `i64`
  range remains unranged. For this model-declaration route, the generator shall return
  `StateClauseReplayError::ModelFields { object, cause }` when the accessor call fails or a listed
  field is absent from its returned table. The new `StateClauseModelFieldsCause` is
  `Accessor(CheckedModelFieldsError)` for
  `UnknownNode`, `NotModelObjectType` or `AmbiguousField(name)`, preserving that error; otherwise
  it is `Absent { field }`, naming the first absent `state_fields` name in input order. The
  accessor error takes precedence over absence because the accessor returns no table. This check
  occurs after `call_site` and operation-shape validation but before playback or post-state
  binding, range checks, document construction and replay. Consequently `ModelFields` wins over
  simultaneous `MissingField`, `UndeclaredField`, `DuplicateField` or `OutOfDomain` defects in
  either binding; it calls neither `replay_state_clause` nor its executor.
- If the playback or the post state binds no value for a declared state field, then the generator shall
  return `StateClauseReplayError::MissingField` naming the field, without supplying a
  default value.
- If the playback or the post state binds a name outside the path's declared field list (the
  input's `state_fields`), then the
  generator shall return `StateClauseReplayError::UndeclaredField`, and if either binds a state
  field more than once, then the generator shall return `StateClauseReplayError::DuplicateField`,
  each naming the field.
- If the generator returns `StateClauseReplayError::MissingField`, `UndeclaredField` or
  `DuplicateField`, then it shall not call `replay_state_clause`.
- The generator shall take the model header from the one supplied domain package whose identity
  owns the state object's type address (`ix://<identity>/<name>`) and which is addressed by a
  `sha256-jcs` digest, and shall return `StateClauseReplayError::Document` with a distinct
  `ModelError` for an unreadable package, a package under another digest domain, a type that is
  no address, a type no package owns and a type more than one package owns.
- When the generator returns a state-clause replay result or error, the Kani adapter shall read
  it as FR-029-AC-16 states.
- Where the generator derives reduced candidates from a `Witness`-arm counterexample, it shall
  retain a candidate only as a new envelope, after both a backend re-run and native replay preserve
  domain validity and the failure.
- Where the generator retains a reduced `Witness`-arm envelope, the envelope shall carry the
  `Witness` arm of its own backend re-run and not the original counterexample's transcript.
- The generator shall retain the original failing envelope as the root revision. For each
  retained reduction it shall create a new immutable CG lineage record identifying that exact
  admitted QSL envelope, the same opaque obligation identity, and the exact parent revision from
  which it was reduced. Prior envelopes, replay evidence and lineage records shall remain
  unchanged. A candidate rejected for invalid domain, QSL admission refusal, failed backend
  re-run or changed verdict shall create no revision; the previously retained evidence remains
  unchanged.
- When a reduction is retained or rejected, CG shall preserve the prior revision identifier,
  parent reference, opaque obligation identity, QSL envelope value and typed replay evidence.
  The prior envelope remains equal to a clone captured before the proposal under QSL's
  `WitnessEnvelope<P>: Eq`; the prior record's identity, parent, obligation and evidence fields
  remain equal to their captured typed values. No byte serialization is required.
- The generator shall define none of `Witness`, `ReplaySource`, `WitnessEnvelope`,
  `TerminalRecord` or `ObligationIdentity`.
- The generator shall import none of those types from Contract IR.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-024-AC-1 | RETIRED, replaced by FR-024-AC-47; the former CG-minted O-09 digest scheme is obsolete and earns no implementation credit. | Inspection |
| FR-024-AC-2 | Every backend-witness transcript the generator passes to `Witness::parse` is produced by the one adapter rendering function from decoded values, and that function takes no backend-native text as input. | Test (TC-035) |
| FR-024-AC-3 | A counterexample with any value outside its declared domain is reported out-of-domain and neither `replay`, `replay_frame` nor `replay_state_clause` is called. The endpoints of each declared domain are admitted, and the values immediately outside it are not. | Test (TC-035) |
| FR-024-AC-4 | For each common required `WitnessPacket` member in turn, excluding the three alternative replay-target members of FR-024-AC-41, a packet missing that member is refused by `WitnessEnvelope::reconstruct` with QSL's `MissingMember` naming it, and no replay runs. A packet whose trace position is present with the value none is admitted. | Test (TC-035) |
| FR-024-AC-6 | A frame counterexample is submitted as a `WitnessEnvelope<FrameCounterexample>` through `qsl_replay::replay_frame`, and its clause, frame, anchor and occurrence identities reach QSL unchanged. | Test (TC-035) |
| FR-024-AC-7 | A reduced `Witness`-arm candidate is retained only when a backend re-run and native replay both preserve domain validity and the failure; a candidate that changes domain validity or the verdict is discarded. | Test (TC-035) |
| FR-024-AC-8 | A retained reduced `Witness`-arm envelope carries its own backend re-run's transcript, never the original counterexample's. | Test (TC-035) |
| FR-024-AC-10 | No type named `Witness`, `ReplaySource`, `WitnessEnvelope`, `TerminalRecord` or `ObligationIdentity` is defined under `src/`, and no source file under `src/` imports any of them from `quire_contract_ir`. | Test (TC-035) |
| FR-024-AC-11 | `StateClauseReplay::replay` returns exactly the `StateClauseReplayResult` that `qsl_replay::replay_state_clause` returns when a test calls it directly with the same request and envelope, for a violating run and for a respecting run, and the two results differ from each other. `StateClauseReplay::replay_through` hands the executor the request and envelope it built and returns the executor's value as it is: an executor returning a sentinel result changes the returned result to that sentinel. | Test (TC-035) |
| FR-024-AC-12 | The state-clause envelope's `clause_node` equals the `ClauseSite` node, and its `occurrence_key` the `ClauseSite` occurrence, that `qsl_replay::call_site` returns for the clause name. Its packet and request carry `state_clause` as that clause's declared `QualifiedName`, with neither other replay target; its payload has no `clause` selection and its `observation` is the `Invocation` arm. For a unit with two postcondition clauses, each clause's envelope carries its own node, occurrence and declared name. A clause name the unit does not declare returns `StateClauseReplayError::CallSite` holding the call site's refusal when the request is built, and the executor is not called. | Test (TC-035) |
| FR-024-AC-13 | The pre snapshot holds, for every declared field of the state object, the value the Kani playback bound to that field, and the post snapshot holds the post-state value the caller supplied for it; each is one `complete` population holding one object with each field as an integer. The invocation document holds the members the Behavior bullets state, with `parameters` empty, `result` null and `created` and `deleted` empty, and its `pre` and `post` carry each snapshot's identity and `sha256-jcs` digest. Changing one playback value changes the pre snapshot's bytes and digest, and changing one post-state value changes the post snapshot's. | Test (TC-035) |
| FR-024-AC-14 | For a document vector whose members are in non-sorted source order, whose text holds characters JSON escapes (a quote, a backslash, a control character, a non-ASCII letter) and whose numeric members include a large integer, a number with a fractional part and an exponent form, the bytes and the digest the state-clause builder returns equal what `core::canonical`'s bytes and digest functions return for the same value. The source of the new state-clause module names none of `quire_canonical`'s encoder functions, `sha2`, `ByteDigest::of`, a `sort` call over document members or a hand-written string-escaping routine, and `sha2` stays out of `[dependencies]`. | Test (TC-035) |
| FR-024-AC-15 | A playback that binds no value for a declared state field returns `StateClauseReplayError::MissingField` naming that field, the executor is not called, and no snapshot holds a default for it. A playback or post-state name outside the path's declared field list returns `StateClauseReplayError::UndeclaredField` and a field bound twice returns `DuplicateField`, each naming the field, in the playback and in the post state alike. | Test (TC-035) |
| FR-024-AC-16 | For a subject mutated to violate the postcondition, the replay settles `reproduced-with-evaluated-witness` with category `violation` and an evaluated `false`. For the unmutated subject's run over the same pre state it settles `inconclusive` with cause `Verdicts` (`violation` proved, `success` replayed). The envelope is on the `Witness` arm with a payload `witness` of none in both cases. | Test (TC-035) |
| FR-024-AC-17 | A state field value outside its declared integer range returns `StateClauseReplayError::OutOfDomain` naming the field and the executor is not called. The range's two endpoints are admitted and the values one below and one above are not. | Test (TC-035) |
| FR-024-AC-18 | With the installed backend, the falsified operation-contract harness of a postcondition state clause over a subject mutated to debit is replayed from its real Kani playback through `replay_state_clause` and settles `reproduced-with-evaluated-witness`, `violation`. The test is `tc_035_real_kani_state_clause_counterexample_replays_through_qsl` in the module `kani_obligations_state_clause_replay`, so the `kani_obligations` filter of `make kani` selects it. | Test (TC-035) |
| FR-024-AC-19 | An operation that declares a parameter, and one that declares a result, each return `StateClauseReplayError::UnsupportedOperationShape` carrying the operation and the declaration (its parameters and result), build no document, and do not call the executor; an operation that declares neither is not refused for its shape. The shape check reads the admitted package the caller supplies, and QSL's admission reads the documents provided in the byte provision; where the two disagree, the disagreement surfaces as a QSL refusal and not as this error. The error is not read as a QSL refusal: FR-029-AC-16 maps it to `Failed`. | Test (TC-035) |
| FR-024-AC-20 | RETIRED, replaced by FR-024-AC-48; the former four-member O-09 preimage and encoder are obsolete and earn no implementation credit. | Inspection |
| FR-024-AC-21 | RETIRED, replaced by FR-024-AC-49; the former kind/occurrence-inside-digest control is obsolete and earns no implementation credit. | Inspection |
| FR-024-AC-22 | RETIRED, replaced by FR-024-AC-50; the former frame preimage member rule is obsolete and earns no implementation credit. | Inspection |
| FR-024-AC-23 | `StateFrameIdentity` and its persisted record carry `state_fields`, every field the harness draws in draw order, which `domains` (ranged fields only) and `property`'s granted and checked lists (unordered) do not give. `state_fields` is used for decoding and ordering only and is not minted. A field with no declared range is listed in `state_fields`, has no entry in `domains`, is decoded and is not range-checked or refused (the AD-003 owner may tighten that later). A regenerated harness from equal inputs has a byte-identical record, and a record without `state_fields`, or one that lists a state field twice, is not read as a frame identity; a playback schema that binds a name twice is a typed decode refusal. | Test (TC-035) |
| FR-024-AC-24 | RETIRED, replaced by FR-024-AC-51; the former frame identity minting rule is obsolete and earns no implementation credit. | Inspection |
| FR-024-AC-25 | The frame witness is built from the real playback: the playback text decodes through the one decoder of `src/replay/witness.rs` against bindings built from the harness's `state_fields` (each `i64`, in draw order), and the transcript passed to `Witness::parse` is rendered by the one adapter rendering function from the decoded values, with the harness path and the check text the decode names. The fixed per-operation assertion text that `frame.rs` passes to `Witness::parse` today is gone. A playback of another harness, a playback with the wrong number of values and one with a wrong-width value each return a typed decode refusal carrying the decoder's cause, and call neither `call_site` nor `replay_frame`. | Test (TC-035) |
| FR-024-AC-26 | A decoded state-field value outside the field's declared range returns `FrameReplayError::OutOfDomain` naming the field and its value, and calls neither `call_site` nor `replay_frame`; the range's two endpoints are admitted and the values one below and one above are not. A field with no declared range is not range-checked. | Test (TC-035) |
| FR-024-AC-27 | The decoded pre-state is tied to the invocation: for every state field, the integer the pre snapshot named by the invocation holds for the object the invocation's `self` addresses equals the decoded value. A differing field returns `FrameReplayError::PreState` naming the field, decoded value and snapshot value. An invocation or pre snapshot absent from the provided documents, not in the state-clause path's typed document shape, or lacking the object or a field returns `PreState` naming what is missing. None calls `replay_frame`. A forbidden-write playback replayed against an invocation of a different pre state is refused; its own pre state settles `reproduced-with-evaluated-witness` with category `violation`. The comparison uses the provided document values and does not compute or compare a document digest. | Test |
| FR-024-AC-28 | A harness whose `scope.operation` is not the operation requested returns `FrameReplayError::ScopeMismatch` naming `operation` before `call_site` is called; one whose `scope.anchor` or `scope.frame` is not the anchor or frame `call_site` names returns `ScopeMismatch` naming that member and does not call `replay_frame`. The harness's module and harness symbols name the generated artifact, are not members of the identity, and are checked only by the decode (FR-024-AC-25). | Test (TC-035) |
| FR-024-AC-29 | The frame envelope's `declared_domains` is the empty list, no source of `src/replay/frame.rs` builds a `DeclaredDomain` or a `DomainKey`, and the module's header states, once, that the declaration is empty until QSL-345 settles the declared-domain key and refuses an empty declaration. The frame path adopts no key shape before then, whatever shape another path builds. | Test (TC-035) |
| FR-024-AC-30 | PLANNED (IR-1086, IR-624). With the installed backend, the real playback of the falsified frame harness of a subject that writes a forbidden field is replayed through `FrameReplay::new` and `replay`, the caller supplying the harness's `StateFrameIdentity`, the parsed playback produced from the falsified outcome's complete counterexample text by the bounded Kani output reader, and the inputs of FR-024's Inputs list but no obligation identity and no transcript, and settles a reproduced violation naming the written field. The caller parses under byte and value limits no wider than the component-wise minimum of its requested limits and `run.decode_limits`; `FrameReplay::new` recomputes that minimum and refuses a supplied `Playback` whose `source_len` or value count exceeds it, even if the caller parsed under wider limits. The harness is generated from QSL's emitted package and its scope node ids equal those `qsl_replay::call_site` names, without `Twin::aligned` (FR-024-AC-32). A subject that writes only a granted field leaves its harness verified and yields no playback. The test is `tc_035_real_kani_frame_counterexample_replays_through_qsl` in `kani_obligations_state_frame`, selected by `make kani`. | Test (TC-035) |
| FR-024-AC-31 | PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation pending. For a QSL-emitted package, `StateClauseReplay` takes names and order from its inputs' `state_fields` and validates each name with `model_object_fields(&object_id)`: for the twin's `balance` and unread `audit`, both are present with the accessor's 0 to 1000 inclusive range. A playback or post state omitting one returns `MissingField`; a bound name outside the list returns `UndeclaredField`; a listed name absent from the accessor returns `ModelFields { object, cause: Absent { field } }`, and an accessor error returns `ModelFields { object, cause: Accessor(error) }`, preserving the exact IR error without a body or read fallback. A non-model object or an admitted unselected model/object_type with a nonempty body causes `NotModelObjectType`, even when its body appears to contain valid ranges. A selected model/object_type whose tampered nonempty body is read by a state clause is refused by IR admission as `StaleNodeKey` before CG replay receives a package; TC-227's admitted selected tamper is unread and is not this replay fixture. After `call_site` and operation-shape validation, `ModelFields` precedes playback and post-state binding errors, including a simultaneous missing playback field; neither model-field error invokes replay or its executor. Endpoints are admitted and one below or above returns `OutOfDomain` (FR-024-AC-17). A present listed field with `member_type()` of `None`, a non-`IntRange` variant, or an `IntRange` outside `i64` has no `DeclaredDomain`; replay carries its `i64` value without range checking. Cases QSL cannot emit directly use an admitted QSL-emitted graph with selected-model-document override and recomputed digests, not a forged field-read range. | Test (TC-035) |
| FR-024-AC-32 | PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation pending. A frame harness generated from the package QSL emits for the twin is replayed through `FrameReplay::new` and `replay`: its generated `scope.anchor` and `scope.frame` equal the ids `call_site` names, asserted before replay; `Twin::aligned` is gone from test support; a forbidden write settles `reproduced-with-evaluated-witness` with category `violation` naming the written field; its `state_fields` retain request order and its `domains` equal the accessor's ranges for listed fields, including an unread ranged field; a playback value one outside a declared range returns `OutOfDomain` (FR-024-AC-26). A non-model object or admitted unselected model/object_type body is refused and supplies no range. A selected/read nonempty-body tamper is refused by IR admission before this frame replay path. | Test (TC-035) |
| FR-024-AC-33 | PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation pending. A postcondition harness generated from the package QSL emits for the twin, with no identity alignment, is replayed through `StateClauseReplay::new` and `replay`: the debit mutation settles `reproduced-with-evaluated-witness` with category `violation` and evaluated `false`; the unmutated subject over the same pre state settles `inconclusive` with cause `Verdicts` (FR-024-AC-16); the clause id is the one `call_site` names for `BalanceNeverDrops`, and the declared field ranges equal those returned by the accessor, including an unread ranged field. | Test (TC-035) |
| FR-024-AC-34 | PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation pending. With the installed backend, the real playback of the falsified frame harness of a forbidden write (FR-024-AC-30) and the falsified postcondition harness of a debit mutation (FR-024-AC-18) replay and settle as those criteria state, each harness generated from QSL's emitted package using accessor-derived ranges. The modules `kani_obligations_state_frame` and `kani_obligations_state_clause_replay` are selected by the `kani_obligations` filter of `make kani`. | Test (TC-035) |
| FR-024-AC-35 | PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation pending. A falsified frame or postcondition harness draws a present model field whose accessor type has no `i64` range (for example `IntRange` with a lower endpoint below `i64::MIN` and an upper endpoint of -1), and playback binds an `i64` value outside that model range. QSL refuses the pre state and settlement is `Inconclusive` with `ReplayRefused` (FR-029-AC-16), naming the field and its persisted `TypeNotRange` reason; it never reports `Verified` or a violation. Playback inside the model range is unaffected, and an unread present `IntRange` within `i64` is bounded and is never reported as unranged. The source and IR range ceiling is `i128::MIN..=i128::MAX`. Same-model wide-range replay evidence is blocked by QSL-642; an IR-admitted selected-model override alone is not such evidence. | Test (TC-035) |
| FR-024-AC-36 | Two distinct test backend adapters each produce a typed outcome: success carries equivalent assignments, each adapter's backend identity, the same opaque obligation identity, declared domains, source/provenance and replay packet members; decode refusal carries only backend identity, obligation identity and typed cause. Native text is observed only within its own adapter; the common intake neither parses native text nor remints the obligation identity, and QSL receives each successful adapter's original backend and source identity in a complete packet. | Test (TC-035) |
| FR-024-AC-37 | The common intake maps an adapter's decode-refusal arm to a typed decode evidence failure retaining its cause, with no envelope or replay. Before native replay, an out-of-domain assignment yields a distinct typed domain evidence failure; an incomplete or QSL-refused witness/packet yields a distinct typed QSL admission evidence failure retaining its cause. A replay verdict disagreement remains a distinct typed evidence failure. None settles as contract success or contract failure, and none of the pre-replay failures invokes replay. Kani's exact decode and refusal partition remain governed by FR-016. | Test (TC-035) |
| FR-024-AC-38 | Starting from one admitted original failure, one valid reduced candidate that preserves the failure becomes a new CG lineage revision referring to its exact admitted QSL envelope, its exact parent revision and the unchanged opaque obligation identity. After retention, the original QSL envelope compares equal to its pre-reduction clone by `WitnessEnvelope<P>: Eq`, and the original CG record's revision identity, parent, obligation and typed evidence fields read back equal to their captured values. No lineage member is added to QSL's envelope or packet. | Test (TC-035) |
| FR-024-AC-39 | From a retained failure revision, an out-of-domain reduction and a domain-valid reduction that changes the native verdict each create no revision. After each rejection, every prior QSL envelope compares equal to its pre-proposal clone and every prior CG record's revision identity, parent, obligation and typed evidence fields equal their captured values; a later valid reduction names the actual retained parent, with no skipped or substituted parent. | Test (TC-035) |
| FR-024-AC-40 | A retained reduced `Witness` envelope holds a transcript from its own backend re-run, bound to its own reduced assignments; it cannot reuse any ancestor's transcript. | Test (TC-035) |
| FR-024-AC-41 | A function counterexample packet and its admitted `WitnessEnvelope` carry `function` as the selected function's `QualifiedName`, with no `frame_operation` or `state_clause`; a frame counterexample packet and envelope carry `frame_operation` as `{object, operation}` from QSL's validated typed call-site result, with `object` naming the operation's declaring object type, process or state machine (also when the caller selected an inherited operation) and `operation` its `Identifier`, and no `function` or `state_clause`; a postcondition state-clause counterexample packet and envelope carry `state_clause` as the clause's own declared `QualifiedName` after QSL's `ClauseSite` validates that selection, and no `function` or `frame_operation`. CG refuses packet construction if the relevant validated selection is unavailable. None contains a `selected_function` member or alias. For a frame or state clause, the replay request has no `arguments` or `occurrence_key`; the frame packet's ADR-017 PF-4 frame and generated-occurrence identities remain unchanged, and a state-clause payload has no `clause` member. | Test (TC-035) |
| FR-024-AC-42 | A function, frame or state-clause packet with none of `function`, `frame_operation` and `state_clause` is refused as a missing replay target; one with any two or all three is refused as a conflicting replay target. Each refusal is typed and distinguishable, `WitnessEnvelope::reconstruct` admits no envelope, and no replay executor is called, even when every common member is present. An absent selection is not replaced by an empty name, an inferred payload operation or clause, or `selected_function`. | Test (TC-035) |
| FR-024-AC-43 | A function-contract adapter supplies a completed `WitnessPacket<NoPayload>` carrying its decoded witness and retained proving-run members. CG admits it through `WitnessEnvelope::reconstruct` before request construction and requires its `Witness` source arm. The request builder accepts the admitted envelope and request-only members, and copies exactly its function `declaration` occurrence key, `function` target and opaque `obligation_identity`; it accepts no bare key or selected-site substitute. Omitting the producer packet, substituting a different admitted function occurrence or changing its source to `Input` while leaving the selected claim fixed produces no request or replay. | Test (TC-035) |
| FR-024-AC-44 | An input-arm parity claim producer supplies a completed `WitnessPacket<NoPayload>` before its request builder. CG admits it through `WitnessEnvelope::reconstruct` and requires its `Input` source arm. The operator-parity request copies that packet's checked scalar application occurrence key, and the value-parity request copies its function `declaration` occurrence key, target and opaque obligation identity. Typed request serialization and readback preserve each full key, including two admitted occurrences of one node and an `expression` occurrence with a nonzero ordinal. No request builder mints a key from caller-supplied parity metadata or takes a bare key. Replacing the producer packet with the other admitted occurrence or a `Witness` source while holding the selected claim fixed produces no request. | Test (TC-035) |
| FR-024-AC-45 | A `function` replay request missing `occurrence_key` refuses through QSL's request reader as `invalid_runtime_input`/`missing-member` at `/replay/occurrence_key` before byte provision and recompile; the same request with a byte-provision entry also removed reports the same refusal. Neither refusal invokes replay or becomes a contract verdict. | Test (TC-035) |
| FR-024-AC-46 | Before constructing a function-contract, operator-parity or value-parity request, CG compares the admitted packet's `function` target, complete occurrence key, subject `clause_node`, `package_id`, source arm and opaque obligation identity with the independently selected claim from the admitted package and proving-run identity or original checked parity claim, never from that packet. A wrong target, key node, role or ordinal, subject node, package, source arm or obligation identity returns a typed claim-binding refusal naming the mismatched member, with no request or replay. A different valid occurrence of one node is accepted only for its own selected claim; the other claim's packet is refused. A missing packet key is QSL's packet-admission refusal and is never defaulted. | Test (TC-035) |
| FR-024-AC-47 | PLANNED (IR-1079; replaces AC-1). The envelope and replay request carry the exact producer-issued QSL `ObligationIdentity(NodeKey, Option<ModelIdentity>, ordinal)` for the selected function or frame, unchanged by CG. Source-span, harness symbol, solver and options changes leave that structural identity unchanged. Changing node, model or producer-assigned ordinal changes it. Kind, complete occurrence and sorted argument domains remain distinct obligation-record members; changing any one under a held structural identity refuses the record/selected-claim match before replay. The producer assigns different ordinals to distinct obligations of one node/model (QSL ADR-013 O-09, 878381034d5ddda662def7cffe0096d0e3c5f0cb). | Test (TC-035) |
| FR-024-AC-48 | PLANNED (IR-1079; replaces AC-20). The function and frame paths use the QSL producer identity as a typed three-component value, with no CG preimage object, identity encoder or digest calculation. The function obligation record retains the selected function occurrence, kind and complete parameter bindings sorted by declared identifier, each with its declared Boolean or bounded-integer domain; the frame record has its frame occurrence, `frame` kind and empty argument list. Equal record and claim members admit the same identity; a member mismatch refuses before replay. | Test, Inspection (TC-035) |
| FR-024-AC-49 | PLANNED (IR-1079; replaces AC-21). Structural identity changes for a different frame node, owning model or assigned ordinal and remains equal for unchanged components across comments, blank lines, source-span, symbols, solver, options and state-field display changes. A change of frame grant that rekeys its producer node changes identity; if the node remains unchanged, full grant and field comparison detects a changed body. A changed kind or occurrence under held identity refuses before replay. Distinct postcondition claims of one operation retain their own complete occurrences even if they share one frame obligation. | Test (TC-035) |
| FR-024-AC-50 | PLANNED (IR-1079; replaces AC-22). A frame record and envelope carry the exact `OperationSite` frame node and frame occurrence; the obligation identity carries that node, the producer-supplied optional model and producer-assigned ordinal. The record kind is `frame` and its arguments are empty. A postcondition property returns `FrameReplayError::NotAFrame`; a frame whose granted and checked fields differ from its state fields returns `FrameReplayError::FieldSetMismatch`; neither calls `call_site` or `replay_frame`. Field and range controls remain checked outside structural identity. | Test (TC-035) |
| FR-024-AC-51 | PLANNED (IR-1079; replaces AC-24). `FrameReplayInputs` has no `obligation_identity` member. The frame request and envelope carry the same producer-issued typed identity from the selected frame claim; no caller stand-in, CG mint or hash reaches either. A changed grant that rekeys the producer frame node changes both identities; a grant/body mismatch under a held node is refused by the full record comparison. The state-clause path remains outside this criterion. | Test (TC-035) |

### Mutations FR-024-AC-31 to FR-024-AC-35 detect

| ID | Mutation that breaks it |
|----|-------------------------|
| FR-024-AC-31 | Read fields from the empty object body or a read's `result_type`, omit unread `audit`, sort caller order by accessor name, collapse absent and accessor-error causes, return `MissingField` when a listed model field is also absent from playback, or narrow an `i128` endpoint. |
| FR-024-AC-32 | Generate from the hand-built fixture, align scope ids, or drop an unread ranged field from `domains`. |
| FR-024-AC-33 | Give replay a field list or ranges read from the package's object body or from reads. |
| FR-024-AC-34 | Run the real-Kani cases over a hand-built fixture harness. |
| FR-024-AC-35 | Settle refusal as a violation or `Verified`, omit `TypeNotRange`, or call an unread bounded field unranged. |

A transcript `Witness::parse` refuses is an adapter
refusal under FR-016-AC-11.

## Current state

FR-024-AC-2, FR-024-AC-3, FR-024-AC-4 and FR-024-AC-6 are backed by focused
TC-035 tests: the `Witness::parse` source scan in `tests/it/layout.rs`, the function and
frame domain-endpoint tests, the missing-packet-member test in
`tests/it/kani_obligations_state_clause_replay.rs`, and the frame packet identity test in
`tests/it/kani_obligations_state_frame.rs`. FR-024-AC-1, FR-024-AC-7 and
FR-024-AC-8 remain planned. FR-024-AC-5 and FR-024-AC-9 are retired (IR-890);
their IDs are not reused.
FR-024-AC-10 has a source-scan test over `src/`;
FR-024-AC-11 and FR-024-AC-13 to FR-024-AC-29 have implemented slices described below.
FR-024-AC-12 remains planned under IR-965. FR-024-AC-30's typed frame intake is implemented under
IR-1086; its QSL-emitted harness without `Twin::aligned` remains planned under IR-624:

- `src/replay/witness.rs` decodes the playback into `qsl_replay::WitnessValue` and imports no
  Contract IR witness type.
- The bounded-Kani corpus retains no counterexample packet, so no corpus counterexample reaches
  QSL as the `Input` arm. Its replay uses Kani playback on the witness path. The corpus path is
  planned for removal under FR-015-AC-109 to FR-015-AC-111, so the former `Input`-arm obligation
  FR-024-AC-5 and FR-024-AC-9 are retired without replacement.
- A postcondition state-clause counterexample is replayed (IR-460): `src/replay/state_clause.rs`
  builds the invocation and snapshot documents through `core::canonical` and replays through
  `qsl_replay::replay_state_clause`, so FR-024-AC-11 and FR-024-AC-13 to FR-024-AC-19 are backed by tagged tests
  (FR-024-AC-18 in the `kani` lane; its test uses the fixture subject that debits to the floor of
  the range, `deposit_debiting_within_range`).
- Declared-domain key divergence, open: `src/replay/state_clause.rs` keys each state field's
  `DeclaredDomain` as `DomainKey::Node { node: the self parameter node, path: [CG's field
  position] }`. QSL `main` bcca433 (ADR-012 section 15.4, QSL-345 item 4) defines a state field's
  key as `Node { the declaring object_type node, [the field's ordinal in ascending UTF-8 name
  order] }` and returns it from `call_site` as `FieldSite.domain`. Nothing breaks today, because
  QSL has no declared-domain check yet. The state-clause path should adopt `FieldSite.domain`
  before that check lands, and compute no key of its own.
- Out-of-range post-state values are preserved as observations, while an out-of-range pre state
  or argument remains refused by QSL admission (QSL-634). The wrapping debit subject
  `deposit_debiting` reaches -1 from the floor of `balance`'s range. CG puts that exact value in
  the post snapshot, QSL settles the witness as a reproduced violation, and CG maps the run to
  the ordinary `Refuted` terminal (FR-029-AC-18, IR-460).
- The state-clause adapter in `src/replay/state_clause.rs` decodes a falsified postcondition
  harness's Kani playback into the named `i64` pre-state values that
  `StateClauseReplayInputs::playback` takes. The remaining AD-004 step 5 L-6 boundary change
  adds `kani/classify.rs::parse_state_clause_falsification` as the production entry. It checks
  `KaniRunOutcome::Falsified` before parsing, returns its typed `NotFalsified` error for other
  outcomes, and retires the now-unreachable replay error variant of that name. It passes the
  full falsified counterexample text to `parse_assertion_playback` in
  `kani/output/playback.rs`. The parser receives the smaller requested and run byte and value
  limits before reading the text, preserving its multiple-assertion refusal and full-text
  `source_len`. Its parsed `Playback` reaches `StateClauseReplayInputs::decode_falsification`
  without a second parse. The public method recomputes those effective limits from its requested
  limits and `run.decode_limits`, even if supplied a `Playback` parsed under wider bounds. It
  derives ordered bindings from the harness's state fields
  and retains its playback-path versus harness-identity, selected-clause, postcondition-property,
  generated-assertion and field-order checks before populating the symbolic pre state. The
  non-falsified-run test moves to the classifier helper's pre-parse boundary, and TC-026 covers
  both the helper's pre-parse limits and the method's post-parse checks; the real-Kani test drives this path;
  FR-024's acceptance criteria and replay settlement are unchanged.
- The frame path's own hand-built invocation document and encoder remain in
  `tests/state_frame_support/native_twin.rs`; its `result` member is now `null`, as the twin's
  operations declare no result.
- The frame path's typed intake is implemented (IR-1086): `FrameReplayInputs` carries the
  falsified harness's `StateFrameIdentity` and parsed `Playback`, and no obligation identity or
  transcript. The one
  function of `src/replay/obligation.rs` mints the identity from the `OperationSite` and the
  harness's property; `StateFrameIdentity` and its record carry `state_fields`; the playback is
  decoded by the one decoder and rendered by `render_witness`; and the harness's field set,
  scope, ranges and pre snapshot are checked before the request is built. The twin
  (`tests/state_frame_support/native_twin.rs`) passes no identity. Its harness scope is aligned
  to the node ids QSL names (`Twin::aligned`), because the fixture's checked package is hand-built
  and its node ids are its own; the package QSL itself emits carries them (measured below). The frame path's `declared_domains` stays `Some(Vec::new())` (FR-024-AC-29).
- Node ids, measured: `qsl_replay::call_site` returns the compiled package's
  `quire.checked-package/v2` bytes (`CallSite::package`), and the model reader admits them given
  the domain package. In the package QSL emits from the twin's unit, the `operation_anchor` and
  `frame` nodes have exactly the ids `call_site` names as the site's `anchor` and `frame`, and the
  anchor binds that frame (`tc_035_the_node_ids_of_the_package_qsl_emits_are_the_ids_call_site_names`),
  so the generator's node ids and the replay's wire ids are one id and a production harness
  generated from QSL's package needs no rebase.
- Planned, IR-624 (FR-015-AC-77 to FR-015-AC-81 and FR-024-AC-31 to FR-024-AC-35):
  the current generator still reads the framed object's body, so it refuses QSL's emitted
  model declaration object with `MemberAbsent` and current positive replay tests generate from
  the hand-built fixture and call `Twin::aligned`; IR-624 retires those positives. IR-628's `model_object_fields` supplies the
  emitted object's effective fields and value bounds from the selected domain document; the CG
  code change will use it after CG updates its IR dependency and implements the merged accessor.
  A read's `result_type` remains untrusted. The hand-built non-declaration body-member path is
  removed; its inputs are refused rather than trusted. The request still supplies
  `state_fields` order, and the accessor checks their membership and ranges.
  FR-024-AC-35's same-model out-of-`i64` replay case remains gated on QSL-642, whose owner plans
  authoritative source emission and replay through `i128` and a typed check refusal above it.
  An IR-admitted model override checks CG's unranged handling but does not prove QSL replay of that
  overridden model. The Boolean same-source refusal and an in-`i64` source range are narrower
  executable cases; neither closes the wide-range acceptance criterion.
- `quire coverage --strict` reports 555/909 backed criteria and 232 unbacked rows at this
  revision, compared with 551/909 backed and the same 232 unbacked rows on `main` before this
  intake test change. The four additional backed criteria are FR-024-AC-2, AC-3, AC-4 and AC-6.
  The unbacked-row count is a separate diagnostic keyed by verification reference: every FR-024
  row points to TC-035, which already had tagged tests, so even the planned FR-024 criteria are
  absent from those 232 rows. Their absence does not establish backing. The retired FR-024-AC-20 to AC-24 had tagged tests of the former digest path; replacement AC-47 to AC-51 require structural controls. AC-25 to AC-30 retain their own tagged tests.
- `FR-024-AC-10` has a source-scan test for literal declarations, macro token declarations,
  Contract IR imports and paths, including alias chains across source files. Imports and qualified
  paths inside opaque macro tokens, macro expansion, generated files outside `src/`, and constructed
  identifiers remain outside that test.
- `FR-024-AC-36` to `FR-024-AC-40` are planned (IR-335). The common typed intake,
  cross-adapter test, typed evidence failures and immutable CG minimization lineage have no
  production implementation or tagged test yet.
- The one adapter transcript rendering function is `render_witness` in `src/replay/function.rs`,
  used by the skeleton spine, the state-clause path and the frame path.
- IR-958 and IR-965 selection is specified but not yet implemented: `src/replay/function.rs`,
  `src/replay/frame.rs` and `src/replay/state_clause.rs` still construct the old
  `selected_function` member. This slice specifies all three packet selections. QSL STD-245
  and STD-261 specify the envelope admission and `replay_frame`/`replay_state_clause` consumer
  changes; their code is pending. QSL's typed `OperationSite` must expose the validated declaring
  owner and operation identifier before CG can consume that pair under FR-024-AC-41. The
  state-clause path must take its validated
  clause name from the `ClauseSite` and remove the old payload `clause` member.
- IR-968's FR-024-AC-43 to FR-024-AC-46 are specified and planned: the
  function-contract and parity request entry points do not yet accept an
  admitted packet. Their current call-site and caller-supplied parity keys are not
  packet provenance. The function-contract missing-request-key test for
  FR-024-AC-45 is drafted on held CG PR #550 but awaits the final QSL lock.
  Implementation follows QSL's replay-target code (QSL-973) and occurrence-key code
  (STD-264); no tagged CG coverage is claimed for these new criteria.

## Open questions

None of these is decided here. FR-024-AC-24 excludes the state-clause path because of Q-1, and
Q-3 is resolved by QSL ADR-013 O-09 structural identity; Q-1 and Q-4 remain open.

- Q-1. The postcondition state-clause path takes a caller-supplied `obligation_identity`
  (`StateClauseReplayInputs`). Whether it takes the harness's `StateFrameIdentity` and mints the
  identity too belongs to the state-clause code; how the replay is given the state fields the
  harness draws is settled by FR-024-AC-31 (a `state_fields` member of its inputs). Packet assembly stays local to each path and whichever code change lands
  second extracts the shared piece; this requirement assigns neither.
- Q-3 (resolved): QSL structural `ObligationIdentity` has no digest or digest domain label.
- Q-4. The claimed change is caller-supplied and is not tied to the post snapshot, as the pre
  state now is tied (FR-024-AC-27). Deriving it from the two snapshots needs the shared document
  building piece of Q-1.

- Q-5 is settled by Contract IR FR-038-AC-136 to AC-144. Its merged
  `CheckedPackageV2::model_object_fields` returns the selected model declaration's effective
  fields and their derived types, including unread fields and value bounds. CG still needs to
  update its IR dependency and implement the accessor route. CG does
  not re-resolve the document. For a hand-built object that is not a model declaration, the
  accessor returns `NotModelObjectType`; CG refuses it with `ModelFields`, without reading its
  body. The same refusal is reachable for an admitted unselected model/object_type with
  a nonempty body. A selected/read nonempty-body tamper refuses `StaleNodeKey` at IR
  admission, before CG. A field read's `result_type` is never the range.

Settled here and not open: a frame's `arguments` are empty and E-1 is not widened. A state field
with no declared range is carried in the record and decode, which draw it unconstrained, and is
not refused (FR-024-AC-23; the AD-003 owner may tighten that later, as a function argument with
no bound is refused under AD-016 arrow 5).

## Dependencies

- **Upstream**: [FR-016](./FR-016-witness-native-replay.md), which decodes the playback, refuses and
  partitions the outcomes; [FR-017](../../kani/functional/FR-017-kani-execution-evidence.md), which retains the
  playback; [FR-025](../../kani/functional/FR-025-generated-subject-abi.md), which gives each argument binding its
  parameter node id and declared domain; [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md), whose
  operation-contract harness (FR-015-AC-26) is the state-clause counterexample's source;
  [FR-029](../../kani/functional/FR-029-run-outcome-terminal-record.md), which reads the state-clause
  result and errors (FR-029-AC-16); QSpec FR-323's replay-target selection; QSL ADR-017 PF-4
  and QSL's `qsl-replay` crate.
- **Downstream**: [TC-035](../matrix/TC-035-counterexample-envelope-intake.md).
