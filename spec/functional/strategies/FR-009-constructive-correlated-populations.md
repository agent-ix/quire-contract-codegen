---
id: FR-009
title: "Construct satisfying and violating populations for correlated values"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-008
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/interface-001
    type: implements
---
# FR-009: Construct satisfying and violating populations for correlated values

## Description

When a bound clause is admitted by [FR-008](./FR-008-bound-domain-strategy-admission.md), the
generator shall emit proptest populations that construct every case directly inside the shared domain
and on the requested side of the relation. A case is a complete valuation of the clause's reads,
including any `Post` read, and its tag is the value the clause's generated oracle returns for that
valuation. The generator never draws a candidate and then filters it out.

## Inputs

- An admitted relation `left op right` and the shared domain from the FR-008 census.
- A requested population: `Satisfying`, `Violating`, or `Broad`.

## Outputs

- Generated Rust whose case type carries one `i64` per read and an expectation tag of `Holds` or
  `Violated`.

## Behavior

- The generator shall place every generated read value within the shared inclusive domain.
- For a primary read compared with a literal, the generator shall compute the satisfying and
  violating value sets as at most two inclusive intervals intersected with the domain.
- The generator shall draw a primary read compared with a literal directly from those intervals.
- For a primary read and a partner read, the generator shall draw the primary value only from values
  whose partner set on the requested side of the relation is non-empty.
- The generator shall draw the partner value from the partner set computed from the drawn primary
  value.
- For `Equal` between two reads, the generator shall set the partner value of every satisfying case
  equal to its primary value. ConfigVersion `VersionUnchanged` (post = pre) is this case.
- For `NotEqual`, the generator shall construct "domain minus one point" by drawing an index over the
  reduced size and mapping it around the excluded point.
- The generator shall compute interval sizes, index offsets, and edge values with checked 128-bit
  integer arithmetic, so a domain as wide as `i64::MIN..=i64::MAX` needs no wrapping, saturation, or
  truncation.
- The `Broad` population shall contain both the `Satisfying` and `Violating` populations, with each
  case keeping its own tag.
- If the requested side has no value over the domain, then the generator shall refuse that population
  with `EmptyPopulation` naming the empty side.
- If either side is empty, then the generator shall refuse the `Broad` population with
  `EmptyPopulation` naming the empty side.
- The generator shall report `EmptyPopulation` with terminal state `unsupported`.
- Generated populations shall contain no `prop_filter`, `prop_filter_map`, `prop_assume`, global
  reject, or explicit discard.
- The generator shall not claim that a case satisfies any clause other than the requested one.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-009-AC-1 | For `VersionUnchanged` over 0..=1000, every `Satisfying` case has post = pre, every `Violating` case has post ≠ pre, and every value lies in 0..=1000. | Test (TC-018) |
| FR-009-AC-2 | For every comparison operator, every domain of 1, 2, 3, and 4 members, and both a two-read relation and a read-versus-literal relation with the literal at each domain member, the generatable set of each population equals the independently enumerated satisfying or violating set: none missing, none extra. An empty side is refused with `EmptyPopulation` and never produces an empty strategy. | Test (TC-018) |
| FR-009-AC-3 | Generated population source contains no filter, assume, reject, or discard call, and a seeded 10,000-case run of each population records zero discards and zero proptest global rejects. | Test (TC-018) |
| FR-009-AC-4 | For `amount < 0` over 0..=1000, `Satisfying` and `Broad` are refused with `EmptyPopulation` naming `Satisfying`, and `Violating` generates; for `amount >= 0`, `Violating` and `Broad` are refused naming `Violating`; no refused request emits an artifact or attestation. | Test (TC-018) |
| FR-009-AC-5 | A domain of `i64::MIN..=i64::MAX` with `NotEqual`, `Less`, and `Equal` compiles as generated Rust and generates 10,000 seeded cases without overflow or panic, every case in domain and on its side. | Test (TC-018) |
| FR-009-AC-6 | Repeated generation for the same package, clause, and population is byte-identical. | Test (TC-018) |

## Dependencies

- **Upstream**: [FR-008](./FR-008-bound-domain-strategy-admission.md).
- **Constrained by**:
  [NFR-004](../../nonfunctional/strategies/NFR-004-constructive-generation-efficiency.md).
- **Downstream**: [FR-011](./FR-011-numeric-harness-campaigns.md),
  [FR-012](./FR-012-constraint-preserving-shrinking.md),
  [TC-018](../../test/strategies/TC-018-constructive-correlated-populations.md).
