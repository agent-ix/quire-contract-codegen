---
id: SR-17940486709564847047729
title: "IR-304 PR #804 code-review (rust-review lane)"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen PR #804 at b168f9b3 (base main 63cf3ca0); IR-304 slice (b); src/kani/generate/negotiate.rs, src/kani/generate/negotiate/guard.rs, src/kani/generate/scalar.rs, src/kani/generate/scalar/guard.rs, src/kani/generate/outcome.rs, src/kani/identity.rs, src/replay/scalar.rs, src/replay/scalar_builder.rs, src/routed/generate.rs, src/core/requirement.rs, tests/it/requirement_records.rs, tests/it/scalar_playback.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-022
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-032
    type: reviews
---

## Summary

Ticket: IR-304. The PR lowers routed scalar path guards (Boolean parameter, integer
lt/le/gt/ge/eq/ne against bounded parameters or `i64` literals) to `Statement::Assume`, narrows
operand intervals from parameter-vs-literal guards, draws `guard_N` inputs for parameters no
`Bound` operand reads, adds `PathConditionUnsatisfiable`, the four `PathGuardRefusal` reasons,
the whole-call `GuardContextMissing`, three `ScalarObligationIdentity` fields and FR-032-AC-19
guard-entry decoding; it removes `GuardLoweringNotImplemented`.

Run in an own worktree at b168f9b3 under `flock /tmp/cargo-slot.lock`. A temporary probe test
printed generated harnesses: the AC-33 `x > 5` harness narrows `left` to `[6, 9]` and assumes
`left_native > 5_i64`; the nested `p`, `q` harness draws `guard_0`, `guard_1` and assumes them in
path order; `scalar_replay_body()` is `Ok` for the Boolean-guarded harness and `Err(Ranges)` for
the narrowed one (FND-001).

Reviewer mutants (all reverted, worktree deleted afterwards):

- Batch S (whole `cargo test --locked --test it`, 940 passed incl. the probe, 0 failed, so every
  mutant in it survives): S1 `scalar_builder.rs` max values reverted to `identity.arguments.len()`;
  S2 guard assumptions emitted in reverse order (`.rev()` in `GuardPlan::render`); S3 `Ge`
  narrowing `low.max(c + 1)`; S4 `UnboundedGuardOperand` names the V2 node instead of the original
  parameter node.
