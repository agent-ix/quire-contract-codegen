---
id: "SR-3041"
title: "IR-629 PR 322 EARS conformance analysis: process domain-kind settlement"
type: SpecReview
analysis: ears-conformance
review_set: subset
scope: "agent-ix/quire-contract-codegen PR 322, branch spec/ir-629-fr290-domain-kind (frozen head recorded in the IR-629 Linear review marker, not here); spec/routed/functional/FR-019-capability-settlement.md"
---

# SR-3041: IR-629 PR 322 EARS conformance analysis

## Summary

Ticket: IR-629. This review checks the requirement statements that the PR adds or edits in
FR-019 against EARS grammar and atomicity:

- the unbounded process-provider bullet (lines 150-163)
- the admission bullet (lines 164-169)
- FR-019-AC-12, AC-17, AC-21 and AC-22

`quire validate` reported no EARS warning for FR-019.

The unedited neighbouring bullets (lines 141-149 and 170-178) and AC-16 were read for context
only.

## Verdict

The criteria are testable and use the correct QSpec cause names. Three wording defects remain.
The unbounded bullet packs several normative outcomes, plus one statement with no `shall`, into
a single bullet. The admission bullet mixes another owner's description with CG's one
obligation. And AC-22's causal clause invites two readings of where unavailability comes from.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The unbounded process-provider bullet is not atomic. It holds five outcomes in one bullet: `supported` on an `unbounded` advertisement, the boundable-only comparison, `unsupported-requested-capability`, `requires-bound` and `unbounded-extent`. The sentence about non-boundable kinds ("are not compared and continue to the FR-290 advertised-mode table") has no `shall`. | spec/routed/functional/FR-019-capability-settlement.md:150-163 |
| FND-002 | low | The admission bullet is mostly a description of QSpec/QSL admission ("is refused per registration") with one CG obligation embedded in it ("shall not supply a missing `domains` fallback"). That obligation has no EARS subject-and-condition form of its own. | spec/routed/functional/FR-019-capability-settlement.md:164-169 |
| FND-003 | low | FR-019-AC-22 says "including when a `quantity`, `loop`, or `infinite-trace` kind makes a finite bound unavailable". This can be read as CG deriving unavailability from the kind. FR-019 Inputs and ADR-014 say CG reads `finite_bound_available` and computes nothing. The two readings disagree on the case `finite_bound_available=true` with a non-boundable kind present (unreachable under ADR-014). | spec/routed/functional/FR-019-capability-settlement.md:264 |

## Dispositions

Round 1, against the fix-round head recorded in the IR-629 Linear dispositions marker.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | fix commit 'spec(IR-629): address process domain review findings' (the fix-round head recorded in the IR-629 Linear dispositions marker): the unbounded rule is split into four conditional shall bullets (FR-019 lines 157-177), and the non-boundable sentence now reads "It shall skip". |
| FND-002 | fixed | fix commit 'spec(IR-629): address process domain review findings' (the fix-round head recorded in the IR-629 Linear dispositions marker): the admission bullet is now "When the process arm receives an admitted bounded-capable descriptor, it shall use its manifest domains and supply no missing-domains fallback" (FR-019 lines 178-182). |
| FND-003 | fixed | fix commit 'spec(IR-629): address process domain review findings' (the fix-round head recorded in the IR-629 Linear dispositions marker): FR-019-AC-22 now says "CG reads that flag from the item and does not derive it from the domain kinds". |
