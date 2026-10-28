---
id: SR-619003
title: Base spec review of CG PR 620 authenticated operation views
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; IR-1079; CG PR 620; FR-014-AC-2/18/48/50/51/55, FR-022-AC-30, TC-024/030/033, two TestMatrix files"
review_set: subset
---

## Summary

Independent base specification review of CG PR #620 at `41aeb6db37261e19fb68de5425cb3952a6ad26c3`. I checked the seven changed files against Contract IR FR-035-AC-25/26 at `3e74887e` and the public CG #608 behavior. The new typed refusal is explicit and the test procedures distinguish authenticated, plain view-dependent, and plain view-independent inputs. The edit nonetheless narrows existing scalar obligations without an owner change to IR-1079's no-new-behavior acceptance, and it assigns a V1 input to a V2-only public function.

## Scope

```yaml
scope:
  - {id: FR-014-AC-2, path: spec/oracle/functional/FR-014-exact-scalar-oracles.md, role: examined, excerpt: "Entry-point dispositions for a `quire.op.quantity.convert` node: derivation and generation produce the oracle (FR-014-AC-48); the routed path, `negotiate_kani_obligations` and capability settlement settle it as `OperationNotRendered` after source-authenticated V1 admission (FR-014-AC-51)."}
  - {id: FR-014-AC-18, path: spec/oracle/functional/FR-014-exact-scalar-oracles.md, role: examined, excerpt: "For an admitted V2 package or a V1 package with source-authenticated operation views, `derive_exact_scalar_items` returns, per node id in input order, the descriptor determined by the node's `operation.identity`, `operation.mode`, `operation.laws`, operand forms and the one bound of the needed form on its result type."}
  - {id: FR-014-AC-48, path: spec/oracle/functional/FR-014-exact-scalar-oracles.md, role: examined, excerpt: "For an admitted V2 package or a V1 package with source-authenticated operation views, a `quire.op.quantity.convert` node whose `type_argument` names a quantity unit and whose `operation.mode` is `exact` derives and generates an `ir_confirmed` oracle."}
  - {id: FR-014-AC-50, path: spec/oracle/functional/FR-014-exact-scalar-oracles.md, role: examined, excerpt: "For an admitted V2 package or a V1 package with source-authenticated operation views, no oracle generated for `quire.op.quantity.convert` returns a scalar: its result is always a `Quantity` in the supplied target unit."}
  - {id: FR-014-AC-51, path: spec/oracle/functional/FR-014-exact-scalar-oracles.md, role: examined, excerpt: "After admitted V2 or source-authenticated V1 operation views are available, a selected `quire.op.quantity.convert` claim settles through capability settlement like any other quantity operation, as `OperationNotRendered` (Planned, IR-269)."}
  - {id: FR-014-AC-55, path: spec/oracle/functional/FR-014-exact-scalar-oracles.md, role: examined, excerpt: "A plain reader-admitted V1 package with a represented application that reaches a mode, law-definition or member-target view required by a routed request is refused by `generate_routed` as the whole-call `RoutedGenerationError::UnauthenticatedOperationView { request_index, node_id }` naming the first such request in request order."}
  - {id: FR-022-AC-30, path: spec/routed/functional/FR-022-routed-generation.md, role: examined, excerpt: "For a plain reader-admitted V1 package, a routed item requiring an operation mode, law-definition or member-target view whose accessor returns `NotAdmitted` makes `generate_routed` refuse the whole call as `UnauthenticatedOperationView { request_index, node_id }`."}
  - {id: TC-024, path: spec/oracle/matrix/TC-024-exact-scalar-oracles.md, role: examined, excerpt: "For FR-014-AC-55, run public routed generation on a plain strict V1 read with represented quantity conversion and integer division operations."}
  - {id: TC-030, path: spec/routed/matrix/TC-030-capability-settlement.md, role: examined, excerpt: "For FR-014-AC-55, use a plain strict V1 read containing a represented quantity-conversion operation."}
  - {id: TC-033, path: spec/routed/matrix/TC-033-routed-generation.md, role: examined, excerpt: "For FR-022-AC-30, pass a canonical plain reader-admitted V1 package containing one view-independent integer operation and view-dependent division and quantity-conversion operations to `generate_routed`."}
  - {id: FR-035-AC-25, path: quire-contract-ir/spec/checked_package/functional/FR-035-complete-v1-contract-package-lowering.md, role: context_only, excerpt: "A consumer must use authenticated read or propagate `NotAdmitted`; it cannot silently omit operation-dependent behavior or parse JSON to recover the facts."}
  - {id: FR-035-AC-26, path: quire-contract-ir/spec/checked_package/functional/FR-035-complete-v1-contract-package-lowering.md, role: context_only, excerpt: "With the exact reader-admitted V2 source supplied through an explicit authenticated handoff, strict reading the lowerer's canonical V1 bytes returns a `ContractPackageV1` whose every represented term equals the producer's and source's term."}
```

