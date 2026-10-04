---
id: "SR-1412"
title: "CG PR 260 gap analysis: the cover-last rule of FR-015 against the state and frame harnesses"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@a85f1cf666589eb5599328705df7ff737ca2f849; src/kani/generate/frame.rs, tests/it/kani_obligations_state_frame.rs, tests/state_frame_support/subject.rs; read against spec/kani/functional/FR-015-bounded-kani-obligations.md (Outputs, Behavior, AC-7, AC-26, AC-30 to AC-32, AC-46), spec/kani/functional/FR-017-kani-execution-evidence.md (Behavior, AC-4, AC-5), spec/kani/functional/FR-029-run-outcome-terminal-record.md (AC-2, AC-5), spec/kani/matrix/tests.md"
---

# SR-1412: CG PR 260 gap analysis

## Summary

Ticket: IR-451. PR: agent-ix/quire-contract-codegen#260 at a85f1cf. Plan completion: not
assessed.

- FR-015 Behavior says "The generator shall end every generated harness with exactly one
  non-vacuity cover". On main the state and frame harnesses broke that statement: the cover sat
  before the assertion. The PR brings them into line. The PR's claim that no spec change is
  needed holds for the behavior statement, which already requires the cover to come last. It
  does not hold for the acceptance criteria: no AC states the ordering (FND-001).
- FR-015-AC-7 asks only that "a contract harness's cover stands after the contract call". On
  main, the state harness's cover came after the subject call, so the matrix row "FR-015-AC-7
  through FR-015-AC-12 ✅ Covered" was literally true while the defect was present. The row is
  honest as worded. The AC is weaker than its statement.
- FR-015-AC-26 asks only for "exactly one non-vacuity cover" per state or frame harness. The
  new order assertion is traced FR-015-AC-26. It checks more than AC-26 says, which is behaviour
  that no AC owns.
- FR-015-AC-46 (PLANNED, IR-489) applies to V2 clause harnesses and places the cover "after
  every assumption and after the subject call". It also leaves the assertions out.
- FR-017: "A playback printed for a satisfied cover ... shall never be taken as a
  counterexample." The classifier is unchanged and still meets this, and a harness that fails
  everywhere classifies `Falsified` (measured, SR-1411). FR-029-AC-5 maps
  `FailedWithoutCounterexample` to `Failed`. After this PR the state and frame lane no longer
  produces it for the shared-valuation case.
- FR-017's batch rule says "Kani prints one per failed check (and a cover block beside them)".
  Measured on Kani 0.68.0, Kani prints one block per distinct valuation (FND-002).
- Original intent of the fixture. In IR-412 (8fcb51f), the module doc of `subject.rs` gave the
  funded-only guard (`balance > 0`) one purpose: to avoid exactly this collision. Removing it
  loses no other meaning. The native replay test (AC-32) runs whatever valuation Kani returns
  through `deposit_touching_audit`, and at (0, 0) audit still changes. The allowed-effect half
  uses (5, 0). AC-30, AC-31 and AC-32 still assert what they did.
