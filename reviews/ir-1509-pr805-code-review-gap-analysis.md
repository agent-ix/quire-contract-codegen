---
id: SR-17940486709564847047733
title: "IR-1509 PR #805 gap-analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen PR #805 at 48809302; IR-1509; FR-015-AC-41, FR-015-AC-119, FR-015-AC-126, FR-015-AC-132, FR-015-AC-151, FR-017-AC-60, FR-017-AC-63, FR-017-AC-64, TC-025 step 56; spec/kani/matrix/tests.md, tests/it/kani_obligations_v2_clause_controls.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: reviews
---

## Summary

Ticket: IR-1509. Plan completion: not assessed. The PR adds no production code; it adds one test
module (13 tests: 8 default lane, 5 `#[ignore]` Kani lane) and spec status edits. Computed matrix
(`quire matrix --scope .`): FR-015-AC-132 and FR-017-AC-64 are tagged by the six enumeration and
record tests plus the matrix wording test; FR-015-AC-151 by the playback test and the five
installed tests; FR-015-AC-41, AC-119 and AC-126 gain the installed tests as extra binders;
FR-017-AC-60 stays untagged and PLANNED. The module name `kani_obligations_v2_clause_controls`
matches the `kani_obligations` filter of `KANI_FILTERS`, so `make kani` (`--ignored
--test-threads=1`) runs the five installed tests serially; each `#[ignore]` reason names that lane.
Run in an own worktree: the eight default-lane tests pass (11.6 s).

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | high | Coverage inflation: the FR-015-AC-151 matrix row is `Covered` though the criterion's pipeline-compiled subject item has no test and no code (the subject is test-written; no CG or QSL step emits one). Same root as SR spec-review FND-001; the row must read Partial with the unmet item and its owner | spec/kani/matrix/tests.md:57 |
| FND-002 | medium | The Kani-backed 999 playback that AC-151 now defers to FR-017-AC-60 is owned by IR-917, whose code PR #583 is a draft last updated 2026-10-26. If IR-917 is not being built, the deferral parks the control under a ticket nobody is building (planner rule). The lead should confirm IR-917 is live before AC-151 points at it | spec/kani/functional/FR-017-kani-execution-evidence.md:912, spec/kani/functional/FR-015-bounded-kani-obligations.md:966 |

## Verdict

Not mergeable at 48809302 until FND-001 is fixed (with the spec-review FND-001 ruling); FND-002
needs a lead confirmation, not necessarily a code change. Checked with no gap: every default-lane
test runs the real QSL-compiled package through `negotiate_kani_native_obligations`; owner
mutants are made in copies of the locked checkouts and `cargo metadata` must resolve the owner to
the copy; no stub or tautology in the enumeration path (reviewer mutants in SR code-review); the
installed tests call the public `execute_kani_obligation` runner, not raw `cargo kani`; no
production code without an owning requirement was added.

## Dispositions

Round 1, head bc809502.

| FND | outcome | reason |
|---|---|---|
| FND-001 | fixed | The FR-015-AC-151 matrix row reads `Partial`, with the pipeline-compiled subject PLANNED (blocked on QSL operation bodies) and the Kani-backed playback PLANNED under IR-917 |
| FND-002 | deferred | Planner ruling 2026-11-12: the Kani-backed 999 playback stays FR-017-AC-60 under IR-917, which is to be rebuilt on current main after #805 merges; AC-151, the matrix row and the record name that owner |
