---
id: "SR-11915"
title: "IR-823 code review (incl. rust-review lane): TC-187 quantity rows in the exact-scalar agreement cases"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@ce909c258ceef3ab6920407d7c77ad2a95cf951e; tests/exact_scalar_support/agreement_cases.rs, tests/exact_scalar_support/agreement.rs"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: reviews
---

# SR-11915: IR-823 code review

## Summary

Ticket: IR-823. PR: agent-ix/quire-contract-codegen#404, head ce909c2 (06df2a1 plus one commit),
diffed with `git diff origin/main...HEAD`. The rust-review lane is folded into this file. The
production code is unchanged; the diff is test code and spec text. The spec text is reviewed in
SR-11916.

The PR adds TC-187 agreement cases for U02, U05, U09, U12, U14, U20 and U27, adds the `s` family to
the generated unit grid, runs U03, U13, U19, U21, U23, U24 and U28 at QSpec's own limit tuples with
the incomplete record asserted, and asserts U06 and U11 against the runtime's unit-graph admission.

## Method

I read the diff and compared each vector with QSpec TC-187 at quire-specification origin/main,
row by row. I checked each tabled limit tuple and each incomplete record against Contract Runtime
`RationalArithmetic::charge_bits` and `evaluate` at the locked revision 3cc88c48. U19: amount
max(2+1, 2+1)+1 = 4. U24: max(1+1, 2+1)+1 = 4. U23: the offset add of 5463/20 is max(1+5, 13+1)+1 =
15. U28: the `in` multiply is max(1+7, 1+13) = 14 and the product is 20. U13: 2^64 x 2 =
36893488147419103232. The head's tuples and records all match QSpec. The tuples at 06df2a1
(integer_bits 3, 2, 19/12/18 and 13) did not, and the follow-up commit fixed them.

Mutation testing, in a throwaway worktree of the head, with a throwaway Contract Runtime worktree at
the locked revision patched in through `--config patch`. Every mutant was killed:

| Mutant | Killed by |
| --- | --- |
| RT: affine check removed from add/subtract | U05, U14, U20 |
| RT: affine check removed from multiply/divide | U05, U14, U20 |
| RT: affine check removed from power | U05 |
| RT: dimension equality by exponent shape, ignoring node keys | U09 |
| RT: zero scale admitted | U06/U11 |
| RT: missing-root check removed | U06/U11 |
| RT: duplicate-root check removed | U06/U11 |
| RT: cross-dimension-target check removed | U06/U11 |
| RT: a cycle reported as `UnknownTarget` | U06/U11 |
| RT: zero divisor detected after the edges | U27, grid |
| RT: affinity from the unit's own edge offset, not the composed one | U20 |
| RT: zero exponents kept in compound terms | U02, U12 |
| RT: compound quotient adds unit exponents | U02, U12 |
| RT: add/subtract charge amount loses its +1 | tabled tuples (U28 record) |
| RT: multiply charge amount is a max, not a sum | tabled tuples |
| RT: quantity arithmetic skips `unit.target-domain` | tabled tuples (U19 record) |
| generated: `quantity_add` calls Subtract | grid, U09, U20 |
| generated: `quantity_multiply` calls Divide | grid, U02, U12 |
| generated: `quantity_divide` calls Multiply | grid, U02, U12, U27 |

Every runtime mutant survives the grid, because both sides call the same runtime. The new
expected-disposition assertions are what catch them. That is the point of this PR, and it works.

The U11 raw-key graphs are sound. `node(n)` gives distinct 64-digit keys. A graph with no root must
be cyclic once every target exists, so the two-cycle is the only possible no-root shape.
`UnknownTarget`, `CrossDimensionTarget` and the self-loop `TargetCycle` each reach the cause they
assert. The one caveat is FND-002.

