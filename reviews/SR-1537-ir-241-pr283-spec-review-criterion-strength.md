---
id: SR-1537
title: "Criterion-strength review of quire-contract-codegen PR #283"
type: SpecReview
analysis: criterion-strength
scope: "agent-ix/quire-contract-codegen@1f3025ea99c3fe1fa804ed43fce116e339f9c96d; FR-028-AC-13..23, FR-015-AC-69..76, FR-018-AC-21..22 and their mutation rows"
review_set: subset
---
# Criterion-strength review of PR #283

## Summary

Ticket: IR-241 (also IR-264). This review asks, for each of the 21 new criteria, whether it can
actually fail, and whether each mutant the PR names is one that its criterion can tell apart from
correct behaviour. The judgments were made by reading the spec text by hand, with the rules
checked against QSpec FR-149 (quire-specification origin/main 2b2dd28) and Contract Runtime's
`plan_pairs` (9597b43). Jev was not used.

Most criteria are direct and failable, with a named mutant: FR-028-AC-13, AC-14, AC-15, AC-17,
AC-19, AC-20, AC-21, AC-22 and AC-23; FR-015-AC-69 to AC-73 and AC-76; FR-018-AC-21. Five findings.
One is high: two mutants FR-015-AC-75 requires to be caught are equivalent under what the harness
and the refinement compare. Four are medium.

## Scope examined

