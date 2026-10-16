---
id: SR-11855
title: "IR-815 base spec review of FR-002 and FR-004 V2 criteria"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen@ee7d99fb9588404b1b4ec84c69f22d778fe45b09 (PR 401, diff vs origin/main fe20ecc); spec/strategy/functional/FR-002-tristate-proptest.md; spec/evidence/functional/FR-004-vacuity-evidence.md; spec/strategy/matrix/TC-004-proptest-strategies.md; spec/evidence/matrix/TC-006-vacuity.md; spec/strategy/matrix/tests.md; spec/evidence/matrix/tests.md; spec/assurance/AD-004-cg-crate-layout.md"
review_set: subset
---

## Summary

Ticket: IR-815. Base checklist over PR #401 (head ee7d99f, diff vs origin/main fe20ecc): FR-002-AC-7 to AC-12, FR-004-AC-16 to AC-19, their behaviour statements, Open Questions, TC-004/TC-006 V2 blocks, both matrix files and the AD-004 paragraph. Measured against current main, not the ticket text.

## Verdict

**Not mergeable as is: three medium findings**, all cheap spec edits. Clean: ids continue FR-002-AC-6 and FR-004-AC-15 with no renumbering and no edit to an existing criterion; Test vs Inspection is honest (only the two source-inspection criteria are Inspection); FR-014-AC-38 exists and is cited exactly; the AD-004 sentence is true (no code reads `CheckedPackageV2` in `strategy/harness.rs` or `evidence/bound_coverage.rs`); no code, compatibility layer, hash, digest or pin text; both Open Questions defer to FR-011 OQ-1/OQ-2 and interface-001 without deciding them. quire coverage reports AC-9 and AC-17 as `method-without-symbol`, exactly as it reports FR-008-AC-8 and FR-008-AC-10, which are Inspection rows with an `Inspection` matrix cell and absent from TC-017's Traces To; IR-815 follows that pattern, so that flag is not a defect.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-7 classifies every case that is neither precondition-false nor postcondition-false as a pass. The embedded FR-014 oracles return `Outcome<bool>` and may stop (FR-014 description: an oracle 'that may stop'; AC-38 admits integer add/subtract/multiply, which can be undefined), so a stopped or undefined precondition or postcondition outcome is a pass under AC-7 as written. State the disposition of a non-completed oracle outcome (not a pass) and narrow 'any other' to both oracles completing true. | spec/strategy/functional/FR-002-tristate-proptest.md:84 |
| FND-002 | medium | AC-19 says 'an evaluated implication with an unobserved consequent is `vacuous`'. FR-004's rule classifies a clause as vacuous only when no expected consequent probe was observed, and partially_exercised when some but not all were. For a clause with two implications and one consequent observed, AC-19's first clause says vacuous and its third says partially_exercised, contradicting the mutual exclusivity FR-004 requires. Restate per clause: evaluated, implication-bearing, no consequent observed. | spec/evidence/functional/FR-004-vacuity-evidence.md:165, spec/evidence/functional/FR-004-vacuity-evidence.md:59-65 |
| FND-003 | medium | AC-9 cannot fail against today's V1 harness. `src/strategy/harness.rs` names neither `BoundPackage` nor `ClauseRef`; it reads V1 meaning through `quire_contract_model::{ClauseId, RequirementRef, TypedExpression}` (lines 5-7) and `oracle::boolean_v1` (lines 16-19), and `quire-contract-model` is a Contract IR crate. So the forbidden-name list passes the V1 state AC-9 exists to exclude. FR-004-AC-17 does discriminate (`bound_coverage.rs` names both). Name the V1 inputs the harness actually uses (`TypedExpression`, `oracle::boolean_v1`) or require the accessor/`core/ir` route positively. | spec/strategy/functional/FR-002-tristate-proptest.md:86, src/strategy/harness.rs:5-19 |
| FND-004 | low | AC-8 says the order is observed 'as an instrumented subject observes', but a subject sees only its own invocation; it cannot observe whether the pre-state snapshot came before precondition evaluation. That half of the order needs an instrumented snapshot or oracle (TC-004 likewise records only call order and invocation). | spec/strategy/functional/FR-002-tristate-proptest.md:85, spec/strategy/matrix/TC-004-proptest-strategies.md:30-35 |
| FND-005 | low | The inserted sentence ends '...now carry planned V2 criteria too (IR-815), which no code implements yet.' The next sentence, 'They sit in their final directories, named V1 in the module map, and each is deleted...', now reads as referring to the criteria rather than the five V1 modules. Name the subject ('The V1 modules sit...'). | spec/assurance/AD-004-cg-crate-layout.md:534-536 |

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
- id: FR-002 OQ-1
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: '- OQ-1 (execution point and identity): the V2 source of the harness''s execution point and of the requirement and revision identity is the open question recorded as FR-011 OQ-1 and OQ-2, and the V2 entry shape follows the open interface-001 direction. No FR-002 V2 criterion depends on the answer, and none invents a source.'
- id: FR-004 OQ-1
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: '- OQ-1 (anchor and identity): the V2 source of a clause''s execution anchor and of the requirement and revision identity is the open question recorded as FR-011 OQ-1 and OQ-2, and the V2 entry shape follows the open interface-001 direction. No FR-004 V2 criterion depends on the answer, and none invents a source.'
- id: TC-004 V2 controls
  path: spec/strategy/matrix/TC-004-proptest-strategies.md
  role: examined
  excerpt: 'Planned V2 controls (IR-815): from an admitted `CheckedPackageV2`, select one precondition and one postcondition and generate the harness. Drive seeded cases that make the precondition false, the postcondition false, and neither, with an instrumented subject that records call order and whether it was invoked. Select an unknown clause identity, a clause of the wrong kind, the same clause for both selections, and a body outside the FR-014-AC-38 shape. Generate twice and compare bytes. Inspect the generator source for any `BoundPackage`, `ClauseRef` or private body decoder. The V2 controls expect'
