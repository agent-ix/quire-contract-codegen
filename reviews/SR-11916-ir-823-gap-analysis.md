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

## New findings (disposition pass 1)

Reviewed at 218f97bc6d560e9af7b8dd40a4cecbb9ef8fdb3d.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | low | AC-6's new tuple clause lists U03 among the vectors QSpec "tables" a limit tuple for. QSpec TC-187 U03 gives none; the test runs it under `L(0,0,0,0,0,0)`, the tuple QSpec gives U14's identical `1 m + 1 s` case. The test is stricter than unlimited, which is fine, but the clause misattributes the tuple to U03. | spec/oracle/functional/FR-014-exact-scalar-oracles.md:447, tests/exact_scalar_support/agreement_cases.rs:503 |

## Dispositions

Round 1, at head 218f97bc6d560e9af7b8dd40a4cecbb9ef8fdb3d (fix commit 218f97b). Judged against the spec text and code. No ruling text from titan:kani:plan was on IR-823 when I checked.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | still-open | Deferred pending the titan:kani:plan ruling, and not addressed in 218f97b. TC-024 is unchanged. Its Description (line 17) still says there is no QSL leg, while line 221 refers to "three-way vectors", Boolean step 1 (lines 226-232) still runs "beside ... the QSL value authority", and FR-014-AC-37 (line 478) still claims QSL agreement. The PR stays unmergeable until a ruling resolves this or explicitly defers it. |
| FND-002 | fixed | FR-014's Dependencies now say no lane evidences QSL agreement for these vectors today. IR-669 owns an RT-against-QSL composition, and generated-oracle agreement would follow only by composing it with FR-014-AC-6. tests.md row 38 says "nothing evidences it today". |
| FND-003 | fixed | AC-47 is now a direct assertion: "No test name, trace tag, test output line or doc comment of the TC-187 agreement cases, no TC-024 step and no oracle-matrix row for a TC-187 vector names agreement with the QSL value authority." The ownership pointer moved to Dependencies. `make spec` no longer warns at line 487. |
| FND-004 | fixed | AC-6 now says the U06/U11 test "exercises no CG code, so it is not evidence about any generated oracle". The trace to FR-014-AC-6 now carries no claim about a generated oracle. |
| FND-005 | fixed | AC-6 now separates the tabled vectors, run at QSpec's tuple, from those QSpec gives none, which run unlimited (U02, U05, U09, U12 and the untabled parts of U23). The residual U03 attribution is FND-006. |
| FND-001 | fixed | Round 2, at 9fbbd957c07aca05b23f20b194b7261c3a661067. The contradiction is gone. TC-024's Description now says QSL agreement "is not evidenced and not claimed by CG". The "three-way vectors" sentence is replaced by "checked against direct runtime execution only". Boolean step 1, FR-014-AC-37's verification column and tests.md row 28 each mark AC-37's QSL leg as an open gap that CG does not claim and that nothing evidences. A grep of FR-014, TC-024 and tests.md finds no remaining statement that claims QSL evidence. AC-37 stays Planned and still states a requirement CG cannot satisfy under QSL arch-lint T12-A. That ruling is outside this finding, and no Linear ticket tracks it yet; see FND-007. |
| FND-006 | fixed | Round 2, at 9fbbd95. AC-6 now lists the tabled vectors without U03, and says "U03 under U14's zero tuple, which QSpec gives its identical `1 m + 1 s` case", which matches agreement_cases.rs:503. |

## New findings (disposition pass 2)

