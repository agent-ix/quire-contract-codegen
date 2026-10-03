---
id: "SR-1009"
title: "CG PR 242 code review (Rust lane): duplicate declaring node ids in function oracle generation"
type: SpecReview
analysis: code-review
review_set: subset
scope: "agent-ix/quire-contract-codegen@19ceb7921981554fc0a5814aa9bba6743236c5fe; src/oracle/function/mod.rs, tests/it/exact_function_generation.rs; diff origin/main...HEAD, base c2099b8"
---

# SR-1009: CG PR 242 code review (Rust lane): duplicate declaring node ids in function oracle generation

## Summary

Ticket: IR-540. PR: agent-ix/quire-contract-codegen#242 at 19ceb79, base main c2099b8. Diff `git diff origin/main...HEAD` only. Methods: code-review with the rust-review lane folded in.

What was checked, and measured:

- `ExactFunctionRefusal::DuplicateDeclaringNode { node_id }`: struct variant like its siblings, so the enum's `#[serde(tag = "code", rename_all = "snake_case")]` gives `{"code":"duplicate_declaring_node","node_id":...}`. Doc comment present and accurate.
- `FunctionNames` (private): built once from node-sorted declarations. Stage 1 checks the node id first, then the name. `resolve` checks the duplicate node first, then ambiguity, then absence. `duplicate_node_by_name` keeps the first node id in node order, which is the smallest.
- Nested `call` to a refused duplicate: `unique_node_id` excludes shared-node names, so the call gets `UnknownCallee`.
- The third declaration (same name, its own node id) is refused `AmbiguousFunctionName` and kept out of `checked_package()` and the location map.
- Permutation independence: the stable sort leaves a pair's relative order up to the request. Both members are refused and every table built from them is order-free, so the output does not depend on it. The tests assert byte equality across all permutations.
- Lowering alignment: `CheckedPackageV2::lower` (IR 968ba9b) returns one record per requested id with no dedup, so `zip(&lowering.records)` stays aligned when node ids repeat.
- FR-021-AC-21: the added lines contain no `unwrap(`, `expect(`, `unreachable!`, `panic!`, `todo!` or `unimplemented!`. The new code uses `.unwrap_or(0)` and `ok_or_else`.
- Comments: the module doc ("Package assembly") and the `own_shape`/`bodies` comment no longer claim that node-id keying alone prevents collapse. They now say it is sound only because shared-node declarations are refused first. That is true.
- `#[allow(clippy::too_many_arguments)]` is removed. `item_disposition` now takes `&FunctionNames`, and clippy `--all-targets -D warnings` is clean (run here).
- Public API: one new variant on a public enum that is not `#[non_exhaustive]`. This is a breaking change for any exhaustive matcher. In quire-driver, origin/main fc53a75 and the driver-ir-478 worktree have zero references to `ExactFunctionRefusal`. The driver imports only `EnvelopeRefusal`, `KaniObligationError`, `OracleGenerationError` and `RoutedGenerationError`. The driver will not break on its next CG bump. No schema, snapshot or layout test names refusal codes.
- Kani: no Kani harness or kani-lane module calls `generate_exact_function_oracles`. The change is not Kani-visible, so `make kani` is not needed.

## Verdict

Approve with one low finding. The change is correct, panic-free, and keeps request order out of the output. The comments are now true, and nothing downstream breaks. FND-001 is a corner the AC's literal wording covers: a malformed request with two items on one call node. Fix it or record the precedence in FR-021; it does not block the merge.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Two items on one call node naming different members of a duplicate-node pair collapse into one DuplicateRequest claim (ItemKey keys by the shared node id), not DuplicateDeclaringNode per item (measured) | src/oracle/function/mod.rs:1074, src/oracle/function/mod.rs:1091, src/oracle/function/mod.rs:1355-1366 |

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
