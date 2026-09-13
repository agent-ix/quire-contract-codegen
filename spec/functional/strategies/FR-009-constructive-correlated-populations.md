---
id: FR-009
title: "Construct satisfying and violating populations for correlated values"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-008
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/interface-001
    type: implements
---
# FR-009: Construct satisfying and violating populations for correlated values

## Description

When a bound clause is admitted by [FR-008](./FR-008-bound-domain-strategy-admission.md), the
generator shall emit proptest populations that construct every case directly inside the declared
domains and on the requested side of the relation. It shall never draw a candidate and then filter it
out.

## Inputs

- An admitted relation `left op right` with the domains from the FR-008 census.
- A requested population: `Satisfying`, `Violating`, or `Broad`.

## Outputs

- Generated Rust whose cases carry one `i64` per generated read (declaration name and observation) and
  an expectation tag `Holds` or `Violated`.

## Behavior

- Each generated read value shall lie within its declaration's inclusive domain.
- For a read compared with a literal, the generator shall compute the satisfying and violating value
  sets as at most two inclusive intervals intersected with the domain. It shall draw directly from
  those intervals.
- For two reads, including the `Pre` and `Post` reads of one `State` declaration, the generator shall
  draw the first value only from values that have a non-empty partner set on the requested side of
  the relation. It shall then draw the second value from that partner set, computed from the first
  value.
- For `Equal` between two reads, the satisfying population shall set the second value equal to the
  first, restricted to the intersection of both domains. ConfigVersion `VersionUnchanged`
  (post = pre) is this case.
- For `NotEqual`, the generator shall build the value set "domain minus one point" by drawing an index
  over the reduced size and mapping around the excluded point, not by rejecting the excluded point.
- The `Broad` population shall combine the `Satisfying` and `Violating` populations as a union, so
  every case keeps its own tag.
- Every boundary and endpoint computation shall use checked `i64` arithmetic. An unrepresentable
  endpoint shall shrink the interval rather than wrap.
- If a requested population's value set is empty over the declared domains, then the generator shall
  refuse that population with `EmptyPopulation`. The diagnostic shall state which side (`Satisfying`
  or `Violating`) is empty, so a tautological or unsatisfiable clause is visible rather than silently
  generating nothing.
- Generated populations shall contain no `prop_filter`, `prop_assume`, global reject, or explicit
  discard.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-009-AC-1 | For `VersionUnchanged` over 0..=1000, every `Satisfying` case has post = pre and every `Violating` case has post ≠ pre, and all values lie in 0..=1000. | Test (TC-018) |
| FR-009-AC-2 | For every operator and every pair of small domains in an exhaustive census (domains with 1 to 4 members, including disjoint, touching, and equal domains), the set of generatable values equals the true satisfying or violating set: none missing, none extra. | Test (TC-018) |
| FR-009-AC-3 | Generated population source contains no `prop_filter`, `prop_filter_map`, `prop_assume`, reject, or discard call, and a seeded campaign of 10,000 cases records zero discards and zero global rejects. | Test (TC-018) |
| FR-009-AC-4 | A clause whose satisfying set is empty over its domains (for example `amount < 0` with domain 0..=1000) refuses the `Satisfying` and `Broad` populations with `EmptyPopulation` naming the empty side, and still generates `Violating`. | Test (TC-018) |
| FR-009-AC-5 | Domains touching `i64::MIN` or `i64::MAX` generate without overflow or panic, and every case stays in domain. | Test (TC-018) |
| FR-009-AC-6 | Repeated generation for the same package, clause, and population is byte-identical. | Test (TC-018) |

## Dependencies

- **Upstream**: [FR-008](./FR-008-bound-domain-strategy-admission.md).
- **Downstream**: [FR-011](./FR-011-numeric-harness-campaigns.md),
  [FR-012](./FR-012-constraint-preserving-shrinking.md),
  [TC-018](../../test/strategies/TC-018-constructive-correlated-populations.md).
