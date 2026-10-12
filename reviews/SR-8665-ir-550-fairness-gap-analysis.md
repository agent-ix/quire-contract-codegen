---
id: SR-8665
title: "IR-550 fairness follow-up diff-scoped gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen@0219c9791b3b8552c3fb06f39655128bdc50c8f8; PR-preparation diff; FR-021-AC-33, TC-031 step 13; changed fixture helper and integration test"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-021", type: references }
---

## Summary

This assigned diff-scoped reconciliation checks FR-021-AC-33 against the computed Quire matrix, the changed integration test and fixture helper, and the public-reader admission path. The new fairness case closes the direct fairness coverage gap for this acceptance criterion; IR-550 remains open for work outside this reviewed slice.

## Verdict

**PASS** — the scoped AC-33 direct-root cases are trace-bound and the fairness test exercises a reader-admitted fixture with observable refusal and sibling invariance.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No gaps found in the reviewed slice | - |

## Coverage

- Reconciliation: `quire matrix --scope . --format json` (Quire 0.36.2, engine 0.50.2); interpreted for FR-021-AC-33 only; no run evidence read.
- Plan completion: not assessed
- Criteria: tagged 1 · untagged 0 · tagged-by-ignored-test 0 · method-without-symbol 0 (reviewed subset only)
- Evidence: no run evidence
- Untraced behaviors / stubs: no new production behavior in this diff; test fixture support is exercised by the new integration test
- Semantic review: skipped (not requested)

## Reviewed Units

- **FR-021-AC-33 (examined):** Direct admitted formula, fairness and case roots retain request identity and typed `Unsupported`; no affected schedule/symbol/location or panic; healthy output remains byte-identical; declared-body refusal and precedence remain unchanged.
- **TC-031 step 13 (examined):** Public V2 reader admission for formula, fairness and case; direct call-node requests; claim/cause identity; absence of generated artifacts; healthy-sibling invariance.

The computed matrix reports FR-021-AC-33 as tagged by `tc_031_ac33_call_claim_retains_identity_and_uses_declared_body_refusal`, `tc_031_ac33_direct_formula_call_node_is_unsupported`, `tc_031_ac33_direct_case_call_node_is_unsupported`, and the new `tc_031_ac33_direct_fairness_call_node_is_unsupported`. The changed fairness test uses `PackageBuilder::read_resolved` and asserts an admitted `temporal/fairness` node before invoking the generator.
