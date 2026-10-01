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
- A decode size limit bounding the witness bytes and value count read.

## Outputs

- A typed replay result: reproduced failure, evidence failure, or replay
  unavailable, carried as a FR-333 method result. Evidence failure is one verdict
  with a typed cause: a malformed witness (decode cause), an out-of-domain witness
  (domain cause) or a mismatch (verdict cause). This document's "malformed
  witness", "out-of-domain witness" and "mismatch" name those three causes, and
  none of them is ever reported as a clause success or failure.

## Behavior

- When a counterexample is retained, the generator shall check that it is
  bound to the identity of the harness being replayed, and decode
  each witness value into its declared complete-V1 scalar type within the
  decode size limit.
- The generator shall read the concrete values of a Kani playback only
  through the Kani adapter's transcript module, `src/kani_transcript.rs`.
  At this revision the witness join
  (`src/kani_witness_join.rs`) decodes the playback itself rather than
  through the transcript module, so this bullet states the target and is not
  yet met.
- The generator shall decode a witness against the harness's own persisted
  obligation schema: the obligation's argument bindings, in the order the
  obligation persists them, typed position for position against the
  harness's symbolic arguments, and shall name each decoded value by the
  binding at its position. The harness emits its symbolic arguments in that
  same persisted order (FR-015).
- If an obligation binding is not an argument, then the generator shall refuse
  the witness schema with a typed schema refusal that reports no failure. The
  refusal is a fault in the schema, not in the witness, so it is not one of
  the five replay results. A binding that is not an argument has no symbolic
  position, and typing it would mistype a position that does not exist in the
  witness bytes.
- If a witness is bound to a different harness identity, fails to
  decode, or exceeds the decode size limit, then the generator shall report a
  malformed witness and shall not report a failure.
- If a decoded value is outside its declared domain, then the generator shall
  report an out-of-domain witness and shall not report a failure.
- When a witness is in domain, the generator shall replay it through native
  runtime execution and compare the typed outcome with the harness result.
  The same typed outcome means an equal value or typed refusal, equal admitted
  charges, equal consumed counters, and equal limits.
- The generator shall classify every witness into exactly one of the
  outcomes in the partition below. The conditions are checked in the order
  the table lists them, and the first that holds decides the outcome.

  | Condition | Outcome |
  |---|---|
  | The witness is bound to another harness identity, fails to decode, or exceeds the decode size limit | malformed witness |
  | A decoded value lies outside its declared domain | out-of-domain witness |
  | The adapter cannot build an admitted request: a decoded value no replay parameter binds, a transcript field holding a delimiter, a transcript `Witness::parse` refuses, or an envelope QSL refuses to reconstruct | adapter refusal (FR-016-AC-11) |
  | `qsl_replay::replay` returns `Err(ReplayRefusal::Fault(_))`, an internal fault of the executor | replay unavailable |
  | `qsl_replay::replay` returns any other `Err(ReplayRefusal)` | QSL request refusal, returned with its cause (FR-016-AC-11) |
  | `qsl_replay::replay` returns the `Input` arm for a witness-sourced request | adapter refusal (FR-016-AC-11) |
  | The `Witness` arm settles `inconclusive`, or matches the harness value but differs in admitted charges, consumed counters or limits | mismatch |
  | The `Witness` arm settles `reproduced-with-evaluated-witness` with category `violation` | reproduced failure |
  | The `Witness` arm settles `reproduced-with-evaluated-witness` with any category other than `violation` | mismatch |

- If native replay runs and disagrees with the harness result, then the
  generator shall report a typed mismatch rather than a failure or an
  unavailable result.
