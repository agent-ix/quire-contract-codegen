---
id: SR-17940486709564847047986
title: "IR-260 PR #834 gap-analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen PR #834 at 55457462, git diff origin/main...HEAD; IR-260 and IR-262; FR-053-AC-1 through FR-053-AC-14, FR-049-AC-8, FR-049-AC-9, TC-067, TC-063; tests/it/float_scalar_harness.rs, tests/it/enum_scalar_harness.rs, src/kani/generate/float_scalar.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-053
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-067
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-049
    type: reviews
---

## Summary

Ticket: IR-260 (IR-262 for FR-049-AC-8/AC-9). Plan completion: not assessed. This is a manual
check of each AC against the tests in the diff. The brief excluded `quoin matrix` and
`quire matrix --strict`, so neither was run. No new production code lacks an owning criterion
(`float_scalar.rs`, `settle_float_items`, the float decode and replay are FR-053's). No stubs. No
tautological assertions: every test asserts concrete values from a real QSL-compiled, IR-admitted
package.

Examined and delivered for the comparison row: AC-1 (comparisons; arithmetic keeps no harness),
AC-2 (encoding and domain in the routed harness), AC-5 (no outcome-dependent assume, stopped outcomes
asserted by name), AC-6 (typed arithmetic refusals with an unchanged supported sibling in one routed
request), AC-7 (Verified needs one final cover; healthy harnesses Verified), AC-11 (decode, range and
schema refusals, exact bits, owner replay of a real Kani counterexample). IR-262: FR-049-AC-8's
same-generation refused sibling and FR-049-AC-9's float family are delivered as claimed.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | high | FR-053-AC-12 is claimed "Tagged and passing", but no test checks the run evidence. The Kani lane's `run` helper prints argv, unwind and solver to stderr and asserts only the outcome. No test asserts the operation and source association, ordered bindings, every option, target, limits, checks, cover or exit in the evidence. Add one assertion over `KaniExecutionEvidence` from a routed float run (argv equals the identity options plus the runner suffix, unwind, solver, target, cover and checks), or move AC-12 to the not-delivered list. | spec/kani/functional/FR-053-float-scalar-harnesses.md:459-461, tests/it/float_scalar_harness.rs:710-745 |
| FND-002 | medium | AC-3's operand half is CG's own work but is filed as upstream. FR-053 says a comparison or arithmetic operand whose type reaches no `float_rounding` yields source-associated `requires-bound`, and that until IR-69 lands "it is CG's own check". The generator draws any float parameter with no such check, and the Status lists the whole of AC-3 as not delivered with owner "upstream IR-69", adding that "comparisons need no bound". The operand half applies to comparisons too. Implement the CG-side check in this PR (it is a check on the operand's type node) or record it as CG-owned and open, not as IR-69's. | src/kani/generate/float_scalar.rs:317-337, spec/kani/functional/FR-053-float-scalar-harnesses.md:385, spec/kani/functional/FR-053-float-scalar-harnesses.md:463-464 |
| FND-003 | medium | Work for the comparison row that is tractable now is left open as "IR-260's follow-up" with no ticket, which breaks one-AC-family-one-ticket. The open items are AC-2's host enumeration of the class-boundary set and its seven correspondence mutants, AC-4's charges, denied charge and counters at zero, exact and short budgets for comparisons, AC-8's nine unrun mutants (b, c, e to m; each is about 20 s under Kani), AC-13's host differential of `compare_native_float` against `compare_ieee`, and AC-14's call-graph inspection of the comparison expected side. All are host tests or small Kani runs over the supported row, so they belong in this PR's fix round. The arithmetic halves of AC-9, AC-10, AC-13 and AC-14 are settled by the planner's refusal ruling. State them as dispositioned by refusal, not as owned by an unnamed follow-up. If any comparison item is truly deferred, the lead must file a named ticket for it first. | spec/kani/functional/FR-053-float-scalar-harnesses.md:463-468 |
| FND-004 | low | AC-1's literal-operand and non-parameter-operand controls are not exercised. Neither a float literal operand nor an expression operand is in the test corpus, and the code mislabels the expression case (code-review FND-001). | tests/it/float_scalar_harness.rs:19-35, src/kani/generate/float_scalar.rs:317-337 |
| FND-005 | low | AC-7's forced controls are not run for the float row: unsatisfied cover, failed unwind, timeout, a memory ceiling below the peak and a missing verdict. Only the healthy Verified path and the one-cover text check are tested. The shared runner covers these classifications for other rows, so state that reuse in the Status, or add one forced-cover control on a float harness. | tests/it/float_scalar_harness.rs:328-342, spec/kani/matrix/TC-067-float-scalar-harnesses.md:73-78 |

## Verdict

Runs on head 55457462, own target dir, under `flock /tmp/cargo-slot.lock`, one job at a time: whole `cargo test --locked --test it` gave 1064 passed, 0 failed, 97 ignored. This is the first whole run after the Makefile/kani_gate fix. Installed Kani, each test in its own MemoryMax=24G scope: `kani_numeric_equal_float64_verifies` Verified (6.5 s), `kani_total_order_float32_verifies` Verified (2.8 s), `kani_bit_identical_asked_as_numeric_equal_is_falsified` Falsified with a counterexample (26.8 s), `kani_nan_reflexive_numeric_equal_expectation_is_falsified` Falsified (27.4 s), and `kani_a_falsifying_counterexample_replays_through_the_owner_route` Falsified, decoded and replayed through `qsl_replay` (18.4 s). All five agree with the measurement doc's verdict table. The argv was the identity options with unwind 2 and cadical.

Not mergeable. FND-001 is a false delivery claim. FND-002 and FND-003 leave CG-owned, tractable comparison-row work under an unnamed follow-up, which one-AC-family-one-ticket forbids. Recommendation: in this PR's fix round, add an AC-12 evidence assertion, the CG-side AC-3 operand check, the AC-2 host enumeration, the remaining AC-8 mutants, and the comparison halves of AC-4, AC-13 and AC-14. State the arithmetic halves of AC-9, AC-10, AC-13 and AC-14 as settled by the refusal ruling. Only items the lead explicitly defers get a named ticket.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-006 | low | The AC-3 operand `requires-bound` is tested only through the direct generator (`corpus.outcome`). The new routed mapping (`into_disposition` to `ObligationDisposition::RequiresBound`), which supplies the "source-associated" record AC-3 requires, has no test, so the Status claim "source-associated requires-bound" rests on untested code. The Status does say honestly that the case is reachable only as hand-edited Contract IR (QSL pins a bare `Float64` to `exact`). Route the hand-edited package once through `generate_routed` and assert the `RequiresBound` record, or say in the Status that the routed mapping is untested. | src/kani/generate/float_scalar.rs:145-154, src/routed/generate.rs:933-935, tests/it/float_scalar_harness.rs |

## Dispositions

Round 1, checked at f11eef79 (runs as in the code-review SR's round 1).

| FND | outcome | reason |
|---|---|---|
| FND-001 | fixed | `the_identity_records_the_operation_bindings_options_and_dependency_cohort` (host) and `kani_the_run_evidence_records_every_option_and_no_tool_version` (installed Kani, rerun here: passes) assert argv order, unwind, solver, ceilings, symbolic bounds, one satisfied cover, raw checks, exit 0 and no `version` key. |
| FND-002 | fixed | The CG operand check is implemented (`operand_own_bound` plus `read_float_rounding`). It yields `RequiresBound` naming the type node, or `float.rounding_unreadable`, and is tested on hand-edited IR. The Status states that it is CG's own check and reachable only from hand-edited IR. The routed mapping is untested (new FND-006). |
| FND-003 | fixed | Comparison halves delivered: the AC-2 class-boundary enumeration, with two source mutations of the generated harness and five modelled mutants of the checker (stated as models in the Status); the AC-4/AC-13 differential of `compare_native_float` against `compare_ieee` at every limit kind, with denied point and consumed counters; all 13 AC-8 mutants (the ten new ones are real expected-side replacements falsified under Kani, rerun here); the AC-14 inspection, with three sharing mutants that fail it. The arithmetic halves are now stated as dispositioned by the refusal ruling. No unticketed follow-up remains. |
| FND-004 | fixed | `lit64` and `nested64` controls, direct and routed, with a supported sibling unchanged. |
| FND-005 | fixed | `kani_an_unsatisfied_cover_is_not_verified` (rerun here: `CoverUnsatisfied` 0/1). The other forced classifications are stated as the shared runner's (`tests/it/terminal_map.rs`), and a loop-free harness has no unwind to force. |

Round 2, checked at fe8ad242 (diff against f11eef79 only, read-only).

| FND | outcome | reason |
|---|---|---|
| FND-006 | still-open: fe8ad242 changes only FR-056, the `into_disposition` visibility (`pub(crate)` to `pub`), an outcome.rs doc comment and a kani_tractability test for FR-056-AC-4. No test calls `into_disposition` (the only caller is still `src/routed/generate.rs:934`), `tests/it/float_scalar_harness.rs` is unchanged, and neither the FR-053 nor the TC-067 Status mentions the routed-mapping limit. The worktree is clean, so the described test and Status edit were never committed. Commit a test asserting `FloatScalarRefusal::RequiresBound { .. }.into_disposition()` equals `ObligationDisposition::RequiresBound { unbounded_type }`, plus the Status sentence. | - |

Round 3, checked at 63190956 (diff against fe8ad242 only, read-only).

| FND | outcome | reason |
|---|---|---|
| FND-006 | fixed | `an_unpinned_float_operand_requires_a_bound_naming_its_type` now asserts that `FloatScalarRefusal::RequiresBound { unbounded_type }.into_disposition()` equals `ObligationDisposition::RequiresBound { unbounded_type }`, the seam `settle_float_items` uses. The FR-053 and TC-067 Status state the limit: the hand-edited package lowers to an empty set and cannot be routed, so the routed mapping is covered at the seam only. |
