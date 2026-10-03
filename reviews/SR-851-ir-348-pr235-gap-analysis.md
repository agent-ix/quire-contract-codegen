---
id: "SR-851"
title: "CG PR 235 gap analysis: AD-004 step 2g-0 scope against the crate"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@84d951a7d013147b6e5bea2c79b9b01c03c66813; spec/assurance/AD-004-cg-crate-layout.md (step 2 text on 2g-0, L-2, Risks: pure motion) checked against every file under src/ at the reviewed sha; base 603326990802012d0a096cbbf4144e8e4ea8c14f"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AD-004
    type: references
---

# SR-851: CG PR 235 gap analysis

## Summary

Ticket: IR-348. PR: agent-ix/quire-contract-codegen#235 at
84d951a7d013147b6e5bea2c79b9b01c03c66813. Planless run. The acceptance criteria are AD-004's
2g-0 text: rewrite every remaining root-path import (`use crate::{..., Item}` and
`use crate::Item`, `#[cfg(test)]` modules included), inline `crate::Item` paths in code, and
intra-doc links naming a root item; leave string literals alone; `lib.rs` keeps its re-export
list; paths only. The Risks section adds that 2g-0 is refused if it carries a logic change.

Examined, with the result:

- Nothing in scope is left. The reviewer scanned every `.rs` file under `src/` except `lib.rs`
  at head: each `use crate::{...}` list split at top-level commas, each `use crate::X`, and every
  `crate::X` in any line (code, doc comments, test modules), with X checked against the eleven
  top-level module names in `lib.rs`. The only hits are five string literals (`"crate::State"`,
  `"crate::operate"` in `kani/test_support.rs` and `kani/generate/frame.rs`,
  `"crate::assumed_predicate"` in the corpus), which the AD says to leave alone. No `super::`
  path in a top-level file reaches the crate root (the one hit is a test module importing its
  own parent). No `$crate::` path exists.
- Nothing out of scope changed. Every hunk is a `use` list, an inline path or a doc-link target;
  `lib.rs`, string literals, `tests/` and the build files are untouched. Visibility unchanged.
- Counts against the AD. Two inline code paths (`clause.rs`, `v1_bundle.rs`) and ten doc links,
  as the AD says. The AD names six files by their pre-2d/2f names (`generation`,
  `kani_obligations`, `spine_replay`, `kani_witness_join`, `kani_identity`, `oracle`); since
  then `kani_obligations` split into `kani/generate/outcome.rs` and `negotiate.rs` (2 + 2 links)
  and `generation`/`oracle` live under `oracle/` (`claim.rs` 2, `boolean_v1.rs` 1), so the ten
  links now sit in seven files. The AD text is qualified "At the time of writing", so it stays
  true and needs no amendment.
- The "12 files" of `use` lines the PR body mentions: steps 2d to 2f rewrote imports in the files
  they moved, as the AD requires, so `routed_generation.rs` was the one file left. The diff
  changes only that `use` list, and the scan at head finds no other, so none was missed.
- Pure motion. No logic, no generated-text code, no public API change; the rustdoc `all.html`
  item list is byte-identical between base and head (rebuilt by the reviewer).
- Test oracle. No test changed, which is right for a paths-only step. The compiler is a full
  oracle for the code paths, and each new path names the same item the root re-export did.
  Rustdoc `-Dwarnings` only proves a link resolves, not that it resolves to the same item; the
  reviewer closed that by comparing `href`s at base and head, which are identical.
- Merged state. origin/main has one newer commit (#232); it merges cleanly and adds no
  `crate::` path, so the sweep is still complete after merge.

## Verdict

PASS, no findings. Every item in 2g-0's scope is rewritten, nothing outside it changed, and the
AD stays accurate. The doc-link display form is filed once, in SR-850 FND-001.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
