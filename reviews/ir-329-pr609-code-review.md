---
id: SR-609001
title: 'code-review of IR-329 CG PR #609 (spec-to-source alignment)'
type: SpecReview
analysis: code-review
scope: 'agent-ix/quire-contract-codegen; PR #609 IR-329; spec/spec.md (Kani ownership map and reverse inventory), spec/kani/functional/FR-015-bounded-kani-obligations.md (status prose); measured against src/kani/**, src/lib.rs, tests/it/**'
review_set: subset
relationships:
- target: ix://agent-ix/quire-contract-codegen/FR-015
  type: references
- target: ix://agent-ix/quire-contract-codegen/FR-040
  type: references
- target: ix://agent-ix/quire-contract-codegen/FR-024
  type: references
---

## Summary

Ticket: IR-329. Independent static review of spec-only CG PR #609 at frozen head
`450a380b235630d46a71c37ff39e9712a31a0ee2` (merge-base `6b834b872ab8f36aaddec5f475aa3204ae3fa692`).
The diff changes only `spec/spec.md` and FR-015 status prose; this review checks every source and
public-operation claim it makes against the tree. Tools: quoin 0.29.1, quire 0.36.2 (engine 0.50.2).
No Cargo, Kani or full gate was run (spec-only PR, per brief).

## Verdict

CHANGES REQUESTED — two medium findings. Forward ownership is complete: all 41 files under
`src/kani/` map to a row, and every `pub use kani::…` re-export in `src/lib.rs` falls under a row.
V1 retirement is real (`v1_bundle`, `generate_kani_bundle` absent from `src/` and `tests/`;
`src/kani/generate/clause.rs` is only `pub(super) mod v2;`). The V2 census claim is backed:
`negotiate.rs` and `clause/v2.rs` use `kani::census` typed requests, and
`tests/it/kani_obligations_state_frame.rs:442,519` tag FR-015-AC-44/45. The `HarnessSpec` claim is
accurate: scalar, the V2 precondition arm and collection call `render::render`; the V2
postcondition, frame and corpus keep their own templates. `census_validation.rs` is now used only by
the corpus (`validate_dependencies`) plus the `KaniDiagnostic`/`KaniErrorCode` re-export, as stated.
AC-99 (PARTIAL), AC-110 and AC-111 (PLANNED, untagged in the computed matrix) are stated honestly.
The two defects are a false "planned" status for a live requirement and a wrong owner for the #605
public operation.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | New kani/run row and reverse inventory call FR-040's cancellation producer planned, but it is live (IR-701, CG #383): KaniExecutionRequest.cancel, BatchCancelled, and all seven FR-040 ACs tagged by execute.rs batch tests | spec/spec.md:138, spec/spec.md:149, src/kani/run/execute.rs:58, src/kani/run/execute.rs:1693 |
| FND-002 | medium | kani/classify.rs row names FR-017/FR-016 as owner, but the #605 public parse_state_clause_falsification is owned by FR-024 (state-clause envelope intake, AD-004 step 5 L-6); FR-024 appears only on the replay row anchored to playback.rs | spec/spec.md:139, src/kani/classify.rs:104, spec/replay/functional/FR-024-counterexample-envelope-intake.md:550 |

## Coverage

Forward: `find src/kani -type f` (41 files) and every `kani::` re-export in `src/lib.rs` were
matched to a map row. Reverse: each Kani requirement (FR-015/017/025/028/029/030/034/040/041/046,
NFR-006) was checked against the computed matrix (`quoin matrix --json`) and source. FR-034 and
FR-046 have no tags and are honestly planned. FR-025-AC-1 to AC-8 are untagged although their ABI
code is live; that is a pre-existing trace gap, not introduced by this diff, and is not raised here.
No hash, digest or pin tracking is introduced.

## Examined Scope

```yaml
scope:
- {id: spec/spec.md#kani-ownership-map, path: spec/spec.md, role: examined, excerpt: "The current Kani module and public-surface ownership map, measured against the source tree and crate-root re-exports, is:"}
- {id: spec/spec.md#reverse-inventory, path: spec/spec.md, role: examined, excerpt: "FR-034/040, FR-041 and FR-046 retain planned obligations without a completed implementation claim here."}
- {id: FR-015, path: spec/kani/functional/FR-015-bounded-kani-obligations.md, role: examined, excerpt: "AD-004 step 4f has retired the V1 bundle and V1-only clause arm immediately (CG #601). The V2 clause census input is implemented for supported claims (AC-44/45), while full V2 clause proof controls remain open."}
- {id: FR-040, path: spec/kani/functional/FR-040-cancellation-producer.md, role: context_only, excerpt: "Verification: TC-055 is planned; no implementation or passing test is claimed here."}
- {id: FR-024, path: spec/replay/functional/FR-024-counterexample-envelope-intake.md, role: context_only, excerpt: "The remaining AD-004 step 5 L-6 boundary change adds kani/classify.rs::parse_state_clause_falsification as the production entry."}
```
