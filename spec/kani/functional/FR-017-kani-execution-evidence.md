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
  - target: ix://agent-ix/quire-contract-codegen/FR-035
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/interface-001
    type: implements
  - target: ix://agent-ix/quire-specification/FR-196
    type: references
---
# FR-017: Run bounded Kani obligations, singly or in a batch, and retain their evidence

## Description

When a caller runs an FR-015, FR-022/FR-014 or FR-035 harness, the code generator shall invoke the
installed Kani backend and retain the backend's own reported outcome as typed
execution evidence. A caller may hand the generator several harnesses to run together
(FR-017-AC-21 to FR-017-AC-25); the generator then forms groups, runs each
group in one launcher process, and retains one evidence record per harness. Unless a
statement says "batch" or "group", it describes a run of one harness.

[FR-040](./FR-040-cancellation-producer.md) adds the one launcher-observed cancellation outcome:
the backend did not report a verdict for a run stopped by caller cancellation. Its typed source
comes from the live execution control, and its group refusal produces no member evidence.

Two words are used for two things below. A **batch** is the list of harnesses the caller hands to
the batch entry, one call. A **group** is the harnesses of one batch that share one launcher
process, so G groups are G processes; a group of one is a single run. A rule that says "the
batch" is about the call: a batch refusal (a member not in the crate) refuses the call and starts
no process. A rule that says "the group" is about one process: a group refusal (an output stream
over its limit or unread, an outer bound that elapsed, a missing report after a clean exit, a
report that lacks, repeats or adds a harness, a playback for a non-member) refuses that group,
classifies none of its members and leaves every other group's result as it is. The generator
does not split or retry a refused group.

FR-015 and FR-035 emit harnesses and typed refusals and assert nothing about
whether one ever ran; this requirement owns the run and everything read back
from it. The FR-035 caller-ingress harness is a distinct executable kind, separate
from both checked-package harness kinds. Its evidence keeps the caller
Text-admission identity rather than claiming a checked node or contract role.

## Inputs

- One harness of any kind this requirement executes: an FR-015 contract
  harness (`KaniObligationHarness`, contract role in
  `ObligationKind`) or an FR-022/FR-014 exact-scalar harness
  (`KaniScalarObligationHarness`, no contract role), or the distinct FR-035
  caller Text-admission harness (no contract role, with a typed caller claim
  identity naming profile, bounds and finite payload class). Each identity
  carries the option vector to invoke, the unwind bound and the solver. This
  requirement reads the same handful of facts from whichever one the caller
  hands it, through one borrowed view, rather than owning separate execution
  paths.
- For a batch, the list of such harnesses, each with the request fields below. The
  caller passes the list to the batch entry `execute_kani_obligations` in `kani/run/execute.rs`;
  that entry, not the caller, forms the groups.
- A Kani installation: the `cargo-kani` launcher to invoke.
- For a paired routed-scalar proof/native run, the retained scalar harness, its complete routed
  oracle artifact package, the concrete integer operands in call order, and the request's
  `ScalarLimits`. The upstream caller supplies the limits it intends to use; the caller supplies no
  crate directory, proof-content identity or native outcome for this operation.
- The harness identity's `ProofCeilings`: `wall_clock` bounds one harness's run and
  `memory_bytes` bounds aggregate resident memory of its backend process tree (FR-028).
  `KaniExecutionRequest` carries the harness, installation and crate/target directories;
  it has no separate timeout field. The launcher's stdout and stderr are each bounded
  to 8 MiB per harness, and a stream over that is refused, never truncated.
- The crate directory whose library source contains that harness's generated
  source byte for byte, and the Cargo target directory the run builds into.
- The home directory of the launcher's environment, `KaniExecutionRequest::home_directory`, an
  optional absolute path the caller supplies. The generator uses it for two things only: to redact
  the NoVerdict diagnostic excerpts, and as a grouping key, so requests that name different home
  directories are never grouped. The generator never reads it from the process environment, and
  when it is absent nothing is redacted as `<HOME>`.

## Outputs

- Execution evidence naming the harness path, the exact invocation, and the
  backend-reported outcome. The obligation-kind field is the contract harness's
  role for a contract harness, and `None` for an exact-scalar or caller
  Text-admission harness, neither of which has a contract role. Caller
  Text-admission evidence carries its typed caller-ingress identity so it
  cannot be attributed to a checked-package scalar claim.
- For a `NoVerdict` run only, a diagnostic record in that evidence: a typed failure phase and a
  bounded, redacted tail excerpt of each of the launcher's stdout and stderr (see NoVerdict
  diagnostics below).
- For a `FailedWithoutCounterexample` run whose report states an exit status for the harness, the
  typed cause that exit status names; and, when that cause is a CBMC failure rather than a failed
  property, the same diagnostic record (see CBMC failure cause below).
- A typed refusal, and no run, when the launcher is absent or when the crate
  does not contain the harness.
- For a paired routed-scalar run, an opaque receipt containing the measured proof-content
  identity, the retained scalar identity, the ordered concrete operands and exact `ScalarLimits`
  supplied to and used by the native invocation,
  verified Kani execution evidence, and the actual typed outcome returned by invoking the
  staged oracle, or a typed failure with no receipt.

## Behavior

- If the launcher is absent, then the generator shall refuse with a typed
  reason naming its path, and shall run nothing.
- When a caller requests a paired routed-scalar proof/native receipt, the runner shall bind the
  retained scalar harness to its complete generated oracle package and validate the selected
  public oracle, its arity and argument order, the concrete operands against their retained
  ranges before starting either child process. It shall pass the request's exact `ScalarLimits` to
  `KaniScalarNativeRequest`, initialize the native meter from those values, and return those same
  values in the receipt.
- The scalar runner shall stage one private snapshot containing the complete routed package, the
  retained scalar harness, the canonical native invocation adapter, a private observer companion
  manifest, and independently generated production and companion dependency locks.
- The scalar runner shall measure one canonical proof-content identity over the exact paths and
  bytes of every staged execution input: the complete generated package, retained harness, native
  invocation adapter, observer manifest and both resolved dependency locks. This identity names
  the content actually proved and executed; it does not track tool versions or add a separate file/version identity.
- The scalar receipt shall retain the selected scalar identity, oracle symbol, ordered concrete
  operands and the exact request `ScalarLimits` used by the native meter.
- While either child executes, the runner shall prevent that child from modifying the staged source
  tree.
- When the runner constructs a receipt, it shall verify that the staged tree has its expected path
  set and contents and that all execution-relevant staged paths and bytes still match the measured
  proof-content identity; a changed, replaced or additional staged file shall produce a typed
  failure and no receipt.
- The runner shall execute Kani against the staged harness.
- The runner shall invoke the public oracle from that same staged library source at the retained
  ordered operands with a meter initialized from the exact `ScalarLimits` supplied in
  `KaniScalarNativeRequest`.
- The runner shall take the native outcome from the runtime invocation's typed result, never infer
  it from the Kani verdict or caller-supplied text.
- When Kani reports `Verified`, the staged native call produces one supported typed runtime
  outcome, and the measured snapshot remains unchanged, the runner shall return a receipt.
- If proof execution is failed or inconclusive, native launch or decoding fails, or the snapshot
  changes, then the runner shall return a typed failure with no receipt.
- The public receipt shall expose read-only access to its measured identity, scalar identity,
  ordered operands, exact supplied and meter-used limits, Kani evidence and actual typed native
  outcome, and shall have
  no caller-accessible constructor, deserializer or setters for its identity or native outcome.
- The CG runner shall attest which `ScalarLimits` it received and used.
- The codegen crate root SHALL publicly re-export the exact
  `quire_contract_runtime::exact::ScalarLimits` type of
  `KaniScalarNativeRequest::scalar_limits`, with no wrapper, constructor or default.
- The CG runner shall make no claim that its supplied limits are the limits retained by the
  upstream scalar-generation context.
- The IR-773 driver shall retain the original scalar-generation `ScalarLimits` and pass that value
  to CG.
- When it evaluates a receipt for acceptance, the IR-773 driver shall compare both its request and
  the returned receipt against its retained original limits. This provenance check belongs to the
  driver, not CG.
- When either value differs from the driver's retained original limits, the IR-773 driver shall
  detect the mismatch locally, refuse receipt acceptance and skip all QSL evaluation stages.
- When IR-773 has no defined typed outcome for this mismatch, its owner shall specify and plan one
  before implementing the consumer check; CG defines no typed outcome for this mismatch.
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
- The generator shall append `-Z`, `unstable-options`, `--export-json`, and `/proc/self/fd/N`, in
  that exact order after the unchanged harness options. `N` is the actual child-only mapped
  anonymous report-pipe writer descriptor, at least 5. Evidence records these actual argument bytes,
  not a synthetic target-directory pathname. All other argument bytes, stdin, environment and cwd
  remain unchanged.
- The generator shall use per-run unnamed kernel report storage and the collector obligations in
  FR-034 AC-32 and AC-33. The generator shall create no target-directory report, pathname reader or
  named fallback. This explicitly replaces the internal named-report allocation.
- When actual pipe EOF contains zero bytes, the generator shall treat the run as exporting no
  report: successful exit refuses as missing report; unsuccessful exit is `NoVerdict`, except an
  independently established resource/deadline stop retains its existing resource classification.
- When nonempty bytes end at actual EOF, the generator shall parse the complete bytes under the
  existing schema rules. An independently established memory/resource or deadline stop takes
  precedence over partial/malformed content, preserving the single-run resource outcome and
  existing whole-batch resource/deadline refusal with no member classified. Otherwise, an
  incomplete/malformed partial write refuses regardless of process exit; a valid full report
  retains existing exit/report classification rules. EOF alone neither proves Completed nor
  authorizes classification before confirmed owned cleanup.
- When the live collector observes report-limit overflow, the generator shall cancel owned execution
  and classify a single run as `KaniRunOutcome::Inconclusive` with
  `KaniInconclusiveReason::MemoryExhausted` (`memory_exhausted`). Evidence shall name the 16 MiB
  report-allocation cap separately from the original whole-run memory ceiling; no tree peak is
  fabricated. A batch shall retain whole-batch memory-exhausted refusal with no member classified.
  FR-029 maps this source reason to QSL `Incomplete(ResourceExhausted)`, never `Failed`. This is an
  explicit outcome delta from the superseded named-reader `TooLarge` refusal, not a reinterpretation
  of the unchanged legacy reader's current behavior.
