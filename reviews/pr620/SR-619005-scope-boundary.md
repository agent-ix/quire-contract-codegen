---
id: SR-619005
title: Scope boundary analysis of CG PR 620 authenticated operation views
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/quire-contract-codegen; IR-1079; CG PR 620; IR FR-035-AC-25/26 handoff, CG FR-014/FR-022 routed generation"
review_set: subset
---

## Summary

Scope review of CG PR #620 head `41aeb6db37261e19fb68de5425cb3952a6ad26c3`. Contract IR owns operation-view authentication and the source-correspondence handoff; CG owns the routed choice to use those views, propagate `NotAdmitted` as a typed error, and preserve generation for view-independent requests. The new AC-55/AC-30 allocation fits this boundary. Existing FR-014 criteria must not assign V1 handling to its V2-only direct-derivation function or silently retire the old public plain-reader outcome.

## Scope

```yaml
scope:
  - {id: FR-014-AC-18, path: spec/oracle/functional/FR-014-exact-scalar-oracles.md, role: examined, excerpt: "For an admitted V2 package or a V1 package with source-authenticated operation views, `derive_exact_scalar_items` returns, per node id in input order, the descriptor determined by the node's `operation.identity`, `operation.mode`, `operation.laws`, operand forms and the one bound of the needed form on its result type."}
  - {id: FR-014-AC-48, path: spec/oracle/functional/FR-014-exact-scalar-oracles.md, role: examined, excerpt: "For an admitted V2 package or a V1 package with source-authenticated operation views, a `quire.op.quantity.convert` node whose `type_argument` names a quantity unit and whose `operation.mode` is `exact` derives and generates an `ir_confirmed` oracle."}
  - {id: FR-014-AC-51, path: spec/oracle/functional/FR-014-exact-scalar-oracles.md, role: examined, excerpt: "After admitted V2 or source-authenticated V1 operation views are available, a selected `quire.op.quantity.convert` claim settles through capability settlement like any other quantity operation, as `OperationNotRendered` (Planned, IR-269)."}
  - {id: FR-014-AC-55, path: spec/oracle/functional/FR-014-exact-scalar-oracles.md, role: examined, excerpt: "A plain reader-admitted V1 package with a represented application that reaches a mode, law-definition or member-target view required by a routed request is refused by `generate_routed` as the whole-call `RoutedGenerationError::UnauthenticatedOperationView { request_index, node_id }` naming the first such request in request order."}
  - {id: FR-022-AC-30, path: spec/routed/functional/FR-022-routed-generation.md, role: examined, excerpt: "For a plain reader-admitted V1 package, a routed item requiring an operation mode, law-definition or member-target view whose accessor returns `NotAdmitted` makes `generate_routed` refuse the whole call as `UnauthenticatedOperationView { request_index, node_id }`."}
  - {id: FR-035-AC-25, path: quire-contract-ir/spec/checked_package/functional/FR-035-complete-v1-contract-package-lowering.md, role: context_only, excerpt: "A consumer must use authenticated read or propagate `NotAdmitted`; it cannot silently omit operation-dependent behavior or parse JSON to recover the facts."}
  - {id: FR-035-AC-26, path: quire-contract-ir/spec/checked_package/functional/FR-035-complete-v1-contract-package-lowering.md, role: context_only, excerpt: "With the exact reader-admitted V2 source supplied through an explicit authenticated handoff, strict reading the lowerer's canonical V1 bytes returns a `ContractPackageV1` whose every represented term equals the producer's and source's term."}
```

## Verdict

**FAIL.** New boundary obligations are allocated to the right components, but the edited existing criteria misallocate a V1 path and narrow a prior CG result without an owner decision.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-014-AC-18 allocates authenticated V1 handling to `derive_exact_scalar_items`, although that API belongs to the V2 direct-derivation boundary; the routed V1 path is owned by `generate_routed`/contract scalar generation | spec/oracle/functional/FR-014-exact-scalar-oracles.md:591, src/oracle/scalar/mod.rs:807 |
| FND-002 | high | FR-014-AC-48/51 now exclude plain reader-admitted V1 quantity conversion from the oracle and `OperationNotRendered` obligations, while IR-1079 AC-3 still says no new behavior; the new CG refusal contract needs an explicit owner resolution of that boundary change | spec/oracle/functional/FR-014-exact-scalar-oracles.md:621, spec/oracle/functional/FR-014-exact-scalar-oracles.md:623 |

## Boundary allocation

| Requirement | Owner | Class | External dependency |
| --- | --- | --- | --- |
| IR FR-035-AC-25/26 | Contract IR | core | V2 reader-admitted source and canonical V1 bytes |
| CG FR-014-AC-55 | CG scalar oracle/routed consumer | core | IR authenticated operation views, guaranteed by matching-source handoff |
| CG FR-022-AC-30 | CG routed generation | core | IR `NotAdmitted` typed accessor result, guaranteed by contract test |
| CG FR-014-AC-18/48/50/51 | CG direct oracle and routed scalar paths | core | IR V2 admission and authenticated V1 handoff, with separate CG entry points |

No new external service, identity provider, or storage boundary is introduced. The matching V2 source is trusted only through IR's checked handoff; CG does not parse represented JSON to recover a view.

## Dispositions

Round 1, CG #620 exact head `b701f8bcf100fd848dfc6237c284f496b65e7e43`.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | FR-014-AC-18 assigns direct derivation to admitted V2 only; authenticated V1 remains on the routed path. |
| FND-002 | rejected | The prior plain-read quantity success consumed operation views that Contract IR FR-035-AC-25 now classifies as NotAdmitted, so it was never a valid admitted-view oracle or settlement. The revised FR-014 preamble states that boundary and keeps authenticated quantity oracle/OperationNotRendered plus view-independent plain-read results. The planner acting in the assigned spec lane explicitly accepted this correction of invalid prior behavior. IR-1079 AC-3 still describes compile-only adaptation; its literal observed-output conflict remains a ticket closeout matter, not an unsupported narrowing of valid CG behavior in this spec PR. |

## Current verdict

Round 1 at `b701f8bcf100fd848dfc6237c284f496b65e7e43`: no remaining finding in this spec review. The literal IR-1079 AC-3 ticket wording remains for code-lead closeout.
