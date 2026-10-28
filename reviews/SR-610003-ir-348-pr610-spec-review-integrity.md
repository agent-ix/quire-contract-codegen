---
id: SR-610003
title: 'spec-review integrity analysis of IR-348 CG PR #610 (AD-004 reconciliation)'
type: SpecReview
analysis: integrity
scope: 'agent-ix/quire-contract-codegen; PR #610 IR-348; exact b312b2d19187fda5ad2c7fcbab0b84af163f8c38 (merge-base 6b834b872ab8f36aaddec5f475aa3204ae3fa692); spec/assurance/AD-004-cg-crate-layout.md; measured against origin/main src/kani/**, src/replay/**, src/routed/**, tests/it/layout.rs'
review_set: subset
relationships:
- target: ix://agent-ix/quire-contract-codegen/AD-004
  type: references
---

## Summary

Ticket: IR-348. Independent integrity and consistency review of spec-only CG PR #610 at frozen head
`b312b2d19187fda5ad2c7fcbab0b84af163f8c38`. The PR edits only AD-004. It reconciles AD-004 with
merged PR #588 (collection `HarnessSpec`, step 4h), PR #601 (V1 Kani deletion, step 4f) and PR #605
(typed state-clause `Playback`). This review checks every touched current-state row and the
remaining-truth statements the PR asserts against `origin/main` source, read through `git show` and
`git grep`. `quire validate --scope .` passes on AD-004 (exit 0), and `make spec` passes (exit 0).
The only warnings are pre-existing EARS warnings in FR-017 and FR-014, outside this diff. No Cargo,
Kani or full gate was run (spec-only PR, per brief). Tools: quoin 0.29.1, quire 0.36.2 (engine 0.50.2).

## Verdict

CHANGES REQUESTED: two medium and three low findings. The rows the PR touched for the three merged
slices are accurate against source:

- `src/kani/generate/clause.rs` is only `pub(super) mod v2;`. There is no `precondition.rs`,
  `contract.rs` or `v1_bundle.rs`. `generate_kani_bundle` and `V1BoundClause` are absent from `src/`
  and `tests/`.
- `BoundPackage` and `BoundClause` are named only in `evidence/bound_coverage.rs`,
  `oracle/bound_v1.rs` and `strategy/bound/`, so the L-12 interim rewrite is true.
- `collection.rs` and `clause/v2.rs` build `HarnessSpec` and call `render`. Non-test Kani spellings
  outside `render.rs` remain only in `frame.rs` and `corpus/bounded_kani_corpus.rs`. The scalar and
  spec hits are `#[cfg(test)]`. So the L-3 residual set (frame and corpus) is correct.
- `kani/classify.rs::parse_state_clause_falsification` exists with `NotFalsified`.
  `StateClauseReplayInputs::decode_falsification` takes `&Playback<'_>`. Transcript text survives
  only in `replay/function.rs` and `replay/frame.rs`, both of which call `parse_assertion_playback`.
  So the L-6 residual, the witness-decode row and the interim-table removal are correct.
- `frame.rs` imports only `outcome` from `generate/`.

No acceptance criterion or layout rule is narrowed:

- L-5 is retained as open.
- The L-12 end-state stays absolute.
- L-3 gains the frame residual rather than losing one.
- The 4g collection slice gains a control condition rather than losing one.
- The CG-side-control wording now forbids copied fixtures.

