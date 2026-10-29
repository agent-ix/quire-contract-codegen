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
- Planned (IR-1072): a V2 selected-claim entry request holds one admitted `CheckedPackageV2` and
  the complete claim key: checked clause id, claim owner node id, and authentic `claim`
  `CheckedOccurrence`. The owner is part of that key; the caller supplies no second owner field.
- Planned (IR-1013, FR-058): one admitted `CheckedPackageV2`, the selected checked clause id, and
  that clause's authentic `claim` `CheckedOccurrence`. Together they select one accepted QSL
  `ClauseBinding.execution_point`; Contract IR's typed accessor returns its owned
  `ExecutionPoint` with an `ExecutionPointName` (variant and exact nonempty point name) for that
  package, clause, and occurrence. QSpec's V2 point-name wire domain admits every nonempty string;
  `ExecutionPointName` preserves that domain, while authored `AnchorName` identifier validation
  remains at its own boundary. QSL owns `ClauseBinding`; the
  caller supplies no separate point. The QSL driver supplies the FR-331 bound item for that selected
  `claim`. A process-provider FR-331 frame carries `request.package_id`, which its reader checks
  against the frame's canonical package bytes; CG compares that id with its separately admitted
  `CheckedPackageV2`. An in-process linked provider has no FR-331 package frame or independent
  owner field. The driver calls `export_bindings` on one S4 `CheckedPackage`, emits that same
  package at E4, then E5 admits those exact emitted bytes as the `CheckedPackageV2` it passes to
  CG; E5 checks the emitted and admitted ids. The linked CG input is this admitted package and its
  selected claim, so the package tie is by construction. CG still verifies the claim is authentic
  in that held package and does not substitute a sibling claim. No QSL facade, owner parameter,
  or FR-331 wire member is added.
- A caller-owned proptest `TestRunner` and `quire_contract_runtime::CampaignReport`.
- The existing campaign policy: minimum accepted, minimum rejected, and maximum discarded counts.
- Planned (IR-1124, after IR-1072): the typed V2 selected-claim result, the FR-008
  contextual comparison and V2 oracle for that same claim, and the requested
  constructive or census population. No V1 bound strategy output is an input.

## Outputs

- A generated population runner and a generated census runner.
- A generated summary type with the existing `attempted`, `accepted`, `rejected`, `failed`, and
  `discarded` fields, plus `discard_rate()` and `rejection_rate()`.
- The existing generated campaign error type, plus a `ConformanceMismatch` variant carrying the case
  values, the expected tag, and the observed verdict kind.
- For generation refusal, `StrategyErrorCode::MissingClaimOccurrence`,
  `StrategyErrorCode::MissingExecutionPoint`, `StrategyErrorCode::ExecutionPointMismatch`,
  `StrategyErrorCode::OwnerIdentityMismatch`, `StrategyErrorCode::UnboundModelElement`, or
  `StrategyErrorCode::InvalidIdentityRendering`, with no artifact.
  QSL `export_bindings` refusals remain with the driver.
- Planned (IR-1072): the selected-claim entry returns either a typed V2 selection for the exact
  supplied key or a distinct typed V2 selection diagnostic retaining Contract IR's refusal code,
  clause id, claim owner id, and occurrence. This entry alone emits no strategy bundle or runner.
- Planned (IR-1124): one complete V2 source, map, census and numeric runner bundle for
  the selected claim, or one typed diagnostic with zero published artifacts and no
  campaign, verdict or coverage result.

## Behavior

- When the selected V2 claim and its FR-008 comparison are admitted, CG shall construct
  the numeric population, census and oracle-conformance runner from that admitted
  checked-package selection and shall publish the complete generated bundle atomically.
  The V2 entry shall not call the V1 bound strategy generator or adapt V1 output.
- When publishing a V2 bundle, CG shall require the same selected package id, checked
  clause id, claim owner node id and authentic claim occurrence throughout selection,
  comparison, generated source/map and runner identity. CG shall require a complete
  selected comparison oracle, nonempty requested population, valid generated syntax,
  and the rendered point equal to the selected checked point.
