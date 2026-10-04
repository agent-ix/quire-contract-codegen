---
id: NFR-005
title: "No panic on a generation or analysis path"
type: NFR
quality_attribute: reliability
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: constrains
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: constrains
  - target: ix://agent-ix/quire-contract-codegen/FR-019
    type: constrains
  - target: ix://agent-ix/quire-contract-codegen/FR-021
    type: constrains
  - target: ix://agent-ix/quire-contract-codegen/FR-022
    type: constrains
---
# NFR-005: No panic on a generation or analysis path

## Statement

- The generator's production code under `src/` shall contain no panic token: no `unwrap`, `expect`,
  `unreachable!`, `assert!` or any other call FR-014-AC-39 lists, so that no panic token aborts the
  generator instead of a typed refusal.
- The sites of the four IR-577 behavior-table rows (in `generate_kani`,
  `observe_clause` and `generate_boolean_oracle_inner`) shall not index, slice or subtract on a
  value that an earlier function establishes.
- Each of those sites shall read the value with a checked accessor (`get`, a slice pattern,
  `checked_sub`, or a value built before it is stored) and, where the read can fail, return the
  typed refusal its row names.
- No other index, slice or subtraction site is covered: Scope measures them and says why each is
  left unchanged.
- The code shall use a guarantee an earlier pass establishes by pattern matching, by iterating the
  earlier result, or by carrying the earlier pass's value forward, and shall not re-derive it behind
  a panic token or exempt the site as an invariant.
- If a condition cannot be expressed that way, then the code shall return the typed refusal this
  requirement names for the site, or, if the enclosing function already returns a typed refusal for
  the same cause, that refusal.

## Scope

- Applies to: every non-test `.rs` file under `src/`, including the Kani, routed, evidence, replay
  and publication code that FR-014-AC-39, FR-018-AC-19 and FR-021-AC-21 do not scan.
- Excludes: `#[cfg(test)]` items, a file whose parent module declares it under `#[cfg(test)]`
  (today `src/kani/test_support.rs`), comments, and string and character literals. A literal is
  excluded because the Kani generators emit `assert!` into the harnesses they render, which is the
  harness's own proof obligation and not a generator panic, and because `src/oracle/boolean_v1.rs`
  names the expression kind `"unwrap"` in a literal.
- The NFR-005 scan covers non-test code with literals removed. FR-014-AC-39, FR-018-AC-19 and
  FR-021-AC-21 are separate checks of the three oracle generators: FR-014-AC-39 and FR-018-AC-19
  count literals over the full token list, and FR-021-AC-21 counts four macros over the whole file.
  The oracle generators' emitted source stays under FR-014-AC-39, FR-018-AC-17 and FR-021-AC-19.
  This requirement edits none of them.
- Index, slice and arithmetic panics: no lexical scan of identifiers finds them, so NFR-005-AC-1
  does not. IR-577 measured every index, slice and `as` cast expression and every `+ - * / %` over
  integers in the non-test code of `src/` (CG main 3d13550). None is reachable from untrusted input
  today. Four sites are covered (behavior table): `driver[*first_index]` in `generate_kani` (its
  comment reads "index directly and panic rather than leave it unmapped"), `map[1]` and `map[2..]`
  in `observe_clause`, and in `generate_boolean_oracle_inner` the two
  `source_lines[.. as usize - 1]` reads and the `regions[0]` write. The rest are left unchanged, in
  two groups.
  - Guarded inside their own function: `refuse_inconsistent_routing`,
    `reject_duplicates_and_mixtures`, the renaming loops of `src/kani/generate/negotiate.rs` and the
    claim-name loops of `src/oracle/{function,equality,scalar}/mod.rs` and `src/core/naming.rs`
    (`enumerate`, `windows(2)`, or a map built from the same sequence); `check_maps`
    (`lines[p.line as usize - 1]` after the `||` terms before it fix the line and column ranges);
    `read_block` and the playback scan (offsets that `find` and `split_once` return);
    `decode_values` in `src/replay/witness.rs` (`copy_from_slice` after the width check above it);
    `compute_census` and `ScalarOperation::reachable` (`i128` over `i64`); and the `as u32` casts of
    line and column numbers, bounded by `MAX_GENERATED_SOURCE_BYTES`; `harness_binding`
    (`path()[0]` after its own `path().len() != 1` refusal) and `typed_dependency_parameters`
    (`path()[0]` after its own `path().len()` checks); and the location-map
    `function_index[&node_id]` read in `generate_exact_function_oracles`, which builds the map in the
    same function.
  - Resting on an invariant that code outside the reading function establishes: the strategy
    renderers' `names.fields[0]` and `names.identifiers[0]` and `[1]` (`validate_names` in
    `census.rs`; `validate_identifiers` in `render_population`, a caller of `side_expression`);
    `path()[0]` in `runner_source`, `oracle_arguments` and `dependency_key` (a type,
    `DependencyIdentity`, refuses an empty path in its constructor and `Deserialize`, and the
    oracle generators, upstream of those reads, refuse a path of length other than one);
    `analysis.references[key]` in `typed_dependency_parameters` (the keys come from
    `analyze_supported_expression`, an earlier pass of the generator); `parameter_kinds[0]` and
    `[1]` in `validate_signature` (the caller builds `parameter_kinds` one per parameter and the
    function checks `declaration.parameters.len()`); and `function_index[&node_id]` in
    `item_disposition` (the caller builds the map). None is reachable from input today. Only
    `path()[0]` rests on a type; the others rest on generator code and are the same class as the
    covered sites. Leaving them out is a scoping choice for IR-577, made on the measurement: they were
    found unreachable from untrusted input today. They are neither shown safe nor covered, and a
    change that breaks one of those invariants aborts the generator.
  FR-018-AC-17 and FR-021's Behavior section keep their own index and arithmetic rules for emitted
  source.
