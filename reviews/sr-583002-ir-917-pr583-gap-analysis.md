---
id: SR-583002
title: Gap analysis of IR-917 CG PR 583 (V2 clause harness oracle and arithmetic proof controls)
type: SpecReview
analysis: gap-analysis
scope: 'agent-ix/quire-contract-codegen; PR #583 head cc02657484bbd99fdba3ec26326968434ee5a7b8 vs base
  9196cbe40a9341c32b0f85afda22d342dddf193c; src/kani/generate/clause/v2.rs, src/kani/generate/negotiate.rs,
  src/oracle/clause.rs, tests/it/kani_obligations_state_frame.rs; criteria FR-014-AC-38, FR-015-AC-40,
  FR-015-AC-41, FR-015-AC-119, TC-025'
review_set: subset
date: '2026-10-26'
relationships:
- target: ix://agent-ix/quire-contract-codegen/FR-014
  type: references
- target: ix://agent-ix/quire-contract-codegen/FR-015
  type: references
- target: ix://agent-ix/quire-contract-codegen/TC-025
  type: references
---

## Summary

Ticket: IR-917. PR: quire-contract-codegen#583, frozen head cc02657484bbd99fdba3ec26326968434ee5a7b8
over base 9196cbe40a9341c32b0f85afda22d342dddf193c. Reviewer model: Claude Opus 5.5
(`claude-opus-5-5`). Method: quoin gap-analysis, planless. The matrix was computed with
`quire matrix --scope . --format json` at the head (quire 0.36.2, engine 0.50.2; quoin 0.29.1). The
semantic review step was not offered as a separate opt-in. The dispatcher asked for specific intent
checks, and those are recorded per criterion under Coverage.

## Verdict

FAIL for repository-wide traceability by the verdict rule: 315 criteria are untagged at this head and
16 are tagged only by ignored tests. All of these are inherited. The PR adds no criteria, edits no spec
file and removes no trace tag, so it introduces none of them. Scoped to this PR the result is
CONDITIONAL, with one medium and three low findings. No acceptance credit is given: no installed-Kani
verdict exists at this head, and FR-015-AC-40, AC-41 and AC-119 correctly remain PLANNED.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-015-AC-41's installed-backend clause rests only on the `#[ignore]`d, unrun `tc_025_real_kani_falsifies_both_same_clause_addition_mutants`. The matrix's `tagged` status reflects generation-time assertions only: Verified with cover, and Falsified with playback 999, have no run evidence. | tests/it/kani_obligations_state_frame.rs:767-910 |
| FND-002 | low | FR-015-AC-119 is untagged at this head. Its CG-side generation clauses (one draw, same value to the subject, independent `!left \|\| right`, no helper in oracle or expectation) are asserted by a test tagged AC-38/40/41 rather than AC-119. The installed healthy and helper-body mutant runs belong to quire-integration #24 and have not run. | spec/kani/functional/FR-015-bounded-kani-obligations.md:866 |
| FND-003 | low | `native_clause_expectation_accepts_only_in_range_literal_narrows` is tagged FR-015-AC-40, which is oracle byte-identical embedding. The test asserts the native expectation's literal-narrow admission, which belongs to the AC-41 independent-assertion clause. | src/kani/generate/clause/v2.rs:857-869 |
| FND-004 | low | The literal-narrow elision in clause lowering and in the native expectation is behaviour no criterion states. FR-014-AC-38 lists literals, references and two field terms as operands and does not mention a `quire.op.numeric.narrow` wrapper, and FR-014-AC-45 covers narrowing only for scalar outer operations. The refusal of out-of-range or non-literal narrows is therefore unspecified. | src/oracle/clause.rs:510-525 |

## Coverage

Per criterion, at the head:

- FR-014-AC-38 (Test, TC-024): tagged by 15 tests. The new unit test `a_literal_narrow_is_elided_only_when_its_value_is_inside_the_target`
  asserts the exact retained literal step and four refusals (out-of-range, no bound, non-i128, Boolean
  argument). Its assertions can fail.
