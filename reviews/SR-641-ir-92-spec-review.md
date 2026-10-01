---
id: "SR-641"
title: "CG PR 206 spec review: FR-016-AC-14, FR-015-AC-33, TC-025, TC-026, interface-001 and the matrix"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@16828648175edf3190ad799e975a944a5a8ab518; spec/functional/complete-v1/FR-015-bounded-kani-obligations.md, spec/functional/complete-v1/FR-016-witness-native-replay.md, spec/interface/interface-001-codegen-api.md, spec/test-matrix.md, spec/test/complete-v1/TC-025-bounded-kani-obligations.md, spec/test/complete-v1/TC-026-witness-native-replay.md"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-016
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-026
    type: reviews
---

# SR-641: CG PR 206 spec review

## Summary

Ticket: IR-92. PR: agent-ix/quire-contract-codegen#206, head 1682864. This is a base-checklist spec
review of the six edited spec files, with the EARS/atomicity and integrity checks applied to the
two new ACs and to the interface edits. No requirement or AC is deleted. The only AC text
changes are the two new rows.

## Method

I read each edited file in full at the head and diffed it against main 8fcb51f. I compared the
interface-001 signatures with `src/spine_replay.rs`, `src/frame_replay.rs` and the `src/lib.rs`
exports. I compared the matrix rows with the AC tables and the test `Trace:` tags. `make spec`
(quire validate) ran inside `make ci`.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The FR table in the matrix marks FR-015-AC-33 `Covered` by TC-025. The TC index row for TC-025 does not list FR-015-AC-33, and it also omits FR-015-AC-13 and FR-015-AC-26 to FR-015-AC-32, which the FR table marks covered by TC-025. The same PR adds FR-016-AC-14 to the TC-026 row, so the two rows now follow different rules. A reader of the TC index cannot find AC-33. | spec/test-matrix.md:51, spec/test-matrix.md:160 |
| FND-002 | low | FR-016-AC-14 bundles five separately testable behaviours into one AC: compile with the lock source and fill `package.dependencies`, settle through the imported function, refuse a missing import, refuse an identity mismatch, and refuse an unselected library. Four tests carry its tag. When one of them regresses, the matrix cannot say which behaviour broke. FR-015-AC-33 is compound in the same way, with three behaviours. | spec/functional/complete-v1/FR-016-witness-native-replay.md:131 |

## Verdict

Mergeable once FND-001 is fixed; it is a one-line matrix edit. FND-002 is a style point. Other ACs
in this repository (for example FR-016-AC-11) are compound in the same way.

What is right:
- interface-001 `ReplayPackage::new` matches the code. The output lists
  `Dependencies(DependencyLockError: Duplicate | Input)` and `CallSite`, and the semantics say the
  lock is the call site's dependency input and the request's `package.dependencies` carries "the
  dependency's own source", singular. The old "compiles standalone" text is gone.
  `FrameReplay::new` lists the six `FrameReplayError` variants that exist in the code, and its
  semantics match the code (payload from the answer, `clause_node` and `occurrence_key` from the
  payload, `replay` calls `replay_frame`).
- FR-016 Dependencies now says `call_site` compiles the unit with its locked dependencies, which
  matches the code. TC-026's status paragraph no longer claims the replay is unreachable.
- FR-015-AC-33 and FR-016-AC-14 can both fail, and each has a tagged test that fails when its main
  clause is false (see SR-640). Both are marked `Test (TC-025)` / `Test (TC-026)`, which matches the
  default-suite tests.
- TC-025's Expected Results item names the default-suite frame request builder and the no-frame
  refusal under FR-015-AC-33.
- `make spec` passed.

## Dispositions

Round 1, reviewed at e0fc6407f5812faa593af53581f92f265b491f29 (fix commit e0fc640).

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | e0fc640 |
| FND-002 | fixed | e0fc640 |

Evidence for this round:
- **FND-001.** The TC-025 index row now lists FR-015-AC-13 and FR-015-AC-26 to FR-015-AC-36. The
  TC-026 row lists FR-016-AC-14 to FR-016-AC-20 one by one. The FR table rows (AC-26 to AC-36 for
  TC-025, and AC-14 to AC-20 for TC-026) agree with both.
- **FND-002.** FR-016-AC-14 is split into AC-14 to AC-20 and FR-015-AC-33 into AC-33 to AC-36.
  - Each new AC has a single behaviour, and each has its own tagged test: AC-15 is the
    request-shape test; AC-16, AC-17 and AC-18 are the refusal tests; AC-19 and AC-20 are new
    tests; AC-33/34 share the envelope test; AC-35 is the refusal loop; AC-36 is the settle test.
  - Probes P1, P2, P4 and P5 show these tests fail when the behaviour is removed.
  - No AC was deleted: the old AC-14 and AC-33 ids are kept, with narrowed text.
- `make spec` passed.