- If the executor reports an internal fault, then the generator shall report
  a typed unavailable result rather than a failure, a refusal or a mismatch.
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

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-016-AC-1 | Every retained witness decodes into typed values or is refused as malformed. | Test (TC-026) |
| FR-016-AC-2 | An out-of-domain witness is reported as such and never as a contract failure. | Test (TC-026) |
| FR-016-AC-3 | An in-domain witness is reported as a failure only when native replay reproduces the same typed outcome. | Test (TC-026) |
| FR-016-AC-4 | A native replay that runs and disagrees with the harness result yields a typed mismatch, and never an unavailable result or a failure. | Test (TC-026) |
| FR-016-AC-5 | A witness bound to a different harness identity is reported as malformed and never replayed. | Test (TC-026) |
| FR-016-AC-6 | A witness over the decode size limit is reported as malformed without decoding past the limit. | Test (TC-026) |
| FR-016-AC-7 | A native replay that matches the harness value but differs in admitted charges, consumed counters or limits yields a typed mismatch. | Test (TC-026) |
| FR-016-AC-8 | A witness schema binds the obligation's persisted argument bindings, in the order the obligation persists them, position for position to the harness's symbolic arguments, and each decoded value is named by the binding at its position; a binding that is not an argument is refused with a typed schema refusal that reports no failure and is none of the five replay results. | Test (TC-026) |
| FR-016-AC-9 | A decoded falsification whose witness makes the native function evaluate the clause to false settles as `reproduced-with-evaluated-witness` with category `violation`; the verdict depends on the witness value, so a witness at which the function holds does not settle so. | Test (TC-026) |
| FR-016-AC-10 | A decoded falsification at which the native function evaluates the clause to true settles `inconclusive` with the proved violation and the replayed success both named. | Test (TC-026) |
| FR-016-AC-11 | The adapter refuses, with a distinct typed error each, a decoded value no replay parameter binds, a transcript field holding a delimiter, a transcript QSL does not admit, a request QSL refuses with any `ReplayRefusal` other than `Fault` (returned with its cause), and a witness-sourced request that settles on the input arm. | Test (TC-026) |
| FR-016-AC-12 | A `replay` call that returns `ReplayRefusal::Fault` yields a typed unavailable result, and never a mismatch, a refusal or a failure; no other condition yields unavailable. | Test (TC-026) |
| FR-016-AC-13 | A `Witness` arm that settles `reproduced-with-evaluated-witness` with a category other than `violation` yields a typed mismatch, and never a reproduced failure. | Test (TC-026) |
| FR-016-AC-14 | A proved unit that imports a locked dependency is compiled by `qsl_replay::call_site` together with that dependency's lock source, and `qsl_replay::replay` settles the unit's falsification through the imported function. | Test (TC-026) |
| FR-016-AC-15 | The replay request's `package.dependencies` carries one entry per lock dependency selection, in ascending identity order, with the lock's recorded `package_id`. | Test (TC-026) |
| FR-016-AC-16 | A unit whose import no lock dependency selection supplies is refused at the call site. | Test (TC-026) |
| FR-016-AC-17 | A lock recording another `package_id` for a dependency the unit imports is refused by QSL as a dependency identity mismatch. | Test (TC-026) |
| FR-016-AC-18 | A lock selecting a library the unit does not import is refused by QSL as unselected. | Test (TC-026) |
| FR-016-AC-19 | Lock libraries whose sources share a source owner are refused as no dependency input, before the call site compiles anything. | Test (TC-026) |
| FR-016-AC-20 | A lock library whose source has the unit's own source owner is refused by the call site. | Test (TC-026) |

## Dependencies

- **Upstream**: [FR-015](./FR-015-bounded-kani-obligations.md); QSL's
  `qsl-replay` crate, the only QSL crate this
  repository depends on. The replay package takes a compiled unit's package id and
  parameter node ids from `qsl_replay::call_site`, which compiles the unit with its
  locked dependencies, and the request's types from `qsl-replay`'s re-exports.
- **Downstream**: [TC-026](../../test/complete-v1/TC-026-witness-native-replay.md),
  [FR-024](./FR-024-counterexample-envelope-intake.md), which carries the decoded
  values to QSL in QSL's counterexample envelope.
