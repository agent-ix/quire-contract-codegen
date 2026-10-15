---
id: SR-11044
title: "IR-304 occurrence item base review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; IR-304; spec/routed/functional/FR-022-routed-generation.md, spec/routed/matrix/TC-033-routed-generation.md, spec/routed/matrix/tests.md"
review_set: subset
---

## Summary

Diff-scoped base review of CG head 4e80da1258d0091e996411376aa1103dc0ecad83 and disposition against corrected head e57e37f9afd6cef550f8060a8eec78058233fb8f, QSL origin/main 66fcace109488f1bef273926e92a1c63aafd0d49, QSL FR-062/FR-075 and current source. Ticket: IR-304. No code or test execution is claimed.

## Verdict

Current disposition: PASS on e57e37f9afd6cef550f8060a8eec78058233fb8f; FND-001 fixed. Current QSL origin/main 66fcace provides `RequirementItem::{occurrence, result_bound, conversion_context, path_condition}` and `RequirementRecord::{result_bound, conversion_context, path_condition}`. My earlier high finding came from stale local QSL main and is withdrawn. The remaining wording issue is that the application node is part of the occurrence map key, not a `RequirementRecord` field. FR-022-AC-28 and TC-033 should express the mismatch as key-versus-retained-selection disagreement (or another actually representable input discrepancy), rather than replacing the record's application node.

## Examined scope

- FR-022-AC-26 (examined): Given the checked QSL RR-16 package with two `x + 1` occurrences of one application node, directly narrowed to `Int[0, 10]` and `Int[0, 20]`, the driver supplies two distinct request indexes and occurrence keys. CG matches each to its own requirement record and selected narrow; both supported scalar outputs use the shared application operation and their respective result bounds, and their obligation and harness identities differ by application occurrence. Reversing routed-item order leaves these pairings unchanged.
- FR-022-AC-27 (examined): For `if p then x + 1 else x + 1`, each occurrence's generated scalar claim is conditional on its own QSL path guard: the same guard occurrence with `holds=true` for the then item and `holds=false` for the else item. CG preserves the full outermost-first guard sequence and uses it when forming the proof obligation; a result under `p=false` cannot prove the then claim and one under `p=true` cannot prove the else claim. An empty path condition remains unguarded.
- FR-022-AC-28 (examined): If a routed scalar item's occurrence has no entry in the supplied requirement records, or the keyed record disagrees with its result bound, direct conversion occurrence, application node or ordered path condition, CG returns a typed refusal identifying the request index and occurrence before emitting an oracle, harness or obligation identity for that request; it does not use another record with the same node or reconstruct missing values from the IR graph or source text.
- TC-033 (examined): New procedure and expected results 20-22 for occurrence pairings, guard proof conditions, and keyed-record refusals.
- spec/routed/matrix/tests.md (examined): New planned IR-304 row and TC-033 coverage-status change.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Application node is encoded in the occurrence map key, so a keyed record cannot disagree with its own application-node field; make the check and TC mutation target the key or retained selection | spec/routed/functional/FR-022-routed-generation.md:61,351; spec/routed/matrix/TC-033-routed-generation.md:61 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | FR-022 now compares the occurrence key's node with the authenticated application selection; AC-28 and TC-033 mutate this representable boundary separately from record fields. The initial claim that QSL lacked conversion and guard fields is withdrawn because it came from stale local QSL main; QSL origin/main 66fcace supplies them. |
