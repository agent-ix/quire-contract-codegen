---
id: SR-585009
title: IR-348 CG589 exact scalar code and Rust review
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen; IR-348; PR589; af15747de1c9ffe1cdd9b5d7c8c64b7fd02f142c; src/core/ir/scalar.rs, src/oracle/scalar/derive.rs, src/oracle/scalar/lower.rs"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-contract-codegen/AD-004", type: references }
---

## Summary

Ticket: IR-348. Independent Opus 5.5 review of CG #589 at original head `af15747d`. Rust review is the lane of code review. The new scalar access layer preserves the old identity, mode, law and quantity-conversion decisions by source inspection. No cross-repository copy, compatibility forwarding layer, unsafe operation or panic was introduced.

## Verdict

CONDITIONAL at the original head: one medium duplicate decoder finding and two low boundary/guard findings. The review did not run gates; the code lead owns the final gate.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | New scalar::NodeShape redeclares frame::NodeShape, and scalar::identity duplicates frame::application_identity. The identity read could diverge when the typed operator source changes. | src/core/ir/scalar.rs:11-21,31-33; src/core/ir/frame.rs:12-16,91-96 |
| FND-002 | low | scalar::shape wraps only two node_tag/semantic_form reads while adjacent reads still use the fields directly; it enforces no scalar payload boundary. | src/core/ir/scalar.rs:16-21; src/oracle/scalar/derive.rs:421; src/oracle/scalar/lower.rs:135-138 |
| FND-003 | low | Scoped L-8 guard misses CheckedTerm::Application { operation, .. }, arbitrary operation["member"] indexing and serde_json::Value body reads; the old conversion body would pass. | src/core/ir/scalar.rs:68-82; spec/assurance/AD-004-cg-crate-layout.md:946 |

## Coverage

Examined AD-004 L-8 and step 3, the five changed Rust files, the checked term reader, and FR-014 scalar oracle test bindings. Source comparison found the same refusal triggers and payloads, and no render path changed. The original reviewer ran a read-only `quire matrix --scope . --format json`; no fmt, Clippy, Cargo test, Kani or aggregate gate was run. Plan completion: not assessed.

## Examined scope and bindings

```yaml
clean: false
scope:
- id: AD-004-L-8
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: 'L-8. `serde_json::Value` and `.get("` appear for body terms only in `core/ir`.
    Test: a grep gate.'
- id: AD-004-step-3
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: this step introduces no old-path forwarding module or other compatibility
    layer. L-8 lands with the last typed-node-access PR.
- id: src/core/ir/scalar.rs
  path: src/core/ir/scalar.rs
  role: examined
  excerpt: "/// The admitted node's scalar classification fields.\npub(crate) struct\
    \ NodeShape<'a> {\n    pub(crate) tag: &'a str,\n    pub(crate) form: &'a str,\n\
    }\n\npub(crate) fn shape<Body>(node: &CheckedSemanticNodeV2<Body>) -> NodeShape<'_>\
    \ {\n    NodeShape {\n        tag: &node.node_tag,\n        form: &node.semantic_form,\n\
    \    }\n}"
- id: src/core/ir/frame.rs
  path: src/core/ir/frame.rs
  role: examined
  excerpt: 'pub(crate) fn application_identity(term: &CheckedTerm) -> Option<&str>'
- id: src/oracle/scalar/derive.rs
  path: src/oracle/scalar/derive.rs
  role: examined
  excerpt: scalar_ir::shape(node).tag
- id: src/oracle/scalar/lower.rs
  path: src/oracle/scalar/lower.rs
  role: examined
  excerpt: scalar_ir::shape(&node.node).form
- id: src/oracle/checked_graph.rs
  path: src/oracle/checked_graph.rs
  role: examined
  excerpt: Removed the unused CheckedGraph::operation payload accessor.
bindings: []
```

## Dispositions

### Round 1 — exact `15cfb7a636ed5f72a0804a8c63954445e94ad082`

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Verified against `af15747d...15cfb7a6` by independent Opus 5.5; exact after excerpt is in the paired Linear disposition comment. |
| FND-002 | fixed | Verified against `af15747d...15cfb7a6` by independent Opus 5.5; exact after excerpt is in the paired Linear disposition comment. |
| FND-003 | fixed | Verified against `af15747d...15cfb7a6` by independent Opus 5.5; exact after excerpt is in the paired Linear disposition comment. |
