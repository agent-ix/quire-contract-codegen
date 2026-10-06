---
id: SR-1630
title: "IR-241 ceiling slice code review (Rust lane)"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen#295; src/kani/run/memory.rs, src/kani/run/launch.rs, src/kani/run/execute.rs, src/kani/run/harness.rs, src/kani/run/mod.rs, src/kani/identity.rs, src/kani/classify.rs, src/kani/terminal.rs, src/kani/generate/corpus/bounded_kani_corpus.rs, src/kani/generate/frame.rs, src/kani/generate/negotiate.rs, src/kani/generate/outcome.rs, src/kani/generate/scalar.rs, src/kani/generate/v1_bundle.rs, src/kani/test_support.rs, src/lib.rs, src/routed/generate.rs, schemas/kani-corpus-proof-graph-v1.schema.json, schemas/kani-proof-graph-v2.schema.json, tests/it/bounded_kani_corpus.rs, tests/it/kani_batching.rs, tests/it/kani_generation.rs, tests/it/kani_obligations.rs, tests/it/kani_obligations_state_frame.rs, tests/it/kani_witness_join.rs, tests/it/no_generation_panics.rs, tests/it/oracle_arithmetic.rs, tests/it/routed_generation.rs, tests/it/skeleton_spine.rs, tests/it/terminal_map.rs, tests/common/withdraw_fixture.rs"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-028
    type: reviews
---

# SR-1630: IR-241 ceiling slice code review (Rust lane)

## Summary

Ticket: IR-241. PR: quire-contract-codegen#295. Method: code-review with the rust-review lane
in this one artifact. The exact reviewed candidate is recorded in the private tracker marker,
not here.

The slice makes the memory and wall-clock ceilings required on the three executable identities
(contract, scalar, state-frame), the V1 bundle graph and the corpus identity. Execution reads
those ceilings from the identity, and a shared bounded launcher enforces them. A Linux procfs
observer samples the aggregate resident memory of the backend tree. The change adds a
`MemoryExhausted` reason and maps it to `Incomplete(ResourceExhausted)`.

## Method

The diff was read against its merge base with `main`, with unchanged context in `launch.rs`,
`execute.rs`, `clause.rs` and `frame.rs`. Repo conventions applied: `CLAUDE.md` (no new
hash/pin/tool-version records, single `it` test binary, `// SAFETY:` audit), `clippy.toml` and
`rustfmt.toml`. The rust-review checklist was applied, covering errors, panic surface,
conversions, resource bounds, lifecycle and tests.

Areas audited:

- Process identity: the pid plus start-tick key, pinned `status` fds, and the pidfd claim
  before the start-tick check.
- RSS disappearance during exit: the released-mm zombie path.
- Escape and reparenting: descendants that leave the group or are reparented before the first
  observation.
- Cleanup ordering on every conclusion.
- Deadline-before-completion ordering, including a zero ceiling and `Duration::MAX`.
- Batch `N*T` overflow and grouping by unequal ceilings.
- Refusal before spawn on an unsupported platform.
- Output bounds when a memory stop races a pipe failure.

No gates were run by this reviewer, as dispatch required. These leader-supplied receipts were
read as evidence:

- Default CI: 183 unit, 394 integration and 1 doc test on both toolchains.
- Kani: 25 passed, 0 failed.
- Mutation logs.

