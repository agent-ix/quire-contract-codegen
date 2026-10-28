---
id: TC-020
title: "Verify numeric conformance campaigns and rate reporting"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-011
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/FR-004
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/NFR-004
    type: verifies
---
# TC-020: Verify numeric conformance campaigns and rate reporting

## Description

Verify that the generated conformance runner maps oracle results to verdicts by clause kind, records
them without global rejects, runs the census exhaustively, detects oracle disagreement, and reports
exact rates.

## Test Procedure

1. In a generated-crate fixture built from IR-admitted `CheckedPackageV2` selected claims, run `Satisfying`, `Violating`, and `Broad` campaigns through the
   generated population runner for `VersionUnchanged` (`Postcondition`), `amount < 7` (`Invariant`),
   and a `Precondition` fixture `amount <= 500`, each with a fresh report, at 256 cases under the
   deterministic configuration with `max_global_rejects: 0` and at 10,000 cases.
2. Repeat step 1's 256-case `Broad` campaigns with each selected claim's V2 comparison oracle
   replaced by its negation.
3. Run the census runner for `VersionUnchanged` with an instrumented oracle that records every
   valuation it receives.
4. Run a campaign over a report seeded with fixed prior accepted, rejected, and discarded counts,
   then read both rates; read both rates on a fresh zero-attempt summary and on a runtime snapshot
   decoded with counters at `u64::MAX`. The runtime deliberately exposes no conversion from
   an imported snapshot back to a mutable report, so the generated snapshot-summary operation is
   the public boundary for this saturated-rate check.
5. Read the runner's proptest global reject count and each campaign conclusion from steps 1 and 4.
6. Run a generated campaign with a report whose requirement identity is wrong and whose prior
   discarded count already exceeds the configured ceiling.
7. Generate from accepted QSL bindings at distinct typed points: one invariant at an initialization
   anchor, two otherwise equivalent invariants at different handler anchors, and precondition and
   postcondition bindings. Instrument `construct_verdict` and compare each runtime string with the
   source binding's variant, UTF-8 anchor name and exact byte-length rendering. Keep clause kind
   fixed while changing only the invariant binding's anchor, then compare the runtime strings.
   Use one admitted package with one invariant checked clause node and two authentic `claim`
   occurrences bound to distinct initialization and handler points. Select each exact `(checked
   clause id, claim occurrence)` in turn and compare its constructed verdict point with that
   occurrence's accepted binding; also vary the handler name on a third authentic claim.
8. Independently select a wrong-role, wrong-ordinal, and other-node occurrence against a checked
   clause with another authentic claim that has a point; verify that the sibling point cannot satisfy
   the lookup. Supply an authentic claim lacking its accepted point beside another authentic claim
   of the same node that has a point; inspect the generation refusal with no sibling substitution.
   Also select an absent clause id and a present non-state-clause id to inspect the mapped Contract IR
   FR-058 refusals before occurrence checking. Independently apply a renderer mutant that changes only the generated
   runner's point after binding selection; inspect the refusal before artifact publication.
9. From a QSL driver-bound FR-331 claim and matching admitted Contract IR package, generate a
   numeric runner twice and compare source and map bytes. Inspect runtime `RequirementId` and
   `RevisionId`, header, source map, and symbol suffix against FR-011's `qocc2_`/`qpkg2_` grammar.
   Independently change the authentic occurrence's node, role, and ordinal and each package-id
   field using separate accepted matching request/package pairs; compare identity text.
10. For the process form, pair a bound FR-331 request item's `request.package_id` with a different
    admitted Contract IR package, then vary only its domain, algorithm, or digest. Separately keep
    a linked driver's E4 emitted package id with its `export_bindings`-accepted item and E5
    admission from the same emitted bytes. Attempt an E4/E5 id mismatch and verify the driver
    refuses before CG. Supply a foreign claim occurrence absent from the held linked package and
    verify CG refuses it as `StrategyErrorCode::MissingClaimOccurrence`; CG has no separate linked
    owner to compare. Verify also that a process package-byte/id mismatch is refused by the
    FR-331 reader before CG. Form out-of-driver requests with a missing or non-claim occurrence and with a selected claim whose referenced model element is
    absent from the admitted V2 relation. Inspect refusal, artifact output, verdict construction,
    and report counters; keep a matched sibling selection in the batch. Verify separately that QSL
    keeps `UnknownItem`, `Unbound`, and `Fault` export refusals on the driver side.
