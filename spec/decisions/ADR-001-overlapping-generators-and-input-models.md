---
id: ADR-001
title: "Overlapping Kani generators, oracle generators and input models"
type: ADR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AD-001
    type: relates_to
  - target: ix://agent-ix/quire-contract-codegen/FR-001
    type: relates_to
  - target: ix://agent-ix/quire-contract-codegen/FR-003
    type: relates_to
  - target: ix://agent-ix/quire-contract-codegen/FR-007
    type: relates_to
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: relates_to
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: relates_to
---
# ADR-001: Overlapping Kani generators, oracle generators and input models

## Status

Proposed. Each question below is open until the owner rules. The recommendations are the author's
and decide nothing.

## Context

The generator has three Kani generation requirements, two scalar oracle generators and two input
models. Each pair or triple overlaps in what it claims. Each one is specified, implemented and
tested on its own, so no gate reports the overlap. This record states each overlap as measured at
this revision.

| Surface | Requirement | Source | Input | Profile or schema |
|---|---|---|---|---|
| Boolean and numeric/state Kani lowering | [FR-003](../functional/FR-003-kani-lowering.md) | `src/kani.rs` | V1 `BoundPackage` clauses | `kani-0.67.0-function-contracts-v2`, caller-supplied solver, optional stubbing, proof graph with `not_run` |
| Bounded Kani profile corpus | [FR-007](../functional/FR-007-bounded-kani-profile-corpus.md) | `src/bounded_kani_corpus.rs`, `src/bounded_kani_profile.rs`, `src/bounded_kani_replay.rs` | Contract IR `kani-bounded/1` profile | Contract IR `KaniOutcome` and `ReplaySource`, native replay through an injected closure |
| Separate complete-V1 obligations | [FR-015](../functional/complete-v1/FR-015-bounded-kani-obligations.md) | `src/kani_obligations.rs` | `ObligationItem::BoundClause` over `BoundPackage` and `ObligationItem::ScalarClaim` over `CheckedPackageV2` | `kani-0.67.0-separate-obligations-v1`, `cadical`, no stubbing |
| V1 oracles | [FR-001](../functional/FR-001-deterministic-oracles.md) | `src/oracle.rs` | V1 `BoundPackage` | Boolean and bounded-integer comparison grammar |
| Exact scalar oracles | [FR-014](../functional/complete-v1/FR-014-exact-scalar-oracles.md) | `src/exact_scalar.rs` | `CheckedPackageV2` | complete-V1 exact scalar families through `quire_contract_runtime::exact` |

FR-015 is the Kani arm that routed generation calls (FR-022). Its harnesses embed FR-014 oracles
for `ScalarClaim` items and FR-001 clause oracles for `BoundClause` items.

## Decision

Nothing is decided here. The owner rules on each question below.

### Q1: Which Kani generation requirement is the Kani backend's generator?

Options:

1. **FR-015 only.** FR-003's function-contract lowering and FR-007's corpus are withdrawn. Any
   behaviour of theirs that FR-015 lacks moves into FR-015 as new criteria first, for example the
   FR-003 post-state result binding and the FR-007 finite object, reference, graph and collection
   constructs.
2. **FR-015 plus FR-007.** FR-003 is withdrawn. FR-007 stays as a separate corpus generator over
   the Contract IR profile.
3. **All three stay.** Each gets a stated, disjoint domain, and FR-022 names which one each routed
   kind reaches.

Recommendation: option 1. FR-015 is the only generator that routed generation calls, that runs
under FR-017's pinned execution and that feeds the QSL replay facade. FR-007's replay path runs
native replay through a caller-supplied closure (`src/bounded_kani_replay.rs`), which QSL ADR-011
FB-07 does not count as replay evidence. FR-003's optional stubbing flag contradicts FR-015's
no-stubbing rule for the same backend. Carrying the unique FR-003 and FR-007 behaviours into FR-015
first keeps the working path until its replacement works.

### Q2: Which oracle generator owns the Boolean and bounded-integer comparison family?

Options:

1. **FR-014 only.** FR-001's comparison grammar is expressed as FR-014 claims over
   `CheckedPackageV2`, and FR-001 keeps only its attestation and source-map obligations.
2. **Both, split by input model.** FR-001 serves the V1 package and FR-014 serves the V2 package.
   This decision follows Q3.
3. **Both, split by family.** FR-001 keeps the comparison grammar for every input, and FR-014
   refuses it.

Recommendation: option 2 now, becoming option 1 when Q3 retires the V1 package. The split by input
model is what the code does today. Deciding Q3 then decides this question.

### Q3: Does the V1 `BoundPackage` input model stay beside `CheckedPackageV2`?

Options:

1. **Retire the V1 model.** Every generator reads `CheckedPackageV2`. The `BoundClause` arm of
   `ObligationItem`, FR-001's and FR-003's V1 inputs and FR-007's profile input move to V2 or are
   withdrawn.
2. **Keep both, with a stated boundary.** The V1 model serves the FR-008 to FR-013 strategy
   campaigns and the FR-003 proof graph, and the V2 model serves every complete-V1 surface. No
   single request mixes them. FR-015 already refuses a mixed V1 request as `MixedBoundPackages`
   and would refuse a mixed V1/V2 request too.
3. **Keep both, unbounded.** Any generator may read either model.

Recommendation: option 1, done in stages so that each V1 path is removed only after its V2
replacement is green. `CheckedPackageV2` is the model QSL emits and the one the replay facade
recompiles against, so a V1-only obligation cannot be replayed through QSL at all. Option 3
recommends nothing, because it leaves the overlap as it is.

## Consequences

Until the owner rules, each overlapping requirement keeps its own criteria and coverage rows, and
no new criterion is added to more than one of them. A ruling becomes spec changes to the named
requirements and the test matrix, and code follows the spec.

## Alternatives Considered

- **Leave the overlaps implicit.** Each path is green on its own, and the overlap stays invisible
  to every gate. Rejected, because an unstated overlap is how FR-003's optional stubbing and
  FR-015's no-stubbing rule ended up applying to the same backend.
