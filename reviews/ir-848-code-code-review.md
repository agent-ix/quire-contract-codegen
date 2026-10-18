---
id: SR-12940
title: IR848 implementation code-review
type: SpecReview
analysis: code-review
scope: agent-ix/quire-contract-codegen; IR-848 PR443; frozen eleven-file implementation
  delta and owning FR017/FR029/AD004
review_set: subset
---

## Summary

Ticket: IR-848. Production borrows one evidence object. The sole new policy is the exhaustive typed cause match inside FailedWithoutCounterexample/None: Timeout→Incomplete(TimedOut), OutOfMemory→Incomplete(ResourceExhausted), absent/PropertiesFailed/ExitCode/Unknown→Failed. All other outcomes and seven reasons ignore cause; missing/unexpected settlement priority, exact cancellation, replay parity causes and refusals, IR map and classification remain unchanged. No diagnostics parsing, cause mutation, cloned payload, OOM detector, native error-class coupling, public test API, dependency/lock/CI/recipe/staging edit or held AD work was introduced.

## Verdict

PASS. No new findings.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Production borrows one evidence object. The sole new policy is the exhaustive typed cause match inside FailedWithoutCounterexample/None: Timeout→Incomplete(TimedOut), OutOfMemory→Incomplete(ResourceExhausted), absent/PropertiesFailed/ExitCode/Unknown→Failed. All other outcomes and seven reasons ignore cause; missing/unexpected settlement priority, exact cancellation, replay parity causes and refusals, IR map and classification remain unchanged. No diagnostics parsing, cause mutation, cloned payload, OOM detector, native error-class coupling, public test API, dependency/lock/CI/recipe/staging edit or held AD work was introduced.

The independent literal expected row exhaustively matches outcome/reason/cancellation; cause_column exhaustively matches the optional closed cause set; settlement expectations exhaustively match ReplaySettlement. This compiler enforcement is distinct from the manually enumerated probes. Today probes include every variant, absent cause, i32 minimum/zero/64/maximum and Unknown text resembling resource causes, counts 0/7/19 with empty check vectors. Literal QSL expected values distinguish timeout/OOM, zero-count vacuity/category, non-resource failures and both cancellations. Existing QSL/public refusal and exact-state replay controls supply independent expected results. The cross-product preserves every valid replay settlement and invalid pair. Its mutation oracle demonstrably fails old cause-blind behavior.

Existing four execution tests became observed helper calls, with original report/wire/classification/diagnostic assertions still executed by four canonically traced crate-root private wrappers. No launcher infrastructure was copied. Run modules, including test modules, import no terminal/generate/replay back-edge; the root private module orchestrates existing earlier/later helpers. Scanner, HarnessSpec privacy and native class policy are unchanged. Single reports include no-check absence/null/properties and timeout/OOM; failed-check reports retain all five absent-attribution shapes. Five-member batch retains c TimedOut/no cause, d FailedWithoutCounterexample/Timeout, exact timeout T and diagnostics while both terminal values are TimedOut. Raw evidence is unchanged. Real unknown-option assertion reads actual retained evidence before consuming diagnostics, with original option vector/export/unsuccessful exit/message preserved. No newly introduced panic, unsafe, numeric truncation, concurrency or unbounded resource path was found.

Frozen candidate f071895af7d20463423904b93d377d2f8726ab95, base e61c1ec1858aedfd70c719c1603b34fce816437c; PR443, ticket IR-848. Source/spec/tests/branch remained read-only. No applicable AssuranceProfile exists under spec/. No reviewer backend/compiler run was needed; author logs and actual assertions were inspected. Exact process guard was empty at explicit slot release (heavy-release.txt). Root retains final full actual Kani and aggregate make ci gates after artifact custody. The intentional driver/provider compile break requires retained evidence at its next dependency bump; no compatibility shim or downstream edit is requested here.

Author actual full make kani-gate passed on clean 17c51aba7f4f9bf3f6084b6d78eea8bc22c472ad only: 36/36, Kani 0.68.0, tests 751.03 seconds, gate 801 seconds (../kani-gate-author.log and .exit; pre/post candidate records). Independent git diff confirms only the TC-043 coverage-pointer paragraph changed afterward. Source/test/schema/dependency/recipe trees are identical; final-head confined Markdown validation passed. Both refresh attempts were interrupted (143), never credited as passes. Inspected compile/fmt/lint logs, focused 119-pass/7-ignored integration and 7-pass private controls, and old cause-blind mutation exit 101 with Failed versus Incomplete(TimedOut), not setup failure. No full backend pass is claimed on f071895. Synthetic cross-products are mapping evidence; single/five-member execution controls are actual bounded launcher/report paths with declared stand-ins; the unknown-CBMC-option and state-clause tests in the actual gate are genuine Kani.

Methods: code-review with Rust lane, genuine planless gap-analysis, spec-review/base and spec-review/evidence. Other lenses do not apply to status/pointer-only documentation: no changed requirement grammar, structure, relationship, object or architectural boundary. No agents were spawned.

## Examined units

Full excerpts follow; tracker comments use explicitly declared verbatim prefixes for size.

