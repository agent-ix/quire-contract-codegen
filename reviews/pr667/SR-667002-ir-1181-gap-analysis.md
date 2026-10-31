---
id: SR-667002
title: "CG #667 changed-criterion gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; PR #667 8cc398a5; changed Kani and replay source and tests; FR-017-AC-13/61, FR-024-AC-52/53, FR-033-AC-19/20"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-017", type: references }
  - { target: "ix://agent-ix/quire-contract-codegen/FR-024", type: references }
  - { target: "ix://agent-ix/quire-contract-codegen/FR-033", type: references }
---

## Summary

Read the computed `quire matrix --scope . --format json` at PR #667 head 8cc398a5 and inspected changed tests and production source for the changed criteria. No plan was supplied.

## Verdict

**CONDITIONAL** for this diff: the replacement collection proof criterion is untagged while six tests still cite its retired predecessor. The other changed requirements have binders or remain explicitly Planned without implementation credit. This is a scoped PR review, not a repository-wide PASS claim.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-017-AC-61 has zero binders in the computed matrix despite live collection proof tests; retired FR-017-AC-13 still has six. Rebind actual AC-61 tests and remove retired trace tags. | spec/kani/functional/FR-017-kani-execution-evidence.md:843; src/kani/run/execute.rs:1352; src/kani/classify.rs:418; tests/it/kani_obligations_symbolic_domains.rs:2302 |

## Coverage

- FR-017-AC-13: method-without-symbol; six binders, retired.
- FR-017-AC-61: untagged; zero binders.
- FR-024-AC-52: tagged; four binders.
- FR-024-AC-53: tagged; three binders.
- FR-033-AC-19/20: untagged; explicitly Planned, no false credit claimed.
- Reverse ownership: changed Kani collection and replay code remains under FR-017, FR-024 and FR-033 respectively. No new stub or vendored file was observed.
- Plan completion: not assessed.

## Dispositions

Disposition pass 1 at head 0ba810c587f7d850603c421a4ff075032f0cae29. The trace-only fix changes six binders and no product code.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Six live collection controls now tag FR-017-AC-61, and `git grep` finds no FR-017-AC-13 in source or tests. |
