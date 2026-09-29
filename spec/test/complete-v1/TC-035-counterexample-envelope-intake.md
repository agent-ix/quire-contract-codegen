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
failure-preserving linked revisions, and that the generator holds no copy of QSL's replay types.

## Test Procedure

1. Build two obligation identities that differ only in `source_span`, then two that differ in
   obligation kind, then two that differ in one argument binding. Compare their QSL
   `ObligationIdentity` values.
2. Render a decoded falsification into QSL's backend-witness transcript and admit it through
   `Witness::parse`. Then render one carrying a field delimiter, which QSL refuses. Scan every
   non-test source file under `src/` for backend-native counterexample wording outside the
   adapter.
3. For a parameter with declared domain `[lo, hi]`, submit counterexamples with the value at `lo`,
   `hi`, `lo - 1` and `hi + 1`, and record whether `replay` was called.
4. Submit one counterexample per envelope member, each missing only that member.
5. Submit a corpus counterexample that agrees with native execution.
6. Submit a frame counterexample through `replay_frame`, and read back the clause, frame, anchor and
   occurrence identities QSL reports.
7. Minimize one `Witness`-arm and one `Input`-arm counterexample with a candidate set that holds a
   failure-preserving candidate, an out-of-domain candidate and a verdict-changing candidate.
8. Scan `src/` for definitions of `Witness`, `ReplaySource`, `WitnessEnvelope`, `TerminalRecord`
   and `ObligationIdentity`, and for imports of any of them from `quire_contract_ir`.

## Expected Results

1. The `source_span` pair has equal identities, and the kind pair and the argument pair each
   differ.
2. The admitted transcript replays. The refused transcript returns a typed refusal carrying QSL's
   cause, and `replay` is not called. The scan finds wording only in the adapter.
3. The values at `lo` and `hi` replay. The values at `lo - 1` and `hi + 1` are reported
   out-of-domain, and `replay` is not called for either.
4. Each is refused with a typed cause naming its missing member, and no replay runs.
5. The corpus counterexample is an `Input`-arm envelope keyed by parameter node id. It settles
   `reproduced-without-witness`, and no `Witness` was built.
6. Each identity QSL reports equals the one submitted.
7. Only the failure-preserving candidate is retained, as a revision linked to its parent. The
   `Witness`-arm revision carries its own re-run's transcript, and the `Input`-arm revision is an
   `Input`-arm envelope.
8. Both scans find nothing.

## Status

Planned. No step is implemented. Step 2's transcript rendering and QSL's refusal of a field
delimiter exist for the skeleton spine (`src/spine_replay.rs`, TC-026). Step 8 fails at this
revision, because `src/kani_witness_join.rs` imports `Witness` and `src/bounded_kani_corpus.rs`
imports `ReplaySource` from `quire_contract_ir`.
