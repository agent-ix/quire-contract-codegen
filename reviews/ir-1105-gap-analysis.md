---
id: SR-645002
title: "IR-1105 CG #645 scalar receipt gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen PR #645 head 0e848e23f6330c8f5ab1fa750760b6bd64ec1e6d; FR-017-AC-29, AC-41 through AC-50"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: reviews
---

## Summary

Plan completion: not assessed. Scoped the computed Test Matrix and source-to-test bindings to the changed scalar receipt behavior in CG #645, comparing tests with merged CG #644 criteria. No optional repository-wide semantic review was requested. No build or gate result is claimed.

## Verdict

CONDITIONAL for the changed scalar receipt trace chain. The real routed receipt test checks original emitted artifact bytes, but the computed matrix has no FR-017-AC-49 binder. This is not a whole-repository assurance verdict.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Merged FR-017-AC-49 requires a CG test comparing staged entries byte for byte with original routed artifacts, harness and record. `assert_retained_scalar_content` does this, but neither caller has an AC-49 trace tag, so `quire matrix` reports zero binders. Tag the genuine routed receipt test with AC-49. | tests/it/kani_obligations.rs:2227, spec/kani/functional/FR-017-kani-execution-evidence.md:815 |

## Coverage

FR-017-AC-29, AC-41 through AC-50 examined. Changed source: `src/kani/run/scalar.rs`, `src/kani/test_support.rs`, `src/lib.rs`; changed tests: `tests/it/kani_obligations.rs`, `tests/it/kani_scalar_native_private.rs`, removed `tests/it/kani_scalar_native_digest.rs`. `ensure_snapshot_matches` checks each expected byte and the complete file path set; `capture_entries` sorts, rejects duplicate names and validates UTF-8 of runner-read entries. Tests bind the ten-entry capture and mutation controls, receipt absence of digest APIs, same staged entries across separate native results, cleanup diagnostics, and typed error codes. They verify original emitted bytes but fail to tag that check to AC-49. Some real Kani tests are intentionally ignored in ordinary Cargo tests and require the Kani lane. Plan completion: not assessed.
