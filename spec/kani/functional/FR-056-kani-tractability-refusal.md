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

## Outputs

- `Run`: the clause proceeds to generation and execution exactly as before.
- `Refuse`: `UnsupportedObligation::V2ClauseNotTractable` and no harness, with the fields of
  FR-056-AC-4 (the clause `node_id` as `V2ClauseOperatorNotSupported` carries, the operation id,
  width, evidence row and versions).

## Behavior

### Decider seam

The decider SHALL take one value, the clause features, and return `Run` or
`Refuse { reason, evidence_row }`. The features SHALL be: the ordered operation ids, each operation's
width after range propagation, the input bounds, the collection bound (absent when the clause has
none) and the generator and kernel versions. The decider SHALL NOT read the clause body, the
package or the filesystem.

### Measured table

The table SHALL be data with one row per measured probe: `operation_ids` (catalog ids),
`max_operand_bits`, `collection_bound`, `verdict` (`tractable` or `not_tractable`), `wall_seconds`,
`peak_bytes`, the Kani, CBMC and solver versions, and the generator and kernel version the probe
ran against. CG SHALL load it from a file and SHALL contain no per-operation tractability code. A
malformed row SHALL fail the load with the row index; it SHALL NOT be skipped silently.

### Lookup

The table decider SHALL refuse a clause when some operation id of the clause appears in a current
row with verdict `not_tractable` whose `max_operand_bits` is less than or equal to that
operation's width: a `not_tractable` row at `B` bits says the operation is intractable at `B` bits
and wider. An operation whose width is unknown (no integer operand, or an operand interval beyond
`i128`) matches no row. A row is current when its generator and kernel version equal CG's. A stale row is
ignored. When several rows qualify, the decider SHALL report the one with the smallest
`max_operand_bits`, then the first in file order, and the first qualifying operation in
`operation_ids` order.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-056-AC-1 | The decider is one interface taking only the clause-feature value of the Behavior section and returning `Run` or `Refuse { reason, evidence_row }`. The table lookup is the only production decider and implements that interface; test-only deciders (FR-056-AC-12) are allowed. The interface's parameter type is the feature value alone, so a decider cannot receive the clause body, package or filesystem handle; this is checked by the compiler and by inspection of the interface signature. | Inspection |
| FR-056-AC-2 | CG exposes the width of each operation after range propagation as a clause feature, equal to the width the clause oracle rendering already computes (v2.rs and oracle/clause/native.rs), for an integer chain `a * b + c` over `Int[0, 2^k]` bounds at three widths. Mutant: exposing the declared operand width instead of the propagated width fails at the multiply-then-add case. | Test |
| FR-056-AC-3 | The table loads from a data file with the fields of the Behavior section; a row with an unknown verdict, a missing field or a non-integer width fails the load naming the row index; an empty table loads and every clause runs. No per-operation code exists: adding a row for a new operation id changes a clause's decision with no source change. Mutant: skipping a malformed row fails the load check. | Test |
| FR-056-AC-4 | A clause containing operation id O (accepted by the V2 clause path) at width W with a current `not_tractable` row for O at `max_operand_bits <= W` is refused as `V2ClauseNotTractable` with no harness. The refusal names the clause `node_id`, the operation id, the width, the evidence row (its full fields) and the Kani, CBMC, solver, generator and kernel versions. A supported sibling item of the same request is unaffected (FR-015-AC-23). A clause holding an operator the V2 path refuses is still `V2ClauseOperatorNotSupported`, not this refusal. Mutant: omitting the node, row or versions, or reporting the refusal as `V2ClauseOperatorNotSupported`, fails. | Test |
| FR-056-AC-5 | The boundary is inclusive and per operation, in bits as defined in Terms: a row at `max_operand_bits = 64` refuses an operation over `Int[0, 2^64 - 1]` (64 bits) and over `Int[0, 2^64]` (65 bits), and does not refuse one over `Int[0, 2^32]` (33 bits); a `not_tractable` row for operation A does not refuse a clause holding only operation B; a clause holding A and B refuses on A. Mutant: strict `<`, a width counted without the sign bit, or matching any operation fails. | Test |
| FR-056-AC-6 | A clause with no qualifying row is `Run`, and its generation output, harness bytes and identity record are byte-identical to the output with an empty table. A `tractable` row never refuses. Mutant: refusing on a `tractable` row, or changing the harness when the table is non-empty, fails. | Test |
| FR-056-AC-7 | A row whose generator version or kernel version differs from CG's is ignored: a clause that a current copy of the row would refuse runs, and changing only the row's Kani version, CBMC version or solver version does not make it stale. Mutant: ignoring the version check refuses the clause. | Test |
| FR-056-AC-8 | When several rows qualify, the reported refusal is the one of FR-056 Behavior (smallest `max_operand_bits`, then file order; first qualifying operation); reordering unrelated rows does not change it. Mutant: reporting the last qualifying row fails. | Test |
| FR-056-AC-9 | The wall-clock ceiling is unchanged: a clause the decider runs and that exceeds the ceiling still ends `KaniRunOutcome::Inconclusive` with the timed-out reason (FR-028-AC-2) and the decider is not consulted again. Mutant: converting that outcome into a `V2ClauseNotTractable` refusal fails. | Test |
| FR-056-AC-10 | A tractability corpus file holds clauses as plain data, each row recording its features (FR-056 Behavior), its MEASURED ground-truth verdict (`verified`, `falsified` or `timed_out`, with `wall_seconds` where measured) and its measurement source. It is seeded from probe rows, the IR-1022 clause and its mutants, the Meter, integer and decimal cases, and a few multi-operation and relational clauses. A row is ground-truth `timed_out` only if a run was measured; a row never takes its verdict from the decider. | Inspection |
| FR-056-AC-11 | A scoring test runs the current decider over every corpus row, prints the false-refuse count (decider `Refuse`, ground truth `verified` or `falsified`) and the false-accept count (decider `Run`, ground truth `timed_out`), and passes when neither count exceeds its value in the baseline file, a data file in this repository beside the corpus; it fails when either count exceeds it and does not require zero. A count below the baseline passes. Mutants: a decider that refuses everything fails on false-refuse; a decider that runs everything fails on false-accept. | Test |
| FR-056-AC-12 | Adding a corpus row changes the scored counts with no source change, and a second decider implementing the FR-056-AC-1 interface (for example, one that always returns `Run`) is scored against the same file with no change to the corpus or the scoring test. | Test |
| FR-056-AC-13 | Through CG's runner under installed Kani, one clause the table lets through ends with an executed verdict (`Verified` or `Falsified`), recorded with its wall time, the Kani version and CG's generator and kernel versions; and a clause holding a measured-bad row for an integer operation the V2 path accepts (integer multiply at a width at or above the row's bound) is refused as `V2ClauseNotTractable` with no Kani launch, and a narrower multiply is not refused. The Decimal multiply at full width row is exercised the same way only once IR-1444 lets it reach the decider. A probe record does not substitute. | Test (installed Kani) |

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

PARTIAL (IR-1526). FR-056-AC-1 to AC-9 and AC-11 to AC-12 are implemented and test-backed on the host
(`tests/it/kani_tractability.rs`; corpus and baseline in `tests/fixtures/tractability/`).
FR-056-AC-10 is partial: the shipped corpus holds ten measured rows, all `verified` or `falsified`
and all of the one IR-1022 clause and its mutants. It has no `timed_out` row (CG's own evidence has
no measured wall-clock timeout of a V2 clause), no Meter, decimal or pure-relational case and no other
multi-operation clause, so the false-accept count is not exercised by the shipped file. The one timeout
CG does hold (`docs/qualification/ir1530-general-route-scalar-not-tractable.md`) is of the general-route
`rt::Integer` scalar harness, which is not a V2 clause: the same features verify in seconds on the native
route, so it is out-of-domain evidence and is not a corpus row. Those rows
need new installed-Kani measurements. FR-056-AC-13 is coded as an ignored kani-lane test and has not
been run under installed Kani; its multiply refusal and pass-through fixtures also run as a host test
(`tc_070_the_ac13_multiply_fixtures_split_on_the_row_bound`). The narrow multiply is negotiated on the
general route because the native route renders no two-parameter precondition. FR-056-AC-1 to AC-12 need no Kani run and are host tests. FR-056-AC-13
needs an installed Kani run and is the only criterion that does; FR-056-AC-10 is an inspection of the
corpus.
Out of scope: a threshold classifier, predicting combined cost, and state-frame, collection and
scalar-claim forms beyond what the descriptor already exposes (named as a follow-up only if the
probes show a need). The table's source rows are measured elsewhere and loaded at run time; CG
does not hold a copy of another repository's table.
