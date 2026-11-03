---
id: SR-2026110283854
title: "IR-1285 PR 738 routed fixture gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; tests/it/routed_generation.rs; spec/routed/functional/FR-022-routed-generation.md; IR-1285 PR #738"
review_set: subset
---

## Summary

Ticket: IR-1285; PR #738 at `d910d0f958fc17addb74e3390e4bd61cf2177af2`. The planless computed matrix and scoped assertion comparison found no new trace, test, or reverse-ownership gap. Plan completion: not assessed.

## Scope

- examined: all 35 routed module tests, the 14 calls to `fixture()`, and the unchanged assertions and trace tags in `tests/it/routed_generation.rs`.
- context_only: FR-022-AC-2/8/9/12 in `spec/routed/functional/FR-022-routed-generation.md`; IR-1285 AC2/AC3.

## Scope examined

```yaml
scope:
- id: tests/it/routed_generation.rs:236
  path: tests/it/routed_generation.rs
  role: examined
  excerpt: fn fixture() -> &'static Fixture {
- id: tests/it/routed_generation.rs:242
  path: tests/it/routed_generation.rs
  role: examined
  excerpt: fn build_fixture() -> Fixture {
- id: tests/exact_scalar_support/package.rs:109
  path: tests/exact_scalar_support/package.rs
  role: context_only
  excerpt: fn application_registry() -> &'static Mutex<BTreeMap<u32, String>> {
- id: FR-022-AC-2
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: For a set of routed Kani items, each record and harness in the output equals what `negotiate_kani_obligations` returns for the same items in ascending request-index order with the same context. The one difference is that every index inside a record is the driver's request index, and each harness is paired with the record whose `harness_symbol` names it.
- id: FR-022-AC-8
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: Every routed item appears exactly once in `items`, in ascending request index, under its routed backend and the `KindOutput` variant of its routed kind. An item the arm refuses at lowering keeps its typed FR-015 refusal and no harness. An empty routed set returns an empty result.
- id: FR-022-AC-9
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: Regeneration from equal inputs is byte-identical, and a permutation of the routed-item slice yields an identical result. The same node routed at a different request index or under another candidate of the same backend yields a byte-identical harness when the admitted FR-330 package and selected occurrence are the same.
- id: FR-022-AC-12
  path: spec/routed/functional/FR-022-routed-generation.md
  role: examined
  excerpt: '`RoutedGeneration.claim_map` is `Some` after a Kani group and contains one claim per selected node/occurrence admitted past FR-039 accounting, retaining any earlier typed `DuplicateRequest` refusal; an underivable node has one refused claim of provenance `underived`, ordered by node id and then selected occurrence key.'
- id: IR-1285-AC2
  path: Linear:IR-1285
  role: context_only
  excerpt: the generated-agreement child compiles share one build (one CARGO_TARGET_DIR and shared artifacts) instead of compiling per test, or an equivalent measured reduction.
- id: IR-1285-AC3
  path: Linear:IR-1285
  role: context_only
  excerpt: the full CG integration target finishes in a time budget agreed on this ticket after AC1, with no test deleted and no assertion weakened.
```

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

- `quoin matrix --repo . --json` (quoin 0.29.1) reports 1,029 criteria: 610 tagged, 314 untagged, 12 tagged only by ignored tests, 93 method-without-symbol. No run evidence is bound. Repository-wide gaps predate this one-file PR.
- FR-022-AC-2/8/9/12 remain tagged by routed test functions. The change retains the same 35 test names, all original assertions, and the corpus-derived node selections. Focused candidate execution passed 35/35.
- Reverse gap: no production behavior or public API was added. Shared fixture setup supports IR-1285 runtime reduction; AC3 still requires final whole-suite timing and acceptance.
- Plan completion: not assessed. Optional repository-wide semantic review was not selected.

## Verdict

**PASS for scoped traceability and assertion preservation.** This is not a whole-repository gap clearance or a full-suite runtime acceptance.
