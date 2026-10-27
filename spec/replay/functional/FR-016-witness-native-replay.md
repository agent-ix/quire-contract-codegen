---
id: FR-016
title: "Decode and natively replay complete-V1 proof witnesses"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-197
    type: references
  - target: ix://agent-ix/quire-specification/FR-333
    type: references
  - target: ix://agent-ix/quire-specification/TC-219
    type: references
---
# FR-016: Decode and natively replay complete-V1 proof witnesses

## Description

When a FR-015 harness yields a counterexample, the code generator shall decode
the witness into typed complete-V1 values, validate each value against its
declared domain, and replay it through native runtime execution before any
failure is reported.

## Inputs

- A retained Kani counterexample and the harness identity that
  produced it.
- The obligation's declared input domains.
- Caller-supplied witness decode limits, in bytes and value count. Both bounds are required,
  non-optional fields of replay inputs. Every witness-decoding entry point receives them as a
  required parameter or through a required field of its inputs; the public
  `decode_falsification` and `StateClauseReplayInputs::decode_falsification` also take a direct
  parameter. The decode-limit type and replay inputs have no `Default`.
  QSL's `ReplayLimits.input_bytes` bounds the replay request after CG decodes playback and does
  not bound CG's witness decoding.

## Outputs

- A typed replay result (reproduced failure or evidence failure), or a typed
  replay error. A `ReplayRefusal::Fault` remains in the replay error and maps to
  terminal `Failed` under FR-029-AC-10. Evidence failure is one verdict
  with a typed cause: a malformed witness (decode cause), an out-of-domain witness
  (domain cause) or a mismatch (verdict cause). This document's "malformed
  witness", "out-of-domain witness" and "mismatch" name those three causes, and
  none of them is ever reported as a clause success or failure.

## Behavior

- When a counterexample is retained, the generator shall check that it is
  bound to the identity of the harness being replayed, and decode
  each witness value into its declared complete-V1 scalar type within the
  caller-supplied witness byte and value-count limits.
- When the installed backend classifies a run as falsified, the generator shall retain the first
  eligible assertion playback for that harness in the run's captured launcher output order after
  excluding cover blocks, preserve its bytes through witness decoding, then replay the values
  decoded from that retained playback.
- The generator shall read the concrete values of a Kani playback only
  through the Kani adapter's output module, `src/kani/output/playback.rs`.
  At this revision `src/replay/witness.rs` selects and reads the block through
  that module's `select_assertion_block` and `read_block` and decodes the typed
  values from what they return.
- The generator shall decode a witness against the harness's own persisted
  obligation schema: the obligation's argument bindings, in the order the
  obligation persists them, typed position for position against the
  harness's symbolic arguments, and shall name each decoded value by the
  binding at its position. The harness emits its symbolic arguments in that
  same persisted order (FR-015).
- If an obligation binding is not an argument, then the generator shall refuse
  the witness schema with a typed schema refusal that reports no failure. The
  refusal is a fault in the schema, not in the witness, so it is not a
  replay result. A binding that is not an argument has no symbolic
  position, and typing it would mistype a position that does not exist in the
  witness bytes.
- If a witness is bound to a different harness identity, fails to
  decode, or exceeds either supplied decode bound, then the generator shall report a
  malformed witness and shall not report a failure.
- If a decoded value is outside its declared domain, then the generator shall
  report an out-of-domain witness and shall not report a failure.
- When a witness is in domain, the generator shall replay it through native
  runtime execution and compare the typed outcome with the harness result.
  The same typed outcome means an equal value or typed refusal. Kani reports no
  scalar accounting charges or consumed counters to compare with QSL's replay
  charges; the two tools' limit types bound different work and are not compared.
