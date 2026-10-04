---
id: "SR-1388"
title: "CG PR 259 code review (with rust-review lane): Kani harness batching, output cap and launcher cleanup"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen@0516084a1d8f15b28dc7bf140cb4307872fa095c; src/kani/{classify,test_support}.rs, src/kani/output/{playback,report}.rs, src/kani/run/{execute,harness,launch}.rs, src/lib.rs, tests/it/{kani_batching,main}.rs, Makefile (diff origin/main...HEAD, merge base fc41528)"
---

# SR-1388: CG PR 259 code review

## Summary

Ticket: IR-277. PR: agent-ix/quire-contract-codegen#259 at 0516084 (two commits on fc41528, the
merged spec PR #258). The `rust-review` lane is folded into this file.

What I ran myself, in a fresh detached worktree with its own target directory:

- `make ci` on the head: exit 0 (fmt-check, spec, lint, msrv, deny, audit-unsafe, rustdoc, test;
  158 lib tests and 293 `it` tests pass, 17 ignored).
- The real-Kani lane filter (`cargo test --test it -- --ignored --test-threads=1 kani_batching`,
  Kani 0.68.0, CBMC 6.11.0): 2 passed in 38.9 s. `process-counts.json`: N=1 1/1 process
  (382/488 ms), N=10 10/1 (3718/2240 ms), N=50 50/1 (19082/9589 ms). The PR body's numbers
  (361/342, 3018/1728, 16507/8098 ms) are the same shape; the process counts match exactly.
- 40 mutants in a throwaway copy with its own target dir: 34 killed, 6 survived. The survivors are
  test gaps and are recorded in SR-1389 (gap analysis), not here.
- A real Kani probe outside the repo (scratch crate): a harness whose two assertions fail on
  independent paths, batched with a second falsified harness (FND-001).

Rust-review checklist: no new panic token, index, slice or unchecked subtraction in non-test code
(the old `chunk[..read]` and `kept.drain(..len - limit)` are gone; `chunk.iter().take(read)` is
used). The NFR-005 scan bodies (`generate_kani`, `route_records`, `rewrite_duplicate_position`,
`observe_clause`, `generate_boolean_oracle_inner`) are not touched. All new `unwrap`/`expect`
are in `#[cfg(test)]` code. `test_support` is `#[cfg(test)]`. Integer conversions are checked
(`u32::try_from` for `--harness-timeout` and the member count, `checked_mul`, saturating
`whole_seconds` and limit). The capture failure path no longer swallows a join panic or a
poll/read error.

## Verdict

Not mergeable as is: FND-001 is a real-Kani behaviour the batch refuses on, reproduced.

Judged and accepted:

- `write_launcher` retrying `ExecutableFileBusy` is test-only and sound. The race is the known
  fork-while-a-write-fd-is-open ETXTBSY: once a `--probe` exec succeeds, no process can still hold
  the write descriptor, so the retry cannot hide a later race. No production path writes a
  launcher.
- The `FlagOnFailure` drop guard is correct. The whole guard moves into the thread (the comment on
  edition-2021 disjoint capture is right), and a panic unwinds through it and sets the flag.
  Mutant M28 (guard not armed) is killed.
- The public API change (`launch_evidence` now returns `KaniExecutionRefusal`, new refusal
  variants, `KaniExecutionEvidence.batch`, new `LaunchOutcome` variants) has no consumer outside
  this repo under `~/dev`, interface-001 is updated, and the single-run wire form is unchanged
  (`batch` is skipped when absent). `KaniExecutionRefusal` is not `#[non_exhaustive]`, so the new
  variants break an exhaustive external match; prerelease, no consumer, noted only.
