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
9. Over a unit with two `postcondition` state clauses, replay a violating and a respecting run
   through `StateClauseReplay::replay`, and call `qsl_replay::replay_state_clause` directly with
   the same request and envelope. Replay once more through `replay_through` with an executor that
   records its arguments and returns a sentinel result.
10. Read back each clause's payload `clause` and `observation` and the envelope's `clause_node`
    and `occurrence_key`, and the `ClauseSite` that `qsl_replay::call_site` returns for each
    clause name. Request an undeclared clause name.
11. Read the invocation document and its pre and post snapshots and their digests; change one
    playback value and one post-state value and read the digests again.
12. Build the document vector of FR-024-AC-14 (unsorted members, escaped characters, a large
    integer, a fractional number and an exponent form) through the builder and through
    `core::canonical`, and read the source of the new module and `Cargo.toml`.
13. Submit a playback, and a post state, that binds no value for one declared state field, one
    that binds an undeclared name, and one that binds a field twice.
14. Replay the mutated subject's counterexample and the unmutated subject's run over the same pre
    state, and read the envelope's arm and payload `witness`.
15. Submit state field values at, and immediately outside, each declared range endpoint.
16. With the installed backend (`make kani`), replay the real playback of the falsified
    operation-contract harness of the mutated-to-debit subject.
17. Request a replay for an operation that declares a parameter, one that declares a result, and
    one that declares neither.
