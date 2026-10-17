---
id: "SR-11990"
title: "IR-821 code review (incl. rust-review lane): shared locked TemporaryDirectory under CARGO_TARGET_TMPDIR"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@070a708a695077215c4cd2c1e51e1599ed7286cb; tests/it/scratch_crate.rs, tests/it/bound_populations.rs, tests/it/bound_census.rs, tests/it/harness_generation.rs, tests/it/strategy_generation.rs, tests/it/oracle_generation.rs, tests/it/oracle_arithmetic.rs, tests/it/bounded_kani_corpus.rs, tests/it/bound_strategy_generation.rs, tests/it/composite_equality_generation.rs, tests/it/exact_scalar_generation.rs, tests/it/kani_generation.rs"
relationships: []
---

# SR-11990: IR-821 code review

## Summary

Ticket: IR-821 (test hygiene). PR: agent-ix/quire-contract-codegen#407 (draft), head 070a708,
diffed with `git diff origin/main...origin/peter/ir-821-bound-populations-scratch`. The
rust-review lane is folded into this file.

The PR replaces 11 file-local `TemporaryDirectory` copies (each a `temp_dir()/<prefix>-<pid>-<nonce>`
path) with one shared type in tests/it/scratch_crate.rs. The new type uses the stable path
`CARGO_TARGET_TMPDIR/<prefix>`, holds an exclusive `File::lock` on a sibling `<prefix>.lock` file,
clears any leftover directory on creation and removes the directory on `Drop`. Apart from that, the
diff is mechanical: `.0` becomes `.path`, and the imports change.

## Method

I read the whole diff and every call site of `TemporaryDirectory::new`, and checked each scratch
manifest for a `[workspace]` table. I also checked the ancestors of the target directory for
`Cargo.toml`, `.cargo/config*` and `rust-toolchain*`, and the environment of the nested cargo
processes during the run (`/proc/<pid>/environ`).

Runs, with `CARGO_TARGET_DIR` set to the review worktree's target:

- Focused tests: `cargo test --locked --test it --` with the 11 module filters plus `scratch_crate::`.
  201 passed, 0 failed, 14 ignored, in 24 s. Afterwards target/tmp held only the zero-byte `.lock`
  files; every scratch directory had been removed.
- A standalone flock probe (scratchpad ir821/probe/flock_probe.rs). It copies the open-and-lock steps
  of `in_directory` exactly. Results:
  - A second thread in the same process blocks while the first holder lives, and proceeds once the
    holder is dropped.
  - A second open of the same lock file in the same thread gets `try_lock` -> `WouldBlock`.
  - A blocking `lock()` does not return while the first value is alive.
- The bounded_kani_corpus ignored lane, once, inside the 16G cgroup. Its result is in the Verdict.

Findings on the concurrency question (1). `File::lock` is `flock(LOCK_EX)` on Linux, and flock
locks belong to the open file description, not to the process. Every `TemporaryDirectory::new`
call opens the file again, so two threads that use the same prefix really do exclude each other.
The in-repo test `scratch_directory_is_locked_while_live` shows the same thing, because its probe
runs in the same process. std opens files with O_CLOEXEC, so a spawned `cargo`/`cargo kani` child
does not inherit the lock and cannot keep it alive after `Drop`. Field drop order is
`Drop::drop` (remove the directory) and then the lock field, so a waiter never sees a
half-removed live directory.

Question (2), prefixes: all 21 literal and formatted prefixes are distinct across modules.
`{name}` takes the values dependency-free and state-only, `{label}` the values overflow, grid,
compare, stop and agree, and `{crate_name}` the values generated-oracle-tc001 and
generated-oracle-differential. The exception is `write_generated_crate` in kani_generation.rs,
whose fixed prefix "quire-generated-kani" is shared by six tests. They serialize across threads,
and one of them deadlocks against itself (FND-001).

