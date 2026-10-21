---
id: SR-15221
title: Gap analysis — IR-615 profile consumer
type: SpecReview
analysis: gap-analysis
scope: 'agent-ix/quire-contract-codegen; PR #485 IR-615; Cargo.lock, spec/kani/functional/FR-015-bounded-kani-obligations.md,
  spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md, src/kani/generate/lower/bounded_collections.rs,
  src/kani/generate/lower/bounded_kani_profile.rs, src/kani/generate/lower/finite_reference_graphs.rs,
  src/kani/generate/lower/mod.rs, reviews/ir-615-consumer-code-review.md, reviews/ir-615-consumer-gap-analysis.md,
  reviews/ir-615-consumer-spec-review-base.md, reviews/ir-615-current-code-review.md, reviews/ir-615-current-gap-analysis.md,
  reviews/ir-615-current-spec-review-base.md'
review_set: subset
relationships:
- target: ix://agent-ix/quire-contract-codegen/FR-015
  type: references
---

## Summary

Ticket IR-615, CG PR #485. Non-author review of merge-base `8ac09bc85f2a34b7b91d36bd59ce8e92e6ced59d` through exact head `7d2c2b20dbdd677b31f2639e1e63f62e41784485`. This review examined the changed production, tests, spec, lock and review custody files.

## Verdict

FAIL for repository-wide traceability: 309 inherited gaps remain on the committed SR-14861 inventory; no gap introduced by this PR.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | 309 inherited repository traceability gaps remain as individually recorded in SR-14861; none is introduced by PR #485. | reviews/ir-615-current-gap-analysis.md |

## Coverage

The computed matrix at this head binds FR-015-AC-82 with four tests, AC-83 with two, AC-86 with two, and AC-88 with two. These tests assert real production results and typed refusals; the graph and collection regressions distinguish the prior reviewer mutations. The whole-repository debt in SR-14861 is unchanged by this narrow PR and has explicit deferred dispositions to IR-878. Plan completion: not assessed.

The author reports final-head `make ci` PASS and real Kani 37/37 PASS on IR-615; this review did not rerun the heavy gate. An independent reviewer compile on the exact PR head passed `cargo check --locked --all-targets` against committed IR/model d41e521f and also passed after selective IR/model update to b488dbc0 in a separate scratch worktree. The PR tree was not edited for either check.

## Examined Scope

```yaml
scope:
- id: FR-015-AC-82
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until FR-015-AC-111 deletes the bounded-profile classifier, a request for distinct named constructs
    returns one `CapabilityEntry` per requested construct in request order, preserving each admitted `Supported`
    module or `Refused` cause code exactly. The Contract IR profile reader rejects the removed `inconclusive`
    disposition rather than admitting it; no profile disposition or profile-supplied code becomes an `Inconclusive`
    run outcome or asserts proof vacuity.
- id: FR-015-AC-83
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until FR-015-AC-111 deletes the bounded-profile classifier, an empty, duplicate or absent construct
    is a typed `KaniOutcome` refusal rather than a partial disposition census.
- id: FR-015-AC-86
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the graph slice (FR-015-AC-111) deletes the finite-graph lowerer, a profile/dispatch-admitted
    request follows only the selected reference field, finds a positive-length path in sorted depth-first
    order, and checks the expansion bound before entering each new object; a target edge is found without
    expanding its target.
- id: FR-015-AC-88
  path: spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: examined
  excerpt: Until the collection slice (FR-015-AC-110) deletes the finite-collection lowerer, a profile/dispatch-admitted
    `ForAllNonNegative` or `ExistsEqual` query retains input order and duplicates and returns its truth
    value and examined-item count; a decisive item ends that count, otherwise every item counts.
- id: NFR-006-AC-13
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: context_only
  excerpt: For a Kani-touching pull request, the author runs the full gate on its clean, stable candidate
    before opening it, and the merger runs it again on the clean, stable current candidate before merge;
    both actual processes exit successfully with all expected tests run. A copied body line alone cannot
    satisfy either run.
- id: NFR-006-AC-15
  path: spec/kani/non-functional/NFR-006-real-kani-lane-gating.md
  role: context_only
  excerpt: A Kani-touching pull request whose required lane did not run or failed remains unmerged and
    does not claim real-Kani verification; a written explanation does not waive the gate.
bindings:
- test_id: src/kani/generate/lower/bounded_kani_profile.rs::tests::tc_023_census_reports_every_construct_including_ones_after_an_early_refusal
  ac_id: FR-015-AC-82
  trace: correct
- test_id: src/kani/generate/lower/bounded_kani_profile.rs::tests::empty_or_duplicate_construct_name_refuses_without_a_partial_census
  ac_id: FR-015-AC-83
  trace: correct
- test_id: src/kani/generate/lower/finite_reference_graphs.rs::tests::branching_graph_uses_a_stable_bounded_discovery_order
  ac_id: FR-015-AC-86
  trace: correct
- test_id: src/kani/generate/lower/bounded_collections.rs::tests::both_query_kinds_preserve_order_duplicates_and_exact_decision_counts
  ac_id: FR-015-AC-88
  trace: correct
```