18. Mint a frame identity (empty `arguments`) and a function identity from fixed sites, and compute the
    digest of each hand-written preimage text (members listed in an order other than the
    encoder's); run the function path's existing golden vectors unedited. Read `Cargo.toml` and
    the non-test source of `src/replay/obligation.rs`.
19. Change one input of a frame identity at a time: the kind, the `function` node and each part
    of the `declaration` key; change the clause node, the module and harness symbols, the state
    and subject paths, the unwind bound, the options and the state fields and their ranges; mint
    the identity of two clauses of one operation. Compile, through
    `qsl_replay::call_site`, two units that differ only in the frame's grants, two operations of
    one object with equal frame text, one unit twice with the second shifted by blank lines, and a
    unit with a clause added on an operation that sorts earlier, and mint each identity.
20. Generate a frame harness and read its identity record; then request a replay with a
    `postcondition` harness, and with a frame harness whose granted and checked fields are not
    its state fields; read the record of a harness regenerated from equal inputs.
21. Build a frame replay and read the request's and the envelope's `obligation_identity` against
    the identity minted from the site and the harness; change a grant and read both again; read
    `FrameReplayInputs` and the twin's inputs.
22. Decode the playback of the frame harness's falsified run into its state fields and read the
    transcript `Witness::parse` is given; submit the playback of another harness, one with a
    missing value and one with a value of the wrong width.
23. Submit decoded field values at, and immediately outside, each declared range endpoint, and a
    field with no declared range.
24. Replay a forbidden-write playback with the invocation of its own pre state, and with the
    invocation of a different pre state; replay with an invocation or pre snapshot that is
    absent, unreadable, lacks the object, or lacks a field.
25. Request a replay whose harness names another operation, another anchor and another frame.
26. Read the frame envelope's `declared_domains` and the source of `src/replay/frame.rs`.
27. With the installed backend (`make kani`), replay the real playback of the falsified frame
    harness of a subject that writes a forbidden field, supplying the harness's identity, the
    playback text and the Inputs list, but no obligation identity and no transcript.

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
9. `replay` returns the direct `replay_state_clause` result for each run, the two runs' results
   differ, and the sentinel executor's value is returned as it is (FR-024-AC-11).
10. Each payload names its clause and the `Invocation` arm, each envelope's identities equal that
    clause's `ClauseSite` node and occurrence, and the undeclared name returns
    `StateClauseReplayError::CallSite` with the executor not called (FR-024-AC-12).
11. The pre snapshot holds the playback's values and the post snapshot the supplied post-state
    values, and the invocation document holds the stated members with its two digests; each
    changed value changes its snapshot's digest (FR-024-AC-13).
12. The builder's bytes and digest equal `core::canonical`'s, and the module names no encoder
    function of `quire_canonical`, no `sha2`, no `ByteDigest::of`, no member sort or hand-written
    escaping, and `sha2` is not in `[dependencies]` (FR-024-AC-14).
13. `StateClauseReplayError::MissingField` names the field, the executor is not called and no
    snapshot holds a default; a name the object does not declare and a field bound twice return
    `UndeclaredField` and `DuplicateField` (FR-024-AC-15).
14. The mutated subject settles `reproduced-with-evaluated-witness`, `violation`, evaluated
    `false`; the unmutated run settles `inconclusive`, `Verdicts`; both envelopes are on the
    `Witness` arm with a payload `witness` of none (FR-024-AC-16).
15. The endpoint values replay, and a value outside the range returns
    `StateClauseReplayError::OutOfDomain` with the executor not called (FR-024-AC-17).
16. The real playback settles `reproduced-with-evaluated-witness`, `violation`
    (FR-024-AC-18).
17. The first two return `StateClauseReplayError::UnsupportedOperationShape` carrying the operation
    and its declaration, with no document built and the executor not called; the third is not refused for its shape
    (FR-024-AC-19).
18. Each identity equals the digest of its hand-written text, the function's golden vectors pass
    unedited; `sha2` is not in `[dependencies]` and the non-test source names no encoder function
    of `quire_canonical`, no `sha2` and no `ByteDigest::of` (FR-024-AC-20).
19. Each change alone changes the identity; the clause node, symbol, path, unwind, option, field
    and range changes do not; two clauses of one operation mint one identity; the two
    grant variants and the two equal-frame operations differ; the shifted unit's identity equals
    the first; the identity after adding a clause on an earlier-sorting operation is read and
    recorded as the stated edge (FR-024-AC-21).
20. The record holds `state_fields` in draw order; the postcondition harness returns
    `FrameReplayError::NotAFrame` and the mismatched field set `FieldSetMismatch`, each with no
    `call_site` or replay; the minted frame identity has empty `arguments`; a regenerated record
    is byte-identical; a field with no declared range is listed in `state_fields`, decoded and not
    refused (FR-024-AC-22, FR-024-AC-23).
21. Request and envelope both equal the minted identity and both change with a grant; the twin
    passes no identity and `FrameReplayInputs` has no identity member (FR-024-AC-24).
22. The transcript is the one rendering function's, over the decoded values; the other harness's
    playback, the short playback and the wrong-width playback each return a typed decode refusal
    with `call_site` and replay not called (FR-024-AC-25).
23. Endpoints are admitted; one outside returns `FrameReplayError::OutOfDomain` naming the field
    with nothing called; the unranged field is not checked (FR-024-AC-26).
24. The own-pre-state invocation settles `reproduced-with-evaluated-witness`, `violation`; the
    different pre state, and each absent or unreadable document, return
    `FrameReplayError::PreState` naming the field, both values or what is missing, with replay not
    called (FR-024-AC-27).
25. The operation mismatch returns `ScopeMismatch` before `call_site`; the anchor and frame
    mismatches return it after, naming the member, with replay not called (FR-024-AC-28).
26. `declared_domains` is empty, the source builds no `DeclaredDomain` or `DomainKey`, and the
    header names QSL-345 once (FR-024-AC-29).
27. The forbidden-write playback settles a reproduced violation naming the written field
    (FR-024-AC-30).
28. Over the QSL-emitted twin package, check `balance` and unread `audit` against
    `model_object_fields(&object_id)` and replay them in the order of input `state_fields`;
    endpoints of their 0 to 1000 ranges admit, one outside refuses `OutOfDomain`, a missing
    playback binding returns `MissingField`, and a binding outside the input list returns
    `UndeclaredField`. A listed name absent from the accessor returns `ModelFields` with
    `Absent { field }`; an accessor error returns `ModelFields` with `Accessor(error)` and the
    unchanged IR error. Supply both an absent listed name and a missing playback binding,
    then an ambiguous accessor table and a missing playback binding; in both cases read
    `ModelFields` before `MissingField`, with no replay or executor call. A valid table with a
    missing binding still returns `MissingField`. A present field with a non-range type remains
    unranged (FR-024-AC-31, IR-624; gated on
    CG dependency update and implementation).
29. Generate the frame harness from the emitted package with accessor ranges, assert scope
    ids equal `call_site`'s before replay, remove `Twin::aligned`, and replay the forbidden
    write to a violation. Check request order and domains, including unread ranged `audit`;
    one outside a declared range returns `OutOfDomain`. Comparing the hand-built body's
    ranges remains gated on IR-627 (FR-024-AC-32, IR-624).
30. Generate the postcondition harness from the emitted package, replay the debiting and
    unmutated subjects, read `violation` with evaluated `false` and `inconclusive` with
    `Verdicts`, and assert its replay domains equal accessor ranges (FR-024-AC-33, IR-624).
31. With installed Kani, replay the real falsified frame and postcondition playbacks from
    emitted-package harnesses, with the settlements of steps 27 and 16 (FR-024-AC-34, IR-624).
32. Replay a falsified run whose present model field has `IntRange` bounds outside `i64`,
    so CG records `TypeNotRange`, and whose `i64` playback lies outside the model range;
    read `Inconclusive` with `ReplayRefused`, not a violation or `Verified`. Replay inside the
    range is unaffected, and an unread present `IntRange` within `i64` is never called
    unranged (FR-024-AC-35, IR-624).

## Status

Partly implemented. Steps 1 to 8 are not: the skeleton spine renders a QSL transcript from decoded
values (`src/replay/function.rs`, TC-026, now through `render_witness`), which is the shape step 2
checks, but it builds no envelope. Step 8 holds for the decode path: `src/replay/witness.rs` uses
no Contract IR witness type. The bounded-Kani corpus retains no counterexample packet, so step 5
(FR-024-AC-5) has nothing to submit. Steps 9 to 17 (FR-024-AC-11 to FR-024-AC-19, IR-460) are
implemented by `src/replay/state_clause.rs` and the tests of
`tests/it/kani_obligations_state_clause_replay.rs`, plus the `src` unit tests of step 12. Step 16
is a real-Kani test in the module `kani_obligations_state_clause_replay`, run through the
`kani_obligations` filter of `make kani`; its subject debits within the declared range, because a
debit past the floor runs to a post state QSL's snapshot admission refuses.

Steps 18 to 27 (FR-024-AC-20 to FR-024-AC-30, IR-459) are implemented by `src/replay/obligation.rs`
and `src/replay/frame.rs`, the `src` unit tests of steps 18 and 26, and the tests of
`tests/it/kani_obligations_state_frame.rs` over the QSL twin of the state-frame fixture. The twin's
harness scope is aligned to the node ids QSL names for the twin's compiled unit (`Twin::aligned`),
because the fixture's checked package is hand-built and its node ids are its own. The package QSL
itself emits (`call_site`'s package bytes, admitted by the model reader) carries exactly the anchor
and frame ids `call_site` names (step 25's ids, read from the emitted package); a harness cannot yet
be generated from that package, because the object type QSL emits has an empty body and the
field-range reader finds no member. Steps 28 to 32 (FR-024-AC-31 to FR-024-AC-35, IR-624) are
planned: they specify end-to-end replay from an emitted-package harness whose field ranges come
from `model_object_fields` (FR-015-AC-77), after CG updates its IR dependency and implements the
merged accessor. Step 27 is a real-Kani test in the module
`kani_obligations_state_frame`, run through the `kani_obligations` filter of `make kani`. Step 19's
last case (a clause added on an operation that sorts earlier) is measured on the twin's operations
`deposit` and `transfer`: a clause added on `deposit` changes the identity of `transfer`.
