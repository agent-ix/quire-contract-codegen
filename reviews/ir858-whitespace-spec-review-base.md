---
id: SR-12921
title: Base specification review of scalar whitespace correction
type: SpecReview
analysis: base
scope: agent-ix/quire-contract-codegen; spec/assurance/AD-004-cg-crate-layout.md (H-2
  whitespace, R-1.4, P-2); src/kani/generate/scalar.rs:390-443 (read-only context)
review_set: subset
relationships:
- target: ix://agent-ix/quire-contract-codegen/AD-004
  type: references
---

## Summary

Ticket: IR-858. PR: quire-contract-codegen#438.

The revised wording is precise, consistent with P-2, and limited to the existing architecture contract.

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

Checked the changed statements for unique interpretation, terminology, verifiability, duplicate ownership, references and unintended scope changes. The existing AD-004 ID, architecture type/status, relationships and headings remain identical. P-2 compares exactly with base: every accepted claim retains artifact path/source bytes, identity, options, record and derived identity values without an explicit exception. The text describes current whitespace rather than allowing a normalization. No new FR/AC, option, state transition, refusal, tracking record, or test obligation is introduced. The six coverage rules gain no new unit from this prose correction; pending implementation evidence remains P-3. No applicable AssuranceProfile or mandatory review selection exists in spec/. The leader explicitly selected this subset.

## Verdict

**PASS** — no defects found in the scoped correction. No production or proof-completion claim is made.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
