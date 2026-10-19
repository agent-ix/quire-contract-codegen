---
id: "SR-13171"
title: "CG PR 453 EARS analysis: the new FR-019 Behavior statements"
type: SpecReview
analysis: ears-conformance
review_set: subset
scope: "agent-ix/quire-contract-codegen@bf69bb4f23a976e8a858ac16460f1c740ab491ae; spec/routed/functional/FR-019-capability-settlement.md (Outputs bullet at 107-109, Behavior bullets at 121-143, AC-10 and AC-45 at 425-426) (diff origin/main...HEAD)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-019
    type: reviews
---

# SR-13171: CG PR 453 EARS analysis

## Summary

Ticket: IR-887. PR: agent-ix/quire-contract-codegen#453 at bf69bb4. This pass covers only the
statements the PR adds or rewords. `make spec` (`quire validate`, quire 0.36.2) reports no EARS
warning in FR-019, so every finding below comes from reading the text by hand.

Clean units:

- FR-019-AC-10 ("Only an item settled `supported` routes.") is a ubiquitous criterion.
- The Outputs bullet is a list item in Outputs, not a requirement statement.
- "CG shall not encode this cause as a string in `EnvelopeRefusal::InvalidCapability`" is a
  well-formed ubiquitous negative.
- The first sentence of the second Behavior bullet ("the generator shall compare ...") is
  ubiquitous with a locus phrase.

## Verdict

PASS with low findings. None blocks on its own. Fix them in the same edit as SR-13170's
findings.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | "If an identity has differing descriptors, it shall refuse every registration of that identity" is unwanted-behaviour form without "then" and with the pronoun "it" as subject. In the same bullet, "registrations of other identities shall remain eligible" makes the registrations the subject rather than the system. Rewrite as "If ..., then the generator shall ..." and "the generator shall keep registrations of other identities eligible" | spec/routed/functional/FR-019-capability-settlement.md:129-132 |
| FND-002 | low | "The choice and reported identity shall not depend on manifest order": "the choice" has no antecedent. Byte-equal repeats need no choice, and for differing repeats nothing is chosen | spec/routed/functional/FR-019-capability-settlement.md:132-133 |
| FND-003 | low | "The conflicting identity shall produce a dedicated typed registration refusal" makes the identity, not the generator, the actor. Use "the generator shall return ... for the conflicting identity" | spec/routed/functional/FR-019-capability-settlement.md:135-137 |

## Dispositions

Round 1, reviewed at def6c3f4529e4439f83cc8b5972d7600eb40e364. The Behavior bullets were rewritten. `make spec` reports no EARS warning in FR-019.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | Rewritten as "If ..., then the generator shall refuse the whole envelope ..." with the generator as subject |
| FND-002 | fixed | The "choice" sentence is gone; replaced by "If several identities repeat, then the generator shall report the bytewise-first repeated identity, independent of manifest order or adjacency" |
| FND-003 | fixed | The generator is now the actor of every new shall |
