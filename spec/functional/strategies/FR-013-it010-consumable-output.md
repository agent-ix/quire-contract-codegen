---
id: FR-013
title: "Emit strategy output consumable by SL IT-010 without a local wire schema"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-010
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-011
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/NFR-002
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/interface-001
    type: implements
---
# FR-013: Emit strategy output consumable by SL IT-010 without a local wire schema

## Description

The generator shall deliver bound strategy, census, and runner output as typed generated Rust plus
Quoin's packaged `ProofAttestationV1` body, so a downstream crate such as
agent-ix/quire-spec-language#84 (IT-010) can place every case's values into its own runtime inputs by
declaration name and observation. The generator shall not require any serialized case format or
schema owned by this repository for that consumption.

## Inputs

- Generated bound strategy, census, and runner artifacts from
  [FR-009](./FR-009-constructive-correlated-populations.md) through
  [FR-011](./FR-011-numeric-harness-campaigns.md).

## Outputs

- Generated Rust whose public case type exposes:
  - one `i64` field per read, named with the same identifier the clause's generated oracle uses for
    that dependency parameter;
  - for each field, a constant pair of the IR declaration name as `&'static str` and the IR
    observation's serialized name (`"current"`, `"pre"`, or `"post"`) as `&'static str`;
  - the expectation tag, for in-domain cases.
- One `ProofAttestationV1` body per artifact under proof obligation
  `PROOF-codegen-generated-rust-strategy`.

## Behavior

- Every generated case type shall carry the exact IR declaration name and observation name of each
  read, so a consumer needs no mapping table of its own.
- The generator shall take field identifiers from bound oracle generation's dependency parameter
  names.
- If two reads map to one field identifier, then the generator shall refuse with the oracle's
  `NameCollision` diagnostic preserved under `UnsupportedClause`.
- Generated output shall depend only on `proptest`, `quire-contract-runtime`, and `core`/`std`.
- Generated output shall not depend on quire-spec-language, which depends on this crate.
- The generator shall not emit a JSON, YAML, or other serialized case, census, or summary format.
- This slice shall add no file under `schemas/`.
- The attestation shall be Quoin's packaged `ProofAttestationV1` emitted form defined in
  interface-001 `identity_envelope`, with `--backend none`.
- The attestation argv shall render `--requirement <requirement>@<revision> --clause <clause id>`,
  as bound oracle attestations do under [FR-001](../FR-001-deterministic-oracles.md).
- The attestation argv shall bind the source package through `--input-digest` carrying the
  `BoundPackage` digest.
- The generated Rust header shall state the `BoundPackage` digest and the full `ClauseRef`.
- For a read whose observation is `Input`, the generated observation name shall be `"current"`, as
  the oracle's reference identifiers render it.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-013-AC-1 | A consumer fixture crate whose manifest depends only on the generated artifact, `proptest`, and `quire-contract-runtime` compiles under denied warnings, draws `VersionUnchanged` cases, and reads `versionNumber` at `"pre"` and `"post"` through the generated name and observation constants. | Test (TC-022) |
| FR-013-AC-2 | The generated bundle contains no serialized case, census, or summary file, and the change adds no file under `schemas/`. | Test (TC-022) |
| FR-013-AC-3 | The strategy attestation body validates against the bytes `quoin change-assurance schema` publishes, with format assertion on, and seals through the real CLI. | Test (TC-022) |
| FR-013-AC-4 | The generated Rust header carries the `BoundPackage` digest and full `ClauseRef`; the attestation argv carries `--requirement <requirement>@<revision>`, `--clause <clause id>`, and `--input-digest <BoundPackage digest>`; changing the package digest or the `ClauseRef` changes the artifact identity. | Test (TC-022) |
| FR-013-AC-5 | interface-001 `bound_strategy_slice.consumer` records the case, census, and runner surface a downstream consumer compiles against. | Inspection |

## Dependencies

- **Upstream**: [FR-010](./FR-010-domain-boundary-campaigns.md),
  [FR-011](./FR-011-numeric-harness-campaigns.md),
  [NFR-002](../../nonfunctional/NFR-002-provenance-boundary.md).
- **Downstream**: agent-ix/quire-spec-language#84 IT-010, which uses
  `quire_spec_language::runtime::execute`;
  [TC-022](../../test/strategies/TC-022-it010-consumable-output.md).