- The generator shall classify every witness into exactly one of the
  outcomes in the partition below. The conditions are checked in the order
  the table lists them, and the first that holds decides the outcome.

  | Condition | Outcome |
  |---|---|
  | The witness is bound to another harness identity, fails to decode, or exceeds either supplied decode bound | malformed witness |
  | A decoded value lies outside its declared domain | out-of-domain witness |
  | The adapter cannot build an admitted request: a decoded value no replay parameter binds, a transcript field holding a delimiter, a transcript `Witness::parse` refuses, or an envelope QSL refuses to reconstruct | adapter refusal (FR-016-AC-11) |
  | `qsl_replay::replay` returns `Err(ReplayRefusal::Fault(_))`, an internal fault of the executor | typed replay refusal retaining the fault; the terminal value is `Failed` (FR-029-AC-10) |
  | `qsl_replay::replay` returns any other `Err(ReplayRefusal)` | QSL request refusal, returned with its cause (FR-016-AC-11) |
  | `qsl_replay::replay` returns the `Input` arm for a witness-sourced request | adapter refusal (FR-016-AC-11) |
  | The `Witness` arm settles `inconclusive` | mismatch |
  | The `Witness` arm settles `reproduced-with-evaluated-witness` with category `violation` | reproduced failure |
  | The `Witness` arm settles `reproduced-with-evaluated-witness` with any category other than `violation` | mismatch |

- If native replay runs and disagrees with the harness result, then the
  generator shall report a typed mismatch rather than a failure.
- If the executor reports an internal fault, then the generator shall retain
  it as `SpineReplayError::Refused(ReplayRefusal::Fault)`.
  FR-029-AC-10 reads that fault as terminal `Failed`.
- If the `Witness` arm settles `reproduced-with-evaluated-witness` with a
  category other than `violation`, then the generator shall report a typed
  mismatch. QSL's `WitnessArmResult::settle` takes the category independently
  of the settlement, and only `violation` is a false predicate, so an
  agreement in any other category does not reproduce the falsification the
  harness reported.
- The generator shall replay through QSL's layer-6 `replay` facade
  (`qsl_replay::replay`). The caller supplies the complete request: the proved
  package's `package_id`, the selected function's qualified name, the limits, and
  the byte provision holding the proved unit's source; the
  generator reads no path. The generator supplies the request's replay source,
  the decoded values as a backend-witness transcript keyed by parameter node id,
  and calls `replay`. QSL recompiles the source and evaluates the selected
  function, so the replayed verdict is QSL's evaluation and never a value this
  generator supplies. A settlement of `reproduced-with-evaluated-witness` with
  category `violation` is the reproduced failure, a settlement of `inconclusive`
  is a mismatch, and an executor refusal is returned with its own cause.

