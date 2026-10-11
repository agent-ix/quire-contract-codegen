---
id: SR-004
title: "Integrity review of IR-295"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen@44afebc0f9e1e042447536703761368f873e2bde; FR-019-AC-25, FR-019 Inputs/Outputs/Behavior, TC-030 new procedure; FR-015 and QSL ADR-012 §7.2 as context"
review_set: subset
---

## Summary

Reviewed AC atomicity, falsifiability, typed-form boundaries, and the test procedure against FR-015 and QSL ADR-012 §7.2. AC-25 combines several independently verifiable settlement obligations and leaves the form-to-capability boundary implicit.

## Verdict

**CONDITIONAL** — split the compound criterion and name the governing typed-form admission set.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-019-AC-25 combines bounded admission, form refusal, refusal precedence, display/debug independence, unbounded requires-bound, and a subsequent bounded request in one criterion, obscuring independent verification and trace coverage. | spec/routed/functional/FR-019-capability-settlement.md:309 |
| FND-002 | medium | FR-019-AC-25 uses “admitted” and “outside Kani's capability” without naming the typed tag/form pairs or pointing to the FR-015 rule that defines the Kani-supported set, so the same request can be classified differently by independent implementers. | spec/routed/functional/FR-019-capability-settlement.md:55-59, 114-122, 309 |

## Failure-domain check

The new ingress is described as typed, in-process data and does not add a callback, external lookup, or graph operation. No additional failure-domain omission was identified beyond the undefined typed-form capability boundary in FND-002.


## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | medium | The newly defined Kani-supported set covers only V2 `state_clause` precondition, postcondition and invariant forms, leaving the ticket's complete-V1 IR-269 proof forms without a settlement rule. | spec/routed/functional/FR-019-capability-settlement.md:311; IR-295 acceptance |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | `256d7bf1c61863839f13a0f8cf15c20e98c03bd3`; after_excerpt: `For the V2 clause negotiation path, the Kani-supported typed IR tag/form is state/state_clause with operation.member.clause equal to precondition, postcondition or invariant (FR-015-AC-38); its body uses the operators and input domains defined by FR-015-AC-39 through FR-015-AC-42.` |
| FND-002 | fixed | `256d7bf1c61863839f13a0f8cf15c20e98c03bd3`; after_excerpt: `The Kani-supported typed IR tag/form is state/state_clause with operation.member.clause equal to precondition, postcondition or invariant (FR-015-AC-38); its body uses the operators and input domains defined by FR-015-AC-39 through FR-015-AC-42.` |
| FND-003 | still-open | This fix round defines only the V2 clause negotiation path; the IR-295 ticket requires form-aware settlement over the complete-V1 admitted proof set. No settlement mapping for the other MVP forms is specified. |
| FND-001 | fixed | `256d7bf1c61863839f13a0f8cf15c20e98c03bd3`; after_excerpt: `The Kani-supported typed IR tag/form is state/state_clause with operation.member.clause equal to precondition, postcondition or invariant (FR-015-AC-38); its body uses the operators and input domains defined by FR-015-AC-39 through FR-015-AC-42.` |
| FND-002 | fixed | `256d7bf1c61863839f13a0f8cf15c20e98c03bd3`; after_excerpt: `The Kani-supported typed IR tag/form is state/state_clause with operation.member.clause equal to precondition, postcondition or invariant (FR-015-AC-38); its body uses the operators and input domains defined by FR-015-AC-39 through FR-015-AC-42.` |
| FND-003 | still-open | The current AC explicitly excludes most IR-269 complete-V1 coverage rows from the Kani-supported set and says the complete-V1 family list is not an admission rule. IR-269 states those scalar families, composite equality, function claims, clause kinds, state/frames, and bounded collections/graphs are coverage rows that must have verifying and falsified harnesses. Current FR-015 itself labels substantial V2 and state/frame coverage planned; a partial current FR-015 implementation set does not supersede the ticket's complete-V1 acceptance. The ticket's actual admitted MVP forms still need a settlement mapping that preserves the V1 obligations, or the ticket/spec scope must be revised through its owner. |
