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
the same proving-run members. IR-459's frame-envelope work owns the shared packet-assembly piece,
a module `src/replay/envelope.rs`, and the state-clause code builds on it once it has merged. The
state-clause code does not edit `src/replay/frame.rs`. If the state-clause code lands first, it
fills the packet inside its own module, and IR-459 then replaces that fill with the shared one in
the same change that adds the module. No second shared module is introduced.

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
  - each state field's declared integer range (FR-015-AC-27), read from the admitted package.

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
  For a state-clause counterexample the value is a state field, bound through `self`, and its
  declared domain is the integer range its framed object's body member declares (FR-015-AC-27).
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
  and `created` and `deleted` empty. The subject ABI takes `self` alone and returns nothing
  (FR-015-AC-26), and the frame of a supported clause creates and deletes nothing (FR-015-AC-29),
  so each of those four members is a consequence of the harness and not a placeholder. Whether the
  operation declares a result is QSL's admission check, and a refusal there settles as
  FR-029-AC-16 states.
- The generator shall encode each document and compute its `sha256-jcs` digest only through
  `core::canonical` (AD-004), the one RFC 8785 encoder, which gains the function that returns the
  encoded bytes beside `content_digest`. The generator shall add no second encoder and no hashing
  dependency to `[dependencies]`.
- If the playback binds no value for a declared state field, then the generator shall return
  `StateClauseReplayError::MissingField` naming the field and shall supply no default value.
- If the generator returns `StateClauseReplayError::MissingField`, then it shall not call
  `replay_state_clause`.
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
| FR-024-AC-1 | The envelope's obligation identity is QSL's `ObligationIdentity` built as the ADR-013 O-09 digest: the subject node id (clause, application or function, FR-016-AC-21), its occurrence key, the obligation kind and the arguments, each a parameter node id with its declared domain, and no `source_span`. Changing `source_span` leaves it unchanged, and changing the obligation kind or any argument binding changes it. | Test (TC-035) |
| FR-024-AC-2 | Every backend-witness transcript the generator passes to `Witness::parse` is produced by the one adapter rendering function from decoded values, and that function takes no backend-native text as input. | Test (TC-035) |
| FR-024-AC-3 | A counterexample with any value outside its declared domain is reported out-of-domain and neither `replay` nor `replay_frame` is called. The endpoints of each declared domain are admitted, and the values immediately outside it are not. | Test (TC-035) |
| FR-024-AC-4 | For each `WitnessPacket` member in turn, a packet missing that member is refused by `WitnessEnvelope::reconstruct` with QSL's `MissingMember` naming it, and no replay runs. A packet whose trace position is present with the value none is admitted. | Test (TC-035) |
| FR-024-AC-5 | A corpus counterexample is submitted as the `Input` arm keyed by parameter node id. It settles `reproduced-without-witness` when it agrees, and no `Witness` is built for it. | Test (TC-035) |
| FR-024-AC-6 | A frame counterexample is submitted as a `WitnessEnvelope<FrameCounterexample>` through `qsl_replay::replay_frame`, and its clause, frame, anchor and occurrence identities reach QSL unchanged. | Test (TC-035) |
| FR-024-AC-7 | A reduced `Witness`-arm candidate is retained only when a backend re-run and native replay both preserve domain validity and the failure; a candidate that changes domain validity or the verdict is discarded. | Test (TC-035) |
| FR-024-AC-8 | A retained reduced `Witness`-arm envelope carries its own backend re-run's transcript, never the original counterexample's. | Test (TC-035) |
| FR-024-AC-9 | A reduced `Input`-arm candidate is retained only when native replay preserves domain validity and the verdict, and it is itself an `Input`-arm envelope. | Test (TC-035) |
| FR-024-AC-10 | No type named `Witness`, `ReplaySource`, `WitnessEnvelope`, `TerminalRecord` or `ObligationIdentity` is defined under `src/`, and no source file under `src/` imports any of them from `quire_contract_ir`. | Test (TC-035) |
| FR-024-AC-11 | `StateClauseReplay::replay` returns exactly the `StateClauseReplayResult` that `qsl_replay::replay_state_clause` returns when a test calls it directly with the same request and envelope, for a violating run and for a respecting run, and the two results differ from each other. `StateClauseReplay::replay_through` hands the executor the request and envelope it built and returns the executor's value as it is: an executor returning a sentinel result changes the returned result to that sentinel. PLANNED (IR-460). | Test (TC-035) |
| FR-024-AC-12 | The state-clause envelope's `clause_node` equals the `ClauseSite` node, and its `occurrence_key` the `ClauseSite` occurrence, that `qsl_replay::call_site` returns for the clause name, and the payload's `clause` is that name and its `observation` is the `Invocation` arm. For a unit with two postcondition clauses, each clause's envelope carries its own node and occurrence. A clause name the unit does not declare returns `StateClauseReplayError::CallSite` holding the call site's refusal when the request is built, and the executor is not called. PLANNED (IR-460). | Test (TC-035) |
| FR-024-AC-13 | The pre snapshot holds, for every declared field of the state object, the value the Kani playback bound to that field, and the post snapshot holds the post-state value the caller supplied for it; each is one `complete` population holding one object with each field as an integer. The invocation document holds the members the Behavior bullets state, with `parameters` empty, `result` null and `created` and `deleted` empty, and its `pre` and `post` carry each snapshot's identity and `sha256-jcs` digest. Changing one playback value changes the pre snapshot's bytes and digest, and changing one post-state value changes the post snapshot's. PLANNED (IR-460). | Test (TC-035) |
| FR-024-AC-14 | The documents' bytes are RFC 8785 text and each digest is the SHA-256 of those bytes as `core::canonical` computes it: a snapshot with an integer above 2^53 and members in non-sorted source order equals the canonical encoding, the source of the new state-clause module names neither `quire_canonical`'s encoder functions nor `sha2`, and `sha2` stays out of `[dependencies]`. PLANNED (IR-460). | Test (TC-035) |
| FR-024-AC-15 | A playback that binds no value for a declared state field returns `StateClauseReplayError::MissingField` naming that field, the executor is not called, and no snapshot holds a default for it. PLANNED (IR-460). | Test (TC-035) |
| FR-024-AC-16 | For a subject mutated to violate the postcondition, the replay settles `reproduced-with-evaluated-witness` with category `violation` and an evaluated `false`. For the unmutated subject's run over the same pre state it settles `inconclusive` with cause `Verdicts` (`violation` proved, `success` replayed). The envelope is on the `Witness` arm with a payload `witness` of none in both cases. PLANNED (IR-460). | Test (TC-035) |
| FR-024-AC-17 | A state field value outside its declared integer range returns `StateClauseReplayError::OutOfDomain` naming the field and the executor is not called. The range's two endpoints are admitted and the values one below and one above are not. PLANNED (IR-460). | Test (TC-035) |
| FR-024-AC-18 | With the installed backend, the falsified operation-contract harness of a postcondition state clause over a subject mutated to debit is replayed from its real Kani playback through `replay_state_clause` and settles `reproduced-with-evaluated-witness`, `violation`. The test is `tc_035_real_kani_state_clause_counterexample_replays_through_qsl` in the module `kani_obligations_state_clause_replay`, so the `kani_obligations` filter of `make kani` selects it. PLANNED (IR-460). | Test (TC-035) |

