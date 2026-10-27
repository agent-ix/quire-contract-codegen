---
id: SR-597004
title: "IR-348 PR #597 spec-review/dependency review (4f gate removal and external consumers)"
type: SpecReview
analysis: dependency
scope: "agent-ix/quire-contract-codegen; PR #597 head 5e5245233fb8ab99000ce81563a6d66fb9ea5160 against origin/main 83721c07; spec/assurance/AD-004-cg-crate-layout.md (step 4 ordering, 4f, decisions table, risks); IR-348"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AD-004
    type: reviews
---

## Summary

Ticket: IR-348. This lens checks the dependency edges the PR changes. It reorders step 4f ahead of
4a-4e and drops 4f's dependency on QSL's exemplar move and on the installed-Kani control. Each
removed edge becomes open follow-up work. The question is whether each open obligation is routed to
an identifiable owner.

QSL-1154 ("Retire quire-integration V1 Kani bundle calls while preserving IT-011 controls",
Backlog) was confirmed in Linear. Its text was not relied on.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The AD routes open work to unnamed owners. The QSL exemplar and IT-011 disposition is routed with no ticket, although QSL-1154 exists. The FR-031 bundle-row, NFR-006 and V2 census follow-ups say only "tracked by its owner" or "an owning follow-up", and step 4c says no ticket was found. The open obligations cannot be followed from the AD. | spec/assurance/AD-004-cg-crate-layout.md:308, 1108-1111, 1112-1122, 1658-1660, 1696 |

## Verdict

Approved for this lens, apart from the low traceability finding. The new ordering has no cycle:

- 4f no longer depends on 4c, 4d or 4e.
- 4a now follows the QSL migration.
- 4c and 4e still precede the QSL follow-up.
- 4g still depends on `census_validation.rs` surviving 4f, which the AD keeps.

## Coverage

```yaml
scope:
- {id: AD-004:step-4-intro, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "Step 4f, the deprecated V1 retirement, proceeds now and is independent of 4a to 4e."}
- {id: AD-004:4f-gates, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "The V2 census (4d), V2 contract family (4c), public route (4e), installed Kani control and QSL exemplar move are not deletion gates."}
- {id: AD-004:decision-public-entry, path: spec/assurance/AD-004-cg-crate-layout.md, role: examined, excerpt: "Each other former consumer is inventoried and routed to its repository owner; no compatibility entry remains in CG."}
- {id: AD-004:4g, path: spec/assurance/AD-004-cg-crate-layout.md, role: context_only, excerpt: "It removes the corpus's calls to validate_dependencies but not census_validation.rs"}
```

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The AD 4f table and the FR/TC notes now name concrete owners: IR-1044 for the V2 census, IR-917 for the V2 clause and mutation controls, IR-1045 for native FR-031, QSL-1154 for QI IT-011, and IR-348 for the route and suite reconciliation. The step 4c "no ticket" text is gone. Each ticket was confirmed to exist in Linear. One residue: the AD's FR-031-AC-18 row says "separately owned" where FR-031 itself names IR-1045. |