```yaml
scope:
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
- id: FR-029-AC-4
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: The no-verdict reason maps to `Failed`.
- id: FR-029-AC-5
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: The missing-cover-summary reason maps to `Failed`. The failure-without-counterexample
    reason maps by its cause (FR-029-AC-32 to FR-029-AC-35).
- id: FR-029-AC-6
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: The ordinary production/source-predicate map returns no `Tested`.
- id: FR-029-AC-8
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '`falsified` with a replay disagreement of each `DisagreementCause` (`Verdicts`,
    `Witness` and `NoValue`) maps to `Inconclusive(ReplayParity)` carrying that `DisagreementCause`.'
- id: FR-029-AC-9
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '`falsified` with a non-fault `ReplayRefusal` maps to `Inconclusive(ReplayRefused)`
    carrying `ReplayRefusal::code()` of that refusal.'
- id: FR-029-AC-10
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '`falsified` with a fault maps to `Failed` for each of: the call-site fault''s
    own reading (`ReplaySettlement::Fault`, which QSL''s replay result does not carry:
    QSL reports a fault as an error); `ReplayRefusal::Fault` and `ReplayRefusal::Admission(AdmissionFailure::Fault)`,
    each bare and as the cause of `SpineReplayError::Refused`, `FrameReplayError::Refused`
    and `StateClauseReplayError::Refused`; and `CallSiteRefusal::Fault` bare and wrapped
    in `ReplayPackageError::CallSite`, `FrameReplayError::CallSite` and `StateClauseReplayError::CallSite`.'
- id: FR-029-AC-11
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '`falsified` with each CG-raised failure that carries no QSL code maps
    to `Failed`: `SpineReplayError::UnboundArgument`, `FieldDelimiter`, `Transcript`,
    `WrongArm` and `Identity`; `FrameReplayError::Transcript`, `Envelope`, `Name`,
    `NotAFrame`, `FieldSetMismatch`, `Decode`, `OutOfDomain`, `PreState` (a pre state
    the generator cannot read from the driver''s documents, or one that differs from
    the playback; a document whose bytes do not match its digest is QSL''s `Refused`,
    read by its code), `ScopeMismatch` and `Identity`; `ReplayPackageError::InvalidFunction`;
    a playback outside the harness proof bound; a decode failure; and a replay reproduced
    in a category other than `violation`. No `Inconclusive(ReplayRefused)` value carries
    a code that no QSL refusal value supplied.'
- id: FR-029-AC-12
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: Across every replay settlement other than reproduced, `falsified` maps
    to a value other than `Refuted`.
- id: FR-029-AC-13
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '`falsified` with a non-fault `CallSiteRefusal` or a `DependencyLockError::Input`,
    each bare and wrapped in `ReplayPackageError` and `FrameReplayError`, maps to
    `Inconclusive(ReplayRefused)` carrying `CallSiteRefusal::code()` or `DependencyInputRefusal::code()`
    of that refusal, and never to `Declined`.'
- id: FR-029-AC-14
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '`falsified` with a `DependencyLockError::Input` that carries QSL''s `DuplicateIdentity`
    refusal (code `invalid_package`), as a lock whose only defect is a repeated library
    identity produces it (FR-016-AC-24), maps to `Inconclusive(ReplayRefused)` carrying
    `invalid_package`.'
- id: FR-029-AC-15
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: For the ordinary source-predicate `ReplaySettlement` input, a falsified
    outcome given no replay settlement is refused with `TerminalPairError::MissingSettlement`,
    and each other outcome (`verified`, `cover-unsatisfied`, `cancelled` and every
    inconclusive reason) given a settlement is refused with `TerminalPairError::UnexpectedSettlement`;
    neither returns a terminal value. The distinct typed composite-parity settlement
    input is governed by AC-17 and AC-19 to AC-28, not this criterion.
- id: FR-029-AC-16
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'The state-clause replay path (FR-024) settles as the other replay paths
    do: a `StateClauseReplayResult` that settles `ReproducedWithEvaluatedWitness`
    in category `violation` is a reproduction and an `inconclusive` one is a disagreement
    carrying its `DisagreementCause`, so `falsified` maps to `Refuted` and `Inconclusive(ReplayParity)`;
    `StateClauseReplayError::Refused` and `CallSite` read as `ReplayRefusal` and `CallSiteRefusal`
    do for a non-fault refusal (`Inconclusive(ReplayRefused)` with its catalog code;
    their fault reading is FR-029-AC-10''s), and `Dependencies` as `DependencyLockError`
    does; `Name`, `Transcript`, `Envelope`, `Document`, `MissingField`, `UndeclaredField`,
    `DuplicateField`, `OutOfDomain` and `UnsupportedOperationShape` carry no QSL code
    and each maps to `Failed`. A missing state field in CG''s own harness playback
    and a value outside the proof bound are CG defects, and an operation shape CG
    does not support is a CG limit, not a QSL data refusal; none maps to `Incomplete`
    or to `Inconclusive(ReplayRefused)`.'
- id: FR-029-AC-18
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: For the `falsified` state-clause run of the wrapping debit subject `deposit_debiting`,
    whose post-state value (-1) lies outside `balance`'s declared range, this repository
    builds the post snapshot with that exact unclamped value and does not refuse it
    itself (not as `StateClauseReplayError::OutOfDomain` or any other CG defect),
    and the run maps to `Refuted`, the ordinary violated terminal, and not to `Inconclusive(ReplayRefused)`
    or `Failed`. QSL settles the exact post-state observation as a reproduced violation
    (QSL-634).
- id: FR-029-AC-29
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: A direct `KaniRunOutcome::Cancelled { source }` maps both IR source variants
    to QSL `IncompleteCause::Cancelled` with the matching exact cause, using the same
    exhaustive conversion as FR-030; supplying a replay settlement returns `UnexpectedSettlement`.
    The production cancellation path publishes only FR-030's result.
- id: FR-029-AC-32
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: PLANNED (IR-848). A `FailedWithoutCounterexample` run with cause `out_of_memory`
    maps to `Incomplete(ResourceExhausted)`, and the same reason with cause `properties_failed`,
    `exit_code` (any code), `unknown` or no cause maps to `Failed`. Over a single-run
    stand-in that exits 1 and exports a `Failure` report for `proofs::check` with
    no checks and one `error_details` entry with exit status `out_of_memory`, the
    evidence passed to `run_terminal_value` gives `Incomplete(ResourceExhausted)`;
    the same report with exit status `exit_code_64`, `cbmc_exploded`, `properties_failed`,
    a null status or no `error_details` gives `Failed`.
- id: FR-029-AC-33
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'PLANNED (IR-848). A `FailedWithoutCounterexample` run with cause `timeout`
    maps to `Incomplete(TimedOut)`: the single-run stand-in of FR-029-AC-32 with exit
    status `timeout` gives `Incomplete(TimedOut)`. The batch member `c::check` of
    FR-017-AC-39 (a `Failure` with no checks and exit status `timeout`, classified
    `TimedOut`) keeps `Incomplete(TimedOut)` through the timed-out row, and the member
    `d::check` (a `Failure` listing a failed check with no playback, exit status `timeout`,
    classified `FailedWithoutCounterexample` with cause `timeout`) gives `Incomplete(TimedOut)`
    through the cause.'
- id: FR-029-AC-34
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'PLANNED (IR-848). CG''s own memory stop (the memory-exhausted reason)
    and a `FailedWithoutCounterexample` run with cause `out_of_memory` give the same
    terminal value, `Incomplete(ResourceExhausted)`, so one exhaustion has one value
    whoever observed it. The cause is read for the failure-without-counterexample
    reason only: every other outcome and reason maps identically whatever cause is
    passed, including a cause passed with `NoVerdict` or `MissingCoverSummary`.'
- id: FR-029-AC-35
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'PLANNED (IR-848). The cause never creates a verdict. Over every combination
    of outcome, reason and cause (absent, `properties_failed`, `timeout`, `out_of_memory`,
    `exit_code`, `unknown`), and with the replay settlement each outcome takes: a
    `falsified` outcome with a reproduced replay is `Refuted` whatever the cause;
    the cause never turns a non-`Refuted` result into `Refuted`, and never turns any
    result into `Proved` (`Proved` appears only for the rows FR-029-AC-1 and FR-029-AC-2
    give). The `NoVerdict` and `MissingCoverSummary` reasons map to `Failed` whatever
    the cause. Classification is unchanged: the same stand-ins classify exactly as
    FR-017-AC-40 states.'
- id: FR-029-AC-36
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: PLANNED (IR-848). The rationale of this requirement states that Kani reports
    any `SIGKILL` of CBMC as `out_of_memory`, that an `out_of_memory` cause is accepted
    as `Incomplete(ResourceExhausted)` without corroboration, and that the raw cause
    stays in the evidence.
- id: FR-017-AC-37
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'Over single-run launcher stand-ins that exit 1, print `Checking harness
    proofs::check...` and `CBMC failed with status 64` on stdout and `Unknown option:
    --no-such-option` on stderr, and export a report whose one result `proofs::check`
    is `Failure` with no checks and whose one `error_details` entry for `proofs::check`
    has the exit status shown: `exit_code_64` is `Inconclusive` `FailedWithoutCounterexample`
    with `failureCause` `{ "kind": "exit_code", "code": 64 }` and a `diagnostics`
    record of phase `unknown` whose stderr excerpt holds `Unknown option: --no-such-option`;
    `exit_code_15` has `code` 15; `exit_code_0` has `code` 0; `exit_code_-1` has `code`
    -1; `exit_code_2147483647` has `code` 2147483647; `exit_code_-2147483648` has
    `code` -2147483648; `out_of_memory` is `{ "kind": "out_of_memory" }`; `timeout`
    is `{ "kind": "timeout" }` and stays `FailedWithoutCounterexample`; `cbmc_exploded`
    is `{ "kind": "unknown", "exitStatus": "cbmc_exploded" }`; `exit_code_`, `exit_code_064`,
    `exit_code_+64`, `exit_code_-0`, `exit_code_2147483648`, `exit_code_64x` and `EXIT_CODE_64`
    are each `unknown` with that value as `exitStatus`; a 64-byte ASCII value is `unknown`
    with all 64 bytes, a 65-byte and a 100-byte ASCII value are `unknown` with their
    first 64 bytes, and a value whose 64th byte falls inside a 4-byte character is
    cut before that character; with crate directory `/h/u/ws/crate` in the request,
    the value `/h/u/ws/crate/x` is `unknown` with `exitStatus` `<CRATE_DIR>/x`, a
    value holding an ANSI escape is `unknown` with the escape removed, and the value
    `killed pid 4242 after 3.5s` is `unknown` with `exitStatus` `killed pid <PID>
    after <DURATION>`. Every one of these evidences carries a `diagnostics` record.'
- id: FR-017-AC-38
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'A single-run stand-in that exits 1 and exports a `Failure` report listing
    a failed assertion check with no playback on the console, whose entry for the
    harness has exit status `properties_failed`, is `FailedWithoutCounterexample`
    with `failureCause` `{ "kind": "properties_failed" }` and no `diagnostics` member.
    The same report with no `error_details` member, with an entry that names another
    harness only, with two entries naming the harness, with an entry whose `exit_status`
    is null, or whose one result names no `harness_id`, is `FailedWithoutCounterexample`
    with neither a `failureCause` nor a `diagnostics` member. An entry with exit status
    `exit_code_64` beside a verified or cover-unsatisfied report with exit 0, or beside
    a falsified (its playback printed) or unwind-bound-exhausted report with exit
    1, leaves that outcome unchanged with no `failureCause` member; a `NoVerdict`
    run (a success report beside exit 1 with that entry, or no report), a timed-out
    run and a memory-exhausted run have none either. A report whose `exit_status`
    is the number `64`, and one whose `exit_status` is the string `"\ud800"` (a lone
    surrogate escape), each stay refused as malformed with no evidence. A rustdoc
    `compile_fail` example on `KaniFailureCause` that calls `serde_json::from_str`
    for it fails to compile, beside a compiling control that calls `serde_json::to_string`
    on the same value.'
- id: FR-017-AC-39
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'A batch stand-in of members `a::check`, `b::check`, `c::check`, `d::check`
    and `e::check` whose process exits 1 and whose report has `a::check` `Success`
    with a passing assertion, every cover satisfied and no exit status, `b::check`
    `Failure` with no checks and exit status `exit_code_64`, `c::check` `Failure`
    with no checks and exit status `timeout`, `d::check` `Failure` listing a failed
    assertion with no playback block headed for its path and exit status `timeout`,
    and `e::check` `Failure` listing a failed assertion with no playback block headed
    for its path and exit status `properties_failed` (the entry real Kani writes for
    a failed assertion), gives `a::check` `Verified` with no `failureCause`; `b::check`
    `FailedWithoutCounterexample` with `{ "kind": "exit_code", "code": 64 }` and a
    `diagnostics` record of phase `unknown` holding the group''s excerpts; `c::check`
    inconclusive timed out naming T, with no `failureCause` and no `diagnostics`;
    `d::check` `FailedWithoutCounterexample` with `{ "kind": "timeout" }` and a `diagnostics`
    record; and `e::check` `FailedWithoutCounterexample` with `{ "kind": "properties_failed"
    }` and no `diagnostics` member. The same group whose report holds two entries
    for `b::check`, one `timeout` and one `exit_code_64`, gives `b::check` inconclusive
    timed out with no `failureCause`, the timeout rule reading the entries as before.'
- id: FR-017-AC-40
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'The cause never feeds classification: three single-run stand-ins with
    the same `Failure` report, no checks and the same exit, whose entries give `exit_code_64`,
    `out_of_memory` and `cbmc_exploded` and whose streams differ, classify to the
    same outcome, checks, SUCCESS count and playback, and their evidence is equal
    in every field but `failureCause`, `diagnostics`, `memory` and the report-path
    argument; the cause is the only input of the FR-029 terminal value that differs:
    `exit_code_64` and `cbmc_exploded` give `Failed`, equal to the terminal value
    of the same evidence with `failureCause` and `diagnostics` absent, and `out_of_memory`
    gives `Incomplete(ResourceExhausted)` (IR-848, FR-029-AC-32). In the `make kani`
    lane, a generated harness in a crate whose manifest carries `[package.metadata.kani]`
    with `unstable = { unstable-options = true }` and `flags = { cbmc-args = ["--no-such-option"]
    }`, its option vector unchanged, is run through `execute_kani_obligation`: the
    run exits unsuccessfully, exports a report and is `Inconclusive` `FailedWithoutCounterexample`
    with `failureCause` `{ "kind": "exit_code", "code": 64 }` and a `diagnostics`
    record whose stderr excerpt holds `Unknown option: --no-such-option`.'
- id: FR-029-AC-25
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: context_only
  excerpt: 'PLANNED (IR-806). With no retained refinement disagreement, every bounded_shadow
    inconclusive reason maps through the ordinary rows: vacuous proof and cover-unsatisfied
    are Proved0; timeout is Incomplete(TimedOut); memory and unwind exhaustion are
    Incomplete(ResourceExhausted); no-verdict and missing-summary are Failed; failure-without-counterexample
    takes the cause rows (FR-029-AC-32 to FR-029-AC-35). No vacuous outcome is coerced
    into verified evidence.'
- id: FR-029-AC-30
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: context_only
  excerpt: PLANNED (IR-806). A `bounded_shadow` outcome (verified, falsified, inconclusive
    or cover-unsatisfied) reaches only the composite parity route on the production
    path and never `run_terminal_value`, because `KaniRunOutcome` cannot tell a shadow
    from a production proof and a verified shadow outcome there would read as `Proved`,
    which FR-028-AC-17 forbids.
- id: FR-017-AC-42
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: context_only
  excerpt: '`class()` is `CompletionRefusal` for `invalid_request`, `artifact_package`
    and `oracle_stop`, and `EnvironmentFault` for `snapshot_io`, `native_launch`,
    `dependency_lock`, `native_result`, `snapshot_changed`, `identity_encode` and
    `codegen_lock`. For `proof_not_verified` it is `CompletionRefusal` for evidence
    outcome `Falsified`, `CoverUnsatisfied`, and `Inconclusive` `MissingCoverSummary`,
    `VacuousProof`, `UnwindBoundExhausted` or `FailedWithoutCounterexample` with no
    `failureCause` or `properties_failed`, and `EnvironmentFault` for `Cancelled`,
    `Inconclusive` `TimedOut`, `MemoryExhausted` or `NoVerdict`, and `FailedWithoutCounterexample`
    with a `failureCause` other than `properties_failed`, and for the internal inconsistency
    of outcome `Verified`. For `kani_execution` it is `CompletionRefusal` for nested
    `HarnessNotInCrate`, `HarnessProvenanceMismatch`, `OutputOverLimit` and `PlaybackForNonMember`,
    and `EnvironmentFault` for every other nested refusal. Real errors produced by
    the runner for the request-mismatch, oracle-stop and falsified-proof causes are
    refusals (`dependency_lock` is an `EnvironmentFault` unreachable from a canonical
    request and is tested only through an in-crate seam); a real single run cancelled
    by its source and one stopped by its wall-clock timeout are faults, as are the
    injected snapshot-change, native-launch, malformed-helper-output and missing-embedded-lock-revision
    causes.'
- id: TC-027
  path: spec/kani/matrix/TC-027-kani-execution-evidence.md
  role: examined
  excerpt: '## Expected Results


    Only the all-covers-satisfied success is verified. The unsatisfied and partial

    covers are cover-unsatisfied with their counts. The failed run with an

    assertion playback is falsified carrying that playback and not the cover

    playback; the failure with only a cover playback, the build failure, the

    unsuccessfully exited process, and the success report with no cover property are

    inconclusive with their own reasons. The failed unwinding assertion is

    inconclusive as an exhausted bound and not falsified, and the succeeded

    unwinding check in a listing is verified. The run with zero successful checks
    is inconclusive

    with the vacuous-proof reason (FR-017-AC-13), unless it is a precondition harness,
    which its

    cover summary decides.


    For a caller Text-admission harness, the same classifications apply while the

    evidence remains bound to its caller identity and harness path, never a

    checked-package node. The caller route and its real-Kani outcomes are Planned

    under FR-017-AC-26.


    The launcher, exercised with real short-lived processes: a `Duration::MAX`

    timeout does not panic; a capture thread told to stop returns what is already
    in its pipe, stops

    while a write end is still open and idle, and stops within its drain limit while
    a straggler keeps

    writing; and a run that times out has a real grandchild killed with it (FR-017-AC-15
    through

    FR-017-AC-17). The output cap (FR-017-AC-14) moved to TC-043.


    Each capture''s exported report parses to the expected typed report and classifies
    to verified, falsified with

    the assertion playback passed through verbatim, exhausted bound, cover-unsatisfied
    0 of 1,

    cover-unsatisfied 1 of 2 and missing cover summary respectively (FR-017-AC-12).


    NoVerdict diagnostics (FR-017-AC-32 to FR-017-AC-35): the non-compiling harness

    exits unsuccessfully, exports no report and is `Inconclusive` `NoVerdict`, with
    phase `compile`, the

    rustc error code and Kani''s `error: Failed to execute cargo` line in the stdout
    excerpt and Cargo''s

    `error: could not compile` line, its crate path as `<CRATE_DIR>`, in the stderr
    excerpt; the silent

    missing-report stand-in is `NoVerdict` with phase `launcher` and two empty excerpts
    with

    `omittedBytes` 0. The stdout stand-ins are, in the order listed, `solver`, `backend`,
    `compile`,

    `unknown`, `backend` and `unknown`; the `-j 2` run is `unknown`; then `compile`;
    `launcher`; and

    `unknown` for the report of success beside exit 1. A marker outside the excerpt
    window still

    decides the phase. The 8192-byte stream is whole with `omittedBytes` 0; the 8193-byte
    stream keeps

    its longest line-aligned suffix of at most 8192 bytes with `omittedBytes` the
    bytes before it; the

    long final line keeps at most 8192 bytes ending at its last byte from a character
    boundary; the

    10000-byte line with its line feed keeps its last 8192 bytes ending in the line
    feed; the line

    followed by `error: last` keeps exactly `error: last` and its line feed; no character
    is split;

    invalid UTF-8 becomes U+FFFD; escapes are gone; the line-feed-only stream is kept;
    no excerpt

    exceeds 8192 bytes. Every FR-017-AC-35 input yields its stated output; home `/`
    gives no `<HOME>`,

    no home leaves the `.kani` path as `<PATH>`, the two process `HOME` values give
    the same record,

    and the runs that differ only in redacted values serialize byte-identical records
    as

    `diagnostics { phase, stdout { text, omittedBytes }, stderr { text, omittedBytes
    } }`. The

    `compile_fail` rustdoc example that deserializes the record fails to compile while
    its

    `serde_json::to_string` control compiles. No evidence of a non-`NoVerdict` run
    has a `diagnostics`

    member, apart from the `FailedWithoutCounterexample` evidence of a CBMC failure
    cause below.


    CBMC failure cause (FR-017-AC-37, FR-017-AC-38, FR-017-AC-40): every exit-status
    run

    is `Inconclusive` `FailedWithoutCounterexample`. `exit_code_64`, `exit_code_15`,
    `exit_code_0` and

    `exit_code_-1`, `exit_code_2147483647` and `exit_code_-2147483648` carry

    `failureCause { kind: "exit_code" }` with `code` 64, 15, 0, -1, 2147483647 and
    -2147483648;

    `out_of_memory` and `timeout` carry their own `kind`; `cbmc_exploded` and each
    malformed

    `exit_code` spelling carry `{ kind: "unknown", exitStatus }` with the value verbatim;
    the 64-byte

    value is kept whole, the 65-byte and 100-byte values keep their first 64 bytes
    and the straddling

    value is cut before the character; the path value is `<CRATE_DIR>/x`, the escape
    is removed, and `killed pid 4242 after 3.5s` is

    `killed pid <PID> after <DURATION>`. Each of them

    carries a `diagnostics` record of phase `unknown` whose stderr excerpt holds

    `Unknown option: --no-such-option`. The `properties_failed` run carries

    `{ kind: "properties_failed" }` and no `diagnostics` member; the runs with no
    entry, another

    harness''s entry, two entries, a null exit status and no `harness_id` are

    `FailedWithoutCounterexample` with neither member. The verified, cover-unsatisfied,
    falsified,

    unwind-bound-exhausted, `NoVerdict`, timed-out and memory-exhausted runs keep
    their outcomes and have

    no `failureCause` member. The numeric `exit_status` and the lone-surrogate string
    are each refused as malformed with no

    evidence. The

    `compile_fail` rustdoc example that deserializes `KaniFailureCause` fails to compile
    while its

    `serde_json::to_string` control compiles. The three runs differing in cause and
    stream text have

    equal outcome, checks, SUCCESS count and playback, equal evidence in every field
    but

    `failureCause`, `diagnostics`, `memory` and the report-path argument, and FR-029
    terminal value

    `Incomplete(ResourceExhausted)` for the `out_of_memory` run and `Failed` for the
    `exit_code_64` and

    `cbmc_exploded` runs (FR-029-AC-32), as asserted by

    `cbmc_failure_terminal_value_reads_the_retained_cause` in `tests/it/kani_obligations.rs`
    and

    `bounded_cause_changes_only_terminal_projection_and_retains_evidence` in

    `tests/it/kani_terminal_private.rs`. The real run whose manifest passes `--no-such-option`
    to CBMC exits

    unsuccessfully, exports a report and is `FailedWithoutCounterexample` with

    `{ kind: "exit_code", code: 64 }` and a stderr excerpt holding `Unknown option:
    --no-such-option`.


    The absent launcher is a typed tool refusal naming the launcher and its path.


    The routed scalar harness''s crate missing its `src/lib.rs` source is `HarnessNotInCrate`;
    its

    cover classifies identically to a contract harness''s; and, in the `make kani`
    lane, it runs with

    its evidence carrying `None` for obligation kind.


    The paired routed-scalar run with operand `4` produces Kani `Verified` and an
    actual native

    `Outcome::Completed(5)`. Its receipt retains the selected scalar identity, operand
    vector, exact

    request limits used by the native meter, proof-content identity and Kani evidence.
    Any unsuccessful proof, native execution fault,

    malformed native response or changed staged tree yields no receipt. Request-context
    mismatch is

    typed and occurs before either child is started; changing an otherwise valid operand
    produces a

    receipt for that exact ordered request, which cannot be substituted for a prior
    receipt.


    Paired routed-scalar proof/native receipt (FR-017-AC-27 to AC-31): generate a
    genuine routed

    `x + 1` harness for an admitted integer domain, retain its complete `oracle_artifacts`
    and

    request `ScalarLimits` and request concrete operand `4` within the generated range.
    In the

    `make kani` lane, require the paired runner to stage the package and harness,
    run real Kani on the

    staged harness, and report `Verified`. Require the runner to execute the staged
    public oracle at

    that operand with those exact limits; the opaque receipt must contain actual typed

    `Outcome::Completed(5)`, the exact operand and limits, the scalar identity, measured
    canonical

    identity and Kani evidence. No result derived from the Kani verdict or supplied
    by the test caller

    counts.


    Order-sensitive binary receipt (FR-017-AC-31): generate a genuine routed subtraction
    operation

    with two symbolic operands over a domain admitting both concrete vectors `[4,
    1]` and `[1, 4]`,

    and one exact request `ScalarLimits` value.

    Run each vector through the paired runner separately in the `make kani` lane.
    Each run must execute

    real Kani and report `Verified`, then invoke the staged public oracle with the
    exact ordered vector

    and request limits. Require native `Outcome::Completed(3)` for `[4, 1]` and

    `Outcome::Completed(-3)` for `[1, 4]`; each opaque receipt must carry its own
    exact ordered vector

    and actual typed result. The receipts share the same canonical proof-content identity
    because the

    proved and executed staged bytes are unchanged. Both runs and both receipt assertions
    are

    mandatory; neither the unary `x + 1` case nor an optional binary case substitutes
    for them.


    Request-boundary controls (FR-017-AC-28): before either child starts, separately
    change the

    generated library bytes, generated artifact path, selected scalar identity, selected
    oracle symbol,

    operand count and operand value outside its retained range. Each inconsistent
    request must return

    a typed mismatch/refusal, produce no receipt, and leave a launch marker absent.
    For the limits

    control, first run the same generated harness, artifacts and operand `[4]` with
    the normal request

    limits; require Kani `Verified`, native `Outcome::Completed(5)`, and the exact
    request limits in the

    receipt and meter observation. Then change only `result_units` to `0` and repeat
    against the same

    generated staged bytes. Require Kani `Verified` again, the same proof-content
    identity as the

    `Completed(5)` receipt, and `Outcome::Incomplete` with `limit_kind = ResultUnits`,
    `limit = 0`,

    `consumed = 0`, `next_charge = 1`, and charge point `IntegerArithmeticResultRetain`.
    The meter

    observation must report zero consumed result units and no admitted result-retain
    charge. In both

    runs, assert that the exact supplied `ScalarLimits`, the limit value observed
    at meter

    initialization, and the receipt''s returned limits agree. This proves supplied-to-used-to-returned

    equality while the same artifact identity is retained across distinct native outcomes;
    it does not

    assert that either request matches the upstream generation context.

    For a two-operand scalar identity, change only the

    operand order and confirm the receipt retains the exact changed order; it must
    not be possible to

    reuse an earlier receipt as evidence for the reordered request. The mandatory
    AC-31 binary runs

    must demonstrate this with the distinct subtraction results above. A different
    in-range operand

    vector may be run as a distinct request, but its receipt must bind that vector
    separately from the

    unchanged proof-content identity. A caller-selected digest must not make any inconsistent
    request

    acceptable. The IR-773 consumer''s comparison against its retained original limits,
    and its

    local refusal/skip behavior on mismatch, are consumer-owned checks outside this
    CG test. If the

    driver has no defined typed outcome for a mismatch, its owner must specify and
    plan that outcome

    before implementation. No driver adversarial coverage is claimed here.


    Scalar native error discriminant (FR-017-AC-41 to AC-44): build one `KaniScalarNativeError`
    of each

    of the twelve variants, the nested `Kani` error once for each `KaniExecutionRefusal`
    variant, and

    `ProofNotVerified` once for each evidence outcome.

    (1) Assert `code()` of each equals its table code, that the twelve codes are pairwise
    distinct, that

    `Kani` gives `kani_execution` for every nested refusal, and that the type, the
    class enum and the

    code constants are reachable at the crate root. (2) Assert `class()` of each variant,
    of each nested

    `Kani` refusal, and of `ProofNotVerified` for `Falsified`, `CoverUnsatisfied`,
    each inconclusive

    reason, `Cancelled`, `Verified` (an internal inconsistency, requiring `EnvironmentFault`
    and no panic), and `FailedWithoutCounterexample` with no `failureCause`, with
    `properties_failed` (a real single-run stand-in of AC-38''s failed assertion without
    playback, requiring `CompletionRefusal`) and with each of `timeout`, `out_of_memory`,
    `exit_code` and `unknown` (requiring `EnvironmentFault`), against

    the FR-017 table. Produce real errors by the runner for a request mismatch, an
    oracle stop and a falsified proof, and require `CompletionRefusal`;

    produce a real single run cancelled by its retained source and a real single run
    stopped by its

    wall-clock timeout (launcher stand-ins), each `ProofNotVerified`, and require
    `EnvironmentFault`

    with the evidence outcome unchanged; construct `DependencyLock` through an in-crate
    seam only (it is unreachable from a canonical request, whose manifest must equal
    the canonical one, and a lock update that writes no matching revision or an unreadable
    lock is its host fault) and require `EnvironmentFault`; inject a changed staged
    byte, a native launch failure,

    malformed helper output and an embedded lock with no runtime revision, and require

    `EnvironmentFault` (the last as `codegen_lock`). (3) Force the canonical identity
    encode to fail (a

    content limit of zero in a test seam) and require `IdentityEncode`, `identity_encode`,

    `EnvironmentFault`, and not `NativeResult`; feed malformed helper output and require

    `native_result`. (4) Keep a frozen list of all variants and codes and of the mapping
    to class: one class for a fixed

    variant, and for `ProofNotVerified` and `Kani` the class of every `KaniRunOutcome`,

    `KaniInconclusiveReason`, `KaniFailureCause` and nested-refusal value, with exhaustive
    `match`es (no

    wildcard) over those types inside the defining crate that stop compiling when
    one is added, and

    assert the nested `OutputOverLimit` and `OutputUnread` codes are unchanged. (5)
    Non-exhaustive and

    closed class: a rustdoc `compile_fail` example matches `KaniScalarNativeError`
    exhaustively without

    a wildcard arm and must fail to compile, with a compiling control that adds the
    wildcard arm; the

    class enum is closed by a compiling control that matches `KaniScalarNativeErrorClass`
    exhaustively

    with exactly `CompletionRefusal` and `EnvironmentFault` and no wildcard, together
    with the absence of

    `#[non_exhaustive]` on it; the doctests run by `cargo test --doc`, the pattern
    of the AC-38

    serialize-only check.

    These controls are implemented in the scalar runner''s unit tests and public rustdoc
    examples.

    The crate-root private scalar-native tests exercise cancellation and timeout with
    bounded

    single-run launcher stand-ins. Their real-Kani seeded false-bound fixture keeps
    the routed

    package and embedded oracle intact, changes the asserted bound before the normal
    renderer and

    origin seal, and requires the paired runner to return real `Falsified` evidence
    without starting

    native execution. It is a deliberately defective proof property, not a naturally
    failing admitted

    production claim. A separate real-proof control selects a bounded native-helper
    protocol

    stand-in through an immutable private installation dependency; it records the
    actual native

    arguments and emits an oracle-stop record, and the public paired runner must return

    `OracleStop` / `oracle_stop` / `CompletionRefusal`. This exercises runner propagation
    without

    claiming an actual runtime-produced stop for a supported admitted integer operation.
    Fabricated

    proof evidence is confined to the exhaustive classification table.


    Snapshot controls (FR-017-AC-29 and AC-30): measure the staged path/byte set for
    the complete

    generated package, retained harness, native invocation adapter and resolved dependency
    lock;

    attempt a source-tree write from each child execution boundary, and require that
    neither child can

    change it. Inject a changed byte in each execution-relevant staged item, a replaced
    path and an

    unexpected file after staging and across the Kani/native boundary;

    each must produce a typed failure and no receipt carrying the earlier identity.
    Check that changes

    to the concrete operand vector, its order, request limits or scalar identity cannot
    be represented

    by reusing an earlier receipt. Inspect the public receipt surface to verify that
    it has read-only access to

    the actual Kani evidence, exact scalar invocation context, canonical identity
    and typed native

    outcome, and that callers cannot construct, deserialize or mutate it. The generated
    package is

    staged for this run and is not copied into a repository.


    Every generation-time refusal above exposes no harness, so `execute_kani_obligation`
    — which

    takes a `KaniObligationHarness` — has nothing to run for it: a generation-time
    classification can

    never surface as one of FR-017''s execution outcomes because no code path converts
    one into the

    other.


    For the scalar error discriminant, each variant has its table code and class,
    the codes are distinct

    and frozen, the three refusal variants, the nested request-fixed `Kani` refusals
    and a `ProofNotVerified` carrying a

    property or proof verdict are `CompletionRefusal`; the seven fault variants, every
    other nested refusal and a

    `ProofNotVerified` of a cancelled, timed-out, memory-exhausted or no-verdict run
    are

    `EnvironmentFault`, and an identity-encode failure is `IdentityEncode`, never
    `NativeResult`.


    In the `make kani` lane the three healthy harnesses are verified, each with exit
    code zero and an

    argument vector that begins, after the subcommand, with its harness identity''s
    options; the seeded defect is

    falsified with a concrete counterexample naming its harness symbol and a nonzero
    exit code; the

    jointly unsatisfiable contract is cover-unsatisfied rather than verified; and
    the crate that does

    not contain the harness is refused with no run.


    ## Implementation


    `src/kani/classify.rs` unit tests for classification, including

    `a_report_with_no_successful_check_is_inconclusive_not_verified_even_with_every_cover_satisfied`
    for

    FR-017-AC-13, and the launcher tests in `src/kani/run/launch.rs` (`a_capture_thread_*`,

    `a_timeout_of_duration_max_never_elapses_and_does_not_panic`,

    `a_run_exceeding_its_budget_kills_a_real_grandchild_not_only_the_direct_child`)
    for FR-017-AC-15

    through FR-017-AC-17. The two tests that asserted the silent tail an over-long
    stream used to

    keep are deleted; the TC-043 refusal tests replace them;

    the `src/kani/classify.rs` tests `tc_027_a_report_that_changed_shape_is_refused_not_classified`,

    `tc_027_a_report_without_exactly_one_harness_is_refused`,

    `tc_027_real_kani_the_per_check_view_carries_id_class_location_and_status`,

    `tc_027_an_unknown_line_is_none_and_a_non_numeric_line_is_refused`,

    `tc_027_the_console_banner_never_decides_the_verdict` and the six `tc_027_real_kani_*`
    capture

    tests for FR-017-AC-12 and FR-017-AC-18, with

    `tc_027_the_per_check_view_has_one_serialized_wire_shape` (FR-017-AC-20) in

    `src/kani/output/report.rs` and

    `tc_027_playback_scanning_returns_the_property_block_and_stops_at_an_unterminated_fence`
    in

    `src/kani/output/playback.rs`; the `src/kani/run/execute.rs` tests

    `tc_027_execution_reads_only_the_report_its_own_run_exported`,

    `tc_027_the_launch_exports_the_report_after_the_harness_options` and

    `tc_027_concurrent_runs_in_one_target_directory_keep_their_own_reports`, the

    `src/kani/run/report_file.rs` test `tc_027_the_report_is_read_bounded_and_refused_not_truncated`

    and the `src/kani/classify.rs` test

    `tc_027_an_unreadable_or_missing_report_is_refused_never_inconclusive` for FR-017-AC-18
    (none of

    them covers FR-017-AC-19, which is Planned), with the `src/kani/classify.rs` test

    `tc_027_a_success_report_listing_a_failed_check_is_refused_never_verified` (FR-017-AC-18)
    and the

    `src/kani/output/report.rs` test `tc_027_a_class_spelled_cover_or_unwind_is_never_other`

    (FR-017-AC-20); and `tests/it/kani_obligations.rs` for the refusals, the

    generation/execution boundary and the `make kani` lane. The lane is `#[ignore]`d
    and runs through

    `make kani` under a host-wide lock, because Kani and CBMC are memory-heavy and
    must run one harness

    at a time.


    FR-017-AC-32 to FR-017-AC-35 are covered by: the `make kani` lane test

    `tc_027_real_kani_non_compiling_crate_is_no_verdict_with_phase_compile` in

    `tests/it/kani_obligations.rs` (real Kani on a crate holding the harness beside
    an item that does

    not compile); the `src/kani/run/execute.rs` stand-in tests in `batch_tests::no_verdict_diagnostics`

    (`tc_027_a_silent_launcher_with_no_report_is_no_verdict_with_phase_launcher`,

    `tc_027_marker_stand_ins_decide_the_failure_phase`,

    `tc_027_a_runs_streams_are_redacted_against_the_requests_directories`,

    `tc_027_runs_differing_only_in_redacted_values_serialize_identical_records`,

    `tc_027_the_same_run_under_two_process_homes_gives_the_same_record` and

    `tc_027_evidence_of_every_other_outcome_has_no_diagnostics_member`); and the `src/kani/output/diagnostics.rs`

    tests of the phase rules, the excerpt bounds and every redaction pair. The serialize-only
    check is

    the rustdoc `compile_fail` example on `KaniNoVerdictDiagnostics`, run by `cargo
    test --doc`, with

    its compiling `serde_json::to_string` control beside it.


    The CBMC failure cause (FR-017-AC-37, FR-017-AC-38, FR-017-AC-40) is covered by
    the bounded

    report controls in `tests/it/kani_terminal_private.rs`, which reuse

    `batch_tests::no_verdict_diagnostics::failure_cause` in `src/kani/run/execute.rs`,
    and the

    remaining execution tests `tc_027_the_cause_is_kept_for_no_other_outcome` and

    `tc_027_a_malformed_exit_status_still_refuses_the_report`. For the FR-029 terminal
    value,

    `cbmc_failure_terminal_value_reads_the_retained_cause` in `tests/it/kani_obligations.rs`

    passes `&KaniExecutionEvidence` to `run_terminal_value`: `out_of_memory` gives

    `Incomplete(ResourceExhausted)`, `timeout` gives `Incomplete(TimedOut)`, and `exit_code_64`
    and

    `cbmc_exploded` give `Failed` (FR-029-AC-32, FR-029-AC-33). The map never reads
    the diagnostic

    record. The cause is also covered by the tests of the exit-status

    vocabulary, the 64-byte cut and the one-entry rule in `src/kani/output/report.rs`;
    by the serialize-only

    check, which is the rustdoc `compile_fail` example on `KaniFailureCause` run by
    `cargo test --doc`

    beside its `serde_json::to_string` control and the `tc_027_the_cause_implements_no_deserialize`

    autoref probe; and, in the `make kani` lane, by

    `tc_027_real_kani_unknown_cbmc_option_keeps_exit_code_64_as_the_failure_cause`
    in

    `tests/it/kani_obligations.rs`, which runs real Kani on a crate whose manifest
    hands CBMC

    `--no-such-option`.


    The previously named-file AC-19 tests demonstrate the superseded internal allocation
    only; they do

    not cover the unnamed writer bound, descriptor roundtrip or kernel reclamation
    above. No current

    main executable test covers FR-017-AC-27 to AC-31; those paired-run and mutation-control
    tests are

    required as part of the IR-791 implementation, not credited by this specification
    amendment.



    Evidence staging follows TC-049''s explicit delivery allocation. Slice 1 uses
    ordinary normal-library

    report/export, collector and kernel descriptor seams without fixture extension.
    Slices are internal

    commit stages of ONE lifecycle CODE PR, not separate merges. IR-655 SPEC may use
    real unmerged O

    source but must merge before its fixture CODE and the single CODE PR merge. All
    internal stages

    and required pre-PR/full gates remain mandatory. Its assertions do not tag a mixed
    FR-034

    collector/lifetime criterion whose O-origin or independent termination

    witness remains owed to IR-655 slice 2. Both slices remain PLANNED/UNRUN; actual
    export or unit-test

    compile/adaptation gaps are reported, not bypassed with named compatibility, helper
    identity

    override, skipped tests or invented coverage. All guarantees precede complete
    IR-639/MVP acceptance.


    Scalar proof-content digest (FR-017-AC-45 to AC-50): run the paired runner on
    a genuine routed

    `x + 1` package. (1) Assert the receipt''s `DigestRecord` has domain `sha256-jcs`
    and 32 bytes equal

    to `as_bytes()`, and recompute SHA-256 of the RFC 8785 encoding of the schema
    object and the sorted

    artifact list in the test, independently of the runner''s encoder. (2) Assert
    the exposed entry

    paths are exactly the AC-46 set in UTF-8 byte order, with the library under

    `src/oracle-artifacts-lib.rs` and the harness under `src/lib.rs`; then drop, rename,
    reorder,

    duplicate and one-byte-change each entry in turn and require a different digest;
    through a test

    seam that writes non-UTF-8 bytes to the snapshot''s `Cargo.lock` (and again to
    the adapter) after

    it is generated, require `Snapshot`, `snapshot_io`, `EnvironmentFault`, and no
    receipt. (3) Run the two operand orders of AC-31 and a different `ScalarLimits`

    value and require equal `DigestRecord`s. (4) After the receipt is returned, assert
    the snapshot path no longer exists (the private snapshot is dropped before the
    receipt is returned), then require the exposed entries to still read and the digest
    to recompute byte-identically; this is a mutation-style check, not a source scan.
    (5) Recompute from only the exposed entries, including

    `Cargo.lock` and `src/bin/ir791_native.rs`, and require equality; substitute one
    byte of the

    package, harness or record and require inequality. (6) Compute a golden digest
    with the measurement function over fixed literal entries, including a

    literal `Cargo.lock` and adapter and not a real staged run, and pin `(schema string,
    golden)` in one

    assertion. Change the `Cargo.lock` bytes in a test seam to see the digest change
    with the schema

    unchanged. A golden edited without a schema bump is a review rule, not a test.
    (7) Meter limits (FR-017-AC-50): require the meter observation''s `ScalarLimits`
    equal the request''s and `scalar_limits()`, equal and unchanged after the snapshot
    is gone; through a seam that makes the adapter report a different limit, require
    `native_result`, `EnvironmentFault` and no receipt. (8) Name `quire_contract_codegen::DigestRecord`
    and `qsl_replay::DigestRecord` as the same type, and make the snapshot directory
    removal fail through a seam: require the same verdict, exposed entries and digest
    as an un-failed run, a non-empty `cleanup_diagnostic()` naming the removal failure,
    and an empty `cleanup_diagnostic()` when removal succeeds; the diagnostic is in
    neither the digest nor the entries No

    driver-side authentication coverage is claimed here: the driver''s recomputation
    and refusal are

    driver-owned (FR-032 boundary), as is the quire-driver FR-009 amendment named
    in FR-017. These controls are implemented by the scalar runner unit tests, the
    real paired receipt and

    subtraction tests in `tests/it/kani_obligations.rs`, and the private cleanup control
    in

    `tests/it/kani_scalar_native_private.rs`. The private real controls run separately
    from the

    integration lane with `cargo test --locked --lib kani_scalar_native_private --
    --ignored

    --test-threads=1`; the default private control covers cancellation and timeout
    stand-ins.

    The literal JCS/SHA-256 golden stays independent of real lock contents, and schema/golden

    changes remain subject to review. No driver authentication coverage is claimed.

    '