- Stale branch `fix/ir-464-nonvacuity-cover` (worktree cg-ir-464, 4822e52, no PR). I read it
  and did not touch it. It is based on the pre-AD-004 flat layout (`src/kani.rs`,
  `src/bounded_kani_corpus.rs`) and adds a cover after the call or assertion in the V1 bundle
  and corpus harnesses. It also adds an FR-015-AC-37 row, but that id is now taken on main by
  the V2 arithmetic AC. It does not touch `frame.rs`, so there is no textual overlap or conflict
  with this PR, and it puts the cover last, the same order this PR adopts. Linear IR-464 reads
  Done ("closed structurally by CG AD-004 step 4b: the HarnessSpec constructor refuses an empty
  cover list"; untrusted ticket text). On main there is no `HarnessSpec`, and
  `v1_bundle.rs::render_kani_source` and the corpus harness still emit no cover. That is a
  pre-existing gap outside this diff, recorded here for the ticket owner and not as a finding
  against this PR.

## Verdict

The code change meets the FR-015 behavior statement and FR-017 without a classifier change. One
medium and one low spec gap sit outside the diff, and neither blocks the merge. Mergeable. A
follow-up spec PR should pin the cover-last rule in an AC.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | No AC pins the FR-015 rule "end every generated harness with exactly one non-vacuity cover". AC-7 asks only that a contract harness's cover come after the contract call. AC-26 asks only for exactly one cover in the state and frame harnesses. AC-46 (V2, planned) asks only for "after every assumption and after the subject call". So the IR-451 defect, with the cover after the call but before the assertion, met every AC while the matrix read ✅ Covered. The new order assertion, traced to AC-26, tests behaviour that no AC owns. AC-7 (or AC-26 and AC-46) should say that the cover is the last statement of the harness, after every assertion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:251, spec/kani/functional/FR-015-bounded-kani-obligations.md:270, spec/kani/functional/FR-015-bounded-kani-obligations.md:290 |
| FND-002 | low | The FR-017 batch rule says "Kani prints one per failed check (and a cover block beside them)". Measured on Kani 0.68.0, the playback test name is a hash of the concrete values: all-zero gives `kani_concrete_playback_check_14500938541994739385` in every harness. Kani prints one block per distinct valuation, so two checks at the same valuation collapse to the one listed first. That is the mechanism behind IR-451. The sentence, and the measurement note after it (two independently failing assertions gave two blocks), hold only for distinct valuations and should say so. | spec/kani/functional/FR-017-kani-execution-evidence.md:152-157 |

## Dispositions

Round 1, reviewed at b2598f8f468629fd2ebba5eaa2d9193f0b403850 (fix commit b2598f8 over a85f1cf).

FR-015-AC-7 now states the rule directly, and it can be checked: "The cover is the last
statement of the harness, after every assertion the harness carries". Its "tested by" clause
matches the tests:

- State and frame harnesses: source order, checked by
  `tc_025_a_postcondition_yields_a_contract_harness_and_a_scoped_frame_harness`, which is tagged
  FR-015-AC-7, AC-26, AC-27 and AC-28. AC-27 and AC-28 were there before, and the test does check
  bounds and fields, so the tags are not inflated.
- Scalar harness: source order, checked by
  `tc_025_scalar_harness_asserts_the_native_arithmetic_relation`, already tagged AC-7.
- Contract harness: cover after the contract call, checked at `kani_obligations.rs:524-525`. Its
  ensures check runs inside that call.
- Precondition harness: it asserts nothing.

With a hand mutation of each family (state, frame and scalar), the order tests fail. AC-46 now
says "after every assertion the harness carries". The matrix row "FR-015-AC-7 through FR-015-AC-12
✅ Covered" is honest for these families.

Still outside this diff, as recorded in round 0: the V1-bundle and corpus harnesses emit no
cover, so "every generated harness" in AC-7 is not met there (IR-464).

FR-017, measured again on Kani 0.68.0:

- With the cover before the assertion, the report lists the cover as Check 2 and the assertion
  as Check 3, and only the cover's block is printed.
- With the cover after the assertion, the assertion is Check 2 and the cover Check 3, and both
  blocks are printed, the assertion's first.

So "listing first the check that comes first in its report" holds. The rest of the bullet agrees
with `classify_report`, which returns `FailedWithoutCounterexample` when the only block is a
cover block. It also agrees with `run_group` (the member takes the first counterexample block for
its path). The case of two failing assertions at one valuation is stated conditionally. I could
not construct it, because a failing `assert!` blocks its path and every branch is decided by a
drawn input. As written, the bullet's conclusion for that case follows from the hash dedup and the
classifier. A grep of spec/, src/ and tests/ finds no remaining "per failed check".

`quire coverage --strict`:

- Unbacked rows: 65 at the head and on main 83f9687. `status_lies` is 0 on both.
- `unmatched_tags` goes from 86 to 87. The one new entry is the prose reference "(IR-451)" in the
  new test's doc comment, which the scanner reads as a trace id. This has no effect on coverage.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed b2598f8 | FR-015-AC-7 now says "The cover is the last statement of the harness, after every assertion the harness carries", with per-family "tested by" markings that match the tests. AC-46 adds "after every assertion the harness carries". |
| FND-002 | fixed b2598f8 | The FR-017 batch bullet now says "one block per distinct input valuation (the playback test's name is a hash of the concrete values), listing first the check that comes first in its report", and gives each of the three cases. I measured these on real Kani and read them against the classifier. |
