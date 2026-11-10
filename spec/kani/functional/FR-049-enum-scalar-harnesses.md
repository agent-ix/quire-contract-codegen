---
id: FR-049
title: "Generate enum scalar comparison harnesses"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-025
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-028
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-039
    type: depends_on
---
# FR-049: Generate enum scalar comparison harnesses

## Description

When an admitted request selects an `enum` scalar comparison node
(`quire.op.enum.eq`, `.ne`, `.lt`, `.le`, `.gt` or `.ge`) over a model-owned
enum whose complete member set is finite and declared, CG SHALL generate a
bounded Kani row proving that the generated enum oracle of
[FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md) agrees with a
native expression of the same comparison over every pair of declared members.
This row replaces the `unsupported`/`ShadowFamilyNotBuilt` disposition for the
enum leaf family only; every other family keeps that disposition.

The input authority and per-item accounting remain
[FR-039](../../core/functional/FR-039-contract-package-consumption.md)'s one
admitted ContractPackage. This requirement defines required meaning, not a new
IR schema or a currently available Rust API. Enum semantics (member identity,
declared order, `compare_enum`) belong to the QSV owner through FR-014; CG does
not reimplement them.

## Inputs

- An admitted enum comparison node, its operator, its two operand parameters and
  the selected model-owned `model`/`variant_type` with its checked, ordered
  member identities as supplied by the owner route.
- The requested proof subject, backend and execution bounds governed by
  [FR-015](./FR-015-bounded-kani-obligations.md),
  [FR-025](./FR-025-generated-subject-abi.md) and
  [FR-028](./FR-028-bounded-proof-ceilings.md).

## Outputs

A supported item has its generated enum oracle, native expression, harness,
symbolic input encoding correspondence and source-associated proof record. A
refused item has its named disposition and no Rust harness. Execution results
and counterexample replay remain governed by
[FR-017](./FR-017-kani-execution-evidence.md),
[FR-029](./FR-029-run-outcome-terminal-record.md) and
[FR-016](../../replay/functional/FR-016-witness-native-replay.md).

## Behavior

When the comparison is representable, CG SHALL draw each operand as one symbolic
unsigned index and constrain it to `0..member_count` of the selected enum, so
that every declared member, and no other value, is reachable. The index to
member mapping is the declared member order of the admitted enum; the harness
SHALL build each operand through the owner's checked member identity for that
index and SHALL NOT mint, rename or reorder members. Operand indices are drawn
independently, covering equal, unequal and every ordered pair.

When asserting, the harness SHALL compare the oracle's `Outcome<bool>` against a
native expression of the operator over the two indices: `==`, `!=`, `<`, `<=`,
`>`, `>=` respectively, with ordering following declared member order as
FR-014 defines `compare_enum`. The assertion SHALL require `Completed` with the
native Boolean. An `Undefined`, `Refused` or `Incomplete` outcome is never an
assumed-away input or a successful result.

When two operands are drawn from different enums, or a node's operand is not an
admitted member of its enum, the item is refused with a typed reason before any
harness; it is not compared by index.

When the selected enum has zero members, more members than the proof ceiling's
index bound, or a member set the owner route does not supply completely, the
item is `unsupported` (or `requires-bound` for an unbounded member set) naming
the enum, and no harness is emitted.

When emitting an enum harness, CG SHALL include exactly one final completion cover after
all assertions, so that a vacuous constraint cannot yield `Verified`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-049-AC-1 | For each of `quire.op.enum.eq`, `.ne`, `.lt`, `.le`, `.gt` and `.ge` over an admitted enum with at least three declared members, CG generates one harness whose oracle is FR-014's generated enum comparison and whose expected value is the native operator over declared-order indices. The oracle and native sides are generated from the same admitted node and retain its source association. | Test |
| FR-049-AC-2 | The symbolic domain is exactly the declared members: each operand index is constrained to `0..member_count`, every ordered pair of members is reachable, and no out-of-range index is reachable. Admitting one extra index, dropping the last member or fixing one operand to a constant fails the domain correspondence check. | Test |
| FR-049-AC-3 | Operand values are built from the owner's checked member identities in declared order. A harness that reorders members, mints a member identity or compares by a name other than the checked identity fails verification of the ordered operators. | Test |
| FR-049-AC-4 | The assertion requires `Completed` with the native Boolean. No assumption depends on the oracle outcome, and a control producing `Undefined`, `Refused` or `Incomplete` is retained with that typed outcome and cannot become `Verified`. | Test |
| FR-049-AC-5 | The healthy enum comparison receives `Verified` only from a successful proof with every check and the completion cover satisfied. Each mutant (below) produces a failed assertion with solver playback of in-domain member indices, and replaying the playback through the owner route reproduces the disagreement. | Test |
| FR-049-AC-6 | The mutant controls each fail the proof: `lt` swapped with `le`, `gt` swapped with `ge`, `eq` swapped with `ne`, ordering reversed, and ordering by index of a reordered member list. | Test |
| FR-049-AC-7 | Each emitted enum harness has exactly one completion cover after all assertions. An unsatisfied or unreported cover, failed unwind, timeout, memory exhaustion or missing verdict cannot yield `Verified`; classifications retain FR-017 and FR-029 distinctions. | Test |
| FR-049-AC-8 | Operands of different enums, a non-member operand, a zero-member enum, a member count above the index bound and an incomplete or unbounded member set are each refused with a typed, source-associated reason naming the enum and no harness. Every requested item has exactly one disposition under FR-039 and a refused item does not alter a supported sibling. | Test |
| FR-049-AC-9 | A leaf or member of any other family (text, rational, decimal, quantity, float, sequence, set, bag, ordered set) remains `unsupported`/`ShadowFamilyNotBuilt` and an enum node inside an unsupported composite keeps its composite disposition. | Test |
| FR-049-AC-10 | Run evidence records the selected node and source association, the enum identity and ordered member list, operator, symbolic encoding, actual subject and full ordered backend arguments, every unwind and option, tool versions, target, limits, raw checks and cover, playback, exit and typed terminal outcome. Replay uses the retained generated artifact without hand editing. | Test |

## Dependencies

- [FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md) owns enum
  comparison semantics and the generated oracle; this requirement adds only the
  bounded proof row over it.
- [FR-025](./FR-025-generated-subject-abi.md) owns the symbolic argument
  schema; the enum index row is planned there as a single unsigned binding.
- [TC-063](../matrix/TC-063-enum-scalar-harnesses.md) defines the planned
  verification. This requirement changes none of FR-015's existing ceilings.

## Status

PLANNED (IR-1438). This specification supplies no implementation, generated
proof or acceptance credit. All ten criteria remain planned and untagged until
actual implementation and verification.
