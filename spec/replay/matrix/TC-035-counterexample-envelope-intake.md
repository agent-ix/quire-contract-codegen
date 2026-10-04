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

## Status

Planned. No step is implemented. The skeleton spine renders a QSL transcript from decoded values
(`src/replay/function.rs`, TC-026), which is the shape step 2 checks, but it builds no envelope. Step 8
holds for the decode path: `src/replay/witness.rs` uses no Contract IR witness type. The bounded-Kani corpus retains
no counterexample packet, so step 5 (FR-024-AC-5) has nothing to submit.
