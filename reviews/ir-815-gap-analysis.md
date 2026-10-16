---
id: SR-11858
title: "IR-815 gap analysis of FR-002 and FR-004 V2 criteria"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen@ee7d99fb9588404b1b4ec84c69f22d778fe45b09 (PR 401, diff vs origin/main fe20ecc); spec/strategy/functional/FR-002-tristate-proptest.md; spec/evidence/functional/FR-004-vacuity-evidence.md; spec/strategy/matrix/TC-004-proptest-strategies.md; spec/evidence/matrix/TC-006-vacuity.md; spec/strategy/matrix/tests.md; spec/evidence/matrix/tests.md; spec/assurance/AD-004-cg-crate-layout.md"
review_set: subset
---

## Summary

Ticket: IR-815. Planless gap analysis of the spec-only PR #401 (head ee7d99f): computed matrix status of the new criteria, deferred decisions, and dependencies on other requirements. Plan completion: not assessed. No production code changed, so no code-side gap.

## Verdict

**One medium and two low findings.** `quire matrix` shows FR-002-AC-7, 8, 10, 11, 12 and FR-004-AC-16, 18, 19 `untagged` and FR-002-AC-9 and FR-004-AC-17 `method-without-symbol`. All are labelled PLANNED with a 🚧 matrix status, which is honest; no test claims them. The Inspection rows read the same way as FR-008-AC-8/AC-10. The medium finding is that FR-002's Open Question understates what depends on it.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | OQ-1 says 'No FR-002 V2 criterion depends on the answer'. The generated harness does depend on requirement and revision identity. Today's harness writes `// Requirement: {requirement}@{revision}`, `RequirementId::new(...)`, and oracle and harness symbols from them (`src/strategy/harness.rs:138-139, 886-907, 1066-1075`). NFR-002-AC-1 requires that header on every generated file. AC-12's determinism inputs (package, clause ids, campaign policy) omit that identity. So either the V2 harness cannot meet NFR-002-AC-1, or AC-12's input set is incomplete. Keep the question open. Say that the generated harness bytes and NFR-002-AC-1 wait on FR-011 OQ-2. | spec/strategy/functional/FR-002-tristate-proptest.md:93-96, spec/strategy/functional/FR-002-tristate-proptest.md:89, src/strategy/harness.rs:886-907 |
| FND-002 | low | FR-004-AC-16 to AC-19 avoid identity, as OQ-1 says. But the V2 analyzer that replaces `analyze_bound_coverage` must still satisfy FR-004-AC-10/AC-11/AC-14, which bind the run's requirement/revision. AD-004 step 6 deletes the V1 reader once its V2 replacement exists. The spec does not say that deleting it waits on the OQ. | spec/evidence/functional/FR-004-vacuity-evidence.md:181-183, spec/evidence/functional/FR-004-vacuity-evidence.md:197-200 |
| FND-003 | low | FR-002 consumes the FR-014-AC-38 oracle and shape from the harness. FR-014's behaviour statement scopes that oracle to 'where FR-015 names a V2 clause claim' (FR-014 lines 423-429), and its matrix row names only IR-489/FR-015-AC-40. FR-014 has not been widened to an FR-002 consumer. | spec/strategy/functional/FR-002-tristate-proptest.md:88, spec/oracle/functional/FR-014-exact-scalar-oracles.md:423-429 |

## Examined units

```yaml
scope:
- id: FR-002 OQ-1
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: '- OQ-1 (execution point and identity): the V2 source of the harness''s execution point and of the requirement and revision identity is the open question recorded as FR-011 OQ-1 and OQ-2, and the V2 entry shape follows the open interface-001 direction. No FR-002 V2 criterion depends on the answer, and none invents a source.'
- id: FR-004 OQ-1
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: '- OQ-1 (anchor and identity): the V2 source of a clause''s execution anchor and of the requirement and revision identity is the open question recorded as FR-011 OQ-1 and OQ-2, and the V2 entry shape follows the open interface-001 direction. No FR-004 V2 criterion depends on the answer, and none invents a source.'
- id: FR-002-AC-7
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). For one precondition and one postcondition selected from an admitted `CheckedPackageV2`, the generated harness embeds each selected clause's FR-014 V2 clause oracle, and a seeded case on which the precondition is false is rejected with the subject not invoked, one on which the postcondition is false is a failed postcondition, and any other is a pass.
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
- id: FR-014-AC-38
  path: spec/oracle/functional/FR-014-exact-scalar-oracles.md
  role: context_only
  excerpt: 'PLANNED (IR-489). The Boolean condition of a `state`/`state_clause` node, addressed by the clause node id, derives a descriptor and generates an `ir_confirmed` oracle over its connectives, integer comparisons and integer add, subtract, multiply and negate, returning `Outcome<bool>`. A `project(deref(self), field)` term is an operand that takes a value in the object member''s `integer_range` as a state field read, and a `pre(...)` of one takes the pre-state value of that field. FR-014-AC-10 is not weakened: an operand that is neither a literal, a reference nor one of those two terms is still ref'
- id: FR-008-AC-8
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: context_only
  excerpt: PLANNED. The V2 comparison oracle takes only the accessor's typed result for the selected claim, and no V1 bound oracle or `BoundPackage` is an input to it.
- id: FR-008-AC-10
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: context_only
  excerpt: PLANNED. Source inspection shows the generator reads no `CheckedSemanticNodeV2.body` field and decodes no body JSON (the verification of FR-008-CON-1).
- id: FR-011 OQ-1/OQ-2
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: context_only
  excerpt: '- OQ-1 (execution point): the runtime execution point (`initialization`, `handler`, `pre`, `post`) has no V2 source. IR''s accessor returns only the clause kind (`Invariant`, `Precondition`, `Postcondition`), and IR main has no execution-point type, so an invariant could map to `initialization` or `handler`. No source is invented here. The planner is taking this to the owner; no FR-011 criterion depends on the answer. - OQ-2 (requirement and revision identity): the V1 `ClauseRef` supplied the requirement and revision identity for `construct_verdict`. The V2 request (selected clause id and claim'
- id: NFR-002-AC-1
  path: spec/core/non-functional/NFR-002-provenance-boundary.md
  role: context_only
  excerpt: Every generated Rust file's header names the generator and the requirement, revision and clause it was generated from.
bindings: []
```

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | - |
| FND-003 | fixed | - |

## Disposition round 1 verdict

**Round 1: PASS.** `quire matrix` at 55c3347 reports FR-002-AC-7, 8, 10 to 16 and FR-004-AC-16, 18 to 24 `untagged`, and AC-9/AC-17 `method-without-symbol`. All are PLANNED with a 🚧 status, so none is overclaimed. Both Open Questions now say what waits on FR-011 OQ-2 without answering it. FR-014's IR-489 behaviour statement now admits an FR-002 consumer. FR-014-AC-38's text was already consumer-neutral.
