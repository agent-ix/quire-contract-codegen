---
id: SR-8664
title: "IR-550 fairness follow-up code review with Rust lane"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen@0219c9791b3b8552c3fb06f39655128bdc50c8f8; tests/composite_equality_support/package.rs, tests/it/exact_function_generation.rs; FR-021-AC-33, TC-031 step 13"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-021", type: references }
---

## Summary

Reviewed the exact pre-PR IR-550 fairness follow-up diff against `824c56c33f9d469d83c9fe2fb2b6da8b05555ace`, including the Rust test/helper changes and their direct criterion binding. The added QSpec-backed fairness root is built through the public V2 reader and the test checks identity-preserving refusal and healthy-sibling invariance.

## Verdict

**PASS** — no defect found in the scoped code or Rust review. The diff is test and fixture support only; no production source behavior changed.

## Reviewed Units

- **FR-021-AC-33 (examined):** Direct formula, fairness and case call-node requests retain their claim identity and typed refusal; affected items produce no schedule, symbol or location, while healthy siblings remain byte-identical. The declared-body-root case and refusal precedence are separately covered.
- **TC-031 step 13 (examined):** Direct formula, fairness and case roots are admitted through the public V2 reader, requested as call-node ids, and checked for claim identity, cause id/tag, no generation artifacts and healthy-sibling invariance.
- **Fixture support (examined):** `tests/composite_equality_support/package.rs` adds a model-owned object-type fixture node and resolves evidence from the model selection in the current wire.
- **Fairness test (examined):** `tests/it/exact_function_generation.rs::tc_031_ac33_direct_fairness_call_node_is_unsupported` mutates the authoritative QSpec fixture, verifies admission through `PackageBuilder::read_resolved`, then checks the direct refusal and output/location invariance.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Review Notes

The computed Quire matrix binds FR-021-AC-33 to the existing declared-body and direct formula/case tests and the new direct fairness test. No Cargo test, full CI, or Kani gate was run in this reviewer pass.
