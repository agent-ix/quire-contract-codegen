---
id: SR-35016
title: "Integrity review of AD-004 L-12 correction"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen; spec/assurance/AD-004-cg-crate-layout.md"
review_set: subset
---

## Summary

Reviewed the changed L-12 census, step 4f and the AD's interim-exceptions table at PR #567 head 1340120353d0b47773d308a3772846b2232d60ae. The source census is accurate, but the table that purports to list each interim exception remains incomplete. Ticket: IR-348. No applicable AssuranceProfile was found in the repository spec tree.

## Verdict

**CONDITIONAL** — one medium completeness finding.

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| AD-004-dependency-rule | examined | BoundPackage and BoundClause are named only in strategy/, evidence/, oracle/boolean_v1.rs, oracle/bound_v1.rs, the V1 Kani arm and routed/capability.rs's public V1 clause form and preflight. |
| AD-004-L-12 | examined | Until step 4f the interim exceptions include outcome.rs, negotiate.rs, clause.rs, and routed/capability.rs's V1 clause form, identity and preflight. |
| AD-004-interim-exceptions | examined | Each is a place where the target of this AD is not yet true, stated so that no step stops for it. |
| AD-004-step-4f | context_only | Also remove routed/capability.rs's public KaniForm::V1BoundClause, its KaniFormName identity and V1 clause preflight, plus V1-only tests. |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The exhaustive interim-exceptions table still omits routed/capability.rs's V1 form, identity and preflight, despite L-12 newly listing them. Add a row with step 4f expiry so the table and L-12 agree. | spec/assurance/AD-004-cg-crate-layout.md:1409 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | At e72d3f432af5582de0711729600bba2ba83bc927, the interim-exceptions table now lists the public V1 clause form, its identity and preflight in routed/capability.rs, with removal at step 4f after the V2 route is live. |
