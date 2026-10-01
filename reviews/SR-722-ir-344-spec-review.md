---
id: "SR-722"
title: "CG PR 219 spec review: AD-004 step 2 and module-map amendments"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@a5aafb81ca65bc42a1be3a3c187665e99fa58ab4; spec/assurance/AD-004-cg-crate-layout.md"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AD-004
    type: reviews
---

# SR-722: CG PR 219 spec review (AD-004 amendments)

## Summary

Ticket: IR-344. PR: agent-ix/quire-contract-codegen#219 at a5aafb8. The PR amends AD-004 in
three places: two module-map rows (lines 227-228 and 240) and the step 2 text (lines 582-605).
Methods: spec-review (consistency, integrity, scope) of the amended text against the rest of
AD-004 and against the code at head, plus the remaining layering edges re-derived from the
`use crate::` lines and checked against the steps that own them.

Checked and found accurate:

- `Artifact` and the generation diagnostic types belong in `core`: the target tree already puts
  them in `core/artifact.rs` and `core/diagnostic.rs` (lines 159-166). `PublicationDiagnostic`
  holds `GenerationTerminalState` and `PublicationDestinationState` as fields, so moving the
  diagnostic to `core` without them would leave `core` importing `oracle` and `publication`,
  against "core depends on nothing" (line 269). The amendment is required and correct.
  `ProofDependencyState` is a field of `ProofDependencyEdge` and `ProofDependencyRequest`; adding
  it to the census row is correct.
- The flat names: `kani_identity` because `src/identity.rs` exists and becomes
  `core/identity.rs`; correct and recorded in step 2.
- The removed edges, re-derived at base and head: `bound`, `bound_coverage` -> `publication`
  gone; `publication` -> `oracle` gone; `vacuity`, `bound_coverage` -> `oracle` (source map)
  gone; `kani_execution` -> `kani_obligations`, `state_frame`, `oracle` gone (non-test and test);
  `kani_witness_join`, `spine_replay` -> `kani_obligations` gone; `kani_transcript` test now
  imports `kani_identity`. New edges: `artifact` -> `diagnostic` (core to core),
  `kani_identity` -> `artifact`, `identity`, `kani` (abi types only), `kani` -> `kani_census`
  (used only by the V1 bundle). None violates the direction rules. No `evidence` -> `strategy`
  edge exists.
- Remaining violating edges and their owners: root-path imports everywhere (2c-2g, L-2);
  `kani_transcript` test -> `kani_execution` (2f, "the test back-edges"); `kani` ->
  `kani_obligations` for `MAX_OBLIGATION_UNWIND`, used only by the V1 `validate_request` (4f);
  `bounded_kani_corpus` -> `kani::deterministic_json` (1a). One edge has no owner (FND-003).
- interface-001 names none of the moved module paths; the public API is the crate-root
  re-export list, unchanged.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The target tree still places the destination state in publication: `publish.rs  write_bundle_atomic, destination state, published identity`, and `core/artifact.rs` lists no destination state. The amended map row (line 227) and step 2a now put `PublicationDestinationState` in `core/artifact.rs`, and the code has it there. The AD now contradicts itself; amend line 209 (and add the state to lines 159-160) in this PR | spec/assurance/AD-004-cg-crate-layout.md:209, spec/assurance/AD-004-cg-crate-layout.md:159-160 |
| FND-002 | low | Lines 99 and 294-295 still describe "generator imports at `kani_execution.rs:767`" that move to `tests/it/`. At base the test module imported only identity types from `state_frame` (`StateComparison`, `StateFrameIdentity`, `StateFrameProperty`, `StateFrameScope`, at `:908` and `:947`), never a generator, and after 2b it imports them from `kani_identity`. The rule at 294-295 tells step 2f to move an edge that does not exist. Drop the `:767` clause from the rule (the `kani_transcript.rs:261` half stays) | spec/assurance/AD-004-cg-crate-layout.md:294-295, spec/assurance/AD-004-cg-crate-layout.md:99 |
| FND-003 | medium | No step owns `bounded_kani_corpus`'s call to `kani::validate_dependencies` (`bounded_kani_corpus.rs:430`). Step 4f deletes "bundle validation" with the V1 bundle, and the census row (line 240) does not list `validate_dependencies`, but the corpus keeps calling it until 4g, and 4f does not wait for 4g. As written, 4f breaks the corpus build or silently keeps V1 code. Record where it goes: move it to `kani/census.rs` (it validates the census request) or state in 4f that it survives until 4g | spec/assurance/AD-004-cg-crate-layout.md:629-635, spec/assurance/AD-004-cg-crate-layout.md:240 |

## Verdict

The amendments are accurate and minimal for what they change, and needed: without them `core`
would import `oracle` or `publication`. FND-001 is an inconsistency the amendment left in the
target tree; FND-002 a stale rule; FND-003 an AD gap the coder reported in the PR body but did
not record in the AD. None blocks the code. FND-001 and FND-002 are one-line edits for this PR;
FND-003 can be recorded here or ticketed against step 4f.
