---
id: SR-2026103022
title: "Base review of IR-1146 structural equality key specification"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; spec/oracle/functional/FR-018-composite-equality-oracles.md, spec/oracle/matrix/tests.md"
review_set: base
---

## Summary

Reviewed CG PR #662 at 64b1429698e505accf538d9c524033509f597003 for IR-1146. The digest-era criterion remains as a retired tombstone, and AC-46 retains claim metadata, structural owner identity, lookup, ordering, malformed/unknown/foreign refusal and no-constructor obligations. The first reviewed head incorrectly marked retired AC-13 covered in the detailed matrix; the current head fixes that row and marks AC-46 Planned. No applicable AssuranceProfile was found in the scoped spec tree.

## Verdict

**READY for the spec-only PR.** The one finding below was fixed on the reviewed head. AC-46 remains untagged in the computed matrix until the separate code leg migrates the historical test trace and supplies structural-key evidence; the matrix states this openly. This review claims no implementation acceptance.

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-018-AC-13 | examined | Retired digest-era declaration-key comparison. FR-018-AC-46 carries forward every claim-entry, ownership, identity, lookup, order and refusal check using structural `NodeKey`; the digest oracle is not reused. |
| FR-018-AC-46 | examined | Every claim-map entry carries the node id, IR id, package id, source map, claims, reconstructed declaration keys and selected schedule of its item. For every V2 node id its operand types reach, the declaration key is the exact owner-issued `NodeKey` selected through admitted QSL lookup: its package, ordered path segments and ordinal equal the admitted node's structural identity. A malformed, unknown or foreign identity retains its typed owning refusal and cannot become a valid key. Claim selection and order remain deterministic; CG and emitted source do not construct or decode keys. |
| FR-018-AC-46 mutation | examined | Key declarations by request ordinal instead of the admitted V2 structural identity, or accept a foreign key as local. |
| TM-003 FR-018 coverage row | examined | FR-018-AC-1, FR-018-AC-3, FR-018-AC-6, FR-018-AC-11, FR-018-AC-12, FR-018-AC-14, FR-018-AC-15: Covered. FR-018-AC-46: Planned (IR-1146). |
| TM-003 TC-029 trace | examined | TC-029 traces FR-018-AC-46 and omits retired FR-018-AC-13; the test case remains Planned. |
| FR-018-AC-36 | context_only | Emitted source uses only WireNodeId and case identifiers for lookup and contains no NodeKey constructor/decoder or generated nominal preimage/hash. |
| TC-029 step 2 | context_only | Reconstructed declaration keys are compared against genuine owner-issued keys corresponding to exact V2 node identities, without backend key construction. |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The detailed FR-018 coverage row credited retired AC-13 through the AC-11–AC-15 range and omitted AC-46, obscuring the pending structural-key evidence. | spec/oracle/matrix/tests.md:35 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | At 64b1429698e505accf538d9c524033509f597003, the covered range excludes AC-13 and a separate AC-46 row states Planned (IR-1146), with the missing structural owner-key, adverse refusal and emitted no-constructor evidence named. |
