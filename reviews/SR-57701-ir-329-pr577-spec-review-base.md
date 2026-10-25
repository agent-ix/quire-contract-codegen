---
id: SR-57701
title: "IR-329 PR 577 Kani module ownership map base spec review"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-codegen@21f28b1bec5df6471dd6735214ce3145545d21a4; spec/spec.md (Kani module and public-surface ownership map, four added rows)"
---

## Summary

Ticket: IR-329. PR: quire-contract-codegen#577, frozen head 21f28b1b over base b60b18ef (CG
main at review time). The diff adds four rows, and changes no existing row, to the Kani
module and public-surface ownership map in `spec/spec.md`. This base review checks each added
row's owner and status against the code on main, FR-015 and AD-004.

## Examined scope

- Row `kani/generate/{record,spec,render}`: examined. The files exist. AD-004 places
  `record.rs`, and places `spec.rs` and `render.rs` at step 4b. FR-015-AC-112 to AC-117 are
  Implemented (IR-858). `scalar.rs` and `clause/v2.rs` call `render::render`. `collection.rs`
  already imports `record`.
- Row `kani/generate/{collection,clause/v2}`: examined. Both files exist. `clause/v2.rs` is
  only the "bounded cover harness for an admitted V2 parameter precondition", which
  `negotiate.rs` wires in. FR-015-AC-38 to AC-49 are all PLANNED (IR-489).
- Row `kani/generate/census_validation` and `kani/generate/corpus/mod.rs`: examined.
  Checked against AD-004's `census_validation.rs` entry and the FR-015-AC-109 to AC-111
  slices.
- Row `role_parity_tests`, `test_support` and module roots: examined. See SR-57702.
- Existing rows 1 and 3 (`{negotiate,...}`, `{clause,v1_bundle}`): context only, compared
  for contradiction.
- FR-015 "Planned (IR-489)" paragraph and "Partly implemented (IR-268)" paragraph: context only.

## Verdict

CHANGES REQUESTED. Every path named exists on main. No existing row or acceptance criterion
changed. The diff adds no pin or SHA. `quire validate --scope . spec/spec.md` and
`quire matrix` both pass. Two status cells say more than the code and FR-015 support.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The `{collection,clause/v2}` row says "V2 clause and collection lowerers are live". `clause/v2.rs` only lowers the V2 parameter-precondition cover. FR-015-AC-38 to AC-49 are all PLANNED (IR-489), and the existing `{negotiate,...}` row still says "V2 contract-clause items planned". The cell should say that only the V2 parameter-precondition cover is live and that AC-38 to AC-49 remain planned. | spec/spec.md:132 |
| FND-002 | low | The `{record,spec,render}` row groups `record` with the unported pieces: "live for scalar and admitted V2 clause paths; collection and remaining families still need their AD-004 ports". `collection.rs` already uses `record`. Only `HarnessSpec` and `render` are still unported for collection (step 4h). | spec/spec.md:131 |

## Dispositions

Round 1, checked against fix-round head 59b9b4e031340a6642cdb160408c12f0f1b11d71.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | `{collection,clause/v2}` is split into two rows. The `clause/v2` row now says only the parameter-precondition cover harness is live and that FR-015-AC-38 to AC-49 remain PLANNED (IR-489). This matches `clause/v2.rs` and FR-015. |
| FND-002 | fixed | The `{record,spec,render}` row now says the shared record is already used by collection and that only the step 4h rendering port remains. This matches the `record::` import in `collection.rs`. |