Question (3), environment: the nested cargo inherits `RUSTUP_TOOLCHAIN=1.98.1` from the outer
rustup proxy (`RUSTUP_TOOLCHAIN_SOURCE=toolchain-file`). The repository's rust-toolchain.toml
therefore changes nothing. There is no `.cargo/config` in the repository or above it. The root
Cargo.toml has no `[workspace]` table, and no ancestor directory has a Cargo.toml. So today the
nested crates resolve exactly as they did in /tmp. That depends on the repository staying this
way (FND-002).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | `kani_executes_the_generated_contract_proof` deadlocks against itself. It calls `write_generated_crate` twice, at :1614 and :1673, and the first `directory` is still alive when the second call runs. Both calls use the fixed prefix "quire-generated-kani" (:304). The second `in_directory` opens a new file description and calls the blocking `File::lock` (scratch_crate.rs:69), which never returns while the same thread holds the first lock. `make kani` and `make kani-gate` run `--ignored --test-threads=1 ... kani_generation`, and libtest has no timeout. So the real-Kani lane hangs forever after the first proof of this test. | tests/it/kani_generation.rs:1614, tests/it/kani_generation.rs:1673, tests/it/kani_generation.rs:304, tests/it/scratch_crate.rs:63-69, Makefile:86 |
| FND-002 | medium | Eight scratch manifests have no `[workspace]` table, and they now sit inside the repository tree at target/tmp, where they used to sit in /tmp. They work only while no ancestor Cargo.toml declares `[workspace]`. If the repository adds one (for example for `[workspace.lints]` or a second crate), or someone points `CARGO_TARGET_DIR` into another workspace's tree, each nested `cargo test`/`cargo check` stops with "current package believes it's in a workspace when it's not". The failure looks like a generator regression. A future repository-level `.cargo/config.toml` would also leak into these builds. The other manifests in the PR already end with `[workspace]`. | tests/it/bound_populations.rs:922, tests/it/bound_census.rs:371, tests/it/harness_generation.rs:200, tests/it/harness_generation.rs:655, tests/it/strategy_generation.rs:95, tests/it/strategy_generation.rs:363, tests/it/strategy_generation.rs:592, tests/it/oracle_generation.rs:371 |
| FND-003 | low | The lock wait has no bound and prints no diagnostic, and a nested acquisition is not detected. Because one stable path per prefix replaced unique paths, any hold-and-reacquire becomes a silent hang instead of an error: FND-001 today, and any later helper reused inside a test that is still holding one. A hung run elsewhere that holds the same `<prefix>.lock` in a shared target directory has the same effect. Making `try_lock` panic on `WouldBlock` when the holder is this process (a process-local set of live prefixes), or waiting a bounded time and then naming the lock file, would turn the hang into a failure that names the cause. | tests/it/scratch_crate.rs:69 |
| FND-004 | low | The new tests do not pin the self-cleaning half of the contract, or the waiting behaviour. `scratch_directory_is_locked_while_live` checks only `try_lock` and never asserts that `Drop` removed the directory. No test shows that a same-prefix waiter proceeds after the drop, or that the same thread reacquiring is rejected. Deleting the `Drop` impl, or replacing `lock()` with an unchecked `try_lock()`, still passes all three new tests. | tests/it/scratch_crate.rs:97-131 |

### FND-001 detail

Reproduction without Kani: the scratchpad probe ir821/probe/flock_probe.rs holds the lock on q.lock
and attempts a second acquire of the same file. The output was "same-thread second OFD try_lock:
Err(WouldBlock)" and "nested acquire returns within 5s while outer held: false". The test is
`#[ignore]`, so the focused run above, which does not use `--ignored`, cannot catch the hang.

Fix: `drop(directory)` before the conditional crate is written. Or let `write_generated_crate` take
a distinct prefix suffix, as `harness_generation` does with `{name}`.

### FND-002 detail

Fix: end each of the eight manifests with `\n[workspace]\n`, as bounded_kani_corpus.rs:560,
kani_generation.rs:317 and bound_strategy_generation.rs:275 already do.

## Verdict

Changes requested. The mechanism is sound. Same-process threads really do exclude each other, a
leftover is reclaimed, and Drop runs before the lock is released. All 201 focused non-ignored tests
pass, and they leave only lock files behind. What the converted tests prove is unchanged: the
manifests, sources, commands and assertions are byte-identical apart from `.0` becoming `.path`.
But FND-001 hangs the real-Kani lane (`make kani` / `make kani-gate`), so the PR must not merge
before that one-line fix. FND-002 should be fixed in the same round. FND-003 and FND-004 are
hardening.

Red/green tests (question 5). `scratch_directory_lives_under_target_tmpdir` and
`scratch_directory_reclaims_leftover_from_dead_run` assert exactly what the old type violated:
temp_dir() placement, and a pid/nonce path that a leftover never matched. The old type was local to
each file, so neither test could have compiled against it as written; they are red by construction,
not by a recorded run. `scratch_directory_is_locked_while_live` has no old-behaviour counterpart;
it pins the new lock.

bounded_kani_corpus (question 4) is settled. I ran
`cargo test --locked --test it -- --ignored --test-threads=1 bounded_kani_corpus::` once, inside
`systemd-run --user --scope -p MemoryMax=16G -p MemorySwapMax=0`. 8 passed and 0 failed, including
the real `cargo kani` Verified, Falsified and missing-cover classifications. Afterwards no
quire-codegen-bounded-kani directory was left under target/tmp. That manifest carries
`[workspace]`, so it is not exposed to FND-002.
