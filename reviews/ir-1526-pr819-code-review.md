---
id: SR-17940486709564847047937
title: "IR-1526 PR #819 wide multiply probe and measured table code review"
type: SpecReview
analysis: code-review
date: 2026-11-13
scope: "agent-ix/quire-contract-codegen; PR #819 @ f261f105; IR-1526; tests/fixtures/tractability/corpus.json, tests/fixtures/tractability/table.json, tests/it/kani_obligations_v2_clause_controls.rs, tests/it/kani_tractability.rs, spec/kani/functional/FR-056-kani-tractability-refusal.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-056
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-070
    type: references
---

## Summary

Ticket: IR-1526. This is the code review of PR #819, with the rust-review lane folded in. It is scoped
to `git diff origin/main...HEAD`: 5 files, +288/-10, all Rust changes in tests. The PR adds an
ignored installed-Kani probe, `ir1526_probe_wide_integer_multiply_under_installed_kani`, for the
clause `other * offset + 3 <= 100` on the general route. It also adds three `timed_out` corpus rows
from that probe at 16, 24 and 32-bit operands, and a CG-own `table.json` with one `not_tractable`
row for `quire.op.integer.mul` at 15 bits. The scoring test now scores that table, and the PR
rewrites the FR-056 AC-10 Status.

Focused run: `cargo test --test it -- tc_070 wide_multiply_probe` through `locked-build.sh` with a
throwaway target dir that was deleted afterwards. Result: 19 passed, 1 ignored. No Kani run and no
full gate. The measurement logs (kani-probe-16/24/32.log) show `outcome=Inconclusive { reason:
TimedOut }` at 600.0, 600.1 and 600.1 s, with `runner_peak_resident=None` and `checks=0`.

## Verdict

**NOT MERGEABLE as is.** One finding is high and one is medium. Both are small to fix.

What holds up:

- The corpus rows are honest measured ground truth for what was run. The probe goes through the
  public production entry point `negotiate_kani_obligations_with_options(.., IntegerRoute::General)`.
  The harness is the one `render_clause` emits for every V2 precondition (src/kani/generate/clause/v2.rs:1091-1290).
  On the general route that means `rt::Integer::from(..)` inputs, a meter, a call to `oracle_clause`,
  forgets, and a single `kani::cover!(matches!(outcome, CompletedTrue))`. A precondition harness is
  cover-only by design (classify.rs:164, execute.rs:1726), so "cover-only, no subject" is the
  production shape, not a test-only shortcut.
- The features match what the decider computes. The new test `tc_070_the_probe_corpus_rows_carry_the_decider_features`
  rebuilds them through `derive_v2_clause_operation_widths` and the oracle descriptor.
- The wall time of 600 s and the source string are accurate. The source names the test, the route,
  Kani 0.68.0 and CBMC 6.11.0. Generator 0.1.0 matches Cargo.toml and kernel 1 matches `quire_exact::KERNEL_VERSION`.
- No row is copied from another lane or repository. The probe stays `#[ignore]`d, and its doc comment
  holds the results.
- `max_operand_bits = 15` is coherent with FR-056 Terms. Width is the operand interval's bit width,
  not the result's, and `Int[0, 2^15 - 1]` is 15 unsigned bits. The add and le widths of 30, 46 and 62
  are the propagated mul result. The inclusive `<=` lookup refuses all three rows, which is consistent with AC-5.
- The scoring changes are sound. The table decider scores 0/0 over the 13 rows. AlwaysRun gives
  false-accept 3 and AlwaysRefuse gives false-refuse 10, and both now regress against baseline {0,0}.
  AC-10 stays PARTIAL.

What is wrong is covered by the findings below. The table row's `peak_bytes` is an invented number,
and the Status claims the measurements justify a mul row when the corpus cannot tell mul from add,
le or the general-route carrier.

## Findings