## Checks

`make spec` passes at the reviewed head, with only pre-existing EARS warnings in FR-017 and FR-014 line 305. No applicable AssuranceProfile with `review_policy` was found under `spec/`. The new IDs are unique and correctly formed, Test verification cells use a method, and the new criteria have planned status rather than false coverage credit. The available `quire matrix --scope . --format tsv` produced no criterion records in this checkout, so computed tag coverage could not be independently confirmed. No new hash, digest, pin, copied artifact, or version-tracking requirement was introduced.

## Verdict

**FAIL.** The changed scope of existing ACs conflicts with IR-1079 AC-3's no-new-behavior obligation, and AC-18's V1 branch cannot be exercised through the named public function. The order of refusal and the claimed TC-030 linkage need reconciliation.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Existing FR-014-AC-2/48/51 success and settlement obligations gain a source-authenticated V1 precondition while IR-1079 AC-3 still requires no new behavior; the plain reader-admitted quantity route now changes from oracle generation to whole-call error without an owner-approved criterion change | spec/oracle/functional/FR-014-exact-scalar-oracles.md:575, spec/oracle/functional/FR-014-exact-scalar-oracles.md:623 |
| FND-002 | high | FR-014-AC-18 says `derive_exact_scalar_items` accepts authenticated V1, but that public function takes only `&CheckedPackageV2`; the V1 branch of the criterion cannot be tested or implemented by that named API | spec/oracle/functional/FR-014-exact-scalar-oracles.md:591, src/oracle/scalar/mod.rs:807 |
| FND-003 | medium | FR-014-AC-55 and FR-022-AC-30 say the first failing request is chosen in “request order,” which can mean caller slice order; FR-022-AC-9 requires permutation-invariant results and the implementation uses ascending request index | spec/oracle/functional/FR-014-exact-scalar-oracles.md:627, spec/routed/functional/FR-022-routed-generation.md:373 |
| FND-004 | medium | The oracle TestMatrix adds FR-014-AC-55 mapped to TC-030, but TC-030's `relationships:` still verifies only FR-019; the TC-024 catalog row still stops at FR-014-AC-42 while the new procedure claims AC-55 | spec/oracle/matrix/tests.md:17, spec/routed/matrix/TC-030-capability-settlement.md:4 |

## Dispositions

Round 1, CG #620 exact head `b701f8bcf100fd848dfc6237c284f496b65e7e43`.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | rejected | The prior plain-read quantity success consumed operation views that Contract IR FR-035-AC-25 now classifies as NotAdmitted, so it was never a valid admitted-view oracle or settlement. The revised FR-014 preamble states that boundary and keeps authenticated quantity oracle/OperationNotRendered plus view-independent plain-read results. The planner acting in the assigned spec lane explicitly accepted this correction of invalid prior behavior. IR-1079 AC-3 still describes compile-only adaptation; its literal observed-output conflict remains a ticket closeout matter, not an unsupported narrowing of valid CG behavior in this spec PR. |
| FND-002 | fixed | FR-014-AC-18 now limits `derive_exact_scalar_items` to an admitted V2 package; TC-024 explicitly checks that direct API separately from routed authenticated V1 behavior. |
| FND-003 | fixed | FR-014-AC-55 and FR-022-AC-30 now select the lowest affected `request_index` regardless of routed-item slice permutation; TC-024 and TC-033 require both controls. |
| FND-004 | fixed | TC-030 now verifies FR-014, and the oracle TestMatrix TC-024 inventory includes FR-014-AC-55 while its detailed row retains TC-024/TC-030 as Planned. |

## Current verdict

Round 1 at `b701f8bcf100fd848dfc6237c284f496b65e7e43`: no remaining finding in this spec review. The literal IR-1079 AC-3 ticket wording remains for code-lead closeout.