All 21 new ACs (examined) and the 13 mutation rows (examined). QSpec FR-149 "Occurrence-pair plan"
and Contract Runtime `src/exact/equality.rs` `plan_pairs` (context_only).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-015-AC-75 requires two production-side catches that the compared observables cannot make. (a) "one operand's descriptor swapped with the other's": AC-69 admits only operands of one type, and AC-73 refuses `convert<T>`, so both descriptors are identical. FR-149 equality is symmetric and the pair count is a node count, so the swap is an equivalent mutant and refinement agrees. (b) "a mutation of the closure reader's field order ... settles `refinement_failed`": the verdict is a conjunction over fields and the count is a sum with no early exit (FR-149), so neither changes under a consistent permutation of fields. And if the production oracle reads "through FR-018's reader", which FR-018-AC-21 requires to be the single reading, the mutation moves both sides. The clause can then pass only by accident, for example through a construction refusal. Replace each with a mutant the observables distinguish (for example a reader that drops or duplicates a field, or swaps a field's presence), or say how the mutant is caught. | FR-015-AC-75; FR-018-AC-21; QSpec FR-149 Occurrence-pair plan |
| FND-002 | medium | FR-028-AC-18 requires that "a mutation of each listed shared helper, run through the shadow harness and the refinement run together, fails at least one of them". For a helper whose only output is an order (the field-order helper the FR-015 prose names), every consistent permutation is an equivalent mutant (FND-001(b)), so the clause cannot be met as worded. It should require a mutation that changes an observable the comparison reads, or exempt order-only helpers with a reason. | FR-028-AC-18; FR-015 IR-264 prose ("the closure reader and the field order it returns") |
| FND-003 | medium | FR-018-AC-22 claims the property "for every pair of operand values admitted by the declared operand types", a universal claim over ranges as wide as `i64`. TC-029 step 14 tests "an integer range of three values". So either the AC is not testable as stated, or the test does not back it. The AC should be restated over the domain the test enumerates (or the FR-028 refinement domain), and the universal property left to the refinement obligation's class. | FR-018-AC-22; TC-029 step 14 |
| FND-004 | medium | FR-028-AC-16's `sampled` class has no floor. It sets no minimum on the seeded remainder, does not say whether the boundary set is combined per leaf or as a cross product (the product of five boundary values over eight leaves is already 390,625 cases), and does not say where the seed comes from. A sampled run of the boundary set with zero remainder, or a single boundary value per leaf, meets the AC. Two implementers will differ, and the weakest legal `shadow_proved_refinement_sampled` is close to vacuous. | FR-028-AC-16 |
| FND-005 | medium | FR-015-AC-74 accepts the corpus refinement run "classified as FR-028-AC-16 states", so either class passes. The only real-Kani acceptance of the family can therefore pass with a `sampled` refinement and never show `shadow_proved_refinement_exhaustive`, the class FR-028's Q1 offers the owner as the candidate end state. The corpus case's domain is small by construction, so the AC should pin it to `exhaustive` and record the case count. | FR-015-AC-74; FR-028-AC-16; FR-028-AC-17 |

## Verdict

The criteria are mostly strong. FND-001 has to be fixed before merge: as worded, FR-015-AC-75 asks
a test to catch mutants that cannot be caught. FND-002 to FND-005 make the sampled path, the
shared-helper guard and the oracle-totality claim precise enough that two implementers would
write the same test.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | medium | One of FR-015-AC-75's production mutants, "an optional field's declaration emitted as required", is not distinguished by the refinement observables as specified. Contract Runtime's `check_type` is type-level, and `plan_pairs` walks the two values' slots without reading presence from the environment. An oracle whose environment declares the field required therefore still completes absent and null pairs with the shadow's verdict and count. The mutant surfaces only if the abstraction function builds production values through the oracle's own (mutated) environment constructor, so that `evaluate_record` refuses `MissingField` or `NullForRequiredField`, and a construction refusal counts as a disagreement. FR-028-AC-14 and AC-15 say neither, so the clause can pass or fail depending on an unspecified choice. Either state in FR-028-AC-14 and AC-15 that the abstraction function constructs through the production oracle's environment and that a construction refusal is a disagreement, or replace the mutant. FR-018-AC-23 catches a closure read as required, but not this oracle-side mutant. | FR-015-AC-75; FR-028-AC-14; FR-028-AC-15; Contract Runtime 9597b43 src/exact/equality.rs plan_pairs, src/exact/composite.rs check_type and evaluate_record |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed 3e0bc7c | The descriptor-swap mutant and the field-order reader mutant are removed. The production mutants are now opposite operator, absent equal to null, one pair too many, optional declared required, and a legal pair Refused. The reader mutants (dropped member, bound narrowed by one, optional read as required) are caught by FR-018-AC-23's independent read. Against FR-149 and RT `plan_pairs`, the first, second, third and fifth production mutants change the verdict, the count or the outcome. The fourth does not on its own, see New FND-006. |
| FND-002 | fixed 3e0bc7c | FR-028-AC-18 marks shared helpers `value_bearing` or `order_only`, and the mutation rule applies only to `value_bearing` ones. The exemption is correct under QSpec FR-149: the verdict is a conjunction over fields, the pair count is the node count with no early exit, and 'once the plan is reserved, an implementation's evaluation order is unobservable'. A consistent permutation applied to both operands of one type therefore changes neither observable. |
| FND-003 | fixed 3e0bc7c | FR-018-AC-22 is now stated over the item's refinement domain, and TC-029 step 14 enumerates the whole of a domain chosen to fit the case cap. |
| FND-004 | fixed 3e0bc7c | FR-028-AC-16: boundary cases are taken one leaf at a time plus all-lower and all-upper, not as a cross product. A sampled run adds exactly the request's sample size (a positive integer) of uniform seeded cases from a request-carried u64 seed. A run with fewer is not `sampled`. |
| FND-005 | fixed 3e0bc7c | FR-015-AC-74 requires the corpus case's domain to fit the cap and classify `exhaustive` with its case count, and requires a second case over the cap to show `sampled`. |
| FND-006 | fixed 5833419 | FR-028-AC-14 requires the abstraction function to build production values through the oracle's own environment, using the evaluator's declaration-checked constructors. FR-028-AC-15 and the FR-028 Behavior count a construction refusal as a disagreement, and the AC-15 mutation row and TC-039 step 13 add that stand-in. FR-015-AC-75 names the catch. Verified in the code: Contract Runtime 9597b43 `TypeEnvironment::evaluate_record` refuses `MissingField` (absent slot, Required field) and `NullForRequiredField` against the declared presence. quire-exact 2ec5e1e `value.rs` `evaluate_record` refuses the same causes. So the mechanism survives IR-349, where CG's generated oracle, which today uses `quire_contract_runtime::exact`, moves to quire-exact. The corpus domain covers every presence state, so the mutant is reached. |
