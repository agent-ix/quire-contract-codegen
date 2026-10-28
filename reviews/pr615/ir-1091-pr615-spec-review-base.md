---
id: SR-615001
title: IR-1091 PR 615 unique collection capacity refusal — base checklist
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; IR-1091; PR 615 head 18ebb99edf790cd9546ad2fa8d3329a2bf758fbe vs main 92ebb4e2943a47cc172a4237c82077c8b7da9859; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md"
review_set: subset
---

## Summary

Ticket: IR-1091. Independent base-checklist review of draft PR 615 at exact
head 18ebb99edf790cd9546ad2fa8d3329a2bf758fbe against fresh main 92ebb4e2.
The PR adds one capacity rule to FR-015-AC-96 and one TC-025 procedure
paragraph. The rule defines a finite distinct-value capacity for admitted
unique `Boolean` (2) and `IntRange` (`upper - lower + 1`) fields and
settles `max > capacity` as `unsupported` with
`CollectionClaimNotRendered` and a typed
`UniqueElementCapacityExceeded { capacity, declared_maximum }` reason, no
harness and no proof credit. The feasible obligation (`max <= capacity`,
equality included) is kept intact. No other AC row changed. The spec tree
validates with `quire validate --scope . "spec/**/*.md"` (exit 0, no
FR-015/TC-025 diagnostics).

## Verdict

PASS WITH FINDINGS. The new rule is semantically sound. It preserves AC-96's
inclusive-cardinality and uniqueness obligations when the maximum is feasible,
and it distinguishes finite distinct-value capacity from AC-101's
representability and resource ceilings. It handles min > capacity and
max == capacity explicitly. The capacity formulas match the Boolean and
inclusive IntRange domains. It forbids a vacuous proof, a harness, a reduced
maximum, a non-unique substitute and a narrowed element domain. It does not
reclassify the field as invalid.

The new reason follows the existing `CollectionUnsupportedReason` style.
`UnwindBelowCollectionBound { required, supplied }` and
`SymbolicDomainExceedsMemoryCeiling` in `src/kani/generate/outcome.rs:441-470`
both carry their values the same way, so the taxonomy does not need a new
top-level `UnsupportedObligation` variant.

The current code refuses every unique field as `UnsupportedFieldType`
(`tests/it/kani_obligations_symbolic_domains.rs:1446`). The existing fixture
`items` (SmallInt [0,3], max 3, capacity 4) stays feasible, so that control
is unaffected. The `flags` fixture (Boolean, max 2) is a ready-made equality
case.

Two findings need the owner's or author's attention:

- FND-001 (high, required by the checklist): a commit SHA in the edited row.
  It is inherited, not introduced by this PR.
- FND-002 (medium): the precedence between the capacity refusal and AC-97's
  unsupported-form refusal is not stated.

No AssuranceProfile exists in the repo, so no enforced human-gate profile
applies.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The edited FR-015-AC-96 row keeps the commit SHA `fe20ecc` in "PARTIAL (IR-268, CG #396 fe20ecc)", a recorded-revision pin that the hash/digest/pin checklist and the repo CLAUDE.md forbid. It is inherited, not introduced by this PR, and the same pin appears in AC-97 and AC-99 to AC-102. The fix is to drop the SHA and keep the PR reference. | spec/kani/functional/FR-015-bounded-kani-obligations.md:846 |
| FND-002 | medium | The new rule does not say which refusal wins when an admitted unique field has max > capacity but its collection form is one the generator cannot encode yet, for example an unordered unique Set. The capacity rule requires `UniqueElementCapacityExceeded`; FR-015-AC-97 requires `unsupported` naming the unencodable form; and today every unique field is refused as `UnsupportedFieldType` at lowering, before `render_collection`. Two implementers would report different typed reasons. | spec/kani/functional/FR-015-bounded-kani-obligations.md:846; spec/kani/functional/FR-015-bounded-kani-obligations.md:847; src/kani/generate/collection.rs:73-85 |

## Coverage

Examined: the FR-015-AC-96 row before and after (the only AC row in the diff)
and the new TC-025 paragraph (lines 71-86). Context: FR-015-AC-97 (form and
unbounded refusals), FR-015-AC-99 and AC-100 (proof and identity, unaffected),
FR-015-AC-101 (representational and resource ceilings) and FR-015-AC-102 (one
disposition per item). Code read only to test assumptions:
`src/kani/generate/collection.rs` `render_collection` and
`CollectionDomainRefusal`; `src/kani/generate/outcome.rs`
`UnsupportedObligation::CollectionClaimNotRendered` and
`CollectionUnsupportedReason`; and the `model_document` and
`ordered_set_shape_is_refused_without_erasing_its_uniqueness_semantics`
fixtures.

No AC other than AC-96 is narrowed: AC-97, AC-99, AC-100, AC-101 and AC-102
text is unchanged. AC-96's own harness obligation is narrowed only for
`max > capacity`, which is the intended owner decision and is explicitly
marked PLANNED pending that decision. Checklist: AC id format holds and no
`TC-` id appears in a Verification cell. No new hash, digest or pin was
introduced; FND-001 is inherited.

Methods run: spec-review/base, ears-conformance (SR-615002), failure-domain
(SR-615003) and evidence (SR-615004). Code-review, rust-review and
gap-analysis were not run: the diff is spec-only and the brief limited the
lens set. No Cargo, Kani or test gate was run.

## Dispositions

Round 1, exact fix head 00c799c99c82626ab55109d7e6750baaba5f4d73 (fix delta 18ebb99e..00c799c9; merge-base with main 92ebb4e2). Checked against the fix diff and the current spec/code context; `quire validate --scope . "spec/**/*.md"` exits 0. The PR remains a draft for the owner decision; AC-96 is not complete.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Line 846 now reads "PARTIAL (IR-268, CG #396)" with no SHA; `fe20ecc` no longer occurs in the AC-96 row. It remains in the unedited AC-97 and AC-99 to AC-102 rows, which are outside this PR's edited row, per the scoped fix. |
| FND-002 | fixed | AC-96 now restricts the rule to a supported collection form and states that the FR-015-AC-97 unsupported-form refusal takes precedence; TC-025:94-101 requires `UnsupportedFieldType` for an unsupported form paired with max above capacity. |
