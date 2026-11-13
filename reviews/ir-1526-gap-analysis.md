---
id: SR-17940486709564847047902
title: "IR-1526 PR #814 FR-056 gap analysis"
type: SpecReview
analysis: gap-analysis
date: 2026-11-13
scope: "agent-ix/quire-contract-codegen; PR #814 @ e02f9053; IR-1526; FR-056-AC-1 to FR-056-AC-13, TC-070, the PR diff's source and tests"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-056
    type: references
  - target: ix://agent-ix/quire-contract-codegen/TC-070
    type: references
---

## Summary

Ticket: IR-1526. Planless gap analysis of FR-056 and TC-070 against the PR #814 diff.
`quire trace --id` resolves a `#[trace]` test claim for every Test-method AC. AC-1 and AC-10
are Inspection ACs and are also tagged. AC-13 is bound only by an `#[ignore]`d kani-lane test
and is honestly reported as unrun. Each AC was checked to see whether its test can fail on the
AC's named mutant. Four ACs are weaker than claimed, and one spec rule looks inverted.

## Verdict

**CONDITIONAL** - medium findings only, no untagged criterion once the matrix/trace mismatch in
FND-005 is accounted for. Clean units (tests whose mutants fail): AC-1, AC-2, AC-3, AC-5
(decider level, plus 21-bit through generation), AC-6, AC-7, AC-8, AC-9 and AC-12. AC-13 is
correctly left unrun and the Status says so. No reverse gap: every new public item traces to
FR-056.

## Findings

| ID      | Severity | Summary | Refs |
| ------- | -------- | ------- | ---- |
| FND-001 | medium   | FR-056-AC-4: the case "an operator the V2 path refuses keeps V2ClauseOperatorNotSupported" cannot fail on the ordering mutant. The division clause `other mod offset > 0` has no multiply and the table holds only a multiply row, so a decider run before the operator check would still return Run. The test needs a clause that has both a refused operator and a qualifying operation, or a row for the refused operator. | tests/it/kani_tractability.rs:341-375 |
| FND-002 | medium   | FR-056-AC-11: with the committed corpus and baseline, the "run-everything fails on false-accept" mutant passes. The decider being scored is `TractabilityTable::empty()`, which behaves exactly like always-Run, and `baseline.json` {0,1} is always-Run's own score. The mutants fail only on a two-row synthetic corpus against a zero baseline, so the real gate cannot tell the current decider apart from run-everything. | tests/it/kani_tractability.rs:686-731, tests/fixtures/tractability/baseline.json |
| FND-003 | medium   | FR-056-AC-10: the corpus is missing required seed families. There are no probe rows, no Meter case and no decimal case, and 10 of the 11 rows share one IR-1022 feature vector. The only `timed_out` row (ir304) says itself that it is not a V2 clause and that its features (lt/add at 4 bits) do not carry the cost. A decider that clears that false accept would have to refuse 4-bit lt/add. The AC-10 test checks fields, not seed families (TC-070 step 10). The Status line still claims AC-10 IMPLEMENTED. | tests/fixtures/tractability/corpus.json, tests/it/kani_tractability.rs:656-675, spec/kani/functional/FR-056-kani-tractability-refusal.md:130 |
| FND-004 | medium   | FR-056 Lookup (spec soundness; the code follows the spec): a `not_tractable` row at `max_operand_bits` B refuses every width up to B and none above it. So a timed-out 64-bit multiply probe refuses a 21-bit multiply (the AC-13 test asserts exactly this) and lets a 65-bit one through. Cost grows with width, so the comparison looks inverted. This needs a planner/spec ruling. | spec/kani/functional/FR-056-kani-tractability-refusal.md:94-99, tests/it/kani_obligations_v2_clause_controls.rs:960-975 |
| FND-005 | low      | Tooling: `quire matrix` (quire 0.44.0) lists FR-056-AC-2..9 and 11..13 as `untagged` with no binders, while `quire trace --id FR-056-AC-4` resolves the `#[trace]` claim at tests/it/kani_tractability.rs:332. The same mismatch exists on `main` for FR-040..FR-055 (FR-049-AC-1 is tagged in src/routed/contract_package_tests.rs). This predates the PR and is not a PR defect, but the matrix gate cannot confirm coverage here. | spec/kani/functional/FR-056-kani-tractability-refusal.md |

## Coverage

- Criteria examined: 13 (FR-056-AC-1 to AC-13). All have a trace claim per `quire trace`.
  AC-13 is bound only by an ignored kani-lane test and was not run.
- The matrix computes `untagged` for the Test-method ACs. See FND-005 for why that conflicts
  with `quire trace`.
- Reverse gaps: none. `interval_bits`, `derive_v2_clause_operation_widths`,
  `negotiate_kani_obligations_with_options`, `GenerationOptions` and the table types all serve
  FR-056.
- Stubs: none.
- Semantic review: skipped as a fan-out pass; the per-AC mutant checks above were done inline.
- Plan completion: not assessed

## Dispositions

Round 1, checked against `git diff e02f9053..3d070958`.

| FND | outcome | reason |
| --- | ------- | ------ |
| FND-001 | fixed | The operator-refused clause is now `other * offset > 0 and other mod offset > 0` at `Int[0, 2^20]`, and the row covers the 21-bit multiply. A decider placed before the operator check would report `V2ClauseNotTractable` and fail the assertion. |
| FND-002 | deferred | once the mislabelled ir304 row is gone, CG's own evidence holds no measured wall-clock timeout of a V2 clause. Inventing a `timed_out` row would break AC-10 ("timed_out only if a run was measured"). The synthetic-corpus checks still show that the scoring fails both trivial deciders, and the FR-056 Status now says the shipped file does not exercise false-accept. The residue closes only with a measured timed-out V2 clause. The AC-13 run records a Verified/Falsified row, not a timeout, so it does not close this. |
| FND-003 | deferred | the mislabelled non-V2 `timed_out` row is removed, and FR-056/TC-070 Status now say PARTIAL for AC-10 and name the missing families (probe, Meter, decimal, other multi-operation). Those rows need new installed-Kani measurements, which a host-only PR cannot supply. |
| FND-004 | fixed | The spec Lookup is amended to "max_operand_bits <= width" (a row at B bits is intractable at B and wider), and AC-4, AC-5, AC-13 and TC-070 steps 4, 5 and 13 are updated to match. `decide` uses `bits >= row.max_operand_bits`, and the tests assert that 33 bits runs while 64 and 65 bits are refused. |
| FND-005 | deferred | quire tooling behaviour that predates the PR (`quire matrix` against `quire trace`, also on main for FR-040..FR-055). It is not a defect of this PR and goes to quoin/quire triage. |
