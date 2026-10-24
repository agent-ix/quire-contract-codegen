---
id: SR-16529
title: "IR-275 CG558 incremental acceptance and gap review"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; PR 558 head f79bfe5a5c3d572e44d3212ee669c1431e2c9ec6; incremental IR-275 AC-2 changed-code scope"
review_set: subset
---

## Summary

The changed CG V2 consumer paths use `CheckedTerm` for admitted term shape and retain JSON only for operation member metadata that the IR API still exposes as JSON. A production source sweep found no remaining `node.body.get` term read in CG.

## Verdict

PASS — no incremental implementation or trace gap in PR #558. The IR-only protocol slice merged separately; this review does not itself close the cross-repository ticket.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Compared the PR's ten changed files with IR-275 AC-2 and the recorded wire-carrier scope correction. Searched non-test CG production for `body.get`, `body.pointer`, and term-key string access, then inspected remaining operation metadata lookups and the one test-only raw lookup in `src/kani/generate/negotiate.rs`. Existing traced collection, frame and composite tests plus the author's focused routed and integration test results back the affected paths; the PR adds one typed helper unit adaptation. No Test Matrix was authored. Optional repository-wide semantic analysis was not requested.

Plan completion: not assessed

## Examined scope and bindings

```yaml
scope:
  - id: IR-275-AC-2
    path: "src/core/ir/clause.rs; src/kani/generate/collection.rs; src/kani/generate/frame.rs; src/kani/generate/negotiate.rs; src/oracle/mod.rs; src/replay/composite_builder.rs; src/routed/capability.rs"
    role: examined
    excerpt: "No non-test code in IR or CG reads a term through Value::get on a string key; public wire body remains Value by the recorded scope correction."
  - id: FR-040-AC-18
    path: "src/kani/generate/frame.rs"
    role: examined
    excerpt: "The checked frame body and its field modifications are interpreted from admitted typed structure."
  - id: FR-018-AC-35
    path: "src/replay/composite_builder.rs"
    role: examined
    excerpt: "Composite replay checks the selected equality operator identity from the admitted node."
bindings:
  - test_id: tc_035_original_composite
    ac_id: FR-018-AC-35
    trace: correct
```
