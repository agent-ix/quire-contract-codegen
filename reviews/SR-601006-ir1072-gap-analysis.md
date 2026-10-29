---
id: SR-601006
title: "IR-1072 PR 607 gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; src/lib.rs, src/strategy/mod.rs, src/strategy/v2_selected_claim.rs, tests/it/main.rs, tests/it/v2_selected_claim_entry.rs; spec/strategy/functional/FR-011-numeric-harness-campaigns.md, spec/strategy/matrix/TC-020-numeric-harness-campaigns.md"
review_set: subset
---

## Summary

Reviewed PR #607 at 55637acf891ca18422105948599990ffbf4bf4e7 against FR-011-AC-13. The computed Quoin matrix marks the criterion tagged by tests/it/v2_selected_claim_entry.rs::v2_selected_claim_entry_keeps_point_and_exact_refusal_key at line 17. The test uses QSL-produced, IR-admitted clauses in one package, checks distinct exact points, then changes the occurrence role, occurrence ordinal, and owner separately and checks all four retained IR diagnostic fields. The source has no stub or duplicate selector; the only new production path delegates directly to Contract IR. Plan completion: not assessed.

## Scope

- examined: src/lib.rs and src/strategy/mod.rs public export and module wiring.
- examined: src/strategy/v2_selected_claim.rs typed request, key, accessor call, diagnostic.
- examined: tests/it/main.rs and tests/it/v2_selected_claim_entry.rs fixture, trace and assertions.
- context_only: FR-011-AC-13, TC-020 step 12, Contract IR FR-058 accessor and V1 diagnostic convention.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |


## Coverage

- FR-011-AC-13: static_status tagged; one test binder. Its assertions can fail on wrong point or missing/changed IR refusal fields.
- Run evidence: unavailable because QSL-1190 is unmerged and the CG lock must await its merge. The matrix reports no run evidence; this is a gate dependency, not a missing test binding.
- Reverse gap: new request, key, and diagnostic are owned by FR-011-AC-13; no new behavior lacks an owning criterion.
- Plan completion: not assessed.

## Verdict

PASS for static traceability and source/test correspondence within the PR diff. Runtime discharge is pending the QSL-1190 dependency and final head gate.

## Final-head recheck

At rebased PR #607 head `fed57257c8b9d8e4bee0300cd18268e05b33d5c5` against CG main `218e4cdec8799fa48526c4a4ca623063fb7cccaf`, the source/test patch has the same stable patch-id `5ef5bba71fa11ca5be21dd13c46c28971be66e2f` as reviewed head `55637acf891ca18422105948599990ffbf4bf4e7`. The PR changes no Cargo manifest, lockfile, CI workflow, or specification relative to current main. Current CG main already pins QSL `c8584d28a20b584a43b61a3e40ffeeaff8edfb48` and Contract IR `a97babf051510a35f9fea27e67ab07784aec8edd`. The pinned Contract IR accessor still returns `ExecutionPoint` and preserves `CheckedExecutionPointError`'s typed code, clause, owner, and occurrence. No new static finding arose from the rebase. The earlier QSL-unmerged note was historical; final-head focused execution and aggregate gate evidence remain pending, so this review does not claim a runtime pass.
