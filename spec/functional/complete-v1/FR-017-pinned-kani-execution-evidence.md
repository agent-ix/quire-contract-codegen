---
id: FR-017
title: "Run one bounded Kani obligation under the committed pins and retain its evidence"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/interface-001
    type: implements
  - target: ix://agent-ix/quire-specification/FR-196
    type: references
---
# FR-017: Run one bounded Kani obligation under the committed pins and retain its evidence

## Description

When a caller runs an FR-015 harness, the code generator shall measure the
installed Kani backend, refuse the run unless both the harness identity and the
installed backend are the committed pins, invoke the backend, and retain the
backend's own reported outcome as typed execution evidence.

This is the surface that turns a generated harness into an assurance claim, and
it is separate from FR-015 for that reason. FR-015 emits harnesses and typed
refusals and asserts nothing about whether one ever ran; this requirement owns
the run and everything read back from it. It is the execution half of issue #49,
and it is stated here because `src/kani_execution.rs` had no owning requirement
(issue #55).

## Inputs

- One FR-015 harness, whose identity carries the backend pins the run is held
  to, the option vector to invoke, the unwind bound and the solver.
- A Kani installation: the `cargo-kani` launcher to invoke, and the Kani home
  holding the release whose driver, CBMC and toolchain are measured.
- The crate directory whose library source contains that harness's generated
  source byte for byte, and the Cargo target directory the run builds into.

## Outputs

- Execution evidence identifying its own schema, naming the harness that ran,
  the backend measured immediately before it ran, the exact invocation, and the
  backend-reported outcome.
- A typed refusal, and no run, when the backend cannot be measured, when any
  pin differs, or when the crate does not contain the harness.

## Behavior

- The generator shall measure every pin from the installed files and processes
  rather than from a declaration: the launcher and driver executable digests
  from their bytes, the Kani and CBMC versions and the host target triple from
  the programs' own output, and the Rust toolchain from the release's recorded
  toolchain.
- When a run is requested, the generator shall compare both the harness
  identity's pins and the measured pins with the committed pins, field by
  field, before it starts any process.
- If any of those twelve comparisons differs, then the generator shall refuse
  the run with a typed reason naming the first differing field with its
  expected and observed values, and shall start no process.
- If any backend component is absent, unreadable, exits unsuccessfully, or
  prints output the measurement cannot read, then the generator shall refuse
  with a typed reason naming that component and its path, and shall run
  nothing.
- If the crate's library source does not contain the harness's generated source
  byte for byte, then the generator shall refuse, because evidence about a
  harness the crate does not contain is evidence about nothing.
- The generator shall classify a run as verified only when the process exited
  successfully, the backend reported success, and a readable cover summary
  reports every cover property satisfied. A run is never defaulted to verified:
  a harness this generator did not observe verifying is not verified.
- If the backend reported success but its covers are not all satisfied, then
  the generator shall classify the run as cover-unsatisfied with the satisfied
  and total counts, because the FR-015 covers are what separate a proof from a
  vacuous run.
- If the backend reported a failed check that is not an unwinding assertion and
  printed a concrete playback for it, then the generator shall classify the run
  as falsified and retain that playback verbatim as the counterexample. A
  playback printed for a satisfied cover witnesses reachability and shall never
  be taken as a counterexample.
- If the backend reported a failed unwinding assertion, then the
  generator shall classify the run as inconclusive with the
  exhausted-loop-bound reason instead of falsified.
- Where a run establishes nothing, the generator shall classify it as
  inconclusive with the reason that applies: no verdict was printed, a failure
  carried no counterexample, or no readable non-empty cover summary was printed
  so non-vacuity was not observed.
- The generator shall retain, in the evidence, the harness identity digest and
  obligation kind, the harness path and source digest, the pins measured
  immediately before the run, the invoked launcher path, the complete argument
  vector, the generated crate's lockfile digest, the oracle digest, the runtime
  revision, the unwind bound, the solver, the process exit code and the
  outcome. The argument vector after the `kani` subcommand shall be the harness
  identity's option vector unchanged, so the evidence cannot claim an
  invocation the harness did not specify.
- The generator shall compute no aggregate verdict over runs.
- The generator shall retain no evidence of its own, because retention, audit
  and attestation are Quoin's.

## Constraints

| ID | Constraint | Type | Validation |
|----|------------|------|------------|
| FR-017-CON-1 | The generator SHALL NOT convert a non-verified outcome into a proof claim. | Integrity | Test (TC-027) |
| FR-017-CON-2 | The generator SHALL NOT report a generation-time classification as an execution outcome. | Integrity | Test (TC-027) |

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-017-AC-1 | A harness identity or an installed backend differing from the committed pins in any of the six fields is refused with a typed reason naming that field with its expected and observed values, the backend is never invoked, and no target directory is created. | Test (TC-027) |
| FR-017-AC-2 | An absent or unmeasurable backend component is refused with a typed reason naming the component and its path before anything runs. | Test (TC-027) |
| FR-017-AC-3 | The committed pins are the Kani version, launcher digest, driver digest, CBMC version, Rust toolchain and host target triple of one installation, and a difference in each of the six is reported as that field. | Test (TC-027) |
| FR-017-AC-4 | A run is verified only when the process exited successfully, the backend reported success, and every cover property is reported satisfied; a successful run with an unsatisfied cover is cover-unsatisfied with its satisfied and total counts; success text from an unsuccessfully exited process, an absent cover summary, a zero-total summary and an unreadable summary are each inconclusive with their own reason. | Test (TC-027) |
| FR-017-AC-5 | A failed non-unwinding check with a concrete playback is falsified carrying that playback verbatim and never the playback of a satisfied cover; a failure with no playback is inconclusive for that reason; a failed unwinding assertion is inconclusive as an exhausted bound rather than falsified, and a succeeded unwinding check in a results listing is not a failure. | Test (TC-027) |
| FR-017-AC-6 | Execution evidence identifies its schema and carries the harness identity digest, obligation kind, harness path and source digest, the pins measured immediately before the run, the launcher path, the complete argument vector, the crate lockfile digest when the lockfile was readable after the run, the oracle digest, the runtime revision, the unwind bound, the solver, the exit code and the outcome. | Test (TC-027) |
| FR-017-AC-7 | A crate whose library source does not contain the harness source byte for byte is refused, and no backend runs. | Test (TC-027) |
| FR-017-AC-8 | The generator computes no aggregate verdict over runs: no function in the execution surface accepts more than one run's evidence or outcome to produce a summary. | Test (TC-027) |
| FR-017-AC-9 | The generator retains no evidence of its own: the execution surface writes no file. The caller receives the returned evidence and owns its retention. | Test (TC-027) |

## Dependencies

- **Upstream**: [FR-015](./FR-015-bounded-kani-obligations.md),
  [interface-001](../../interface/interface-001-codegen-api.md).
- **Downstream**: [TC-027](../../test/complete-v1/TC-027-pinned-kani-execution-evidence.md),
  [FR-016](./FR-016-witness-native-replay.md), which decodes the counterexample
  this requirement retains.

## Open items

Two items are open here, one a defect filed rather than specified because a
requirement must not be written to bless it, the other a gap in this
requirement's own criteria rather than a defect in the code:

- The outcome is read from the backend's human-readable output rather than a
  machine-readable one (agent-ix/quire-contract-codegen#59). The classification
  rule above is the intended rule; the format it reads is the defect.
- The run now carries a caller-declared wall-clock budget and reports a
  timed-out run as its own `KaniInconclusiveReason::TimedOut`
  (agent-ix/quire-contract-codegen#58, closed at the code level) — a
  classification distinct from the corpus path's `KaniOutcomeKind`, which
  FR-017-CON-2 forbids converting between. FR-017-AC-4 and FR-017-AC-5
  enumerate the inconclusive reasons they cover by name — unsuccessful exit,
  absent/zero-total/unreadable cover summary, no playback, exhausted unwind
  bound — and timed-out is not among them. Adding it as a named criterion is
  agent-ix/quire-contract-codegen#55.

The retained argument vector is the `kani` subcommand followed by the harness identity's option
vector unchanged (`src/kani_execution.rs`): the generator builds the invoked command line and the
retained vector from the same `identity.options` value, so their equality is structural rather
than an independently checkable behavior, and FR-017-AC-6 does not restate it as a criterion that
could fail.
