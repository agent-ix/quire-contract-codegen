<!-- reviewer repo=agent-ix/quire-contract-codegen visibility=public quoin=0.28.4 module=spec-artifacts-process@b82e42e893612b4b4ee94127714b4577ee893502 id=SR-15291 method=gap-analysis lang=rust pr=quire-contract-codegen#501 model=unavailable run=d9d507d8-42eb-4ddc-a8d0-3eed0b37342f -->

SR-15291 gap-analysis, part 11/18. FAIL: 311 computed untagged/ignored-only obligations, four manual IR17 stage-evidence gaps and four actual inherited dangling trace annotations remain. The two IR877 diagnostic controls now bind real tests; actual AC56 inspection is clean. No whole-ticket completion is claimed.

| FND | Severity | Check | Summary |
| --- | --- | --- | --- |
| FND-181 | high | coverage | Inherited FR-032-AC-1 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-182 | high | coverage | Inherited FR-032-AC-4 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-183 | high | coverage | Inherited FR-032-AC-5 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-184 | high | coverage | Inherited FR-032-AC-6 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-185 | high | coverage | Inherited FR-032-AC-7 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-186 | high | coverage | Inherited FR-032-AC-8 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-187 | high | coverage | Inherited FR-032-AC-9 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-188 | high | coverage | Inherited FR-032-AC-10 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-189 | high | coverage | Inherited FR-032-AC-11 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-190 | high | coverage | Inherited FR-032-AC-12 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-191 | high | coverage | Inherited FR-032-AC-13 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-192 | high | coverage | Inherited FR-032-AC-14 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-193 | high | coverage | Inherited FR-033-AC-2 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-194 | high | coverage | Inherited FR-033-AC-3 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-195 | high | coverage | Inherited FR-033-AC-4 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-196 | high | coverage | Inherited FR-033-AC-5 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-197 | high | coverage | Inherited FR-033-AC-6 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-198 | high | coverage | Inherited FR-033-AC-8 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |

+++ [reviewer data]

