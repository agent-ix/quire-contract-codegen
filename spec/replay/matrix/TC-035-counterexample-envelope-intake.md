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
replay, that frame counterexamples carry their QSL identities, that minimization keeps only
failure-preserving envelopes, and that the generator holds no copy of QSL's replay types.

## Test Procedure

1. Build two obligation identities that differ only in `source_span`, then two that differ in
   obligation kind, then two that differ in one argument binding. Compare their QSL
   `ObligationIdentity` values.
2. Decode a Kani counterexample, render it through the adapter rendering function from its decoded
   values and parameter bindings, and pass the rendered transcript to `qsl_replay::Witness::parse`.
3. For a parameter with declared domain `[lo, hi]`, submit counterexamples with the value at `lo`,
   `hi`, `lo - 1` and `hi + 1`, and record whether `replay` was called.
4. Build one `WitnessPacket` per common required member QSL defines, excluding the three alternative
   replay-target members, each missing only that common member, and submit each. Then submit a
   complete packet whose trace position is present with the value none. With every common member
   present, submit function, frame and state-clause packets with none of `function`,
   `frame_operation` and `state_clause`, each pair of targets, and all three; record the typed
   admission refusal and whether an envelope or replay executor is reached.
   Inspect the packet type and construction path to confirm an empty name or old
   `selected_function` placeholder cannot supply the absent selection.
5. Retired (FR-024-AC-5, IR-890): no corpus `Input`-arm submission is tested.
6. Submit a function counterexample and a frame counterexample through their respective QSL replay
   paths. Read the function packet and admitted envelope's `function` selection and absent
   `frame_operation` and `state_clause`; read the frame packet and envelope's
   `frame_operation` (`object` and `operation`) and absent `function` and `state_clause`.
   Inspect that neither has a `selected_function` member or
   alias. For the frame, compare the pair with QSL's validated call-site result, including an
   inherited operation whose selected object differs from its declaring owner; confirm the packet
   names the declaring owner. When QSL supplies no validated pair, confirm packet construction
   refuses instead of using the caller's `OperationName`. Confirm the request carries no
   `arguments` or `occurrence_key`, and the packet and payload
   retain the ADR-017 PF-4 clause, frame, anchor and generated-occurrence identities QSL reports.
7. Reduce one `Witness`-arm counterexample with a candidate set that holds a
   failure-preserving candidate, an out-of-domain candidate and a verdict-changing candidate.
   Begin with an admitted original failure. Retain the valid equivalent reduction, then propose
   the two invalid reductions from that retained parent, and finally retain another valid
   reduction. Before each proposal clone each retained QSL envelope and capture each CG record's
   revision identity, parent reference, obligation identity and typed replay evidence. Compare
   the captured values with those read back after each proposal; use QSL envelope equality and
   inspect each retained backend transcript and replay result.
8. Submit a counterexample through the generator's replay entry point and pass the `Witness`,
   `ReplaySource`, `WitnessEnvelope`, `TerminalRecord` and `ObligationIdentity` values it produces
   directly to `qsl_replay`'s own functions.
9. Over a unit with two `postcondition` state clauses, replay a violating and a respecting run
   through `StateClauseReplay::replay`, and call `qsl_replay::replay_state_clause` directly with
   the same request and envelope. Replay once more through `replay_through` with an executor that
   records its arguments and returns a sentinel result.
10. Read back each clause's packet and request `state_clause`, its payload `observation` and
    absence of payload `clause`, and the envelope's `clause_node` and `occurrence_key`. Compare
    each selection and identity with the `ClauseSite` that `qsl_replay::call_site` validates for
    that clause name. Confirm both other target members are absent and the request carries no
    `arguments` or `occurrence_key`. Request an undeclared clause name.
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
    parsed playback produced from the falsified outcome's complete counterexample text by the
    bounded Kani output reader and the Inputs list, but no obligation identity and no transcript.
    Parse with byte and value limits no wider than the component-wise minimum of the requested
    limits and `run.decode_limits`; submit playback parsed under wider limits and verify
    `FrameReplay::new` refuses it when `source_len` or value count exceeds that minimum.
