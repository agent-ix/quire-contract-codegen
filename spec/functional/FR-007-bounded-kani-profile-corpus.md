---
id: FR-007
title: "Generate the bounded Kani profile corpus"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-002
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-003
    type: depends_on
  - target: ix://agent-ix/quire-contract-ir/FR-029
    type: references
  - target: ix://agent-ix/quire-contract-ir/FR-030
    type: references
  - target: ix://agent-ix/quire-contract-ir/FR-031
    type: references
---
# FR-007: Generate the bounded Kani profile corpus

## Description

When a caller selects the reviewed `kani-bounded/1` Contract IR profile, the
code generator shall derive the complete cross-backend corpus for every
selected definedness/arithmetic, finite object/reference/graph, and bounded
collection/query construct without giving Contract IR a dependency on codegen.

## Inputs

- A public, versioned Contract IR bounded-Kani profile, finite input ABI,
  capability disposition, typed outcome, and provenance selection.
- A source-linked executable clause and the exact finite model-domain,
  snapshot, population, invocation, and resource bounds it selects.
- The pinned cargo-kani backend identity, executable digest, solver/options,
  customer subject binding, and declared proof dependency census.
- A reviewed public Contract IR revision that exposes the selected profile
  without a Contract IR dependency on codegen.

## Outputs

- Deterministic executable-oracle, generated strategy, Kani harness, proof
  dependency graph, and corpus-case artifacts for supported constructs.
- A source-linked typed refusal or inconclusive result with no generated
  partial artifact for every non-supported construct or invalid finite input.
- Concrete counterexample packets that can be replayed by Contract IR's native
  runtime boundary.

## Behavior

- When code generation begins, the generator shall select exactly one supported,
  refused, or inconclusive disposition for every encountered construct from
  the supplied profile.
- When a disposition is supported, the generator shall derive oracle,
  strategy, and Kani artifacts from the same validated finite ABI input and
  retain the selected profile revision, bounds, Kani version, executable
  digest, options, assumptions, and proof dependencies in every artifact
  identity.
- When the generator emits a Kani harness for a supported corpus case, the
  generator shall derive the harness's `#[kani::proof]` symbol from the case's
  own identity digest rather than from its semantic family alone, carrying the
  same identity the case's artifact paths carry.
- When a finite population, snapshot, reference, collection, or invocation is
  malformed, incomplete, unavailable, or over its selected bound, the
  generator shall return the matching typed non-Boolean result before it emits
  a strategy assumption or a partial artifact.
- If an object reference is absent, dangling, foreign, wrong-nominal-type, or
  outside the selected snapshot/universe, then the generator shall retain its
  source-linked typed non-Boolean disposition without constructing a graph
  harness.
- If a collection has an out-of-domain element, duplicate-sensitive ordering
  conflict, unavailable member, or cardinality above its selected bound, then
  the generator shall retain its source-linked typed non-Boolean disposition
  without constructing a strategy assumption.
- Where a construct is refused or inconclusive, the generator shall preserve
  its source identity and disposition without approximating it through another
  semantic family or reporting a proof.
- For every supported corpus case, the generator shall compare native runtime,
  executable oracle, generated strategy, and Kani outcomes using their exact
  typed classifications.
- When the generator retains a Kani counterexample, the generator shall supply
  the packet to the Contract IR native replay boundary.
- If native replay mismatches or is unavailable, then the generator shall
  retain the typed non-Boolean replay result.

## Constraints

| ID | Constraint | Type | Validation |
|----|------------|------|------------|
| FR-007-CON-1 | The generator SHALL consume Contract IR public interfaces only. | Architecture | Inspection |
| FR-007-CON-2 | The Contract IR package SHALL NOT depend on codegen. | Architecture | Inspection |
| FR-007-CON-3 | The generator SHALL NOT use a Kani assumption to erase invalid, rejected, incomplete, unavailable, timed-out, or exhausted inputs. | Integrity | Test |
| FR-007-CON-4 | The generator dependency set SHALL pin a reviewed public Contract IR revision exposing the selected bounded-Kani profile before implementation begins. | Compatibility | Inspection |

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-007-AC-1 | Every selected arithmetic/definedness, graph, and collection construct receives one exact profile disposition before lowering. | Test (TC-023) |
| FR-007-AC-2 | Supported finite corpus cases produce deterministic oracle, strategy, Kani, provenance, and proof-dependency artifacts from one validated input selection. | Test (TC-023) |
| FR-007-AC-3 | Invalid, incomplete, unavailable, over-bound, refused, inconclusive, timed-out, and exhausted cases retain a typed non-Boolean result and no partial artifact. | Test (TC-023) |
| FR-007-AC-4 | Every retained Kani counterexample replays through Contract IR's native runtime boundary with the same false classification, or the replay returns a typed non-Boolean mismatch or unavailable result. | Test (TC-023) |
| FR-007-AC-5 | The generated corpus preserves the Contract IR to codegen dependency direction and no generated artifact requires a reverse Contract IR dependency. | Inspection (TC-023) |
| FR-007-AC-6 | The generated `#[kani::proof]` symbol for a supported corpus case is derived from the case's own identity digest and carries the same identity the case's artifact paths carry. | Test (TC-023) |

## Dependencies

- **Upstream**: [FR-002](./FR-002-tristate-proptest.md), [FR-003](./FR-003-kani-lowering.md),
  Contract IR FR-029 through FR-031, and the accepted Contract IR #69 profile.
- **Downstream**: [TC-023](../test/TC-023-bounded-kani-profile-corpus.md).
