---
id: FR-013
title: "Emit strategy output consumable by SL IT-010 without a local wire schema"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-011
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/NFR-002
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/interface-001
    type: implements
---
# FR-013: Emit strategy output consumable by SL IT-010 without a local wire schema

## Description

The generator shall deliver numeric strategy and campaign output as typed generated Rust plus Quoin's
packaged `ProofAttestationV1` body. Then agent-ix/quire-spec-language#84 (IT-010) can compile it,
feed each case into `runtime::execute`, and compare verdicts. The generator shall not require any
serialized case format or schema owned by this repository for that consumption.

## Inputs

- Generated numeric strategy and harness artifacts from
  [FR-009](./FR-009-constructive-correlated-populations.md) through
  [FR-011](./FR-011-numeric-harness-campaigns.md).

## Outputs

- Generated Rust whose public case type exposes:
  - each generated read as an `i64` field named from the IR `SymbolName` and `StateObservation`;
  - an accessor returning the pair (declaration name, observation) for each field, as `&'static str`
    and the runtime observation kind;
  - the expectation tag.
- One `ProofAttestationV1` body per artifact under proof obligation
  `PROOF-codegen-generated-rust-strategy`.

## Behavior

- Every generated case type shall carry the exact IR declaration name and observation of each value,
  so a consumer can place the value into a runtime snapshot without a mapping table of its own.
- Generated strategy output shall depend only on `proptest`, `quire-contract-runtime`, and `core`/
  `std`. It shall not depend on quire-spec-language, which depends on this crate.
- The generator shall not emit a JSON, YAML, or other serialized case, census, or summary format.
- This slice shall add no file under `schemas/`.
- The attestation shall be Quoin's packaged `ProofAttestationV1` emitted form defined in
  interface-001 `identity_envelope`, with `--backend none`.
- The bundle shall name the source `BoundPackage` digest and the full `ClauseRef` in both the generated
  Rust header and the attestation argv, so SL can bind a campaign to the checked package it lowered.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-013-AC-1 | A consumer fixture crate that depends only on the generated artifact, `proptest`, and `quire-contract-runtime` compiles the ConfigVersion `VersionUnchanged` strategy and reads `pre`/`post` values by declaration name and observation. | Test (TC-022) |
| FR-013-AC-2 | The generated bundle contains no serialized case, census, or summary file, and the change adds no file under `schemas/`. | Test (TC-022) |
| FR-013-AC-3 | The strategy attestation body validates against the bytes `quoin change-assurance schema` publishes, with format assertion on, and seals through the real CLI. | Test (TC-022) |
| FR-013-AC-4 | The generated Rust header and attestation argv carry the `BoundPackage` digest and full `ClauseRef`, and changing either changes the artifact identity. | Test (TC-022) |
| FR-013-AC-5 | The strategy interface consumed by IT-010 is recorded in interface-001 and linked on agent-ix/quire-spec-language#84 before implementation merges. | Inspection |

## Dependencies

- **Upstream**: [FR-011](./FR-011-numeric-harness-campaigns.md),
  [NFR-002](../../nonfunctional/NFR-002-provenance-boundary.md).
- **Downstream**: agent-ix/quire-spec-language#84 IT-010,
  [TC-022](../../test/strategies/TC-022-it010-consumable-output.md).