28. Submit equal parameter-bound values from two test backend adapters through the same common intake,
    with distinct backend identities and native transcript formats. Inspect the typed assignments,
    opaque obligation identity, declared domains, source/provenance and replay packet members at
    the adapter boundary, then the complete packet at QSL admission. Separately submit a decode
    refusal, an out-of-domain assignment, a QSL-refused packet and a native replay verdict
    disagreement. Send the decode refusal through the adapter outcome's refusal arm, with its
    backend identity, obligation identity and typed cause but no assignments, packet or native
    text; record the common intake's typed result, contract settlement and whether replay ran.
29. Have the function adapter complete a `WitnessPacket<NoPayload>` from its falsified run and
    decoded witness before invoking CG's function replay intake. Supply the independently
    selected function claim, admit the packet through QSL, then build a `function` request from
    that envelope. Compare the complete packet and request keys (node, role and ordinal),
    `function` target and `obligation_identity` before and after typed request serialization and
    readback. Replay a falsified Boolean predicate through QSL and record its reproduced
    violation. With the selected claim held fixed, omit the producer packet, then substitute a
    second QSL-admitted packet for another declaration occurrence; neither may build a request.
    Substitute an otherwise admitted `Input`-arm packet and a packet from another package
    revision with the same function node, occurrence and obligation identity; both refuse.
    Inspect that the request builder accepts an admitted envelope and no bare key or call site.
30. Have the checked parity claim producer complete an input-arm `WitnessPacket<NoPayload>`
    before invoking CG's parity intake. Build operator-parity and value-parity requests from
    their admitted packets and independently selected checked claims, and compare each
    request's complete key with its own packet's key after typed readback. Include an admitted
    scalar `expression` occurrence with a nonzero ordinal and two authentic occurrences of one
    node with different ordinals; check that the request retains the chosen occurrence and its
    `obligation_identity`. Swap the two admitted packets while holding each selected claim fixed;
    neither swapped packet may build a request. Substitute a `Witness`-arm packet and an
    otherwise admitted packet from another package revision with the same selected node,
    occurrence and obligation identity; both refuse. The value-parity request uses its packet's
    function `declaration` occurrence. Recheck that frame and state-clause requests from steps 6 and 10 have neither
    `arguments` nor `occurrence_key` while their envelope and payload identities remain.
31. Remove `occurrence_key` from an otherwise valid serialized `function` request, then also
    remove one byte-provision entry. Decode both with QSL's public request reader and record
    the code, path and phase of each refusal; confirm no replay executor is called.
32. For each of function contract, operator parity and value parity, hold an independently
    selected claim fixed and replace one producer packet member at a time: `function` target,
    occurrence node, role, ordinal, subject `clause_node`, `package_id`, source arm or opaque
    obligation identity. Use a second otherwise valid
    packet when testing the different ordinal of the same node. Record the typed claim-binding
    refusal and confirm no request builder or replay executor runs. Remove the packet occurrence
    key and check QSL packet admission refuses without a CG default. Check that a packet
    assembled inside `request_for` from the selected site or `ParityPreimage` cannot satisfy the
    producer-packet input or this substitution control.

## Expected Results

1. The `source_span` pair has equal identities, and the kind pair and the argument pair each
   differ (FR-024-AC-1).
2. `Witness::parse` accepts the rendered transcript, and the rendering function takes decoded
   values and parameter bindings, with no string of backend output (FR-024-AC-2).
3. The values at `lo` and `hi` replay. The values at `lo - 1` and `hi + 1` are reported
   out-of-domain, and `replay` is not called for either (FR-024-AC-3).
4. Each packet missing one common required member is refused by `WitnessEnvelope::reconstruct`
   with `MissingMember` naming it, and no replay runs. The packet with trace position none is
   admitted (FR-024-AC-4). No selection and each two-or-three-target combination produce
   distinguishable typed packet-admission refusals, admit no envelope and call no replay
   executor. An empty name or old
   `selected_function` placeholder cannot supply the absent selection (FR-024-AC-42).
5. Retired (FR-024-AC-5, IR-890): this step asserts no result.
6. The function packet and envelope carry only `function`; the frame packet and envelope carry
   only `frame_operation` with QSL's validated declaring owner's qualified name and operation
   identifier, including for an inherited operation. If QSL supplies no validated pair, CG
   refuses packet construction instead of deriving an owner from the caller's selection;
   neither carries `selected_function` or an alias. The frame request has no `arguments` or
   `occurrence_key`, while the packet and payload retain their PF-4 identity members and every
   identity QSL reports equals the one submitted (FR-024-AC-6, FR-024-AC-41).
