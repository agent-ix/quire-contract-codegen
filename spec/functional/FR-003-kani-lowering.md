---
id: FR-003
title: "Generate Kani obligations and proof dependencies"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-001
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/interface-001
    type: implements
---
# FR-003: Generate Kani obligations and proof dependencies

## Description

Where bounded proof lowering is supported, the generator SHALL emit Kani requires, ensures, proof
harnesses, framing, bindings, and a proof dependency graph from the exact typed clauses accepted by
the executable-oracle lowering core.

## Inputs

- A typed Boolean precondition and postcondition, their clause identities, and their complete direct
  dependency censuses.
- Checked Boolean or bounded-integer dependency types, including each integer domain, inclusive
  minimum, inclusive maximum, overflow policy, observation, and source span.
- A customer subject path, exact Kani backend identity, explicit unwind and solver, declared proof
  dependencies, and attestation context.

## Outputs

- Version-adapted Kani source with separately inspectable framing, binding, contract, and proof-harness
  regions.
- A schema-validated proof graph that retains the normalized subject ABI, exact bound sources,
  dependency closure, adapter options, and generation-time readiness.
- Structured diagnostics with no partial bundle, or generated-source and proof-graph artifacts with
  one Quoin proof-attestation body per artifact.

## Behavior

- The Kani adapter SHALL reuse executable-oracle dependency analysis and rendered predicates without
  independently interpreting clause semantics.
- The Kani adapter SHALL order subject arguments and post-state results by normalized dependency
  identity, while retaining kind, observation, Rust type, IR integer domain, inclusive bounds,
  overflow policy, and source span in the binding graph.
- The Kani adapter SHALL bind direct current inputs, current state, and pre-state dependencies as
  symbolic `bool` or `i64` subject arguments.
- The Kani adapter SHALL bind direct post-state dependencies as the subject result, using `()` for
  no post-state value, the primitive for one value, and an ordered tuple for multiple values.
- When a symbolic argument is a bounded integer, the Kani harness SHALL emit `kani::assume` from the
  checked IR `IntegerType` inclusive minimum and maximum.
- The generator SHALL NOT accept a caller override, infer a wider range, or substitute a strategy
  range for a checked IR integer domain.
- When a post-state result is a bounded integer, the generated ensures contract SHALL require that
  result to remain inside the same checked IR domain before the clause can hold.
- The adapter SHALL preserve the existing Boolean transition ABI as the one-input, one-pre-state,
  one-post-state instance of the generalized ordering rule.
- Every stubbed or assumed proof edge SHALL appear in the proof dependency graph.
- Model-domain assumptions SHALL appear separately as typed binding bounds rather than as completed
  dependency proofs.
- Missing or failed required dependencies SHALL prevent a ready classification.
- When any proof dependency is assumed or stubbed, the generator SHALL classify readiness as
  conditional.
- The generator SHALL always retain proof execution as `not_run`.
- The adapter SHALL pin cargo-kani 0.67.0, its executable digest, the
  `kani-0.67.0-function-contracts-v2` profile, solver, unwind, exact harness, function-contract,
  concrete-playback, output-format, and optional stubbing flags in the proof graph and both
  generated-artifact attestations.
- If the request contains a cross-clause type or domain conflict, post-state data in a precondition, an unsupported observation or dependency shape, a definedness obligation, or an expression outside the executable-oracle grammar, then the Kani adapter SHALL refuse explicitly with the originating diagnostic and source span.
- When the Kani adapter refuses a request, the generator SHALL emit no bundle for that request.
- The generated framing region SHALL represent only the explicit primitive subject arguments and
  result, without claiming unmodeled global, heap, alias, object, or graph state.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-003-AC-1 | A proof graph is incomplete while any required dependency is missing or failed, conditional while any assumption or stub is present, and ready only for a complete passed dependency census; every graph still records proof execution as `not_run`. | Test (TC-005) |
| FR-003-AC-2 | Generated Kani contracts embed the byte-identical executable-oracle predicates and agree with their verdicts for every supported Boolean and bounded-integer comparison in the shared bounded corpus. | Test (TC-007, TC-014) |
| FR-003-AC-3 | Definedness obligations, arithmetic and numeric negation, indirect or object/graph reads, unsupported observations, inconsistent cross-clause types/domains, and unrepresentable subject bindings produce explicit diagnostics with no partial bundle. | Test (TC-003, TC-014) |
| FR-003-AC-4 | The proof graph and attestations retain cargo-kani 0.67.0, executable digest, v2 adapter profile, exact option vector, subject ABI, typed domain bounds, dependency assumptions/stubs, and output identities without claiming proof completion. | Test (TC-005, TC-014) |
| FR-003-AC-5 | For every bounded-integer subject argument, the generated harness assumes exactly the checked IR inclusive minimum and maximum; exact endpoints are admitted and immediately outside values are excluded without clamping or approximation. | Test (TC-014) |
| FR-003-AC-6 | A zero-input ConfigVersion-style transition binds one bounded pre-state integer to one bounded post-state integer, proves an identity subject, and produces a concrete Kani counterexample for a changed-value subject. | Test (TC-014) |
| FR-003-AC-7 | A plain bounded-integer comparison can be checked without a state result, and a falsifying in-domain assignment is reported through the pinned concrete-playback profile. | Test (TC-014) |
| FR-003-AC-8 | Boolean generation retains its existing one-input/one-state behavior under the generalized ABI, and repeated generation with the same clauses, domains, subject, dependencies, and pins is byte-identical. | Test (TC-014) |

## Dependencies

- **Upstream**: [FR-001](./FR-001-deterministic-oracles.md).
- **Downstream**: `ix://agent-ix/quire-spec-language/IT-010` owns replay of a printed Kani
  counterexample through native `runtime::execute`; this repository supplies the exact generated
  harness, option vector, and typed binding/domain record but does not manufacture a native runtime
  verdict.
