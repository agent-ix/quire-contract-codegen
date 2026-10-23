---
id: SR-16305
title: "CG 546 independent second incremental gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen@eca45769fb66ac72faf71c8542ebd9452ee24f3f; fixture repair delta 4362b233..eca45769; FR-014-AC-44; TC-024; src/lib.rs; tests/checked_package_support/evidence.rs; tests/composite_equality_support/package.rs; tests/it/exact_scalar_generation.rs; tests/it/main.rs"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-014", type: references }
  - { target: "ix://agent-ix/quire-contract-codegen/TC-024", type: references }
---
# SR-16305: CG 546 independent second incremental gap analysis

## Summary

Planless, diff-scoped check of QSpec evidence, the two repaired fixture consumers, and changed test-to-criterion bindings. The fixture repair is behaviorally supported, but both new helper tests overclaim FR-014-AC-44.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The published-evidence test tags FR-014-AC-44 but checks only IR reader admission and graph fields, never a scalar oracle claim. | tests/checked_package_support/evidence.rs:117; FR-014-AC-44 |
| FND-002 | medium | The mismatched-reference test tags FR-014-AC-44 but checks only IR reader refusal, never a scalar oracle claim. | tests/checked_package_support/evidence.rs:140; FR-014-AC-44 |

## Coverage

- FR-014-AC-44 requires a direct admitted temporal/formula, fairness or case request to produce one `ExactScalarRefusal::Unsupported` claim with specified node identity and no generated artifact. The two new tests call only `CheckedPackageV2::read`, then inspect admission, graph fields or `UnknownRequiredCapability`; they never submit an oracle request or inspect a claim map. Their `#[trace("FR-014-AC-44", "TC-024")]` tags therefore count prerequisite tests as direct AC evidence. The existing `tc_024_ac44_direct_temporal_formula_is_unsupported` test does exercise the oracle criterion and remains unchanged, so this is trace inflation, not a demonstrated product regression. Remove the two FR-014-AC-44 tags or extend each test to assert the named oracle result.
- QSpec's exact selected retention reference was independently checked against the published definitions at `e10b75d5e495ef0cdece93623ef2f92396813112`. The positive and inverse private tests exercise real reader behavior; they are meaningful evidence for fixture setup and strict evidence admission.
- The two existing fixture consumers now call the shared helper. Their original oracle and composite assertions remain in place. The changed test support adds no new unowned production behavior, stub, copied fixture or matrix artifact.
- This is an incremental review of the five-file repair, not a claim of repository-wide gap clearance. The exact-head native CI summary in IR-975 is author evidence and was not independently rerun.
- Plan completion: not assessed.

## Verdict

CONDITIONAL for the incremental slice because the two new FR-014-AC-44 trace bindings do not assert that criterion. The fixture admission repair itself passes the second code review.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | CG #557 at `7f026c1909f54c893a576e84ec3eb100fa232ebc` removes the FR-014-AC-44/TC-024 trace attribute from `published_extension_evidence_is_required_for_strict_all_families_admission`; its test body is unchanged. |
| FND-002 | fixed | The same commit removes the FR-014-AC-44/TC-024 trace attribute from `mismatched_selected_extension_refs_do_not_manufacture_reader_evidence`; its test body is unchanged. The two remaining AC-44 binders in `tests/it/exact_scalar_generation.rs` make direct oracle requests and assert typed `Unsupported` refusals. |

The disposition is scoped to the three-line trace cleanup on CG main `a3aad3b757bdd3b9028094c0b562c26970aaa3d3`. The original conditional verdict above records the initial review; both findings are now fixed. No production, proof, lock, or test-body line changed in this follow-up.
