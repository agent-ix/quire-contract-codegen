<!-- reviewer repo=agent-ix/quire-contract-codegen visibility=public quoin=0.28.4 module=spec-artifacts-process@b82e42e893612b4b4ee94127714b4577ee893502 id=SR-15291 method=gap-analysis lang=rust pr=quire-contract-codegen#501 model=unavailable run=d9d507d8-42eb-4ddc-a8d0-3eed0b37342f -->

SR-15291 gap-analysis, part 12/18. FAIL: 311 computed untagged/ignored-only obligations, four manual IR17 stage-evidence gaps and four actual inherited dangling trace annotations remain. The two IR877 diagnostic controls now bind real tests; actual AC56 inspection is clean. No whole-ticket completion is claimed.

| FND | Severity | Check | Summary |
| --- | --- | --- | --- |
| FND-199 | high | coverage | Inherited FR-033-AC-10 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-200 | high | coverage | Inherited FR-033-AC-14 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-201 | high | coverage | Inherited FR-033-AC-15 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-202 | high | coverage | Inherited FR-033-AC-17 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-203 | high | coverage | Inherited FR-034-AC-1 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-204 | high | coverage | Inherited FR-034-AC-2 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-205 | high | coverage | Inherited FR-034-AC-3 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-206 | high | coverage | Inherited FR-034-AC-4 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-207 | high | coverage | Inherited FR-034-AC-5 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-208 | high | coverage | Inherited FR-034-AC-6 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-209 | high | coverage | Inherited FR-034-AC-7 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-210 | high | coverage | Inherited FR-034-AC-8 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-211 | high | coverage | Inherited FR-034-AC-9 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-212 | high | coverage | Inherited FR-034-AC-10 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-213 | high | coverage | Inherited FR-034-AC-11 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-214 | high | coverage | Inherited FR-034-AC-12 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-215 | high | coverage | Inherited FR-034-AC-13 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |
| FND-216 | high | coverage | Inherited FR-034-AC-14 is untagged at the frozen candidate: no test carries this criterion ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify that obligation. |

+++ [reviewer data]

