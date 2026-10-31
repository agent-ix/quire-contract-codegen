---
id: SR-2026103023
title: "IR-1146 PR 665 source-reference fixture code review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; tests/exact_function_support/package.rs, tests/it/exact_function_generation.rs; PR 665 at c5280c21"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-021
    type: references
---

## Summary

Ticket IR-1146, PR 665, head c5280c21b9f5101eb787153cb64c3efb6c1da46b. Reviewed only the two-file diff against fresh origin/main. The locked Contract IR revision 8320ebe7 defines `CheckedSourceRef` as the closed `{authority, identity}` shape with `deny_unknown_fields`. No applicable AssuranceProfile was found. The separate emitted NodeKey work remains outside this PR.

## Verdict

PASS. The fixture now uses the pinned IR source-reference shape; the source-map association, each span, and each authored source slice are still checked. The byte-ceiling test still exercises admitted lowering and typed `Failed(Bytes)` records. No production, spec, gate, threshold, or trace-tag file changed. `cargo fmt --check` and `git diff --check origin/main...HEAD` passed. The focused Cargo test was run by the dispatching lead and is not claimed here.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Examined the source-reference construction and source-map regions in `tests/exact_function_support/package.rs`, and the `FR-021-AC-23`/`TC-031` small-chain test in `tests/it/exact_function_generation.rs`. Compared the current test assertions with the FR-021-AC-23 text and TC-031 step 10. A change that misbinds the source reference, offsets, or source text still fails the remaining assertions. The removed digest assertion checked a field the pinned IR reader no longer admits. No new copy or compatibility path was introduced.

## Examined Scope

```yaml
scope:
  - id: FR-021-AC-23
    path: spec/oracle/functional/FR-021-function-application-oracles.md
    role: examined
    excerpt: "For an admitted FR-330 package carrying a `Failed` feature-accounting record, a function item retains that exact typed record, its `limit_kind`, `limit`, `consumed` and source occurrences in `PackageAccounting`, without relabelling bytes as work exhaustion."
  - id: TC-031-step-10
    path: spec/oracle/matrix/TC-031-function-application-oracles.md
    role: examined
    excerpt: "Retain the `Failed`-record seam for work, bytes and unrecognised lowering limit kinds; assert the mapping of FR-021-AC-23, body/call-node priority, per-function isolation and no panic."
  - id: tests/exact_function_support/package.rs
    path: tests/exact_function_support/package.rs
    role: examined
    excerpt: 'let source_ref = serde_json::json!({ "authority": "agent-ix", "identity": "ir764-ac23-source", });'
  - id: tests/it/exact_function_generation.rs
    path: tests/it/exact_function_generation.rs
    role: examined
    excerpt: 'assert_eq!(entry["regions"][0]["source"], *source_ref); assert_eq!(entry["regions"][0]["start"], *start); assert_eq!(entry["regions"][0]["end"], *end); assert_eq!(&source[*start..*end], format!("n{code}"));'
```

Plan completion: not assessed.
