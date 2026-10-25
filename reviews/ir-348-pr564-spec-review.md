---
id: SR-17018
title: "IR-348 CG564 scoped spec-review of the FR-014 and TC-024 path edits"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen; IR-348; PR564 fix round; exact da6d12a72e92f080eaae4887dfd52b122c4777a5 (fix diff 1aa7807f..da6d12a7); spec/oracle/functional/FR-014-exact-scalar-oracles.md:525,575,578; spec/oracle/matrix/TC-024-exact-scalar-oracles.md:309-313,318"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-014", type: references }
  - { target: "ix://agent-ix/quire-contract-codegen/TC-024", type: references }
---

## Summary

Ticket: IR-348. This scoped spec-review (integrity and consistency) covers the two spec files the fix round changed, and only the changed lines. Both files pass `quire validate --scope .` (exit 0). The one warning, `ears:non-canonical-trigger` at FR-014 line 269, is identical at `1aa7807f` and outside the changed lines. EARS conformance is recorded separately in SR-17019.

The edits replace the single scanned scalar file `src/oracle/scalar/mod.rs` with the four scalar files. They do this in the FR-014 panic-token statement (line 525), FR-014-AC-39, FR-014-AC-42, and TC-024 limit step 4 and panic-scan step 1. They also relocate AC-42's single `Failed` arm to `src/oracle/scalar/lower.rs`.

No criterion is narrowed:
- The panic scan's file set grows from one file to four.
- The equality and function obligations of AC-42 (no direct-lowering arm, `PackageAccounting` retained) are unchanged.
- The token list and the cfg(test) and comment exclusion rules are byte-identical.

The path claims are accurate at `da6d12a7`. The only scalar `CompleteLoweringRecordV2::Failed` arm in non-test code is at `src/oracle/scalar/lower.rs:191`. `tc_024_ac39_*`, `tc_024_ac42_*` and `oracle_generators_have_no_panicking_arms` all scan the four files. The shared readers moved to `src/core/ir/{checked_term,scalar_bounds}.rs` contain no emitted string literals, and the whole-`src` NFR-005-AC-1 walk scans them with the same `panic_tokens_in` scanner.

## Verdict

PASS with one low wording finding. The edits are accurate, consistent with each other and with the code, and they narrow no criterion. A separate gap finding, SR-17016 FND-004, notes that AC-42's new location clause has no test behind it.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-014-AC-42's file list now reads "`mod.rs`, `derive.rs`, `lower.rs` and `render.rs`, `equality/mod.rs` or `function/mod.rs`". The mixed `and`/`or` series is grammatically ambiguous. Its closing sentence, "Each relevant file names `limit_kind` only as the field of its own `LoweringLimitUnrecognised` refusal", leaves "relevant" undefined. `derive.rs` and `render.rs` own no such refusal, and the test counts `limit_kind` across the concatenated scalar set rather than per file. | spec/oracle/functional/FR-014-exact-scalar-oracles.md:578 |

## Coverage

Examined units:
- FR-014 statement line 525
- FR-014-AC-39
- FR-014-AC-42
- TC-024 step 4 (lines 309-313) and panic-scan step 1 (line 318)

Read as context only: FR-039 line 231, which still correctly places `generate_contract_scalar_oracles` and `preflight_contract_scalar_claim` in `src/oracle/scalar/mod.rs` (mod.rs:858, 872).

Plan completion: not assessed

## Examined scope and bindings

