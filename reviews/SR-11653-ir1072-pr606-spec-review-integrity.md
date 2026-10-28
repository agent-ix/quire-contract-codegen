---
id: SR-11653
title: "IR-1072 PR 606 specification integrity review"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen; spec/strategy/functional/FR-011-numeric-harness-campaigns.md"
review_set: subset
---

## Summary

Reviewed the new V2 selection requirement against FR-011's existing generation-refusal clauses at frozen head ab7cb945c3c2c8a0c32b058ae8a2055709568035. No applicable AssuranceProfile was found under spec/.

## Scope

- examined: FR-011 Inputs, Outputs and Behavior additions for IR-1072.
- examined: FR-011-AC-13.
- context_only: FR-011-AC-7, which still plans generation refusal mapping into StrategyErrorCode.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | New generator-wide refusal rule overlaps existing generation mapping and makes V1 enum widening ambiguous; scope it to the selected-claim entry | spec/strategy/functional/FR-011-numeric-harness-campaigns.md:91 |
| FND-002 | low | New Inputs wording has broken grammar and a transient current-API assertion that will go stale when IR-1072 lands | spec/strategy/functional/FR-011-numeric-harness-campaigns.md:58 |

## Verdict

CONDITIONAL. Make the new rule explicitly entry-local and repair the changed Inputs sentence.

## Dispositions

Round 1 reviewed fix head 0e2b18b81d7375af3970365d490b240b8f590c91.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | Behavior now says `When the IR-1072 V2 selected-claim entry is called` and confines the refusal diagnostic and no-V1-widening rule to `this entry`, leaving later generator mapping distinct. |
| FND-002 | fixed | Inputs now reads `QSL owns ClauseBinding; the caller supplies no separate point`, removing the broken and transient current-API clause. |
