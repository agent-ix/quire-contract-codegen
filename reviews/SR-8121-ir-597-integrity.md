---
id: SR-8121
title: "IR-597 integrity spec review"
type: SpecReview
analysis: integrity
scope: "CG IR-597 three-file spec diff 91d594387257dbe0e6bbf2c69cfb88f58d6e118f..ac350b807e01c876088889da594f93ad0bac19da"
review_set: subset
---

## Summary

Compared the new source-inventory promise with FR-014’s already-specified derivable set and the current implementation. A direct cross-requirement conflict remains.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-038-AC-1 calls integer eq/ne state/frame-only until another V2 consumer is specified, and AC-2 omits Boolean from its listed families. FR-014 already specifies exact-scalar derivation of integer eq/ne and Boolean and/or/not/implies/eq/ne. Current ExactScalarOperation lacks those variants, so “currently derive” follows implementation rather than FR-014’s normative set. Reconcile or explicitly gate that existing FR-014 obligation before defining the closed inventory; otherwise the catalog may permanently exclude specified scalar operations. | spec/core/functional/FR-038-typed-operator-catalog.md:32-36, :55-56; spec/oracle/functional/FR-014-exact-scalar-oracles.md:135-168, :367-368; src/oracle/scalar/mod.rs:193-291, :1943-2061 |

## Final disposition

Rechecked local fix `c3248d34827369510c06580af025737032aae784` against reviewed head `ac350b807e01c876088889da594f93ad0bac19da`. FND-001 **FIXED**. FR-038 lines 35-45 and AC-1/AC-2 lines 68-69 explicitly compare FR-014’s specified set with implemented selections, name integer eq/ne and six Boolean variants as open FR-014 work, and require them to join the same source when implemented. State/frame eq/ne is no longer presented as discharging scalar eq/ne.
