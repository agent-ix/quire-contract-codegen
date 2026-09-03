---
id: SR-007
title: "Tri-state harness and shaped-strategy gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "Agent B issue #3 harness and strategy implementation after PR #22 exact-head review"
review_set: all
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-002
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TM-001
    type: references
---

# SR-007: Tri-state harness and shaped-strategy gap analysis

## Summary

This is the author's postimplementation reconciliation record. It does not substitute for the
independent review or promote the planned FR-002 and TC-004 matrix rows.

## Preimplementation findings

| Finding | Disposition | Verification |
|---|---|---|
| FND-401 | Closed in the Boolean slice: generated harness code owns snapshot, precondition, invocation, and postcondition order. | Generated-crate rejected, passed, and failed-path tests. |
| FND-402 | Closed for supported populations: generation uses direct construction and has no filtering fallback. | Seeded range, membership, correlation, and residual shrink tests. |
| FND-403 | Closed: the actual emitted boundary population must contain both admitted and rejected values. | Boundary census and full-width fail-closed fixtures. |
| FND-404 | Closed for the declared invocation unit: integer expectations are executable; harness summaries retain attempted, accepted, rejected, failed, and explicit-discard invocation counts, including retries and shrink replays. | Exact typed campaign-outcome fixtures and strategy shrink tests. |
| FND-405 | Closed within the declared Boolean slice; integer and customer-enum entry-point binding remains outside this interface. | Public API and generated private-field case inspection. |

## Findings

PR #22's exact-head review at `b32ae2a` found three high, six medium, and five low issues. The
remediation makes the following changes for rereview:

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-2201 | high | R7-01: Campaign accounting is concluded after every framework result. The runner no longer rejects low configured case counts before execution, and exact fixtures distinguish below-floor outcomes. | FR-002-AC-5, TC-004 |
| FND-2202 | high | R7-02: Harness and strategy source enforce the same 1 MiB maximum declared by the shared attestation command, returning `ResourceLimitExceeded` and `Unsupported`. | interface-001, TC-004 |
| FND-2203 | high | R7-03: Enum output states plainly that its finite membership is all-admitted and no longer emits an unreachable rejected variant. Integer boundary cases retain value-dependent accepted/rejected classification. | FR-002-AC-6, TC-004 |
| FND-2204 | medium | R7-04: `minimum_rejected_cases` is caller-supplied, rendered once, enforced, and included in deterministic identity; zero declares a total precondition. | FR-002-AC-5, interface-001 |
| FND-2205 | medium | R7-05: Generated campaigns return typed below-accepted-floor, below-rejected-floor, above-discard-ceiling, exhausted, and failed outcomes with retained summaries. | FR-002-AC-5, TC-004 |
| FND-2206 | medium | R7-06: One diagnostic constructor derives terminal state from the lower-level generation code or from direct-code mappings; wrapper codes have no independent terminal answer. | interface-001, TC-004 |
| FND-2207 | medium | R7-07: The interface distinguishes precondition test-function/global rejection from the explicit-discard constructor and names runner exhaustion separately. The generated fixture uses `max_global_rejects`; proptest 1.5's `run_one_with_replay` routes `TestCaseError::Reject` through `reject_global`. | interface-001, TC-004 |
| FND-2208 | medium | R7-08: Summary documentation and interface text say that counters count adapter invocations, including retries and shrink replays; `attempted` supplies the rate denominator. | FR-002-AC-1, interface-001 |
| FND-2209 | medium | R7-09: Negative generated-crate fixtures match exact typed outcomes rather than `is_err()`. | TC-004 |
| FND-2210 | low | R7-10: Each policy value is rendered into one generated constant used by every check. | interface-001 |
| FND-2211 | low | R7-11: The generated harness compiled and executed by TC-004 uses an accepted floor above one, a positive rejected floor, and a zero explicit-discard ceiling. | TC-004 |
| FND-2212 | low | R7-12: The stale Makefile comment announcing a removed final guard is deleted without recreating a local assurance control. | Makefile |
| FND-2213 | low | R7-13: This postimplementation analysis lives beside the repository's other SpecReviews. | SR-007 |
| FND-2214 | low | R7-14: REV-007 remains a planning-only, future-tense preimplementation record. | REV-007 |

## Remaining declared gaps

- Generated integer and customer-enum strategy cases are not automatically converted into the
  Boolean harness case type; callers still own that entry-point binding.
- Enum memberships cannot synthesize a customer enum variant outside the declared membership, so
  enum populations exercise admission only.
- Framework search limits remain caller-owned. Campaign summaries separately retain precondition
  rejections and explicit discards and classify framework exhaustion without parsing its prose.
- FR-002 and TC-004 remain planned until the independent rereview accepts the exact pushed head.