A transcript `Witness::parse` refuses is an adapter
refusal under FR-016-AC-11.

## Current state

At this revision the generator meets none of these criteria in full:

- `src/replay/witness.rs` decodes the playback into `qsl_replay::WitnessValue` and imports no
  Contract IR witness type.
- The bounded-Kani corpus retains no counterexample packet, so no corpus counterexample reaches
  QSL as the `Input` arm and FR-024-AC-5 is unbacked.
- No domain check runs before replay, and no `WitnessEnvelope` is built.
- No state-clause counterexample is replayed: `src` has no consumer of
  `qsl_replay::replay_state_clause` or of the `ClauseSite` selection, and no code under `src`
  builds an invocation or snapshot document (only `tests/state_frame_support/native_twin.rs`
  does, with its own encoder), so FR-024-AC-11 to FR-024-AC-18 are unbacked (IR-460). `qsl-replay` at the locked revision exports
  `replay_state_clause`, `StateClauseCounterexample`, `StateClauseReplayResult`,
  `ClauseSelectionInput`, `ClauseSite` and `ClauseName`.
- Only the skeleton spine renders a QSL transcript (`src/replay/function.rs`), and it builds the
  request without an envelope.

## Dependencies

- **Upstream**: [FR-016](./FR-016-witness-native-replay.md), which decodes the playback, refuses and
  partitions the outcomes; [FR-017](../../kani/functional/FR-017-kani-execution-evidence.md), which retains the
  playback; [FR-025](../../kani/functional/FR-025-generated-subject-abi.md), which gives each argument binding its
  parameter node id and declared domain; [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md), whose
  operation-contract harness (FR-015-AC-26) is the state-clause counterexample's source;
  [FR-029](../../kani/functional/FR-029-run-outcome-terminal-record.md), which reads the state-clause
  result and errors (FR-029-AC-16); QSL's `qsl-replay` crate.
- **Downstream**: [TC-035](../matrix/TC-035-counterexample-envelope-intake.md).