- No scratch heredoc or probe file leaked into the diff: the 18 changed paths are the
  `src/`, `spec/`, `tests/` and `Makefile` files listed in scope.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Real Kani 0.68 prints one counterexample playback block per failed property check. A member whose harness fails two property checks therefore has two counterexample blocks under its own path, and `run_group` refuses the whole process as `PlaybackDuplicated`: every other member, verified ones included, loses its evidence. The same harness run alone is `Falsified` with the first block. Reproduced: harness `two::check` with `if kani::any() { assert!(x < 5) } else { assert!(x != 7) }` printed two blocks headed `two::check` (`Check for assertion: "first"` and `"second"`). This contradicts FR-017-AC-22 (a falsified member never changes another member's outcome) and single-run parity; the spec rule the code follows rests on the same false premise (SR-1390 FND-001) | src/kani/run/execute.rs:569-579 |
| FND-002 | low | `kill_process_tree` now runs after `try_wait` has reaped a launcher that exited on its own. If no other group member survives, the pgid is free and `killpg(pid)` can in principle hit a recycled process group. The window is tiny on Linux; killing the group before reaping (`waitid` with `WNOWAIT`) closes it | src/kani/run/launch.rs:174-178 |
| FND-003 | low | The real mixed batch runs at T = 2 s, so its outer bound is 6 s for the whole `cargo kani` process, build included. It passed twice (the coder's run and mine, the second under load from a parallel mutation run), but a slow build on a loaded host would make it a `BatchTimedOut` refusal rather than the asserted outcomes. A larger T with a slower `SLOW` harness, or a separate warm build, makes the lane deterministic | tests/it/kani_batching.rs:262-268 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | low | Wording only. After FR-017 defined "batch" (the call) and "group" (one process), two code texts still say "batch" where they mean the group: the `PlaybackForNonMember` message "which is not in the batch" and the `BatchTimedOut` doc "A batch process was still running". No behaviour is affected | src/kani/run/execute.rs:106, src/kani/run/execute.rs:169 |

## Dispositions

Round 1, reviewed at f5ca91c755ed76b936b4f6495cceb4e031cc1566 (one fix commit on 0516084; base still fc41528). Re-measured in a fresh worktree with its own target dir. `make ci` exit 0 (161 lib and 293 `it` tests pass, 17 ignored; `unsafe audit passed`, and the diff adds no `unsafe`). The real-Kani `kani_batching` filter on Kani 0.68.0 passed 3 of 3 runs (56.8 s, 51.2 s, 51.3 s). In each run the real `two::check` member, which fails two assertions, was `Falsified` with the block for `"first"` and not `"second"`. 45 mutants in a throwaway copy with its own target dir: the 43 that apply are all killed. They include take-the-last-block (M41), the duplicate refusal restored (M42), NOWAIT dropped (M43), no kill after a normal exit (M44) and every mutant that survived round 0.

The pgid change is sound. `waitid(P_PID, WEXITED|WNOHANG|WNOWAIT)` leaves the exited leader a zombie, so its pid, which is the group id, stays allocated while `killpg` and `child.kill()` run. Then `child.wait()` reaps it, and the reaped status is the real exit, because SIGKILL to a zombie changes nothing. Hang and leak: `child.wait()` now runs on every path. After an exit it returns at once. On a timeout or capture failure, `child.kill()` SIGKILLs the leader directly even if `killpg` failed, so the wait returns. A `waitid` error other than EINTR still kills and reaps before it is returned. Portability: rustix 1.1.5 compiles `waitid` and `WNOWAIT` on Linux and Apple; it excludes only cygwin, horizon, openbsd, redox and wasi. I could not compile for `aarch64-apple-darwin` here, because a C dependency (psm) needs a macOS cross toolchain.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | f5ca91c |
| FND-002 | fixed | f5ca91c |
| FND-003 | fixed | f5ca91c |

Round 2, reviewed at 8a2632347343526a1421b98a34fa9639d69bd57a (one wording commit on f5ca91c; base still fc41528). The `BatchTimedOut` doc and message and the `PlaybackForNonMember` message now say "group". `make ci` exit 0 (161 lib and 293 `it` tests pass, 17 ignored; `unsafe audit passed`). The commit changes no behaviour, so the round-1 Kani and mutation evidence stands. Sweep of the touched files: no user-facing text says "batch" where it means one process. Two doc comments still say "batch" in the loose sense: the `run_launcher_with_timeout` rustdoc ("a batch multiplies the limit") and a test doc ("refuses the whole batch"). Neither is wrong enough to record. No new finding.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-004 | fixed | 8a26323 |
