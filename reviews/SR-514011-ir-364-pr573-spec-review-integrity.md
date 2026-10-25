---
id: SR-514011
title: "Integrity review of IR-364 PR 573 shared V2 comparison core"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen; spec/strategy/functional/FR-008-bound-domain-strategy-admission.md, spec/core/functional/interface-001-codegen-api.md (PR #573 head abe97829, base 1944268f)"
review_set: subset
---

## Summary

Ticket: IR-364. Integrity review (completeness, consistency, atomicity) of the PR #573 diff. The
change is internally consistent with the ruling and adds no contradiction inside FR-008. Its one
medium gap: FR-008 now binds itself to FR-014's comparison core and to FR-014-AC-35's planned
integer eq/ne derivation, but declares no FR-014 relationship, and FR-014 carries no reciprocal
statement. The second finding is a duplicated normative statement.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | FR-008 now SHALL share FR-014's clause-oracle comparison core, constrains FR-014-AC-35's planned eq/ne derivation ("SHALL NOT introduce a second evaluator"), and AC-7 requires exercising FR-014's public path for eq and ne, which cannot pass until FR-014-AC-35 is implemented. FR-008's `relationships` has no FR-014 edge, and FR-014 states nothing about being shared with FR-008. A change to FR-014 shows no FR-008 blast radius, and the sequencing dependency is invisible to the dependency graph. Add a `depends_on` FR-014 edge, and state the shared-core obligation in FR-014 (or cite it there). | spec/strategy/functional/FR-008-bound-domain-strategy-admission.md:1-16; spec/strategy/functional/FR-008-bound-domain-strategy-admission.md:29-33; spec/oracle/functional/FR-014-exact-scalar-oracles.md:571 | missing-requirement |
| FND-002 | low | The shared-core obligation is stated twice as normative SHALLs, in the Description (lines 29-33) and in a new Behavior bullet (lines 94-98), with slightly different scope words ("operators and operand forms they share" versus "corresponding operator and operand forms"). Keep the normative statement in Behavior only, so the two cannot drift. | spec/strategy/functional/FR-008-bound-domain-strategy-admission.md:29-33; spec/strategy/functional/FR-008-bound-domain-strategy-admission.md:94-98 | wrong-requirement |

## Examined

- FR-008 Description and the new Behavior bullet against FR-008-CON-1, AC-3, AC-7, AC-8 and AC-9:
  no contradiction. Sharing the evaluator explicitly does not widen the admitted grammar or erase
  the typed refusal and locus.
- interface-001 `generate_bound_strategy` and `v2_bound_strategy_slice` against FR-008: the
  operator set, ordered operands, refusal atomicity and no-poisoning rule agree. The
  execution-point inputs are absent from FR-008 and owned by FR-011 in #572 (SR-514010 FND-004).
- interface-001 against AD-004 L-12: `BoundPackage` is confined to the declared V1 interim, which
  expires in the same change that ships the replacement.

## Verdict

One medium and one low finding. Fix them in the same round as SR-514010.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | FR-008 now declares `depends_on` FR-014 and a "Shared comparison core" dependency naming FR-014 AC-35 and AC-38. FR-014 adds a `references` FR-008 edge, a Description statement of the shared unit, and a Dependencies entry. |
| FND-002 | fixed | The Description now only defines the unit and the form set (no SHALL). The normative obligation is stated once, in Behavior. |
