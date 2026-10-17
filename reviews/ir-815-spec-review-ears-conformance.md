---
id: SR-11856
title: "IR-815 EARS conformance review of FR-002 and FR-004 V2 criteria"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-codegen@ee7d99fb9588404b1b4ec84c69f22d778fe45b09 (PR 401, diff vs origin/main fe20ecc); spec/strategy/functional/FR-002-tristate-proptest.md; spec/evidence/functional/FR-004-vacuity-evidence.md; spec/strategy/matrix/TC-004-proptest-strategies.md; spec/evidence/matrix/TC-006-vacuity.md; spec/strategy/matrix/tests.md; spec/evidence/matrix/tests.md; spec/assurance/AD-004-cg-crate-layout.md"
review_set: subset
---

## Summary

Ticket: IR-815. EARS and singularity check of the seven new behaviour statements and ten new criteria in FR-002 and FR-004 at PR #401 head ee7d99f.

## Verdict

**PASS with five low findings.** The two refusal statements in each FR use the unwanted-behaviour pattern ('if ..., then ... shall return'), the derive statement uses the event pattern, and the API-boundary statements are ubiquitous. FR-002-AC-8, AC-9, AC-10, AC-11, AC-12 and FR-004-AC-17, AC-18 each state one response (AC-10 enumerates one trigger class). The findings are compound criteria and statements. The repo's existing FR-004-AC-13 to AC-15 are also compound, so these are low.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | AC-7 states two obligations: the harness embeds each selected clause's FR-014 V2 oracle (a generation property) and three tri-state case dispositions (a runtime property). Split the embedding from the dispositions. | spec/strategy/functional/FR-002-tristate-proptest.md:84 |
| FND-002 | low | AC-16 joins population/count equality with four artifact-removal refusals. They fail independently; split equality from the removal refusals. | spec/evidence/functional/FR-004-vacuity-evidence.md:162 |
| FND-003 | low | AC-19 carries four classification outcomes plus the FR-004-AC-5 malformed-input non-success. Split the malformed-input half into its own criterion. | spec/evidence/functional/FR-004-vacuity-evidence.md:165 |
| FND-004 | low | One event statement with two responses: 'shall embed ... the FR-014 V2 clause oracle' and 'shall keep the snapshot, precondition, invocation and postcondition ordering'. Split into two statements. | spec/strategy/functional/FR-002-tristate-proptest.md:60-63 |
| FND-005 | low | One event statement with two responses: 'shall derive its expected clause population ...' and 'shall take no `BoundPackage` input'. The second is a ubiquitous boundary statement; state it on its own. | spec/evidence/functional/FR-004-vacuity-evidence.md:134-136 |

## Examined units

```yaml
scope:
- id: FR-002-AC-7
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). For one precondition and one postcondition selected from an admitted `CheckedPackageV2`, the generated harness embeds each selected clause's FR-014 V2 clause oracle, and a seeded case on which the precondition is false is rejected with the subject not invoked, one on which the postcondition is false is a failed postcondition, and any other is a pass.
- id: FR-002-AC-8
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). In the V2 harness the pre-state snapshot precedes precondition evaluation, which precedes subject invocation, which precedes postcondition evaluation, for every case, as an instrumented subject observes.
- id: FR-002-AC-9
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). Source inspection shows the V2 harness generator reads clause meaning only through Contract IR's public typed API and names no `BoundPackage`, `ClauseRef` or private body decoder.
- id: FR-002-AC-10
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). A selected clause identity that is unknown, of the wrong clause kind, or the same for both selections yields a typed, located diagnostic naming the selection and no artifact.
- id: FR-002-AC-11
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). A selected clause body outside the FR-014-AC-38 shape yields a structured unsupported diagnostic and no artifact, and the generator emits no filtering strategy for it.
- id: FR-002-AC-12
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). Repeated V2 harness generation for the same admitted package, selected clause identities and campaign policy is byte-identical.
- id: FR-004-AC-16
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). The expected clause population and implication counts of the V2 analyzer equal those independently derived from the admitted `CheckedPackageV2` clause bodies, and removing one clause, one consequent, one evaluation probe or every clause from the generated artifacts prevents successful analysis.
- id: FR-004-AC-17
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). Source inspection shows the V2 analyzer reads clause meaning only through Contract IR's public typed API and names no `BoundPackage`, `ClauseRef` or private body decoder.
- id: FR-004-AC-18
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). A V2 clause whose generated source map declares an implication count unequal to the count derived from its body yields a structured binding non-success naming the clause and no classification, and is never treated as implication-free.
- id: FR-004-AC-19
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). Over a V2 clause population, an evaluated implication with an unobserved consequent is `vacuous`, an unevaluated clause is `unexecuted`, a clause with some consequents observed is `partially_exercised`, a fully observed clause is `exercised`, and each malformed input class of FR-004-AC-5 yields a non-success outcome with no invented classification.
- id: FR-002 Behavior (IR-815, embed oracle)
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: '- Planned (IR-815): when a request selects one precondition and one postcondition of an admitted `CheckedPackageV2`, the generator shall embed in the harness the FR-014 V2 clause oracle of each selected clause body (FR-014-AC-38) and shall keep the snapshot, precondition, invocation and postcondition ordering of this requirement.'
- id: FR-002 Behavior (IR-815, public API)
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: '- Planned (IR-815): the V2 harness generator shall read clause kind, identity and body only through Contract IR''s public typed API for the admitted package, and shall take no `BoundPackage` or `ClauseRef` input.'
- id: FR-002 Behavior (IR-815, wrong selection)
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: '- Planned (IR-815): if a selected clause identity is unknown, is not of the expected kind, or names the same clause as both selections, then the V2 harness generator shall return a typed, located diagnostic and no artifact.'
- id: FR-002 Behavior (IR-815, unsupported body)
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: '- Planned (IR-815): if a selected clause body is outside the FR-014-AC-38 shape, then the V2 harness generator shall return a structured unsupported diagnostic and no artifact, with no fallback to filtering.'
- id: FR-004 Behavior (IR-815, derive population)
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: '- Planned (IR-815): when the analyzer is given an admitted `CheckedPackageV2`, it shall derive its expected clause population and per-clause implication counts from that package''s executable clause bodies through Contract IR''s public typed API, and shall take no `BoundPackage` input.'
- id: FR-004 Behavior (IR-815, count mismatch)
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: '- Planned (IR-815): if the implication count declared by a V2 clause''s generated source map differs from the count derived from that clause''s body, then the analyzer shall return a structured binding non-success and no classification for that clause.'
- id: FR-004 Behavior (IR-815, same rules)
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: '- Planned (IR-815): the analyzer shall apply the classification and non-success rules above to a V2 clause population without change.'
bindings: []
```

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | - |
| FND-003 | fixed | - |
| FND-004 | fixed | - |
| FND-005 | fixed | - |

## Disposition round 1 verdict

**Round 1: PASS.** All five compound items are split. New criteria FR-002-AC-13 to AC-15 and FR-004-AC-20, AC-21, AC-23 and AC-24 each state one response. FR-004-AC-22 gives one outcome (`exercised`) under the two conditions FR-004 itself states. The new non-completion statement uses the unwanted-behaviour pattern. FR-002-AC-16's invocation gap is recorded as SR-11855 FND-006, not here.