- The generator shall fill the function path's request `obligation_identity` with the ADR-013
  O-09 function-contract obligation identity of the function being replayed (FR-016-AC-21 to
  AC-23).

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-016-AC-1 | Every retained witness decodes into typed values or is refused as malformed. | Test (TC-026) |
| FR-016-AC-2 | An out-of-domain witness is reported as such and never as a contract failure. | Test (TC-026) |
| FR-016-AC-3 | An in-domain witness is reported as a failure only when native replay reproduces the same typed outcome. | Test (TC-026) |
| FR-016-AC-4 | A native replay that runs and disagrees with the harness result yields a typed mismatch, and never a failure or a replay error. | Test (TC-026) |
| FR-016-AC-5 | A witness bound to a different harness identity is reported as malformed and never replayed. | Test (TC-026) |
| FR-016-AC-6 | Every witness decode receives required caller-supplied byte and value-count bounds at its entry point; replay inputs carry both as non-optional fields with no default, and an over-bound witness is reported malformed without reading beyond either bound. | Test (TC-026) |
| FR-016-AC-8 | A witness schema binds the obligation's persisted argument bindings, in the order the obligation persists them, position for position to the harness's symbolic arguments, and each decoded value is named by the binding at its position; a binding that is not an argument is refused with a typed schema refusal that reports no failure and is none of the replay results. | Test (TC-026) |
| FR-016-AC-9 | A decoded falsification whose witness makes the native function evaluate the clause to false settles as `reproduced-with-evaluated-witness` with category `violation`; the verdict depends on the witness value, so a witness at which the function holds does not settle so. | Test (TC-026) |
| FR-016-AC-10 | A decoded falsification at which the native function evaluates the clause to true settles `inconclusive` with the proved violation and the replayed success both named. | Test (TC-026) |
| FR-016-AC-11 | The adapter refuses, with a distinct typed error each, a decoded value no replay parameter binds, a transcript field holding a delimiter, a transcript QSL does not admit, a request QSL refuses with any `ReplayRefusal` other than `Fault` (returned with its cause), and a witness-sourced request that settles on the input arm. | Test (TC-026) |
| FR-016-AC-12 | A `replay` call that returns `ReplayRefusal::Fault` is returned as `SpineReplayError::Refused` carrying that fault. | Test (TC-026) |
| FR-016-AC-13 | A `Witness` arm that settles `reproduced-with-evaluated-witness` with a category other than `violation` yields a typed mismatch, and never a reproduced failure. | Test (TC-026) |
| FR-016-AC-14 | A proved unit that imports a locked dependency is compiled by `qsl_replay::call_site` together with that dependency's lock source, and `qsl_replay::replay` settles the unit's falsification through the imported function. | Test (TC-026) |
| FR-016-AC-15 | The replay request's `package.dependencies` carries one entry per lock dependency selection, in ascending identity order, with the lock's recorded `package_id`. | Test (TC-026) |
| FR-016-AC-16 | A unit whose import no lock dependency selection supplies is refused at the call site. | Test (TC-026) |
| FR-016-AC-17 | A lock recording another `package_id` for a dependency the unit imports is refused by QSL as a dependency identity mismatch. | Test (TC-026) |
| FR-016-AC-18 | A lock selecting a library the unit does not import is refused by QSL as unselected. | Test (TC-026) |
| FR-016-AC-19 | Lock libraries whose sources share a source owner are refused as no dependency input, before the call site compiles anything. | Test (TC-026) |
| FR-016-AC-20 | A lock library whose source has the unit's own source owner is refused by the call site. | Test (TC-026) |
| FR-016-AC-21 | The function path's request `obligation_identity` holds the function-contract obligation digest of quire-spec-language ADR-013 O-09: the digest of the RFC 8785 encoding, made by `quire-canonical` (ADR-013 §2's one encoder, AD-003 E-1), of the `FunctionSite`'s `function` node id, its `declaration` occurrence key, the `ObligationKind` of the harness being replayed and `arguments`, each a parameter node id with its declared domain, ordered ascending by identifier as O-09 fixes and not in the declared order of `FunctionSite.parameters`. It is never the digest of the transcript. The `arguments` are all of the function's parameters: a harness that leaves a parameter without an argument, or declares an unbounded integer or a bounded Boolean, has no identity and is refused. | Test (TC-026) |
| FR-016-AC-22 | For two functions with the same parameters, the identities differ. The identity of one function is the same across recompiles that only add comments or blank lines, and across a change of the harness's source span. It differs between two different `ObligationKind`s over the same function, and when any argument's domain differs. A function yields one identity per existing `ObligationKind` it requests, not one per conjunct of its body: a function whose body has two conjuncts, replayed under one kind, has one identity. | Test (TC-026) |
| FR-016-AC-23 | The function-contract identity function derives nothing from the package: with the `FunctionSite`'s `function` and `declaration`, the kind and the arguments held fixed, perturbing every other node of the compiled package (an unrelated declaration, the order of `FunctionSite.parameters`) leaves the identity unchanged, and a perturbed `function` or `declaration` changes it. | Test (TC-026) |
| FR-016-AC-24 | A lock whose only defect is one library identity selected twice is refused by QSL's `DependencyInput::new` as `DuplicateIdentity`, code `invalid_package` with cause `conflicting-definition`, and reaches the caller as `DependencyLockError::Input`. | Test (TC-026) |
| FR-016-AC-25 | In an installed-Kani run that classifies a generated harness as `Falsified`, the retained counterexample is byte-identical to the first eligible assertion playback for that harness in the same launcher's complete captured stdout order, excluding cover blocks, and native replay uses values decoded from that retained counterexample; a `Verified` run supplies no counterexample. A test oracle observing stdout independently compares the complete fence interior, trimmed only of surrounding whitespace, with the retained counterexample before decoding or replay. Substituting another valid, self-consistent playback immediately after classification fails that oracle even when the replacement's comments, bytes, decoded values and replay agree. | Test (TC-026) |

