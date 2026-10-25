---
id: SR-514012
title: "EARS conformance review of IR-364 PR 573 shared V2 comparison core"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-codegen; spec/strategy/functional/FR-008-bound-domain-strategy-admission.md (PR #573 head abe97829, base 1944268f)"
review_set: subset
---

## Summary

Ticket: IR-364. EARS review of the requirement statements added or changed in FR-008 by PR #573.
interface-001 contains YAML operation data, not EARS statements, so it was not graded here. The
new statements are ubiquitous and measurable in intent. Their subjects are not the system under
specification: one statement has a compound subject and two have non-system subjects.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | low | Three new SHALL statements lack a single system subject. "The V2 comparison oracle and FR-014's clause oracle SHALL use one ..." has a compound subject. "The planned integer eq/ne derivation of FR-014-AC-35 ... SHALL NOT introduce ..." has another FR's artifact as its subject. "sharing the evaluator SHALL NOT admit a broader clause shape" has an activity as its subject. Recast each with the generator as subject (for example "The generator shall evaluate the selected-claim and FR-014 clause comparisons through one ..."). | spec/strategy/functional/FR-008-bound-domain-strategy-admission.md:29-33; spec/strategy/functional/FR-008-bound-domain-strategy-admission.md:94-98 | wrong-requirement |

## Examined

- Description: "The V2 comparison oracle and FR-014's clause oracle SHALL use one executable
  comparison evaluation core for the operators and operand forms they share." Ubiquitous, but with
  a compound subject.
- Description: "The planned integer eq/ne derivation of FR-014-AC-35 extends that core; it SHALL
  NOT introduce a second evaluator or a V1-to-V2 adapter." The subject is not the system.
- Behavior: "The selected-claim comparison oracle SHALL evaluate its ordered operands through the
  same executable comparison core used by FR-014's clause oracle ..." Ubiquitous and conforming
  (term definition: SR-514010 FND-002).
- Behavior: "sharing the evaluator SHALL NOT admit a broader clause shape or erase the accessor's
  typed refusal and locus." The subject is not the system.

## Verdict

One low finding. Not blocking on its own.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The Behavior bullet now uses the generator as the single subject of each SHALL ("The generator SHALL lower ...", "It SHALL extend ...", "The generator SHALL retain ..."). The Description carries no SHALL. |
