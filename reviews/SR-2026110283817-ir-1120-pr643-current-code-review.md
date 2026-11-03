---
id: SR-2026110283817
title: "IR-1120 PR 643 current-head code and Rust review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen PR #643 head 92262c6b; Cargo.lock, src/routed/smt/mod.rs, src/routed/smt/refinement.rs, src/routed/smt/refinement_source_tests.rs, and added reviews/ SR records"
review_set: subset
---

## Summary

Reviewed the rebased proof-output framing, typed Base/Step records, association checks, serialization, and source-authored tests. The changed Rust paths show no concrete behavioral defect in this scope. Prior review records added by this PR still carry stale `still-open` outcomes at the current head.

## Verdict

**CONDITIONAL** — reconcile the prior review dispositions before treating the PR as reviewed for merge. FR-314 live readback remains an explicitly partial upstream dependency (IR-1287).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Prior SR dispositions in this PR still mark the now-corrected seven-row test and status edits `still-open`; append current-head outcomes so the review record matches the code and spec that would merge. | reviews/SR-1121021-ir-1120-pr643-code-review.md:82; reviews/SR-1121022-ir-1120-pr643-gap-analysis.md:92-96; reviews/SR-1121024-ir-1120-pr643-spec-review-base.md:77-80; reviews/SR-1121029-ir-1120-pr643-blocker-status-spec-review.md:99-100 |

## Evidence

- `src/routed/smt/refinement_source_tests.rs:285-319` now asserts pairwise-distinct row queries and each named row's permutation; `:370-382` adds source-authored zero-step. `src/routed/smt/refinement.rs:187-265` checks count, roles, attempts, association, status, process completion, proof presence and captured-byte provenance before `:276-302` serializes.
- `src/routed/smt/mod.rs:229-312` keeps generic status parsing distinct from proof extraction; the changed parser retains a status-only `success` tail while refusing malformed proof framing in the extractor. No CI workflow or gate target changed in this PR.
- Coder reported focused refinement 7/7, empty-response 1/1, scoped Clippy/format/diff and targeted Quire validation at `92262c6b`. This reviewer did not rerun Cargo or the aggregate gate; final-head gate belongs to the lead after review.
- The prior `still-open` rows are findings about the PR's own added review records, not a demand to claim AC-5 complete. The current FR-045 Status explicitly keeps production FR-315 driver, FR-315 refusal, and live QSL readback open.

Plan completion: not assessed.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | At PR head `be595f7f`, the four prior SR files append canonical current-head disposition rows inside their existing `## Dispositions` tables. SR-1121021 FND-002 and SR-1121022 FND-001 now say fixed with the seven-row source evidence; SR-1121022 FND-002 defers live FR-314 readback to IR-1287 and FND-004 defers FR-315 refusal under IR-1120; SR-1121024 and SR-1121029 fix their status findings. Original findings and older outcomes remain as historical rows. |
