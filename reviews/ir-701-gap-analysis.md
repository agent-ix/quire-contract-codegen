---
id: SR-9301
title: "IR-701 coherent cohort gap-analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; IR-701; Cargo.lock, Cargo.toml, src/kani/classify.rs, src/kani/generate/frame.rs, src/kani/run/execute.rs, src/kani/run/launch.rs, src/kani/run/mod.rs, src/kani/run/namespace.rs, src/kani/run/owners.rs, src/kani/run/tool.rs, src/kani/terminal.rs, src/routed/contract_package_tests.rs, tests/composite_equality_support/package.rs, tests/exact_function_support/package.rs, tests/exact_scalar_support/package.rs, tests/it/composite_original_eq.rs, tests/it/composite_parity_converter.rs, tests/it/exact_scalar_generation.rs, tests/it/kani_batching.rs, tests/it/kani_obligations.rs, tests/it/kani_obligations_state_frame.rs, tests/it/kani_witness_join.rs, tests/it/skeleton_spine.rs, tests/it/terminal_map.rs, tests/state_frame_support/native_twin.rs"
review_set: subset
---

## Summary

Reviewed frozen candidate `887462b3e4be778bae3e60aa04c1db12825eecd9` against `dfb6113bbbe9948f1072ed12647f121afe4c796a`. Ticket: IR-701. Source and index were clean before inspection. This is the one prospective review of the eventual PR; publication and tracker posting wait for its actual number.

No applicable committed AssuranceProfile was found. No spec or plan path changed, so no spec-review method applies. Repository AGENTS/CLAUDE, Cargo, deny, clippy and rustfmt conventions and the Rust idiom checklist were applied; repository conventions supersede portable defaults.

## Verdict

**FAIL** — Two medium evidence gaps require fixes; the untagged examined criterion makes the gap verdict FAIL.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-030-AC-18 has no trace-tagged test: the changed certified Checks assertions bind only AC-4 and AC-5, so the computed matrix leaves the delivered Kani certification criterion untagged. | tests/it/terminal_map.rs:1005 |
| FND-002 | medium | Measured memory exhaustion is not exercised while cancellation competes: the overage control records Deadline only after execute returns, so moving cancellation ahead of memory sampling would leave the controls green while violating memory-stop priority. | src/kani/run/execute.rs:1494 |

## Inspection

The real launcher reads the authentic `Cancel::cause()` before spawn and in its existing bounded loop; it retains its source in the selected conclusion before teardown. Memory/failure/elapsed checks precede cancellation and accepted completion is never relabelled. Cleanup and capture settlement preserve existing refusals. Single cancellation retains invocation/identity/ceilings/resource observations with no exit/checks/backend verdict; a multi-member group yields `BatchCancelled` with no member evidence. Handle equality uses the dependency's shared identity, and installation grouping includes immutable owner composition. The failure decorators call real RSS/poll operations and inject at the owning port; Drop retries genuine teardown rather than replacing the event reader or settlement.

Both ordinary terminal maps exhaustively construct `Checks` plus `Certified`, preserve zero-check vacuity and ordinary settlement refusal, and share typed source conversion; shadow settlement remains QSL-derived. Variant fields use the existing non-i64 range refusal, backed by a QSL-emitted selected-model variant test and persisted identity roundtrip. Result-returning evidence insertion sites fail explicitly instead of swallowing refusals.

The fixture edits remove false result-type disambiguators and use distinct genuine operand identities, preserving declared Integer/Boolean result correspondence. The classifier-only function corpus remains explicitly separate from source-bound declarations; unchanged source-call construction retains callee argument zero, declared parameter slots and ordered arguments. Existing conversion/refusal expectations remain; no negative oracle was removed. All 25 changed paths were read. Cargo.lock contains one entry per first-party git crate; direct quire-exact and QSL resolve one coherent exact cohort. No new copied upstream file, encoder, compatibility layer, unsafe block, lint waiver or CI-lane narrowing was found.

