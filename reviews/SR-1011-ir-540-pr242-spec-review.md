---
id: "SR-1011"
title: "CG PR 242 spec review: the FR-021-AC-22 amendment and status flip"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-codegen@19ceb7921981554fc0a5814aa9bba6743236c5fe; spec/oracle/functional/FR-021-function-application-oracles.md, spec/oracle/matrix/TC-031-function-application-oracles.md, spec/oracle/matrix/tests.md; diff origin/main...HEAD, base c2099b8"
---

# SR-1011: CG PR 242 spec review: the FR-021-AC-22 amendment and status flip

## Summary

Ticket: IR-540. PR: agent-ix/quire-contract-codegen#242 at 19ceb79.

The PR changes the spec in three ways. It adds one clause to the FR-021 duplicate-node Behavior bullet and to AC-22: the refusal carries the smallest node id when a name is held by several duplicate groups. It removes the Planned markers. It rewrites TC-031 step 9 to name the four tests.

The added clause fills a real gap: it fixes which `node_id` the refusal carries, which merged #241 left unstated. It agrees with the code, which keeps the first node id in node-id order. The clause is consistent across FR-021, AC-22 and TC-031. The test gap for it is recorded in SR-1010 FND-001.

Sub-analyses: base only. No new requirement and no new relationship was added, and the edit is too small to need integrity, EARS or criterion-strength passes.

## Verdict

The spec edits are consistent and truthful. Two low findings: one AC clause names a refusal reason that no output can show, and two wording nits. Neither blocks the merge.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The AC-22 clause 'third declaration is refused as AmbiguousFunctionName' cannot be observed: the reason reaches no output, and a BodyMismatch mutant passes; only absence is testable | spec/oracle/functional/FR-021-function-application-oracles.md:262, spec/oracle/matrix/TC-031-function-application-oracles.md:105-107 |
| FND-002 | low | TC-031 step 9 starts with a leftover '9. (FR-021-AC-22):'; the new FR-021 Behavior sentence's 'the name' has no clear referent (the item's name) | spec/oracle/matrix/TC-031-function-application-oracles.md:103, spec/oracle/functional/FR-021-function-application-oracles.md:219-220 |

## Dispositions

Round 1, reviewed at fc8dae48e71932948465561c74d462d6f840bda1 (fix-round delta 19ceb79..fc8dae4, one commit). Measured in a detached
throwaway worktree:

- `cargo test --test it exact_function`: 28 of 28 pass. Clippy `--all-targets -D warnings`, fmt-check, `quire validate`
  (exit 0) and `quire coverage --strict` (66 unbacked, unchanged) are clean. Cargo.lock and Cargo.toml are unchanged.
- The delta adds no `unwrap(`, `expect(`, `unreachable!`, `panic!`, `todo!` or `unimplemented!` to `src/` (FR-021-AC-21).
  The new `panic!` is in test code (fixture v), which is fine.
- Mutants: M3 (largest node id wins) is killed by `tc_031_ac22_fixture_v`. M8 (DuplicateRequest before the node refusal),
  M9 (`ItemKey.refused_name` always None) and M10 (drop the stage-3 precedence branch) are each killed by
  `tc_031_ac22_fixture_vi`. M7 no longer applies: AC-22 and TC-031 now state only the observable outcome for the third
  declaration.
- `ItemKey.refused_name` is `None` for every request without a duplicate node id. It is the last field, so it only
  breaks ties in the derived order. A probe dumped every artifact (lib.rs, claim map, location map, Cargo.toml) for
  four requests without duplicate node ids under three sources: fc8dae4, 19ceb79 and base c2099b8. The requests were
  the plain corpus with a duplicate request and an unknown name, an ambiguous-name request, and the two open-point
  shapes below. The output was byte-identical across all three (27586 bytes).
- The author's open point (author's statement, measured here):
  - (a) Two different ambiguous names on distinct node ids, with items on one call node, do NOT collapse. They give
    two `AmbiguousFunctionName` claims, because the highest-sorted node id per name is injective when no node id is
    shared.
  - (b) What does collapse is two different unknown names on one call node. `function_node_id` is `None` for both,
    so they become one `DuplicateRequest` claim. Base c2099b8 produces the same bytes, so this is pre-existing,
    belongs to FR-021-AC-1/AC-13 rather than AC-22, and is out of scope for this PR. It is not a finding here; the
    lead may ticket it separately.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | fc8dae4 |
| FND-002 | fixed | fc8dae4 |