```yaml
clean: false
scope:
- id: FR-024-AC-1
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: 'The envelope''s obligation identity is QSL''s `ObligationIdentity` built as the ADR-013 O-09
    digest by the one scheme of FR-024-AC-20: the subject node id (function or state frame; a clause or
    application subject is not yet built), its occurrence key, the obligation kind and the arguments,
    each a parameter node id with its declared domain (empty for a frame, which has no parameters), and
    no `source_span`. Changing `source_span` leaves it unchanged, and changing the obligation kind or
    any argument binding changes it. The scheme''s spelling is CG''s own and carries no digest label (AD-003
    E-1, open '
- id: FR-024-AC-2
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: Every backend-witness transcript the generator passes to `Witness::parse` is produced by the
    one adapter rendering function from decoded values, and that function takes no backend-native text
    as input.
- id: FR-024-AC-3
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: A counterexample with any value outside its declared domain is reported out-of-domain and neither
    `replay` nor `replay_frame` is called. The endpoints of each declared domain are admitted, and the
    values immediately outside it are not.
- id: FR-024-AC-4
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: For each `WitnessPacket` member in turn, a packet missing that member is refused by `WitnessEnvelope::reconstruct`
    with QSL's `MissingMember` naming it, and no replay runs. A packet whose trace position is present
    with the value none is admitted.
- id: FR-024-AC-6
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: A frame counterexample is submitted as a `WitnessEnvelope<FrameCounterexample>` through `qsl_replay::replay_frame`,
    and its clause, frame, anchor and occurrence identities reach QSL unchanged.
- id: FR-024-AC-7
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: A reduced `Witness`-arm candidate is retained only when a backend re-run and native replay
    both preserve domain validity and the failure; a candidate that changes domain validity or the verdict
    is discarded.
- id: FR-024-AC-8
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: A retained reduced `Witness`-arm envelope carries its own backend re-run's transcript, never
    the original counterexample's.
- id: FR-024-AC-10
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: No type named `Witness`, `ReplaySource`, `WitnessEnvelope`, `TerminalRecord` or `ObligationIdentity`
    is defined under `src/`, and no source file under `src/` imports any of them from `quire_contract_ir`.
- id: FR-024-AC-11
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: '`StateClauseReplay::replay` returns exactly the `StateClauseReplayResult` that `qsl_replay::replay_state_clause`
    returns when a test calls it directly with the same request and envelope, for a violating run and
    for a respecting run, and the two results differ from each other. `StateClauseReplay::replay_through`
    hands the executor the request and envelope it built and returns the executor''s value as it is: an
    executor returning a sentinel result changes the returned result to that sentinel.'
- id: FR-024-AC-12
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: The state-clause envelope's `clause_node` equals the `ClauseSite` node, and its `occurrence_key`
    the `ClauseSite` occurrence, that `qsl_replay::call_site` returns for the clause name, and the payload's
    `clause` is that name and its `observation` is the `Invocation` arm. For a unit with two postcondition
    clauses, each clause's envelope carries its own node and occurrence. A clause name the unit does not
    declare returns `StateClauseReplayError::CallSite` holding the call site's refusal when the request
    is built, and the executor is not called.
- id: FR-024-AC-13
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: The pre snapshot holds, for every declared field of the state object, the value the Kani playback
    bound to that field, and the post snapshot holds the post-state value the caller supplied for it;
    each is one `complete` population holding one object with each field as an integer. The invocation
    document holds the members the Behavior bullets state, with `parameters` empty, `result` null and
    `created` and `deleted` empty, and its `pre` and `post` carry each snapshot's identity and `sha256-jcs`
    digest. Changing one playback value changes the pre snapshot's bytes and digest, and changing one
    post-
- id: FR-024-AC-14
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: For a document vector whose members are in non-sorted source order, whose text holds characters
    JSON escapes (a quote, a backslash, a control character, a non-ASCII letter) and whose numeric members
    include a large integer, a number with a fractional part and an exponent form, the bytes and the digest
    the state-clause builder returns equal what `core::canonical`'s bytes and digest functions return
    for the same value. The source of the new state-clause module names none of `quire_canonical`'s encoder
    functions, `sha2`, `ByteDigest::of`, a `sort` call over document members or a hand-written stri
- id: FR-024-AC-15
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: A playback that binds no value for a declared state field returns `StateClauseReplayError::MissingField`
    naming that field, the executor is not called, and no snapshot holds a default for it. A playback
    or post-state name outside the path's declared field list returns `StateClauseReplayError::UndeclaredField`
    and a field bound twice returns `DuplicateField`, each naming the field, in the playback and in the
    post state alike.
- id: FR-024-AC-16
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: For a subject mutated to violate the postcondition, the replay settles `reproduced-with-evaluated-witness`
    with category `violation` and an evaluated `false`. For the unmutated subject's run over the same
    pre state it settles `inconclusive` with cause `Verdicts` (`violation` proved, `success` replayed).
    The envelope is on the `Witness` arm with a payload `witness` of none in both cases.
- id: FR-024-AC-17
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: A state field value outside its declared integer range returns `StateClauseReplayError::OutOfDomain`
    naming the field and the executor is not called. The range's two endpoints are admitted and the values
    one below and one above are not.
- id: FR-024-AC-18
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: With the installed backend, the falsified operation-contract harness of a postcondition state
    clause over a subject mutated to debit is replayed from its real Kani playback through `replay_state_clause`
    and settles `reproduced-with-evaluated-witness`, `violation`. The test is `tc_035_real_kani_state_clause_counterexample_replays_through_qsl`
    in the module `kani_obligations_state_clause_replay`, so the `kani_obligations` filter of `make kani`
    selects it.
- id: FR-024-AC-19
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: 'An operation that declares a parameter, and one that declares a result, each return `StateClauseReplayError::UnsupportedOperationShape`
    carrying the operation and the declaration (its parameters and result), build no document, and do
    not call the executor; an operation that declares neither is not refused for its shape. The shape
    check reads the admitted package the caller supplies, and QSL''s admission reads the documents provided
    in the byte provision; where the two disagree, the disagreement surfaces as a QSL refusal and not
    as this error. The error is not read as a QSL refusal: FR-029-AC-16'
- id: FR-024-AC-20
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: 'One obligation-identity function in `src/replay/obligation.rs` mints the identity of the function
    path and of the frame path: the SHA-256 of the RFC 8785 encoding, made by `core::canonical`, of one
    object with the four members `function`, `declaration`, `kind` and `arguments` of AD-003 E-1, under
    their existing spelling. No function-path identity changes: the existing golden vectors still pass
    unedited. A frame identity over a fixed site and kind, with empty `arguments`, equals the digest of
    a hand-written text of that preimage, written in the RFC 8785 member order, which differs from the
    prei'
- id: FR-024-AC-21
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: 'The identity changes when any one of these changes alone: the kind, the `function` node and
    the `declaration` key (node, role or ordinal); it does not change when the harness''s clause node,
    module symbol, harness symbol, state or subject path, solver, unwind bound, options, state fields
    or their ranges change, and no input of the function is a source span. Two postcondition clauses of
    one operation therefore mint one frame identity, as QSL gives them one shared frame occurrence. Two
    units that differ only in the frame''s `modifies` grants have different frame identities, and two
    operations of o'
- id: FR-024-AC-22
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: 'A frame obligation''s identity takes `function` and `declaration` from the `OperationSite`''s
    `frame` and `frame_occurrence`, so they equal the envelope''s `clause_node` and `occurrence_key`;
    its `kind` is `frame` and is derived from the harness''s `property`, never supplied; its `arguments`
    are the empty list, since a frame has no parameters, and no state field or range is a member. A harness
    whose `property` is a postcondition returns `FrameReplayError::NotAFrame`, and a frame whose granted
    and checked fields are not exactly its state fields returns `FrameReplayError::FieldSetMismatch`;
    neither '
- id: FR-024-AC-23
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: '`StateFrameIdentity` and its persisted record carry `state_fields`, every field the harness
    draws in draw order, which `domains` (ranged fields only) and `property`''s granted and checked lists
    (unordered) do not give. `state_fields` is used for decoding and ordering only and is not minted.
    A field with no declared range is listed in `state_fields`, has no entry in `domains`, is decoded
    and is not range-checked or refused (the AD-003 owner may tighten that later). A regenerated harness
    from equal inputs has a byte-identical record, and a record without `state_fields`, or one that lists
    a state '
- id: FR-024-AC-24
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: '`FrameReplayInputs` has no `obligation_identity` member. The frame request''s `obligation_identity`
    and the envelope''s both equal the identity minted from the site and the harness, and a harness with
    a changed grant gives a different value in both. The frame twin of `tests/state_frame_support/native_twin.rs`
    passes no identity of its own, so no stand-in value reaches either member; no source scan is part
    of this criterion. The state-clause path''s caller-supplied `obligation_identity` is not covered by
    this criterion (Q-1).'
- id: FR-024-AC-25
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: 'The frame witness is built from the real playback: the playback text decodes through the one
    decoder of `src/replay/witness.rs` against bindings built from the harness''s `state_fields` (each
    `i64`, in draw order), and the transcript passed to `Witness::parse` is rendered by the one adapter
    rendering function from the decoded values, with the harness path and the check text the decode names.
    The fixed per-operation assertion text that `frame.rs` passes to `Witness::parse` today is gone. A
    playback of another harness, a playback with the wrong number of values and one with a wrong-width
    value ea'
- id: FR-024-AC-26
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: A decoded state-field value outside the field's declared range returns `FrameReplayError::OutOfDomain`
    naming the field and its value, and calls neither `call_site` nor `replay_frame`; the range's two
    endpoints are admitted and the values one below and one above are not. A field with no declared range
    is not range-checked.
- id: FR-024-AC-27
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: 'The decoded pre-state is tied to the invocation: for every state field, the integer the pre
    snapshot (the provided document whose `sha256-jcs` digest the invocation''s `pre` names) holds for
    the object the invocation''s `self` addresses equals the decoded value, and a field that differs returns
    `FrameReplayError::PreState` naming the field, the decoded value and the snapshot''s value; an invocation
    or pre snapshot that is not among the provided documents, is not the document shape the state-clause
    path writes (the typed leaf shapes of `src/replay/state_clause.rs`), or lacks the object or a field '
- id: FR-024-AC-28
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: A harness whose `scope.operation` is not the operation requested returns `FrameReplayError::ScopeMismatch`
    naming `operation` before `call_site` is called; one whose `scope.anchor` or `scope.frame` is not
    the anchor or frame `call_site` names returns `ScopeMismatch` naming that member and does not call
    `replay_frame`. The harness's module and harness symbols name the generated artifact, are not members
    of the identity, and are checked only by the decode (FR-024-AC-25).
- id: FR-024-AC-29
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: The frame envelope's `declared_domains` is the empty list, no source of `src/replay/frame.rs`
    builds a `DeclaredDomain` or a `DomainKey`, and the module's header states, once, that the declaration
    is empty until QSL-345 settles the declared-domain key and refuses an empty declaration. The frame
    path adopts no key shape before then, whatever shape another path builds.
- id: FR-024-AC-30
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: With the installed backend, the real playback of the falsified frame harness of a subject that
    writes a forbidden field is replayed through `FrameReplay::new` and `replay`, the caller supplying
    the harness's `StateFrameIdentity`, the playback text and the inputs of FR-024's Inputs list but no
    obligation identity and no transcript, and settles a reproduced violation naming the written field.
    The harness is generated from QSL's emitted package and its scope node ids equal those `qsl_replay::call_site`
    names, without `Twin::aligned` (FR-024-AC-32). A subject that writes only a granted field leave
- id: FR-024-AC-31
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: Read fields from the empty object body or a read's `result_type`, omit unread `audit`, sort
    caller order by accessor name, collapse absent and accessor-error causes, return `MissingField` when
    a listed model field is also absent from playback, or narrow an `i128` endpoint.
- id: FR-024-AC-32
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: Generate from the hand-built fixture, align scope ids, or drop an unread ranged field from
    `domains`.
- id: FR-024-AC-33
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: Give replay a field list or ranges read from the package's object body or from reads.
- id: FR-024-AC-34
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: Run the real-Kani cases over a hand-built fixture harness.
- id: FR-024-AC-35
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: Settle refusal as a violation or `Verified`, omit `TypeNotRange`, or call an unread bounded
    field unranged.
- id: FR-024-AC-36
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: 'Two distinct test backend adapters each produce a typed outcome: success carries equivalent
    assignments, each adapter''s backend identity, the same opaque obligation identity, declared domains,
    source/provenance and replay packet members; decode refusal carries only backend identity, obligation
    identity and typed cause. Native text is observed only within its own adapter; the common intake neither
    parses native text nor remints the obligation identity, and QSL receives each successful adapter''s
    original backend and source identity in a complete packet.'
- id: FR-024-AC-37
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: The common intake maps an adapter's decode-refusal arm to a typed decode evidence failure retaining
    its cause, with no envelope or replay. Before native replay, an out-of-domain assignment yields a
    distinct typed domain evidence failure; an incomplete or QSL-refused witness/packet yields a distinct
    typed QSL admission evidence failure retaining its cause. A replay verdict disagreement remains a
    distinct typed evidence failure. None settles as contract success or contract failure, and none of
    the pre-replay failures invokes replay. Kani's exact decode and refusal partition remain governed
    by FR
- id: FR-024-AC-38
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: 'Starting from one admitted original failure, one valid reduced candidate that preserves the
    failure becomes a new CG lineage revision referring to its exact admitted QSL envelope, its exact
    parent revision and the unchanged opaque obligation identity. After retention, the original QSL envelope
    compares equal to its pre-reduction clone by `WitnessEnvelope<P>: Eq`, and the original CG record''s
    revision identity, parent, obligation and typed evidence fields read back equal to their captured
    values. No lineage member is added to QSL''s envelope or packet.'
- id: FR-024-AC-39
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: From a retained failure revision, an out-of-domain reduction and a domain-valid reduction that
    changes the native verdict each create no revision. After each rejection, every prior QSL envelope
    compares equal to its pre-proposal clone and every prior CG record's revision identity, parent, obligation
    and typed evidence fields equal their captured values; a later valid reduction names the actual retained
    parent, with no skipped or substituted parent.
- id: FR-024-AC-40
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: A retained reduced `Witness` envelope holds a transcript from its own backend re-run, bound
    to its own reduced assignments; it cannot reuse any ancestor's transcript.
- id: FR-025-AC-1
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  excerpt: For every harness kind, the identifiers of the emitted `kani::any()` bindings, in source order,
    equal the identity's `arguments` identifiers in order, and those ascend by identifier.
- id: FR-025-AC-2
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  excerpt: Every argument binding in a persisted identity names its parameter node id and declared domain.
- id: FR-025-AC-3
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  excerpt: A Boolean argument is a `bool` that reaches the oracle unchanged, and a bounded-integer argument
    is an `i64` that reaches the oracle as `rt::Integer::from` of that value. The values `i64::MIN`, `-1`,
    `0`, `1` and `i64::MAX` each reach the oracle equal to their mathematical value.
- id: FR-025-AC-4
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  excerpt: An argument whose domain has no declared finite bound yields no harness and is never narrowed
    to a machine range.
- id: FR-025-AC-5
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  excerpt: A frame obligation is accounted `unsupported` with a typed reason and no harness.
- id: FR-025-AC-6
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  excerpt: A binding that names no parameter of the selected function, and a parameter of that function
    with no binding, are each refused at generation with a typed reason and no harness.
- id: FR-025-AC-7
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  excerpt: An argument of a family with no row in the Rust-type table (rational, decimal, IEEE, text,
    enum, composite, collection or function) is accounted `unsupported` with a typed reason naming the
    family, and no harness is emitted.
- id: FR-025-AC-8
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  excerpt: A harness over a state-reading obligation passes each state value to the subject as `&mut`
    to a harness-owned value and copies it before the call; the pre-state the harness asserts over is
    the copy, and the post-state is the value after the call.
- id: FR-025-AC-9
  path: spec/kani/functional/FR-025-generated-subject-abi.md
  role: examined
  excerpt: PLANNED (IR-806). Every emitted composite shadow leaf binding retains its original operand/parameter
    node, typed path, declared domain and primitive draw type in symbolic call order; controls distinguish
    absent/null/present slots and option/union members. A missing, duplicate, unknown or domain/type-conflicting
    leaf refuses with a typed reason and no harness; parameter/literal and literal/literal operands retain
    their literal singleton identity/value with no unconstrained draw. Public reconstruction preserves
    these facts through FR-033.
- id: FR-026-AC-1
  path: spec/routed/functional/FR-026-backend-adapter-contract.md
  role: examined
  excerpt: The Kani adapter implements the adapter trait, and a function generic over that trait reaches
    the Kani adapter's generation arm, execution, transcript parser and witness renderer through it.
- id: FR-026-AC-4
  path: spec/routed/functional/FR-026-backend-adapter-contract.md
  role: examined
  excerpt: The execution evidence type is an associated type of the adapter trait, and the Kani adapter's
    is `KaniExecutionEvidence`.
- id: FR-026-AC-5
  path: spec/routed/functional/FR-026-backend-adapter-contract.md
  role: examined
  excerpt: PLANNED (IR-629). `Process(id)` has no CG adapter trait implementation or CG execution path;
    an inspection of all adapter implementations and execution dispatch arms finds no process arm.
findings:
- fnd: FND-181
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-032-AC-1
  related_ids: []
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  lines: '382'
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance). A public
    consumer passes the routed harness's `ScalarObligationIdentity` directly to the scalar builder; its
    typed call constructs neither `KaniObligationIdentity` nor V1 `ReplayPackage`. After the ordered setup
    checks, an otherwise valid request returns unsupported-capability only while a named step-5 capability
    is unavailable; when all are available it returns an admitted plan, and authenticated completion enables
    replay/conversion. Conversion of a capability refusal returns no settlement rather than a synthe
  finding: 'Inherited FR-032-AC-1 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-182
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-032-AC-4
  related_ids: []
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  lines: '385'
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance). The driver's
    actual native observation of the proved generated oracle is required and tied to this operand vector,
    limits and canonical proof-content identity; the checked completion requires driver authentication;
    a bare outcome/digest echo, missing observation, another artifact's observation or fresh regeneration
    that removes the arithmetic mutation refuses before QSL evaluation. Playback bytes and Kani falsification
    alone cannot supply a generated outcome.
  finding: 'Inherited FR-032-AC-4 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-183
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-032-AC-5
  related_ids: []
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  lines: '386'
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance). The public
    consumer executes the same proved generated artifact and invokes QSL between the builder and converter.
    QSL compares its actual native observation with authoritative exact operator evaluation using the
    four-operator projection owned by [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md)
    AC-37, without adding refusal-cause or accounting-counter equality; changing the generated arithmetic
    while keeping original node/operator identity changes the measured parity result. `Undefined`, `Inco
  finding: 'Inherited FR-032-AC-5 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-184
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-032-AC-6
  related_ids: []
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  lines: '387'
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance). For every
    `Diverged`, `Agrees`, `GeneratedFault`, `RefusedInput`, exact `Incomplete` and other `Refused` outcome,
    the converter compares the report's full claim to the retained sent claim and actual sent wire against
    the original proving context before conversion; a changed package/function/source/dependency or original
    limit refuses even with an equal claim. At the immutable-report/retained-binding boundary, an absent
    report, another result/run, or changing only the obligation, node, occurrence, kind, canonica
  finding: 'Inherited FR-032-AC-6 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-185
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-032-AC-7
  related_ids: []
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  lines: '388'
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance). QSL's divergence
    between generated and authoritative exact outcomes becomes a CG lowering fault and `Failed`; agreement
    despite Kani falsification becomes `Inconclusive(ScalarAgrees)`; native `Incomplete`/execution fault
    becomes `GeneratedFault` and `Failed` while exact `Incomplete` becomes `Incomplete(ResourceExhausted)`.
    `RefusedInput` preserves `invalid_runtime_input`; other typed QSL refusals follow the existing refusal
    map and non-fault refusals retain their code. This scalar route never returns `Reproduce
  finding: 'Inherited FR-032-AC-7 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-186
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-032-AC-8
  related_ids: []
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  lines: '389'
  excerpt: PLANNED (IR-631) / CODE-GATED (CG retention, observation and consumer conformance). The real
    QSL-emitted `x + 1` with input `Int[0,9]` and result `Int[0,10]` exercises routed generation, actual
    Kani arithmetic-mutation playback and same-proved-artifact native observation. A separate bounded-addition
    fixture with both operands and result in `[-1000,1000]` exercises an admitted operand pair whose exact
    result is outside the result range. The expected exact result is derived from the actual retained
    playback, and its correct refusal is never labelled a source violation. The fixture ranges and wit
  finding: 'Inherited FR-032-AC-8 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-187
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-032-AC-9
  related_ids: []
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  lines: '390'
  excerpt: PLANNED (IR-631) / CODE-GATED (authoritative metadata access and CG retention). Independent
    canonical-preimage checks include only the claimed application node, authentic expression occurrence,
    authoritative scalar kind and one argument per operand position in actual draw order, each with position
    ordinal, typed `OperandIdentity` and harness operand range. Actual graph references use `GraphChild
    { node_id }`; inline literals use `InlineLiteral { node_id, occurrence_key, position }` from the authentic
    application/occurrence and position, with their value in the singleton range. QSL emits no ext
  finding: 'Inherited FR-032-AC-9 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-188
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-032-AC-10
  related_ids: []
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  lines: '391'
  excerpt: PLANNED (IR-631) / CODE-GATED (typed scalar-family retention and actual CG/QSL conformance).
    When the authoritative scalar kind and retained positional metadata are available, the public shared
    mint produces the identity for the independently checked exact O-09 canonical scalar preimage, and
    the actual QSL operator route accepts it. A copied encoder, stand-in verifier or synthetic success
    cannot pass.
  finding: 'Inherited FR-032-AC-10 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-189
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-032-AC-11
  related_ids: []
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  lines: '392'
  excerpt: PLANNED (IR-631) / CODE-GATED (typed scalar-family/metadata retention and actual CG/QSL route).
    When the obligation digest differs from the retained positional preimage, after an otherwise membership-valid
    preimage member change with the old digest, the consumer observes QSL's typed `ScalarIdentity::Obligation`
    refusal with claimed/recomputed digests and `stale_dependency`/`revision-mismatch` before comparison.
  finding: 'Inherited FR-032-AC-11 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-190
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-032-AC-12
  related_ids: []
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  lines: '393'
  excerpt: PLANNED (IR-631) / CODE-GATED (typed scalar-family/metadata retention and actual CG/QSL route).
    When a correctly minted claim invents an application occurrence or operand child, or misstates an
    inline literal's value/singleton range, the consumer observes the corresponding merged typed membership
    refusal before identity comparison; self-consistent hashing cannot admit the invented metadata.
  finding: 'Inherited FR-032-AC-12 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-191
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-032-AC-13
  related_ids: []
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  lines: '394'
  excerpt: PLANNED (IR-631) / CODE-GATED (typed scalar-family/metadata retention and CG builder). When
    the owning mint cannot encode the preimage, the public `parity_obligation` returns `IdentityEncodeError`
    with no identity, and the CG builder returns typed setup refusal without replay. No claimed digest
    or usable sentinel can replace that error.
  finding: 'Inherited FR-032-AC-13 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-192
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-032-AC-14
  related_ids: []
  path: spec/replay/functional/FR-032-routed-scalar-replay-binding.md
  lines: '395'
  excerpt: PLANNED (IR-631) / CODE-GATED (actual CG scalar builder/converter). Adding the scalar builder/converter
    preserves existing function/clause carry-only behavior and function/frame identity encoding; none
    is redirected through scalar parity or changed by its positional argument rule.
  finding: 'Inherited FR-032-AC-14 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-193
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-033-AC-2
  related_ids: []
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  lines: '362'
  excerpt: PLANNED/GATED. Canonical round trips preserve every listed leaf family, including beyond-i64
    composite integer leaves as ExactInteger (while top-level Integer remains i64), IEEE NaN and signed-zero
    bits, normalized rational, retained decimal, delimiter-containing Unicode text, enum member, exact
    quantity/unit and reference identity, and every listed composite kind. Mixed nested values retain
    declaration/slot/member/presence/arity/order/multiplicity. No decoder claim depends on a currently
    supported Boolean/i64 harness alone.
  finding: 'Inherited FR-033-AC-2 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-194
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-033-AC-3
  related_ids: []
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  lines: '363'
  excerpt: 'PLANNED/GATED. Noncanonical JCS/tag/scalar/escape spelling, unknown declaration or unit/reference
    identity, wrong enum/union member, missing/extra or wrongly present record slot, wrong tuple arity,
    excess cardinality, out-of-domain leaf, duplicate set/ordered-set element under authoritative equality
    or altered canonical element order yields a typed operand/decode refusal before exact evaluation when
    that stage is reached, after common claim checks and same-valid-claim Disagreed precedence. An admitted
    claim whose operand field x is declared Int[0, 9] but supplied as x: 12 with Disagreed yields'
  finding: 'Inherited FR-033-AC-3 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-195
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-033-AC-4
  related_ids: []
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  lines: '364'
  excerpt: PLANNED/GATED. Public caller input exactly at the configured encoded-byte limit is admitted
    and one byte over refuses before parsing; occurrence/work exhaustion refuses with checked accounting.
    A 100,000-link admitted recursive value decodes, clones, compares, formats redacted Debug and drops
    on a 512 KiB native stack without a depth refusal. Its one-byte-too-small budget refuses truthfully;
    launcher capture limits alone cannot satisfy this check.
  finding: 'Inherited FR-033-AC-4 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-196
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-033-AC-5
  related_ids: []
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  lines: '365'
  excerpt: PLANNED/GATED. The retained actual assertion playback reconstructs operand values by the FR-025
    original-node/leaf bindings; another harness or persisted binding schema refuses claim binding. For
    the same admitted claim, swapped playback leaf order, missing/extra playback leaf or wrong primitive
    width refuses reconstruction only after Disagreed precedence; retained Disagreed remains Failed/CgDefect.
    Literal payloads remain their singleton value and absent/null/present states remain distinct. Removing
    or changing a leaf binding changes reconstruction or refuses, rather than silently decoding an
  finding: 'Inherited FR-033-AC-5 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-197
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-033-AC-6
  related_ids: []
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  lines: '366'
  excerpt: PLANNED/GATED. For falsified replay the driver executes the same proved generated artifact
    and supplies its actual native outcome/count; QSL receives this observation, the retained shadow result/count
    and the canonical content tie. Another artifact/context, fresh regeneration that removes the mutation
    or changed original limits refuses at claim binding. For the same admitted claim, missing observation
    cannot mask Disagreed/Failed/CgDefect; without Disagreed it refuses a replay continuing beyond F-1
    before exact evaluation, with no fabricated observation. A canned observation or Kani transcript
  finding: 'Inherited FR-033-AC-6 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-198
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-033-AC-8
  related_ids: []
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  lines: '368'
  excerpt: PLANNED/GATED. CG submits the original claim's operand declarations and literal singleton domains
    and consumes QSL-derived bound keys without a CG-authored declared-bound list. Omitting a bounded
    position or using an empty list with declared/unbounded/recursive keys leaves it uncovered; unknown/duplicate/kind-mismatched
    keys refuse. Exact declared coverage and a wider legal harness range cover; a tightened range/cardinality/depth
    does not silently cover under ADR-021 TX3. Enum Variants bounds cover only when every source-declared
    variant is included; a Variants bound naming an undeclared varia
  finding: 'Inherited FR-033-AC-8 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
bindings: []
review_head: 3d303ea6988496c4e1102faa58a56c1f260193c3
review_verdict: 'FAIL: 311 computed untagged/ignored-only obligations, four manual IR17 stage-evidence
  gaps and four actual inherited dangling trace annotations remain. The two IR877 diagnostic controls
  now bind real tests; actual AC56 inspection is clean. No whole-ticket completion is claimed.'
part: 11
parts: 18
scope_kind: full-repository criterion presence; no blanket semantic acceptance
```

+++