---
id: SR-17940486709564847047958
title: "IR-1526 PR #824 native-route Kani probe and corpus rows code review"
type: SpecReview
analysis: code-review
date: 2026-11-13
scope: "agent-ix/quire-contract-codegen; PR #824 @ 6c3387de; IR-1526; spec/kani/functional/FR-056-kani-tractability-refusal.md, tests/fixtures/tractability/corpus.json, tests/it/kani_obligations_v2_clause_controls.rs, tests/it/kani_tractability.rs, tests/state_frame_support/native_twin.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-056
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-070
    type: references
---

## Summary

Ticket: IR-1526. Code review of PR #824 with the rust-review lane folded in. Scope is
`git diff origin/main...HEAD`: 5 files, +771/-16, all tests, fixtures and spec prose, with no
production source. The PR adds an ignored native-route Kani probe and twelve measured native
corpus rows: integer add, multiply and multiply-then-add, each at 10, 15, 23 and 31 operand
bits. It also adds a host test that recomputes each row's decider features, and it updates
the FR-056 Status.

## Verdict

**FAIL**, because FND-001 is `high`: the code-review rule records any duplication inside one
repo at `high`. The fix is mechanical. The rest of the PR is sound:

- **The twelve rows match the logs.** Each row was checked against `native-probes.summary` and
  `native-probe-<op>-<bits>.log`: operation ids, widths, input bounds, `wall_seconds`, the
  `verified` verdict, the `native` route, the measurement-source text, and generator `0.1.0` /
  kernel `1`. Every row matched. Each log shows Kani 0.68.0 and CBMC 6.11.0.
- **The feature test can fail.** It compares every row's features with the decider's own
  computation by full equality. It recomputes the features through
  `derive_v2_clause_operation_widths`, which wraps the same `clause_operation_widths` the
  production decider uses (`refuse_if_not_tractable`), and through the native oracle's
  `descriptor.inputs`. A wrong width, bound, operation or route makes the assertion fail. A
  wrong `wall_seconds` would not, because only `is_some` is checked. This review checked the
  values against the logs instead.
- **The 0/0 baseline is right.** The shipped table holds only `general` rows, so all twelve
  native rows and the ten IR-1022 rows get `Run`, and none of those rows has an unfinished
  ground truth. The four general-route rows are refused at 15 bits or more. On the shipped
  26 rows, an always-Refuse decider would score 22 false refuses and an always-Run decider
  4 false accepts, so both still regress. The test asserts these two mutants on its
  synthetic corpus.
- **No rows were copied from titan:IR:proofs.** No other checkout holds `ir1526-native` rows,
  and each row's source is this PR's probe log.
- **The new helper is sound.** `Twin::with_bounded_amount_result_postcondition` reuses the
  existing Boolean-result package and rewrites only the `max` constraint of the `balance` range
  type. The recomputed inputs (`upper = 2^n - 1`) confirm that the rewrite reaches `amount`.
  Its `expect`s are test-fixture panics on a malformed twin.
- **The subject cannot overflow.** `check(amount: i64)` with `amount <= 2^31 - 1` makes
  `amount * amount + 3` at most about 2^62.

On the harness-shape question, the recommendation is to **keep** both shapes in one corpus,
labelled as they are now, and to defer a shape feature. The reasons:

- The native and general rows never share a feature vector. Route, inputs (one operand plus a
  Boolean result, against two operands) and operations (`boolean.eq`, the `le` widths) all
  differ, so no row contradicts another.
- The native route renders arithmetic only as a postcondition, so on that route the shape is
  constant and a shape feature would carry no information.
- The confound is interpretive, and FND-002 covers it. The contrast between the native and
  general rows changes carrier, harness shape, operand count and subject all at once, so it
  does not isolate the carrier.

A shape feature becomes worth adding when one route renders the same arithmetic clause in both
shapes. The general route does: a general-route postcondition probe at 15 bits or more is the
measurement that would isolate the carrier. That is follow-up work, not this PR.

Gates: `cargo fmt --check` is clean. The focused host tests are in Coverage below. Clippy,
the full suite and Kani were not run, as the brief directed.

## Findings