7. Only valid failure-preserving candidates are retained (FR-024-AC-7). The root
   and both retained revisions refer to their exact QSL envelopes and unchanged obligation;
   each child names the actual retained parent. The invalid-domain and verdict-changing proposals
   create no revision. Each prior envelope compares equal to its clone captured before the
   proposal using QSL's `WitnessEnvelope<P>: Eq`; prior CG record identity, parent, obligation and
   typed evidence fields read back equal to their captured values (FR-024-AC-38,
   FR-024-AC-39). Each retained `Witness` envelope carries its own re-run transcript bound to its
   reduced assignments (FR-024-AC-8, FR-024-AC-40).
8. Every value is QSL's own `qsl_replay` type and is accepted by `qsl_replay` unchanged
   (FR-024-AC-10).
9. `replay` returns the direct `replay_state_clause` result for each run, the two runs' results
   differ, and the sentinel executor's value is returned as it is (FR-024-AC-11).
10. Each packet and request carry only `state_clause` as that clause's validated declared name;
    each payload has no `clause` member and carries the `Invocation` arm, each envelope's
    identities equal that clause's `ClauseSite` node and occurrence, and the undeclared name returns
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
    (FR-024-AC-30). A parsed playback whose `source_len` or value count exceeds the effective
    bound returns a typed decode refusal before QSL replay.
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
    unranged. Also check a non-model object and an admitted unselected model/object_type with
    a nonempty body: each returns `ModelFields` with `Accessor(NotModelObjectType)` and no
    replay despite an apparently valid body range. A selected/read nonempty-body tamper
    instead refuses `StaleNodeKey` during IR admission, before CG replay receives a package;
    do not expect a CG `ModelFields` result from it (IR TC-227). For `None` or out-of-`i64` accessor types
    QSL cannot emit, use an admitted QSL-emitted graph with selected-model-document override
    and recomputed digests (FR-024-AC-31, IR-624; gated on
    CG dependency update and implementation).
29. Generate the frame harness from the emitted package with accessor ranges, assert scope
    ids equal `call_site`'s before replay, remove `Twin::aligned`, and replay the forbidden
    write to a violation. Check request order and domains, including unread ranged `audit`;
    one outside a declared range returns `OutOfDomain`. A non-model object or admitted
    unselected model/object_type body cannot supply a range; a selected/read body tamper
    is rejected by IR admission before frame replay (FR-024-AC-32, IR-624).
30. Generate the postcondition harness from the emitted package, replay the debiting and
    unmutated subjects, read `violation` with evaluated `false` and `inconclusive` with
    `Verdicts`, and assert its replay domains equal accessor ranges (FR-024-AC-33, IR-624).
31. With installed Kani, replay the real falsified frame and postcondition playbacks from
    emitted-package harnesses, with the settlements of steps 27 and 16 (FR-024-AC-34, IR-624).
32. Replay a falsified run whose present model field has `IntRange` bounds outside `i64`,
    so CG records `TypeNotRange`, and whose `i64` playback lies outside the model range;
    read `Inconclusive` with `ReplayRefused`, not a violation or `Verified`. Replay inside the
    range is unaffected, and an unread present `IntRange` within `i64` is never called
    unranged. Build the out-of-`i64` model type through a selected-model-document override
    with recomputed digests on a QSL-emitted graph (FR-024-AC-35, IR-624).
33. Both adapters use the same typed outcome boundary while their native text stays inside each
    adapter; QSL receives each successful submission's backend, obligation and source without
    reminting. The decode-refusal arm reaches common intake with the original typed cause and no
    successful submission or envelope. The four refusals remain distinct typed evidence failures,
    never contract success or failure. Decode, domain and QSL admission refusals invoke no replay
    (FR-024-AC-36,
    FR-024-AC-37). Kani-specific refusal details follow FR-016.
34. The function-contract request retains the independently supplied admitted packet's full
    `declaration` key, selected function and obligation identity through typed readback. An
    omitted, other-occurrence, wrong-arm or other-package packet builds no request; no packet is assembled inside the
    request builder. QSL replay settles the valid falsified predicate as a reproduced violation
    (FR-024-AC-43).
