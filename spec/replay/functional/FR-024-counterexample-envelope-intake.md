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
  - target: ix://agent-ix/quire-spec-language/FR-116
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-122
    type: references
  - target: ix://agent-ix/quire-specification/FR-197
    type: references
  - target: ix://agent-ix/quire-specification/FR-312
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

The frame path's obligation identity, witness and pre-state tie are specified by FR-024-AC-20 to
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

- A counterexample from one of two sources:
  - a backend counterexample: an [FR-017](../../kani/functional/FR-017-kani-execution-evidence.md) `falsified`
    run's playback, decoded by FR-016 into values named by their argument bindings;
  - a corpus counterexample: canonical assignments that no backend run produced.
- The obligation's `KaniObligationIdentity` and its argument bindings, each naming its parameter
  node id and declared domain ([FR-025](../../kani/functional/FR-025-generated-subject-abi.md)).
- The proving run's envelope members, supplied by the caller: every `WitnessPacket` member QSL
  defines, the trace position included (present in every packet, with the value none for a family
  that has no trace position).
- The proving run's request members, which belong to the replay request and not to the envelope:
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
- For a frame counterexample (IR-459): the falsified harness's `StateFrameIdentity` and the
  falsified run's playback text, both read by the generator, and the proving run's members, the
  operation, the invocation document reference, the pre, post and invocation documents and the
  claimed change, which the caller supplies as `FrameReplayInputs` does today. The caller supplies
  no obligation identity and no witness transcript.

## Outputs

- A `WitnessEnvelope` admitted through `WitnessEnvelope::reconstruct`, whose `ReplaySource` is
  `Witness` for a backend counterexample and `Input` for a corpus counterexample.
- The replay request built from that envelope and the request members.
- The QSL replay result for the envelope's arm, or a typed refusal with no replay.
- For a postcondition state-clause counterexample: a `StateClauseReplay` holding the request and
  the envelope packet, whose `replay` returns QSL's `StateClauseReplayResult`, or a typed
  `StateClauseReplayError` with no replay.

## Behavior

- The generator shall build the envelope's `qsl_replay::ObligationIdentity` as the ADR-013 O-09
  digest of the obligation's subject node id, occurrence key, obligation kind and arguments, with
  no `source_span` (AD-003 E-1).
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
- The generator shall fill every `WitnessPacket` member from the proving run's envelope members and
  the obligation identity, and shall invent none.
- The generator shall admit the envelope only through `WitnessEnvelope::reconstruct`.
- If `WitnessEnvelope::reconstruct` refuses the packet, then the generator shall return QSL's
  `WitnessRefusal` and shall not call `replay`, `replay_frame` or `replay_state_clause`.
- The generator shall build the replay request from the admitted envelope and the proving run's
  request members, and shall invent no request member. The state-clause path's invocation and
  snapshot documents are the one addition to the byte provision, built as the state-clause
  bullets below state, and no other document is added.
- When a counterexample did not come from a backend transcript, the generator shall submit it as
  the `Input` arm, keyed by parameter node id, and shall never build a `Witness` for it.
- When a counterexample is a frame counterexample, the generator shall submit a
  `WitnessEnvelope<FrameCounterexample>` through `qsl_replay::replay_frame`.
- The generator shall mint every obligation identity, for a function and for a state frame, by
  the one function of `src/replay/obligation.rs`, over the four members AD-003 E-1 names and
  under their existing spelling: `function`, `declaration`, `kind` and `arguments`. The members
  are not widened and no function-path identity changes. For a frame the existing names read
  awkwardly: `function` holds the frame node and `declaration` the frame's own occurrence key.
  The digest is `core::canonical`'s and nothing else encodes or hashes it (FR-024-AC-20).
- The generator shall read the frame obligation's `function` and `declaration` members from the
  `OperationSite` that `qsl_replay::call_site` returns (`frame` and `frame_occurrence`, the pair
  the envelope names, FR-015-AC-34); QSL gives each operation its own frame occurrence, so no
  further member names the operation (FR-024-AC-22).
