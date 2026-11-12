---
id: SR-17940486709564847047736
title: "IR-1448 PR #806 spec-review ears-conformance"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-codegen PR #806 at c2a8f9e0; IR-1448; spec/kani/functional/FR-055-option-clause-harnesses.md, spec/kani/matrix/TC-069-option-clause-harnesses.md, spec/kani/matrix/tests.md, spec/spec.md, docs/qualification/ir1448-kani-option-probe.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-055
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-069
    type: reviews
---

## Summary

Ticket: IR-1448. EARS check of the eleven FR-055 Behavior statements (Draw 2, Operations 4,
Refusal 5, Evidence 2). All use the event form (`When ...`, system subject, SHALL). quire's
grammar lint reports no EARS warning for FR-055.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | low | Four Behavior sentences carry two SHALL obligations, against the one-SHALL-per-statement lane rule: absent read (oracle SHALL stop ... and the harness SHALL NOT substitute), guarded read (oracle SHALL evaluate ...; absent draw SHALL complete), refusal tail (refusals SHALL NOT map ..., and a refused item SHALL NOT discard a sibling), evidence (playback SHALL decode ... and SHALL refuse). Split each into its own statement. | spec/kani/functional/FR-055-option-clause-harnesses.md:85-88, 90-93, 112-114, 120-122 |
| FND-002 | low | Two statements are not event-triggered requirements but ubiquitous constraints written without a trigger: "The refusals SHALL NOT map to ..." and "The harness identity record SHALL retain ...". Acceptable as ubiquitous EARS, but the identity statement's "so that a change of encoding changes the identity" is a rationale clause inside the SHALL. | FR-055:112, 118-120 |

## Verdict

The statements are well formed and individually testable apart from the compound sentences
above; the substantive scope problems are recorded in the base SR. Low findings only.

## Dispositions

Round 1, reviewed head a9adfb39b95bb563fd3e4f64c8f277c9fe0cc0b3.

| FND | outcome | reason |
|---|---|---|
| FND-001 | fixed | The Behavior statements are split to one SHALL each, e.g. "When a clause applies `quire.op.option.value` to an absent option, the oracle SHALL stop with `ClauseStop::Undefined`." / "The harness SHALL NOT substitute a default value or a canonical payload for an absent read." / "The harness SHALL NOT add an assumption that excludes the absent draw." Playback decode and refusal are now separate When-statements. |
| FND-002 | fixed | The identity statement now reads "The harness identity record SHALL retain the presence leaf, the payload leaves and the canonical-absent rule." with the rationale clause removed. The ubiquitous "The refusals SHALL NOT map to ..." is acceptable EARS. quire grammar lint: no FR-054 warning. |
