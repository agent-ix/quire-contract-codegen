---
id: SR-17940486709564847047940
title: "IR-1526 PR #822 route feature spec review (integrity)"
type: SpecReview
analysis: integrity
date: 2026-11-13
scope: "agent-ix/quire-contract-codegen; PR #822 @ a10eb3ca; IR-1526; spec/kani/functional/FR-056-kani-tractability-refusal.md, spec/kani/matrix/TC-070-kani-tractability-refusal.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-056
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-070
    type: references
---

## Summary

Ticket: IR-1526. Integrity analysis of PR #822: internal consistency, cross-references and AC to
TC-step traceability of the route additions. The diff is the scope. The base checklist findings
are in SR-17940486709564847047939.

## Verdict

**Four consistency defects: one medium, three low.** The route is consistent across Terms, Inputs,
Outputs, the decider seam, the table schema and Lookup, and across AC-3/4/5/14 and TC-070 steps
3/4/5/7a/10. The defects are one untraced AC clause and three wrong cross-references or Status
claims.

## Findings

| ID      | Severity | Summary | Refs |
| ------- | -------- | ------- | ---- |
| FND-001 | medium   | AC-2's new route clause ("CG also exposes the route … reporting a fixed route fails the route check") has no TC-070 step. Step 2 is unchanged and checks widths only. TC-070 Status still lists step 2 among the route additions. Fix: add the route assertion and its fixed-route mutant to step 2. | spec/kani/functional/FR-056-kani-tractability-refusal.md:114, spec/kani/matrix/TC-070-kani-tractability-refusal.md:22-24 |
| FND-002 | low      | AC-7 says the route "is matched by the next criterion". The next criterion is AC-8, which is about precedence. Route matching is Lookup and AC-14. Fix: cite FR-056-AC-14. | spec/kani/functional/FR-056-kani-tractability-refusal.md:119 |
| FND-003 | low      | Status lists "the route additions to FR-056-AC-2 to AC-5, AC-7, AC-8 and AC-10", but the PR leaves AC-8 unchanged. The route precedence case is in AC-14. Fix: drop AC-8 from the list. | spec/kani/functional/FR-056-kani-tractability-refusal.md:141-142 |
| FND-004 | low      | Status says that among native-route cases at and above 15 bits, "none has been measured except the AC-13 clause that Verified in 235.5 s on the native route". That clause is `with_bounded_amount_implication` over `Int[0, 1000]` (10 bits), so it is not a case at or above 15 bits. The same Status still says FR-056-AC-13 "has not been run under installed Kani", which contradicts the 235.5 s run it cites. Fix: describe it as a below-15-bit native row and update the AC-13 run sentence. | spec/kani/functional/FR-056-kani-tractability-refusal.md:161,164-165, tests/it/kani_obligations_v2_clause_controls.rs:1516-1520 |