- id: TC-040
  path: spec/kani/matrix/TC-040-run-outcome-terminal-record.md
  role: examined
  excerpt: '## Status


    Step 5 and FR-029-AC-32 to FR-029-AC-35 are covered by the literal cause matrix
    in

    `tests/it/terminal_map.rs`, the bounded report controls in `tests/it/kani_terminal_private.rs`

    (reusing the execution fixtures in `src/kani/run/execute.rs`), and

    `cbmc_failure_terminal_value_reads_the_retained_cause` in `tests/it/kani_obligations.rs`.


    Partly covered. Steps 1 to 15 are tests of `tests/it/terminal_map.rs` and the
    bounded report

    controls above. Step 3 includes timed-out, memory-exhausted and exhausted-unwind-bound
    reasons.

    Step 9 (FR-029-AC-10) is covered

    by `tc_040_a_fault_in_any_replay_wrapper_is_failed`, which builds each fault wrapper
    from QSL''s

    constructible `InternalFault` (QSL bcca433) and asserts `Failed`. Step 15 (FR-029-AC-16,
    IR-460)

    is covered: the reproduced and the inconclusive results are real `StateClauseReplayResult`s
    of the

    QSL twin. The state-clause fault readings are step 9''s (FR-029-AC-10). Step 16
    remains planned

    (IR-806 parity in TC-048). Step 17 is covered by the normal replay control and
    real-Kani

    `deposit_debiting` test.

    '