- Known blind spots of the scan, none present in `src/` today: a panic macro imported under another
  name (`use core::panic as fail;` then `fail!(..)`), which the scan does not see because the
  `use` line has no `!` after `panic`, and `std::process::exit`, which is not on the token list.
- Interim exception, dated 2026-10-03 and tied to IR-344: `CaseIdentity::digest` in
  `src/kani/generate/corpus/bounded_kani_corpus.rs` is left to IR-344's layout step 1a (draft
  PR #222), which replaces its serialization and makes the failure reachable from input, refused as
  `KaniOutcomeKind::InvalidInput` with `kani_corpus_identity_unencodable`. Until that code lands the
  scan allows one `expect`, the one inside the body of `fn digest` of `impl CaseIdentity` in that
  file: it locates that function's body, asserts it holds exactly one `expect` there, and allows
  only that one. Any other `expect` in the file, including one in `render_artifacts`, fails the
  scan, so a later merge cannot bring back a site this requirement fixed. The change that lands
  IR-344's `digest` removes the allowance; it expires at that merge. Whatever that change
  serializes in `render_artifacts`, `render_artifacts` carries the
  `kani_corpus_serialization_failed` refusal this requirement names.

## Rationale

`panic_scan` and FR-021-AC-21 ban the panic tokens in three oracle generators and, in
`src/oracle/function/mod.rs`, ban only four macros. A measurement of CG main (IR-543) found fourteen
panic sites outside that cover: the twelve `unwrap`, `expect` and `unreachable!` sites the ticket
reported, and an `assert_eq!` and a `debug_assert!` it did not. Each is behind an invariant an
earlier pass establishes, or is a failure of plain data, but an invariant that is wrong, or that a
later change breaks, turns a request into an abort of the whole generator with no refusal, no
diagnostic and no partial output. The scalar and equality generators already hold the full rule and
carry the same kind of invariants without a panic.

This change adds one public enum variant, `RoutedGenerationError::KaniRecordCountMismatch`
(FR-022, interface-001), and one refusal code, `kani_corpus_serialization_failed`. It adds no other
public error type.

IR-577 extends this requirement and does not add a sibling: it is the same failure (an invariant that
breaks aborts the generator with no refusal), it shares the scope, the dependencies and TC-042, and
a sibling would copy all three. It adds one more public enum variant,
`RoutedGenerationError::KaniDuplicatePositionOutOfRange` (FR-022, interface-001), and no refusal
code. The measurement found no index, slice or arithmetic panic reachable from untrusted input; the
covered sites are held by a guarantee another function gives (or, for the `regions[0]` write, by
nothing at all, and the row removes the index), which is the class this requirement already treats
as a site (see the invariant rows above). Other sites of the same class are left out by a scoping
choice, not covered, and named in Scope.

## Behavior of each measured site

Each site is named by function, not by line. The class is how the code must express it.

