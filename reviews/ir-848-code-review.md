---
id: SR-12780
title: code-review review of IR-848 terminal-map specification
type: SpecReview
analysis: code-review
scope: agent-ix/quire-contract-codegen; spec/core/functional/interface-001-codegen-api.md,
  spec/kani/functional/FR-017-kani-execution-evidence.md, spec/kani/functional/FR-029-run-outcome-terminal-record.md,
  spec/kani/matrix/TC-027-kani-execution-evidence.md, spec/kani/matrix/TC-040-run-outcome-terminal-record.md,
  spec/kani/matrix/TC-043-kani-batching-and-output-bounds.md, spec/kani/matrix/tests.md
review_set: subset
date: '2026-10-18'
---

## Summary

Ticket: IR-848. Independent supplemental review of PR #422 frozen head f9b9325c0bb9ca4099a382ac2277ef7d1bbd582a, own diff against #418 at 4d1a1ede2b1795663dff459d5929f04745b1e9d9. The final reviewed patch remains spec-only; implemented behavior is explicitly deferred to the implementation ticket. No Cargo/Kani was run and no source files changed. Existing SR-12190..SR-12193 are preserved, not replaced. No applicable AssuranceProfile was found under spec/.

## Verdict

**PASS** — no new defects. The changed interface, FR-017 independence/classification wording, FR-029 terminal tables and criteria, and TC-027/040/043 procedures agree. The planned public input is evidence plus settlement, reading outcome, count and cause from the same evidence. Both absent-cause shapes remain Failed/CompletionRefusal; only the terminal map consumes resource causes. No copied implementation, new ownership duplication, suppressed check, or changed executable test is introduced. Current tests assert the prior value and the partial/planned notes name that limitation. This is a specification review, not an implementation-completion claim.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Scope examined

```yaml
scope:
- id: FR-017:L565
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: "- The generator shall compute the outcome, the checks, the SUCCESS-check\
    \ count, the playback and\n  every refusal exactly as the rules above state, before\
    \ and independently of the cause and the\n  diagnostic record. A `FailedWithoutCounterexample`\
    \ run stays\n  `FailedWithoutCounterexample` whatever its cause. Its FR-029 terminal\
    \ value is read from the cause\n  by the terminal map alone (IR-848): `out_of_memory`\
    \ gives `Incomplete(ResourceExhausted)`,\n  `timeout` gives `Incomplete(TimedOut)`,\
    \ and any other cause or none gives `Failed`. The cause\n  never feeds classification,\
    \ refusal or the batch tim"
- id: FR-017:L596
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'Rationale, not a requirement: two splits follow from keeping classification
    unchanged, and IR-848

    closes both in the FR-029 terminal map, which reads the cause. First, when a cgroup
    memory limit

    outside CG''s own meter kills only CBMC with `SIGKILL`, Kani reports `out_of_memory`
    and the run is

    `FailedWithoutCounterexample` with cause `out_of_memory`; the same exhaustion
    seen by CG''s own memory

    stop is `MemoryExhausted`. Both now have the FR-029 terminal value `Incomplete(ResourceExhausted)`.

    Kani reports any `SIGKILL` of CBMC as `out_of_memory`, and FR-029 accepts it without
    corroboration.

    Seco'
- id: FR-017:L659
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: 'For `ProofNotVerified`, `class()` is `CompletionRefusal` when the evidence
    outcome is a genuine

    property or proof verdict, or a no-cause failure with no checks (no verdict evidence,
    classed as refusal by rule): `Falsified`, `CoverUnsatisfied`, or `Inconclusive`
    with reason

    `MissingCoverSummary`, `VacuousProof`, `UnwindBoundExhausted`, or `FailedWithoutCounterexample`

    with `failureCause` `properties_failed` (a failed property, which real Kani writes
    for a failed assertion without playback), or with no `failureCause` in either
    of two shapes: a failed check listed with no exit status stated (a fa'
- id: FR-017-AC-40
  path: spec/kani/functional/FR-017-kani-execution-evidence.md
  role: examined
  excerpt: '| FR-017-AC-40 | The cause never feeds classification: three single-run
    stand-ins with the same `Failure` report, no checks and the same exit, whose entries
    give `exit_code_64`, `out_of_memory` and `cbmc_exploded` and whose streams differ,
    classify to the same outcome, checks, SUCCESS count and playback, and their evidence
    is equal in every field but `failureCause`, `diagnostics`, `memory` and the report-path
    argument; the cause is the only input of the FR-029 terminal value that differs:
    `exit_code_64` and `cbmc_exploded` give `Failed`, equal to the terminal value
    of the same evidence with `f'
