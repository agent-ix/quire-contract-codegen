---
id: "SR-720"
title: "CG PR 219 code review (Rust lane): AD-004 steps 2a-2b definition moves"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@a5aafb81ca65bc42a1be3a3c187665e99fa58ab4; src/artifact.rs, src/diagnostic.rs, src/source_map.rs, src/kani_census.rs, src/kani_identity.rs, src/lib.rs, src/publication.rs, src/oracle.rs, src/kani.rs, src/kani_obligations.rs, src/state_frame.rs, and the import-only edits in 18 other src files"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AD-004
    type: references
---

# SR-720: CG PR 219 code review (Rust lane)

## Summary

Ticket: IR-344. PR: agent-ix/quire-contract-codegen#219 at a5aafb8, one commit on main 886ce0a
(merge base equals main; PR 210, the step 2 precondition, is merged at b75e1ac). Methods:
code-review with the rust-review lane folded in, a per-item motion check, a re-derivation of the
`use crate::` edges at base and head, a public-API comparison, and an independent byte-identity
dump run in two uncommitted scratch worktrees (deleted afterwards).

Checked and found as claimed:

- Pure motion. Every moved item was extracted at base (from `oracle.rs`, `publication.rs`,
  `kani.rs`, `kani_obligations.rs`, `state_frame.rs`) and at head (from the five new modules) and
  compared verbatim, doc comments and attributes included. All 47 items are byte-identical
  except three visibility widenings that the split needs: `ArtifactBundle::revalidate`,
  `artifact::diagnostic` and `kani_census::dependency_site` went from private to `pub(crate)`
  (`diagnostic`'s signature reflows to four lines under rustfmt). No derive, serde attribute,
  field order, variant order or Display/label string changed. The only non-import body edit
  outside the moved items is `bundle.artifacts` to `bundle.artifacts()` in `publication.rs` (the
  field is now in another module; the accessor returns the same slice). No `type_name`,
  `module_path!` or `file!` exists in `src/`, so no output can carry a module path.
- Byte identity, reproduced independently. Four `#[ignore]`d dump tests were appended to the
  existing test modules in base and head scratch worktrees (identical text both sides): the V1
  contract harnesses over all five clause kinds via `negotiate_kani_obligations`, the golden V2
  scalar set and its refused items, both state-frame harnesses and two refusals, the V1
  `generate_kani_bundle` bundle with no dependencies, with required/assumed/stubbed dependencies
  and with an invalid one, the Boolean oracle and its source map, `ArtifactBundle` success and
  four refusals with their serde JSON, an atomic publish and the written files,
  `GenerationTerminalState::ALL` labels, and one corpus case. Debug output plus serde JSON of
  every record and identity: 14 files, 768,340 bytes, `diff -r` identical. The head build was
  forced (`touch` of every source) and the compile line names the head worktree.
- No compatibility layer. The 244 names re-exported from `lib.rs` are the same set at base and
  head; every new module is private (`mod`, not `pub mod`); no `pub use` keeps an old module
  path; nothing outside `publication.rs` names `publication` except `lib.rs`.
- Rust idioms: docs on every new module, `#![deny(missing_docs)]` still holds, no `allow` or
  `expect` added, no new `unwrap` in non-test code, visibility kept at the minimum the split
  needs. Flat names `kani_census` and `kani_identity` map to the AD's `kani/census.rs` and
  `kani/identity.rs`; `artifact`, `diagnostic`, `source_map` map to `core/*`.
- Gates at this head, run by this reviewer: `make ci` exit 0 (115 unit, 246 integration, 9
  ignored, `make spec` with the 3 baseline warnings, rustdoc, msrv, deny, audit-unsafe);
  `make kani` run by this reviewer at a5aafb8: exit 0, 9 passed in 1247.66 s (the four
  `kani_obligations` real runs, the three state-frame runs, the witness join and the skeleton
  spine).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The publication diagnostic constructor, private in `publication.rs` before, is now the crate-visible `artifact::diagnostic`, the same name as the sibling module `crate::diagnostic`. `publication.rs` imports it as a bare `diagnostic`, so a reader of `use crate::artifact::{diagnostic, ...}` or of any later `use crate::diagnostic` sees two unrelated things under one name. A distinct name (`publication_diagnostic`) avoids the collision; harmless to behaviour | src/artifact.rs:212-224, src/publication.rs:12-15 |

## Verdict

Mergeable on the code: the move is verbatim, outputs are byte-identical on an independent dump,
the public surface is unchanged and no shim was added. FND-001 is a naming nit that can be fixed
in this PR or at step 2c when `artifact` becomes `core/artifact.rs`.
