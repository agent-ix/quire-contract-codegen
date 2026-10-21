<!-- reviewer repo=agent-ix/quire-contract-codegen visibility=public quoin=0.28.4 module=spec-artifacts-process@b82e42e893612b4b4ee94127714b4577ee893502 id=SR-15291 method=gap-analysis lang=rust pr=quire-contract-codegen#501 model=unavailable run=d9d507d8-42eb-4ddc-a8d0-3eed0b37342f -->

SR-15291 gap-analysis, part 6/18. FAIL: 311 computed untagged/ignored-only obligations, four manual IR17 stage-evidence gaps and four actual inherited dangling trace annotations remain. The two IR877 diagnostic controls now bind real tests; actual AC56 inspection is clean. No whole-ticket completion is claimed.

| FND | Severity | Check | Summary |
| --- | --- | --- | --- |
| FND-091 | high | coverage | Inherited FR-018-AC-33 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-092 | high | coverage | Inherited FR-018-AC-34 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-093 | high | coverage | Inherited FR-018-AC-35 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-094 | high | coverage | Inherited FR-018-AC-36 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-095 | high | coverage | Inherited FR-018-AC-37 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-096 | high | coverage | Inherited FR-018-AC-38 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-097 | high | coverage | Inherited FR-018-AC-39 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-098 | high | coverage | Inherited FR-018-AC-40 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-099 | high | coverage | Inherited FR-018-AC-41 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-100 | high | coverage | Inherited FR-018-AC-42 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-101 | high | coverage | Inherited FR-018-AC-43 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-102 | high | coverage | Inherited FR-019-AC-3 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-103 | high | coverage | Inherited FR-019-AC-15 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-104 | high | coverage | Inherited FR-019-AC-24 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-105 | high | coverage | Inherited FR-019-AC-34 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-106 | high | coverage | Inherited FR-019-AC-35 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-107 | high | coverage | Inherited FR-019-AC-36 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-108 | high | coverage | Inherited FR-019-AC-37 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |

+++ [reviewer data]

