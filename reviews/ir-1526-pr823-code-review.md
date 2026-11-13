---
id: SR-17940486709564847047941
title: "IR-1526 PR #823 route as a tractability decider feature code review"
type: SpecReview
analysis: code-review
date: 2026-11-13
scope: "agent-ix/quire-contract-codegen; PR #823 @ 73e4d64a; IR-1526; src/kani/tractability.rs, src/kani/generate/clause/v2.rs, src/kani/generate/negotiate.rs, src/kani/generate/outcome.rs, src/lib.rs, tests/fixtures/tractability/corpus.json, tests/fixtures/tractability/table.json, tests/it/kani_tractability.rs, tests/it/kani_obligations_v2_clause_controls.rs, spec/kani/functional/FR-056-kani-tractability-refusal.md, spec/kani/matrix/TC-070-kani-tractability-refusal.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-056
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-070
    type: references
---

## Summary

Ticket: IR-1526. Code review of PR #823 with the rust-review lane folded in, scoped to
`git diff origin/main...HEAD` (11 files, +222/-49). The PR makes the integer route a decider
feature: `IntegerRoute` moves from `negotiate.rs` to `tractability.rs` and gains serde
(`general` / `native`), `ClauseFeatures` and `TractabilityRow` carry a required `route`,
`TractabilityRow::is_current` also requires `row.route == features.route`, the negotiation
threads the route into `refuse_if_not_tractable`, and `V2ClauseNotTractable` reports it. The
corpus and table rows are relabelled and the FR-056 / TC-070 Status lines are rewritten.

Focused run: `cargo test --locked --test it -- kani_tractability layout
kani_obligations_v2_clause_controls` through `locked-build.sh` with a throwaway target dir
(deleted afterwards): 39 passed, 0 failed, 13 ignored (kani lane). No Kani run, no full gate.

Mutants, each applied in a throwaway detached worktree (removed afterwards) and run against
`kani_tractability`:

- ignore-route (`self.route == features.route` removed from `is_current`): 3 tests fail
  (`tc_070_lookup_matches_rows_only_for_the_clause_route`,
  `tc_070_the_ac13_multiply_fixtures_split_on_the_row_bound`,
  `tc_070_the_negotiated_route_decides_the_refusal`).
- fixed-route (`IntegerRoute::from_native(native)` replaced by `IntegerRoute::General` in
  `classify_v2_clause`): 2 tests fail (the AC-13 split test and the negotiated-route test).
- wrong reported route (`route: IntegerRoute::General` in the `V2ClauseNotTractable`
  construction): 1 test fails (`tc_070_the_negotiated_route_decides_the_refusal`).

## Verdict

**MERGEABLE.** Two low findings; neither is a wrong result.

What holds up:

- `route` is required everywhere: no `#[serde(default)]` on `ClauseFeatures::route` or
  `TractabilityRow::route`, both structs `deny_unknown_fields`, the enum has no catch-all or
  alias. A missing and an unknown (`"fast"`) route both fail the table load as `MalformedRow`
  at row index 1 (tests/it/kani_tractability.rs:318-344).
- Lookup never matches across routes: the route check sits in `is_current`, which gates every
  row before width matching, so a native row can neither refuse a general clause nor displace a
  qualifying general row (the precedence case is tested and killed the ignore-route mutant).
- AC-14 tests can fail: all three mutants above are killed.
- The flipped host test (`tc_070_the_ac13_multiply_fixtures_split_on_the_row_bound`) is honest:
  on main the native negotiation of the wide fixture reached the decider and was refused, so the
  new negated assertion is not vacuous; the ignore-route and fixed-route mutants both fail it at
  line 959. Its negative form matches AC-14's allowance for another disposition (`RenderFailed`).
- `IntegerRoute` keeps its public path: it was and is re-exported only at the crate root
  (`quire_contract_codegen::IntegerRoute`); `kani` is a private module, so the move is invisible
  to consumers. The L-2 layout tests pass.
- `V2ClauseNotTractable` is constructed in one place (src/kani/generate/clause/v2.rs:422) and
  every other match uses `{ .. }` or names fields explicitly in tests that were updated; the
  enum derives no `Serialize`, so no wire shape changes.
- Data relabelling is right: `docs/qualification/ir1022-kani-clause-measurements.md` and
  `ir1509-kani-clause-controls.md` both state the harness came from
  `negotiate_kani_native_obligations` (native); the four `ir1526-*` rows' measurement sources
  say "General route", and the two `table.json` rows derive from them (general).
- Status lines keep AC-10 PARTIAL and list the missing native-route families.
- No compatibility layer, no default route, no files owned by other lanes touched
  (`src/kani/generate/spec.rs`, `collection.rs`, `src/oracle/scalar/render.rs` untouched).

## Findings

| ID      | Severity | Summary | Refs |
| ------- | -------- | ------- | ---- |
| FND-001 | low      | The FR-056 Status says `route` is "a required key of every table row and corpus row (no default; a missing or unknown route fails the load with the row index)". That holds for the table (`TractabilityTable::parse` reports `MalformedRow { row }`), but the corpus loader `load_corpus` deserializes the whole array with `serde_json::from_slice(..).expect("a corpus file")`, so a corpus row without `route` panics with a serde line/column message and no row index; no test loads a corpus row without `route`. The rejection itself is real (no default, `deny_unknown_fields`); only the "with the row index" claim is overstated for the corpus. Either narrow the Status sentence to the table or load the corpus row by row like the table. | spec/kani/functional/FR-056-kani-tractability-refusal.md:141-145, tests/it/kani_tractability.rs:725-727 |
| FND-002 | low      | The route travels enum -> bool -> enum through one call chain: `negotiate_kani_obligations_with_options` turns `IntegerRoute` into `route == IntegerRoute::Native` for `negotiate_integer_policy(native: bool)`, and `classify_v2_clause` rebuilds it with the new bridge `IntegerRoute::from_native(native)`. rust-style api-design: use an enum rather than a `bool` parameter when the call does not reveal the choice (`negotiate_integer_policy(request, true, ..)` at negotiate.rs:355). Threading `IntegerRoute` through `negotiate_integer_policy` and `classify_v2_clause` removes the bridge and the second representation. | src/kani/generate/negotiate.rs:330-360, src/kani/generate/negotiate.rs:996, src/kani/tractability.rs:38-46 |
