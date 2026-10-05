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