- For a V2 numeric request, CG shall preserve FR-008's comparison-admission
  first-defect order: unknown selected clause; absent or inauthentic claim; IR
  accessor's remaining order, including unsupported clause kind; CG relation/domain;
  selected-oracle construction; then rendering. After a successful comparison
  admission, CG shall refuse a missing accepted point before population construction,
  and shall check population, identity, rendered point, generated syntax and resource
  limits before publication. The FR-011-AC-15 point-selection entry retains its
  own IR FR-058 refusal order when called alone; its diagnostic shall not replace
  an earlier FR-008 admission refusal in the numeric request. The FR-331 reader
  and driver retain their earlier package/owner and export refusals.
- The V2 diagnostic shall retain the selected clause, owner, occurrence,
  underlying typed IR cause and reached structural locus when available.
- The V2 numeric diagnostic shall map an unknown selected clause to
  StrategyErrorCode::UnknownClause; an absent or inauthentic claim or owner to
  MissingClaimOccurrence; an unsupported clause kind to UnsupportedClauseKind;
  an absent accepted point to MissingExecutionPoint; an unbound model element
  to UnboundModelElement; a noncanonical or mismatched identity to
  InvalidIdentityRendering or OwnerIdentityMismatch according to the owner
  comparison boundary; and a changed rendered point to ExecutionPointMismatch.
  CG shall preserve a distinct typed FR-008 cause and locus for other comparison
  refusals, and shall use the existing EmptyPopulation,
  UnsupportedCampaignConstraint, InvalidGeneratedSyntax and
  ResourceLimitExceeded codes for their corresponding post-admission failures.
- When a V2 generated campaign runs, CG shall keep its result, source/map bytes,
  selected claim identity and package revision associated with that runner invocation
  for FR-004 live producer binding. Publication alone shall convey no authenticated
  campaign, measured coverage or proof credit; only FR-004's same-invocation
  producer/profile/export association may qualify campaign evidence.
- If V2 selection or prepublication generation refuses, then CG shall publish nothing
  for that selection and shall not create a campaign result or offer that selection as
  evidence to the V2 analyzer. A valid sibling selection shall retain its own result.

- When the IR-1072 V2 selected-claim entry is called, that entry shall pass its complete claim key to
  `CheckedPackageV2::clause_execution_point(&clause_id, (&claim_owner_node_id,
  &claim_occurrence))` and retain the selected checked point only for that key. If Contract IR
  refuses this entry's lookup, then this entry shall preserve its refusal code and selected
  identities in a distinct typed V2 selection diagnostic without publishing an artifact. This
  entry shall not widen the V1 `StrategyDiagnostic` or infer the owner from the occurrence.
