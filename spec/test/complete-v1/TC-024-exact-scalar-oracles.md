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

1. Build an admitted V2 package holding expression nodes for every scalar
   operator, division law, IEEE operator and comparison, text profile and
   comparison operator, each depending on the `bounded_domain` nodes its
   descriptor needs, plus unlowered, unbounded, mismatched, mis-bounded,
   duplicate and non-scalar nodes and unsupported and literal operands.
2. Generate twice and with a permuted request; compare bytes with each other
   and with the committed golden.
3. Inspect each claim-map entry, its ordering, its checked bounds and
   `caller_declared` provenance, and each refusal; request a node under a
   descriptor naming a different law; exhaust lowering work; exceed the
   generated source ceiling; construct invalid and incomplete body records.
4. Compile the golden oracle into the test crate and execute it on vectors
   adapted from QSpec TC-185, TC-186, TC-187, TC-192 and TC-193; compare each
   outcome, admitted charges and consumed counters with direct runtime
   execution and with `quire_spec_language::value`, including the outcome of
   denying each admitted charge in turn.
5. Compile the generated crate manifest.

## Expected Results

Every family is generated; every refused item is absent from the source and
carries its typed reason; a descriptor unequal to its IR bound is refused, and
one naming a different law over equal bounds is generated only as
`caller_declared`; bytes are identical across runs and orderings; all
three executions agree on every vector; the generated crate compiles with
`publish = false` and contains no charge literal.

Integer arithmetic, rational arithmetic and ordering (including the decimal
ordering charges QSpec added after QSL d9d5273) have no operator in the pinned
authority. Their oracles agree with direct runtime execution only, and are
counted separately.
