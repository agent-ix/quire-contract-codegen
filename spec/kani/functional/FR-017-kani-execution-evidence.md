---
id: FR-017
title: "Run bounded Kani obligations, singly or in a batch, and retain their evidence"
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
# FR-017: Run bounded Kani obligations, singly or in a batch, and retain their evidence

## Description

When a caller runs an FR-015 harness, the code generator shall invoke the
installed Kani backend and retain the backend's own reported outcome as typed
execution evidence. A caller may hand the generator several harnesses to run together
(FR-017-AC-21 to FR-017-AC-25); the generator then forms groups, runs each
group in one launcher process, and retains one evidence record per harness. Unless a
statement says "batch" or "group", it describes a run of one harness.

Two words are used for two things below. A **batch** is the list of harnesses the caller hands to
the batch entry, one call. A **group** is the harnesses of one batch that share one launcher
process, so G groups are G processes; a group of one is a single run. A rule that says "the
batch" is about the call: a batch refusal (a member not in the crate) refuses the call and starts
no process. A rule that says "the group" is about one process: a group refusal (an output stream
over its limit or unread, an outer bound that elapsed, a missing report after a clean exit, a
report that lacks, repeats or adds a harness, a playback for a non-member) refuses that group,
classifies none of its members and leaves every other group's result as it is. The generator
does not split or retry a refused group.

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
- For a batch, the list of such harnesses, each with the request fields below. The
  caller passes the list to the batch entry `execute_kani_obligations` in `kani/run/execute.rs`;
  that entry, not the caller, forms the groups.
- A Kani installation: the `cargo-kani` launcher to invoke.
- The caller's wall-clock timeout, `KaniExecutionRequest::timeout`, per harness. The run has no memory ceiling
  (until FR-028 lands); the launcher's stdout and stderr are each bounded to 8 MiB per harness, and a stream over that is refused, never truncated.
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
- The generator shall read the launcher's stdout and stderr to their end or until the launcher has
  ended, and return within the timeout plus a small fixed constant even when a process that left
  the group still holds a pipe open.
- If either stream carries more than 8 MiB times the number of harnesses the process runs (one for
  a single run), then the generator shall stop the run, kill the launcher's process group and
  refuse with the stable code `kani_output_over_limit`, naming the stream, the limit and the
  harness count. It shall retain no text and classify no outcome, because a silently truncated
  stream is evidence the generator cannot vouch for; a stream of exactly the limit is retained
  whole. The batch limit is the same per-harness bound applied to each member, so it is not a
  raised limit; the group is refused, not split or retried.
- If a thread reading the launcher's stdout or stderr panics, or a poll or read on the stream
  fails, then the generator shall refuse with the stable code `kani_output_unread` and classify no
  outcome, because an unread stream is not an empty one.
- The generator shall leave no process the launcher started in its process group running when a
  run concludes, whether it timed out or the launcher exited on its own.
- Where harnesses are run as a batch, the generator shall group them by equal option vector (the
  harness selection removed) and equal request timeout T, and start one launcher process per group.
  One process has one launcher, one working directory and one target directory, so requests that
  name different ones are never grouped. Until FR-028 lands and identities record ceilings, those
  are the whole key; the FR-028 ceilings join the key when it does.
- Where a group holds more than one harness, the generator shall pass its process one
  `--harness <module::harness> --exact` pair per member, in request order, then `--harness-timeout
  <T>` (whole seconds, rounded up; Kani 0.68 takes it under `-Z unstable-options`, which the launch
  already passes for the report) and the shared options. A group of one is a single run with the
  argument vector the single-run rule below states.
- If T rounded up to whole seconds exceeds 4294967295, then the generator shall omit
  `--harness-timeout` from that group's launch, because Kani 0.68 refuses a larger value (exit 2,
  no report); the group then has no per-member bound.
- Where a group holds more than one harness, the generator shall bound the process to N times T
  (N the member count, checked multiplication, never elapsing if the product does not fit, as
  FR-017-AC-15) and, if it is not done by then, kill its process group. Kani writes its report
  only at the end, so a killed group leaves none and the generator shall refuse it as timed out and
  classify no member. Kani's own per-harness timeout makes that wedge the only way a group is
  killed.
- The generator shall match a batch member to its report entry by the `module::harness` path the
  launch passed to `--harness`, which Kani echoes as the entry's `harness_id`, and never by the
  bare harness symbol, which every frame harness shares (`check`).
- The generator shall take each member's outcome, checks and SUCCESS-check count from its own
  report entry, so a falsified member never changes another member's outcome.
- The generator shall take a group member's falsifying playback from the console block Kani heads
  for that member's `module::harness` path ("Concrete playback unit test for `<harness>`:"), never
  from the first failing block in the console. `counterexample_playback` is given the member's
  path.