The `classification` mutant log shows `1 passed`, a survivor, and is not counted as a kill.
The targeted single and batch classification mutants, the launcher-only, ignored-ceiling,
unbounded-unavailable and pid-reuse mutants each show the intended test failing.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Public `kani_launch_command` + `run_launcher_with_timeout` + `launch_evidence` (re-exported from the crate root) run the backend with no memory ceiling and a caller-chosen wall clock, and still classify a Verified/Falsified result. The `kani_launch_command` doc invites exactly this use. This contradicts FR-028-AC-21 ("the backend process tree of every run is held to the harness identity's memory ceiling") and the module doc claim that "there is no independent execution budget that can weaken that identity". | src/lib.rs:112, src/lib.rs:117, src/kani/run/launch.rs:153, src/kani/run/execute.rs:693, src/kani/run/execute.rs:5-6 |
| FND-002 | medium | Tree observation and cleanup cover only descendants seen at a 20 ms poll. A process that leaves the launcher group and is reparented (its parent exits) before the next poll is never added, because its group is not the launcher's and its parent is not known. It is never counted against the ceiling and never killed. Children forked by an already escaped process after the last poll are in that escapee's group, so neither `kill_known` nor the group kill reaches them. The `MemoryExhausted` doc ("the tree was killed") and the module doc overstate the mechanism. The child-overage test keeps the escapee's parent alive (`wait`), so it cannot fail for this case. | src/kani/run/memory.rs:104-122, src/kani/run/memory.rs:150-174, src/kani/run/memory.rs:1-6, src/kani/run/launch.rs:46-47, src/kani/run/launch.rs:233-236, src/kani/run/execute.rs:1144-1158 |
| FND-003 | low | An exited thread-group leader whose other threads are still running has released its own mm: `status` has no `VmRSS` and `stat` reports zero vsize and rss. `resident_bytes` therefore returns `Some(0)` for a process whose live threads still hold all of its memory. | src/kani/run/memory.rs:231-240 |
| FND-004 | low | The observed "aggregate resident memory" is a sum of per-process `VmRSS`, which counts shared pages (shared libraries, copy-on-write after fork) once per process. The ceiling comparison can kill a tree whose real resident footprint is under the ceiling, and `peakResidentBytes` is documented as tree resident memory. | src/kani/run/memory.rs:129-145, src/kani/run/memory.rs:24-36 |
| FND-005 | low | Every 20 ms poll reads and parses `/proc/<pid>/stat` for every process on the host, then runs an O(n x depth) fixpoint, for the whole of a multi-minute Kani run. On a shared host with 1-2k processes this is 50-100k procfs reads per second. | src/kani/run/memory.rs:104-122, src/kani/run/memory.rs:176-194 |
| FND-006 | low | `HarnessView` now allocates and clones a `Vec<SymbolicArgumentBounds>` (with `String` identifiers) on every `view()` call. Execution calls `view()` many times per run: pairwise in `shares_process` during grouping, in `start`, repeatedly in `run_group`, and in `batch_launch_command`. Only `evidence_of` reads the vector. | src/kani/run/harness.rs:65-137, src/kani/run/execute.rs:451-452, src/kani/run/execute.rs:302-303 |
| FND-007 | low | The fixture `ProofCeilings { 16 GiB, 600 s }` literal is repeated about 60 times across unit and integration tests, so the fact lives in many places. A test-support constructor would hold it once. | tests/it/kani_obligations_state_frame.rs:617-620, src/kani/generate/corpus/bounded_kani_corpus.rs:946-949, tests/it/bounded_kani_corpus.rs:149-152 |

### Failure scenarios

- FND-001: a caller runs `let (_, cmd) = kani_launch_command(&request); let launch =
  run_launcher_with_timeout(cmd, Duration::from_secs(86_400))?; launch_evidence(launch,
  report, kind)` for a harness whose identity says 1 GiB and 60 s. CBMC grows to 20 GiB with
  no observer and settles `Verified` after hours. No refusal, memory evidence or ceiling is
  recorded. Fix: make the timeout-only launcher and `launch_evidence` crate-private, or have
  the public entry take the request and route it through `run_bounded_launcher` with the
  identity ceilings.
- FND-002: the launcher runs `sh -c '(setsid python3 -c "b=bytearray(4<<30); import
  time; time.sleep(600)" &); sleep 60'`. The subshell exits within microseconds, so the python
  process has PPid 1 and its own group before the first poll. With a 1 GiB ceiling the run
  ends `TimedOut` at 60 s rather than `MemoryExhausted`, and the 4 GiB process keeps running
  after the call returns. Kani's own tree does not daemonize (launch.rs:486-487), so this is a
  mechanism limitation, not a current Kani failure. Fix: either use a containment mechanism
  that cannot be escaped (a cgroup v2 leaf with `memory.peak` and `cgroup.kill`, or
  `PR_SET_CHILD_SUBREAPER` with a kill-and-rescan loop until no tracked process remains), or
  state the limitation where the mechanism, `MemoryExhausted` and TC-039 are documented.
- FND-003: a backend whose main thread calls `pthread_exit` while worker threads allocate
  6 GiB. The leader is a zombie whose mm has been released, so each sample counts 0 bytes and
  a 1 GiB ceiling never trips. Fix: when the leader's `status` has no `VmRSS` but `Threads:` is
  above 1, read a live thread's `/proc/<pid>/task/<tid>/status`.