```yaml
clean: false
scope:
- id: FR-028-AC-1
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: Every harness identity records the request's memory ceiling and wall-clock ceiling, and changing
    either ceiling changes the identity.
- id: FR-028-AC-2
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: A backend Kani run that exceeds its wall-clock ceiling is stopped and classified KaniRunOutcome::Inconclusive
    with the timed-out reason naming the ceiling, never verified or falsified. FR-029 maps this source
    outcome to final QSL Incomplete(TimedOut); the backend classification is not the terminal category.
- id: FR-028-AC-3
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: A backend Kani run that exceeds its memory ceiling is stopped and classified KaniRunOutcome::Inconclusive
    with the memory-exhausted reason naming the ceiling, distinct from timed-out and never verified or
    falsified. FR-029 maps it to final QSL Incomplete(ResourceExhausted); FR-028-AC-24 governs the separate
    refinement run.
- id: FR-028-AC-4
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: Every execution evidence records the bounds of each symbolic argument and the two ceilings
    the run was held to.
- id: FR-028-AC-5
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: Every harness identity, generation refusal and execution evidence names its family.
- id: FR-028-AC-6
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: A harness of a family with no supplied shadow records proof subject `production`.
- id: FR-028-AC-7
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: A family with a supplied shadow yields a harness with proof subject `bounded_shadow` and its
    refinement obligation in the same result, and never the shadow harness alone.
- id: FR-028-AC-8
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: An argument narrowed inside its declared domain records the declared domain, the narrowed bound
    and that the harness covers only the narrowed bound, in the harness identity and in the evidence.
- id: FR-028-AC-9
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: A narrowing outside the argument's declared domain is refused with a typed reason and no harness.
- id: FR-028-AC-12
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: In a group of N harnesses (one launcher process of a FR-017 batch) sharing a wall-clock budget
    T, a member the backend cuts off (entry status Failure, no checks, exit status `timeout`) is `inconclusive`
    timed-out naming T and never falsified, while members that finished keep their verified or falsified
    results; a group still running at N times T is killed, refused as timed out, and no member is classified;
    an N times T too large to represent never elapses and does not panic; a T above 4294967295 whole seconds
    launches the group without `--harness-timeout` and the group still runs, its outer bo
- id: FR-028-AC-13
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: Emit the shadow as a thin wrapper that calls the production oracle, or replace a hard sub-claim
    with `kani::stub`, or add `kani::assume(left == right)` to make an assertion hold; each makes the
    proof a statement about the code it was meant to stand in for, or about nothing.
- id: FR-028-AC-14
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: Return the shadow harness without its refinement obligation, or emit an obligation whose abstraction
    function lives in the shadow's own module, so the shadow's representation is read back into the production
    value and the comparison follows any shadow mutation.
- id: FR-028-AC-15
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: Build production values by a constructor that ignores the oracle's environment, or skip a case
    the environment refuses to construct, so an oracle that declares an optional field required agrees
    on every verdict and count; skip a case whose production outcome is `Refused`, or compare verdicts
    only, so a production subject that charges one pair too many, or refuses a legal pair, agrees with
    the shadow.
- id: FR-028-AC-16
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: Report every run as `exhaustive`, or enumerate only the first cap cases of a larger domain
    and call it `exhaustive`, so a sampled run reads as a complete one.
- id: FR-028-AC-17
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: Map a verified shadow harness to `production_proved`, or report the shadow harness's verdict
    when the refinement run disagrees.
- id: FR-028-AC-18
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: Leave an abstraction undischarged (for example the omitted payload of an absent slot) and pass;
    or let the shadow and its expectation share one bound-reading helper and list none, so a mutation
    of that helper moves both and the proof holds vacuously (ADR-011 FB-10); or mark a bound-reading helper
    `order_only` to escape the mutation rule.
- id: FR-028-AC-19
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: Refine over a smaller integer range than the shadow draws, so the cases the shadow proves and
    the production subject was never compared on coincide only in the cases that were.
- id: FR-028-AC-20
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: Treat an unbounded integer leaf as the whole `i64`, a recursive declaration as depth one, or
    an unsupported family as a production harness, so a harness exists that neither covers the declared
    domain nor says it does not.
- id: FR-028-AC-21
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: Enforce the ceiling on the launcher process alone (CBMC runs as a child and holds the memory),
    classify a memory kill as `timed_out` or `no_verdict`, or run the backend unbounded when the platform
    offers no mechanism, or record a peak that was never observed.
- id: FR-028-AC-22
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: A suite that mutates only the production side, so a shadow that always answers `true` verifies;
    or only the shadow side, so a production subject that never compares a field agrees; or counts a permutation
    of an order-only helper as a mutant the machinery must catch.
- id: FR-028-AC-23
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: Record the version of the first `kani` or `rustc` on `PATH` rather than the one the run launched,
    or omit the solver, so two runs under different backends read as one.
- id: FR-028-AC-24
  path: spec/kani/functional/FR-028-bounded-proof-ceilings.md
  role: examined
  excerpt: Run the refinement in the test process with no ceiling, so an unbounded domain hangs the lane;
    or report a refinement that hit a ceiling as `not_run` or `sampled`, so a run that never finished
    reads as one that did.
- id: FR-029-AC-1
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '`verified` with three SUCCESS checks maps to `Proved { success_checks: 3 }`, and `falsified`
    with a reproduced replay maps to `Refuted`.'
- id: FR-029-AC-2
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '`inconclusive` with the vacuous-proof reason and `cover-unsatisfied` each map to `Proved {
    success_checks: 0 }`, whose QSL category is `inconclusive` with cause `KaniVacuousProof`.'
- id: FR-029-AC-3
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: The timed-out reason maps to `Incomplete(TimedOut)`, and the memory-exhausted and exhausted-unwind-bound
    reasons each map to `Incomplete(ResourceExhausted)`.
- id: FR-029-AC-4
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: The no-verdict reason maps to `Failed`.
- id: FR-029-AC-5
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: The missing-cover-summary reason maps to `Failed`. The failure-without-counterexample reason
    maps by its cause (FR-029-AC-32 to FR-029-AC-35).
- id: FR-029-AC-6
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: The ordinary production/source-predicate map returns no `Tested`.
- id: FR-029-AC-8
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '`falsified` with a replay disagreement of each `DisagreementCause` (`Verdicts`, `Witness`
    and `NoValue`) maps to `Inconclusive(ReplayParity)` carrying that `DisagreementCause`.'
- id: FR-029-AC-9
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '`falsified` with a non-fault `ReplayRefusal` maps to `Inconclusive(ReplayRefused)` carrying
    `ReplayRefusal::code()` of that refusal.'
- id: FR-029-AC-10
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '`falsified` with a fault maps to `Failed` for each of: the call-site fault''s own reading
    (`ReplaySettlement::Fault`, which QSL''s replay result does not carry: QSL reports a fault as an error);
    `ReplayRefusal::Fault` and `ReplayRefusal::Admission(AdmissionFailure::Fault)`, each bare and as the
    cause of `SpineReplayError::Refused`, `FrameReplayError::Refused` and `StateClauseReplayError::Refused`;
    and `CallSiteRefusal::Fault` bare and wrapped in `ReplayPackageError::CallSite`, `FrameReplayError::CallSite`
    and `StateClauseReplayError::CallSite`.'
- id: FR-029-AC-11
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '`falsified` with each CG-raised failure that carries no QSL code maps to `Failed`: `SpineReplayError::UnboundArgument`,
    `FieldDelimiter`, `Transcript`, `WrongArm` and `Identity`; `FrameReplayError::Transcript`, `Envelope`,
    `Name`, `NotAFrame`, `FieldSetMismatch`, `Decode`, `OutOfDomain`, `PreState` (a pre state the generator
    cannot read from the driver''s documents, or one that differs from the playback; a document whose
    bytes do not match its digest is QSL''s `Refused`, read by its code), `ScopeMismatch` and `Identity`;
    `ReplayPackageError::InvalidFunction`; a playback outside the harness proof'
- id: FR-029-AC-12
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: Across every replay settlement other than reproduced, `falsified` maps to a value other than
    `Refuted`.
- id: FR-029-AC-13
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '`falsified` with a non-fault `CallSiteRefusal` or a `DependencyLockError::Input`, each bare
    and wrapped in `ReplayPackageError` and `FrameReplayError`, maps to `Inconclusive(ReplayRefused)`
    carrying `CallSiteRefusal::code()` or `DependencyInputRefusal::code()` of that refusal, and never
    to `Declined`.'
- id: FR-029-AC-14
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '`falsified` with a `DependencyLockError::Input` that carries QSL''s `DuplicateIdentity` refusal
    (code `invalid_package`), as a lock whose only defect is a repeated library identity produces it (FR-016-AC-24),
    maps to `Inconclusive(ReplayRefused)` carrying `invalid_package`.'
- id: FR-029-AC-15
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: For the ordinary source-predicate `ReplaySettlement` input, a falsified outcome given no replay
    settlement is refused with `TerminalPairError::MissingSettlement`, and each other outcome (`verified`,
    `cover-unsatisfied`, `cancelled` and every inconclusive reason) given a settlement is refused with
    `TerminalPairError::UnexpectedSettlement`; neither returns a terminal value. The distinct typed composite-parity
    settlement input is governed by AC-17 and AC-19 to AC-28, not this criterion.
- id: FR-029-AC-16
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'The state-clause replay path (FR-024) settles as the other replay paths do: a `StateClauseReplayResult`
    that settles `ReproducedWithEvaluatedWitness` in category `violation` is a reproduction and an `inconclusive`
    one is a disagreement carrying its `DisagreementCause`, so `falsified` maps to `Refuted` and `Inconclusive(ReplayParity)`;
    `StateClauseReplayError::Refused` and `CallSite` read as `ReplayRefusal` and `CallSiteRefusal` do
    for a non-fault refusal (`Inconclusive(ReplayRefused)` with its catalog code; their fault reading
    is FR-029-AC-10''s), and `Dependencies` as `DependencyLockError` doe'
- id: FR-029-AC-17
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: Add a proof strength to FR-028-AC-17 without a row here, or map it through a wildcard arm to
    `Proved` or `Failed`, so a verified shadow harness whose refinement hit a ceiling has no defined output
    or reads as proved.
- id: FR-029-AC-18
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: For the `falsified` state-clause run of the wrapping debit subject `deposit_debiting`, whose
    post-state value (-1) lies outside `balance`'s declared range, this repository builds the post snapshot
    with that exact unclamped value and does not refuse it itself (not as `StateClauseReplayError::OutOfDomain`
    or any other CG defect), and the run maps to `Refuted`, the ordinary violated terminal, and not to
    `Inconclusive(ReplayRefused)` or `Failed`. QSL settles the exact post-state observation as a reproduced
    violation (QSL-634).
- id: FR-029-AC-19
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: PLANNED/GATED (IR-806/QSL-640). After the cross-outcome disagreement rule, verified shadow
    CeilingReached yields Incomplete(ResourceExhausted); nonzero Exhausted with QSL-derived complete coverage
    yields Proved n; remaining completed nonzero verification yields Tested. Tested is excluded for disagreement,
    ceilings and zero checks; tightening or missing coverage cannot promote it to Proved.
- id: FR-029-AC-20
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'PARTIAL (IR-666 direct F-1 to F-7 report consumption covered; IR-806 must reach these rows
    from a CG-built request over the original proving context). An identity-valid falsified composite
    shadow consumes the shared Refinement and ordered F-1 to F-7 result: Disagreed/Failed/CgDefect; native
    Incomplete or ExecutionFault as GeneratedFault/Failed with NativeCause, winning over an invalid operand
    and every later limit; RefusedInput/ReplayRefused with QSL code and operand index, winning over CeilingReached;
    admission accounting exhaustion as Incomplete(ResourceExhausted)/Admission with counter; exa'
- id: FR-029-AC-21
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'PLANNED (IR-806): `run_terminal_value` has no proof-subject or proof-strength input and `TerminalPairError`
    has no variant for a bounded-shadow outcome; adding either to the map is a deliberate, tested change
    rather than a wildcard arm.'
- id: FR-029-AC-22
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: PLANNED (IR-806). The strength set enumerated by the map equals FR-028-AC-17's closed set;
    adding a strength without its projection and outcome row fails the build or test rather than entering
    a wildcard mapping.
- id: FR-029-AC-23
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: PLANNED/GATED (IR-806/QSL-640). A stopped backend run retains its ordinary timeout/memory inconclusive
    classification; a stopped refinement retains FR-028-AC-24's CeilingReached projection and recorded
    ceiling. Neither stop, nor QSL settlement resource refusal, becomes completed NotExhausted evidence
    or Tested.
- id: FR-029-AC-24
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'PLANNED/GATED (IR-806/QSL-640). After common claim/source/node/bounds/O-09 validation, retained
    refinement_failed/Disagreed evidence for the same valid claim yields Failed/CgDefect for verified,
    falsified, inconclusive and cover-unsatisfied backend outcomes, including zero checks, an operand
    field x declared Int[0, 9] but supplied as x: 12, missing/fault native observation and a falsified
    replay agreeing on its retained case; no early operand/native setup refusal loses this disagreement,
    which precedes resource, vacuity and agreement rows. Missing observations are never fabricated.'
- id: FR-029-AC-25
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'PLANNED (IR-806). With no retained refinement disagreement, every bounded_shadow inconclusive
    reason maps through the ordinary rows: vacuous proof and cover-unsatisfied are Proved0; timeout is
    Incomplete(TimedOut); memory and unwind exhaustion are Incomplete(ResourceExhausted); no-verdict and
    missing-summary are Failed; failure-without-counterexample takes the cause rows (FR-029-AC-32 to FR-029-AC-35).
    No vacuous outcome is coerced into verified evidence.'
- id: FR-029-AC-26
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'PLANNED/GATED (IR-806/QSL-640). The converter preserves the actual QSL record category/cause:
    Proved0 remains inconclusive/KaniVacuousProof and is never promoted by its payload. Zero-count backend
    cases use AC-25, not an unreachable verified-count-zero row.'
- id: FR-029-AC-27
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: PLANNED/GATED (IR-806/QSL-640). The distinct typed composite settlement input refuses a missing
    settlement or another claim's node/run/operation/operand/domain/limits/content binding with no value;
    ordinary source-predicate AC-15 does not consume or reject a valid verified-parity settlement as UnexpectedSettlement.
- id: FR-029-AC-28
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'IR-666 public converter delivered. For any full-identity-bound QSL `CompositeParityResult::Refused`
    report, CG preserves `report.terminal_value()`: non-fault refusal is Inconclusive(ReplayRefused) with
    QSL''s code, while `Fault` and `Admission(Fault)` are Failed. QSL `prepare` refusal precedes F-1;
    operand-admission and exact-comparison refusals follow F-1, so Disagreed wins over those later faults.
    CG verifies a reachable non-fault report by invoking QSL''s public facade with a valid wire request
    and a replay input-byte limit below that request''s size, then passing the actual report through its'
- id: FR-029-AC-29
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: A direct `KaniRunOutcome::Cancelled { source }` maps both IR source variants to QSL `IncompleteCause::Cancelled`
    with the matching exact cause, using the same exhaustive conversion as FR-030; supplying a replay
    settlement returns `UnexpectedSettlement`. The production cancellation path publishes only FR-030's
    result.
findings:
- fnd: FND-199
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-033-AC-10
  related_ids: []
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  lines: '370'
  excerpt: PLANNED/GATED. Each lawful generated family exercised by the completed route has a real verify
    and falsify control with retained backend evidence and mutation-killed same-artifact replay; all canonical
    families have the AC-2/3 decode checks. Families awaiting shadow generation or upstream admission
    remain explicitly unsupported/gated and are reported as gaps, rather than counted as passing end-to-end
    coverage. Open real Text-profile, nested set/bag performance and source-unbounded coverage limitations
    remain explicit; API presence or rational substitution supplies no generated-family acceptanc
  finding: 'Inherited FR-033-AC-10 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-200
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-033-AC-14
  related_ids: []
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  lines: '374'
  excerpt: PLANNED (IR-814; literal operands are admitted one-field `Integer` records, with bounded-field
    narrowing literals (IR-691) and collection-constructor literals gated). When the original application
    node is a `structural.ne` whose two operands are both composite literals and the requested operator
    is NotEqual, the verified request carries NotEqual and an O-09 of two positional `graph_child` arguments,
    each the literal's own node with empty Bounds and no parameter position.
  finding: 'Inherited FR-033-AC-14 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-201
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-033-AC-15
  related_ids: []
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  lines: '375'
  excerpt: PLANNED (IR-814). When the original application node is a `structural.ne` comparing one parameter
    with itself and the requested operator is NotEqual, the verified request carries NotEqual and an O-09
    of two positional `graph_child` arguments with the same node identity, each retaining the drawn Node-keyed
    bounds naming that parameter and neither deduplicated.
  finding: 'Inherited FR-033-AC-15 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-202
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-033-AC-17
  related_ids: []
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  lines: '377'
  excerpt: PLANNED (IR-814). If the owning operand projection refuses an inline non-integer literal operand
    (`InlineNonInteger`), then the builder returns that refusal through `CompositeBuildError::Operands`
    before any QSL invocation, with no report or terminal value and no new CG error variant. No test of
    this path exists yet.
  finding: 'Inherited FR-033-AC-17 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-203
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-1
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1844'
  excerpt: Before the exclusive caller lease, matching helper identity, verified monitor/INIT/peer, observer
    readiness and positive typed Dispatch are established, no production backend instruction is created.
    Caller SIGKILL, abort or OOM-equivalent forced death during initialization may start only bounded
    guardian bootstrap, which exits without backend on absent lease or authorization.
  finding: 'Inherited FR-034-AC-1 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-204
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-2
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1845'
  excerpt: Before any M spawn, O is positively verified outer PID 1 with armed parent-death protection
    and original-exclusive bootstrap authority. Unclaimed inner I is contained by O even if M dies before
    internal child_wait handoff or startup-info reader loss aborts M. Owned outer pidfd termination confirms
    teardown; missing authority refuses before spawn. No host scan, monitor/group exit or successful signal
    proves cleanup.
  finding: 'Inherited FR-034-AC-2 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-205
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-3
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1846'
  excerpt: In ClaimedBootstrap, InitReady or Dispatched, original-caller lease EOF causes guardian cancellation/exit
    and confirmed INIT termination; Dispatched descendants are torn down by the kernel. The test controller
    observes dead-caller cleanup without manufacturing that caller's outcome or evidence.
  finding: 'Inherited FR-034-AC-3 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-206
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-4
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1847'
  excerpt: 'Mutual authority is the specific anonymous connected pair: guardian checks the actual mapped
    creator UID and only the original caller holds its executor endpoint. The bounded executor binds kernel
    credentials of Ready''s actual sender to claimed host INIT/start/pidfd/parent/namespace, not socketpair
    creator credentials alone. Guardian''s creator PID0 is not a host identity. A foreign actor cannot
    rebind a public name or supply replacement lease/Dispatch; another pair, mismatched UID or replayed
    authority refuses.'
  finding: 'Inherited FR-034-AC-4 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-207
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-5
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1848'
  excerpt: C retains actual unreaped Child L; L/O/M/I ownership is verified through host pidfds, start/parent/namespace
    identities and actual outer/inner PID 1. O owns actual M Child and I is M's inner child. Reused, stale,
    incomplete, translated incorrectly or dead identity refuses before Dispatch.
  finding: 'Inherited FR-034-AC-5 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-208
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-6
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1849'
  excerpt: The owned tree memory observer is ready before typed Dispatch; failed observer preparation
    releases no backend instruction.
  finding: 'Inherited FR-034-AC-6 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-209
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-7
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1850'
  excerpt: Bounded inner startup/identity recovery is required before Dispatch. M death/reparenting or
    malformed/missing info cancels through retained outer INIT authority and yields typed setup refusal;
    complete outer termination is independently confirmed. No host scan or gate EOF authorizes a backend.
  finding: 'Inherited FR-034-AC-7 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-210
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-8
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1851'
  excerpt: After Dispatch, original-caller death cancels double-fork/reparented, setsid, late-born and
    nested-PID-namespace descendants through kernel-owned INIT teardown, including descendants absent
    from previous samples. Observed-PID lists or normal-operation host scans are not cancellation authority.
  finding: 'Inherited FR-034-AC-8 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-211
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-9
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1852'
  excerpt: Cancelling one owned namespace leaves concurrent independent runs and unrelated host children
    unaffected.
  finding: 'Inherited FR-034-AC-9 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-212
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-10
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1853'
  excerpt: Normal backend completion, backend exec failure, explicit cancellation, timeout, memory excess,
    capture failure and observation failure each require confirmed owned teardown before any proof conclusion.
    Backend exit or valid success output alone never authorizes acceptance.
  finding: 'Inherited FR-034-AC-10 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-213
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-11
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1854'
  excerpt: 'Guardian SIGKILL, abort or OOM-equivalent death during bootstrap, InitReady or immediately
    after Dispatch cannot release an unowned production backend: the guardian is INIT, and its death tears
    down that namespace. With a live original caller, guardian failure always yields a typed refusal and
    never verified/falsified evidence, even after confirmed teardown and beside a valid success report.'
  finding: 'Inherited FR-034-AC-11 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-214
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-12
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1855'
  excerpt: Every startup refusal, completion and cancellation settles actual owned roles before conclusions.
    Anonymous controls and report pipe/memfd have no persistent names and end at final close. All report-owner
    death reclaims kernel report backing without Drop/unlink; other assigned artifacts retain surviving-owner
    cleanup and truthful all-owner-death limits. No dead actor is claimed to reap or unlink.
  finding: 'Inherited FR-034-AC-12 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-215
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-13
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1856'
  excerpt: The bounded executor requires an explicit path to this package's actual Cargo executable quire-kani-guardian,
    built and supplied with the library from the same package source/build inputs. A missing, non-executable
    or unusable helper gives a typed setup refusal before backend Dispatch, with no PATH/global discovery,
    copied executable, shell substitute or alternate launcher. Cargo library dependency resolution alone
    is not helper delivery.
  finding: 'Inherited FR-034-AC-13 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
- fnd: FND-216
  severity: high
  confidence: high
  method: gap-analysis
  check: coverage
  artifact_id: FR-034-AC-14
  related_ids: []
  path: spec/kani/functional/FR-034-caller-death-ownership.md
  lines: '1857'
  excerpt: Setup docs require the matched package helper roles, safe caller-context user/PID/private-mount
    permissions, verified mapping/setgroups/private proc, nested installed bwrap and pidfd/observer facilities.
    The compiled caller security profile is checked, not inferred from binary mode. Caller-profile restriction
    can refuse runs formerly available through bwrap-only permission; ordinary-caller availability requires
    actual measurement. Missing capability gives typed pre-Dispatch refusal without host-policy mutation
    or weaker mode. Added L/O roles and bounded controls/storage are charged to existing
  finding: 'Inherited FR-034-AC-14 is untagged at the frozen candidate: no test carries this criterion
    ID. Existing IR-878/program ownership remains; this diagnostic slice does not implement or qualify
    that obligation.'
bindings: []
review_head: 3d303ea6988496c4e1102faa58a56c1f260193c3
review_verdict: 'FAIL: 311 computed untagged/ignored-only obligations, four manual IR17 stage-evidence
  gaps and four actual inherited dangling trace annotations remain. The two IR877 diagnostic controls
  now bind real tests; actual AC56 inspection is clean. No whole-ticket completion is claimed.'
part: 12
parts: 18
scope_kind: full-repository criterion presence; no blanket semantic acceptance
```

+++