The defects are remaining-truth gaps in the sections the PR reconciles.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The PR's new intro says the remaining exceptions are listed under Interim exceptions, but that table (and the target tree and module map) still carries `deterministic_json` in `census_validation.rs` and an `artifact` helper "by step 1a". Neither has any definition on origin/main: both were deleted by #356 (IR-746). | spec/assurance/AD-004-cg-crate-layout.md:1497, spec/assurance/AD-004-cg-crate-layout.md:1498, spec/assurance/AD-004-cg-crate-layout.md:239-242, spec/assurance/AD-004-cg-crate-layout.md:315, spec/assurance/AD-004-cg-crate-layout.md:980, src/kani/generate/census_validation.rs:1 |
| FND-002 | medium | The PR assigns the frame renderer port, now an L-3 blocker, to "step 4d" in the target tree and the new interim row. But step 4d's own text (unchanged) defines 4d as the IR-1044 V2 census input and says "The frame family is ported separately". The frame port has no owning step or ticket. | spec/assurance/AD-004-cg-crate-layout.md:1499, spec/assurance/AD-004-cg-crate-layout.md:225-226, spec/assurance/AD-004-cg-crate-layout.md:1101-1107, spec/assurance/AD-004-cg-crate-layout.md:1502 |
| FND-003 | low | The rewritten dependency rule says `negotiate` imports `record`. On origin/main `negotiate.rs` imports `clause::v2`, `collection`, `frame`, `outcome` and `scalar` from `generate/`, and no `record`. | spec/assurance/AD-004-cg-crate-layout.md:438, src/kani/generate/negotiate.rs:80-114 |
| FND-004 | low | Present-tense V1 consumer statements survive the reconciliation, although 4f has deleted the entry: "currently passes through `generate_kani_bundle`", "which call `generate_kani_bundle` today", "QSL's exemplars currently call `generate_kani_bundle`", and "this IR-1044 PR does not retire it". | spec/assurance/AD-004-cg-crate-layout.md:734-735, spec/assurance/AD-004-cg-crate-layout.md:1120, spec/assurance/AD-004-cg-crate-layout.md:1703, spec/assurance/AD-004-cg-crate-layout.md:1737 |
| FND-005 | low | Step 4b still says "L-3 lands after 4f, 4g and 4h". That contradicts the updated L-3 and step-4g text, under which L-3 also waits on the frame renderer port. | spec/assurance/AD-004-cg-crate-layout.md:1093, spec/assurance/AD-004-cg-crate-layout.md:937-941, spec/assurance/AD-004-cg-crate-layout.md:1192-1193 |

## Examined Scope

```yaml
scope:
- {id: AD-004#views-intro, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "At the current source state, the collection family uses HarnessSpec and render.rs (step 4h, PR #588); the V1 Kani bundle, clause arm and routed form are deleted (step 4f, PR #601); and state-clause falsification is parsed at the classifier boundary and decoded from typed Playback (step 5 state-clause slice, PR #605). The remaining exceptions are listed under Interim exceptions"}
- {id: AD-004#target-tree-generate, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "scalar.rs  frame.rs  scalar renders through HarnessSpec; frame retains its own template until its step-4d renderer port. The former V1 precondition.rs and contract.rs were deleted by 4f; V2 clauses live in clause/v2.rs"}
- {id: AD-004#module-map-v1-bundle, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "Retired at step 4f. Step 2f moved them to the interim file v1_bundle.rs, which 4f deleted whole."}
- {id: AD-004#dependency-direction-generate, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "then negotiate, which imports outcome, record, scalar, collection, clause and frame ... frame imports outcome only."}
- {id: AD-004#bound-package-rule, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "BoundPackage and BoundClause remain in the V1 oracle, strategy and evidence readers. Step 4f deleted the V1 Kani arm and routed form; neither type is named in kani/, routed/ or replay/."}
- {id: AD-004#one-generator-families, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "The scalar, V2 clause and collection families return HarnessSpecs. ... The frame family retains its own template until its renderer port; collection was ported at step 4h."}
- {id: AD-004#render-contract-1, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "The frame and corpus still spell their own templates, so L-3 ... remains open until frame rendering is ported and 4g retires the corpus."}
- {id: AD-004#witness-decode-row, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "state-clause decoding also takes typed Playback, while public replay entries in function and frame retain transcript text (L-6 residual below)"}
- {id: AD-004#one-input-model, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "Kani side: step 4f deleted the deprecated V1 arm and bundle. The V2 contract-family input remains partly open"}
- {id: AD-004#L-3, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "Steps 4f and 4h removed the V1 and collection exceptions. The frame template in kani/generate/frame.rs remains until its renderer is ported, and the corpus template in kani/generate/corpus/bounded_kani_corpus.rs remains until 4g. L-3 remains open for both."}
- {id: AD-004#L-5, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "a distinct CG-side control only if the facade allows it without copying fixtures. ... L-5 is retained as an open obligation, not a deletion gate."}
- {id: AD-004#L-6, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "replay/function.rs and replay/frame.rs still accept transcript text and call the output parser before decoding. The state-clause L-6 slice makes its decode entry take parsed Playback from kani/output/playback.rs"}
- {id: AD-004#L-12, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "were the V1 arm in kani/generate/{outcome,negotiate,clause}.rs and the V1 clause form and preflight in routed/capability.rs. Step 4f removed these paths and their V1-only tests; the L-12 end-state remains absolute."}
- {id: AD-004#step-4g-collection, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "The collection slice can proceed after completed step 4h, and deletes the Collection arm and bounded_collections.rs only when its own symbolic replacement and control obligations pass."}
- {id: AD-004#step-4h, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "4h (landed in PR #588): the collection family."}
- {id: AD-004#D-2, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "D-2. kani.rs (historical step-2f allocation). ... Step 2f placed it in kani/generate/v1_bundle.rs, a file that step 4f deleted whole."}
- {id: AD-004#interim-exceptions, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "validate_dependencies, the V1 error type it returns and deterministic_json | kani/generate/census_validation.rs | 4g or the V2 census input for the first two; step 1a for deterministic_json"}
- {id: AD-004#planner-decisions, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "Public entry and external consumers | Step 4f deleted generate_kani_bundle. Step 4e supplies the supported V2 public entry"}
- {id: AD-004#step-4d, path: spec/assurance/AD-004-cg-crate-layout.md, role: context_only, excerpt: "4d: the V2 census input (FR-015-AC-22/25/44/45) is implemented by IR-1044 ... The frame family is ported separately, with generate_state_frame_obligations becoming an ObligationItem arm."}
```

