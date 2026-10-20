---
id: SR-14080
title: IR-808 residual PR472 code-review
type: SpecReview
analysis: code-review
scope: agent-ix/quire-contract-codegen; IR-808 residual PR472; src/kani/run/execute.rs
  and owner/architecture context
review_set: subset
---

## Summary

Ticket IR-808, residual PR472. One independent reviewer applied the reviewer skill, code-review with its Rust lane, and separate whole-repository planless gap-analysis. Own diff is solely execute.rs cfg(test), +309/-34; no CI, policy, dependency, production, spec or plan change. Spec review does not apply to this own diff. No applicable AssuranceProfile exists under spec/; ordinary repository/AD-004 rules apply.

Both original tests retain both caller orders, first cause, exact a/b membership, no cancelled member evidence, independent c Verified, two launch calls and exact a/b selections; single execution retains no exit/check/verdict, actual memory and settled IR/QSL mapping plus late immutability. ControlledMemory calls the actual observer first, gates only after readiness, and returns its measured sample unchanged. The real cleanup poll returns before the gate; successful observed>0/IN assertions run only after release. The ordinary runtime priority and owner ceilings remain unchanged. The late arm's sleep45 fits the shared a/b outer60 budget and accepts actual completion without a report, yielding Report(Missing), while c retains30 and Verified. Its early arm yields Requested BatchCancelled(2). No synthetic poll/sample/outcome or mapper was introduced.

OwnerReleases aborts both real handles and releases each one-shot gate before join; catch_unwind protects scheduling assertions and Drop is a fallback. Main-path panic and worker disconnect diagnostics retain received Debug values. One timeout-edge diagnostic loss is FND-001. worker.join itself has no separate kernel-level bounded guarantee; its practical settlement relies on the real runtime owners, their finite waits and OS behavior. The21/51-second observation allowance accounts conservatively for first reap and independent startup/cleanup/reap plus work/drains; it changes no production ceiling. A real elapsed cancellation deadline still retains existing precedence. Ordinary memory/refusal/capture/late-completion/timeout controls are preserved.

Rust checks covered duplication/helper reuse, test seams and behavioral assertions, lint/unsafe/panic and conversion surfaces, synchronization and ownership, stop/release/drain/join, real namespace/PID-aware gone checks, resource bounds, and code/criterion alignment. All added dependencies are through existing run owners/std; allocator remains the single std-only core helper. New cfg(test) imports obey AD-004 L-2; library-included private tests retain their current core allocator imports. No gate/scanner/policy bypass, copied utility or new runtime API exists. Gates already executed on identical source are retained rather than repeated for freshness; root final aggregate/full Kani remain pending.

## Verdict

**CONDITIONAL** — FND-001 requires a same-PR fix before review approval.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | A result sent during timeout abort/join is discarded instead of preserved in the failure diagnostic. | src/kani/run/execute.rs:2306, src/kani/run/execute.rs:2982 |

Concrete failure scenario and requested correction are recorded with FND-001 in the machine data below. This source-proven edge requires no heavy execution to establish.

## Coverage

Plan completion: not assessed

Author evidence remains attributed to clean bad7e43bbe100f3c5f2238c675d5647e37c5096c and QSPEC fd7a16bce02dacc1229f741a9f23a15375c68a6c. Read coder-report.md, raw candidate-02 whole-suite outcome lines, the three red-control logs and root independent verification JSON. Exactly three full make-test runs each passed 376 library/4 ignored, 645 integration/37 ignored and 16 docs; both ticket-named tests passed each. Serial libtest with four bounded workers sharing CPUs0-1 is external load evidence, not concurrent whole-suite libtest execution. All workers were reaped; maximum measured VmHWM8660KiB. Each one-shot wrong-source, broken-sharing and cancelled-independent control exited101 at its intended original assertion and was restored. These are native test-quality controls, not Kani mutation proofs. Actual full author make kani-gate passed37/37 using Kani0.68.0, scope/command0, 813s gate/759.49s body. No author aggregate. Source/build/test-input identity across the later unrelated doc-only upstream rebase is verified by root, with execute bytes also independently compared here to the red-control pristine snapshot. No passing author run is relabeled as5039. Historical natural flake remains UNREPRODUCED; injected reachability is not historical reproduction or actual-Kani proof. Final integrated make ci/full actual Kani belong to root after fixes/latest custody.

No heavy reservation acquired or heavy command executed by this reviewer. Exact final guard: pgrep -x 'cargo|rustc|cbmc|kani-driver|cargo-kani|kani' returned no processes, exit1. Both build directories retained. Actual model provenance: parent native rollout latest turn_context model gpt-6.1-sol, no spawn override; fresh invocation UUID below.

## Examined Scope and Bindings

All scope excerpts are nonempty verbatim recorded source substrings of at most600 characters. Criteria are structurally examined; contexts are explicitly marked; recorded semantic bindings are partial evidence only.