## Coverage

Plan completion: not assessed

Reconciliation: `quire matrix --scope /home/peter/dev/worktrees/cg-ir701-coherent-cohort --format json`, computed static axis saved as `matrix.json`; no Quoin execution-store evidence claimed. This is a diff-scoped subset review, not a whole-repository verdict. Semantic intent/source/oracle inspection was explicitly assigned in the dispatch. Existing planned SMT certificate checking (FR-030-AC-19 through AC-22) is context only; this Kani cohort adaptation adds no SMT producer/checker, and cannot certify that planned seam. Existing planned composite/source-call obligations are not relabelled as delivered by fixture edits.

All seven FR-040 criteria have computed binders. FR-030-AC-18 has zero binders, despite certification assertions tagged to neighboring criteria. The FR-040-AC-4 binding at the memory-overage control only exercises late post-return cancellation, leaving the live overage competition subcase untested. No new unowned production behavior or stub was found.

## Evidence Limitations

Reviewer executed no Cargo/build/Kani command or gate-lock operation, as assigned. Inspected author receipt, native-counts and actual `ci.log` at `/tmp/ix-handoff/ir701-author-prepr-native-887462b/`: make ci exit 0 on this exact candidate, final clean guard and unchanged lock; actual log shows twice 263 unit + 545 integration + 1 doc tests passed, with 29 named proof controls ignored per native lane (1618 total native passes). These are attributed author-run results, not independently executed reviewer gates. Current-candidate real Kani is pending; old-head Kani is not candidate proof. `gh repo view` returned HTTP 401, and actual PR number is unresolved; root independently verified public visibility through the GitHub connector, and marked-comment payloads remain unposted. No aggregate merge approval is given.

## Examined Scope

