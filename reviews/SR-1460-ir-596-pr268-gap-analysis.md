---
id: "SR-1460"
title: "IR-596 PR 268 gap analysis: FR-031 closing set against tests and the Test Matrix"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@57ad290a70d39f20989635a10fc6af8bce3dc677; spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md, spec/oracle/matrix/TC-044-boolean-oracle-integer-arithmetic.md, spec/oracle/matrix/tests.md, spec/tests.md, tests/it/oracle_arithmetic.rs, tests/it/kani_generation.rs, tests/it/oracle_generation.rs, tests/it/bound_strategy_generation.rs, src/oracle/boolean_v1.rs"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-031
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-044
    type: reviews
---

# SR-1460: IR-596 PR 268 gap analysis

## Summary

Ticket: IR-596. PR: agent-ix/quire-contract-codegen#268, head 57ad290. Plan completion: not
assessed. The scope is the closing set of FR-031 (AC-1, AC-3 to AC-5, AC-7 to AC-15 and AC-18 to
AC-21) and the held ACs (AC-2, AC-6, AC-16 and AC-17), checked against their trace tags, the
matrix rows, `quire coverage --strict --json` on the base and the head, and a run of the new tests
on the base. Also read: TC-044, interface-001's `oracle_slice`, FR-008-AC-3 and AD-004 L-5.

## Method

- Ran `quire coverage --strict --json` (quire 0.36.1) on the base 3fa7408 and on the head, and
  diffed `unbacked_rows`, `coverage_matrix`, `undeclared_statuses` and `unmatched_tags`.
- Grepped every `Trace:` line in the diff for FR-031 ids.
- Ran `tests/it/oracle_arithmetic.rs` on the base, with the two error-code variants added.
- Read each flipped AC against the test that carries its tag.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-031-AC-11 and the Status and Test obligations sections require QSL's exemplar (quire-integration IT-011, IT-010-SC-05 and IT-011-SC-03) to be run against this branch before and after the change, and the run recorded. It has not been run. The in-repo stand-in (`kani_exemplar_verifies_and_its_addition_mutant_is_falsified_at_999`) builds the clause directly through `DeclarationEnvironment`. So it does not show that QSL's frontend lowers the exemplar to this typed expression. It does not run QSL's syn mutation, the connective mutation (IT-011-SC-02) or IT-010-SC-05's comparison clause, and it does not assert AD-004 L-5's "every cover satisfied". This is a merge gate, not a spec defect: AC-11 says honestly that the run is pending. | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:61-65, spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:331-335, tests/it/kani_generation.rs:1552-1608 |
| FND-002 | low | The `--strict` unbacked count does not see the four held ACs, which no test backs. The head reports 67 unbacked rows against 91 on the base. The 24 rows that disappear are the 21 FR-031 verification rows, the two FR-031 matrix rows and the TC-044 traces-to row. Five of them disappear for no test-backed reason: the verification rows of AC-2, AC-6, AC-16 and AC-17, and the Held matrix row (`spec/oracle/matrix/tests.md:46`). Each lists `TC-044` as a target, and the new tests tag `TC-044`. The per-AC `coverage_matrix` stays truthful (those four are `untagged`, the rollup is 17 of 21), and no test tags a held AC. So nothing claims more than the tests back. But the PR's "24 FR-031 rows are gone" reads as if all 24 were now backed. No partial AC-11 row is added to `unbacked_rows`. The one new entry is in `undeclared_statuses` (the AC-11 matrix row's `⚠️ Partially covered` status, the same form as the existing FR-014, FR-018, FR-021 and FR-022 rows). | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:342-357, spec/oracle/matrix/tests.md:46 |
| FND-003 | low | The AC-13 and AC-14 tests pass on the base as well, which was measured. Like AC-4 and AC-12 they cannot show the defect, but the spec labels only AC-4 and AC-12 as held agreements. They still test the new code: adding `unwrap` to the emitter, or a non-deterministic order, would fail them. | tests/it/oracle_arithmetic.rs:883-936 |

## Verdict

There is no matrix inflation, and the IR-29 / IR-576 failure mode is not present:

- No trace tag names AC-2, AC-6, AC-16 or AC-17, and `coverage_matrix` lists those four as
  `untagged`. The held matrix row reads `🚧 Held`, and the AC rows read `HELD (IR-601...)`.
- The other 17 ACs are each tagged by a test whose assertions match the AC text. AC-20 is tagged
  only by an ignored real-Kani test (`tagged-by-ignored-test`), as the matrix row says. AC-11 is
  `⚠️ Partially covered` with the QSL run pending.
- The tc_017 trace line wraps onto a second `///` line, and quire does read it: tc_017 backs AC-10
  and AC-18.
- `unmatched_tags` gains IT-011, IT-010-SC-05, IR-601 and AC-19. These are prose tokens in doc
  comments and harmless.
- AC-5, AC-7 and AC-18 each have a test that fails on the base: 13 of the 18 non-ignored new tests
  fail there. The AC-5 failure on the base is a compile error in the generated crate. The panic
  itself follows from the base's raw `i64` operators, which I confirmed in the base's emitted
  text, and from the PR's recorded probe.
- FR-008-AC-3 and interface-001's `oracle_slice` (supported grammar, undefined-result boundary,
  refusal locus, terminal states) match the code.

Mergeability: mergeable only after the QSL exemplar run (FND-001) is made and recorded against this
branch, and the low findings are dispositioned.

## Dispositions

Round 1, reviewed at c2e897c3ce84e0bc115f3c339e1e0ff36a342daf (fix commit c2e897c on 57ad290).
FND-001 (the QSL exemplar run) stays open and is held by the coordinator. It gets no row this
round.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-002 | accepted-no-change | The masking is quire's row-level granularity, not a repository defect. Re-measured at c2e897c, `quire coverage --strict` still reports 67 unbacked rows (the same set as 57ad290), and the per-AC `coverage_matrix` still shows AC-2, 6, 16 and 17 as `untagged`. The PR description now says the held ACs and the held matrix row are still unbacked even though they left the unbacked list. Nothing in the repository claims they are backed. |
| FND-003 | fixed | c2e897c: FR-031-AC-13 and AC-14, TC-044 step 2 and the oracle matrix row now label AC-13 and AC-14 as held agreements, alongside AC-4 and AC-12. |