- When Completed is authenticated, the generator shall follow FR-034 AC-33's ordered final handoff,
  verified immutable descriptor read and cleanup under the original deadline before classification.
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
  harness selection removed) and equal identity `ProofCeilings` (both `wall_clock` T and
  `memory_bytes`), and start one launcher process per group.
  One process has one launcher, one working directory and one target directory, so requests that
  name different ones are never grouped. The shared process tree is held to the group's one
  `memory_bytes` ceiling in aggregate; the memory ceiling is not multiplied by member count.
- Where harnesses are run as a batch, the generator shall also never group requests whose
  `home_directory` differs (absent being one value), so that one group has one home directory for
  its NoVerdict diagnostics (FR-017-AC-36).
- Where a group holds more than one harness, the generator shall pass its process one
  `--harness <module::harness> --exact` pair per member, in request order, then `--harness-timeout
  <T>` (whole seconds, rounded up; Kani 0.68 takes it under `-Z unstable-options`, which the launch
  already passes for the report) and the shared options. A group of one is a single run with the
  argument vector the single-run rule below states.
- If the identity's T rounded up to whole seconds exceeds 4294967295, then the generator shall omit
  `--harness-timeout` from that group's launch, because Kani 0.68 refuses a larger value (exit 2,
  no report); the group then has no per-member bound.
- Where a group holds more than one harness, the generator shall bound the process to N times
  the shared identity's `wall_clock` T (N the member count, checked multiplication, never
  elapsing if the product does not fit, as
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
  fails several property checks at different input valuations has several counterexample blocks
  under its own path, because Kani prints one block per distinct input valuation (the playback
  test's name is a hash of the concrete values), listing first the check that comes first in its
  report, and a cover block beside them at a valuation of its own; that is not unattributable.
  The member takes the first counterexample block headed for its path, as the same harness run
  alone does, so that a member's own failures never change another member's outcome (measured on
  Kani 0.68: one harness with two independently failing assertions at different valuations
  printed two counterexample blocks, both headed for it). Checks that fail at the same valuation
  print one block, the first-listed check's: a member whose failing assertions share a valuation
  has one counterexample block and is falsified with it, and a failed check whose valuation a
  satisfied cover also took prints only the cover's block, which is no counterexample, so the
  member classifies as a failure with no counterexample. The generator therefore never places a
  cover where a failing check can share its valuation (FR-015-AC-7).
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
  single-run argument vector the group's complete argument vector, the member list, a statement
  that the invocation was a batch and the process exit code. This does not weaken the single-run rule below, whose purpose
  is that evidence cannot claim an invocation the harness did not specify: the group's vector is
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
- If a single run does not conclude within its harness identity's `wall_clock` ceiling, then the
  generator shall kill it and classify it as inconclusive with the timed-out reason.
- If the backend reported a failed unwinding assertion, then the
  generator shall classify the run as inconclusive with the
  exhausted-loop-bound reason instead of falsified.
- Where a run establishes nothing, the generator shall classify it as
  inconclusive with the reason that applies: the process failed before exporting a report,
  a failure carried no counterexample, or the report listed no cover property
  so non-vacuity was not observed.
- The generator shall run the FR-035 caller harness through the same execution path as the other kinds:
  the byte-for-byte crate check, the launch and the outcome classification
  read the identity, the source artifact, the unwind bound, the solver
  and the option vector from whichever kind's identity the caller supplied,
  and none of those steps branches on which kind it is, except that the
  classification applies the zero-checks rule to every harness but a precondition harness.
- When executing an FR-035 caller Text-admission harness, the generator shall
  retain its typed caller-ingress identity from the selected harness in the
  execution evidence, bind the outcome and any playback to that identity and
  harness path, and carry `None` for contract obligation kind. It shall apply
  the same report-based verified, cover-unsatisfied, falsified-playback and
  inconclusive classifications and the same source-mismatch and infrastructure
  refusals as for the other harness kinds. A refused launch or report shall
  produce no verified caller-admission claim.
- The generator shall retain, in the evidence of a single run, the obligation kind when the
  harness carries one, the harness path, the invoked
  launcher path, the complete argument vector, the unwind bound, the solver,
  the process exit code, the outcome and the count of checks the report lists as holding. The generator shall make the argument vector
  hold the `kani` subcommand, the harness identity's option vector unchanged and the
  report-export flags, in that order and nothing else, so the evidence cannot claim an
  invocation the harness did not specify.
- The generator shall read Kani's output in exactly one place, `src/kani/output/` (`report.rs` for the exported report, which it returns as a
  typed report, and `playback.rs` for the console's playback block, a payload), and shall classify every run from that report's fields and never from
  console text. The verdict, the check and cover counts and the unwinding failures come from
  Kani's exported report, whose members and status vocabulary `report.rs` reads exactly.
  A report that is absent after a successful exit, unreadable, not the one
  schema version the module reads, malformed or of an unknown check status, that (for a single run)
  does not hold exactly one harness result, or whose harness states success while it lists a failed, errored, undetermined or
  unknown check, is a typed refusal with a stable cause and never an outcome: it is not classified
  inconclusive. Live report-limit overflow instead uses the explicit resource-stop rule above.
  A run that exited unsuccessfully and exported no report is `NoVerdict`.
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

### NoVerdict diagnostics

A `NoVerdict` run carries no report-derived fields that say why the backend gave no verdict, so
its evidence carries a bounded diagnostic record taken from the launcher's two streams. The record
is diagnostic only: it never changes an outcome, a check, a count or a playback, and it makes no
verdict claim.

- When the classified outcome of a run or of a batch member is `KaniRunOutcome::Inconclusive` with
  `KaniInconclusiveReason::NoVerdict`, the generator shall retain in that evidence a diagnostic
  record holding a typed failure phase, a stdout excerpt and a stderr excerpt.
- When the classified outcome is any other outcome, the generator shall retain no diagnostic
  record, and the evidence's wire form shall have no `diagnostics` member, except for the
  `FailedWithoutCounterexample` evidence that CBMC failure cause below gives a record. When the run is
  refused, the generator shall return no evidence and so no diagnostic record.
- The generator shall compute the outcome, the checks, the SUCCESS-check count and the playback
  exactly as the rules above state, before and independently of the diagnostic record. The
  generator shall use neither the phase nor an excerpt as an input to classification, playback
  attribution, group refusal or the FR-029 terminal map; `NoVerdict` stays `NoVerdict`.
- The generator shall read the phase markers and build the excerpts in `src/kani/output/`, the one
  place Kani's output is read, and shall derive the phase from the whole captured streams, never
  from an excerpt.
- The generator shall keep stdout and stderr as separate streams for the diagnostic record and
  shall leave unchanged the combined console text that classification reads.

#### Failure phase

- The generator shall split stdout into harness segments: a segment starts at a line that is
  exactly `Checking harness <path>...`, `<path>` being a `module::harness` path the launch passed
  to `--harness`, and runs to the line before the next such line or to the end of stdout. A member
  has a segment when stdout holds a segment headed by its own path.
- The generator shall derive the failure phase `KaniFailurePhase` of each member whose evidence
  carries a diagnostic record (a `NoVerdict` member, or a `FailedWithoutCounterexample` member whose
  cause CBMC failure cause below names as a CBMC failure) from the
  run's exported report, that member's own segment and the group's streams, by these rules in
  order, the first that applies deciding:
  1. `unknown` when the run exported a report. For a report of success beside an unsuccessful exit,
     the report shows the backend finished, so no stage is named as the one that failed. For a
     report whose entry states a CBMC failure, the evidence's `failureCause` already names the
     stage that failed (CBMC), so the phase names none.
  2. `solver` when the member's segment holds a line beginning `Solving with ` (the line CBMC
     prints when it hands the problem to its SAT solver).
  3. `backend` when the member has a segment: Kani printed `Checking harness <path>...` for it,
     after the crate compiled and before it links and instruments the harness's goto program and
     starts CBMC, so the member reached Kani's per-harness verification pipeline.
  4. `unknown` when the member has no segment and stdout holds a segment of another member: the
     crate compiled, and the output shows no stage this member started.
  5. `compile` when stdout holds a line beginning `error: Failed to execute cargo (exit status: `
     or stderr holds a line beginning `error: could not compile ` (Kani's and Cargo's report of a
     crate that did not build).
  6. `launcher` when stdout holds no line beginning `Kani Rust Verifier ` (the launcher ended
     before Kani printed its banner, as on an argument error, which Kani reports on stderr with
     nothing on stdout).
  7. `unknown` otherwise.
- The generator shall derive a single run's phase by the same rules, the run being its group's one
  member.
- The generator shall name, by the phase, the last stage the output shows the member itself
  started, and shall name a stage that failed only by `compile` and `launcher`. The generator shall
  never give a member a phase taken from another member's segment: a member that never started is
  `unknown` even when a neighbour reached `solver`.
- When a marker the generator does not read appears (another solver's wording, a later Kani
  wording), the generator shall leave the phase at an earlier stage or at `unknown`, never at a
  later one: the phase under-claims and never over-claims. The generator shall classify a missing,
  unparsable or contradictory marker set as `unknown`, never as a refusal.
- Where the launch's option vector holds `-j`, `--jobs`, an argument beginning `--jobs=`, `-j<digits>` or `-j=<digits>` (at least one digit, nothing after them), the
  generator shall give `unknown` to every member that rules 2 to 4 would decide, because harnesses
  checked in parallel interleave their lines and a segment is no longer one member's output.

Rationale, not a requirement: Kani exports a report when CBMC itself fails (an unknown CBMC option,
CBMC out of memory, CBMC killed by a signal), with the harness entry `Failure`, no checks and an
`error_details` exit status. Such a run is classified from that report and is not `NoVerdict`;
CBMC failure cause below retains its cause. A `backend` or
`solver` phase therefore describes a Kani driver that ended without exporting its report after the
member's verification pipeline started.

