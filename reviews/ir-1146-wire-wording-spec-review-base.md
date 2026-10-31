---
id: SR-2026103027
title: "Base review of IR-1146 checked-node wording"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; spec/oracle/functional/FR-018-composite-equality-oracles.md"
review_set: base
---

## Summary

Reviewed the wording follow-up at ed76b5f6ad23c0982c764df0ddf99c7fffdfb3a1 for IR-1146. The pinned Contract IR `CheckedNodeId` has package, path and ordinal, while the pinned QSL `AdmittedContext` currently takes `NodeKey` for enum and composite lookup. The revised prose names the structural checked-package identifier and states the missing owner lookup capability without claiming that it already exists. No applicable AssuranceProfile was found in the scoped spec tree.

## Verdict

**READY.** The wording preserves original checker-issued key selection, identity comparison, complete enum cases, typed context and lookup failures, AC-17 reconstruction/refusal behavior, and AC-36's prohibition on emitted key construction. Record, tuple and recursive closure obligations remain pending; the change grants no `Unsupported` substitute or implementation credit. No acceptance criterion was narrowed.

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-018 owner-key prose | examined | A declaration or case key SHALL be the original checker-issued key selected through its owning admitted lookup. Equality with the corresponding V2 wire identity is retained, including its package, ordered path segments and ordinal; it is not permission for CG to hash a preimage, decode a digest into `NodeKey`, or emit a key constructor. |
| FR-018 enum lookup prose | examined | For enums, the generator SHALL use the owning `AdmittedContext` lookup by `CheckedNodeId` and `enum_case`, verify the declaration's qualified name and ordering against the selected IR node, and use all `AdmittedEnum::cases()`, including unused cases. |
| FR-018 residual owner gap | examined | The published facade supplies enum and composite lookups by `NodeKey`, but neither lookup currently accepts the structural checked-package `CheckedNodeId` that emitted source may retain. A record or tuple `CompositeDeclaration` still requires its genuine owner-issued `NodeKey`. |
| FR-018 Outputs | examined | `MissingAdmittedContext` names `quire_contract_model::CheckedNodeId`; `Composite` reports an invalid retained composite `CheckedNodeId` literal or a genuine composite lookup whose form or qualified name disagrees. |
| FR-018-AC-17 | examined | The emitted composite lookup maps an invalid retained `CheckedNodeId` literal or a successful genuine lookup with a record/tuple form or qualified-name disagreement to `EnvironmentError::Reconstruction(ReconstructionError::Composite)`, while facade Unknown/Foreign/WrongKind stay `EnvironmentError::Lookup`. |
| FR-018-AC-36 | examined | Emitted source uses only structural checked-package `CheckedNodeId` and case identifiers for lookup and contains no NodeKey constructor/decoder or generated nominal preimage/hash. |
| FR-018-AC-46 | context_only | For every V2 node id its operand types reach, the declaration key is the exact owner-issued `NodeKey` selected through admitted QSL lookup: its package, ordered path segments and ordinal equal the admitted node's structural identity. |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
