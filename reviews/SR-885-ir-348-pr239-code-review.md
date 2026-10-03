---
id: "SR-885"
title: "CG PR 239 code review (with rust-review lane): AD-004 step 2g replay, routed and publication moves and the L-1/L-2 layout test"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen@807755fbeb8b79156ca168185172722dfb9021ac; src/lib.rs, src/replay/{mod,witness,function,frame}.rs, src/routed/{mod,capability,generate}.rs, src/publication/{mod,publish}.rs, tests/it/layout.rs, tests/it/main.rs, tests/it/kani_argument_order.rs (diff origin/main...HEAD, merge base 81a9c69)"
---

# SR-885: CG PR 239 code review

## Summary

Ticket: IR-348. PR: agent-ix/quire-contract-codegen#239 at 807755f, one commit on merge base
81a9c69. `origin/main` is one commit ahead (80f4785, #237: spec and reviews only, including a
5-line edit to AD-004's Risks). `git merge-tree` against it is clean, and no `src/` or `tests/`
file differs between 81a9c69 and 80f4785. The `rust-review` lane is folded into this file.

What I checked, independently of the PR body:

- Rename similarity (`git diff -M`): `publication.rs` to `publication/publish.rs` 100%,
  `capability.rs` to `routed/capability.rs` 100%, and 99% for `kani_witness_join.rs` to
  `replay/witness.rs`, `spine_replay.rs` to `replay/function.rs`, `frame_replay.rs` to
  `replay/frame.rs` and `routed_generation.rs` to `routed/generate.rs`. The full diff of each
  moved file is one `use` path (`function.rs`, `frame.rs`), one `use` entry moved inside the
  braced `use crate::{..}` block by rustfmt order (`generate.rs`), and the two intra-doc link
  paths `crate::spine_replay` to `crate::replay::function` (`witness.rs`). Nothing else.
- Item proof: every item declaration line in `src/` (fn, struct, enum, trait, const, static,
  type, impl, macro_rules, any visibility and indentation), sorted, is 1361 lines at base and at
  head and the lists are equal. The six moved files hold 156 items by the AD's counting rule on
  each side.
- New `mod.rs` files hold only `//!` headers, comments and `pub(crate) mod` lines. That is the
  same form as the 2e and 2f `mod.rs` files (`evidence`, `strategy`, `kani`, `core`, `oracle`).
  The modules were private `mod` in `lib.rs`, and `pub(crate) mod` inside a private parent is
  what the `lib.rs` `pub use` needs. No item visibility changed.
- `Implements:` tags: the multiset of `Implements:` lines in `src/` is 22 at base and at head and
  equal. FR-005, NFR-001, FR-019, FR-022, FR-016 and FR-015-AC-33 moved from `lib.rs` to the `mod`
  lines in the new `mod.rs` files. The old `IR-211:` comment on `mod kani_witness_join` became a
  plain description with no ticket id, which is correct (no migration history).
- `lib.rs` `pub use`: I parsed every `pub use` leaf at base and at head. Both have 272 names, all
  distinct, and the sets are equal. The edits are repointed paths plus rustfmt ordering.
- rustdoc. I rebuilt both sides in one target dir. Public `all.html` is byte-identical: sha256
  61a1a1c44741fbb1ef592b24e2d2621b6b503d7c80fad68d91426b900601b2cb on both, which matches the
  PR body. Private-mode `all.html` has 1042 links on each side, equal after mapping the six old
  module prefixes to the new ones. The private doc file tree differs only by the three new
  module index pages. `RUSTDOCFLAGS=-Dwarnings cargo doc --document-private-items` exits 0 on
  head. The two edited links resolve to the same module page: before, `../spine_replay/index.html`
  ("mod quire_contract_codegen::spine_replay"); after, `../function/index.html`
  ("mod quire_contract_codegen::replay::function"). They sit in the `//!` header of a private
  module, so no public page shows them. The SR-850 class (a private path shown on a public page)
  does not arise from this diff.
- Generated-artifact byte identity: no file in `src/` uses `file!`, `line!`, `column!`,
  `module_path!`, `type_name`, `CARGO_MANIFEST_DIR`, `env!`, `include_str!`, `include_bytes!`,
  `track_caller` or `panic::Location`, and items moved verbatim. So no emitted byte can depend on
  a module path, and the hooked dump was not needed.
- Layout test (`tests/it/layout.rs`). Its scanner blanks comments, string, raw-string and char
  literals, then expands `crate::` use trees. On the real tree it finds 441 edges in 39 files.
  A grep finds `crate::` in 40 files; the one extra, `oracle/claim.rs`, has it only in doc
  comments. So the scan is not vacuous on this source. I ran nine mutation probes on my own
  worktree and reverted each one:
  - P1, `oracle/claim.rs` imports `kani::abi`: the direction test failed.
  - P2, `fn _probe(_: crate::KaniSolver)` in `replay/function.rs`: the root-path test and the
    direction test failed.
  - P3, `kani/generate/outcome.rs` imports `negotiate`: the acyclic test failed.
  - P4, `routed/capability.rs` renamed and loaded with `#[path]`: L-1 failed.
  - P5, `kani/generate/frame.rs` imports `negotiate`: all 5 tests passed (FND-001).
  - P6, `kani/output/report.rs` imports `kani::generate::outcome`: the direction test failed.
  - P7, a `#[cfg(test)]` module in `core/naming.rs` imports `oracle`: the direction test failed.
  - P8, `kani/output/report.rs` imports `routed`: the direction test failed.
  - P9, a stray `src/stray.rs`: L-1 failed.
- Rust idiom in `layout.rs`: no `unsafe`, `expect` only in test I/O, clippy clean under
  `-D warnings` (the coder's `make ci` log). `may_import` panics on an unknown directory, which
  L-1 would already have failed on. The test carries `Trace: AD-004 L-1` / `Trace: AD-004 L-2`,
  so it does have trace tags. That matches the existing precedent: `core/identity.rs` L-10 tests
  and `tests/it/scratch_crate.rs` both use `Trace: AD-004`. `quire coverage` lists all of these as
  untracked symbols, the same as before.
- Gates: `cg-2g-ci.log` ends `head=807755fbeb8b79156ca168185172722dfb9021ac exit=0` (fmt-check,
  spec, clippy, MSRV test, deny, audit-unsafe, rustdoc, test with 252 passed). `cg-2g-kani.log`
  ends with the same head, exit 0, 15 passed. I did not rerun `make kani`. I ran
  `cargo test --locked --test it layout` myself: 5 passed.
- Hygiene: the title has no bare ticket id, the body says "Part of IR-348", and the only SHA in
  the body is the full head. No shim, re-export alias or compatibility layer.

## Verdict

The motion is clean. It is verbatim, the item set and the public API are unchanged, the
rustdoc public output is byte-identical, and the Implements tags are preserved one for one. The
layout test is real and not vacuous, and eight of nine probes are killed by the right test.
One medium finding: the L-2 test leaves out the `kani/generate/` order rule from the AD's
dependency direction, and probe P5 passes all five tests. Three low findings. Changes requested
for FND-001. Either add the check or amend the AD so that L-2 explicitly scopes it out.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The L-2 test does not check the AD's `kani/generate/` order rule (outcome, record, census_validation, families, then negotiate; a family never imports negotiate, negotiate imported by none; frame imports outcome only). It checks only the top-level `kani/` order. Probe P5 (`frame.rs` importing `negotiate`) passes all five tests, and P3 (`outcome.rs` importing `negotiate`) is caught only by the cycle. The `test_support` rank also lets it import `census`, where the AD says identity and abi only | tests/it/layout.rs:119-132, tests/it/layout.rs:499-516 |
| FND-002 | low | The assertion message in `kani_argument_order.rs` still names the deleted `src/kani_witness_join.rs`. A failure would send the reader to a path that no longer exists. The AD allows only comment-line path updates under `tests/` in motion steps, so the fix needs either an AD amendment that allows path-only edits to failure-message strings, or a follow-up | tests/it/kani_argument_order.rs:119-123 |
| FND-003 | low | The `Cargo.toml` comment on `qsl-replay` still cites `src/spine_replay.rs`, which is now `src/replay/function.rs`. It is a comment outside `tests/`, within step 2g's path fixes, and was missed | Cargo.toml:23-25 |
| FND-004 | low | `l_2_the_module_graph_is_acyclic` has its only assertion inside `if let Some(cycle)`, and no test asserts a floor on the real edge set (for example, edges > 0, or `negotiate` reaches `outcome`). `quire coverage` adds a `vacuous-under-guard` suspicion for it, 10 to 11 against the base. A future regression in `code_only` that blanks real code would leave every L-2 test green | tests/it/layout.rs:518-545 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | medium | The L-2 test does not check the AD's rule for `kani/run`: it "imports `identity`, `output` and `classify` (and `abi`, for `KaniSolver`), not `generate`". The fix gives `output` and `test_support` exact import sets but leaves `run` on the plain rank, and `generate` (3) ranks before `run` (5). Probe P10, `kani/run/execute.rs` importing `kani::generate::outcome`, passes all 5 tests. The real `run/` files import only `abi`, `classify`, `identity`, `output` and `test_support`, so an exact set `{identity, output, classify, abi}` (with `test_support` exempt, as for every test module) would pass today | tests/it/layout.rs:551-557 |

## Dispositions

Round 1, reviewed at ea99091f21f71954932d36f6059d93ca00c21929. The PR was rebased onto 80f4785; `origin/main` has not moved since. `git range-diff` shows the reviewed commit unchanged, plus the fix commit ea99091. That commit changes `Cargo.toml`, AD-004, `tests/it/kani_argument_order.rs` and `tests/it/layout.rs`, and no `src/` line, so the `make kani` evidence from 807755f still stands. `cg-2g-r2-ci.log` ends `head=ea99091f21f71954932d36f6059d93ca00c21929 exit=0`. I ran the layout tests myself: 5 passed. I ran 8 probes, each reverted:

| Probe | Result |
| --- | --- |
| P5 `frame` imports `negotiate` | direction test FAILED |
| P11 `test_support` imports `census` | FAILED |
| P12 `output` imports `census` | FAILED |
| P13 `outcome` imports `record` | direction and cycle tests FAILED |
| P14 `precondition` imports `contract` | FAILED |
| P15 `frame` imports `scalar` | FAILED |
| P10 `run` imports `generate` | all 5 passed (FND-005) |
| P16 `test_support` imports `oracle` | all 5 passed. Not a defect: the AD's "identity and abi" names `kani/` modules, and `test_support` already imports `core` |

`quire coverage` suspicions are back to 10, and none is in `layout.rs`.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | ea99091 |
| FND-002 | fixed | ea99091 |
| FND-003 | fixed | ea99091 |
| FND-004 | fixed | ea99091 |

Round 2, reviewed at fbed9858a726d6346976075d2c2723631d6ab805. The delta since ea99091 is one line in `tests/it/layout.rs`, `"run" => Some(&["identity", "output", "classify", "abi"])`, plus the AD-004 step 7 text. `origin/main` moved to 348fb33 (#238, oracle/function and its tests). `git merge-tree` against it is clean. I ran the layout tests on the merged tree and all 5 passed. At head, the layout tests pass, 5 of 5. Probe P10 (`run` imports `generate`) and probe P17 (`run` imports `census`) now each fail the direction test; both were reverted. `cg-2g-r3-ci.log` ends `head=fbed9858a726d6346976075d2c2723631d6ab805 exit=0`.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-005 | fixed | fbed985 |
