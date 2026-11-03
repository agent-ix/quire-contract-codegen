---
id: SR-2026110283853
title: "IR-1285 PR 738 routed fixture code review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; tests/it/routed_generation.rs; IR-1285 PR #738"
review_set: subset
---

## Summary

Ticket: IR-1285; PR #738 at `d910d0f958fc17addb74e3390e4bd61cf2177af2`. Reviewed the one-file Rust diff and every routed test call to `fixture()`. No introduced code or Rust defect was found.

## Scope

- examined: `Fixture`, `fixture()`, `build_fixture()`, and all 14 immutable callers in `tests/it/routed_generation.rs`.
- context_only: `tests/it/kani_obligations.rs::scalar_package`, the `application_registry` in `tests/exact_scalar_support/package.rs`, FR-022-AC-2/8/9/12, and IR-1285 AC2/AC3.
- No applicable AssuranceProfile was found under `spec/`.

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
  role: context_only
  excerpt: For a set of routed Kani items, each record and harness in the output equals what `negotiate_kani_obligations` returns for the same items in ascending request-index order with the same context. The one difference is that every index inside a record is the driver's request index, and each harness is paired with the record whose `harness_symbol` names it.
- id: FR-022-AC-8
  path: spec/routed/functional/FR-022-routed-generation.md
  role: context_only
  excerpt: Every routed item appears exactly once in `items`, in ascending request index, under its routed backend and the `KindOutput` variant of its routed kind. An item the arm refuses at lowering keeps its typed FR-015 refusal and no harness. An empty routed set returns an empty result.
- id: FR-022-AC-9
  path: spec/routed/functional/FR-022-routed-generation.md
  role: context_only
  excerpt: Regeneration from equal inputs is byte-identical, and a permutation of the routed-item slice yields an identical result. The same node routed at a different request index or under another candidate of the same backend yields a byte-identical harness when the admitted FR-330 package and selected occurrence are the same.
- id: FR-022-AC-12
  path: spec/routed/functional/FR-022-routed-generation.md
  role: context_only
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

## Evidence

`OnceLock<Fixture>` requires `Fixture: Sync`, which the focused candidate build established. The initializer creates the same admitted package, selected IDs, and scalar baseline as the former `fixture()` body. All callers borrow the returned package, IDs, or node IDs; none mutates the fixture, alters an ambient environment, or needs fresh per-test identity. The process-global `application_registry` already guarded code registration with a mutex and asserted repeated code-to-digest equality before this PR. The new `#[ix_trace_rs::string_edge]` covers the moved branch-gating comparisons. No original routed test, assertion, trace tag, source input, public API, dependency pin, or CI gate changed.

The supplied pre-change routed module ran 35/35 in 104.92 seconds; the candidate ran the same 35/35 in 56.88 seconds. `cargo fmt --all -- --check` and `git diff --check origin/main...HEAD` pass. The code lead supplied scoped Clippy and string-edge passes. This reviewer did not run a heavy build or the final aggregate gate.

## Verdict

**PASS for the scoped code and Rust review.** The final whole-suite runtime measurement remains pending; this verdict does not accept IR-1285 AC3 or close the ticket.
