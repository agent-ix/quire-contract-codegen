---
id: SR-002
title: "Base spec review of IR-295"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen@44afebc0f9e1e042447536703761368f873e2bde; FR-019-AC-25 and TC-030 form-aware settlement procedure"
review_set: subset
---

## Summary

Reviewed the changed FR-019 behavior and acceptance criterion, the new TC-030 procedure, criterion identifiers, and the computed Test Matrix. The new criterion is not included in the parsed acceptance-criteria table or the routed Test Matrix.

## Verdict

**CONDITIONAL** — FR-019-AC-25 must be part of the computed criteria and trace plan before its new obligation can be verified.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-019-AC-25 is separated from the acceptance-criteria table by a blank line, so Quire omits it from the computed criteria; the routed Test Matrix also has no AC-25 mapping to TC-030. | spec/routed/functional/FR-019-capability-settlement.md:309; spec/routed/matrix/tests.md:13-26 |

The computed matrix for this SHA lists FR-019-AC-24 as the last criterion and does not include AC-25. The TC-030 procedure adds relevant cases, but the corresponding FR-019 row in `spec/routed/matrix/tests.md` does not yet name AC-25. Remove the separating blank line and add the criterion-to-TC mapping so the new obligation participates in matrix and trace review.
