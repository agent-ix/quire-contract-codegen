---
id: FR-017
title: "Run one bounded Kani obligation and retain its evidence"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-022
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/interface-001
    type: implements
  - target: ix://agent-ix/quire-specification/FR-196
    type: references
---
# FR-017: Run one bounded Kani obligation and retain its evidence

## Description

When a caller runs an FR-015 harness, the code generator shall invoke the
installed Kani backend and retain the backend's own reported outcome as typed
execution evidence.

FR-015 emits harnesses and typed
refusals and asserts nothing about whether one ever ran; this requirement owns
the run and everything read back from it.

## Inputs

- One harness of either kind this requirement generates: an FR-015 contract
  harness (`KaniObligationHarness`, contract role in
  `ObligationKind`) or an FR-022/FR-014 exact-scalar harness
  (`KaniScalarObligationHarness`, no contract role). Either way its identity
  carries the option vector to invoke, the unwind bound and the solver. The two identity types are distinct
  structs (`KaniObligationIdentity`, `ScalarObligationIdentity`); this
  requirement reads the same handful of facts from whichever one the caller
  hands it, through one borrowed view, rather than owning two execution
  paths.
- A Kani installation: the `cargo-kani` launcher to invoke.
- The caller's wall-clock timeout, `KaniExecutionRequest::timeout`. The run has no memory ceiling;
  what the generator keeps of the launcher's stdout and stderr is bounded to the last 8 MiB of each.
- The crate directory whose library source contains that harness's generated
  source byte for byte, and the Cargo target directory the run builds into.

## Outputs

- Execution evidence naming the harness path, the exact invocation, and the
  backend-reported outcome. The obligation-kind field is the contract harness's
  role for a contract harness, and `None` for an exact-scalar harness, which
  has no contract role to report.
- A typed refusal, and no run, when the launcher is absent or when the crate
  does not contain the harness.

## Behavior

- If the launcher is absent, then the generator shall refuse with a typed
  reason naming its path, and shall run nothing.
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
- If the process exited successfully and the backend reported success with zero successful
  checks, then the generator shall classify the run as inconclusive with the vacuous-proof reason;
  for a precondition harness, whose only property is its non-vacuity cover, the generator shall
  instead classify the run by its cover summary alone, as verified, cover-unsatisfied or
  inconclusive under the cover rules above.
- The generator shall run the launcher as the leader of its own process group and, when the run
  does not conclude within the timeout, kill that group, so that the solver and every other
  process the launcher started in it are killed with it.
- The generator shall keep at most the last 8 MiB of the launcher's stdout and of its stderr,
  read the pipes to their end or until the launcher has ended, and return within the timeout plus
  a small fixed constant even when a process that left the group still holds a pipe open.
- If the run does not conclude within the caller's timeout, then the generator shall kill it and
  classify it as inconclusive with the timed-out reason.
- If the backend reported a failed unwinding assertion, then the
  generator shall classify the run as inconclusive with the
  exhausted-loop-bound reason instead of falsified.
- Where a run establishes nothing, the generator shall classify it as
  inconclusive with the reason that applies: no verdict was printed, a failure
  carried no counterexample, or no readable non-empty cover summary was printed
  so non-vacuity was not observed.
- The generator shall run either harness kind through the one execution path:
  the byte-for-byte crate check, the launch and the outcome classification
  read the identity, the source artifact, the unwind bound, the solver
  and the option vector from whichever kind's identity the caller supplied,
  and none of those steps branches on which kind it is, except that the
  classification applies the zero-checks rule to every harness but a precondition harness.
- The generator shall retain, in the evidence, the obligation kind when the
  harness carries one, the harness path, the invoked
  launcher path, the complete argument vector, the unwind bound, the solver,
  the process exit code and the outcome. The argument vector after the `kani` subcommand shall be the harness
  identity's option vector unchanged, so the evidence cannot claim an
  invocation the harness did not specify.
- The generator shall read the backend's printed output to decide a verdict in
  exactly one module, `src/kani_transcript.rs`, which returns a typed
  transcript, and shall classify every run from that transcript's fields and
  never from text. Kani publishes no machine-readable verdict, so the
  wording that module matches is Kani's own and not this repository's.
  A falsifying playback block is passed through verbatim as the counterexample,
  which FR-016 decodes.

