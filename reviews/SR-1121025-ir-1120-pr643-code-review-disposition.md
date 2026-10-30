---
id: SR-1121025
title: "IR-1120 CG #643 code-review disposition"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; PR #643 head f8b6b0573c71cb0476a67253c00c1cb3a34005e6; code-review disposition of SR-1121021 and the fixes since 3f40b53ab057c52d72fdf49ca29faf2bcef89572"
review_set: subset
---

## Summary

Disposition of SR-1121021 at head f8b6b0573c71cb0476a67253c00c1cb3a34005e6. Both original findings are fixed and no regression was found. The seven operations now encode distinct Step queries, and the test asserts that distinctness and all seven reversed source rows.

## Verdict

PASS — the row test can now catch a misbinding. This verdict covers row binding only: AC5 stays PARTIAL and nothing here claims Verified.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Placeholder: no open code-review finding at this head. | - |

## Coverage

Original finding outcomes at this head:

| Original | Outcome | Evidence |
| --- | --- | --- |
| SR-1121021 FND-001 | fixed | `valid_solver_tail` accepts a whitespace-delimited `success` (src/routed/smt/mod.rs:326-330). `captured_alethe_proof` still refuses a tail that does not start with `(` (src/routed/smt/mod.rs:275-277), so no proof comes from `unsat\nsuccess\n`. |
| SR-1121021 FND-002 | fixed | See the three points below. |

Why SR-1121021 FND-002 is closed:

- Distinct encoding: operation `opN` now has `modifies_receiver` set to its own `flagN` (src/routed/smt/refinement_source_tests.rs:58-71). QSL adds an unchanged conjunct for every field the frame does not let the operation write (`qsl-eval/src/smt/relation.rs:501,552-561,617-633`), and each Step script is built from the selected row's operation (`qsl-eval/src/smt/refinement.rs:157-166,259-275`). So each operation leaves a different flag unconstrained.
- New assertions: rows 0-6 are compared pairwise for query bytes (src/routed/smt/refinement_source_tests.rs:308-318), and `reordered_sources[row] == 6 - first_sources[row]` is asserted for all seven rows (src/routed/smt/refinement_source_tests.rs:304-306).
- Detection: binding `Step { row: sources[row] }` in the helper would give `Q(s[row])` in the first compile and `Q(6 - s[row])` in the second. Those differ for every row except source position 3, so the equality at line 323 fails. This is a static derivation; no mutation was run.

Regression check at this head: the `Shape` relabel, query-byte mutation and base/step exchange controls are intact (src/routed/smt/refinement_source_tests.rs:341-375). The pairwise loop skips index 0 (`Base`), which is correct because the requirement concerns Step rows and the exchange control covers `Base`. The proof-capture and status-tail code named above was read at this head and shows no regression. No `git diff` of 3f40b53..f8b6b05 was available, because this review was read-only, so the delta was judged by reading the head files against `fix-diff.patch`.

Evidence: `cg643-ac4-review-fix.log` shows `source_row_permutation_keeps_canonical_refinement_query_bindings ... ok` against QSL `93fcdc17`, so the pairwise assertions passed at run time. The log records no head SHA; file ordering shows it is newer than the test source. This reviewer ran no Cargo, Kani, spec validation or `git` command. The `/code-review` skill was not loadable here, so its method was applied from the brief. Plan completion: not assessed.

## Examined scope and bindings

```yaml
scope:
- id: FR-045-AC-4
  path: spec/routed/functional/FR-045-hyper-smt-induction-map.md
  role: examined
  excerpt: For a refinement with seven step rows, CG retains one `Base` and exactly
    seven typed `Step { row }` records numbered 0 through 6 from the recompiled, canonically
    ordered `steps` array, even when records arrive out of order or the source rows
    were written in another order. Each record binds the refinement item, subjects,
    distinct invocation attempt, role and row, and exact FR-315 `Initial` or `Step
    { row }` query bytes sent on stdin; changing a query byte or exchanging the base
    and a step role refuses the handoff. With zero step rows, the complete shape has
    only `Base`.
bindings: []
```

## Dispositions

| Finding | Outcome | Evidence |
| --- | --- | --- |
| FND-001 | accepted-no-change | Placeholder with nothing to fix. Original outcomes are listed under Coverage. |
