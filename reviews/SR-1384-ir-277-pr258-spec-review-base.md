---
id: "SR-1384"
title: "CG PR 258 spec review (base): Kani batching, output-cap refusal and launcher gaps"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-codegen@1add57d77cfcd3a9e39e515f063fde12b81f4d70; spec/kani/functional/FR-017-kani-execution-evidence.md, spec/kani/functional/FR-028-bounded-proof-ceilings.md, spec/kani/matrix/TC-027-kani-execution-evidence.md, spec/kani/matrix/TC-039-bounded-proof-ceilings.md, spec/kani/matrix/tests.md, spec/assurance/AD-004-cg-crate-layout.md (diff origin/main...HEAD, base 4de9f20); src/kani/run/launch.rs, src/kani/abi.rs, src/kani/output/*.rs, src/kani/run/execute.rs read at the base as context"
---

# SR-1384: CG PR 258 spec review (base)

## Summary

Ticket: IR-277. Spec-only PR. Review set: base plus failure-domain (SR-1385), integrity
(SR-1386) and EARS conformance (SR-1387). Scope is the PR diff only.

What I checked, each re-measured rather than taken from the PR:

- Launcher gaps against `src/kani/run/launch.rs` at 4de9f20, which the PR leaves unchanged.
  - Line 106-107: `stdout_reader.join().unwrap_or_default()` turns a reader panic into empty text.
    Confirmed.
  - Line 99: `Instant::now().checked_add(timeout)` gives `None`, so the run never elapses.
    FR-017-AC-15 stays Covered and matches the code and
    `a_timeout_of_duration_max_never_elapses_and_does_not_panic`. Confirmed.
  - Lines 101-104: `kill_process_tree` runs only when `wait_until` did not return `Ok(Some(_))`, so
    nothing is killed on a normal exit. Confirmed.
  - Line 54 (`CAPTURE_LIMIT`) and lines 151-201 (`capture_tail`): the tail is kept silently.
    Confirmed.
  - The two tests the spec says are replaced exist under the names it gives:
    `a_stream_longer_than_the_capture_limit_keeps_only_its_tail` (l.410) and
    `a_launcher_printing_more_than_the_limit_completes_with_bounded_text` (l.533). Both carry
    `Trace: FR-017-AC-14`.
- Numbering.
  - FR-017-AC-21 to AC-25 and FR-028-AC-12 never appear in `git log -S` on origin/main, so they are
    fresh.
  - FR-028-AC-11 appears only in SR disposition prose from commit 0411157, about an old draft.
    FR-028-AC-10 appears nowhere. Skipping both is conservative and harmless.
  - FR-017-AC-14 is rewritten in place (FND-001).
- Matrix consistency.
  - AC-14 moves from the Covered row to a new Planned row.
  - The TC-027 and TC-039 index rows list every new AC.
  - TC-027 prose names the replaced tests, and TC-039 adds step 10 with its expected result.
  - The AD-004 note at line ~1256 matches the FR-028 rule and is labelled a default the owner may
    change.
  - Every new item is marked Planned (IR-277). The planned markers are honest.
- `make spec` passes, with the standing module warnings only.
- `quire coverage --strict`: 76 at base, 77 at head. The one added row is FR-028-AC-12, because
  TC-039 has no backing. The five new FR-017 criteria are unbacked (`minted_targets` backed:false),
  but they do not add to the strict count: their rows also name TC-027, which has backing.
  FR-017-AC-14 reports `backed: true` from the two tests that assert what it now forbids. So 77 is
  the tool's honest mechanical result, but it understates what is unbacked.

## Verdict

The base checks pass: ids, matrix, TC prose, planned markers and the gate. The defects that block
merging are in SR-1385 and SR-1386. The base findings below are low.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-017-AC-14 is rewritten in place with the opposite meaning. Two tests still tagged `Trace: FR-017-AC-14` assert the silent tail, so `quire coverage` reports AC-14 `backed: true` from tests that contradict it. The 76 to 77 strict count hides the five new unbacked FR-017 criteria. Either mint a fresh id and mark AC-14 superseded, or say in the matrix row that the AC-14 backing is stale until the code PR. | spec/kani/functional/FR-017-kani-execution-evidence.md:187, spec/kani/matrix/tests.md:28, src/kani/run/launch.rs:408,531 |
| FND-002 | low | The FR-017 title and Description still say "Run one bounded Kani obligation". The new Inputs bullet "Or a batch of such harnesses" hangs off the launcher bullet above it instead of the harness bullet. | spec/kani/functional/FR-017-kani-execution-evidence.md:3,21,41 |

## New findings (disposition pass 1)

Reviewed at a0eba86e850e08f0e19edfa99ce804b166dfda53.

Coverage: `quire coverage --strict` reports 83 at a0eba86 and 76 on main. I diffed the sorted output; the +7 rows are:

- FR-017-AC-21, FR-017-AC-22, FR-017-AC-23, FR-017-AC-24 and FR-017-AC-25 (verification);
- the moved FR-017 row in `tests.md` that names TC-043 (functional-coverage);
- FR-028-AC-12 (verification).

The count is honest about what the tool can see. It still leaves out two rows that are backed only by the two tests asserting the old tail: the FR-017-AC-14 verification row and the TC-043 index row. The matrix row and TC-027 now say this in words.

TC-043 is a fresh id: `git log -S` finds nothing before a0eba86, and TC-042 is the previous highest. `make spec` exits 0.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | TC-043 Expected Results skip number 5 and no longer line up with the Test Procedure. Result 6 answers step 5, result 7 answers steps 6 and 7, and result 8 answers step 8. A reader cannot pair each step with its result. | spec/kani/matrix/TC-043-kani-batching-and-output-bounds.md:46-79 |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed a0eba86 | The tests.md row and the TC-027 Implementation now say that the two launcher tests still tagged FR-017-AC-14 assert the old silent tail and are not its evidence, and that the code change deletes them. The stale backing is now disclosed. |
| FND-002 | fixed a0eba86 | The title and Description now cover single and batch runs ("Unless a statement says batch, it describes a run of one harness"). The batch Inputs bullet follows the harness bullet and names the batch entry. |
| FND-003 | fixed ce7e0f0 | TC-043 now has Test Procedure steps 1-10 and Expected Results 1-10, one result per step. Step 5 (`Duration::MAX` T) and step 6 (no-report exits) are new. Rechecked at ce7e0f0: `make spec` exits 0 and `quire coverage --strict` is still 83, identical to a0eba86. |
