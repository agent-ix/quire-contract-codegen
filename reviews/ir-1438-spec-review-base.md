---
id: SR-17940486709564847047577
title: "spec-review/base of IR-1438 PR 764"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; spec/kani/functional/FR-049-enum-scalar-harnesses.md, spec/kani/matrix/TC-063-enum-scalar-harnesses.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-049
    type: reviews
---

## Summary

Ticket: IR-1438. PR #764 at dec7b1331330face200854102bcb755ce11f9aad, scope `git diff origin/main...HEAD`.
Base spec review of FR-049 and TC-063, including criterion strength of the mutant and refusal controls
(Jev unavailable, so criterion strength was judged manually here). FR-049 operator semantics were
checked against `src/oracle/scalar/derive.rs:271-276` (six `quire.op.enum.*` identities map to
`EnumComparison(ComparisonOperator)`), `src/oracle/scalar/render.rs:539-540,637-657` (oracle calls
`enum_owner::compare_enum(rt::ComparisonOperator::<op>, left, right, meter).map_err(OracleStop::IllTyped)`)
and the owner `quire-semantic-value` `src/enumeration.rs` at ad6e975 (`EnumDeclaration::model_owned`,
`member`, `compare_enum`, `compare`). The operator-to-native mapping (`==`, `!=`, `<`, `<=`, `>`, `>=`)
matches the six render arms. The owner semantics contradict FR-049's ordered-operator scope and one
mutant control.

## Verdict

**FAIL.** The six-operator mapping is correct and the TC-063 procedure covers every AC, but FR-049
plans ordered comparisons over model-owned enums, which the owner defines as unordered and ill-typed
for ordering, and AC-3 names a mutant (minted member identity) that cannot fail the operators it
names. Two high and four medium findings.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-049 plans `lt/le/gt/ge` harnesses over a model-owned enum with declared-order ordering, but QSV `EnumDeclaration::model_owned` always builds an unordered declaration (members sorted by case name) and `compare_enum` returns `IllTyped(UnorderedEnumOrdering)` for any ordering operator on it; FR-014-AC-2 only plans equality and inequality for model-owned enums. The ordered rows and their controls are unrealisable as written. | spec/kani/functional/FR-049-enum-scalar-harnesses.md:25-32,71-75 |
| FND-002 | high | FR-049-AC-3 says a harness that mints a member identity fails verification of the ordered operators; owner ordering compares `position`, taken from the declaration by case name, never the member key, so a minted key leaves every ordered operator verified. Only `eq`/`ne` (which compare member keys) can detect it. | spec/kani/functional/FR-049-enum-scalar-harnesses.md:96 |
| FND-003 | medium | FR-049-AC-6 says each mutant fails the proof without pairing mutant to operator; "ordering reversed" and "reordered member list" are bijections that leave `eq`/`ne` harnesses verified, and with FND-001 the four ordering mutants have no admissible ordered harness to run against. | spec/kani/functional/FR-049-enum-scalar-harnesses.md:99 |
| FND-004 | medium | FR-049-AC-8 requires refusal controls that cannot be constructed through admission: QSV rejects an empty member list as `NonCanonicalPreimage` (zero-member enum never admitted), and a declaration is always a finite list, so an "unbounded member set" and its `requires-bound` disposition are unreachable. | spec/kani/functional/FR-049-enum-scalar-harnesses.md:101 |
| FND-005 | medium | "more members than the proof ceiling's index bound" names a bound no spec defines; FR-028 and FR-015 have no enum index ceiling and FR-049 states it changes none of FR-015's ceilings, so the AC-8 over-bound control has no threshold. | spec/kani/functional/FR-049-enum-scalar-harnesses.md:82-85 |
| FND-006 | medium | FR-049-AC-4 requires controls producing `Undefined` and `Refused`, but `compare_enum` can only yield `Completed` or `Incomplete` (meter stop) inside `Outcome`; its ill-typed path is the oracle's `Err(OracleStop::IllTyped)` channel, which AC-4 and the behaviour never say must also be non-`Verified`. | spec/kani/functional/FR-049-enum-scalar-harnesses.md:72-76,97 |

## Examined scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-049 | examined | When an admitted request selects an `enum` scalar comparison node (`quire.op.enum.eq`, `.ne`, `.lt`, `.le`, `.gt` or `.ge`) over a model-owned enum whose complete member set is finite and declared, CG SHALL generate a bounded Kani row proving that the generated enum oracle of FR-014 agrees with a native expression of the same comparison over every pair of declared members. |
| FR-049-AC-1 | examined | For each of eq, ne, lt, le, gt and ge over an admitted enum with at least three declared members, CG generates one harness whose oracle is FR-014's generated enum comparison and whose expected value is the native operator over declared-order indices. |
| FR-049-AC-2 | examined | The symbolic domain is exactly the declared members; admitting one extra index, dropping the last member or fixing one operand fails the domain correspondence check. |
| FR-049-AC-3 | examined | A harness that reorders members, mints a member identity or compares by a name other than the checked identity fails verification of the ordered operators. |
| FR-049-AC-4 | examined | The assertion requires `Completed` with the native Boolean; a control producing `Undefined`, `Refused` or `Incomplete` cannot become `Verified`. |
| FR-049-AC-5 | examined | Healthy comparison receives `Verified` only from a successful proof; each mutant produces a failed assertion with in-domain playback replayed through the owner route. |
| FR-049-AC-6 | examined | Mutant controls: lt/le swap, gt/ge swap, eq/ne swap, ordering reversed, ordering by index of a reordered member list. |
| FR-049-AC-7 | examined | Exactly one completion cover after all assertions; cover, unwind, timeout, memory or missing verdict failures cannot yield `Verified`. |
| FR-049-AC-8 | examined | Different enums, non-member operand, zero-member enum, member count above the index bound, incomplete or unbounded member set each refused with typed reason and no harness. |
| FR-049-AC-9 | examined | Other families remain `unsupported`/`ShadowFamilyNotBuilt`; an enum node inside an unsupported composite keeps its composite disposition. |
| FR-049-AC-10 | examined | Run evidence records node, enum identity and ordered member list, operator, encoding, argv, tool versions, limits, checks, cover, playback, exit and terminal outcome. |
| TC-063 | examined | Eight-step planned procedure covering AC-1 to AC-10 through admission, generation, installed proof execution and owner replay. |
| FR-014-AC-2 | context_only | enum comparison (including equality and inequality over a selected model-owned `model`/`variant_type`, using its original checked member identities) |