35. The operator-parity request retains its own scalar application occurrence, including
    the nonzero ordinal and the difference between occurrences of one node; the value-parity
    request retains its own function `declaration` occurrence. Both retain their obligation
    identity; swapping authentic packets or supplying a wrong-arm or other-package packet while
    the selected claim stays fixed builds no request.
    Frame and state-clause requests retain their existing key-free shape
    (FR-024-AC-44, FR-024-AC-41).
36. Both missing-key requests refuse `invalid_runtime_input`/`missing-member` at
    `/replay/occurrence_key` at decode before byte provision or recompile, and neither
    invokes replay or produces a contract verdict (FR-024-AC-45).
37. Each mismatched target, key node, role, ordinal, subject node, package, source arm and
    obligation identity produces a typed claim-binding refusal naming the member, before request
    construction or replay. A packet
    whose key is absent receives QSL's packet-admission refusal. Two occurrences of one node
    remain distinct, and neither a call site nor `ParityPreimage` can replace an independently
    supplied packet (FR-024-AC-46).

## Status

Partly implemented. Steps 1 to 8 and 28 are planned: the skeleton spine renders a QSL transcript from decoded
values (`src/replay/function.rs`, TC-026, now through `render_witness`), which is the shape step 2
checks, but it builds no envelope. Step 8 holds for the decode path: `src/replay/witness.rs` uses
no Contract IR witness type. The bounded-Kani corpus retains no counterexample packet, so step 5
(FR-024-AC-5) is retired, as is the `Input`-arm reduction (FR-024-AC-9) in step 7.
Steps 9 and 11 to 17 (FR-024-AC-11 and FR-024-AC-13 to FR-024-AC-19, IR-460) are
implemented by `src/replay/state_clause.rs` and the tests of
`tests/it/kani_obligations_state_clause_replay.rs`, plus the `src` unit tests of step 12. Step 16
is a real-Kani test in the module `kani_obligations_state_clause_replay`, run through the
`kani_obligations` filter of `make kani`; its subject debits within the declared range, because a
debit past the floor runs to a post state QSL's snapshot admission refuses.
Step 10 (FR-024-AC-12) is planned under IR-965: the current state-clause path still carries
the old payload selection and has not been tested against the `state_clause` replay target.

Steps 18 to 26 (FR-024-AC-20 to FR-024-AC-29, IR-459) are implemented by `src/replay/obligation.rs`
and `src/replay/frame.rs`, the `src` unit tests of steps 18 and 26, and the tests of
`tests/it/kani_obligations_state_frame.rs` over the QSL twin of the state-frame fixture. The twin's
harness scope is aligned to the node ids QSL names for the twin's compiled unit (`Twin::aligned`),
because the fixture's checked package is hand-built and its node ids are its own. The package QSL
itself emits (`call_site`'s package bytes, admitted by the model reader) carries exactly the anchor
and frame ids `call_site` names (step 25's ids, read from the emitted package); a harness cannot yet
be generated from that package, because the object type QSL emits has an empty body and the
field-range reader finds no member. Step 27's formerly implemented hand-built positive
must be migrated to the emitted package to meet the restated FR-024-AC-30; that
criterion is planned with steps 28 to 32 (FR-024-AC-31 to FR-024-AC-35, IR-624). They are
planned: they specify end-to-end replay from an emitted-package harness whose field ranges come
from `model_object_fields` (FR-015-AC-77), after CG updates its IR dependency and implements the
merged accessor. Step 27 is a real-Kani test in the module
`kani_obligations_state_frame`, run through the `kani_obligations` filter of `make kani`. Step 19's
last case (a clause added on an operation that sorts earlier) is measured on the twin's operations
`deposit` and `transfer`: a clause added on `deposit` changes the identity of `transfer`.
FR-024-AC-36 to FR-024-AC-40, including common typed intake and CG lineage retention, are
planned. No production minimizer or lineage record is claimed by this test case.
Steps 29 to 32 and their results 34 to 37 are planned for IR-968. They require QSL's
QSL-973 replay-target and STD-264 occurrence-key implementations and authentic admitted
occurrence-bearing packets; no CG implementation or tagged test coverage is claimed yet.
