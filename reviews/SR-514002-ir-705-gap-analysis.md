---
id: SR-514002
title: "IR-705 CG #514 gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen#514@27d2cfd086fa40a74d50af769f2542a908961bdf vs origin/main 4eccd682; spec/kani/functional/FR-015-bounded-kani-obligations.md (FR-015-AC-119, FR-015-AC-125, FR-015-AC-126, FR-015-AC-127), spec/replay/functional/FR-016-witness-native-replay.md (FR-016-AC-1, FR-016-AC-2, FR-016-AC-6), spec/assurance/AD-006-admitted-native-integer-oracles.md, src/, tests/it/"
review_set: subset
---
# SR-514002: IR-705 CG #514 gap analysis

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
- **AC3 (three original scalar proofs at 16 GiB / unwind 3 / Cadical): NOT complete.** All three real runs are pending, and this analysis claims none of them. SR-514001 FND-001 and FND-002 mean the native healthy proof subject is not the routed production oracle, and that the violating-bound control as authored cannot falsify.
- **AC4: pending.** Full gates and final-head review are still to come, and SR-514001 FND-003 shows `make test` fails at this head.

Old FND-005 recheck: corrected. `native_v2_helper_mutant_playback_decodes_typed_original_witness` traces FR-016-AC-1 (typed decoding). `native_v2_decoder_enforces_original_inclusive_bounds` traces FR-016-AC-2 (out-of-domain). No test claims FR-016-AC-6 for a positive witness, and no byte/value-limit control is laundered into AC-1.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-015-AC-127 is claimed by two tests that exercise none of its obligations. The AC requires end-to-end denied/cancelled admitted-native controls retaining every real Meter counter, charge order, complete Incomplete/refusal fields and admitted prefixes, compared with independent expectations and the general route. `native_observer_preserves_native_operands_result_and_admission_stop` checks observer substrings and Rust syntax. `completed_prefix_bounds_preserve_the_original_semantic_refusal` checks one `Refused(IntegerOutOfDomain)` outcome and one completed product under unlimited limits. Neither installs a denial or cancellation, reads a counter, or compares charge order. The computed trace still lists AC-127 as backed, which overstates the evidence. | src/kani/run/scalar.rs:1699, src/oracle/clause/native.rs:267, spec/kani/functional/FR-015-bounded-kani-obligations.md:874 |
| FND-002 | medium | The FR-015-AC-126 claimants do not back the falsifying TC-027 control they name. `native_original_tc027_false_bound_falsifies` and `native_scalar_false_bound_control_keeps_original_draws` regenerate with a changed semantic domain, not a changed independent bound (SR-514001 FND-002). Their static assertion, `exact <= i128::from(5_i64)`, is equally true of a healthy 0..5 claim. Once its real run is made, the matrix will report the AC-126 bound-mutant obligation as backed by a control that cannot be a substantive mutant. | tests/it/kani_obligations.rs:3040, tests/it/kani_obligations.rs:3073 |
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

## New findings (disposition pass 1)

Found at head `cb2c12e004002c2d08752d666c5c59486a4d9dc4` in the fix-round commits `dcfcebb9` and `cb2c12e0`.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | medium | The grouped branch of `publish_native_scalar_artifacts` has no test. That branch wraps each claim's oracle in `mod oracle_{index}`, re-exports it with `pub use`, and shifts every source-map region by a declaration-line offset. The new FR-015-AC-125 / AD-006 clause "for a group, each embedded oracle source is a byte-identical contiguous module body in routed src/lib.rs" is therefore an untested branch of the AC. The only `generate_routed_native` caller, `kani_scalar_native_fixture`, sends one item. The branch also moves `OracleStop` from the crate root into each per-claim module, and no test shows the general claims in a mixed native group still compile against their harness or source map. | src/oracle/scalar/render.rs:691, src/routed/generate.rs:585, tests/it/kani_scalar_native_fixture.rs:114 |
| FND-006 | low | `tc_024_ac42_only_the_shared_classifier_reads_a_failed_records_limit_kind` checks a fixed list of four scalar files (mod/derive/lower/render). It does not list the new `src/oracle/scalar/native.rs`. The current counts are still 1/2/0/0, and `native.rs` names `limit_kind` zero times. But a later `limit_kind` read or `Failed` arm in the new scalar submodule would escape the guard that FR-014-AC-42 relies on. | tests/it/exact_scalar_generation.rs:3624, src/oracle/scalar/native.rs:1 |

## Dispositions

Disposition pass 1 at head `cb2c12e004002c2d08752d666c5c59486a4d9dc4` (rebased onto `dd59aa14`, the #564 scalar split). `git range-diff 4eccd682..27d2cfd0 dd59aa14..cb2c12e0` shows the two reviewed commits are patch-identical (`a25cea4f = 0bed25e6`, `27d2cfd0 = c13bb455`), so their verdicts carry. The fix commits `dcfcebb9` and `cb2c12e0` were reviewed in full.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | Both misbound tests now trace only FR-015-AC-125. New non-ignored `routed_native_oracle_preserves_denial_cancellation_counters_and_refusal` compiles the routed production oracle. At each of the three charge points it runs injected denial and cancellation against the general exact route and compares every LimitKind counter, the admission count, and the full Incomplete record. It also checks the out-of-domain refusal after two admissions with zero result units. |
| FND-002 | fixed | The bound control now keeps `Bound::Integer(0, 9)` and edits only the harness literal from 9 to 5. The real-Kani run asserts that the generated sound assertion fails and decodes an original-domain witness with left + 1 > 5. |
| FND-003 | fixed | The helper-witness test now traces FR-016-AC-1 with TC-026, and the decoder control traces FR-016-AC-2 with TC-026. |
| FND-004 | fixed | The digest tautology no longer claims FR-015-AC-125. `native_scalar_increment_keeps_original_profile_and_checked_capacity` now asserts that the persisted harness record's `identity.nativeIntegerSelection` equals the typed selection. A change to how the identity binds the selection would fail it. |

Disposition pass 2 at head `6d405d29c7255654da4e96376bfaebcda9b0e7ee` (repair commit `8dca8c01`). Checked against the code at that head.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-005 | fixed | New non-ignored lib test `grouped_native_publication_keeps_each_proof_body_and_source_map_aligned` (`#[trace("FR-015-AC-125")]`) publishes two native claims through `publish_native_scalar_artifacts`. It asserts that `src/lib.rs` starts with `mod oracle_0 {`, that `OracleStop` appears once per module, and that each claim's `oracle_source` is a contiguous `mod oracle_{index} { … } pub use oracle_{index}::{symbol};` body. It also checks that every shifted source-map region lands on its own published declaration line. The test passes. A mixed native-plus-general group remains uncompiled, but this covers the AC's group byte-identity clause. |
| FND-006 | fixed | `tc_024_ac42` (and `tc_024_ac39`) now include `src/oracle/scalar/native.rs`, with an expected `limit_kind` count of 0. `tc_024_ac42` passes. |

Disposition pass 3 at head `ff42c8496613f5f297e5ca8e5bb9bf90898d7713` (rebased onto `1944268f`). No findings are open, so no rows are added. The rebase-only adaptations to #570's typed output reader were reviewed and add no gap or spec defect.
