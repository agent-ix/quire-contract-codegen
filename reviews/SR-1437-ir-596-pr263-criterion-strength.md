---
id: "SR-1437"
title: "CG PR 263 criterion strength: can the FR-031 planned rows fail"
type: SpecReview
analysis: criterion-strength
review_set: subset
scope: "agent-ix/quire-contract-codegen@e27fe84435e45db9987a40c39f0b5549c28c5c09; spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md, spec/oracle/matrix/TC-044-boolean-oracle-integer-arithmetic.md; diff origin/main...HEAD, merge base 83f9687; constructions run with a scratch example against the current generator and quire-contract-model check_expression (removed after review)"
---

# SR-1437: CG PR 263 criterion strength: can the FR-031 planned rows fail

## Summary

Ticket: IR-596. Each of FR-031-AC-1..AC-11 was checked for whether a test can fail on today's code and whether TC-044 can satisfy it. The AC-5, AC-6 and AC-7 vectors were constructed through `DeclarationEnvironment::check_expression` and passed to `generate_boolean_oracle` on the current tree. Every vector is reachable under `reject`, which the TC calls uncertain:

- O-1 `i64::MAX + 1`: type `reject -10..=0`, clause `x >= -5 && (y >= -5 && x + y <= 0)`, called with x=MAX, y=1. Admitted, and today's oracle emits raw `+`.
- O-2 `i64::MIN - 1` as a real Subtract: type `reject -10..=0`, clause `x <= -5 && (y >= -5 && x - y <= 0)`, called with x=MIN, y=1. Admitted.
- O-3 and O-5: type `-5..=5`, with `y >= -1 && y <= -1` guarding `x / y` and `x % y`. O-4: type `0..=10`, clause `x < 5 && x * 2 <= 10`. All admitted.
- Z-1, from the TC's shape: admitted.
- Z-2 `0 / 0`: type `1..=10`, clause `y <= 1 && x / y <= 10`, called with x=0, y=0. Admitted.
- Z-3 and Z-4, remainder by zero: type `reject -10..=11`, clause `y <= 0 && x % (y + 11) <= 10`, called with y=-11, so the compound divisor is 0. Admitted.
- `saturate` rows: `MAX + 1` over full i64, `MIN / -1` with a `y` guard, and `x / y` over `1..=10`. All admitted and emitted raw today.

AC-1, AC-2, AC-3, AC-7, AC-9 and AC-10 fail on today's code: either raw operators are emitted, or no refusal is given. AC-4's native half is declared a held agreement, which is honest.

## Verdict

AC-8 cannot detect a wrong rendering, and the "nearest reachable vector" escape hatch weakens AC-5 and AC-6 although no substitution is needed. Both are fixable in TC-044 and FR-031 text.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-8's reference is "the direct runtime evaluation of the same expression". If built from the same `exact` calls, it agrees with any rendering by construction; it would pass the SR-1435 FND-001 pair refusal. The reference must be independent of the emitter (mathematical result, truncating law, membership of the selected result in the type interval), and the grid must include in-domain divide/remainder vectors whose unselected member leaves the interval | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:217 |
| FND-002 | medium | FR-031 "Test obligations" and TC-044 steps 4-5 allow a "nearest reachable vector" substitution, and say O-1 and the remainder zero-divisor rows may have no reject construction. Every AC-5/AC-6 vector is admitted by `check_expression` (constructions in Summary), so the hatch only weakens the literal vectors the ACs name. The TC's O-2 shape `x + y` with y=-1 is an Add, not the Subtract its table names, and its Z-1/Z-2 shared shape never evaluates `0 / 0`, because the `x >= 10` guard short-circuits at x=0 | spec/oracle/matrix/TC-044-boolean-oracle-integer-arithmetic.md:60-72 |
| FND-003 | low | AC-7's before-change clause says each generated function "panics on the named operands" without AC-5's build qualifier. Under release overflow-checks the `saturate` `MAX + 1` and `MIN - 1` rows wrap rather than panic; TC-044 step 6 says debug build, and the AC should too | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:216 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | low | AC-7's multiply row (`x * 2` over `saturate 0..=10`) and TC-044 step 11's `saturate` divide and remainder rows name no operand vector, yet AC-7 and AC-18 require the before-change function to panic in a debug build "on the named operands" or "on the vector of its row". `x * 2` panics only near the i64 extremes, not in-domain, where it is wrong without panicking (x=10 gives `20 <= 10`). Name a vector, such as `x = i64::MAX`, or assert the in-domain wrong value instead | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:264 |

## Dispositions

Round 1 was reviewed at 8435fbeb50f2b817315902736ccdbff921e26a6b.

