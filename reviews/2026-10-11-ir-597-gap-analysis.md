---
id: SR-8661
title: "IR-597 typed operator catalog repository gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "quire-contract-codegen@39c4df68d9eb8626d1927c502772d76360c34a65; spec/, src/, tests/"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-038", type: references }
---

## Summary

Ran the planless repository audit over the specification, source and tests at the reviewed commit. The computed matrix has substantial repository-wide traceability gaps; the changed FR-038 criteria have trace tags for their test methods, while a new assertion type mismatch is recorded in the companion code review.

## Verdict

**FAIL** — the computed matrix reports 318 untagged criteria and 11 criteria backed only by ignored tests.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The computed matrix reports 318 untagged criteria, including criteria outside this change; they have no bound test symbol | 318 untagged criteria in quoin matrix at the reviewed scope |
| FND-002 | high | Eleven criteria are tagged only by ignored tests and therefore have no active test binder | FR-015-AC-30, FR-015-AC-31, FR-015-AC-32, FR-015-AC-56, FR-015-AC-57, FR-015-AC-80, FR-017-AC-7, FR-017-CON-1, FR-024-AC-18, FR-024-AC-34, FR-031-AC-20 |
| FND-003 | medium | Coverage reports 11 test symbols tagged with undeclared IDs | AD-004; src/core/identity.rs, src/kani/generate/scalar.rs, tests/it/layout.rs, tests/it/oracle_generation.rs, tests/it/scratch_crate.rs |

## Coverage

- Reconciliation: quoin matrix (quoin 0.28.3, quire 0.36.2, engine 0.50.2).
- Plan completion: not assessed
- Criteria: tagged 368 · untagged 318 · tagged-by-ignored-test 11 · method-without-symbol 41
- Evidence: no run evidence for all 738 criteria; no execution evidence was assessed.
- Coverage diagnostics: 13 declarations/verification-method diagnostics were reported by `quire coverage`; 11 untracked test symbols were reported. The untracked tags are pre-existing AD-004/FR-322 identifiers and not introduced by the changed files.
- Changed criterion bindings: FR-038-AC-1 and AC-5 are `method-without-symbol` (Inspection); AC-2, AC-3, AC-4 and AC-6 are tagged. AC-3's binders include catalog, oracle and Kani consumer tests.
- Reverse-gap inventory: the changed production surface adds one private module with internal operator metadata and updates three existing consumers; these behaviors are owned by FR-038 and the existing FR-014/FR-015 consumer requirements. No new public API, source stub, test stub, or unowned behavior was identified in the changed files.
- Delta applicability: rechecked commit `43af85923ec459bc56530f6907f8131b5ae08b7d` against the original reviewed commit. The two test-only deltas change expected identity data/fixture-ID resolution only; they change no specification criterion or trace tag. The original computed matrix assessment remains applicable; matrix was not rerun.
- Semantic review: skipped (not requested).
