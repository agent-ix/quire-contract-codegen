---
id: AD-001
title: Contract codegen architecture
type: ArchitectureDescription
status: proposed
owner: codegen-maintainers
system: quire-contract-codegen v0.1
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AP-001
    type: realizes
---
# Contract codegen architecture

## System Boundary

The owned boundary parses only the already-serialized IR package into pinned model types, validates
its declared compatibility, lowers a shared semantic plan into backend artifacts, validates a staged
bundle, and publishes it atomically. The IR canonicalizer, Rust compiler, proptest, Kani, LLVM
coverage, runtime, customer types, and human decision remain external and version-identified.

## Views

The derivation view is input package → validated lowering plan → executable/proptest/Kani/coverage
backends → bundle validator → atomic publisher. The evidence view attaches one immutable identity
envelope and source map to every output. The failure view retains invalid input, unsupported semantic,
backend unavailable, I/O failure, incomplete proof, vacuity, rejection, discard, and differential
states without a success fallback.

## Decisions

A backend-neutral lowering plan prevents independent clause interpretation. `quote` and `syn` emit
Rust syntax. Stable ordering and path-independent names support byte reproducibility. Kani syntax is
isolated by version adapter. LLVM export is consumed as data. Atomic directory replacement is limited
to generator-owned boundaries. Provisional upstream pins remain explicit until reconciled.

The structured evidence verifier and its integrity probes are a reusable program surface. A future
shared suite should own checksum, artifact-census, schema, source-identity, and mutation-probe logic
for all participating repositories; this foundation records that boundary but does not build or
publish the shared component.

The program's eight Rust crates currently expose divergent architecture and evidence structures.
Their ownership markers, gate names, evidence layouts, and architecture records must converge through
a separately reviewed cross-repository change before any claim of program-wide structural
consistency. This repository does not claim that convergence from its local controls.

## Risks

The IR candidate and runtime release decision remain under review; Kani syntax changes; platform formatting/path behavior can
threaten reproducibility; coverage regions can drift after formatting; third-party differential
fixtures require provenance review. These risks are measured and cannot be released away by tooling.
