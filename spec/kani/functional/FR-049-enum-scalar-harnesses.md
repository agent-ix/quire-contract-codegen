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
  - target: ix://agent-ix/quire-exact/FR-407
    type: references
---
# FR-049: Generate enum scalar comparison harnesses

## Description

When an admitted request selects an `enum` scalar comparison node
(`quire.op.enum.eq`, `.ne`, `.lt`, `.le`, `.gt` or `.ge`) over an admitted enum
declaration (eq and ne over any admitted declaration; lt, le, gt and ge over an
`ordered enum` declaration only), CG SHALL generate a bounded Kani row proving that the
native enum comparison of quire-exact
([FR-407](ix://agent-ix/quire-exact/FR-407), `compare_native_enum` over two
`NativeEnum` carriers) agrees with a
native expression of the same comparison over every pair of declared members.
Today such a node settles as `unsupported` / `OperationNotRendered`
(negotiate.rs, scalar lowering `NoRenderer`); this row replaces that
disposition for the enum comparison node only. Every other family keeps its
existing disposition, and the `ShadowFamilyNotBuilt` leaf row of
[FR-015](./FR-015-bounded-kani-obligations.md) is unchanged for enum leaves of
composites.

Ordering operators (`lt`, `le`, `gt`, `ge`) are meaningful only for an
`ordered enum` declaration. `compare_native_enum` returns the ill-typed
cause `UnorderedEnumOrdering` for an ordering operator over an unordered
declaration, and `DistinctEnumDeclarations` for operands of different
declarations. Both are `Err(IllTyped)` from `compare_native_enum`,
not a Boolean outcome.

The input authority and per-item accounting remain
[FR-039](../../core/functional/FR-039-contract-package-consumption.md)'s one
admitted ContractPackage. This requirement defines required meaning, not a new
IR schema or a currently available Rust API. The enum carrier, its construction and its comparison
(`NativeEnum`, `from_member`, `from_index`, `compare_native_enum`) belong to
quire-exact (FR-407); the checked member list and ordered flag come from the
owner route ([FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md)).
CG does not reimplement them.

## Inputs

- An admitted enum comparison node, its operator, its two operand parameters and
  the selected enum declaration with its checked member identities, declared
  member order and ordered flag, as supplied by the owner route.
- The requested proof subject, backend and execution bounds governed by
  [FR-015](./FR-015-bounded-kani-obligations.md),
  [FR-025](./FR-025-generated-subject-abi.md) and
  [FR-028](./FR-028-bounded-proof-ceilings.md).

## Outputs

A supported item has its native enum comparison, native expression, harness,
symbolic input encoding correspondence and source-associated proof record. A
refused item has its named disposition and no Rust harness. Execution results
and counterexample replay remain governed by
[FR-017](./FR-017-kani-execution-evidence.md),
[FR-029](./FR-029-run-outcome-terminal-record.md) and
[FR-016](../../replay/functional/FR-016-witness-native-replay.md).

## Behavior

When the comparison is representable, CG SHALL draw each operand as one
symbolic signed `i64` index constrained to `0..member_count` of the selected
declaration, so that every declared member, and no other value, is reachable.
The index to member mapping is the declared member order of the admitted
declaration. Operand indices are drawn independently, covering equal, unequal
and every ordered pair.

At generation time CG SHALL run `from_member` over the `EnumShape` the owner
route admitted, and over each checked member of it, and SHALL use that shape
as supplied: a shape CG derived, renamed or rebuilt is not the owner-admitted
shape. `from_member` refuses a member the shape does not admit at its position
with `MemberNotInShape` ([FR-407-AC-3](ix://agent-ix/quire-exact/FR-407)), so a
minted identity (same position, different key) yields no operand. Inside the
proof the harness builds each operand with `from_index(tag, ordered, index,
count)`, so the proof holds no variant list and no heap value. The carrier
holds the member's rank, the member count, the ordered flag and an opaque
declaration tag, so the `eq` and `ne` proofs are over rank only.

The harness SHALL NOT mint, rename or reorder members.

When asserting, the harness SHALL compare the native comparison's `Outcome<bool>` against
a native expression of the operator over the two indices: `==`, `!=`, `<`,
`<=`, `>`, `>=` respectively. Equality and inequality are over rank;
ordering follows declared position and applies to an `ordered enum` only.

The harness SHALL require `Completed` with the native Boolean. `Incomplete` is
the only non-Completed outcome `compare_native_enum` returns for admitted operands; it
is never an assumed-away input or a successful result.

If an ordering operator is requested over an unordered declaration, or the
operands belong to different declarations, then CG SHALL refuse the item with
the typed ill-typed cause (`UnorderedEnumOrdering` or
`DistinctEnumDeclarations`), source association and no harness.

If the comparison's `Err(IllTyped)` path is reached at run time, then
CG SHALL report a non-`Verified` outcome retaining the ill-typed cause.

If the owner route does not supply the declaration's complete checked member
list and ordered flag, then CG SHALL account the item `unsupported` naming the
enum, with no harness. A zero-member enum is rejected at owner admission and
never reaches CG, so CG defines no zero-member disposition. The index is bounded
by the admitted member count, which the owner bounds at admission; this row
introduces no separate index ceiling.

When emitting an enum harness, CG SHALL include exactly one final completion
cover after all assertions, so that a vacuous constraint cannot yield
`Verified`.

### Symbolic ABI and witness roundtrip (planned)

When expanding a logical enum operand, CG SHALL persist exactly one physical
`i64` binding for its one real parameter, with original parameter/source
identity, declaration identity, ordered member list and inclusive index bounds
`0..member_count-1`. Logical parameter order remains ascending by parameter
identifier; Boolean and integer ordering is unchanged. These are required
conceptual binding facts, not a claim that a current Rust field carries them.

When decoding retained playback, CG SHALL validate the entry with the existing
I64 width, little-endian byte and decoded-comment checks and the byte and
value-count limits, then check the index lies in `0..member_count`. An
out-of-range index, or a missing, duplicate or foreign binding, refuses with a
typed cause and no replay. CG SHALL reconstruct the member from the declaration's
checked identity at that index and submit it through the existing owner replay
route; it introduces no transcript grammar or occurrence key. If that route is
unavailable the item is `unsupported` with no harness.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-049-AC-1 | For `eq` and `ne` over an admitted enum declaration of at least three members, and for each of `lt`, `le`, `gt` and `ge` over an `ordered enum` of at least three members, CG generates one harness whose generated side is quire-exact's `compare_native_enum` over carriers and whose expected value is the native operator over indices. The comparison and native sides come from the same admitted node and retain its source association. | Test |
| FR-049-AC-2 | The symbolic domain is exactly the declared members: each operand index is an `i64` constrained to `0..member_count`, every ordered pair is reachable and no out-of-range index is reachable. Admitting one extra index, dropping the last member or fixing one operand to a constant fails the correspondence check. | Test |
| FR-049-AC-3 | At generation time `from_member` is run over the owner-admitted `EnumShape` and the owner's checked members, in declared order; the shape is the owner route's own, not one CG derived. A minted member identity (same position, different key) is refused with `MemberNotInShape` (FR-407-AC-3) and no harness is built from it. A generator-mint mutant that emits a renamed key in both the shape it passes and the members it passes fails the comparison of that shape with the owner-admitted shape. Inside the proof operands are built with `from_index`, and the `eq` and `ne` proofs are over rank only and make no claim about minted identity. A member list reordered relative to the declaration makes the ordering proofs over an `ordered enum` fail. Ordering operators are not claimed to detect a minted identity. | Test |
| FR-049-AC-4 | The assertion requires `Completed` with the native Boolean and no assumption depends on the comparison outcome. A control producing `Incomplete` keeps that typed outcome and cannot become `Verified`. A path reaching the comparison's `Err(IllTyped)` is non-`Verified` and retains its cause. | Test |
| FR-049-AC-5 | The healthy comparison receives `Verified` only from a successful proof with every check and the completion cover satisfied. Each mutant of AC-6 produces a failed assertion with solver playback of in-domain indices, and replay through the owner route reproduces the disagreement. | Test |
| FR-049-AC-6 | Mutants fail the operator that can detect them: `lt` rendered as `le` (and the converse) fails `lt` and `le`; `gt` as `ge` (and the converse) fails `gt` and `ge`; `eq` as `ne` (and the converse) fails `eq` and `ne`; reversed ordering fails `lt`, `le`, `gt` and `ge`. | Test |
| FR-049-AC-7 | Each emitted enum harness has exactly one completion cover after all assertions. An unsatisfied or unreported cover, failed unwind, timeout, memory exhaustion or missing verdict cannot yield `Verified`; classifications retain FR-017 and FR-029 distinctions. | Test |
| FR-049-AC-8 | An ordering operator over an unordered declaration and operands of different declarations are each refused with the typed ill-typed cause, source association and no harness. An incomplete member list or ordered flag is `unsupported` naming the enum. Every requested item has exactly one disposition under FR-039 and a refused item does not alter a supported sibling. | Test |
| FR-049-AC-9 | A leaf or member of any other family (text, rational, decimal, quantity, float, sequence, set, bag, ordered set) keeps its existing disposition, and an enum leaf inside a composite keeps the composite's `ShadowFamilyNotBuilt` disposition. | Test |
| FR-049-AC-10 | Each supported enum operand has exactly one `i64` physical binding grouped to its original parameter with declaration, member list and bounds. Playback decodes with the existing I64 width/comment checks and limits; an out-of-range index, or a missing, duplicate or foreign binding, refuses with a typed cause and no replay, and a valid index replays as the declaration's member at that position. Boolean and integer controls are unchanged. | Test |
| FR-049-AC-11 | Run evidence records the selected node and source association, declaration identity, ordered member list and ordered flag, operator, encoding, actual subject and full ordered backend arguments, every unwind and option, tool versions, target, limits, raw checks and cover, playback, exit and typed terminal outcome. Replay uses the retained generated artifact without hand editing. | Test |

## Dependencies

- quire-exact [FR-407](ix://agent-ix/quire-exact/FR-407) owns the carrier, its
  construction and `compare_native_enum`; this requirement adds only the
  bounded proof row over them. [FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md)
  supplies only the checked member list and ordered flag.
- [FR-025](./FR-025-generated-subject-abi.md) owns the symbolic argument
  schema; the enum index row is planned there as a single `i64` binding.
- [TC-063](../matrix/TC-063-enum-scalar-harnesses.md) defines the planned
  verification. This requirement changes none of FR-015's existing ceilings.

## Status

PARTIAL (IR-262). The generator `generate_enum_scalar_harness` (src/kani/generate/enum_scalar.rs)
settles an `enum` comparison node to a harness over `NativeEnum` carriers built with
`from_index`, or to a typed refusal, from a genuine QSL source admitted through Contract IR and
the owner's admitted context. `generate_routed_admitted` carries that context into the routed
route, so an enum node settles to a harness (or to `EnumComparisonNotRendered` with a named
capability) instead of `OperationNotRendered`; `generate_routed` and `generate_routed_native` pass no
context and every other family's records and identities stay byte-identical (tested).

Tagged and passing: AC-1, AC-2, AC-3 (carrier admission, minted identity, generator-mint through
the generation path, reorder, zero-member, u32-overflow and missing-declaration capabilities),
AC-4 (assertion shape; an exhausted meter is Falsified under installed Kani 0.68.0, `--solver
cadical`, unwind 1), AC-5 and AC-6 (healthy harnesses Verified, every mutant Falsified with
playback, in the ignored Kani lane of `tests/it/enum_scalar_harness.rs`; measurements in
`docs/qualification/ir262-enum-scalar-kani-measurements.md`), AC-7 (one cover, last; the
unsatisfied-cover, missing-cover-summary and unwind classifications are bound to the enum
harness' cover count, and a timeout and a missing verdict are driven through the runner against
the enum harness with their terminal values), AC-8, AC-9, AC-10 (one `i64` index per operand
with declaration, member list and bounds in the identity; `decode_enum_falsification` decodes a
valid pair to its declared members and refuses out-of-range, missing, extra, foreign-harness,
narrow and bound-mismatched bindings with typed causes) and the terminal-outcome part of AC-11
(`run_terminal_value` over the enum evidence).

Tractability: enum has no refusal row in the IR-1526 table because it verifies (seconds, about
0.3 GB healthy); that table lists only shapes that do not.

QSL refuses the ill-typed sources (an ordering over an unordered enum, operands of different
enums) before Contract IR, so no ill-typed enum node reaches CG from source; the CG refusal for
them is exercised on the carrier (`check_typed`) and a QSL test asserts the source refusal.

Not done, with owner:

- Owner replay of an enum counterexample under AC-5 and AC-10's "replays as the declaration's
  member": `qsl_replay::ScalarOperation` has only `Add`, `Subtract`, `Multiply` and `Negate`, so
  the owner replay route cannot take an enum comparison. Owner: quire-spec-language `qsl-replay`
  (a comparison operation over declared members). The counterexample is decoded to members and
  replayed against the declared operator in CG only.
- AC-11 tool versions and target: `KaniExecutionEvidence` carries the argv, unwind, solver,
  ceilings, memory, checks, exit code and outcome for every family, but no tool version or
  target field. Owner: the CG runner evidence (all families, not an enum row).
