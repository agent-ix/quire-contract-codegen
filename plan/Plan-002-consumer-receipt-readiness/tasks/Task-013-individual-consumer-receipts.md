---
id: Task-013
title: "IR-1114 — retain each genuine consumer receipt"
type: Task
status: blocked
track: A
priority: P1
relationships:
  - { target: ix://agent-ix/quire-contract-codegen/Task-012, type: depends_on }
  - { target: ix://agent-ix/quire-contract-codegen/FR-017, type: references }
  - { target: ix://agent-ix/quire-exact/FR-359, type: references }
  - { target: ix://agent-ix/quire-exact/FR-371, type: references }
  - { target: ix://agent-ix/quire-contract-codegen/TC-025, type: verifies }
  - { target: ix://agent-ix/quire-contract-codegen/TC-027, type: verifies }
---
# Task-013: Individual genuine consumer receipts

## Scope

After the release/adoption gate only, obtain the unchanged actual CG consumer evidence required by Exact FR-359-AC-10 and FR-371-AC-6. This task is not an immediate gate and remains blocked. Four current entries are in `tests/it/kani_obligations.rs`; the healthy wrapper is in `tests/it/kani_generation.rs`.

## Subtasks

- [ ] Preserve the per-test obligations below; select one genuine test at a time and verify a nonzero selected-test count. Use the original symbolic domains, oracle, unwind 3, Cadical, 16 GiB and 600-second ceilings, assertions, completion and counterexample checks. No instruction here authorizes executing them now.

| Exact normative consumer name | Current CG entry in `it` | Required individual result |
| --- | --- | --- |
| `tc_027_a_routed_scalar_harness_verifies` | `kani_generation::kani_generation_real_v2_scalar_verifies`, delegating `kani_obligations::verify_routed_scalar_harness_real_kani`; same proof entry moved by merged CG PR #601 | Verified with original completion cover; the existing missing-harness refusal control also holds. |
| `tc_027_a_routed_scalar_harness_violating_its_bound_is_falsified` | `kani_obligations::tc_027_a_routed_scalar_harness_violating_its_bound_is_falsified` | Falsified with the counterexample for asserted result bound 5 and unchanged original input draws. |
| `tc_025_scalar_harness_falsifies_a_mutated_oracle_arithmetic` | `kani_obligations::tc_025_scalar_harness_falsifies_a_mutated_oracle_arithmetic` | Healthy Verified, Add-to-Subtract mutant Falsified at the generated sound assertion, with the specified concrete counterexample. |
| `tc_027_attested_scalar_receipt_binds_real_proof_and_native_result` | `kani_obligations::tc_027_attested_scalar_receipt_binds_real_proof_and_native_result` | Both ordinary and zero-result-unit proof receipts Verified; native completed 5 and ResultUnits Incomplete at result-retain respectively, with retained content/accounting checks. |
| `tc_027_attested_scalar_subtraction_receipts_bind_both_operand_orders` | `kani_obligations::tc_027_attested_scalar_subtraction_receipts_bind_both_operand_orders` | Both orders Verified with native results 3 and -3 and retained two-operand domains. |

- [ ] Retain the genuine native-route healthy/bound/arithmetic controls required by CG FR-015-AC-126 separately when that route is the reviewed subject; source/ABI changes are disclosed and preserve semantic populations. They do not silently replace the five exact normative consumer obligations or waive an attested case.
- [ ] Record each actual backend result, process cleanup, completion cover, required counterexample and subject relationship separately. Missing cover/mutant/cleanup, memory exhaustion, timeout, unknown or compile failure is inconclusive; a healthy-only or split-subset result is not whole qualification.
- [ ] Independently assess all required individual observations. The first three controls address the shared FR-371-AC-6 consumer boundary; both attested cases are additionally needed for FR-359-AC-10. Other kernel/refinement criteria remain their owners' separate obligations. Send resulting CG PR merge requests to Luna with repo, PR and exact runtime head.

## Deliverables

Actual per-test external consumer receipts and an honest criterion-level disposition after independent review, or an explicit unqualified result naming each inconclusive/missing obligation. This plan, host tests and static source observations produce none of those receipts.

## Notes

Do not copy Exact harnesses into CG, represent consumer receipts by unrelated Exact trace tags, edit generated fixtures into private evaluators or reuse old verdicts for a changed kernel. Keep test code and required tags under their existing owners. No public API exists merely for receipt bookkeeping.
