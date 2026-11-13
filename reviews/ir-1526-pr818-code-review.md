---
id: SR-17940486709564847047933
title: "IR-1526 PR #818 tractability corpus timed_out rows code review"
type: SpecReview
analysis: code-review
date: 2026-11-13
scope: "agent-ix/quire-contract-codegen; PR #818 @ c767df66; IR-1526; tests/fixtures/tractability/corpus.json, tests/fixtures/tractability/baseline.json, spec/kani/functional/FR-056-kani-tractability-refusal.md, spec/kani/matrix/TC-070-kani-tractability-refusal.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-056
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-070
    type: references
---

## Summary

Ticket: IR-1526. Code review of PR #818, scoped to `git diff origin/main...HEAD` (4 files,
+62/-7, no Rust source changed, so the rust-review lane did not run). The PR adds two
`timed_out` corpus rows taken from `docs/qualification/ir1530-general-route-scalar-not-tractable.md`:
the general-route (`rt::Integer`) scalar harnesses `x + 1` over `Int[0, 9]` (add, 4 bits) and
`x - y` over two `Int[0, 9]` (sub, 4 bits). It moves the baseline from false-accept 0 to 2 and
rewrites the AC-10 Status in FR-056 and TC-070.

Focused run: `cargo test --test it kani_tractability` through `locked-build.sh` with a throwaway
target dir: 16 passed; the scoring test printed "12 rows: false-refuse 0 (baseline 0),
false-accept 2 (baseline 2)". So the baseline is the empty-table decider's real score.

## Verdict

**NOT MERGEABLE as is** (one high finding). The rows are not V2-clause measurements. FR-056 is
scoped to V2 clauses (title; Description: "it sees only operations the V2 clause path accepts").
AC-10 requires the corpus to hold "clauses ... with MEASURED ground truth". The decider never
sees a general-route harness. The same feature vector on the native route verified in 7 s
(IR-1530 doc, `kani_generation_real_v2_scalar_verifies`). So the rows record a ground truth that
is false for the decider's own domain. PR #814's review removed the earlier ir304 row for the same
reason (reviews/ir-1526-gap-analysis.md FND-003, disposition "the mislabelled non-V2 `timed_out`
row is removed"), and this PR adds it back in a new form. Recommendation: drop both rows,
restore baseline `{0, 0}`, and keep the Status note that CG has no measured `timed_out` V2 clause.
The IR-1530 doc can be cited there as out-of-domain evidence. Relabelling the rows with an
out-of-scope marker that the scoring test skips is not recommended: it would add a schema field
and a test branch for rows that no decider is scored on, so it fails the value test.

Clean parts: the baseline count of 0/2 over 12 rows is correct per `score()`. The AC-10 and AC-11
Status stays PARTIAL and says plainly that always-Run equals the baseline on the shipped file.
The JSON parses and the features match the doc's operations, bounds and widths (9 needs 4 bits).
No row comes from another repository or the titan proofs lane: every new `measurement_source`
cites this repo's own IR-1530 doc.

## Findings

| ID      | Severity | Summary | Refs |
| ------- | -------- | ------- | ---- |
| FND-001 | high     | FR-056-AC-10: the two new rows are general-route rt::Integer scalar harnesses, not V2 clauses, so the decider can never be handed them. Their features (add or sub at 4 bits, no route) match native-route harnesses that verify in seconds, so their timed_out ground truth is false for the decider's domain. This bakes a false-accept of 2 into the baseline that a correct V2 decider cannot reduce, and it rewards a later rules decider that refuses small add/sub. It reverses PR #814's FND-003 disposition, which removed the ir304 row for this reason. Drop both rows and restore baseline {0,0}. | tests/fixtures/tractability/corpus.json:372-417, tests/fixtures/tractability/baseline.json:1 |
| FND-002 | medium   | FR-056 Status is factually wrong: "A table row refusing these operations at 4 bits would refuse tractable native-route clauses, a false refuse the corpus does not yet carry." Under the Lookup rule (row bits <= width) an add row at 4 bits also refuses the ten IR-1022 rows (add at 10 bits, verified or falsified), so the shipped corpus already scores that as 10 false refuses. | spec/kani/functional/FR-056-kani-tractability-refusal.md:141-142 |
| FND-003 | low      | Provenance is overstated. The Status says the rows are "from IR-1530's own measurement", but the IR-1530 doc says "These base timeouts were not re-run by the IR-1530 coder". The add timeout comes from ir304-kani-guard-native-harness.md and the reporter/reviewer runs, and the sub timeout from an IR-1530 review run of tc_027_attested_scalar_subtraction_receipts_bind_both_operand_orders at 600.33 s. The rows cite only the doc's summary table and not the test or run. The sub row also stores wall_seconds 600.0 while its source text says 600.33 s. Moot if FND-001 drops the rows. | tests/fixtures/tractability/corpus.json:390-391,415-416, spec/kani/functional/FR-056-kani-tractability-refusal.md:135 |
