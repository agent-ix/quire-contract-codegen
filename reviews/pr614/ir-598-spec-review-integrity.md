---
id: SR-613003
title: IR-598 PR 614 AC-123 collection retirement reconciliation — integrity
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen; IR-598; PR 614 head 43bc439725a81c41f9361f35cd20dd4115f74d2e vs main fb50acdc82cd026c442cfa76681e195104b164e6; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/core/functional/interface-001-codegen-api.md, spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md, spec/assurance/AD-004-cg-crate-layout.md, spec/spec.md"
review_set: subset
---

## Summary

Ticket: IR-598. Independent spec review of draft PR 614 at exact head
43bc439725a81c41f9361f35cd20dd4115f74d2e against fresh main fb50acdc. The PR
changes only FR-015-AC-123 among acceptance criteria and reconciles
interface-001, the spec.md ownership map, TC-023 and AD-004's step-2f
inventory. The AC-123 time-bounding is correct: the collection-call
direct-result obligation ends at FR-015-AC-110, while the graph and corpus
obligations keep their full original strength (direct `Result<_, KaniOutcome>`
signatures, no `FamilyLoweringError`/`BoundedCorpusError`/`OutcomeConstruction`
branch, no alias or wrapper) until FR-015-AC-111. Every retirement adds a
no-replacement-alias/wrapper check, so the change introduces no compatibility
layer. The PR body's Before/After AC-123 text matches the diff byte for byte;
no other AC row changed. Two low/medium follow-ups: a stale "planned"
error_surface claim left in the edited interface block, and TC-023 not
directing the corpus direct-result control off the Collection request that
AC-110 deletes.

## Verdict

PASS WITH FINDINGS. The core change is sound and resolves the real conflict
between AC-123's timeless external-call wording and AC-110/AC-111's deletion
requirements, consistent with AD-004 step 4g and with the owner rule
authorizing retirement of legacy paths. No false live claim is introduced:
interface statuses, spec.md rows and AD-004's "historical step-2f inventory,
live only until" notes match current src (all three operations and their
direct signatures exist; `FamilyLoweringError`, `BoundedCorpusError` and
`OutcomeConstruction` are absent from `src/` and `tests/`). FND-001 (medium)
and FND-002 (low) should be fixed in this PR; neither blocks the AC-123
semantics.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-023's AC-123 procedure does not require the corpus direct-result control to use a family retained after AC-110; the only AC-123-tagged test drives the corpus through `BoundedCorpusRequest::Collection`, which AC-110 deletes, so graph/corpus AC-123 coverage can silently lapse between AC-110 and AC-111. | spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md:29; tests/it/bounded_kani_corpus.rs:901 |
| FND-002 | low | interface-001 `generate_bounded_kani_corpus_case.error_surface` still says "planned IR-621 retirement" although IR-621 landed and the PR rewrote the block's header comment to drop "planned", leaving a false planned claim inside the edited block. | spec/core/functional/interface-001-codegen-api.md:95 |
| FND-003 | low | AC-123's verification method is bare `Test`, but its new post-retirement clause (no exported alias or wrapper) is an absence check that TC-023 performs by inspection; AC-109/110/111 record the same kind of check as `Test (TC-023) and Inspection`. | spec/kani/functional/FR-015-bounded-kani-obligations.md:873 |

## Coverage

Examined every changed hunk in the five files plus direct dependencies:
FR-015-AC-86/87/88/89/109/110/111/123/124, interface-001 operations
`prepare_bounded_collection_query`, `prepare_finite_graph_reaches`,
`generate_bounded_kani_corpus_case`, TC-023 test procedure, AD-004 step-2f
inventory and spec.md ownership rows 135/136. Checked src/tests for the
retired error names (none) and the AC-123-tagged test
`public_family_and_corpus_results_have_direct_ir_errors_and_unchanged_successes`.

Checks:

- Collection obligation time-bounded at AC-110: yes, in AC-123, interface-001
  (comment, status, closing prose), spec.md row 136, TC-023 and AD-004.
- Graph/corpus obligation not weakened before AC-111: yes. Same signatures,
  same error-type ban, same no-alias/wrapper rule; TC-023 still requires the
  unconditional "retired public error names have no exported type, alias or
  replacement wrapper" inspection.
- No compatibility alias/wrapper: AC-123 adds an explicit post-retirement
  no-alias/wrapper clause; AC-110/111 no-wrapper inspection unchanged.
