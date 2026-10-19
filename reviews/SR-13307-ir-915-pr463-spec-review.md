---
id: SR-13307
title: "IR-915 PR 463 spec review: excluded accounting-only CG positions"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen@fc288843 vs origin/main merge-base; spec/core/functional/FR-039-contract-package-consumption.md, spec/routed/functional/FR-019-capability-settlement.md, spec/routed/functional/FR-022-routed-generation.md, spec/core/matrix/TC-053-contract-package-consumption.md, spec/routed/matrix/TC-030-capability-settlement.md, spec/core/matrix/tests.md, spec/routed/matrix/tests.md"
review_set: subset
---

## Summary

Ticket: IR-915. PR agent-ix/quire-contract-codegen#463 changes spec files only. It adds
FR-039-AC-9 and FR-019-AC-53, changes the FR-039, FR-019 and FR-022 statements, and adds
TC-053 step 7, a TC-030 procedure and planned matrix rows.

This review checked the PR against the CG code and against quire-driver `origin/main`.

- CG code: `PackageItems::new` (`src/core/contract_package.rs:194-223`, Count and
  AccountingIdentity), `driver_to_accounting` and `has_context_root` (`:205-264`),
  `negotiate_contract_package` with `EnvelopeItemCount` (`src/routed/capability.rs:1499-1538`),
  `ItemSettlement.request_index` (`:1443-1460`) and the generation `driver_selections`
  (`src/routed/generate.rs:489-509`).
