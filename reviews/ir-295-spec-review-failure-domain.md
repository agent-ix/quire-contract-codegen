---
id: SR-005
title: "Failure-domain review of IR-295"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-contract-codegen@44afebc0f9e1e042447536703761368f873e2bde; FR-019 typed IR form input and Kani settlement behavior; TC-030 form refusal cases"
review_set: subset
---

## Summary

Checked the added typed-form negotiation input for extension-point failure behavior, identity ambiguity, side effects, and topology risks. The typed in-process input adds no extension callback or graph traversal; the capability-set boundary ambiguity is recorded by the integrity analysis.

## Verdict

**PASS** — no separate failure-domain finding applies.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
