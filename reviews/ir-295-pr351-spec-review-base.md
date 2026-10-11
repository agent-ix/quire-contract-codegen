---
id: SR-7023
title: "IR-295 PR351 base spec review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen@a84869ca87712c57309c2d2fea4ee13deb0bd306; spec/routed/functional/FR-019-capability-settlement.md, spec/routed/matrix/TC-030-capability-settlement.md, spec/routed/matrix/tests.md; IR-295"
review_set: subset
---

## Summary

Reviewed the three-file diff at the exact head, FR-019-AC-25/26/27/33, the TC-030 status and procedure, and the computed criterion-to-test mapping. Two gaps remain in the Planned refusal contract.

## Verdict

**CONDITIONAL — AC-33 needs disjoint selectors/precedence and complete Planned StateFrame coverage.**

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Planned selectors overlap, leaving AC-33 without one exact refusal cause for forms matching multiple rows. | spec/routed/matrix/TC-030-capability-settlement.md:185,199-200; spec/routed/functional/FR-019-capability-settlement.md:313 |
| FND-002 | medium | TC-030 omits the in-scope StateFrame frame-role refusal when the frame grants every field. | spec/routed/matrix/TC-030-capability-settlement.md:202; spec/kani/functional/FR-015-bounded-kani-obligations.md:634,639 |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 5b7e3505d0f5ba359033beaff23f1f3d7ed2d00f |
| FND-002 | fixed | 5b7e3505d0f5ba359033beaff23f1f3d7ed2d00f |

FND-001 after_excerpt:

```text
V1 `BoundClause`, precondition/postcondition/invariant, Boolean root with a rational, decimal, IEEE-float, text, enum or quantity dependency, and zero definedness obligations | Planned (IR-269) | Remaining V1 scalar clause domains that reach Boolean oracle lowering. | `ClauseLowering { generation_code: UnsupportedDependency }` (`lower_clause` maps `typed_dependency_parameters`' diagnostic after the zero-obligations check).
```

FND-002 after_excerpt:

```text
V2 `StateFrame` frame role for a `postcondition` whose frame grants every state field | Planned (IR-269) | All-fields-granted frame form; FR-015-AC-63 and AC-68 name this refusal shape. | `StateFrameRefused { NothingForbidden }` (`generate_state_frame_role`, FR-015-AC-63 and AC-68).
```
