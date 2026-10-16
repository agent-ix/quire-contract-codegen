---
id: "SR-11916"
title: "IR-823 gap analysis: FR-014-AC-6, FR-014-AC-47 and TC-024 against QSpec TC-187"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@ce909c258ceef3ab6920407d7c77ad2a95cf951e; spec/oracle/functional/FR-014-exact-scalar-oracles.md, spec/oracle/matrix/TC-024-exact-scalar-oracles.md, spec/oracle/matrix/tests.md, tests/exact_scalar_support/agreement_cases.rs, tests/exact_scalar_support/agreement.rs"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-024
    type: reviews
---

# SR-11916: IR-823 gap analysis

## Summary

Ticket: IR-823. PR: agent-ix/quire-contract-codegen#404, head ce909c2. Scope: the PR diff, the
criteria it edits (FR-014-AC-6, the new FR-014-AC-47), TC-024's description and step 4, and the
two oracle-matrix rows. The authority is QSpec TC-187 at quire-specification origin/main, rows
U02, U05, U06, U09, U11, U12, U14, U20 and U27, plus the tabled rows AC-6 already listed. Plan
completion: not assessed.

## Method

I read each AC-6 clause as a claim and checked it against the test code and against TC-187. I ran
the agreement crate under 19 runtime and generator mutants (SR-11915). I ran `quire matrix
--strict` and `make spec`. I grepped the FR-014 and TC-024 text, the tests and the matrix for every
QSL value-authority claim. The cited owners (Contract Runtime FR-007-AC-5, TC-022, FR-012-AC-4 and
TC-033, and Linear IR-669) were read as data.

The ticket's QSL value-authority comparison was not done. Under the planner's ruling (Linear,
2026-10-16), FR-014-AC-47 replaces it. I judged the wording, not the absence.

What matches:

- AC-6's row list is exactly the set of rows the tests run. Every vector it names has a test
  traced to FR-014-AC-6. `quire matrix --strict` lists 15 binders for AC-6, the new tests included.
- The dispositions asserted match TC-187:
  - U02: `L` and the empty dimension, no zero exponents.
  - U05: `ill_typed`.
  - U09: `ill_typed`, cause `IncompatibleDimensions`, with a same-key control.
  - U12: 6, 3, 4 and 1 under `m*s`, `m*s^-1`, `m^2` and dimensionless.
  - U14 and U20: refused with no charge under `L(0,…)`, and `3 u2` with no edge.
  - U27: undefined after two identity reads, then QSpec's incomplete record.
  - U03, U13, U19, U21, U23, U24 and U28: QSpec's own tuples and incomplete records.
- U06/U11 are stated, in AC-6, TC-024 step 4, the matrix row and the test doc, to have no
  generated oracle. They are asserted against the runtime's graph admission. That is true.
- In TC-024 step 4, AC-6, tests.md row 14, agreement.rs and the reworded `println!`s, nothing
  claims QSL agreement for a TC-187 vector. AC-47's inspection claim holds for its stated scope.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-024 now contradicts itself on the QSL leg. The PR's new Description says "the QSL value authority is not a leg of this case". But TC-024 still says integer, rational and ordering oracles "are counted separately from the three-way vectors", and none exist. Its Boolean step 1 (FR-014-AC-37) still runs each oracle "beside direct runtime execution and the QSL value authority". FR-014-AC-37 still claims agreement with the QSL value authority, and T12-A rules that out for AC-37 exactly as for TC-187. AC-47 is scoped to TC-187 vectors only, so it does not cover AC-37. | spec/oracle/matrix/TC-024-exact-scalar-oracles.md:17, spec/oracle/matrix/TC-024-exact-scalar-oracles.md:217-221, spec/oracle/matrix/TC-024-exact-scalar-oracles.md:226-232, spec/oracle/functional/FR-014-exact-scalar-oracles.md:478, spec/oracle/functional/FR-014-exact-scalar-oracles.md:487 |
| FND-002 | low | AC-47 says the QSL comparison "is owned by quire-integration IR-669 (Contract Runtime FR-007-AC-5, TC-022)". IR-669 (Backlog) owns an RT-against-QSL composition, not a comparison with CG's generated oracles. Generated-against-QSL follows only by composing that with FR-014-AC-6. RT's own matrix marks FR-007-AC-5 and TC-022 planned with no evidence. So today no lane anywhere evidences QSL agreement for these vectors, and AC-47 does not say so. | spec/oracle/functional/FR-014-exact-scalar-oracles.md:487, spec/oracle/matrix/tests.md:38 |
| FND-003 | low | AC-47 is obligation-shaped ("CG shall claim no agreement …"), and `make spec` warns `[ac:non-canonical-shape]` at FR-014 line 487. It is also a negative non-goal verified by inspection: it can only fail when someone adds a claim. A direct assertion would be canonical, for example "No TC-187 agreement test name, trace tag, output line or doc comment, TC-024 step or oracle-matrix row names agreement with the QSL value authority". The ownership pointer could move to FR-014's Dependencies. | spec/oracle/functional/FR-014-exact-scalar-oracles.md:487 |
| FND-004 | low | `tc_024_tc187_u06_u11_unit_graph_admission_refuses_each_malformed_graph` exercises no CG code. Every generator mutant left it green, and only runtime mutants failed it. Contract Runtime TC-033 (FR-012-AC-4) already asserts the same six causes. The test is labelled honestly, and IR-823's acceptance criteria allow it. But it is traced to FR-014-AC-6, a generated-oracle criterion, so it counts as AC-6 evidence for rows that AC-6 itself says are not oracle agreement. | tests/exact_scalar_support/agreement_cases.rs:1060-1067, spec/oracle/functional/FR-014-exact-scalar-oracles.md:447 |
| FND-005 | low | AC-6 says the listed TC-187 vectors run "each under QSpec's own limit tuple, unchanged". QSpec gives no tuple for U02, U05, U09, U12, or U23's `rev` and `degC == degC` parts. Those run under `UNLIMITED`. The clause is true only for the tabled vectors, and it should say so. | spec/oracle/functional/FR-014-exact-scalar-oracles.md:447, tests/exact_scalar_support/agreement_cases.rs:609-612 |

## Verdict

Not mergeable as-is: fix FND-001, or defer it under an explicit planner ruling. It is a spec
contradiction that the PR's own TC-024 Description edit makes visible. The rest is low.

On the known open item: the AC-6, TC-024 step 4 and tests.md wording ("claims no agreement with the
QSL value authority (FR-014-AC-47)") is true for the TC-187 vectors and for AC-6's other vectors.
It is not complete for FR-014 as a whole:

- FR-014-AC-37 and TC-024's Boolean step still require a QSL leg (FND-001).
- The "owned by IR-669" pointer overstates what IR-669 compares, and does not say that IR-669 is
  unstarted (FND-002).

Ruling needed from the planner (titan:kani:plan) or the owner. One option is to extend AC-47, or a
sibling criterion, to every FR-014 criterion that names the QSL value authority, AC-37 included,
and strike the "three-way vectors" sentence. The other is to state that AC-37 keeps its QSL leg,
and how CG would run it under QSL arch-lint T12-A. The ruling should also confirm that IR-669's
scope covers generated-oracle agreement through FR-014-AC-6 composition. Otherwise
generated-against-QSL agreement for these vectors has no owner. FR-018-AC-2 and FR-021-AC-18 have
the same shape, but they are outside this PR.
