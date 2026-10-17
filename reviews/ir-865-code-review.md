---
id: SR-12335
title: "IR-865 CG #427 code review (Rust lane): trace attribute migration"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen@472397829f3d453fcd426a1160a7179574027b7c; six changed Rust test files"
review_set: subset
---

## Summary

Ticket: IR-865. PR: quire-contract-codegen#427. This Rust review covers all six changed test files and their 62 migrated test functions. The diff changes only trace documentation, `ix_trace_rs::trace` imports, and trace attributes. The `ix-trace-rs` macro validates arguments and returns the annotated item unchanged.

## Reviewed scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| `tests/checked_package_support/rekey.rs` | examined | Five `#[trace(...)]` migrations in the internal `#[cfg(test)]` module. |
| `tests/composite_equality_support/package.rs` | examined | Two fixture-identity test migrations in its `#[cfg(test)]` module. |
| `tests/it/composite_equality_agreement.rs` | examined | One generated composite equality agreement test migration. |
| `tests/it/composite_original_eq.rs` | examined | Sixteen original equality test migrations. |
| `tests/it/no_generation_panics.rs` | examined | Seven panic-boundary test migrations. |
| `tests/it/terminal_map.rs` | examined | Thirty-one terminal-map test migrations. |

## Bindings examined

The table below records every migrated test function and its IDs. An independent script compared each function's removed doc IDs with its added attribute IDs at base `836b298` and head `4723978`; all 62 match, and every new attribute is on a `#[test]` function.

