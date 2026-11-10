---
id: SR-17940486709564847047594
title: "IR-304 PR #768 FR-022 per-occurrence requirement records: spec-review ears-conformance"
type: SpecReview
analysis: ears-conformance
date: 2026-11-10
scope: "agent-ix/quire-contract-codegen; PR #768 @ de3d7ee1; IR-304; spec/routed/functional/FR-022-routed-generation.md, spec/routed/matrix/TC-033-routed-generation.md, spec/routed/matrix/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-022
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-033
    type: reviews
---

## Summary

Ticket: IR-304. Spec-only PR #768 (head de3d7ee1, base origin/main cf10ec78) rewrites FR-022's QSL requirement-record intake, adds the 'Per-occurrence requirement records' and 'Path conditions in the scalar harness' subsections, rewrites AC-26/27/28, adds AC-31..35 and an IR-304 Rationale, and updates the TC-033 procedure for slices (a)/(b) and the routed tests.md row. Verified against CG src/routed/generate.rs, src/kani/generate/{spec,scalar}.rs, src/kani/identity.rs, src/replay/scalar.rs, src/core/contract_package.rs, src/oracle/scalar/lower.rs; IR quire-contract-model c8dca8c (ContractOccurrenceKeyV1, result_context); QSL origin/main ffec6c72 (qsl-semantics check/claims.rs PathGuard/RequirementRecord/key_claims, qsl-route request.rs RequirementItem).

## Verdict

PASS. Every new Behavior bullet uses an EARS pattern with the named subject 'the generator': ubiquitous ('shall run/check/look up/emit/set'), event ('When a guard compares ...') or unwanted behaviour ('If a routed Kani item's requirement ... fails', 'If a guard is not one of the two lowerable forms', 'If narrowing empties ...'). None is compound beyond a single response.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-022-behavior-record-check-order | examined | spec/routed/functional/FR-022-routed-generation.md | - The generator shall run the requirement-record checks after the `ScalarSelectionMismatch` check, for routed Kani items in ascending `request_index`, and report the first failing item. - The generator shall check one item in the order `RequirementMissing`, `RecordMissing`, `ApplicationMismatch`, `ResultBoundMismatch`, `ConversionContextMismatch`, `PathConditionMismatch`, `GuardContextMissing`. |
| FR-022-behavior-guards | examined | spec/routed/functional/FR-022-routed-generation.md | - The generator shall emit each lowered guard of an item's path condition as one `Statement::Assume`, in outermost-first order, after every symbolic input and before the oracle call. - When a guard compares a parameter with a literal, the generator shall range each symbolic input bound to that parameter over the parameter's declared interval intersected with the interval the guard admits. - If a guard is not one of the two lowerable forms, then the generator shall settle the item `unsupported` as `PathGuardNotLowered` with no harness. - If narrowing empties a parameter's interval, then the gen |
| FR-022-intake | context_only | spec/routed/functional/FR-022-routed-generation.md | For a routed scalar claim, the driver also hands CG the originating QSL `RequirementItem`'s occurrence key, result-bound type node, direct conversion occurrence and ordered path condition (as a `RoutedRequirement` on the item), together with the checked package's requirement records keyed by occurrence (as `RequirementRecords` on the Kani context). CG binds the item to the record at that exact key, compares the key with the authenticated application occurrence, and compares the record's result bound, direct conversion occurrence and path condition with the item's projection and the authenticat |
