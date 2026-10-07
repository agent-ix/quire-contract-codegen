---
id: "SR-2921"
title: "IR-629 EARS conformance analysis: process-provider settlement"
type: SpecReview
analysis: ears-conformance
review_set: subset
scope: "agent-ix/quire-contract-codegen branch spec/ir-629-input-contract (frozen head recorded in the IR-629 Linear review marker, not here); spec/routed/functional/FR-019-capability-settlement.md, spec/routed/functional/FR-022-routed-generation.md, spec/routed/functional/FR-026-backend-adapter-contract.md"
---

# SR-2921: IR-629 EARS conformance analysis

## Summary

Ticket: IR-629. I checked each new or edited requirement statement and AC for EARS form, a
named system actor, one testable obligation per statement, and the absence of numeric
admission.

Statements examined: the FR-019 process-provider Behavior bullets (lines 117-150),
FR-019-AC-11, FR-019-AC-12, FR-022 Behavior lines 169-175, 235-242, FR-022-AC-1, FR-022-AC-4,
FR-022-AC-5, FR-022-AC-17, FR-026 Description, FR-026 Behavior lines 45-48 and FR-026-AC-5.

Clean units: the FR-019 unbounded bullet (lines 133-139) uses a proper "shall ... only when ...
otherwise" form with the system as actor. FR-022-AC-4 and AC-5 each state one refusal per
sentence. FR-026 Behavior lines 45-48 are well-formed. No statement makes a numeric admission:
the only numeric wording is the prohibition at FR-019 lines 140-143.

## Verdict

Not merge-ready from this lens. FR-019-AC-12 bundles about a dozen independent obligations
into one criterion. FR-022 states a traversal-order "shall" that no output can observe. Two
Behavior statements name a type, not the generator, as the actor, or restrict the arm's
inputs in a way that excludes the input it then compares.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-019-AC-12 is compound. It joins invariance under identity, position, ambient state and run defaults, four bounded and unbounded routing outcomes with their causes, and three prohibitions (`DomainKey` derivation, numeric comparison, temporal-subject reads) in one criterion. Only part of it can be covered before QSL-654, and the matrix cannot say which part. | spec/routed/functional/FR-019-capability-settlement.md:230 |
| FND-002 | medium | The FR-022 traversal-order obligation ("traverse Kani first, then process groups in bytewise order") cannot be observed. `items` are re-sorted by request index, `rejected` can only hold Kani, and the process arm emits nothing. Any traversal order gives identical output, so FR-022-AC-17's clause and TC-046 step 4's "inspect the group traversal" cannot fail. | spec/routed/functional/FR-022-routed-generation.md:169-173, 268 |
| FND-003 | low | FR-022-AC-17 is compound. It covers the empty output, the absence of any adapter, execution or terminal path, retention without context or membership in `ALL`, and permutation invariance. | spec/routed/functional/FR-022-routed-generation.md:268 |
| FND-004 | low | "`BackendKind::ALL` shall list the finite built-in kinds only" makes a constant the actor, not the generator. Also, "settle an item from the candidate's descriptor ... and the requested item's full `extent` alone" excludes the item's capability kind, which the next bullet compares. | spec/routed/functional/FR-019-capability-settlement.md:118-127 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | low | New shall-obligations (classify named candidates by origin; expose a descriptor-to-kind conversion) sit in a 'Measured present fact' paragraph, not in Behavior, and the conversion has no name or signature. | spec/routed/functional/FR-019-capability-settlement.md:186-195 |
| FND-006 | low | FR-019-AC-20 (and to a lesser degree AC-19) is compound: descriptor members, domains affecting coverage, and bounds not affecting it. | spec/routed/functional/FR-019-capability-settlement.md:245, 244 |

## Dispositions

Round 1, against the fix-round head recorded in the IR-629 Linear dispositions marker.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | fix commit 'spec(CG): resolve IR-629 process review findings' |
| FND-002 | fixed | fix commit 'spec(CG): resolve IR-629 process review findings' |
| FND-003 | fixed | fix commit 'spec(CG): resolve IR-629 process review findings' |
| FND-004 | fixed | fix commit 'spec(CG): resolve IR-629 process review findings' |
