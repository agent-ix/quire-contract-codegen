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
outside the supported grammar.

## Expected Results

Each fixture produces its expected stable diagnostic, unsupported nodes and obligations name their
exact IR source span, and no fixture produces a falsely complete backend artifact.
