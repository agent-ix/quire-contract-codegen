---
id: SR-013
title: "Scope and boundary review of numeric and state oracle lowering"
type: SpecReview
analysis: scope-boundary
scope: "FR-001/interface-001 issue #4 allocation under epic quire-spec-language#83"
review_set: subset
---
## Summary

The review assigns deterministic oracle emission and explicit refusal to codegen while preserving IR
typing, runtime operator, SL lowering, strategy, and proof ownership. No adjacent backend work is
absorbed into issue #4.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No issues found | - |

## Responsibilities

| Boundary | Owner | Classification | Contract |
| --- | --- | --- | --- |
| Typed expression, node types, obligations, dependency identities, and SourceSpan | quire-contract-ir | guaranteed by pinned public Rust API | revision 04eb6f849c03be23177d373549c6c272551f957d |
| Boolean operators and generated consumer surface | quire-contract-runtime | guaranteed by generated-source compilation | revision 8a4d02b9ff4633cf6d02fd8bdf6ee1b11ad76354 |
| Boolean/i64 signature, comparison rendering, diagnostics, source maps, attestations | quire-contract-codegen | core, in scope | FR-001 and interface-001 |
| Native source lowering, runtime parity, and IR pin reconciliation | quire-spec-language | external follow-up | epic #83 pin bump and issue #84 |
| Model-domain proptest strategies | quire-contract-codegen issue #3 owner | external sibling | consume after merge; do not edit its branch/files |
| Numeric/state Kani harnesses | quire-contract-codegen issue #2 | external next slice | begins only after issue #4 |

Object/graph reads, arithmetic, numeric negation, scalar-root oracles, definedness discharge, strategy
changes, Kani changes, and SL pin edits are out of scope. Each semantic form refuses explicitly;
none is approximated by codegen.