```yaml
clean: false
scope:
  - {id: FR-014-AC-39, path: spec/oracle/functional/FR-014-exact-scalar-oracles.md, role: examined, excerpt: "The generated `src/lib.rs` of every crate the TC-024 corpus generates, and the non-test code of `src/oracle/scalar/mod.rs`, `src/oracle/scalar/derive.rs`, `src/oracle/scalar/lower.rs` and `src/oracle/scalar/render.rs` (comments and every `#[cfg(test)]` item removed wherever the item sits, string literals the generator emits counted), contain zero panic tokens"}
  - {id: FR-014-AC-42, path: spec/oracle/functional/FR-014-exact-scalar-oracles.md, role: examined, excerpt: "No `limit_kind` of a `failed` record is read in `src/oracle/scalar/mod.rs`, `src/oracle/scalar/derive.rs`, `src/oracle/scalar/lower.rs` and `src/oracle/scalar/render.rs`, `src/oracle/equality/mod.rs` or `src/oracle/function/mod.rs` but through the one shared classifier ... has exactly one such direct-lowering `Failed` arm across the scalar files, in `src/oracle/scalar/lower.rs` ... Each relevant file names `limit_kind` only as the field of its own `LoweringLimitUnrecognised` refusal and in the conversion into it."}
  - {id: FR-014-statement-525, path: spec/oracle/functional/FR-014-exact-scalar-oracles.md, role: examined, excerpt: "The scalar generator, `src/oracle/scalar/mod.rs`, `src/oracle/scalar/derive.rs`, `src/oracle/scalar/lower.rs` and `src/oracle/scalar/render.rs`, and every crate it emits shall contain no panic token in non-test code"}
  - {id: TC-024-limit-step-4, path: spec/oracle/matrix/TC-024-exact-scalar-oracles.md, role: examined, excerpt: "Take the non-test code of `src/oracle/scalar/mod.rs`, `src/oracle/scalar/derive.rs`, `src/oracle/scalar/lower.rs` and `src/oracle/scalar/render.rs`, `src/oracle/equality/mod.rs` and `src/oracle/function/mod.rs` ... and across the scalar files exactly one `Failed` arm in `src/oracle/scalar/lower.rs` that binds nothing and calls `classify_lowering_failure` (FR-014-AC-42)."}
  - {id: TC-024-panic-step-1, path: spec/oracle/matrix/TC-024-exact-scalar-oracles.md, role: examined, excerpt: "the non-test text of `src/oracle/scalar/mod.rs`, `src/oracle/scalar/derive.rs`, `src/oracle/scalar/lower.rs` and `src/oracle/scalar/render.rs`"}
  - {id: FR-039, path: spec/core/functional/FR-039-contract-package-consumption.md, role: context_only, excerpt: "The crate-internal scalar consumers `generate_contract_scalar_oracles` and `preflight_contract_scalar_claim` in `src/oracle/scalar/mod.rs`"}
bindings: []
```

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | low | The round-2 FR-014-AC-42 rewrite says the equality and function files name `limit_kind` "only in each file's own `LoweringLimitUnrecognised` refusal and any conversion into it". `origin/main` said "and in the conversion into it". "Any conversion" admits more than one conversion site, which loosens the AC text; the test still pins the exact counts (1 for equality, 3 for function), so it is stricter than the AC. Say "and, in `function/mod.rs`, its one conversion into the schedule refusal; `equality/mod.rs` has no conversion" to match the test. | spec/oracle/functional/FR-014-exact-scalar-oracles.md:578; tests/it/exact_scalar_generation.rs:3662-3678 |

## Dispositions

### Round 1 — exact `212bff4c65308b04723a114b01c68a5d381e0038` (fix diff `da6d12a7..212bff4c`)

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | FR-014-AC-42 is rewritten as separate sentences. The file list is "In each of `mod.rs`, `derive.rs`, `lower.rs`, `render.rs`, `equality/mod.rs` and `function/mod.rs`", with no mixed `and`/`or` series. "Each relevant file" is gone: the scalar files name `limit_kind` only as the field in `mod.rs` and its conversion in `lower.rs`, `derive.rs` and `render.rs` do not name it, and the equality and function files are covered separately. Both claims are true at this head (`mod.rs:624`, `lower.rs:207,211`). `quire validate` exits 0. |

The new finding FND-002 has no outcome yet; it is routed to the coder. The untested per-file location clauses of the same rewrite are recorded as SR-17016 FND-005. EARS conformance (SR-17019) was not re-run: round 2 changed no requirement statement, only the AC-42 table row.


Artifact note: the frontmatter `analysis` value was changed from `spec-review` to `integrity` this round, because quire 0.36.2 rejects `spec-review` as an `analysis` value. No finding text was changed.

### Round 2 — exact `5e83fe1b70d0bb0b34c1c6ad7912c8d4ef37c53d` (fix diff `212bff4c..5e83fe1b`)

| FND | outcome | reason |
| --- | --- | --- |
| FND-002 | fixed | FR-014-AC-42 now says that `src/oracle/equality/mod.rs` names `limit_kind` only as the field of its own refusal, with no conversion in that file, and that `src/oracle/function/mod.rs` names it only as the field and "in its one conversion from `LoweringFailure::LimitUnrecognised` into `schedule::ScheduleRefusal::LoweringLimitUnrecognised`". This matches the test's pinned counts (1 and 3) and is tighter than `origin/main`'s "the conversion into it". No criterion is narrowed, and `quire validate` exits 0. |
