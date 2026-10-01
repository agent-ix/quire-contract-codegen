---
id: "SR-731"
title: "CG PR 220 gap analysis: AD-004 step 2c, core/"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@c9af05bce5f9fd67b16bb8e03d14db18d22da9c7; src/ (the 27 files the PR touches), spec/assurance/AD-004-cg-crate-layout.md (migration order and module map), spec/spec.md registry, spec/tests.md and the subsystem matrices"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AD-004
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-005
    type: references
---

# SR-731: CG PR 220 gap analysis

## Summary

Ticket: IR-344. PR: agent-ix/quire-contract-codegen#220 at c9af05b. Planless run, scoped to the
PR's change: a rename-only step, so the questions are whether the move lost a spec-test-code
link, whether it does what AD-004 step 2c asks, and whether the AD can be followed for the
remaining steps 2d to 2g.

- Matrix. `quire coverage --scope . --json` (quire 0.33.0) at base 041c5ae and at head: identical
  totals (210 of 319 rows backed, 284 criteria, 66 unbacked, 0 status lies, 4 untracked symbols,
  75 unmatched tags). The one difference in the whole `implements` list (49 entries both sides)
  is the FR-005 tag on symbol `artifact`, which moved from `src/lib.rs` to `src/core/mod.rs`. No
  test was added, removed or retagged.
- Step 2c as written. The AD asks for `git mv` plus path fixes, imports by module path, no logic
  change, for `core`. The five `core/` modules that exist today moved; `canonical`, `naming` and
  `ir/` have no source yet, so there is nothing to move for them. No file outside `lib.rs` imports
  a `core` item through the crate root any more (checked by parsing every `use crate::` group).
- Spec text. No spec, matrix or CLAUDE.md text names the old `src/<module>.rs` paths. The spec.md
  registry's Core row lists `lib`, `publication` and `oracle` and none of the `core` modules; that
  row was already stale after step 2a and AD-004 step 7 rewrites the registry by directory, so it
  is not a finding against this PR.
- Stubs and hollow evidence: none; the PR adds no code.
- Open PRs. CG #209 (draft, parked) already conflicts with main in `src/lib.rs`,
  `src/state_frame.rs` and three spec files; merging #220 adds no new conflicting file
  (`git merge-tree` against main and against this head list the same paths). Report only.

The coder raised four problems with AD-004 for steps 2d to 2g. Each was checked against the code:

1. Root imports. Measured at this head, 12 files still import non-`core` items through the crate
   root (`bound`, `bound_coverage`, `bound_strategy/{census,generation,population}`,
   `bounded_kani_corpus`, `harness`, `kani`, `kani_obligations`, `routed_generation`,
   `state_frame`, `strategy`), down from the AD's 17. L-2 forbids them and lands with 2g, but no
   step says who rewrites them (FND-003).
2. Naming helpers. Confirmed: `bounded_readable_component`, `upper_camel`, `unique_names`,
   `unique_pair`, `oracle_symbol`, `reference_identifier` and `dependency_parameters` are in
   `oracle.rs:320-927`, with `MAX_GENERATED_SOURCE_BYTES` at `oracle.rs:19`, imported by strategy,
   Kani and oracle modules. The AD maps them to `core/naming.rs` but schedules no step that can
   move them (FND-001).
3. Splits in rename steps. Confirmed for 2f (FND-002). For `kani_witness_join` the AD already
   puts the text-scanning move at step 5, not 2g, so 2g moving the file whole to
   `replay/witness.rs` is consistent with the AD; only the "no logic change" wording of 2f needs
   amending.
4. `mod core` and the `core` crate: a code-level hazard, recorded as SR-730 FND-001. The coder's
   E0659 claim did not reproduce (a bare `pub use core::...` in `lib.rs` compiles and resolves to
   the local module). It needs no AD change; the AD's `core/` name can stay.

Where the AD edits belong: not in this PR, which is correct as a rename. FND-001 has to be
settled before step 2d moves `oracle.rs` (after 2d the helpers sit in `oracle/` and a later move
is a cross-directory split), so the amendment goes in the 2d PR, or in a small AD PR ahead of it.
FND-002 and FND-003 can ride in the same amendment.

## Verdict

CONDITIONAL on the AD, not on the code: the PR is a correct step 2c and loses no trace link. Three
AD-004 gaps for the later steps, one medium.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AD-004 maps the shared naming helpers of `oracle` to `core/naming.rs` (module map, target tree) but no migration step moves them: 2a moved only artifact, diagnostic and source-map types, 2c to 2g are renames, and 2d moves `oracle.rs` whole into `oracle/`. As written, `core/naming.rs` is never created, and L-1 ("every module lives where the map names") cannot pass at 2g. `MAX_GENERATED_SOURCE_BYTES` (`oracle.rs:19`, used by strategy and Kani modules) has no target at all. Amend the AD with a definition-move step, like 2a and 2b, before or inside 2d, and give the constant a home | spec/assurance/AD-004-cg-crate-layout.md:167, spec/assurance/AD-004-cg-crate-layout.md:224, spec/assurance/AD-004-cg-crate-layout.md:599-605, src/oracle.rs:19, src/oracle.rs:320-927 |
| FND-002 | low | AD-004 step 2 says "2c to 2g are `git mv` plus path fixes, imports by module path, no logic change", and the same sentence's 2f includes "the `run`, `output`, `classify` split of `kani_execution` and `kani_transcript`, and the test back-edges", which is a split, not a rename. The risk section says each of 2c to 2g "must be refused" if it carries a logic change, so a reviewer of 2f has two rules that disagree. Amend 2f to say the split is a verbatim item move (as 2a and 2b were) and that moving the test back-edge to `tests/it/` is in scope | spec/assurance/AD-004-cg-crate-layout.md:598-605, spec/assurance/AD-004-cg-crate-layout.md:720-722 |
| FND-003 | low | L-2 forbids importing through the crate root and its test lands with 2g, but no step assigns the rewrite: 12 files still do it at this head. The phrase "imports by module path" in 2c to 2g can be read as covering only the moved modules' own paths. State that each of 2d to 2g rewrites the root imports in the files it moves, and that 2g sweeps any left before its layout test | spec/assurance/AD-004-cg-crate-layout.md:523-526, spec/assurance/AD-004-cg-crate-layout.md:79-81, src/routed_generation.rs:22-30 |

## Coverage

- Rows: 210 backed of 319 at base and at head; criteria 284; unbacked 66 (unchanged, none
  attributable to this PR).
- Semantic review: skipped (a pure-motion PR adds no intent to judge).
- Plan completion: not assessed