| Site | Class | Required expression |
|------|-------|---------------------|
| `generate_exact_function_oracles` (`src/oracle/function/mod.rs`), the `own_shape.get(..).unwrap()` read in the resolution loop | invariant of the loop above it | The loop reads each function's own Stage 1 result from the sequence that loop built, not from a map keyed by node id, so no lookup can miss. |
| the same function, the two `.expect(..)` calls that resolve a survivor's parameter and result types in Stage 2 | invariant of Stage 1 | Stage 2 reads the parameter and result kinds Stage 1 resolved for that function and carried forward, and does not resolve them again. |
| the same function, `TypeEnvironment::new(..).expect(..)` in the generation-time package check | invariant of an empty declaration set | The empty environment is `rt::TypeEnvironment::default()`, as FR-021 already requires of the emitted `checked_package()`. |
| `ScalarOperation::reachable` (`src/kani/generate/scalar.rs`), the `_ => unreachable!` arm | invariant of the operand count | The caller builds the operand ranges from the operation's own operand count, so `reachable` takes fixed-arity operands per operation (one operand for `Negate`, two for the others) and has no arm left over. No refusal is added. |
| `lower_scalar_claim` (`src/kani/generate/scalar.rs`), the two `unreachable!` sites | invariant of `check_parameters`, which a caller-supplied claim map need not satisfy | A claim with no first checked bound, and a first checked bound with no `IntegerRange` among the derived domains, each return `Err(ScalarLoweringRefusal::NoRenderer)`, the refusal the same function returns for a claim map this generator did not produce. `classify_claim` maps it to `UnsupportedObligation::OperationNotRendered`, unchanged. The doc comments of `ScalarLoweringRefusal::NoRenderer`, of `UnsupportedObligation::OperationNotRendered` and the comment on `ScalarLoweringRefusal` that says these two causes are asserted with `unreachable!` are rewritten to say "no renderer for its family yet, or a claim map this generator did not produce". |
| `ItemSettlement::warning` (`src/routed/capability.rs`), the `unreachable!` arm over the six `invalid_capability` causes | caller-reachable: `ItemSettlement`, `Disposition` and `Cause` have public fields, so a caller can build `Disposition::Unsupported` with such a cause | The arm returns `None`, the value the `InvalidRequest` arm returns. `None` means this settlement has no warning to name, since the cause is not an `unsupported_projection` one; CG never builds this value, and FR-019's rule that an `unsupported` settlement carries a warning holds for every settlement CG builds. The arm still names the six causes, so a new cause is a compile error, and `warning()`'s doc comment states the `None` case. |
| `observe_clause` (`src/evidence/bound_coverage.rs`), `region.probe.expect("map preflight")` | invariant of map preflight | With the coverage export supplied, a region with no probe is the `MapMismatch` diagnostic with the message `missing semantic probe` the preflight already uses, through the closure's existing `Result`. The closure keeps checking that coverage is supplied first, so without it the result stays `UnavailableObservation`. |
| the same function, `evaluation.expect("observed evaluation")` | invariant of the diagnostics check above it | The classification runs in a `match` or `if let` on `Ok(evaluation)` that also requires empty diagnostics, so no `Err` is unwrapped. |
| `render_artifacts` (`src/kani/generate/corpus/bounded_kani_corpus.rs`), `deterministic_json(&proof_graph_value).expect(..)` | serialization failure of plain data | `render_artifacts` returns a `Result`, and a failure is the case's refusal `KaniOutcome::non_success` with kind `KaniOutcomeKind::Refused` and code `kani_corpus_serialization_failed`, returned by `generate_bounded_kani_corpus_case`, with no artifact emitted, as for its other refusals. `Refused` and not `InvalidInput`, because the caller's input is not at fault. |
| `CaseIdentity::digest` (same file), `deterministic_json(self).expect(..)` | left to IR-344 | Not changed by this requirement's code. See the interim exception in Scope. |
| `generate_kani` (`src/routed/generate.rs`), `assert_eq!(records.len(), group.len())` | FR-015 reports one record per item | A different count is `RoutedGenerationError::KaniRecordCountMismatch { records, items }`, with nothing generated, so the `zip` after it never truncates. |
| `expression_diagnostic` (`src/oracle/boolean_v1.rs`), `debug_assert!(matches!(code, ..))` | caller contract, debug build only | The assertion is removed. The function builds the diagnostic for the code it is given in every build, as it does in a release build today. |
| `generate_kani` (`src/routed/generate.rs`), `driver[*first_index]` on a `DuplicateItem` record | invariant of FR-015: a `DuplicateItem`'s `first_index` is an earlier position in the group it was given, and `pair_records` has already fixed the record count to the group's. Not reachable from a caller today. | `generate_kani` hands the records to `route_records`, which calls `rewrite_duplicate_position` for each, and that function reads `group.get(*first_index)`. A position outside the group is `RoutedGenerationError::KaniDuplicatePositionOutOfRange { first_index, items }`, with nothing generated, and the comment that says the code panics rather than leave it unmapped is removed. |
| `observe_clause` (`src/evidence/bound_coverage.rs`), `map[1]` and `map[2..]` | invariant of `check_maps`, the only caller's guard: the map has the typed implication census plus two regions. Not reachable from a caller today (`observe_clause` is private). | The function destructures the map with a slice pattern, the evaluation region and the consequent regions that follow it. A map with fewer than two regions is the `MapMismatch` diagnostic with the message `typed implication census differs from map` (the text `check_maps` already uses for a wrong region count), pushed to the row's diagnostics, with no classification, an `evaluation_count` of `None` and no consequents, whether or not a coverage export is supplied. A map with two regions behaves as it does today. |
| `generate_boolean_oracle_inner` (`src/oracle/boolean_v1.rs`), `source_lines[offset as usize - 1]` and `source_lines[start as usize - 1]` | invariant of the renderer's own output: the fixed 15-line header makes `offset` exactly 15 and each implication region starts on a line it rendered. Not reachable from input today. | The lookup is `checked_sub(1)` then `get`. A missing line is the `GenerationErrorCode::InvalidGeneratedSyntax` diagnostic through `single_diagnostic`, with no artifact emitted. The source has parsed by then, so the doc comment of `InvalidGeneratedSyntax` in `src/core/diagnostic.rs` is widened to "Generated source did not parse, or did not match its own source map", and the stable code is unchanged (interface-001). |
| the same function, `regions[0].expected_consequents = ..` | write into a one-element vector built the line above. Cannot fail. | The clause region is built with `expected_consequents` set before it is placed in the vector (a struct update or a local binding), so the body holds no index. No refusal is added. |

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
|--------|--------|-----------|--------|
| Panic tokens in the non-test, literal-free code of `src/`, less the one dated exception | 0 | 0 | lexical scan, `tests/common/panic_scan.rs` |
| Measured sites a test can reach that abort instead of returning a typed value | 0 | 0 | unit tests at the four sites with a seam (NFR-005-AC-2 to AC-5) and the two IR-577 seams (NFR-005-AC-6, NFR-005-AC-7) |
| Index, slice and unchecked subtraction expressions in the bodies of the five IR-577 functions | 0 | 0 | lexical scan of those bodies (NFR-005-AC-8) |

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| NFR-005-AC-1 | Every `.rs` file under `src/`, except a file its parent module declares under `#[cfg(test)]`, contains zero panic tokens in its non-test code with comments, every `#[cfg(test)]` item and every string and character literal removed (`non_test_code_outside_literals`, a variant of `non_test_code` that `tests/common/panic_scan.rs` gains), except the one `expect` inside the body of `fn digest` of `impl CaseIdentity` in `src/kani/generate/corpus/bounded_kani_corpus.rs` that the dated exception of Scope allows until IR-344's `digest` lands; any other `expect` in that file, `render_artifacts` included, fails. The tokens are the identifiers `unwrap`, `expect`, `unwrap_unchecked`, `unwrap_err`, `expect_err`, `unwrap_err_unchecked`, `panic_any` and `resume_unwind` however written (called, with whitespace before the paren, named on a path such as `Option::unwrap`, or imported); the macros `panic`, `unreachable`, `todo`, `unimplemented`, `assert`, `assert_eq`, `assert_ne`, `debug_assert`, `debug_assert_eq` and `debug_assert_ne` in any delimiter form, with any whitespace before the `!` and any path prefix; and the identifier `abort` anywhere except as a method call (`.abort()`). The scan walks `src/` itself, so a file added later is scanned. It asserts that it read every file that holds a measured site (`src/oracle/function/mod.rs`, `src/kani/generate/scalar.rs`, `src/kani/generate/corpus/bounded_kani_corpus.rs`, `src/routed/capability.rs`, `src/routed/generate.rs`, `src/evidence/bound_coverage.rs` and `src/oracle/boolean_v1.rs`), the three files FR-014-AC-39, FR-018-AC-19 and FR-021-AC-21 name, and at least 60 files, and that it found exactly one `expect` inside that `digest` body. | Test (TC-042) |
| NFR-005-AC-2 | `lower_scalar_claim` returns `Err(ScalarLoweringRefusal::NoRenderer)` for a claim whose `checked_bounds` is empty and for a claim whose first checked bound has no `IntegerRange` among the derived domains, each built directly against the function, and `classify_claim` reports each as `UnsupportedObligation::OperationNotRendered`. | Test (TC-042) |
| NFR-005-AC-3 | `ItemSettlement::warning` returns `None` for a `Disposition::Unsupported` carrying each of `Cause::AbsentKind`, `UnknownKind`, `AbsentExtent`, `UnknownBackend`, `InconsistentCandidates` and `AmbiguousBackend`, and still returns its warning for each `unsupported_projection` cause. | Test (TC-042) |
| NFR-005-AC-4 | With a coverage export supplied, `observe_clause` given a map whose oracle-evaluation region has no probe returns a row with a `MapMismatch` diagnostic with the message `missing semantic probe`, no classification and an `evaluation_count` of `None`, and given one whose consequent region has no probe returns the same diagnostic with no classification. With no coverage export it returns `UnavailableObservation` for the same maps. | Test (TC-042) |
| NFR-005-AC-5 | The step of `generate_kani` that pairs FR-015's records with the Kani group, given hand-built inputs in a `#[cfg(test)]` module, returns `RoutedGenerationError::KaniRecordCountMismatch { records, items }` carrying both counts when the counts differ, in both directions, and pairs every item with its record when they are equal. | Test (TC-042) |
| NFR-005-AC-6 | `rewrite_duplicate_position`, a function of `src/routed/generate.rs` that `route_records` (which `generate_kani` calls to pair, rewrite and join FR-015's records) calls to rewrite a `DuplicateItem` record's `first_index` into the driver's request index, given hand-built records and a group in a `#[cfg(test)]` module, returns `RoutedGenerationError::KaniDuplicatePositionOutOfRange { first_index, items }` carrying the position and the group's length when `first_index` equals or exceeds the group's length, with nothing generated, and returns the request index of the earlier item when `first_index` is the last valid position; and `route_records`, given hand-built records of which one names a position outside the group, returns the same refusal and no output, so the refusal reaches the whole call. | Test (TC-042) |
| NFR-005-AC-7 | `observe_clause`, given a map of zero regions and a map of one region, each with and without a coverage export, returns a row with a `MapMismatch` diagnostic with the message `typed implication census differs from map`, no classification, an `evaluation_count` of `None` and no consequents; given a map of two probed regions and a coverage export it returns a row with no consequents and no `MapMismatch` diagnostic. | Test (TC-042) |
| NFR-005-AC-8 | The scan locates the body of `generate_kani`, `route_records`, `rewrite_duplicate_position`, `observe_clause` and `generate_boolean_oracle_inner` in the literal-free non-test code of its file, asserts that it found each of the five and that each holds an identifier of its own, and asserts that none contains an index token or a subtraction token, so that a later change cannot bring back one of the IR-577 sites. An operand-ending token is `)`, `]`, `}`, `?`, a numeric literal (so the `0` of a tuple field `t.0` counts), or an identifier that is not a Rust keyword (`in`, `let`, `mut`, `ref`, `return`, `break`, `if`, `else`, `match`, `while`, `as`, `move` are the keywords the bodies can precede a `[` or `-` with). An index token is a `[` whose previous non-whitespace token is operand-ending; a `[` after any other token (`(`, `,`, `=`, `:`, `&`, `#`, a keyword, or `!` as in `vec![`) is an array, a slice pattern, an attribute or a macro, and is not an index. A range slice is an index token, since it has a `[` in the same place. A subtraction token is a `-` that is not part of `->` and whose previous non-whitespace token is operand-ending; `-=` is a subtraction token too. A unary minus (previous token `(`, `,`, `=`, an operator or a keyword) is not. The scan allows no exception. Moving a covered site into a function this AC does not name is not allowed: the five named functions are exactly where the sites live, so an index in an extracted helper has to be named here first. | Test (TC-042) |

## Verification

TC-042 runs the scan over `src/`, the four seam tests, the two IR-577 seam tests
(NFR-005-AC-6, NFR-005-AC-7) and the body scan of NFR-005-AC-8. The `generate_boolean_oracle_inner`
lookup and `regions[0]` write have no fixture, since its header is fixed, and are held by
NFR-005-AC-8 alone. The sites with no fixture (the four
`src/oracle/function/mod.rs` sites, the `ScalarOperation::reachable` arm, the proof-graph
serialization and the `boolean_v1` assertion) are verified by NFR-005-AC-1 alone: each is an
invariant or a failure of plain data that no input reaches, so a test cannot build the case, and the
scan is what keeps a panic from returning.

## Dependencies

- **Upstream**: [FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md),
  [FR-021](../../oracle/functional/FR-021-function-application-oracles.md).
