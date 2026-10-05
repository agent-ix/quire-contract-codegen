---
id: SR-1567
title: "scope-boundary review of IR-629 process-provider BackendKind spec (quire-contract-codegen#287)"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/quire-contract-codegen@43fbf2f6083663e680e1ebeeeeefe11b288fdef5; spec/routed/functional/FR-019-capability-settlement.md, spec/routed/functional/FR-022-routed-generation.md, spec/decisions/ADR-002-backend-adapter-boundary.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-019
    type: reviews
---
# Scope-boundary review: IR-629 process-provider BackendKind (PR #287)

## Summary

Ticket: IR-629. Checked each new FR-019 statement against QSL ADR-029 PV-4 (QSL origin/main
a9cfe11, lines 458-465) and the Amendments line (808-809) for widening, and the CG/driver/QSL
allocation. PV-4 text: one variant for process providers; the arm settles from the manifest alone
(advertised (kind, mode) pairs, domains and bounds) against the extent classification under FR-290;
never calls the plugin; the driver's pre-negotiation conversion maps every plugin `BackendId` to the
variant. The spec matches each of these. Statements beyond PV-4 are CG-local consequences already
stated elsewhere (`ALL` membership per FR-019-AC-1/TC-030 census; disposition-only per FR-019
Outputs and ADR-002 Q3). The paired adaptation (public enum break; driver conversion) is listed and
no other repository is edited. Disposition vocabulary: only FR-019's four `Disposition` values are
used; no third vocabulary beside `Disposition` and FR-015's `ObligationDisposition` is introduced,
and FR-029/FR-030 terminal maps are untouched.

## Verdict

Changes requested (one medium). Widening relative to PV-4 is limited to FR-019-AC-12's bounded
rows, recorded as SR-1565 FND-003; the allocation defect is below.

## Examined

- FR-019 Behavior bullets lines 102-115 against PV-4 (examined; bullets 102-113 within PV-4)
- FR-019 paired-adaptation paragraph and open questions 1-4 (examined, clean)
- `BackendKind::from_identity` call sites: capability.rs:582, 633; generate.rs:268 (context_only)

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The bullet "shall leave the mapping from a plugin `BackendId` to the process-provider variant to the driver's pre-negotiation conversion" has no acceptance criterion, and it pre-empts open question 2, which lists "or does `from_identity` change?" as an option, that is, CG doing the mapping. CG already converts identity to kind itself through `BackendKind::from_identity` in `unroutable_named_backend`, `negotiate_single_candidate` and `refuse_inconsistent_routing`; the bullet does not say whether those conversions stay, go, or must agree with the driver's | spec/routed/functional/FR-019-capability-settlement.md:114-115,128-132 |

## Dispositions

Round 1, reviewed at 48a7a96eca9d7dbc66a3c7a808f042151da2607c. New defects found this round are recorded in SR-1570.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 48a7a96 |
