---
id: TC-014
title: "Verify bounded numeric and state Kani contracts"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-003
    type: verifies
---
# TC-014: Verify bounded numeric and state Kani contracts

## Description

Verify the generalized Kani adapter derives exact Boolean and bounded-`i64` subject bindings from
the same checked clauses as executable oracles, constrains symbolic values from IR model domains,
and emits reproducible proof and counterexample-capable harnesses without widening unsupported input.

## Test Procedure

Construct checked clause pairs over direct Boolean values and all six integer comparisons. Include a
plain comparison and a ConfigVersion-style `post(version) = pre(version)` state clause whose signed
integer type is exactly 0 through 1000. Generate bundles repeatedly with passed, missing, failed,
assumed, and stubbed proof dependencies. Inspect the generated argument/result ABI, model-domain
assumptions, post-state domain guarantee, proof graph, schemas, attestations, adapter profile, backend
executable digest, solver, unwind, exact harness, function-contract, concrete-playback, output-format,
and stubbing options.

Exercise direct current-input, current-state, pre-state, and post-state observations and the zero,
one, and multiple post-state result shapes. Include a mixed Boolean/integer subject and permute the
source and dependency-census order while preserving normalized identities. Compile a fixture with an
invalid customer subject signature and require an external Rust/Kani failure without a generated
success claim; generation itself validates only the subject path syntax and typed IR-owned ABI.

Compile every generated crate with `publish = false`. Run the exact generated harness under
cargo-kani 0.67.0 for an identity state subject and require proof success. Run it for a changed-value
subject and for a falsifiable plain integer comparison and require proof failure plus printed concrete
playback data. Independently evaluate the embedded oracle predicates over a finite corpus containing
-1, 0, 1, 999, 1000, and 1001; distinguish the in-domain Kani population from outside-domain oracle
checks.

Mutate one dimension at a time: cross-clause type or bound, observation placement, indirect path,
definedness obligation, arithmetic/negation, object/graph read, backend version, executable digest,
unwind, identity, and generated-source limit. Require the stable diagnostic path and original
expression source span where one exists.

## Expected Results

The healthy state subject proves for the complete 0 through 1000 input domain. The changed subject
and falsifiable comparison yield counterexamples within that domain, with the exact Kani options and
bindings retained for downstream replay. Values -1 and 1001 are never silently admitted to the Kani
proof population, while executable oracles continue to report their direct Boolean result. Every
unsupported or inconsistent case emits no partial Kani bundle. Boolean bundles retain their existing
semantics, normalized-order permutations reproduce byte-identical artifacts, and unmodeled subject
effects receive no framing or proof claim.