| ID      | Severity | Summary | Refs |
| ------- | -------- | ------- | ---- |
| FND-001 | high     | The native probe clauses and widths are defined twice in one test crate. They are `NativeOp::condition` plus `IR1526_NATIVE_WIDTHS` plus `native_probe_clause` in kani_obligations_v2_clause_controls.rs, and `NATIVE_PROBE_CLAUSES` plus the literal `[10, 15, 23, 31]` plus `native_probe_features`' own `Twin::with_bounded_amount_result_postcondition(...).emitted_package("BalanceNeverDrops")` in kani_tractability.rs. Both files are modules of the one `it` binary. Drift scenario: change the measured clause to one with the same features (for example `amount + 5 <= 100`) and re-measure. The feature test still recomputes the old string, still passes, and binds the new measurement to a clause it never ran. Fix: make `NativeOp`, `IR1526_NATIVE_WIDTHS` and `native_probe_clause` `pub(crate)` and use them from kani_tractability.rs (searched both files and tests/it for an existing shared probe definition; none exists). | tests/it/kani_obligations_v2_clause_controls.rs:1718, tests/it/kani_obligations_v2_clause_controls.rs:1752-1758, tests/it/kani_obligations_v2_clause_controls.rs:1774-1783, tests/it/kani_tractability.rs:1319-1323, tests/it/kani_tractability.rs:1346-1350, tests/it/kani_tractability.rs:1360 |
| FND-002 | medium   | The FR-056 Status draws a causal link the evidence does not support. It says the native harness shape differs and that the general-route rows "therefore do not over-refuse native clauses". They do not refuse native clauses because lookup is route-keyed (AC-14), whatever was measured, so the `Decision::Run` assertion holds by construction. The substantive AC-10 result goes unstated: a route-blind lookup would refuse nine native rows (add, mul and chain at 15, 23 and 31 bits) that verified. The Status also does not say that this contrast changes carrier, harness shape (postcondition against a subject, not a cover-only precondition), operand count (`amount * amount`, not `other * offset`) and subject together, so it bounds the carrier's cost but does not isolate it. Reword it, and optionally assert in the test that the same features with `route: general` are refused at 15 bits or more and run at 10. | spec/kani/functional/FR-056-kani-tractability-refusal.md:179-185, tests/it/kani_tractability.rs:1370-1374 |
| FND-003 | medium   | The ignored probe runs in the Kani merge gate. `make kani` and `make kani-gate` run `cargo test --test it -- --ignored ... kani_obligations ...`. libtest filters match substrings, so `kani_obligations` selects `kani_obligations_v2_clause_controls::ir1526_probe_native_route_under_installed_kani`, which runs at the default `add`/10 bits, about 85 s of Kani. The probe asserts no verdict, only that negotiation reaches `Supported`, so in the gate it costs time and checks nothing. The sibling `ir1526_probe_wide_integer_multiply_under_installed_kani` (PR #819) has the same exposure and defaults to a 600 s timeout. Keep the probes out of the gate's filter: rename them out of the `kani_obligations` prefix, move them into a module the filter does not match, or make them assert a verdict. | Makefile:111-113, tests/it/kani_obligations_v2_clause_controls.rs:1873-1875 |
| FND-004 | low      | A doc comment contradicts FR-056 Terms and the corpus. It says "The native carrier is signed, so FR-056 counts the operand as `n + 1` bits", but FR-056 counts `Int[0, 2^n - 1]` as `n` unsigned bits. The `n + 1` rule is for `Int[0, 2^n]`. The rows confirm this: add at n = 10 has `bits: 10`. | tests/it/kani_obligations_v2_clause_controls.rs:1716-1717 |
| FND-005 | low      | The probe's doc comment names the clauses `other + 3 <= 100`, `other * other <= 100` and `other * other + 3 <= 100`. The probe runs `amount + 3 <= 100`, `amount * amount <= 100` and `amount * amount + 3 <= 100`. | tests/it/kani_obligations_v2_clause_controls.rs:1870-1872 |
| FND-006 | low      | `the_native_probe_clauses_print_their_decider_features` is scaffolding for writing rows: `println!` output plus a single `!widths.is_empty()` assertion, which no realistic change can fail. `tc_070_the_native_probe_rows_carry_the_decider_features_and_are_not_refused` already checks the same features by equality. Delete it, or fold the print into the real test. | tests/it/kani_obligations_v2_clause_controls.rs:1847-1866 |

## Coverage

- Files examined: every file in the diff (listed in `scope`).
- Rust lane: rust-style API, implementation and testing checks applied to the test code. No
  public API, unsafe code, async code or wire format changed.
- Duplication and vendoring: FND-001 is duplication inside one repo. No copy from another repo
  was found.
- Ceremony and file tracking: none was added. `generator_version` and `kernel_version` on the
  rows are decider staleness inputs (FR-056-AC-7), not provenance.
- Focused host tests: `cargo test --test it -- kani_tractability layout
  kani_obligations_v2_clause_controls` through `locked-build.sh` with a throwaway target dir:
  47 passed, 0 failed, 14 ignored. FND-003 was confirmed with
  `cargo test --test it -- --ignored --list kani_obligations`, which lists both
  `ir1526_probe_*` tests.
