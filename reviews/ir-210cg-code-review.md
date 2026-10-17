---
id: SR-12080
title: "IR-210 CG slice 1 code review (rust lane)"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen@52f9a65c2e9e31f8726096837ad4ff684b9c74cf; tests/exact_scalar_support/agreement_cases.rs, tests/it/bound_strategy_generation.rs, tests/it/capability_settlement.rs, tests/it/composite_equality_generation.rs, tests/it/exact_function_generation.rs, tests/it/exact_scalar_generation.rs, tests/it/harness_generation.rs, tests/it/oracle_generation.rs, tests/it/strategy_generation.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-002
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-021
    type: references
  - target: ix://agent-ix/quire-contract-codegen/NFR-001
    type: references
  - target: ix://agent-ix/quire-contract-codegen/NFR-002
    type: references
---

## Summary

Ticket: IR-210 (CG slice 1, non-Kani), PR quire-contract-codegen#414, head 52f9a65 over origin/main 9446ca9 (the merge base). The PR is one commit touching 9 test files (+33/-29). Every changed line is a `///` doc line except one: a function rename in `tests/exact_scalar_support/agreement_cases.rs` from `tc_026_integer_add_refuses_a_result_leaving_its_declared_domain` to `integer_add_refuses_a_result_leaving_its_declared_domain`. No test body or assertion changed. `make kani-scope` against origin/main reports "not required". The rust-review lane is folded into this file. Every added id was read against its criterion text in `spec/**` and against the body of the test that now carries it. Coverage accounting is in the gap-analysis artifact (SR-12081).

## Reviewed scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| NFR-001-AC-1 | examined | Identical inputs and configuration produce byte-identical bundles. |
| NFR-002-AC-3 | examined | Unsupported and inconclusive states remain explicit rather than silently succeeding. |
| FR-002-AC-1 | examined | Rejected inputs remain distinct from passing cases in generated reports. |
| FR-002-AC-2 | examined | Boundary strategies exercise immediately inside and outside supported constraints. |
| FR-002-AC-5 | examined | The generated campaign runner owns execution, explicit-discard accounting, and conclusion, returns a typed terminal result, and rejects campaigns below either configured coverage floor or above the explicit-discard ceiling; exhausted searches retain their reason and any missed floor without becoming a completed floor failure. |
| FR-002-AC-6 | examined | Integer and generated Boolean harness campaign cases bind exact values to executable accepted/rejected expectations; enum cases execute an all-admitted membership expectation. |
| FR-021-AC-33 | examined | With an IR-admitted temporal/formula, temporal/fairness or expression/case node used directly as an item's call-node id, the claim-map entry retains that id and is Refused { refusal: ExactFunctionRefusal::Unsupported } ... for a lowered case root they name that root and "expression". |
| FR-021-AC-32 | context_only | Neither the schedule crate nor eventual concrete oracle requires quire_contract_runtime::exact, an alias, wrapper or compatibility layer ... (Inspection) |
| FR-021-AC-1 | examined | Every requested call item receives exactly one scheduled or typed-refused disposition; a refused item contributes no instruction root, concrete symbol or location entry, while an unrelated admitted item is unchanged. |
| FR-021-AC-12 | examined | A declared function whose own body fails to lower ... refuses every item bound to that function with a typed reason, without changing an unrelated package's items. |
| FR-021-AC-15 | examined | Each scheduled call point has one constant-size location record with function origin, stable occurrence ordinal, parent ordinal, child ordinal and instruction slot. ... |
| FR-021-AC-17 | examined | Each location record uses the request function binding and function-body ordering for its origin; ... PLANNED (QSL-358/IR-349 for owner-authored cross-check). |
| FR-021-AC-27 | examined | An admitted nested body consists of separate V2 expression nodes connected only by reference leaves ... callee argument 0 binds an exact V2 function node, parameter references bind the declared parameter slots, and remaining arguments preserve source order. ... |
| FR-018 | examined | If a node is unlowered, invalid, over its work limit, of a form other than binary, or disagrees with its descriptor's arity or operand types, then the generator shall refuse the item with a typed reason. (Behavior) |
| FR-019-AC-26 | context_only | For any form in FR-019-AC-25 with one validated Kani candidate, a bounded extent settles supported exactly when the form's FR-015 lowering succeeds and the candidate advertises the bounded mode; ... |
| FR-011-AC-5, FR-012-AC-4, FR-013-AC-1, FR-031-AC-10, FR-031-AC-18, TC-044 | context_only | Ids on the two bound_strategy_generation wrapped lines; only their doc-line form changed. |
| TC-001 | examined | Verify deterministic derivation |
| TC-002 | examined | Compile generated code and publish atomically |
| TC-003 | examined | Reject unsupported or invalid constructs explicitly |
| TC-004 | examined | Verify shaped proptest strategies |
| TC-026 | examined | Verify witness decoding and native replay |
| TC-029 | examined | Verify composite equality oracle generation and three-way agreement |
| TC-030 | examined | Verify capability settlement at one negotiation point |
| TC-031 | examined | Verify function-application oracle generation, agreement, and static location tagging |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | `tc_031_ac32_direct_control_case_is_unsupported` now has the doc line `Trace: FR-021-AC-33, TC-031`, which is correct for its body, but the function name still says `ac32`. FR-021-AC-32 is the unrelated Inspection criterion about the `runtime::exact` re-export. A reader who trusts the name misreads the binding. The PR renames `tc_026_...` for exactly this kind of name/claim mismatch, but leaves this one. | tests/it/exact_scalar_generation.rs:2411-2414 |
| FND-002 | low | The reworded prose changes what AC-17 means in the doc line. The old line `FR-021-AC-15/AC-17 (duplicate declared names)` claimed AC-15 and AC-17 together. The new line `FR-021-AC-15, TC-031 (AC-17 duplicate declared names)` reads as if AC-17 were the duplicate-declared-names criterion, but AC-17 is about the location origin coming from the request function binding. The body does assert that origin (`RecordedOrigin::Body { function, index }` per survivor). The parenthetical should either name AC-17's real subject or be dropped. AC-17 has never bound on this test: it is an `evidence_near_miss` citation on both base and head, so no binding changed. | tests/it/exact_function_generation.rs:2503 |

