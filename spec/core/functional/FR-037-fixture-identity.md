---
id: FR-037
title: "Preserve producer-assigned structural checked-package fixture identities"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-ir/FR-038
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-018
    type: references
---
# FR-037: Preserve producer-assigned structural checked-package fixture identities

## Description

When the exact-scalar or composite-equality fixture builder assembles a
checked-package V2 graph, it SHALL preserve the producer-assigned structural
`CheckedNodeId` of each application and canonical enum member. It SHALL use
those identities consistently in node declarations, grouped members, references
and dependent source correspondence. It SHALL NOT derive a node ID from its
body, an application or nominal preimage, or a local digest. An in-process key
used solely to distinguish direct/generated agreement fixtures is fixture data,
not a canonical checked-package identity.

The builder SHALL submit the complete package to Contract IR's strict reader.
Only the reader's admitted result authorizes use of the fixture; assigning a
structural key locally does not establish admission. A body-only mutation may
leave a structural key unchanged, so the negative control SHALL compare the
body represented by the admitted source with the changed body rather than
expecting an identity mismatch.

## Behavior

- Each ungrouped application and canonical enum member SHALL keep the exact
  structural identity supplied by its producer, including when its body is
  assembled after its declaration.
- Each grouped application SHALL keep its producer-assigned identity and
  reference targets for every member of the complete group. Each member's
  `node_id.ordinal` SHALL remain the producer-assigned value; it is not a group
  position and need not be contiguous. A missing referenced member or a
  declaration path inconsistent with its structural ID SHALL receive the
  reader's typed refusal, without a synthesized candidate key.
- The builder SHALL preserve the reader's typed byte-ceiling refusal when the
  complete package exceeds the configured limit. It SHALL NOT recast that
  refusal as an identity derivation error.
- Positive exact-scalar and composite-equality packages SHALL reach reader
  admission. The builder SHALL leave a declaration-path/key mismatch
  uncorrected for the reader to refuse. It SHALL separately detect a changed
  body under an unchanged structural key by comparing with the expected
  admitted source body.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-037-AC-1 | RETIRED, replaced by FR-037-AC-6; `derive_application_node_id`, `ApplicationNodeDraft` and application preimage identities no longer exist. | Inspection |
| FR-037-AC-2 | RETIRED, replaced by FR-037-AC-7; `ApplicationIdentityContext` and canonical preimage byte-limit derivation no longer exist. | Inspection |
| FR-037-AC-3 | RETIRED, replaced by FR-037-AC-8; enum-member digest identity no longer defines its structural node ID. | Inspection |
| FR-037-AC-4 | RETIRED, replaced by FR-037-AC-9; body-only changes need an admitted-body comparison, not automatic rekeying. | Inspection |
| FR-037-AC-5 | RETIRED, replaced by FR-037-AC-10; package identity refresh through a typed digest preimage is obsolete. | Inspection |
| FR-037-AC-6 | Both fixture builders preserve every producer-assigned structural application `CheckedNodeId` in the assembled node and its dependent references; an ungrouped fixture's requested ID equals the reader-admitted node ID. Inspection finds no local application ID derivation, application preimage encoder, version constant or digest call for that identity. | Test and Inspection |
| FR-037-AC-7 | A recursion-group fixture retains the complete group, each member's exact producer-assigned structural ID, including its own ordinal, and references to those exact IDs; the strict reader admits the valid group without imposing contiguous group positions. A missing referenced member or a declaration path inconsistent with `node_id.path` produces the reader's typed refusal and no admitted fixture. Exceeding the complete-package byte ceiling retains the reader's typed limit result and admits no fixture. | Test |
| FR-037-AC-8 | A canonical enum-member fixture carries its producer-assigned structural `CheckedNodeId`; the reader admits its member declaration and exact reference. Changing the case through the producer yields the corresponding structural member ID without a CG nominal preimage or digest. In-process agreement keys never compared with a checked-package identity remain classified as fixture-only. | Test and Inspection |
| FR-037-AC-9 | Positive exact-scalar and composite-equality fixture packages remain reader-admitted. A declaration whose path disagrees with `node_id.path` is refused by the reader. A changed body under the same valid structural key is detected separately by comparison with the expected admitted source body and is not silently credited as the original fixture; neither negative control is repaired before its assertion. A different valid unused ordinal with consistent references is not itself a reader refusal. | Test |
| FR-037-AC-10 | Deterministic fixture-only keys remain distinct from canonical checked-package identities. Ordinary canonical-value identities use their owning API where needed; originating package identity comes from the producer and reader admission, with no CG package preimage, package digest refresh or independent application identity rule. | Inspection |

## Dependencies

Contract IR [FR-038](ix://agent-ix/quire-contract-ir/FR-038) owns strict
checked-package admission and typed structural, graph and byte-limit refusals.
The checked-package producer assigns structural IDs; CG's builders preserve
them and do not become an identity authority.
