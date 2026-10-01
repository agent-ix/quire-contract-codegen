---
id: "SR-721"
title: "CG PR 219 gap analysis: AD-004 steps 2a-2b definition moves"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@a5aafb81ca65bc42a1be3a3c187665e99fa58ab4; src/ (the 29 files the PR touches), spec/tests.md and the subsystem matrices"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AD-004
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-005
    type: references
---

# SR-721: CG PR 219 gap analysis

## Summary

Ticket: IR-344. PR: agent-ix/quire-contract-codegen#219 at a5aafb8. Planless run, scoped to the
PR's change: the PR moves definitions and adds no behaviour, so the question is whether the move
lost or broke any spec-test-code link.

- Matrix. `quire coverage --scope . --json` (quire 0.33.0) at base 886ce0a and at head: the same
  totals (210 of 306 rows backed, 66 unbacked, 0 status lies, 4 untracked symbols, 75 unmatched
  tags). The only differences are line numbers of the same symbols in `kani_obligations.rs`,
  `exact_scalar.rs`, `kani_execution.rs` and `composite_equality.rs`. No test was added, removed
  or retagged, and no tagged test changed module.
- Implements tags. The `implements` list is identical at base and head (47 entries; FR-005 and
  NFR-001 on `lib.rs` `publication` and `publication::write_bundle_atomic`; FR-015 on
  `kani_obligations`, `state_frame` and `negotiate_kani_obligations`).
- Stubs and hollow evidence: none introduced; every moved item is verbatim (see SR-720).
- Tests of the moved bundle validation (`bundle_construction_refuses_*`,
  `unsafe_and_duplicate_artifact_paths_are_refused`, traced to FR-005-AC-5 and TC-002) stay in
  `publication.rs`'s test module and pass; they reach the moved code through `ArtifactBundle::new`.

## Verdict

CONDITIONAL: one low traceability finding. No matrix row lost its backing and the coverage
report is unchanged.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The bundle limits and path validation that FR-005 (AC-5 and the path-safety criteria) requires moved from `publication`, tagged `// Implements: FR-005, NFR-001` in `lib.rs`, to the new `artifact` module, which carries only a prose comment. The code that enforces those criteria now has no owning-requirement tag, so a reverse-gap scan sees `artifact` as unowned. The same holds for `kani_identity` (the FR-015 harness and identity records, moved out of the FR-015-tagged `kani_obligations` and `state_frame`). Add `// Implements: FR-005` on `mod artifact` and `// Implements: FR-015` on `mod kani_identity` | src/lib.rs:6-7, src/lib.rs:21-22 |

## Coverage

- Rows: 210 backed of 306 at base and at head; criteria 271; unbacked 66 (unchanged, none
  attributable to this PR).
- Semantic review: skipped (not requested; a pure-motion PR has no new intent to judge).
- Plan completion: not assessed