- **FND-001.** AC-8 now compares against an i128 member-range model computed in the test, with no `exact::` call and nothing read from the emitter. The divide and remainder differential is held as AC-17, and its grid contains the two pair-versus-member reproductions.
- **FND-002.** The nearest-vector hatch is gone ("No case is replaced by a nearby vector"). TC-044 step 4 and step 5 carry concrete constructions, including O-1, a real Subtract O-2 (`x - y`), Z-2 with the `y <= 1` guard, and the compound-divisor Z-3/Z-4.
  - Re-run through `check_expression` and `generate_boolean_oracle` on the PR head (oracle code identical on current main): O-1, O-2, O-3, O-4, O-5, Z-1/Z-2 and Z-3/Z-4 are all admitted, and all are emitted with raw `+ - * / %` today.
  - The `saturate` rows `x + 1`, `x - 1` (full i64), `x * 2` over `0..=10`, and divide/remainder over `1..=10` are all admitted and emitted raw.
- **FND-003.** AC-7 now says "panics in a debug build".
- **Split ACs.** AC-12..AC-18, `UnsupportedIntegerDivision`, `UnsupportedSaturatingArithmetic` and IR-601 appear in git history only in 8435fbe. Each new AC can fail on today's tree, except AC-4 and AC-12, which are declared held agreements.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 8435fbe |
| FND-002 | fixed | 8435fbe |
| FND-003 | fixed | 8435fbe |
| FND-004 | still-open | new this round; AC-7 multiply row and step 11 saturate divide/remainder rows name no vector |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | low | FR-031-AC-20 and TC-044 step 12 require Kani's counterexample to be at `x = i64::MIN`. Kani returns any satisfying assignment: this run gave i64::MIN, but any `x < -2^62` overflows and is equally valid, so the assertion is solver-dependent. AC-20 also reads both "unconstrained operands" and "called at x = i64::MIN". Assert the failing "attempt to multiply with overflow" check, and that the playback value satisfies `x < 5` and overflows `x * 2` | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:351 |
| FND-006 | low | The existing guarded-division case in tests/it/bound_strategy_generation.rs (`generate_bound_oracles` expected `Generated`, then strategy `UnsupportedRelation`) flips to an oracle refusal and `UnsupportedClause` under FR-031-AC-18 and the amended FR-008-AC-3. TC-044 names only tc_023 and tc_003 for rewriting; name this TC-017 case too | spec/oracle/matrix/TC-044-boolean-oracle-integer-arithmetic.md:143-144 |

## Dispositions (round 2)

Round 2 was reviewed at 5c83391072d085766dd5e1c7bba71c3b1b100785. For FND-004, AC-7 now names `x = i64::MAX` for `x + 1` and `x * 2`, and `x = i64::MIN` for `x - 1`, and TC-044 step 11 adds S-1..S-4 with vectors. Re-run on the head (scratch crate r2-kani-review/admit): every row is admitted by `check_expression` and emitted raw today, and in a debug build every named vector panics (add/subtract/multiply overflow, divide by zero, divide overflow, remainder overflow).

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-004 | fixed | 5c83391 |
| FND-005 | still-open | new this round; AC-20 pins a solver-chosen counterexample value |
| FND-006 | still-open | new this round; guarded-division bound-strategy test not named for rewrite |

## Dispositions (round 3)

Round 3 was reviewed at 4399a5784c1ddf492647c0ecdb9b27e80383bb99.

- **FND-005.** AC-20 and TC-044 step 12 now require the failing "attempt to multiply with overflow" check, plus a playback value that satisfies `x < 5` and overflows `x * 2`, and they name no value. The stated range "any x below -2^62" is exact: -2^62 * 2 = i64::MIN does not overflow.
  - Re-run under Kani 0.68 / CBMC 6.11 (unwind 4, cadical, scratch crate r2-kani-review/bundle).
  - Unconstrained: 1 of 7 failed, failed check "attempt to multiply with overflow", playback -9223372036854775808, which satisfies both conditions.
  - Domain-assumed: 0 of 7 failed.
- **FND-006.** TC-044 step 11 names three rewrites. All exist and are the right tests:
  - `tc_023_native_proven_numeric_obligations_render_without_assumptions` (tests/it/oracle_generation.rs) asserts a `/` for a guarded division.
  - `tc_003_unsupported_expression_and_root_map_to_declared_terminal_states` (same file) puts its locus on the negation after a saturate add.
  - The guarded-division case is inside `tc_017_bound_admission_uses_the_public_clause_and_domain` (tests/it/bound_strategy_generation.rs, traced TC-017 / FR-008-AC-3). It expects `generate_bound_oracles` to generate and the strategy to return `UnsupportedRelation`. After the change, `map_oracle_error` yields `UnsupportedClause` carrying the oracle code, as step 11 says.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-005 | fixed | 4399a57 |
| FND-006 | fixed | 4399a57 |