## Verdict

APPROVE WITH NITS. Two low findings, both about doc wording and naming. Neither changes a binding or an assertion. Mergeable on this method; FND-001 and FND-002 can be fixed in this PR's fix round as two one-line edits.

Per-binding judgement (added ids, each checked against the body):

- **NFR-001-AC-1 on `tc_002_integer_and_state_comparisons_are_deterministic_compile_and_match_the_model`: correct.** Each of the six numeric/state models is generated twice from the same request, and the test asserts `assert_eq!(bundle, repeated, ...)`. TC-002's procedure states this leg outright ("Generate every numeric/state case twice from identical inputs and require byte-identical Rust and source-map artifacts"), and TC-002 `verifies` NFR-001.
- **NFR-002-AC-3 on the five tests: correct.**
  - `oracle_generation::tc_003_unsupported_dependency_reports_the_first_reference_span` gives `UnsupportedDependency` at the reference span.
  - `oracle_generation::tc_003_source_size_limit_rejects_without_a_partial_bundle` gives one `ResourceLimitExceeded` with `terminal_state == Unsupported`.
  - `harness_generation::tc_004_invalid_execution_point_is_a_structured_failure_without_artifact` (:712) covers `InvalidExecutionPoint`/`InvalidInput`, then `ResourceLimitExceeded` with terminal `Unsupported`.
  - `harness_generation::tc_004_multiple_state_bindings_fail_closed` (:850) gives `UnsupportedHarnessBinding` with terminal `Unsupported`.
  - `strategy_generation::tc_004_invalid_shapes_fail_with_structured_diagnostics` covers `UnsupportedCampaignConstraint` with terminal `Unsupported` and the resource limit with terminal `Unsupported`.

  Each one asserts an explicit typed refusal and never a success. `tc_004_invalid_campaign_inputs_fail_before_clause_generation` asserts only `InvalidInput` and was correctly left without NFR-002-AC-3.
