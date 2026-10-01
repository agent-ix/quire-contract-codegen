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
  successfully, the backend's exported report states success, and the report lists at
  least one cover property and every one is satisfied. A run is never defaulted to verified:
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
- The generator shall launch Kani with `-Z unstable-options --export-json <file>`, the file in the
  request's target directory under a name unique to that launch (the process id and a process-wide
  sequence), and shall remove only that file, before launching and after reading it. Runs sharing a
  target directory, in this process or in others at the same time, therefore never write, remove
  or read each other's report, and a file left by an earlier run is never read as this run's
  verdict.
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
  inconclusive with the reason that applies: the process failed before exporting a report,
  a failure carried no counterexample, or the report listed no cover property
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
  the process exit code, the outcome and the count of checks the report lists as holding. The generator shall make the argument vector
  hold the `kani` subcommand, the harness identity's option vector unchanged and the
  report-export flags, in that order and nothing else, so the evidence cannot claim an
  invocation the harness did not specify.
- The generator shall read Kani's output in exactly one module, `src/kani_transcript.rs`, which
  returns a typed report, and shall classify every run from that report's fields and never from
  console text. The verdict, the check and cover counts and the unwinding failures come from
  Kani's exported report, whose members and status vocabulary that module reads exactly.
  A report that is absent after a successful exit, unreadable, over the read bound, not the one
  schema version the module reads, malformed or of an unknown check status, that does not hold exactly one harness
  result, or whose harness states success while it lists a failed, errored, undetermined or
  unknown check, is a typed refusal with a stable cause and never an outcome: it is not classified
  inconclusive. A run that exited unsuccessfully and exported no report is `NoVerdict`.
- The generator shall retain, in the evidence and in the classified run, every check the report
  lists, each with its position in the report, its class, its source file and line and its status,
  so a consumer attributes each successful check to source. A line Kani states as unknown is absent;
  a line that is neither a number nor unknown makes the report malformed. The per-check view has
  one wire shape, `id`, `class`, `location { file, line }` and `status`, and is serialize-only: it
  is never read back from Kani's own spelling (`category`, a line string).
- The generator shall carry on the evidence and the classified run the SUCCESS-check count: the
  report's checks that are not covers and hold, plus, for a precondition harness, whose one property
  is its cover, the satisfied covers.
- The report carries no concrete playback. The generator shall take a falsifying playback from the
  console as a payload only, after the report names a failed property check, and pass it
  through verbatim as the counterexample, which FR-016 decodes. A playback printed for a cover is
  never the counterexample.

## Constraints

| ID | Constraint | Type | Validation |
|----|------------|------|------------|
| FR-017-CON-1 | The generator SHALL NOT convert a non-verified outcome into a proof claim. | Integrity | Test (TC-027) |
| FR-017-CON-2 | The generator SHALL NOT report a generation-time classification as an execution outcome. | Integrity | Test (TC-027) |

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-017-AC-2 | An absent launcher is refused with a typed reason naming its path before anything runs. | Test (TC-027) |
| FR-017-AC-4 | A run is verified only when the process exited successfully, the report states success, and every cover property is reported satisfied; a successful run with an unsatisfied cover is cover-unsatisfied with its satisfied and total counts; a report of success from an unsuccessfully exited process, a report with no cover property and a report with no successful check are each inconclusive with their own reason; console banners never decide the verdict. | Test (TC-027) |
| FR-017-AC-5 | A failed non-unwinding check with a concrete playback is falsified carrying that playback verbatim and never the playback of a satisfied cover; a failure with no playback is inconclusive for that reason; a failed unwinding assertion is inconclusive as an exhausted bound rather than falsified, and a succeeded unwinding check in a results listing is not a failure. | Test (TC-027) |
| FR-017-AC-6 | Execution evidence carries the obligation kind, the harness path, the launcher path, the complete argument vector, the unwind bound, the solver, the exit code and the outcome. | Test (TC-027) |
| FR-017-AC-7 | A crate whose library source does not contain the harness source byte for byte is refused, and no backend runs. | Test (TC-027) |
| FR-017-AC-11 | A routed FR-022/FR-014 exact-scalar harness (`KaniScalarObligationHarness`) runs through `execute_kani_obligation` and `kani_launch_command` the same way an FR-015 contract harness does: a crate whose library source lacks its generated source byte for byte is `HarnessNotInCrate`, its covers classify a run identically (all satisfied is verified, an unsatisfied one is cover-unsatisfied, none printed is inconclusive), and its evidence carries `None` for obligation kind, since an exact-scalar claim carries no contract role. | Test (TC-027) |
| FR-017-AC-12 | Real Kani captures of a verified run, a falsified run with a playback, an exhausted unwind bound, a run whose only check is unreachable, a partly satisfied cover and a run with no cover each parse into the expected typed report of harness status, successful checks, cover counts and failed checks, and classify to the expected outcome; the falsifying playback block passes through verbatim. | Test (TC-027) |
| FR-017-AC-13 | A run whose process exited successfully and whose backend reported success with zero successful checks is inconclusive with the vacuous-proof reason, never verified, except that a precondition harness, which asserts nothing beyond its cover, is decided by its cover summary. | Test (TC-027) |
| FR-017-AC-14 | A launcher that prints more than 8 MiB completes with its real exit status and only the tail of each stream retained. The verdict is read from the exported report, not from the stream, so truncating a stream never loses it. | Test (TC-027) |
| FR-017-AC-15 | A timeout too large to add to the current instant never elapses and does not panic. | Test (TC-027) |
| FR-017-AC-16 | The launcher's capture threads are stopped and joined on every outcome: they return what the launcher wrote before it ended, stop while a write end is still held open, and stop within their drain limit while a straggler keeps writing, so a process holding a pipe open does not delay the return. | Test (TC-027) |
| FR-017-AC-17 | A run that times out has the whole group it leads killed, a real grandchild included. | Test (TC-027) |
| FR-017-AC-18 | A report that is malformed, of an unknown check or harness status, of another schema version, that holds other than one harness result, or whose harness states success while it lists a failed, errored, undetermined or unknown check (of any class, including an unwinding assertion) is refused with its own typed cause and never classified, for every obligation kind; a run that exited successfully and exported no report is refused, and one that exited unsuccessfully and exported none is `NoVerdict`. | Test (TC-027) |
| FR-017-AC-19 | The launch exports its report after the harness options; its report file name is unique to the launch, so a report another run left or is writing in the same target directory is never read, removed or overwritten by this run, and the run removes its own file; a report over the read bound is refused and not truncated. | Test (TC-027) |
| FR-017-AC-20 | The evidence and the classified run list every check of a real run with its id, class, source file and line and status, a line stated as unknown is absent, and a non-numeric line or a check with no location is a refused report; the view serializes as `id`, `class`, `location { file, line }` and `status` and is not deserializable. | Test (TC-027) |

## Dependencies

- **Upstream**: [FR-015](./FR-015-bounded-kani-obligations.md),
  [FR-022](../../routed/functional/FR-022-routed-generation.md), whose `generate_routed` is the source
  of the exact-scalar harness this requirement also runs,
  [interface-001](../../core/functional/interface-001-codegen-api.md).
- **Downstream**: [TC-027](../matrix/TC-027-kani-execution-evidence.md),
  [FR-029](./FR-029-run-outcome-terminal-record.md), which maps the outcome to QSL's terminal value,
  [FR-016](../../replay/functional/FR-016-witness-native-replay.md), which decodes the counterexample
  this requirement retains.
