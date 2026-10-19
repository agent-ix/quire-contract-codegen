---
id: "SR-13221"
title: "CG PR 455 gap analysis: FR-019-AC-48 to AC-50 coverage delta"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@cfa6dcf6bf52f886e780e8234bffa249b8bd44b9; spec/routed/functional/FR-019-capability-settlement.md, spec/routed/matrix/tests.md, spec/core/matrix/tests.md, spec/tests.md, src/oracle/function/mod.rs, src/routed/capability.rs (diff origin/main...HEAD)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-019
    type: reviews
---

# SR-13221: CG PR 455 gap analysis

## Summary

Ticket: IR-892. Planless. Plan completion: not assessed.

I compared `quire matrix --format tsv` at origin/main with the result at PR head cfa6dcf.
The strict delta is three added rows and nothing else: FR-019-AC-48, AC-49 and AC-50, each
`untagged`. No existing row changed status.

The matrix prose marks the three criteria as Planned (IR-892) with no implementation and
no tagged tests. That matches the source. `src/oracle/function/mod.rs:1745-1752` still drops
`Refused`/`Incomplete`, and `src/routed/capability.rs:867-884` still returns
`KaniFormRefused { OperationNotRendered }`. No test claims these criteria, so there is
no coverage inflation.

`make spec` exits 0. `quire validate` on the changed files reports only warnings. The diff
has no author-written review file.

## Verdict

PASS for coverage bookkeeping. The new criteria are honestly Planned. The code deviation
is the known one that IR-892 tracks, and FR-039-AC-5's Analysis still names it.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