## Constraints

| ID | Constraint | Type | Validation |
|----|------------|------|------------|
| FR-017-CON-1 | The generator SHALL NOT convert a non-verified outcome into a proof claim. | Integrity | Test (TC-027) |
| FR-017-CON-2 | The generator SHALL NOT report a generation-time classification as an execution outcome. | Integrity | Test (TC-027) |

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-017-AC-2 | An absent launcher is refused with a typed reason naming its path before anything runs. | Test (TC-027) |
| FR-017-AC-4 | A run is verified only when the process exited successfully, the backend reported success, and every cover property is reported satisfied; a successful run with an unsatisfied cover is cover-unsatisfied with its satisfied and total counts; success text from an unsuccessfully exited process, an absent cover summary, a zero-total summary and an unreadable summary are each inconclusive with their own reason. | Test (TC-027) |
| FR-017-AC-5 | A failed non-unwinding check with a concrete playback is falsified carrying that playback verbatim and never the playback of a satisfied cover; a failure with no playback is inconclusive for that reason; a failed unwinding assertion is inconclusive as an exhausted bound rather than falsified, and a succeeded unwinding check in a results listing is not a failure. | Test (TC-027) |
| FR-017-AC-6 | Execution evidence carries the obligation kind, the harness path, the launcher path, the complete argument vector, the unwind bound, the solver, the exit code and the outcome. | Test (TC-027) |
| FR-017-AC-7 | A crate whose library source does not contain the harness source byte for byte is refused, and no backend runs. | Test (TC-027) |
| FR-017-AC-11 | A routed FR-022/FR-014 exact-scalar harness (`KaniScalarObligationHarness`) runs through `execute_kani_obligation` and `kani_launch_command` the same way an FR-015 contract harness does: a crate whose library source lacks its generated source byte for byte is `HarnessNotInCrate`, its covers classify a run identically (all satisfied is verified, an unsatisfied one is cover-unsatisfied, none printed is inconclusive), and its evidence carries `None` for obligation kind, since an exact-scalar claim carries no contract role. | Test (TC-027) |
| FR-017-AC-12 | Real Kani captures of a verified run, a falsified run with a playback, an exhausted unwind bound, an unreachable cover, a partly satisfied cover and a run with no cover summary each parse into the expected typed transcript of verdict banners, failed checks, check and cover summaries and playback tests, and classify to the expected outcome; the falsifying playback block passes through verbatim. | Test (TC-027) |
| FR-017-AC-13 | A run whose process exited successfully and whose backend reported success with zero successful checks is inconclusive with the vacuous-proof reason, never verified, except that a precondition harness, which asserts nothing beyond its cover, is decided by its cover summary. | Test (TC-027) |
| FR-017-AC-14 | A launcher that prints more than 8 MiB completes with its real exit status and only the tail of each stream retained, the verdict lines included. | Test (TC-027) |
| FR-017-AC-15 | A timeout too large to add to the current instant never elapses and does not panic. | Test (TC-027) |
| FR-017-AC-16 | The launcher's capture threads are stopped and joined on every outcome: they return what the launcher wrote before it ended, stop while a write end is still held open, and stop within their drain limit while a straggler keeps writing, so a process holding a pipe open does not delay the return. | Test (TC-027) |
| FR-017-AC-17 | A run that times out has its whole process group killed, a real grandchild included. | Test (TC-027) |

## Dependencies

- **Upstream**: [FR-015](./FR-015-bounded-kani-obligations.md),
  [FR-022](./FR-022-routed-generation.md), whose `generate_routed` is the source
  of the exact-scalar harness this requirement also runs,
  [interface-001](../../interface/interface-001-codegen-api.md).
- **Downstream**: [TC-027](../../test/complete-v1/TC-027-kani-execution-evidence.md),
  [FR-029](./FR-029-run-outcome-terminal-record.md), which maps the outcome to QSL's terminal value,
  [FR-016](./FR-016-witness-native-replay.md), which decodes the counterexample
  this requirement retains.