- id: TC-043
  path: spec/kani/matrix/TC-043-kani-batching-and-output-bounds.md
  role: examined
  excerpt: "## Expected Results\n\n1. Each over-limit stream is refused with `kani_output_over_limit`\
    \ naming the stream, the limit and\n   the harness count, the group is killed,\
    \ and nothing is classified; exactly the limit completes\n   with its real exit\
    \ status and the whole stream (FR-017-AC-14).\n2. Both are refused with `kani_output_unread`\
    \ and no outcome (FR-017-AC-25).\n3. The grandchild is killed by the time the\
    \ run returns (FR-017-AC-24).\n4. One process for each group, with one `--harness\
    \ <module::harness> --exact` pair per member in\n   request order, `--harness-timeout`\
    \ T and then the shared options; fewer than N processes for\n   N > 1 compatible\
    \ harnesses; differing identity wall-clock or memory ceilings, launcher, crate\n\
    \   directory or target directory form separate groups. The grouped process tree\
    \ has one aggregate\n   M ceiling; exceeding it kills and refuses the whole group\
    \ with no member classified. The outer\n   wall-clock bound is N times T (a group\
    \ that runs longer than T and inside N times T completes),\n   and a group killed\
    \ at it is refused as timed out with no member classified (FR-017-AC-21;\n   FR-028-AC-21\
    \ covers the memory enforcement mechanism).\n5. The `Duration::MAX` batch carries\
    \ no `--harness-timeout` and its outer bound does not elapse; the\n   batch at\
    \ 4294967295 seconds runs (FR-017-AC-21, FR-028-AC-12).\n6. The successful exit\
    \ with no report is refused with the missing-report refusal and no member is\n\
    \   classified; the unsuccessful exit with no report leaves every member inconclusive\
    \ `NoVerdict`\n   (FR-017-AC-21).\n7. Members are matched by `module::harness`;\
    \ a falsified member beside a verified one leaves the\n   verified one verified;\
    \ Success in a non-zero exit with no Failure entry is inconclusive and with\n\
    \   one is verified; the timeout entry is inconclusive timed-out naming T while\
    \ the others keep their\n   results; the entry with no checks and no timeout is\
    \ inconclusive with no counterexample; each\n   member's evidence carries its\
    \ kind, harness path, launcher path, unwind bound, solver, outcome and\n   checks\
    \ plus the group's vector, the member list, the batch statement and the exit code\n\
    \   (FR-017-AC-22); a sub-second T is carried rounded up. A missing, duplicated\
    \ or unrequested\n   harness refuses the group (FR-017-AC-23).\n8. Each falsified\
    \ member's playback is the block headed for its own path, never an earlier member's;\n\
    \   the falsified member with no block is inconclusive with no counterexample\
    \ and its neighbours keep\n   their results; the block for a non-member path refuses\
    \ the group; the member with two\n   counterexample blocks takes the first, as\
    \ the same harness alone does, and neither it nor its\n   neighbour loses its\
    \ evidence (FR-017-AC-23).\n9. The whole batch is refused with `HarnessNotInCrate`\
    \ and launches nothing (FR-017-AC-23).\n10. Fewer processes after than before\
    \ for N > 1, and the numbers are in the code PR (they are also\n    written by\
    \ the lane to `process-counts.json`).\n11. In the no-report group `a::check` is\
    \ `solver`, `b::check` is `backend` and `c::check` is\n    `unknown`, all `NoVerdict`\
    \ with the group's excerpts; the compile-failure group gives every\n    member\
    \ `compile`; the all-Success report beside exit 1 gives every member `NoVerdict`\
    \ with phase\n    `unknown`; the report with a Failure entry has no member `NoVerdict`\
    \ and no member record; the\n    two runs with the same report and exit have the\
    \ same outcome, checks, SUCCESS count and\n    playback; the two no-report runs\
    \ have equal evidence in every field but `diagnostics` and\n    `memory`; the\
    \ over-limit and unread streams remain refused with `kani_output_over_limit` and\n\
    \    `kani_output_unread`, with no evidence and no record; the FR-029 terminal\
    \ value is `Failed`\n    with and without the record; the differing-home pairs\
    \ start two processes and the equal-home\n    pair one, whose members' excerpts\
    \ are byte-identical (FR-017-AC-36).\n12. `a::check` is `Verified` with no `failureCause`;\
    \ `b::check` is `FailedWithoutCounterexample`\n    with `failureCause { kind:\
    \ \"exit_code\", code: 64 }` and a `diagnostics` record of phase\n    `unknown`\
    \ holding the group's excerpts; `c::check` is inconclusive timed out naming T\
    \ with no\n    `failureCause` and no `diagnostics`; `d::check` is `FailedWithoutCounterexample`\
    \ with\n    `failureCause { kind: \"timeout\" }` and a `diagnostics` record; `e::check`\
    \ is\n    `FailedWithoutCounterexample` with `failureCause { kind: \"properties_failed\"\
    \ }` and no\n    `diagnostics` member. With the two entries for\n    `b::check`,\
    \ `b::check` is inconclusive timed out with no `failureCause` (FR-017-AC-39).\n\
    13. Both are `Incomplete(TimedOut)`: `c::check` through the timed-out row, `d::check`\
    \ through its\n    cause `timeout` (FR-029-AC-33).\n\n## Implementation\n\nLauncher\
    \ stand-ins and real short-lived processes, in `src/kani/run/launch.rs`:\n`tc_043_a_capture_over_its_limit_is_refused_and_one_at_its_limit_is_whole`,\n\
    `tc_043_a_launcher_stream_over_the_limit_is_refused_and_one_at_the_limit_completes`,\n\
    `tc_043_a_batch_stream_is_bounded_by_the_limit_times_the_member_count` and\n`tc_043_an_over_limit_run_is_stopped_and_kills_the_launcher_group`\
    \ (step 1; FR-017-AC-14),\n`tc_043_a_capture_that_panics_or_whose_poll_or_read_errs_is_unread_not_empty`\
    \ and\n`tc_043_an_unread_stream_names_the_stream_in_the_launch_outcome` (step\
    \ 2; FR-017-AC-25), and\n`tc_043_a_launcher_that_exits_on_its_own_has_its_grandchild_killed`\
    \ (step 3; FR-017-AC-24).\n\nA shell-script launcher that records each process\
    \ it is started as and the arguments it received,\nin `src/kani/run/execute.rs`\
    \ (`batch_tests`): the process counts and argument vectors for N = 1, 10\nand\
    \ 50 compatible harnesses and for mixed option vectors and identity wall-clock\
    \ ceilings, the\n`--harness-timeout` and outer-bound arithmetic including `Duration::MAX`\
    \ and 4294967295 seconds, a\nbatch killed at its outer\nbound, both no-report\
    \ exits, the keyed split of a report whose results are in Kani's own order\nrather\
    \ than the request's (two members sharing the bare symbol `check`), the exit-status\n\
    predicate, the `timeout` entry (a sub-second T rounded up), the lacking, repeating\
    \ and unrequested\nharness, the member missing from the crate, the grouping of\
    \ requests that name another launcher,\ncrate directory or target directory, distinct\
    \ identity memory ceilings, the batch that runs longer\nthan T and inside N times\
    \ T, the playback attribution cases (including\n`tc_043_a_member_failing_two_checks_takes_its_first_block_and_keeps_its_neighbour`,\n\
    whose console is the playback section of a real Kani 0.68 batch, verbatim) and\
    \ the over-limit\nrefusal of a group by its member count (steps 4 to 9).\n`tc_043_a_playback_is_taken_by_the_path_it_is_headed_for`\
    \ in `src/kani/output/playback.rs` covers\nthe path-keyed scan.\n\nThe `make kani`\
    \ lane runs real Kani 0.68 in `tests/it/kani_batching.rs`: the launcher process\
    \ count\nand wall time before and after batching for N = 1, 10 and 50 (step 10),\
    \ and one real batch of a\nverified, a falsified, a two-failed-assertion and a\
    \ timed-out member (T = 20 s, after a warm-up\nbuild so that the outer bound never\
    \ covers a cold compile), then one at 4294967295 seconds (steps 5\nand 7). The\
    \ unit lane's batch reports are built in the shape the real batch report has\n\
    (`harness_id`, an `error_details` entry whose `exit_status` is `timeout` for a\
    \ member Kani cut off,\nresults in the order Kani ran them); no real batch report\
    \ is stored as a fixture.\n\nStep 11 (FR-017-AC-36) is covered by the tests of\
    \ `batch_tests::no_verdict_diagnostics` in\n`src/kani/run/execute.rs` (`tc_043_group_members_take_their_own_phase_and_share_the_groups_excerpts`,\n\
    `tc_043_a_group_report_decides_between_unknown_and_no_record`,\n`tc_043_stream_text_never_changes_the_classification`,\n\
    `tc_043_over_limit_and_unread_streams_stay_refused_with_no_record` and\n`tc_043_requests_with_different_home_directories_are_not_grouped`),\
    \ by the phase tests in\n`src/kani/output/diagnostics.rs`, and, for the FR-029\
    \ terminal value,\n`tc_043_the_terminal_value_of_no_verdict_evidence_ignores_its_diagnostics`\
    \ in\n`tests/it/kani_obligations.rs`.\n\nStep 12 (FR-017-AC-39) is covered by\n\
    `bounded_five_member_batch_keeps_classifications_and_agrees_on_timeouts` in\n\
    `tests/it/kani_terminal_private.rs`, which reuses `batch_failure_reports`, and\
    \ by\n`tc_043_two_entries_for_a_member_leave_the_timeout_rule_unchanged` in\n\
    `batch_tests::no_verdict_diagnostics::failure_cause` in `src/kani/run/execute.rs`.\n\
    \n## Status\n\nImplemented for FR-017 batching and output bounds (IR-277). Identity\
    \ memory-ceiling separation is\ncovered by `unequal_identity_memory_ceilings_run_in_separate_backend_processes`\
    \ and\n`unequal_memory_ceilings_plan_separate_groups_with_their_own_selections`\
    \ in\n`src/kani/run/execute.rs`; grouped aggregate memory exhaustion is covered\
    \ under TC-039\n(FR-028-AC-21).\n"