```yaml
scope:
- id: FR-040-AC-1
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: A caller retains a live `Cancel`, sends its clone through a public single-run
    request, and the bounded launcher observes it before spawn and during an actual
    running backend. Pre-spawn cancellation starts no backend; running cancellation
    confirms owned teardown before evidence returns.
- id: FR-040-AC-2
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: In two live single runs, issue `Requested` then `Deadline`, and reverse
    the calls. Each run's actual launcher stop, `KaniExecutionEvidence.outcome`, constructed
    IR `Cancelled { source }` and QSL `IncompleteCause::Cancelled { source }` retain
    the first cause; a later event cannot change them. A hand-built IR outcome alone
    does not satisfy this criterion.
- id: FR-040-AC-3
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: A run that reaches its execution ceiling with no cancellation event remains
    `TimedOut`. A `Deadline` event recorded only after completion, memory/capture
    stop or elapsed-ceiling stop does not turn that result into `Cancelled`.
- id: FR-040-AC-4
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: Completion before observed cancellation keeps the report-based result;
    measured memory exhaustion, memory observation failure, output over-limit, unread
    capture and unconfirmed cleanup keep their existing result/refusal and confirmed
    teardown rules when cancellation competes. A cancellation observed first and causing
    stop is not overwritten by later completion or elapsed time.
- id: FR-040-AC-5
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: A multi-member group sharing one handle stops wholly on its observed cause,
    returns `BatchCancelled { source, members }`, and classifies no member; another
    group using a distinct handle retains its own result. Reversing the two cause
    calls changes the cancelled group's source and not the other group's evidence.
    Distinct handles never share one launcher.
- id: FR-040-AC-6
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: The single-run producer constructs an IR `KaniOutcomeKind::Cancelled {
    source }` from settled execution evidence, and its real call path reaches the
    FR-030 map. Mapping both IR source variants exhaustively yields the identical
    `quire_exact::CancelCause` inside QSL's `IncompleteCause::Cancelled`; neither
    metadata nor a copied enum determines the source.
- id: FR-040-AC-7
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: The direct `quire-exact::CancelCause` constructs `qsl-replay::IncompleteCause::Cancelled`
    through a type-checked exhaustive conversion, while all QSL imports remain through
    `qsl-replay`; no implicit cancellation control, source fallback, or compatibility
    overload is present.
- id: FR-030-AC-1
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: Every pair (`KaniOutcomeKind`, replay settlement) that the map's input
    can express maps to exactly one `TerminalValue`.
- id: FR-030-AC-2
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: '`Refused`, `InvalidInput` and `IncompleteInput` map to `Declined` with
    `ProofRefusalCause::Refused`, `InvalidInput` and `IncompleteInput` respectively,
    so no refusal cause is lost, and each carries the outcome''s `Std001Code` unchanged
    as `DeclineCode::Std001`, including a code STD-001 does not register.'
- id: FR-030-AC-3
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: '`TimedOut` and `ResourceExhausted` map to their distinct `Incomplete`
    causes. `Cancelled { source: Requested }` and `Cancelled { source: Deadline }`
    map to `IncompleteCause::Cancelled` carrying the corresponding `quire_exact::CancelCause`,
    with no source inferred from `code`, `source_id`, `context` or elapsed time.'
- id: FR-030-AC-4
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: '`Proved` with three SUCCESS checks maps to `Proved { basis: Checks { success_checks:
    3 }, certification: Certified }`, zero checks maps to `Proved { basis: Checks
    { success_checks: 0 }, certification: Certified }` (category inconclusive/vacuous),
    and `Counterexample` with reproduced replay maps to `Refuted`.'
- id: FR-030-AC-5
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: '`Inconclusive` with cause `kani_vacuous_proof` maps to `Proved { basis:
    Checks { success_checks: 0 }, certification: Certified }` (category inconclusive/vacuous),
    and `Inconclusive` with any other cause maps to `Failed`.'
- id: FR-030-AC-6
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: No outcome maps to `Tested`.
- id: FR-030-AC-7
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: The map is one `match` over the pair (`KaniOutcomeKind`, replay settlement)
    with no wildcard arm.
- id: FR-030-AC-8
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: '`Unavailable` with cause `kani_solver_absent` maps to `Unsupported(SolverAbsent)`;
    with `kani_backend_absent` or any other cause it maps to `Unsupported(BackendAbsent)`.'
- id: FR-030-AC-9
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: '`Counterexample` with a replay disagreement maps to `Inconclusive(ReplayParity)`,
    and with a non-fault `ReplayRefusal` maps to `Inconclusive(ReplayRefused)` carrying
    `ReplayRefusal::code()` of that refusal.'
- id: FR-030-AC-10
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: '`Counterexample` with a fault, walked through every wrapper FR-029-AC-10
    lists, and with each CG-raised failure FR-029-AC-11 lists, maps to `Failed`.'
- id: FR-030-AC-11
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: Across every replay settlement other than reproduced, `Counterexample`
    maps to a value other than `Refuted`.
- id: FR-030-AC-12
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: '`Counterexample` with a non-fault `CallSiteRefusal` or a `DependencyLockError::Input`,
    each bare and wrapped, maps to `Inconclusive(ReplayRefused)` carrying that refusal''s
    QSL catalog code, never to `Declined`.'
- id: FR-030-AC-13
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: '`Counterexample` with a `DependencyLockError::Input` that carries QSL''s
    `DuplicateIdentity` refusal (code `invalid_package`), as a lock whose only defect
    is a repeated library identity produces it (FR-016-AC-24), maps to `Inconclusive(ReplayRefused)`
    carrying `invalid_package`.'
- id: FR-030-AC-14
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: A `Counterexample` given no replay settlement is refused with `TerminalPairError::MissingSettlement`,
    and each other kind given a settlement is refused with `TerminalPairError::UnexpectedSettlement`;
    neither returns a terminal value.
- id: FR-030-AC-18
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: examined
  excerpt: Every Kani `Proved` row carries `ProofBasis::Checks` with the transcript
    count and `Certification::Certified`, including the zero-check row; zero checks
    remain category inconclusive/vacuous. No Kani non-`Proved` row carries basis or
    certification.
- id: FR-029-AC-1
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '`verified` with three SUCCESS checks maps to `Proved { success_checks:
    3 }`, and `falsified` with a reproduced replay maps to `Refuted`.'
- id: FR-029-AC-2
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '`inconclusive` with the vacuous-proof reason and `cover-unsatisfied` each
    map to `Proved { success_checks: 0 }`, whose QSL category is `inconclusive` with
    cause `KaniVacuousProof`.'
- id: FR-029-AC-3
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: The timed-out reason maps to `Incomplete(TimedOut)`, and the memory-exhausted
    and exhausted-unwind-bound reasons each map to `Incomplete(ResourceExhausted)`.
- id: FR-029-AC-15
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: For the ordinary source-predicate `ReplaySettlement` input, a falsified
    outcome given no replay settlement is refused with `TerminalPairError::MissingSettlement`,
    and each other outcome (`verified`, `cover-unsatisfied`, `cancelled` and every
    inconclusive reason) given a settlement is refused with `TerminalPairError::UnexpectedSettlement`;
    neither returns a terminal value. The distinct typed composite-parity settlement
    input is governed by AC-17 and AC-19 to AC-28, not this criterion.
- id: FR-029-AC-29
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: A direct `KaniRunOutcome::Cancelled { source }` maps both IR source variants
    to QSL `IncompleteCause::Cancelled` with the matching exact cause, using the same
    exhaustive conversion as FR-030; supplying a replay settlement returns `UnexpectedSettlement`.
    The production cancellation path publishes only FR-030's result.
- id: FR-015-AC-78
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: PLANNED (IR-624), IR-628 accessor merged; CG implementation pending. `field_range`
    returns `MemberAbsent` for a requested name absent from the selected model declaration's
    accessor, including a clause's own field; an accessor `UnknownNode`, `NotModelObjectType`
    or `AmbiguousField(name)` returns `ModelFieldsUnavailable` with that error and
    object id, with no body or read fallback. A non-model object and an admitted unselected
    model/object_type with a nonempty body both yield `NotModelObjectType` and refuse
    even if body members claim valid ranges. A selected model/object_type whose nonempty
    body
- id: FR-015-AC-81
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: PLANNED (IR-624), IR-628 accessor merged; CG dependency update and implementation
    pending. A present listed model field with `member_type()` of `None`, a non-`IntRange`
    variant, or `IntRange` outside `i64` is recorded as unranged with `TypeNotRange`
    in `StateFrameIdentity` and its persisted record, even when no read names it;
    a present unread `IntRange` within `i64` is recorded in `domains` and is absent
    from the unranged list. Equal inputs produce byte-identical records; a record
    naming a field twice is invalid. The model declaration path records no `NoRead`
    reason. QSL-emission gaps for `Non
