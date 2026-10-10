---
id: SR-4897
title: "IR-496 PR 340 composite equality gap analysis"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@d8cb477ba86482e7cbf3a197be03b5a93bc65890; FR-018-AC-28..31, TC-029, src/oracle/equality/mod.rs, src/oracle/equality/resolution.rs, tests/it/composite_equality_generation.rs; changed production code and computed Test Matrix"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-018
    type: references
  - target: ix://agent-ix/quire-contract-codegen/TC-029
    type: references
---

## Summary

Ticket: IR-496. Plan completion: not assessed. The computed Test Matrix reports FR-018-AC-28, AC-29, AC-30 and AC-31 as tagged, with test functions at the reviewed head. The changed source has an owning criterion in this slice; no stub or inflated trace claim was found in the changed path. The repository-wide matrix has older unrelated gaps, so this verdict is confined to the PR diff and these four criteria.

## Verdict

**PASS for the reviewed slice.** All four criteria have meaningful test bindings and the changed behavior matches their stated scope. Quoin's optional repository-wide semantic review was not run; code-to-test intent was inspected manually for this PR diff. The computed matrix contains no run evidence for these criteria, and the independent review did not rerun the heavy gate.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

- FR-018-AC-28: tagged to `deep_acyclic_closure_generates_flat_compilable_source_on_a_small_stack` at `src/oracle/equality/mod.rs:2720`.
- FR-018-AC-29: tagged to `cycle_refusal_follows_its_charged_entry_and_work_refusal_wins_at_the_boundary` at `src/oracle/equality/mod.rs:2583`.
- FR-018-AC-30: tagged to `source_limit_refuses_before_a_function_or_module_fragment_is_appended` at `src/oracle/equality/mod.rs:2693` and the deep test at line 2720.
- FR-018-AC-31: tagged to `maximum_work_setting_refuses_and_last_denied_count_is_representable` at `src/oracle/equality/mod.rs:2671` and `invalid_type_resolution_work_limit_refuses_before_lowering` at `tests/it/composite_equality_generation.rs:141`.
- Reverse gap check: the new public limits, typed errors, source modules and iterative rendering are owned by AC-28 through AC-31 and the existing FR-018 generator contract. No plan was selected.
