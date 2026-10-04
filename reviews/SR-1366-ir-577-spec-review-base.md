---
id: "SR-1366"
title: "CG PR 255 spec review: NFR-005 index, slice and arithmetic sites (IR-577)"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-codegen@3b08c1752f8209a46ea2f3605a769a1916f5a025; spec/core/non-functional/NFR-005-no-generation-panics.md, spec/core/matrix/TC-042-no-generation-panics.md, spec/core/matrix/tests.md, spec/tests.md, spec/routed/functional/FR-022-routed-generation.md, spec/core/functional/interface-001-codegen-api.md (diff origin/main...HEAD, base 3d13550); src/ at 3d13550 and tests/common/panic_scan.rs read as context"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/NFR-005
    type: references
---

# SR-1366: CG PR 255 spec review (base)

## Summary

Ticket: IR-577. The PR is spec-only. It extends NFR-005 with a statement bullet that forbids
indexing, slicing or subtracting on a value whose range another function or module
establishes. It covers three sites (`driver[*first_index]` in `generate_kani`, `map[1]` and
`map[2..]` in `observe_clause`, and the two `source_lines[.. as usize - 1]` reads in
`generate_boolean_oracle_inner`) and adds NFR-005-AC-6 to AC-8, TC-042 steps 6 to 8, one planned
`RoutedGenerationError` variant (FR-022, interface-001) and the matrix rows.

I re-measured the author's claims against `src/` at origin/main 3d13550.

- The three covered sites cannot be reached from input today. `reject_duplicates_and_mixtures`
  sets `first_index` from `states[..index].position(..)`, and `states` has one entry per item of
  the group. `check_maps` refuses any map whose length is not `count + 2`. The boolean renderer's
  fixed 15-line header makes `offset` at least 14. These sites are the right class.
- No index, slice or arithmetic panic on a generation path can be reached from untrusted input.
  `DependencyIdentity::new` and its `Deserialize` in quire-contract-model (cbcd790) refuse an
  empty path, so every `path()[0]` is safe by the model type.
- The planned typed refusals can be built. `KaniDuplicatePositionOutOfRange` is a new variant
  with two `usize` fields. The `MapMismatch` text `typed implication census differs from map`
  already exists in `check_maps`. The `InvalidGeneratedSyntax` code is reused (see FND-005).
- Numbering is new. `git log -S` on origin/main finds no earlier NFR-005-AC-6, AC-7 or AC-8.
  TC-042 keeps its id and adds steps 6 to 8. interface-001, FR-022, TC-042, the core matrix and
  `spec/tests.md` agree with each other.
- Statuses are honest. Each new AC, behavior row and TC step carries `PLANNED (IR-577)`, and the
  matrix rows show 🚧.
- `make spec` (quire validate) passes. `quire coverage --strict` reports 66 unbacked rows and 0
  contradicted, the same on main 3d13550 and on the PR head.

The Scope measurement is not complete, and AC-8 cannot pass as written. Details follow.

## Verdict

Not mergeable. FND-001 and FND-002 are high.

- FND-001: the new statement bullet forbids, in its own words, at least seven more sites. The
  PR's Scope says those sites are "provably unreachable inside their own function", and TC-042's
  Expected Results repeats the claim.
- FND-002: AC-8 allows no exception, but `generate_boolean_oracle_inner` has a `regions[0]` index
  that the measurement never lists.