- quire-driver: FR-001-AC-15, FR-001-AC-17, the FR-001 Status note, FR-010-AC-4 and TC-017.
  These came from STD-200 (quire-driver#84, merged) and STD-224 (quire-driver#85, merged).

The ticket's three constraints are each addressed:

1. Positions must equal `feature_accounting`. FR-039 and AC-9 keep the excluded position in
   the full correspondence, including its `node_id`, and refuse a missing, extra or changed
   position.
2. There is one envelope item per `DriverItem`. FR-019 and AC-53 count envelope items over bound
   items only.
3. Indexes are numbered over `DriverItem` entries only. FR-039, FR-022 and AC-9 count excluded
   positions in driver indexes and exclude context roots.

The semantics match the driver side. In the A/B/C vector, A keeps index 0, the excluded B keeps
index 1 and its `node_id`, C keeps index 2, B is absent from the FR-331 envelope, and B has no
claim or result selection. Context roots do not advance the index.

Neither new id appears anywhere in `origin/main` history (`git log -S`), so no retired id is
reused. The matrix rows are marked Planned, which is true: `quire matrix` shows both criteria
untagged. FR-019-AC-48..52 and the FR-039 context-root rules are not contradicted, but see
FND-003.

## Examined criteria

- FR-039-AC-9 (examined): "In an admitted package with a bound driver item, an excluded accounting-only position and another bound driver item, CG matches all three positions and exact `node_id` values to `feature_accounting`; the excluded position keeps its driver index but has no claim, routed item or generated artifact, and the following bound item keeps its original driver index. ..."
- FR-019-AC-53 (examined): "Given an admitted package with bound driver items on either side of an excluded accounting-only position, an FR-331 envelope with exactly two items settles those bound items at their original driver indexes without an `EnvelopeItemCount` refusal. ..."
- FR-039 statement, new excluded-position bullet (examined): "CG SHALL map an excluded accounting-only position to its exact admitted `feature_accounting` position and `node_id`, advance the driver request index over it, and consume no FR-331 envelope item for it. ..."
- FR-019 statement, new admitted-path bullet (examined): "On the admitted package path, CG shall check the envelope item count against bound driver items, not excluded positions or context roots. ..."
- FR-022 Inputs, `ContractRequestPosition` bullet (examined): "an excluded accounting-only position carries its original key and occupies a driver index but is never routed ..."
- FR-039-AC-2 and AC-8 rewording, "bound driver item" (examined).
- FR-019-AC-48 through AC-52 (context_only), checked for contradiction.
- FR-022 `request_index` input (context_only): "the item's index in the driver's request, as FR-019's `ItemSettlement::request_index` numbered it".
- quire-driver FR-001-AC-15, FR-001-AC-17, FR-010-AC-4 and TC-017 (context_only).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Two index spaces now exist, and the spec does not say which one `ItemSettlement::request_index` carries. FR-022 defines `RoutedGenerationItem.request_index` as "the item's index in the driver's request, as FR-019's `ItemSettlement::request_index` numbered it". Today `ItemSettlement::request_index` is the FR-331 envelope position (`enumerate()` over `envelope.items`). Once excluded positions leave the envelope, item C has envelope index 1 but driver index 2. AC-53 says only "settles those bound items at their original driver indexes". It does not say whether that holds for both the outer `ContractItemSettlement.request_index` and the nested `backend: ItemSettlement.request_index`. A test can pass while the nested settlement, and every routed item built from it, carries the envelope index. Fix: in FR-019, state that on the admitted path every settlement `request_index` is the driver index. Have AC-53 assert it on both the outer and the nested settlement, or repoint FR-022 at `ContractItemSettlement::request_index`. | spec/routed/functional/FR-019-capability-settlement.md:526; spec/routed/functional/FR-022-routed-generation.md:157-158; src/routed/capability.rs:1443-1449,1513-1536 |
| FND-002 | medium | The phrase "no routed item" in FR-039-AC-9 cannot be observed. CG never creates routed items: the driver builds them with `RoutedGenerationItem::from_descriptor(request_index, node_id, ...)`. The real adverse case is a routed item whose `request_index` and `node_id` name an excluded position. No criterion covers it, although FR-022 now says an excluded position "is never routed". Today `generate_kani` indexes `driver_selections`, which is filtered to `DriverItem`. After the change, an implementation could silently pick the wrong item's selection, or accept the excluded one, and still pass AC-9. Fix: replace "no routed item" with an FR-022/TC-033 criterion. A routed item at an excluded driver index refuses the whole call with a typed error before any harness. | spec/core/functional/FR-039-contract-package-consumption.md:205; spec/routed/functional/FR-022-routed-generation.md:141-147; src/routed/generate.rs:489-509 |
| FND-003 | medium | The new rule "An excluded position ends the preceding bound item's contiguous context span and owns no context span" has no criterion. It also leaves undefined a `ContextRoot` placed directly after an excluded position, which belongs to no item: the spec neither accepts nor refuses it. TC-053 step 7 puts the context root before the excluded position, so the span-ending rule is never exercised. Existing `has_context_root` stops at the first non-`ContextRoot`, so the rule is implementable, but nothing would catch an implementation that skips over excluded positions. Fix: add a control to AC-9. A conversion root placed after the excluded position yields `IncompleteContext` for the preceding bound item. Also state whether an orphan context root after an excluded position is accepted or is a correspondence refusal. | spec/core/functional/FR-039-contract-package-consumption.md:93-95,154-156; spec/core/matrix/TC-053-contract-package-consumption.md:78-86; src/core/contract_package.rs:256-264 |
| FND-004 | medium | AC-53 and AC-9 do not require the excluded position's admitted accounting to be `lowered`. The statement explicitly covers that case ("even if its accounting record is `lowered`"), and it is the only case in which "no negotiation call" can fail. With a refused record, CG never classifies or reads anyway. An implementation that classifies or runs the Kani preflight for the excluded position, then drops the result, passes AC-53 with a refused-record fixture. Fix: require the excluded record to be `lowered` with a Kani-supported form. Assert zero classifier calls and zero preflight reads for it, for example with the existing `take_function_preflight_reads` counter. | spec/routed/functional/FR-019-capability-settlement.md:164-168,526; spec/routed/matrix/TC-030-capability-settlement.md:37-43 |
| FND-005 | low | The term "bound driver item" is introduced without a definition, and "driver item" now has two readings. FR-039 Inputs types each root as "a driver item, an excluded accounting-only position, or a context-only root", and FR-022 says "Driver indexes count driver items and excluded positions", so an excluded position is not a driver item. FR-039:79 ("map each driver item to its own accounting position") and FR-039:154-156 ("before the next driver item") were not updated, and the new text says "bound driver item". quire-driver calls the bound kind `DriverItem`. Fix: define "bound driver item" once as a `DriverItem` position, name the excluded kind, and use the terms consistently in FR-039, FR-019 and FR-022. | spec/core/functional/FR-039-contract-package-consumption.md:49-55,79,154-156; spec/routed/functional/FR-022-routed-generation.md:141-147 |

## Verdict

**PASS with findings (not yet mergeable as-is).** The direction is right and agrees with
quire-driver's merged FR-001-AC-15 and FR-010-AC-4. All three IR-915 constraints are addressed,
the new ids are fresh, and the matrix rows truthfully say Planned.

Before this lands as the contract the implementation will be tested against:

- FND-001: define which index the nested settlement carries.
- FND-002: add a refusal criterion for a routed item that names an excluded index.
- FND-003: add a span-end control.
- FND-004: require a `lowered` excluded record.