- id: FR-029:L78
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: "- Planned (IR-806): for the composite parity route, the proof subject\
    \ (`production` or\n  `bounded_shadow`, FR-028), the verified outcome's proof\
    \ strength (FR-028-AC-17) and retained\n  `refinement_failed` disagreement whatever\
    \ the backend outcome. `run_terminal_value` takes none of\n  these; its inputs\
    \ are the run's evidence (outcome, SUCCESS-check count, cause) and the optional\n\
    \  replay settlement."
- id: FR-029:L83
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: "- Planned (IR-848): the CBMC failure cause. It is not a free input: `run_terminal_value`\
    \ takes the\n  run's evidence, `&KaniExecutionEvidence`, and reads the outcome\
    \ and the `failureCause` from it, so\n  a caller cannot omit the cause of a run\
    \ that has one. The planned signature is\n  `run_terminal_value(evidence: &KaniExecutionEvidence,\
    \ settlement)`. It reads the outcome, the\n  SUCCESS-check count (`KaniExecutionEvidence.success_checks`)\
    \ and the cause from the evidence; no\n  separate count parameter remains. The\
    \ cause is\n  that FR-017 retains in the evidence of a `FailedWithoutCounterexample`\
    \ ru"
- id: FR-029:L120
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: "- The Kani adapter shall map run outcomes in exactly one function, public\
    \ so the driver calls it,\n  whose `match` over the pair (`KaniRunOutcome` with\
    \ `KaniInconclusiveReason`, replay settlement)\n  has no wildcard arm. When the\
    \ reason is failure-without-counterexample, the map shall match the\n  `KaniFailureCause`\
    \ (or its absence) exhaustively too, with no wildcard arm, so a new cause is a\n\
    \  compile error in the map (the type is serialize-only and its FR-017 cause set\
    \ is closed)."
- id: FR-029:L142
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: "  | Outcome | Result |\n  |---|---|\n  | `inconclusive` with the timed-out\
    \ reason | `Incomplete(IncompleteCause::TimedOut)` |\n  | `inconclusive` with\
    \ the memory-exhausted reason | `Incomplete(IncompleteCause::ResourceExhausted)`\
    \ |\n  | `inconclusive` with the exhausted-unwind-bound reason | `Incomplete(IncompleteCause::ResourceExhausted)`\
    \ |\n  | `inconclusive` with the no-verdict reason | `Failed` |\n  | `inconclusive`\
    \ with the failure-without-counterexample reason, cause `out_of_memory` | `Incomplete(IncompleteCause::ResourceExhausted)`\
    \ |\n  | `inconclusive` with the failure-without-counterexample "
- id: FR-029:L164
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: "- A missing cover summary maps to `Failed`: under the option vector every\
    \ harness identity records,\n  the backend prints a cover summary for every run,\
    \ so its omission is the tool breaking its output\n  contract, not a verdict on\
    \ the property."
- id: FR-029:L167
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: "- A failure without a counterexample is read through the CBMC failure\
    \ cause FR-017 retains (IR-848).\n  The cause `out_of_memory` and the cause `timeout`\
    \ each state that CBMC was stopped by a resource\n  limit, so the run is incomplete,\
    \ not broken: `Incomplete(ResourceExhausted)` and\n  `Incomplete(TimedOut)`. The\
    \ same exhaustion that CG's own memory stop sees is `MemoryExhausted`\n  and maps\
    \ to `Incomplete(ResourceExhausted)`, so one physical event has one terminal value\
    \ whoever\n  observed it. Classification is unchanged: the run stays `FailedWithoutCounterexample`."
- id: FR-029:L173
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: "- The cause `properties_failed` is `Failed` because Kani reported a failed\
    \ property and the backend\n  printed no playback: a genuine failed-property verdict\
    \ that CG cannot replay, the reading\n  FR-017's failure-class rule takes (class\
    \ `CompletionRefusal`)."