- FND-004: a tree of four processes each mapping 200 MiB of shared libraries and COW pages
  reports about 800 MiB more than its real footprint. With a ceiling near the real peak, a
  run that fits is stopped as `MemoryExhausted`. Fix: sample `Pss` from `smaps_rollup`, or
  document the value as the sum of per-process RSS, an upper bound.
- FND-005: a 20-minute Kani run on a host with 1,500 processes performs about 90 million
  procfs reads. Fix: walk `/proc/<pid>/task/*/children` from the known set each poll, with a
  full scan less often, or use a cgroup.
- FND-006: grouping 50 requests calls `view()` O(n^2) times, each cloning every argument
  identifier. Fix: compute the bounds only in `evidence_of`.
- FND-007: changing the fixture default means editing about 60 sites.

## Verdict

**Changes requested.** FND-001 must be fixed or explicitly scoped out of FR-028-AC-21 before
merge. FND-002 needs either a stronger mechanism or a stated limitation in the docs and in
TC-039. The remaining findings are low.

These were checked and are sound:

- Process identity. The pid plus start-tick key is checked after pinning the `status` fd and
  after claiming a pidfd, so a recycled pid is neither counted nor signalled.
- Released-mm exit. The released-mm zombie path re-reads `stat` and accepts zero only when
  both vsize and rss are zero.
- Kill ordering. The launcher stays unreaped until the group kill, so its pgid cannot be
  reused.
- Conclusion order. Memory is sampled before the deadline check, and the deadline is checked
  before `waitid`. An exited launcher cannot turn an elapsed zero ceiling into completion, and
  `Duration::MAX` or an overflowing `N*T` never elapses, as FR-028-AC-12 requires.
- Batches. A memory stop owns the conclusion over a racing pipe failure. A batch memory stop
  refuses every member with no report read. Unequal ceilings split groups.
- Unsupported platform. The observer refuses before spawn when procfs ancestry, `VmRSS` or a
  pidfd is unavailable.
- Wire and terminal mapping. `NonZeroU64` and the schema `minimum: 1` agree.
  `MemoryExhausted` is distinct and maps to `Incomplete(ResourceExhausted)`.
- Repo conventions. No new hash, pin or tool-version record was introduced. No `unsafe` was
  added.

## New findings (disposition pass 1)

These were reviewed at the PR #295 fix-round head; the exact identity is in the private tracker
marker. They are new defects in the fix, or gaps the fix exposed. None of them reopens an original
finding.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-008 | medium | When a multithreaded process exits normally, there is a window where the leader has released its mm (no `VmRSS`), `Threads:` is still above 1, and every worker has also passed `exit_mm` but has not been released. No worker then reports `VmRSS`, so `resident_bytes` returns "live worker RSS unavailable". The healthy run is stopped and refused as `MemoryObservationFailed`. Cargo, rustc and CBMC are multithreaded and exit many times per Kani run. | src/kani/run/memory.rs:256-304 |
| FND-009 | low | Namespace startup is capped at a fixed 5 s, independent of the identity ceiling. When that cap, and not the identity deadline, elapses, `read_info` reports `TimedOut`. `run_monitored` then settles it as `WaitConclusion::TimedOut`, so the run is classified `inconclusive` timed-out "naming the ceiling", or refused as `BatchTimedOut`, although the wall-clock ceiling was never reached. | src/kani/run/namespace.rs:156-157, src/kani/run/launch.rs:276-278 |
| FND-010 | low | A missing `/proc/<pid>/task/<tid>/children` file is treated as process disappearance and skipped, and `prepare` never checks that the file exists. On a kernel built without `CONFIG_PROC_CHILDREN`, the observer sees only the namespace init. It records a tiny peak and never trips the memory ceiling, while evidence names the observing mechanism. That is unsupported observation accepted rather than refused before spawn. | src/kani/run/memory.rs:213-227, src/kani/run/memory.rs:57-92 |
| FND-011 | low | Before dispatch, bwrap's `--block-fd` treats EOF as release. If the caller process dies (SIGKILL, abort, OOM kill) between spawn and gate write, the kernel closes the gate write end before the outer wrapper is signalled. The gated init can then exec the backend, unowned. In-process error and unwind paths are correct, because `cleanup`/`Drop` kill the pinned group before closing the gate; this window exists only for caller-process death. | src/kani/run/namespace.rs:90-141, src/kani/run/namespace.rs:209-213 |
| FND-012 | low | Every bounded Kani execution now requires bubblewrap and permission to create unprivileged user and PID namespaces. Without them, every run is refused `MemoryMechanismUnavailable`. The public entry docs (`execute_kani_obligation`, `execute_kani_obligations`) and the repo setup docs (`CLAUDE.md` commands, `make tools`) do not state this prerequisite. | src/kani/run/execute.rs:268-269, src/kani/run/execute.rs:419, src/kani/run/namespace.rs:1-5 |
| FND-013 | low | The `memory.rs` module doc still describes the removed design ("Descendants are retained by pid and start time after leaving the launcher's process group; a reused pid is never ... signalled"). The observer now walks the namespace init's task children and signals nothing. | src/kani/run/memory.rs:1-6 |

