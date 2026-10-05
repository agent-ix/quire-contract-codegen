---
id: TC-035
title: "Verify counterexample submission in QSL's counterexample envelope"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-024
    type: verifies
  - target: ix://agent-ix/quire-specification/TC-219
    type: references
---
# TC-035: Verify counterexample submission in QSL's counterexample envelope

## Description

Verify that every counterexample reaches QSL as QSL's counterexample envelope keyed by QSL's
obligation identity, that out-of-domain and incomplete counterexamples are refused before any
replay, that corpus and frame counterexamples take their own arms, that minimization keeps only
failure-preserving envelopes, and that the generator holds no copy of QSL's replay types.

## Test Procedure

1. Build two obligation identities that differ only in `source_span`, then two that differ in
   obligation kind, then two that differ in one argument binding. Compare their QSL
   `ObligationIdentity` values.
2. Decode a Kani counterexample, render it through the adapter rendering function from its decoded
   values and parameter bindings, and pass the rendered transcript to `qsl_replay::Witness::parse`.
3. For a parameter with declared domain `[lo, hi]`, submit counterexamples with the value at `lo`,
   `hi`, `lo - 1` and `hi + 1`, and record whether `replay` was called.
4. Build one `WitnessPacket` per member QSL defines, each missing only that member, and submit
   each. Then submit a complete packet whose trace position is
   present with the value none.
5. Submit a corpus counterexample that agrees with native execution.
6. Submit a frame counterexample through `replay_frame`, and read back the clause, frame, anchor and
   occurrence identities QSL reports.
7. Reduce one `Witness`-arm and one `Input`-arm counterexample with a candidate set that holds a
   failure-preserving candidate, an out-of-domain candidate and a verdict-changing candidate.
8. Submit a counterexample through the generator's replay entry point and pass the `Witness`,
   `ReplaySource`, `WitnessEnvelope`, `TerminalRecord` and `ObligationIdentity` values it produces
   directly to `qsl_replay`'s own functions.
9. Over a unit with one `postcondition` state clause, submit a postcondition counterexample for a
   subject mutated to violate it, once through the generator and once by calling
   `qsl_replay::replay_state_clause` directly with the same request and envelope. Repeat with a
   stub verdict in place of the replay.
10. Read back the payload's `clause` and `observation` and the envelope's `clause_node` and
    `occurrence_key`, and the `ClauseSite` that `qsl_replay::call_site` returns for the clause
    name. Request an undeclared clause name.
11. Read the invocation document's pre and post snapshots and their digests; change one playback
    value and read the pre digest again.
12. Submit a playback that binds no value for one declared state field.
13. Replay the mutated subject's counterexample and the unmutated subject's run over the same pre
    state; submit values at, and immediately outside, each declared domain endpoint.
14. With the installed backend (`make kani`), replay the real playback of the falsified
    operation-contract harness of the mutated-to-debit subject.

## Expected Results

1. The `source_span` pair has equal identities, and the kind pair and the argument pair each
   differ (FR-024-AC-1).
2. `Witness::parse` accepts the rendered transcript, and the rendering function takes decoded
   values and parameter bindings, with no string of backend output (FR-024-AC-2).
3. The values at `lo` and `hi` replay. The values at `lo - 1` and `hi + 1` are reported
   out-of-domain, and `replay` is not called for either (FR-024-AC-3).
4. Each incomplete packet is refused by `WitnessEnvelope::reconstruct` with `MissingMember` naming
   its member, and no replay runs. The packet with trace position none is admitted (FR-024-AC-4).
5. The corpus counterexample is an `Input`-arm envelope keyed by parameter node id. It settles
   `reproduced-without-witness`, and no `Witness` was built (FR-024-AC-5).
6. Each identity QSL reports equals the one submitted (FR-024-AC-6).
7. Only the failure-preserving candidate is retained (FR-024-AC-7, FR-024-AC-9). The `Witness`-arm
   envelope carries its own re-run's transcript (FR-024-AC-8), and the retained `Input`-arm
   candidate is an `Input`-arm envelope.
8. Every value is QSL's own `qsl_replay` type and is accepted by `qsl_replay` unchanged
   (FR-024-AC-10).
9. The generator's result equals the direct `replay_state_clause` result, and the stub verdict
   differs and fails (FR-024-AC-11).
10. The payload names the clause and the `Invocation` arm, the envelope's identities equal the
    `ClauseSite` node and occurrence, and the undeclared name is refused by the call site with no
    replay (FR-024-AC-12).
11. The pre snapshot holds the playback's values and the post snapshot the native run's, both
    supplied by digest, and the pre digest changes with the playback value (FR-024-AC-13).
12. The result is a typed `Incomplete` naming the field, with no replay call and no default
    (FR-024-AC-14).
13. The mutated subject settles `reproduced-with-evaluated-witness`, `violation`, evaluated
    `false`; the unmutated run settles `inconclusive`, `Verdicts`; values outside a domain are
    reported out-of-domain with no replay call, and the endpoint values replay
    (FR-024-AC-15, FR-024-AC-16).
14. The real playback settles `reproduced-with-evaluated-witness`, `violation`
    (FR-024-AC-17).

## Status

Planned. No step is implemented. The skeleton spine renders a QSL transcript from decoded values
(`src/replay/function.rs`, TC-026), which is the shape step 2 checks, but it builds no envelope. Step 8
holds for the decode path: `src/replay/witness.rs` uses no Contract IR witness type. The bounded-Kani corpus retains
no counterexample packet, so step 5 (FR-024-AC-5) has nothing to submit. Steps 9 to 14
(FR-024-AC-11 to FR-024-AC-17, IR-460) are planned: `src` has no consumer of
`qsl_replay::replay_state_clause`. Step 14 is a real-Kani test run only through `make kani`.
