---
id: FR-056
title: "Refuse a V2 clause whose operation at its width is a measured not-Kani-tractable row"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-028
    type: depends_on
---
# FR-056: Refuse a V2 clause whose operation at its width is a measured not-Kani-tractable row

## Description

Some V2 clause operations, at some operand widths, are known by measurement not to finish under
Kani within the wall-clock ceiling (for example integer multiply at a wide width). Today CG
generates and runs them and the run ends `KaniRunOutcome::Inconclusive` with the timed-out reason (FR-028-AC-2). This requirement lets CG
refuse such a clause before running Kani, with a typed reason, from measured data. It adds no
threshold, formula or prediction of combined cost.

The decision sits behind one narrow interface, the tractability decider. The measured-table lookup
is its first and only implementation in this requirement. A later decider (rules, thresholds,
anything) is dropped in behind the same interface and scored against the same corpus (FR-056-AC-12).
Complex clauses may be misclassified; the goal is the seam and a corpus that scores any decider,
not a perfect first decider.

The refusal is one more arm of the existing `UnsupportedObligation`, beside
`V2ClauseOperatorNotSupported` (FR-015-AC-42) and `NoFiniteEncoding`; it is not a parallel family.
This requirement amends no FR-015 or FR-017 text. The wall-clock ceiling of FR-028 stays the
unchanged backstop: a clause the decider lets through still runs under it.

The decider runs AFTER the operator check of FR-015-AC-42: it sees only operations the V2 clause
path accepts today (connectives, the six integer comparisons, integer add, subtract, multiply and
negate). An operation refused earlier as `V2ClauseOperatorNotSupported` never reaches it. A row for
a Decimal operation applies only once the Decimal carrier (IR-1444, FR-051) lets such a clause reach
the decider; until then it matches nothing. This requirement changes no FR-015 text.

Terms:
- The *route* is the integer encoding a negotiation selects, the code's `IntegerRoute`: `general`
  (the `rt::Integer` carrier, `IntegerRoute::General`) or `native` (the kernel-admitted native
  integer encoding, `IntegerRoute::Native`). Measurement shows the two differ in tractability by
  more than any width does, so the route is a decider feature, not a detail of the run.
- A clause's *operation ids* are `V2ClauseDescriptor.operation_identities`.
- The *width* of an operation is the largest bit width, over its operands, of the
  `NativeIntegerInterval` that range propagation yields for that operand (the value CG computes
  while rendering the clause oracle and does not yet expose). The bit width of an interval
  `[lo, hi]` is the smallest `n` such that every value is representable in `n` bits: unsigned
  `n = bits(hi)` when `lo >= 0`, else two's complement signed `n` (for `[-2^(n-1), 2^(n-1)-1]`).
  Because the native carrier is signed, `Int[0, 2^k]` is counted as `k + 1` bits; `Int[0, 2^63 - 1]`
  is 63 unsigned bits and `Int[0, 2^64 - 1]` is 64.
- The *generator version* is the CG crate version (`CARGO_PKG_VERSION` of this crate); the *kernel
  version* is the integer `quire_exact::KERNEL_VERSION` of the quire-exact crate CG links (a row's
  kernel version is that integer). CG exposes one constant for each; no hash or pin is involved.

## Inputs

- An admitted V2 clause claim as FR-015-AC-40 and FR-015-AC-41 admit it.
- The measured table: a data file whose path the caller gives CG in the generation request's
  options (a `tractability_table` path). When the option is absent, no table is loaded and every
  clause runs (an empty table). When the path is given but the file is missing or unreadable, or
  any row is malformed, the request fails with a typed load error naming the path (and the row
  index) and generates nothing; it never falls back to an empty table.
- CG's own generator version and kernel version.
- The route of the negotiation (`IntegerRoute`), which the caller already chooses.

## Outputs

- `Run`: the clause proceeds to generation and execution exactly as before.
- `Refuse`: `UnsupportedObligation::V2ClauseNotTractable` and no harness, with the fields of
  FR-056-AC-4 (the clause `node_id` as `V2ClauseOperatorNotSupported` carries, the operation id,
  width, route, evidence row and versions).

## Behavior

### Decider seam

The decider SHALL take one value, the clause features, and return `Run` or
`Refuse { reason, evidence_row }`. The features SHALL be: the route, the ordered operation ids, each operation's
width after range propagation, the input bounds, the collection bound (absent when the clause has
none) and the generator and kernel versions. The decider SHALL NOT read the clause body, the
package or the filesystem.