11. At each generated identity text comparison boundary, corrupt one `qocc2_` or `qpkg2_` field,
    truncate it, uppercase hex, or add a leading-zero count. Compare against the exact typed pair
    and inspect any artifact, verdict, or report mutation.
12. Independently of runner generation, call the public V2 selected-claim entry with one admitted
    package containing two distinct checked state clauses, each with an authentic `claim`
    occurrence and a different accepted point. Supply each complete typed key in turn: checked
    clause id, claim owner node id, and that clause's authentic occurrence. Compare each selected
    point with its own accepted point and the other clause's different point. For one selection,
    replace only the occurrence with the other clause's claim while retaining the correct owner;
    then restore the occurrence and replace only the owner with the other clause's node id. Inspect
    both typed V2 diagnostics, their retained identities and Contract IR refusals, and artifact
    output. Inspect the entry signature for a second owner field and the V1 `BoundPackage` entry
    and `StrategyDiagnostic` for changes.

## Expected Results

- `VersionUnchanged` records `Holds` as `Passed` and `Violated` as `FailedPostcondition`; the
  `Precondition` fixture records `Violated` as `RejectedPrecondition` in `rejected`; `amount < 7`
  records `Violated` as `FailedPostcondition` with an `Invariant` observation; every campaign passes.
- Each negated-oracle campaign fails with `ConformanceMismatch` carrying the case values, expected
  tag, and observed verdict kind.
- The instrumented oracle receives exactly the 10 in-domain census cases, once each, in census order,
  and no out-of-domain case.
- The rates equal the seeded `(discarded, attempted)` and `(rejected, attempted)`, and both are `None`
  at zero attempts and at `at_limit`.
- Every fresh campaign in step 1 reports `discard_rate() == Some((0, attempted))` with
  `attempted > 0`, passes a zero discard ceiling, records zero global rejects, and does not end
  `Exhausted`.
- The report in step 6 returns `IdentityMismatch`; its prior discard does not mask the identity
  failure as `AboveDiscardCeiling`.
- Step 7 preserves each accepted binding point through injective runtime rendering, including both
  invariant variants and both distinct handler names. Changing only the invariant's bound anchor
  changes the runtime point while its clause kind remains `Invariant`. The same checked node's two
  authentic claim occurrences produce distinct points with no node-level reuse.
- Each wrong occurrence in step 8 returns `StrategyErrorCode::MissingClaimOccurrence` with no
  artifact even when a sibling claim has a point. The authentic claim lacking a point returns
  `StrategyErrorCode::MissingExecutionPoint` despite its pointed sibling. An absent clause id returns
  `StrategyErrorCode::UnknownClause` and a non-state-clause returns
  `StrategyErrorCode::UnsupportedClauseKind`, each before occurrence checking and without an
  artifact. The renderer mutant returns `StrategyErrorCode::ExecutionPointMismatch` before
  publication and produces no artifact. Clause kind supplies no default point.
- Step 9 yields byte-identical output for unchanged inputs (FR-011-AC-12) and exact, lossless
  canonical text in every named location; changing one typed field changes its rendered identity
  (FR-011-AC-8).
- Step 10 returns `StrategyErrorCode::OwnerIdentityMismatch` for each unequal process FR-331/IR
  owner; upstream process and linked handoff mismatches retain their own refusals. The foreign
  linked claim and other out-of-driver cases return `StrategyErrorCode::MissingClaimOccurrence` for an absent or non-claim occurrence, and
  `StrategyErrorCode::UnboundModelElement` naming the absent referenced element. Each refused
  selection produces no bundle, verdict, or report mutation while the matched sibling retains its disposition. QSL
  export refusals never enter CG (FR-011-AC-9 and AC-10).
- Step 11 returns `StrategyErrorCode::InvalidIdentityRendering` before bundle publication or
  verdict/report mutation without normalizing malformed or noncanonical text; canonical text and typed identity agree
  (FR-011-AC-11).
- Step 12 selects each matching claim's exact `ExecutionPoint`, never the other clause's different
  point. Both the swapped occurrence and wrong owner return distinct typed V2 diagnostics retaining
  `CheckedExecutionPointRefusal::MissingClaimOccurrence`, each supplied clause, owner and occurrence,
  and no artifact. The request has no second caller-supplied owner;
  the V1 entry and diagnostic remain unchanged (FR-011-AC-13).
