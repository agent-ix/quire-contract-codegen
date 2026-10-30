---
id: SR-1121034
title: "dependency review of IR-1120 PR 655 QSL and QSpec row-order contract"
type: SpecReview
analysis: dependency
scope: "agent-ix/quire-contract-codegen; spec/routed/functional/FR-045-hyper-smt-induction-map.md; QSL FR-492, FR-315, FR-314 at #1864 head 9e7c1abe; QSpec FR-375 on main"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-045", type: reviews }
---

# SR-1121034: IR-1120 PR 655 dependency analysis

## Summary

Ticket: IR-1120. Examined the new `references` edge to QSL FR-492 and the upstream contract the amended text relies on. Measured upstream state directly with `gh`: QSL #1864 is OPEN at 9e7c1abe and blocked on QSL-1282 by its own ticket comment; QSpec FR-375 on main already states declaration order; IR #704 is merged; CG's locked IR c3dc73ec is 15 commits behind merge commit 77cbbd1f.

## Verdict

**CHANGES REQUESTED (two medium).** The direction agrees with upstream: QSpec FR-375 and FR-492 at the #1864 head both say declaration order. CG #655 must not land before #1864, and QSL's FR-315 needs one sentence corrected. The CG lock lag is expected for a spec-only PR and belongs to the implementing slice, so it is not a finding.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-045 and TC-059 state as settled a contract from an unmerged PR. FR-492 on QSL main still defines `refinement_step_sources` and a mapping from `steps` positions to written rows, so if #655 merges first, its "FR-492 as amended" citation describes text that does not exist. | spec/routed/functional/FR-045-hyper-smt-induction-map.md:105-109, spec/routed/matrix/TC-059-hyper-smt-induction-map.md:51-54 |
| FND-002 | medium | QSL #1864 leaves FR-315 saying "The row index in `Step { row }` comes from FR-375's `steps` order, which is sorted by node key", which contradicts FR-492 at the same head and QSpec FR-375. FR-045 depends_on FR-315 for `Step { row }`, so the contract CG follows disagrees with itself. The fix belongs in QSL, not CG. | QSL spec/functional/FR-315-encode-the-smt-lib-transition-relation.md:896 at 9e7c1abe; spec/routed/functional/FR-045-hyper-smt-induction-map.md:103-107 |

## Coverage

Clean: the edge target FR-492 exists, the `references` verb validates, and CG has no use of the removed `refinement_step_sources`. FR-314 at the #1864 head defers to "QSpec FR-375" for `steps`, which now means declaration order, so it needs no edit. Remedy for FND-001 is to hold #655 until #1864 merges, or to cite QSpec FR-375, which has already landed. FND-002 should be raised with the QSL lane on QSL-1265.

## Examined scope

```yaml
scope:
  - {id: FR-045, path: "spec/routed/functional/FR-045-hyper-smt-induction-map.md", role: examined, excerpt: "where `r` is the zero-based position in the recompiled refinement's `steps` array, which follows the authored declaration order under QSL FR-492 as amended by QSL #1864, for every `r` in `0..N`."}
  - {id: QSL-FR-492, path: "quire-spec-language spec/functional/FR-492-expose-the-refinement-simulation-query-through-the-replay-facade.md", role: examined, excerpt: "The refinement node's `steps` array holds the rows in declaration order, so a query's `row` is the row's position among the rows as written."}
  - {id: QSL-FR-315, path: "quire-spec-language spec/functional/FR-315-encode-the-smt-lib-transition-relation.md", role: examined, excerpt: "The row index in `Step { row }` comes from FR-375's `steps` order, which is sorted by node key; it selects the row and plays no part in any assertion order."}
  - {id: QSpec-FR-375, path: "quire-specification spec/functional/analysis/FR-375-declare-model-refinement.md", role: examined, excerpt: "The `steps` array is in **declaration order**: the order in which the declaration lists its step rows, for a model subject and a protocol subject alike."}
  - {id: QSL-FR-314, path: "quire-spec-language spec/functional/FR-314-check-an-smt-proof-certificate.md", role: context_only, excerpt: "number of step rows of the recompiled refinement's `steps` array."}
```

## Dispositions

Round 1, head a9ba1d012345b250793b0b66678c04d4be184157.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | At a9ba1d01: which follows the authored declaration order under QSpec FR-375-AC-22, for every `r` in `0..N`. |
| FND-002 | deferred | QSL-owned. QSL #1864 at 634d6f0d corrects FR-315, but it is unmerged, so QSL main still says sorted by node key until QSL-1265 lands. It does not block #655: FR-045 now cites merged QSpec FR-375-AC-22, and FR-315 defines row as the position in the steps array under either wording. |
