---
id: "SR-618"
title: "IR-433 code review (incl. rust-review lane): one copy of first-party crates in CG"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@a57aa9108f0d49227ec8d818605acaf7bb895b53; Cargo.toml, Cargo.lock, Makefile, deny.toml, .gitignore, scripts/check_one_copy.awk"
relationships: []
---

# SR-618: IR-433 code review

## Summary

Ticket: IR-433 (the reviewer created it; the PR had no ticket). PR:
agent-ix/quire-contract-codegen#196, head a57aa91, diffed against origin/main c9856b0.

The PR switches the four direct first-party git dependencies (IR, runtime, qsl-replay,
verification-contracts) from `rev =` to `branch = "main"` and re-resolves Cargo.lock. It adds
`make use-local` / `make use-remote`, a `LOCKED` variable that drops `--locked` while a patch is
active, and `scripts/check_one_copy.awk`, which `make deny` runs. The rust-review lane covers the
Cargo and Makefile changes; no `.rs` file changed.

## Method

I read the whole non-lock diff and the owner-approved plan. I compared the IR and RT versions of
the same snippets on their `origin/main`. For the lock, I compared every changed line against
origin/main. I ran the awk gate on the head lock and on origin/main's lock, with and without
`-F'"'`, and ran `cargo tree --duplicates --target all -e normal,dev,build`.

For use-local I made a scratch clone of the head (the worktree was not touched). Its sibling
directory held the cargo git checkouts at the exact locked commits. I ran `make use-local` with
the default list and with a reordered list. I parsed each generated file with `tomllib` and
`cargo metadata`, then ran `make use-remote`.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | `use-local` writes invalid TOML, a duplicate `[patch."…/<repo>"]` table, when one repo's `LOCAL_PATCHES` entries are not adjacent. It still exits 0 with "wrote .cargo/config.toml", because the post-check ignores `cargo metadata`'s exit status and only greps for "was not used". | Makefile:127, Makefile:144-160 |
| FND-002 | low | `check_one_copy.awk` depends on its caller passing `-F'"'`. Run without it, it passes vacuously: exit 0 on origin/main's lock, which has 13 duplicated first-party crates. | scripts/check_one_copy.awk:3-7 |
| FND-003 | low | `use-remote` runs `git checkout -- Cargo.lock` unconditionally. It silently throws away any uncommitted lock change, including a `cargo update -p <crate>` move-up (the plan's documented way to pick up a new commit) made before `use-local`. | Makefile:164-166 |

### FND-001 detail

Reproduced with `LOCAL_PATCHES="quire-spec-language:qsl-replay:qsl-replay
quire-contract-ir:quire-contract-ir:. quire-spec-language:qsl-foundation:qsl-foundation"`. The
make target exits 0. `cargo metadata` then exits 101 with `TOML parse error at line 7 … duplicate
key`. The default list is adjacent, so the default path works: it parses as four patch tables
(IR 2 crates, runtime 1, verification-contracts 1, QSL 9). The comment at line 127 documents the
constraint, but nothing enforces it, and `LOCAL_PATCHES` is `?=`, so it can be overridden.

Fix: loop over `$(sort $(LOCAL_PATCHES))`. Make's word sort groups entries by their `<repo>:`
prefix. Alternatively, group per repo the way RT's `FIRST_PARTY_GIT_DEPS` loop does (`sort -u`
over the repos). Also fail when `cargo metadata` exits non-zero, not only on the "was not used"
warning.

### FND-002 detail

`awk -f scripts/check_one_copy.awk <origin/main Cargo.lock>` gives exit 0. With `-F'"'` it
reports the 13 duplicates and exits 1. Fix: add `BEGIN { FS = "\"" }` to the script so it cannot
be run in a way that passes vacuously. IR's merged copy has the same shape.

### FND-003 detail

Fix: snapshot the lock in `use-local` (for example `.cargo/Cargo.lock.remote`, which is
gitignored with the directory's config) and restore it in `use-remote`. Or restore only when
`.cargo/config.toml` exists, and print that the lock was reset.

## Verdict

Approve after FND-001, the one medium. It is a one-line fix (`$(sort …)`) plus a non-zero-exit
check.

What is right:

- One copy holds. The awk gate exits 0 on the head lock, and `cargo tree --duplicates` lists no
  agent-ix crate. The awk gate catches the real regression: on origin/main's lock it names
  quire-contract-model ×3 and 12 other crates ×2, exit 1. It reads Cargo.lock, which records
  dev-only and optional dependencies, so it avoids the dev-dependency and feature blind spots
  found in the cargo-deny bans on RT#88 (SR-613).
- The direct dependencies are all `branch = "main"`; `git grep` for `agent-ix/… rev|tag =` in
  Cargo.toml finds nothing. The lock diff is limited to first-party sources and edges: no
  third-party `version` line changed, and yoke / yoke-derive stay at 0.8.3. Three transitive
  first-party sources are still pinned by QSL's manifests: quire-canonical `tag=v0.3.0`,
  filament-core-data `rev=033e228` and quire-rs `rev=2823a93`. That is QSL-lane work (plan step
  2), and each still resolves to one copy.
- use-local, default list: it emits one `[patch]` header per repo, and the file is valid TOML.
  `cargo metadata` uses every patch, and the patched lock still passes the awk gate with no
  duplicates. use-remote removes the config and restores the committed lock (`git status` clean).
  `LOCKED` drops `--locked` only while `.cargo/config.toml` exists. `SIBLINGS` resolves through
  `--git-common-dir` to the main checkout's parent from this linked worktree. Absolute paths avoid the
  config-relative `../<repo>` trap that RT's version has in a worktree.
- Cross-repo: IR's merged snippet prints one header per entry. That is valid today only because
  IR lists two different repos. RT groups by repo. The three copies have diverged, and FND-001's
  fix would bring CG in line with RT.

Gates at a57aa91 (CARGO_TARGET_DIR inside the worktree, deleted afterwards):

- `make fmt-check`: exit 0.
- `make lint`: exit 0.
- `make deny`: exit 0 (advisories, bans, licenses and sources ok; awk ok).
- `cargo test --locked --workspace --all-targets --no-fail-fast`: exit 101. That is the 8 known
  tooling failures: `cargo kani` is not installed, and the LLVM export is missing. Lib: 80 passed.
  it: 205 passed, 8 failed, 5 ignored.
