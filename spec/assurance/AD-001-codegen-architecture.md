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
proof attestation and source map to every output. The failure view retains invalid input, unsupported semantic,
backend unavailable, I/O failure, incomplete proof, vacuity, rejection, discard, and differential
states without a success fallback.

The numeric/state Kani view consumes the same typed dependency analysis and rendered predicates as
the executable oracle. It normalizes direct dependencies into a subject ABI: current inputs/current
state/pre-state are ordered arguments, while post-state values are an ordered result. Boolean values
remain `bool`; bounded integers remain `i64` with their IR-owned domain, inclusive bounds, overflow
policy, observation and source span. The proof graph records this binding separately from required,
assumed and stubbed proof-dependency edges.

## Decisions

A backend-neutral lowering plan prevents independent clause interpretation. `quote` and `syn` emit
Rust syntax. Stable ordering and path-independent names support byte reproducibility. Kani syntax is
isolated by the `kani-0.67.0-function-contracts-v2` adapter. Its v2 source and graph identities are
new contracts; the historical v1 schema files are retained and are not relabeled. LLVM export is
consumed as data. Atomic directory replacement is limited to generator-owned boundaries. Provisional
upstream pins remain explicit until reconciled.

The adapter derives every integer argument assumption directly from the checked IR `IntegerType` and
places each post-state domain in the ensures contract. It accepts no caller-supplied bound and does
not consume the proptest strategy interface, so agent E's strategy ownership remains independent.
Zero post-state values use `()`, one uses its primitive type, and multiple use a deterministic tuple;
this keeps the existing Boolean transition as one ordinary instance rather than a special semantic
path. Conflicting types/domains, post-state data in a precondition, unsupported observations,
definedness obligations, arithmetic/negation and indirect/object/graph reads fail closed before any
bundle is returned.

The pinned option vector enables Kani 0.67.0 function contracts and printed concrete playback, names
one exact harness, and records unwind, solver, regular output and optional stubbing. Generated graph
readiness remains a dependency-census result with proof execution `not_run`. Actual Kani execution is
a local verification step; quire-spec-language IT-010 owns converting the printed counterexample and
typed binding record into native runtime input and comparing the `runtime::execute` verdict.

The structured evidence verifier and its integrity probes were a reusable program surface, and the
boundary they identified is the one this repository now sits on: checksum, artifact-census, schema,
source-identity and retention logic belong to Engineering Assurance and Quoin, not to a participating
repository. The verifier went with the shared-assurance migration and its last local mutation probes
went with the retained evidence they guarded, so no instance of that surface remains here. The
observation is kept because it is why the boundary is drawn where it is; the component is not built
or published here.

The program's eight Rust crates currently expose divergent architecture and evidence structures.
Their ownership markers, gate names, evidence layouts, and architecture records must converge through
a separately reviewed cross-repository change before any claim of program-wide structural
consistency. This repository does not claim that convergence from its local controls.

## Risks

The runtime release decision remains under review; Kani syntax and concrete-playback text can change;
platform formatting/path behavior can threaten reproducibility; coverage regions can drift after
formatting; third-party differential fixtures require provenance review. The Kani version/profile and
complete option vector therefore remain exact evidence inputs, and downstream replay refuses output
that cannot be bound to the recorded typed ABI. These risks are measured and cannot be released away
by tooling.
