---
id: SR-11800
title: "IR-268 PR #396 code review (rust-review lane) at head 85c0e97"
type: SpecReview
scope: "agent-ix/quire-contract-codegen#396 @85c0e9747ad5ca0dad5411087c93895a9ee99050 vs origin/main b8bc0f0; src/, tests/, Cargo.toml, Cargo.lock, Makefile"
relationships: []
---

# SR-11800: IR-268 PR #396 code review at head 85c0e97

## Summary

Ticket: IR-268. PR agent-ix/quire-contract-codegen#396, frozen head 85c0e97, rebased on
origin/main b8bc0f0, diff `git diff origin/main...HEAD` (33 files). The rust-review lane is
folded into this file. Spec-text quality of the AD-004 and FR-014/TC-024 amendments is left to
the separate spec reviewer. The code and tests were checked against those amendments.

## Method

I read the whole src/ diff: `kani/generate/collection.rs`, `negotiate.rs`, `outcome.rs`,
`kani/identity.rs`, `kani/run/{harness,execute}.rs`, `oracle/scalar/mod.rs`, `routed/capability.rs`,
`replay/*`, `lib.rs`. I read the test diffs and the new `tests/it/kani_obligations_symbolic_domains.rs`.
I also read the locked runtime source (quire-contract-runtime 3cc88c4, `src/exact/quantity.rs`).

Checks run, one cargo invocation at a time, with `CARGO_TARGET_DIR=<clone>/target-review`:

- `cargo test --locked --lib`: exit 0, 271 passed.
- `cargo test --locked --test it -- exact_scalar_generation routed_generation
  kani_obligations_symbolic_domains cover_last layout skeleton_spine composite_parity_converter
  exact_scalar_agreement exact_function_agreement`: exit 0, 180 passed, 2 ignored (Kani lane).
- `cargo clippy --locked --all-targets -- -D warnings`: exit 0.
- Mutation run (then reverted with `git checkout`). I disabled the `quantity.convert` guard and
  mapped `quire.op.quantity.convert` to a scalar family (`QuantityArithmetic(Add)`). This
  simulates restored scalar emission. I also forced `is_top_level_conjunct` to `true`. Exit 101,
  and all three targeted tests went red:
  `tc_024_quantity_conversion_is_refused_with_its_named_capability` (derivation returned
  `Ok(QuantityArithmetic{Add})`), `tc_033_a_quantity_conversion_is_unsupported_with_its_named_capability_and_no_harness`
  (it got `OracleRefused{FormMismatch}`), and
  `collection_claims_nested_under_not_or_disjunction_are_refused`.

Kani and the aggregate gate were not run, as the brief asked.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Cargo.toml is still edited: `sha2` was moved within `[dev-dependencies]`. The bytes differ from main, although the owner rules out Cargo.toml edits | Cargo.toml:40 |
| FND-002 | low | New code comments cite tickets (`IR-268`). Six new comments cite an upstream commit `IR f16d0fd` that is not the locked IR rev | src/kani/run/harness.rs:26; tests/it/kani_obligations_symbolic_domains.rs:1; src/replay/composite_builder.rs:157 |
| FND-003 | low | `QUANTITY_CONVERSION_CAPABILITY` says the runtime "has no quantity conversion target", but the locked runtime exports `QuantityTarget::{Exact,Decimal,Integer}` and `convert_quantity` | src/oracle/scalar/mod.rs:3814-3815 |
| FND-004 | low | `collection_identity_fields_are_discriminating_with_constant_package_digest` is tautological: it edits a cloned identity and asserts that the derived `PartialEq` sees the edit | tests/it/kani_obligations_symbolic_domains.rs:1461-1501 |

### FND-001

At main, `sha2 = "=0.10.9"` is the last line of `[dev-dependencies]` (Cargo.toml:47). At head it
is the first line of that section (Cargo.toml:40). Commit ef6fff1 moved it to `[dependencies]`.
The fix in d1e0735 moved it back to `[dev-dependencies]`, but at a different line, so Cargo.toml
still differs from main by a reorder. Cargo behaves the same either way. It is still a Cargo.toml
edit, which the owner rules out. Fix: restore Cargo.toml to main's bytes. The diff should then
leave Cargo.toml untouched.

### FND-002

There are two new ticket citations in comments. `/// A V2 admitted symbolic collection claim
(IR-268).` is at src/kani/run/harness.rs:26, and `//! IR-268 source-backed symbolic collection
...` is at tests/it/kani_obligations_symbolic_domains.rs:1. Six new comments in
src/replay/composite_builder.rs (lines 157, 245, 280, 340, 382, 438) cite "IR f16d0fd's 192-byte
refusal". f16d0fd is not the locked quire-contract-ir rev (38f35814), so the stated reason will go
stale silently. These comments should state the reason, the size of the `PackageRead` refusal,
without a ticket or commit id.

### FND-003

The locked quire-contract-runtime (3cc88c4, src/exact/quantity.rs:140-240) has
`pub enum QuantityTarget { Exact, Decimal(DecimalType), Integer { .. } }` and
`pub fn convert_quantity(source, unit, target, meter)`. The real gap is narrower. IR now requires
a `type_argument` declaration equal to the result type, which makes a conversion node quantity-typed
(the fixtures at tests/exact_scalar_support/package.rs:2669-2700 are unit to unit under `exact`).
The runtime has no target whose result is a `Quantity`. A reader of the refusal is told that the
runtime has no conversion target at all, which is false at the locked rev. The text should name
the missing capability exactly, for example "no quantity-valued conversion target". FR-014 quotes
this text, so the spec wording should change with it.

