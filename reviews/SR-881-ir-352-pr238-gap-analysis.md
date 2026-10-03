---
id: "SR-881"
title: "CG PR 238 gap analysis: FR-021-AC-19 to AC-21 against the merged spec"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@5cb264db4df4ba16290d19236369e530d5debb62; spec/oracle/functional/FR-021-function-application-oracles.md (AC-19, AC-20, AC-21, Behavior bullets), spec/oracle/matrix/TC-031-function-application-oracles.md, spec/oracle/matrix/tests.md, spec/assurance/AD-004-cg-crate-layout.md, src/oracle/function/mod.rs, tests/it/exact_function_generation.rs, tests/it/exact_scalar_generation.rs (diff origin/main...HEAD, base 80f4785)"
---

# SR-881: CG PR 238 gap analysis

## Summary

Ticket: IR-352. PR: agent-ix/quire-contract-codegen#238 at 5cb264d. I checked the spec as merged
in #237 (squash 80f4785). Plan completion: not assessed.

Each criterion against the code and tests:

- FR-021-AC-19: emitted source has zero `.unwrap(`, `.expect(` and panicking macros in the main
  and chain corpora. Code: met. I dumped and grepped both corpora myself. Test:
  `tc_031_ac19_emitted_function_oracle_source_has_no_panicking_path` scans `src/lib.rs` of
  `main_oracles()` and `chain_oracles()`. It checks the four macro names with a matcher that
  catches any delimiter or path prefix, and it also asserts `TypeEnvironment::default()`. The
  main corpus contains a scalar, an equality and a call body, so the scan covers all three
  templates. Mutants that put back the `unreachable!` catch-all or the `.expect` failed the test.
- FR-021-AC-20: the ordered arms and the final catch-all. Code: met in both templates. Test:
  `tc_031_ac20_unknown_outcome_variant_refuses_checked_invariant` checks, for `add_fn` and
  `eq_fn`, all six arms in order, including the Integer or Boolean rewrap. It also checks that
  nothing but `}` follows the catch-all. Mutants that dropped the catch-all, reordered it, or
  changed its value all failed the test.
- FR-021-AC-21, refusal and absence: `tc_031_ac21_unsupported_operator_refuses_unary_negate_scalar_body`
  is renamed from `tc_031_unsupported_operator_...`. It keeps the `UnsupportedOperator`
  assertion and adds that `negate_fn` is absent from `src/lib.rs` and that no
  `FunctionDeclaration` is emitted. A mutant that removed the refusal failed it.
- FR-021-AC-21, zero macros in the generator source: `oracle_generators_have_no_unexcused_panicking_arms`
  scans `src/oracle/function/mod.rs` with no excusals, and comment lines are skipped. A mutant
  that added a defensive `Negate => unreachable!` arm failed it. I grepped the file myself: zero
  invocations.
- FR-021 Behavior bullets. The bullets on emitted source, `checked_package()` via `default()`,
  the forwarding `match`, the unknown-variant refusal, Negate refused and omitted, and zero
  macros in the generator file are all met. No new outcome or refusal type was added.
- Matrix. In `tests.md`, the row `FR-021-AC-19 through FR-021-AC-21` flips to Covered. Each AC
  has a passing test that I saw fail under mutation, so the flip is justified. In the TC-031
  file, the Description, step 8 and Expected Results lose their Planned notes, and the AC-18
  note stays. The FR-021-AC-15 and AC-18 rows stay Planned. The TC-031 summary row is unchanged.
- AD-004. Not touched by the diff. The merged sentence (line 1243) already reads "converted by
  #232; the remaining arms are tracked by IR-352", so it is still correct after this PR.
- Gate. `cg-352-ci.log` ends `head=5cb264db4df4ba16290d19236369e530d5debb62 exit=0`. It covers
  fmt-check, `quire validate` (spec), clippy `-D warnings`, the MSRV and stable test suites
  (249 passed each, including the four tests above and `tc_031_generated_function_crate_agrees_with_direct_runtime`,
  which compiles the regenerated crate), deny, audit-unsafe and rustdoc. Kani was not run, and
  the diff touches no Kani path. I reran the focused `exact_function_generation` tests and the
  guard test myself: 22 passed (21 plus a temporary dump test, since reverted).
- PR hygiene. The title has no bare ticket id. The body says "Part of IR-352", which is correct:
  FR-018's emitted `.expect` calls and the two equality generator-side `unreachable!` arms
  remain open. The body has no short SHA, there is no compatibility layer, and no pin, SHA,
  lockfile or vendored file changed.

## Verdict

FR-021-AC-19, AC-20 and AC-21 are implemented and backed by tests that can fail. The matrix
flips are true. One low trace finding.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | `oracle_generators_have_no_unexcused_panicking_arms` is re-tagged `FR-021-AC-19`, but it scans the generator's own source files, not the emitted `src/lib.rs` that AC-19 is about. It cannot see an emitted `.expect(` or `.unwrap(` (the `.expect` mutant in `checked_package()` passed it). It backs AC-21's source half only. AC-19 is backed by `tc_031_ac19_...`, so the overclaim inflates the trace without leaving AC-19 uncovered | tests/it/exact_scalar_generation.rs:2111 |

## Dispositions

Round 1, reviewed at e44365d0d552844ca2aeb5bca9ab43e705638b3c, base 80f4785 (current). The guard
test is re-tagged FR-021-AC-21 only. TC-031 step 8 already assigns the emitted-source count to
AC-19 and the generator-source count to AC-21, so the new tag matches the matrix. AC-19's
binding is now `tc_031_ac19_...` alone, and that test still fails when an `.expect` is put back in
`checked_package()` (probe rerun at e44365d).

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | e44365d |
