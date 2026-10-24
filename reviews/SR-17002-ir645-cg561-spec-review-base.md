---
id: SR-17002
title: "IR-645 CG #561 FR-030 and TC-041 spec review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen#561; spec/kani/functional/FR-030-ir-outcome-terminal-map.md; spec/kani/matrix/TC-041-ir-outcome-terminal-map.md"
review_set: subset
---

## Summary

Ticket: IR-645. Reviewed the FR-030 and TC-041 amendments at exact head `0a6d44b71da57f51b9216305655a648096e22cf6`. They distinguish IR's retained positive proof count from a vacuous Inconclusive and specify a typed overflow refusal. The test procedure names the same cases and keeps the prior settlement controls.

## Examined criteria

- FR-030-AC-1 (examined): total mapping over representable valid pairs and typed overflow refusal.
- FR-030-AC-4 (examined): retained count and refusal cases.
- FR-030-AC-5 (examined): vacuous Inconclusive and invalid-cause refusal.
- FR-030-AC-8 (examined): admitted Unavailable causes and invalid-cause refusal.
- FR-030-AC-18 (examined): proof basis and certification.
- TC-041 (examined): updated procedure and expected results.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

**PASS** for the changed specification text. The implementation and evidence defects are recorded independently in SR-17000 and SR-17001.

## Narrow delta review

**PASS** at exact head `b7dedc2776608b5f84941af3ff609d9a2b78433a` for the new FR-015-AC-52/91/95 and TC-023 wording. The interim generator's successful case carries an expectation with one or zero predicted checks, its refusal remains typed, and only an FR-017 run supplies a proof verdict. AC-95 remains explicitly planned for final retirement, consistent with the current interim API.

## Review ID correction

This artifact supersedes the same-method IR-645 CG #561 reviewer record originally labeled with a colliding SR id. The finding and disposition substance is unchanged; only the review ids and cross-references were reissued.
