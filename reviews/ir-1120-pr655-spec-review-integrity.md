---
id: SR-1121033
title: "integrity review of IR-1120 PR 655 FR-045 and TC-059"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen; spec/routed/functional/FR-045-hyper-smt-induction-map.md, spec/routed/matrix/TC-059-hyper-smt-induction-map.md"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-045", type: reviews }
---

# SR-1121033: IR-1120 PR 655 integrity analysis

## Summary

Ticket: IR-1120. Checked consistency among the FR-045 narrative, FR-045-AC-4, TC-059 step 5, `spec/routed/matrix/tests.md`, and the rest of the CG spec tree for residual sorted-row wording.

## Verdict

**CHANGES REQUESTED (one high, one low).** The narrative, AC and TC agree with each other, the PR's two files add no hash, digest or pin, and the criterion ids and the `tests.md` planned status are unchanged and still true. However, two other CG artifacts still require the removed sorted order.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-015-AC-122 and AD-003 E-1 still define step rows by the sorted order. FR-015-AC-122 requires tied rows to use the FR-375-AC-19 class-spelling order "unchanged by source permutation", which contradicts FR-045-AC-4 after this PR. FR-375-AC-19 is withdrawn on QSpec main. Both are planned under IR-942, so no code is affected yet, but the repo would hold two contradictory requirements. | spec/kani/functional/FR-015-bounded-kani-obligations.md:924, spec/assurance/AD-003-evidence-chain.md:225-228 |
| FND-002 | low | The normative text names a PR number, "QSL #1864", which goes stale once merged, and the FR-045 Dependencies section does not list the row-order owner. QSpec FR-375 owns `steps` order and FR-492 is only the replay facade; the citation should name FR-375 and Dependencies should say so. | spec/routed/functional/FR-045-hyper-smt-induction-map.md:106-107, spec/routed/functional/FR-045-hyper-smt-induction-map.md:191, spec/routed/functional/FR-045-hyper-smt-induction-map.md:196-204, spec/routed/matrix/TC-059-hyper-smt-induction-map.md:52-53 |

## Examined scope

```yaml
scope:
  - {id: FR-045-AC-4, path: "spec/routed/functional/FR-045-hyper-smt-induction-map.md", role: examined, excerpt: "Permuting the authored source rows correspondingly permutes the recompiled step rows: each named row's `Step { row }` label and exact FR-315 query association follow its new position, and a record bound to the wrong row refuses handoff."}
  - {id: FR-045, path: "spec/routed/functional/FR-045-hyper-smt-induction-map.md", role: examined, excerpt: "which follows the authored declaration order under QSL FR-492 as amended by QSL #1864"}
  - {id: TC-059, path: "spec/routed/matrix/TC-059-hyper-smt-induction-map.md", role: examined, excerpt: "take row `r` from its `steps` array in authored declaration order under QSL FR-492 as amended by QSL #1864."}
  - {id: FR-015-AC-122, path: "spec/kani/functional/FR-015-bounded-kani-obligations.md", role: examined, excerpt: "Two step rows tied on node-key digest and member name use the FR-375-AC-19 alias-free class spelling order, unchanged by source permutation."}
  - {id: AD-003, path: "spec/assurance/AD-003-evidence-chain.md", role: examined, excerpt: "The index is in QSpec FR-375's total `steps` order; ties on node-key digest and member name are broken by alias-free class spelling in UTF-8 byte order (FR-375-AC-19). CG does not derive that order from source positions."}
```
