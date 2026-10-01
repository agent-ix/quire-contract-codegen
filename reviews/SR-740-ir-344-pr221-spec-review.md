---
id: "SR-740"
title: "CG PR 221 spec review: AD-004 step 2d-0, 2f wording and 2g-0 sweep"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@5f257db12acc16dac22fece8c0a9e93aa3955183; spec/assurance/AD-004-cg-crate-layout.md"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AD-004
    type: reviews
---

# SR-740: CG PR 221 spec review (AD-004 amendments before step 2d)

## Summary

Ticket: IR-344. PR: agent-ix/quire-contract-codegen#221 at 5f257db, base 66a67c8 (step 2c
merged). The PR changes one file. It recounts the crate-root imports (lines 79-86), adds the
generated-source cap to the target tree (164-166), adds two module-map rows (231-232), adds step
2d-0, the 2f verbatim-move wording and step 2g-0 (606-626), and amends Risks (741-746) and Not
verified (796-798). Methods: integrity, consistency and id checks of the amended text, both
against the rest of AD-004 and against the code at base.

- Ids: no requirement id, L-id or step number changed. L-1 to L-n read as before. 2d-0 and 2g-0
  do not collide with any existing label, and planner-tracked steps 4e, 5, 6 and 7 are untouched.
  No other file in the repo names an AD-004 step label.
- No pin, SHA or digest added.
- `make spec` (quire at the brief's TRUSTED_HOME): the 3 baseline warnings (FR-017 line 159,
  FR-014 line 278 twice). AD-004 adds none.
- Helper list: all nine named items exist at `src/oracle.rs:821-933`. No other module defines one
  of the seven public names. `src/bound_strategy/generation.rs:802` has its own private
  `observation_name` with a different signature. That is a near-duplicate, but no collision.
- Constant home: `core/artifact.rs:16-18` already holds `MAX_ARTIFACTS`, `MAX_ARTIFACT_BYTES` and
  `MAX_BUNDLE_BYTES`, so it is the right home for the cap. Every user is above `core`.
- Root-import count: re-measured by parsing every `use crate::` statement, braces and multi-line
  forms included. The 12 files are exactly the ones listed. "Six further files" that use
  `use crate::{...}` with module paths only is also right: 18 files minus 12.
- Consistency: Risks now names 2d-0 and the 2f split as the only edits beyond renames, which
  matches step 2. The 2g-0 sweep comes before the layout test that lands with 2g, as L-2 needs.

## Verdict

Mergeable after two low wording fixes. Neither changes the plan.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The new row 231 and step 2d-0 call `rust_component` and `observation_name` "the private ... they use", and 2d-0 says "the items move unchanged". But `observation_name` is also called by `reference_key` (`src/oracle.rs:818`), which feeds the V1 analysis (`oracle.rs:511`, `670`, `814`) and stays in `oracle`. So `observation_name` cannot move as a private, unchanged item: it must become `pub(crate)` in `core/naming.rs`, or `reference_key` and `dependency_key` must move with it. State which, so a 2d-0 reviewer holding the "unchanged" rule does not refuse a needed visibility change | spec/assurance/AD-004-cg-crate-layout.md:231, spec/assurance/AD-004-cg-crate-layout.md:607-614, src/oracle.rs:817-831 |
| FND-002 | low | The existing row 230 still maps `oracle` "naming" to `core/naming.rs` as a split with no step. The new row 231 maps the same helpers again, with step 2d-0. Two rows now own one split. Drop "naming" and `core/naming.rs` from row 230, or point row 230 at row 231 | spec/assurance/AD-004-cg-crate-layout.md:230-231 |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 0d1a306: `observation_name` is listed as moving `pub(crate)`, the only edit to a moved item. `reference_key` and `dependency_key` stay in oracle. Checked: `reference_key` (oracle.rs:817) is called by `analyze_node` (511), `render_node` (670) and `dependency_key` (814), all V1, and it calls `observation_name`. The claim is right |
| FND-002 | fixed | 0d1a306: row 231 no longer lists naming or `core/naming.rs`. Row 232 alone owns the naming split, with its tests |
