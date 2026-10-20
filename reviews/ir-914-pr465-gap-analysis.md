---
id: SR-13316
title: gap-analysis review of CG ScalarLimits root export
type: SpecReview
analysis: gap-analysis
scope: agent-ix/quire-contract-codegen; spec/kani/functional/FR-017-kani-execution-evidence.md,
  src/lib.rs, tests/it/kani_obligations.rs
review_set: subset
relationships:
- target: ix://agent-ix/quire-contract-codegen/FR-017
  type: reviews
---

## Summary

Ticket: IR-914. Compared FR-017-AC-51 with the computed matrix, tagged test, and root export. The criterion is tagged and the compile test passes, but the test does not prove the RT owner identity demanded by the new FR statement.

## Verdict

CONDITIONAL. Plan completion: not assessed. Computed matrix reports FR-017-AC-51 tagged to the new test; no new source stub or unowned code was found.

## Examined

FR-017 added root-export statement (examined): The codegen crate root SHALL publicly re-export the exact `quire_contract_runtime::exact::ScalarLimits` type of `KaniScalarNativeRequest::scalar_limits`, with no wrapper, constructor or default.
FR-017-AC-51 (examined): A consumer compile test names and constructs `quire_contract_codegen::ScalarLimits` using only codegen crate paths, and a typed accessor to `KaniScalarNativeRequest::scalar_limits` compiles with that same return type.
TC-027 (context only): existing Kani execution suite.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The compile test also passes if both the root export and request field become the same local wrapper; add a separate compile equality check against the RT type. | tests/it/kani_obligations.rs:3306 |

## Coverage

Plan completion: not assessed. Matrix: FR-017-AC-51 tagged to `tc_027_scalar_limits_is_available_at_codegen_root_with_request_type_identity`; reverse ownership: `src/lib.rs:178` implements the added FR statement. Optional repository-wide semantic review was not run.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | A local wrapper cannot be assigned to the RT type without an explicit conversion. |