FND-003 and FND-004 are AC-8 testability defects. FND-005 and FND-006 are low.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | NFR-005 Scope says only three sites rely on a range set by another function and that the rest are guarded "inside their own function". The new statement bullet forbids at least seven more under its own wording: `runner_source` `dependency.path()[0]` (no length check anywhere in that function; the Scope's claim that the read follows a `path().len()` check is false here), `oracle_arguments` `path()[0]` (the check is in `harness_binding`), `side_expression` `names.identifiers[0]` and `[1]` (the check is `validate_identifiers` in its caller), `dependency_key` `path()[0]` (the guard is in its callers), `typed_dependency_parameters` `analysis.references[key]` (the keys come from `analyze_supported_expression`), `validate_signature` `parameter_kinds[0] != parameter_kinds[1]` (the guard is `declaration.parameters.len()`, a different slice whose length the caller matches) and `item_disposition` `function_index[&declaration.node_id]` (the map is built in the caller). None can be reached today. Either cover them, or narrow the bullet to the class that the three sites actually share. TC-042 Expected Results ("guarded inside their own function") is wrong for the same reason. | spec/core/non-functional/NFR-005-no-generation-panics.md:25-31, :53-74; spec/core/matrix/TC-042-no-generation-panics.md:63-64; src/strategy/bound/generation.rs:591; src/strategy/harness.rs:841; src/strategy/bound/population.rs:737, :757; src/oracle/boolean_v1.rs:354, :813; src/oracle/function/mod.rs:861, :1360 |
| FND-002 | high | NFR-005-AC-8 asserts that the body of `generate_boolean_oracle_inner` holds no index expression and "allows no exception". That body holds `regions[0].expected_consequents = ..`, which the Scope list and the behavior table never name. As written, the AC fails once implemented unless the code also changes a site the spec says nothing about. The measurement's claim to cover "every index ... expression" missed it. | spec/core/non-functional/NFR-005-no-generation-panics.md:154; src/oracle/boolean_v1.rs:291 |
| FND-003 | medium | NFR-005-AC-8 defines an index expression as "an identifier, `)` or `]` followed by `[`", and exempts only an array type, a slice pattern and an attribute. The body of `generate_boolean_oracle_inner` holds `for line in [ ... ]`, an array expression after the keyword `in`. The existing byte-level scanner (`identifiers()` in tests/common/panic_scan.rs) treats keywords as identifiers, so this flags a false positive. The exemptions need to cover array and `vec!` expressions after a keyword. | spec/core/non-functional/NFR-005-no-generation-panics.md:154; src/oracle/boolean_v1.rs:242; tests/common/panic_scan.rs:50-66 |
| FND-004 | medium | NFR-005-AC-6 tests "the step of `generate_kani` that rewrites" a `DuplicateItem` position with hand-built records. That implies a helper function, like `pair_records` for AC-5. NFR-005-AC-8 scans only the body of `generate_kani`, so an index in that helper escapes the regression guard. AC-8 (and TC-042 step 8) should name the helper, or every function the three sites move into. | spec/core/non-functional/NFR-005-no-generation-panics.md:152, :154; spec/core/matrix/TC-042-no-generation-panics.md:49-53 |
| FND-005 | low | The `generate_boolean_oracle_inner` refusal reuses `GenerationErrorCode::InvalidGeneratedSyntax`, whose doc says "Generated tokens did not parse as a Rust source file". The missing line is detected after `syn::parse_file` succeeded, so it is a source-map/renderer inconsistency, not a syntax failure. The terminal state (Inconclusive) is right. The spec should say the variant doc is widened, or should fix a distinguishing diagnostic path (for example `generated.source_map`). | spec/core/non-functional/NFR-005-no-generation-panics.md:133; src/core/diagnostic.rs:72-73 |
| FND-006 | low | NFR-005-AC-8 forbids "an integer subtraction", but a lexical scan cannot see types. The AC should state the token rule (a binary `-` or `-=`, not `->` and not a unary minus) so two implementers build the same check. | spec/core/non-functional/NFR-005-no-generation-panics.md:154 |

## New findings (disposition pass 1)

Reviewed at e26183b146a5d77790418d3b6d32cd6504df208a. I re-measured `src/` at 3d13550 myself and did not take the fix commit's word for anything.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-007 | medium | The known-limit rationale is false for most of its entries. It says each rests "on an invariant a validator or type states at the boundary where the data enters, not on a sibling pass of a generator that could drift". Only `path()[0]` rests on a type (`DependencyIdentity`). `analysis.references[key]` relies on the sibling pass `analyze_supported_expression`. `validate_signature` relies on its caller building `parameter_kinds` one per parameter. `item_disposition` relies on its caller building `function_index`. `side_expression` relies on `validate_identifiers` in `render_population`. These are generator-internal invariants of the same class as the covered sites. The known limit itself is honest ("not a claim that they cannot panic"), but the reason given for the line between covered and limit is not. Drop or correct the rationale, or say plainly that these are the same class and deferred. | spec/core/non-functional/NFR-005-no-generation-panics.md:71-84; src/oracle/boolean_v1.rs:354; src/oracle/function/mod.rs:861, :947-952, :1360; src/strategy/bound/population.rs:531, :737 |
| FND-008 | low | The "guarded only by a caller, a validator or a type" group lists sites that are guarded inside their own function. `harness_binding` reads `path()[0]` (:764) after its own `path().len() != 1` check (:758). `typed_dependency_parameters` reads `path()[0]` (:391, :400) after its own check (:371). The location-map `function_index[&node_id]` (:1077) is in `generate_exact_function_oracles`, which builds the map at :1049. | spec/core/non-functional/NFR-005-no-generation-panics.md:71-80; src/strategy/harness.rs:758-764; src/oracle/boolean_v1.rs:371-400; src/oracle/function/mod.rs:1049, :1077 |
| FND-009 | medium | The `InvalidGeneratedSyntax` widening was written into interface-001's `kani_slice.refusal`. That entry is `KaniErrorCode::InvalidGeneratedSyntax` (src/kani/generate/census_validation.rs:19), which is emitted only by `syn::parse_file` (src/kani/generate/v1_bundle.rs:249-251). So interface-001 now says the Kani code also means "did not match its own source map", which is false. The variant being widened is `GenerationErrorCode::InvalidGeneratedSyntax` (src/core/diagnostic.rs:72), and its interface home (`oracle_slice` / `generate_boolean_oracle`) is unchanged. | spec/core/functional/interface-001-codegen-api.md:275, :61-64, :224-232; spec/core/non-functional/NFR-005-no-generation-panics.md:147 |
| FND-010 | low | Stale counts. NFR-005 Statement says "the three behavior-table rows marked PLANNED (IR-577)", but the table now has four (the new `regions[0]` row). TC-042 Description still says "the three index and arithmetic sites of IR-577". | spec/core/non-functional/NFR-005-no-generation-panics.md:25; spec/core/matrix/TC-042-no-generation-panics.md:13-14 |
| FND-011 | low | The AC-8 token rule has false negatives. An index after `?` (`x?[0]`), after a tuple-field literal (`t.0[1]`) or after `}` is not flagged, and neither is a subtraction after `?` (`x? - 1`). Applied by hand to the current bodies, the rule leaves no false positive (`for line in [`, `vec![`, `let [..] =` and `->` all pass). It does miss these spellings of a reintroduced site. | spec/core/non-functional/NFR-005-no-generation-panics.md:169 |


## New findings (disposition pass 2)

Reviewed at fc1527b038d5702e126c11ffe6e14c29f14d62ff. I re-measured `src/` at origin/main 3d13550 myself.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-012 | low | The Scope says the deferred same-class sites are left out "because IR-577 covers the sites the ticket named and the sites in those functions". That does not explain the coverage. The ticket names only `generate_kani` and `observe_clause`, while `generate_boolean_oracle_inner`, a function it does not name, is covered. The sites are also called "deferred" with no follow-up ticket to track them. Either name a follow-up ticket, or say plainly that the cut was a choice made for this ticket. | spec/core/non-functional/NFR-005-no-generation-panics.md:86-89 |

## New findings (disposition pass 3)

Reviewed at 62c8066bd35332b82c6958faa38ae82accf42c23.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-013 | low | TC-042 Expected Results still says the outside-invariant sites are ones "which NFR-005 names as a known limit". NFR-005 no longer uses the term: Scope now calls them "a scoping choice for IR-577". Use the same wording in both. | spec/core/matrix/TC-042-no-generation-panics.md:65-66 |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | e26183b: the Statement now covers only the PLANNED rows, and the other sites are split into two Scope groups and stated as a known limit. TC-042 Expected Results matches. Rationale and grouping errors are recorded as FND-007 and FND-008. |
| FND-002 | fixed | e26183b: a `regions[0]` behavior row is added, it is listed among the covered sites, and AC-8 and TC-042 cover it. |
| FND-003 | fixed | e26183b: AC-8 excludes keywords (`in`, `let`, ...) and tokens such as `!`, `(`, `,`. TC-042 step 8 passes `for l in [a, b]` and `vec![a]`. Applied by hand to the current bodies, no false positive is left. |
| FND-004 | fixed | e26183b: AC-6 and AC-8 name `rewrite_duplicate_position`, and AC-8 forbids moving a site into an unnamed helper. |
| FND-005 | fixed | e26183b: the NFR-005 row says the `GenerationErrorCode::InvalidGeneratedSyntax` doc is widened. The interface-001 misplacement is recorded as FND-009. |
| FND-006 | fixed | e26183b: AC-8 defines the subtraction token (binary `-`, `-=`; not `->` and not unary minus). |
| FND-007 | fixed | fc1527b: the rationale is corrected. Only `path()[0]` rests on a type, the others "rest on generator code and are the same class as the covered sites", and they are deferred and not shown safe. The reason given for the cut is recorded as FND-012. |
| FND-008 | fixed | fc1527b: `harness_binding`, `typed_dependency_parameters` `path()[0]` and the location-map `function_index` read move to the guarded-in-own-function group. I re-measured: `runner_source` (generation.rs:591), `oracle_arguments` (harness.rs:841) and `dependency_key` (boolean_v1.rs:813) have no length check of their own, so they belong in the outside-invariant group. |
| FND-009 | fixed | fc1527b: `kani_slice.refusal` is byte-identical to origin/main, and a new `oracle_slice.invalid_generated_syntax` key names `GenerationErrorCode::InvalidGeneratedSyntax` with the widened meaning. |
| FND-010 | fixed | fc1527b: NFR-005 Statement and TC-042 Description both say four. |
| FND-011 | fixed | fc1527b: the operand-ending set adds `}`, `?` and numeric literals, and TC-042 step 8 flags `x?[0]`, `t.0[1]`, `{ v }[0]` and `x? - 1`. Applied by hand to the four bodies (with the planned rewrites) and to every step-8 case, the rule gives the expected result with no false positive. |
| FND-012 | fixed | 62c8066: Scope states the cut as "a scoping choice for IR-577, made on the measurement", and the false ticket-naming reason is gone. "deferred" no longer appears in NFR-005, TC-042, FR-022 or interface-001. |
| FND-013 | fixed | 493495c: TC-042 Expected Results now says the sites are left out "by a scoping choice for IR-577", matching NFR-005. My own sweep of spec/ finds no remaining 'known limit', 'deferred' or 'three sites' wording for IR-577. |