```yaml
repo: agent-ix/quire-contract-codegen
ticket: IR-808
pr: quire-contract-codegen#472
checked_head: 5039d518b6dd460b776a4f2f15acfc8f5c213fd7
assigned_base: 7cb4efa01405303e0e26d0ede76f0fecf7702504
model: gpt-6.1-sol
run: 18e0fc3b-126f-44a4-a2bc-8450c0173fa2
artifact_id: SR-14080
method: code-review
clean: false
scope:
- id: FR-040-AC-1
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: A caller retains a live `Cancel`, sends its clone through a public single-run request, and the bounded
    launcher observes it before spawn and during an actual running backend. Pre-spawn cancellation starts no backend;
    running cancellation confirms owned teardown before evidence returns.
  static_status: tagged
  binder_count: 2
  assessment: computed structural reconciliation; no complete semantic discharge
- id: FR-040-AC-2
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: In two live single runs, issue `Requested` then `Deadline`, and reverse the calls. Each run's actual
    launcher stop, `KaniExecutionEvidence.outcome`, constructed IR `Cancelled { source }` and QSL `IncompleteCause::Cancelled
    { source }` retain the first cause; a later event cannot change them. A hand-built IR outcome alone does not
    satisfy this criterion.
  static_status: tagged
  binder_count: 1
  assessment: computed structural reconciliation; no complete semantic discharge
- id: FR-040-AC-3
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: A run that reaches its execution ceiling with no cancellation event remains `TimedOut`. A `Deadline`
    event recorded only after completion, memory/capture stop or elapsed-ceiling stop does not turn that result
    into `Cancelled`.
  static_status: tagged
  binder_count: 4
  assessment: computed structural reconciliation; no complete semantic discharge
- id: FR-040-AC-4
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: Completion before observed cancellation keeps the report-based result; measured memory exhaustion, memory
    observation failure, output over-limit, unread capture and unconfirmed cleanup keep their existing result/refusal
    and confirmed teardown rules when cancellation competes. A cancellation observed first and causing stop is not
    overwritten by later completion or elapsed time.
  static_status: tagged
  binder_count: 7
  assessment: computed structural reconciliation; no complete semantic discharge
- id: FR-040-AC-5
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: A multi-member group sharing one handle stops wholly on its observed cause, returns `BatchCancelled {
    source, members }`, and classifies no member; another group using a distinct handle retains its own result.
    Reversing the two cause calls changes the cancelled group's source and not the other group's evidence. Distinct
    handles never share one launcher.
  static_status: tagged
  binder_count: 3
  assessment: computed structural reconciliation; no complete semantic discharge
- id: FR-040-AC-6
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: The single-run producer constructs an IR `KaniOutcomeKind::Cancelled { source }` from settled execution
    evidence, and its real call path reaches the FR-030 map. Mapping both IR source variants exhaustively yields
    the identical `quire_exact::CancelCause` inside QSL's `IncompleteCause::Cancelled`; neither metadata nor a copied
    enum determines the source.
  static_status: tagged
  binder_count: 2
  assessment: computed structural reconciliation; no complete semantic discharge
- id: FR-040-AC-7
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: The direct `quire-exact::CancelCause` constructs `qsl-replay::IncompleteCause::Cancelled` through a type-checked
    exhaustive conversion, while all QSL imports remain through `qsl-replay`; no implicit cancellation control,
    source fallback, or compatibility overload is present.
  static_status: tagged
  binder_count: 2
  assessment: computed structural reconciliation; no complete semantic discharge
- id: FR-017
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: context_only
  excerpt: 'When a caller runs an FR-015, FR-022/FR-014 or FR-035 harness, the code generator shall invoke the

    installed Kani backend and retain the backend''s own reported outcome as typed

    execution evidence. A caller may hand the generator several harnesses to run together

    (FR-017-AC-21 to FR-017-AC-25); the generator then forms groups, runs each

    group in one launcher process, and retains one evidence record per harness. Unless a

    statement says "batch" or "group", it describes a run of one harness.


    [FR-040](./FR-040-cancellation-producer.md) adds the one launcher-observed cancellation outcome:

    the backe'
- id: FR-028
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: context_only
  excerpt: 'The code generator shall bound every Kani harness so that its run can finish: each harness identity

    records a memory ceiling and a wall-clock ceiling, a run that exceeds either settles `inconclusive`

    with its own reason, and the evidence records the bounds and ceilings the run used. Proof coverage

    is maximised per family. A family proves the production code where its harness verifies within the

    ceilings, and a bounded shadow together with a refinement obligation where it does not

    ([ADR-003](../../decisions/ADR-003-kani-tractability.md)).


    Planned (IR-241): ADR-003 decides that a shadow exists '
- id: FR-030
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: context_only
  excerpt: 'The generator shall map every Contract IR `KaniOutcome`, paired with the settlement of its replay, to

    exactly one `qsl_replay::TerminalValue`. The map is total over the pair (outcome, replay

    settlement), not over the outcome alone, and it preserves the outcome''s refusal cause: the outcome

    kinds that QSL''s terminal value collapses into one variant stay distinguishable through that

    variant''s typed cause. This is QSL ADR-013 C-09 as merged, whose two inputs are the IR `KaniOutcome`

    and, for a `Counterexample`, the result of its replay. The replay settlement, its six readings and

    the rules that cl'
- id: FR-040
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: context_only
  excerpt: 'When a caller requests cancellation of a bounded Kani execution, the generator SHALL observe the

    caller-owned `quire_exact::Cancel` handle at the real launcher and retain the cause of the first

    cancellation event that stops that launcher. The caller owns the handle and any timer: a timer can

    call `cancel(CancelCause::Deadline)`, while an elapsed [FR-028](./FR-028-bounded-proof-ceilings.md)

    execution ceiling without a recorded cancellation event remains `TimedOut`. This requirement adds

    one producer path to [FR-017](./FR-017-kani-execution-evidence.md); it does not treat a Contract IR

    outcome r'
- id: src/kani/run/execute.rs
  path: src/kani/run/execute.rs
  role: examined
  excerpt: //! Execution of generated Kani obligation harnesses, singly or as a batch (FR-017).
- id: src/kani/run/launch.rs
  path: src/kani/run/launch.rs
  role: examined
  excerpt: '//! Launching the Kani launcher process: spawn, bounded capture, the wall-clock budget and the'
- id: src/kani/run/owners.rs
  path: src/kani/run/owners.rs
  role: examined
  excerpt: //! Immutable dependencies of one bounded execution's existing observation and teardown owners.
- id: src/kani/run/namespace.rs
  path: src/kani/run/namespace.rs
  role: examined
  excerpt: '//! Per-run PID-namespace ownership: gated startup, claimed init, and confirmed teardown.'
- id: src/core/test_support.rs
  path: src/core/test_support.rs
  role: examined
  excerpt: //! Scratch directories shared by unit tests across subsystems.
- id: src/core/mod.rs
  path: src/core/mod.rs
  role: examined
  excerpt: '//! The leaf of the crate: the shared primitives every other directory builds on (AD-004).'
- id: src/lib.rs
  path: src/lib.rs
  role: examined
  excerpt: //! Deterministic Rust, property-test, proof, and evidence generation from Quire contracts.
- id: tests/it/layout.rs
  path: tests/it/layout.rs
  role: examined
  excerpt: //! AD-004 layout decisions L-1 and L-2 and FR-024-AC-10, checked over `src/`.
- id: spec/spec.md
  path: spec/spec.md
  role: context_only
  excerpt: 'This specification defines deterministic lowering from a validated contract package into Rust

    oracles, tri-state test harnesses, shaped proptest strategies, Kani obligations and coverage maps. Generated
    artifacts remain traceable to one authoritative source contract.'
- id: spec/assurance/AD-004-cg-crate-layout.md
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: context_only
  excerpt: 'In scope: every file under `src/`, the import edges between them, the public entry points that

    `lib.rs` re-exports, and the helper, identity, node-access and version-profile code the modules

    share. Also in scope are the two layout exceptions recorded in `spec/spec.md` (FR-005 and the

    SuiteRegistry file), which the Subsystem Registry defers to this AD and which this AD settles.


    Out of scope: behaviour. Every requirement keeps its id and its criteria. No requirement id is

    minted here; the labels `L-1` and onward are local to this document, as the labels in the other

    ADs are. Two places record t'
- id: spec/kani/matrix/TC-055-cancellation-producer.md
  path: spec/kani/matrix/TC-055-cancellation-producer.md
  role: context_only
  excerpt: 'Plan a discriminating production-path check for [FR-040](../functional/FR-040-cancellation-producer.md).

    The event reader must supply the observed source; a prebuilt IR outcome tests only the terminal

    map. The existing [TC-041](./TC-041-ir-outcome-terminal-map.md) continues to own the separate

    [FR-030](../functional/FR-030-ir-outcome-terminal-map.md) AC-7 source inspection.'
- id: spec/kani/matrix/TC-043-kani-batching-and-output-bounds.md
  path: spec/kani/matrix/TC-043-kani-batching-and-output-bounds.md
  role: context_only
  excerpt: 'Verify that several harnesses run in one launcher process with each member decided from its own

    report entry and its own console playback, that an output stream over its limit is refused and never

    truncated, that a capture failure is refused, and that no launcher descendant outlives a run

    (IR-277; FR-017-AC-14 and FR-017-AC-21 to FR-017-AC-25), and that the diagnostic record of a

    `NoVerdict` group never feeds classification (FR-017-AC-36), and that a batch member''s CBMC failure

    cause is kept beside the batch timeout rule (FR-017-AC-39). The batch timeout rule it exercises is

    FR-028-AC-12, veri'
- id: spec/core/functional/interface-001-codegen-api.md
  path: spec/core/functional/interface-001-codegen-api.md
  role: context_only
  excerpt: 'The interface''s features in declaration order: every operation the contract above declares, one row
    each.


    IR-598 / FR-015-AC-109 retired the finite-input checked-arithmetic auxiliary and

    its request/result types. The collection and graph auxiliaries and corpus API

    remain public until their separately gated retirement slices.


    | Feature | Kind |

    |---|---|

    | source_obligation_census | operation |

    | generate_bound_oracles | operation |

    | generate_bundle | operation |

    | generate_tristate_harness | operation |

    | generate_i64_strategy | operation |

    | generate_enum_strategy | operation |

    | generate_'
- id: AGENTS.md
  path: AGENTS.md
  role: context_only
  excerpt: See CLAUDE.md.
- id: CLAUDE.md
  path: CLAUDE.md
  role: context_only
  excerpt: '# quire-contract-codegen'
- id: src/kani/run/execute.rs::controlled_owners
  path: src/kani/run/execute.rs
  role: examined
  excerpt: fn controlled_owners(&mut self) -> (OwnerControl, OwnerControl) {
- id: src/kani/run/execute.rs::cancel_running
  path: src/kani/run/execute.rs
  role: examined
  excerpt: 'fn cancel_running<R: Send + std::fmt::Debug>('
- id: src/kani/run/execute.rs::cancellation_settlement_budget
  path: src/kani/run/execute.rs
  role: examined
  excerpt: 'fn cancellation_settlement_budget(remaining_budget: Duration) -> Duration {'
- id: src/kani/run/execute.rs::ControlledMemory
  path: src/kani/run/execute.rs
  role: examined
  excerpt: struct ControlledMemory {
- id: src/kani/run/execute.rs::OwnerReleases
  path: src/kani/run/execute.rs
  role: examined
  excerpt: struct OwnerReleases<'a> {
- id: src/kani/run/execute.rs::controlled_batch_cancellation_arm
  path: src/kani/run/execute.rs
  role: examined
  excerpt: 'fn controlled_batch_cancellation_arm(withhold: bool) {'
- id: src/kani/run/execute.rs::OwnerGate
  path: src/kani/run/execute.rs
  role: examined
  excerpt: struct OwnerGate {
- id: src/kani/run/execute.rs::OwnerControl
  path: src/kani/run/execute.rs
  role: examined
  excerpt: struct OwnerControl {
- id: src/kani/run/execute.rs::ControlledPoll
  path: src/kani/run/execute.rs
  role: examined
  excerpt: struct ControlledPoll {
- id: src/kani/run/execute.rs::recorded_process_gone
  path: src/kani/run/execute.rs
  role: examined
  excerpt: 'fn recorded_process_gone(path: &Path) {'
findings:
- fnd: FND-001
  severity: medium
  confidence: high
  method: code-review
  check: code-bug
  artifact_id: src/kani/run/execute.rs::cancel_running
  related_ids:
  - src/kani/run/execute.rs::controlled_batch_cancellation_arm
  path: src/kani/run/execute.rs
  lines: 2306-2317; 2982-2991
  excerpt: let received = receiver.recv_timeout(cancellation_settlement_budget(remaining_budget));
  finding: A result sent during timeout abort/join is discarded instead of preserved in the failure diagnostic.
  scenario: The first recv_timeout returns Timeout; abort requests cancellation/releases gates; the worker sends
    its real public Result before join completes. No post-join receive reads that queued Result, so Debug prints
    Err(Timeout), and dropping the receiver loses the actual classification/refusal. The same path exists in the
    diagnostic arm.
  requested_fix: Preserve the initial timeout as a failure, join only after releasing/aborting owners, then nonblockingly
    drain any public result sent during join and include that actual result in Debug diagnostics in both call sites.
    Keep original assertions and budgets; never turn a timeout into success.
bindings:
- test_id: batch_tests::cancellation_before_spawn_has_no_backend_or_report
  ac_id: FR-040-AC-1
  trace: correct
  path: src/kani/run/execute.rs
  line: 2239
  assessment: direct assertion/real-execution support for the exercised subset only
  complete_criterion_discharge: false
- test_id: batch_tests::live_cancellation_preserves_first_event_through_ir_and_qsl
  ac_id: FR-040-AC-1
  trace: correct
  path: src/kani/run/execute.rs
  line: 2802
  assessment: direct assertion/real-execution support for the exercised subset only
  complete_criterion_discharge: false
- test_id: batch_tests::live_cancellation_preserves_first_event_through_ir_and_qsl
  ac_id: FR-040-AC-2
  trace: correct
  path: src/kani/run/execute.rs
  line: 2802
  assessment: direct assertion/real-execution support for the exercised subset only
  complete_criterion_discharge: false
- test_id: batch_tests::child_memory_overage_is_inconclusive_and_kills_the_entire_backend_tree
  ac_id: FR-040-AC-3
  trace: correct
  path: src/kani/run/execute.rs
  line: 1686
  assessment: direct assertion/real-execution support for the exercised subset only
  complete_criterion_discharge: false
- test_id: batch_tests::output_over_limit_is_not_relabelled_as_cancellation
  ac_id: FR-040-AC-3
  trace: correct
  path: src/kani/run/execute.rs
  line: 2179
  assessment: direct assertion/real-execution support for the exercised subset only
  complete_criterion_discharge: false
- test_id: batch_tests::controlled_caller_order_distinguishes_cancelled_and_completed_batch
  ac_id: FR-040-AC-3
  trace: correct
  path: src/kani/run/execute.rs
  line: 2913
  assessment: direct assertion/real-execution support for the exercised subset only
  complete_criterion_discharge: false
- test_id: batch_tests::completed_and_elapsed_runs_do_not_acquire_late_cancellation
  ac_id: FR-040-AC-3
  trace: correct
  path: src/kani/run/execute.rs
  line: 3074
  assessment: direct assertion/real-execution support for the exercised subset only
  complete_criterion_discharge: false
- test_id: batch_tests::child_memory_overage_is_inconclusive_and_kills_the_entire_backend_tree
  ac_id: FR-040-AC-4
  trace: correct
  path: src/kani/run/execute.rs
  line: 1686
  assessment: direct assertion/real-execution support for the exercised subset only
  complete_criterion_discharge: false
- test_id: batch_tests::measured_memory_overage_competing_with_cancellation_keeps_resource_outcome
  ac_id: FR-040-AC-4
  trace: correct
  path: src/kani/run/execute.rs
  line: 1738
  assessment: direct assertion/real-execution support for the exercised subset only
  complete_criterion_discharge: false
- test_id: batch_tests::output_over_limit_is_not_relabelled_as_cancellation
  ac_id: FR-040-AC-4
  trace: correct
  path: src/kani/run/execute.rs
  line: 2179
  assessment: direct assertion/real-execution support for the exercised subset only
  complete_criterion_discharge: false
- test_id: batch_tests::memory_observation_failure_competing_with_cancellation_refuses_public_evidence
  ac_id: FR-040-AC-4
  trace: correct
  path: src/kani/run/execute.rs
  line: 2597
  assessment: direct assertion/real-execution support for the exercised subset only
  complete_criterion_discharge: false
- test_id: batch_tests::unread_capture_competing_with_cancellation_refuses_public_evidence
  ac_id: FR-040-AC-4
  trace: correct
  path: src/kani/run/execute.rs
  line: 2671
  assessment: direct assertion/real-execution support for the exercised subset only
  complete_criterion_discharge: false
- test_id: batch_tests::cleanup_nonconfirmation_competing_with_cancellation_refuses_public_evidence
  ac_id: FR-040-AC-4
  trace: correct
  path: src/kani/run/execute.rs
  line: 2732
  assessment: direct assertion/real-execution support for the exercised subset only
  complete_criterion_discharge: false
- test_id: batch_tests::completed_and_elapsed_runs_do_not_acquire_late_cancellation
  ac_id: FR-040-AC-4
  trace: correct
  path: src/kani/run/execute.rs
  line: 3074
  assessment: direct assertion/real-execution support for the exercised subset only
  complete_criterion_discharge: false
- test_id: batch_tests::cancelled_batch_retains_first_source_and_independent_group
  ac_id: FR-040-AC-5
  trace: correct
  path: src/kani/run/execute.rs
  line: 2852
  assessment: direct assertion/real-execution support for the exercised subset only
  complete_criterion_discharge: false
- test_id: batch_tests::controlled_caller_order_distinguishes_cancelled_and_completed_batch
  ac_id: FR-040-AC-5
  trace: correct
  path: src/kani/run/execute.rs
  line: 2913
  assessment: direct assertion/real-execution support for the exercised subset only
  complete_criterion_discharge: false
- test_id: batch_tests::distinct_cancellation_handles_have_distinct_launchers
  ac_id: FR-040-AC-5
  trace: correct
  path: src/kani/run/execute.rs
  line: 3044
  assessment: direct assertion/real-execution support for the exercised subset only
  complete_criterion_discharge: false
- test_id: batch_tests::cancellation_before_spawn_has_no_backend_or_report
  ac_id: FR-040-AC-6
  trace: correct
  path: src/kani/run/execute.rs
  line: 2239
  assessment: direct assertion/real-execution support for the exercised subset only
  complete_criterion_discharge: false
- test_id: batch_tests::live_cancellation_preserves_first_event_through_ir_and_qsl
  ac_id: FR-040-AC-6
  trace: correct
  path: src/kani/run/execute.rs
  line: 2802
  assessment: direct assertion/real-execution support for the exercised subset only
  complete_criterion_discharge: false
- test_id: batch_tests::cancellation_before_spawn_has_no_backend_or_report
  ac_id: FR-040-AC-7
  trace: correct
  path: src/kani/run/execute.rs
  line: 2239
  assessment: direct assertion/real-execution support for the exercised subset only
  complete_criterion_discharge: false
```

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Shared settlement recovers/logs late result after join while preserving the original timeout failure; both callers and causal regressions checked. |

