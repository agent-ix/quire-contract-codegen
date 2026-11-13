---
id: SR-17940486709564847047931
title: "IR-1530 PR #816 gap-analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen PR #816 at c17e1ee3 (git diff origin/main...HEAD); IR-1530 outcome contract; tests/it/kani_obligations.rs, tests/it/kani_generation.rs, tests/it/kani_scalar_native_private.rs, tests/it/kani_scalar_native_fixture.rs, src/routed/generate.rs, src/oracle/scalar/native.rs, src/kani/generate/scalar.rs, docs/qualification/ir1530-general-route-scalar-not-tractable.md, spec/kani/functional/FR-056-kani-tractability-refusal.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-056
    type: reviews
---

## Summary

Ticket: IR-1530. Plan completion: not assessed. The ticket's outcome contract (treated as data):
each of the three tests ends `Verified`/`Falsified` as asserted in a recorded executed run, or the
general integer scalar route is recorded as a measured not-tractable row, its tests move to the
native-integer route of CG #809, and the typed refusal is IR-1526's mechanism; no weakening of the
assertions.

What the PR does: four `#[ignore]` kani-lane tests switch from `routed_scalar_increment*`
(`generate_routed`, `rt::Integer`) to `native_scalar_increment*` (`generate_routed_native`). Every
assertion is kept. The `tc_025` mutation site moves from `rt::IntegerArithmetic::Add(left, right)` to
`nx::NativeIntegerOperation::Add {`. One assertion becomes stricter (the launcher argument vector).
A qualification record states the general route not tractable.

Is the move a weakening? Not of the assertions. The `tc_025` mutant still edits the embedded
production oracle: `src/oracle/scalar/native.rs:116` emits the `NativeIntegerOperation` descriptor
inside the oracle function, and the harness's `exact` stays an independent `i128` expression
(`src/kani/generate/scalar.rs:1094-1101`). `NativeIntegerPlan::new` accepts `Subtract` over the same
intervals, so the mutant computes `x - 1`, not a plan error. The failing check is the `sound`
assertion, and the test pins its message. In the `tc_027` bound mutant the harness's `admitted`
bound is narrowed to 5 while the native oracle completes up to 9. That is the same property as on
the general route. Matrix bindings are unchanged.

What it does lose: the general route (`generate_routed`, still public and the default entry point,
`src/routed/generate.rs:335`) no longer has a real-Kani verdict for `x + 1` in the lane. The moved
tests duplicate controls that already exist on the native route
(`native_original_tc027_increment_verifies`, and the private `native_tc025_arithmetic_mutant_falsifies`
and `native_original_tc027_false_bound_falsifies`), so the move adds no new native coverage. This
follows from the not-tractable branch of the contract, and the record states it. Whether that
branch is fully met is FND-001.

Evidence for the not-tractable row: the base `TimedOut` at 600 s was not re-run for this PR. It is
taken from the IR-304 record, which measured it (14.5 GB at 8 min). The IR-304 reviewer (SR-751
disposition round 1) reproduced it independently: `tc_027_a_routed_scalar_harness_violating_its_bound_is_falsified`
reached `TimedOut` at 600.24 s on a general harness byte-identical to main. The ticket also says a
reviewer reproduced it. The forget-recipe row is a fresh probe for this PR, and it repeats the
IR-304 first-attempt OOM (24 GiB within about a minute). Two independent base measurements and two
OOM observations are enough for "not tractable at unwind 3 / 600 s / 24 GiB", the only configuration
the row claims. The probe log itself is thin (code-review SR FND-003).

