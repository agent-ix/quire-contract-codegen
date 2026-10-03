---
id: "SR-850"
title: "CG PR 235 code review: AD-004 step 2g-0, root-import sweep"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen@84d951a7d013147b6e5bea2c79b9b01c03c66813; src/kani/generate/{clause,negotiate,outcome,v1_bundle}.rs, src/kani/identity.rs, src/kani_witness_join.rs, src/oracle/{boolean_v1,claim}.rs, src/routed_generation.rs, src/spine_replay.rs; base 603326990802012d0a096cbbf4144e8e4ea8c14f"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AD-004
    type: references
---

# SR-850: CG PR 235 code review (code-review with the rust-review lane)

## Summary

Ticket: IR-348. PR: agent-ix/quire-contract-codegen#235 at
84d951a7d013147b6e5bea2c79b9b01c03c66813, two commits on base
603326990802012d0a096cbbf4144e8e4ea8c14f. A paths-only PR. The Rust lane is folded in, as
`rust-review` says. The PR body was treated as a claim and re-measured.

Measured by the reviewer:

- Diff. `git diff origin/main...HEAD`: 10 files, +24/-18. Every hunk is a `use` list, an inline
  type path or an intra-doc link target. No file added, removed or renamed. `src/lib.rs`,
  `Cargo.toml` and `Cargo.lock` are untouched, so the crate-root `pub use` set is unchanged.
  No visibility changed, no shim, no history comment.
- Same items. Every new path names the item the old root path re-exported: `lib.rs` re-exports
  `OperationProvenance` and the scalar items from `oracle::scalar`, `ClaimMap` and
  `OracleGenerationError` from `oracle::claim`, `BackendKind`/`Candidate` from `capability`,
  `negotiate_kani_obligations` from `kani::generate::negotiate`, the obligation vocabulary from
  `kani::generate::outcome`, `KaniRunOutcome` from `kani::classify`, `KaniErrorCode` from
  `kani::generate::census_validation`, `generate_state_frame_obligations` from
  `kani::generate::frame`, `generate_bound_oracles` from `oracle::bound_v1`,
  `MAX_GENERATED_SOURCE_BYTES` and `GenerationDiagnostic` from `core`, and
  `decode_falsification` from `kani_witness_join`. The code paths compile (gate log), so they
  resolve; the doc links were checked by rendering (next item).
- Rustdoc, rebuilt by the reviewer at base and head, in public mode and in `make rustdoc` mode
  (`--document-private-items`, `-Dwarnings`, warning-free at head). `all.html` is byte-identical
  in both modes. Every link `href` is unchanged, so each rewritten link resolves to the same
  target. The rendered *link text* did change on public pages (FND-001).
- Second commit. 84d951a adds `GenerationDiagnostic` to `clause.rs`'s existing
  `core::diagnostic::{...}` import and uses the bare name in the closure; consistent with the
  first commit. `v1_bundle.rs` spells the path inline instead; that file is interim and both
  forms are correct.
- Generated output. No changed line is in code that builds or renders generated text; the edits
  are imports, one closure parameter type and comments. The bytes of generated output cannot
  change.
- Gates. `cg-2g0-ci.log` (fmt, quire validate, clippy `-D warnings`, 115 unit + 246 integration
  (15 ignored) + 1 doc test, deny, unsafe audit, rustdoc `-Dwarnings`) ends
  `head=84d951a7d013147b6e5bea2c79b9b01c03c66813 exit=0`. `cg-2g0-kani.log`: 15 passed, ends
  with the same head and `exit=0`. Neither gate was re-run.
- Base. origin/main has advanced one commit (#232, 335d25b3162e0c9e82b66a4728d4b26cbd291115).
  `git merge-tree` merges cleanly and #232 adds no `crate::` path, so the sweep stays complete
  after merge.
- Hygiene. Title carries no bare ticket id; the body says "Part of IR-348"; full SHAs only.

## Verdict

PASS with one low, non-blocking finding. The sweep is paths only, resolves every path to the
same item, leaves `lib.rs` and the public API alone, and cannot change generated output. The
gates pass on the final head. Six new doc lines pass 100 columns; rustfmt does not wrap
comments here and SR-840 already declined to file that, so it is not filed. Using the
`[`Name`](path)` form from FND-001 would fix both.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Rewritten intra-doc links use the bare `[`crate::path::Item`]` form, so public rustdoc pages now show private module paths a downstream user cannot name (for example `crate::kani::classify::KaniRunOutcome::Falsified` on `decode_falsification` and the crate index); before, they showed the root path that works. The `href` targets are unchanged. Writing `[`KaniRunOutcome::Falsified`](crate::kani::classify::KaniRunOutcome::Falsified)`, as `outcome.rs:176` already does, keeps the displayed text usable | src/kani_witness_join.rs:92; src/kani/generate/outcome.rs:173,187; src/kani/identity.rs:27; src/oracle/claim.rs:30,74; src/oracle/boolean_v1.rs:22; src/spine_replay.rs:5; src/kani/generate/negotiate.rs:31,44 |

## Dispositions

Round 1, reviewed at 2487f620d4f3fcd15daf48b285f4b3e1dd01c477 (rebased onto origin/main
335d25b3162e0c9e82b66a4728d4b26cbd291115). `git range-diff` shows the two reviewed commits
unchanged by the rebase; the delta is the single doc-comment commit 2487f62. Rustdoc was
rebuilt at head and at origin/main, in public and `--document-private-items` modes. `all.html`
is byte-identical in both. Apart from source-line anchors, the only visible changes are the ten
link texts, from `crate::X` to the short root-exported name, for example
`KaniRunOutcome::Falsified` on `decode_falsification` and the index, and
`OperationProvenance::CallerDeclared` on `UpstreamBlocker`. Every `href` is unchanged. The three
bare short links in `spine_replay.rs` and `negotiate.rs` name items those files import, and they
render the same public names. Four added doc lines still pass 100 columns, because an inline
link target cannot wrap. That is accepted: rustfmt does not wrap comments here, and SR-840 set
the same precedent. Three `crate::kani::...` link texts still render on public pages
(`KaniObligationIdentity` and two `ProofDependencyGraph`/`validate_dependencies` links). They
date from step 2f and are outside this diff.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 2487f620d4f3fcd15daf48b285f4b3e1dd01c477 |
