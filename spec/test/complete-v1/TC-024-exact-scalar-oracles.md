---
id: TC-024
title: "Verify exact complete-V1 scalar oracle generation and agreement"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: verifies
  - target: ix://agent-ix/quire-specification/TC-218
    type: references
---
# TC-024: Verify exact complete-V1 scalar oracle generation and agreement

## Description

Verify that scalar oracles generated from an admitted CheckedPackage V2 cover
every scalar family, refuse every non-generated item with a typed reason, are
byte-deterministic, and agree with the runtime and the QSL value authority.

## Test Procedure

1. Build an admitted V2 package holding one expression node per scalar family
   plus unlowered, mismatched, duplicate and non-scalar nodes.
2. Generate twice and with a permuted request; compare bytes with each other
   and with the committed golden.
3. Inspect each claim-map entry and each refusal.
4. Compile the golden oracle into the test crate and execute it on vectors
   adapted from QSpec TC-185, TC-186, TC-187, TC-192 and TC-193; compare each
   outcome, admitted charges and consumed counters with direct runtime
   execution and with `quire_spec_language::value`.
5. Compile the generated crate manifest.

## Expected Results

Every family is generated; every refused item is absent from the source and
carries its typed reason; bytes are identical across runs and orderings; all
three executions agree on every vector; the generated crate compiles with
`publish = false` and contains no charge literal.
