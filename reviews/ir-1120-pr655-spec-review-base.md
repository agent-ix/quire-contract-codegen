---
id: SR-1121031
title: "base review of IR-1120 PR 655 FR-045-AC-4 and TC-059"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; spec/routed/functional/FR-045-hyper-smt-induction-map.md, spec/routed/matrix/TC-059-hyper-smt-induction-map.md; FR-045-AC-4"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-045", type: reviews }
  - { target: "ix://agent-ix/quire-contract-codegen/TC-059", type: reviews }
---

# SR-1121031: IR-1120 PR 655 base spec review

## Summary

Ticket: IR-1120. PR: quire-contract-codegen#655 at f40b1bd88889cc2833a708b1df1cf40c5d314e65, reviewed against origin/main d91bf1bd and QSL #1864 head 9e7c1abe. Review set is subset: base, integrity, dependency and ears-conformance, chosen by the reviewer dispatch rules because the edit changes requirement text and adds a relationship edge. No AssuranceProfile exists under `spec/`, so no enforced run applies. `quire validate --scope . "spec/**/*.md"` and `quoin validate --repo .` report no FR-045 or TC-059 finding.

## Verdict

**CHANGES REQUESTED (one medium).** FR-045-AC-4, the narrative and TC-059 step 5 correctly replace canonical order with declaration order. Seven rows, zero rows, exact query bytes and Base/Step role protections are all retained. FR-045-AC-4, TC-059 and step 5 keep their ids and numbers, and the edit is in place with its git history. The new label exchange reaches association refusal, because labels stay unique and `query != expected.query_bytes` fires in `validate_records`; the old single relabel would have produced a duplicate and a shape refusal. One test oracle needs restoring.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-059 step 5 removed the assertion that a named row's FR-315 query bytes stay equal across the two compilations, and replaced it with "follow its new position", which names no observable. Both expected bytes and label come from the same recompile, so a position mismatch between `Step { row }` and the `steps` array passes. | spec/routed/matrix/TC-059-hyper-smt-induction-map.md:58-63 |

## Coverage

The step 5 oracle should assert that each named row's `Step { row }` bytes from the permuted compile equal that named row's bytes from the first compile, with the row now at its new position. QSL FR-315 at the #1864 head says the row "plays no part in any assertion order", and FR-492-AC-3 on QSL main states the same query invariance. #1864 retires that AC and deletes its test `reordering_the_written_rows_changes_the_mapping_not_the_query`, so no remaining test in either repo would catch the mismatch. Confidence is medium because invariance was read from spec text, not measured.

## Examined scope

```yaml
scope:
  - {id: FR-045-AC-4, path: "spec/routed/functional/FR-045-hyper-smt-induction-map.md", role: examined, excerpt: "For a refinement with seven step rows, CG retains one `Base` and exactly seven typed `Step { row }` records numbered 0 through 6 from the recompiled `steps` array in authored declaration order (QSL FR-492 as amended by QSL #1864), even when run records arrive out of order."}
  - {id: TC-059, path: "spec/routed/matrix/TC-059-hyper-smt-induction-map.md", role: examined, excerpt: "Assert that the recompiled `steps` array follows the new declaration order and that each named row's `Step { row }` label and exact FR-315 query association follow its new position"}
  - {id: FR-045, path: "spec/routed/functional/FR-045-hyper-smt-induction-map.md", role: examined, excerpt: "Permuting the authored rows correspondingly permutes the `steps` array and each named row's `Step { row: r }` association."}
  - {id: FR-045-AC-5, path: "spec/routed/functional/FR-045-hyper-smt-induction-map.md", role: context_only, excerpt: "CG emits one deterministic handoff sequence in `Base`, `Step { row: 0 }` through `Step { row: 6 }` order for either input permutation."}
  - {id: FR-045-AC-6, path: "spec/routed/functional/FR-045-hyper-smt-induction-map.md", role: context_only, excerpt: "swapping row 3's record with row 4's while retaining the old row labels, yields a typed association refusal and no handoff."}
```

## Dispositions

Round 1, head a9ba1d012345b250793b0b66678c04d4be184157.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | At a9ba1d01: For each named row, independently compare the FR-315 query bytes obtained before and after permutation and assert equality, then compare the bytes sent on stdin and retained in that row's newly labelled record with those same bytes. |
