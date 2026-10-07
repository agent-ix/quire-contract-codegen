---
id: "SR-3042"
title: "IR-629 PR 322 integrity analysis: process domain-kind settlement"
type: SpecReview
analysis: integrity
review_set: subset
scope: "agent-ix/quire-contract-codegen PR 322, branch spec/ir-629-fr290-domain-kind (frozen head recorded in the IR-629 Linear review marker, not here); spec/routed/functional/FR-019-capability-settlement.md, spec/routed/matrix/TC-046-process-provider-settlement.md, spec/routed/matrix/tests.md; context spec/decisions/ADR-002-backend-adapter-boundary.md, spec/core/functional/interface-001-codegen-api.md, spec/routed/functional/FR-022-routed-generation.md"
---

# SR-3042: IR-629 PR 322 integrity analysis

## Summary

Ticket: IR-629. This review checks the PR's three files for consistency, completeness and id
integrity, and checks them against these CG documents:

- the merged IR-629 FR-019 text
- the ADR-002 IR-629 amendment
- interface-001's `negotiate_backend_provider` and `BackendKind::from_descriptor` entries
- FR-022

The IR-682, IR-664 and IR-666 specs touch the kani, replay and oracle areas, not routed
settlement, so this change neither overlaps nor conflicts with them.

Results:

- FR-019 has 21 unique AC ids.
- No AC id was added or removed.
- Every AC that TC-046 Expected Results cite exists.
- The routed matrix row still lists FR-019-AC-1 (process arm), AC-11 to AC-13 and AC-16 to
  AC-23 against TC-046.
- `quire validate` on the three files exits 0. It prints module-registry notices
  (`semantic.inline-data-schema`, five `DuplicateArchetype`, one `DuplicateInverseEdge`), so
  the run is not clean. `make spec` exits 0 with the same notices plus two EARS warnings on
  unchanged FR-017 line 174.

## Verdict

Consistent with ADR-002, interface-001 and FR-022. There are two integrity defects. TC-046
asserts that the warning names the kind, which no FR-019 criterion requires, although QSpec
FR-290-AC-13 does. And the boundable and non-boundable kind lists are copied into four places.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | QSpec FR-290-AC-13 requires an `unsupported-requested-capability` decline from the domain-kind check to name the domain kind (and the candidate). FR-019's bounded and unbounded bullets, AC-12 and AC-21 do not require it. TC-046 asserts "naming `population`" for the two unbounded rows only, not for the bounded uncovered-domain rows, so the test asserts something its criteria do not, and only for some cells. | spec/routed/functional/FR-019-capability-settlement.md:155-157, 257, 263; spec/routed/matrix/TC-046-process-provider-settlement.md:47-48, 55-56 |
| FND-002 | low | The ADR-014 boundable set and the non-boundable set are spelled out in FR-019 lines 153-154 and 158, in AC-22, in TC-046 Expected Results 2, and in the TC-046 rows. FR-019 never cites ADR-014 section 4 as the source of that partition, so a change upstream would have to be found and edited in four places. | spec/routed/functional/FR-019-capability-settlement.md:153-158, 264; spec/routed/matrix/TC-046-process-provider-settlement.md:89-90 |

## Dispositions

Round 1, against the fix-round head recorded in the IR-629 Linear dispositions marker.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | fix commit 'spec(IR-629): address process domain review findings' (the fix-round head recorded in the IR-629 Linear dispositions marker): FR-019 lines 149-152 and 166-170, AC-12 and AC-21 require the decline to name the offending domain kind and candidate. TC-046 rows 47, 48, 50, 56 and 57 assert it, including the bounded uncovered-domain rows. |
| FND-002 | fixed | fix commit 'spec(IR-629): address process domain review findings' (the fix-round head recorded in the IR-629 Linear dispositions marker): FR-019 lines 159-165 cite QSL ADR-014 section 4 as the source of the boundable partition. AC-22 no longer lists the kinds. |