### Failure scenarios (disposition pass 1)

- FND-008: a Kani run whose cargo build spawns rustc with worker threads. At a 20 ms sample
  rustc is in `exit_group`: the leader has passed `exit_mm`, and the workers are between
  `exit_mm` and `release_task` (closing files, task work). `Threads:` reads 4 and no task
  reports `VmRSS`, so the run is refused although nothing was over the ceiling. Fix: when no
  worker reports `VmRSS`, read each worker's task `stat`. If every task shows vsize 0 and rss 0,
  the thread group has released its address space, so return `Some(0)`. Refuse only when a task
  has a live address space but no readable RSS. Add a fixture for the all-released case.
- FND-009: on a loaded host bwrap takes more than 5 s to write its info for a harness whose
  identity ceiling is 600 s. Evidence reads `inconclusive` `timed_out`, attributed to the 600 s
  ceiling. Fix: map the startup-cap expiry to a typed startup refusal (unavailable or
  observation failure), and keep `TimedOut` for when the identity deadline itself has passed.
- FND-010: a custom kernel without `CONFIG_PROC_CHILDREN` and a backend allocating 8 GiB under a
  1 GiB ceiling. The run completes `Verified`, with a peak equal to bwrap init's RSS. Fix: in
  `prepare`, require `/proc/self/task/<tid>/children` to be readable. In `processes`, treat
  `NotFound` on a child file as disappearance only after re-confirming the task directory is
  gone.
- FND-011: an OOM killer or `kill -9` hits the caller between `spawn` and `dispatch`. bwrap's
  init reads EOF, forks the Kani backend, and runs it unbounded after the caller is gone. Fix
  options: write the gate byte only after the claim, as now, and also make the backend command
  check a positive token; or keep a guardian that SIGKILLs the startup group when the caller
  dies. At minimum, state the window in the namespace module doc.
- FND-012: a consumer upgrades and every `execute_kani_obligation` call returns
  `MemoryMechanismUnavailable` on Ubuntu with `kernel.apparmor_restrict_unprivileged_userns=1`
  and no profiled bwrap. No doc says why. Fix: state the prerequisite on both public entries and
  in the repo's tool and setup documentation.
- FND-013: a reader of `memory.rs` concludes descendants are tracked by process group and
  signalled by the observer. Fix: rewrite the module doc for owned-init traversal.

## Dispositions

Round 1. The commit identity of each outcome is in the private tracker marker for this round.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | PR #295 fix round: `kani_launch_command`, `launch_evidence`, `run_launcher_with_timeout` and `LaunchOutcome` are no longer crate-root exports; the timeout-only launcher and launch-command builder are `#[cfg(test)]`; the only production spawn path is `run_bounded_launcher` through `NamespaceOwner`. |
| FND-002 | fixed | PR #295 fix round: a mandatory per-run bubblewrap PID namespace with a pidfd-claimed, identity-verified init; cleanup kills init and confirms teardown on every conclusion. The fixtures cover a double-forked, setsid, reparented allocator and a fork after the last sample, and the cleanup-omitted mutant is killed. Residual edges are FND-009 to FND-011. |
| FND-003 | fixed | PR #295 fix round: a released leader mm uses one live worker's RSS, never a sum over threads; missing worker RSS or task ancestry is refused. FND-008 is a new consequence. |
| FND-004 | fixed | PR #295 fix round: the mechanism, evidence field and TC-039 now state a conservative sum of per-process RSS, with shared pages counted per process and no physical-footprint claim. |
| FND-005 | fixed | PR #295 fix round: steady-state sampling walks only the owned init's task `children`; a host scan runs only on pre-claim startup abort. |
| FND-006 | fixed | PR #295 fix round: `HarnessView` no longer carries bounds; `symbolic_arguments()` runs only in `evidence_of`. |
| FND-007 | fixed | PR #295 fix round: `tests/common/proof_ceilings.rs` is the single fixture source, reused by `src/kani/test_support.rs`; no other copy of the literal remains. |

