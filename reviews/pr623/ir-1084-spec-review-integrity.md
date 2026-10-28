---
id: SR-623004
title: Integrity review of CG PR 623 AD-004 step 4i
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen; IR-1084; CG PR 623; spec/assurance/AD-004-cg-crate-layout.md H-3 and R-1"
review_set: subset
---

## Summary

The step 4i amendment correctly describes the new frame path in several places, but leaves two earlier universal design claims in AD-004 inconsistent with the added `HarnessShape::Frame` and renderer. This review compares the changed architecture text with its own H-3/R-1 decisions and the code at `081d9aad28de2b4d5b4e6d9e24dc35e32e58bbd4`.

## Scope

```yaml
scope:
  - {id: AD-004-H-3, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "Invariants, enforced by the family-specific `HarnessSpec` constructors. A refusal is a typed `HarnessSpecError`; none is a panic."}
  - {id: AD-004-R-1, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "It holds no family-specific template text. It has one emitter per `Statement` variant, per `ProofKind` and per `Init`; a family's meaning reaches it only as data (`label`, `ExprText`, `message`)."}
  - {id: AD-004-L-3, path: spec/assurance/AD-004-cg-crate-layout.md, role: context_only, excerpt: "Step 4i removed the frame template from `kani/generate/frame.rs`; the corpus template in `kani/generate/corpus/bounded_kani_corpus.rs` remains until 4g."}
```

## Verdict

CONDITIONAL. Two medium documentation inconsistencies should be fixed in this PR without narrowing any FR-015 criterion.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | H-3 now says family-specific constructors enforce all listed invariants, but items 2-4 describe statement-only `Call`/`Input` structures, and item 6 says every message follows H-2 while frame assertion messages are escaped rather than refused; the 4d future-tense cover exception is also stale. State which rules apply to each shape without weakening the frame cover requirement | spec/assurance/AD-004-cg-crate-layout.md:583-607; src/kani/generate/spec.rs:370-419 |
| FND-002 | medium | R-1 says render.rs holds no family-specific template and emits only `Statement`, `ProofKind` and `Init` variants, while step 4i adds `render_frame` with a frame-only proof template; the ported-family paragraph also says the only supplied text is `ExprText` although `FrameSpec` carries header text and an operator. Describe the frame shape as an explicit renderer case while retaining the single spelling-site rule | spec/assurance/AD-004-cg-crate-layout.md:488-494,625-632; src/kani/generate/render.rs:86-151 |

## Analysis

The first conflict is introduced by changing H-3's heading from the singular `HarnessSpec::new` to plural family-specific constructors without qualifying the statement-only bullets. `new_frame` accepts frame assertion messages containing braces because `render_frame` escapes them with Rust's debug formatting; the statement constructor's H-2 message restriction does not describe that behavior. The second is a direct conflict between unchanged R-1 point 3 and the new `render_frame` arm; it is not a request to move Kani syntax back to `frame.rs`. The repair is an accurate architecture account of the reviewed implementation. No source or fixture copy, new version pin, or acceptance-criterion change is needed.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | At `bafb278796927f3a5ed69f45f9308b79d7de7d68`, H-3 explicitly assigns items 2-4 and the statement part of item 6 to `HarnessShape::Statements`, documents how collection and frame each hold one cover, and specifies frame assertion-message escaping and the literal-true frame cover. The stale 4d future tense is gone. |
| FND-002 | fixed | The same fix identifies the three explicit renderer shapes, states that `render_frame` owns its frame-only template, and updates the ported-family paragraph to list the frame's provenance header, validated paths, fields, ranges, operator and messages as data. The single Kani spelling and cover-placement site remains `render.rs`. |

## Disposition evidence

Inspected only `081d9aad28de2b4d5b4e6d9e24dc35e32e58bbd4..bafb278796927f3a5ed69f45f9308b79d7de7d68` in a detached review worktree. The diff changes AD-004 and commits the four prior review artifacts unchanged; no code or tests changed. No aggregate build or test gate was run in this disposition pass.