## Status

FR-016-AC-1 through AC-5 and AC-21 through AC-23 retain default-suite evidence through a
direct test-only `KaniObligationIdentity` fixture and the surviving replay and identity APIs.
That fixture is an input to those APIs, not a replacement V1 generator or a proof claim.
FR-016-AC-9 and AC-10 retain their default native replay tests, but the
`skeleton_spine.rs` installed-prover E7-E9 control was removed with the V1 `BoundClause` generator.
That installed path is Open and partially backed until IR-1047 supplies a genuine V2 generated
harness and real-Kani counterexample control. The original AC text and verification methods
remain unchanged.

FR-016-AC-8 remains Open and partially backed after the 2026-10-27 IR-348 retirement ruling.
The positional decoder and typed non-argument refusal tests in `src/replay/witness.rs` survive.
The V2 `state_clause_kani_playback_adapter_refuses_wrong_evidence` test in
`tests/it/kani_obligations_state_clause_replay.rs` also survives: it checks a generated
state-frame postcondition's persisted draw order against playback and refuses mismatched
assertion evidence. It does not compare emitted symbolic argument order with persisted bindings
or execute the assertion in real Kani. The former emission-order and real-Kani persisted-schema
controls in `tests/it/kani_argument_order.rs` and `tests/it/kani_witness_join.rs` used the retired
V1 `BoundClause` producer and were removed. A genuine V2 producer must supply persisted argument bindings and
emitted symbolic arguments for corresponding order, naming and real-Kani decode evidence.
The original AC-8 wording and verification method above remain unchanged.

FR-016-AC-25 remains Planned/Open under IR-604. The removed
`tc_026_installed_kani_falsification_is_bound_to_process_stdout` used a V1 `BoundClause`
harness. It compared the retained counterexample byte-for-byte with the first eligible assertion
playback in independently captured stdout, excluding cover blocks, and rejected a valid
post-classification substitute. That V1 evidence retired with its producer and gives no
credit to a surviving harness. The observing-launcher helper and the other three AC-25-tagged
tests have no V1 dependency and survive. IR-604 still owes the same independent stdout binding
and substitution control on a surviving generated harness. The original AC-25 wording and
verification method above remain unchanged.

## Dependencies

- **Upstream**: [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md); QSL's
  `qsl-replay` crate, the only QSL crate this
  repository depends on. The replay package takes a compiled unit's package id and
  parameter node ids from `qsl_replay::call_site`, which compiles the unit with its
  locked dependencies, and the request's types from `qsl-replay`'s re-exports.
- **Downstream**: [TC-026](../matrix/TC-026-witness-native-replay.md),
  [FR-024](./FR-024-counterexample-envelope-intake.md), which carries the decoded
  values to QSL in QSL's counterexample envelope.

## Rationale

FR-016-AC-7 was retired under IR-604. The installed Kani runner's
`KaniExecutionEvidence` carries proof ceilings, an observed memory peak,
backend checks and an outcome, but no admitted scalar charges or consumed
counters. QSL's `WitnessArmResult` carries replay charges;
`ReplayInputs.accounting_limits` and `ReplayInputs.replay_limits` bound the QSL
replay, while `ReplayInputs.decode_limits` bounds CG witness decoding. None
reports Kani proof accounting. Comparing these as if they were two observations
of the same accounting fact would invent proof evidence.
The typed value/refusal and verdict comparisons remain required by AC-3, AC-4,
AC-9, AC-10 and AC-13. AC-6 continues to bound witness decoding independently.