Round-1 verdict: the original findings are all fixed. New FND-008 (medium) and FND-009 to
FND-013 (low) are open and go back to the coder. This was a static disposition: no gates were
run by the reviewer. The final rebased full CI, `cargo deny` over the new `command-fds` and `nix`
dependencies, and the second Kani run (the first run of real Kani inside the namespace) are
still required.

Round 2. The commit identity of each outcome is in the private tracker marker for this round.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-008 | fixed | PR #295 fix round 2: when no task reports `VmRSS`, each task's `stat` is read. Every task showing a released address space (vsize 0, rss 0), together with a fresh, identity-matched leader `stat` and a released count at least the fresh `Threads:`, yields `Some(0)`. A task with a live mm but no readable RSS is still refused. The live-worker RSS and pid/start checks are kept, and the released-mm mutant is killed. |
| FND-009 | fixed | PR #295 fix round 2: startup-cap expiry is `TimedOut` only when the identity deadline has itself elapsed. Otherwise it is a typed startup refusal (`MemoryMechanismUnavailable`), backed by `startup_cap_refuses_without_claiming_the_identity_wall_ceiling_elapsed`. |
| FND-010 | fixed | PR #295 fix round 2: `prepare` requires readable `children` files for the caller's tasks before dispatch. During traversal, a missing `children` file is skipped only once the task's own directory is confirmed gone; a live task without one is refused. The children mutant is killed. |
| FND-011 | deferred | Too large for this PR: a safe caller-death supervisor needs a first-party guardian (about 500-700 lines of guardian and control code plus caller, test and packaging work). Deferred to IR-639, a child of IR-241 that blocks it (relations verified in Linear). `CLAUDE.md`, both public entry docs, the namespace module doc, FR-028-AC-21's marker and TC-039 Status all state the startup caller-death non-guarantee, and in-process ownership and teardown claims are unchanged. IR-241 cannot complete until IR-639 delivers. |
| FND-012 | fixed | PR #295 fix round 2: the Linux, procfs `children` and RSS, pidfd, bubblewrap and namespace-permission prerequisites (including AppArmor) are now stated on `execute_kani_obligation`, on `execute_kani_obligations` and in `CLAUDE.md`, which also says `make tools` installs neither. No CI workflow change. |
| FND-013 | fixed | PR #295 fix round 2: the `memory.rs` module doc now describes owned-init task-child traversal with start identities, says the observer signals no process and `NamespaceOwner` owns teardown, and states the conservative RSS-sum semantics. The source has no signal call. |

Round-2 verdict: FND-008, -009, -010, -012 and -013 are fixed, and FND-011 is deferred to IR-639,
transparently. None of FND-001 to FND-007 regressed on the paths round 2 changed. No new
findings. With every SR-1630 to SR-1633 finding now at a non-open outcome, the PR is
review-mergeable. Merge still waits on the final rebase (with a same-session regression
disposition if owned semantics change), full CI and the second Linux Kani run, none of which has
run yet. IR-241 itself stays incomplete: IR-639 blocks it.

## New findings (disposition pass 3)

