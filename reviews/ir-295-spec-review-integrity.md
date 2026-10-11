---
id: SR-004
title: "Integrity review of IR-295"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen@44afebc0f9e1e042447536703761368f873e2bde; FR-019-AC-25, FR-019 Inputs/Outputs/Behavior, TC-030 new procedure; FR-015 and QSL ADR-012 §7.2 as context"
review_set: subset
---

## Summary

Reviewed AC atomicity, falsifiability, typed-form boundaries, and the test procedure against FR-015 and QSL ADR-012 §7.2. AC-25 combines several independently verifiable settlement obligations and leaves the form-to-capability boundary implicit.

## Verdict

**CONDITIONAL** — split the compound criterion and name the governing typed-form admission set.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-019-AC-25 combines bounded admission, form refusal, refusal precedence, display/debug independence, unbounded requires-bound, and a subsequent bounded request in one criterion, obscuring independent verification and trace coverage. | spec/routed/functional/FR-019-capability-settlement.md:309 |
| FND-002 | medium | FR-019-AC-25 uses “admitted” and “outside Kani's capability” without naming the typed tag/form pairs or pointing to the FR-015 rule that defines the Kani-supported set, so the same request can be classified differently by independent implementers. | spec/routed/functional/FR-019-capability-settlement.md:55-59, 114-122, 309 |

## Failure-domain check

The new ingress is described as typed, in-process data and does not add a callback, external lookup, or graph operation. No additional failure-domain omission was identified beyond the undefined typed-form capability boundary in FND-002.
