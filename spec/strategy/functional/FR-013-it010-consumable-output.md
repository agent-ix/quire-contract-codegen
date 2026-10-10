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
  - target: ix://agent-ix/quire-contract-codegen/interface-001
    type: implements
  - target: ix://agent-ix/quire-spec-language/FR-034
    type: references
---
# FR-013: Emit strategy output consumable by SL IT-010 without a local wire schema

## Description

The generator shall deliver bound strategy, census, and runner output as typed generated Rust, so a
downstream crate can place every case's values into its own runtime inputs by typed declaration identity, selected path and observation. The generator shall not
require any serialized case format or schema owned by this repository for that consumption.

## Inputs

- Generated bound strategy, census, and runner artifacts from
  [FR-009](./FR-009-constructive-correlated-populations.md) through
  [FR-011](./FR-011-numeric-harness-campaigns.md).

## Outputs

- Generated Rust whose public case type exposes:
  - one `i64` field per read, named with the identifier derived from that read's typed V2 dependency identity and observation;
  - for each field, constants exposing the typed V2 declaration/provenance identity,
    normalized selected path, and observation (`"current"`, `"pre"`, or `"post"`), so the consumer
    can bind the value without recovering a V1 `SymbolName` or parsing generated source;
  - the expectation tag, for in-domain cases.

## Behavior

- Every generated case type shall carry constants rendered from each read's typed V2 declaration identity, provenance kind, selected path and observation, so a consumer needs no mapping table of its own.
- The generator shall derive field identifiers from the same typed V2 dependency identity and
  observation used by its selected-claim oracle, with deterministic collision handling.
- Generated output shall depend only on `proptest`, `quire-contract-runtime`, and `core`/`std`.
- Generated output shall not depend on quire-spec-language, which depends on this crate.
- The generator shall not emit a JSON, YAML, or other serialized case, census, or summary format.
- This slice shall add no file under `schemas/`.
- The generated Rust header shall state the selected checked clause id and authentic claim occurrence.
- For a typed `OperationInput` read, the generated observation name shall be `"current"`.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-013-AC-1 | A consumer fixture crate that includes the generated Rust source and whose manifest depends only on `proptest` and `quire-contract-runtime` compiles under denied warnings, draws `VersionUnchanged` cases, and reads the selected `versionNumber` StateField at `"pre"` and `"post"` through the generated typed-identity, path and observation constants. | Test (TC-022) |
| FR-013-AC-2 | The generated bundle contains no serialized case, census, or summary file, and the change adds no file under `schemas/`. | Test (TC-022) |
| FR-013-AC-4 | The generated Rust header carries the selected checked clause id and authentic claim occurrence, and generating two distinct selected claims from one package produces different headers and artifact paths. | Test (TC-022) |
| FR-013-AC-5 | interface-001 `bound_strategy_slice.consumer` records the case, census, and runner surface a downstream consumer compiles against. | Inspection |

## Dependencies

- **Upstream**: [FR-010](./FR-010-domain-boundary-campaigns.md) and
  [FR-011](./FR-011-numeric-harness-campaigns.md).
- **Downstream**: [TC-022](../matrix/TC-022-it010-consumable-output.md).