- id: FR-029:L176
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: "- No cause at all has two shapes, both `Failed`, for different reasons,\
    \ and both with FR-017 class\n  `CompletionRefusal`. (a) A failed check is listed\
    \ and the entry states no exit status\n  (FR-017-AC-38's no-cause shapes): a genuine\
    \ failed-property verdict that CG cannot replay.\n  (b) No checks and no exit\
    \ status (FR-029-AC-32's case): no evidence of a verdict or of a resource\n  stop,\
    \ so the tool broke its output contract."
- id: FR-029:L181
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: "- The cause `exit_code` or `unknown` states a CBMC failure that is neither\
    \ a resource stop nor a\n  failed property: FR-017's class is `EnvironmentFault`,\
    \ and the terminal value is `Failed`, the\n  tool breaking its output contract.\
    \ That `Failed` is stated, not defaulted: it keeps a CBMC crash\n  loud."
- id: FR-029:L185
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: "- Rationale, not a requirement: Kani reports any `SIGKILL` of CBMC as\
    \ `out_of_memory`, whether the\n  kernel OOM killer, a cgroup limit or an operator\
    \ sent it. This requirement accepts that signal as\n  `ResourceExhausted` without\
    \ corroboration, as a known limit: an operator kill is read as\n  exhaustion.\
    \ No verdict is claimed, nothing becomes `Proved` or `Refuted`, and the raw cause\
    \ stays\n  in the evidence."
- id: FR-029:L190
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: "- The two timeout paths agree on the terminal value and differ in classification.\
    \ A batch member\n  with no checks and exit status `timeout` is classified `TimedOut`\
    \ (FR-017-AC-22) and maps to\n  `Incomplete(TimedOut)` through the timed-out row.\
    \ A single run, which reads no exit status for\n  classification, is `FailedWithoutCounterexample`\
    \ with cause `timeout` and reaches\n  `Incomplete(TimedOut)` through the cause.\
    \ A batch member whose `timeout` entry lists a failed\n  check with no playback\
    \ (FR-017-AC-39's `d::check`) stays `FailedWithoutCounterexample` with cause\n\
    \  `timeout`, and its terminal v"
- id: FR-029:L253
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'In the absence of a retained refinement disagreement, every `bounded_shadow`
    inconclusive backend

    outcome takes the ordinary inconclusive rows, without a proof strength or a completed
    parity

    settlement: vacuous proof and cover-unsatisfied give the vacuous record; timed-out
    gives

    `Incomplete(TimedOut)`; memory-exhausted and unwind-bound-exhausted give

    `Incomplete(ResourceExhausted)`; a failure without a counterexample takes the
    cause rows

    (`out_of_memory` gives `Incomplete(ResourceExhausted)`, `timeout` gives `Incomplete(TimedOut)`,
    any

    other cause or none gives `Failed`); no-verdict and missin'
- id: FR-029-AC-5
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '| FR-029-AC-5 | The missing-cover-summary reason maps to `Failed`. The
    failure-without-counterexample reason maps by its cause (FR-029-AC-32 to FR-029-AC-35).
    | Test (TC-040) |'
- id: FR-029-AC-25
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '| FR-029-AC-25 | PLANNED (IR-806). With no retained refinement disagreement,
    every bounded_shadow inconclusive reason maps through the ordinary rows: vacuous
    proof and cover-unsatisfied are Proved0; timeout is Incomplete(TimedOut); memory
    and unwind exhaustion are Incomplete(ResourceExhausted); no-verdict and missing-summary
    are Failed; failure-without-counterexample takes the cause rows (FR-029-AC-32
    to FR-029-AC-35). No vacuous outcome is coerced into verified evidence. | Test
    |'
- id: FR-029-AC-32
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '| FR-029-AC-32 | PLANNED (IR-848). A `FailedWithoutCounterexample` run
    with cause `out_of_memory` maps to `Incomplete(ResourceExhausted)`, and the same
    reason with cause `properties_failed`, `exit_code` (any code), `unknown` or no
    cause maps to `Failed`. Over a single-run stand-in that exits 1 and exports a
    `Failure` report for `proofs::check` with no checks and one `error_details` entry
    with exit status `out_of_memory`, the evidence passed to `run_terminal_value`
    gives `Incomplete(ResourceExhausted)`; the same report with exit status `exit_code_64`,
    `cbmc_exploded`, `properties_failed`, a nul'