## Disposition Round 1 Verdict

**PASS (current code/Rust review)** — FND-001 fixed; no new finding. Latest artifact custody and root final integrated gates remain required.

Disposition round1 independently checked frozen pushed5ebf572ce523a32d7880c889c088739b381314d6 against reviewed5039 and assigned base7cb4efa. The exact initial SR copies and external original artifact prefixes were first compared byte for byte with root's initial-root-verified snapshots. Original Findings, Verdict, full scope and binding records above remain unchanged. Source-only fix is execute.rs cfg(test), +120/-12; the other two files are exact initial review custody. No source/branch/PR code was edited or committed by the reviewer.

FND-001 is fixed. Both callers use one shared CancellationSettlement/settle_cancellation_worker. It retains the original recv_timeout result, invokes real abort before join only for receive failure, joins, and nonblockingly try_recv drains any late payload after join. Both callers print the actual initial receive, late payload or explicit unavailability, and worker state before their existing failures propagate. The original received.is_ok() still controls settled, and received.expect still rejects the initial timeout even when late is Some(Ok). Scheduling panic, initial timeout/disconnection, absent late payload and worker panic remain distinct; no Clone bound or invented backend result was added. Existing runtime owner guarantees still govern join; this change does not establish a separate kernel-level bounded join.

Both original caller-order/cancellation tests, their real-memory-before-caller-cause-before-release handshakes, cleanup-poll-return acknowledgement and post-release IN assertions, first-cause/group/independent-c/launcher/IR/QSL/late-immutability assertions, namespace/PID-aware teardown, original finite45/60/30 and21/51 observation budgets are intact. No production/dependency/policy/CI/allocator/AD-004 direction or scanner change. Adjacent healthy/refusal/resource/capture/completion controls remain unmodified. Canonical traces stay only on existing actual wrappers.

