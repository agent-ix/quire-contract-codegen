---
id: SR-15620
title: "Code and Rust review — IR-276 CG replay equality index"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; IR-276 AC-5, FR-033-AC-1, TC-048; src/replay/composite_builder.rs, tests/it/composite_original_eq.rs"
review_set: subset
---

## Summary

Reviewed CG PR #536 at e80dfa074e526f5a17a887e7d683bf36351b2296 against its main base. The constructor builds one first-occurrence node index; each request reads the selected admitted node through that index. The focused test exercises Eq and Ne on one retained context plus a typed mismatch.

## Verdict

**PASS** — no code or Rust defect found in the changed lines.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Rust review

The index is owned by the immutable context and stores positions into the same retained package; it cannot outlive or desynchronize from the nodes. `BTreeMap::entry(...).or_insert` preserves the earlier `iter().find` first-match behavior, and `nodes.get` keeps the access checked. The change introduces no panic, unsafe operation, unchecked conversion, external input, or wire change. No gate or source file was edited during this review.

## Scope and evidence

- Examined IR-276 AC-5: "No per-node or per-obligation path in IR or CG does a linear `find` or `filter` over all graph nodes, all declarations, or the whole source map." This PR removes the request-local full-graph `find` from this replay path; other paths remain outside this slice.
- Examined FR-033-AC-1 and TC-048's original-node Eq/Ne operator and typed `OperatorMismatch` controls. The code still calls the owning operand projection first, then reads the actual operation from the selected admitted node and refuses a mismatch before invocation.
- Examined `repeated_requests_keep_each_nodes_original_operation`: one retained context serves two distinct application nodes, asserts each returned preimage node ID, and checks Equal requested for the NotEqual node produces both typed operators. This test checks result preservation; the removal of a full-graph scan is established by code inspection because the test passed on the baseline as well.
- Plan completion: not assessed. Full aggregate and Kani gates were not run by this reviewer; the author owns the final-head gate.
