---
id: SR-17940486709564847047930
title: "IR-1530 PR #816 code-review (rust-review lane)"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen PR #816 at c17e1ee3 (git diff origin/main...HEAD); IR-1530; tests/it/kani_obligations.rs, tests/it/kani_generation.rs, tests/it/kani_scalar_native_private.rs, tests/it/kani_scalar_native_fixture.rs, docs/qualification/ir1530-general-route-scalar-not-tractable.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-027
    type: reviews
---

## Summary

Ticket: IR-1530. The diff is test-only plus one doc. `native_scalar_increment_with_artifacts()` is
split out of `native_scalar_increment()`. Four kani-lane tests switch to it:
`kani_generation_real_v2_scalar_verifies` (through `verify_routed_scalar_harness_real_kani`),
`tc_027_a_routed_scalar_harness_violating_its_bound_is_falsified`,
`tc_025_scalar_harness_falsifies_a_mutated_oracle_arithmetic` and
`tc_027_attested_scalar_receipt_binds_real_proof_and_native_result`. The `tc_025` mutation site
becomes `nx::NativeIntegerOperation::Add {` -> `Subtract {`, and the argument-vector check becomes
exact (`kani`, the options, `-Z unstable-options --export-json`, path; length `options + 5`).

rust-review lane checks, no defect: no `src/` change, so generated text is byte-identical to main;
`rustfmt --check` passes; the test binary builds at the head without warnings. The mutation site is
asserted unique (`matches(call).count() == 1`) and sits in the embedded production oracle
(`src/oracle/scalar/native.rs:116`), not in the harness's `i128` reference. `NativeIntegerPlan::new`
accepts `Subtract` over the same intervals, so the mutant fails on the arithmetic, not on a plan
error, and the test pins the `sound` assertion message. The stricter argument check is correct
against `src/kani/run` (the coder's first run failed it with 16 vs 12, which is the stale `+1` count
the timeout had hidden). `routed_scalar_increment*` stays in use by host tests, so no dead code.

Reviewer runs at c17e1ee3 (PR worktree, `flock /tmp/cargo-slot.lock`,
`systemd-run --user --scope -p MemoryMax=24G -p MemorySwapMax=0`, foreground, one Kani at a time,
`CARGO_TARGET_DIR=<worktree>/target`, `QUIRE_CVC5_BIN` set; log
`/tmp/claude-1000/-home-peter-dev/946968da-15f0-4263-9043-0b20b68f7193/scratchpad/rev1530-repro.log`):
`tc_025_scalar_harness_falsifies_a_mutated_oracle_arithmetic` ok 36.06 s, peak 2.50 GB;
`tc_027_a_routed_scalar_harness_violating_its_bound_is_falsified` ok 28.03 s, peak 2.98 GB.
Both match the record. Also run, for FND-001:
`tc_027_attested_scalar_subtraction_receipts_bind_both_operand_orders` FAILED,
`ProofNotVerified(... outcome: Inconclusive { reason: TimedOut } ...)` at 600.33 s, cgroup peak
15.0 GB (log `.../scratchpad/rev1530-subtraction.log`).

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | high | One general-route real-Kani scalar test is still in the `make kani` lane (`kani_obligations` filter) and is red for the same reason: `tc_027_attested_scalar_subtraction_receipts_bind_both_operand_orders` builds its harness with `routed_scalar_binary_subtraction_with_artifacts()` (`generate_routed`, `rt::Integer`). The reviewer run ended `ProofNotVerified(Inconclusive { TimedOut })` at 600.33 s, peak 15.0 GB. The PR records the general integer scalar route as not tractable, but leaves this FR-017-AC-31 test (which TC-027 calls mandatory) on that route, with no row for it in the record. Move it to the native route (`routed_native_scalar_with_artifacts` over `bounded_binary_subtraction_package`), keep its assertions, and record its executed verdict, in this PR | tests/it/kani_obligations.rs:2371-2381, tests/it/kani_obligations.rs:2626-2653 |
| FND-002 | low | The `#[ignore]` private lib controls `cleanup_success_and_failure_preserve_real_receipt_after_snapshot_removal`, `seeded_false_bound_harness_produces_a_paired_completion_refusal` and `bounded_paired_runner_propagates_the_native_helpers_oracle_stop` run real Kani on `fixture()`, which is the same general `x + 1` route (`routed_scalar_with_artifacts`, 600 s). They are outside `make kani`, but by this PR's own record they cannot end as asserted. The record should list them (moved, or named as not runnable on the general route), so the not-tractable row states every test it affects | tests/it/kani_scalar_native_private.rs:209-217, tests/it/kani_scalar_native_private.rs:236-238, tests/it/kani_scalar_native_private.rs:402-411, tests/it/kani_scalar_native_private.rs:446-453 |
| FND-003 | low | The forget-recipe probe's only log (`p1530-gen_forget.log`) holds no test or Kani output. It has a shell quoting error (`Invalid environment variable name evaluates to an empty string: PIPESTATUS[0]`), then `wall=50s peak=25769803776`. "Killed at the 24 GiB cgroup limit" is inferred from `memory.peak` equal to `MemoryMax`, with no OOM line, and the probe's harness edit is not recorded. The record also leaves out where its logs live, and the CBMC version. State the inference, the edited harness lines and the versions, so the row can be re-run | docs/qualification/ir1530-general-route-scalar-not-tractable.md:13-16 |

## Verdict

The four moved tests are correct, keep every assertion (one is stricter), and two of them reproduce.
Not mergeable at c17e1ee3: FND-001 leaves a red general-route test in the same lane the PR claims to
make green, with the same cause. FND-002 and FND-003 are record completeness, fixable in the same
round.

## Dispositions

Round 1, reviewed head 7be5af40 (four commits on c17e1ee3: 61a49047, 10bd2b48, d3690eef, 7be5af40). Reviewer runs at this head (PR worktree, `flock /tmp/cargo-slot.lock`, `systemd-run --user --scope -p MemoryMax=24G -p MemorySwapMax=0`, foreground, one at a time; log `/tmp/claude-1000/-home-peter-dev/946968da-15f0-4263-9043-0b20b68f7193/scratchpad/rev1530-r1.log`): `tc_027_attested_scalar_subtraction_receipts_bind_both_operand_orders` ok 19.56 s, peak 0.94 GB; private lib `routed::scalar_native_private::seeded_false_bound_harness_produces_a_paired_completion_refusal` ok 31.68 s, peak 3.10 GB. The only `src/` change is in `#[cfg(test)] false_bound_harness_for_test`, which now copies `native_integer_selection` from the original harness. A general original still carries `None`, so its output is unchanged.

| FND | outcome | reason |
|---|---|---|
| FND-001 | fixed | The subtraction receipt test uses the new `native_scalar_binary_subtraction_with_artifacts()` (`routed_native_scalar_with_artifacts`, same package, code 49_102 and `Int[-9, 9]`) with every assertion kept; reviewer run Verified in both orders, 19.56 s; record row added |
| FND-002 | fixed | The three private controls use `native_fixture()` (same package and code 49_190, native route); the codegen-only control at line 524 stays general (no verification). Reviewer ran `seeded_false_bound_harness_produces_a_paired_completion_refusal`: ok 31.68 s. The record lists all three with verdicts |
| FND-003 | fixed | The probe was re-run and is recorded: exact generator edit (unguarded forgets), fixture swap, scope OOM-kill at 57 s per the user journal (`Failed with result 'oom-kill'`, cbmc anon-rss 25.0 GB), Kani 0.68.0 / CBMC 6.11.0, edits reverted; the earlier log-less probe is named as superseded |
