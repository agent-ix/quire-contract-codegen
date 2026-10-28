---
id: SR-619004
title: Integrity analysis of CG PR 620 authenticated operation views
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen; IR-1079; CG PR 620; FR-014-AC-2/18/48/50/51/55, FR-022-AC-30 and TC/TestMatrix consistency"
review_set: subset
---

## Summary

Integrity review of the changed requirement set at CG PR #620 head `41aeb6db37261e19fb68de5425cb3952a6ad26c3`. The new ACs name a typed refusal and give authenticated and view-independent controls, but the wording creates an unimplementable V1 branch for `derive_exact_scalar_items` and leaves request-order precedence ambiguous against the existing permutation contract.

## Scope

```yaml
scope:
  - {id: FR-014-AC-18, path: spec/oracle/functional/FR-014-exact-scalar-oracles.md, role: examined, excerpt: "For an admitted V2 package or a V1 package with source-authenticated operation views, `derive_exact_scalar_items` returns, per node id in input order, the descriptor determined by the node's `operation.identity`, `operation.mode`, `operation.laws`, operand forms and the one bound of the needed form on its result type."}
  - {id: FR-014-AC-55, path: spec/oracle/functional/FR-014-exact-scalar-oracles.md, role: examined, excerpt: "A plain reader-admitted V1 package with a represented application that reaches a mode, law-definition or member-target view required by a routed request is refused by `generate_routed` as the whole-call `RoutedGenerationError::UnauthenticatedOperationView { request_index, node_id }` naming the first such request in request order."}
  - {id: FR-022-AC-30, path: spec/routed/functional/FR-022-routed-generation.md, role: examined, excerpt: "For a plain reader-admitted V1 package, a routed item requiring an operation mode, law-definition or member-target view whose accessor returns `NotAdmitted` makes `generate_routed` refuse the whole call as `UnauthenticatedOperationView { request_index, node_id }`."}
  - {id: TC-024, path: spec/oracle/matrix/TC-024-exact-scalar-oracles.md, role: examined, excerpt: "For FR-014-AC-55, run public routed generation on a plain strict V1 read with represented quantity conversion and integer division operations."}
  - {id: TC-030, path: spec/routed/matrix/TC-030-capability-settlement.md, role: examined, excerpt: "For FR-014-AC-55, use a plain strict V1 read containing a represented quantity-conversion operation."}
  - {id: TC-033, path: spec/routed/matrix/TC-033-routed-generation.md, role: examined, excerpt: "For FR-022-AC-30, pass a canonical plain reader-admitted V1 package containing one view-independent integer operation and view-dependent division and quantity-conversion operations to `generate_routed`."}
```

## Verdict

**FAIL.** The AC-18 API mismatch is a hard consistency failure; the order and test-linkage gaps are repairable in this PR.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-014-AC-18 grants a V1 input to the named `derive_exact_scalar_items` API, but its signature accepts only `CheckedPackageV2`; V1 routed generation uses a different internal function | spec/oracle/functional/FR-014-exact-scalar-oracles.md:591, src/oracle/scalar/mod.rs:807 |
| FND-002 | medium | “Request order” does not specify ascending request index, so the first-refusal oracle can disagree with FR-022-AC-9's permutation-invariant result for reordered routed-item slices | spec/oracle/functional/FR-014-exact-scalar-oracles.md:627, spec/routed/functional/FR-022-routed-generation.md:352 |
| FND-003 | medium | The new FR-014-AC-55 TestMatrix row claims TC-030, but TC-030 has no FR-014 `verifies` relationship; its TC-024 inventory row omits AC-55 despite the new TC-024 procedure | spec/oracle/matrix/tests.md:17, spec/oracle/matrix/tests.md:106 |

## Coverage

The changed FRs still satisfy StR-001 and each new AC names Test verification. The new ACs are planned, and their TC procedures cover missing view, authenticated success, view-independent success, mixed request, and first-failure selection. `quire matrix --scope . --format tsv` emitted no criterion records in this checkout; computed trace-tag coverage was unavailable. No new constraint boundary, numeric limit, or state transition is introduced by this spec diff.

## Dispositions

Round 1, CG #620 exact head `b701f8bcf100fd848dfc6237c284f496b65e7e43`.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | FR-014-AC-18 is now V2-only and TC-024 distinguishes it from routed authenticated V1 behavior. |
| FND-002 | fixed | Both new ACs and the FR-014/FR-022 preambles define lowest affected `request_index`, independent of routed-item slice order. |
| FND-003 | fixed | TC-030 declares FR-014 verifies and the TC-024 catalog row includes AC-55. |

## Current verdict

Round 1 at `b701f8bcf100fd848dfc6237c284f496b65e7e43`: no remaining finding in this spec review. The literal IR-1079 AC-3 ticket wording remains for code-lead closeout.
