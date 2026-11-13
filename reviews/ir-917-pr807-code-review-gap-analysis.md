---
id: SR-17940486709564847047741
title: "IR-917 PR #807 gap-analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen PR #807 at c5323378; IR-917; FR-014-AC-54, FR-015-AC-41, FR-015-AC-119, FR-015-AC-126, FR-015-AC-151, FR-017-AC-57 to AC-60, FR-025-AC-11 to AC-13; spec/kani/matrix/tests.md, spec/oracle/matrix/tests.md, TC-024, TC-025, TC-027"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: reviews
---

## Summary

Ticket: IR-917. Plan completion: not assessed. Computed matrix (`quire matrix` at c5323378):
FR-017-AC-57 and AC-58 tagged by `concrete_playback_refuses_unsealed_sources_and_bounds_the_launch`
(execute.rs:2270); AC-59 additionally by `concrete_playback_parser_pairs_the_exact_test_and_summary`
and `the_playback_entry_launches_only_for_the_sealed_artifact_with_one_appended_test`; AC-60 by
that stand-in test and the three ignored `tc_027_kani_playback_at_999_*` tests; FR-014-AC-54 by
`a_literal_narrow_is_elided_only_when_its_value_is_inside_the_target`,
`native_clause_expectation_accepts_only_in_range_literal_narrows`,
`qsl_literal_narrow_oracle_is_independent_of_a_generated_sibling` and
`public_v2_integer_literals_preserve_owner_semantics_and_every_denial_prefix`; FR-015-AC-41 by
seven tests (four ignored Kani-lane). Every `#[ignore]` carries the reason
`kani lane: run serially through make kani`, and `make kani` runs `--ignored --test-threads=1`
over `it`. No untagged criterion among the ones the PR marks Implemented.

Status flips judged: FR-017-AC-57/58/59, FR-014-AC-54 honest (the tests exist and run in the
default lane). FR-017-AC-60 and FR-015-AC-41 are backed by the coder's executed logs, which match
the record; AC-60's teardown clause is not shown by the installed runs (code-review SR-17940486709564847047740 FND-001).
FR-015-AC-119 and FR-015-AC-126 stay open; their owners are judged below.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | medium | FR-015-AC-119 stays PLANNED with two open parts and no live owner for either: FR-025-AC-11 through AC-13 are assigned to "owner IR-948", but IR-948 is Done (Linear state Done; the FR-025 rows still read "PLANNED until IR-948 lands"), and the pipeline-compiled subject is "blocked on QSL operation bodies with no ticket". Under the planner rule (no AC parked under a ticket nobody builds) the lead must name a live ticket for the FR-025 result mapping and either file a QSL ticket for operation bodies or record the item as out of scope with the ruling's id. The PR newly writes "owner IR-948" into AC-119 | spec/kani/functional/FR-015-bounded-kani-obligations.md:936, spec/kani/functional/FR-025-generated-subject-abi.md:186 |
| FND-002 | medium | FR-015-AC-126 (multi-profile regeneration umbrella, including "Each healthy profile verifies at 16 GiB") carries no status marker and names no owner ticket in its row; its IR-917 profile is the clause this PR closes and its IR-948 profile belongs to a Done ticket, so once IR-917 closes no ticket builds it. The 16 GiB sentence is unexercised (the IR-917 runs used a 24 GiB cgroup). Needs a status (PLANNED/PARTIAL) with a live owner, or a split so the delivered IR-917 profile is credited and the remainder owned | spec/kani/functional/FR-015-bounded-kani-obligations.md:943 |

## Verdict

The PR's own criteria are covered; the two findings are ownership gaps for the lead and planner
(pre-existing in part, but this PR is the one that closes IR-917 and so orphans them). They do
not block merging the code if the lead records owners in this PR's spec rows or files the
tickets before IR-917 closes. Checked with no gap: FR-014-AC-54 refusal cases (out of range,
missing target, non-literal, Boolean, empty, unparseable, other `convert` identity) each have a
test and leave a sibling unchanged; the QSL-authored `self.balance = 0` case asserts that the
emitted package really carries `quire.op.numeric.narrow`; the matrix rows added to
`spec/kani/matrix/tests.md` and `spec/oracle/matrix/tests.md` agree with the computed tags; the
PR adds no quire warning (it removes one `ears:unclassifiable` on FR-017).

## Dispositions

Round 2, head b088fa31. Linear states at this pass: IR-948 Done, IR-1509 In Progress, QSL-1899 In
Progress, IR-1528 In Progress.

| FND | outcome | reason |
|---|---|---|
| FND-001 | fixed | FR-015-AC-119 now records FR-025-AC-11 to AC-13 as implemented (the FR-025 rows read Implemented, backed by the new structural tests and the two tc_036 installed runs) and gives the pipeline-compiled subject the owner chain IR-1509 behind QSL-1899 behind IR-1528, all In Progress |
| FND-002 | deferred | planner ruling 2026-11-12. AC-126 is PARTIAL, credits the IR-917 profile (installed controls through AC-151 and AC-60, impossible-precondition control `CoverUnsatisfied`), and leaves the IR-948 profiles PLANNED under IR-948, closing with the last of the two owners. Follow-up for the lead: IR-948 is Done in Linear, so it must be reopened (or the profiles moved to an open ticket), or the row is parked under a closed ticket |