- id: AD-004-dependency-direction
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: "### Dependency direction\n\n```text\npublication --> core\nrouted    \
    \  --> replay, kani, strategy, oracle, core\nreplay      --> kani, core\nkani\
    \        --> oracle, core\nstrategy    --> oracle, core\nevidence    --> oracle,\
    \ core\noracle      --> core\ncore        --> (nothing in this crate)\nlib   \
    \      --> everything (re-exports only)\n```\n\nEvery arrow has a basis in code\
    \ or spec. `evidence --> oracle` is FR-004's `depends_on` FR-014 and\nthe `bound_coverage`\
    \ edge. `strategy --> oracle` is `bound_strategy` and `harness` calling oracle\n\
    generation. `kani --> oracle` is the scalar family embedding the FR-014 oracle.\
    \ `replay --> kani`\nis the identity and binding types the witness decode and\
    \ replay read. `routed --> ...` is the\n`BackendKind` match calling each generator.\n\
    \nRules, each checkable:\n\n- A directory imports only the directories to its\
    \ right. `strategy`, `evidence` and `kani` are peers\n  and import none of each\
    \ other. `kani` and `replay` import nothing from `routed`, and the adapter\n \
    \ trait, when it exists, is defined in the reserved `routed/adapter.rs` with the\
    \ Kani\n  implementation beside it. The driver\n  (`quire-driver`, outside this\
    \ crate) pairs a Kani outcome with a replay result, as QSL's merged\n  ADR-011\
    \ T-13 says; no module of this crate does.\n- Imports use a module path (`use\
    \ crate::core::artifact::Artifact`), never an item re-exported\n  from the crate\
    \ root. `lib.rs` is the only file that names a root re-export.\n- Inside `kani/`\
    \ the order is `abi`, `census`, `identity`, then `generate` and `output`, then\n\
    \  `classify`, `run`, `terminal`. A file imports only earlier names in that order.\
    \ `output` imports\n  `identity` and `abi` and nothing from `generate`, `classify`\
    \ or `run`. `run` imports `identity`,\n  `output` and `classify` (and `abi`, for\
    \ `KaniSolver`), not `generate`: the harness and identity\n  record types it needs\
    \ live in `kani/identity.rs` (step 2b), so a harness is run from its identity\n\
    \  and its source text.\n- Inside `kani/generate/` the order is `outcome`, `record`,\
    \ `spec`, `render`, `census_validation`, then the families\n  (`scalar` and `collection`,\
    \ `clause`, then `precondition` and `contract`, `frame`, `lower`,\n  `corpus`,\
    \ `v1_bundle`), then `negotiate`, which imports `outcome`, `record`, `scalar`,\n\
    \  `collection`, `clause`, `precondition`, `contract` and `frame` (the `StateFrame`\
    \ arm of step 4d\n  calls the frame family's role function; none of `lower`, `corpus`,\
    \ `v1_bundle` or\n  `census_validation`) and is imported by none. `collection`\
    \ imports `outcome` and `record` from\n  this directory, besides `core`, `kani::abi`\
    \ and `kani::identity`, and imports no other crate\n  path, so never `negotiate`,\
    \ `lower` or `corpus` either (the layout test lists exactly these\n  five prefixes\
    \ for `collection.rs`); `scalar` and `collection` share a rank, so neither imports\
    \ the other. `spec` imports `outcome` (for `MAX_OBLIGATION_UNWIND`), `core::identity`\
    \ (for `HarnessPath`) and `kani::abi`; `render` imports `spec`, `outcome`, `core::artifact`\
    \ and `kani::abi`, and not `record`; a family that has been ported (scalar at\
    \ 4b) imports `spec` and `render`. A family file may import `outcome` and `record`\
    \ and never `negotiate`: the passes that read the whole request\n  (classification,\
    \ name settlement, assumption resolution) are `negotiate`'s, and what lowers or\n\
    \  renders one item is its family's. `corpus` imports `lower` and `census_validation`;\
    \ `v1_bundle`\n  imports `census_validation`; `frame` imports `outcome` only.\
    \ `kani/test_support.rs` is\n  `#[cfg(test)]`, imports `identity` and `abi` and\
    \ no generator, and may be imported by any test\n  module under `kani/`.\n- A\
    \ `#[cfg(test)]` module obeys the same direction as the file it sits in. A test\
    \ that needs a\n  generator as a fixture lives in `tests/it/`, not in the runner's\
    \ file. The back-edge in the pair\n  `kani_execution` and `kani_transcript` (the\
    \ `kani_transcript` tests that import\n  `classify_kani_run`, `KaniInconclusiveReason`\
    \ and `KaniRunOutcome`) is not moved to `tests/it/`:\n  it needs no generator,\
    \ it is a classification test, and classification is a later file than the\n \
    \ report parse, so those tests move to `kani/classify.rs`, which may import `output`.\
    \ The tests\n  that need only the report parse stay with `output/report.rs`. The\
    \ step 2f item map decides each\n  test by name.\n- One interim exception to the\
    \ typed-values target, recorded as the code has it. `classify_kani_run`\n  takes\
    \ the console text beside the typed report, and `classify_report` calls\n  `counterexample_playback`\
    \ on it for a falsifying check, so `kani/classify.rs` imports\n  `kani/output/playback.rs`.\
    \ The Kani run table below records the target and today's reading side\n  by side.\
    \ The edge points in the allowed direction (`classify` is after `output`), so\
    \ no order rule\n  is broken; the exception is to L-6 (the classifier takes typed\
    \ values) and is listed there. It\n  holds before this AD, and step 2f keeps it\
    \ as it is. Step 5 removes it by changing the classifier's\n  input from the text\
    \ to the playback `output/playback.rs` extracts; that signature change is public\n\
    \  (`classify_kani_run` is `pub` so tests can reach the production classifier)\
    \ and is step 5's to\n  design.\n- `serde_json::Value` is named only in `core/ir`,\
    \ `core/canonical` and the report parser. No other\n  file reads a field of a\
    \ body term.\n- `BoundPackage` and `BoundClause` are named only in `strategy/`,\
    \ `evidence/`, `oracle/boolean_v1.rs`,\n  `oracle/bound_v1.rs` and the V1 arm\
    \ that step 4f deletes. After step 4f none is named in `kani/`,\n  `routed/` or\
    \ `replay/`.\n\n"
