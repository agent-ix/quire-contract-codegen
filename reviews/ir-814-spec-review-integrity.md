---
id: SR-11781
title: IR-814 integrity review of the FR-033 AC-14 to AC-17 composite equality remainder
type: SpecReview
analysis: integrity
scope: agent-ix/quire-contract-codegen@216b447a1f9475534df11056df849eb667203beb (PR 399, diff vs origin/main 007417d); spec/replay/functional/FR-033-composite-parity-replay-binding.md,
  spec/replay/matrix/TC-048-composite-parity-replay-binding.md, spec/replay/matrix/tests.md, spec/assurance/AD-003-evidence-chain.md
review_set: subset
---

## Summary

Ticket: IR-814. Cross-document consistency check of the new AC-14 to AC-17 against the AC-1 and
AC-11 headers, AD-003 E-1, the TC-048 steps, the observation and mutation table, and both replay
matrix rows.

## Verdict

**PASS** for consistency between documents. Every place that names the remainder (AC-1 header,
AC-11 header, AD-003 E-1, TC-048:236, replay tests.md:22, :23 and :40) uses the same split:

- AC-14 and AC-16/AC-17 belong to AC-1; AC-15 belongs to AC-11; all four are under IR-814.
- Eq self-comparison is credited as covered.
- The TC-048 summary row lists AC-14 to AC-17.
- The mutation rows name a failing mutation for each new AC.

The inline-operand problem is one of substance against IR FR-038, not of consistency between
these documents, so it is recorded once, as SR-11780 FND-001 and FND-002.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Examined units

```yaml
scope:
- id: FR-033-AC-1
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: "Ne literal/literal is AC-14 and inline operands are AC-16 and AC-17, all PLANNED under IR-814; falsified and generated-artifact routes planned, IR-806"
- id: FR-033-AC-11
  path: spec/replay/functional/FR-033-composite-parity-replay-binding.md
  role: examined
  excerpt: "Eq self-comparison is covered; Ne literal/literal (AC-14), Ne self-comparison (AC-15) and inline operands (AC-16, AC-17) are PLANNED under IR-814"
- id: AD-003
  path: spec/assurance/AD-003-evidence-chain.md
  role: examined
  excerpt: "Ne literal/literal, Ne self-comparison and inline literals (FR-033-AC-14 to AC-17) are PLANNED under IR-814"
- id: tests.md
  path: spec/replay/matrix/tests.md
  role: examined
  excerpt: "FR-033-AC-14, FR-033-AC-15, FR-033-AC-16, FR-033-AC-17 | TC-048 | Planned (IR-814); Ne literal/literal, Ne self-comparison, inline integer singleton Range operands and typed refusal of other inline literal types are not yet built or traced; Eq self-comparison is already covered under AC-11"
bindings: []
```
