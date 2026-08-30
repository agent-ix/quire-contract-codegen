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
to generator-owned boundaries. Draft upstream pins remain explicit until reconciled.

## Risks

IR and runtime contracts are still draft; Kani syntax changes; platform formatting/path behavior can
threaten reproducibility; coverage regions can drift after formatting; third-party differential
fixtures require provenance review. These risks are measured and cannot be released away by tooling.
