---
id: SR-12625
title: "IR-865 code and Rust review of CG PR 433"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen PR 433, 0cb20cf3..6de93115; src/evidence/bound_coverage.rs, src/oracle/mod.rs, src/oracle/operator_catalog.rs, src/routed/contract_package_tests.rs, src/routed/generate.rs, src/strategy/bound/generation.rs"
review_set: subset
---

# IR-865 code and Rust review of CG PR 433

## Summary

The diff converts 21 `/// Trace:` lines on existing test functions to `ix-trace-rs` attributes in six Rust files. A function-by-function script compared every ordered ID list, including `FR`, `NFR`, `AC`, `TC`, `CON`, and `M` segments, and found an identical `#[trace]` list on the same test function. Removing only trace metadata and the new test-module imports makes each base and head source file byte-identical.

## Verdict

**PASS** — no code or Rust review finding in the PR diff.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No finding in the reviewed diff (placeholder). | - |

## Evidence

- Converted ID lists by file: `bound_coverage.rs` 3, `oracle/mod.rs` 1, `operator_catalog.rs` 1, `contract_package_tests.rs` 9, `routed/generate.rs` 5, and `strategy/bound/generation.rs` 2. The audit script is `cg-pr433-trace-audit.py` in the review scratchpad; it checks ordered IDs and `#[test]` placement on all 21 functions.
- No test body, assertion, helper, production function, Kani proof function, explanatory prose, or non-trace doc changes after metadata normalization. No new `/// Trace:`/`Tracing:` line or orphan blank doc line appears. `git diff --check` and `make fmt-check` pass.
- `make kani-scope` reports `required` for four changed paths, consistent with the author's reported 34/34 real Kani gate. No proof body changed. The reviewer did not repeat the full Kani gate.

## Gates

Independent `cargo test --locked -j2 --lib` at the PR head ran 278 tests: 277 passed and `kani::run::execute::batch_tests::tc_043_a_batch_may_run_longer_than_t_when_it_is_inside_n_times_t` failed with `MemoryObservationFailed { detail: "namespace cleanup failed: namespace teardown was not confirmed" }`. That function is outside the diff and passes when run alone at the head with one test thread. The same isolated test passes at base `0cb20cf3` with one test thread. IR-877 tracks the intermittent namespace teardown failure. The author reports full `make -k ci` and `make kani-gate` passes on this head; this reviewer independently checked migration, formatting, and Kani scope.

## Examined test bindings

| Test function | File | Retained IDs |
| --- | --- | --- |
| `tc_042_ac4_a_region_without_a_probe_is_a_map_mismatch_not_a_panic` | `src/evidence/bound_coverage.rs` | `NFR-005-AC-4`, `NFR-005-M-2`, `TC-042` |
| `tc_042_ac7_a_map_shorter_than_two_regions_is_a_census_mismatch_not_a_panic` | `src/evidence/bound_coverage.rs` | `NFR-005-AC-7`, `NFR-005-M-2`, `TC-042` |
| `output_bound_counts_exact_bytes_before_allocating_output` | `src/evidence/bound_coverage.rs` | `TC-006`, `FR-004-AC-5` |
| `tc_024_the_classifier_reads_each_limit_kind_once` | `src/oracle/mod.rs` | `FR-014-AC-41`, `TC-024` |
| `catalog_entries_preserve_identity_family_arity_result_and_consumer_projection` | `src/oracle/operator_catalog.rs` | `FR-038-AC-1`, `FR-038-AC-2`, `FR-038-AC-3` |
| `admitted_ordering_variants_change_routed_oracle_bytes_and_keep_refused_sibling` | `src/routed/contract_package_tests.rs` | `FR-039-AC-1`, `FR-039-AC-2`, `TC-053` |
| `scalar_own_result_without_conversion_differs_from_not_applicable` | `src/routed/contract_package_tests.rs` | `FR-039-AC-2`, `FR-049-AC-1`, `TC-053` |
| `duplicate_expression_uses_each_selected_conversion_context` | `src/routed/contract_package_tests.rs` | `FR-039-AC-2`, `FR-049-AC-1`, `FR-049-AC-5`, `FR-019-AC-43`, `FR-019-AC-44`, `FR-022-AC-23`, `FR-022-AC-24`, `TC-030`, `TC-033`, `TC-053` |
| `admitted_bounded_integer_operation_generates_from_reached_bounds` | `src/routed/contract_package_tests.rs` | `FR-039-AC-1`, `TC-053` |
| `admitted_failed_accounting_refuses_without_an_oracle` | `src/routed/contract_package_tests.rs` | `FR-039-AC-6`, `TC-053` |
| `genuine_qsl_function_claim_uses_admitted_package_identity_and_reached_keys` | `src/routed/contract_package_tests.rs` | `FR-039-AC-7`, `FR-039-AC-8`, `TC-053` |
| `one_routed_profile_admits_function_and_equality_beside_scalar_bound_refusal` | `src/routed/contract_package_tests.rs` | `FR-039-AC-2`, `FR-039-AC-7`, `TC-053` |
| `selected_function_targets_keep_reader_identity_and_order` | `src/routed/contract_package_tests.rs` | `FR-039-AC-3`, `TC-053` |
| `checked_reader_refuses_forged_same_key_local_function` | `src/routed/contract_package_tests.rs` | `FR-039-AC-3`, `TC-053` |
| `mismatched_private_kinds_refuse_before_generation` | `src/routed/generate.rs` | `FR-022-AC-4`, `FR-022-AC-18`, `TC-046` |
| `tc_042_ac5_a_record_count_other_than_the_item_count_is_a_typed_refusal` | `src/routed/generate.rs` | `NFR-005-AC-5`, `NFR-005-M-2`, `TC-042` |
| `tc_042_ac6_a_duplicate_position_outside_the_group_is_a_typed_refusal` | `src/routed/generate.rs` | `NFR-005-AC-6`, `NFR-005-M-2`, `TC-042` |
| `tc_042_ac6_an_out_of_range_duplicate_position_refuses_the_whole_routing` | `src/routed/generate.rs` | `NFR-005-AC-6`, `TC-042` |
| `tc_033_two_harnesses_sharing_a_symbol_are_a_typed_error_not_a_silent_overwrite` | `src/routed/generate.rs` | `TC-033`, `FR-022-AC-16` |
| `tc_017_direct_relation_admission_refuses_wide_integer_domain` | `src/strategy/bound/generation.rs` | `FR-008-CON-3`, `TC-017` |
| `tc_017_direct_literal_conversion_refuses_outside_i64` | `src/strategy/bound/generation.rs` | `FR-008-CON-4`, `TC-017` |
