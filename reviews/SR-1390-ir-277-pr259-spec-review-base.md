---
id: "SR-1390"
title: "CG PR 259 spec review (base): the coder's amendments to FR-017, FR-028, interface-001, TC-027, TC-039, TC-043 and the matrix"
type: SpecReview
analysis: base
review_set: base
scope: "agent-ix/quire-contract-codegen@0516084a1d8f15b28dc7bf140cb4307872fa095c; spec/kani/functional/FR-017-kani-execution-evidence.md, spec/kani/functional/FR-028-bounded-proof-ceilings.md, spec/core/functional/interface-001-codegen-api.md, spec/kani/matrix/{TC-027,TC-039,TC-043,tests}.md (diff origin/main...HEAD)"
---

# SR-1390: CG PR 259 spec review (base)

## Summary

Ticket: IR-277. Base checklist over the spec text this code PR changed. The edits drop the
"(Planned, IR-277)" markers, flip the matrix rows and add five amendments, which I judged against
what #258 meant and against real Kani 0.68:

1. Group key gains launcher, working directory and target directory (FR-017 batch bullet,
   interface-001). Faithful: one process physically has one of each, so requests naming different
   ones can never share a process. Reflected in FR-017 and interface-001; not in TC-043 (no step
   mixes crates), see SR-1389 FND-001.
2. The "two blocks for one path" rule counts only counterexample blocks, not cover playbacks
   (FR-017 bullet and AC-23). The cover exclusion is right (real Kani prints a cover block beside a
   failing check's block), but the rule is still wrong: FND-001.
3. T is carried on the member's `batch` statement (`KaniBatchInvocation.timeout_seconds`) instead
   of in the `TimedOut` reason. Faithful: "naming T" is satisfied by the evidence record, the
   reason enum stays a stable code, and FR-017 and FR-028's timed-out-member bullet both say where T is.
4. "Refuse the whole batch" read per launcher process, except a member missing from the crate,
   which refuses the whole call. A reasonable reading (FR-017-AC-21 already uses "batch" for one
   group, N its member count), but the spec does not say it: FND-002.
5. `execute_kani_obligations` returns `Result<Vec<KaniGroupRun>, KaniExecutionRefusal>` and
   `launch_evidence` returns `KaniExecutionRefusal`. Reflected in interface-001's operations list;
   the slice text further down was not updated: FND-003.

`quire validate` passes on the head (`make spec` inside `make ci`). ID formats, AC numbering and
matrix links are intact; FR-028-AC-1..9 stay Planned and FR-028-AC-12 has its own Covered row.

## Verdict

Amendments 1, 3 and 5 are faithful to #258. Amendment 2 needs an owner or spec decision because the
rule it refines refuses a batch on ordinary real-Kani output. Amendment 4 is sound but should be
written into FR-017 itself.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-017's unattributable-playback bullet and FR-017-AC-23 refuse the whole batch when two counterexample blocks are headed for one path. Real Kani 0.68 prints one counterexample block per failed property check (reproduced: two independently failing assertions in one harness gave two `two::check` blocks), so any member that fails two checks refuses every member. That contradicts FR-017-AC-22 ("a falsified member never changes another member's outcome") and the single-run rule, which takes the first counterexample block. The amendment's own parenthetical describes the cover case only. The rule needs restating, e.g. a member's playback is the first counterexample block headed for its path, as in a single run, and only a block for a non-member path refuses | spec/kani/functional/FR-017-kani-execution-evidence.md:141-144, spec/kani/functional/FR-017-kani-execution-evidence.md:245 |
| FND-002 | medium | "The whole batch" has two meanings in FR-017 after the edit. FR-017-AC-21 uses "batch" for one group (N its member count, one outer bound), while the HarnessNotInCrate bullet now says "not even for the members of another group", i.e. the whole call. The report-completeness, no-report, unattributable-playback and over-limit bullets say "refuse the whole batch" and only interface-001 says the refusal is per process. Two implementers could read the over-limit or playback refusal as voiding every group. FR-017 should define batch (the caller's list) and group (one process) and say which refusals cover which | spec/kani/functional/FR-017-kani-execution-evidence.md:166-171, spec/core/functional/interface-001-codegen-api.md:115-116 |
| FND-003 | medium | interface-001's `kani_obligation_execution_slice` was not updated. Its scope still says "running one FR-015 harness", its refusals list omits `kani_output_over_limit`, `kani_output_unread`, `BatchTimedOut`, the three report-completeness refusals and the two playback refusals, and its evidence list omits the `batch` statement. The operations list above it was updated, so the two parts of the same interface now disagree | spec/core/functional/interface-001-codegen-api.md:282-288 |
| FND-004 | low | TC-043 step 8 and expected result 8 still say "two blocks for one path" and "the pair of blocks", although the PR amended FR-017-AC-23 to "two counterexample blocks". The test case should follow the AC it verifies | spec/kani/matrix/TC-043-kani-batching-and-output-bounds.md:39, spec/kani/matrix/TC-043-kani-batching-and-output-bounds.md:70 |
| FND-005 | low | The TC-027 paragraph the PR rewrote still attributes the launcher tests to `src/kani_execution.rs`, which no longer exists (they live in `src/kani/run/launch.rs` and `execute.rs` since AD-004 step 2). The edited sentence should name the real files | spec/kani/matrix/TC-027-kani-execution-evidence.md:117-123 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | low | Wording only, and already on main. The interface-001 execution slice this round rewrote still says, in its unchanged `outcome_source` line, that the report is read "in `src/kani_transcript.rs`". That file no longer exists; the reader is `src/kani/output/report.rs` | spec/core/functional/interface-001-codegen-api.md:288 |

## Dispositions

Round 1, reviewed at f5ca91c755ed76b936b4f6495cceb4e031cc1566. `make spec` passes inside `make ci`. Sweeps of the head: no "two blocks", "pair of blocks" or `PlaybackDuplicated` remains in `spec/`, `src/` or `tests/`. "Whole batch" now appears only for the HarnessNotInCrate refusal of the call, which is the defined meaning. `src/kani_execution.rs` and `src/kani_transcript.rs` remain only in AD-001, AD-003 and AD-004, which this PR does not touch, and in interface-001 line 288 (FND-006).

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | f5ca91c |
| FND-002 | fixed | f5ca91c |
| FND-003 | fixed | f5ca91c |
| FND-004 | fixed | f5ca91c |
| FND-005 | fixed | f5ca91c |

Round 2, reviewed at 8a2632347343526a1421b98a34fa9639d69bd57a. interface-001 `outcome_source` and the FR-017 single-reader bullet now name `src/kani/output/report.rs` (and `playback.rs` for the console payload). Evidence wording in FR-017, FR-017-AC-22, interface-001 and TC-043 now says "the group's argument vector". `make spec` passes inside `make ci`. `quire coverage --strict`: 65 unbacked rows, minted flags unchanged, `status_lies` 0. In the files this PR touches, the sweep finds no `kani_transcript`, `kani_execution.rs`, `PlaybackDuplicated` or "two blocks" in spec text. One code comment says a member "has two blocks under its own path", which describes the new behaviour and is correct. No new finding.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-006 | fixed | 8a26323 |
