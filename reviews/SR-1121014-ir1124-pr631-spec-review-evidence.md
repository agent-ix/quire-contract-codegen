---
id: SR-1121014
title: IR-1124 PR 631 evidence review
type: SpecReview
analysis: evidence
scope: agent-ix/quire-contract-codegen; IR-1124; CG PR 631; FR-011-AC-16..18, TC-020,
  TC-006, TestMatrix
review_set: subset
---

## Summary

Independent spec-review/evidence review of frozen CG PR #631 head b4b3c4f598eae7651e75c7d2cda0060f16a2b52f. Test verification and TC-020/TC-006 mapping; no campaign credit on publication alone. The V2 runner is Planned behind IR-1072 and FR-008; FR-004 producer binding remains separate.

## Scope

```yaml
scope:
- id: FR-011-AC-16
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: PLANNED (IR-1124; depends on IR-1072 and FR-008). A public V2 strategy
    request over an admitted checked package and one authentic selected claim produces
    a complete source, map, census and numeric runner bundle whose selected clause,
    owner, occurrence, package revision, comparison and runner identities agree exactly.
    The runner executes the FR-009 constructive population and FR-010 census with
    the selected V2 oracle; source inspection finds no V1 bound package, clause reference,
    bound strategy generator call or adapter in this V2 path.
- id: FR-011-AC-17
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: PLANNED (IR-1124). Independently injected unknown clause, unsupported kind,
    wrong owner/role/ordinal, missing accepted point, FR-008 comparison refusal, empty
    population, identity mismatch, point-render mutant, invalid generated syntax and
    resource overflow each return a typed diagnostic with the selected key and available
    structural locus, publish zero artifacts and start no campaign. Pairwise overlapping
    defects return the first cause in Behavior order; a valid sibling still publishes
    its complete bundle.
- id: FR-011-AC-18
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: examined
  excerpt: PLANNED (IR-1124; FR-004 live binding remains separate). A campaign run
    from the emitted V2 runner retains the selected claim and package identity with
    its own source/map and result for same-invocation producer binding. Published
    bytes or a detached/copied result alone yield no authenticated campaign, measured
    coverage or proof credit. A refused selection yields no campaign evidence, including
    when another claim of the same package succeeds.
- id: TC-020
  path: spec/strategy/matrix/TC-020-numeric-harness-campaigns.md
  role: examined
  excerpt: Steps 13-15 verify complete V2 publication, typed first-refusal order,
    and same-invocation evidence binding.
- id: TC-006
  path: spec/evidence/matrix/TC-006-vacuity.md
  role: examined
  excerpt: Planned V2 runner publication control rejects copied bytes and detached
    results for coverage credit.
```

## Verdict

**PASS** for this method.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