- **FR-002-AC-1, AC-5 and AC-6 on `harness_generation::tc_004_generated_harness_binds_clauses_and_executes_all_three_terminal_paths` (:156): correct.**
  - AC-1: `VerdictKind::RejectedPrecondition` and `report.counts().rejected()` are counted apart from accepted.
  - AC-5: the generated `run_campaign` owns the conclusion. The test covers the `BelowAcceptedFloor`/`BelowRejectedFloor`/`AboveDiscardCeiling` typed errors, a prior-discard ceiling, a zero-case floor, and `Exhausted { reason, policy }` keeping both the reason and the missed floor.
  - AC-6: the Boolean leg is covered, since `accepted_case(true, true)`/`rejected_case(false, true)` bind exact values, and the mismatched `rejected_case(true, true)` fails. The integer and enum legs are covered by the strategy tests.
- **FR-002-AC-2 on `strategy_generation::tc_004_boundary_campaigns_tag_inside_and_immediately_outside_values`: correct.** The range boundary emits exactly 9/10/20/21. Membership emits the adjacent values 2/3/4/6/7/8, residual exclusion emits 3/4/5/7/8/9, and the correlated boundary emits inside and outside pairs with at least 4 Accepted and at least 4 Rejected.
- **FR-002-AC-6 on `tc_004_boundary_pinned_and_no_event_bodies_compile_and_execute_with_denied_warnings`: correct.** The generated integer boundary case runs `expected.verify(...)` against a verdict computed independently from the domain.
- **FR-021-AC-33 on `tc_031_ac32_direct_control_case_is_unsupported`: correct, partial.** The test asserts the direct `case` call-node leg: `Refused { Unsupported { unsupported_node_id == id, node_tag: "expression" } }`. It does not assert the healthy-sibling byte-identity leg, which the `tc_031_ac33_*` tests own. See FND-001 for the name.
- **FR-018 + TC-029 on `tc_029_a_body_operand_disagreeing_with_the_descriptor_refuses_before_generation`: correct.** FR-018 has no numbered AC for operand-type disagreement; the Behavior clause states it. The FR-018 binding already existed on main through `rust-trace-line`, and the original prose is kept word for word as the next paragraph. Only TC-029 moved from the name form to the trace line.
- **TC-030 on `tc_030_the_advertised_mode_table_settles_each_row`: correct.** It is a TC-030 test, and only the binding form moved.
- **The six TC-031 prose rewrites in exact_function_generation.rs: no change in meaning except FND-002.**
  - AC-1 "(duplicate handling, matching FR-014/FR-018)" is kept.
  - AC-12 "(signature/body-kind agreement)" is kept.
  - AC-1 "(item-level arity)" is kept.
  - The AC-15 "(origin half only -- see module doc ... vocabulary)" parenthetical is kept intact.
  - AC-27 has TC-031 appended.
  - `tc_031_scalar_body_operator_must_match_checked_operation_identity` has a new `Trace: TC-031` line.
- **bound_strategy_generation wrapped lines: correct.** Restarting each continuation line with `/// Trace:` moved TC-044, FR-031-AC-10, FR-031-AC-18, FR-011-AC-5, FR-012-AC-4 and FR-013-AC-1 from `rust-comment-id` to `rust-trace-line`. The set of ids is unchanged.
- **TC-001/TC-002/TC-003/TC-004 conversions from bare `/// TC-00N.`: correct.** Every converted test's name carries the same TC number, or the TC (TC-003 on the tc_004 failure tests) was already present as a bare doc id.
- **Rename: correct.** The function's own comment opens "Deliberately untraced: this exists as `render_scalar`'s ... F1 mutation-testing vehicle". The `tc_026_` prefix was a false TC-026 (witness native replay) claim made through `rust-test-name-id`. `grep -rn` over the whole worktree (src, tests, scripts, spec, Makefile; there is no scratch-crate runner reference) finds only the new definition. TC-026 still has 47 binding symbols, down from 48, and stays backed.

Rust lane:

- **Format.** `rustfmt --edition 2021 --check tests/exact_scalar_support/agreement_cases.rs` exits 0. The other edits are doc lines only.
- **Convention.** The legacy `/// Trace: ID, ID` doc line is used throughout, per the planner decision, with no attributes. The ids are comma-separated, and prose parentheticals come after the id list. `quire trace --file` on head shows every added id binding as `rust-trace-line`.
- **Surface.** The diff has no panic, unsafe, integer-conversion or behavioral surface. The rename does not change test selection: the function is a `#[test]` in a support module, and nothing filters on its name.
- **Gates.** No mutants were run. No body changed, so a mutant would only re-measure tests that already exist on main.
