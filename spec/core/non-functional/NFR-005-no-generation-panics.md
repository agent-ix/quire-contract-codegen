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
  `unreachable!`, `assert!` or any other call FR-014-AC-39 lists, so that no input a caller can
  build makes the generator abort instead of returning a typed refusal.
- Where an earlier pass guarantees a condition, the code shall use that guarantee by pattern
  matching, by iterating the earlier result, or by carrying the earlier pass's value forward, and
  the code shall not re-derive it behind a panic token or exempt the site as an invariant.
- Where a condition cannot be expressed that way, the code shall return the typed refusal this
  requirement names for the site, or, if the enclosing function already returns a typed refusal for
  the same cause, that refusal.
- The generator shall keep FR-014-AC-39, FR-018-AC-19 and FR-021-AC-21 as the stricter scans of the
  three oracle generators, and this requirement extends the ban to the rest of `src/` and, for
  `src/oracle/function/mod.rs`, whose FR-021-AC-21 bans four macros only, to `unwrap`, `expect` and
  the other panic tokens.
- The generator shall add no new public error type to meet this requirement: the one new variant it
  needs is `RoutedGenerationError::KaniRecordCountMismatch`.

## Scope

- Applies to: every non-test `.rs` file under `src/`, including the Kani, routed, evidence, replay
  and publication code that FR-014-AC-39, FR-018-AC-19 and FR-021-AC-21 do not scan.
- Excludes: `#[cfg(test)]` items, a file whose parent module declares it under `#[cfg(test)]`
  (today `src/kani/test_support.rs`), comments, and string and character literals. A literal is
  excluded because the Kani generators emit `assert!` into the harnesses they render, which is the
  harness's own proof obligation and not a generator panic, and because `src/oracle/boolean_v1.rs`
  names the expression kind `"unwrap"` in a literal. The oracle generators' emitted source stays
  under FR-014-AC-39, FR-018-AC-17 and FR-021-AC-19, which count literals.
- Excludes index and arithmetic panics, which no lexical scan of identifiers finds. FR-018-AC-17
  and FR-021's Behavior section keep their own index and arithmetic rules for the emitted source.

## Rationale

`panic_scan` and FR-021-AC-21 ban the panic tokens in three oracle generators and, in
`src/oracle/function/mod.rs`, ban only four macros. A measurement of CG main (IR-543) found fourteen
panic sites outside that cover: the twelve `unwrap`, `expect` and `unreachable!` sites the ticket
reported, and an `assert_eq!` and a `debug_assert!` it did not. Each is behind an invariant an
earlier pass establishes, or is a failure of plain data, but an invariant that is wrong, or that a later change breaks, turns a
request into an abort of the whole generator with no refusal, no diagnostic and no partial output.
The three oracle generators already hold the stricter rule and carry the same kind of invariants
without a panic.

## Behavior of each measured site

Each site is named by function, not by line. The class is how the code must express it.

