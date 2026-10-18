---
id: SR-12626
title: "IR-865 gap analysis of CG PR 433"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen PR 433, 0cb20cf3..6de93115; six changed Rust files, 21 trace bindings, and computed matrix"
review_set: subset
---

# IR-865 gap analysis of CG PR 433

## Summary

All 21 removed doc-line ID lists are retained in order on the same test functions. No test assertion or production behavior changed, so the diff adds no evidence gap, stub, or unsupported implementation claim.

## Verdict

**PASS for the PR diff** — base and head coverage and normalized computed matrix are equal.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No new traceability gap in the reviewed diff (placeholder). | - |

## Coverage

Independent `quire coverage --strict` runs at base `0cb20cf3` and head `6de93115` each report 499/855 backed rows, 221 unbacked, and zero contradicted statuses. The computed JSON matrix has 812 criterion records on each side (813 TSV lines including its header). Criterion IDs, statements, statuses, and binders match after source line and column normalization; 24 raw criterion records differ only in shifted binder positions. Strict exits 1 on both sides because of the existing 221 unbacked rows. Plan completion: not assessed. Optional repository-wide semantic review: not run.

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
