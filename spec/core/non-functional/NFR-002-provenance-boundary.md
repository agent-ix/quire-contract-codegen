---
id: NFR-002
title: "Traceability, licensing, and qualification boundary"
type: NFR
quality_attribute: compliance
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: constrains
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: constrains
---
# NFR-002: Traceability, licensing, and qualification boundary

## Statement

- Every generated Rust file shall have a header naming `quire-contract-codegen` and every
  input-carried generation subject identity it renders, as defined in Scope.
- Where the generation input carries a requirement, revision or clause identifier, the generated
  Rust file header shall name it.
- Generated Rust shall carry `MIT OR Apache-2.0` SPDX identity.
- No automated output shall claim project-specific validation, accreditation, certification, or human release approval.

## Scope

Generated source, diagnostics, source maps, proof graphs, and reports. Generated Rust families are
strategy harnesses; integer, enum and bound strategies and bound populations; Boolean, exact
scalar, composite equality and function-application oracles; V1 and V2 contract, exact-scalar,
collection and state-frame Kani harnesses; and the public bounded Kani corpus (`corpus/*.oracle.rs`,
`*.strategy.rs` and `*.kani.rs`) until FR-015-AC-111 deletes it.

The subject of a clause-keyed Boolean or bound oracle, strategy harness, bound strategy or V1
contract harness is its input-carried requirement, revision and clause identity. Other generated
files name their input-carried strategy, population, corpus case, source node, obligation, claim or
operation identity as applicable. A file rendering multiple subjects names every source node or
obligation it renders; this includes oracle-package `src/lib.rs` files, equality extra modules and
function schedule part files (`src/schedule_*.rs`). Each emitted `.rs` file is covered separately.

## Rationale

Opaque licensing prevents independent audit, while automated qualification claims
would exceed the tool's authority and hide consuming-project responsibilities.
Some generation inputs, including V2 and corpus inputs, carry no requirement, revision or clause
identifier; headers name these identifiers only when supplied by the input and never infer or
invent them.

Local independent evaluation and third-party differential comparison are separate evidence
classes. The 265-expression evaluator in `tests/it/oracle_generation.rs` and generated
compile/runtime controls establish local implementation checks. A Sireum/INSPECTA comparison
requires the permitted provenance and actual observations specified by
[FR-005](../functional/FR-005-cli-conformance.md). The current open claim and unavailable
third-party lane are recorded in [AA-001](../../assurance/AA-001-differential-fixture-provenance.md).

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
|--------|--------|-----------|--------|
| Generated Rust files missing generator or input-carried subject identity | 0 | 0 | contract-testing |
| Generated files missing dual-license SPDX header | 0 | 0 | inspection |
| Silent unsupported/inconclusive states | 0 | 0 | negative-abuse-testing |

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| NFR-002-AC-1 | Every generated Rust file in a family named in Scope has a header naming `quire-contract-codegen`, every input-carried subject identity it renders as defined in Scope, and every requirement, revision or clause identifier present in the generation input. | Test (TC-001) |
| NFR-002-AC-2 | Every generated Rust file carries the `MIT OR Apache-2.0` SPDX identity. | Test (TC-001) |
| NFR-002-AC-3 | Unsupported and inconclusive states remain explicit rather than silently succeeding. | Test (TC-003) |
| NFR-002-AC-4 | Generated output makes no project-specific validation, accreditation, certification, or release-approval claim. | Inspection |
| NFR-002-AC-5 | Assurance claims distinguish local independent evaluation from attributed third-party differential evidence. An unexecuted or unavailable Sireum/INSPECTA lane supplies no third-party agreement claim; tool metadata and a proposed invocation alone do not establish execution or semantic agreement. | Inspection |

## Verification

TC-001 validates the generated header identity and licensing; TC-003 validates explicit diagnostic
states.

## Dependencies

- **Upstream**: [FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md).