| Site | Class | Required expression |
|------|-------|---------------------|
| `generate_exact_function_oracles` (`src/oracle/function/mod.rs`), the `own_shape.get(..).unwrap()` read in the resolution loop | invariant of the loop above it | The loop reads each function's own Stage 1 result from the sequence that loop built, not from a map keyed by node id, so no lookup can miss. |
| the same function, the two `.expect(..)` calls that resolve a survivor's parameter and result types in Stage 2 | invariant of Stage 1 | Stage 2 reads the parameter and result kinds Stage 1 resolved for that function and carried forward, and does not resolve them again. |
| the same function, `TypeEnvironment::new(..).expect(..)` in the generation-time package check | invariant of an empty declaration set | The empty environment is `rt::TypeEnvironment::default()`, as FR-021 already requires of the emitted `checked_package()`. |
| `ScalarOperation::reachable` and `lower_scalar_claim` (`src/kani/generate/scalar.rs`), the three `unreachable!` sites | invariant of `check_parameters` and of the operand count | A claim with no first checked bound, a first checked bound with no `IntegerRange` among the derived domains, and an operand-range list of a length other than the operation's operand count each return `Err(ScalarLoweringRefusal::NoRenderer)`, the refusal the same function returns for a claim map this generator did not produce. `classify_claim` maps it to `UnsupportedObligation::OperationNotRendered`, unchanged. |
| `ItemSettlement::warning` (`src/routed/capability.rs`), the `unreachable!` arm over the six `invalid_capability` causes | invariant of settlement | `Disposition::Unsupported` carrying one of those causes returns `None`, the value the `InvalidRequest` arm returns. The arm still names the six causes, so a new cause is a compile error. |
| `observe_clause` (`src/evidence/bound_coverage.rs`), `region.probe.expect("map preflight")` | invariant of map preflight | A region with no probe is the `MapMismatch` diagnostic with the message `missing semantic probe` the preflight already uses, through the closure's existing `Result`. |
| the same function, `evaluation.expect("observed evaluation")` | invariant of the diagnostics check above it | The classification runs in a `match` or `if let` on `Ok(evaluation)` that also requires empty diagnostics, so no `Err` is unwrapped. |
| `CaseIdentity::digest` and `render_artifacts` (`src/kani/generate/corpus/bounded_kani_corpus.rs`), the two `deterministic_json(..).expect(..)` calls | serialization failure of plain data | A failure is the case's refusal `KaniOutcome::non_success` with kind `KaniOutcomeKind::Refused` and code `kani_corpus_serialization_failed`, returned by `generate_bounded_kani_corpus_case`, with no artifact emitted, as for its other refusals. |
| `generate_kani` (`src/routed/generate.rs`), `assert_eq!(records.len(), group.len())` | FR-015 reports one record per item | A different count is `RoutedGenerationError::KaniRecordCountMismatch { records, items }`, with nothing generated, so the `zip` after it never truncates. |
| `expression_diagnostic` (`src/oracle/boolean_v1.rs`), `debug_assert!(matches!(code, ..))` | caller contract, debug build only | The assertion is removed. The function builds the diagnostic for the code it is given in every build, as it does in a release build today. |

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
|--------|--------|-----------|--------|
| Panic tokens in the non-test, literal-free code of `src/` | 0 | 0 | lexical scan, `tests/common/panic_scan.rs` |
| Measured sites that abort instead of returning a typed value | 0 | 0 | unit tests at each site's own seam |

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| NFR-005-AC-1 | Every `.rs` file under `src/`, except a file its parent module declares under `#[cfg(test)]`, contains zero panic tokens in its non-test code with comments, every `#[cfg(test)]` item and every string and character literal removed (`non_test_code_outside_literals`, a variant of `non_test_code` that `tests/common/panic_scan.rs` gains): the identifiers `unwrap`, `expect`, `unwrap_unchecked`, `unwrap_err`, `expect_err`, `unwrap_err_unchecked`, `panic_any` and `resume_unwind` however written (called, with whitespace before the paren, named on a path such as `Option::unwrap`, or imported); the macros `panic`, `unreachable`, `todo`, `unimplemented`, `assert`, `assert_eq`, `assert_ne`, `debug_assert`, `debug_assert_eq` and `debug_assert_ne` in any delimiter form, with any whitespace before the `!` and any path prefix; and the identifier `abort` anywhere except as a method call (`.abort()`). The scan walks `src/` itself, so a file added later is scanned, and it asserts that it read at least the files FR-014-AC-39, FR-018-AC-19 and FR-021-AC-21 name. | Test (TC-042) |
| NFR-005-AC-2 | `lower_scalar_claim` returns `Err(ScalarLoweringRefusal::NoRenderer)` for a claim whose `checked_bounds` is empty and for a claim whose first checked bound has no `IntegerRange` among the derived domains, each built directly against the function, and `classify_claim` reports each as `UnsupportedObligation::OperationNotRendered`. | Test (TC-042) |
| NFR-005-AC-3 | `ItemSettlement::warning` returns `None` for a `Disposition::Unsupported` carrying each of `Cause::AbsentKind`, `UnknownKind`, `AbsentExtent`, `UnknownBackend`, `InconsistentCandidates` and `AmbiguousBackend`, and still returns its warning for each `unsupported_projection` cause. | Test (TC-042) |
| NFR-005-AC-4 | `observe_clause` given a map whose oracle-evaluation region has no probe returns a row with a `MapMismatch` diagnostic with the message `missing semantic probe`, no classification and an `evaluation_count` of `None`, and given one whose consequent region has no probe returns the same diagnostic with no classification. | Test (TC-042) |
| NFR-005-AC-5 | The step of `generate_kani` that pairs FR-015's records with the Kani group, given hand-built inputs in a `#[cfg(test)]` module, returns `RoutedGenerationError::KaniRecordCountMismatch { records, items }` carrying both counts when the counts differ, in both directions, and pairs every item with its record when they are equal. | Test (TC-042) |

## Verification

TC-042 runs the scan over `src/` and the four seam tests. The sites with no fixture (the four
`src/oracle/function/mod.rs` sites, the two corpus serializations and the `boolean_v1` assertion)
are verified by NFR-005-AC-1 alone: each is an invariant or a failure of plain data that no input
reaches, so a test cannot build the case, and the scan is what keeps a panic from returning.

## Dependencies

- **Upstream**: [FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md),
  [FR-021](../../oracle/functional/FR-021-function-application-oracles.md).