### FND-004

This test was the response to SR-11681 FND-002. It never regenerates anything. Its macro clones
`base`, overwrites one public field and asserts `base != changed`. That holds for any struct with
derived `PartialEq`, whatever `render_collection` writes into the identity or into
`symbol_material`. Example: drop `minimum_length`, `maximum_length`, `element_domain` and
`predicate` from `symbol_material` in collection.rs:213-236. The test still passes. The real values
are pinned elsewhere, at lines 975 and 1346, by absolute assertions. So this test adds no evidence
for FR-015-AC-100. Replace it with a regeneration over the same package that varies only a
request input. Alternatively, assert that the harness symbols change when only the identity field
under test changes.

## Answers to the leader's questions

- Do the new tests go red if scalar emission for `quantity.convert` is restored? Yes, along the
  derivation path. With the guard removed and the identity mapped to a scalar family, both tests
  failed (log `mutation-m2-m3.log`, exit 101). A restoration placed after the guard is
  unreachable, because the guard tests the identity before `family_of_identity`. Neither test
  drives the caller-declared `generate_exact_scalar_oracles` path for a conversion node. Today
  that path cannot emit, because `ExactScalarOperation` has no unary unit-to-unit variant, so
  every caller-declared descriptor fails the shape check. A future re-added variant would not be
  caught by these two tests.
- Is any scalar `quantity.convert` path left in src? No. `QuantityTarget`, `convert_quantity`,
  `QuantityConversion` and `rt::Conversion` do not occur in src/. The only `quantity.convert`
  occurrences are the guard and its variant (src/oracle/scalar/mod.rs:3837-3838, 4196-4202).
  `routed/capability.rs` no longer routes the variant.
- Is the src/lib.rs re-export allowed by AD-004? Yes. The `lib` row of AD-004 says it "re-exports
  by explicit list, no logic". The additions (`QUANTITY_CONVERSION_CAPABILITY`,
  `CollectionUnsupportedReason`, the collection identity types) are explicit names. The L-2
  root re-export test passes.
- Was admission loosened? No loosening found. The new `String`/`Uuid`/`Timestamp` arms in
  frame.rs:961-963, replay/frame.rs:380-382 and replay/function.rs:239-241 are refusal arms
  (`NonRangeType`, `UnsupportedWitnessValue`). The `type_argument` fixtures are stricter, and
  `HarnessProvenanceMismatch` adds a refusal. The collection family is additive and refuses
  every unmatched shape with a typed reason.
- Is OPERATION_VARIANTS 15 -> 14 honest? Yes. `ExactScalarOperation` has 14 variants at head, and
  the exhaustive `expected_family` match compiles and passes in `cargo test --lib`.
- Leader claims verified: Cargo.lock pins quire-contract-ir and quire-contract-model at 38f35814.
  The V2 fixtures build `{kind, declaration}` `type_argument` members. cover_last drives
  `kani_obligations_symbolic_domains::guard_sources()` and counts collection.rs as one template.
  AD-004 and layout L-2 list `collection` at rank 3 with imports `outcome` and `record`.
- Hard rules: there are no version-ceiling raises, no vendoring and no compatibility layer. No
  `cfg(kani)` fork was added; the generated harness's `#[cfg(kani)] mod` is the pattern every
  family uses. The Cargo.toml edit is FND-001.

## Verdict

The collection family is sound for the shapes it admits. It asserts a top-level conjunct only,
checks the unwind against the maximum length plus one, takes bounds from the typed field domain,
admits out-of-range literals as falsifiable, puts the cover last and seals provenance at
execution. The quantity-conversion refusal is complete in src, and its tests discriminate. All
four findings are low, with no wrong runtime behaviour. FND-001 breaks the owner's hard rule
against Cargo.toml edits and is a one-line revert, so I would fix it before merge.

## Dispositions

Disposition pass 1 at fix-round commit 7e16de1 (previous head 85c0e97). The checks ran one
cargo invocation at a time:

- Focused `cargo test --lib --test it` over exact_scalar_generation, routed_generation,
  capability_settlement, kani_obligations, layout, exact_scalar_agreement, exact_function_agreement,
  cover_last, kani::generate::collection and oracle::scalar: exit 0 (lib 23 passed; it 289 passed,
  14 ignored).
- Mutant run (reverted): exit 101.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | `git diff origin/main..HEAD -- Cargo.toml` is empty: Cargo.toml is byte-equal to main. |
| FND-002 | fixed | harness.rs:26 and the test-module header no longer cite IR-268. The six composite_builder.rs comments state the reason without a commit id. 7e16de1 adds no new ticket citation to a code comment. |
| FND-003 | fixed | The capability text now says the runtime converts a quantity only to a scalar target and has no conversion whose result is a quantity. That matches runtime 3cc88c4. |
| FND-004 | fixed | The tautological test is deleted. The lib test `every_identity_component_changes_the_rendered_harness_path` renders through `render_collection` with one preimage field varied at a time. A mutant that drops `predicate_id` from `symbol_material` goes red. |
