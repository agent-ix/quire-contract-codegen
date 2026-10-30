---
id: SR-1121030
title: "code-review of IR-1120 PR 655 spec diff"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; spec/routed/functional/FR-045-hyper-smt-induction-map.md, spec/routed/matrix/TC-059-hyper-smt-induction-map.md"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/FR-045", type: reviews }
---

# SR-1121030: IR-1120 PR 655 code review

## Summary

Ticket: IR-1120. The diff is two Markdown files with no code, test, workflow or copied upstream file, so the Rust lane does not apply. The review checked the diff against the code it describes, `src/routed/smt/refinement.rs`. Per the spec-only cadence, no Cargo test or build ran.

## Verdict

**PASS WITH ONE LOW.** `validate_records` compares against caller-supplied expected entries and never sorts rows, so the new declaration-order text needs no behavior change there. The stale wording is documentation only.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Rustdoc in the code this requirement governs still says canonical. It describes the row as a position in the "canonical steps array" and says expected entries are "indexed by canonical step position", which contradicts FR-045 after this PR. The PR is spec-only, so this belongs in the next IR-1120 code slice. | src/routed/smt/refinement.rs:10, src/routed/smt/refinement.rs:15-17, src/routed/smt/refinement.rs:29, src/routed/smt/refinement.rs:44, src/routed/smt/refinement.rs:105 |

## Examined scope

```yaml
scope:
  - {id: FR-045-AC-4, path: "spec/routed/functional/FR-045-hyper-smt-induction-map.md", role: examined, excerpt: "a record bound to the wrong row refuses handoff"}
  - {id: src/routed/smt/refinement.rs, path: "src/routed/smt/refinement.rs", role: examined, excerpt: "/// A query's position in the recompiled refinement's canonical steps array."}
```
