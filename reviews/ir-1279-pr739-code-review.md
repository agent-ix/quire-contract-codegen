---
id: SR-2026110283853
title: "IR-1279 PR 739 temporal occurrence relock code review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen PR #739 at 16b2ae165cbcae338b797291a957bc63416c7319; Cargo.lock; changed src/ and tests/ files"
review_set: subset
---

## Summary

Reviewed the complete 47-file diff, focusing on the IR/QSL lock cohort, temporal occurrence and name sidecars, source and document identity changes, fixture construction, and existing replay and equality callers. No applicable AssuranceProfile or CI workflow change was found. The Rust idiom and boundary checks found no new defect or copied upstream artifact.

## Verdict

**PASS.** The changed production encoders preserve both temporal sidecars, and the revised fixture admits the paired occurrence and name. Final aggregate gates belong to the code lead.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

The review covered all changed `src/` and `tests/` paths listed by `git diff --name-status origin/main...HEAD`, plus `Cargo.lock`, repository instructions and Rust lint settings. The new fixture assertion reads the claim occurrence from its typed graph row; `TemporalBinding` and `TemporalClauseName` share the same declaration and occurrence, and the admitted package test checks those values. The three changed production `CheckedPackageSidecarsRef` sites pass `temporal_clause_names()` alongside `temporal_bindings()`. QSL's changed source/document identities are carried as two labels through `LockedSource`, the replay request, state-clause document labels, and corresponding tests. Existing focused checks were reported by the coder at this head; this reviewer did not repeat them or run the aggregate gate.