The two new std-channel tests use a worker blocked on an abort channel while the actual initial zero-duration receive times out. Only the helper's subsequent abort signal permits a payload send or deliberate worker panic, before join returns. The first checks the real late String in Debug and catches the original timeout expect failure; the second checks Disconnected and the actual joined panic independently. This is meaningful deterministic custody regression coverage for the shared helper used by both callers; it is not a real-backend cancellation/prover test and carries no new trace attribute or semantic criterion credit. No red-before run is claimed.

Read the actual coder fix-round1 report and author/fix-round-1 raw results/regression/affected/scope/release receipts. Seven one-shot confined stages at clean5ebf/QSPECfd7a all exited0: fmt-check, all-target lint, library and integration compilation, layout8, new regressions2, affected ordinary batch controls50 in63.46s (new regressions explicitly skipped because separately run). Raw regression output includes the intended caught timeout panic, actual late Debug payload and separate joined-worker panic; overall regression exit0 is not a missing assertion. Scope MemoryMax17179869184, MemorySwapMax0, own target; no external load. Root independently verified these receipts. No author/full-suite/aggregate/Kani rerun was requested or executed by the reviewer. Original N3/three red controls/full author37PASS remain bad7 evidence; they are not relabeled as the fix head.

Fresh full matrix/coverage at5ebf has identical criterion IDs, statements, methods, statuses and normalized binder memberships to reviewed5039:40 documents/866 criteria, tagged507, untagged291, ignored-only18, method-without-symbol50. Compared with assigned base7cb, only the already-reviewed two wrapper bindings on FR040AC3/5 are added. Current coverage still has919 reference rows/576 backed,230 unbacked,50 no-symbol,0 status lies,14 undeclared statuses; untracked5, unmatched123, diagnostics10 unchanged. Textual evidence-symbol candidates rise1035 to1037 for the two deliberately untraced infrastructure tests; tagged1014/bound1001 unchanged. Structural tags do not gain semantic credit. No evidence-store run was added or asserted. Inherited owned whole-repository debt remains FAIL under IR878/IR726/IR865; no duplicate NEW finding. Plan completion: not assessed.

