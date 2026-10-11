---
id: SR-7801
title: "Base review of fixture identity spec"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen@5c79de161d9a7efc589ae39bbc6340f0ec127ffd; spec/core/functional/FR-037-fixture-identity.md, spec/core/matrix/TC-052-fixture-identity.md, spec/spec.md"
review_set: subset
---

## Summary

The new requirement and test case validate structurally, but their planned coverage is absent from both coverage authorities. Ticket: IR-377.

## Verdict

**CONDITIONAL** — The new requirement and test case validate structurally, but their planned coverage is absent from both coverage authorities.

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-037-AC-1 | examined | Both `tests/exact_scalar_support/package.rs` and `tests/composite_equality_support/package.rs` derive every candidate application node identity through `derive_application_node_id` over the complete draft node; neither retains an application preimage encoder, application version constant, or SHA call for that identity. An ungrouped fixture's candidate equals the checked reader's admitted node ID. |
| FR-037-AC-2 | examined | A fixture containing an application in a recursion group supplies the complete ordered draft graph to `ApplicationIdentityContext`; its candidate agrees with the reader after group-reference rewriting. Missing or mismatched context and a canonical byte-limit refusal retain Contract IR's typed error and produce no candidate. |
| FR-037-AC-3 | examined | A canonical enum-member fixture identity is derived through `NominalIdentityPreimage::EnumMember(...).digest`, agrees with the checked reader, and changes when the member case changes. In-process agreement keys that are never compared with a checked-package identity remain classified as fixture-only keys. |
| FR-037-AC-4 | examined | Positive exact-scalar and composite-equality packages remain admitted. A stale candidate key or a body mutation without rekeying is refused by the checked reader; the fixture builder does not repair those negative controls before the assertion. |
| FR-037-AC-5 | examined | The consumer change leaves deterministic fixture-only keys distinct from application identities and uses `quire-canonical` for ordinary canonical-value hashes where those hashes are needed. Package identity refresh uses Contract IR's typed package identity preimage and `quire-canonical`; no independent application or package preimage rule is added. |
| FR-037 | examined | When the exact-scalar or composite-equality fixture builder assigns a candidate identity to an application-bodied checked-package node, the builder shall use Contract IR's `derive_application_node_id` with an `ApplicationNodeDraft` made from that node. |
| TC-052 | examined | Verify checked-package fixture identity ownership |
| spec/spec.md | examined | \| Checked-package fixture identity ownership \| [FR-037](core/functional/FR-037-fixture-identity.md) application and nominal fixture identities \| [TC-052](core/matrix/TC-052-fixture-identity.md) planned consumer regression \| |
| TM-001 | context_only | Contract codegen core test matrix |
| TM-008 | context_only | quire-contract-codegen test matrix index |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-037 and TC-052 appear in the registry but have no rows in the core coverage matrix or matrix index; add FR-037-AC-1 through AC-5 to TM-001 with TC-052 and register FR-037 in TM-008. | spec/core/functional/FR-037-fixture-identity.md:53 |