```yaml
clean: false
scope:
- id: FR-015-AC-110
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: PLANNED (IR-598 collection slice; gated on IR-858, AD-004 step 4b, and on AD-004 step 4h, because
    the collection family renders through `HarnessSpec` and `render.rs` only after 4h). The gate opens
    when the AC-99 installed-backend controls are green, with their tagged tests present (`tc_025_real_kani_collection_claim_verifies_and_mutated_claim_falsifies`
    and the symbolic-domain tests tagged FR-015-AC-96 and FR-015-AC-99 in `tests/it/kani_obligations_symbolic_domains.rs`).
    Evidence for a real-Kani precondition is a recorded `make kani` or `make kani-gate` run on the slice
    PR's head that is green
- id: FR-015-AC-111
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: PLANNED (IR-598 graph and final slice; gated on IR-858, AD-004 step 4b, on FR-015-AC-109 and
    FR-015-AC-110 having landed, and on IR-805, whose graph claims need CG integration of the merged IR-851/IR-852
    accessors and QSL-820 real-package population emission (after QSL-819)). Graph claims are served by
    FR-015-AC-98 and FR-015-AC-105 to FR-015-AC-108, and FR-015-AC-106's real-Kani graph replay is the
    replacement for the corpus graph replay; Evidence for a real-Kani precondition is a recorded `make
    kani` or `make kani-gate` run on the slice PR's head that is green, cited in the PR (the real-Kani
- id: FR-015-AC-112
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Implemented (IR-858, AD-004 step 4b). `HarnessSpec::new` is the only constructor and refuses
    with a typed `HarnessSpecError`, never a panic and never a harness: a spec with no cover or more than
    one cover; a cover whose condition is the literal `true`; no `Call` or more than one; an `Input` or
    `Assume` after the `Call`; an `Assert` before it; a binding name bound twice (other than an input''s
    own rebinding); a name, `result` or `callee` that is not a valid Rust identifier; an input domain
    with `minimum` above `maximum` or outside its type; an `unwind` outside `1..=MAX_OBLIGATION_UNWIND`;
    a mess'
- id: FR-015-AC-113
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Implemented (IR-858). For every spec that constructs, `render` emits the header, the oracle
    source, the module wrapper and the proof attribute, then the spec''s statements in spec order, then
    its one cover as the last statement of the function: after every assumption, the subject call and
    every assertion, whatever order a family listed them in. A hand-built spec in each of several statement
    orders renders its cover last; a cover is not a field of `body` and no statement order moves it (AD-004
    "The HarnessSpec and render contract (step 4b)", R-1; FR-015-AC-7).'
- id: FR-015-AC-114
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Implemented (IR-858). `render` of one spec returns equal bytes on repeated calls and in a second
    process, and a spec whose `Input` list is reordered renders the reordered source and not the same
    bytes (AD-004 "The HarnessSpec and render contract (step 4b)", R-1).
- id: FR-015-AC-115
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Implemented (IR-858). After the scalar port, for each claim of a fixed set (each of `quire.op.integer.add`,
    `sub`, `mul` and `negate`; a claim with a literal operand; a claim with an inferred operand; a claim
    with each operand at an `i64` extreme; claims at two unwind values including `MAX_OBLIGATION_UNWIND`;
    claims at two ceilings values) the ported path renders the same artifact (path and source bytes),
    `ScalarObligationIdentity`, `options` and `ObligationRecord`, and the same digests derived from them,
    as a test-only reference copy of the pre-port `render_scalar` computes in the same test. '
- id: FR-015-AC-116
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'Implemented (IR-858). `render` holds no family-specific template text: for hand-built specs
    whose label, expressions and messages are not the scalar family''s, the rendered source consists of
    the fixed header, the spec''s `oracle_source`, the module wrapper, the proof attribute, the spec''s
    statements rendered from their typed fields, and the one cover, and contains no other text; and the
    ported scalar harness is emitted by `render`, so that the FR-015-AC-58 inspection drives `render.rs`
    for it (AD-004 "The HarnessSpec and render contract (step 4b)", R-1 item 3).'
- id: FR-015-AC-117
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Implemented (IR-858). With the installed backend, after the scalar port, the unmutated scalar
    harness classifies `Verified` with its cover satisfied and the existing scalar mutation control (the
    oracle's `Add` swapped for `Subtract`, FR-015-AC-37) classifies `Falsified`; and a hand-built `HarnessSpec`
    with an unsatisfiable `Assume`, rendered by `render`, does not classify `Verified` (L-4). This run
    is a smoke over the ported source and not the evidence of equivalence, which FR-015-AC-115 is (AD-004
    "The HarnessSpec and render contract (step 4b)", P-3).
- id: FR-015-AC-118
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'For an admitted V2 scalar `implies` claim over two Boolean operands, the generated harness
    embeds its FR-014 oracle as FR-015-AC-20 requires and draws both operands symbolically. It calls `quire_contract_runtime::operators::implies_short_circuit`
    with `left` and a closure returning `right` as the subject expression, asserts that subject result
    equals an independent native `!left \|\| right` expectation over the same operands, and separately
    asserts that the oracle''s `Outcome<bool>` is `Completed` with that expectation; neither subject nor
    oracle computes the native expectation. With installed '
- id: FR-015-AC-119
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'PLANNED (IR-917; depends on FR-015-AC-40 and FR-015-AC-41). QSL source authors an operation
    with one parameter `amount: Int[0, 1000]` and its V2 postcondition `amount < 1000 implies amount +
    1 <= 1000`; both occurrences of `amount` read that operation parameter, not a state field or subject-produced
    post-state value. The admitted `CheckedPackageV2` yields a clause-identified FR-014-AC-38 oracle and
    an FR-015 postcondition harness for that same clause, not a scalar implication harness. Under FR-015-AC-39
    the harness draws the parameter once within its declared finite domain and passes that same'
- id: FR-015-AC-120
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'PLANNED (IR-942; depends on QSL ADR-013 O-07/O-09 and QSV FR-123). After the selected owner
    and occurrence validate under AC-122, CG forms the per-step record content identity from an RFC 8785
    object whose only members are `subject_kind: "protocol-refinement-step"` and `record: StepSimulationRecord`
    encoded by QSV. Two separately valid selected refinements with the same obligation and hypotheses
    but different record `refinement` values, each with its own matching occurrence, yield different content
    identities; a record with a mismatched owner has no identity to compare. Within one admitted ref'
- id: FR-015-AC-121
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: PLANNED (IR-942). A per-step `KaniObligationIdentity` uses the QSV-owned `StepSimulationRecord`
    as a typed subject, carries its content identity and the owning refinement's O-07 `generated` occurrence
    key separately, and keeps any CG proof or obligation kind in the harness record rather than the record
    content preimage. CG publishes a persisted harness-record digest over every `KaniObligationIdentity`
    member except `source_span`; changing any included member changes this digest, while changing only
    `source_span` does not. This digest is distinct from the per-step record content identity. The r
- id: FR-015-AC-122
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: 'PLANNED (IR-942). A selected proof-eligible refinement under QSpec FR-322-AC-88 through AC-90
    with an `Initial` record and `Step { row: i }` records for its admitted FR-375 `steps` order yields
    generated occurrences on the owner node at ordinals 0 and `i + 1`, respectively. An ineligible refinement
    under FR-322-AC-91 (hidden field, history field, or `any` step target) carries only its `declaration`
    occurrence and yields no per-step record, harness, or proof credit; the IR reader refuses a generated
    occurrence on it as `invalid_package`/`invalid-value` before CG setup. Two step rows tied on nod'
- id: FR-015-AC-123
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: External public-consumer calls to `prepare_bounded_collection_query`, `prepare_finite_graph_reaches`
    and `generate_bounded_kani_corpus_case` have the direct result signatures specified by `interface-001`,
    with `KaniOutcome` as their error type and no public `FamilyLoweringError`, `BoundedCorpusError`,
    `OutcomeConstruction` branch, or replacement alias or wrapper.
- id: FR-015-AC-124
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: After infallible-constructor adoption, each retained family/corpus adverse control returns
    the original exact non-success kind, typed cause, source identity and selected profile/bound context,
    with no Boolean claim, partial artifact or premature identity claim; successful outputs and failure
    ordering remain unchanged.
- id: FR-015-CON-1
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Clause ABI construction SHALL check both endpoints of a bounded integer dependency before conversion
    to i64 and refuse one outside i64 as `UnsupportedDependency` in the clause-lowering detail, without
    producing a binding. | Integrity
- id: FR-015-CON-2
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Subject binding SHALL check both endpoints of a bounded integer dependency before conversion
    to i64 and refuse one outside i64 as `UnsupportedBinding`, without producing a binding or harness.
    | Integrity
- id: FR-016-AC-1
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: Every retained witness decodes into typed values or is refused as malformed.
- id: FR-016-AC-2
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: An out-of-domain witness is reported as such and never as a contract failure.
- id: FR-016-AC-3
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: An in-domain witness is reported as a failure only when native replay reproduces the same typed
    outcome.
- id: FR-016-AC-4
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A native replay that runs and disagrees with the harness result yields a typed mismatch, and
    never a failure or a replay error.
- id: FR-016-AC-5
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A witness bound to a different harness identity is reported as malformed and never replayed.
- id: FR-016-AC-6
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: Every witness decode receives required caller-supplied byte and value-count bounds at its entry
    point; replay inputs carry both as non-optional fields with no default, and an over-bound witness
    is reported malformed without reading beyond either bound.
- id: FR-016-AC-8
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A witness schema binds the obligation's persisted argument bindings, in the order the obligation
    persists them, position for position to the harness's symbolic arguments, and each decoded value is
    named by the binding at its position; a binding that is not an argument is refused with a typed schema
    refusal that reports no failure and is none of the replay results.
- id: FR-016-AC-9
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A decoded falsification whose witness makes the native function evaluate the clause to false
    settles as `reproduced-with-evaluated-witness` with category `violation`; the verdict depends on the
    witness value, so a witness at which the function holds does not settle so.
- id: FR-016-AC-10
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A decoded falsification at which the native function evaluates the clause to true settles `inconclusive`
    with the proved violation and the replayed success both named.
- id: FR-016-AC-11
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: The adapter refuses, with a distinct typed error each, a decoded value no replay parameter
    binds, a transcript field holding a delimiter, a transcript QSL does not admit, a request QSL refuses
    with any `ReplayRefusal` other than `Fault` (returned with its cause), and a witness-sourced request
    that settles on the input arm.
- id: FR-016-AC-12
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A `replay` call that returns `ReplayRefusal::Fault` is returned as `SpineReplayError::Refused`
    carrying that fault.
- id: FR-016-AC-13
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A `Witness` arm that settles `reproduced-with-evaluated-witness` with a category other than
    `violation` yields a typed mismatch, and never a reproduced failure.
- id: FR-016-AC-14
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A proved unit that imports a locked dependency is compiled by `qsl_replay::call_site` together
    with that dependency's lock source, and `qsl_replay::replay` settles the unit's falsification through
    the imported function.
- id: FR-016-AC-15
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: The replay request's `package.dependencies` carries one entry per lock dependency selection,
    in ascending identity order, with the lock's recorded `package_id`.
- id: FR-016-AC-16
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A unit whose import no lock dependency selection supplies is refused at the call site.
- id: FR-016-AC-17
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A lock recording another `package_id` for a dependency the unit imports is refused by QSL as
    a dependency identity mismatch.
- id: FR-016-AC-18
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A lock selecting a library the unit does not import is refused by QSL as unselected.
- id: FR-016-AC-19
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: Lock libraries whose sources share a source owner are refused as no dependency input, before
    the call site compiles anything.
- id: FR-016-AC-20
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A lock library whose source has the unit's own source owner is refused by the call site.
- id: FR-016-AC-21
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: 'The function path''s request `obligation_identity` holds the function-contract obligation
    digest of quire-spec-language ADR-013 O-09: the digest of the RFC 8785 encoding, made by `quire-canonical`
    (ADR-013 §2''s one encoder, AD-003 E-1), of the `FunctionSite`''s `function` node id, its `declaration`
    occurrence key, the `ObligationKind` of the harness being replayed and `arguments`, each a parameter
    node id with its declared domain, ordered ascending by identifier as O-09 fixes and not in the declared
    order of `FunctionSite.parameters`. It is never the digest of the transcript. The `arguments` are'
- id: FR-016-AC-22
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: 'For two functions with the same parameters, the identities differ. The identity of one function
    is the same across recompiles that only add comments or blank lines, and across a change of the harness''s
    source span. It differs between two different `ObligationKind`s over the same function, and when any
    argument''s domain differs. A function yields one identity per existing `ObligationKind` it requests,
    not one per conjunct of its body: a function whose body has two conjuncts, replayed under one kind,
    has one identity.'
- id: FR-016-AC-23
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: 'The function-contract identity function derives nothing from the package: with the `FunctionSite`''s
    `function` and `declaration`, the kind and the arguments held fixed, perturbing every other node of
    the compiled package (an unrelated declaration, the order of `FunctionSite.parameters`) leaves the
    identity unchanged, and a perturbed `function` or `declaration` changes it.'
- id: FR-016-AC-24
  path: spec/replay/functional/FR-016-witness-native-replay.md
  role: examined
  excerpt: A lock whose only defect is one library identity selected twice is refused by QSL's `DependencyInput::new`
    as `DuplicateIdentity`, code `invalid_package` with cause `conflicting-definition`, and reaches the
    caller as `DependencyLockError::Input`.
- id: FR-017-AC-2
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: An absent launcher is refused with a typed reason naming its path before anything runs.
- id: FR-017-AC-4
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A run is verified only when the process exited successfully, the report states success, and
    every cover property is reported satisfied; a successful run with an unsatisfied cover is cover-unsatisfied
    with its satisfied and total counts; a report of success from an unsuccessfully exited process, a
    report with no cover property and a report with no successful check are each inconclusive with their
    own reason; console banners never decide the verdict.
- id: FR-017-AC-5
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A failed non-unwinding check with a concrete playback is falsified carrying that playback verbatim
    and never the playback of a satisfied cover; a failure with no playback is inconclusive for that reason;
    a failed unwinding assertion is inconclusive as an exhausted bound rather than falsified, and a succeeded
    unwinding check in a results listing is not a failure.
- id: FR-017-AC-6
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: The execution evidence of a single run carries the obligation kind, the harness path, the launcher
    path, the complete argument vector, the unwind bound, the solver, the exit code and the outcome.
- id: FR-017-AC-7
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A crate whose library source does not contain the harness source byte for byte is refused,
    and no backend runs.
- id: FR-017-AC-11
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'A routed FR-022/FR-014 exact-scalar harness (`KaniScalarObligationHarness`) runs through `execute_kani_obligation`
    and `kani_launch_command` the same way an FR-015 contract harness does: a crate whose library source
    lacks its generated source byte for byte is `HarnessNotInCrate`, its covers classify a run identically
    (all satisfied is verified, an unsatisfied one is cover-unsatisfied, none printed is inconclusive),
    and its evidence carries `None` for obligation kind, since an exact-scalar claim carries no contract
    role.'
- id: FR-017-AC-12
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: Real Kani captures of a verified run, a falsified run with a playback, an exhausted unwind
    bound, a run whose only check is unreachable, a partly satisfied cover and a run with no cover each
    parse into the expected typed report of harness status, successful checks, cover counts and failed
    checks, and classify to the expected outcome; the falsifying playback block passes through verbatim.
- id: FR-017-AC-13
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A run whose process exited successfully and whose backend reported success with zero successful
    checks is inconclusive with the vacuous-proof reason, never verified, except that a precondition harness,
    which asserts nothing beyond its cover, is decided by its cover summary.
- id: FR-017-AC-14
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A launcher whose stdout or stderr carries more than 8 MiB times its harness count is stopped,
    its process group is killed, and the run is refused with `kani_output_over_limit` naming the stream,
    the limit and the count, with no outcome and no retained text; a launcher whose stream is exactly
    the limit completes with its real exit status and the whole stream retained. No stream is ever truncated
    and then classified.
- id: FR-017-AC-15
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: A timeout too large to add to the current instant never elapses and does not panic.
findings:
- fnd: FND-091
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-018-AC-33
  related_ids: []
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  lines: '701'
  excerpt: Compare the context against lowered ContractPackage identity instead of original source package
    identity, skip full-package/source-lock authentication, or expose the retained context before those
    checks.
  finding: 'Inherited FR-018-AC-33 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-092
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-018-AC-34
  related_ids: []
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  lines: '702'
  excerpt: Build shape/index from only used literals, substitute positions for member keys, or conflate
    identical case names under distinct declarations.
  finding: 'Inherited FR-018-AC-34 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-093
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-018-AC-35
  related_ids: []
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  lines: '703'
  excerpt: Collapse Unknown case and declaration, drop Foreign package details, map Resource/Fault to
    Package, or publish a partial context after stopped admission.
  finding: 'Inherited FR-018-AC-35 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-094
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-018-AC-36
  related_ids: []
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  lines: '704'
  excerpt: Emit a NodeKey literal, accept an arbitrary caller index, skip check_type on the prepared environment,
    or evaluate a wrapper for another descriptor.
  finding: 'Inherited FR-018-AC-36 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-095
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-018-AC-37
  related_ids: []
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  lines: '705'
  excerpt: Add replay compilation to scalar proof execution, let scratch and emitted manifest sources
    drift, or count native enum success as an original Kani receipt.
  finding: 'Inherited FR-018-AC-37 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-096
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-018-AC-38
  related_ids: []
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  lines: '706'
  excerpt: Universally demand context before a key-free refusal, mistake standalone absence for source
    authentication, or return an empty index before complete closure resolution.
  finding: 'Inherited FR-018-AC-38 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-097
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-018-AC-39
  related_ids: []
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  lines: '707'
  excerpt: Check only an operand's outer type, skip a nominal conversion target/nested field, fabricate
    a key, or let capability checks reorder charged refusal/cycle precedence.
  finding: 'Inherited FR-018-AC-39 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-098
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-018-AC-40
  related_ids: []
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  lines: '708'
  excerpt: Treat Some(wrong) as None or skip the package guard because all selected types are primitive.
  finding: 'Inherited FR-018-AC-40 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-099
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-018-AC-41
  related_ids: []
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  lines: '709'
  excerpt: Build only used enum literals or recover from a failed nominal lookup with an empty index.
  finding: 'Inherited FR-018-AC-41 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-100
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-018-AC-42
  related_ids: []
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  lines: '710'
  excerpt: Drop private binding/context checks for None, allow a nominal recipe without context, omit
    check_type, accept a caller index, or make native authentication optional.
  finding: 'Inherited FR-018-AC-42 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-101
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-018-AC-43
  related_ids: []
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  lines: '711'
  excerpt: Narrow a direct canonical Integer operand or target using an unrelated sibling integer_range,
    infer member ownership from the shared base key, or label an erased source conversion as a retained
    conversion-node control.
  finding: 'Inherited FR-018-AC-43 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-102
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-019-AC-3
  related_ids: []
  path: spec/routed/functional/FR-019-capability-settlement.md
  lines: '519'
  excerpt: On the Kani arm, an unbounded extent against a `bounded`-only advertisement settles `requires-bound`
    when a finite bound is available, rather than `unsupported`; with no finite bound available it settles
    `unsupported`, warned, with `unsupported_projection`/`unbounded-extent`; and it never settles `supported`.
  finding: 'Inherited FR-019-AC-3 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-103
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-019-AC-15
  related_ids: []
  path: spec/routed/functional/FR-019-capability-settlement.md
  lines: '547'
  excerpt: PLANNED (IR-633). CG's `BackendDescriptor` has a typed `ProviderOrigin` projecting exactly
    QSL layer R's `Linked` and `Process` meanings (QSL FR-288, ADR-029 PV-1); layer R owns that vocabulary.
    The driver projects each descriptor from `Registry::descriptors()` once into CG's descriptor, copying
    `id` and advertised pairs unchanged and mapping `origin()` exhaustively, `Linked` to `Linked` and
    `Process` to `Process`, with no wildcard or origin inference from identity, manifest or provider bytes
    or a side map. Two registry descriptors identical except for origin produce CG descriptors identical
    e
  finding: 'Inherited FR-019-AC-15 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-104
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-019-AC-24
  related_ids: []
  path: spec/routed/functional/FR-019-capability-settlement.md
  lines: '548'
  excerpt: 'PLANNED (IR-338). Given QSL FR-335''s `value-validity` claim rooted at an unbounded `s: Set<Int[0,9999]>`,
    an empty candidate registry settles `unsupported` with a warning naming `value-validity` and emits
    no artifact or Kani outcome; one linked Kani candidate advertising (`value-validity`, `bounded`) with
    `finite_bound_available=true` settles the unbounded item `requires-bound` without substituting a bound
    or emitting an artifact; a separately requested bounded `ProofBound::Cardinality{maximum: 8}` item
    with that same sole candidate settles `supported` under its own request index, while the un'
  finding: 'Inherited FR-019-AC-24 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-105
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-019-AC-34
  related_ids: []
  path: spec/routed/functional/FR-019-capability-settlement.md
  lines: '558'
  excerpt: PLANNED (IR-516). With one EN-1 candidate, an advertised admitted TP-1 through TP-4, TP-5,
    SG-1 through SG-3, or HP-1 model/HP-2/HP-3/HP-6 form settles through its explicit-state arm with the
    recorded form and model-or-protocol subject intact; an unadvertised form or HP-1 code claim does not
    settle `supported` through EN-1.
  finding: 'Inherited FR-019-AC-34 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-106
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-019-AC-35
  related_ids: []
  path: spec/routed/functional/FR-019-capability-settlement.md
  lines: '559'
  excerpt: PLANNED (IR-516). With one SMT temporal candidate advertising the item's kind and form, TP-1
    through TP-4 route only to an EN-2 or EN-3 method ADR-018 assigns to that form and subject; TP-4 does
    not route to EN-3, and a form absent from the candidate's advertised form set settles `unsupported`
    with no engine request.
  finding: 'Inherited FR-019-AC-35 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-107
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-019-AC-36
  related_ids: []
  path: spec/routed/functional/FR-019-capability-settlement.md
  lines: '560'
  excerpt: PLANNED (IR-516). A TP-5 whole-refinement `temporal-satisfaction` item retains its safety and
    optional liveness halves on the concrete subject through EN-1, and the same whole item reaches an
    advertising SMT arm for RE-3 refutation; a separately requested RE-2 `operation-contract` item reaches
    the SMT arm for per-step safety checking, but its disposition cannot substitute for the whole-refinement
    item. An arm lacking the required TP-5 form settles the item `unsupported`.
  finding: 'Inherited FR-019-AC-36 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-108
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-019-AC-37
  related_ids: []
  path: spec/routed/functional/FR-019-capability-settlement.md
  lines: '561'
  excerpt: PLANNED (IR-516). HP-1 model, HP-2, HP-3 and HP-6 reach an advertising EN-1 arm; only HP-2
    reaches an advertising SMT arm under its applicable EN-2 or EN-3 method. HP-4 and an unadvertised
    HP form settle unsupported before an engine, HP-5 stays on its possible-property path, and an HP-1
    code claim is not settled as an EN-1 model claim.
  finding: 'Inherited FR-019-AC-37 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
bindings: []
review_head: 3d303ea6988496c4e1102faa58a56c1f260193c3
review_verdict: 'FAIL: 311 computed untagged/ignored-only obligations, four manual IR17 stage-evidence
  gaps and four actual inherited dangling trace annotations remain. The two IR877 diagnostic controls
  now bind real tests; actual AC56 inspection is clean. No whole-ticket completion is claimed.'
part: 6
parts: 18
scope_kind: full-repository criterion presence; no blanket semantic acceptance
```

+++