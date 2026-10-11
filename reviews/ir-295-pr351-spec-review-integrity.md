---
id: SR-7024
title: "IR-295 PR351 spec integrity review"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen@a84869ca87712c57309c2d2fea4ee13deb0bd306; spec/routed/functional/FR-019-capability-settlement.md, spec/routed/matrix/TC-030-capability-settlement.md, spec/routed/matrix/tests.md; IR-295"
review_set: subset
---

## Summary

Checked consistency among normative V1/V2 scope, settlement rules, Planned status rows, exact typed refusal causes, and procedure/test trace. AC-25 remains the full normative V1 set and AC-26/27 remain distinct; the new Planned contract has two integrity gaps.

## Verdict

**CONDITIONAL — Planned selectors and StateFrame refusal inventory are not complete/consistent.**

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Overlapping Planned rows declare different current causes without a precedence rule, so the register is not internally determinate. | spec/routed/matrix/TC-030-capability-settlement.md:185,199-200; spec/routed/functional/FR-019-capability-settlement.md:313 |
| FND-002 | medium | StateFrame planned status is incomplete: the supported-source refusal inventory in FR-015 includes an omitted frame-role case. | spec/routed/matrix/TC-030-capability-settlement.md:201-202; spec/kani/functional/FR-015-bounded-kani-obligations.md:634,639 |

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