This is a rebase-regression check of the PR #295 head after rebasing onto `main`, which now
includes the IR-624 code change (#297) and the IR-635 spec change (#298). The exact head and the
byte-comparison evidence are in the private tracker marker.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-014 | high | Semantic merge conflict. `tests/it/kani_obligations_state_clause_replay.rs`, added on `main` by #297 and so outside this PR's diff, builds a `StateFrameRequest` literal with no `ceilings` field. This PR makes that field required: the struct has no `Default`, no `..` base is used and it is not `#[non_exhaustive]`. The module is compiled unconditionally into the `it` test binary (`tests/it/main.rs:59`), and `#[ignore]` does not stop compilation, so the rebased head's integration-test target does not compile (E0063). `cargo test`, `cargo clippy --all-targets` and `make ci` cannot pass. Git merged it cleanly, and the author's rebase evidence does not mention the file. | tests/it/kani_obligations_state_clause_replay.rs:758-765, tests/it/main.rs:59, src/kani/generate/frame.rs:78 |

Failure scenario: the queued focused run, or the final full CI, builds the `it` target on the
rebased head and stops with `error[E0063]: missing field 'ceilings' in initializer of
'StateFrameRequest'`, so no gate result is produced. Fix: add
`ceilings: crate::common::proof_ceilings::proof_ceilings_with_wall_clock(<that file's Kani budget>)`
(or `proof_ceilings()`) to that literal, the same mechanical propagation used for the other 19
state-frame literals. Then re-run the focused job.

Round-3 regression verdict:

- Every other PR file carries exactly the patch reviewed in round 2. Comparing `+`/`-` lines with
  hunk positions stripped, all of `src/kani/run/*` (launcher, namespace, memory, execute,
  harness) is byte-identical to the round-2 head.
- The only patch that changed is `tests/it/kani_obligations_state_frame.rs`, which now also gives
  `ceilings:` to the state-frame literals #297 introduced. It is purely additive and mechanical,
  uses the single fixture source, and drops no `main` line except the PR's intended `timeout:`
  removal. It keeps the merged QSL-emitted accessor, positive-range and model-field fixtures, and
  it does not bring back the removed `generate_over` route.
- The matrix keeps `main`'s FR-025-AC-9 planned row beside the PR's partial FR-028 rows.
- FND-001 to FND-010, FND-012 and FND-013 show no regression.
- FND-011 stays deferred to IR-639, which is In Progress, a child of IR-241 and blocks it. The
  caller-death disclosure is unchanged in `CLAUDE.md`, `execute.rs`, `namespace.rs`, FR-028 and
  TC-039.
- The focused run, full CI and the second Kani run are all pending; no runtime receipt exists for
  this head. Not mergeable until FND-014 is fixed and those gates pass.

## Dispositions (round 4)

Round 4. The commit identity of each outcome is in the private tracker marker for this round.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-014 | fixed | PR #295 rebase correction: the #297 `StateFrameRequest` literal in `tests/it/kani_obligations_state_clause_replay.rs` now takes `ceilings` from the single fixture source (`proof_ceilings()`). A tree-wide audit finds all 36 request literals with `ceilings` and no `KaniExecutionRequest` still passing `timeout`. The focused receipt shows the `it` binary, which includes this module, compiling and running. |

Round-4 verdict:

- FND-014 is fixed. The correction touches only tests: no production source, spec, schema,
  manifest or lockfile changed.
- The ceiling-identity test `both_state_obligation_identities_change_with_either_required_ceiling`
  now builds its request from the real QSL-emitted `BalanceNeverDrops` package, exactly as the
  existing `generated_from_twin` helper does, rather than the synthetic fixture that #297's
  fail-closed model-field rule no longer admits. Its memory and wall-clock identity and record
  assertions are unchanged, and #297's own fixtures and semantics are untouched.
- FND-001 to FND-010, FND-012 and FND-013 show no regression. FND-011 stays deferred to IR-639,
  which is a child of IR-241 and blocks it; the guardian is not implemented, so IR-241 remains
  incomplete.
- Actual runtime evidence for this head: the focused job's 61 runner unit tests, 49 state-frame
  integration tests (5 Kani tests ignored) and 9 ceiling integration tests passed. The narrow
  clippy step finished with exit 0. Full rebased CI and the second Linux Kani run are still
  pending and required before merge.

## New findings (disposition pass 5)

This round reviewed the fixes for the final rebased `make ci` failures: launcher preflight within
the run deadline, the scoped `test_support` module path, and the split `cfg(test)` /
`cfg(target_os = "linux")` attributes. The exact head is in the private tracker marker.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-015 | low | The new pre-dispatch launcher check (`fs::metadata`) restores the typed `Tool`/`Launcher` fault only for a launcher that does not exist. A launcher path that exists but cannot be executed, such as a file without the exec bit (which `KaniInstallation::discover` admits, since it checks only `is_file()`) or a directory set through the public `launcher` field, passes the check. Inside bwrap, `execvp` then fails, the wrapper exits 1 with no report, and the run is classified as a backend outcome instead of the typed launcher fault the pre-PR direct spawn returned. A launcher removed between the check and exec takes the same path. | src/kani/run/execute.rs:314-325, src/kani/run/tool.rs:55-58, src/kani/run/tool.rs:96 |

Failure scenario: `KaniInstallation { launcher: "/opt/kani/cargo-kani" }` with mode 0644 gives
an evidence record with an inconclusive (no-verdict) outcome, where it should have been
`KaniExecutionRefusal::Tool(Io { PermissionDenied })`. Fix: in the same preflight, also require
a regular file with an execute bit, and return the typed `Tool` fault for a directory or a
non-executable file. Alternatively, have the namespace dispatch distinguish bwrap's own exec
failure from a backend that ran.

Round-5 verdict:

- The production changes are correct. `start()` fixes the absolute deadline once, before any
  launcher inspection. An already expired deadline skips the inspection entirely, so a zero
  ceiling still times out before any I/O or spawn. The same deadline goes to
  `run_bounded_launcher_at`, which no longer resets it, so the identity wall-clock ceiling covers
  the preflight. `Duration::MAX` (no deadline) still inspects and never elapses. Batches inspect
  their one shared launcher.
- An absent launcher is again the typed `Tool`/`Launcher` fault before any helper or backend
  starts. `tc_027_a_missing_launcher_is_refused_before_anything_runs` is unchanged and still
  asserts the typed fault and that nothing ran.
- `test_support` drops its re-export and exposes the `proof_ceilings` module itself. Every caller
  now uses the defining-module path, and the fixture still has one source.
- The namespace test `cfg` is split into `#[cfg(test)]` and `#[cfg(target_os = "linux")]`, which
  is semantically identical. No test, scanner or exception was weakened: `tests/` and `scripts/`
  are unchanged.
- FND-001 to FND-010 and FND-012 to FND-014 show no regression. FND-011 stays deferred to IR-639;
  IR-241 is incomplete until the guardian is delivered.
- Actual runtime evidence for this head: the focused receipt shows exit 0 for the three tests
  that previously failed, the 61 runner tests and narrow clippy (`--lib --test it -D warnings`).
  Full rebased CI failed at the previous head and has not been re-run here. The second Linux Kani
  run has not been released. Not mergeable until both pass.

## Dispositions (round 6)

Round 6. The commit identity of each outcome is in the private tracker marker for this round.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-015 | fixed | PR #295 fix round 6: the private `KaniInstallation::require_executable` (in the owning `tool.rs`) refuses a launcher that is not a regular file or has no Unix execute bit, with the typed `Tool`/`Launcher` `Io` fault naming the path (`PermissionDenied`). It runs under the same absolute deadline and is skipped once that deadline has passed. `tc_027_non_executable_launchers_are_refused_before_anything_runs` covers a mode-0644 file and a directory through the public API, asserting the exact fault, the path and that nothing ran. The doc comment says the check is a snapshot that does not pin the file or close replacement or permission races. "Any execute bit" is not the caller's own access (ACLs, `noexec` mounts, interpreters), and that residual is stated rather than claimed closed. |

## New findings (disposition pass 6)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-016 | low | `tc_027_non_executable_launchers_are_refused_before_anything_runs` traces `FR-017-AC-2`, whose text is "An absent launcher is refused with a typed reason naming its path before anything runs". The test asserts a present but non-executable file and a directory, not an absent launcher, so it binds behaviour the cited criterion does not state. No criterion owns the non-executable-launcher refusal. | tests/it/kani_obligations.rs:1793-1796, spec/kani/functional/FR-017-kani-execution-evidence.md:250 |

Failure scenario: the computed matrix counts this test as evidence for FR-017-AC-2, while the
refusal it actually checks has no owning criterion. A later change that weakens the
non-executable refusal fails only a test whose cited criterion it does not violate. Fix: either
broaden FR-017-AC-2's statement to "an absent or non-executable launcher" (a spec edit, with its
own review), or trace the test to TC-027 alone and record the refusal as owned by an existing or
new criterion. This is not a correctness defect in the code.

