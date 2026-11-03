---
id: SR-2026110283850
title: "IR-1285 PR 736 scoped gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; tests/it/kani_obligations_state_frame.rs; spec/oracle/functional/FR-014-exact-scalar-oracles.md; IR-1285 PR #736"
review_set: subset
---

## Summary

Ticket: IR-1285; PR #736 at `ee52766fa529af2cb22e0309b03fa57d159b1cfb`. Planless, repository-driven matrix check plus scoped reverse-gap and assertion review found no gap introduced by the test-infrastructure diff. Plan completion: not assessed.

## Scope

- examined: four FR-014-AC-38-tagged tests in `tests/it/kani_obligations_state_frame.rs`: `qsl_emitted_state_clause_derives_a_confirmed_field_backed_oracle`, `admitted_inline_clause_keeps_the_clause_identity`, `qsl_bounded_operation_parameter_becomes_a_clause_input`, and `qsl_emitted_clause_composes_boolean_and_integer_operations`.
- context_only: FR-014-AC-38 in `spec/oracle/functional/FR-014-exact-scalar-oracles.md`, shared-target helper `tests/it/scratch_crate.rs:20-34`, and IR-1285 AC1-AC5.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

- Reconciliation: `quoin matrix --repo . --json` (quoin 0.29.1); 1,029 criteria: 610 tagged, 314 untagged, 12 tagged only by ignored tests, 93 method-without-symbol. No run evidence was bound in the matrix. Existing repository gaps are outside this PR's four-test diff.
- FR-014-AC-38 is tagged with 13 binders, including the four changed tests at lines 91, 260, 351, and 1936. Their actual generated source and result assertions remain unchanged; only the build target and mutex lifetime changed. A broken oracle value would still fail the child tests.
- Reverse gap: no new production behavior, public API, or unowned test path is introduced. The changed test execution mechanism supports IR-1285 AC2 and leaves AC3 for final runtime acceptance.
- Plan completion: not assessed. Optional semantic review was not selected; the scoped assertion comparison above does not claim repository-wide semantic agreement.

## Verdict

**PASS for traceability and assertion preservation in the reviewed diff.** This is not a whole-repository gap clearance or an acceptance of the pending IR-1285 time budget.
