---
id: "SR-1459"
title: "IR-596 PR 268 code review: Boolean oracle integer arithmetic through the runtime"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@57ad290a70d39f20989635a10fc6af8bce3dc677; src/oracle/boolean_v1.rs, src/oracle/bound_v1.rs, src/core/diagnostic.rs, src/kani/generate/clause.rs, src/kani/generate/negotiate.rs, src/kani/generate/v1_bundle.rs, src/strategy/harness.rs, tests/it/oracle_arithmetic.rs, tests/it/kani_generation.rs, tests/it/oracle_generation.rs, tests/it/bound_strategy_generation.rs, tests/it/main.rs"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-031
    type: reviews
---

# SR-1459: IR-596 PR 268 code review

## Summary

Ticket: IR-596. PR: agent-ix/quire-contract-codegen#268, head 57ad290, base 3fa7408 (current
`origin/main`, which is the merge base). Scope is `git diff origin/main...HEAD` only. This file is
the code-review artifact with the Rust lane (rust-review) folded in.

## Method

- Read the whole of `src/oracle/boolean_v1.rs` at the head and every other changed source and test
  file in the diff.
- Built a probe crate twice, once against the head and once against the base, that runs the same
  clauses through `generate_boolean_oracle`, `generate_kani_bundle` and
  `generate_tristate_harness`, and diffed the emitted text.
- Ran `make ci` on a fresh target directory in a review worktree: exit 0.
- Ran the new `oracle_arithmetic` module against an unchanged copy of the base with only the two
  new `GenerationErrorCode` variants added.
- Ran the two new real-Kani tests on the head, and mutation-tested them on a throwaway copy with
  its own target directory (results in the Verdict).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The renderer's infix path ignores the oracle shape. `infix_operator` always passes `OracleShape::KaniBundle` to `arithmetic_refusal`, so `plain_node` would print a raw `+`, `-` or `*` inside a native oracle if it ever reached a `reject` arithmetic node. Today only the arithmetic marks keep it away, and `holds_arithmetic` falls back to `false` (`unwrap_or(false)`) when an index is missing. The refusal rule is also checked three times: once in the analysis with the real shape, and twice in the renderer with a hard-coded shape. Suggested fix: keep the shape on `Renderer` and have `plain_node` refuse a `Numeric` node unless the shape is `KaniBundle`. | src/oracle/boolean_v1.rs:950-955, src/oracle/boolean_v1.rs:1027-1074, src/oracle/boolean_v1.rs:1259-1267 |
| FND-002 | low | `tc_023_native_proven_numeric_obligations_render_without_assumptions` now asserts that the guarded division is refused with `UnsupportedIntegerDivision`, so its name says the opposite of what it checks. | tests/it/oracle_generation.rs:1356-1432 |
| FND-003 | low | `generate_bound_strategy` runs `generate_bound_oracles` over the whole package, and that call is all-or-nothing. So a divide, remainder or `saturate` node in any clause now refuses the strategy for every other clause in the package as `UnsupportedClause`, naming the other clause. Before this PR those clauses generated. The all-or-nothing design was already there, but FR-031 widens the refused set, and neither FR-031's Consumers section nor FR-008-AC-3 states this or tests it. | src/strategy/bound/generation.rs:120-139, src/oracle/bound_v1.rs:141-153 |

## Verdict

The change does what it claims. It is mergeable once the findings are dispositioned and after the
QSL exemplar run (FR-031-AC-11). That run is a required acceptance run and is still pending.

Measured:

- **Arithmetic-free output is byte-identical.** Four arithmetic-free clauses were run through each
  of the three consumers (native oracle with source map, Kani bundle with proof graph, and
  tri-state harness). That includes all six comparisons, every connective, negation, Boolean
  literals and a `saturate` integer that is only compared. Base and head output are identical.
- **Exemplar-shaped bundle.** The input was the postcondition `amount < 1000` implies
  `amount + 1 <= 1000` over `0..=1000` under `reject`, requirement `PopulationRule@7`, clause
  `population-rule` (giving `fn oracle_populationrule_7_population_rule`), with a `true`
  precondition. The base and head bundles differ by exactly one added line. It is a plain `//`
  line comment, line 24 of the file, in the oracle's own header, before the identity consts and
  outside the oracle function:
  `// Integer arithmetic in this oracle is checked by Kani, which fails the proof on overflow; this file is not a native evaluator.`
  The comment holds no `+`, and syn does not keep line comments as tokens, so it cannot add a
  `BinOp::Add`. The function body is unchanged, with one `+` and the
  `quire_contract_runtime::operators::implies_short_circuit(` call. The proof graph JSON is
  unchanged. The `O-2` and `O-4` bundles differ the same way, by that one line.
- **The defects are fixed.** The base emits raw `(x_current) * (2_i64)`, `(x_current) /
  (y_current)` and a raw `*` over `saturate`, all on `i64`. These panic in a debug build at the
  TC-044 vectors, as Rust's own overflow and division-by-zero checks do. On the head, `reject`
  add, subtract and multiply are `rt::evaluate_integer_arithmetic` with `bound(min, max)`.
  Divide and remainder are refused with `UnsupportedIntegerDivision`, and the message names
  IR-601. `saturate` arithmetic is refused with `UnsupportedSaturatingArithmetic`. This holds in
  the native oracle, the bundle and the harness.
