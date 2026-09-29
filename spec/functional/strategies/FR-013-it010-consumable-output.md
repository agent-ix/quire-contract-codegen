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
downstream crate such as the
consumer planned in agent-ix/quire-spec-language#84 can place every case's values into its own
runtime inputs by declaration name and observation. quire-spec-language has no IT-010 specification
yet, so this requirement references the issue, not a specification ID. The generator shall not
require any serialized case format or schema owned by this repository for that consumption.

## Inputs

- Generated bound strategy, census, and runner artifacts from
  [FR-009](./FR-009-constructive-correlated-populations.md) through
  [FR-011](./FR-011-numeric-harness-campaigns.md).

## Outputs

- Generated Rust whose public case type exposes:
  - one `i64` field per read, named with the same identifier the clause's generated oracle uses for
    that dependency parameter;
  - for each field, a constant pair of the IR declaration `SymbolName` as `&'static str` and the IR
    observation's serialized name (`"current"`, `"pre"`, or `"post"`) as `&'static str`. For a
    quire-spec-language field projection the `SymbolName` is SL's deterministic field alias, and a
    consumer maps it back to the model field through SL's read correspondence (quire-spec-language
    FR-034);
  - the expectation tag, for in-domain cases.

## Behavior

- Every generated case type shall carry the exact IR declaration name and observation name of each
  read, so a consumer needs no mapping table of its own.
- The generator shall take field identifiers from bound oracle generation's dependency parameter
  names.
- Generated output shall depend only on `proptest`, `quire-contract-runtime`, and `core`/`std`.
- Generated output shall not depend on quire-spec-language, which depends on this crate.
- The generator shall not emit a JSON, YAML, or other serialized case, census, or summary format.
- This slice shall add no file under `schemas/`.
- The generated Rust header shall state the full `ClauseRef`.
- For a read of an input declaration, whose observation is always `current` under
  quire-contract-ir FR-014, the generated observation name shall be `"current"`.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-013-AC-1 | A consumer fixture crate whose manifest depends only on the generated artifact, `proptest`, and `quire-contract-runtime` compiles under denied warnings, draws `VersionUnchanged` cases, and reads the `versionNumber` field's declaration, by its SL field-alias `SymbolName`, at `"pre"` and `"post"` through the generated name and observation constants. | Test (TC-022) |
| FR-013-AC-2 | The generated bundle contains no serialized case, census, or summary file, and the change adds no file under `schemas/`. | Test (TC-022) |
| FR-013-AC-4 | The generated Rust header carries the full `ClauseRef`, and generating two different `ClauseRef`s from one package produces different headers and artifact paths. | Test (TC-022) |
| FR-013-AC-5 | interface-001 `bound_strategy_slice.consumer` records the case, census, and runner surface a downstream consumer compiles against. | Inspection |

## Dependencies

- **Upstream**: [FR-010](./FR-010-domain-boundary-campaigns.md) and
  [FR-011](./FR-011-numeric-harness-campaigns.md).
- **Downstream**: the consumer planned in agent-ix/quire-spec-language#84;
  [TC-022](../../test/strategies/TC-022-it010-consumable-output.md).