- id: src/kani/run/execute.rs
  path: src/kani/run/execute.rs
  role: examined
  excerpt: "    /// FR-029 reads from this evidence; FR-030 takes it as an explicit\
    \ input.\npub(crate) mod batch_tests {\n    pub(crate) mod no_verdict_diagnostics\
    \ {\n        pub(crate) mod failure_cause {\n            pub(crate) fn single_failure_reports(\n\
    \                mut observe: impl FnMut(&str, &KaniExecutionEvidence),\n    \
    \        ) {\n                observe(\"exit_code_64\", &evidence);\n        \
    \            observe(&status, &evidence);\n                }\n               \
    \ for (label, details) in [\n                    (\n                        \"\
    properties_failed\",\n                        Some(json!([entry_for(HARNESS, json!(\"\
    properties_failed\"))])),\n                    ),\n                    (\"absent\"\
    , None),\n                    (\"null\", Some(json!([entry_for(HARNESS, Value::Null)]))),\n\
    \                ] {\n                    fs::write(\n                       \
    \ stand_in.directory.join(\"report.json\"),\n                        document(Some(HARNESS),\
    \ \"Failure\", &[], details),\n                    )\n                    .unwrap();\n\
    \                    let evidence = run(&stand_in, &harness);\n              \
    \      assert_eq!(evidence.outcome, NO_COUNTEREXAMPLE, \"{label}\");\n       \
    \             if label == \"properties_failed\" {\n                        assert_eq!(\n\
    \                            evidence.failure_cause,\n                       \
    \     Some(KaniFailureCause::PropertiesFailed)\n                        );\n \
    \                   } else {\n                        assert_eq!(evidence.failure_cause,\
    \ None);\n                    }\n                    assert!(evidence.diagnostics.is_none());\n\
    \                    observe(label, &evidence);\n            pub(crate) fn unattributed_failure_reports(\n\
    \                mut observe: impl FnMut(&str, &KaniExecutionEvidence),\n    \
    \        ) {\n                observe(\"properties_failed\", &evidence);\n   \
    \                 observe(label, &evidence);\n            pub(crate) fn batch_failure_reports(mut\
    \ observe: impl FnMut(&[KaniExecutionEvidence])) {\n                observe(&evidence);\n\
    \            pub(crate) fn cause_independent_classification(\n               \
    \ mut observe: impl FnMut(&str, &KaniExecutionEvidence),\n            ) {\n  \
    \                  observe(status, &evidence);"
- id: src/kani/terminal.rs
  path: src/kani/terminal.rs
  role: examined
  excerpt: "//! total `match` each, from [`KaniExecutionEvidence`] or an IR [`KaniOutcome`],\
    \ paired with the\nuse crate::kani::output::report::KaniFailureCause;\n/// Maps\
    \ one run's evidence and, for a falsified outcome, its replay settlement to QSL's\n\
    /// terminal value (FR-029). Outcome, SUCCESS count and CBMC cause come from the\
    \ same evidence.\n/// A resource cause affects only failure-without-counterexample;\
    \ diagnostics are not read.\n    evidence: &KaniExecutionEvidence,\n    Ok(match\
    \ (&evidence.outcome, settlement) {\n            basis: ProofBasis::Checks {\n\
    \                success_checks: evidence.success_checks,\n            },\n  \
    \              reason: Reason::NoVerdict | Reason::MissingCoverSummary,\n    \
    \    (\n            Inconclusive {\n                reason: Reason::FailedWithoutCounterexample,\n\
    \            },\n            None,\n        ) => match evidence.failure_cause.as_ref()\
    \ {\n            Some(KaniFailureCause::OutOfMemory) => {\n                TerminalValue::Incomplete(IncompleteCause::ResourceExhausted)\n\
    \            }\n            Some(KaniFailureCause::Timeout) => TerminalValue::Incomplete(IncompleteCause::TimedOut),\n\
    \            None\n            | Some(KaniFailureCause::PropertiesFailed)\n  \
    \          | Some(KaniFailureCause::ExitCode { .. })\n            | Some(KaniFailureCause::Unknown\
    \ { .. }) => TerminalValue::Failed,\n        },"
- id: src/lib.rs
  path: src/lib.rs
  role: examined
  excerpt: '#[cfg(all(test, target_os = "linux"))]

    #[path = "../tests/it/kani_terminal_private.rs"]

    mod kani_terminal_private;

    '
- id: tests/it/kani_obligations.rs
  path: tests/it/kani_obligations.rs
  role: examined
  excerpt: "    let terminal = run_terminal_value(&evidence, None).unwrap();\n   \
    \ assert_eq!(terminal, qsl_replay::TerminalValue::Failed);\n/// Bounded launcher\
    \ stand-ins preserve failure classification while resource causes determine\n\
    /// the terminal value from the retained execution evidence.\n#[trace(\"FR-017-AC-40\"\
    , \"FR-029-AC-32\", \"FR-029-AC-33\", \"TC-027\")]\nfn cbmc_failure_terminal_value_reads_the_retained_cause()\
    \ {\n    for (status, expected) in [\n        (\"exit_code_64\", qsl_replay::TerminalValue::Failed),\n\
    \        (\n            \"out_of_memory\",\n            qsl_replay::TerminalValue::Incomplete(qsl_replay::IncompleteCause::ResourceExhausted),\n\
    \        ),\n        (\n            \"timeout\",\n            qsl_replay::TerminalValue::Incomplete(qsl_replay::IncompleteCause::TimedOut),\n\
    \        ),\n        (\"cbmc_exploded\", qsl_replay::TerminalValue::Failed),\n\
    \    ] {\n        let terminal = run_terminal_value(&evidence, None).unwrap();\n\
    \        assert_eq!(terminal, expected, \"{status}\");\n    assert_eq!(\n    \
    \    run_terminal_value(&evidence, None),\n        Ok(qsl_replay::TerminalValue::Failed)\n\
    \    );"
- id: tests/it/kani_obligations_state_clause_replay.rs
  path: tests/it/kani_obligations_state_clause_replay.rs
  role: examined
  excerpt: "        run_terminal_value(\n            &crate::terminal_map::synthetic_evidence(outcome.clone(),\
    \ 0),\n            Some((&result).into())\n        )\n        .expect(\"falsified\
    \ run maps\"),\n    let evidence = super::kani_obligations_state_frame::prove_evidence(&generated.postcondition);\n\
    \    let outcome = &evidence.outcome;\n        .decode_falsification(&generated.postcondition.identity,\
    \ outcome)\n        run_terminal_value(&evidence, Some((&result).into())).expect(\"\
    falsified run maps\"),"
- id: tests/it/kani_obligations_state_frame.rs
  path: tests/it/kani_obligations_state_frame.rs
  role: examined
  excerpt: "        &crate::terminal_map::synthetic_evidence(\n            KaniRunOutcome::Falsified\
    \ {\n                counterexample: \"model-type mismatch\".to_owned(),\n   \
    \         },\n            0,\n        ),\n        &crate::terminal_map::synthetic_evidence(\n\
    \            KaniRunOutcome::Falsified {\n                counterexample: \"model-type\
    \ mismatch\".to_owned(),\n            },\n            0,\n        ),\n    prove_evidence(harness).outcome\n\
    }\n\n/// Retains real run evidence for terminal projection after replay.\npub(crate)\
    \ fn prove_evidence(\n    harness: &StateFrameHarness,\n) -> quire_contract_codegen::KaniExecutionEvidence\
    \ {\n    evidence"