- id: TC-006 V2 population controls
  path: spec/evidence/matrix/TC-006-vacuity.md
  role: examined
  excerpt: Admit a `CheckedPackageV2` holding a vacuous implication, a never-evaluated clause, a partially exercised multi-implication clause and an exercised clause. Independently derive each clause's expected population entry and implication count from its body, then compare the analyzer's expectation. Remove one clause, one consequent, one evaluation probe and every clause from the generated artifacts, each alone. Alter one source-map implication count against its body. Repeat the FR-004-AC-5 malformed-input mutations. Inspect the analyzer source for any `BoundPackage`, `ClauseRef` or private body dec
- id: strategy matrix FR-002 V2 rows and TC-004 Traces To
  path: spec/strategy/matrix/tests.md
  role: examined
  excerpt: '| FR-002 | FR-002-AC-7, FR-002-AC-8, FR-002-AC-10 through FR-002-AC-12 | TC-004 | V2 planned | ; | FR-002 | FR-002-AC-9 | Inspection | V2 planned | ; TC-004 Traces To FR-002-AC-1..AC-8, AC-10..AC-12'
- id: evidence matrix FR-004 V2 rows and TC-006 Traces To
  path: spec/evidence/matrix/tests.md
  role: examined
  excerpt: '| FR-004 | FR-004-AC-16, FR-004-AC-18, FR-004-AC-19 | TC-006 | V2 planned (IR-815) | ; | FR-004 | FR-004-AC-17 | Inspection | V2 planned (IR-815) | ; TC-006 Traces To FR-004-AC-1..AC-16, AC-18, AC-19'
- id: AD-004 one-input-model paragraph
  path: spec/assurance/AD-004-cg-crate-layout.md
  role: examined
  excerpt: '- Oracle, strategy and evidence side: `harness`, `strategy/bound`, `evidence/bound_coverage`, `oracle/bound_v1.rs` and `oracle/boolean_v1.rs` read V1. FR-008 to FR-013 and NFR-004 now carry planned V2 criteria over `CheckedPackageV2`, which `strategy/bound` has not yet implemented; FR-002 (`harness`, FR-002-AC-7 to AC-12) and FR-004 (`evidence/bound_coverage`, FR-004-AC-16 to AC-19) now carry planned V2 criteria too (IR-815), which no code implements yet. They sit in their final directories, named V1 in the module map, and each is deleted in the PR that adds its V2 replacement. A layout test f'
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

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | medium | New in the fix round: FR-002-AC-16 says a precondition that does not complete skips the postcondition, but it says nothing about the subject. The harness could still invoke the subject on an input whose precondition was undefined, which AC-14 forbids for a false precondition. Neither the matching behaviour statement nor TC-004's expected results ('skips the postcondition') constrains invocation. State that the subject is not invoked when the precondition oracle does not complete. | spec/strategy/functional/FR-002-tristate-proptest.md:98, spec/strategy/functional/FR-002-tristate-proptest.md:65-67, spec/strategy/matrix/TC-004-proptest-strategies.md:53-54 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | - |
| FND-003 | fixed | - |
| FND-004 | fixed | - |
| FND-005 | fixed | - |

## Disposition round 1 verdict

**Round 1: PASS for FND-001 to FND-005, but one new medium finding (FND-006) keeps the PR not mergeable.** Reviewed 55c3347 against origin/main fe20ecc. FR-002-AC-13 to AC-16 cover every completion combination: both true, precondition false, precondition true with postcondition false, and either one not completing. They do not overlap. FR-004-AC-19 to AC-22 restate FR-004's four per-clause rules exactly. AC-9 now names `TypedExpression` and `oracle::boolean_v1`, so the V1 harness fails it. FR-002 OQ-2 leaves the verdict kind open and only fixes 'not a pass, keeps its cause', so it decides nothing. No existing id is renumbered. FR-002-AC-1 to AC-6 and FR-004-AC-1 to AC-15 are unchanged.
