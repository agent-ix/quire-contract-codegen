---
id: SR-12631
title: "IR-210 gap analysis of CG PR 436"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen PR 436, 1fcf975d..5bec9cbf; src/oracle/equality/mod.rs, src/oracle/function/mod.rs, src/oracle/function/schedule.rs; 28 test bindings"
review_set: subset
---

# IR-210 gap analysis of CG PR 436

## Summary

Each of the 28 deleted trace declarations has an identical ordered ID list on the same test function at the PR head. Test bodies and production source are unchanged, so this conversion adds no evidence stub, unsupported implementation claim, or reverse ownership gap.

## Verdict

**PASS for the PR diff** — same-base coverage and the computed Test Matrix are unchanged after source-position normalization.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No new traceability gap (placeholder). | - |

## Coverage

Independent `quire coverage --scope . --strict` runs at base `1fcf975d` and head `5bec9cbf` each report 499/855 backed rows, 221 unbacked, and zero contradicted statuses. Both strict runs exit 1 because of the same existing unbacked rows. The computed TSV matrix has 812 criterion records plus its header on both sides; document, criterion ID, status, statement, and binder paths match after normalizing source line and column positions. Seventeen raw records differ only in those source positions. Plan completion: not assessed. Optional repository-wide semantic review: not run.

## Examined test bindings

| File | Test function | Retained IDs |
| --- | --- | --- |
| `src/oracle/equality/mod.rs` | `tc_029_qsl_recursive_package_uses_the_reader_shapes` | `FR-018-AC-26`, `FR-018-AC-27`, `TC-029` |
| `src/oracle/equality/mod.rs` | `tc_029_unclosed_option_and_sequence_cycles_refuse_at_the_repeated_node` | `FR-018-AC-24`, `TC-029` |
| `src/oracle/equality/mod.rs` | `tc_029_type_resolution_charges_each_entry_and_the_first_over_limit_entry` | `FR-018-AC-25`, `TC-029` |
| `src/oracle/equality/mod.rs` | `tc_029_deep_option_resolution_and_runtime_type_operations_fit_a_small_stack` | `FR-018-AC-25`, `TC-029` |
| `src/oracle/equality/mod.rs` | `cycle_refusal_follows_its_charged_entry_and_work_refusal_wins_at_the_boundary` | `FR-018-AC-29`, `TC-029` |
| `src/oracle/equality/mod.rs` | `maximum_work_setting_refuses_and_last_denied_count_is_representable` | `FR-018-AC-31`, `TC-029` |
| `src/oracle/equality/mod.rs` | `source_limit_refuses_before_a_function_or_module_fragment_is_appended` | `FR-018-AC-30`, `TC-029` |
| `src/oracle/equality/mod.rs` | `deep_acyclic_closure_generates_flat_compilable_source_on_a_small_stack` | `FR-018-AC-28`, `FR-018-AC-30`, `TC-029` |
| `src/oracle/equality/mod.rs` | `tc_029_inline_collection_bound_is_charged_before_lookup` | `FR-018-AC-25`, `TC-029` |
| `src/oracle/equality/mod.rs` | `tc_029_recursive_list_and_tree_shapes_close_at_record_keys` | `FR-018-AC-26`, `FR-018-AC-27`, `TC-029` |
| `src/oracle/equality/mod.rs` | `tc_029_optional_wrapper_malformed_shapes_refuse_without_affecting_tree` | `FR-018-AC-26`, `TC-029` |
| `src/oracle/equality/mod.rs` | `tc_029_direct_option_field_is_required_not_silently_optional` | `FR-018-AC-26`, `TC-029` |
| `src/oracle/equality/mod.rs` | `tc_029_sequence_bound_shape_mutants_refuse_without_affecting_list` | `FR-018-AC-27`, `TC-029` |
| `src/oracle/equality/mod.rs` | `tc_029_bound_members_are_read_from_named_bindings` | `FR-018-AC-14`, `TC-029` |
| `src/oracle/equality/mod.rs` | `tc_029_bare_literal_bound_members_are_refused` | `FR-018-AC-14`, `TC-029` |
| `src/oracle/equality/mod.rs` | `tc_029_ac18_render_value_type_refuses_quantity_and_reference_with_a_typed_error` | `FR-018-AC-18`, `TC-029` |
| `src/oracle/equality/mod.rs` | `tc_029_ac18_the_item_boundary_refuses_the_item_and_fails_the_call_on_a_generation_error` | `FR-018-AC-18`, `TC-029` |
| `src/oracle/equality/mod.rs` | `tc_029_ac18_a_render_failure_refuses_only_its_item_and_never_renames_a_sibling` | `FR-018-AC-18`, `TC-029` |
| `src/oracle/function/mod.rs` | `ac27_inline_scalar_literal_is_retained_with_its_admitted_type` | `FR-021-AC-27`, `TC-031` |
| `src/oracle/function/mod.rs` | `schedule_call_item_lowering_failure_keeps_its_own_limit_details` | `FR-021-AC-23`, `TC-031` |
| `src/oracle/function/schedule.rs` | `ac7_hundred_thousand_call_nodes_schedule_on_a_512_kib_stack` | `FR-021-AC-7`, `FR-021-AC-15`, `FR-021-AC-25`, `FR-021-AC-27`, `FR-021-AC-28`, `FR-021-AC-29`, `TC-031` |
| `src/oracle/function/schedule.rs` | `ac25_nested_add_schedule_is_postorder_and_budget_refusal_publishes_nothing` | `FR-021-AC-25`, `FR-021-AC-26`, `FR-021-AC-27`, `TC-031` |
| `src/oracle/function/schedule.rs` | `ac27_integer_literal_is_an_ordered_inline_schedule_operand` | `FR-021-AC-25`, `FR-021-AC-27`, `TC-031` |
| `src/oracle/function/schedule.rs` | `equality_operation_identity_preserves_its_scalar_domain` | `FR-021-AC-27`, `TC-031` |
| `src/oracle/function/schedule.rs` | `ac27_call_uses_exact_declaration_identity_and_rejects_mismatched_binding` | `FR-021-AC-27`, `TC-031` |
| `src/oracle/function/schedule.rs` | `ac27_unknown_operation_and_mismatched_operand_types_refuse_before_schedule` | `FR-021-AC-27`, `TC-031` |
| `src/oracle/function/schedule.rs` | `ac25_max_limit_is_rejected_before_any_graph_action` | `FR-021-AC-25`, `TC-031` |
| `src/oracle/function/schedule.rs` | `ac27_cycle_shared_node_and_unbound_parameter_refuse_without_a_schedule` | `FR-021-AC-27`, `TC-031` |