#### Excerpts

- The generator shall form each excerpt from the stream's captured bytes by these steps, in order:
  1. decode the bytes as UTF-8, replacing each invalid sequence with U+FFFD;
  2. remove ANSI escape sequences, and replace every other control character except line feed and
     tab with U+FFFD;
  3. redact paths (below), then RFC 3339 date-times, then durations, then process identities, in
     one left-to-right pass per rule over the text outside placeholders already written, never
     rescanning a placeholder;
  4. keep the tail (below).
- The generator shall treat as a path token a maximal run of characters that starts with `/`, whose
  preceding character is the start of the text, a line feed, a space, a tab or one of `(` `[` `{`
  `'` `"` `` ` `` `=`, and that ends before the first space, tab, line feed or one of `)` `]` `}`
  `'` `"` `` ` `` `,` `;` `:`. A `/` preceded by any other character (as in `https://host/x` or
  `and/or`) starts no token, so a URL is left as written, and a closing `)` or a `:line:column`
  suffix after a path is never part of the token.
- The generator shall redact a path token against the known directories, the request's crate
  directory as `<CRATE_DIR>`, its target directory as `<TARGET_DIR>` and its home directory as
  `<HOME>`:
  1. the generator shall try the known directories longest first by byte length, and on equal
     length in the order crate, target, home, so a crate directory under the home directory is
     `<CRATE_DIR>`, not `<HOME>/…`;
  2. a known directory shall match only at a path-component boundary: the token equals it or
     continues with `/` directly after it, so `/x/crate2` is not `<CRATE_DIR>2` for crate directory
     `/x/crate`;
  3. on a match the generator shall replace only the matched prefix and keep the remainder, so
     `/x/crate/src/lib.rs` becomes `<CRATE_DIR>/src/lib.rs`;
  4. the generator shall never use as a known directory one that is `/`, empty or not absolute,
     and shall compare a known directory without a trailing `/`;
  5. a token that matches no known directory and holds a second `/` shall become `<PATH>`; a token
     with one `/` only (such as `/`) shall be left as written.
- The generator shall replace an RFC 3339 date-time with `<TIMESTAMP>`. The accepted form is
  `YYYY-MM-DD`, then `T`, `t` or one space, then `hh:mm:ss`, then an optional `.` with one or more
  digits, then `Z`, `z` or `+hh:mm` or `-hh:mm`, every `hh`, `mm`, `ss`, `MM` and `DD` being exactly
  two digits and every numeric field in range (month 01 to 12, day 01 to 31, hour 00 to 23, minute 00
  to 59, second 00 to 60, offset hour 00 to 23, offset minute 00 to 59). The date-time shall not be
  preceded by a letter, digit, `_` or `.`; a text that does not match the whole form is left as
  written.
- The generator shall replace a duration with `<DURATION>`: a decimal number not preceded by a
  letter, digit, `_` or `.`, with an optional fraction and an optional exponent (`e` or `E`, an
  optional `+` or `-`, digits), followed directly by `s` or `ms` and then by a character that is
  not a letter, digit or `_`, or by the end of the text. `1.385e-05s` and `0.02s` are durations;
  `check_ttl_30s` and `u32s` are not.
- The generator shall replace with `<PID>` the decimal number after the word `pid`, `PID` or
  `process` and one space, the word not preceded by a letter, digit or `_`, and the parenthesized
  decimal thread id after a quoted thread name in a `thread '<name>' (<id>) panicked` line.
- The generator shall keep the tail of the redacted text. A line is a run of characters ending in a
  line feed, or the characters after the last line feed. The final line is the last line holding a
  character other than line feed, together with every line feed after it. When the final line is
  at most 8192 bytes, the excerpt shall be the longest suffix of the redacted text that is at most
  8192 bytes and starts at the start of the text or directly after a line feed. When the final line
  exceeds 8192 bytes, the excerpt shall be the longest suffix of the redacted text that is at most
  8192 bytes and starts at a character boundary, so the closing line is kept to its last byte and
  is never lost to an empty excerpt. When the redacted text holds only line feeds, or is empty, the
  excerpt shall be the longest suffix that is at most 8192 bytes and starts at a line start.
- The generator shall give each excerpt its text and `omittedBytes`, the byte length of the
  redacted text before the kept tail, so a reader knows whether the excerpt is the whole redacted
  stream (`omittedBytes` 0); an empty stream shall yield an empty excerpt with `omittedBytes` 0.
- The generator shall make the excerpt a function of the captured stream bytes and the request's
  crate, target and home directories alone, reading no process environment, clock or process
  table for it: two runs whose streams differ only in redacted content shall yield byte-identical
  diagnostic records.
- The generator shall take an excerpt only from a stream it read whole within its
  8 MiB-per-harness limit: the excerpt is a copy cut for reading, not a truncated stream. A stream
  over that limit shall stay refused with `kani_output_over_limit`, and an unread stream with
  `kani_output_unread`, with no evidence and so no diagnostic record.

Rationale, not a requirement: the tail is kept because Kani and Cargo print the error that ended
the run last. 8192 bytes per stream holds the whole stdout and stderr of a measured failed build of
one harness (each under 1 KiB), and bounds the record at 16 KiB per harness, so a batch of N members
carries at most N times 16 KiB of diagnostic text however much its streams carried.

#### Batch members and wire shape

- Where a group's process exits unsuccessfully with no report, every member being `NoVerdict`, the
  generator shall give each member its own phase by the rules above and the excerpts of the
  group's streams, redacted with the group's one crate, target and home directory, so every
  member's redaction of the shared output is identical.
- Where a group's process exits unsuccessfully and its report states success for every member and
  failure for none, every member being `NoVerdict`, the generator shall give each member the phase
  `unknown` and the excerpts of the group's streams.
- Where a group's process exits unsuccessfully and its report states failure for at least one
  member, no member being `NoVerdict` under FR-017-AC-22, the generator shall give a diagnostic
  record only to a `FailedWithoutCounterexample` member that CBMC failure cause below gives one.
- The generator shall serialize the record as `diagnostics { phase, stdout { text, omittedBytes },
  stderr { text, omittedBytes } }`, with `phase` one of `launcher`, `compile`, `backend`, `solver`
  and `unknown`, and shall implement no deserialization for the record, its phase or its excerpts.

### CBMC failure cause

When CBMC itself fails, Kani still exports a report: the harness entry states failure, lists no
checks, and the report's `error_details` entry for the harness carries an `exit_status` naming why.
The run is classified from that report, as `FailedWithoutCounterexample`, so its evidence keeps the
cause the report states. The cause is diagnostic only: it never changes an outcome, a check, a
count, a playback or a refusal.

- The generator shall read, in `src/kani/output/report.rs`, the reported exit status of each
  harness result: the `exit_status` of the one `error_details` entry whose `harness_id` equals the
  result's `harness_id`. The harness has no reported exit status when no entry or more than one
  entry names it, when the result names no `harness_id`, or when that entry's `exit_status` is
  absent or null.
- The generator shall map a reported exit status to the closed vocabulary `KaniFailureCause`, the
  first rule that applies deciding:
  1. `properties_failed` is `properties_failed`: a property check failed.
  2. `timeout` is `timeout`: Kani's own per-harness timeout stopped the harness.
  3. `out_of_memory` is `out_of_memory`: Kani's statement that CBMC was killed by `SIGKILL`, which
     Kani reports as out of memory whatever sent the signal.
  4. `exit_code_` followed by a canonical decimal integer that fits a signed 32-bit integer (`0`,
     or an optional `-` then a digit 1 to 9 then any digits) is `exit_code` with that integer as
     `code`. The generator keeps the number Kani states and does not read it as an exit code or a
     signal number.
  5. Any other value is `unknown` with the value as `exitStatus`: the generator shall first apply
     excerpt steps 2 and 3 of Excerpts above to it (removal of ANSI escape sequences and
     replacement of every other control character except line feed and tab with U+FFFD; then
     redaction of paths against the request's crate, target and home directories, of RFC 3339
     date-times as `<TIMESTAMP>`, of durations as `<DURATION>` and of process identities as
     `<PID>`), and keep the result whole when it is at most 64
     bytes and otherwise cut to its longest prefix of at most 64 bytes that ends on a character
     boundary.
- When the classified outcome of a run or of a batch member is `KaniRunOutcome::Inconclusive` with
  `KaniInconclusiveReason::FailedWithoutCounterexample` and the harness has a reported exit status,
  the generator shall retain in that evidence the cause as `failureCause`. For every other
  outcome, and for a harness with no reported exit status, the evidence's wire form shall have no
  `failureCause` member.
