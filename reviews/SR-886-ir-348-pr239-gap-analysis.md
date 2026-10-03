---
id: "SR-886"
title: "CG PR 239 gap analysis: AD-004 step 2g and L-1/L-2 against the module map, the dependency direction and the motion rules"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@807755fbeb8b79156ca168185172722dfb9021ac; spec/assurance/AD-004-cg-crate-layout.md (target tree, module-to-subsystem map, Dependency direction, L-1, L-2, Migration order step 2 and 2g, Step 2f item map kani_witness_join table, Risks), src/replay/, src/routed/, src/publication/, src/lib.rs, tests/it/layout.rs, tests/it/kani_argument_order.rs (diff origin/main...HEAD, merge base 81a9c69; AD read at origin/main 80f4785)"
---

# SR-886: CG PR 239 gap analysis

## Summary

Ticket: IR-348. PR: agent-ix/quire-contract-codegen#239 at 807755f. I read AD-004 at
`origin/main` (80f4785, one commit newer than the PR's base; that commit touches only Risks
wording). Plan completion: not assessed.

Each AD statement against the code:

- Module-to-subsystem map. Each destination is set by a map row: `spine_replay` to
  `replay/function.rs`, `frame_replay` to `replay/frame.rs`, `capability` to
  `routed/capability.rs`, `routed_generation` to `routed/generate.rs`, and `publication` (writer,
  published identity) to `publication/publish.rs`. `kani_witness_join` goes to `replay/witness.rs`
  as the decode half, by the map row and by the step 2f item map's 2g rows: 5 private items, 2
  `pub` items and 15 tests. After 2f the flat file held exactly those, so the whole-file move
  matches the table. Visibility is "unchanged" in the table and unchanged in the code. Met.
- Step 2g ("`replay`, `routed`, `publication` (with the decode half of `kani_witness_join`)";
  "2c to 2g are `git mv` plus path fixes ... no logic change"). Met (see SR-885 for the
  rename-similarity, item, API and rustdoc evidence).
- A `mod.rs` holds only module declarations and their comments. Met for the three new files.
- "The layout test (L-1, L-2) lands with 2g". Step 2 authorizes the new `tests/it/layout.rs`. The
  "no test code under `tests/` edited" rule is about existing tests, and
  `kani_argument_order.rs` changes one comment line, which is the allowed exception.
- L-1: the test compares the top of `src/` with a list held in the test (eight directories plus
  `lib.rs`), as the AD says. It also requires every mapped module file to be present. Probes for
  a missing mapped file and a stray top-level file both fail it. The test asks for nothing beyond
  the AD's tree. `kani/test_support.rs` and the `mod.rs` files are tree entries. Met.
- L-2: "no file imports an item through the crate root". Met, and a probe fails it.
  "Acyclic": met, and a probe fails it. "`#[cfg(test)]` modules included": met, because the test
  scans whole files, and a probe fails it. "Follows the dependency direction above": the
  directory table and the top-level `kani/` order are met. The `kani/generate/` order bullet of
  the same section is not checked (SR-885 FND-001).
- Risks: the test's blind spots (`super::`, macros, intra-doc links) match what the AD's Risks
  section records.

## Verdict

Step 2g implements its AD rows completely, and L-1 is exactly the test the AD describes. L-2
leaves out one rule of the dependency direction, recorded once in SR-885 (FND-001) and not
repeated here. Two low findings on the AD itself, neither blocking this PR.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | AD-004's target tree lists `routed/adapter.rs` ("the adapter trait and its one Kani implementation"), and the Dependency direction rules say in the present tense that the adapter trait "is defined in `routed/adapter.rs`". But no module-map row, migration step or ticket creates it, and `src/` has no trait at all. L-1 as written (the map's modules) does not need the file, so the PR is right not to create it. The AD needs an amendment that either names the step that creates it, or marks it reserved and not yet created, the way it marks `terminal.rs`, `spec.rs` and `render.rs` | spec/assurance/AD-004-cg-crate-layout.md:247, spec/assurance/AD-004-cg-crate-layout.md:324-326 |
| FND-002 | low | Spec prose still cites the deleted flat paths `src/kani_witness_join.rs`, `src/spine_replay.rs` and `src/capability.rs`: FR-016:51, FR-024:121 and 126, TC-026:56, 65 and 68, TC-035:63-64, ADR-002:32 and AD-001:192 and 197. Step 7 lists only `interface-001`, `tests.md` and AD-001's Current state and Risks, so no step owns the FR, TC and ADR prose fixes | spec/replay/functional/FR-016-witness-native-replay.md:51, spec/replay/functional/FR-024-counterexample-envelope-intake.md:121 |

## Dispositions

Round 1, reviewed at ea99091f21f71954932d36f6059d93ca00c21929 (fix commit ea99091; AD-004 amended).
I grepped the spec tree for citations of deleted flat CG source paths. Leaving out AD-004, and
the IR and RT paths, which do not belong to CG, the citing documents are AD-001, AD-002, AD-003,
interface-001, ADR-002, FR-016, FR-017, FR-021, FR-024, TC-026, TC-027, TC-035 and tests.md.
The amended step 7 names FR-016, FR-024, TC-026, TC-035, ADR-002, AD-001, AD-002 and AD-003,
alongside interface-001 and tests.md. It misses FR-017:114 (`src/kani_transcript.rs`), TC-027:115,
122, 130 and 135 (`src/kani_execution.rs`, `src/kani_transcript.rs`) and FR-021:264
(`src/oracle.rs`).

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | ea99091 |
| FND-002 | still-open | The step 7 list is incomplete. It omits FR-017 (`src/kani_transcript.rs`, line 114), TC-027 (`src/kani_execution.rs` and `src/kani_transcript.rs`, lines 115, 122, 130, 135) and FR-021 (`src/oracle.rs`, line 264), which also cite flat paths deleted by steps 2d and 2f |

Round 2, reviewed at fbed9858a726d6346976075d2c2723631d6ab805. I re-grepped `spec/` (outside AD-004) for every deleted flat CG source file name, at head and at `origin/main` 348fb33. The documents that cite one are AD-001, AD-002, AD-003, interface-001, ADR-002, FR-016, FR-017, FR-021, FR-024, TC-026, TC-027, TC-035 and `spec/oracle/matrix/tests.md`. That is exactly the list step 7 now names. Step 7 also requires a re-grep before the step starts.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-002 | fixed | fbed985 |