| ID      | Severity | Summary | Refs |
| ------- | -------- | ------- | ---- |
| FND-001 | high     | table.json `peak_bytes: 25769803776` is the 24 GiB lane cap, not a measurement. All three runs logged `runner_peak_resident=None`. The field means "Peak memory of the probe in bytes" (src/kani/tractability.rs:160) and is reported as evidence in every `V2ClauseNotTractable` refusal (AC-4 "the evidence row (its full fields)"), so a refusal would assert a 24 GiB peak that nobody measured. The disclosure sentence in FR-056 Status does not travel with the row. Fix: make `peak_bytes: Option<u64>` with `#[serde(deserialize_with = "required")]` (the same pattern as `collection_bound`, so AC-3's missing-key check still holds), write `null` in table.json, and amend the Measured table text to "`peak_bytes` (null when the runner reported no peak)". Then drop the Status sentence. | tests/fixtures/tractability/table.json:10, src/kani/tractability.rs:160-161, spec/kani/functional/FR-056-kani-tractability-refusal.md:86,144 |
| FND-002 | medium   | The Status says the measured table "holds the one not_tractable row those measurements justify, integer multiply from 15 bits". The measurements do not attribute the timeout to mul. (a) Every probe row also holds add and le at 30 or more bits, and an add or le row at any `max_operand_bits` from 11 to 30 also scores 0/0 on the 13-row corpus, so the corpus cannot tell mul from add or le. (b) All three runs use the general-route `rt::Integer` carrier, which docs/qualification/ir1530-general-route-scalar-not-tractable.md measured as not tractable even for add at 4 bits. The ten other rows are native-route (IR-1022). The decider's features hold no route, so the mul row would refuse native-route mul clauses at 15 or more bits with no native measurement, and it would let a general-route add-only clause through that likely times out. The "no false-refuse and no false-accept" score is therefore not evidence for this row. Fix: restate it as "general-route V2 precondition holding mul at 15 or more bits timed out; the corpus cannot attribute the cost to mul versus the general carrier, and features carry no route". Either keep the row as a provisional fixture with that caveat, or add one control probe (`other + offset <= 100` general at 16 bits, or the AC-13 narrow mul at 10 bits) before claiming mul. Name the missing route feature under AC-10's open items. | spec/kani/functional/FR-056-kani-tractability-refusal.md:136-139, tests/fixtures/tractability/table.json:3-5, tests/fixtures/tractability/corpus.json:372-469 |
| FND-003 | medium   | table.json pins `generator_version: "0.1.0"` and the corpus features default to CG's current `CARGO_PKG_VERSION` (src/kani/tractability.rs:66). The next crate version bump makes the row stale under AC-7. The table decider then runs everything, false-accept goes to 3 against baseline 0, and `tc_070_the_current_decider_scores_no_worse_than_the_baseline` turns red with no behaviour change. The only ways back to green are a 3x600 s Kani re-measurement or hand-editing the version into the row, and hand-editing would invent provenance. Fix: state this in the test comment and Status so a release owner knows the expected red, or score the corpus with features at the row's own generator version, which is honest because the corpus rows were measured at that version. | tests/fixtures/tractability/table.json:14, tests/it/kani_tractability.rs:759-773 |
| FND-004 | low      | The width wording mixes two conventions. The corpus ids, the Status ("16, 24 and 32-bit operands") and `IR1526_PROBE_WIDTHS` count signed bits with the sign included. The table row and FR-056 Terms count 15, 23 and 31 unsigned bits. Readers have already confused 15 with the "mul result width". Saying "operand bound 2^15 - 1 (width 15 per Terms)" in the Status and in the probe doc comment removes the ambiguity. | spec/kani/functional/FR-056-kani-tractability-refusal.md:136-139, tests/it/kani_obligations_v2_clause_controls.rs:1558-1560 |

## New findings (disposition pass 1)

| ID      | Severity | Summary | Refs |
| ------- | -------- | ------- | ---- |
| FND-005 | medium   | The add-control corpus row `ir1526-wide-add-15bit-operands` is labelled ground truth `timed_out` with `wall_seconds` 542.5, below the 600 s ceiling, so the row contradicts itself. The run did not time out: the kernel log for that run shows `Memory cgroup out of memory: Killed process 17052 (cbmc) ... anon-rss:25055932kB` at 02:01:17, under the 24 GiB scope. That is the probe's own scope, started 01:52:17, 540 s earlier. The measurement_source says "resource exhaustion suspected", but this is measured and can be stated as fact. AC-10 requires MEASURED ground truth, and `timed_out` is a different measured fact (the wall-clock ceiling). Fix: add a ground-truth value for the memory ceiling (for example `memory_exhausted`) to FR-056-AC-10 and to the corpus `GroundTruth` enum, scored as a false-accept exactly like `timed_out`. Then relabel the row and cite the OOM-kill in its source. Optionally, the add table row may record `peak_bytes` from the kill (25055932 kB anon-rss), which is a real measurement. Dropping the row is the fallback, but it loses the evidence that backs the add table row. | tests/fixtures/tractability/corpus.json:477-506, spec/kani/functional/FR-056-kani-tractability-refusal.md:134-141 |

## Dispositions

| FND | outcome | reason |
| --- | ------- | ------ |
| FND-001 | fixed | 56d6f270: `peak_bytes: Option<u64>` with `deserialize_with = "required"`; table rows carry `null`; Measured table text says null means no peak reported; a host test loads a null peak; the missing-key check still fails the load. |
| FND-002 | fixed | 28053a1b: the add control run shows an add-only general clause also fails, and the Status now says the measurements do not show multiply is the cause, that all runs used the general-route rt::Integer carrier, and that the route is not a feature, so native-route false-refuse is unmeasured. Table holds mul and add rows at 15 bits, disclosed as such. |
| FND-003 | fixed | 56d6f270: probe corpus rows record `generator_version` 0.1.0 and `kernel_version` 1 in their features, the versions they were measured at, so the table rows stay current against them after a crate bump; the IR-1022 rows default to the current version and are verified/falsified, so a stale table cannot false-refuse them. The feature test compares at the row's recorded versions. |
| FND-004 | fixed | 56d6f270: probe widths, row ids, Status and doc comment all use unsigned bits per FR-056 Terms (15/23/31, `Int[0, 2^n - 1]`); operand values unchanged; each measurement_source records the earlier 16/24/32 label. |