- If a member's report entry names a failed property check and no console block is headed for its
  path, then the generator shall classify that member inconclusive for carrying no counterexample,
  as a single run does (FR-017-AC-5).
- If a console block is headed for a path that is not a member of the group, then the generator
  shall refuse the group as an unattributable playback and classify no member. A member that
  fails several property checks has several counterexample blocks under its own path, because
  Kani prints one per failed check (and a cover block beside them); that is not unattributable.
  The member takes the first counterexample block headed for its path, as the same harness run
  alone does, so that a member's own failures never change another member's outcome (measured on
  Kani 0.68: one harness with two independently failing assertions printed two counterexample
  blocks, both headed for it).
- If a member's entry states success, the batch process exited non-zero and no member's entry
  states failure, then the generator shall classify that member inconclusive as a success reported
  by an unsuccessfully exited process. A member's success is verified when the process exited 0, or
  exited non-zero and at least one member's entry states failure; Kani exits 1 for any failed
  harness and for its own errors alike, so that predicate is all the exit code can show.
- If a member's entry states failure with no checks and its `error_details` give the exit status
  `timeout`, then the generator shall classify that member inconclusive with the timed-out reason
  naming T (the member's evidence carries T in whole seconds, rounded up, in its batch statement),
  and the other members keep their results. If it states failure with no checks and no
  timeout, the generator shall classify it inconclusive for carrying no counterexample.
- Where a group holds more than one harness, the generator shall retain in each member's evidence
  the obligation kind when the harness carries one, its harness path, the launcher path, the
  unwind bound, the solver, the outcome and the checks, as a single run does, and in place of the
  single-run argument vector the batch's complete argument vector, the member list, a statement
  that the invocation was a batch and the process exit code. This does not weaken the single-run rule below, whose purpose
  is that evidence cannot claim an invocation the harness did not specify: the batch vector is
  the invocation that happened, and the member list and batch statement say it was shared.
- If a group's process exits with status zero and exported no report, then the generator shall
  refuse the group with the refusal a single run gets for that, and classify no member.
- If a group's process exits unsuccessfully on its own without exporting a report, as on an argument
  error or a failed build, then the generator shall classify every member inconclusive with the
  `NoVerdict` outcome a single run gets, because no member's entry exists to tell them apart.
- If the report of a group lacks a requested harness, holds one twice or holds one that was not
  requested, then the generator shall refuse the group with a typed cause and classify no
  member.
- If a batch member's source is not in its crate, then the generator shall refuse the whole batch,
  the call, with `HarnessNotInCrate` before any process starts, not even for the members of
  another group.
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
- The generator shall retain, in the evidence of a single run, the obligation kind when the
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
  schema version the module reads, malformed or of an unknown check status, that (for a single run)
  does not hold exactly one harness result, or whose harness states success while it lists a failed, errored, undetermined or
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
| FR-017-AC-6 | The execution evidence of a single run carries the obligation kind, the harness path, the launcher path, the complete argument vector, the unwind bound, the solver, the exit code and the outcome. | Test (TC-027) |
| FR-017-AC-7 | A crate whose library source does not contain the harness source byte for byte is refused, and no backend runs. | Test (TC-027) |
| FR-017-AC-11 | A routed FR-022/FR-014 exact-scalar harness (`KaniScalarObligationHarness`) runs through `execute_kani_obligation` and `kani_launch_command` the same way an FR-015 contract harness does: a crate whose library source lacks its generated source byte for byte is `HarnessNotInCrate`, its covers classify a run identically (all satisfied is verified, an unsatisfied one is cover-unsatisfied, none printed is inconclusive), and its evidence carries `None` for obligation kind, since an exact-scalar claim carries no contract role. | Test (TC-027) |
| FR-017-AC-12 | Real Kani captures of a verified run, a falsified run with a playback, an exhausted unwind bound, a run whose only check is unreachable, a partly satisfied cover and a run with no cover each parse into the expected typed report of harness status, successful checks, cover counts and failed checks, and classify to the expected outcome; the falsifying playback block passes through verbatim. | Test (TC-027) |
| FR-017-AC-13 | A run whose process exited successfully and whose backend reported success with zero successful checks is inconclusive with the vacuous-proof reason, never verified, except that a precondition harness, which asserts nothing beyond its cover, is decided by its cover summary. | Test (TC-027) |
| FR-017-AC-14 | A launcher whose stdout or stderr carries more than 8 MiB times its harness count is stopped, its process group is killed, and the run is refused with `kani_output_over_limit` naming the stream, the limit and the count, with no outcome and no retained text; a launcher whose stream is exactly the limit completes with its real exit status and the whole stream retained. No stream is ever truncated and then classified. | Test (TC-043) |
| FR-017-AC-15 | A timeout too large to add to the current instant never elapses and does not panic. | Test (TC-027) |
| FR-017-AC-16 | The launcher's capture threads are stopped and joined on every outcome: they return what the launcher wrote before it ended, stop while a write end is still held open, and stop within their drain limit while a straggler keeps writing, so a process holding a pipe open does not delay the return. | Test (TC-027) |
| FR-017-AC-17 | A run that times out has the whole group the launcher leads killed, a real grandchild included. | Test (TC-027) |
| FR-017-AC-18 | A report that is malformed, of an unknown check or harness status, of another schema version, that holds other than one harness result in a single run, or whose harness states success while it lists a failed, errored, undetermined or unknown check (of any class, including an unwinding assertion) is refused with its own typed cause and never classified, for every obligation kind; a run that exited successfully and exported no report is refused, and one that exited unsuccessfully and exported none is `NoVerdict`. | Test (TC-027) |
| FR-017-AC-19 | The launch exports its report after the harness options; its report file name is unique to the launch, so a report another run left or is writing in the same target directory is never read, removed or overwritten by this run, and the run removes its own file; a report over the read bound is refused and not truncated. | Test (TC-027) |
| FR-017-AC-20 | The evidence and the classified run list every check of a real run with its id, class, source file and line and status, a line stated as unknown is absent, and a non-numeric line or a check with no location is a refused report; the view serializes as `id`, `class`, `location { file, line }` and `status` and is not deserializable. | Test (TC-027) |
| FR-017-AC-21 | N harnesses with equal option vectors (the harness selection removed) and equal request timeout T start exactly one launcher process whose argument vector holds one `--harness <module::harness> --exact` pair per member in request order, `--harness-timeout` T, then the shared options; N harnesses in G such groups start G processes; so N > 1 compatible harnesses start fewer than N processes; one harness starts one process with the single-run argument vector. The group's outer bound is N times T, never elapsing when the product does not fit, and a group killed at it leaves no report and is refused as timed out with no member classified. A T that rounded up exceeds 4294967295 seconds (a `Duration::MAX` request) launches without `--harness-timeout`. A group that exits successfully with no report is refused with the missing-report refusal and classifies no member; one that exits unsuccessfully with no report (an argument error, a failed build) has every member inconclusive `NoVerdict`. Requests that name another launcher, crate directory or target directory are never grouped. | Test (TC-043) |
| FR-017-AC-22 | Over a real or captured batch report, members are matched by `module::harness` path (two members with the bare symbol `check` stay distinct), and each member's outcome, checks and SUCCESS count come from its own entry: a falsified member beside a verified member leaves the verified member verified; a Success member in a non-zero exit with no Failure entry is inconclusive, and with one Failure entry is verified; a Failure entry with no checks and exit status `timeout` is inconclusive timed-out naming T while the others keep their results; a Failure entry with no checks and no timeout is inconclusive with no counterexample; each member's evidence carries its kind, harness path, launcher path, unwind bound, solver, outcome and checks plus the batch argument vector, the member list, a statement that it was a batch, and the exit code. | Test (TC-043) |
| FR-017-AC-23 | A group's report that lacks a requested harness, holds one twice or holds an unrequested one refuses the group with a typed cause and classifies no member; a member whose source is not in the crate refuses the whole batch with `HarnessNotInCrate` and launches nothing; a falsified member's playback is the first counterexample block headed for its own path and never another member's block, a member that fails two property checks (two counterexample blocks under its path, as real Kani prints) takes the first, as the same harness alone does, and neither it nor its neighbours lose their evidence, a failed property check with no block headed for its path is that member inconclusive with no counterexample, as in a single run; a block headed for a path that is not a member refuses the group. | Test (TC-043) |
| FR-017-AC-24 | A launcher that exits on its own after leaving a real grandchild in its process group has that grandchild killed by the time the run returns, the same as on a timeout. | Test (TC-043) |
| FR-017-AC-25 | A capture thread that panics, or whose poll or read of the stream errs, refuses the run with `kani_output_unread`; the run is never classified from empty text. | Test (TC-043) |

## Dependencies

- **Upstream**: [FR-015](./FR-015-bounded-kani-obligations.md),
  [FR-022](../../routed/functional/FR-022-routed-generation.md), whose `generate_routed` is the source
  of the exact-scalar harness this requirement also runs,
  [interface-001](../../core/functional/interface-001-codegen-api.md).
- **Downstream**: [TC-027](../matrix/TC-027-kani-execution-evidence.md),
  [TC-043](../matrix/TC-043-kani-batching-and-output-bounds.md),
  [FR-029](./FR-029-run-outcome-terminal-record.md), which maps the outcome to QSL's terminal value,
  [FR-016](../../replay/functional/FR-016-witness-native-replay.md), which decodes the counterexample
  this requirement retains.
