---
id: SR-623002
title: Gap analysis of CG PR 623 frame renderer port
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; IR-1084; CG PR 623; FR-015-AC-7, AC-26, AC-27, AC-28, AC-58; AD-004 L-3"
review_set: subset
---

## Summary

Planless diff-scoped acceptance and computed-matrix review of the frame renderer port at `081d9aad28de2b4d5b4e6d9e24dc35e32e58bbd4`. Plan completion: not assessed. No applicable AssuranceProfile was found under `spec/`.

## Scope

```yaml
scope:
  - {id: IR-1084-AC-1, path: src/kani/generate/frame.rs, role: examined, excerpt: "Port the supported frame obligation family in `kani/generate/frame.rs` to the shared `HarnessSpec` and `kani/generate/render.rs` contract. The frame path retains its typed admitted domain, role selection, identity, refusal/limit behavior and one public generator route; it does not add a forwarding or compatibility entry."}
  - {id: IR-1084-AC-2, path: tests/it/cover_last.rs, role: examined, excerpt: "The rendered frame proof uses `HarnessSpec` ordering and cover-last behavior. Non-test frame source no longer spells proof-module Kani syntax; after this slice only the still-open 4g corpus template may do so outside `render.rs` (subject to the exact L-3 source scan)."}
  - {id: IR-1084-AC-3, path: tests/it/kani_obligations_state_frame.rs, role: examined, excerpt: "Tagged focused tests exercise frame generation, each supported frame role, deterministic harness identity, typed refusals and the emitted source/cover ordering. Inspect the L-3 literal scan and FR-015-AC-58 guard for the frame path. Preserve the existing frame replay and output semantics; any L-6 entry changes are separate work."}
  - {id: FR-015-AC-7, path: spec/kani/functional/FR-015-bounded-kani-obligations.md, role: examined, excerpt: "Every generated harness contains exactly one non-vacuity cover"}
  - {id: FR-015-AC-26, path: spec/kani/functional/FR-015-bounded-kani-obligations.md, role: examined, excerpt: "Each has exactly one non-vacuity cover, and its identity is scoped to the operation, anchor, frame and framed object"}
  - {id: FR-015-AC-27, path: spec/kani/functional/FR-015-bounded-kani-obligations.md, role: context_only, excerpt: "The operation-contract harness assumes each ranged state field within the inclusive `i64` endpoints returned by `model_object_fields`"}
  - {id: FR-015-AC-28, path: spec/kani/functional/FR-015-bounded-kani-obligations.md, role: examined, excerpt: "The frame-effect harness takes the granted fields from the frame node's `modifies` entries and asserts every other state field the caller names unchanged."}
  - {id: FR-015-AC-58, path: spec/kani/functional/FR-015-bounded-kani-obligations.md, role: examined, excerpt: "A test generates a harness through every emitting entry point"}
```

## Analysis

The computed Quire matrix reports FR-015-AC-7, AC-26, AC-27, AC-28 and AC-58 as tagged. I checked their concrete binders in `tests/it/kani_obligations_state_frame.rs` and `tests/it/cover_last.rs`, including the dual-role emitted sources, identity and bound assertions, AST cover-last inspection and literal scan. The new source paths have the owning FR-015 criterion and IR-1084 ticket. No new test stub, ignored test, untagged changed criterion, reverse ownership gap or hollow assertion was found in this diff. The two wider fixture failures are pre-render IR admission failures and reproduce at the unchanged #622 base; they limit end-to-end evidence but are not caused by this diff. No plan was supplied and no repository-wide completion claim is made.

## Verdict

PASS for diff-scoped trace and evidence structure. Full final-head gate and installed Kani evidence remain for the code lead. Plan completion: not assessed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Plan completion: not assessed. Computed matrix statuses for the five examined FR-015 criteria are tagged. Existing IR-624 planned installed-Kani and model-accessor obligations are outside this PR.
