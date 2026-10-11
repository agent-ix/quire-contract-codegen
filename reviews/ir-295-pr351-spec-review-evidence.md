---
id: SR-7025
title: "IR-295 PR351 spec evidence review"
type: SpecReview
analysis: evidence
scope: "agent-ix/quire-contract-codegen@a84869ca87712c57309c2d2fea4ee13deb0bd306; spec/routed/functional/FR-019-capability-settlement.md, spec/routed/matrix/TC-030-capability-settlement.md, spec/routed/matrix/tests.md; IR-295"
review_set: subset
---

## Summary

Inspected the criterion verification methods, TC-030 test procedure, tests matrix, computed matrix and `quoin advise` recommendations. AC-33 names a test procedure and appears in the mapping; however, the overlapping Planned selectors make one stated exact-cause test oracle indeterminate.

## Verdict

**CONDITIONAL — the AC-33 evidence procedure requires disjoint selectors or explicit precedence to be falsifiable for all planned forms.**

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The TC-030 exact-cause oracle is ambiguous for a valid form that matches multiple Planned selectors; one test input can be required to return conflicting row causes. | spec/routed/matrix/TC-030-capability-settlement.md:52-56,185,199; spec/routed/functional/FR-019-capability-settlement.md:313 |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 5b7e3505d0f5ba359033beaff23f1f3d7ed2d00f |

FND-001 after_excerpt:

```text
V1 `BoundClause`, precondition/postcondition/invariant, Boolean root with a rational, decimal, IEEE-float, text, enum or quantity dependency, and zero definedness obligations | Planned (IR-269) | Remaining V1 scalar clause domains that reach Boolean oracle lowering. | `ClauseLowering { generation_code: UnsupportedDependency }` (`lower_clause` maps `typed_dependency_parameters`' diagnostic after the zero-obligations check).
```
