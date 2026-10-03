---
id: "SR-1115"
title: "CG PR 245 gap analysis: FR-014-AC-40..42, FR-015-AC-50, FR-018-AC-2/AC-20, FR-021-AC-23 against code and tests"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@5d60e06c836b96a3f803b53a267326d98656b57d; spec/oracle/functional/FR-014-exact-scalar-oracles.md, spec/oracle/functional/FR-018-composite-equality-oracles.md, spec/oracle/functional/FR-021-function-application-oracles.md, spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/oracle/matrix/tests.md, spec/kani/matrix/tests.md, src/oracle/**, src/kani/generate/negotiate.rs, tests/it/*.rs, tests/composite_equality_support/**; diff origin/main...HEAD, base 27153ba"
---

# SR-1115: CG PR 245 gap analysis: FR-014-AC-40..42, FR-015-AC-50, FR-018-AC-2/AC-20, FR-021-AC-23 against code and tests

## Summary

Ticket: IR-548 (also IR-547, IR-536, IR-537). PR: agent-ix/quire-contract-codegen#245 at 5d60e06. Plan completion: not assessed.

Each AC the PR implements or flips was checked against code, its tagged test and a mutation probe:

- **FR-014-AC-40.** `tc_024_a_byte_ceiling_lowering_failure_is_refused_per_item_as_its_own_refusal` covers the whole call: the ceiling sits one byte below the lowered package, the ceiling itself is shown to be exact, every item carries one shared `consumed`, and no function is generated. `tc_024_a_failed_record_is_refused_by_its_limit_kind` in the scalar module covers the hand-built per-node record.
- **FR-014-AC-41.** The scalar module test and `tc_024_the_classifier_reads_each_limit_kind_once` cover it. The probes that mapped bytes to work, swapped an unrecognised name, and mapped the scalar unrecognised kind to work were all killed.
- **FR-014-AC-42.** `tc_024_ac42_only_the_shared_classifier_reads_a_failed_records_limit_kind` covers the reworded AC.
- **FR-015-AC-50.** The hand-placed refusals in `tc_025_a_byte_ceiling_and_an_unrecognised_lowering_refusal_are_oracle_refused_unchanged` cover it. The negotiate probe was killed.
- **FR-018-AC-20.** `tc_029_ac20_*` covers the whole call and the module test covers the per-record mapping. The equality arm reverted to work exhaustion was killed.
- **FR-021-AC-23.** `tc_031_ac23_*` asserts that the call-node `consumed` differs from the body `consumed`, so it checks the call-node-first order. The module test covers the per-record mapping. The `limit` and `consumed` swap was killed.
- **FR-018-AC-2.** E_SELF is restored to `corpus_package`, `golden_items`, `agreement_names` and the agreement vectors, verbatim from before baab597. The probe that dropped E_SELF from the golden items was killed.
- **IR-537.** The deleted `tc_029_a_cyclic_compared_type_is_refused_by_ir_today` pinned an IR refusal that IR #256 (1a59efc, an ancestor of the locked cbcd790) removed. No other assertion was weakened: the diff deletes only that test, its fixture and a comment, and the scalar corpus floors are unchanged.
- **Coverage.** `quire coverage --strict` reports 66 unbacked rows on both this head and a clean main worktree at 27153ba. Backed rows rise from 219 to 225 of 334. `quire validate` over spec, plan and reviews exits 0.

## Verdict

The implemented ACs are backed by tests that a plausible regression fails. One low finding: a matrix status claims more than the corpus shows.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The FR-018-AC-2 row is flipped to "✅ Covered", but AC-2 also requires agreement with `quire_spec_language::value`. That QSL leg has not run in this repo since `agree3!` was deleted (IR-254); it is delegated to RT's conformance/qsl-agreement lane, and nothing shows that lane covers a recursive record like E_SELF. The row should say the QSL leg holds transitively through RT, or stay partial for E_SELF. | spec/oracle/matrix/tests.md:29, tests/composite_equality_support/agreement_cases.rs:15-18 |

## Dispositions

Round 1, reviewed at b8cf39ddc15e33d730d1dbc1d0deb349343f0d11.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed b8cf39d | The FR-018-AC-2 row is now "⚠️ Partially covered". It says the generated oracle agrees with `quire_contract_runtime`'s own check and evaluation (E_SELF included), and that the `quire_spec_language::value` leg has not run in this repo since `agree3!` was deleted, is delegated to Contract Runtime's conformance lane, and is not shown to cover a recursive record. This is accurate. Coverage stays at 66 unbacked: the row is unbacked again, as it was on main, and a new test backs another row. |
