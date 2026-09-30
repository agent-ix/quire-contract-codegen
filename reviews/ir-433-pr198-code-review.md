---
id: "SR-624"
title: "IR-433 code review: use-local validates LOCAL_PATCHES before writing the local config"
type: SpecReview
date: 2026-09-30
scope: "agent-ix/quire-contract-codegen@9bf963dbf188450097d8b8e240fffc4ac363ffc0; Makefile"
relationships: []
---

# SR-624: IR-433 code review (PR #198)

## Summary

Ticket: IR-433. PR: agent-ix/quire-contract-codegen#198 (branch fix/use-local-rerun), head
9bf963d, diffed against origin/main 88d559e.

The PR changes only the Makefile. `use-local` no longer truncates `.cargo/config.toml` before
the LOCAL_PATCHES validation loop. The truncation now comes after validation and after the
Cargo.lock snapshot. The loop's `rm -f .cargo/config.toml` calls are gone. So a re-run with a
bad entry fails and leaves a working config in place. The block comment and the `use-remote`
help line now say that `use-remote` restores Cargo.lock from the pre-local snapshot. No `.rs`
file and no spec file changed, so there is no rust-review or gap-analysis lane.

## Method

- I compared the head Makefile with `origin/main:Makefile` of quire-contract-ir, after a fresh
  fetch. The use-local/use-remote comment, recipe and `use-remote` help line are the same in
  both. The only difference in that block is CG's own `LOCAL_PATCHES` list.
- I ran these cases in the worktree with
  `LOCAL_PATCHES=quire-verification-contracts:quire-verification-contracts:.`:
  1. Good use-local: exit 0, and config.toml was written (sha256 6f4d95d2...).
  2. Re-run with a malformed extra entry (`bad::entry`): exit 2. Config sha256 unchanged.
  3. Re-run with a missing sibling (`no-such-repo:x:.`): exit 2. Config sha256 unchanged, and
     the snapshot was kept.
  4. use-remote: the lock was restored from the snapshot, and `git status --porcelain` was
     empty.
  5. Fresh tree (no `.cargo/`), with a missing sibling and then a malformed entry: both exit 2.
     No files are left under `.cargo/`; only the empty directory from `mkdir -p` remains, which
     git does not track.
- `make deny`: exit 0 (advisories, bans, licenses and sources ok; the one-copy awk gate passes).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Approve. The diff matches the merged quire-contract-ir #227 and quire-contract-runtime #89
block, apart from CG's LOCAL_PATCHES. Every requested behaviour check passed. `use-local`
still creates `.cargo/` before validating, so a failed fresh run leaves an empty directory.
That holds no files, git ignores it, and IR does the same, so it is not a defect.
