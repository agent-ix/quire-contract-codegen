---
id: FR-011
title: "Run numeric oracle-conformance campaigns and report rates"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-008
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-009
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-010
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-002
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/interface-001
    type: implements
  - target: ix://agent-ix/quire-contract-runtime/FR-001
    type: depends_on
  - target: ix://agent-ix/quire-contract-runtime/FR-003
    type: references
  - target: ix://agent-ix/quire-contract-runtime/FR-004
    type: depends_on
  - target: ix://agent-ix/quire-contract-runtime/interface-001
    type: depends_on
---
# FR-011: Run numeric oracle-conformance campaigns and report rates

## Description

When a strategy bundle is generated for a selected claim of a `CheckedPackageV2`
([FR-008](./FR-008-bound-domain-strategy-admission.md)), the generator shall also emit an
oracle-conformance campaign runner. The runner evaluates the selected claim's V2 comparison oracle
on each generated valuation, records the runtime verdict for the checked clause kind, and requires
that verdict to equal the case's
expectation. Its summary reports the discard rate and the rejection rate as exact counts, or as
unavailable when a counter may be saturated.

The runner invokes no subject. A numeric subject harness, where an operation produces the post-state,
is outside this slice and stays under [FR-002](./FR-002-tristate-proptest.md)'s Boolean harness
until specified.

## Inputs

- A generated population from [FR-009](./FR-009-constructive-correlated-populations.md), or the
  in-domain census from [FR-010](./FR-010-domain-boundary-campaigns.md).
- The selected claim's V2 comparison oracle (FR-008), embedded in the same bundle; no V1 bound
  oracle generation output is an input.
- The selected QSL `RequirementItem`, whose `occurrence()` has an `Ok` result from
  `qsl_package::export_bindings`, and the admitted `CheckedPackageV2` whose `package_id`
  identifies its revision.
- A caller-owned proptest `TestRunner` and `quire_contract_runtime::CampaignReport`.
- The existing campaign policy: minimum accepted, minimum rejected, and maximum discarded counts.

## Outputs

- A generated population runner and a generated census runner.
- A generated summary type with the existing `attempted`, `accepted`, `rejected`, `failed`, and
  `discarded` fields, plus `discard_rate()` and `rejection_rate()`.
- The existing generated campaign error type, plus a `ConformanceMismatch` variant carrying the case
  values, the expected tag, and the observed verdict kind.

## Behavior

