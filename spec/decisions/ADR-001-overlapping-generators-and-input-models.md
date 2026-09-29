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

Accepted. The owner ruled on Q1, Q2 and Q3 as the Decision states.

## Context

The generator had three Kani generation requirements, two scalar oracle generators and two input
models. Each pair or triple overlapped in what it claimed. Each was specified, implemented and
tested on its own, so no gate reported the overlap. The table states each overlap as it was
measured when the question was raised.

| Surface | Requirement | Source | Input | Profile or schema |
|---|---|---|---|---|
| Boolean and numeric/state Kani lowering | [FR-003](../functional/FR-003-kani-lowering.md) | `src/kani.rs` | V1 `BoundPackage` clauses | caller-supplied solver, optional stubbing, proof graph with `not_run` |
| Bounded Kani profile corpus | [FR-007](../functional/FR-007-bounded-kani-profile-corpus.md) | `src/bounded_kani_corpus.rs`, `src/bounded_kani_profile.rs`, `src/bounded_kani_replay.rs` | Contract IR `kani-bounded/1` profile | Contract IR `KaniOutcome` and `ReplaySource`, native replay through an injected closure |
| Separate complete-V1 obligations | [FR-015](../functional/complete-v1/FR-015-bounded-kani-obligations.md) | `src/kani_obligations.rs` | `ObligationItem::BoundClause` over `BoundPackage` and `ObligationItem::ScalarClaim` over `CheckedPackageV2` | `cadical`, no stubbing |
| V1 oracles | [FR-001](../functional/FR-001-deterministic-oracles.md) | `src/oracle.rs` | V1 `BoundPackage` | Boolean and bounded-integer comparison grammar |
| Exact scalar oracles | [FR-014](../functional/complete-v1/FR-014-exact-scalar-oracles.md) | `src/exact_scalar.rs` | `CheckedPackageV2` | complete-V1 exact scalar families through `quire_contract_runtime::exact` |

FR-015 is the Kani arm that routed generation calls (FR-022). Its harnesses embed FR-014 oracles
for `ScalarClaim` items and FR-001 clause oracles for `BoundClause` items.

The evidence that decided each question:

- FR-015 is the only Kani generator that routed generation calls, that runs under FR-017's
  execution and that feeds QSL's replay facade.
- FR-007 replays through a caller-supplied closure (`src/bounded_kani_replay.rs`), which QSL
  ADR-011 FB-07 does not count as replay evidence.
- FR-003's optional stubbing flag contradicts FR-015's no-stubbing rule for the same backend.
- `CheckedPackageV2` is the model QSL emits and the one QSL's replay facade recompiles against, so
  an obligation over the V1 model cannot be replayed through QSL.

## Decision

### Q1: FR-015 is the Kani backend's one generator

FR-015 is the only Kani generation requirement. FR-003 and FR-007 are retired, and every
behaviour of theirs that FR-015 lacked is carried into FR-015 as its own criterion:

- from FR-003: the bounded state transition, with its state bound by `&mut` reference as ADR-004 Q2
  decides; the plain comparison falsified through concrete playback; the post-state-in-precondition
  refusal; the proof-dependency census and its readiness; and embedding the byte-identical oracle;
- from FR-007: one disposition per construct with its source identity kept, the case-derived proof
  symbol and the declared-census validation.

FR-003's caller-supplied solver and its optional stubbing are not carried. FR-015 lowers against
`cadical`, and FR-015-AC-9's ban on every stubbing option stands. FR-007's replay through an
injected closure is not carried. A counterexample replays only through QSL's replay facade
([FR-024](../functional/complete-v1/FR-024-counterexample-envelope-intake.md)).

Each retired requirement keeps its file and its criterion numbers, marked retired, with each
criterion naming the criterion that now carries it.

### Q2: FR-014 is the one oracle generator for the families it covers

FR-014 generates the oracle of every family it covers, the Boolean connectives and the
bounded-integer comparisons included. FR-001, the V1 oracle generator, is retired with the V1
model. FR-001's coverage-probe source map is carried into FR-014 as FR-014's own criterion.

### Q3: `CheckedPackageV2` is the one input model

The V1 `BoundPackage` input model is retired. Every generator reads an admitted
`quire.checked-package/v2` package through Contract IR's `CheckedPackageV2`. The generator has no
second input path and no adapter between the two models. An obligation that exists only over the
V1 model cannot be replayed through QSL, and it is not kept.

## Consequences

- FR-001, FR-003 and FR-007 are retired. Their criteria keep their numbers, and the test matrix
  marks each retired row with the criterion that carries it.
- FR-014 and FR-015 gain the carried criteria. Each one stays planned until code and a test back it.
- Code replaces each V1 path with its V2 equivalent and deletes the V1 path in the same change, so
  that each step leaves the repository green.
- FR-002, FR-004, FR-005, FR-008 to FR-013, NFR-004 and interface-001 still state the V1
  `BoundPackage` as their input, and their test cases (TC-004, TC-006, TC-017 to TC-022, and TC-028)
  build V1 fixtures. Those texts contradict this decision until a separate spec change
  restates them over `CheckedPackageV2`. Until then their matrix rows record coverage of their
  current V1 text only.

## Alternatives Considered

- **FR-015 plus FR-007.** FR-007 would stay a separate corpus generator over the Contract IR
  profile. Rejected, because FR-007's replay runs through an injected closure and its input is not
  the model QSL replays against.
- **All three Kani generators stay, each with a disjoint domain.** Rejected, because two of them
  read a model QSL cannot replay, and FR-003's stubbing contradicts FR-015's ban.
- **Two oracle generators, split by input model or by family.** Rejected, because one input model
  leaves nothing to split by model, and a split by family keeps two generators for one family set.
- **Keep both input models, with or without a stated boundary.** Rejected, because every surface
  the V1 model serves has a V2 equivalent, and a V1-only obligation cannot replay through QSL.
- **Leave the overlaps implicit.** Rejected, because an unstated overlap is how FR-003's optional
  stubbing and FR-015's no-stubbing rule ended up applying to the same backend.
