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

QSL owns the envelope, the witness, the replay source, the FR-331 terminal record and the obligation
identity type (QSL ADR-013 O-25 to O-27). This requirement states what the generator puts into
those types. It does not restate their shape. [FR-016](./FR-016-witness-native-replay.md) owns
decoding a Kani playback into typed values and settling the replay result, and this requirement
owns what reaches QSL.

## Inputs

- A counterexample from one of two sources:
  - a backend counterexample: an [FR-017](./FR-017-pinned-kani-execution-evidence.md) `falsified`
    run's playback, decoded by FR-016 into values named by their argument bindings;
  - a corpus counterexample: canonical assignments that no backend run produced.
- The obligation's `KaniObligationIdentity` and its argument bindings, each naming its parameter
  node id and declared domain ([FR-025](./FR-025-generated-subject-abi.md)).
- The proving run's members, supplied by the caller: the FR-322 package reference with one
  dependency entry per `dependency_selections` entry (QSL ADR-015 D-4), the selected function's
  `QualifiedName`, the failing node's occurrence key, the semantic profile selections, the
  accounting limits and S1 to S4 stage limits, the `backend` member, the trace position where the
  family has one, and the byte provision.

## Outputs

- A `WitnessEnvelope` whose `ReplaySource` is `Witness` for a backend counterexample and `Input`
  for a corpus counterexample, and the replay request built from it.
- The QSL replay result for that envelope's arm, or a typed refusal with no replay.

## Behavior

- The generator shall compute the obligation-identity digest over every `KaniObligationIdentity`
  member except `source_span`, and shall carry it only as `qsl_replay::ObligationIdentity`.
- The generator shall turn a backend's native counterexample into QSL's backend-witness transcript
  in exactly one function of that backend's adapter, and shall admit the transcript only through
  `qsl_replay::Witness::parse`.
- No generator code outside the backend adapter shall read backend-native counterexample text.
- If `Witness::parse` refuses a rendered transcript, then the generator shall return a typed
  refusal carrying QSL's cause, and shall not call `replay`.
- When a counterexample is to be replayed, the generator shall check every decoded or canonical
  value against the declared domain of the parameter it binds before it builds a replay request.
- If a value lies outside its declared domain, then the generator shall report an out-of-domain
  counterexample and shall not call `replay` or `replay_frame`.
- The generator shall copy every envelope member from the proving run's members and the obligation
  identity, and shall invent none.
- If any member of the envelope is absent, then the generator shall refuse the counterexample with
  a typed cause naming that member, before any replay.
- When a counterexample did not come from a backend transcript, the generator shall submit it as
  the `Input` arm, keyed by parameter node id, and shall never build a `Witness` for it.
- When a counterexample is a frame counterexample, the generator shall submit a
  `WitnessEnvelope<FrameCounterexample>` through `qsl_replay::replay_frame`.
- Where the generator derives reduced candidates from a `Witness`-arm counterexample, it shall
  retain a candidate only as a new envelope revision linked to its parent, after both a backend
  re-run and native replay preserve domain validity and the failure.
- Where the generator retains a reduced `Witness`-arm revision, the revision shall carry the
  `Witness` arm of its own backend re-run and not its parent's transcript.
- Where the generator derives reduced candidates from an `Input`-arm counterexample, it shall
  retain only `Input`-arm revisions linked to their parent that preserve domain validity and the
  native verdict.
- The generator shall define none of `Witness`, `ReplaySource`, the counterexample envelope, the
  FR-331 terminal record or the obligation identity type.
- The generator shall import none of those types from Contract IR.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-024-AC-1 | The envelope's obligation identity is QSL's `ObligationIdentity` wrapping the digest over every `KaniObligationIdentity` member except `source_span`. Changing `source_span` leaves it unchanged, and changing the obligation kind or any argument binding changes it. | Test (TC-035) |
| FR-024-AC-2 | A backend counterexample reaches QSL only as a transcript that `qsl_replay::Witness::parse` admitted, rendered by the one adapter function. No other non-test source file under `src/` reads backend-native counterexample text, and a transcript QSL refuses returns a typed refusal with no replay. | Test (TC-035) |
| FR-024-AC-3 | A counterexample with any value outside its declared domain is reported out-of-domain and neither `replay` nor `replay_frame` is called. The endpoints of each declared domain are admitted, and the values immediately outside it are not. | Test (TC-035) |
| FR-024-AC-4 | For each envelope member in turn (package reference, dependency entries, selected function, occurrence key, profile selections, limits, declared domains, backend), a counterexample missing that member is refused with a typed cause naming it, and no replay runs. | Test (TC-035) |
| FR-024-AC-5 | A corpus counterexample is submitted as the `Input` arm keyed by parameter node id. It settles `reproduced-without-witness` when it agrees, and no `Witness` is built for it. | Test (TC-035) |
| FR-024-AC-6 | A frame counterexample is submitted as a `WitnessEnvelope<FrameCounterexample>` through `qsl_replay::replay_frame`, and its clause, frame, anchor and occurrence identities reach QSL unchanged. | Test (TC-035) |
| FR-024-AC-7 | A minimized `Witness`-arm revision is retained only when a backend re-run and native replay both preserve domain validity and the failure. It links to its parent and carries its own re-run's transcript, and a candidate that changes domain validity or the verdict is discarded. | Test (TC-035) |
| FR-024-AC-8 | A minimized `Input`-arm revision is retained only when native replay preserves domain validity and the verdict. It links to its parent and is itself an `Input`-arm envelope. | Test (TC-035) |
| FR-024-AC-9 | No type named `Witness`, `ReplaySource`, `WitnessEnvelope`, `TerminalRecord` or `ObligationIdentity` is defined under `src/`, and no source file under `src/` imports any of them from `quire_contract_ir`. | Test (TC-035) |

## Dependencies

- **Upstream**: [FR-016](./FR-016-witness-native-replay.md), which decodes the playback and settles
  the result; [FR-017](./FR-017-pinned-kani-execution-evidence.md), which retains the playback;
  [FR-025](./FR-025-generated-subject-abi.md), which gives each argument binding its parameter node
  id and declared domain; QSL's `qsl-replay` crate at the revision `Cargo.toml` pins.
- **Downstream**: [TC-035](../../test/complete-v1/TC-035-counterexample-envelope-intake.md).
