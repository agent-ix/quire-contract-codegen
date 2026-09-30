---
id: "SR-621"
title: "IR-433 code review: use-local snapshots Cargo.lock after LOCAL_PATCHES validation"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@003b2ed002b3c8c9e908dbb45d141cdc2fdc8b1f; Makefile"
relationships: []
---

# SR-621: IR-433 code review (PR #197)

## Summary

Ticket: IR-433. PR: agent-ix/quire-contract-codegen#197, head 003b2ed, diffed against
origin/main.

The PR moves one line in the `use-local` recipe. The Cargo.lock snapshot
(`.cargo/Cargo.lock.pre-local`) is now taken after the LOCAL_PATCHES validation loop, not
before it. Before this change, a malformed entry or a missing sibling made the target fail but
left a stale snapshot behind. No `.rs` file changed, so there is no rust-review lane. There is no
spec change.

## Method

I read the whole `use-local` / `use-remote` block at the head. The QSL sibling is not cloned on
this machine, so I used a reduced LOCAL_PATCHES that points at quire-verification-contracts.
I ran each case in the worktree and compared the Cargo.lock sha256 against the committed lock:

1. Malformed entry (`bad::entry`): exit 2, `.cargo/` empty.
2. Missing sibling (default list, QSL absent): exit 2, `.cargo/` empty.
3. Successful use-local: the config is written and the lock re-resolves. Then use-remote restores
   the lock byte for byte, and `git status` is clean.
4. A fake cargo (via `TRUSTED_HOME`) records `.cargo/` when metadata runs, changes the lock, and
   fails. When it ran, both the snapshot and config.toml existed. After the failure the lock was
   restored byte for byte and `.cargo/` was empty.
5. Real cargo failure (a sibling Cargo.toml that does not parse): exit 2, lock restored,
   `.cargo/` empty.
6. A successful use-local, then a malformed entry: the earlier snapshot is kept and config.toml
   is removed. Then use-remote restores the committed lock.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Clean. All four properties in the brief hold, and each was checked by running it.

- A bad or missing entry exits non-zero and leaves no `.cargo` files.
- The snapshot exists before the first patch line is written, and before `cargo metadata` runs.
- When metadata fails, the lock is restored and the snapshot is removed.
- use-remote still restores the lock after a successful use-local.

The `[ -f ... ] ||` guard still keeps an older snapshot when use-local is run again, so that
case is unchanged.

A note that is not a defect: a failed first run leaves an empty `.cargo/` directory. It was
created before this change as well, and git does not track it.