Reviewer reproduction (2 of 4, PR worktree, `flock /tmp/cargo-slot.lock`,
`systemd-run --user --scope -p MemoryMax=24G -p MemorySwapMax=0`, foreground, one at a time,
`CARGO_TARGET_DIR=<worktree>/target`, `QUIRE_CVC5_BIN` set):
`tc_025_scalar_harness_falsifies_a_mutated_oracle_arithmetic` ok in 36.06 s, peak 2.50 GB;
`tc_027_a_routed_scalar_harness_violating_its_bound_is_falsified` ok in 28.03 s, peak 2.98 GB. Both
match the record (31 s / 2.37 GB, 28 s / 2.98 GB).

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | high | The contract's not-tractable branch needs "the typed refusal from IR-1526". The record and PR body say the typed refusal of general-route integer scalars "belongs to the IR-1526 seam" / "stays with IR-1526". It does not: FR-056 (IR-1526) is scoped to V2 clauses (`V2ClauseDescriptor.operation_identities`, after the FR-015-AC-42 operator check), and IR-1526 lists "scalar-claim forms beyond what the descriptor already exposes" as not in scope. So `generate_routed` still emits, with no refusal, a general integer scalar harness this PR records as not tractable, and no ticket owns refusing it. Either file the follow-up (route the scalar to a typed refusal or default it to the native route) through the planner per the triage rule, or amend FR-056's scope, and correct the record's sentence to name the real owner | docs/qualification/ir1530-general-route-scalar-not-tractable.md:17-19, spec/kani/functional/FR-056-kani-tractability-refusal.md:15-41, src/routed/generate.rs:335-343 |
| FND-002 | medium | The "measured not-tractable row" exists only as prose in a qualification doc. It has none of the fields the IR-1526 measured table loads: `operation_ids`, `max_operand_bits`, `collection_bound`, `verdict`, `wall_seconds`, `peak_bytes`, Kani/CBMC/solver versions, and generator and kernel versions. The CBMC version (6.11.0 on this host) and the generator/kernel versions are missing even from the prose. No decider can consume the row, and its staleness cannot be judged. Record the row in the table's shape (or in the probe-table source), with versions | docs/qualification/ir1530-general-route-scalar-not-tractable.md:7-16 |

## Verdict

The assertions are not weakened. The four moved tests test the same property on the native carrier,
and two of them reproduce. The general route keeps no real-Kani `x + 1` coverage, and the record says
so honestly. The not-tractable evidence is adequate for the one configuration it claims. The
contract's not-tractable branch is not fully met, though: the typed-refusal half rests on an owner
whose scope excludes it (FND-001, needs a planner decision), and the row is not in a usable form
(FND-002). Not mergeable until FND-001 is resolved, either by a filed and owned follow-up that the
record cites or by a scope amendment.

## Dispositions

Round 1, reviewed head 7be5af40 (four commits on c17e1ee3: 61a49047, 10bd2b48, d3690eef, 7be5af40). Reviewer runs at this head (PR worktree, `flock /tmp/cargo-slot.lock`, `systemd-run --user --scope -p MemoryMax=24G -p MemorySwapMax=0`, foreground, one at a time; log `/tmp/claude-1000/-home-peter-dev/946968da-15f0-4263-9043-0b20b68f7193/scratchpad/rev1530-r1.log`): `tc_027_attested_scalar_subtraction_receipts_bind_both_operand_orders` ok 19.56 s, peak 0.94 GB; private lib `routed::scalar_native_private::seeded_false_bound_harness_produces_a_paired_completion_refusal` ok 31.68 s, peak 3.10 GB.

Ruling relayed by the coordinator: option 3, the refusal is not built in this PR. The coder's
generation-level experiment (a `GeneralIntegerScalarNotTractable` arm in `negotiate_integer_policy`
for non-native Add/Subtract/Multiply/Negate) broke 3 `--lib` and 81 `--test it` tests. The reviewer did
not re-run it. The claim is credible: there are 58 `generate_routed(` call sites in `src/` and
`tests/`, and the shared general fixtures feed host-only harness-text, record, playback, replay and
settlement tests that never run Kani. The per-file distribution matches that pattern. So the argument
that refusing at generation is the wrong layer holds: it would remove host evidence that does not
depend on Kani tractability.

| FND | outcome | reason |
|---|---|---|
| FND-001 | deferred | The typed refusal is outside this PR: FR-056/IR-1526 is V2-clause only, and a generation-level refusal is the wrong layer (84-test blast radius, recorded). The record and PR body now say plainly that refusal is not built and is a proposed run/verdict-level follow-up awaiting the planner's ruling. The false claim that it "stays with IR-1526" is removed. IR-1530 must stay In Progress (not Done) until the planner rules and either a follow-up ticket exists or the contract clause is dropped |
| FND-002 | fixed | The record now carries a table-shaped row: operation_ids `quire.op.integer.add`/`quire.op.integer.sub` (matching the catalog ids in `src/`), max_operand_bits 4, collection_bound none, verdict not_tractable, wall 600, peak bytes, Kani 0.68.0 / CBMC 6.11.0 / cadical / unwind 3, generator main bb3e6176, kernel quire-exact e82db487 (matches Cargo.lock), host and date |
