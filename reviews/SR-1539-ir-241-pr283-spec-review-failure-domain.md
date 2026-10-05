---
id: SR-1539
title: "Failure-domain review of quire-contract-codegen PR #283 (shadow + refinement soundness)"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-contract-codegen@1f3025ea99c3fe1fa804ed43fce116e339f9c96d; FR-018-AC-21, FR-028-AC-14..18, FR-015-AC-69..76; Contract Runtime 9597b43 src/exact/equality.rs (context)"
review_set: subset
---
# Failure-domain review of PR #283

## Summary

Ticket: IR-241 (also IR-264). This review asks whether the shadow and refinement design can
silently lose a bug in the production oracle. Most of the design is labelled honestly. A
`bounded_shadow` result never reads `production_proved` inside CG (FR-028-AC-17). Each named
abstraction (`representation`, `integer_widening`, `absent_payload`, `unrolled_walk`) has to carry
its discharging evidence or be refused (FR-028-AC-18). The excluded behaviours are listed in the
identity, and a refinement case the production oracle does not complete is a disagreement, never a
skipped case (FR-028-AC-15).

Two escape paths are not named, and nothing in the design catches them. SR-1534 records two more
that cross requirement boundaries: proof strength does not reach the FR-029 terminal value, and the
excluded behaviours are mapped to native criteria that are unbacked.

## Scope examined

FR-018-AC-21 (examined), FR-028-AC-14 to AC-18 (examined), FR-015-AC-69 to AC-76 (examined), and
Contract Runtime `plan_pairs` and `planned_equality` (context_only). `plan_pairs` walks the two
values and never consults the declaration closure.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | A fault in the declaration reconstruction escapes every check. FR-018-AC-21 requires the claim-map closure to be read "from the same reconstruction the item's environment constructor is emitted from", and checks it against that constructor's `TypeEnvironment`, which comes from the same source. Several things derive from that closure: the shadow's domain (bounds and presence), the abstraction function, the refinement domain and the identity's "declared domain". Suppose the reconstruction narrows a bound or reads `T?` as `T`. The shadow domain and the refinement domain then shrink together, and the identity records the narrowed domain as declared. Runtime `plan_pairs` walks values and never reads the closure, so refinement agrees. Kani, the refinement run and FR-018-AC-21 all pass. No unexercised behaviour names this. Either check AC-21's closure against the checked package's declared types through an independent read, or list a `declaration_reconstruction` behaviour with its covering criterion. | FR-018-AC-21; FR-028-AC-18; FR-015-AC-76 |
| FND-002 | medium | The refinement run has no bound and no evidence home. FR-028 bounds every Kani run, but the native refinement run, whose case cap can be any positive integer, has no wall-clock or memory ceiling and no named executor. FR-017 is Kani-only. There is also no outcome for a refinement that is killed or does not finish: it is not `not_run`, not `sampled` and not `refinement_failed` under FR-028-AC-17. FR-028's own title promises to "bound every Kani proof so it finishes". The refinement half of the proof needs the same treatment, either through a ceiling and an inconclusive strength or by stating that it runs under FR-017's ceilings. | FR-028-AC-16; FR-028-AC-17; FR-028 Inputs (refinement case cap) |

## Verdict

The design is sound in its main structure. Kani proves the shadow against an FR-149 expectation,
and the production oracle has to agree with the shadow case by case on verdict and pair count, with
no refusals tolerated. Both findings should be fixed or recorded explicitly before the code change
starts, because each one is a way for a green result to overstate what was proved.

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed 3e0bc7c | The new FR-018-AC-23 checks each claim-map closure's member names, presence and integer bounds against an independent read through Contract IR's reader. It has a mutation row and TC-029 step 15. FR-015-AC-76 lists `declaration_reconstruction` covered by FR-018-AC-23. |
| FND-002 | fixed 3e0bc7c | The new FR-028-AC-24 runs the refinement through one typed entry that reuses FR-017's launcher, under the identity's memory and wall-clock ceilings, with evidence beside the shadow run. A run that reaches a ceiling yields `shadow_proved_refinement_inconclusive` (added to FR-028-AC-17), with a mutation row and TC-039 step 22. |
