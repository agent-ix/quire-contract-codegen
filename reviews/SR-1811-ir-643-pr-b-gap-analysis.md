---
id: SR-1811
title: IR-643 PR B gap-analysis review
type: SpecReview
analysis: gap-analysis
scope: agent-ix/quire-contract-codegen@5aee240919ff981cde808ec2d9135247b7f3182a; Cargo.lock,
  src/oracle/equality/mod.rs, src/oracle/equality/resolution.rs, tests/composite_equality_support/codes.rs,
  tests/composite_equality_support/package.rs, tests/it/composite_equality_generation.rs
review_set: subset
---

## Summary

Ticket: IR-643. Independently reviewed PR #308 at the pinned head. The QSL lock has one revision and the iterative resolver closes record back-edges and refuses direct Option/Sequence cycles.

## Verdict

**CONDITIONAL** — the AC-24/25 matrix tags lack required end-to-end and boundary evidence.

## Coverage

Plan completion: not assessed. Review is scoped to the PR diff; existing repository-wide matrix gaps are outside this verdict.

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-018-AC-24 | examined | Recurse from `Option` into itself without tracking an unclosed type, or reject every repeated node including the List and Tree record back-edges; the former overflows the stack and the latter refuses valid recursive records. |
| FR-018-AC-25 | examined | Reuse the Contract IR lowering counter for reconstruction, charge only record declarations while skipping option and sequence edges, or recurse through 65,537 acyclic entries before checking the budget; a large item then completes or overflows the stack instead of yielding the named per-item refusal. |
| FR-018-AC-26 | examined | Expect `next`'s value to be a direct reference and reject QSpec's nested optional binding, or ignore the wrapper and treat the field as required; the List item refuses or generates with the wrong presence. |
| FR-018-AC-27 | examined | Expect an inline second bounds reference in the Sequence body, or reject every bounded domain whose semantic type is not scalar; the QSpec Tree item refuses. Ignoring a missing bound instead generates an unbounded collection. |
| TC-029-step-16 | examined | Read the authoritative QSpec `proposals/checked-package-v2/fixtures/positive-recursive-records.json` through Contract IR's strict reader, without copying it into this repository. Build equality expression items over the fixture's List (`Option<List>`) and Tree (`Sequence<Tree>[0,3]`) record declarations in a valid test package; the fixture itself supplies type nodes, not equality expressions. Assert both items generate and the call terminates. |
| TC-029-step-17 | examined | At that same seam, use an acyclic walk whose roots and references across both operands and every conversion target total exactly 65,537 entries, with a valid terminating leaf and a healthy sibling. Assert the first item refuses `TypeResolutionWorkExhausted { limit: 65_536, consumed: 65_537 }`, emits no symbol and leaves the sibling generated; a separate walk totaling exactly 65,536 per-item entries across every operand and target does not receive the work refusal. |
| src/oracle/equality/resolution.rs | examined | The frame loop charges Enter, resolves nested Option/Collection values, and charges an inline collection bound in FinishCollection. |
| src/oracle/equality/mod.rs | examined | check_item resolves four descriptor roots, clones ValueType operands, and render_value_type recursively renders Option/Collection. |
| tests/it/composite_equality_generation.rs | examined | The AC-24 public generation test uses corpus_package and recursive_tree_package, both hand-built. |
| tests/composite_equality_support/package.rs | examined | The new recursive_tree_package builder hand-constructs Tree, Sequence, collection bounds, and equality expression. |
| tests/composite_equality_support/codes.rs | examined | Adds Tree-specific fixture codes. |
| Cargo.lock | examined | Eight QSL crates move together to merged revision 840c36d9a94ecff00700959d23773e8e78c47888. |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The tagged public item test generates from hand-built corpus_package and recursive_tree_package, while the QSL-compiled package test only calls resolve_shape. No equality item is generated over the QSL-produced List/Tree package or the cited authoritative QSpec fixture, so the required end-to-end reader-to-oracle path and exact fixture encoding are unverified. | tests/it/composite_equality_generation.rs:114-146 |
| FND-002 | medium | The tagged 65,537-entry test checks one resolver root and a separately refused synthetic item; its exactly-65,536 case calls only resolve_type. It never asserts item generation at the limit, combined charges across left/right and conversion targets, a healthy sibling in the same call, or no stack abort in a subprocess as TC-029 step 17 requires. The green matrix tag therefore overstates AC-25 coverage. | src/oracle/equality/mod.rs:2073-2163 |