- id: FR-029-AC-33
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '| FR-029-AC-33 | PLANNED (IR-848). A `FailedWithoutCounterexample` run
    with cause `timeout` maps to `Incomplete(TimedOut)`: the single-run stand-in of
    FR-029-AC-32 with exit status `timeout` gives `Incomplete(TimedOut)`. The batch
    member `c::check` of FR-017-AC-39 (a `Failure` with no checks and exit status
    `timeout`, classified `TimedOut`) keeps `Incomplete(TimedOut)` through the timed-out
    row, and the member `d::check` (a `Failure` listing a failed check with no playback,
    exit status `timeout`, classified `FailedWithoutCounterexample` with cause `timeout`)
    gives `Incomplete(TimedOut)` throug'
- id: FR-029-AC-34
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '| FR-029-AC-34 | PLANNED (IR-848). CG''s own memory stop (the memory-exhausted
    reason) and a `FailedWithoutCounterexample` run with cause `out_of_memory` give
    the same terminal value, `Incomplete(ResourceExhausted)`, so one exhaustion has
    one value whoever observed it. The cause is read for the failure-without-counterexample
    reason only: every other outcome and reason maps identically whatever cause is
    passed, including a cause passed with `NoVerdict` or `MissingCoverSummary`. |
    Test (TC-040) |'
- id: FR-029-AC-35
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '| FR-029-AC-35 | PLANNED (IR-848). The cause never creates a verdict.
    Over every combination of outcome, reason and cause (absent, `properties_failed`,
    `timeout`, `out_of_memory`, `exit_code`, `unknown`), and with the replay settlement
    each outcome takes: a `falsified` outcome with a reproduced replay is `Refuted`
    whatever the cause; the cause never turns a non-`Refuted` result into `Refuted`,
    and never turns any result into `Proved` (`Proved` appears only for the rows FR-029-AC-1
    and FR-029-AC-2 give). The `NoVerdict` and `MissingCoverSummary` reasons map to
    `Failed` whatever the cause. Class'
- id: FR-029-AC-36
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: '| FR-029-AC-36 | PLANNED (IR-848). The rationale of this requirement states
    that Kani reports any `SIGKILL` of CBMC as `out_of_memory`, that an `out_of_memory`
    cause is accepted as `Incomplete(ResourceExhausted)` without corroboration, and
    that the raw cause stays in the evidence. | Inspection |'
- id: FR-029:L426
  path: spec/kani/functional/FR-029-run-outcome-terminal-record.md
  role: examined
  excerpt: 'Planned (IR-848): the evidence input and FR-029-AC-32 to FR-029-AC-36.
    FR-030 needs no row: Contract IR

    `KaniOutcomeKind::ResourceExhausted` and `TimedOut` already map to `Incomplete`,
    and the IR outcome has no

    failure-without-counterexample kind.'
- id: TC-027:L227
  path: spec/kani/matrix/TC-027-kani-execution-evidence.md
  role: examined
  excerpt: 'CBMC failure cause (FR-017-AC-37, FR-017-AC-38, FR-017-AC-40): every exit-status
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

    value is kept whole, the 65-byte and 100-byte values keep their '