Reviewed at 9fbbd957c07aca05b23f20b194b7261c3a661067.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-007 | low | FR-014-AC-37's verification column and TC-024 Boolean step 1 cite "the reason given in FR-014-AC-47" (the step says "CG may call the `qsl-replay` facade only"). AC-47 now gives no reason: it is a bare inspection assertion. FR-014's Dependencies note covers only TC-187 vectors and never names the `qsl-replay`-only constraint (QSL arch-lint T12-A). So the pointer leads nowhere, and the open gap's cause and owner for AC-37's Boolean and integer `eq`/`ne` corpus are not stated in FR-014. The AC-37 satisfiability ruling also has no Linear ticket: a title search for "AC-37" finds none. | spec/oracle/functional/FR-014-exact-scalar-oracles.md:478, spec/oracle/functional/FR-014-exact-scalar-oracles.md:487, spec/oracle/functional/FR-014-exact-scalar-oracles.md:494, spec/oracle/matrix/TC-024-exact-scalar-oracles.md:232-235 |

### Round 2 verdict

Mergeable once the full gates pass. FND-001 to FND-006 are resolved, and FND-007 is low. Fix FND-007 in this PR (a Dependencies line naming T12-A and covering AC-37), or defer it to the ticket that will carry the AC-37 ruling. That ticket should be filed before merge, so the open gap has an owner.

## Dispositions (round 3)

At head 4cf122462156f45dae3cac5b64573628d716794a (fix commits dcceb77 and 4cf1224).

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-007 | fixed | FR-014-AC-37's verification column and TC-024 Boolean step 1 now point at an FR-014 Dependencies note, "qsl-replay-only constraint". That note names QSL arch-lint T12-A, covers AC-37's Boolean and integer `eq`/`ne` corpus, and cites IR-833 for the ownership question and IR-669 for the RT-against-QSL composition. IR-833 exists (Backlog, titan:kani:plan lane) and lists AC-37, TC-024 step 1 and the out-of-PR FR-018/TC-029 sites. No regression: FR-014, TC-024 and tests.md contain no "three-way vectors" or "not a leg" text and no claim of QSL evidence. `make spec` reports only the pre-existing line-252 warning. |

## New findings (disposition pass 3)

Reviewed at 4cf122462156f45dae3cac5b64573628d716794a.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-008 | low | The new Dependencies note says the `qsl-replay` facade "exposes no quantity, integer or Boolean value evaluator". At the locked QSL revision 23f9f342, `qsl-replay` does export integer operator parity: `replay_operator_parity`, whose `ScalarOperator` is add, sub, mul and negate and whose outcome is a `Value::Integer`. It also exports `replay_value_parity` for non-Boolean functions, and `replay` settles a Boolean predicate at its bindings. None of these reports the outcome, charges and consumed counters that AC-37 compares, and none covers integer comparisons or connectives. So the conclusion holds for AC-37's corpus, but the sentence overstates the facade's limits. It also misstates the premise IR-833's ruling depends on: the planner's earlier ruling called the qsl-replay route "partial (no ill-typed rows, no charges)", not absent. Suggested wording: "exposes no evaluator that reports the outcome, charges and counters FR-014-AC-37 compares, and none for integer comparisons or Boolean connectives". | spec/oracle/functional/FR-014-exact-scalar-oracles.md:495 |

### Round 3 verdict

Mergeable once the full gates pass. Every finding from FND-001 to FND-007 is resolved. FND-008 is a low wording overstatement in a Dependencies note: nothing claims evidence it lacks. Fix it in this PR, or carry it into IR-833, where the ruling needs the accurate premise.

## Dispositions (round 4)

At head cd665548ef8aafbba7c0ca23b5693b96af4ea190 (fix commit cd66554).

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-008 | fixed | The note now says the facade exposes no evaluator that reports the outcome, admitted charges and consumed counters (with each single-charge denial) that AC-37 compares. I checked it against QSL 23f9f342. `replay_operator_parity` covers only `ScalarOperator` Add, Subtract, Multiply and Negate, and its `OperatorParityResult` carries `charges: ScalarLimits`, the consumed counters from `execute::consumed`, with no admitted-charge sequence or denials. `replay_value_parity` refuses a function with a Boolean result, and `replay` settles a Boolean predicate. The note is accurate. No other spec text changed. |

### Round 4 verdict

Every SR-11916 finding (FND-001 to FND-008) is resolved. Mergeable once the full gates pass.
