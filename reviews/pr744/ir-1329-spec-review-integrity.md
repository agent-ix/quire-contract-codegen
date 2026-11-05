---
id: SR-2026110283863
title: "IR-1329 PR 744 spec-review/integrity"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen; spec/oracle/functional/FR-021-function-application-oracles.md, src/oracle/function/owner_render.rs, tests/exact_function_support/agreement_cases.rs, tests/exact_function_support/invariant_cases.rs"
review_set: subset
---

## Summary

Ticket: IR-1329. Reviewed frozen PR #744 head 6c250a70. Scope: FR-021-AC-43 examined; FR-021-AC-40 and its body-forwarding clause read as context.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | The new stopped-body location rule conflicts with the adjacent statement that Undefined, Refused and Incomplete forward their locations unchanged; specify that payload and charges remain unchanged while body-local stops acquire the checked result location. | spec/oracle/functional/FR-021-function-application-oracles.md:425-436 | wrong-requirement |

## Verdict

CONDITIONAL. The new rule is concrete, but the adjacent forwarding language conflicts with its location assignment.
