---
id: SR-17940486709564847047694
title: "IR-1519 PR #793 status and evidence edits: spec review (base)"
type: SpecReview
analysis: base
date: 2026-11-11
scope: "agent-ix/quire-contract-codegen PR #793 at 48e18b45; IR-1519; spec/replay/functional/FR-024-counterexample-envelope-intake.md, spec/replay/matrix/TC-035-counterexample-envelope-intake.md, spec/replay/matrix/tests.md, spec/core/functional/interface-001-codegen-api.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-024
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-035
    type: reviews
---

## Summary

Ticket: IR-1519. This review covers the four spec edits in the PR. All of them are status and evidence edits; no criterion text changes.

1. FR-024-AC-65 and FR-024-AC-66 change from `PLANNED (IR-1519)` to `Delivered (IR-1519)`. The criterion bodies are byte-identical to the spec merged as CG #790.
2. The FR-024 status paragraph replaces "the caller's obligation identity is not checked against the harness: IR-1516" with the AC-65/66 check at `decode_falsification`.
3. The TC-035 status paragraph marks step 46 implemented.
4. In spec/replay/matrix/tests.md, the AC-65/66 row changes from Planned to Covered (IR-1519).
5. In interface-001, the `decode_falsification` semantics change "planned IR-1519" to the AC-66 citation. That entry's error list already names `ObligationIdentityMismatch`.

## Gates

`quire validate --scope . 'spec/**/*.md' 'plan/**/*.md' 'reviews/**/*.md'` exits 0 on the PR. It gives the same 6 grammar warnings as origin/main, with no diff between the two warning sets, so the PR adds no new warning. `quire matrix --scope .` lists FR-024-AC-65 at tests/it/state_clause_invariant_replay.rs:1107 and AC-66 at :1044, both tagged.

## Verdict

**MERGEABLE; clean.** Each status claim matches the code and the tagged tests. The tests.md row's itemised coverage ("an ordinal-only difference, each other member alone, identity before field order, kind before identity, CG-defect settlement, and an equal identity carried member for member into packet and request") is accurate for the test file it cites. The "kind before identity" ordering is pinned by the file's AC-62-tagged test, not by the AC-65 test's frame case; that is recorded as a test-intent finding in the code-review SR, not as a spec defect.

On interface-001: the coder left the `StateClauseReplayError` lists as they were on main. I accept that. The `decode_falsification` entry this PR touches is complete for that operation. The gaps are elsewhere: the `new_invariant` and `new` entries do not list `RawPackage` or `ProofBound`. Those gaps predate this PR, do not involve the new variant, and belong to a separate interface sweep.

The provenance placement is consistent with the spec. AC-65 orders only NotInvariantHarness, then identity, then HarnessMismatch. A harness whose sealed `obligation` was edited is a provenance failure (`Decode`), which AC-65 does not cover. AC-62's closed set also does not name that provenance refusal; that was true before this PR and this PR does not change it.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-024-AC-65 | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:595 | Delivered (IR-1519). Prerequisite: FR-024-AC-62 and the `KaniV2ClauseObligationHarness.obligation` member of FR-015-AC-48 (IR-1507, merged). |
| FR-024-AC-66 | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:596 | Delivered (IR-1519). Prerequisite: FR-024-AC-65. |
| FR-024 status paragraph | examined | spec/replay/functional/FR-024-counterexample-envelope-intake.md:623 | (FR-024-AC-65 and FR-024-AC-66, IR-1519, check the caller's obligation identity against the harness's at `decode_falsification`) |
| TC-035 status | examined | spec/replay/matrix/TC-035-counterexample-envelope-intake.md:450 | Step 46 (FR-024-AC-65 and FR-024-AC-66, IR-1519) is implemented by the identity comparison in `InvariantReplayInputs::decode_falsification` |
| tests.md FR-024-AC-65/66 row | examined | spec/replay/matrix/tests.md:28 | ✅ Covered (IR-1519); step 46, the invariant path's obligation-identity cross-check in `InvariantReplayInputs::decode_falsification` |
| interface-001 decode_falsification | examined | spec/core/functional/interface-001-codegen-api.md:253-256 | whose obligation equals the inputs' `obligation_identity` (FR-024-AC-65, FR-024-AC-66) |
| FR-029-AC-16 | context_only | spec/kani/functional/FR-029-run-outcome-terminal-record.md:393 | The state-clause replay path (FR-024) settles as the other replay paths do |
