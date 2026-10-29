---
id: SR-623001
title: Code and Rust review of CG PR 623 frame renderer port
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; IR-1084; CG PR 623; src/kani/generate/{frame,render,spec}.rs; tests/it/{cover_last,layout}.rs"
review_set: subset
---

## Summary

Independent diff review at `081d9aad28de2b4d5b4e6d9e24dc35e32e58bbd4` against actual PR base `7dc4b0370d059eb013901aa93ce03edc7182c488`. Applied code-review and its Rust lane, repository CLAUDE.md and AGENTS.md, and rust-style API, implementation, safety, boundary, and testing guidance. No applicable AssuranceProfile was found under `spec/`.

## Scope

```yaml
scope:
  - {id: IR-1084-AC-1, path: src/kani/generate/frame.rs, role: examined, excerpt: "Port the supported frame obligation family in `kani/generate/frame.rs` to the shared `HarnessSpec` and `kani/generate/render.rs` contract. The frame path retains its typed admitted domain, role selection, identity, refusal/limit behavior and one public generator route; it does not add a forwarding or compatibility entry."}
  - {id: IR-1084-AC-2, path: src/kani/generate/render.rs, role: examined, excerpt: "The rendered frame proof uses `HarnessSpec` ordering and cover-last behavior. Non-test frame source no longer spells proof-module Kani syntax; after this slice only the still-open 4g corpus template may do so outside `render.rs` (subject to the exact L-3 source scan)."}
  - {id: IR-1084-AC-3, path: tests/it/cover_last.rs, role: examined, excerpt: "Tagged focused tests exercise frame generation, each supported frame role, deterministic harness identity, typed refusals and the emitted source/cover ordering. Inspect the L-3 literal scan and FR-015-AC-58 guard for the frame path. Preserve the existing frame replay and output semantics; any L-6 entry changes are separate work."}
  - {id: FR-015-AC-7, path: spec/kani/functional/FR-015-bounded-kani-obligations.md, role: examined, excerpt: "Every generated harness contains exactly one non-vacuity cover; a precondition harness covers that the precondition holds within the IR bounds, and a contract harness's cover stands after the contract call, so a run that satisfies every check without satisfying the cover is not a proof."}
  - {id: FR-015-AC-26, path: spec/kani/functional/FR-015-bounded-kani-obligations.md, role: examined, excerpt: "One `postcondition` `state_clause` of an admitted package, whose condition is one integer comparison of the pre and post reads of one field through its first parameter `self`, yields a separate operation-contract harness and frame-effect harness for the operation its anchor names."}
  - {id: FR-015-AC-58, path: spec/kani/functional/FR-015-bounded-kani-obligations.md, role: examined, excerpt: "A test generates a harness through every emitting entry point"}
```

## Analysis

`frame.rs` builds `FrameAssertion` and `FrameSpec` from the admitted role, then calls the private `HarnessSpec::new_frame` and shared renderer. The constructor checks paths, listed fields, domains, cover message and unwind. `render_frame` emits symbolic fields and bounds, calls the subject, emits role assertions, then one final cover; source-size and syntax refusals retain their order and typed mapping. The identity is assembled from the same spec's path, options, solver and unwind. No second public entry, compatibility path, cross-repo copy, unsafe operation, CI change or lowered gate is in the diff.

The seven-family guard now attributes both frame roles to `render.rs`; the emitted-source AST inspection still drives both roles through the public generation path. The new literal scan checks every non-test source string literal for all seven L-3 spellings outside renderer and the still-live corpus exception. The existing frame tests retain typed refusal, scope, deterministic bytes and source-order assertions. The PR's focused receipt records 3 frame unit tests, 6 cover-last tests, and 1 dual-role integration test passing; the two wider state-frame failures reproduce in the unchanged pre-render IR/QSL fixture on the #622 base. I did not run an aggregate gate or installed Kani lane in this review.

## Verdict

PASS for the code diff, subject to code-lead final-head gates and the separately recorded existing upstream fixture failure.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
