---
id: "SR-730"
title: "CG PR 220 code review (Rust lane): AD-004 step 2c, core/"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@c9af05bce5f9fd67b16bb8e03d14db18d22da9c7; src/core/mod.rs, src/core/{artifact,diagnostic,identity,profile,source_map}.rs (renamed from src/), src/lib.rs, and the import-only edits in 21 other src files"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AD-004
    type: references
---

# SR-730: CG PR 220 code review (Rust lane)

## Summary

Ticket: IR-344. PR: agent-ix/quire-contract-codegen#220 at c9af05b, one commit on main 041c5ae
(merge base equals main). Methods: code-review with the rust-review lane folded in, a
`git diff -M` motion check, a public-API comparison, a name-resolution probe of the root `core`
module in a scratch crate, an independent byte-identity dump in two uncommitted scratch
worktrees (deleted afterwards), and both gates run by this reviewer.

Checked and found as claimed:

- Pure motion. `git diff -M origin/main HEAD`: five renames (`identity.rs` and `profile.rs` 100%,
  `artifact.rs` 98%, `diagnostic.rs` 97%, `source_map.rs` 94%), one new `src/core/mod.rs`, and
  import-path edits. The only content edits in moved files are the three module doc lines
  "becomes `core/x.rs`" to "is `core/x.rs`" and `crate::diagnostic` to `crate::core::diagnostic`
  in `artifact.rs`. Every other hunk in the 21 files is a `use` path rewrite (plus rustfmt
  reordering of the `use` group); no expression, string literal, derive or attribute changed.
- Visibility. The five modules were private `mod x;` at the root and are `pub(crate) mod x;`
  inside private `mod core;`. That is the minimum: a private child of `core` would be invisible
  to the sibling subsystems that import it. No item inside the modules changed visibility.
- No shim. No `pub use` or alias keeps an old path; `crate::artifact` and the others no longer
  resolve. The `Implements: FR-005` tag moved with `artifact` into `core/mod.rs`, and quire
  coverage sees it there (SR-731).
- `core` imports nothing else in the crate: `src/core/` names only `crate::core::` and two test
  `use super::` lines.
- Public API. The crate-root `pub use` name set is identical at base and head (244 leaf names by
  this reviewer's count; the coder counts 241 with a different tokenization; the set difference is
  empty either way). The rustdoc `all.html` item list is compared under Verdict.
- No output can carry a module path: `src/` has no `module_path!`, `type_name`, `file!` or
  relative `include_str!` in the moved files.
- Byte identity, reproduced independently. `Artifact::new` and `ArtifactBundle::new` were
  instrumented identically at base and head (a 64-bit SipHash plus length of the `Debug` of every
  constructed artifact and of every bundle result, appended to a file) and the whole non-ignored
  suite (`--lib` and `--test it`) run on each. The sorted record multisets are compared under
  Verdict. This covers every artifact any test generates, through every generator.
- Gates at this head, run by this reviewer: `make ci` exit 0 (115 unit and 246 integration
  passed, 9 ignored, under both `test` and `msrv`; `make spec` with the 3 baseline warnings;
  rustdoc, deny, audit-unsafe clean). `make kani` result under Verdict.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | `mod core;` at the crate root shadows the extern-prelude `core` crate, with no ambiguity error, in `lib.rs` and in any module that glob-imports the crate root. Probed in a scratch crate: with a root `mod core`, `use core::mem::size_of;` in the root is E0432 and `core::mem::size_of` under `use crate::*;` is E0433; in other modules `core::` still means the `core` crate. Probed in this PR's head: `pub use core::profile::RUNTIME_DEPENDENCY_SOURCE;` in `lib.rs` compiles and resolves to `crate::core`, so the coder's stated reason for the `crate::core::` prefix (E0659 ambiguity) does not reproduce; the prefix is still the clearer spelling. Today a mistake is always a compile error, never a silent misresolution, because no `crate::core` child shares a name with a libcore module (`artifact`, `diagnostic`, `identity`, `profile`, `source_map`, and the AD's future `canonical`, `naming`, `ir` are all free). It becomes silent if a future `core/` file takes a libcore module name (`fmt`, `ops`, `hint`, `str`, ...) and the crate-root or glob scope uses that path. Nothing records this. The AD names the directory `core/`, so no rename is needed: add one comment at `mod core;` saying it shadows the `core` crate in this file, that `lib.rs` paths are written `crate::core::` or `::core::`, and that `core/` children must not take libcore module names | src/lib.rs:10-11, src/lib.rs:110-111 |

## Verdict

- Public API: rustdoc `all.html` lists 273 item pages at base and at head, the same set (a
  `RUSTDOCFLAGS=-Dwarnings cargo doc --no-deps` build in each tree).
- Byte identity: 7,187 records at base and at head (7,095 artifacts and 92 bundle results,
  166,489,871 bytes of `Debug` text hashed), sorted record files byte-identical (sha256
  206675be...992c8d both sides); 115 unit and 246 integration tests passed in each instrumented
  run. The head build compiled the head scratch tree. Two base runs were not needed: the base
  and head record sets matched exactly, so no record was nondeterministic.
- `make kani` run by this reviewer at c9af05b: make exit 0 (recorded from make itself), 9 passed, 0 failed in 1385.70 s (four `kani_obligations` real runs, three state-frame runs, the witness join, the skeleton spine).

Mergeable. The change is pure motion, the public surface is unchanged, no shim was added and the
gates pass. FND-001 is a one-comment hardening that can land in this PR or in step 2d.
