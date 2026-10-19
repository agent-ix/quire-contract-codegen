---
id: SR-13295
title: "IR-892 PR 459 gap analysis: typed routed reader outcomes"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen@171768d7dac3204f3d52a175b35e21720f9aa18e vs origin/main merge-base; spec/routed/matrix/tests.md, spec/core/matrix/tests.md, spec/tests.md, spec/core/functional/FR-039-contract-package-consumption.md, spec/routed/functional/FR-019-capability-settlement.md, src/routed/capability.rs, src/oracle/function/mod.rs"
review_set: subset
---

## Summary

Ticket: IR-892. PR agent-ix/quire-contract-codegen#459. This is a planless gap analysis. Plan completion: not assessed. `quoin matrix --json` (quoin 0.28.3) shows FR-019-AC-48 to AC-52 as `tagged`, each with at least one `#[trace("TC-030", ...)]` binder. Test bindings checked:

- AC-48: `tests::imported_function_reader_refusal_is_whole_envelope` and `reader_refusal_projection_preserves_every_public_field`
- AC-49: `reader_incomplete_projection_preserves_the_exhausted_limit` and `oversized_admitted_function_read_stops_the_envelope`
- AC-50: `admitted_function_read_keeps_supported_settlement`
- AC-51: `tests::first_reader_failure_wins_after_an_earlier_supported_item`
- AC-52: `genuine_function_backend_refusals_stay_item_level`

All seven bind to the criterion they assert. `make spec` passes. `spec/tests.md`, the routed TC-030 row and the routed FR-019 row agree with one another.

## Examined criteria

- FR-019-AC-48 (examined): "A `Refused(refusal)` outcome from the current Kani oracle-function preflight read produces `Err(EnvelopeRefusal::ContractPackageReadRefused { refusal: ContractPackageRefusalProjection })` for the whole request, with the reader's `code`, `path`, `cause`, `locus`, `contract_version`, `document_pointer`, `expected` and `actual` copied into the named CG projection; the private `expected_node_id` is absent. No item settlement or artifact is returned."
- FR-019-AC-49 (examined): "An `Incomplete(incomplete)` outcome ... produces `Err(EnvelopeRefusal::ContractPackageReadIncomplete { incomplete: ContractPackageIncompleteProjection })` ... with the reader's `limit_kind`, `limit`, `consumed` and `path` copied into the named CG projection."
- FR-019-AC-50 (examined): "An `Admitted` Kani function-application preflight read whose exact generator produces a claim for an otherwise supported item retains the existing `supported` settlement."
- FR-019-AC-51 (examined): "Among several request items with non-`Admitted` preflight outcomes, the first failing item in manifest/item request order determines the whole-request ... variant and projection, irrespective of outcome kind."
- FR-019-AC-52 (examined): "An `Admitted` Kani function-application read whose exact generator emits no claim, and a structural preflight rejection before any read, each retain item-level `KaniFormRefused { OperationNotRendered }` without a reader envelope refusal."
- FR-039-AC-5 (examined): "... CG preserves the reader's typed refusal or limit result and performs no second parse or checked-graph fallback."

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The new FR-039-AC-5 Analysis says "On the FR-330 production path, only the caller's `Admitted` package reaches CG's typed `&ContractPackageV1` consumer; CG performs no second read or checked-graph fallback there." `negotiate_contract_package` forwards its envelope to `negotiate_backend_provider`, and a `V2FunctionApplication` item there re-lowers and re-reads (see SR-13294 FND-002). No test backs the claim. Fix: make the code match the claim, with a test, or state in the Analysis and the core matrix row that the FR-330 path can still reach the internal preflight re-read. | spec/core/functional/FR-039-contract-package-consumption.md:217-222 |
| FND-002 | medium | The routed matrix marks FR-019-AC-48 through AC-52 "✅ Covered", but AC-48 and AC-49 require every listed field to be copied. Mutants that drop `locus`, `contract_version`, `document_pointer` (refusal) or `path` (incomplete) pass every bound test, so the Covered status overstates the evidence (see SR-13294 FND-003). Fix: add the field-present cases, or mark AC-48 and AC-49 partial until they exist. | spec/routed/matrix/tests.md:15 |
| FND-003 | low | FR-019's IR-892 rationale still describes the current oracle-function path as one that "collapses either non-admitted outcome to `false`" and says AC-48/AC-49 apply "once the code PR preserves its outcome". After this PR that text is stale, while FR-039's Analysis was updated. Fix: update the FR-019 paragraph to describe the path as implemented. | spec/routed/functional/FR-019-capability-settlement.md:448-452 |

## Verdict

**PASS with findings for traceability; FAIL for the evidence claims.** Every new criterion has correctly bound, criterion-level `#[trace]` tests, and the matrix, tests.md and TC-030 rows agree with one another. The Analysis claim that the FR-330 path does no second read, and the Covered status for full field copying, are not yet supported by the code and tests.

## Dispositions

Disposition pass 1, against head d132db3510170a80379ed5975c8d36cc1419ab00.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The FR-330 path now settles from the admitted package with no second read. `admitted_function_settlement_does_not_read_again` (tagged FR-019-AC-50 and FR-039-AC-5) asserts zero internal reads, and the FR-039 Analysis now cites that test. The core matrix row stays Partial, which matches the Analysis |
| FND-002 | fixed | Every field in AC-48 and AC-49 now has a test that fails when the field is dropped, as the reviewer's mutants confirm, so the Covered status for AC-48 to AC-52 is supported |
| FND-003 | fixed | FR-019 now says the internal path "preserves `Refused` and `Incomplete` as typed whole-envelope outcomes under FR-019-AC-48 and AC-49" |
