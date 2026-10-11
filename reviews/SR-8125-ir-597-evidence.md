---
id: SR-8125
title: "IR-597 evidence spec review"
type: SpecReview
analysis: evidence
scope: "CG IR-597 three-file spec diff 91d594387257dbe0e6bbf2c69cfb88f58d6e118f..ac350b807e01c876088889da594f93ad0bac19da"
review_set: subset
---

## Summary

Computed Quoin matrix shows FR-038-AC-2 through AC-4 untagged, and inspection-only AC-1/AC-5 without symbols; this is a local spec stage, not an implementation claim. Catalog advice flags AC-5’s Inspection method as mismatched with behavioral verification.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-038-AC-5 combines preservation of V1 strategy/bound host comparison semantics with structural exclusion of the corpus, but declares Inspection only. Comparison-semantic preservation is behavioral and can regress while the file layout still looks correct. Split the checks or add focused tests that discriminate mirrored operand order and each relation operator; keep inspection for migration/exclusion. The Quoin method advisor also flags AC-5 Inspection versus Test recommendations. | spec/core/functional/FR-038-typed-operator-catalog.md:59; src/strategy/bound/relation.rs:25-35, :133-134; quoin advise --repo . --json: FR-038-AC-5 |

## Final disposition

Rechecked local fix `c3248d34827369510c06580af025737032aae784` against reviewed head `ac350b807e01c876088889da594f93ad0bac19da`. FND-001 **FIXED**. AC-5 line 72 is now structural Inspection of V1/corpus exclusion; new AC-6 line 73 is Test verification of all six V1 comparison operators and left/right primary orientation for noncommutative comparisons. It matches strategy/bound/relation.rs:9-45 and :127-135.