Round-6 verdict: FND-015 is fixed. New FND-016 (low, trace) is open. No other changed path
regresses: the only production change is the private preflight helper and its call site.
FND-001 to FND-010 and FND-012 to FND-014 stay fixed, and FND-011 stays deferred to IR-639,
which is not implemented, so IR-241 is incomplete. Actual runtime evidence for this head: the
focused job exited 0, covering the new and original launcher tests, the layout and panic-scan
tests, 61 runner tests and narrow clippy. Full rebased CI must still be re-run after its earlier
failure, and the second Linux Kani run has not been released.

## Dispositions (round 7)

Round 7. The commit identity of each outcome is in the private tracker marker for this round.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-016 | fixed | PR #295 fix round 7: `tc_027_non_executable_launchers_are_refused_before_anything_runs` now traces `TC-027` only. `quire trace --id FR-017-AC-2` binds just the original absent-launcher test and the discovery test, so the wrong binding is gone. Assertions, logic, production code and spec are unchanged. The non-executable refusal is still not stated by any FR-017 criterion; it is accepted as a TC-027-level defensive check of the existing launcher contract, not a new obligation for this PR. |

Round-7 verdict: clean. FND-016 is fixed, there are no new findings, and nothing regressed: the
only non-artifact change is one trace comment. The latest SR-1630 to SR-1633 outcomes are 20
fixed, 1 deferred (FND-011, startup caller-death supervision, deferred to IR-639; the guardian is
not implemented, so IR-241 remains incomplete) and 0 open. Runtime evidence is unchanged from the
product tested at the round-6 head; only a comment changed since. Full rebased CI must still be
re-run after its earlier failure, and the second Linux Kani run has not been released. Neither is
claimed here.

