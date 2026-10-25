---
id: SR-17016
title: "IR-705 CG #514 gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen#514@27d2cfd086fa40a74d50af769f2542a908961bdf vs origin/main 4eccd682; spec/kani/functional/FR-015-bounded-kani-obligations.md (FR-015-AC-119, FR-015-AC-125, FR-015-AC-126, FR-015-AC-127), spec/replay/functional/FR-016-witness-native-replay.md (FR-016-AC-1, FR-016-AC-2, FR-016-AC-6), spec/assurance/AD-006-admitted-native-integer-oracles.md, src/, tests/it/"
review_set: subset
---
# SR-17016: IR-705 CG #514 gap analysis

## Summary

Ticket: IR-705. Plan completion: not assessed (no plan supplied). Planless audit of the computed trace data (`quire trace --scope . --id …`, `quire coverage --scope .`) against the tests' actual behaviour at exact head `27d2cfd0`.

Traceability state at this head:

- **FR-015-AC-125:** nine claiming tests.
- **FR-015-AC-126:** seven claiming tests. Four are ignored real-Kani or external-artifact tests and have not run.
- **FR-015-AC-127:** two claiming tests.
- **FR-016-AC-1 and FR-016-AC-2:** each gains one new claimant.

Reverse gap: every new production item is owned by FR-015 / AD-006 or FR-016 (witness decoding):

- `negotiate_kani_native_obligations`
- `generate_v2_clause_native_oracle`
- `decode_v2_clause_falsification`
- `NativeIntegerSelection` / `NativeIntegerCapacityPolicy`
- `ScalarIntegerRepresentation`
- the V2 postcondition `prove_clause` harness (FR-015-AC-119)

Stub scan: no `todo!`/`unimplemented!` and no placeholder return on a live path.

IR-705 acceptance status measured at this head:

- **AC1 / AC2 (kernel representation and refinement): consumed, not re-implemented.** CG calls the merged quire-exact native API through the unchanged lock. Add/Subtract/Multiply/Negate selection runs through `NativeIntegerPlan::new` (`native_selection_uses_owner_plan_for_all_arithmetic_operations`). The signed32 Multiply refusal is tested. Kernel refinement evidence belongs to the kernel PRs and is not re-proved here.
- **AC3 (three original scalar proofs at 16 GiB / unwind 3 / Cadical): NOT complete.** All three real runs are pending, and this analysis claims none of them. SR-17015 FND-001 and FND-002 mean the native healthy proof subject is not the routed production oracle, and that the violating-bound control as authored cannot falsify.
- **AC4: pending.** Full gates and final-head review are still to come, and SR-17015 FND-003 shows `make test` fails at this head.

Old FND-005 recheck: corrected. `native_v2_helper_mutant_playback_decodes_typed_original_witness` traces FR-016-AC-1 (typed decoding). `native_v2_decoder_enforces_original_inclusive_bounds` traces FR-016-AC-2 (out-of-domain). No test claims FR-016-AC-6 for a positive witness, and no byte/value-limit control is laundered into AC-1.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-015-AC-127 is claimed by two tests that exercise none of its obligations. The AC requires end-to-end denied/cancelled admitted-native controls retaining every real Meter counter, charge order, complete Incomplete/refusal fields and admitted prefixes, compared with independent expectations and the general route. `native_observer_preserves_native_operands_result_and_admission_stop` checks observer substrings and Rust syntax. `completed_prefix_bounds_preserve_the_original_semantic_refusal` checks one `Refused(IntegerOutOfDomain)` outcome and one completed product under unlimited limits. Neither installs a denial or cancellation, reads a counter, or compares charge order. The computed trace still lists AC-127 as backed, which overstates the evidence. | src/kani/run/scalar.rs:1699, src/oracle/clause/native.rs:267, spec/kani/functional/FR-015-bounded-kani-obligations.md:874 |
| FND-002 | medium | The FR-015-AC-126 claimants do not back the falsifying TC-027 control they name. `native_original_tc027_false_bound_falsifies` and `native_scalar_false_bound_control_keeps_original_draws` regenerate with a changed semantic domain, not a changed independent bound (SR-17015 FND-002). Their static assertion, `exact <= i128::from(5_i64)`, is equally true of a healthy 0..5 claim. Once its real run is made, the matrix will report the AC-126 bound-mutant obligation as backed by a control that cannot be a substantive mutant. | tests/it/kani_obligations.rs:3040, tests/it/kani_obligations.rs:3073 |
| FND-003 | low | `native_v2_decoder_enforces_original_inclusive_bounds` traces FR-016-AC-2 together with `TC-025`. FR-016's table assigns AC-1 to AC-6 to `Test (TC-026)`. The positive helper-witness test carries no TC id. Suite attribution in the computed matrix is wrong for this binding. | tests/it/kani_obligations_state_frame.rs:608, spec/replay/functional/FR-016-witness-native-replay.md:134 |
| FND-004 | low | `native_policy_revision_changes_canonical_identity_without_changing_domains`, which claims FR-015-AC-125's "changing that policy changes identity", edits a `serde_json::Value` and compares the two digests. Any two distinct JSON values give different digests. No source change to how the harness record or identity binds `native_integer_selection` could make it fail. Only one policy variant exists, so the identity-binding clause has no test that can fail. | src/core/native_integer.rs:88 |

## Bindings

| Test | AC | Trace |
| --- | --- | --- |
| native_v2_helper_mutant_playback_decodes_typed_original_witness | FR-016-AC-1 | correct |
| native_v2_decoder_enforces_original_inclusive_bounds | FR-016-AC-2 | correct |
| native_selection_uses_owner_plan_for_all_arithmetic_operations | FR-015-AC-125 | correct |
| native_selection_refuses_complete_multiply_population_outside_signed32 | FR-015-AC-125 | correct |
| native_scalar_increment_keeps_original_profile_and_checked_capacity | FR-015-AC-125 | correct |
| native_observer_preserves_native_operands_result_and_admission_stop | FR-015-AC-127 | wrong |
| completed_prefix_bounds_preserve_the_original_semantic_refusal | FR-015-AC-127 | wrong |
| native_original_tc027_false_bound_falsifies | FR-015-AC-126 | wrong |

## Verdict

**FAIL WITH FINDINGS.** Production ownership is complete, and the kernel is consumed rather than copied. Two problems remain:

- FR-015-AC-127 is backed only by tests that cannot fail for its subject (FND-001).
- The AC-126 bound-mutant claim rests on a control that cannot falsify (FND-002).

IR-705 AC3 is unproven at this head. AC-126's IR-948 invariant and impossible-precondition CoverUnsatisfied obligations are outside this PR's IR-705 slice and stay open.
