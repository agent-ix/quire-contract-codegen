---
id: SR-8141
title: "IR-550 integrity spec review"
type: SpecReview
analysis: integrity
scope: "CG IR-550 six-file spec diff 91d594387257dbe0e6bbf2c69cfb88f58d6e118f..9148d5b5a5d99350d2d5becba4761a2dd331c4d3"
review_set: subset
---

## Summary

The direct-item and body-root legs of FR-021-AC-33 are distinct routes. The latter currently lacks the precondition needed to isolate a declaration-body refusal.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-021-AC-33 says every item naming a function with a formula/fairness/case body root receives that body's typed `Unsupported`, while preserving existing lowering precedence. TC-031 step 13 does not require the body-root leg's item to name a **distinct, admitted, successfully lowered `expression/call` node** with no duplicate or earlier refusal. Current `item_disposition` lowers the item's call node and rejects a non-`call` form before resolving the named declaration and propagating Stage 1's body refusal. Reusing the direct unsupported root as the item's call ID, or using an invalid call node, can return the call-node refusal or `FormMismatch` without ever testing body refusal. State this precondition in AC-33/TC-031 and compare the result to the body's IR lowering record, preserving the existing earlier-failure precedence. | spec/oracle/functional/FR-021-function-application-oracles.md:262; spec/oracle/matrix/TC-031-function-application-oracles.md:121-136; src/oracle/function/mod.rs:909-945, :1322-1345 |

## Final disposition

Rechecked exact local fix `47fe2d2afca6a3190478ed09adc5bfc32a65b1b3` against reviewed head `9148d5b5a5d99350d2d5becba4761a2dd331c4d3`. FND-001 **FIXED**. FR-021-AC-33 now requires the separate body-root item to name a distinct, IR-admitted `expression`/`call` node that lowers successfully and has no duplicate, capability, or earlier item refusal. TC-031 step 13 requires the same precondition and compares the refusal cause to the body-root IR function-profile lowering record. This reaches the declaration Stage 1 result through the existing item-disposition order without changing earlier refusal precedence. No new issue appears in the two changed spec files.
