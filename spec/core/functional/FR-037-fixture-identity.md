---
id: FR-037
title: "Derive checked-package fixture identities through their owning APIs"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-ir/FR-046
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-018
    type: references
---
# FR-037: Derive checked-package fixture identities through their owning APIs

## Description

When the exact-scalar or composite-equality fixture builder assigns a candidate
identity to an application-bodied checked-package node, the builder shall use
Contract IR's `derive_application_node_id` with an `ApplicationNodeDraft` made
from that node. For a grouped node, it shall supply an
`ApplicationIdentityContext` made from the complete, ordered draft graph. The
builder shall use the resulting candidate node ID for fixture assembly and
retain the checked-package reader as the admission authority. It shall not
construct or hash a local copy of the application preimage or its version tag.

When a fixture needs the canonical identity of an enum member, the fixture shall
derive it with Contract IR's existing
`NominalIdentityPreimage::EnumMember(...).digest`, rather than a locally
encoded enum-member preimage. This rule applies to canonical checked-package
node identities. Keys used only to keep in-process direct/generated agreement
fixtures distinct are fixture data; ordinary canonical-value and package
identities follow their own owning canonical/typed APIs.

## Behavior

- For an ungrouped application, the builder shall derive a candidate from the
  complete node after its declaration, semantic type and body are set.
- For a grouped application, the builder shall derive the candidate with the
  complete ordered draft graph, including the target and all members of its
  recursion group, so Contract IR owns the group ordinal and reference rewrite.
- If Contract IR refuses candidate derivation, the builder shall propagate or
  assert that typed refusal instead of substituting a locally computed key.
- After assigning candidate keys and refreshing dependent references and the
  package identity, the builder shall submit the fixture to the checked-package
  reader. A candidate alone does not establish admission.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-037-AC-1 | Both `tests/exact_scalar_support/package.rs` and `tests/composite_equality_support/package.rs` derive every candidate application node identity through `derive_application_node_id` over the complete draft node; neither retains an application preimage encoder, application version constant, or SHA call for that identity. An ungrouped fixture's candidate equals the checked reader's admitted node ID. | Test |
| FR-037-AC-2 | A fixture containing an application in a recursion group supplies the complete ordered draft graph to `ApplicationIdentityContext`; its candidate agrees with the reader after group-reference rewriting. Missing or mismatched context and a canonical byte-limit refusal retain Contract IR's typed error and produce no candidate. | Test |
| FR-037-AC-3 | A canonical enum-member fixture identity is derived through `NominalIdentityPreimage::EnumMember(...).digest`, agrees with the checked reader, and changes when the member case changes. In-process agreement keys that are never compared with a checked-package identity remain classified as fixture-only keys. | Test |
| FR-037-AC-4 | Positive exact-scalar and composite-equality packages remain admitted. A stale candidate key or a body mutation without rekeying is refused by the checked reader; the fixture builder does not repair those negative controls before the assertion. | Test |
| FR-037-AC-5 | The consumer change leaves deterministic fixture-only keys distinct from application identities and uses `quire-canonical` for ordinary canonical-value hashes where those hashes are needed. Package identity refresh uses Contract IR's typed package identity preimage and `quire-canonical`; no independent application or package preimage rule is added. | Inspection |

## Dependencies

Contract IR [FR-046](ix://agent-ix/quire-contract-ir/FR-046) owns the typed
application candidate API and its refusal behavior. Contract IR's checked
reader remains the final identity and wire-admission authority.
