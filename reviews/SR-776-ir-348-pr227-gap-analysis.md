---
id: "SR-776"
title: "CG PR 227 gap analysis: AD-004 step 2d, oracle/ module move"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@8692a13a50fd56ab35e4ebbab4bc264ae47bd264; src/ (the 13 files the PR touches), spec/assurance/AD-004-cg-crate-layout.md (target tree :159-217, module map :228-266, dependency direction :268-310, migration order :573-753), spec/oracle (FR-014, FR-018, FR-021, matrix/tests.md), spec/assurance/AD-001-codegen-architecture.md:198"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AD-004
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-018
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-021
    type: references
---

# SR-776: CG PR 227 gap analysis

## Summary

Ticket: IR-348. PR: agent-ix/quire-contract-codegen#227 at 8692a13. Planless run scoped to the
change: does the PR do what AD-004 step 2d says, no more and no less, does it lose a
spec-test-code link, and does the AD still read consistently after it.

- Matrix. `quire coverage --scope . --json` (quire 0.33.0) at base ea07b79 and at head: identical
  totals (210 of 319 rows backed, 284 criteria, 66 unbacked, 0 status lies, 4 untracked symbols,
  75 unmatched tags, 290 obligations, 50 implements). After mapping the six old paths to the new
  ones, `obligations`, `unbacked_rows`, `untracked_symbols` and `suspicions` are equal and
  `unmatched_tags` is equal as a multiset (two pre-existing `exact_scalar.rs` test tags now on
  `oracle/scalar/mod.rs`). `implements` loses the three `lib.rs` lines on `exact_scalar`,
  `composite_equality`, `exact_function` and gains the same FR-014, FR-018, FR-021 lines on
  `oracle/mod.rs` (`scalar`, `equality`, `function`). No trace link lost.
- Step 2d as written (AD-004:620-631): "`git mv` plus path fixes, imports by module path, no logic
  change", "2d `oracle` (with `bound` landing as `oracle/bound_v1.rs`)", and "each of 2d to 2g
  rewrites every root-path import in the files it moves to a module path". Met: the six files
  moved to exactly the target-tree names (`claim.rs`, `scalar/`, `equality/`, `function/`,
  `boolean_v1.rs`, `bound_v1.rs`, AD-004:175-181 and map rows :235, :238-242); every `use` in the
  moved files is a module path; no logic edit (SR-775: byte-identical output, identical rustdoc).
  The AD says nothing about `oracle/mod.rs` beyond the directory; `lib.rs` "declares directories,
  re-exports the public API, holds no logic" holds, and `oracle/mod.rs` holds only `pub(crate)
  mod` lines. No re-exports or visibility are asked for, and none were added.
- Not more than 2d. `canonical.rs`, the Kani outcome map, the V2 contract arm and replay are
  untouched. Root-path imports in files 2d does not move (for example `kani_obligations`) are
  left for 2g-0, as AD-004:628-631 says.
- Direction (AD-004:270-279). `oracle/*` imports only `core` and `oracle`. The new `oracle::`
  edges come from `kani`, `kani_obligations`, `state_frame` (kani) and `harness`,
  `bound_strategy/generation` (strategy), both allowed arrows. No cycle.
- Stale path text outside `src/`. `spec/oracle/matrix/tests.md:22` (`src/exact_scalar.rs`) and
  AD-001:198 (`src/oracle.rs`) are spec text, which AD-004 step 7 ("Spec follows the code";
  `tests.md` fixed; AD-001's Current state fixed in that spec PR) defers: acceptable here.
  `FR-021:241` is a rejected-alternative row describing what `src/oracle.rs` did before FR-018's
  fix: history, still true of that file at that time; no change needed. Test comments naming the
  old `src/` paths are code, not spec, and no step defers them: SR-775 FND-001.
- PR #226 (IR-498, head ea3a898, based on 224ca6e) edits `src/composite_equality.rs`.
  `git merge-tree --write-tree` of ea3a898 with 8692a13 is clean (tree bc1fecd): rename detection
  carries #226's hunks into `src/oracle/equality/mod.rs`, and the merged tree has no
  `src/composite_equality.rs`. Either order rebases mechanically. Recommend #226 first (urgent fix);
  then rebase #227 and check that `git range-diff` shows the move patch unchanged, and re-run
  `make ci` on the rebased head. #226 changes generator output, so the byte-identity comparison
  must be redone against the new main only if the range-diff is not empty.

## Verdict

PASS for the PR: it does what step 2d says and loses no trace link. One AD gap found, which is
not a defect of this PR and does not block it: FND-001, to be closed by an AD amendment in its
own spec PR with a ticket.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AD-004 promises a split of `oracle/scalar/` that no migration step owns. The target tree says `scalar/` is "was exact_scalar, split along derivation, lowering and rendering" and the module map gives `exact_scalar` the fate "split" (`oracle/scalar/`; walkers to `core/ir/`). Step 2d is `git mv` only, so `exact_scalar` correctly lands whole at `oracle/scalar/mod.rs`. Step 3 moves `exact_scalar` onto `core/ir` (the walkers), and step 4b ports the scalar Kani family; neither schedules the derivation/lowering/rendering split. A reader planning step 3 or 4 cannot tell who does it, and L-1 checks directories only, so no gate notices if the split never happens. Amend AD-004 (own spec PR, own ticket) to name the step and PR that splits `oracle/scalar/`, or drop the split from the tree and map. | spec/assurance/AD-004-cg-crate-layout.md:177, spec/assurance/AD-004-cg-crate-layout.md:239, spec/assurance/AD-004-cg-crate-layout.md:620-622, spec/assurance/AD-004-cg-crate-layout.md:641-642 |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | deferred | IR-501: AD-004 amendment in its own spec PR (name the step that splits `oracle/scalar/`, or drop the split); ticket verified to capture the finding; not a defect of PR 227 (round 1, reviewed at 47e7699) |