- The runner shall map an oracle result to a runtime verdict by the selected checked clause's kind
  (the clause kind IR's comparison accessor returns for the selected clause):
  - for a `Precondition` clause, `true` to `Passed` and `false` to `RejectedPrecondition`;
  - for a `Postcondition` clause, `true` to `Passed` and `false` to `FailedPostcondition`;
  - for an `Invariant` clause, `true` to `Passed` and `false` to `FailedPostcondition` whose clause
    observation carries the runtime's invariant clause kind, which quire-contract-runtime
    implements but does not yet specify.
- When a V2 claim is selected, the generator shall call the Contract IR FR-058 typed
  accessor on the admitted package with the checked clause id and selected owner-bound `claim`
  occurrence.
- The generator shall use only the accepted QSL binding point returned for that exact tuple.
- When generation is requested, CG shall require the selected FR-331 bound item's `claim`
  occurrence to be authentic for the checked clause in the admitted package and shall read every
  referenced model binding from that package's V2 abstraction relation. The QSL driver shall
  forward only items accepted by `export_bindings`; `UnknownItem`, `Unbound`, and `Fault` remain
  driver-side refusals. CG shall defend an out-of-driver request without calling
  `export_bindings` or trusting a bare item: an absent or non-claim occurrence returns
  `StrategyErrorCode::MissingClaimOccurrence`; a referenced model element absent from the admitted
  relation returns `StrategyErrorCode::UnboundModelElement` naming that element. Neither publishes
  an artifact.
- For a process-provider request, CG shall compare FR-331 `request.package_id` with the separately
  admitted `CheckedPackageV2::package_id()` over exact domain, algorithm, and digest fields. If any
  field differs, then CG shall return `StrategyErrorCode::OwnerIdentityMismatch` with no artifact
  or verdict. A request id unequal to its package bytes remains the FR-331 reader's
  `stale_dependency`/`content-mismatch` refusal and shall not be relabelled by CG. For linked
  generation, CG shall use the driver's one E5-admitted package; a foreign or non-claim occurrence
  absent from that package returns `StrategyErrorCode::MissingClaimOccurrence`, and E4/E5 identity
  disagreement stays the driver's handoff refusal. CG shall not substitute an occurrence or
  revision from a sibling package.
- If the accessor returns `UnknownClause`, then the generator shall return the existing
  `StrategyErrorCode::UnknownClause` and no artifact before checking occurrence authenticity.
- If the accessor returns `NotStateClause`, then the generator shall return the existing
  `StrategyErrorCode::UnsupportedClauseKind` and no artifact before checking occurrence
  authenticity.
- If the selected occurrence is not an authentic `claim` of that checked clause in that package,
  then the generator shall return `StrategyErrorCode::MissingClaimOccurrence` and no artifact.
- The generator shall not substitute another occurrence of the same checked clause node for the
  selected occurrence.
- If the selected package, clause, and authentic claim occurrence has no accepted binding point,
  then the generator shall return
  `StrategyErrorCode::MissingExecutionPoint` and no artifact.
- If the rendered runner's point differs from the point returned for the selected claim occurrence,
  then the generator shall return `StrategyErrorCode::ExecutionPointMismatch` and no artifact before
  publication.
- The generator shall not derive an execution point from clause kind.
- The generator shall render the accessor's `ExecutionPoint` into the runtime string injectively as
  `<variant>:<utf8-byte-length>:<point-name>`, where variant is exactly `initialization`,
  `handler`, `pre`, or `post` and the length counts the exact nonempty point name's UTF-8 bytes.
  CG shall preserve every admitted name byte, including names that are not identifiers.
- CG shall use the returned `ExecutionPoint` without adding a fallback point.
- The runner shall construct each verdict through the runtime `construct_verdict` operation
  (quire-contract-runtime interface-001) with the selected driver-bound FR-331 claim's complete
  occurrence key as requirement identity, the same admitted
  `CheckedPackageV2::package_id()` as revision identity, the runtime execution point rendered from
  the selected occurrence's accepted binding, and the clause observations. The runner shall retain
  the occurrence's node, role, and ordinal and the package id's domain, algorithm, and digest via
  the canonical V2 rendering below. CG shall create no separate requirement-revision artifact.
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

For the driver's selected FR-331 bound `claim`, CG shall render its `OccurrenceKey` as
`qocc2_<node>_<role-byte-count>_<role-hex>_<ordinal>`. `<node>` is the `WireNodeId`'s exactly 64
lowercase hexadecimal digits. `<role-hex>` is the lowercase hexadecimal encoding of every UTF-8
byte of `occurrence.origin().role().as_str()`, with two digits per byte; `<role-byte-count>` is the
UTF-8 byte length in base-10 without leading zeroes except `0`. `<ordinal>` is the occurrence
ordinal in base-10 without leading zeroes except `0`.

CG shall render the admitted `CheckedPackageV2::package_id()` as
`qpkg2_<domain-byte-count>_<domain-hex>_<algorithm-byte-count>_<algorithm-hex>_<digest-byte-count>_<digest-hex>`.
Each count and hex field follows the role rule, using the exact UTF-8 bytes of the
`CheckedSemanticId`'s `domain`, `algorithm`, and `digest` fields in that order. These encodings
are injective and reversible. CG shall not truncate, case-fold, or substitute any field, compute
a new identity digest, or treat the digest field alone as the revision. `qocc2_` and `qpkg2_`
separate the two identity domains. The rendered strings are the runtime `RequirementId` and `RevisionId` values. Generated headers and source maps shall carry
these same strings; generated symbol identity suffixes shall be
`cg_<qocc2-string>_<qpkg2-string>`, where both components follow the grammar above.

Comparisons shall use exact typed fields before rendering or exact canonical strings after
rendering. If an identity text comparison boundary receives malformed or noncanonical text, then
CG shall return `StrategyErrorCode::InvalidIdentityRendering` before bundle publication,
verdict construction, or report mutation; CG shall not normalize it. This rendering does not authenticate the same-run
producer and artifact association required by FR-004.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-011-AC-1 | PLANNED (IR-364). Over V2 selected claims, a QSL-produced `VersionUnchanged` (`Postcondition`) `Broad` campaign records `Holds` cases as `Passed` and `Violated` cases as `FailedPostcondition` and passes; a `Precondition` fixture records `Violated` cases as `RejectedPrecondition` in `rejected` and passes with zero proptest global rejects; the `amount < 7` `Invariant` records `Violated` cases as `FailedPostcondition`. | Test (TC-020) |
| FR-011-AC-2 | PLANNED (IR-364). A V2 comparison oracle deliberately swapped for its negation fails each campaign with `ConformanceMismatch` carrying the case values, expected tag, and observed verdict kind. | Test (TC-020) |
| FR-011-AC-3 | PLANNED (IR-364). The census runner evaluates each of the 10 `VersionUnchanged` in-domain census cases exactly once, in census order, and evaluates none of the out-of-domain cases. | Test (TC-020) |
| FR-011-AC-4 | PLANNED (IR-364). A campaign over a report seeded with fixed prior counts returns `discard_rate() == Some((discarded, attempted))` and `rejection_rate() == Some((rejected, attempted))` with those exact values, a fresh zero-attempt summary returns `None` for both, and a runtime snapshot whose counters are `at_limit` returns `None` for both through the generated snapshot-summary operation. | Test (TC-020) |
| FR-011-AC-5 | PLANNED (IR-364). A fresh 10,000-case `Broad` campaign for each fixture returns `discard_rate() == Some((0, attempted))` with `attempted > 0`, passes a zero discard ceiling, and does not end `Exhausted`. | Test (TC-020) |
| FR-011-AC-6 | SUPERSEDED by FR-011-AC-14 under QSpec #449 / IR-1070; no implementation credit. Original planned text: For a selected checked clause id and authentic `claim` occurrence in one admitted V2 package, every constructed verdict carries the runtime point rendered from the accepted QSL `ClauseBinding.execution_point` of that exact tuple, with variant and anchor name preserved. Invariant fixtures at both initialization and handler anchors, and two distinct handler anchors, produce distinct runtime point strings; `Precondition` and `Postcondition` fixtures preserve their own binding points. Two authentic claim occurrences on one invariant node with identical clause kind and body anchor but different accepted points yield their distinct rendered points. The last case is infeasible because the point contributes to node identity (QSpec FR-322-AC-131, FR-341-AC-14). | Test (TC-020) |
| FR-011-AC-7 | SUPERSEDED by FR-011-AC-15 under QSpec #449 / IR-1070; no implementation credit. Original planned text: An absent checked clause maps Contract IR FR-058 `UnknownClause` to `StrategyErrorCode::UnknownClause`; a non-state-clause maps `NotStateClause` to `StrategyErrorCode::UnsupportedClauseKind`, before occurrence checks and with no artifact. A wrong-role, wrong-ordinal, or other-node `claim` occurrence returns `StrategyErrorCode::MissingClaimOccurrence` and no artifact, even when another authentic occurrence of the selected node has a point. An authentic selected tuple lacking an accepted binding point returns `StrategyErrorCode::MissingExecutionPoint` and no artifact, even when another authentic occurrence of that node has a point. A render mutant that changes only the generated runner's execution point returns `StrategyErrorCode::ExecutionPointMismatch` and no artifact before publication. An invariant's clause kind supplies no point: two otherwise equivalent invariant bindings at distinct anchors produce their distinct bound runtime points. The pointed/unpointed sibling case is infeasible because an optional point is a node member, not an occurrence sidecar (QSpec FR-322-AC-131, FR-341-AC-14). | Test (TC-020) |
| FR-011-AC-8 | PLANNED. The numeric runner's `RequirementId`, `RevisionId`, generated header, source map, and symbol suffix retain the selected driver-bound FR-331 claim's complete occurrence and the same-owner admitted package's full id through exact `qocc2_`/`qpkg2_` renderings. Changing one occurrence component or package-id field changes the corresponding text. | Test (TC-020) |
| FR-011-AC-9 | PLANNED. For a process request, if FR-331 `request.package_id` differs from the separately admitted Contract IR package's full domain, algorithm, or digest, CG returns `StrategyErrorCode::OwnerIdentityMismatch` before any bundle, verdict, or report mutation, while an unrelated matched selection retains its disposition. A package-byte/id mismatch stays the FR-331 reader's `stale_dependency`/`content-mismatch` refusal. For a linked request, the driver exports, emits, and admits one package; an E4/E5 id mismatch stays the driver's handoff refusal, and CG verifies the selected claim belongs to its held admitted package without a second owner field. | Test (TC-020) |
| FR-011-AC-10 | PLANNED. The QSL driver forwards only FR-331 items accepted by `export_bindings`; its `UnknownItem`, `Unbound`, and `Fault` results stay driver-side. An out-of-driver non-claim or absent occurrence returns CG `StrategyErrorCode::MissingClaimOccurrence`, and a selected claim with an unbound referenced model element returns `StrategyErrorCode::UnboundModelElement` naming it. Neither produces an artifact or verdict, and a matched sibling remains unaffected. | Test (TC-020) |
| FR-011-AC-11 | PLANNED. Malformed, truncated, case-folded, or otherwise noncanonical `qocc2_`/`qpkg2_` text yields `StrategyErrorCode::InvalidIdentityRendering` before bundle publication or verdict/report mutation, without normalization; exact typed or canonical-text comparisons agree on the accepted pair. | Test (TC-020) |
| FR-011-AC-12 | PLANNED. Repeated numeric runner generation with the same admitted package, selected driver-bound FR-331 claim, checked clause, and campaign policy yields byte-identical source and map. | Test (TC-020) |
| FR-011-AC-13 | PLANNED (IR-1072). The public V2 selected-claim entry accepts an admitted `CheckedPackageV2` and a typed complete claim key (checked clause id, claim owner node id, authentic `claim` `CheckedOccurrence`) without a second caller-supplied owner; a matching key selects only that claim and its exact checked point. With the same clause and occurrence but a different claim owner, the entry returns a distinct typed V2 diagnostic retaining Contract IR `CheckedExecutionPointRefusal::MissingClaimOccurrence` and the supplied clause, owner, and occurrence, and publishes no artifact. The V1 `BoundPackage` entry and `StrategyDiagnostic` are unchanged. | Test |
| FR-011-AC-14 | PLANNED (IR-1070). For selected authentic `claim` occurrences of distinct V2 invariant clause nodes whose condition and anchor are equal but whose accepted points are respectively `initialization{name: "startup"}` and `handler{name: "tick"}`, each constructed verdict carries the exact point of its selected node through the FR-058 accessor and injective runtime rendering. The nodes and otherwise identical packages have distinct identities; changing only the accepted handler name changes its rederived node and package identities and runtime point. Precondition and postcondition claims preserve their accepted `pre` and `post` points, including their operation names (QSpec FR-322-AC-131, FR-341-AC-14, FR-342-AC-10; IR FR-058-AC-9). Two authentic claims of one pointed node return its one identical point; no claim occurrence supplies a second point. | Test (TC-020) |
| FR-011-AC-15 | PLANNED (IR-1070). An absent checked clause returns `StrategyErrorCode::UnknownClause`, a non-state clause returns `StrategyErrorCode::UnsupportedClauseKind`, and a wrong-role, wrong-ordinal, or other-node occurrence returns `StrategyErrorCode::MissingClaimOccurrence`, each before artifact publication and without substituting another claim or node's point. An authentic claim of a separate V2 clause node whose optional point is absent returns `StrategyErrorCode::MissingExecutionPoint` without borrowing a pointed sibling node's point. A renderer mutant changing only the selected point returns `StrategyErrorCode::ExecutionPointMismatch` before publication. Two authentic claims of one pointed node use the same point; clause kind and anchor never synthesize a missing point. | Test (TC-020) |
| FR-011-AC-16 | PLANNED (IR-1124; depends on IR-1072 and FR-008). A public V2 strategy request over an admitted checked package and one authentic selected claim produces a complete source, map, census and numeric runner bundle whose selected clause, owner, occurrence, package revision, comparison and runner identities agree exactly. The runner executes the FR-009 constructive population and FR-010 census with the selected V2 oracle; source inspection finds no V1 bound package, clause reference, bound strategy generator call or adapter in this V2 path. | Test |
| FR-011-AC-17 | PLANNED (IR-1124). Independently injected unknown clause, unsupported kind, wrong owner/role/ordinal, missing accepted point, FR-008 comparison refusal, empty population, identity mismatch, point-render mutant, invalid generated syntax and resource overflow each return a typed diagnostic with the selected key and available structural locus, publish zero artifacts and start no campaign. Pairwise overlapping defects return the first cause in Behavior order; a valid sibling still publishes its complete bundle. | Test |
| FR-011-AC-18 | PLANNED (IR-1124; FR-004 live binding remains separate). A campaign run from the emitted V2 runner retains the selected claim and package identity with its own source/map and result for same-invocation producer binding. Published bytes or a detached/copied result alone yield no authenticated campaign, measured coverage or proof credit. A refused selection yields no campaign evidence, including when another claim of the same package succeeds. | Test |

## Open Questions

- OQ-2 (resolved, requirement and revision identity): the QSL driver's selected FR-331 bound
  `claim` supplies its authentic occurrence key; after process-owner comparison or linked
  single-package admission, the same admitted `CheckedPackageV2::package_id()` supplies the revision. A rebinding of the V2
  abstraction relation changes the package id (QSL ADR-017 AR-4). QSL supplies no separate
  requirement-revision artifact, and CG creates none. These identities do not authenticate the
  producer/run/artifact association of FR-004-AC-10 and AC-14.

## Dependencies

- **Upstream**: [FR-002](./FR-002-tristate-proptest.md) campaign policy and conclusions;
  quire-contract-runtime FR-001, FR-003, FR-004, and interface-001;
  QSL-1082 (accepted binding point on the V2 carrier; planned prerequisite);
  IR-1013 / Contract IR FR-058 (typed accessor for the accepted QSL binding point keyed by admitted
  package, checked clause id, and authentic claim occurrence; planned,
  blocks this V2 execution-point slice);
  [FR-008](./FR-008-bound-domain-strategy-admission.md) (the selected claim's V2 comparison
  oracle), [FR-009](./FR-009-constructive-correlated-populations.md),
  [FR-010](./FR-010-domain-boundary-campaigns.md).
- **Constrained by**:
  [NFR-004](../non-functional/NFR-004-constructive-generation-efficiency.md).
- **Downstream**: [FR-012](./FR-012-constraint-preserving-shrinking.md),
  [FR-013](./FR-013-it010-consumable-output.md),
  [TC-020](../matrix/TC-020-numeric-harness-campaigns.md).
- **Driver boundary**: QSL ADR-017 AR-4 and FR-307 put `export_bindings` after S4 and before
  FR-331 request construction. The linked driver exports and emits the same S4 package, then
  retains E5's admission of those bytes; the process frame carries a reader-validated package id.
  CG receives the bound claim, checks the admitted package's occurrence and V2 relation, and on
  the process route compares that frame id; it adds no QSL export facade or copied relation type.
