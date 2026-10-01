---
id: "SR-647"
title: "CG PR 208 code review: scalar Kani harness asserts the native arithmetic relation"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@2bea27d5504126a0c32ab5d37fef02c39f9bf0f2; src/kani_obligations.rs, tests/it/kani_obligations.rs, tests/it/routed_generation.rs"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: references
---

# SR-647: CG PR 208 code review

## Summary

Ticket: IR-458. PR: agent-ix/quire-contract-codegen#208, head 2bea27d, branch
`fix/ir-458-scalar-harness-real-property`, base main 8fcb51f. Main is now 5a924e1 (#206 merged);
`git merge-tree` of 5a924e1 with 2bea27d is clean. This review covers code-review with the
rust-review lane folded in, scoped to `git diff origin/main...HEAD` (3 files, +103/-55). No spec
file changed, so no spec-review ran.

## Method

I read the diff and the code around it: `lower_scalar_claim`, `scalar_arity`,
`reachable_results` and `render_scalar` in `src/kani_obligations.rs`; `check_parameters`,
`Bounds::equal` and `SourceBuilder::oracle`/`body` in `src/exact_scalar.rs`; and the runtime's
`evaluate_integer_arithmetic` and `Integer: From<i128>` (quire-contract-runtime ccc722b). I checked
the IR `OverflowPolicy` enum (quire-contract-ir). I ran six source mutations in the default lane
and five harness-text mutations under real Kani (listed under Verdict). I ran `make ci` at 2bea27d
with a private scratchpad TRUSTED_HOME, and the real-Kani scalar tests.

Claims re-measured on main 8fcb51f: `src/kani_obligations.rs:2061` rendered
`Ok(rt::Outcome::Completed(value)) => domain.contains(value)`, and the runtime returns
`Refused(IntegerOutOfDomain)` when `!bound.contains(&result)` before `Completed`. So the old
assertion restated the oracle's own check. The ticket's premise holds.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Three of the four native expressions are pinned by no test, in either lane. `native_expression` renders `sub`, `mul` and `negate`, but only the `add` text is asserted (tc_025) and only `add` runs under Kani. Measured: swapping the operands of `sub` (`right - left`), rendering `mul` as `+`, or dropping the minus from `negate` each leaves `cargo test --test it -- tc_025_scalar_harness tc_033 tc_027 tc_025_symbolic` green (28 passed). Such a slip would make the first real Kani run of a sub, mul or negate claim falsify a correct oracle. tc_033 already renders sum and negation harnesses and checks their `admitted` line. It should also assert each harness's `let exact: i128 = ...` line, and a mul case is needed. | src/kani_obligations.rs:1983-1986, tests/it/routed_generation.rs:599-708, tests/it/kani_obligations.rs:778 |
| FND-002 | low | The mutation-control test's doc says the failing check "is the generated `sound` assertion". The test asserts only that the counterexample names the harness symbol and the exit code is nonzero, so any other failing property in the mutated crate would satisfy it. The real counterexample text carries `Check for \`assertion\`: ""the oracle must complete with exactly the native quire.op.integer.add result ...""`, so the test can assert it. | tests/it/kani_obligations.rs:1996, tests/it/kani_obligations.rs:2017 |
| FND-003 | low | No test pins the `_ => false` arm. Changing it to `_ => true` leaves the default lane green. The arm is load-bearing for an oracle that returns `Ok(Undefined)` or `Err(OracleStop)` on some inputs and `Completed` on others: with `true`, that oracle would verify. tc_025 should assert the arm's text, as it does for the other two arms. | src/kani_obligations.rs:2075 |
| FND-004 | low | One fact, four places. Adding a fifth operation means editing four string matches by hand: `scalar_arity`, `reachable_results`, `native_expression`, and the operand names in `render_scalar`. The compiler checks none of them. `native_expression` hard-codes `left_native`, `right_native` and `operand_native`, so it depends on the `names` slice at :2018-2021 without a shared source, and its catch-all `unreachable!` panics if the two drift. An operator enum resolved once from the identity, with arity, operand names and native expression as exhaustive methods, would remove the drift. Style note: there is no wrong behaviour today. | src/kani_obligations.rs:1339-1346, src/kani_obligations.rs:1480-1506, src/kani_obligations.rs:1981-1989, src/kani_obligations.rs:2018-2021 |

## Verdict

Mergeable once FND-001 is fixed. FND-002 to FND-004 are low and could be fixed in the same round.
The core change is correct and closes IR-458.

What is right:
- **Independence.** `exact` is plain `i128` arithmetic over the raw `i64` operands. It shares no
  arithmetic with `rt::Integer`. The only runtime code it touches is `Integer: From<i128>` and
  `PartialEq`, for the final equality.
- **No i128 overflow.** The largest product is `i64::MIN * i64::MIN = 2^126 < 2^127`. Negating
  `i64::MIN` gives `2^63`. Sums and differences stay within ±2^64. `reachable_results` already relies
  on the same widening.
- **Same bound, by construction, and a wrong embedded bound is caught.** `admitted` reads the result
  bound's literals. They come from the first `checked_bounds` id, which `check_parameters` gets from
  `Bounds::equal`. That call refuses with `BoundMismatch` unless the descriptor's `IntegerDomain` is
  `Bounded` and equal to that same IR bound. So the oracle's `Some(&domain)` and the harness's
  `admitted` always name one interval, and `Mathematical` never reaches this renderer. A bound
  wrong in the IR itself would pass both, but checking that is the spec's job, not the oracle's.
  The real-Kani probes show a drifted embedded oracle bound is now caught. Narrowing
  `integer("9")` to `5` and widening it to `10` were both Falsified, on the `sound` assertion.
  The old in-domain assertion verifies the narrowed case.
- **Overflow policies.** The V2 `ExactScalarOperation::IntegerArithmetic` carries only `operator`
  and `domain`. `evaluate_integer_arithmetic` only ever refuses out of the bound; it never
  saturates or wraps. IR's `OverflowPolicy {Reject, Saturate}` lives on the V1 `IntegerType` and
  cannot reach this path. So `Refused => !admitted` holds for every claim this renderer accepts.
  Division, modulo and comparisons are not routed here, because `scalar_arity` admits only
  add/sub/mul/negate.
- **`Incomplete` cannot occur.** The meter limits are all `u64::MAX`, so failing on it is right.
- **The tc_027 edit does not weaken the test.** It narrows the harness's own `admitted` upper bound
  from 9 to 5 while the oracle completes up to 9, the same mutation as before against the new text.
  It still requires `Falsified`. The two tc_033 edits only re-point the bound check at the new
  `admitted` line.
- **Doc comments** on `native_expression` and `render_scalar` describe what the code does. The
  stale "NOT discharged" paragraph is gone, correctly, now that the kani lane runs it.

Mutation probes. Each was applied to a temporary copy at 2bea27d and reverted; nothing was
committed or pushed.
- Default lane (`cargo test --test it -- tc_025_scalar_harness tc_033 tc_027 tc_025_symbolic`):
  - `Refused => !admitted` changed to `admitted`: red (tc_025).
  - `admitted &&` dropped from the `Completed` arm: red (tc_025).
  - `sub` operands swapped: green (FND-001).
  - `mul` rendered as `+`: green (FND-001).
  - `negate` sign dropped: green (FND-001).
  - `_ => false` changed to `true`: green (FND-003).
- Real Kani, on the routed `x + 1` harness:
  - `+ 1` added to the native expression: Falsified, on the `sound` assertion.
  - Embedded oracle bound 9 narrowed to 5: Falsified.
  - Embedded oracle bound 9 widened to 10: Falsified.
  - `Add` changed to `Subtract` with `assert!(sound, ..)` removed: Verified. The cover is still
    reached, so the `sound` assertion is the only check that catches the wrong arithmetic.
  - The PR's own control (`Add` changed to `Subtract`): Falsified.

Gates at 2bea27d:
- `make ci` with a private scratchpad TRUSTED_HOME exited 0. It ran fmt-check, spec (only the
  existing FR-014 and FR-017 EARS warnings), clippy `-D warnings`, MSRV 1.98.1 tests (94 unit and
  230 integration passed, 9 ignored), deny (all ok), audit-unsafe and rustdoc, then test (94 and
  230 passed, 9 ignored).
- Real Kani, which `make kani` runs with cargo +1.98.1, `--ignored` and `--test-threads=1`, ran
  on the PR head. The coder's log was at 6f386ab; I re-ran it at 2bea27d:
  - `tc_025_scalar_harness_falsifies_a_mutated_oracle_arithmetic`: ok.
  - `tc_027_a_routed_scalar_harness_verifies`: ok.
  - `tc_027_a_routed_scalar_harness_violating_its_bound_is_falsified`: ok.
  - All six reviewer probes ran in the same pass: 9 passed, 4035 s, under high machine load.

## Dispositions

Round 1, reviewed at a03a883. The branch is rebased on main 94ab14d, which is still main.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | a03a883: `tc_025_every_rendered_operation_states_its_own_native_relation` renders every generated corpus claim (add, sub, mul, negate). For each, it asserts exactly one `let exact: i128 = ` line and the exact line written out in the test. I re-ran the three mutations: sub operands swapped, mul as `+`, and negate sign dropped are each red, on this test. No real-Kani run for sub, mul or negate was added; the text pin is sufficient for this finding. |
| FND-002 | fixed | a03a883: the Kani mutation test now also asserts the counterexample contains "Check for `assertion`" and the `sound` assertion's message. |
| FND-003 | fixed | a03a883: tc_025 asserts `_ => false,`. I mutated it to `_ => true` and the test went red. The arm appears once per harness in the rendered text. |
| FND-004 | fixed | a03a883: one `ScalarOperation` enum with exhaustive `of`, `operand_names`, `native_expression` and `reachable`. The native expression is built from `operand_names`. Rendered output is byte-identical before (e46e823) and after for all four corpus harnesses (add, sub, mul, negate) and the routed `x + 1` harness. |

Round-1 checks at a03a883:
- Default-lane mutations, all red on the intended test: sub swapped, mul as `+`, negate sign
  dropped, `_ => true`, `Refused => admitted`, and `assert!(sound, ..)` removed.
- `make ci` with a private scratchpad TRUSTED_HOME exited 0: 101 unit and 239 integration tests
  passed, 9 ignored, in both the MSRV and the test pass.
- Real Kani: I re-ran the three scalar tests myself with `make kani` settings. All three passed in 908 s: `tc_025_scalar_harness_falsifies_a_mutated_oracle_arithmetic`, which includes the new assertion-message check, `tc_027_a_routed_scalar_harness_verifies` and `tc_027_a_routed_scalar_harness_violating_its_bound_is_falsified`.