- id: FR-018-AC-5
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: A descriptor with a `convert<T>` operand outside `admits_equality_conversion`,
    distinct text profiles, distinct enum declarations, incompatible dimensions, distinct
    units, or no common type is refused at generation time with its `IllTypedCause`,
    emits no code, and admits no charge on any `Meter`.
- id: FR-018-AC-10
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: Generated bytes are identical across repeated runs and across permutations
    of the request order, including a request carrying two descriptors over one node
    id, and claim-map entries are ordered by the descriptor key — expression node
    id, operator rank, then each operand's source-type and conversion-target node
    ids, every node id compared in node-id order; the generated crate is compiled
    and executed under AC-2 against a native run driven from the request's own descriptor
    (AC-3), so a generated crate with a changed emitted operator, operand order or
    descriptor fails AC-2.
- id: FR-018-AC-15
  path: spec/oracle/functional/FR-018-composite-equality-oracles.md
  role: examined
  excerpt: 'A `binary` node''s operand is read through its `reference`: a reference
    to a `convert` expression node is read as the type of what it converts, following
    nested conversions to the first node that is not a conversion, and a reference
    to any other node, including a non-`convert` application, as that node''s own
    `semantic_type`. A descriptor whose `source_type` is the read type generates,
    and any other source is refused as an operand-type disagreement at that position,
    reporting the read type as found.'
