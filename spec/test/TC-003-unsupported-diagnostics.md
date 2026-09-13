---
id: TC-003
title: "Reject unsupported or invalid constructs explicitly"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-001
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/FR-003
    type: verifies
---
# TC-003: Reject unsupported or invalid constructs explicitly

## Description

Verify malformed, orphaned, partial, unsupported and undefined-result constructs cannot yield
complete artifacts or lose their refusal locus.

## Test Procedure

Run every negative conformance fixture through every applicable backend and inspect diagnostics,
exact rejected IR source spans, attestation completeness state, exit status, and staged output
directory. Include definedness obligations, scalar roots, numeric arithmetic/negation, indirect
dependencies, object/graph reads such as dereference and reachability, and every expression node
outside the supported grammar. Where more than one expression node or obligation is unsupported,
require the deterministic first locus declared by FR-001 rather than accepting any failing span.

For Kani, also vary pre/post placement and one cross-clause binding dimension at a time: dependency
kind, observation, Boolean/integer type, integer domain/minimum/maximum/overflow policy, subject ABI,
backend version, executable digest, unwind, solver and identity. A post-state dependency in a
precondition, conflicting declarations for one dependency, or any range not taken from the checked
IR is unrepresentable and must refuse before source, graph, or attestation publication.

## Expected Results

Each fixture produces its expected stable diagnostic, unsupported nodes and obligations name their
exact IR source span, and no fixture produces a falsely complete backend artifact. Kani binding
refusals never fall back to Boolean parameters, unconstrained `i64`, machine extrema, or a proptest
strategy range.
