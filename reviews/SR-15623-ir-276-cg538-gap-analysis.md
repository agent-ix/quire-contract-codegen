---
id: SR-15623
title: IR-276 CG PR538 scoped gap analysis
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen; IR-276; CG #538 at 80cfc2c1d5434e4bbcec704e9b6e89537f00ea98; src/kani/generate/collection.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: references
---

## Summary

Scoped chain check of collection indexing intent, new test, and changed source on CG #538. The PR explicitly supplies only the reusable internal lookup structure; the request caller remains a separate planned slice.

## Verdict

PASS for the preparatory scope. This does not qualify request-level IR-276 AC-5; the remaining K×R construction is tracked for the caller PR. FND-001 in SR-15622 covers the inadequate counted lookup assertion.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Plan completion: not assessed. Examined the changed `collection.rs` source and test, existing collection negotiation caller, FR-015-AC-96/97 as context, and IR-276 AC-5. The existing integration test reaches supported and refused collection items. The new unit test checks first entry and missing node behavior. It does not yet prove request-level linear work; the PR body declares that limitation and the separate caller slice is assigned. No production behavior outside the changed collection path was claimed as reviewed. No applicable AssuranceProfile was found under `spec/`.

## Examined scope

```yaml
scope:
  - id: FR-015-AC-96
    path: spec/kani/functional/FR-015-bounded-kani-obligations.md
    role: context_only
    excerpt: "An admitted QSL V2 clause/function body that contains a quire.op.collection.forall or quire.op.collection.exists operation over a selected model collection field is a collection claim."
  - id: FR-015-AC-97
    path: spec/kani/functional/FR-015-bounded-kani-obligations.md
    role: context_only
    excerpt: "An admitted collection claim with no finite upper bound or finite element domain settles requires_bound without a harness; unsupported forms settle unsupported."
  - id: IR-276-AC-5
    path: src/kani/generate/collection.rs
    role: examined
    excerpt: "No per-node or per-obligation path in IR or CG does a linear find or filter over all graph nodes, all declarations, or the whole source map."
```