- id: FR-021-AC-11
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: 'Model and relation lowering records map to the quire-spec-language#120
    blocker; state, temporal nodes other than directly requested `formula` and `fairness`
    roots, and protocol lowering records map to quire-spec-language#121; the blockers
    remain distinct. A requested temporal clause keeps that blocker even if its first
    unsupported reachable node is a formula or fairness child. TC-031''s defensive
    Model/State classifier case pairs those request roots with call items bound to
    a different checked source function, so it does not establish applied-call behavior.
    The source-bound applied-call leg is '
- id: FR-021-AC-27
  path: spec/oracle/functional/FR-021-function-application-oracles.md
  role: examined
  excerpt: An admitted nested body consists of separate V2 expression nodes connected
    only by `reference` leaves under QSpec FR-322, with each expression node reached
    from one parent in this CG subset; callee argument 0 binds an exact V2 function
    node, parameter references bind the declared parameter slots, and remaining arguments
    preserve source order. Missing, cyclic, shared-expression, unbound-parameter,
    ill-typed or wrong-operation edges refuse with a typed cause before schedule emission.
    PLANNED (IR-511).
- id: FR-030-AC-19
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: context_only
  excerpt: For both `BoundedComplete { depth }` and `Inductive { depth }`, an SMT
    proof whose certificate FR-314 verifies maps to `Proved` with the same basis/depth
    and `Certified`; no certificate or an `Unverifiable` unchecked rule maps to `Proved`
    with the same basis/depth and `Uncertified`, retaining the unverifiable locus
    when present. No SMT proof maps to `Trusted`.
- id: FR-030-AC-20
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: context_only
  excerpt: For either SMT basis, a foreign query, wrong certificate shape, invalid
    checked proof step or missing final empty clause rejected by FR-314 maps to `Inconclusive(CertificateRejected
    { rule, at })` retaining the exact rejection; none produces `Proved`, basis or
    certification. When cvc5 and Z3 also disagree, `CertificateRejected` remains the
    one terminal cause while the parity failure and both verdicts are retained alongside
    it.
- id: FR-030-AC-21
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: context_only
  excerpt: SMT/cross-check disagreement without certificate rejection keeps both verdicts
    and settles inconclusive; agreement does not change certification; missing Z3
    retains its warning and cvc5 settlement. An SMT counterexample becomes `Refuted`
    only after FR-197 replay reproduces it; replay disagreement, non-fault refusal
    and fault map to their distinct non-proof results.
- id: FR-030-AC-22
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: context_only
  excerpt: Every expressible accepted SMT backend/checker/replay settlement produces
    one terminal result, never `Tested`; no non-`Proved` result carries basis or certification.
    Negotiation refusals produce no terminal from this map, and neither SMT nor Kani
    map consumes the other backend's outcome.
bindings:
- test_id: batch_tests::cancellation_before_spawn_has_no_backend_or_report
  ac_id: FR-040-AC-1
  trace: correct
- test_id: batch_tests::live_cancellation_preserves_first_event_through_ir_and_qsl
  ac_id: FR-040-AC-1
  trace: correct
- test_id: batch_tests::live_cancellation_preserves_first_event_through_ir_and_qsl
  ac_id: FR-040-AC-2
  trace: correct
- test_id: batch_tests::child_memory_overage_is_inconclusive_and_kills_the_entire_backend_tree
  ac_id: FR-040-AC-3
  trace: correct
- test_id: batch_tests::output_over_limit_is_not_relabelled_as_cancellation
  ac_id: FR-040-AC-3
  trace: correct
- test_id: batch_tests::completed_and_elapsed_runs_do_not_acquire_late_cancellation
  ac_id: FR-040-AC-3
  trace: correct
