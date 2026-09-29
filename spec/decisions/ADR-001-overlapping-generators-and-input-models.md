---
id: ADR-001
title: "Overlapping Kani generators, oracle generators and input models"
type: ADR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AD-001
    type: relates_to
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: relates_to
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: relates_to
---
# ADR-001: Overlapping Kani generators, oracle generators and input models

## Status

Accepted.

## Context

The generator emits Kani harnesses and scalar oracles, and reads a contract package. Each needs one
owning requirement and one input model. `CheckedPackageV2` is the model QSL emits and the one QSL's
replay facade recompiles against.

## Decision

### Q1: FR-015 is the Kani backend's one generator

[FR-015](../functional/complete-v1/FR-015-bounded-kani-obligations.md) is the only Kani generation
requirement. It lowers against `cadical` with no stubbing option, and a counterexample replays only
through QSL's replay facade ([FR-024](../functional/complete-v1/FR-024-counterexample-envelope-intake.md)).

### Q2: FR-014 is the one oracle generator for the families it covers

[FR-014](../functional/complete-v1/FR-014-exact-scalar-oracles.md) generates the oracle of every
family it covers, the Boolean connectives and the bounded-integer comparisons included, with its
coverage-probe source map.

### Q3: `CheckedPackageV2` is the one input model

Every generator reads an admitted `quire.checked-package/v2` package through Contract IR's
`CheckedPackageV2`. The generator has no second input path and no adapter between input models. The
V1 `BoundPackage` input goes away once `CheckedPackageV2` covers what it serves.

## Consequences

- FR-014 and FR-015 each state their criteria over `CheckedPackageV2`.
