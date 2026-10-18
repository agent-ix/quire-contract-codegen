---
id: SR-12641
title: "IR-876 gap analysis of CG PR 432"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen PR 432, 71182aef..3e1cbfa1; QSpec fixture preflight and its self-test"
review_set: subset
---

# IR-876 gap analysis of CG PR 432

## Summary

The preflight uses exactly the three fixture paths read by the existing direct-form integration tests. The new self-test covers each changed failure condition, and the PR changes no spec criterion, Rust trace tag, or Rust assertion.

## Verdict

**PASS for the PR diff** — no new evidence gap or unsupported code path was found.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No new gap in the reviewed diff (placeholder). | - |

## Coverage

`quire coverage --strict` on the reviewed head reports 499/855 backed rows, 221 pre-existing unbacked rows, and zero contradicted statuses. The diff changes no spec or trace-bearing Rust files, so the computed binding population is unaffected. The changed Makefile behavior is covered by the shell self-test's negative and positive path checks; fixture names agree with `tests/it/exact_function_generation.rs`, `tests/it/exact_scalar_generation.rs`, and `tests/it/composite_equality_generation.rs`. A stale explicit checkout remains outside the gate's stated presence check. Plan completion: not assessed.