- FR-015-AC-40 (Test, TC-025): tagged by 4 tests. The 12-condition loop asserts that the public
  `generate_v2_clause_oracle` source appears exactly once, contiguously, in each general harness:
  connectives, `not`, `implies`, Boolean `=`/`!=`, and the six comparisons. The implication test does
  the same for the native oracle. Same-clause oracle bytes are verified at generation time. PLANNED,
  unchanged.
- FR-015-AC-41 (Test, TC-025): see FND-001. At generation time, the native probe has exactly one
  `evaluate_native_integer_arithmetic` site, and the probe assertion compares the kernel result with an
  independent i128 expression. The mutant test's reachability was checked: `make kani` runs
  `--ignored` with filter `kani_obligations`, which matches the module path. Its assertion strength is
  recorded in SR-583001 FND-001.
- FR-015-AC-119: see FND-002. PLANNED, owned with quire-integration #24 SC-02.
- TC-025 step 11: the generation half is backed. The installed-backend half is unrun.
- Checked literal narrow semantics: the value is preserved and refusals are typed. I did not inspect
  parity of QSL runtime narrow metering, so that is an evidence limit, not a pass.
- General postcondition RT equality: `amount = 0` binds `rt::Integer::from`, calls
  `environment.check_equality`, and typechecks offline in a scratch crate (author-run). The other
  general shapes are not compile-checked (SR-583001 FND-006).
- Reverse gaps: Boolean `eq`/`ne` in the native expectation is owned by FR-015-AC-40. The general-route
  arithmetic refusal is near FR-015-AC-42 scope, which is untagged repository-wide, and is untested
  (SR-583001 FND-002). No stub, `todo!` or placeholder return was introduced.

Evidence limits: this reviewer ran no cargo build, test or Kani, by dispatcher instruction. Focused
test results are author-reported. Plan completion: not assessed.

## New findings (disposition pass 1)

Disposition pass 1 reviewed fix head 738ca6faa267e2966456a2bb26066a70ae19cca4 (rebased onto main
1175b995bc4314798cafe839a85a1ca2a745408c). The matrix was recomputed at that head: 565 tagged, 313
untagged, 16 tagged only by ignored tests, 53 method-without-symbol.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | medium | `general_v2_arithmetic_postcondition_refuses_without_a_harness` is tagged FR-015-AC-42 and is that criterion's only binder. AC-42 states that an unsupported operator (division, modulo, absolute value) is refused with a typed reason naming the node and the operator. The test asserts a general-route render refusal of the supported `+`, and its reason `V2ClauseConditionNotRendered { node_id }` names no operator. AC-42 changes from untagged to tagged without its claim being tested. | tests/it/kani_obligations_state_frame.rs:638-640 |
| FND-006 | low | FR-014-AC-54's two tagged tests cover the unchanged value and the refusal shapes (missing target, out of range, nonliteral, unparseable). They do not cover parity of ordered charges, consumed counters and outcome with direct-owner evaluation, independence from a generated sibling, or refusal of another `convert` identity. AC-54 correctly remains PLANNED. | src/oracle/clause.rs:743-784 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | deferred | The installed-backend clause of FR-015-AC-41 still has no run evidence. `tc_025_real_kani_falsifies_both_same_clause_addition_mutants` is ignored and was explicitly not run this round, because CG #569 holds the host. This PR claims no AC-41 acceptance, AC-41 and TC-025 remain PLANNED, and IR-917 stays open until a recorded `make kani` run exists. |
| FND-002 | fixed | `qsl_v2_implication_harness_embeds_its_clause_oracle_and_checks_native_arithmetic` now carries FR-015-AC-119 for its generation clauses. The installed healthy and helper-body runs remain owed by quire-integration #24 SC-02, so AC-119 remains PLANNED. |
| FND-003 | fixed | The narrow unit test is retagged `FR-014-AC-54`, `FR-015-AC-41`, `TC-025`. |
| FND-004 | fixed | FR-014-AC-54 (CG #584, merged to main and present at this head) now specifies checked literal narrowing for V2 clause oracles, including its refusals. |
