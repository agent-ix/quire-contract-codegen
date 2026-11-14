---
id: SR-17940486709564847047983
title: "IR-262 PR #833 gap-analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-contract-codegen PR #833 at 36cb1232, git diff origin/main...HEAD; IR-262; FR-049-AC-1 to FR-049-AC-11, TC-063; src/kani/generate/enum_scalar.rs, src/routed/generate.rs, src/replay/scalar.rs, tests/it/enum_scalar_harness.rs, src/routed/contract_package_tests.rs (pre-existing FR-049 tags)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-049
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-063
    type: reviews
---

## Summary

Ticket: IR-262. This is an AC-to-test gap analysis of FR-049 against PR #833 at 36cb1232. I read
`quoin matrix --json` (quoin 0.29.1) for FR-049. All eleven ACs have binders, and every one is
reported `undischarged` (no evidence bound). I then read each bound test against the AC wording.

Focused tests at the head, in a scratch copy, own target dir, under `flock /tmp/cargo-slot.lock`:
`--lib enum_scalar` gave 9 passed; `--test it enum_scalar_harness` gave 14 passed and 20 ignored.

**AC-by-AC:**

- **Fully backed:** AC-1, AC-3, AC-6, AC-7, AC-8 (on the carrier, plus the QSL source refusal) and
  AC-10's decode and refusal half.
- **Partly backed:** AC-2, AC-4, AC-5, AC-9, AC-10's replay clause and AC-11. The findings below
  give the detail.

## Verdict

