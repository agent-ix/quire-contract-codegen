---
id: SR-021
title: "EARS conformance review of numeric and state Kani lowering"
type: SpecReview
analysis: ears-conformance
scope: "FR-003 numeric/state Kani requirement statements"
review_set: subset
---

## Summary

Quire 0.31.0 classifies all 38 repository spec documents as grammar-clean with zero EARS findings.
Semantic inspection of FR-003's 18 normative statements found concrete subjects and responses after
one compound requirement was split during review.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The initial ABI statement contained two SHALL clauses for arguments and results. It is now split into two atomic ubiquitous requirements. | FR-003 |

## Semantic check

The description's `Where bounded proof lowering is supported` correctly scopes an optional backend
feature. Bounded-integer argument and post-result conditions use `When` for events; the conflicting
or unsupported request condition uses the unwanted-behavior `If ... then ...` form. Readiness,
framing, option identity, and proof-execution statements are persistent ubiquitous obligations.

Every statement names the generator, Kani adapter, harness, contract, graph, assumption, dependency,
or framing region as its subject. Responses are measurable Rust types, order keys, inclusive bounds,
option values, classifications, diagnostic data, or artifact absence. No statement relies on a vague
`support`, `handle`, `manage`, `provide`, or subjective quality response.