Vendoring: the new code (the `Arithmetic` helper, `agree_arithmetic`, `agree_power`,
`agree_comparison`, `assert_incomplete`, the U-tests and the raw-key graphs) is CG's own. Contract
Runtime `tests/exact_vocabulary.rs` has admission tests for the same causes, but they are written
differently: other key constructors, a two-cycle with a root, and a different cross-dimension
shape. Nothing here was copied. The harness provenance line that was reworded ("Adapted from the
Contract Runtime agreement harness") is pre-existing. Its source, RT `conformance/qsl-agreement`,
was deleted from RT by IR-430.

Gates, at the head, in my throwaway worktree: `make fmt-check` 0, `make lint` 0, and `make spec` 0
(warnings only). The scratch agreement crate ran 16/16. `make test` exited 2: 32 lib tests failed,
all in `kani::run::execute::batch_tests`, which this PR does not touch. The `it` run and the
main-branch comparison are recorded in the Verdict.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The zero-agreement guards in the TC-187 tests count with literals (`sums += 4`, `named_powers += 4`, `tuple_comparisons += 5`, `agreements += 4`, `("TC-187 U20 affinity", 3)`, `("TC-187 U27 division by zero", 2)`, `assert_eq!(named, 4 + 3 + 4 + 6)`). They cannot fail. Deleting the U19 incomplete-tuple call, or any other straight-line `agree_*` call, leaves every count and the test green, so the guard does not protect the row list FR-014-AC-6 claims. | tests/exact_scalar_support/agreement_cases.rs:559, tests/exact_scalar_support/agreement_cases.rs:596, tests/exact_scalar_support/agreement_cases.rs:618, tests/exact_scalar_support/agreement_cases.rs:626, tests/exact_scalar_support/agreement_cases.rs:828, tests/exact_scalar_support/agreement_cases.rs:992, tests/exact_scalar_support/agreement_cases.rs:1029 |
| FND-002 | low | The U11 cross-dimension graph has two defects: its only `L` unit targets a `T` root, so `L` also has no root. The assertion therefore pins Contract Runtime's internal check order (`CrossDimensionTarget` before `MissingRoot`), not TC-187's rule. If RT reorders two independent checks, the test fails although both refusals are valid under TC-187. Giving `L` its own root makes the graph single-defect. | tests/exact_scalar_support/agreement_cases.rs:1102-1106 |
| FND-003 | low | Style. `compound_terms` takes a parameter it discards (`let _ = f;`). `for (work, left, right, truth) in [(9, …)]` is a one-element loop. agreement_cases.rs is a scratch-crate test target, so `cargo fmt --check` never reaches it, and `rustfmt --check` reports drift at about 16 sites in the new code (for example 543, 611, 704, 749). | tests/exact_scalar_support/agreement_cases.rs:771-772, tests/exact_scalar_support/agreement_cases.rs:598 |

## Verdict

Code: mergeable. All three findings are low. No production code changed. Each new test fails when
the runtime behaviour it names is wrong, and the arithmetic tests also fail when the generated
oracle is wrong; 19 of 19 mutants were killed. The QSpec dispositions asserted match TC-187 for
U02, U03, U05, U09, U12, U13, U14, U19, U20, U21, U23, U24, U27 and U28, tuples and incomplete
records included. U06/U11 honestly says it has no generated oracle and asserts runtime graph
admission. It is traced to FR-014-AC-6 (see SR-11916 FND-004).

Gate detail at the head, in my throwaway worktree under the session scratchpad:

- `cargo test --locked --test it`: 587 passed, 7 failed, 34 ignored.
  `exact_scalar_agreement::tc_024_generated_scalar_crate_agrees_with_direct_runtime` passed. All
  7 failures are AC-44/AC-32/AC-33 direct-request tests that panic with "QSPEC_DIR points at
  published QSpec". The Makefile resolves `QSPEC_DIR` to a sibling checkout, and my worktree has
  no `quire-specification` beside it. That is environmental.
- The 32 lib failures are all fake-launcher batch tests in `src/kani/run/execute.rs`. Every member
  ends `Inconclusive { reason: NoVerdict }`, which matches the known sandbox limit on
  launcher/signal tests.
- The PR changes no file that either set compiles: agreement_cases.rs and agreement.rs are built
  only by the scratch agreement crate. I did not re-run these sets at origin/main.
- The full gates should still run once, unsandboxed and from the real worktree, before merge.