Round 8 (narrow regression check; the exact head is in the private tracker marker): clean, with
no new findings and no outcome changes. Full rebased CI failed only at rustdoc, on two broken
intra-doc links, so the default test lane never ran. The fix touches only those two doc comments:

- `execute.rs`: `batch_launch_command` now links `super::namespace::BackendCommand`, the type it
  actually returns.
- `launch.rs`: the group-leader sentence now links `run_monitored`, which sets
  `.process_group(0)`, replacing the removed `run_launcher_with_timeout`.

Both new links resolve to the defining items and describe the code truthfully. No code, test,
assertion, spec, import or lint `allow` changed. The latest SR-1630 to SR-1633 outcomes stay 20
fixed, 1 deferred (FND-011, deferred to IR-639; the guardian is not implemented, so IR-241 is
incomplete) and 0 open. Actual evidence for this head: `RUSTDOCFLAGS=-Dwarnings cargo doc
--locked --no-deps --document-private-items` exited 0. Full CI must be re-run, including the
default test lane it never reached, and the second Linux Kani run has not been released.

Round 9 (rebase onto `main` with the IR-639 guardian spec (#299) and the QSL update (#300), plus
the procfs parse fix; the exact head is in the private tracker marker): clean, with no new
findings and no outcome changes.

- **Rebase.** With hunk positions stripped, every non-review file's PR patch, `Cargo.lock`
  included, is identical to the round-8 product. The lockfile only adds `cfg_aliases`,
  `command-fds` and `nix` (once each) and removes nothing from `main`, so the QSL and other
  first-party git sources stay at `main`'s revisions. All 36 request literals carry `ceilings`.
- **Fix.** `parse_process` now reads only the unused process-group field as `i32`. Linux
  `do_task_stat` starts `pgid` and `sid` at -1 and `ppid` at 0, and overwrites them only when
  `lock_task_sighand` succeeds, so a task in release (state `X`) prints -1. The old `u32` parse
  refused that valid line as "invalid procfs process ancestry". PID, parent, start and size
  parsing are unchanged, and non-numeric values still fail closed. A dying child with parent 0
  is skipped by the existing parent check; its own children have already been reparented to the
  owned namespace init and are still reached from it.
- **Test.** `released_sighand_signed_group_preserves_owned_rss_and_identity_checks` takes an
  owned root from live 64 KiB to that released state, keeps the observed peak, and still refuses
  a malformed parent and a changed start. The unsigned-group mutant fails it at the
  released-state observation with exactly the message the failed Kani run showed.
- **Root cause.** The real-Kani failure's raw procfs sample was not captured, so this round does
  not claim it is proven; the next Kani run must confirm it.
- **Status.** The latest SR-1630 to SR-1633 outcomes stay 20 fixed, 1 deferred (FND-011, deferred
  to IR-639; the guardian spec has merged but the guardian code does not exist, so IR-241 is
  incomplete) and 0 open.
- **Evidence.** The focused job at this head exited 0: the three earlier regression tests, 62
  runner tests, 49 state-frame and 9 ceiling integration tests, `make deny` and narrow clippy.
  The second Linux Kani run at the previous head failed (24 passed, 1 failed). A new full CI run
  and a new planner-released Kani run are still required, and neither has run.
