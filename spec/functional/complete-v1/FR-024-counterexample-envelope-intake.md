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
`qsl_replay::replay_frame` for a frame counterexample.

The envelope, the witness, the replay source, the FR-331 terminal record and the obligation
identity type are QSL's, as the `qsl-replay` API defines them.
[AD-001](../../assurance/AD-001-codegen-architecture.md) states that ownership and the upstream
records that still differ from it.

This requirement states what the generator puts into those types, and does not restate their shape.
[FR-016](./FR-016-witness-native-replay.md) owns decoding a Kani playback, the adapter refusals and
the partition of replay outcomes. This requirement owns what reaches QSL.

## Inputs

- A counterexample from one of two sources:
  - a backend counterexample: an [FR-017](./FR-017-kani-execution-evidence.md) `falsified`
    run's playback, decoded by FR-016 into values named by their argument bindings;
  - a corpus counterexample: canonical assignments that no backend run produced.
- The obligation's `KaniObligationIdentity` and its argument bindings, each naming its parameter
  node id and declared domain ([FR-025](./FR-025-generated-subject-abi.md)).
- The proving run's envelope members, supplied by the caller: every `WitnessPacket` member QSL
  defines, the trace position included (present in every packet, with the value none for a family
  that has no trace position).
- The proving run's request members, which belong to the replay request and not to the envelope:
  the dependency entries (QSL ADR-015 D-4), the state environment, the accounting limits, the S1 to
  S4 stage limits and the byte provision.

## Outputs

- A `WitnessEnvelope` admitted through `WitnessEnvelope::reconstruct`, whose `ReplaySource` is
  `Witness` for a backend counterexample and `Input` for a corpus counterexample.
- The replay request built from that envelope and the request members.
- The QSL replay result for the envelope's arm, or a typed refusal with no replay.

## Behavior

- The generator shall build the envelope's `qsl_replay::ObligationIdentity` from every
  `KaniObligationIdentity` member except `source_span`.
- The generator shall build QSL's backend-witness transcript from FR-016's decoded values, never
  from backend-native text, in exactly one function of the backend's adapter.
- The generator shall admit that transcript only through `qsl_replay::Witness::parse`.
- When a counterexample is to be replayed, the generator shall check every decoded or canonical
  value against the declared domain of the parameter it binds before it builds a replay request.
- If a value lies outside its declared domain, then the generator shall report an out-of-domain
  counterexample and shall not call `replay` or `replay_frame`.
- The generator shall fill every `WitnessPacket` member from the proving run's envelope members and
  the obligation identity, and shall invent none.
- The generator shall admit the envelope only through `WitnessEnvelope::reconstruct`.
- If `WitnessEnvelope::reconstruct` refuses the packet, then the generator shall return QSL's
  `WitnessRefusal` and shall not call `replay` or `replay_frame`.
- The generator shall build the replay request from the admitted envelope and the proving run's
  request members, and shall invent no request member.
- When a counterexample did not come from a backend transcript, the generator shall submit it as
  the `Input` arm, keyed by parameter node id, and shall never build a `Witness` for it.
- When a counterexample is a frame counterexample, the generator shall submit a
  `WitnessEnvelope<FrameCounterexample>` through `qsl_replay::replay_frame`.
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
| FR-024-AC-1 | The envelope's obligation identity is QSL's `ObligationIdentity` built from every `KaniObligationIdentity` member except `source_span`. Changing `source_span` leaves it unchanged, and changing the obligation kind or any argument binding changes it. | Test (TC-035) |
| FR-024-AC-2 | Every backend-witness transcript the generator passes to `Witness::parse` is produced by the one adapter rendering function from decoded values, and that function takes no backend-native text as input. | Test (TC-035) |
| FR-024-AC-3 | A counterexample with any value outside its declared domain is reported out-of-domain and neither `replay` nor `replay_frame` is called. The endpoints of each declared domain are admitted, and the values immediately outside it are not. | Test (TC-035) |
| FR-024-AC-4 | For each `WitnessPacket` member in turn, a packet missing that member is refused by `WitnessEnvelope::reconstruct` with QSL's `MissingMember` naming it, and no replay runs. A packet whose trace position is present with the value none is admitted. | Test (TC-035) |
| FR-024-AC-5 | A corpus counterexample is submitted as the `Input` arm keyed by parameter node id. It settles `reproduced-without-witness` when it agrees, and no `Witness` is built for it. | Test (TC-035) |
| FR-024-AC-6 | A frame counterexample is submitted as a `WitnessEnvelope<FrameCounterexample>` through `qsl_replay::replay_frame`, and its clause, frame, anchor and occurrence identities reach QSL unchanged. | Test (TC-035) |
| FR-024-AC-7 | A reduced `Witness`-arm candidate is retained only when a backend re-run and native replay both preserve domain validity and the failure; a candidate that changes domain validity or the verdict is discarded. | Test (TC-035) |
| FR-024-AC-8 | A retained reduced `Witness`-arm envelope carries its own backend re-run's transcript, never the original counterexample's. | Test (TC-035) |
| FR-024-AC-9 | A reduced `Input`-arm candidate is retained only when native replay preserves domain validity and the verdict, and it is itself an `Input`-arm envelope. | Test (TC-035) |
| FR-024-AC-10 | No type named `Witness`, `ReplaySource`, `WitnessEnvelope`, `TerminalRecord` or `ObligationIdentity` is defined under `src/`, and no source file under `src/` imports any of them from `quire_contract_ir`. | Test (TC-035) |

A transcript `Witness::parse` refuses is an adapter
refusal under FR-016-AC-11.

## Current state

At this revision the generator meets none of these criteria in full:

- `src/kani_witness_join.rs` imports Contract IR's `Witness`.
- The bounded-Kani corpus retains no counterexample packet, so no corpus counterexample reaches
  QSL as the `Input` arm and FR-024-AC-5 is unbacked.
- No domain check runs before replay, and no `WitnessEnvelope` is built.
- Only the skeleton spine renders a QSL transcript (`src/spine_replay.rs`), and it builds the
  request without an envelope.

## Dependencies

- **Upstream**: [FR-016](./FR-016-witness-native-replay.md), which decodes the playback, refuses and
  partitions the outcomes; [FR-017](./FR-017-kani-execution-evidence.md), which retains the
  playback; [FR-025](./FR-025-generated-subject-abi.md), which gives each argument binding its
  parameter node id and declared domain; QSL's `qsl-replay` crate.
- **Downstream**: [TC-035](../../test/complete-v1/TC-035-counterexample-envelope-intake.md).