### Measured table

The table SHALL be data with one row per measured probe: `operation_ids` (catalog ids),
`route` (`general` or `native`), `max_operand_bits`, `collection_bound`, `verdict` (`tractable` or `not_tractable`), `wall_seconds`,
`peak_bytes` (`null` when the runner reported no peak), the Kani, CBMC and solver versions, and the generator and kernel version the probe
ran against. CG SHALL load it from a file and SHALL contain no per-operation tractability code. A
malformed row SHALL fail the load with the row index; it SHALL NOT be skipped silently. A row
without a `route` key, or with any other `route` value, is malformed like any row with a missing
field; there is no default route.

### Lookup

The table decider SHALL refuse a clause when some operation id of the clause appears in a current
row, whose `route` equals the clause's route, with verdict `not_tractable` whose `max_operand_bits` is less than or equal to that
operation's width: a `not_tractable` row at `B` bits says the operation is intractable at `B` bits
and wider. An operation whose width is unknown (no integer operand, or an operand interval beyond
`i128`) matches no row. A row is current when its generator and kernel version equal CG's. A stale row is
ignored. A row for the other route is ignored: a `not_tractable` general-route row never refuses a
native-route clause, and the reverse. The decider does not infer one route from the other. When several rows qualify, the decider SHALL report the one with the smallest
`max_operand_bits`, then the first in file order, and the first qualifying operation in
`operation_ids` order.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-056-AC-1 | The decider is one interface taking only the clause-feature value of the Behavior section and returning `Run` or `Refuse { reason, evidence_row }`. The table lookup is the only production decider and implements that interface; test-only deciders (FR-056-AC-12) are allowed. The interface's parameter type is the feature value alone, so a decider cannot receive the clause body, package or filesystem handle; this is checked by the compiler and by inspection of the interface signature. | Inspection |
| FR-056-AC-2 | CG exposes the width of each operation after range propagation as a clause feature, equal to the width the clause oracle rendering already computes (v2.rs and oracle/clause/native.rs), for an integer chain `a * b + c` over `Int[0, 2^k]` bounds at three widths. CG also exposes the route of the negotiation as a clause feature, `general` for `IntegerRoute::General` and `native` for `IntegerRoute::Native`, for the same clause. Mutant: exposing the declared operand width instead of the propagated width fails at the multiply-then-add case; reporting a fixed route fails the route check. | Test |
| FR-056-AC-3 | The table loads from a data file with the fields of the Behavior section; a row with an unknown verdict, a missing field, a missing or unknown `route` or a non-integer width fails the load naming the row index; an empty table loads and every clause runs. No per-operation code exists: adding a row for a new operation id changes a clause's decision with no source change. Mutant: skipping a malformed row fails the load check. | Test |
| FR-056-AC-4 | A clause containing operation id O (accepted by the V2 clause path) at width W with a current `not_tractable` row for O at `max_operand_bits <= W` is refused as `V2ClauseNotTractable` with no harness. The refusal names the clause `node_id`, the operation id, the width, the route, the evidence row (its full fields) and the Kani, CBMC, solver, generator and kernel versions. A supported sibling item of the same request is unaffected (FR-015-AC-23). A clause holding an operator the V2 path refuses is still `V2ClauseOperatorNotSupported`, not this refusal. Mutant: omitting the node, route, row or versions, or reporting the refusal as `V2ClauseOperatorNotSupported`, fails. | Test |
| FR-056-AC-5 | The boundary is inclusive and per operation, in bits as defined in Terms: a row at `max_operand_bits = 64` refuses an operation over `Int[0, 2^64 - 1]` (64 bits) and over `Int[0, 2^64]` (65 bits), and does not refuse one over `Int[0, 2^32]` (33 bits); a `not_tractable` row for operation A does not refuse a clause holding only operation B; a clause holding A and B refuses on A. These cases hold the route fixed (a row and a clause on the same route). Mutant: strict `<`, a width counted without the sign bit, or matching any operation fails. | Test |
| FR-056-AC-6 | A clause with no qualifying row is `Run`, and its generation output, harness bytes and identity record are byte-identical to the output with an empty table. A `tractable` row never refuses. Mutant: refusing on a `tractable` row, or changing the harness when the table is non-empty, fails. | Test |
| FR-056-AC-7 | A row whose generator version or kernel version differs from CG's is ignored: a clause that a current copy of the row would refuse runs, and changing only the row's Kani version, CBMC version or solver version does not make it stale (nor does the route, which FR-056-AC-14 matches). Mutant: ignoring the version check refuses the clause. | Test |
| FR-056-AC-8 | When several rows qualify, the reported refusal is the one of FR-056 Behavior (smallest `max_operand_bits`, then file order; first qualifying operation); reordering unrelated rows does not change it. Mutant: reporting the last qualifying row fails. | Test |
| FR-056-AC-9 | The wall-clock ceiling is unchanged: a clause the decider runs and that exceeds the ceiling still ends `KaniRunOutcome::Inconclusive` with the timed-out reason (FR-028-AC-2) and the decider is not consulted again. Mutant: converting that outcome into a `V2ClauseNotTractable` refusal fails. | Test |
| FR-056-AC-10 | A tractability corpus file holds clauses as plain data, each row recording its features (FR-056 Behavior), its MEASURED ground-truth verdict (`verified`, `falsified`, `timed_out` or `memory_exhausted`, with `wall_seconds` where measured) and its measurement source. It is seeded from probe rows (each recording its route as a feature), the IR-1022 clause and its mutants, the Meter, integer and decimal cases, and a few multi-operation and relational clauses. It holds `route` as a feature of every row; a row without it is malformed like a table row. The corpus holds `native`-route rows as well as `general`-route rows: for each of integer add, integer multiply and a two-operation chain (`other * offset + 3 <= 100` shape) on the `native` route, at least one measured width below 15 bits and one at or above 15 bits (the 15, 23 and 31 bit unsigned operand widths of the general-route rows are the natural choice), so the corpus shows whether the general-route rows over-refuse native clauses. A row is ground-truth `timed_out` or `memory_exhausted` only if a run was measured and ended that way; a row never takes its verdict from the decider. | Inspection |
| FR-056-AC-11 | A scoring test runs the current decider over every corpus row, prints the false-refuse count (decider `Refuse`, ground truth `verified` or `falsified`) and the false-accept count (decider `Run`, ground truth `timed_out` or `memory_exhausted`), and passes when neither count exceeds its value in the baseline file, a data file in this repository beside the corpus; it fails when either count exceeds it and does not require zero. A count below the baseline passes. Mutants: a decider that refuses everything fails on false-refuse; a decider that runs everything fails on false-accept. | Test |
| FR-056-AC-12 | Adding a corpus row changes the scored counts with no source change, and a second decider implementing the FR-056-AC-1 interface (for example, one that always returns `Run`) is scored against the same file with no change to the corpus or the scoring test. | Test |
| FR-056-AC-13 | Through CG's runner under installed Kani, one clause the table lets through ends with an executed verdict (`Verified` or `Falsified`), recorded with its wall time, the Kani version and CG's generator and kernel versions; and a clause holding a measured-bad row for an integer operation the V2 path accepts (integer multiply at a width at or above the row's bound) is refused as `V2ClauseNotTractable` with no Kani launch, and a narrower multiply is not refused. The Decimal multiply at full width row is exercised the same way only once IR-1444 lets it reach the decider. A probe record does not substitute. | Test (installed Kani) |
| FR-056-AC-14 | Lookup matches rows only for the clause's route. The same clause (one operation, one width) and a `not_tractable` row for that operation at a bound at or below the width: with the row on `general` the clause is refused as `V2ClauseNotTractable` on the `general` route and the decider returns `Run` on the `native` route (the item is not refused as `V2ClauseNotTractable`; it may still end in another disposition such as `RenderFailed`); with the row on `native` the reverse. A `tractable` row does not refuse on either route. The refusal reports the clause's route. A `native` row at a smaller bound than a qualifying `general` row does not take precedence over it on the `general` route. Mutant: ignoring the route (matching on operation and width alone) refuses the native clause in the first case and fails. | Test |

## Dependencies

- [FR-015](./FR-015-bounded-kani-obligations.md) owns the V2 clause route, FR-015-AC-42 and the
  sibling rule (FR-015-AC-23); this requirement edits none of it.
- [FR-017](./FR-017-kani-execution-evidence.md) owns execution evidence and the two-phase runner
  used for FR-056-AC-13.
- [FR-028](./FR-028-bounded-proof-ceilings.md) owns the wall-clock ceiling that stays the backstop.
- [FR-051](./FR-051-decimal-scalar-harnesses.md) owns the decimal operations; Decimal rows apply
  only once its carrier (IR-1444) lets a decimal clause reach the decider.
- [TC-070](../matrix/TC-070-kani-tractability-refusal.md) defines the verification.

## Status

PARTIAL (IR-1526). The route feature (Terms, Lookup, FR-056-AC-14, and the route additions to
FR-056-AC-2 to AC-5, AC-7 and AC-10) is specified and not yet implemented. The decider and the
table do not carry a route today, and the shipped table and corpus rows have no `route` key. When
the route lands the rows are labelled by how they were measured: the `ir1022-*` and `ir1509-*`
corpus rows are `native` (their harnesses come from `negotiate_kani_native_obligations`; see
`docs/qualification/ir1022-kani-clause-measurements.md` and `ir1509-kani-clause-controls.md`), and
the two `table.json` rows and the four `ir1526-*` corpus rows are `general`. The native `ir1022-*`
rows (integer add at 10 bits) already cover AC-10's native add case below 15 bits. The
host test `tc_070_the_ac13_multiply_fixtures_split_on_the_row_bound` negotiates on the general
route and its expectations flip for the native route when the route lands.
FR-056-AC-1 to AC-9 and AC-11 to AC-12 are implemented and test-backed on the host
(`tests/it/kani_tractability.rs`; corpus and baseline in `tests/fixtures/tractability/`).
FR-056-AC-10 is partial: the shipped corpus holds fourteen measured rows. Ten are `verified` or
`falsified` and all of the one IR-1022 clause and its mutants. Four are general-route V2 clauses that
did not finish under Kani 0.68.0. Three are `other * offset + 3 <= 100`, operands
`Int[0, 2^n - 1]` for n = 15, 23 and 31 unsigned bits, each `TimedOut` at 600 s. One is the control
`other + offset <= 100` (no multiply) at n = 15: the kernel OOM-killed `cbmc` at the 24 GiB memory
cap after 542.5 s (`anon-rss` 25055932 kB), so it is `memory_exhausted`, not a timeout. All were measured by
`ir1526_probe_wide_integer_multiply_under_installed_kani`. Because the add-only clause fails too,
the measurements do not show that the multiply is the cause. All ran on the general-route
`rt::Integer` carrier, which `docs/qualification/ir1530-general-route-scalar-not-tractable.md`
measured not tractable for an add at 4 bits.
CG's measured table (`tests/fixtures/tractability/table.json`) holds `not_tractable` rows for integer
multiply and integer add from 15 bits, the smallest measured-bad width, both from this evidence. Until the route feature lands, the route is not a decider feature, so those rows also refuse clauses that would be tractable on the
native route; the corpus holds no native-route clause of 15 bits or more, so that false-refuse is
unmeasured. Still missing for AC-10: the route in the corpus features and in the decider,
native-route integer add at 15 bits or more, native-route multiply and native-route two-operation chain
(`other * offset + 3 <= 100`) at any listed width (none measured; the AC-13 clause that Verified in
235.5 s on the native route is `with_bounded_amount_implication` over `Int[0, 1000]`, 10 bits), Meter, decimal and pure-relational cases and a multi-operation clause that
is not integer arithmetic. The ir1530 timeout is of the scalar harness, not a V2 clause, so it is
not a corpus row. Those rows need new installed-Kani measurements. The add row's `peak_bytes` is the OOM-killed
`cbmc` process's `anon-rss` from the kernel log; the multiply row's is `null`: the runner reported no peak. FR-056-AC-13 is coded as a kani-lane test and was executed under installed Kani 0.68.0: its clause
ended `Verified` in 235.5 s on the native route. Its multiply refusal and pass-through fixtures also run as a host test
(`tc_070_the_ac13_multiply_fixtures_split_on_the_row_bound`). The narrow multiply is negotiated on the
general route because the native route renders no two-parameter precondition. FR-056-AC-1 to AC-12 and AC-14 need no Kani run and are host tests. FR-056-AC-13
needs an installed Kani run and is the only criterion that does; FR-056-AC-10 is an inspection of the
corpus.
Out of scope: a threshold classifier, predicting combined cost, and state-frame, collection and
scalar-claim forms beyond what the descriptor already exposes (named as a follow-up only if the
probes show a need). The table's source rows are measured elsewhere and loaded at run time; CG
does not hold a copy of another repository's table.
