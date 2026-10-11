---
id: SR-8122
title: "IR-597 failure-domain spec review"
type: SpecReview
analysis: failure-domain
scope: "CG IR-597 three-file spec diff 91d594387257dbe0e6bbf2c69cfb88f58d6e118f..ac350b807e01c876088889da594f93ad0bac19da"
review_set: subset
---

## Summary

Checked unknown and unsupported identities, operand shape, mode, result form, independent native arithmetic and exclusions. A result-form mutation can evade the named adverse controls.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-2 requires applicable result forms and AC-3 names wrong identity, family, arity/order, symbol and callee tests, but neither names a wrong result-form or member-result control. The upstream catalog has fixed integer/boolean results and member-derived results (for example numeric.convert and quantity.convert). Require a distinguishing adverse case for a correct identity/arity with the wrong result form or wrong member target, so the typed entry cannot pass merely by selecting the right operation name. | spec/core/functional/FR-038-typed-operator-catalog.md:23-25, :56-57; quire-verification-contracts/contracts/checked-operation-catalog-v1.json:40, :50-61 (integer/boolean), operations numeric.convert and quantity.convert (result member) |

## Final disposition

Rechecked local fix `c3248d34827369510c06580af025737032aae784` against reviewed head `ac350b807e01c876088889da594f93ad0bac19da`. FND-001 **FIXED**. AC-3 line 70 now requires a correct-identity/arity case with wrong fixed result form or member-selected result target, including conversion. Current scalar.rs:861-886 refuses the wrong result form and check_parameters:1256-1327 checks bounded conversion targets. Mode disagreement remains the distinct caller_declared path of AC-2.
