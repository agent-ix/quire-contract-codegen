---
id: "SR-781"
title: "CG PR 228 gap analysis: AD-004 step 2e, evidence/ and strategy/ module move"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@1338e38cd96066bf7fe89d88ed5fcb842dd20c66; src/ (the 12 src files the PR touches), tests/it/{bound_census,bound_populations}.rs, spec/assurance/AD-004-cg-crate-layout.md (target tree :182-188, module map :230 and :243-247, dependency direction :268-309, migration order :620-640 and step 7 :746-753), spec/core/functional/interface-001-codegen-api.md:165-181 and :346-350, spec/spec.md:88"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AD-004
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-002
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-004
    type: references
---

# SR-781: CG PR 228 gap analysis

## Summary

Ticket: IR-348. PR: agent-ix/quire-contract-codegen#228 at 1338e38. This is a planless run
scoped to the change. It asks whether the PR does what AD-004 step 2e says, no more and no less,
and whether it loses any spec-test-code link.

- Step 2e as written (AD-004:620-624) says: "2c to 2g are `git mv` plus path fixes, imports by
  module path, no logic change ... 2e `evidence` and `strategy` (the one `pub mod bound_strategy`
  path leaves here; its callers in `tests/it/` and any item reached only by that path are
  re-exported by name or made private in the same PR)". The module map row for `lib` (:230) says
  "re-exports by explicit list, no logic. The one `pub mod bound_strategy` path leaves (step 2e);
  callers use the re-exported names". The PR meets this:
  - The five rows moved to exactly the target names: `harness -> strategy/harness.rs`,
    `strategy -> strategy/campaign.rs`, `bound_strategy/* (5 files) -> strategy/bound/`,
    `vacuity -> evidence/vacuity.rs` and `bound_coverage -> evidence/bound_coverage.rs`
    (tree :182-188, map :243-247).
  - `pub mod bound_strategy` is gone, and the 28 items reachable only through it are re-exported
    by name in an explicit `pub use` list in `lib.rs`.
  - The two `tests/it/` callers now import the root names.
  - No logic changed (SR-780 has the byte identity).
  - The public path change (`quire_contract_codegen::bound_strategy::{census,population,relation}::X`
    becomes `quire_contract_codegen::X`) is the change the AD announces. The crate is prerelease,
    and no sibling checkout uses the old path (`quire-driver` and the other checkouts were grepped
    by the coder and re-grepped here for `bound_strategy::`: no hits outside this repo's spec and
    reviews).
- Nothing beyond 2e. Files outside the two subsystems are untouched. Root-path imports in files
  that 2e does not move are left for 2g-0, as AD-004:628-631 says.
- Direction (AD-004:270-309). `strategy` and `evidence` import only `oracle` and `core` (and
  themselves), with no edge between them. The V1-name rule (:307-309) holds.
- Matrix. `quire coverage --scope . --json` (quire 0.33.0) was run at base 879de44 and at head.
  The totals are identical: 211 of 320 rows backed, 285 criteria, 66 unbacked rows, 0 status lies,
  4 untracked symbols, 75 unmatched tags, 291 obligations, 50 implements, 10 suspicions.
  `implements` loses the four `lib.rs` lines (`harness` FR-002, `strategy` FR-002, `vacuity`
  FR-004, `bound_coverage` FR-004) and gains the same four on `strategy/mod.rs` (`harness`,
  `campaign`) and `evidence/mod.rs` (`vacuity`, `bound_coverage`). The only other differences are
  a line shift in one `binding_census` example and +1 production symbol examined (the new module
  declarations), with the same 41 matched. No trace link was lost. Test counts are equal at base
  and head (115 unit, 252 integration with 9 ignored, 1 doc).
- Stale old-path text. No `src/`, `tests/`, `scripts/`, README or CLAUDE.md text still names
  `bound_strategy::`, `src/harness.rs`, `src/strategy.rs`, `src/vacuity.rs`,
  `src/bound_coverage.rs` or `src/bound_strategy/`. The spec still uses the old names in two
  places:
  - `interface-001` lists `bound_strategy::census::{compute_census, render_edge_constants,
    render_boundary_constants}` and `bound_strategy::population::{side_values, render_population}`
    (:165-181, :346-350), paths that no longer exist.
  - The registry row in `spec/spec.md:88` still names `harness`, `strategy` and `bound_strategy`.
  AD-004 step 7 (:746-748: "the registry rows by directory ... `interface-001` and `tests.md`
  fixed") defers both explicitly, so neither is a defect of this PR. The step-7 author should
  rewrite the five interface-001 names to root names. No test checks interface-001 names against
  code. Old file names in `reviews/` and `plan/` are history and stay.
- Open PRs. Draft #222 (IR-344 step 1a, held) edits `src/harness.rs`, `src/strategy.rs` and
  `src/bound_strategy/generation.rs`, all of which this PR moves. Its rebase will need rename
  following and `lib.rs` conflict resolution. This is noted only: #222 is held, and any rebase
  conflict is its problem. Draft #209 also touches `lib.rs`, which is likewise a note only.

## Verdict

PASS, clean. The PR does exactly what AD-004 step 2e says, and the public-path change is the one
the AD announces. Re-exporting all 28 items is within "re-exported by name or made private". It is
also the only choice that keeps every public type nameable, which SR-780 explains. No spec-test-code
link was lost. The stale interface-001 and registry text is deferred to step 7 by the AD itself.
The PR is mergeable.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