| Path | Test function | Migrated IDs |
| --- | --- | --- |
| tests/checked_package_support/rekey.rs | interleaved_function_and_application_candidates_settle_before_reader | FR-021-AC-34, TC-031 |
| tests/checked_package_support/rekey.rs | application_candidates_converge_across_a_reference_chain | FR-037-AC-1, TC-052 |
| tests/checked_package_support/rekey.rs | grouped_application_candidates_require_complete_context | FR-037-AC-2, TC-052 |
| tests/checked_package_support/rekey.rs | frame_entry_rekey_sorts_only_authored_ordered_arrays | FR-037-AC-1, TC-052 |
| tests/checked_package_support/rekey.rs | predecessor_first_candidates_settle_in_one_changing_pass | FR-037-AC-1, TC-052 |
| tests/composite_equality_support/package.rs | composite_application_candidate_matches_admitted_node | FR-037-AC-1, FR-037-AC-4, TC-052 |
| tests/composite_equality_support/package.rs | grouped_application_candidate_matches_admitted_node | FR-037-AC-2, TC-052 |
| tests/it/composite_equality_agreement.rs | tc_029_generated_composite_equality_crate_agrees_with_direct_runtime | FR-018-AC-2, FR-018-AC-8, FR-018-AC-9, FR-018-AC-10, FR-018-AC-11, TC-029 |
| tests/it/composite_original_eq.rs | original_parameter_operands_build_positional_identity_and_public_verified_report | FR-033-AC-1, FR-033-AC-9, FR-033-AC-11, TC-048 |
| tests/it/composite_original_eq.rs | original_literal_operands_retain_graph_children_and_empty_bounds | FR-033-AC-1, FR-033-AC-11, TC-048 |
| tests/it/composite_original_eq.rs | original_ne_parameter_operands_keep_order_and_operator | FR-033-AC-1, FR-033-AC-11, TC-048 |
| tests/it/composite_original_eq.rs | original_ne_literal_operand_keeps_empty_bounds | FR-033-AC-1, FR-033-AC-11, TC-048 |
| tests/it/composite_original_eq.rs | original_operator_mismatch_refuses_with_typed_operators | FR-033-AC-1, TC-048 |
| tests/it/composite_original_eq.rs | self_comparison_keeps_two_positional_arguments | FR-033-AC-11, TC-048 |
| tests/it/composite_original_eq.rs | wrong_source_refuses_before_invocation | FR-033-AC-1, FR-033-AC-9, TC-048 |
| tests/it/composite_original_eq.rs | different_semantic_package_refuses_before_invocation | FR-033-AC-1, FR-033-AC-9, TC-048 |
| tests/it/composite_original_eq.rs | retained_selection_and_recompiled_context_mismatch_refuse | FR-033-AC-1, FR-033-AC-9, TC-048 |
| tests/it/composite_original_eq.rs | wrong_node_and_occurrence_refuse_before_invocation | FR-033-AC-1, FR-033-AC-9, TC-048 |
| tests/it/composite_original_eq.rs | another_functions_node_reaches_binding_checked_qsl_refusal | FR-033-AC-1, FR-033-AC-9, FR-033-AC-12, TC-048 |
| tests/it/composite_original_eq.rs | unrelated_node_bound_reaches_binding_checked_qsl_refusal | FR-033-AC-9, FR-033-AC-11, FR-033-AC-12, TC-048 |
| tests/it/composite_original_eq.rs | original_operand_projection_obeys_work_ceiling | FR-033-AC-1, FR-033-AC-11, TC-048 |
| tests/it/composite_original_eq.rs | original_stage_limit_stops_recompile_before_invocation | FR-033-AC-1, FR-033-AC-9, TC-048 |
| tests/it/composite_original_eq.rs | duplicate_positional_bound_refuses_without_an_obligation | FR-033-AC-11, TC-048 |
| tests/it/composite_original_eq.rs | original_imported_context_refuses_before_invocation | FR-033-AC-1, FR-033-AC-9, TC-048 |
| tests/it/no_generation_panics.rs | tc_042_ac1_the_literal_free_scan_keeps_code_and_drops_literals_comments_and_test_items | NFR-005-AC-1, TC-042 |
| tests/it/no_generation_panics.rs | tc_042_ac1_src_holds_no_panic_token | NFR-005-AC-1, NFR-005-M-1, TC-042 |
| tests/it/no_generation_panics.rs | tc_023_canonical_encoding_has_one_source_boundary | TC-023 |
| tests/it/no_generation_panics.rs | tc_042_ac8_the_ir_577_bodies_hold_no_index_or_subtraction | NFR-005-AC-8, NFR-005-M-3, TC-042 |
| tests/it/no_generation_panics.rs | tc_042_ac8_the_body_check_flags_index_and_subtraction_and_passes_their_lookalikes | NFR-005-AC-8, TC-042 |
| tests/it/no_generation_panics.rs | tc_042_ac2_a_claim_map_this_generator_did_not_produce_is_operation_not_rendered | NFR-005-AC-2, NFR-005-M-2, TC-042 |
| tests/it/no_generation_panics.rs | tc_042_ac3_an_unsupported_settlement_warns_only_for_an_unsupported_projection_cause | NFR-005-AC-3, NFR-005-M-2, TC-042 |
| tests/it/terminal_map.rs | direct_cancelled_run_maps_exact_source_and_rejects_settlement | FR-029-AC-15, FR-029-AC-29, FR-040-AC-7 |
| tests/it/terminal_map.rs | tc_040_verified_proves_its_checks_and_a_reproduced_falsification_refutes | FR-029-AC-1, TC-040 |
| tests/it/terminal_map.rs | tc_040_a_vacuous_proof_and_an_unsatisfied_cover_are_proved_with_zero_checks | FR-029-AC-2, TC-040 |
| tests/it/terminal_map.rs | observed_memory_exhaustion_maps_to_incomplete_resource_exhausted | FR-029-AC-3 |
| tests/it/terminal_map.rs | tc_040_a_timeout_and_an_exhausted_unwind_bound_are_incomplete | TC-040 |
| tests/it/terminal_map.rs | tc_040_a_run_with_no_verdict_is_failed | FR-029-AC-4, TC-040 |
| tests/it/terminal_map.rs | tc_040_a_missing_counterexample_or_cover_summary_is_failed | FR-029-AC-5, TC-040 |
| tests/it/terminal_map.rs | tc_040_no_outcome_maps_to_tested | FR-029-AC-6, TC-040 |
| tests/it/terminal_map.rs | tc_040_a_settlement_accompanies_a_falsified_outcome_only | FR-029-AC-15, TC-040 |
| tests/it/terminal_map.rs | tc_040_a_replay_disagreement_carries_its_cause | FR-029-AC-8, TC-040 |
| tests/it/terminal_map.rs | tc_040_a_replay_refusal_carries_its_code | FR-029-AC-9, TC-040 |
| tests/it/terminal_map.rs | tc_040_each_failure_this_repository_raises_is_failed | FR-029-AC-11, TC-040 |
| tests/it/terminal_map.rs | tc_040_a_fault_in_any_replay_wrapper_is_failed | FR-029-AC-10, TC-040 |
| tests/it/terminal_map.rs | tc_040_only_a_reproduced_replay_refutes | FR-029-AC-12, TC-040 |
| tests/it/terminal_map.rs | tc_040_a_setup_refusal_after_a_refutation_is_replay_refused_never_declined | FR-029-AC-13, TC-040 |
| tests/it/terminal_map.rs | tc_040_a_repeated_dependency_identity_is_replay_refused_with_invalid_package | FR-029-AC-14, TC-040 |
| tests/it/terminal_map.rs | tc_040_the_state_clause_replay_reads_as_fr029_ac16 | FR-029-AC-16, TC-040 |
| tests/it/terminal_map.rs | tc_041_every_expressible_pair_maps_to_one_value | FR-030-AC-1, TC-041 |
| tests/it/terminal_map.rs | tc_041_a_settlement_accompanies_a_counterexample_only | FR-030-AC-14, TC-041 |
| tests/it/terminal_map.rs | tc_041_a_refusal_before_the_run_is_declined_with_its_kind_and_ir_code | FR-030-AC-2, TC-041 |
| tests/it/terminal_map.rs | tc_041_a_limit_or_a_cancellation_is_incomplete_with_its_cause | FR-030-AC-3, TC-041 |
| tests/it/terminal_map.rs | tc_041_a_proof_carries_its_check_count_and_a_reproduced_counterexample_refutes | FR-030-AC-4, FR-030-AC-18, TC-041 |
| tests/it/terminal_map.rs | tc_041_a_vacuous_proof_is_proved_with_zero_checks_and_other_inconclusive_is_failed | FR-030-AC-5, FR-030-AC-18, TC-041 |
| tests/it/terminal_map.rs | tc_041_no_outcome_maps_to_tested | FR-030-AC-6, TC-041 |
| tests/it/terminal_map.rs | tc_041_the_map_is_one_match_over_the_pair_with_no_wildcard_arm | FR-030-AC-7, TC-041 |
| tests/it/terminal_map.rs | tc_041_an_absent_solver_or_backend_is_unsupported_with_its_cause | FR-030-AC-8, TC-041 |
| tests/it/terminal_map.rs | tc_041_a_counterexample_disagreement_or_refusal_is_inconclusive_with_its_cause | FR-030-AC-9, TC-041 |
| tests/it/terminal_map.rs | tc_041_a_counterexample_with_a_fault_or_a_cg_defect_is_failed | FR-030-AC-10, TC-041 |
| tests/it/terminal_map.rs | tc_041_only_a_reproduced_replay_refutes_a_counterexample | FR-030-AC-11, TC-041 |
| tests/it/terminal_map.rs | tc_041_a_setup_refusal_after_a_counterexample_is_replay_refused_never_declined | FR-030-AC-12, TC-041 |
| tests/it/terminal_map.rs | tc_041_a_repeated_dependency_identity_after_a_counterexample_is_replay_refused | FR-030-AC-13, TC-041 |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

**PASS** for the PR diff. The paired two-thread integration runs pass on both base and head. The author's seven-failure report under default test parallelism has not yet been classified per test; a separately queued base/head pair will record that result as an addendum. The trace migration passes the mechanical and source review: no new triple-slash Trace/Tracing lines, no Kani proof or production function tags, no executable test-code changes, no changed CI workflow, and no vendored content.

## Verification

- `cg-427-trace-equivalence.py`: all 62 migrated `(function, IDs)` entries match base doc lines exactly; every added attribute sits on a test function.
- After stripping trace attributes/imports and doc comments, executable source is byte-identical in all six files.
- `cargo fmt --all -- --check` and `git diff --check` pass.
- `quire coverage --scope . --strict`: 499/855 backed and 221 unbacked, 0 contradicted on both trees. Strict exits 1 for the same existing rows.
- `quire matrix --scope .`: identical after normalizing shifted binder line numbers.
- Base integration run with two test threads at 836b298: 594 passed, 0 failed, 34 ignored.
- Head integration run with two test threads at 4723978: 594 passed, 0 failed, 34 ignored.
- Default-thread paired runs, lint and deny: pending.