- id: tests/it/kani_terminal_private.rs
  path: tests/it/kani_terminal_private.rs
  role: examined
  excerpt: "//! Private orchestration: reuse bounded report stand-ins without reversing\
    \ kani layer imports.\nuse crate::kani::{\n    classify::{KaniInconclusiveReason,\
    \ KaniRunOutcome},\n    run::execute::batch_tests::no_verdict_diagnostics::failure_cause,\n\
    \    terminal::run_terminal_value,\n};\nuse ix_trace_rs::trace;\nuse qsl_replay::{Certification,\
    \ IncompleteCause, ProofBasis, TerminalValue};\n\n#[trace(\n    \"FR-017-AC-37\"\
    ,\n    \"FR-029-AC-32\",\n    \"FR-029-AC-33\",\n    \"FR-029-AC-34\",\n    \"\
    TC-027\",\n    \"TC-040\"\n)]\n#[test]\nfn bounded_single_reports_project_resource_causes_and_keep_other_failures()\
    \ {\n    failure_cause::single_failure_reports(|status, evidence| {\n        assert!(\n\
    \            evidence.checks.is_empty(),\n            \"fixture {status} has no\
    \ listed checks\"\n        );\n        let expected = match status {\n       \
    \     \"out_of_memory\" => TerminalValue::Incomplete(IncompleteCause::ResourceExhausted),\n\
    \            \"timeout\" => TerminalValue::Incomplete(IncompleteCause::TimedOut),\n\
    \            _ => TerminalValue::Failed,\n        };\n        assert_eq!(\n  \
    \          run_terminal_value(evidence, None),\n            Ok(expected.clone()),\n\
    \            \"fixture {status}\"\n        );\n        if status == \"out_of_memory\"\
    \ {\n            let mut memory_stop = evidence.clone();\n            memory_stop.outcome\
    \ = KaniRunOutcome::Inconclusive {\n                reason: KaniInconclusiveReason::MemoryExhausted,\n\
    \            };\n            memory_stop.failure_cause = None;\n            assert_eq!(run_terminal_value(&memory_stop,\
    \ None), Ok(expected));\n        }\n    });\n}\n\n#[trace(\"FR-017-AC-38\", \"\
    FR-029-AC-32\", \"TC-027\", \"TC-040\")]\n#[test]\nfn bounded_failed_check_reports_keep_both_absent_cause_shapes_failed()\
    \ {\n    failure_cause::unattributed_failure_reports(|label, evidence| {\n   \
    \     assert!(\n            !evidence.checks.is_empty(),\n            \"fixture\
    \ {label} lists a failed assertion\"\n        );\n        assert_eq!(\n      \
    \      run_terminal_value(evidence, None),\n            Ok(TerminalValue::Failed),\n\
    \            \"fixture {label}\"\n        );\n    });\n}\n\n#[trace(\"FR-017-AC-39\"\
    , \"FR-029-AC-33\", \"TC-043\", \"TC-040\")]\n#[test]\nfn bounded_five_member_batch_keeps_classifications_and_agrees_on_timeouts()\
    \ {\n    failure_cause::batch_failure_reports(|evidence| {\n        assert_eq!(\n\
    \            evidence\n                .iter()\n                .map(|member|\
    \ run_terminal_value(member, None).unwrap())\n                .collect::<Vec<_>>(),\n\
    \            [\n                TerminalValue::Proved {\n                    basis:\
    \ ProofBasis::Checks {\n                        success_checks: evidence[0].success_checks\n\
    \                    },\n                    certification: Certification::Certified\n\
    \                },\n                TerminalValue::Failed,\n                TerminalValue::Incomplete(IncompleteCause::TimedOut),\n\
    \                TerminalValue::Incomplete(IncompleteCause::TimedOut),\n     \
    \           TerminalValue::Failed,\n            ]\n        );\n        assert!(evidence[0].success_checks\
    \ > 0);\n    });\n}\n\n#[trace(\"FR-017-AC-40\", \"FR-029-AC-32\", \"FR-029-AC-35\"\
    , \"TC-027\", \"TC-040\")]\n#[test]\nfn bounded_cause_changes_only_terminal_projection_and_retains_evidence()\
    \ {\n    failure_cause::cause_independent_classification(|status, evidence| {\n\
    \        let before = evidence.clone();\n        let expected = match status {\n\
    \            \"out_of_memory\" => TerminalValue::Incomplete(IncompleteCause::ResourceExhausted),\n\
    \            \"exit_code_64\" | \"cbmc_exploded\" => TerminalValue::Failed,\n\
    \            _ => panic!(\"unexpected fixture {status}\"),\n        };\n     \
    \   assert_eq!(\n            run_terminal_value(evidence, None),\n           \
    \ Ok(expected),\n            \"fixture {status}\"\n        );\n        assert_eq!(evidence,\
    \ &before);\n    });\n}"
- id: tests/it/terminal_map.rs
  path: tests/it/terminal_map.rs
  role: examined
  excerpt: "    KaniExecutionEvidence, KaniFailureCause, KaniInconclusiveReason, KaniRunOutcome,\
    \ KaniSolver,\n    LockedSource, MemoryMechanism, MemoryObservation, ModelError,\
    \ ObligationIdentityError,\n    OperationDeclaration, PreStateFault, ProofCeilings,\
    \ ReplayInputs, ReplayPackage,\n    ReplayPackageError, ReplaySettlement, ReplayVerdict,\
    \ ScopeMember, SpineReplayError,\n    StateClauseReplay, StateClauseReplayError,\
    \ StateFrameRefusal, TerminalPairError,\n/// Explicitly synthetic evidence for\
    \ mapping tables, independent of launcher fixtures.\npub(crate) fn synthetic_evidence(\n\
    \    outcome: KaniRunOutcome,\n    success_checks: u32,\n) -> KaniExecutionEvidence\
    \ {\n    KaniExecutionEvidence {\n        ceilings: ProofCeilings {\n        \
    \    memory_bytes: std::num::NonZeroU64::new(1_073_741_824).unwrap(),\n      \
    \      wall_clock: std::time::Duration::from_secs(30),\n        },\n        memory:\
    \ MemoryObservation {\n            mechanism: MemoryMechanism::LinuxPidNamespaceProcfsTreeRss,\n\
    \            peak_resident_bytes: None,\n        },\n        symbolic_arguments:\
    \ Vec::new(),\n        kind: None,\n        harness_path: \"synthetic::mapping\"\
    .into(),\n        launcher_path: \"synthetic\".into(),\n        arguments: Vec::new(),\n\
    \        unwind: 4,\n        solver: KaniSolver::Cadical,\n        exit_code:\
    \ None,\n        outcome,\n        success_checks,\n        checks: Vec::new(),\n\
    \        batch: None,\n        diagnostics: None,\n        failure_cause: None,\n\
    \    }\n}\n\n            run_terminal_value(&synthetic_evidence(outcome.clone(),\
    \ 0), None),\n            run_terminal_value(\n                &synthetic_evidence(outcome.clone(),\
    \ 0),\n                Some(ReplaySettlement::Reproduced)\n            ),\n  \
    \  run_terminal_value(&synthetic_evidence(outcome.clone(), 3), settlement)\n \
    \       .expect(\"a well-paired outcome maps\")\n        run_terminal_value(&synthetic_evidence(falsified(),\
    \ 0), None),\n            run_terminal_value(\n                &synthetic_evidence(outcome.clone(),\
    \ 3),\n                Some(ReplaySettlement::Reproduced)\n            ),\n\n\
    /// Explicit columns: absent, properties_failed, timeout, out_of_memory, exit_code,\
    \ unknown.\n/// Matching the closed cause type makes an added variant a compile\
    \ error in this inventory.\nfn cause_column(cause: Option<&KaniFailureCause>)\
    \ -> usize {\n    match cause {\n        None => 0,\n        Some(KaniFailureCause::PropertiesFailed)\
    \ => 1,\n        Some(KaniFailureCause::Timeout) => 2,\n        Some(KaniFailureCause::OutOfMemory)\
    \ => 3,\n        Some(KaniFailureCause::ExitCode { .. }) => 4,\n        Some(KaniFailureCause::Unknown\
    \ { .. }) => 5,\n    }\n}\n\nfn cause_probes() -> Vec<Option<KaniFailureCause>>\
    \ {\n    vec![\n        None,\n        Some(KaniFailureCause::PropertiesFailed),\n\
    \        Some(KaniFailureCause::Timeout),\n        Some(KaniFailureCause::OutOfMemory),\n\
    \        Some(KaniFailureCause::ExitCode { code: i32::MIN }),\n        Some(KaniFailureCause::ExitCode\
    \ { code: 0 }),\n        Some(KaniFailureCause::ExitCode { code: 64 }),\n    \
    \    Some(KaniFailureCause::ExitCode { code: i32::MAX }),\n        Some(KaniFailureCause::Unknown\
    \ {\n            exit_status: \"timeout out_of_memory\".into(),\n        }),\n\
    \    ]\n}\n\n/// Independent literal specification rows; the compiler checks every\
    \ outcome and reason.\nfn ordinary_row(outcome: &KaniRunOutcome, count: u32) ->\
    \ Option<[TerminalValue; 6]> {\n    use KaniInconclusiveReason as R;\n    use\
    \ TerminalValue::{Failed, Incomplete};\n    let proved = |success_checks| TerminalValue::Proved\
    \ {\n        basis: ProofBasis::Checks { success_checks },\n        certification:\
    \ Certification::Certified,\n    };\n    let repeated = |value: TerminalValue|\
    \ std::array::from_fn(|_| value.clone());\n    Some(match outcome {\n        KaniRunOutcome::Verified\
    \ => repeated(proved(count)),\n        KaniRunOutcome::CoverUnsatisfied { .. }\
    \ => repeated(proved(0)),\n        KaniRunOutcome::Falsified { .. } => return\
    \ None,\n        KaniRunOutcome::Cancelled { source } => repeated(Incomplete(IncompleteCause::Cancelled\
    \ {\n            source: match source {\n                KaniCancellationSource::Requested\
    \ => CancelCause::Requested,\n                KaniCancellationSource::Deadline\
    \ => CancelCause::Deadline,\n            },\n        })),\n        KaniRunOutcome::Inconclusive\
    \ { reason } => match reason {\n            R::VacuousProof => repeated(proved(0)),\n\
    \            R::TimedOut => repeated(Incomplete(IncompleteCause::TimedOut)),\n\
    \            R::MemoryExhausted | R::UnwindBoundExhausted => {\n             \
    \   repeated(Incomplete(IncompleteCause::ResourceExhausted))\n            }\n\
    \            R::NoVerdict | R::MissingCoverSummary => repeated(Failed),\n    \
    \        R::FailedWithoutCounterexample => [\n                Failed,\n      \
    \          Failed,\n                Incomplete(IncompleteCause::TimedOut),\n \
    \               Incomplete(IncompleteCause::ResourceExhausted),\n            \
    \    Failed,\n                Failed,\n            ],\n        },\n    })\n}\n\
    \n/// Exhaust the closed replay vocabulary independently of the run-map implementation.\n\
    fn assert_settlement_row(evidence: &KaniExecutionEvidence, settlement: ReplaySettlement<'_>)\
    \ {\n    let expected = match &settlement {\n        ReplaySettlement::Reproduced\
    \ => TerminalValue::Refuted,\n        ReplaySettlement::Disagreement(cause) =>\
    \ {\n            TerminalValue::Inconclusive(InconclusiveCause::ReplayParity((*cause).clone()))\n\
    \        }\n        ReplaySettlement::Refused(refusal) => TerminalValue::from_replay_refusal(refusal),\n\
    \        ReplaySettlement::SetupRefused(code) => replayed(*code),\n        ReplaySettlement::Fault\
    \ | ReplaySettlement::CgDefect => TerminalValue::Failed,\n    };\n    assert_eq!(run_terminal_value(evidence,\
    \ Some(settlement)), Ok(expected));\n}\n\n/// Cross every closed outcome/reason\
    \ with every cause, payload probes and varied evidence counts.\n#[trace(\n   \
    \ \"FR-029-AC-32\",\n    \"FR-029-AC-33\",\n    \"FR-029-AC-34\",\n    \"FR-029-AC-35\"\
    ,\n    \"FR-029-AC-15\",\n    \"TC-040\"\n)]\n#[test]\nfn cause_matrix_preserves_literal_terminal_rows_and_pair_refusals()\
    \ {\n    let mut outcomes = settlement_free_outcomes();\n    outcomes.extend([\n\
    \        falsified(),\n        KaniRunOutcome::Cancelled {\n            source:\
    \ KaniCancellationSource::Requested,\n        },\n        KaniRunOutcome::Cancelled\
    \ {\n            source: KaniCancellationSource::Deadline,\n        },\n    ]);\n\
    \    // Compile the intentionally breaking root-exported evidence API too.\n \
    \   let map: fn(\n        &KaniExecutionEvidence,\n        Option<ReplaySettlement<'_>>,\n\
    \    ) -> Result<TerminalValue, TerminalPairError> = run_terminal_value;\n   \
    \ for count in [0, 7, 19] {\n        for outcome in &outcomes {\n            for\
    \ cause in cause_probes() {\n                let mut evidence = synthetic_evidence(outcome.clone(),\
    \ count);\n                evidence.failure_cause = cause;\n                let\
    \ before = evidence.clone();\n                if let Some(row) = ordinary_row(outcome,\
    \ count) {\n                    let value = map(&evidence, None).unwrap();\n \
    \                   assert_eq!(\n                        value,\n            \
    \            row[cause_column(evidence.failure_cause.as_ref())],\n           \
    \             \"{evidence:?}\"\n                    );\n                    if\
    \ matches!(\n                        outcome,\n                        KaniRunOutcome::CoverUnsatisfied\
    \ { .. }\n                            | KaniRunOutcome::Inconclusive {\n     \
    \                           reason: KaniInconclusiveReason::VacuousProof\n   \
    \                         }\n                    ) {\n                       \
    \ assert_eq!(value.category(), Category::Inconclusive);\n                    \
    \    assert_eq!(\n                            value.vacuous_proof_cause(),\n \
    \                           Some(ReportedInconclusiveCause::KaniVacuousProof)\n\
    \                        );\n                    }\n                    assert_eq!(\n\
    \                        map(&evidence, Some(ReplaySettlement::Reproduced)),\n\
    \                        Err(TerminalPairError::UnexpectedSettlement)\n      \
    \              );\n                    with_settlements(|settlement| {\n     \
    \                   assert_eq!(\n                            map(&evidence, Some(settlement)),\n\
    \                            Err(TerminalPairError::UnexpectedSettlement)\n  \
    \                      )\n                    });\n                } else {\n\
    \                    assert_eq!(\n                        map(&evidence, None),\n\
    \                        Err(TerminalPairError::MissingSettlement)\n         \
    \           );\n                    assert_eq!(\n                        map(&evidence,\
    \ Some(ReplaySettlement::Reproduced)),\n                        Ok(TerminalValue::Refuted)\n\
    \                    );\n                    with_settlements(|settlement| assert_settlement_row(&evidence,\
    \ settlement));\n                    for_each_fault(|settlement| {\n         \
    \               assert_eq!(map(&evidence, Some(settlement)), Ok(TerminalValue::Failed))\n\
    \                    });\n                    for_each_cg_failure(|settlement|\
    \ {\n                        assert_eq!(map(&evidence, Some(settlement)), Ok(TerminalValue::Failed))\n\
    \                    });\n                    for_each_setup_refusal(|code, settlement|\
    \ {\n                        assert_eq!(map(&evidence, Some(settlement)), Ok(replayed(code)))\n\
    \                    });\n                    let refusal = non_fault_refusal();\n\
    \                    assert_eq!(\n                        map(&evidence, Some(ReplaySettlement::Refused(&refusal))),\n\
    \                        Ok(replayed(refusal.code()))\n                    );\n\
    \                    for cause in [\n                        DisagreementCause::Verdicts\
    \ {\n                            proved: Verdict::from_category(Category::Violation),\n\
    \                            replayed: Verdict::from_category(Category::Success),\n\
    \                        },\n                        DisagreementCause::NoValue\
    \ {\n                            proved: Verdict::from_category(Category::Violation),\n\
    \                            replayed: Verdict::from_category(Category::Violation),\n\
    \                        },\n                        DisagreementCause::Witness\
    \ {\n                            proved: Verdict::from_category(Category::Violation),\n\
    \                            replayed: Verdict::from_category(Category::Violation),\n\
    \                            given: None,\n                            derived:\
    \ None,\n                            failure: WitnessFailure::Mismatch,\n    \
    \                    },\n                    ] {\n                        assert_eq!(\n\
    \                            map(&evidence, Some(ReplaySettlement::Disagreement(&cause))),\n\
    \                            Ok(TerminalValue::Inconclusive(\n               \
    \                 InconclusiveCause::ReplayParity(cause.clone())\n           \
    \                 ))\n                        );\n                    }\n    \
    \            }\n                assert_eq!(evidence, before, \"terminal projection\
    \ retains raw evidence\");\n            }\n        }\n    }\n}"
