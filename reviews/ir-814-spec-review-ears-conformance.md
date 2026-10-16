---
id: SR-11782
title: IR-814 EARS conformance review of FR-033 AC-14 to AC-17
type: SpecReview
analysis: ears-conformance
scope: agent-ix/quire-contract-codegen@216b447a1f9475534df11056df849eb667203beb (PR 399, diff vs origin/main 007417d); spec/replay/functional/FR-033-composite-parity-replay-binding.md
review_set: subset
---

## Summary

Ticket: IR-814. EARS check of the four new criteria. AC-14 and AC-15 use the event pattern
("When ..., the verified request carries ..."). AC-16 uses the optional-feature pattern
("Where ..."). AC-17 uses the unwanted-behaviour pattern ("If ..., then the builder refuses ...").

## Verdict

**PASS with two low findings.** Each criterion names its subject and one response.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | AC-14 adds a second, unrelated sentence: "Literal operands are admitted one-field `Integer` records; bounded-field narrowing literals (IR-691) and collection-constructor literals remain gated." That is a scope and status statement, not part of the event/response. It also narrows the requirement to the fixture's record shape. Move it to the status prefix or the TC-048 step. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:374 |
| FND-002 | low | AC-16 uses EARS "Where", which is meant for an optional feature of the product, to express a runtime input condition, "the owning operand projection yields an inline integer literal operand". The event or unwanted pattern ("When/If ...") fits a runtime condition. This is secondary to SR-11780 FND-001, which shows the condition cannot occur. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:376 |

## Examined units

```yaml
scope:
- id: FR-033-AC-14
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: "When the original application node is a `structural.ne` whose two operands are both composite literals and the requested operator is NotEqual, the verified request carries NotEqual and an O-09 of two positional `graph_child` arguments, each the literal's own node with empty Bounds and no parameter position. Literal operands are admitted one-field `Integer` records; bounded-field narrowing literals (IR-691) and collection-constructor literals remain gated."
- id: FR-033-AC-15
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: "When the original application node is a `structural.ne` comparing one parameter with itself and the requested operator is NotEqual, the verified request carries NotEqual and an O-09 of two positional `graph_child` arguments with the same node identity"
- id: FR-033-AC-16
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: "Where the owning operand projection yields an inline integer literal operand of an Eq or Ne application, the O-09 carries one argument at that operand's position"
- id: FR-033-AC-17
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: "If an inline literal operand has a type other than integer, then the builder refuses with a typed inline-operand refusal before any QSL invocation, with no report or terminal value."
bindings: []
```

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | - |

## Disposition round 1 verdict

**PASS**. Reviewed 437567e. AC-14's scope note has moved into the status prefix. AC-16 and AC-17 both use the unwanted-behaviour pattern ("If ..., then the builder returns ..."). No new EARS findings.
