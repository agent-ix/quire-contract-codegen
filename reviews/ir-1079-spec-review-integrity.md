---
id: SR-2026102768
title: "integrity review of IR-1079 PR 659"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen; spec/core/functional/FR-037-fixture-identity.md; spec/core/matrix/TC-052-fixture-identity.md; spec/core/matrix/tests.md; spec/oracle/functional/FR-014-exact-scalar-oracles.md; spec/oracle/matrix/TC-024-exact-scalar-oracles.md; spec/oracle/matrix/tests.md; spec/replay/functional/FR-016-witness-native-replay.md; spec/replay/matrix/tests.md"
review_set: subset
---

## Summary

Reviewed exact PR #659 diff at 72a7e769 against merged Contract IR #720 (0fa7cfd) and CG #658 (ed40a1e0). Ticket: IR-1079. No applicable AssuranceProfile was found under spec/.

## Verdict

**FAIL — three high and one medium soundness findings in FR-037, FR-037-AC-9, and FR-016-AC-15/17.**

## Examined Units

- `FR-037-AC-1` (spec/core/functional/FR-037-fixture-identity.md:56, role: examined): RETIRED, replaced by FR-037-AC-6; `derive_application_node_id`, `ApplicationNodeDraft` and application preimage identities no longer exist.
- `FR-037-AC-2` (spec/core/functional/FR-037-fixture-identity.md:57, role: examined): RETIRED, replaced by FR-037-AC-7; `ApplicationIdentityContext` and canonical preimage byte-limit derivation no longer exist.
- `FR-037-AC-3` (spec/core/functional/FR-037-fixture-identity.md:58, role: examined): RETIRED, replaced by FR-037-AC-8; enum-member digest identity no longer defines its structural node ID.
- `FR-037-AC-4` (spec/core/functional/FR-037-fixture-identity.md:59, role: examined): RETIRED, replaced by FR-037-AC-9; body-only changes need an admitted-body comparison, not automatic rekeying.
- `FR-037-AC-5` (spec/core/functional/FR-037-fixture-identity.md:60, role: examined): RETIRED, replaced by FR-037-AC-10; package identity refresh through a typed digest preimage is obsolete.
- `FR-037-AC-6` (spec/core/functional/FR-037-fixture-identity.md:61, role: examined): Both fixture builders preserve every producer-assigned structural application `CheckedNodeId` in the assembled node and its dependent references; an ungrouped fixture's requested ID equals the reader-admitted node ID. Inspection finds no local application ID derivation, application preimage encoder, version constant or digest call for that identity.
- `FR-037-AC-7` (spec/core/functional/FR-037-fixture-identity.md:62, role: examined): A recursion-group fixture retains the complete ordered group, each member's structural ID and ordinal, and references to those exact IDs; the strict reader admits the valid group. Missing, malformed or mismatched group context produces the owner's typed refusal and no admitted fixture. Exceeding the complete-package byte ceiling retains the reader's typed limit result and admits no fixture.
- `FR-037-AC-8` (spec/core/functional/FR-037-fixture-identity.md:63, role: examined): A canonical enum-member fixture carries its producer-assigned structural `CheckedNodeId`; the reader admits its member declaration and exact reference. Changing the case through the producer yields the corresponding structural member ID without a CG nominal preimage or digest. In-process agreement keys never compared with a checked-package identity remain classified as fixture-only.
- `FR-037-AC-9` (spec/core/functional/FR-037-fixture-identity.md:64, role: examined): Positive exact-scalar and composite-equality fixture packages remain reader-admitted. A stale or mismatched structural key is refused by the reader. A changed body under the same structural key is detected by comparison with the expected admitted source body and is not silently credited as the original fixture; neither negative control is repaired before its assertion.
- `FR-037-AC-10` (spec/core/functional/FR-037-fixture-identity.md:65, role: examined): Deterministic fixture-only keys remain distinct from canonical checked-package identities. Ordinary canonical-value identities use their owning API where needed; originating package identity comes from the producer and reader admission, with no CG package preimage, package digest refresh or independent application identity rule.
- `FR-014-AC-5` (spec/oracle/functional/FR-014-exact-scalar-oracles.md:583, role: examined): RETIRED, replaced by FR-014-AC-56; the per-node IR id and digest-shaped package identity are obsolete.
- `FR-014-AC-56` (spec/oracle/functional/FR-014-exact-scalar-oracles.md:634, role: examined): Every claim-map entry retains its item’s exact requested structural `CheckedNodeId`, the admitted `ContractSourceV1` package name and complete immutable lock as `contract_package_identity`, originating checked-package structural `package_id`, admitted `source_map`, typed `claims` and `bounds`, and the item’s operation identity and provenance; a `quire.op.quantity.convert` entry also records its `type_argument` structural type node ID (FR-014-AC-48). No generated scalar claim serializes an `ir_id` or derives a substitute node or package digest.
- `FR-016-AC-15` (spec/replay/functional/FR-016-witness-native-replay.md:149, role: examined): The replay request's `package.dependencies` carries one entry per lock dependency selection, in ascending identity order, with the lock's recorded `package_id`.
- `FR-016-AC-17` (spec/replay/functional/FR-016-witness-native-replay.md:151, role: examined): A lock recording another `package_id` for a dependency the unit imports is refused by QSL as a dependency identity mismatch.
- `FR-037` (spec/core/functional/FR-037-fixture-identity.md:40, role: examined): Each grouped application SHALL keep its producer-assigned identity and consistent group ordinal and reference targets across the complete ordered group.
- `TC-052` (spec/core/matrix/TC-052-fixture-identity.md:13, role: examined): Verify that both checked-package fixture builders preserve producer-assigned
- `TC-024` (spec/oracle/matrix/TC-024-exact-scalar-oracles.md:15, role: examined): Verify that scalar oracles generated from an admitted CheckedPackage V2 cover

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The Behavior requires a “consistent group ordinal,” but Contract IR #720 has no group ordinal or group position. It defines only producer-assigned node_id.ordinal, with no contiguity or sibling ordering requirement. Specify preservation of each member’s exact node ID and reference targets; avoid implying a group-position invariant. | spec/core/functional/FR-037-fixture-identity.md:40-43 |
| FND-002 | high | “A stale or mismatched structural key is refused by the reader” is overbroad. Contract IR #720 explicitly removes stale-content and stale-node-key refusal, and allows producer-assigned noncontiguous ordinals. A key that differs only in a valid unused ordinal can admit when references are consistent. Constrain this negative control to a reader-enforced mismatch such as declaration path versus node_id.path; keep body-only comparison separate. | spec/core/functional/FR-037-fixture-identity.md:64 |
| FND-003 | high | The selected QSL 4926 `DependencyEntryWire` contains `identity` and `sources`, with no `package_id` member. This criterion requires a wire field the owner no longer accepts. Retire it and specify ordered dependency identities and exact selected source references in the replacement. | spec/replay/functional/FR-016-witness-native-replay.md:149 |
| FND-004 | high | QSL 4926 dependency entries carry source references, not a lock `package_id`. A package-id mismatch control cannot reach the stated owner refusal. Retire it and require a mismatched selected source-reference refusal without narrowing the identity-authentication obligation. | spec/replay/functional/FR-016-witness-native-replay.md:151 |

## Source Evidence

- Contract IR #720, commit 0fa7cfd, FR-038 Description: `node_id.ordinal` is producer assigned; the reader does not derive group position, rekey, or seal node identity.
- CG #658, commit ed40a1e0, FR-039-AC-11: exact source name and lock identify generated claims with requested structural node ID.
