---
id: SR-17940486709564847047737
title: "IR-1448 PR #806 spec-review integrity"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen PR #806 at c2a8f9e0; IR-1448; spec/kani/functional/FR-055-option-clause-harnesses.md, spec/kani/matrix/TC-069-option-clause-harnesses.md, spec/kani/matrix/tests.md, spec/spec.md, docs/qualification/ir1448-kani-option-probe.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-055
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-069
    type: reviews
---

## Summary

Ticket: IR-1448. Integrity check of FR-055 frontmatter and relationships, AC atomicity,
TC-069 step-to-AC coverage, and the registry/matrix rows. Frontmatter edges resolve
(StR-001, FR-014, FR-015, FR-017, FR-025, FR-028); TC-069 verifies FR-055; every AC is
reached by a TC-069 step (steps 1-9 cover AC-1..10); both index rows link correctly.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | medium | FR-055-AC-7 and FR-055-AC-9 are multi-obligation: AC-7 bundles four distinct refusal reasons, the not-NoFiniteEncoding rule, sibling retention and three mutants; AC-9 bundles identity retention, identity change on encoding change, decode of present/absent playback and refusal of a non-canonical absent playback. One reason failing cannot be reported against its own criterion. Split per reason / per obligation (lane rule: single-obligation ACs). AC-1 (shape + enumeration) and AC-2 (lowering + healthy proof + mutant) are milder cases of the same. | spec/kani/functional/FR-055-option-clause-harnesses.md:132-133, 138, 140 |
| FND-002 | low | Two criteria have no failing control: FR-055-AC-8 names no mutant (e.g. a lowering that remaps the unbounded payload to `PayloadNotScalar`), and FR-055-AC-6 has no control for its "every leaf canonical when absent" clause (a composite analogue of AC-1's free-absent-payload mutant). | FR-055-AC-6, FR-055-AC-8 |
| FND-003 | low | FR-055-AC-10 has no pass/fail outcome beyond "executed" and "recorded": no expected verdicts restated, no wall/memory ceiling, and "a cgroup the request names" sits oddly with ADR-007/IR-1510 where the memory cap is the caller environment's and peak memory is recorded only when the environment supplies a source. | FR-055-AC-10 |

## Verdict

Structure, links and status are sound; atomicity is the gap. No high finding here.

## Dispositions

Round 1, reviewed head a9adfb39b95bb563fd3e4f64c8f277c9fe0cc0b3.

| FND | outcome | reason |
|---|---|---|
| FND-001 | fixed | 23 ACs. Each refusal reason has its own AC (AC-10 PayloadNotScalar, AC-11 NestedOption, AC-12 NullablePresence, AC-13 OptionOperationNotLowered, AC-14 not-other-codes, AC-15 sibling). Identity, decode and playback refusal are split into AC-18, AC-19 and AC-20. Shape and enumeration are split into AC-1 and AC-2. |
| FND-002 | fixed | AC-16: "Mutant: remapping it to `PayloadNotScalar` fails the disposition check." AC-9: the absent composite leaves are canonical, with 2003 draws and the mutant "leaving the absent Boolean free finds the absent value twice, at Boolean true". |
| FND-003 | fixed | AC-21 now names the expected verdicts per AC, "the memory cap supplied by the caller's environment (ADR-007)", peak memory "only where the environment supplies a source", and "A run that ends `TimedOut` or `Inconclusive` leaves the control unmet." |
