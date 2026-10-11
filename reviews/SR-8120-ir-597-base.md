---
id: SR-8120
title: "IR-597 base spec review"
type: SpecReview
analysis: base
scope: "CG IR-597 three-file spec diff 91d594387257dbe0e6bbf2c69cfb88f58d6e118f..ac350b807e01c876088889da594f93ad0bac19da"
review_set: subset
---

## Summary

Reviewed FR-038, its AD-004 placement and registry entry against FR-014 and current scalar generation. The proposed mismatch disposition changes existing specified oracle behavior.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-038 says a descriptor/checked-node identity or mode mismatch takes a typed refusal/unsupported path. FR-014-AC-12 and AC-13 instead require an item with a present but disagreeing identity, law or mode to generate the descriptor-named oracle as caller_declared with a typed blocked item; current scalar.rs does exactly this. Preserve the caller_declared generated path, distinguishing missing identity from present-but-disagreeing identity, to satisfy FR-038’s public-output preservation rule. | spec/core/functional/FR-038-typed-operator-catalog.md:39-49, :56; spec/oracle/functional/FR-014-exact-scalar-oracles.md:360-363; src/oracle/scalar/mod.rs:687-729, :2145-2173 |

## Final disposition

Rechecked local fix `c3248d34827369510c06580af025737032aae784` against reviewed head `ac350b807e01c876088889da594f93ad0bac19da`. FND-001 **FIXED**. FR-038 Description and Behavior lines 27-31 and 48-55 now preserve generated caller_declared oracles for valid descriptors with present but disagreeing identity, law or mode, and separately retain typed refusal for missing node identity. AC-2 line 69 repeats that observable distinction. This matches FR-014-AC-12/13 and current scalar.rs:687-729.
