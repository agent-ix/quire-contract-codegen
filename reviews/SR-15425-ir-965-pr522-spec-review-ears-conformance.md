---
id: SR-15425
title: "EARS review of IR-965 state-clause replay target"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-codegen; IR-965; spec/replay/functional/FR-024-counterexample-envelope-intake.md, spec/replay/matrix/TC-035-counterexample-envelope-intake.md, spec/replay/matrix/tests.md; uncommitted diff atop 5424f623908da3a086960be14666060c009f0e3c"
review_set: subset
---

## Summary

Independent review of uncommitted IR-965/STD-261 extension at diff SHA256 `3f934b86dae897bd9be90c3cb9eac7f09c811dae7b0f1510a8106e0a4f47666e`. Grounded in merged QSpec FR-323 (#364, 429c59d) and QSL FR-122 (#1211, 95a93c8) plus QSL FR-070/STD-245 (#1212, e0f8ffb). The three exclusive targets, clause-name ownership, no payload `clause`, pairwise refusal, Q-6 removal, and prospective implementation status agree with those owner specs, subject to the findings below.

## Verdict

PASS — Edited normative requirement statements are specific, observable, and use named actors and conditions; no EARS finding in this diff.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Only the new uncommitted FR-024 diff and its directly conflicting TC/matrix claims were examined. The prior SR-15418 disposition remains historical context. FR-024-AC-41/42 have no trace binders yet, consistent with this spec-first draft; AC-12 retains old test binders that do not establish the new selection. Quire validation of edited FR-024 is grammar clean. No Cargo or full gates ran.

```yaml
scope:
- id: FR-024-AC-12
  path: spec/replay/functional/FR-024-counterexample-envelope-intake.md
  role: examined
  excerpt: The state-clause envelope's `clause_node` equals the `ClauseSite` node, and its `occurrence_key`
    the `ClauseSite` occurrence, that `qsl_replay::call_site` returns for the clause name. Its packet
    and request carry `state_clause` as that clause's declared `QualifiedName`, with neither other replay
    target; its payload has no `clause` selection and its `observation` is the `Invocation` arm. For a
    unit with two postcondition clauses, each clause's envelope carries its own node, occurrence and declared
    name. A clause name the unit does not declare returns `StateClauseReplayError::CallSite` holding t
```

## Final scope confirmation

The final uncommitted three-file diff SHA256 is `33ffcc67b065b78d3ef8790b5dfbbfd7e0f9bc35488dd41a959b4f4cb9583a96`; edited requirement sentences remain concrete, named and conditional. No EARS finding.