- test_id: batch_tests::child_memory_overage_is_inconclusive_and_kills_the_entire_backend_tree
  ac_id: FR-040-AC-4
  trace: correct
- test_id: batch_tests::output_over_limit_is_not_relabelled_as_cancellation
  ac_id: FR-040-AC-4
  trace: correct
- test_id: batch_tests::memory_observation_failure_competing_with_cancellation_refuses_public_evidence
  ac_id: FR-040-AC-4
  trace: correct
- test_id: batch_tests::unread_capture_competing_with_cancellation_refuses_public_evidence
  ac_id: FR-040-AC-4
  trace: correct
- test_id: batch_tests::cleanup_nonconfirmation_competing_with_cancellation_refuses_public_evidence
  ac_id: FR-040-AC-4
  trace: correct
- test_id: batch_tests::completed_and_elapsed_runs_do_not_acquire_late_cancellation
  ac_id: FR-040-AC-4
  trace: correct
- test_id: batch_tests::cancelled_batch_retains_first_source_and_independent_group
  ac_id: FR-040-AC-5
  trace: correct
- test_id: batch_tests::distinct_cancellation_handles_have_distinct_launchers
  ac_id: FR-040-AC-5
  trace: correct
- test_id: batch_tests::cancellation_before_spawn_has_no_backend_or_report
  ac_id: FR-040-AC-6
  trace: correct
- test_id: batch_tests::live_cancellation_preserves_first_event_through_ir_and_qsl
  ac_id: FR-040-AC-6
  trace: correct
- test_id: batch_tests::cancellation_before_spawn_has_no_backend_or_report
  ac_id: FR-040-AC-7
  trace: correct
- test_id: direct_cancelled_run_maps_exact_source_and_rejects_settlement
  ac_id: FR-040-AC-7
  trace: correct
- test_id: tc_041_every_expressible_pair_maps_to_one_value
  ac_id: FR-030-AC-1
  trace: correct
- test_id: tc_041_a_refusal_before_the_run_is_declined_with_its_kind_and_ir_code
  ac_id: FR-030-AC-2
  trace: correct
- test_id: tc_041_a_limit_or_a_cancellation_is_incomplete_with_its_cause
  ac_id: FR-030-AC-3
  trace: correct
- test_id: tc_041_a_proof_carries_its_check_count_and_a_reproduced_counterexample_refutes
  ac_id: FR-030-AC-4
  trace: correct
- test_id: tc_041_a_vacuous_proof_is_proved_with_zero_checks_and_other_inconclusive_is_failed
  ac_id: FR-030-AC-5
  trace: correct
- test_id: tc_041_no_outcome_maps_to_tested
  ac_id: FR-030-AC-6
  trace: correct
- test_id: tc_041_the_map_is_one_match_over_the_pair_with_no_wildcard_arm
  ac_id: FR-030-AC-7
  trace: correct
- test_id: tc_041_an_absent_solver_or_backend_is_unsupported_with_its_cause
  ac_id: FR-030-AC-8
  trace: correct
- test_id: tc_041_a_counterexample_disagreement_or_refusal_is_inconclusive_with_its_cause
  ac_id: FR-030-AC-9
  trace: correct
- test_id: tc_041_a_counterexample_with_a_fault_or_a_cg_defect_is_failed
  ac_id: FR-030-AC-10
  trace: correct
- test_id: tc_041_only_a_reproduced_replay_refutes_a_counterexample
  ac_id: FR-030-AC-11
  trace: correct
- test_id: tc_041_a_setup_refusal_after_a_counterexample_is_replay_refused_never_declined
  ac_id: FR-030-AC-12
  trace: correct
- test_id: tc_041_a_repeated_dependency_identity_after_a_counterexample_is_replay_refused
  ac_id: FR-030-AC-13
  trace: correct
- test_id: tc_041_a_settlement_accompanies_a_counterexample_only
  ac_id: FR-030-AC-14
  trace: correct
