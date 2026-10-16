---
id: SR-11773
title: IR-364 slice 2 failure-domain review of the V2 strategy chain criteria
type: SpecReview
analysis: failure-domain
scope: agent-ix/quire-contract-codegen@f30f17b1a086211cd0ab87f2445eb5565fac74b5 (PR 400, diff vs origin/main 007417d); spec/strategy/functional/FR-008..FR-011, spec/strategy/matrix/TC-017..TC-019
review_set: subset
---

## Summary

Ticket: IR-364 (slice 2). This review covers identity confusion and edge cases in the V2 inputs:

- the selected clause and authentic claim identity;
- literal-first operand order;
- the i64 extremes under FR-008-AC-6 and FR-010-AC-3;
- the out-of-domain edge identities in FR-010 Outputs, now keyed on the typed V2 declaration identity
  and observation;
- refusal and locus retention.

## Verdict

**PASS, no new findings.**

- FR-010-AC-3 is consistent with FR-008-AC-6. An accessor domain of exactly `i64::MIN..=i64::MAX` is
  admitted, and anything wider refuses.
- FR-009-AC-6 now keys determinism on package, clause id, claim occurrence and population. That
  closes the V1 "same clause" identity gap.
- FR-009-AC-7 and the new FR-009 Behavior bullet cover the literal-first mirror.
- The FR-010 out-of-domain identity uses the typed declaration identity plus observation. That
  separates the `Post` and `Pre` reads of ConfigVersion, which share one record.project id (IR
  FR-038-AC-204).
- The missing V2 source for the `construct_verdict` execution point and identity is a failure-domain
  gap as well. It is recorded once, as SR-11770 FND-003, and not repeated here.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Examined units

```yaml
scope:
- id: FR-008-AC-6
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: context_only
  excerpt: "Either read endpoint or a literal at i64::MIN - 1, i64::MAX + 1, i128::MIN or i128::MAX refuses `UnsupportedRelation` at that operand without narrowing."
- id: FR-010-AC-3
  path: spec/strategy/functional/FR-010-domain-boundary-campaigns.md
  role: examined
  excerpt: "A selected claim whose accessor domain is exactly `i64::MIN..=i64::MAX` (representable under FR-008-AC-6) compiles as generated Rust without overflow or panic, its out-of-domain array holds no case for either outer edge, and its unrepresentable-edge array lists both."
- id: FR-010
  path: spec/strategy/functional/FR-010-domain-boundary-campaigns.md
  role: examined
  excerpt: "A generated constant array listing every unrepresentable edge value by read (its typed V2 declaration identity and observation), edge, and direction."
- id: FR-009-AC-6
  path: spec/strategy/functional/FR-009-constructive-correlated-populations.md
  role: examined
  excerpt: "Repeated generation for the same `CheckedPackageV2`, selected clause id, authentic claim occurrence, and population is byte-identical."
- id: FR-009-AC-7
  path: spec/strategy/functional/FR-009-constructive-correlated-populations.md
  role: examined
  excerpt: "PLANNED. For each noncommutative operator, a comparison written with the literal in the first position produces the same satisfying and violating sets as its mirrored read-first form, enumerated independently."
- id: TC-019
  path: spec/strategy/matrix/TC-019-domain-boundary-campaigns.md
  role: examined
  excerpt: "Generate and compile the `Boundary` bundle for `amount < 7` with the accessor's domain exactly `i64::MIN..=i64::MAX`, and for `x == y` over that domain."
bindings: []
```
