---
id: SR-17940486709564847047955
title: "IR-1530 PR #821 code-review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-contract-codegen PR #821 at c3a58c73, git diff origin/main...HEAD; IR-1530; src/kani/terminal/tractable.rs, src/kani/terminal.rs, src/kani/tractability.rs, src/lib.rs, tests/it/kani_tractability.rs, tests/it/kani_obligations.rs, docs/qualification/ir1530-general-route-scalar-table.json"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-056
    type: reviews
---

## Summary

Ticket: IR-1530. Code-review with the rust-review lane folded in. The change adds a run-level
pre-launch refusal (`execute_kani_obligation_decided`, `execute_kani_obligations_decided`,
`refuse_if_not_tractable`) that builds `ClauseFeatures` on a new `TractabilityRoute::GeneralScalar`
route from a routed general-integer scalar harness identity, asks the existing
`TractabilityDecider`, and refuses with the existing `UnsupportedObligation::V2ClauseNotTractable`
arm. A `route` field (serde default `v2_clause`, skipped when default) was added to
`TractabilityRow` and `ClauseFeatures`, and row currency now also requires route equality.

Checked clean: serde backward compatibility (existing corpus rows and features carry no `route`
key and deserialize to `v2_clause`; serialization of default-route values is byte-identical, so
FR-056-AC-6 output is unchanged; `deny_unknown_fields` still holds); width derivation
(`interval_bits(0, 9) == 4`, max over argument bounds, equal to the row's `max_operand_bits`);
native-route and non-scalar harnesses short-circuit to `Ok` without consulting the decider;
staleness (the shipped row's `generator_version` 0.1.0 and `kernel_version` 1 are asserted equal to
`GENERATOR_VERSION` / `KERNEL_VERSION` by the test, so a crate or kernel bump fails the test rather
than silently disarming the row); layout dependency direction (`kani::terminal` rank 6 imports
`run` 5, `generate` 3, `identity` 2, unranked `tractability`, and `oracle`; `run` still may not
import `terminal` or `tractability`). Focused tests `layout::` and `kani_tractability::` pass
(see Verdict). The `--lib` failure of
`production_refinement_live_solver_handoff_reaches_qsl_checker` is in `src/routed/smt/`, untouched
by this diff, and only needs `QUIRE_CVC5_BIN`: unrelated.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | medium | Nothing in production calls the decided wrappers. In this crate only tests reach `execute_kani_obligation(s)_decided`; the one downstream consumer, quire-driver `src/provider.rs:308`, calls `execute_kani_obligation` directly on harnesses from `generate_routed` (the general route), and the shipped row lives in `docs/qualification/` with no exported path or embedded copy, so a caller cannot load it without a repo checkout. The production path therefore still launches the general-route harness and ends `Inconclusive{TimedOut}` at 600 s. This is the same opt-in shape as the IR-1526 V2 seam (`generate_routed` / `negotiate_kani_obligations` pass `TractabilityTable::empty()`), so the CG library API meets the planner's ruling (reuse the seam, a test shows the refusal); production enforcement needs a quire-driver change (adopt the decided wrapper with the table) tracked as its own ticket. | src/kani/terminal/tractable.rs:107-129, src/kani/generate/negotiate.rs:330, quire-driver src/provider.rs:308 |
| FND-002 | low | `tc_070_the_runner_does_not_consult_the_decider` was narrowed from any `tractab` mention to `TractabilityDecider` or `.decide(`, although no file under `src/kani/run/` mentions `tractab` on this head, so the narrowing was not needed. The new needles miss a run-layer call of the new wrappers (`refuse_if_not_tractable(`, `execute_kani_obligation_decided(`), the exact mutant this PR introduces; only the separate layout import rule still catches it. Restore the broader needle or add the wrapper names. | tests/it/kani_tractability.rs:644-668 |
| FND-003 | low | The shipped row records `solver_version: "cadical"`, a solver name, not a version; FR-056-AC-4 and AC-14 say the refusal names the solver version, and the test fixtures use `cadical-2.0`. Record the CaDiCaL version CBMC 6.11.0 ran (from the Kani/CBMC log) or state in the qualification doc that it was not captured. | docs/qualification/ir1530-general-route-scalar-table.json:11, tests/it/kani_tractability.rs:1000 |
| FND-004 | low | `DecidedRunRefusal` implements neither `Display` nor `std::error::Error`, unlike the `KaniExecutionRefusal` it wraps (`execute.rs:247`, `:310`), so a caller cannot propagate it with `?` into a boxed error or log it uniformly. | src/kani/terminal/tractable.rs:30-43 |

## Verdict

Mergeable as a CG-side change. The refusal reuses the FR-056 seam, table loader and refusal arm
with no second mechanism, the serde extension is backward compatible, and the route scoping is
correct in both directions the tests check. FND-001 is the substantive gap: the refusal is real but
opt-in and has no production caller; it should be dispositioned as deferred to a quire-driver
ticket rather than fixed here. FND-002 to FND-004 are small and fixable in the same PR.

## New findings (disposition pass 2)

Checked at 7a7ceaa9, which merges origin/main (#819, #820, #822) and reconciles the route onto `IntegerRoute`.

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-005 | low | Stale docs after the reconciliation. `refuse_if_not_tractable` still says "Only a routed scalar harness on the general integer route is asked; every other harness, and a native-route scalar, is `Ok` without consulting the decider", but it now maps `ScalarIntegerRepresentation::Native` to `IntegerRoute::Native` and asks the decider for both routes. The module doc and the qualification doc's "The native-route harness is not refused" hold only because no native row ships. Update the rustdoc to say both routes are decided and that a native harness runs unless a native row qualifies. | src/kani/terminal/tractable.rs:1-8, src/kani/terminal/tractable.rs:75-92, docs/qualification/ir1530-general-route-scalar-not-tractable.md:63 |
| FND-006 | low | With the route reduced to `general`/`native`, a row no longer says which shape it measured (V2 clause or routed scalar harness). The shipped scalar row (`general`, add at 4 bits) would refuse every general-route V2 clause with an add of 4 bits or more if a caller passed it, or merged it into, a generation table. The `table.json` V2 rows (`general`, add and mul at 15 bits) likewise apply to general-route scalar harnesses at run. That matches FR-056's reading that the `rt::Integer` carrier is the cost, but neither FR-056-AC-15 nor the qualification doc says the rows cross shapes. FR-056-AC-15 only says the row "never refuses a native-route harness". State it, or keep the shipped scalar row out of generation tables explicitly. | src/kani/tractability.rs:194-198, docs/qualification/ir1530-general-route-scalar-table.json:4, spec/kani/functional/FR-056-kani-tractability-refusal.md:140 |

## Dispositions

Round 1, checked against 5443b94b (fix commit over c3a58c73).

| FND | outcome | reason |
|---|---|---|
| FND-001 | deferred | Planner ruling: production wiring is a quire-driver PR under IR-1530 (`src/provider.rs` and `src/upstream.rs` to call the decided wrapper with the table). CG now ships the row in-crate as `TractabilityTable::shipped_general_route_scalar()` (`include_bytes!` of the docs JSON), and FR-056 Description and the qualification doc state the refusal is opt-in and name that caller. |
| FND-002 | fixed | The scan needle is restored to any `tractab` mention and also names `.decide(`, `obligation_decided`, `obligations_decided` and `refuse_if_not_tractable` (tests/it/kani_tractability.rs:653-658). |
| FND-003 | fixed | Row `solver_version` is `cadical-3.0.0`. Checked by the reviewer: the installed `~/.kani/kani-0.68.0/bin/cbmc` reports `6.11.0 (cbmc-6.11.0)` and embeds the string `cadical-3.0.0`. FR-015-AC-99 records a real run on Kani 0.68.0, CBMC 6.11.0, CaDiCaL 3.0.0. The doc's TC-043 citation is stand-in fixture text, not a run log, but the value is correct. |
| FND-004 | fixed | `impl Display for DecidedRunRefusal` (operation id and width for the refusal; the runner's message for `Execution`) and `impl std::error::Error` with `source()` returning the wrapped `KaniExecutionRefusal`; the test asserts the message holds the operation id and `4 bits`. |

Round 2, checked against 7a7ceaa9 (5443b94b..7a7ceaa9, plus origin/main...HEAD for the merge).
FND-001 to FND-004 keep their round-1 outcomes: the scan needle, the `cadical-3.0.0` row and
`Display`/`Error` survived the merge, and FND-001 stays deferred to the quire-driver PR under IR-1530.
No regression for IR-1526 consumers: `quire_contract_codegen::IntegerRoute` is still re-exported
from the crate root (it moved from `negotiate` to `tractability`). `ClauseFeatures::new` gained a
leading `route` argument and `V2ClauseNotTractable` gained `route`; no sibling repo under ~/dev
uses either. The rows are labelled as main's FR-056 Status says (ir1022/ir1509 `native`;
`table.json` and ir1526 `general`), and `route` is required with no default. fmt is clean.
`it` `layout::`, `kani_tractability::` and `kani_obligations::`: 65 passed, 10 ignored. `--lib kani::`: 272 passed.

| FND | outcome | reason |
|---|---|---|
| FND-005 | still-open | New in round 2; the docs are not yet updated. |
| FND-006 | still-open | New in round 2; neither the spec nor the doc states that the rows cross shapes. |

Round 3, checked against 5e6e2bbe (7a7ceaa9..5e6e2bbe). fmt clean; `it` layout/kani_tractability/kani_obligations 66 passed, 10 ignored. The new public
`negotiate_kani_obligations_with_decider(request, decider, route)` is a thin entry over the same
`negotiate_integer_policy` and `&dyn TractabilityDecider` seam. `negotiate_kani_obligations_with_options`
now delegates to it after loading the table. It adds no second mechanism or decision path: the
trait was already public, and FR-056-AC-1 and AC-12 already allow other implementations. The table
lookup stays the only decider CG constructs. Non-blocking nits, not findings: TC-070 step 2 now
states the route assertion twice (new sentence plus the old one), and the `tractable.rs` module doc
has a sentence fragment ("Through the one [`TractabilityDecider`] seam and the one ...").

| FND | outcome | reason |
|---|---|---|
| FND-005 | fixed | Rustdoc and qualification doc now say both routes are decided and a native harness is refused only by a `native` row, none shipped. |
| FND-006 | fixed | FR-056-AC-15 and the qualification doc now state rows apply per route across both shapes (the cost is the `rt::Integer` carrier) and the shipped scalar row is scoped by use, for the decided run wrappers, not a generation `tractability_table`. |
