---
id: SR-12923
title: Specification integrity review of scalar whitespace correction
type: SpecReview
analysis: integrity
scope: agent-ix/quire-contract-codegen; spec/assurance/AD-004-cg-crate-layout.md (H-2
  whitespace, R-1.4, P-2); src/kani/generate/scalar.rs:390-443 (read-only context)
review_set: subset
relationships:
- target: ix://agent-ix/quire-contract-codegen/AD-004
  type: references
---

## Summary

Ticket: IR-858. PR: quire-contract-codegen#438.

The correction removes the indentation contradiction without weakening byte identity or changing the ExprText admission contract.

## Scope

```yaml
scope:
- id: AD-004-H-2
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: 'An `ExprText` may span lines;

    `render.rs` preserves its text and indentation verbatim, including lines after
    the first.'
- id: AD-004-R-1.4
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: "The scalar harness wrapper and body preserve the pre-port template's actual\
    \ whitespace: eight\n   spaces before each `Input` declaration and no renderer-added\
    \ indentation on other lines. Rust\n   string continuations in that template strip\
    \ its source indentation."
- id: AD-004-P-2
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: 'For every claim `render_scalar` accepts today, the

    rendered `Artifact` (path and source bytes), the `ScalarObligationIdentity`, the
    `options`, the

    `ObligationRecord` and every digest derived from them equal what the pre-port
    renderer computes.

    The port introduces no explicitly-specified difference.'
- id: AD-004-H-2-admission
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: context_only
  excerpt: '`ExprText` is one Rust expression as text, for the native `i128` expression,
    the outcome `match`

    and the `matches!` of the cover: the family''s own meaning, not harness structure.'
- id: AD-004-P-3.1
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: context_only
  excerpt: "A differential test. The code PR keeps the body of today's `render_scalar`\
    \ as a `#[cfg(test)]`\n   reference function, test-only, and compares the ported\
    \ path against it, byte for byte, over a\n   fixed claim set:"
```

Source context examined: src/kani/generate/scalar.rs:390-393 (declarations) and :412-443 (wrapper/body). The review covers the two changed paragraphs and their unchanged compatibility contract.

## Analysis

Consistency: verbatim ExprText insertion and the declaration/wrapper whitespace description agree with the old scalar template and unchanged P-2. Atomicity/testability: preservation is observable by exact byte comparison; the unchanged P-3 differential reference supplies the implementation verification obligation. Completeness: no US, FR, NFR, StR, relationship, or verification edge changes, so the traceability slice remains AD-004 H-2/R-1/P-2 -> P-3 -> TC-025 steps 42-47, with no new matrix row. Hidden-assumption/failure-domain probes add no scope: no new external tool, extension point, identity key, callback policy, side effect, graph traversal or topological algorithm is introduced. This review does not reopen unchanged baseline requirements, authorize held steps 4c-4e/4f/5/6, or assert that the future port has passed runtime evidence.

## Verdict

**PASS** — no defects found in the scoped correction. No production or proof-completion claim is made.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