- Batch K (the coder's stated mutants, applied together, focused run): M1 Boolean `holds=false`
  not negated killed `boolean_guards_lower_to_assumptions_in_path_order`; M2 literal-left not
  mirrored killed `contradictory_guards_settle_path_condition_unsatisfiable`; M3 `holds=false`
  narrowing not negated killed `integer_guards_narrow_the_operand_domains`; M4 integer guard
  domain unchecked killed `guard_entries_follow_the_operands_and_never_reach_the_operand_vector`;
  M5 `GuardContextMissing` check disabled killed
  `a_guard_whose_context_root_is_dropped_refuses_the_whole_call`. Each mutant has a distinct
  site and the killed test reads only its site, so attribution holds.
- Batch M6: M6 guard refusal moved after the operand `i64` refusal; H1 `UnsatisfiedPath.guard`
  reports the first guard instead of the emptying one. Focused `requirement_records::` run: M6 killed `guard_refusal_precedes_the_bound_and_reachability_refusals` (only failure); H1 survived (FND-007).

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | medium | A narrowed guarded harness (every parameter-vs-literal guard on an operand, e.g. the AC-33 `x > 5` case) binds no replay body: `replay_body` compares the narrowed `self.operands` with the admitted ranges and returns `ScalarAuthorityError::Ranges`, so a falsified run of it settles `ScalarBuildError::Authority(Ranges)` -> `CgDefect` -> `Failed` without QSL operator parity. `Ranges` was an internal-inconsistency cause; it is now the designed outcome for every narrowed item, and neither FR-022 nor FR-032 says so (only the PR body does) | src/kani/generate/scalar.rs:458-469, src/kani/generate/negotiate.rs:411-414 |
| FND-002 | medium | Per-item precedence: an operand `Bound` that is not in `checked_bounds` (a claim map this generator did not produce) yields `NoRenderer` = `operation_not_rendered` (step 2), but it is computed inside `operation.operands(..)` after the guard refusal (step 3) returns, so a guarded item with such a claim map and an unlowerable guard reports `path_guard_not_lowered` | src/kani/generate/scalar.rs:302-333 |
| FND-003 | medium | The `scalar_builder.rs` change (`max_values` = arguments + guard inputs) has no test: mutant S1 (revert to `arguments.len()`, which makes every guarded playback refuse at parse) survives the whole `--test it`; no lib or it test builds a guarded harness through `OriginalScalarContext::plan` | src/replay/scalar_builder.rs:133-139 |
| FND-004 | medium | `boolean_guards_lower_to_assumptions_in_path_order` cannot fail on order: `text.find("assume")` first hits the operand input `kani::assume(left >= 0_i64 ...)`, and `second` is searched after `first`, so `first < second` is always true; the `guard_0 < guard_1` check is string order by construction. Mutant S2 (reversed assumptions) survives. The `"kani::assume(p);"` alternative never matches the generated text, and "exactly once, before the oracle call" and "exchanging the path conditions exchanges the assumptions" (TC-033 step 21) are not asserted | tests/it/requirement_records.rs:1190-1256 |
| FND-005 | medium | Interval narrowing is tested only for `gt` (both `holds`) and literal-left `lt` emptying; `lt`/`le`/`ge`/`eq`/`ne` narrowing (including `ne` endpoint removal) has no test. Mutant S3 (`ge` off by one) survives | src/kani/generate/scalar/guard.rs:250-267 |
| FND-006 | medium | AC-32 refusals are asserted by code/variant only: `UnboundedGuardOperand` is checked with `is_object()` (mutant S4, wrong parameter node, survives), `GuardLiteralNotI64`'s value is not checked, `GuardRootNotLowered` is exercised only through a direct caller whose guard is a literal node, not the routed model-member guard the AC names, and with slice (a)'s test removed no test shows a refused guarded item beside a supported sibling | tests/it/requirement_records.rs:1284-1320, 1482-1494 |
| FND-007 | medium | AC-34 tests assert only `code` (and `declared: boolean`): the reported guard, the parameter and `Integer { minimum: 0, maximum: 9 }` are unchecked (mutant H1, reporting the first guard instead of the inner emptying `p`, survives); the Boolean case routes `if p then x + 1 else x + 1` with a hand-built `[(p,true),(p,false)]` instead of the AC's `if p then (if p then 0 else x + 1) else 0` | tests/it/requirement_records.rs:1409-1445, 1495-1520 |
| FND-008 | low | `integer_guards_narrow_the_operand_domains` carries `#[trace("FR-032-AC-19", "TC-047")]` but decodes no playback (wrong binding); it also checks `contains("assume")`, which every harness satisfies through its operand inputs, instead of the `x > 5` / `x < y` assumption, and has dead code `let _ = (signature, body);` | tests/it/requirement_records.rs:1359-1406 |
| FND-009 | low | `validate_schema` does not check `guard_inputs` (identifier colliding with an argument or another input, `minimum > maximum`) although the fields are public and the decoder trusts them | src/replay/scalar.rs:134-163 |

## Verdict

Not mergeable at b168f9b3 until FND-001 to FND-007 are fixed in this PR (FND-001 may instead be
fixed in the spec by stating the narrowed-harness replay outcome and owning ticket, if that is the
decided behaviour). Checked with no defect: guard-form classification and the per-guard reason
order (`GuardRootNotLowered`, `UnsupportedGuardForm`, `UnboundedGuardOperand`,
`GuardLiteralNotI64`) match FR-022; narrowing removes only values that falsify an assumed guard,
in `i128` with no overflow at `i64::MIN`/`MAX`; mirroring then negation is correct
(`!(20 < x)` gives `x <= 20`); parameter-vs-parameter guards narrow nothing and still assume;
the only harness-emitting arm (`LoweredScalar`) always passes through `resolve_guards`, so no
guarded item yields an unguarded harness; FR-032-AC-19 decoding reads guard entries after the
operands and never into the operand vector (a swapped order fails the valid case); the
`GuardContextMissing` check runs after `PathConditionMismatch` and names the guard; the boxed
fields keep the JSON shape; `requirement_occurrence`/`path_condition`/`guard_inputs` are omitted
when empty so the direct golden is byte-identical. All six stated coder mutants (M1 to M6) were reproduced and each killed its named test.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-010 | medium | A QSL-authored `x = 5` or `x != 5` guard settles `path_guard_not_lowered` / `unsupported_guard_form` (reviewer probe at f00e67a6, routed through `generate_routed`; QSL widens the literal through a conversion, so the side is not a literal), while FR-022-AC-32 lists `eq`/`ne` against an `i64` literal as a lowerable form that "generates a harness" and the matrix marks AC-32/AC-33 covered. Refusing is sound (no guard is dropped), but spec and code disagree and the only `eq`/`ne` evidence is the hand-built plan unit test `equality_guards_narrow_by_the_value_they_name`. Either lower a literal reached through that conversion, or state in FR-022 (forms list, AC-32) that QSL-authored `eq`/`ne` literal guards refuse and record the gap | src/kani/generate/negotiate/guard.rs:186-226, spec/routed/functional/FR-022-routed-generation.md:702 |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-011 | low | `a_guard_root_the_package_does_not_lower_is_not_lowered` runs a second variant with `work_limit = 60`, a number tuned to the current IR work cost of this fixture's closures; any change in IR's work accounting breaks it (noisily, not silently). The `Function`-unsupported variant already drives the same routed branch deterministically, so the tuned pin adds churn and no coverage; drop the `costly` variant | tests/it/requirement_records.rs:1687-1730 |
| FND-012 | low | The two refusal arms of the new `numeric.narrow` read-through are untested: a non-`i64` literal under the conversion (kept as `GuardLiteralNotI64`) and a conversion over a non-literal (`UnsupportedGuardForm`). Mutant N2 (non-`i64` literal mapped to `Form`, conversion over a parameter read as the parameter) survives every `requirement_records` test; reviewer probes `x = 10^23` and `x = y` over `Int[0, 9]`/`Int[0, 20]` did not build a scenario, so these shapes may be unreachable from QSL today. The read-through also does not check that the literal lies in the conversion's target range | src/kani/generate/negotiate/guard.rs:206-216 |

## Dispositions

Round 1, reviewed head f00e67a6 (one commit on b168f9b3). Clean gate at this head, own worktree under the cargo slot lock: whole `cargo test --locked --test it` 944 passed, 0 failed (36 ignored); `cargo test --locked --lib` 548 passed, 0 failed (7 ignored). Reviewer mutants at this head (all
reverted): S1, S2, S3, S4, H1, R1 (`replay_body` strict for guarded harnesses), R2 (the new
operand-bound pre-check disabled), V1 (guard-input schema loop skipped) applied together; each
killed its own test (S1 `a_guard_input_playback_plans_through_the_builder`, S2
`boolean_guards_lower_to_assumptions_in_path_order`, S3 `every_comparison_narrows_by_the_values_that_falsify_it`,
S4 `unlowerable_guards_settle_per_item_with_their_reason`, H1
`a_contradicted_boolean_guard_names_the_inner_occurrence_and_its_domain` and
`contradictory_guards_settle_path_condition_unsatisfiable`, R1
`a_narrowed_guarded_harness_plans_witnesses_inside_its_narrowed_range`, R2 lib
`a_foreign_operand_bound_precedes_the_guard_refusal`, V1 `a_malformed_guard_input_schema_is_refused`);
each test reads only its mutant's site. R3 (`root_lowered` always true) applied alone survives
every `requirement_records` test (FND-006).

| FND | outcome | reason |
|---|---|---|
| FND-001 | fixed | `replay_body` accepts, for a guarded harness only, operand ranges inside the admitted body ranges; unguarded stays exact. Sound: narrowing only intersects with the declared bound, the retained body keeps the admitted ranges QSL parity runs over, and the decoder still checks witnesses against the narrowed arguments, so a planned witness is an input of the admitted claim. FR-022 now states it; R1 is killed |
| FND-002 | fixed | `lower_scalar_claim` checks every `Bound` operand for `checked_bounds` membership and a derived range before the guard refusal; lib test `a_foreign_operand_bound_precedes_the_guard_refusal` covers both causes under a lowered and an unlowered guard; R2 killed |
| FND-003 | fixed | `a_guard_input_playback_plans_through_the_builder` plans a real guarded harness's playback through `OriginalScalarContext::plan`; S1 killed |
| FND-004 | fixed | `guard_assumptions` filters out input-domain assumes; the test asserts the exact assumption list, once each and before the oracle call, the nested order, the exchange case and the unguarded item; S2 killed |
| FND-005 | fixed | `every_comparison_narrows_by_the_values_that_falsify_it` covers lt/le/gt/ge with both `holds` through routed generation; eq/ne narrowing is covered on the plan by a unit test (see FND-010 for why routed eq/ne never lowers); S3 killed |
| FND-006 | still-open: the named parameter, literal value and supported sibling are now asserted (S4 killed), but the AC-32 `GuardRootNotLowered` case for an admitted context root whose accounting is not `lowered` has no test. Neither cited test reaches it: `a_guard_whose_context_root_is_dropped_refuses_the_whole_call` asserts `GuardContextMissing`, and `a_guard_on_a_node_that_is_no_lowered_root_is_not_lowered` is a direct caller whose guard node is not admitted, so mutant R3 (`root_lowered` always true) survives. A routed `g(x) > 5` guard settling `unsupported_guard_form` does not exercise it. Needed: a routed context root with a non-`lowered` record (any profile-unsupported tag, an `invalid_body` or a limit `failed`), or mark that AC-32 clause Partial in the matrix with its owner | - |
| FND-007 | fixed | AC-34 tests assert the reported guard (second guard in `dead-second`, inner `p` in the AC's nested shape), the parameter node and the declared domain; H1 killed |
| FND-008 | fixed | `integer_guards_narrow_the_operand_domains` is tagged only FR-022-AC-33, asserts `kani::assume(left_native > 5_i64);`, its negation, `left_native < guard_0` and the `[0, 200]` assertion; the dead binding is gone |
| FND-009 | fixed | `validate_schema` refuses an inverted guard-input range and a repeated identifier (shared `seen` set with the arguments); `a_malformed_guard_input_schema_is_refused`; V1 killed |

Round 2, reviewed head 82a3ae19 (one commit on f00e67a6). Reviewer mutants (reverted): R3
(`root_lowered` always true) with N1 (read-through disabled), focused `requirement_records::`:
R3 killed `a_guard_inside_the_closure_but_not_a_requested_root_is_not_lowered`, N1 killed
`every_comparison_narrows_by_the_values_that_falsify_it`; the routed
`a_guard_root_the_package_does_not_lower_is_not_lowered` passes under R3 because a routed
non-`lowered` root has no admitted node and refuses before `root_lowered`, so it drives the
AC-32 routed branch independently. N2 alone survives (FND-012). Clean gate at this head, own worktree under the cargo slot lock: whole `cargo test --locked --test it` 946 passed, 0 failed (36 ignored); `cargo test --locked --lib` 548 passed, 0 failed (7 ignored).

| FND | outcome | reason |
|---|---|---|
| FND-006 | fixed | routed `a_guard_root_the_package_does_not_lower_is_not_lowered` (guard calling `g` with `Function` unsupported: `guard_root_not_lowered`, guard named, sibling supported) covers the AC-32 accounting case, and direct `a_guard_inside_the_closure_but_not_a_requested_root_is_not_lowered` kills R3; FR-022 and TC-033 now name the unsupported-function example instead of the model member |
| FND-010 | fixed | `side` reads an `i64` literal through `quire.op.numeric.narrow` (a conversion over anything else stays `UnsupportedGuardForm`); routed table covers `=` and `!=` with both `holds` and the `ne` endpoints; FR-022 forms list states the read-through; N1 killed |

Round 3, reviewed head caecf62f (one commit on 82a3ae19). Reviewer mutant N2 (in
`through_conversion`: a non-`i64` literal mapped to `Form`, anything else passed through, so a
conversion over a parameter reads as the parameter), reverted: killed lib
`a_conversion_passes_only_a_literal_through` and it
`unlowerable_guards_settle_per_item_with_their_reason` (the routed `x = 10^23` over
`x: Int[0, 10^24]` case goes through the conversion). The QSL range claim holds: the round-2
reviewer probe `x = 10^23` over `x: Int[0, 9]` was refused by QSL compilation, matching the
`Unproved(Range)` refusal the FR-032 matrix also records, and CG only reads QSL-checked packages,
so a narrow literal outside its target range cannot reach `side`. Whole `--test it` not re-run
by the reviewer for this commit (tests plus a pure refactor of the read-through; coder reports
946 passed; round 2 reviewer gate was green).

| FND | outcome | reason |
|---|---|---|
| FND-011 | fixed | the `work_limit = 60` variant is removed; the test keeps only the deterministic `Function`-unsupported profile |
| FND-012 | fixed | `through_conversion` isolates the read-through; the unit test covers literal, non-`i64` literal, parameter, unbounded parameter and form inputs, and a routed `=` case asserts `guard_literal_not_i64`; N2 killed. No range check is needed because QSL refuses an out-of-range narrow literal at check time and CG reads only checked packages; FR-022 states this |