- id: TC-027:L420
  path: spec/kani/matrix/TC-027-kani-execution-evidence.md
  role: examined
  excerpt: 'The CBMC failure cause (FR-017-AC-37, FR-017-AC-38, FR-017-AC-40) is covered
    by the stand-in tests

    in `batch_tests::no_verdict_diagnostics::failure_cause` in `src/kani/run/execute.rs`

    (`tc_027_every_exit_status_gives_its_cause_and_a_diagnostic_record`,

    `tc_027_a_failed_property_has_no_record_and_an_unnamed_entry_has_no_cause`,

    `tc_027_the_cause_is_kept_for_no_other_outcome`,

    `tc_027_a_malformed_exit_status_still_refuses_the_report` and

    `tc_027_the_cause_never_feeds_classification`) and, for the FR-029 terminal value,

    `tc_027_the_terminal_value_of_a_cbmc_failure_ignores_its_cause` in `tests/it/'
- id: TC-040:L20
  path: spec/kani/matrix/TC-040-run-outcome-terminal-record.md
  role: examined
  excerpt: "1. Map `verified` with three SUCCESS checks, and `falsified` with a reproduced\
    \ replay.\n2. Map `inconclusive` with the vacuous-proof reason, and `cover-unsatisfied`.\
    \ Read each value's QSL\n   category and vacuity cause.\n3. Map `inconclusive`\
    \ with the timed-out, memory-exhausted and exhausted-unwind-bound reasons.\n4.\
    \ Map `inconclusive` with the no-verdict reason.\n5. Map `inconclusive` with the\
    \ missing-cover-summary reason, and with the failure-without-counterexample\n\
    \   reason once per cause: absent, `properties_failed`, `timeout`, `out_of_memory`,\
    \ `exit_code`\n   and `unknown`. Map the memory-exha"
- id: TC-040:L68
  path: spec/kani/matrix/TC-040-run-outcome-terminal-record.md
  role: examined
  excerpt: "1. `Proved { success_checks: 3 }` and `Refuted` (FR-029-AC-1).\n2. Both\
    \ are `Proved { success_checks: 0 }`, category `inconclusive`, cause `KaniVacuousProof`\n\
    \   (FR-029-AC-2).\n3. `Incomplete(TimedOut)`, `Incomplete(ResourceExhausted)`\
    \ and `Incomplete(ResourceExhausted)`\n   (FR-029-AC-3).\n4. `Failed` (FR-029-AC-4).\n\
    5. Missing-cover-summary is `Failed` (FR-029-AC-5). The failure-without-counterexample\
    \ reason is\n   `Incomplete(ResourceExhausted)` for `out_of_memory` and `Incomplete(TimedOut)`\
    \ for `timeout`, and\n   `Failed` for the other causes and for none (FR-029-AC-32,\
    \ FR-029-AC-33). The memory-"
- id: TC-040:L117
  path: spec/kani/matrix/TC-040-run-outcome-terminal-record.md
  role: examined
  excerpt: 'Planned (IR-848): the cause arguments of step 5 and FR-029-AC-32 to FR-029-AC-35.'
- id: TC-043:L23
  path: spec/kani/matrix/TC-043-kani-batching-and-output-bounds.md
  role: examined
  excerpt: "1. Drive a launcher stand-in that prints more than 8 MiB on stdout, then\
    \ on stderr; one that prints\n   exactly 8 MiB; and a batch stand-in of N members\
    \ that prints more than N times 8 MiB.\n2. Drive a capture whose reader thread\
    \ panics, and one whose pipe read returns an error.\n3. Run a launcher that exits\
    \ on its own, leaving a real grandchild in its process group.\n4. Build N = 1,\
    \ 10 and 50 harnesses with equal option vectors and identity `ProofCeilings`\n\
    \   (`wall_clock` T and `memory_bytes` M), and read the launch's argument vector\
    \ and the number of\n   launcher processes a stand-in counts; mix"
- id: TC-043:L70
  path: spec/kani/matrix/TC-043-kani-batching-and-output-bounds.md
  role: examined
  excerpt: "1. Each over-limit stream is refused with `kani_output_over_limit` naming\
    \ the stream, the limit and\n   the harness count, the group is killed, and nothing\
    \ is classified; exactly the limit completes\n   with its real exit status and\
    \ the whole stream (FR-017-AC-14).\n2. Both are refused with `kani_output_unread`\
    \ and no outcome (FR-017-AC-25).\n3. The grandchild is killed by the time the\
    \ run returns (FR-017-AC-24).\n4. One process for each group, with one `--harness\
    \ <module::harness> --exact` pair per member in\n   request order, `--harness-timeout`\
    \ T and then the shared options; fewer than N processe"
- id: TM-004:L47
  path: spec/kani/matrix/tests.md
  role: examined
  excerpt: '| FR-017 | FR-017-AC-37, FR-017-AC-38 | TC-027 | ✅ Covered (IR-827); the
    typed CBMC failure cause and diagnostic record of `FailedWithoutCounterexample`
    evidence, its independence from classification and the `make kani` lane run with
    an unknown CBMC option, asserted against launcher stand-ins and in the `make kani`
    lane against real Kani |'
- id: TM-004:L49
  path: spec/kani/matrix/tests.md
  role: examined
  excerpt: '| FR-017 | FR-017-AC-40 | TC-027, TC-040 | 🚧 Partial (IR-848); the classification-independence
    half is covered (IR-827) in `tests/it/kani_obligations.rs`, but the terminal-value
    half changes: the bound test `tc_027_the_terminal_value_of_a_cbmc_failure_ignores_its_cause`
    (`kani_obligations.rs:2326-2378`) asserts the old value `Failed` for every cause
    and flips in the code PR |'
- id: TM-004:L61
  path: spec/kani/matrix/tests.md
  role: examined
  excerpt: '| FR-029 | FR-029-AC-32 through FR-029-AC-36 | TC-040 (AC-33 also TC-043;
    AC-36 is Inspection) | 🚧 Planned (IR-848); `run_terminal_value` takes no evidence
    or failure cause yet, so the FWC cause rows, the `MemoryExhausted`/`out_of_memory`
    parity and the batch `c::check`/`d::check` cases have no test |'
- id: TM-004:L106
  path: spec/kani/matrix/tests.md
  role: examined
  excerpt: '| TC-027 | Verify Kani obligation execution and its evidence | Analysis
    | P0 | FR-017-AC-37, FR-017-AC-38 | ✅ Covered (IR-827) |'
- id: TM-004:L107
  path: spec/kani/matrix/tests.md
  role: examined
  excerpt: '| TC-027 | Verify Kani obligation execution and its evidence | Analysis
    | P0 | FR-017-AC-40 | 🚧 Partial (IR-848); see the FR-017 row |'
- id: TM-004:L116
  path: spec/kani/matrix/tests.md
  role: examined
  excerpt: '| TC-040 | Verify the total map from a Kani run outcome to QSL''s terminal
    value | Integration | P0 | FR-029-AC-1, FR-029-AC-2, FR-029-AC-3, FR-029-AC-4,
    FR-029-AC-5, FR-029-AC-6, FR-029-AC-8, FR-029-AC-9, FR-029-AC-10, FR-029-AC-11,
    FR-029-AC-12, FR-029-AC-13, FR-029-AC-14, FR-029-AC-15, FR-029-AC-16, FR-029-AC-18,
    FR-029-AC-28, FR-029-AC-32, FR-029-AC-33, FR-029-AC-34, FR-029-AC-35, FR-017-AC-40
    | 🚧 Planned; ordinary criteria including FR-029-AC-18 are covered as the Functional
    Requirement Coverage table records; AC-28 is covered by the IR-666 converter |'
- id: interface_001:L149
  path: spec/core/functional/interface-001-codegen-api.md
  role: examined
  excerpt: '    inputs: [the run''s KaniExecutionEvidence (IR-848, FR-029: the outcome,
    the SUCCESS-check count and the failureCause are read from it, so the cause cannot
    be omitted), an optional ReplaySettlement (reproduced | disagreement carrying
    QSL''s DisagreementCause | refused carrying a ReplayRefusal | setup refusal carrying
    a QSL Code | fault | CG defect), which the From impls of SpineReplayError, ReplayPackageError,
    FrameReplayError, StateClauseReplayError, DependencyLockError, EvidenceFailureCause,
    ReplayVerdict and the QSL CallSiteRefusal build; the EvidenceFailureCause impl
    reads a decode failur'
- id: interface_001:L353
  path: spec/core/functional/interface-001-codegen-api.md
  role: examined
  excerpt: '  failed_without_counterexample_cause: (FR-017-AC-37 to FR-017-AC-40)
    the evidence of an `inconclusive`/`failed_without_counterexample` run, and only
    that evidence, carries `failureCause` when the report''s one `error_details` entry
    naming the harness states an `exit_status`; `failureCause` is a `KaniFailureCause`,
    serialized as `{ kind }` with `kind` one of `properties_failed`, `timeout` and
    `out_of_memory`, as `{ kind: "exit_code", code }` for `exit_code_<n>` with n a
    canonical signed 32-bit decimal, and as `{ kind: "unknown", exitStatus }` (escape-stripped,
    redacted like an excerpt, at most '
```

## Method selection and limitations

Only code-review and EARS were missing in the original round. Existing base, integrity, failure-domain and evidence reviews retain their original artifacts and fourteen fixed findings. Rust, production gap analysis, runtime tests and aggregate gates do not apply to this Markdown-only delta. No new relationships or domain-object definitions are added. Review history is context, not an executable requirement; historical scope/version references are preserved as records. EARS engine receipt: ears-engine.log adjacent to this review directory.