- test_id: tc_040_verified_proves_its_checks_and_a_reproduced_falsification_refutes
  ac_id: FR-029-AC-1
  trace: correct
- test_id: tc_040_a_vacuous_proof_and_an_unsatisfied_cover_are_proved_with_zero_checks
  ac_id: FR-029-AC-2
  trace: correct
- test_id: observed_memory_exhaustion_maps_to_incomplete_resource_exhausted
  ac_id: FR-029-AC-3
  trace: correct
- test_id: direct_cancelled_run_maps_exact_source_and_rejects_settlement
  ac_id: FR-029-AC-15
  trace: correct
- test_id: tc_040_a_settlement_accompanies_a_falsified_outcome_only
  ac_id: FR-029-AC-15
  trace: correct
- test_id: direct_cancelled_run_maps_exact_source_and_rejects_settlement
  ac_id: FR-029-AC-29
  trace: correct
- test_id: tc_025_an_unselected_model_body_never_supplies_a_range
  ac_id: FR-015-AC-78
  trace: correct
- test_id: tc_025_model_member_types_without_i64_ranges_remain_unranged
  ac_id: FR-015-AC-78
  trace: correct
- test_id: selected_model_variant_field_is_unranged_in_generated_frame_identity
  ac_id: FR-015-AC-78
  trace: correct
- test_id: tc_025_unranged_fields_have_a_reason_and_no_duplicate_record_names
  ac_id: FR-015-AC-81
  trace: correct
- test_id: tc_025_model_member_types_without_i64_ranges_remain_unranged
  ac_id: FR-015-AC-81
  trace: correct
- test_id: selected_model_variant_field_is_unranged_in_generated_frame_identity
  ac_id: FR-015-AC-81
  trace: correct
- test_id: tc_035_a_field_with_no_declared_range_is_decoded_and_not_checked
  ac_id: FR-015-AC-81
  trace: correct
- test_id: tc_025_wide_model_override_preserves_unranged_context_without_a_replay_claim
  ac_id: FR-015-AC-81
  trace: correct
```

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1 at ab8506e: both existing certified Checks/vacuity tests now bind FR-030-AC-18; computed matrix reports tagged with those two real binders, and both exact tests passed. |
| FND-002 | fixed | Round 1 at ab8506e: real RSS overage is positively gated; Deadline is recorded and asserted before the measured sample returns. Public execution retains MemoryExhausted and confirms child/sibling teardown. A post-sample, pre-overage cancellation-priority mutant failed with Cancelled Deadline; freshly rebuilt restored control passed. This proves post-sample reprioritization detection, not pre-sampler detection. |

## Round 1 Verdict

**PASS** — Both original medium findings are fixed on published PR #383 head `ab8506e113db958a21c552d6831a5d070c7931ff`; no new finding was identified in this fix-only disposition scope. The original review verdict and findings above remain historical and unchanged. Final full native/proof gates on the final artifact-bearing head remain pending; this is no merge approval.

## Round 1 Evidence

Independently inspected the two fix source paths, private mutant diff, actual build/list/test logs and author receipts `/tmp/ix-handoff/ir701-author-review-focused-ab8506e-receipt.json` and `/tmp/ix-handoff/ir701-author-priority-mutation-ab8506e-receipt.json`. Focused Clippy, fresh lib/IT builds and binary listings passed; four exact controls each selected one test, passed with zero ignored. The computed matrix binds FR-030-AC-18 to the two changed tagged tests. The private post-sample mutant built successfully, selected the real new control and failed with exit 101 at its outcome assertion (Cancelled Deadline versus MemoryExhausted); fresh restored build selected the same control and passed with zero ignored. Launch source bytes at the restored fix head equal the originally reviewed launcher. Receipt guards attribute clean exact heads, unchanged lock and owned process completion to the author-run harness. Reviewer ran no Cargo/build/Kani or gate-lock operations. Source/index remain clean at ab8506e.