- PR body Before/After: exact match with the diff's removed/added AC-123 row.
- No unrelated AC narrowing: the only AC row in the diff is AC-123.

Methods: spec-review/integrity ran. spec-ears-analysis not applicable (no
requirement statement changed). spec-dependency-analysis not applicable (no
relationships edges changed). spec-criterion-strength-analysis skipped (Jev
client not used for this bounded review). Code-review, rust-review and
gap-analysis not applicable: spec-only diff, no Cargo/Kani/full-suite gate.

```yaml
scope:
- id: FR-015-AC-123
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until FR-015-AC-110 retires `prepare_bounded_collection_query`, external
    public-consumer calls to that collection operation have the direct result signature
    specified by `interface-001`, with `KaniOutcome` as its error type. Until FR-015-AC-111
    retires `prepare_finite_graph_reaches` and `generate_bounded_kani_corpus_case`,
    external public-consumer calls to those graph and corpus operations have their
    direct result signatures specified by `interface-001`, with `KaniOutcome` as their
    error type.
- id: FR-015-AC-110
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: After this slice no source under `src/` contains the corpus `Collection`
    variant and arm, `prepare_bounded_collection_query`, `render_collection_qsl` or
    `lower/bounded_collections.rs` and their re-exports.
- id: FR-015-AC-111
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: After this slice no source or test under `src/` or `tests/` contains the
    corpus `Graph` variant and arm, `prepare_finite_graph_reaches`, ... `generate_bounded_kani_corpus_case`,
    the `BoundedCorpusRequest`, `BoundedCorpusCase`, `BoundedCorpusError` ... types.
- id: FR-015-AC-124
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: After infallible-constructor adoption, each retained family/corpus adverse
    control returns the original exact non-success kind, typed cause, source identity
    and selected profile/bound context.
- id: interface-001
  path: spec/core/functional/interface-001-codegen-api.md
  role: examined
  excerpt: '# IR-621 direct error signatures below apply only while each operation
    remains public; FR-015-AC-110 retires collection and FR-015-AC-111 retires graph
    and corpus.'
- id: TC-023
  path: spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md
  role: examined
  excerpt: 'IR-621 direct-error procedure under FR-015-AC-123: from an external consumer
    test, explicitly assign the collection operation''s result to `Result<CollectionLowering,
    KaniOutcome>` until FR-015-AC-110; assign the graph and corpus operations'' results
    to `Result<GraphLowering, KaniOutcome>` and `Result<BoundedCorpusCase, KaniOutcome>`
    until FR-015-AC-111.'
- id: AD-004
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: '`bounded_collections.rs` to `kani/generate/lower/bounded_collections.rs`
    (1 production, 2 tests; historical step-2f inventory, live only until FR-015-AC-110).'
- id: spec-index
  path: spec/spec.md
  role: examined
  excerpt: The collection lowerer and its AC-123 direct-result obligation retire at
    AC-110; graph/profile lowerers and module entry retire at AC-111, ending the graph
    operation's AC-123 direct-result obligation.
```

## Dispositions

Round 1, fix head 72e7b1e366aa370c2d7771ee3f404dc8cedbeb1a (fix delta from 43bc4397; merge-base with main still fb50acdc). Verdict after round 1: PASS. All three findings fixed; no new findings. The delta changes only the FR-015-AC-123 verification column (widened to Test (TC-023) and Inspection), the interface-001 corpus error_surface tense and one TC-023 sentence, plus a committed copy of this review under reviews/pr614/. No other AC row changed. Out-of-scope note, not a finding: AD-003 line 130 and NFR-005 line 140 still say "planned IR-621" CG adoption; this PR does not touch those files. The committed reviews/pr614 copy predates this section and must be replaced with this file before merge.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | TC-023 now requires the AC-123-tagged control `public_family_and_corpus_results_have_direct_ir_errors_and_unchanged_successes` (present at tests/it/bounded_kani_corpus.rs:902) to stay from AC-110 to AC-111 with a Graph-request corpus case. Its graph and corpus direct-result types and graph checks stay active after the Collection variant retires. |
| FND-002 | fixed | interface-001:95 now reads "IR-621 retired public FamilyLoweringError and BoundedCorpusError, ...". This matches src/ and tests/, where none of those names occur. |
| FND-003 | fixed | FR-015-AC-123's method column now reads `Test (TC-023) and Inspection`, matching AC-109/110/111. |
