---
id: SR-1121012
title: IR-1124 PR 631 integrity review
type: SpecReview
analysis: integrity
scope: agent-ix/quire-contract-codegen; IR-1124; CG PR 631; FR-011-AC-16..18, TC-020,
  TC-006, TestMatrix
review_set: subset
---

## Summary

Independent spec-review/integrity review of frozen CG PR #631 head b4b3c4f598eae7651e75c7d2cda0060f16a2b52f. Boundary consistency with FR-008, FR-004 and FR-011 AC-14/15. The V2 runner is Planned behind IR-1072 and FR-008; FR-004 producer binding remains separate.

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

**FAIL** until the AC-16 source-inspection phrase names the V1 `ClauseRef` rather than an unqualified clause reference.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-16 forbids an unqualified “clause reference” in the V2 path, although the public V2 request must retain its selected checked clause; name the V1 `ClauseRef` that is forbidden | spec/strategy/functional/FR-011-numeric-harness-campaigns.md:281 |

## Dispositions

Round 1, CG #631 exact head `311b2606a6167996725528b29c9f6b7c326ee032`.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | FR-011-AC-16 now forbids V1 `BoundPackage`, V1 `ClauseRef`, `generate_bound_strategy` and V1-to-V2 adapters specifically, while requiring the selected V2 checked clause key; TC-020 step 13 matches this boundary. |

## Current verdict

No remaining finding in the scoped review at the fixed head.