bindings:
- test_id: tests/it/kani_terminal_private.rs::bounded_single_reports_project_resource_causes_and_keep_other_failures
  ac_id: FR-017-AC-37
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::no_verdict_diagnostics::failure_cause::tc_027_the_cause_is_kept_for_no_other_outcome
  ac_id: FR-017-AC-38
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::no_verdict_diagnostics::failure_cause::tc_027_a_malformed_exit_status_still_refuses_the_report
  ac_id: FR-017-AC-38
  trace: correct
- test_id: tests/it/kani_terminal_private.rs::bounded_failed_check_reports_keep_both_absent_cause_shapes_failed
  ac_id: FR-017-AC-38
  trace: correct
- test_id: src/kani/run/execute.rs::batch_tests::no_verdict_diagnostics::failure_cause::tc_043_two_entries_for_a_member_leave_the_timeout_rule_unchanged
  ac_id: FR-017-AC-39
  trace: correct
- test_id: tests/it/kani_terminal_private.rs::bounded_five_member_batch_keeps_classifications_and_agrees_on_timeouts
  ac_id: FR-017-AC-39
  trace: correct
- test_id: tests/it/kani_obligations.rs::cbmc_failure_terminal_value_reads_the_retained_cause
  ac_id: FR-017-AC-40
  trace: correct
- test_id: tests/it/kani_obligations.rs::tc_027_real_kani_unknown_cbmc_option_keeps_exit_code_64_as_the_failure_cause
  ac_id: FR-017-AC-40
  trace: correct
- test_id: tests/it/kani_terminal_private.rs::bounded_cause_changes_only_terminal_projection_and_retains_evidence
  ac_id: FR-017-AC-40
  trace: correct
- test_id: tests/it/terminal_map.rs::tc_040_verified_proves_its_checks_and_a_reproduced_falsification_refutes
  ac_id: FR-029-AC-1
  trace: correct
- test_id: tests/it/terminal_map.rs::tc_040_a_vacuous_proof_and_an_unsatisfied_cover_are_proved_with_zero_checks
  ac_id: FR-029-AC-2
  trace: correct
- test_id: tests/it/terminal_map.rs::observed_memory_exhaustion_maps_to_incomplete_resource_exhausted
  ac_id: FR-029-AC-3
  trace: correct
- test_id: tests/it/terminal_map.rs::tc_040_a_run_with_no_verdict_is_failed
  ac_id: FR-029-AC-4
  trace: correct
- test_id: tests/it/terminal_map.rs::tc_040_a_missing_counterexample_or_cover_summary_is_failed
  ac_id: FR-029-AC-5
  trace: correct
- test_id: tests/it/terminal_map.rs::tc_040_no_outcome_maps_to_tested
  ac_id: FR-029-AC-6
  trace: correct
- test_id: tests/it/terminal_map.rs::tc_040_a_replay_disagreement_carries_its_cause
  ac_id: FR-029-AC-8
  trace: correct
- test_id: tests/it/terminal_map.rs::tc_040_a_replay_refusal_carries_its_code
  ac_id: FR-029-AC-9
  trace: correct
- test_id: tests/it/terminal_map.rs::tc_040_a_fault_in_any_replay_wrapper_is_failed
  ac_id: FR-029-AC-10
  trace: correct
- test_id: tests/it/terminal_map.rs::tc_040_each_failure_this_repository_raises_is_failed
  ac_id: FR-029-AC-11
  trace: correct
- test_id: tests/it/terminal_map.rs::tc_040_only_a_reproduced_replay_refutes
  ac_id: FR-029-AC-12
  trace: correct
- test_id: tests/it/terminal_map.rs::tc_040_a_setup_refusal_after_a_refutation_is_replay_refused_never_declined
  ac_id: FR-029-AC-13
  trace: correct
- test_id: tests/it/terminal_map.rs::tc_040_a_repeated_dependency_identity_is_replay_refused_with_invalid_package
  ac_id: FR-029-AC-14
  trace: correct
- test_id: tests/it/terminal_map.rs::direct_cancelled_run_maps_exact_source_and_rejects_settlement
  ac_id: FR-029-AC-15
  trace: correct
- test_id: tests/it/terminal_map.rs::tc_040_a_settlement_accompanies_a_falsified_outcome_only
  ac_id: FR-029-AC-15
  trace: correct
- test_id: tests/it/terminal_map.rs::cause_matrix_preserves_literal_terminal_rows_and_pair_refusals
  ac_id: FR-029-AC-15
  trace: correct
- test_id: tests/it/terminal_map.rs::tc_040_the_state_clause_replay_reads_as_fr029_ac16
  ac_id: FR-029-AC-16
  trace: correct
- test_id: tests/it/kani_obligations_state_clause_replay.rs::tc_040_out_of_range_debit_post_state_is_exact_and_refuted
  ac_id: FR-029-AC-18
  trace: correct
- test_id: tests/it/kani_obligations_state_clause_replay.rs::tc_040_real_kani_out_of_range_debit_replays_as_refuted
  ac_id: FR-029-AC-18
  trace: correct
- test_id: tests/it/terminal_map.rs::direct_cancelled_run_maps_exact_source_and_rejects_settlement
  ac_id: FR-029-AC-29
  trace: correct
- test_id: tests/it/kani_obligations.rs::cbmc_failure_terminal_value_reads_the_retained_cause
  ac_id: FR-029-AC-32
  trace: correct
- test_id: tests/it/kani_terminal_private.rs::bounded_single_reports_project_resource_causes_and_keep_other_failures
  ac_id: FR-029-AC-32
  trace: correct
- test_id: tests/it/kani_terminal_private.rs::bounded_failed_check_reports_keep_both_absent_cause_shapes_failed
  ac_id: FR-029-AC-32
  trace: correct
- test_id: tests/it/kani_terminal_private.rs::bounded_cause_changes_only_terminal_projection_and_retains_evidence
  ac_id: FR-029-AC-32
  trace: correct
- test_id: tests/it/terminal_map.rs::cause_matrix_preserves_literal_terminal_rows_and_pair_refusals
  ac_id: FR-029-AC-32
  trace: correct
- test_id: tests/it/kani_obligations.rs::cbmc_failure_terminal_value_reads_the_retained_cause
  ac_id: FR-029-AC-33
  trace: correct
- test_id: tests/it/kani_terminal_private.rs::bounded_single_reports_project_resource_causes_and_keep_other_failures
  ac_id: FR-029-AC-33
  trace: correct
- test_id: tests/it/kani_terminal_private.rs::bounded_five_member_batch_keeps_classifications_and_agrees_on_timeouts
  ac_id: FR-029-AC-33
  trace: correct
- test_id: tests/it/terminal_map.rs::cause_matrix_preserves_literal_terminal_rows_and_pair_refusals
  ac_id: FR-029-AC-33
  trace: correct
- test_id: tests/it/kani_terminal_private.rs::bounded_single_reports_project_resource_causes_and_keep_other_failures
  ac_id: FR-029-AC-34
  trace: correct
- test_id: tests/it/terminal_map.rs::cause_matrix_preserves_literal_terminal_rows_and_pair_refusals
  ac_id: FR-029-AC-34
  trace: correct
- test_id: tests/it/kani_terminal_private.rs::bounded_cause_changes_only_terminal_projection_and_retains_evidence
  ac_id: FR-029-AC-35
  trace: correct
- test_id: tests/it/terminal_map.rs::cause_matrix_preserves_literal_terminal_rows_and_pair_refusals
  ac_id: FR-029-AC-35
  trace: correct
```