- FR-017-AC-37 to FR-017-AC-39 extend FR-017-AC-36 for a report entry that states an exit status:
  FR-017-AC-36's group whose report has a Failure entry and exits 1 has no member record when that
  entry states no exit status; when the entry states a CBMC failure (FR-017-AC-39's `b::check` and
  `d::check`) the member carries the record below, and when it states `properties_failed` (the
  entry Kani writes for a failed assertion, FR-017-AC-39's `e::check`) it carries none.
- When that cause is `timeout`, `out_of_memory`, `exit_code` or `unknown`, the generator shall also
  retain in that evidence the run-diagnostics record that NoVerdict diagnostics defines (the same
  `diagnostics` member and record type, unrenamed), with its excerpts built by the same rules from
  the same streams and its phase `unknown`, because the run exported a report and `failureCause`
  already names CBMC as the stage that failed. When the cause is
  `properties_failed`, or the evidence has no cause, the generator shall retain no diagnostic
  record.
- The generator shall compute the outcome, the checks, the SUCCESS-check count, the playback and
  every refusal exactly as the rules above state, before and independently of the cause and the
  diagnostic record. A `FailedWithoutCounterexample` run stays
  `FailedWithoutCounterexample` whatever its cause. Its FR-029 terminal value is read from the cause
  by the terminal map alone (IR-848): `out_of_memory` gives `Incomplete(ResourceExhausted)`,
  `timeout` gives `Incomplete(TimedOut)`, and any other cause or none gives `Failed`. The cause
  never feeds classification, refusal or the batch timeout rule.
- The generator shall apply the batch timeout rule of FR-017-AC-22 to the `error_details` entries
  as that rule reads them, independently of the one-entry rule above: a member whose entry states
  failure with no checks and is named by an entry with exit status `timeout` is inconclusive timed
  out, and so carries no `failureCause`. A member whose `timeout` entry lists checks keeps the
  classification the rules above give it, and when that is `FailedWithoutCounterexample` carries
  the cause `timeout`. A single run's classification reads no exit status; a single run whose entry
  gives `timeout` stays `FailedWithoutCounterexample` with the cause `timeout`.
- The generator shall keep refusing as malformed a report whose `exit_status` is neither null nor a
  JSON string that decodes to Unicode text, as the report reader does today: a number, and a string
  holding a lone surrogate escape such as `"\ud800"`, refuse the report. The generator shall never
  refuse a report or a run for an exit status that is such a string and that the vocabulary above
  does not name; that value is `unknown`.
- The generator shall serialize the cause as `failureCause { kind }` with `kind` one of
  `properties_failed`, `timeout` and `out_of_memory`, as `failureCause { kind: "exit_code", code }`
  with `code` a JSON number, and as `failureCause { kind: "unknown", exitStatus }`, and shall
  implement no deserialization for the cause.

Rationale, not a requirement: Kani's report names a CBMC failure only by this exit status. Measured,
an unknown CBMC option gives `exit_code_64`, `SIGTERM` sent to CBMC gives `exit_code_15` (the signal
number), `SIGKILL` sent to CBMC gives `out_of_memory`, Kani's own per-harness timeout gives
`timeout` and a failed assertion gives `properties_failed`; a successful harness's entry has no
exit status. CBMC's own message (such as `Unknown option: --no-such-option`) is on the launcher's
stderr, which the diagnostic record's excerpt keeps.

Rationale, not a requirement: two splits follow from keeping classification unchanged, and IR-848
closes both in the FR-029 terminal map, which reads the cause. First, when a cgroup memory limit
outside CG's own meter kills only CBMC with `SIGKILL`, Kani reports `out_of_memory` and the run is
`FailedWithoutCounterexample` with cause `out_of_memory`; the same exhaustion seen by CG's own memory
stop is `MemoryExhausted`. Both now have the FR-029 terminal value `Incomplete(ResourceExhausted)`.
Kani reports any `SIGKILL` of CBMC as `out_of_memory`, and FR-029 accepts it without corroboration.
Second, a harness stopped by Kani's own per-harness timeout is `TimedOut` as a batch member with no
checks, but `FailedWithoutCounterexample` with cause `timeout` in a single run, which reads no exit
status for classification; both now have the terminal value `Incomplete(TimedOut)`. A single run
meets this only through a caller option, because the generator passes `--harness-timeout` only to a
batch. This requirement does not change either classification; the classification difference
remains and only the terminal value agrees.

### Scalar native error discriminant

`KaniScalarNativeError` (the failure type of `execute_kani_scalar_native`) is the only typed
failure a paired proof/native run returns. Runtime `Incomplete` and `Undefined` outcomes are
receipt content, never errors. This section gives the error a stable machine discriminant and a
normative split into completion refusals and environment or decode faults, so a consumer such as
the IR-773 driver (quire-driver FR-009-AC-7, "preserves an upstream typed completion refusal
unchanged") can pass a refusal through without remapping it. It extends FR-017-AC-27 to AC-31 and
changes none of their text.

- The scalar runner shall give `KaniScalarNativeError` a total method `code()` returning
  `&'static str`, never an `Option`, from the closed set below, and a total method `class()`
  returning `KaniScalarNativeErrorClass`, a closed two-value enum (`CompletionRefusal`,
  `EnvironmentFault`) that is not `non_exhaustive`. Each code is a snake_case string constant
  exported beside the type. `KaniScalarNativeErrorClass`, `KaniScalarNativeError`'s `code()` constants
  and `class()` are public at the crate root, re-exported beside `KaniScalarNativeError` in
  `src/lib.rs` (the constants as `SCALAR_NATIVE_CODE_INVALID_REQUEST` and so on, one per code).
  `Display` and `Debug` remain human text that no consumer shall parse.
- The scalar runner shall mark `KaniScalarNativeError` `#[non_exhaustive]`. A downstream `match`
  on its variants must then carry a wildcard arm; a consumer shall select on `class()` and
  `code()`, not on variants. Adding a variant is an additive Rust change but is a spec change
  (FR-017-AC-44).
- The scalar runner shall return the identity-encode failure (canonical proof-content identity
  could not be encoded within its byte limit, reported by `canonical_identity`) as the variant
  `IdentityEncode`, code `identity_encode`, and shall return `NativeResult` only for a malformed,
  missing or unsupported structured result from the native helper.
- The scalar runner shall return the failure to find a runtime revision in codegen's own embedded
  lock (today `InvalidRequest` in the lock-revision lookup) as the variant `CodegenLock`, code
  `codegen_lock`: it is fixed by codegen, not by the request, so it is an environment fault.
  `DependencyLock` remains only for a staged lock that does not name codegen's runtime revision after the update step. It is unreachable from a canonical request by a request fault: request validation requires the manifest to equal the canonical one (runtime on `branch = main`, no `rev`), and a package that differs is `InvalidRequest`. Every path left to it is a host or toolchain fault (`cargo update --precise` exits successfully but writes no matching revision, or the lock cannot be read), so its class is `EnvironmentFault`, and a test of it uses an in-crate seam.
- A completion refusal is a failure fixed by the request and staged content, or a typed result of
  the proof itself, that the consumer shall preserve unchanged. An environment or decode fault is
  a failure of the host, toolchain, helper or runner that the consumer shall not present as a
  refusal of the request.

| Variant | `code()` | `class()` | Rationale |
| --- | --- | --- | --- |
| `InvalidRequest` | `invalid_request` | `CompletionRefusal` | Fixed by the request: identity, package, operand or limit mismatch, found before either child starts. |
| `ArtifactPackage` | `artifact_package` | `CompletionRefusal` | Fixed by the supplied package bytes and paths against the standard bounds. |
| `CodegenLock` (new) | `codegen_lock` | `EnvironmentFault` | Codegen's own embedded lock names no runtime revision: a build or packaging fault of codegen, never of the request. |
| `Snapshot` | `snapshot_io` | `EnvironmentFault` | Host I/O failure creating, writing or measuring the private snapshot. |
| `Kani` | `kani_execution` | by the nested refusal, below | The nested `KaniExecutionRefusal` mixes request-fixed refusals and host faults, so the outer variant is neither always. |
| `ProofNotVerified` | `proof_not_verified` | by the carried evidence outcome, below | On a single run, cancellation, timeout, memory exhaustion and no-verdict all arrive as `ProofNotVerified`, while their batch twins are `Kani` faults; the class follows the outcome so the two paths agree. |
| `OracleStop` | `oracle_stop` | `CompletionRefusal` | The runtime oracle rejected a generated operand, constant or width: a typed, deterministic stop. |
| `NativeLaunch` | `native_launch` | `EnvironmentFault` | The bounded native build or invocation did not complete: toolchain or host. |
| `DependencyLock` | `dependency_lock` | `EnvironmentFault` | The staged lock does not name codegen's runtime revision after the update step: `cargo update --precise` wrote no matching revision or the lock was unreadable. A host or toolchain fault, not fixed by the request (see above). |
| `NativeResult` | `native_result` | `EnvironmentFault` | The helper returned malformed, missing or unsupported structured output: a decode fault. |
| `SnapshotChanged` | `snapshot_changed` | `EnvironmentFault` | Staged bytes changed during a child run. The change is not determined by the request, so a retry can succeed; it is an integrity fault, not a refusal of the request. (IR-847's description listed it as a refusal; this spec decides otherwise.) |
| `IdentityEncode` | `identity_encode` | `EnvironmentFault` | Encoding the canonical identity of already-bounded content failed: an internal runner fault. |

For `ProofNotVerified`, `class()` is `CompletionRefusal` when the evidence outcome is a genuine
property or proof verdict, or a no-cause failure with no checks (no verdict evidence, classed as refusal by rule): `Falsified`, `CoverUnsatisfied`, or `Inconclusive` with reason
`MissingCoverSummary`, `VacuousProof`, `UnwindBoundExhausted`, or `FailedWithoutCounterexample`
with `failureCause` `properties_failed` (a failed property, which real Kani writes for a failed assertion without playback), or with no `failureCause` in either of two shapes: a failed check listed with no exit status stated (a failed property), or no checks and no exit status (no verdict evidence, classed as refusal by rule; FR-029 maps it to `Failed` as a tool-contract break). It is `EnvironmentFault` when the run did not reach a
verdict: `Cancelled`, `Inconclusive` with reason `TimedOut`, `MemoryExhausted` or `NoVerdict`, and
`FailedWithoutCounterexample` with a `failureCause` other than `properties_failed` (a CBMC timeout,
out-of-memory, exit code or unknown failure, as `is_cbmc_failure()` tells). `ProofNotVerified` carrying outcome `Verified` is an internal inconsistency: it is an `EnvironmentFault`, returned without a panic (NFR-005), and is part of the frozen mapping. This is consistent with the FR-029 mapping of a
timeout or memory exhaustion to an incomplete terminal (IR-848): such a run is not a verdict.
The evidence is carried unchanged for either class. `class()` matches `KaniRunOutcome`,
`KaniInconclusiveReason` and `KaniFailureCause` exhaustively, with no wildcard arm, as it does the
nested refusals, so a new outcome, reason or cause forces a spec decision.

For `Kani`, `class()` is `CompletionRefusal` when the nested refusal is `HarnessNotInCrate`,
`HarnessProvenanceMismatch`, `OutputOverLimit` or `PlaybackForNonMember` (fixed by the harness,
crate or captured output), and `EnvironmentFault` for `Tool`, `MemoryMechanismUnavailable`,
`MemoryObservationFailed`, `BatchMemoryExhausted`, `BatchTimedOut`, `BatchCancelled`,
`OutputUnread` and `Report` (host, backend, cancellation or report-decode faults). The
nested match is exhaustive over `KaniExecutionRefusal`, so a new nested variant forces this
spec's decision. The outer `code()` of `Kani` is always `kani_execution`; the nested
`KaniExecutionRefusal::code()` is unchanged and remains `Some` only for `OutputOverLimit` and
`OutputUnread`. Giving every nested refusal a stable code is not required here.

Stability contract: the twelve codes above are the whole set. A code is never renamed, reused for
another variant, or removed while its variant exists. A variant's class, or for `ProofNotVerified` and `Kani` its mapping to class, never changes. Adding a
variant, code or nested classification is a spec change to this section, made with its tests.

### Scalar proof-content identity as the FR-357 observation digest

The measured proof-content identity of FR-017-AC-29 is the value QSL FR-357 carries as the
**canonical generated-content identity of the observation**
([QSL FR-357](ix://agent-ix/quire-spec-language/FR-357), Description and AC-14). This section fixes its digest domain, its exact preimage, who mints it and
who authenticates it, so the IR-773 driver can recompute it. It extends FR-017-AC-29; FR-032 refers here for the representation.

The v3 companion execution/custody behavior and its new or extended verification obligations
are **Planned / UNRUN** until implemented and qualified. This amendment claims no companion
compile, sealed invocation, proof success, independent-resolution fix or IR-17 completion.

- **Domain.** The scalar runner shall expose the identity as a `qsl_replay::DigestRecord`, which the crate re-exports at its root (`quire_contract_codegen::DigestRecord`) so a consumer names one type and CG defines no twin, whose domain is
  `sha256-jcs` and whose bytes are the 32-byte SHA-256 of the RFC 8785 (JCS) canonical encoding of
  the structural preimage below. `sha256-jcs` is chosen because the preimage is a structured JSON
  document of several artifacts, not one byte string; QSL's `GeneratedArtifactBytesV1` is
  raw-byte-addressed and would be correct only if CG hashed one artifact's raw bytes, which it does
  not. QSL accepts either domain as an opaque `DigestRecord`; the choice is CG's and is this one.
- **Preimage.** The preimage is the JSON object `{"schema": "agent-ix/cg-kani-scalar-proof-content/v3",
  "artifacts": [ {"path": P, "contents": C}, ... ]}`. `artifacts` contains exactly ten entries:
  the four original emitted artifacts (`Cargo.toml`, `claim-map.json`, `source-maps/lib.json`,
  and the emitted `src/lib.rs` retained as `src/oracle-artifacts-lib.rs`); the retained scalar
  harness at `src/lib.rs`; its record at `kani-obligations/<module>.json`; the production
  `Cargo.lock`; the single runner-generated adapter at `src/bin/ir791_native.rs`; and
  `observer/Cargo.toml` and its independently generated `observer/Cargo.lock`. The source map
  is an original emitted artifact, not optional metadata. This explicitly corrects the previous
  seven-entry description to include the eight original retained entries before adding the two
  companion entries; none is dropped or replaced.
  `path` is the relative, `/`-separated path in the staged tree; `contents` is the file's bytes as
  a UTF-8 string. A non-UTF-8 runner-read input, including either lock, observer manifest or
  adapter, is refused without lossy conversion as `Snapshot` / `snapshot_io` / `EnvironmentFault`,
  not `InvalidRequest`; request artifacts are already `String`. The array is ordered by `path`,
  ascending in UTF-8 byte order, without duplicates; JCS orders object members but not arrays.
  Every actual staged source/build input shall be in this closed set; an additional file is a
  snapshot change, not an unmeasured extension. The encoded preimage ceiling remains 256 MiB;
  exceeding it is `IdentityEncode` (FR-017-AC-43).
- **Private observer execution.** Existing `kani/run` staging owns the companion lifecycle and
  `core/profile` owns its dependency source spellings. The production emitted manifest remains
  byte-identical: its normal runtime dependency uses `exact`, its normal exact dependency enables
  no `test-support`, and no normal/build dependency or CLI feature override may enable that feature.
  The private `observer/Cargo.toml` shall define a non-published isolated package/workspace with resolver 2,
  automatic binary/example discovery disabled, and one explicit example `ir791_native` whose
  source path is `../src/bin/ir791_native.rs`. Its dev-dependencies expose both the owning runtime
  with `exact` and the owning exact kernel with `test-support`, using the existing authoritative
  source spellings. Only that dev edge enables ordered charge logging. It has no library, wrapper,
  copied adapter, symlink, production-package dependency or normal/build dependency enabling
  logging. The canonical adapter loads the retained harness at its actual bin-relative
  `../lib.rs` path, invokes the real embedded oracle with one real meter, and reads that meter's
  actual limits, counters and admitted-charge log through the owning API; no synthesized charge
  oracle or new observation API is assumed. Owning API corrections change adapter bytes normally.
  Native execution uses bounded `cargo run --quiet --locked --manifest-path observer/Cargo.toml
  --example ir791_native -- <ordered operands> <exact limits>` from the sealed root, with its
  target directory outside the snapshot. Kani still executes the unchanged retained proof harness
  from the production root, with its retained ordered options, claims and covers and the existing
  backend gates. Production validation selects the library/proof targets required by that runner;
  compiling every auto-discovered target with `--all-targets` is not the feature-free production
  observer contract. Target selection shall never skip a required proof, claim or cover.
  The manifests, adapter, harness and record bind the build configuration and retained ordered
  proof options; concrete operands and limits remain separately bound receipt inputs below.
- **Independent locks and custody.** Before measurement, the runner freshly generates a lock in
  each package root through the existing bounded offline resolution/update workflow and verifies
  both owning runtime and exact package identities against this build's authoritative Cargo.lock.
  Neither lock is copied from the parent or the other package, and equality of their bytes is not
  assumed. Both locks shall resolve the same owning runtime/exact identities; no cache substitution,
  dependency downgrade or extra version pin may stand in for fresh resolution. The independent
  observer regression shall assert the runtime and exact identities with literal owning package
  names independently of the renderer's dependency list. After both locks exist, the runner seals
  the entire snapshot, captures and measures every entry, and checks the complete path/byte set
  before proof, after proof and after native execution. Existing bounded launch, cancellation,
  error classification, receipt opacity and explicit cleanup behavior apply to both package roots.
- **Exclusions.** The preimage binds the generated artifact alone. Operands, `ScalarLimits`, operand
  and result ranges, the native outcome, the Kani evidence, the meter observation and the scalar
  identity are not in it (FR-017-AC-29, AC-31). They are separate claim fields that QSL FR-357 binds
  in the claim identity; the observation digest shall not bind them, because binding them twice
  breaks FR-357-AC-14. No separate tool/version record is added; actual manifest, lock and adapter bytes
  remain bound.
- **Owners.** CG mints the identity at proof time, from the staged bytes, in
  `KaniScalarNativeReceipt`; the receipt exposes it read-only as the `DigestRecord` above, and its
  bytes equal `ScalarProofContentIdentity::as_bytes()`. The IR-773 driver authenticates it, by
  recomputing it (below) from the artifact it executes before it sends the claim. QSL carries it and
  binds it into the claim identity; QSL never mints, recomputes or authenticates it. A consumer
  shall not mint a wire value from operands, limits or outcome; the only legitimate recomputation
  is the driver's, over the exposed staged entries, below.
- **Retention for recomputation.** The receipt shall retain and expose read-only the complete
  ordered `(path, bytes)` list measured above, including both manifests and locks and the canonical
  adapter. It shall bind those entries to the immutable snapshot actually executed, captured at
  measurement time and owned by the receipt, never a later filesystem read. The driver recomputes
  `sha256-jcs` over the full set, requires equality with the receipt's `DigestRecord`, and compares
  all driver-owned originals byte for byte under the rename map: emitted `Cargo.toml`,
  `claim-map.json` and `source-maps/lib.json` at their own paths; emitted `src/lib.rs` at
  `src/oracle-artifacts-lib.rs`; retained harness at `src/lib.rs`; and retained record at
  `kani-obligations/<module>.json`. The runner shall preserve those original bytes, including the
  emitted manifest, without rewriting them for observation. Any mismatch is a driver-side typed
  refusal with no claim sent. **Trust boundary.** The companion manifest, both fresh locks and
  adapter are CG-owned runner bytes. The driver relies on CG's sealed receipt for them and checks
  their retained bytes and digest consistency; it does not independently reproduce them. Driver
  authentication means comparison of its own originals plus consistency over the complete set.
- **Driver follow-up.** quire-driver FR-009 (statement and AC-7) says the driver verifies "QSL's
  observation digest against that outcome". That conflicts with this section and must be amended:
  the observation digest is artifact-only and authenticated by recomputation over the receipt's
  exposed entries, not checked against the outcome. The amendment is quire-driver PR #78 (FR-009/TC-016), owned by the quire-driver team
  (routed from IR-846) and blocks quire-driver IR-773 AC-6/7.
- **Meter-used limits (IR-857).** The driver must show that the request limits, the limits the meter
  actually used and the limits the receipt returns are the same value before it compares them with
  its retained original (quire-driver FR-009-AC-7; IR-773 AC-12), and the receipt today exposes only
  `scalar_limits()`. The meter observation shall therefore carry the `ScalarLimits` the native meter
  was constructed with, read back from the native invocation itself (the adapter reports the limits
  its meter holds), not copied from the request, and bound to the executed snapshot like the other
  receipt fields (captured at measurement time, immutable, no filesystem read). CG guarantees that
  the meter-used limits equal the request's limits and the receipt's `scalar_limits()`; when the
  reported meter limits differ from the request, the runner returns a typed failure and no receipt
  (`NativeResult`, class `EnvironmentFault`). A meter limit that differs from the request makes the helper's structured result unsupported in the sense of FR-017-AC-43, so `NativeResult` is the correct variant under that rule; it is not an exception to it. A well-formed report of a different value is unsupported, not malformed.
  CG does not attest the provenance of the request limits (FR-017-AC-28).
- **Stability.** The schema string is the version of the preimage. Any change to the artifact set,
  a path name or rename, the ordering, the content encoding, the field names, the canonicalization
  or the hash shall change the schema string (the companion artifact set uses `.../v3`, replacing `.../v2`), and digests under different
  schema strings are never equal or comparable. The domain stays `sha256-jcs`. A change to the
  bytes of a staged input (either manifest, either lock or the adapter) changes the digest without a schema
  bump, because it names different proved content. The golden digest is computed over fixed literal entries (both manifests, both locks, source map and adapter
  included), never a real staged run, so dependency bumps do not move it, and is pinned together
  with the schema string in one assertion. A test cannot detect a golden edited beside an unchanged
  schema string, so that is a review rule: a change to the golden without a schema bump is
  rejected in review. The stale description of the
  digest as "length-framed" appears in the code doc comment of `ScalarProofContentIdentity` only,
  not in spec text; correcting it to "SHA-256 of the RFC 8785 canonical encoding" is code-phase work.

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
| FR-017-AC-19 | The generator appends `-Z unstable-options --export-json /proc/self/fd/N` after unchanged harness options, with actual child-only anonymous pipe writer N >= 5, and records the actual argv bytes. It uses per-run unnamed report authority, never a named target-directory report or fallback. FR-034 AC-32/33 own collector bounds, resource-stop classification, kernel lifetime and immutable final handoff. Zero-byte EOF means no exported report; nonempty partial/malformed content is refused under AC-18 unless an independently established resource/deadline stop takes precedence with the existing single-run or whole-batch classification. | Test (TC-027) |
| FR-017-AC-20 | The evidence and the classified run list every check of a real run with its id, class, source file and line and status, a line stated as unknown is absent, and a non-numeric line or a check with no location is a refused report; the view serializes as `id`, `class`, `location { file, line }` and `status` and is not deserializable. | Test (TC-027) |
| FR-017-AC-21 | N harnesses with equal option vectors (the harness selection removed) and equal identity `ProofCeilings` (`wall_clock` T and `memory_bytes` M) start exactly one launcher process whose argument vector holds one `--harness <module::harness> --exact` pair per member in request order, `--harness-timeout` T, then the shared options; N harnesses in G such groups start G processes; so N > 1 compatible harnesses start fewer than N processes; one harness starts one process with the single-run argument vector. Requests whose wall-clock or memory ceilings differ start separate processes. The group's process tree is held to one aggregate M; memory exhaustion kills and refuses the whole group with no member classified. The group's outer wall-clock bound is N times T, never elapsing when the product does not fit, and a group killed at it leaves no report and is refused as timed out with no member classified. A T that rounded up exceeds 4294967295 seconds (a `Duration::MAX` identity ceiling) launches without `--harness-timeout`. A group that exits successfully with no report is refused with the missing-report refusal and classifies no member; one that exits unsuccessfully with no report (an argument error, a failed build) has every member inconclusive `NoVerdict`. Requests that name another launcher, crate directory or target directory are never grouped. | Test (TC-043) |
| FR-017-AC-22 | Over a real or captured batch report, members are matched by `module::harness` path (two members with the bare symbol `check` stay distinct), and each member's outcome, checks and SUCCESS count come from its own entry: a falsified member beside a verified member leaves the verified member verified; a Success member in a non-zero exit with no Failure entry is inconclusive, and with one Failure entry is verified; a Failure entry with no checks and exit status `timeout` is inconclusive timed-out naming T while the others keep their results; a Failure entry with no checks and no timeout is inconclusive with no counterexample; each member's evidence carries its kind, harness path, launcher path, unwind bound, solver, outcome and checks plus the group's argument vector, the member list, a statement that it was a batch, and the exit code. | Test (TC-043) |
| FR-017-AC-23 | A group's report that lacks a requested harness, holds one twice or holds an unrequested one refuses the group with a typed cause and classifies no member; a member whose source is not in the crate refuses the whole batch with `HarnessNotInCrate` and launches nothing; a falsified member's playback is the first counterexample block headed for its own path and never another member's block, a member that fails two property checks (two counterexample blocks under its path, as real Kani prints) takes the first, as the same harness alone does, and neither it nor its neighbours lose their evidence, a failed property check with no block headed for its path is that member inconclusive with no counterexample, as in a single run; a block headed for a path that is not a member refuses the group. | Test (TC-043) |
| FR-017-AC-24 | A launcher that exits on its own after leaving a real grandchild in its process group has that grandchild killed by the time the run returns, the same as on a timeout. | Test (TC-043) |
| FR-017-AC-25 | A capture thread that panics, or whose poll or read of the stream errs, refuses the run with `kani_output_unread`; the run is never classified from empty text. | Test (TC-043) |
| FR-017-AC-26 | A generated FR-035 caller Text-admission harness enters the same production execution path in single and compatible batch runs; its evidence carries the exact typed caller identity (profile, bounds and finite payload class) supplied with that harness and `None` for contract obligation kind. A matching successful report with a satisfied cover and successful agreement assertion classifies `Verified`; an unsatisfied cover classifies `CoverUnsatisfied` with counts; a failed agreement assertion with its own concrete playback classifies `Falsified` with that playback. A source-mismatched harness, missing launcher or malformed report retains the existing typed refusal and yields no verified caller claim; a report with no cover or a failed assertion with no playback is inconclusive under the existing rules. The outcome and playback remain bound to the caller identity and selected harness path, never to a checked node or another batch member. PLANNED (IR-494). | Test (TC-027, TC-050) |
| FR-017-AC-27 | A genuine routed `x + 1` scalar harness from `generate_routed`, its complete emitted oracle package and one in-range concrete operand are passed to the paired runner. One real Kani run reports `Verified`, and the runner invokes the public oracle from the same staged library source at that operand with the request's exact `ScalarLimits`; the receipt contains the Kani evidence and the actual typed runtime result of that invocation. A Kani verdict alone or a result reconstructed by the caller is insufficient. | Test (TC-027) |
| FR-017-AC-28 | Before either child starts, a package or harness whose path or bytes differ from the retained routed output, a selected oracle symbol or argument order that differs from the retained scalar identity, or an operand count/value outside the retained ordered ranges is refused with a typed cause and no receipt. For an accepted request, the exact supplied `ScalarLimits` are passed unchanged to `KaniScalarNativeRequest`, used to initialize the native meter, and returned in the receipt; no hidden or substituted limit value is permitted. CG attests these supplied, used and returned values are equal, but does not attest their provenance. The IR-773 driver must compare its retained original limits against both the request and receipt before accepting the receipt; on mismatch it detects locally, refuses receipt acceptance and skips QSL evaluation. If IR-773 has no defined typed outcome for this mismatch, its owner must specify and plan one before implementing the consumer check; CG defines no typed outcome for this mismatch. A caller-provided content identity cannot make an inconsistent request acceptable. | Test (TC-027) |
| FR-017-AC-29 | The runner measures one canonical proof-content identity over the exact paths and bytes of every staged execution input: the complete generated package, retained harness, canonical native invocation adapter, observer manifest and both independently generated dependency locks. This identity binds the content actually proved and executed; it is not a tool-version or file-version tracking identity, and no separate identity is added for any staged item. The Kani and native invocations cannot write the staged source tree. A changed, replaced or additional staged file before or after either invocation yields a typed failure and no receipt. The receipt separately binds the scalar identity, oracle symbol, ordered concrete operands and exact supplied `ScalarLimits`; none of these request fields may be substituted after measurement or inferred from the proof-content identity. | Test |
| FR-017-AC-30 | The public receipt exposes the measured identity, scalar identity, ordered operands, exact supplied and meter-used `ScalarLimits`, verified Kani evidence and actual typed native outcome through read-only access. Callers cannot construct, deserialize or mutate a receipt, set its identity or outcome, or turn any non-verified, refused, inconclusive, faulted or malformed run into a receipt. | Test (TC-027) |
| FR-017-AC-31 | The paired runner executes an order-sensitive routed scalar operation with two symbolic operands in two concrete operand orders. Each real Kani run is verified; the native invocation returns the result for that order; and each receipt binds its exact ordered operand vector and typed native result. Swapping the operands changes the observed result and receipt context while retaining the same generated proof-content identity. | Test (TC-027) |
| FR-017-AC-32 | In the `make kani` lane, a crate whose library source holds the generated harness byte for byte and, beside it, an item that does not compile (a reference to an undefined name) is run through `execute_kani_obligation`: the run exits unsuccessfully, exports no report and is `Inconclusive` `NoVerdict` with its exit code, and its evidence carries a diagnostic record whose phase is `compile`, whose stdout excerpt holds the rustc error code and Kani's `error: Failed to execute cargo` line and whose stderr excerpt holds Cargo's `error: could not compile` line with the crate path as `<CRATE_DIR>`. A launcher stand-in that exits 1 with no report and prints nothing is `NoVerdict` with phase `launcher` and two empty excerpts with `omittedBytes` 0. Evidence of a verified, cover-unsatisfied, falsified, timed-out, memory-exhausted or other inconclusive run has no `diagnostics` member on the wire, apart from the `FailedWithoutCounterexample` evidence FR-017-AC-37 gives one, and every refusal returns no evidence and no record. | Test (TC-027) |
| FR-017-AC-33 | Over single-run launcher stand-ins that exit 1: a report of success beside the unsuccessful exit is `unknown` even when its stdout holds `Solving with CaDiCaL 3.0.0`; with no report, stdout holding the Kani banner, `Checking harness proofs::check...` and `Solving with CaDiCaL 3.0.0` is `solver`; the banner and the `Checking harness` line alone is `backend`; the banner and `error: Failed to execute cargo (exit status: 101). Found 1 compilation errors.` is `compile`, as is stderr `error: could not compile` alone with the banner; a usage error on stderr and nothing on stdout is `launcher`; the banner alone is `unknown`; a solver line in a wording other than `Solving with ` after the `Checking harness` line is `backend`, never `solver`; a `Checking harness` line for a path the launch did not pass starts no segment, so the run is `unknown`; `-j 2` in the option vector with the `solver` stdout above is `unknown`. The phase is the same when the marker lies before the 8192-byte excerpt window. | Test (TC-027) |
| FR-017-AC-34 | A redacted stream of exactly 8192 bytes is kept whole with `omittedBytes` 0; one of 8193 bytes whose final line starts within the window keeps the longest line-aligned suffix of at most 8192 bytes with `omittedBytes` the byte count before it; a stream whose final line alone exceeds 8192 bytes keeps at most 8192 bytes ending at its last byte, starting on a character boundary; a stream that is one 10000-byte line followed by `\n` keeps the last 8192 bytes of it, ending in `\n`, never an empty excerpt; a stream of a 10000-byte line, `\n`, `error: last\n` keeps exactly `error: last\n`; a 4-byte character straddling the cut is never split; invalid UTF-8 is decoded with U+FFFD; ANSI escapes are removed; a stream of only line feeds is kept whole up to 8192 bytes. No excerpt exceeds 8192 bytes, and a stream at its 8 MiB limit still yields an excerpt of at most 8192 bytes. | Test (TC-027) |
| FR-017-AC-35 | With crate directory `/h/u/ws/crate`, target directory `/h/u/ws/crate/target` and home directory `/h/u` supplied in the request, a stand-in whose streams print `(/h/u/ws/crate)`, `/h/u/ws/crate/src/lib.rs:6:21`, `/h/u/ws/crate/target/debug/x.out`, `/h/u/.kani/bin`, `/h/u/ws/crate2/src/lib.rs`, `/opt/tool/bin`, `https://host/x/y`, `and/or`, `/`, `2026-10-16T22:27:04.275975036Z`, `Runtime Symex: 0.00331318s`, `Runtime Solver: 1.385e-05s`, `in 0.02s`, `check_ttl_30s`, `u32s`, `pid 4242` and `thread 'main' (4242) panicked` yields, in order, `(<CRATE_DIR>)`, `<CRATE_DIR>/src/lib.rs:6:21`, `<TARGET_DIR>/debug/x.out`, `<HOME>/.kani/bin`, `<HOME>/ws/crate2/src/lib.rs`, `<PATH>`, `https://host/x/y`, `and/or`, `/`, `<TIMESTAMP>`, `Runtime Symex: <DURATION>`, `Runtime Solver: <DURATION>`, `in <DURATION>`, `check_ttl_30s`, `u32s`, `pid <PID>` and `thread 'main' (<PID>) panicked`. A home directory of `/` redacts nothing as `<HOME>`, and an absent one leaves `/h/u/.kani/bin` as `<PATH>`. The generator reads no `HOME` from its environment: the same run under two different process `HOME` values gives the same record. Two runs whose streams differ only in those redacted values serialize byte-identical records. The record serializes as `diagnostics { phase, stdout { text, omittedBytes }, stderr { text, omittedBytes } }`; a rustdoc `compile_fail` example on the record type that calls `serde_json::from_str` for it fails to compile, beside a compiling control that calls `serde_json::to_string` on the same value. | Test (TC-027) |
| FR-017-AC-36 | The diagnostic record never feeds classification: two runs with the same report and exit and different stream text classify to the same outcome, checks, SUCCESS count and playback, and two `NoVerdict` runs with equal exit and no report whose streams differ have equal evidence in every field but `diagnostics`, `memory` (the observed peak resident bytes vary run to run) and the run's report-path argument (the last argument, unique to each run); the FR-029 terminal value of a `NoVerdict` evidence carrying a record is `Failed`, the value FR-029-AC-4 gives the no-verdict reason, equal to the terminal value of the same evidence with `diagnostics` absent. A group of members A, B and C whose process exits 1 with no report, whose stdout holds `Checking harness a::check...`, `Solving with CaDiCaL 3.0.0` and then `Checking harness b::check...`, gives A `solver`, B `backend` and C `unknown`, all `NoVerdict` with the group's excerpts; the same group whose stdout holds a compile failure and no segment gives every member `compile`. A group whose report has every member Success and whose process exits 1 gives every member `NoVerdict` with phase `unknown`; one whose report has a Failure entry and exits 1 has no member `NoVerdict` and no member record. A stream over 8 MiB times the harness count stays refused with `kani_output_over_limit`, and an unread stream with `kani_output_unread`, with no evidence and no record. Two otherwise compatible requests whose `home_directory` differs (one `/h/u`, one `/h/v`, or one absent) start two processes; two with the same `home_directory` start one, and their members' excerpts of the shared output are byte-identical. | Test (TC-043) |
| FR-017-AC-37 | Over single-run launcher stand-ins that exit 1, print `Checking harness proofs::check...` and `CBMC failed with status 64` on stdout and `Unknown option: --no-such-option` on stderr, and export a report whose one result `proofs::check` is `Failure` with no checks and whose one `error_details` entry for `proofs::check` has the exit status shown: `exit_code_64` is `Inconclusive` `FailedWithoutCounterexample` with `failureCause` `{ "kind": "exit_code", "code": 64 }` and a `diagnostics` record of phase `unknown` whose stderr excerpt holds `Unknown option: --no-such-option`; `exit_code_15` has `code` 15; `exit_code_0` has `code` 0; `exit_code_-1` has `code` -1; `exit_code_2147483647` has `code` 2147483647; `exit_code_-2147483648` has `code` -2147483648; `out_of_memory` is `{ "kind": "out_of_memory" }`; `timeout` is `{ "kind": "timeout" }` and stays `FailedWithoutCounterexample`; `cbmc_exploded` is `{ "kind": "unknown", "exitStatus": "cbmc_exploded" }`; `exit_code_`, `exit_code_064`, `exit_code_+64`, `exit_code_-0`, `exit_code_2147483648`, `exit_code_64x` and `EXIT_CODE_64` are each `unknown` with that value as `exitStatus`; a 64-byte ASCII value is `unknown` with all 64 bytes, a 65-byte and a 100-byte ASCII value are `unknown` with their first 64 bytes, and a value whose 64th byte falls inside a 4-byte character is cut before that character; with crate directory `/h/u/ws/crate` in the request, the value `/h/u/ws/crate/x` is `unknown` with `exitStatus` `<CRATE_DIR>/x`, a value holding an ANSI escape is `unknown` with the escape removed, and the value `killed pid 4242 after 3.5s` is `unknown` with `exitStatus` `killed pid <PID> after <DURATION>`. Every one of these evidences carries a `diagnostics` record. | Test (TC-027) |
| FR-017-AC-38 | A single-run stand-in that exits 1 and exports a `Failure` report listing a failed assertion check with no playback on the console, whose entry for the harness has exit status `properties_failed`, is `FailedWithoutCounterexample` with `failureCause` `{ "kind": "properties_failed" }` and no `diagnostics` member. The same report with no `error_details` member, with an entry that names another harness only, with two entries naming the harness, with an entry whose `exit_status` is null, or whose one result names no `harness_id`, is `FailedWithoutCounterexample` with neither a `failureCause` nor a `diagnostics` member. An entry with exit status `exit_code_64` beside a verified or cover-unsatisfied report with exit 0, or beside a falsified (its playback printed) or unwind-bound-exhausted report with exit 1, leaves that outcome unchanged with no `failureCause` member; a `NoVerdict` run (a success report beside exit 1 with that entry, or no report), a timed-out run and a memory-exhausted run have none either. A report whose `exit_status` is the number `64`, and one whose `exit_status` is the string `"\ud800"` (a lone surrogate escape), each stay refused as malformed with no evidence. A rustdoc `compile_fail` example on `KaniFailureCause` that calls `serde_json::from_str` for it fails to compile, beside a compiling control that calls `serde_json::to_string` on the same value. | Test (TC-027) |
| FR-017-AC-39 | A batch stand-in of members `a::check`, `b::check`, `c::check`, `d::check` and `e::check` whose process exits 1 and whose report has `a::check` `Success` with a passing assertion, every cover satisfied and no exit status, `b::check` `Failure` with no checks and exit status `exit_code_64`, `c::check` `Failure` with no checks and exit status `timeout`, `d::check` `Failure` listing a failed assertion with no playback block headed for its path and exit status `timeout`, and `e::check` `Failure` listing a failed assertion with no playback block headed for its path and exit status `properties_failed` (the entry real Kani writes for a failed assertion), gives `a::check` `Verified` with no `failureCause`; `b::check` `FailedWithoutCounterexample` with `{ "kind": "exit_code", "code": 64 }` and a `diagnostics` record of phase `unknown` holding the group's excerpts; `c::check` inconclusive timed out naming T, with no `failureCause` and no `diagnostics`; `d::check` `FailedWithoutCounterexample` with `{ "kind": "timeout" }` and a `diagnostics` record; and `e::check` `FailedWithoutCounterexample` with `{ "kind": "properties_failed" }` and no `diagnostics` member. The same group whose report holds two entries for `b::check`, one `timeout` and one `exit_code_64`, gives `b::check` inconclusive timed out with no `failureCause`, the timeout rule reading the entries as before. | Test (TC-043) |
| FR-017-AC-40 | The cause never feeds classification: three single-run stand-ins with the same `Failure` report, no checks and the same exit, whose entries give `exit_code_64`, `out_of_memory` and `cbmc_exploded` and whose streams differ, classify to the same outcome, checks, SUCCESS count and playback, and their evidence is equal in every field but `failureCause`, `diagnostics`, `memory` and the report-path argument; the cause is the only input of the FR-029 terminal value that differs: `exit_code_64` and `cbmc_exploded` give `Failed`, equal to the terminal value of the same evidence with `failureCause` and `diagnostics` absent, and `out_of_memory` gives `Incomplete(ResourceExhausted)` (IR-848, FR-029-AC-32). In the `make kani` lane, a generated harness in a crate whose manifest carries `[package.metadata.kani]` with `unstable = { unstable-options = true }` and `flags = { cbmc-args = ["--no-such-option"] }`, its option vector unchanged, is run through `execute_kani_obligation`: the run exits unsuccessfully, exports a report and is `Inconclusive` `FailedWithoutCounterexample` with `failureCause` `{ "kind": "exit_code", "code": 64 }` and a `diagnostics` record whose stderr excerpt holds `Unknown option: --no-such-option`. | Test (TC-027, TC-040) |
| FR-017-AC-41 | Every `KaniScalarNativeError` variant has a `code()` that is a total `&'static str` from exactly this set: `invalid_request`, `artifact_package`, `snapshot_io`, `kani_execution`, `proof_not_verified`, `oracle_stop`, `native_launch`, `dependency_lock`, `native_result`, `snapshot_changed`, `identity_encode`, `codegen_lock`. The set is closed and the codes are pairwise distinct. The type is `#[non_exhaustive]` (a rustdoc `compile_fail` exhaustive match without a wildcard arm, beside a compiling control with one), and `class()` returns a closed `KaniScalarNativeErrorClass` (a compiling exhaustive two-arm control with no wildcard, and no `#[non_exhaustive]` on it). For a `Kani` error the code is `kani_execution` for every nested refusal. | Test (TC-027) |
| FR-017-AC-42 | `class()` is `CompletionRefusal` for `invalid_request`, `artifact_package` and `oracle_stop`, and `EnvironmentFault` for `snapshot_io`, `native_launch`, `dependency_lock`, `native_result`, `snapshot_changed`, `identity_encode` and `codegen_lock`. For `proof_not_verified` it is `CompletionRefusal` for evidence outcome `Falsified`, `CoverUnsatisfied`, and `Inconclusive` `MissingCoverSummary`, `VacuousProof`, `UnwindBoundExhausted` or `FailedWithoutCounterexample` with no `failureCause` or `properties_failed`, and `EnvironmentFault` for `Cancelled`, `Inconclusive` `TimedOut`, `MemoryExhausted` or `NoVerdict`, and `FailedWithoutCounterexample` with a `failureCause` other than `properties_failed`, and for the internal inconsistency of outcome `Verified`. For `kani_execution` it is `CompletionRefusal` for nested `HarnessNotInCrate`, `HarnessProvenanceMismatch`, `OutputOverLimit` and `PlaybackForNonMember`, and `EnvironmentFault` for every other nested refusal. Real errors produced by the runner for the request-mismatch, oracle-stop and falsified-proof causes are refusals (`dependency_lock` is an `EnvironmentFault` unreachable from a canonical request and is tested only through an in-crate seam); a real single run cancelled by its source and one stopped by its wall-clock timeout are faults, as are the injected snapshot-change, native-launch, malformed-helper-output and missing-embedded-lock-revision causes. | Test (TC-027) |
| FR-017-AC-43 | A canonical-identity encoding failure is `IdentityEncode` with code `identity_encode` and class `EnvironmentFault`; no path returns `NativeResult` for it. `NativeResult` is produced only for the helper's malformed, missing or unsupported structured result, and a forced identity-encode failure through an in-crate test seam returns `IdentityEncode`, not `NativeResult`. | Test (TC-027) |
| FR-017-AC-44 | The code table and the mapping to class are frozen: a test lists every variant with its expected code and its mapping to class (one class for a fixed variant; for `proof_not_verified` the class of each `KaniRunOutcome`, `KaniInconclusiveReason` and `KaniFailureCause` value; for `kani_execution` the class of each nested refusal), fails if a variant, outcome, reason, cause or nested refusal exists that the list lacks or if a code value or a mapped class differs, and fails if two variants share a code. Adding a variant, code, class or mapping entry is a spec change to the "Scalar native error discriminant" section made together with that table. The nested `KaniExecutionRefusal::code()` of `OutputOverLimit` and `OutputUnread` is unchanged. | Test (TC-027) |
| FR-017-AC-45 | The receipt exposes its proof-content identity as a `DigestRecord`, the `qsl_replay::DigestRecord` re-exported at the crate root (no CG-defined twin type; a compile test names it by the CG path and by the `qsl_replay` path as the same type), with domain `sha256-jcs` and 32 bytes equal to `ScalarProofContentIdentity::as_bytes()`. An independent computation in the test, SHA-256 of the RFC 8785 encoding of `{"schema": "agent-ix/cg-kani-scalar-proof-content/v3", "artifacts": [...]}` over the entries of AC-46, gives the same 32 bytes. The domain is not `GeneratedArtifactBytesV1`. | Test |
| FR-017-AC-46 | For a genuine routed `x + 1` package the v3 preimage has exactly ten artifacts, in UTF-8 byte order: `Cargo.lock`, `Cargo.toml`, `claim-map.json`, the only admitted record `kani-obligations/<module>.json`, `observer/Cargo.lock`, `observer/Cargo.toml`, `source-maps/lib.json`, `src/bin/ir791_native.rs`, `src/lib.rs` (harness), and `src/oracle-artifacts-lib.rs` (emitted library). All four original emitted artifacts, harness and record compare byte-identically under the rename map. Removing, renaming, reordering, duplicating or changing one byte of any entry changes the digest. Non-UTF-8 runner-read bytes in either lock, observer manifest or adapter yield `Snapshot` / `snapshot_io` / `EnvironmentFault` and no receipt. The independent fixed-literal golden covers all ten entries and is asserted with the v3 schema in one assertion; it is never computed from real staged lock contents. | Test |
| FR-017-AC-47 | Two receipts for the same package and harness with different operands, different `ScalarLimits` and different native outcomes have equal `DigestRecord`s; operand order, limits, ranges, outcome and Kani evidence appear nowhere in the preimage. This holds for the swapped-operand runs of FR-017-AC-31. | Test (TC-027) |
| FR-017-AC-48 | The receipt exposes read-only the ordered staged `(path, bytes)` entries the digest was built from, including both manifests, both independently generated locks, the source map and the canonical `src/bin/ir791_native.rs`. All ten entries remain measured, sealed and compared across both execution boundaries. Recomputing the digest from only those exposed entries equals the receipt's `DigestRecord`. A recomputation that substitutes one byte in the package, harness or harness record (which a driver holding different bytes would do) differs from the receipt's digest. The exposed entries are exactly the bytes measured for the identity, captured at measurement time and held in the receipt; no entry accessor reads the filesystem. The runner removes the private snapshot explicitly before it returns the receipt; when that removal succeeds the snapshot path no longer exists, and the exposed entries still read and the digest recomputes byte-identically from them (the receipt never reads the filesystem). `Drop` of the snapshot is a best-effort backstop only. A failure to remove the snapshot directory changes neither the receipt nor the verdict: the receipt exposes it read-only through `cleanup_diagnostic()`, which reports the removal failure (and is empty when removal succeeded). The cleanup diagnostic is excluded from the digest and from the staged entries, and never turns a verified run into a failure or alters an exposed entry or the digest. The receipt has no constructor, deserializer or setter for the entries or the digest, and nothing mints the digest from operands, limits or outcome. | Test |
| FR-017-AC-49 | A change to the preimage schema (artifact set, path names, ordering, encoding, canonicalization or hash) is made only by changing the schema string, with the golden digest of AC-46 updated in the same change; the golden is computed over fixed literal entries, so dependency bumps do not move it, and is asserted together with the schema string in one assertion; a golden change without a schema bump is rejected in review (a test cannot enforce it). A change to either manifest, either lock or adapter bytes changes the digest with the schema string unchanged. The domain remains `sha256-jcs` across a schema bump. | Test |
| FR-017-AC-50 | The `NativeMeterObservation` of a receipt exposes read-only the `ScalarLimits` the native meter was constructed with. They are read back from the native invocation's own output, not copied from the request: a test seam that makes the adapter report a well-formed but different limit value makes the helper's structured result unsupported in FR-017-AC-43's sense and yields `NativeResult` / `native_result` / `EnvironmentFault` and no receipt. For an accepted request the request limits, the meter-observed limits and `scalar_limits()` are equal, and the meter-observed limits are unchanged after the snapshot directory is gone. The receipt has no setter or constructor for them (IR-857). | Test (TC-027) |
| FR-017-AC-51 | A consumer compile test names and constructs `quire_contract_codegen::ScalarLimits` using only codegen crate paths, a typed accessor to `KaniScalarNativeRequest::scalar_limits` compiles with that same return type, and assignment to `quire_contract_runtime::exact::ScalarLimits` compiles without conversion. | Test (TC-027) |
| FR-017-AC-52 | Production emitted bytes and feature-free exact dependency remain unchanged. The isolated private observer uses resolver 2 and its sole example compiles the single canonical adapter in place through `../src/bin/ir791_native.rs`, with runtime `exact` and kernel `test-support` exposed only by dev-dependencies; no wrapper, copied source, symlink, normal/build logging edge or CLI feature override is allowed. Native invocation explicitly names `observer/Cargo.toml` and the example; the real production proof retains its harness, ordered options, claims, covers, bounds and backend gates. A control removing the dev feature fails observer compilation; production library/proof target validation succeeds without it. Actual meter output supplies limits, counters and ordered charges. | Test |
| FR-017-AC-53 | Both package locks are independently freshly resolved with the existing bounded offline generation/update workflow, without copying a parent/other lock or substituting a cached resolution. Each resolves both owning runtime and exact identities to the build's authoritative lock identities; the independent observer regression asserts both literal package names and repository identities independently of the renderer's list. Both manifests, locks and all other AC-46 entries are sealed and compared as a complete path/byte set before proof, after proof and after native execution; mutation/replacement or an extra file anywhere in either package yields a typed failure and no receipt. Snapshot cleanup retains all ten measured entries for byte-identical recomputation, including when cleanup reports a diagnostic. | Test |

## Dependencies

- **Upstream**: [FR-015](./FR-015-bounded-kani-obligations.md),
  [FR-022](../../routed/functional/FR-022-routed-generation.md), whose `generate_routed` is the source
  of the exact-scalar harness this requirement also runs,
  [FR-035](../../oracle/functional/FR-035-caller-text-admission.md), which supplies the distinct
  caller Text-admission harness and identity,
  [interface-001](../../core/functional/interface-001-codegen-api.md).
- **Downstream**: [TC-027](../matrix/TC-027-kani-execution-evidence.md),
  [TC-043](../matrix/TC-043-kani-batching-and-output-bounds.md),
  [FR-029](./FR-029-run-outcome-terminal-record.md), which maps the outcome to QSL's terminal value,
  [FR-016](../../replay/functional/FR-016-witness-native-replay.md), which decodes the counterexample
  this requirement retains.
