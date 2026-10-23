---
id: SR-15631
title: IR-276 CG PR539 clause graph-view gap analysis
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; IR-276; PR #539; clause graph-view delta"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-014", type: references }
---

## Summary

Planless, scoped gap analysis of the clause graph-view delta at 8ca7d984a14d08f306b86f60f2ab3ffd981dc949. FR-014-AC-38 has existing tagged integration tests of public clause oracle behavior. The new tests establish first-entry lookup and a single source-level graph access, but their FR-014-AC-38 tags are incorrect; those binding findings are recorded in companion code review SR-15630. This preparatory slice does not complete IR-276 AC-5 across K clauses.

## Verdict

PASS for the scoped implementation and existing public criterion coverage; no new uncovered behavior or stub found in the changed slice. This is not a claim that IR-276 AC-5 or the whole repository is complete.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No additional gaps found (placeholder) | - |

## Coverage

Reconciliation: `quire matrix --scope . --format tsv` with quire 0.36.2; FR-014-AC-38 computes `tagged` with ten pre-existing integration binders plus two new unit binders. The two new bindings are disputed in SR-15630; the pre-existing public tests keep criterion coverage independent of them. No run evidence was read.

Plan completion: not assessed

Reverse scope: changed `src/core/ir/clause.rs` and `src/oracle/clause.rs`, plus existing public clause tests and FR-014-AC-38. No full-repository reverse-gap or optional semantic audit was run. No applicable AssuranceProfile was found under `spec/`. The request-level `negotiate.rs`/`clause/v2.rs` caller and `evidence/v2_llvm.rs` caller still need the shared view; their changes are outside this PR.

## Examined scope and bindings

```yaml
scope:
  - id: IR-276-AC-5
    path: src/core/ir/clause.rs
    role: examined
    excerpt: "No per-node or per-obligation path in IR or CG does a linear `find` or `filter` over all graph nodes, all declarations, or the whole source map."
  - id: FR-014-AC-38
    path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
    role: examined
    excerpt: "The Boolean condition in the third argument of a `state`/`state_clause` node's `quire.op.state.clause` body, addressed by the clause node id, derives a descriptor and generates an `ir_confirmed` oracle."
findings: []
```