- The generator shall derive a frame obligation's `kind` from the harness's `property` and set
  its `arguments` to the empty list, because a frame harness's subject is `fn(&mut State)` and
  declares no parameter (FR-024-AC-22). The state fields and their ranges are not in the
  preimage: the frame node names the grants, the ranges are the model's and are checked at replay, and the harness is
  tied to the identity by the checks below, not by the digest. E-1 is not widened.
- The generator shall derive no node id and accept no caller-supplied obligation identity
  (FR-024-AC-24).
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
  it. A document whose bytes do not match its digest is different: QSL reports it with a catalog
  code at its own request decode, so the generator lets that decode run first and returns QSL's
  refusal unchanged, which FR-029 reads by its code as `Inconclusive(ReplayRefused)`.
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
- The generator shall set the state-clause payload's `clause` to the clause's declared name.
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
- Where the generator derives reduced candidates from an `Input`-arm counterexample, it shall
  retain only `Input`-arm envelopes that preserve domain validity and the native verdict.
- The generator shall define none of `Witness`, `ReplaySource`, `WitnessEnvelope`,
  `TerminalRecord` or `ObligationIdentity`.
- The generator shall import none of those types from Contract IR.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-024-AC-1 | The envelope's obligation identity is QSL's `ObligationIdentity` built as the ADR-013 O-09 digest by the one scheme of FR-024-AC-20: the subject node id (function or state frame; a clause or application subject is not yet built), its occurrence key, the obligation kind and the arguments, each a parameter node id with its declared domain (empty for a frame, which has no parameters), and no `source_span`. Changing `source_span` leaves it unchanged, and changing the obligation kind or any argument binding changes it. The scheme's spelling is CG's own and carries no digest label (AD-003 E-1, open question Q-3). | Test (TC-035) |
| FR-024-AC-2 | Every backend-witness transcript the generator passes to `Witness::parse` is produced by the one adapter rendering function from decoded values, and that function takes no backend-native text as input. | Test (TC-035) |
| FR-024-AC-3 | A counterexample with any value outside its declared domain is reported out-of-domain and neither `replay` nor `replay_frame` is called. The endpoints of each declared domain are admitted, and the values immediately outside it are not. | Test (TC-035) |
| FR-024-AC-4 | For each `WitnessPacket` member in turn, a packet missing that member is refused by `WitnessEnvelope::reconstruct` with QSL's `MissingMember` naming it, and no replay runs. A packet whose trace position is present with the value none is admitted. | Test (TC-035) |
| FR-024-AC-5 | A corpus counterexample is submitted as the `Input` arm keyed by parameter node id. It settles `reproduced-without-witness` when it agrees, and no `Witness` is built for it. | Test (TC-035) |
| FR-024-AC-6 | A frame counterexample is submitted as a `WitnessEnvelope<FrameCounterexample>` through `qsl_replay::replay_frame`, and its clause, frame, anchor and occurrence identities reach QSL unchanged. | Test (TC-035) |
| FR-024-AC-7 | A reduced `Witness`-arm candidate is retained only when a backend re-run and native replay both preserve domain validity and the failure; a candidate that changes domain validity or the verdict is discarded. | Test (TC-035) |
| FR-024-AC-8 | A retained reduced `Witness`-arm envelope carries its own backend re-run's transcript, never the original counterexample's. | Test (TC-035) |
| FR-024-AC-9 | A reduced `Input`-arm candidate is retained only when native replay preserves domain validity and the verdict, and it is itself an `Input`-arm envelope. | Test (TC-035) |
| FR-024-AC-10 | No type named `Witness`, `ReplaySource`, `WitnessEnvelope`, `TerminalRecord` or `ObligationIdentity` is defined under `src/`, and no source file under `src/` imports any of them from `quire_contract_ir`. | Test (TC-035) |
| FR-024-AC-11 | `StateClauseReplay::replay` returns exactly the `StateClauseReplayResult` that `qsl_replay::replay_state_clause` returns when a test calls it directly with the same request and envelope, for a violating run and for a respecting run, and the two results differ from each other. `StateClauseReplay::replay_through` hands the executor the request and envelope it built and returns the executor's value as it is: an executor returning a sentinel result changes the returned result to that sentinel. | Test (TC-035) |
| FR-024-AC-12 | The state-clause envelope's `clause_node` equals the `ClauseSite` node, and its `occurrence_key` the `ClauseSite` occurrence, that `qsl_replay::call_site` returns for the clause name, and the payload's `clause` is that name and its `observation` is the `Invocation` arm. For a unit with two postcondition clauses, each clause's envelope carries its own node and occurrence. A clause name the unit does not declare returns `StateClauseReplayError::CallSite` holding the call site's refusal when the request is built, and the executor is not called. | Test (TC-035) |
| FR-024-AC-13 | The pre snapshot holds, for every declared field of the state object, the value the Kani playback bound to that field, and the post snapshot holds the post-state value the caller supplied for it; each is one `complete` population holding one object with each field as an integer. The invocation document holds the members the Behavior bullets state, with `parameters` empty, `result` null and `created` and `deleted` empty, and its `pre` and `post` carry each snapshot's identity and `sha256-jcs` digest. Changing one playback value changes the pre snapshot's bytes and digest, and changing one post-state value changes the post snapshot's. | Test (TC-035) |
| FR-024-AC-14 | For a document vector whose members are in non-sorted source order, whose text holds characters JSON escapes (a quote, a backslash, a control character, a non-ASCII letter) and whose numeric members include a large integer, a number with a fractional part and an exponent form, the bytes and the digest the state-clause builder returns equal what `core::canonical`'s bytes and digest functions return for the same value. The source of the new state-clause module names none of `quire_canonical`'s encoder functions, `sha2`, `ByteDigest::of`, a `sort` call over document members or a hand-written string-escaping routine, and `sha2` stays out of `[dependencies]`. | Test (TC-035) |
| FR-024-AC-15 | A playback that binds no value for a declared state field returns `StateClauseReplayError::MissingField` naming that field, the executor is not called, and no snapshot holds a default for it. A playback or post-state name outside the path's declared field list returns `StateClauseReplayError::UndeclaredField` and a field bound twice returns `DuplicateField`, each naming the field, in the playback and in the post state alike. | Test (TC-035) |
| FR-024-AC-16 | For a subject mutated to violate the postcondition, the replay settles `reproduced-with-evaluated-witness` with category `violation` and an evaluated `false`. For the unmutated subject's run over the same pre state it settles `inconclusive` with cause `Verdicts` (`violation` proved, `success` replayed). The envelope is on the `Witness` arm with a payload `witness` of none in both cases. | Test (TC-035) |
| FR-024-AC-17 | A state field value outside its declared integer range returns `StateClauseReplayError::OutOfDomain` naming the field and the executor is not called. The range's two endpoints are admitted and the values one below and one above are not. | Test (TC-035) |
| FR-024-AC-18 | With the installed backend, the falsified operation-contract harness of a postcondition state clause over a subject mutated to debit is replayed from its real Kani playback through `replay_state_clause` and settles `reproduced-with-evaluated-witness`, `violation`. The test is `tc_035_real_kani_state_clause_counterexample_replays_through_qsl` in the module `kani_obligations_state_clause_replay`, so the `kani_obligations` filter of `make kani` selects it. | Test (TC-035) |
| FR-024-AC-19 | An operation that declares a parameter, and one that declares a result, each return `StateClauseReplayError::UnsupportedOperationShape` carrying the operation and the declaration (its parameters and result), build no document, and do not call the executor; an operation that declares neither is not refused for its shape. The shape check reads the admitted package the caller supplies, and QSL's admission reads the documents provided in the byte provision; where the two disagree, the disagreement surfaces as a QSL refusal and not as this error. The error is not read as a QSL refusal: FR-029-AC-16 maps it to `Failed`. | Test (TC-035) |
| FR-024-AC-20 | One obligation-identity function in `src/replay/obligation.rs` mints the identity of the function path and of the frame path: the SHA-256 of the RFC 8785 encoding, made by `core::canonical`, of one object with the four members `function`, `declaration`, `kind` and `arguments` of AD-003 E-1, under their existing spelling. No function-path identity changes: the existing golden vectors still pass unedited. A frame identity over a fixed site and kind, with empty `arguments`, equals the digest of a hand-written text of that preimage, written in the RFC 8785 member order, which differs from the preimage's declaration order. `sha2` stays out of `[dependencies]`, and the non-test source of `src/replay/obligation.rs` (outside its `#[cfg(test)]` module, where the hand-written vector may use `sha2`) names none of `quire_canonical`'s encoder functions, `sha2` or `ByteDigest::of`. | Test (TC-035) |
| FR-024-AC-21 | The identity changes when any one of these changes alone: the kind, the `function` node and the `declaration` key (node, role or ordinal); it does not change when the harness's clause node, module symbol, harness symbol, state or subject path, solver, unwind bound, options, state fields or their ranges change, and no input of the function is a source span. Two postcondition clauses of one operation therefore mint one frame identity, as QSL gives them one shared frame occurrence. Two units that differ only in the frame's `modifies` grants have different frame identities, and two operations of one object whose frames are equal text have different identities, each measured through `qsl_replay::call_site` and not assumed of the node ids. The same operation in a unit shifted by blank lines has the same identity. Edge, stated and not hidden: occurrence ordinals run over the operations the unit's clauses name, so adding a clause on another operation that sorts earlier can change this operation's frame occurrence and so its identity. Not asserted, on purpose: that a changed declared range in the model changes the frame node. A range is a member of the framed object type and not of the frame, so the identity does not name it; a range is tied to the replay by the package QSL recompiles and by the domain check (FR-024-AC-26), and the ranges the identity omits are not claimed to reach the node id. | Test (TC-035) |
| FR-024-AC-22 | A frame obligation's identity takes `function` and `declaration` from the `OperationSite`'s `frame` and `frame_occurrence`, so they equal the envelope's `clause_node` and `occurrence_key`; its `kind` is `frame` and is derived from the harness's `property`, never supplied; its `arguments` are the empty list, since a frame has no parameters, and no state field or range is a member. A harness whose `property` is a postcondition returns `FrameReplayError::NotAFrame`, and a frame whose granted and checked fields are not exactly its state fields returns `FrameReplayError::FieldSetMismatch`; neither calls `call_site` or `replay_frame`. | Test (TC-035) |
| FR-024-AC-23 | `StateFrameIdentity` and its persisted record carry `state_fields`, every field the harness draws in draw order, which `domains` (ranged fields only) and `property`'s granted and checked lists (unordered) do not give. `state_fields` is used for decoding and ordering only and is not minted. A field with no declared range is listed in `state_fields`, has no entry in `domains`, is decoded and is not range-checked or refused (the AD-003 owner may tighten that later). A regenerated harness from equal inputs has a byte-identical record, and a record without `state_fields`, or one that lists a state field twice, is not read as a frame identity; a playback schema that binds a name twice is a typed decode refusal. | Test (TC-035) |
| FR-024-AC-24 | `FrameReplayInputs` has no `obligation_identity` member. The frame request's `obligation_identity` and the envelope's both equal the identity minted from the site and the harness, and a harness with a changed grant gives a different value in both. The frame twin of `tests/state_frame_support/native_twin.rs` passes no identity of its own, so no stand-in value reaches either member; no source scan is part of this criterion. The state-clause path's caller-supplied `obligation_identity` is not covered by this criterion (Q-1). | Test (TC-035) |
| FR-024-AC-25 | The frame witness is built from the real playback: the playback text decodes through the one decoder of `src/replay/witness.rs` against bindings built from the harness's `state_fields` (each `i64`, in draw order), and the transcript passed to `Witness::parse` is rendered by the one adapter rendering function from the decoded values, with the harness path and the check text the decode names. The fixed per-operation assertion text that `frame.rs` passes to `Witness::parse` today is gone. A playback of another harness, a playback with the wrong number of values and one with a wrong-width value each return a typed decode refusal carrying the decoder's cause, and call neither `call_site` nor `replay_frame`. | Test (TC-035) |
| FR-024-AC-26 | A decoded state-field value outside the field's declared range returns `FrameReplayError::OutOfDomain` naming the field and its value, and calls neither `call_site` nor `replay_frame`; the range's two endpoints are admitted and the values one below and one above are not. A field with no declared range is not range-checked. | Test (TC-035) |
| FR-024-AC-27 | The decoded pre-state is tied to the invocation: for every state field, the integer the pre snapshot (the provided document whose `sha256-jcs` digest the invocation's `pre` names) holds for the object the invocation's `self` addresses equals the decoded value, and a field that differs returns `FrameReplayError::PreState` naming the field, the decoded value and the snapshot's value; an invocation or pre snapshot that is not among the provided documents, is not the document shape the state-clause path writes (the typed leaf shapes of `src/replay/state_clause.rs`), or lacks the object or a field returns `PreState` naming what is missing. A provided document whose bytes do not match the digest it is addressed by is not a `PreState` refusal: the tie reads the documents only after QSL's own request decode has checked every provided document against its digest, so that case returns `FrameReplayError::Refused` carrying QSL's refusal and its catalog code. None calls `replay_frame`. A forbidden-write playback replayed against the invocation of a different pre state is refused, and the invocation of its own pre state settles `reproduced-with-evaluated-witness` with category `violation`. | Test (TC-035) |
| FR-024-AC-28 | A harness whose `scope.operation` is not the operation requested returns `FrameReplayError::ScopeMismatch` naming `operation` before `call_site` is called; one whose `scope.anchor` or `scope.frame` is not the anchor or frame `call_site` names returns `ScopeMismatch` naming that member and does not call `replay_frame`. The harness's module and harness symbols name the generated artifact, are not members of the identity, and are checked only by the decode (FR-024-AC-25). | Test (TC-035) |
| FR-024-AC-29 | The frame envelope's `declared_domains` is the empty list, no source of `src/replay/frame.rs` builds a `DeclaredDomain` or a `DomainKey`, and the module's header states, once, that the declaration is empty until QSL-345 settles the declared-domain key and refuses an empty declaration. The frame path adopts no key shape before then, whatever shape another path builds. | Test (TC-035) |
| FR-024-AC-30 | With the installed backend, the real playback of the falsified frame harness of a subject that writes a forbidden field is replayed through `FrameReplay::new` and `replay`, the caller supplying the harness's `StateFrameIdentity`, the playback text and the inputs of FR-024's Inputs list but no obligation identity and no transcript, and settles a reproduced violation naming the written field. The harness is generated from QSL's emitted package and its scope node ids equal those `qsl_replay::call_site` names, without `Twin::aligned` (FR-024-AC-32). A subject that writes only a granted field leaves its harness verified and yields no playback. The test is `tc_035_real_kani_frame_counterexample_replays_through_qsl` in `kani_obligations_state_frame`, selected by `make kani`. | Test (TC-035) |
| FR-024-AC-31 | PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation pending. For a QSL-emitted package, `StateClauseReplay` takes names and order from its inputs' `state_fields` and validates each name with `model_object_fields(&object_id)`: for the twin's `balance` and unread `audit`, both are present with the accessor's 0 to 1000 inclusive range. A playback or post state omitting one returns `MissingField`; a bound name outside the list returns `UndeclaredField`; a listed name absent from the accessor returns `ModelFields { object, cause: Absent { field } }`, and an accessor error returns `ModelFields { object, cause: Accessor(error) }`, preserving the exact IR error without a body or read fallback. A non-model object or an admitted unselected model/object_type with a nonempty body causes `NotModelObjectType`, even when its body appears to contain valid ranges. A selected model/object_type whose tampered nonempty body is read by a state clause is refused by IR admission as `StaleNodeKey` before CG replay receives a package; TC-227's admitted selected tamper is unread and is not this replay fixture. After `call_site` and operation-shape validation, `ModelFields` precedes playback and post-state binding errors, including a simultaneous missing playback field; neither model-field error invokes replay or its executor. Endpoints are admitted and one below or above returns `OutOfDomain` (FR-024-AC-17). A present listed field with `member_type()` of `None`, a non-`IntRange` variant, or an `IntRange` outside `i64` has no `DeclaredDomain`; replay carries its `i64` value without range checking. Cases QSL cannot emit directly use an admitted QSL-emitted graph with selected-model-document override and recomputed digests, not a forged field-read range. | Test (TC-035) |
| FR-024-AC-32 | PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation pending. A frame harness generated from the package QSL emits for the twin is replayed through `FrameReplay::new` and `replay`: its generated `scope.anchor` and `scope.frame` equal the ids `call_site` names, asserted before replay; `Twin::aligned` is gone from test support; a forbidden write settles `reproduced-with-evaluated-witness` with category `violation` naming the written field; its `state_fields` retain request order and its `domains` equal the accessor's ranges for listed fields, including an unread ranged field; a playback value one outside a declared range returns `OutOfDomain` (FR-024-AC-26). A non-model object or admitted unselected model/object_type body is refused and supplies no range. A selected/read nonempty-body tamper is refused by IR admission before this frame replay path. | Test (TC-035) |
| FR-024-AC-33 | PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation pending. A postcondition harness generated from the package QSL emits for the twin, with no identity alignment, is replayed through `StateClauseReplay::new` and `replay`: the debit mutation settles `reproduced-with-evaluated-witness` with category `violation` and evaluated `false`; the unmutated subject over the same pre state settles `inconclusive` with cause `Verdicts` (FR-024-AC-16); the clause id is the one `call_site` names for `BalanceNeverDrops`, and the declared field ranges equal those returned by the accessor, including an unread ranged field. | Test (TC-035) |
| FR-024-AC-34 | PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation pending. With the installed backend, the real playback of the falsified frame harness of a forbidden write (FR-024-AC-30) and the falsified postcondition harness of a debit mutation (FR-024-AC-18) replay and settle as those criteria state, each harness generated from QSL's emitted package using accessor-derived ranges. The modules `kani_obligations_state_frame` and `kani_obligations_state_clause_replay` are selected by the `kani_obligations` filter of `make kani`. | Test (TC-035) |
| FR-024-AC-35 | PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation pending. A falsified frame or postcondition harness draws a present model field whose accessor type has no `i64` range (for example `IntRange` with a lower endpoint below `i64::MIN` and an upper endpoint of -1), and playback binds an `i64` value outside that model range. QSL refuses the pre state and settlement is `Inconclusive` with `ReplayRefused` (FR-029-AC-16), naming the field and its persisted `TypeNotRange` reason; it never reports `Verified` or a violation. Playback inside the model range is unaffected, and an unread present `IntRange` within `i64` is bounded and is never reported as unranged. The source and IR range ceiling is `i128::MIN..=i128::MAX`: QSL-642 shall accept source bounds in that interval and refuse larger bounds with a typed cause at check. Same-model wide-range replay evidence is blocked by QSL-642; an IR-admitted selected-model override alone is not such evidence. | Test (TC-035) |

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

At this revision the generator meets none of FR-024-AC-1 to FR-024-AC-10 in full, and meets
FR-024-AC-11 to FR-024-AC-30:

- `src/replay/witness.rs` decodes the playback into `qsl_replay::WitnessValue` and imports no
  Contract IR witness type.
- The bounded-Kani corpus retains no counterexample packet, so no corpus counterexample reaches
  QSL as the `Input` arm and FR-024-AC-5 is unbacked.
- No domain check runs before replay, and no `WitnessEnvelope` is built.
- A postcondition state-clause counterexample is replayed (IR-460): `src/replay/state_clause.rs`
  builds the invocation and snapshot documents through `core::canonical` and replays through
  `qsl_replay::replay_state_clause`, so FR-024-AC-11 to FR-024-AC-19 are backed by tagged tests
  (FR-024-AC-18 in the `kani` lane; its test uses the fixture subject that debits to the floor of
  the range, `deposit_debiting_within_range`).
- Declared-domain key divergence, open: `src/replay/state_clause.rs` keys each state field's
  `DeclaredDomain` as `DomainKey::Node { node: the self parameter node, path: [CG's field
  position] }`. QSL `main` bcca433 (ADR-012 section 15.4, QSL-345 item 4) defines a state field's
  key as `Node { the declaring object_type node, [the field's ordinal in ascending UTF-8 name
  order] }` and returns it from `call_site` as `FieldSite.domain`. Nothing breaks today, because
  QSL has no declared-domain check yet. The state-clause path should adopt `FieldSite.domain`
  before that check lands, and compute no key of its own.
- Known gap, QSL ruling pending (QSL-634 / IR-460): the post-state values are not range-checked,
  and a post state outside a field's declared range is refused by QSL admission and reads as
  `Inconclusive(ReplayRefused(InvalidRuntimeInput))` (FR-029-AC-16). The wrapping debit subject
  `deposit_debiting` reaches such a post state at the floor of `balance`'s range. QSL ruled
  (QSL-634, filed, not merged) that an out-of-range pre state or argument stays refused and that an
  out-of-range post-state value is the subject's output, so the witness: QSL will admit it as an
  exact out-of-range observation, never clamped, and settle the replay reproduced or violated
  naming the field, the range and the observed value, which CG maps to its ordinary violated
  terminal (FR-029-AC-18, planned). Until QSL-634 lands CG keeps today's behaviour.
- Unbuilt, with no owner yet: nothing in `src` decodes a state-clause harness's Kani playback into
  the named `i64` values `StateClauseReplayInputs::playback` takes. FR-016's decoder works from
  obligation bindings, which the state harness has not. The tests read the playback with a
  test-only helper (`playback_state`), so a Kani run cannot yet drive this path from `src` alone.
- The frame path's own hand-built invocation document and encoder remain in
  `tests/state_frame_support/native_twin.rs`; its `result` member is now `null`, as the twin's
  operations declare no result.
- The frame path is implemented (IR-459): `FrameReplayInputs` carries the falsified harness's
  `StateFrameIdentity` and its playback text and no obligation identity or transcript. The one
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
  FR-024-AC-35's same-model out-of-`i64` replay case remains gated on QSL-642: QSL must emit and
  replay authoritative source bounds through `i128`, while refusing larger source bounds at check.
  An IR-admitted model override checks CG's unranged handling but does not prove QSL replay of that
  overridden model. The Boolean same-source refusal and an in-`i64` source range are narrower
  executable cases; neither closes the wide-range acceptance criterion.
- `FR-024-AC-1` to `FR-024-AC-10` are planned and have no test of their own. `quire coverage
  --strict` does not count them as unbacked (66 unbacked rows on `main` before IR-460; 44 on
  `main` before IR-459 and 44, none contradicted, at the head that implemented the frame path, the
  same rows) only because TC-035 has tagged tests; that is a property of the tool's AC to TC to
  code walk, accepted here, and the criteria stay planned. The tool lists each FR-024
  criterion as an obligation row and does not judge it, so the head's unchanged count says
  nothing about FR-024-AC-20 to FR-024-AC-30; each of those has a tagged test of its own, which
  is the evidence they are backed.
- The one adapter transcript rendering function is `render_witness` in `src/replay/function.rs`,
  used by the skeleton spine, the state-clause path and the frame path.

## Open questions

None of these is decided here. FR-024-AC-24 excludes the state-clause path because of Q-1, and
FR-024-AC-1 and FR-024-AC-20 carry no digest label because of Q-3; no other criterion depends on
an answer to Q-1, Q-3 or Q-4, and a later answer to Q-3 would change every identity once.

- Q-1. The postcondition state-clause path takes a caller-supplied `obligation_identity`
  (`StateClauseReplayInputs`). Whether it takes the harness's `StateFrameIdentity` and mints the
  identity too belongs to the state-clause code; how the replay is given the state fields the
  harness draws is settled by FR-024-AC-31 (a `state_fields` member of its inputs). Packet assembly stays local to each path and whichever code change lands
  second extracts the shared piece; this requirement assigns neither.
- Q-3. Whether the identity needs a digest domain label (AD-003 R-S3, QC-4 / TK-07). Adding one
  changes every identity once.
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
  result and errors (FR-029-AC-16); QSL's `qsl-replay` crate.
- **Downstream**: [TC-035](../matrix/TC-035-counterexample-envelope-intake.md).