## Disposition (round 2, exact head e9ace48ebd8317a9532b84d8ff6521fbb68ce9b7)

Reviewer re-check of FND-001 to FND-005 at exact head `e9ace48ebd8317a9532b84d8ff6521fbb68ce9b7`
(on `origin/main` `66a818b863a9b984fc646da4a0779ae9ba3d63f5`), against fresh source and against
the Linear child ticket IR-1084. Posted to IR-348 as comment `977dbc8d-0da4-4179-81a9-1958291563d8`,
a reply under `b4f81b8b-9c90-4b3d-9942-441651f302dc`. The readback matched byte-for-byte apart from
the single trailing newline that Linear strips. Verdict: **CHANGES REQUESTED, four fixed and one open.**

| ID | Disposition | Evidence |
| --- | --- | --- |
| FND-001 | open (partial) | Fixed: interim census_validation row (1495), target tree (239-241), module-map row 314, D-2 (1397-1399), item-map row 1516 and the step-4f text (1133). Still stale: interim row 1496 (`record.rs`, "not scheduled; `artifact` by step 1a"). D-3 (1426-1430) also still places `artifact` in `record.rs` as a current item. On origin/main `record.rs` defines only `harness_path` and `record`, and `src/` has no `fn artifact(`. |
| FND-002 | fixed | Lines 226, 620, 939, 1191 and 1497 name IR-1084 as the frame renderer port, separate from 4d's V2 census. IR-1084 exists (Backlog), has a structured blocked-by link to IR-1079, and records that its L-6 links follow once those tickets exist. |
| FND-003 | fixed | Line 437 lists `outcome`, `scalar`, `collection`, `clause` and `frame`, which matches the `negotiate.rs` use block on origin/main. |
| FND-004 | fixed | Lines 733, 1119, 1701 and 1735 are past tense. Dropping "Independent acceptance is pending" is accurate because IR-1044 is Done, and the remaining V2 obligations stay open. |
| FND-005 | fixed | Step 4b (1091-1092) waits on IR-1084 and 4g. Non-blocking nit: "the frame IR-1084 frame renderer port" repeats "frame". |

No regression and no AC narrowing. `git diff --name-only origin/main...e9ace48e` lists only AD-004
and the three committed SR artifacts. L-3 and L-5 stay open, and the L-12 end-state is unchanged.
`make spec` exits 0. `quire validate --scope .` on AD-004 and the SR files exits 0 (quire 0.36.2,
engine 0.50.2); its only warnings are the pre-existing FR-017/FR-014 EARS warnings. No Cargo, Kani
or full gate was run, because this is a spec-only PR.