- **Old tree.** Against the base plus the two enum variants, 13 of the 18 non-ignored
  `oracle_arithmetic` tests fail. Five pass: AC-4 (both tests), AC-12, AC-13 and AC-14. The AC-5,
  AC-8, AC-9 and AC-21 failures there are compile errors in the generated crate (the meter
  argument and the `Outcome` type), not the overflow panic. The panic is shown by the base's
  emitted text above.
- **Outcome shape.** `Outcome<bool>` reaches only native consumers: `generate_boolean_oracle` and
  `generate_bound_oracles`. Plain-bool consumers refuse a `reject` add with
  `UnsupportedExpression`. That covers the tri-state harness, Kani obligation clause lowering and
  obligation negotiation. The bundle keeps `bool`. One intended code change was observed: for a
  clause with an integer dependency and an arithmetic node, the harness used to return
  `UnsupportedDependency` and now returns the oracle's refusal first. FR-031 Consumers sanctions
  this.
- **NFR-005.** The diff adds no `unwrap`, `expect`, panic macro, index or unchecked subtraction to
  `src/`. One correction to the claim: `generate_boolean_oracle_inner`, one of the five AC-8 scan
  bodies, is touched, but only with comparisons and `format!`, and the AC-8 scan passes in
  `make ci`.
- **Rust lane.** The `OracleShape` and `Helpers` types are `pub(crate)` and documented. The new
  error variants carry stable codes and terminal state `unsupported`. The generated `carry`
  helper's `_` arm maps a future `Outcome` variant to `Refused(CheckedInvariant)`, an error and
  not a value, as `#[non_exhaustive]` requires. No new `#[allow]` was added.
- **Python heredoc edit.** The `Pair` refactor of `tests/it/oracle_arithmetic.rs` is sound on its
  merits. `Pair::model` uses `i128` and calls nothing in the emitter or runtime, and the generated
  checks crate is `#![deny(warnings)]`.
- **Gates.** `make ci` exit 0 on a clean target: fmt, `make spec` (only the known FR-017 EARS
  warnings), clippy, msrv, deny, audit-unsafe, rustdoc and test. The two test runs show
  161 + 312 passed with 20 ignored.

Real Kani (Kani 0.68.0, CBMC 6.11.0, the `make kani` filters on the head):
`kani_exemplar_verifies_and_its_addition_mutant_is_falsified_at_999` and
`kani_bundle_oracle_overflow_is_a_failing_check_not_a_wrapped_value` pass, 2 of 2, in 111 s.

Mutations of the product code, run against those tests on a throwaway copy with its own target
directory:

- **KM1 (killed).** The bundle oracle renders `*` as `.wrapping_mul(...)`. The AC-20 test fails
  because the unconstrained harness now verifies (`VERIFICATION:- SUCCESSFUL`).
- **KM2 (survived, equivalent).** The generated harness drops the declared-domain `kani::assume`.
  The exemplar test still passes. For this clause the mutant is equivalent: the antecedent
  `amount < 1000` excludes the only operand at which `amount + 1` overflows (`i64::MAX`). The
  assume text is pinned by the existing non-Kani assertions at
  `tests/it/kani_generation.rs:529-541`.
- **Test-shape mutation (reasoned, not run).** Dropping the assume from AC-20's `domain_assumed`
  probe would make it identical to the `unconstrained` probe, which the run shows failing. So the
  assumed half of AC-20 depends on its assume.

## Dispositions

Round 1, reviewed at c2e897c3ce84e0bc115f3c339e1e0ff36a342daf (fix commit c2e897c on 57ad290).

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | c2e897c: `Renderer` now has a `shape` field. `infix_operator` refuses every shape other than `KaniBundle` with `UnsupportedExpression` before it prints an operator. The two renderer calls of `arithmetic_refusal` are gone, so the only call left is the one in the analysis (boolean_v1.rs:663), which runs with the real shape before any rendering. No path prints a raw infix operator for `Native` or `PlainBool`, including the `holds_arithmetic` fallback. Only error paths changed, and the emitted text did not (the AC-12 pinned bytes and AC-19 pass in `make ci`). |
| FND-002 | fixed | c2e897c: renamed to `tc_023_native_proven_division_is_refused_until_the_ir_601_ruling`, and TC-044 step 11 records the rename. |
| FND-003 | fixed | c2e897c: FR-031 Consumers and FR-008-AC-3 now state that bound oracle generation is all-or-nothing over a package, so a refused sibling clause refuses the strategy for every clause and the refusal names the refused clause. The new test `tc_044_a_refused_sibling_clause_refuses_the_strategy_of_every_clause_in_its_package` asserts `UnsupportedClause`, the sibling's `UnsupportedSaturatingArithmetic` code and the sibling's `ClauseRef`. Stating the behaviour instead of scoping per clause is acceptable: per-clause scoping would change `generate_bound_oracles`' atomic contract (NFR-001, interface-001), which this PR should not change. |
