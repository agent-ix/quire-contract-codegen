---
id: SR-17940486709564847047730
title: "IR-304 PR #804 gap-analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen PR #804 at b168f9b3; IR-304 slice (b); FR-022-AC-27, FR-022-AC-31 to AC-38, FR-032-AC-19, TC-033 steps 21 to 30, TC-047 step 9; spec/routed/matrix/tests.md, spec/replay/matrix/tests.md, tests/it/requirement_records.rs, tests/it/scalar_playback.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-022
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-032
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-033
    type: reviews
---

## Summary

Ticket: IR-304. Plan completion: not assessed. Tagged tests: FR-022-AC-27
`boolean_guards_lower_to_assumptions_in_path_order`; AC-31 `direct_increment_record_equals_the_committed_golden`
and `routed_identity_carries_the_occurrence_and_the_direct_one_does_not`; AC-32
`unlowerable_guards_settle_per_item_with_their_reason`, `a_guard_whose_context_root_is_dropped_refuses_the_whole_call`,
`a_guard_on_a_node_that_is_no_lowered_root_is_not_lowered`, `guard_refusal_precedes_the_bound_and_reachability_refusals`;
AC-33 `integer_guards_narrow_the_operand_domains`; AC-34 `contradictory_guards_settle_path_condition_unsatisfiable`,
`a_contradicted_boolean_guard_names_the_parameter_and_its_domain`; AC-37 `an_unlowered_second_guard_precedes_unsatisfiability`;
FR-032-AC-19 `guard_entries_follow_the_operands_and_never_reach_the_operand_vector`. All new production code is
owned by FR-022 / FR-032-AC-19; no unowned code, no stub. The new tests run the real QSL checker and
`generate_routed`; records are the typed projection TC-033 allows.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | medium | The status text claims FR-022-AC-27, AC-32, AC-33 and AC-34 fully covered (FR-022 "Scoped evidence status", routed `tests.md` ✅ rows, TC-033 status) while parts of each are untested: AC-27 assumption order, single assumption before the oracle call and the exchange case; AC-32 the model-member `GuardRootNotLowered` case, the named `n` and literal value and the unaffected sibling; AC-33 the `x > 5`/`x < y` assumption text and `[0, 200]` assertion; AC-34 the named guard, parameter and declared domain and the nested-`p` shape (code-review SR FND-004 to FND-008). These rows should read Partial with the gaps listed, or the tests be completed | spec/routed/matrix/tests.md:25-27, spec/routed/functional/FR-022-routed-generation.md:829-833 |
| FND-002 | medium | The FR-022-AC-35 PLANNED row, TC-033 step 28 and the routed `tests.md` FR-022-AC-35 row name no owning ticket (lane rule: undelivered rows are PLANNED with the owning ticket; the IR-304 planner comment says IR-304 keeps AC-35) | spec/routed/functional/FR-022-routed-generation.md:696, spec/routed/matrix/TC-033-routed-generation.md:317, spec/routed/matrix/tests.md:27 |
| FND-003 | low | FR-032-AC-19 is marked ✅ Covered "in `decode_scalar_falsification`", but only the decoder is tested on a hand-built identity; the replay-builder path a real guarded harness takes (`OriginalScalarContext::plan`, whose playback cap this PR changed) has no test (code-review SR FND-003) | spec/replay/matrix/tests.md:36, tests/it/scalar_playback.rs:774-848 |

## Verdict

Coverage is real for AC-31, AC-36, AC-37 and FR-032-AC-19's decoder, and for the main paths of
AC-27/32/33/34, but the ✅ claims overstate it. FR-022-AC-35 staying Planned is honest: the
qualification note matches the logged claims (Kani 0.68.0, `--unwind 3 --solver cadical`,
1800.7 s, exit 124, cgroup peak 25.29 GB under a 24 GiB scope, feasible run not started) and
the PR, the FR-022 status paragraph, TC-033 and the matrix row state the same numbers; only the
owner is missing (FND-002).

## Dispositions

Round 1, reviewed head f00e67a6.

| FND | outcome | reason |
|---|---|---|
| FND-001 | still-open: the AC-27, AC-33 and AC-34 gaps are closed (code-review SR FND-004, FND-007, FND-008), but the routed `tests.md` row still marks FR-022-AC-32 and AC-33 ✅ while the AC-32 `GuardRootNotLowered` accounting case is untested (code-review FND-006) and QSL-authored `eq`/`ne` literal guards do not lower as AC-32 states (code-review FND-010). Either close those or mark the row Partial naming them | - |
| FND-002 | fixed | FR-022-AC-35 row, TC-033 step 28, the routed `tests.md` row and the qualification note name IR-304 as owner (native-integer routed scalar harness next) |
| FND-003 | fixed | the builder path is now tested on a real guarded harness (`a_guard_input_playback_plans_through_the_builder`, `a_narrowed_guarded_harness_plans_witnesses_inside_its_narrowed_range`) and the FR-032-AC-19 matrix row says so |

Round 2, reviewed head 82a3ae19.

| FND | outcome | reason |
|---|---|---|
| FND-001 | fixed | the routed AC-27/AC-31 to AC-34 row now lists what the tests assert, and each listed item is tested: routed non-`lowered` guard root, refused item beside a supported sibling, narrowing for all six comparisons with QSL-authored `=`/`!=` literals |