- The runner shall map an oracle result to a runtime verdict by the selected checked clause's kind
  (the clause kind IR's comparison accessor returns for the selected clause):
  - for a `Precondition` clause, `true` to `Passed` and `false` to `RejectedPrecondition`;
  - for a `Postcondition` clause, `true` to `Passed` and `false` to `FailedPostcondition`;
  - for an `Invariant` clause, `true` to `Passed` and `false` to `FailedPostcondition` whose clause
    observation carries the runtime's invariant clause kind, which quire-contract-runtime
    implements but does not yet specify.
- The runner shall construct each verdict through the runtime `construct_verdict` operation
  (quire-contract-runtime interface-001) with the selected accepted QSL item's occurrence key as
  requirement identity and the admitted `CheckedPackageV2::package_id()` as revision identity,
  together with the runtime execution point and clause observations. The occurrence key shall
  retain its node, role and ordinal; the revision shall retain the complete package id, using the
  canonical V2 rendering below at each borrowed-string runtime boundary. The
  execution-point source remains open (below), so that part of verdict construction is not yet
  testable. CG shall not create a separate requirement-revision artifact.
- For a false invariant, the runner shall use the failure detail the runtime
  provides for a contract clause, because quire-contract-runtime's specification does not yet state
  the failure detail of an invariant; a runtime specification of that mapping supersedes this rule.
- The runner shall record each verdict through the runtime `record_campaign_verdict` operation, so
  the counters follow quire-contract-runtime FR-004: a passed or failed case counts in `accepted`, a
  failed case also counts in `failed`, and a rejected precondition counts in `rejected`.
- If recording refuses the verdict with an identity mismatch, then the runner shall fail the campaign
  with that error and record nothing further.
- The runner shall not use the runtime `adapt_to_proptest` or `adapt_to_proptest_and_record`
  operations, which map a rejection only to a proptest rejection (quire-contract-runtime FR-003);
  this runner tests oracle conformance, where an expected rejection is the correct oracle result
  rather than a case to skip.
- When the observed verdict equals the verdict the case's tag predicts, the runner shall return a
  passing proptest result, including for an expected `RejectedPrecondition`.
- If the observed verdict differs from the verdict the case's tag predicts, then the runner shall fail
  the campaign with `ConformanceMismatch`.
- The runner shall never return a proptest global reject or an explicit discard, so framework
  exhaustion cannot come from rejected preconditions.
- The runner's passing result shall not count a rejected precondition as contract success: the
  rejection stays counted only in `rejected`, preserving the quire-contract-runtime interface-001
  invariant that rejected preconditions are never successful evidence.
- The census runner shall evaluate every in-domain census case exactly once, in census order, without
  proptest sampling or shrinking.
- The runner shall not evaluate out-of-domain cases.
- The generated summary shall expose `discard_rate()` and `rejection_rate()`, each returning the exact
  pair `(discarded, attempted)` or `(rejected, attempted)` over the existing `attempted` denominator
  from interface-001 `accounting_unit`.
- When `attempted` is zero, `discard_rate()` and `rejection_rate()` shall return `None`.
- When the report snapshot is `at_limit` (quire-contract-runtime FR-004), `discard_rate()` and
  `rejection_rate()` shall return `None`, because a saturated counter is not an exact count.
- The generator shall emit the rate accessors on the strategy-bundle summary type only, leaving the
  harness summary unchanged.
- The runner shall apply the existing floor, ceiling, and `Exhausted` conclusions of
  [FR-002](./FR-002-tristate-proptest.md) to the complete supplied report unchanged.
- When proptest reports a failing case, the runner shall conclude `ConformanceMismatch` after the
  discard-ceiling check, so the runner never concludes the harness `Failed`.

### Canonical V2 identity rendering

For a selected QSL `RequirementItem` whose occurrence was accepted by `export_bindings`, CG shall
render its `OccurrenceKey` as
`qocc2_<node>_<role-byte-count>_<role-hex>_<ordinal>`. `<node>` is the `WireNodeId`'s exactly 64
lowercase hexadecimal digits. `<role-hex>` is the lowercase hexadecimal encoding of every UTF-8
byte of `occurrence.origin().role().as_str()`, with two digits per byte; `<role-byte-count>` is the
UTF-8 byte length in base-10 with no leading zero except `0`. `<ordinal>` is the occurrence
ordinal in base-10 with no leading zero except `0`.

CG shall render the admitted `CheckedPackageV2::package_id()` as
`qpkg2_<domain-byte-count>_<domain-hex>_<algorithm-byte-count>_<algorithm-hex>_<digest-byte-count>_<digest-hex>`.
Each count and hex field follows the role rule above, using the exact UTF-8 bytes of the
`CheckedSemanticId`'s `domain`, `algorithm` and `digest` fields in that order. These encodings
are injective and reversible; CG shall not truncate, case-fold or substitute any field, compute
a new identity digest, or treat the digest field alone as the revision. `qocc2_` and `qpkg2_`
separate the two identity domains. The rendered strings are the values of runtime
`RequirementId` and `RevisionId`, respectively. Generated headers and source maps shall carry
these same strings; generated symbol identity suffixes shall be
`cg_<qocc2-string>_<qpkg2-string>`, where both components follow the grammar above.
Comparisons shall use the exact typed fields before rendering or the exact canonical strings
after rendering, rejecting malformed or noncanonical text instead of normalizing it. This is
an in-memory/text rendering of existing identities, not a separate revision artifact or a
producer/authentication receipt.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-011-AC-1 | Over V2 selected claims, a QSL-produced `VersionUnchanged` (`Postcondition`) `Broad` campaign records `Holds` cases as `Passed` and `Violated` cases as `FailedPostcondition` and passes; a `Precondition` fixture records `Violated` cases as `RejectedPrecondition` in `rejected` and passes with zero proptest global rejects; the `amount < 7` `Invariant` records `Violated` cases as `FailedPostcondition`. | Test (TC-020) |
| FR-011-AC-2 | A V2 comparison oracle deliberately swapped for its negation fails each campaign with `ConformanceMismatch` carrying the case values, expected tag, and observed verdict kind. | Test (TC-020) |
| FR-011-AC-3 | The census runner evaluates each of the 10 `VersionUnchanged` in-domain census cases exactly once, in census order, and evaluates none of the out-of-domain cases. | Test (TC-020) |
| FR-011-AC-4 | A campaign over a report seeded with fixed prior counts returns `discard_rate() == Some((discarded, attempted))` and `rejection_rate() == Some((rejected, attempted))` with those exact values, a fresh zero-attempt summary returns `None` for both, and a runtime snapshot whose counters are `at_limit` returns `None` for both through the generated snapshot-summary operation. | Test (TC-020) |
| FR-011-AC-5 | A fresh 10,000-case `Broad` campaign for each fixture returns `discard_rate() == Some((0, attempted))` with `attempted > 0`, passes a zero discard ceiling, and does not end `Exhausted`. | Test (TC-020) |

## Open Questions

- OQ-1 (execution point): the runtime execution point (`initialization`, `handler`, `pre`, `post`)
  has no V2 source. IR's accessor returns only the clause kind (`Invariant`, `Precondition`,
  `Postcondition`), and IR main has no execution-point type, so an invariant could map to
  `initialization` or `handler`. No source is invented here. The planner is taking this to the
  owner; no FR-011 criterion depends on the answer.
- OQ-2 (resolved, requirement and revision identity): the selected QSL `RequirementItem` whose occurrence was accepted by
  `export_bindings` supplies its requirement occurrence key, including node, role and ordinal;
  the admitted `CheckedPackageV2::package_id()` supplies the revision identity. A rebinding of
  an abstraction relation changes the package id (QSL ADR-017 AR-4). QSL supplies no separate
  requirement-revision artifact, and CG creates none. This identity does not authenticate the
  producer/run/artifact association required by FR-004-AC-10 and AC-14.

## Dependencies

- **Upstream**: [FR-002](./FR-002-tristate-proptest.md) campaign policy and conclusions;
  quire-contract-runtime FR-001, FR-003, FR-004, and interface-001;
  [FR-008](./FR-008-bound-domain-strategy-admission.md) (the selected claim's V2 comparison
  oracle), [FR-009](./FR-009-constructive-correlated-populations.md),
  [FR-010](./FR-010-domain-boundary-campaigns.md).
- **Constrained by**:
  [NFR-004](../non-functional/NFR-004-constructive-generation-efficiency.md).
- **Downstream**: [FR-012](./FR-012-constraint-preserving-shrinking.md),
  [FR-013](./FR-013-it010-consumable-output.md),
  [TC-020](../matrix/TC-020-numeric-harness-campaigns.md).