**FAIL: three medium findings.** Some ACs are tagged as passing when a clause of them has no test,
and the matrix will count them as covered. The PR's own Status gives the owner-replay and
tool-version gaps. It does not give the AC-2 mutants, the AC-4 typed-outcome and IllTyped clauses,
or the AC-9 families, and it says the opposite of the AC-9 gap ("every other family's records and
identities stay byte-identical (tested)").

What is sound:

- Each mutant of AC-6 has its own installed-Kani test, and AC-7's classifier and runner controls
  are bound to the enum harness's own cover count.
- The decode refusals of AC-10 each assert a typed code.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-049-AC-4 is tagged as passing, but two of its three clauses are not met. (1) "A control producing `Incomplete` keeps that typed outcome": `kani_an_incomplete_comparison_is_not_verified` sets `work_units: 0` and gets `Falsified`. The harness folds `Incomplete`, `Err(IllTyped)` and a real disagreement into one `sound == false` assertion, so the evidence cannot tell them apart and the `Incomplete` type is lost. (2) "A path reaching the comparison's `Err(IllTyped)` is non-`Verified` and retains its cause": no test reaches it, and the harness keeps no cause. Fix: give each non-Completed arm its own assertion message (or classification) so `Incomplete` and the IllTyped cause survive into the evidence, and test both. Or, if the team accepts `Falsified` as the typed outcome, amend AC-4's wording and stop claiming the clause. | src/kani/generate/enum_scalar.rs:473-487; tests/it/enum_scalar_harness.rs:661-673 |
| FND-002 | medium | FR-049-AC-2 says "Admitting one extra index, dropping the last member or fixing one operand to a constant fails the correspondence check". No test applies any of those three mutants. `the_symbolic_domain_is_exactly_the_declared_members` only counts `kani::assume(...)` and `kani::any()` substrings in the healthy generated text. Fix: apply each mutant to the generated harness and show it fails: an installed-Kani mutant like the AC-6 ones (an extra index makes `from_index(..).expect` panic), or whatever correspondence check the row defines. | tests/it/enum_scalar_harness.rs:247-287 |
| FND-003 | medium | FR-049-AC-9 lists text, rational, decimal, quantity, float, sequence, set, bag and ordered set, plus an enum leaf inside a composite keeping `ShadowFamilyNotBuilt`. The only routed byte-identity check covers one integer node (`not_enum`). The FR-049 Status says "every other family's records and identities stay byte-identical (tested)", which overstates it. `settle_enum_items` only rewrites `OperationNotRendered` records whose identity starts with `quire.op.enum.`, so the risk is low, but the AC and the status claim are untested. Fix: add the families QSL emits today, and an enum leaf in a record equality, to the routed legacy-vs-admitted comparison. Or narrow the status claim to what is tested. | tests/it/enum_scalar_harness.rs:675-715; spec/kani/functional/FR-049-enum-scalar-harnesses.md (Status) |
| FND-004 | medium | Stale FR-049 trace tags from main inflate the matrix. Four integer-scalar occurrence tests in `src/routed/contract_package_tests.rs` carry `FR-049-AC-1`, and one also carries `FR-049-AC-5`. They are `scalar_own_result_without_conversion_differs_from_not_applicable`, `duplicate_expression_uses_each_selected_conversion_context`, `scalar_selected_occurrence_refusals_preserve_generated_sibling` and the one at line 2886. They test integer add occurrence selection, not enum comparison. Before this PR FR-049 was "untagged" by its own Status. Now that the PR claims FR-049 coverage, `quoin matrix` binds these to AC-1 and AC-5. Fix: drop the FR-049 ids from those four `#[trace]` attributes in this PR. | src/routed/contract_package_tests.rs:2027,2104,2812,2886 |
| FND-005 | low | The `#[trace]` on `two_enum_nodes_of_one_operator_in_one_request_do_not_collide` (FR-049-AC-8) is wrong: the test checks harness-name uniqueness, not a disposition. AC-8's "a refused item does not alter a supported sibling" is only shown across two separate `generate_enum_scalar_harness` calls (`a_missing_owner_declaration_is_unsupported_with_a_named_capability`), never inside one routed request. Fix: retag the collision test (AC-1 or AC-10 identity), and add a routed request that mixes a refused enum item (for example with a foreign context or a path condition) with a supported sibling. | tests/it/enum_scalar_harness.rs:717-732,333-355 |

## Examined scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-049-AC-1 | examined | CG generates one harness whose generated side is quire-exact's `compare_native_enum` over carriers and whose expected value is the native operator over indices. |
| FR-049-AC-2 | examined | each operand index is an `i64` constrained to `0..member_count` ... Admitting one extra index, dropping the last member or fixing one operand to a constant fails the correspondence check. |
| FR-049-AC-3 | examined | `from_member` is run over the owner-admitted `EnumShape` ... A generator-mint mutant ... fails the comparison of that shape with the owner-admitted shape. |
| FR-049-AC-4 | examined | A control producing `Incomplete` keeps that typed outcome and cannot become `Verified`. A path reaching the comparison's `Err(IllTyped)` is non-`Verified` and retains its cause. |
| FR-049-AC-5 | examined | replay through the owner route reproduces the disagreement. |
| FR-049-AC-6 | examined | Mutants fail the operator that can detect them. |
| FR-049-AC-7 | examined | Each emitted enum harness has exactly one completion cover after all assertions. |
| FR-049-AC-8 | examined | refused with the typed ill-typed cause ... a refused item does not alter a supported sibling. |
| FR-049-AC-9 | examined | A leaf or member of any other family ... keeps its existing disposition. |
| FR-049-AC-10 | examined | a valid index replays as the declaration's member at that position. |
| FR-049-AC-11 | examined | Run evidence records ... tool versions, target ... Replay uses the retained generated artifact without hand editing. |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The harness asserts `!incomplete`, `!unordered`, `!distinct` separately before the agreement check; Kani tests assert the counterexample names `stopped Incomplete`, `UnorderedEnumOrdering` and `DistinctEnumDeclarations` (re-run at 377d0dd2). |
| FND-002 | fixed | The three AC-2 mutants are applied to the generated harness and rejected by `domain_corresponds`; the extra-index mutant is also Falsified under installed Kani (`kani_one_extra_index_is_falsified`). |
| FND-003 | accepted-no-change | Planner/coder narrowed AC-9 coverage to Boolean, integer, text and sequence nodes (all now byte-identical in the routed comparison); the FR-049 Status, TC-063 and tests.md state the untested families and FR-049 stays PARTIAL, so the narrowing is honest. Note: QSL already compiles Rational, Decimal and Float64 parameters (QSL tc_906), so those families could be added cheaply later. |
| FND-004 | fixed | FR-049 ids removed from the four `#[trace]` attributes in `src/routed/contract_package_tests.rs`. |
| FND-005 | accepted-no-change | The collision test is retagged to FR-049-AC-1. The refused-sibling clause is stated as tested across separate generations only; `settle_enum_items` rewrites each record independently, so the narrowing is honest and low risk. |
