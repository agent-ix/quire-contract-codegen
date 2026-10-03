---
id: "SR-1010"
title: "CG PR 242 gap analysis: FR-021-AC-22 against its tests and code"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@19ceb7921981554fc0a5814aa9bba6743236c5fe; spec/oracle/functional/FR-021-function-application-oracles.md, spec/oracle/matrix/TC-031-function-application-oracles.md, spec/oracle/matrix/tests.md, src/oracle/function/mod.rs, tests/it/exact_function_generation.rs; diff origin/main...HEAD, base c2099b8"
---

# SR-1010: CG PR 242 gap analysis: FR-021-AC-22 against its tests and code

## Summary

Ticket: IR-540. PR: agent-ix/quire-contract-codegen#242 at 19ceb79. Plan completion: not assessed.

AC-22 to tests. The four `tc_031_ac22_*` tests trace to FR-021-AC-22. They cover fixtures (i) to (iv) under every permutation of the declaration order (up to 24 orders) and assert each of these:

- byte-equal `ExactFunctionOracles` across the orders;
- the pair's names are absent from `src/lib.rs`, `location-map.json` and `location_map`;
- each pair item is `DuplicateDeclaringNode { node_id == FN_ADD }`;
- no generated claim names a pair member;
- oracle symbols and origins are unique;
- each distinct item is Generated and equal to the claim the request gives with the duplicates removed;
- fixture (iv) gives `UnknownCallee { callee: "pair_one" }`.

Focused run: `cargo test --test it exact_function` passed 26 of 26 (target dir in a throwaway worktree).

Mutation probes, run in a detached throwaway worktree and reverted after each:

| Mutant | Result |
| --- | --- |
| M1: drop the Stage 1 node-id refusal | killed (3 tests) |
| M2: ambiguity before the duplicate node for a shared name | killed (1) |
| M3: largest duplicate node id wins | **survived** |
| M4: shared-node names enter `unique_node_id` | survived; equivalent, because the callee then resolves to an `Err` and still gets `UnknownCallee` |
| M5: items are never `DuplicateDeclaringNode` | killed (4) |
| M6: no declaration-level name-ambiguity refusal | killed (2) |
| M7: third declaration refused `BodyMismatch` instead of `AmbiguousFunctionName` | survived; the reason cannot be observed (see SR-1011) |

The author's claim that the tests fail on the old code: on base `c2099b8`, the new tests do not compile, because the variant does not exist. That is true but trivial. The behavioural evidence is M1 and M5: removing either half of the new check fails the tests.

Removed test: `tc_031_ac12_two_declarations_on_one_node_id_each_render_their_own_body` asserted that both same-node declarations render. AC-22 now forbids that, so removing it is correct. AC-12 keeps three tests: `tc_031_ac12_dangling_callee_refuses_only_its_own_items`, `tc_031_ac12_form_mismatch_refuses_only_its_own_item`, and the signature/body-kind test at line 331. The removed test's other claim, that generation does not panic on duplicate node ids, is now exercised by every AC-22 fixture. Nothing is lost.

Matrix. AC-22 lost its Planned marker in FR-021, in TC-031 (intro, step 9 and expected results) and in tests.md, and tests.md now reads 'AC-19 through AC-22: Covered'. The TC-031 inventory row now lists only AC-15 and AC-18 as Planned. The stale SR-880 and AC-13 caveat is gone. `quire validate --scope . 'spec/**/*.md' 'plan/**/*.md' 'reviews/**/*.md'` exits 0 with warnings only. `quire coverage --scope . --strict` reports 66 unbacked rows and 0 contradicted, the same 66 as base c2099b8. The counts are consistent.

Gate adequacy. The author's `make spec` failed only because the Makefile's hardcoded quire path is missing. The quire commands above were run here from PATH instead. The author also ran lint, test, deny, audit-unsafe and rustdoc separately. I additionally ran:

- `cargo fmt --all -- --check`: clean.
- `scripts/check_one_copy.awk Cargo.lock`: clean. Cargo.lock and Cargo.toml are unchanged by the PR, so deny, audit and one-copy are unaffected.
- `cargo clippy --locked --all-targets -D warnings`: clean.

`make msrv` is `cargo +1.98.1 test`. The repo's toolchain is pinned to 1.98.1 (rust-toolchain.toml), so the author's `make test` already ran on the MSRV. `make kani` is not needed: no Kani-lane module touches the function generator. With these, the gate is adequate.

## Verdict

AC-22 is substantively backed, and the tests are strong (M1, M2, M5 and M6 killed). One medium gap: the 'smallest node id' clause this PR adds to AC-22 has no test. Either add a two-group fixture, or keep that clause marked Planned. The row should not say Covered until one of those is done.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The new AC-22 clause 'smallest node id when a name is held by duplicate groups on several node ids' has no test; mutant 'largest wins' passes all 26 tests, yet AC-22 is flipped to Covered | spec/oracle/functional/FR-021-function-application-oracles.md:219-220, spec/oracle/functional/FR-021-function-application-oracles.md:262, spec/oracle/matrix/TC-031-function-application-oracles.md:112-113, spec/oracle/matrix/tests.md:38, src/oracle/function/mod.rs:1218-1220 |

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
