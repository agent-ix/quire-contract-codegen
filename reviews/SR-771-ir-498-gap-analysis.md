---
id: "SR-771"
title: "CG PR 226 gap analysis: bounded_domain member types and the TUP_PAIR corpus change"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@ea3a8985efa9e2381cadbde6075486ed3a76b951; src/composite_equality.rs, tests/composite_equality_support/**, tests/it/composite_equality_*.rs, tests/it/exact_function_*.rs, spec/oracle/functional/FR-018-composite-equality-oracles.md, spec/oracle/matrix/tests.md"
---

# SR-771: CG PR 226 gap analysis

## Summary

Ticket: IR-498. This analysis is scoped to the PR diff. Plan completion: not assessed.

Units examined:

- **The new `BoundedDomain` arm** (src/composite_equality.rs:1023-1032) and the FR-018
  §Inputs clause it implements.
- **The TUP_PAIR corpus node** (tests/composite_equality_support/package.rs:1040-1048) and
  its consumers:
  - composite_equality generation: `tc_029_ac4_schedule_matches_checked_equality_for_every_shape`
    requests E_TUPLE over TUP_PAIR.
  - composite_equality agreement: the E_TUPLE vectors in agreement_cases.rs:80-105, against
    `environment_tuple()`, agreement.rs:223-236.
  - exact_function: tests/exact_function_support/package.rs builds on the same corpus
    package. It only needs that package to admit, and no exact_function item compares a
    TUP_PAIR operand.
- **Whether the text leaf is exercised in both suites.** It is, in the sense the ticket
  needs: both suites admit the package through IR's leaf rule. Only composite_equality
  generates and runs an oracle over the tuple.
- **The matrix row edit** (FR-018-AC-2). It only drops the SHA, and its status stays honest
  at Partially covered.
- **No-weakening check.** No assertion was removed or loosened. No `#[ignore]` was added. The
  cyclic-refusal test changed in its doc comment only.

## Findings

| ID | Severity | Summary | Refs |
|----|----------|---------|------|
| FND-001 | medium | The only evidence for the new `bounded_domain` arm is that generation does not refuse. No test asserts what the arm reconstructs. `generated::environment_e_tuple_*` is never compared with agreement.rs `environment_tuple()`: the oracle takes the caller's environment, and `check_type` only checks that `Composite(TUP_PAIR)` exists. AC-13 compares keys only. So reading the wrong `text_bounds` members, or the union of sibling bounds, would pass. The integer, rational and decimal bases have no test. Neither does the refusal of a non-scalar base (Unsupported `bounded_domain`), nor the "never a sibling bound" clause. A TC-029 test should compare the reconstructed TUP_PAIR declaration with `Tuple[Int[-100,100], Text(0,16,Nfc)]`, and cover one numeric `bounded_domain` member and the non-scalar-base refusal. | src/composite_equality.rs:1023-1032; tests/composite_equality_support/agreement.rs:223-236; tests/it/composite_equality_generation.rs:194-240 |
| FND-002 | low | The corpus is still only partly shaped the way QSL emits it. TUP_PAIR position 0 references the scalar `T_INTEGER_BOUNDED` with a sibling `BD_INTEGER`, but QSL (lowering.rs `ValueType::Int`) emits the `integer_range` `bounded_domain` node as the member type. The top-level text operands (E_TEXT) are typed by the plain `T_TEXT` scalar. The PR title's "as QSL emits" holds for the tuple's text position only. | tests/composite_equality_support/package.rs:1047; tests/composite_equality_support/package.rs:1139 |

## Verdict

The corpus fix is honest and minimal. The generator arm is reached, but how well it is tested
is overstated: it is exercised by one text case, and only as "no refusal". FND-001 should be
fixed in this PR, since the arm is new code the PR introduces. FND-002 is low and can be
recorded on IR-453 next to the hand-patched TUP_PAIR note.

## Dispositions

| FND | outcome | sha/reason |
|-----|---------|------------|
| FND-001 | fixed | 9e564e7: five TC-029 tests. The rendered TUP_PAIR declaration is asserted exactly as `Tuple[Int[-100,100], Text(0,16,Nfc)]`. Integer, decimal and rational `bounded_domain` members are byte-identical to base-scalar members. A sibling-bound test, a wrong-form refusal, and boolean/enum/record refusals. Mutation-tested in a scratch copy, since restored: removing the form check fails 2 tests, and reading the sibling-bound union fails the sibling test. Asserting the rendered source instead of comparing with agreement.rs `environment_tuple()` is sound: that is a hand-built runtime environment, and the rendered string is the generator's own output. |
| FND-002 | deferred | Recorded on IR-453 for step 4g (2026-10-01 comment). The PR was retitled and its body states that only the text position is QSL-shaped. |
