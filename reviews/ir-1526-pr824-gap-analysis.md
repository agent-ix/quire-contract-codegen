---
id: SR-17940486709564847047959
title: "IR-1526 PR #824 native-route Kani probe and corpus rows gap analysis"
type: SpecReview
analysis: gap-analysis
date: 2026-11-13
scope: "agent-ix/quire-contract-codegen; PR #824 @ 6c3387de; IR-1526; FR-056-AC-10, FR-056-AC-11, FR-056-AC-14, FR-056 Status, TC-070; tests/fixtures/tractability/, tests/it/kani_tractability.rs, tests/it/kani_obligations_v2_clause_controls.rs, tests/state_frame_support/native_twin.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-056
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-070
    type: references
---

## Summary

Ticket: IR-1526. This is a planless gap analysis of PR #824, scoped to
`git diff origin/main...HEAD`. Plan completion: not assessed. It covers the native-route
families of FR-056-AC-10, the AC-11 scoring over the 26-row corpus, the AC-14 route-keyed
lookup that the native rows now exercise, and the honesty of the FR-056 Status.

## Verdict

**CONDITIONAL**, with one medium finding.

- **AC-10.** The native-route add, multiply and chain families now have measured rows below
  15 bits (10) and at or above 15 bits (15, 23, 31). The Status keeps AC-10 PARTIAL and names
  what is still missing: Meter, decimal, pure-relational and non-integer multi-operation
  clauses. It also says that two-parameter shapes cannot render natively.
- **AC-11.** Over 26 rows the score is 0 false-refuse and 0 false-accept against a 0/0
  baseline. The refuse-all and run-all mutants still regress.
- **AC-14.** The new rows add a corpus-level case to the existing tagged tests: with the
  shipped general-route table, the decider returns `Run` for every native row.
- **No reverse gap.** The new helpers and probes belong to FR-056-AC-10 and TC-070 step 10.
- **No stubs.** The print-only test is FND-006 of the code review.

## Findings

| ID      | Severity | Summary | Refs |
| ------- | -------- | ------- | ---- |
| FND-001 | medium   | FR-056-AC-10 cannot be met as written. It requires a native-route "two-operation chain (`other * offset + 3 <= 100` shape)", and the Status itself says the native route renders no two-parameter clause. The PR measures `amount * amount + 3 <= 100`, a single operand squared then added, which is a different shape. The AC text was not amended, so it is unclear whether the chain family is met or stays permanently missing. Decide which shape the AC requires on the native route, and amend the AC, or record the family as unrenderable in the AC itself. | spec/kani/functional/FR-056-kani-tractability-refusal.md:135, spec/kani/functional/FR-056-kani-tractability-refusal.md:188-190 |

## Coverage

- Criteria examined: FR-056-AC-10, FR-056-AC-11 and FR-056-AC-14, plus TC-070 steps 7a, 10
  and 11. FR-056-AC-7 and AC-15 were read as context only.
- `quire trace --id FR-056-AC-10` resolves four structural claims, including the new
  `tc_070_the_native_probe_rows_carry_the_decider_features_and_are_not_refused`.
- Matrix: `quire matrix --scope . --format json` (quire 0.44.0, engine 0.59.1) reports every
  FR-056 Test criterion `untagged` even though `quire trace` resolves their `#[trace]` claims.
  This is a tool discrepancy, not a PR finding. It already appears on `main` and was recorded
  in the PR #823 review.
- Measurement fidelity: all twelve rows match the probe logs (see the code-review SR).
- Semantic review: skipped (not requested).
- Plan completion: not assessed
