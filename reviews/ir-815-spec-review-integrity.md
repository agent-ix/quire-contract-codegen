---
id: SR-11857
title: "IR-815 integrity review of FR-002 and FR-004 V2 criteria"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen@ee7d99fb9588404b1b4ec84c69f22d778fe45b09 (PR 401, diff vs origin/main fe20ecc); spec/strategy/functional/FR-002-tristate-proptest.md; spec/evidence/functional/FR-004-vacuity-evidence.md; spec/strategy/matrix/TC-004-proptest-strategies.md; spec/evidence/matrix/TC-006-vacuity.md; spec/strategy/matrix/tests.md; spec/evidence/matrix/tests.md; spec/assurance/AD-004-cg-crate-layout.md"
review_set: subset
---

## Summary

Ticket: IR-815. Consistency of the new criteria with TC-004, TC-006, both matrix files and AD-004, at PR #401 head ee7d99f against origin/main fe20ecc.

## Verdict

**PASS with three low findings.** Clean: no id renumbered or reused; every new Test criterion appears in its TC's Traces To and in exactly one matrix row; the two Inspection criteria have their own `Inspection` row and are absent from Traces To, matching FR-008-AC-8/AC-10 and TC-017; TC-004 and TC-006 procedures exercise every traced criterion; `quire validate` on all seven changed files reports no error. The findings are expected-result gaps and one term that does not match AD-004.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | TC-004's V2 expected results omit two traced obligations: the pass outcome for the 'neither false' case (FR-002-AC-7) and that each wrong-selection diagnostic is located and names the selection (FR-002-AC-10). Its procedure drives both; the expected results should assert them. | spec/strategy/matrix/TC-004-proptest-strategies.md:48-51 |
| FND-002 | low | TC-006's V2 expected results do not assert that the count-mismatch non-success names the clause, nor that the clause is never treated as implication-free (FR-004-AC-18). | spec/evidence/matrix/TC-006-vacuity.md:83-85 |
| FND-003 | low | FR-004-AC-17 and FR-002-AC-9 bound reads by 'Contract IR's public typed API', a phrase used nowhere else in the spec. AD-004 makes `core/ir/` 'the only code in CG that opens a `CheckedPackageV2` body term'. An analyzer or harness that opens body terms directly through IR's public API satisfies these criteria and breaks AD-004. Name `core/ir` (or the FR-008 accessor) as the route. | spec/evidence/functional/FR-004-vacuity-evidence.md:163, spec/strategy/functional/FR-002-tristate-proptest.md:86, spec/assurance/AD-004-cg-crate-layout.md:579-580 |

## Examined units

```yaml
scope:
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
- id: FR-002-AC-7
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). For one precondition and one postcondition selected from an admitted `CheckedPackageV2`, the generated harness embeds each selected clause's FR-014 V2 clause oracle, and a seeded case on which the precondition is false is rejected with the subject not invoked, one on which the postcondition is false is a failed postcondition, and any other is a pass.
- id: FR-002-AC-10
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). A selected clause identity that is unknown, of the wrong clause kind, or the same for both selections yields a typed, located diagnostic naming the selection and no artifact.
- id: FR-004-AC-16
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). The expected clause population and implication counts of the V2 analyzer equal those independently derived from the admitted `CheckedPackageV2` clause bodies, and removing one clause, one consequent, one evaluation probe or every clause from the generated artifacts prevents successful analysis.
- id: FR-004-AC-19
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). Over a V2 clause population, an evaluated implication with an unobserved consequent is `vacuous`, an unevaluated clause is `unexecuted`, a clause with some consequents observed is `partially_exercised`, a fully observed clause is `exercised`, and each malformed input class of FR-004-AC-5 yields a non-success outcome with no invented classification.
- id: FR-002-AC-9
  path: spec/strategy/functional/FR-002-tristate-proptest.md
  role: examined
  excerpt: PLANNED (IR-815). Source inspection shows the V2 harness generator reads clause meaning only through Contract IR's public typed API and names no `BoundPackage`, `ClauseRef` or private body decoder.
- id: FR-004-AC-17
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). Source inspection shows the V2 analyzer reads clause meaning only through Contract IR's public typed API and names no `BoundPackage`, `ClauseRef` or private body decoder.
- id: FR-004-AC-18
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: PLANNED (IR-815). A V2 clause whose generated source map declares an implication count unequal to the count derived from its body yields a structured binding non-success naming the clause and no classification, and is never treated as implication-free.
- id: FR-008-AC-8
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: context_only
  excerpt: PLANNED. The V2 comparison oracle takes only the accessor's typed result for the selected claim, and no V1 bound oracle or `BoundPackage` is an input to it.
- id: FR-008-AC-10
  path: spec/strategy/functional/FR-008-bound-domain-strategy-admission.md
  role: context_only
  excerpt: PLANNED. Source inspection shows the generator reads no `CheckedSemanticNodeV2.body` field and decodes no body JSON (the verification of FR-008-CON-1).
bindings: []
```

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | - |
| FND-003 | fixed | - |

## Disposition round 1 verdict

**Round 1: PASS.** The strategy matrix row (AC-7, AC-8, AC-10 to AC-16) and TC-004's Traces To agree. The evidence matrix row (AC-16, AC-18 to AC-24) and TC-006's Traces To agree. AC-9 and AC-17 keep their own Inspection rows, as FR-008-AC-8/AC-10 do. Both TC expected-results blocks name each traced criterion. AD-004's ranges (FR-002-AC-7 to AC-16, FR-004-AC-16 to AC-24) and FR-004's implementation-boundary range are updated. `quire validate` on every changed spec file reports no error. The only warning, FR-014 line 252, is on an untouched line.