No new finding. Original36 code scope and1066 gap scope remain preserved; latest full records below add exactly four newly inspected implementation/test units, yielding40/1070. Each method retains all20 original semantic binding identities; current locations are recorded below without claiming broader discharge. All excerpts are nonempty meaningful verbatim recorded source <=600 chars. Original private review comments remain unchanged.

No heavy reservation acquired or heavy command executed in this disposition. Exact process guard recorded in round1/reviewer-resource-release.json; both targets retained.

## Disposition Round 1 Scope and Bindings

```yaml
repo: agent-ix/quire-contract-codegen
ticket: IR-808
pr: quire-contract-codegen#472
checked_head: 5ebf572ce523a32d7880c889c088739b381314d6
assigned_base: 7cb4efa01405303e0e26d0ede76f0fecf7702504
model: gpt-6.1-sol
run: 45b8725e-7660-4b43-bc2a-e2020ec1572d
artifact_id: SR-14080
method: code-review
clean: true
scope:
- id: FR-040-AC-1
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: A caller retains a live `Cancel`, sends its clone through a public single-run request, and the bounded
    launcher observes it before spawn and during an actual running backend. Pre-spawn cancellation starts no backend;
    running cancellation confirms owned teardown before evidence returns.
  static_status: tagged
  binder_count: 2
  assessment: computed structural reconciliation; no complete semantic discharge
- id: FR-040-AC-2
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: In two live single runs, issue `Requested` then `Deadline`, and reverse the calls. Each run's actual
    launcher stop, `KaniExecutionEvidence.outcome`, constructed IR `Cancelled { source }` and QSL `IncompleteCause::Cancelled
    { source }` retain the first cause; a later event cannot change them. A hand-built IR outcome alone does not
    satisfy this criterion.
  static_status: tagged
  binder_count: 1
  assessment: computed structural reconciliation; no complete semantic discharge
- id: FR-040-AC-3
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: A run that reaches its execution ceiling with no cancellation event remains `TimedOut`. A `Deadline`
    event recorded only after completion, memory/capture stop or elapsed-ceiling stop does not turn that result
    into `Cancelled`.
  static_status: tagged
  binder_count: 4
  assessment: computed structural reconciliation; no complete semantic discharge
- id: FR-040-AC-4
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: Completion before observed cancellation keeps the report-based result; measured memory exhaustion, memory
    observation failure, output over-limit, unread capture and unconfirmed cleanup keep their existing result/refusal
    and confirmed teardown rules when cancellation competes. A cancellation observed first and causing stop is not
    overwritten by later completion or elapsed time.
  static_status: tagged
  binder_count: 7
  assessment: computed structural reconciliation; no complete semantic discharge
- id: FR-040-AC-5
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: A multi-member group sharing one handle stops wholly on its observed cause, returns `BatchCancelled {
    source, members }`, and classifies no member; another group using a distinct handle retains its own result.
    Reversing the two cause calls changes the cancelled group's source and not the other group's evidence. Distinct
    handles never share one launcher.
  static_status: tagged
  binder_count: 3
  assessment: computed structural reconciliation; no complete semantic discharge
- id: FR-040-AC-6
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: The single-run producer constructs an IR `KaniOutcomeKind::Cancelled { source }` from settled execution
    evidence, and its real call path reaches the FR-030 map. Mapping both IR source variants exhaustively yields
    the identical `quire_exact::CancelCause` inside QSL's `IncompleteCause::Cancelled`; neither metadata nor a copied
    enum determines the source.
  static_status: tagged
  binder_count: 2
  assessment: computed structural reconciliation; no complete semantic discharge
- id: FR-040-AC-7
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: examined
  excerpt: The direct `quire-exact::CancelCause` constructs `qsl-replay::IncompleteCause::Cancelled` through a type-checked
    exhaustive conversion, while all QSL imports remain through `qsl-replay`; no implicit cancellation control,
    source fallback, or compatibility overload is present.
  static_status: tagged
  binder_count: 2
  assessment: computed structural reconciliation; no complete semantic discharge
- id: FR-017
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: context_only
  excerpt: 'When a caller runs an FR-015, FR-022/FR-014 or FR-035 harness, the code generator shall invoke the

    installed Kani backend and retain the backend''s own reported outcome as typed

    execution evidence. A caller may hand the generator several harnesses to run together

    (FR-017-AC-21 to FR-017-AC-25); the generator then forms groups, runs each

    group in one launcher process, and retains one evidence record per harness. Unless a

    statement says "batch" or "group", it describes a run of one harness.


    [FR-040](./FR-040-cancellation-producer.md) adds the one launcher-observed cancellation outcome:

    the backe'
- id: FR-028
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: context_only
  excerpt: 'The code generator shall bound every Kani harness so that its run can finish: each harness identity

    records a memory ceiling and a wall-clock ceiling, a run that exceeds either settles `inconclusive`

    with its own reason, and the evidence records the bounds and ceilings the run used. Proof coverage

    is maximised per family. A family proves the production code where its harness verifies within the

    ceilings, and a bounded shadow together with a refinement obligation where it does not

    ([ADR-003](../../decisions/ADR-003-kani-tractability.md)).


    Planned (IR-241): ADR-003 decides that a shadow exists '
- id: FR-030
  path: spec/kani/functional/FR-030-ir-outcome-terminal-map.md
  role: context_only
  excerpt: 'The generator shall map every Contract IR `KaniOutcome`, paired with the settlement of its replay, to

    exactly one `qsl_replay::TerminalValue`. The map is total over the pair (outcome, replay

    settlement), not over the outcome alone, and it preserves the outcome''s refusal cause: the outcome

    kinds that QSL''s terminal value collapses into one variant stay distinguishable through that

    variant''s typed cause. This is QSL ADR-013 C-09 as merged, whose two inputs are the IR `KaniOutcome`

    and, for a `Counterexample`, the result of its replay. The replay settlement, its six readings and

    the rules that cl'
- id: FR-040
  path: spec/kani/functional/FR-040-cancellation-producer.md
  role: context_only
  excerpt: 'When a caller requests cancellation of a bounded Kani execution, the generator SHALL observe the

    caller-owned `quire_exact::Cancel` handle at the real launcher and retain the cause of the first

    cancellation event that stops that launcher. The caller owns the handle and any timer: a timer can

    call `cancel(CancelCause::Deadline)`, while an elapsed [FR-028](./FR-028-bounded-proof-ceilings.md)

    execution ceiling without a recorded cancellation event remains `TimedOut`. This requirement adds

    one producer path to [FR-017](./FR-017-kani-execution-evidence.md); it does not treat a Contract IR

    outcome r'
- id: src/kani/run/execute.rs
  path: src/kani/run/execute.rs
  role: examined
  excerpt: //! Execution of generated Kani obligation harnesses, singly or as a batch (FR-017).
- id: src/kani/run/launch.rs
  path: src/kani/run/launch.rs
  role: examined
  excerpt: '//! Launching the Kani launcher process: spawn, bounded capture, the wall-clock budget and the'
- id: src/kani/run/owners.rs
  path: src/kani/run/owners.rs
  role: examined
  excerpt: //! Immutable dependencies of one bounded execution's existing observation and teardown owners.
- id: src/kani/run/namespace.rs
  path: src/kani/run/namespace.rs
  role: examined
  excerpt: '//! Per-run PID-namespace ownership: gated startup, claimed init, and confirmed teardown.'
- id: src/core/test_support.rs
  path: src/core/test_support.rs
  role: examined
  excerpt: //! Scratch directories shared by unit tests across subsystems.
- id: src/core/mod.rs
  path: src/core/mod.rs
  role: examined
  excerpt: '//! The leaf of the crate: the shared primitives every other directory builds on (AD-004).'
- id: src/lib.rs
  path: src/lib.rs
  role: examined
  excerpt: //! Deterministic Rust, property-test, proof, and evidence generation from Quire contracts.
- id: tests/it/layout.rs
  path: tests/it/layout.rs
  role: examined
  excerpt: //! AD-004 layout decisions L-1 and L-2 and FR-024-AC-10, checked over `src/`.
- id: spec/spec.md
  path: spec/spec.md
  role: context_only
  excerpt: 'This specification defines deterministic lowering from a validated contract package into Rust

    oracles, tri-state test harnesses, shaped proptest strategies, Kani obligations and coverage maps. Generated
    artifacts remain traceable to one authoritative source contract.'
- id: spec/assurance/AD-004-cg-crate-layout.md
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: context_only
  excerpt: 'In scope: every file under `src/`, the import edges between them, the public entry points that

    `lib.rs` re-exports, and the helper, identity, node-access and version-profile code the modules

    share. Also in scope are the two layout exceptions recorded in `spec/spec.md` (FR-005 and the

    SuiteRegistry file), which the Subsystem Registry defers to this AD and which this AD settles.


    Out of scope: behaviour. Every requirement keeps its id and its criteria. No requirement id is

    minted here; the labels `L-1` and onward are local to this document, as the labels in the other

    ADs are. Two places record t'
- id: spec/kani/matrix/TC-055-cancellation-producer.md
  path: spec/kani/matrix/TC-055-cancellation-producer.md
  role: context_only
  excerpt: 'Plan a discriminating production-path check for [FR-040](../functional/FR-040-cancellation-producer.md).

    The event reader must supply the observed source; a prebuilt IR outcome tests only the terminal

    map. The existing [TC-041](./TC-041-ir-outcome-terminal-map.md) continues to own the separate

    [FR-030](../functional/FR-030-ir-outcome-terminal-map.md) AC-7 source inspection.'
- id: spec/kani/matrix/TC-043-kani-batching-and-output-bounds.md
  path: spec/kani/matrix/TC-043-kani-batching-and-output-bounds.md
  role: context_only
  excerpt: 'Verify that several harnesses run in one launcher process with each member decided from its own

    report entry and its own console playback, that an output stream over its limit is refused and never

    truncated, that a capture failure is refused, and that no launcher descendant outlives a run

    (IR-277; FR-017-AC-14 and FR-017-AC-21 to FR-017-AC-25), and that the diagnostic record of a

    `NoVerdict` group never feeds classification (FR-017-AC-36), and that a batch member''s CBMC failure

    cause is kept beside the batch timeout rule (FR-017-AC-39). The batch timeout rule it exercises is

    FR-028-AC-12, veri'
- id: spec/core/functional/interface-001-codegen-api.md
  path: spec/core/functional/interface-001-codegen-api.md
  role: context_only
  excerpt: 'The interface''s features in declaration order: every operation the contract above declares, one row
    each.


    IR-598 / FR-015-AC-109 retired the finite-input checked-arithmetic auxiliary and

    its request/result types. The collection and graph auxiliaries and corpus API

    remain public until their separately gated retirement slices.


    | Feature | Kind |

    |---|---|

    | source_obligation_census | operation |

    | generate_bound_oracles | operation |

    | generate_bundle | operation |

    | generate_tristate_harness | operation |

    | generate_i64_strategy | operation |

    | generate_enum_strategy | operation |

    | generate_'
- id: AGENTS.md
  path: AGENTS.md
  role: context_only
  excerpt: See CLAUDE.md.
- id: CLAUDE.md
  path: CLAUDE.md
  role: context_only
  excerpt: '# quire-contract-codegen'
- id: src/kani/run/execute.rs::controlled_owners
  path: src/kani/run/execute.rs
  role: examined
  excerpt: fn controlled_owners(&mut self) -> (OwnerControl, OwnerControl) {
- id: src/kani/run/execute.rs::cancel_running
  path: src/kani/run/execute.rs
  role: examined
  excerpt: 'fn cancel_running<R: Send + std::fmt::Debug>('
- id: src/kani/run/execute.rs::cancellation_settlement_budget
  path: src/kani/run/execute.rs
  role: examined
  excerpt: 'fn cancellation_settlement_budget(remaining_budget: Duration) -> Duration {'
- id: src/kani/run/execute.rs::ControlledMemory
  path: src/kani/run/execute.rs
  role: examined
  excerpt: struct ControlledMemory {
- id: src/kani/run/execute.rs::OwnerReleases
  path: src/kani/run/execute.rs
  role: examined
  excerpt: struct OwnerReleases<'a> {
- id: src/kani/run/execute.rs::controlled_batch_cancellation_arm
  path: src/kani/run/execute.rs
  role: examined
  excerpt: 'fn controlled_batch_cancellation_arm(withhold: bool) {'
- id: src/kani/run/execute.rs::OwnerGate
  path: src/kani/run/execute.rs
  role: examined
  excerpt: struct OwnerGate {
- id: src/kani/run/execute.rs::OwnerControl
  path: src/kani/run/execute.rs
  role: examined
  excerpt: struct OwnerControl {
- id: src/kani/run/execute.rs::ControlledPoll
  path: src/kani/run/execute.rs
  role: examined
  excerpt: struct ControlledPoll {
- id: src/kani/run/execute.rs::recorded_process_gone
  path: src/kani/run/execute.rs
  role: examined
  excerpt: 'fn recorded_process_gone(path: &Path) {'
- id: src/kani/run/execute.rs::CancellationSettlement
  path: src/kani/run/execute.rs
  role: examined
  excerpt: struct CancellationSettlement<R> {
  assessment: private controller-custody infrastructure; no cancellation/prover criterion credit
- id: src/kani/run/execute.rs::settle_cancellation_worker
  path: src/kani/run/execute.rs
  role: examined
  excerpt: fn settle_cancellation_worker<R>(
  assessment: private controller-custody infrastructure; no cancellation/prover criterion credit
- id: src/kani/run/execute.rs::controller_timeout_retains_result_sent_during_abort_join
  path: src/kani/run/execute.rs
  role: examined
  excerpt: fn controller_timeout_retains_result_sent_during_abort_join() {
  assessment: private controller-custody infrastructure; no cancellation/prover criterion credit
- id: src/kani/run/execute.rs::controller_timeout_keeps_disconnected_result_and_worker_panic_distinct
  path: src/kani/run/execute.rs
  role: examined
  excerpt: fn controller_timeout_keeps_disconnected_result_and_worker_panic_distinct() {
  assessment: private controller-custody infrastructure; no cancellation/prover criterion credit
findings: []
bindings:
- test_id: batch_tests::cancellation_before_spawn_has_no_backend_or_report
  ac_id: FR-040-AC-1
  trace: correct
  path: src/kani/run/execute.rs
  line: 2239
  assessment: direct assertion/real-execution support for the exercised subset only
  complete_criterion_discharge: false
- test_id: batch_tests::live_cancellation_preserves_first_event_through_ir_and_qsl
  ac_id: FR-040-AC-1
  trace: correct
  path: src/kani/run/execute.rs
  line: 2909
  assessment: direct assertion/real-execution support for the exercised subset only
  complete_criterion_discharge: false
- test_id: batch_tests::live_cancellation_preserves_first_event_through_ir_and_qsl
  ac_id: FR-040-AC-2
  trace: correct
  path: src/kani/run/execute.rs
  line: 2909
  assessment: direct assertion/real-execution support for the exercised subset only
  complete_criterion_discharge: false
- test_id: batch_tests::child_memory_overage_is_inconclusive_and_kills_the_entire_backend_tree
  ac_id: FR-040-AC-3
  trace: correct
  path: src/kani/run/execute.rs
  line: 1686
  assessment: direct assertion/real-execution support for the exercised subset only
  complete_criterion_discharge: false
- test_id: batch_tests::output_over_limit_is_not_relabelled_as_cancellation
  ac_id: FR-040-AC-3
  trace: correct
  path: src/kani/run/execute.rs
  line: 2179
  assessment: direct assertion/real-execution support for the exercised subset only
  complete_criterion_discharge: false
- test_id: batch_tests::controlled_caller_order_distinguishes_cancelled_and_completed_batch
  ac_id: FR-040-AC-3
  trace: correct
  path: src/kani/run/execute.rs
  line: 3020
  assessment: direct assertion/real-execution support for the exercised subset only
  complete_criterion_discharge: false
- test_id: batch_tests::completed_and_elapsed_runs_do_not_acquire_late_cancellation
  ac_id: FR-040-AC-3
  trace: correct
  path: src/kani/run/execute.rs
  line: 3182
  assessment: direct assertion/real-execution support for the exercised subset only
  complete_criterion_discharge: false
- test_id: batch_tests::child_memory_overage_is_inconclusive_and_kills_the_entire_backend_tree
  ac_id: FR-040-AC-4
  trace: correct
  path: src/kani/run/execute.rs
  line: 1686
  assessment: direct assertion/real-execution support for the exercised subset only
  complete_criterion_discharge: false
- test_id: batch_tests::measured_memory_overage_competing_with_cancellation_keeps_resource_outcome
  ac_id: FR-040-AC-4
  trace: correct
  path: src/kani/run/execute.rs
  line: 1738
  assessment: direct assertion/real-execution support for the exercised subset only
  complete_criterion_discharge: false
- test_id: batch_tests::output_over_limit_is_not_relabelled_as_cancellation
  ac_id: FR-040-AC-4
  trace: correct
  path: src/kani/run/execute.rs
  line: 2179
  assessment: direct assertion/real-execution support for the exercised subset only
  complete_criterion_discharge: false
- test_id: batch_tests::memory_observation_failure_competing_with_cancellation_refuses_public_evidence
  ac_id: FR-040-AC-4
  trace: correct
  path: src/kani/run/execute.rs
  line: 2704
  assessment: direct assertion/real-execution support for the exercised subset only
  complete_criterion_discharge: false
- test_id: batch_tests::unread_capture_competing_with_cancellation_refuses_public_evidence
  ac_id: FR-040-AC-4
  trace: correct
  path: src/kani/run/execute.rs
  line: 2778
  assessment: direct assertion/real-execution support for the exercised subset only
  complete_criterion_discharge: false
- test_id: batch_tests::cleanup_nonconfirmation_competing_with_cancellation_refuses_public_evidence
  ac_id: FR-040-AC-4
  trace: correct
  path: src/kani/run/execute.rs
  line: 2839
  assessment: direct assertion/real-execution support for the exercised subset only
  complete_criterion_discharge: false
- test_id: batch_tests::completed_and_elapsed_runs_do_not_acquire_late_cancellation
  ac_id: FR-040-AC-4
  trace: correct
  path: src/kani/run/execute.rs
  line: 3182
  assessment: direct assertion/real-execution support for the exercised subset only
  complete_criterion_discharge: false
- test_id: batch_tests::cancelled_batch_retains_first_source_and_independent_group
  ac_id: FR-040-AC-5
  trace: correct
  path: src/kani/run/execute.rs
  line: 2959
  assessment: direct assertion/real-execution support for the exercised subset only
  complete_criterion_discharge: false
- test_id: batch_tests::controlled_caller_order_distinguishes_cancelled_and_completed_batch
  ac_id: FR-040-AC-5
  trace: correct
  path: src/kani/run/execute.rs
  line: 3020
  assessment: direct assertion/real-execution support for the exercised subset only
  complete_criterion_discharge: false
- test_id: batch_tests::distinct_cancellation_handles_have_distinct_launchers
  ac_id: FR-040-AC-5
  trace: correct
  path: src/kani/run/execute.rs
  line: 3152
  assessment: direct assertion/real-execution support for the exercised subset only
  complete_criterion_discharge: false
- test_id: batch_tests::cancellation_before_spawn_has_no_backend_or_report
  ac_id: FR-040-AC-6
  trace: correct
  path: src/kani/run/execute.rs
  line: 2239
  assessment: direct assertion/real-execution support for the exercised subset only
  complete_criterion_discharge: false
- test_id: batch_tests::live_cancellation_preserves_first_event_through_ir_and_qsl
  ac_id: FR-040-AC-6
  trace: correct
  path: src/kani/run/execute.rs
  line: 2909
  assessment: direct assertion/real-execution support for the exercised subset only
  complete_criterion_discharge: false
- test_id: batch_tests::cancellation_before_spawn_has_no_backend_or_report
  ac_id: FR-040-AC-7
  trace: correct
  path: src/kani/run/execute.rs
  line: 2239
  assessment: direct assertion/real-execution support for the exercised subset only
  complete_criterion_discharge: false
prior_review_head: 5039d518b6dd460b776a4f2f15acfc8f5c213fd7
round: 1
new_findings: []
dispositions:
- fnd: FND-001
  outcome: fixed
  path: src/kani/run/execute.rs
  after_excerpt: "        let received = receiver.recv_timeout(budget);\n        if received.is_err() {\n      \
    \      abort();\n        }\n        let joined = worker.join();\n        let late = received.is_err().then(||\
    \ receiver.try_recv());"
  reason: Both callers use the shared settlement helper and log initial receive, late payload/unavailability and
    join state separately. The original received result still controls settled and expect, so recovering a late
    result never makes a timeout pass. Actual causal regressions verify late Debug custody, retained timeout failure,
    disconnection and worker panic.
original_findings:
- fnd: FND-001
  severity: medium
  confidence: high
  method: code-review
  check: code-bug
  artifact_id: src/kani/run/execute.rs::cancel_running
  related_ids:
  - src/kani/run/execute.rs::controlled_batch_cancellation_arm
  path: src/kani/run/execute.rs
  lines: 2306-2317; 2982-2991
  excerpt: let received = receiver.recv_timeout(cancellation_settlement_budget(remaining_budget));
  finding: A result sent during timeout abort/join is discarded instead of preserved in the failure diagnostic.
  scenario: The first recv_timeout returns Timeout; abort requests cancellation/releases gates; the worker sends
    its real public Result before join completes. No post-join receive reads that queued Result, so Debug prints
    Err(Timeout), and dropping the receiver loses the actual classification/refusal. The same path exists in the
    diagnostic arm.
  requested_fix: Preserve the initial timeout as a failure, join only after releasing/aborting owners, then nonblockingly
    drain any public result sent during join and include that actual result in Debug diagnostics in both call sites.
    Keep original assertions and budgets; never turn a timeout into success.
changed_delta_verdict: PASS
whole_repo_verdict: not assessed by this method
review_verdict: PASS
merge_condition: Review has no open finding; latest exact artifact custody and root final make ci/full actual Kani
  remain required before merge.
clean_definition: No open or new finding under this method; original FND-001 is fixed.
```
