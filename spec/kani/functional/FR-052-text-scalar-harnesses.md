---
id: FR-052
title: "Generate bounded text scalar comparison harnesses"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-014
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-016
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-025
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-028
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-029
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-039
    type: depends_on
  - target: ix://agent-ix/quire-contract-ir/FR-038
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-035
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-046
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-049
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-050
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-051
    type: references
  - target: ix://agent-ix/quire-specification/FR-196
    type: references
---
# FR-052: Generate bounded text scalar comparison harnesses

## Description

When an admitted request selects a reader-admitted `text` scalar comparison
(`quire.op.text.eq`, `.ne`, `.lt`, `.le`, `.gt`, `.ge`) over two parameters each
typed by a `text_bounds` domain, and the planned prerequisite below is available,
CG SHALL generate a bounded Kani harness that proves the generated text oracle of
[FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md) agrees with an
independent native expression of the same comparison over every admitted symbolic
operand pair. This is the text row of the IR-269 scalar renderer that TC-030's Text
scalar row settles today as `OperationNotRendered`, and the row that IR-261 waits
on. It is a planned row, not a new IR schema, a new oracle or a claim of current
Rust support.

Prerequisite P1 is the quire-exact text carrier (ticket IR-1445): a symbolic exact
`Text` operand and a native expression over it, with the tractability contract of
the IR-1436 design review (D4): `Copy`, no allocation, no `chars().count()`, no
SWAR and no iterator loop, counting and ordering as byte loops of at most the
carrier capacity. It is not delivered. Every criterion that describes a supported
harness is conditional on P1. Two facts about today's owner make P1 necessary and
are not assumed away. The only public way to obtain an owner `Text` is the metered
`admit_text`, so a harness built on today's API would run an admission per operand
as setup. `compare_text` prepares both operands again from their original payloads
(it does not read the retained sequences), so one comparison costs at least the
preparation of two admissions.

The operand bound mechanism needs no further prerequisite: a bounded Text is the
admitted `text_bounds` form (`min`, `max`, `text_profile`), and the Contract IR
shape (IR-1263) and literal membership (IR-1260) tickets are recorded Done. A
`text` parameter is unbounded by form, so under a bounds-required profile (Contract
IR FR-038) an unbounded text operand is `requires_bound` upstream. FR-014 reads no
text bound for a comparison, because the comparison descriptor carries none, so it
neither refuses an unbounded text operand nor reads its bound. This row reads the
operand bound itself and refuses defensively (AC-4).

A comparison returns `Outcome<bool>`, takes no `TextType` and has no result bound.
Only the two operands carry bounds.

Three neighbours are outside this requirement. `quire.op.text.size` is refused by
FR-014 as `OperationNotDerivable`, so no oracle exists to prove. Text admission
(`admit_text` into a declared `TextType`) is a caller-ingress operation owned by
[FR-035](../../oracle/functional/FR-035-caller-text-admission.md); no checked
`quire.op.numeric.convert` node represents it, so it selects no harness here and
this row adds no admission claim. A text literal operand and an operand that is not
a parameter are outside this row (AC-1).

### Bound and ceiling

Three quantities are kept apart. The declared bound is the `text_bounds` `max`,
a profile length: Unicode scalars for `unicode-scalars`, `nfc`, `nfd`, `nfkc` and
`nfkd`, UTF-8 bytes for `binary-utf8`. The payload-byte bound P is the largest
byte length of any payload that the owner admits under that declared type. The
carrier capacity C is the byte length the carrier can hold, a constant the
carrier (P1) states and the harness identity records.

The owner admits a payload by its retained sequence, so P is not the declared
`max`. For `binary-utf8` no normalization runs, retained equals payload, and
P = `max`. For `unicode-scalars` retained equals payload and a scalar is at most
four bytes, so P = 4 x `max` (checked arithmetic). For `nfc`, `nfd`, `nfkc` and
`nfkd` the payload can differ in length from the retained sequence in either
direction, and the bound on payload scalars per retained scalar comes from the
pinned Unicode 17.0.0 tables. This requirement asserts no multiplier. The carrier
(P1) must state P for these four profiles with its derivation; until it does, they
are `unsupported`.

The domain is the whole admitted set. It is therefore supported only when P is not
greater than C, so that every admitted payload fits. A narrower capacity would
prove a smaller domain, and generation never clips (AC-3).

No capacity is claimed to fit any ceiling. The memory and wall-clock ceilings are
the request's (FR-028). The bound published for the caller-Text admission proof was
1 GiB, 600 s and `--unwind 8` (TC-050), and it was exceeded: one concrete admission
exhausted 1 GiB and, at a 4 GiB diagnostic ceiling, still ended `MemoryExhausted`
(IR-796), and the IR-1436 design review records 2.6 GiB after a count rewrite. A
byte loop of C iterations also needs an unwind bound above C. A capacity is
supported only for the runs the carrier records within the request's ceilings; any
other run is `inconclusive` under FR-028 and never `Verified`.

## Inputs

- A requested text comparison node, its operator, its two parameter operands and
  the `text_bounds` (`min`, `max`, profile) of each operand, as admitted by FR-014
  and the strict Contract IR reader.
- The text carrier (P1), its capacity C and, for the four normalizing profiles, its
  stated payload-byte bound.
- The proof subject, backend and ceilings of
  [FR-015](./FR-015-bounded-kani-obligations.md),
  [FR-025](./FR-025-generated-subject-abi.md) and
  [FR-028](./FR-028-bounded-proof-ceilings.md); per-item accounting is
  [FR-039](../../core/functional/FR-039-contract-package-consumption.md)'s.

## Outputs

A supported item has its harness, symbolic-input correspondence and
source-associated proof record. A refused item has its named disposition and no
harness. Execution and terminal outcomes remain
[FR-017](./FR-017-kani-execution-evidence.md) and
[FR-029](./FR-029-run-outcome-terminal-record.md)'s; counterexample replay remains
[FR-016](../../replay/functional/FR-016-witness-native-replay.md)'s.

## Behavior

When the comparison is representable, the harness SHALL draw each operand as a
payload through the carrier: a byte length in `0..=C` and C payload bytes, where
every byte at or after the length is zero, the prefix is valid UTF-8, and the
retained sequence of that payload is admitted by the operand's declared
`TextType` (`TextType::admits` over the profile's retained sequence). Every
admitted payload is reachable exactly once, including payloads that differ only in
normal form (`e` followed by U+0301 and U+00E9 under `nfc`), and no other byte
string is reachable. Two distinct parameters are drawn independently, and one
parameter referenced by both operand slots is drawn once. The constraint applies
before evaluation and does not depend on the outcome. Because P is not greater than
C, no admitted payload is outside this encoding.

When evaluating, the harness SHALL compare the FR-014 generated oracle with a
native expression that is independent of that oracle (not a second call of the same
function), from matching entry Meter state and limits. The comparison covers the
full typed outcome, ordered charges, the denied next charge and consumed counters.
The native expression decides the operator with `==`, `!=`, `<`, `<=`, `>` and
`>=` over the byte-lexicographic order of the two retained sequences. UTF-8 byte
order equals Unicode scalar order, which is the order the owner uses for every
profile other than `binary-utf8` (whose order is the byte order). The retained
sequence of a normalizing profile is produced by the owner's public
`NormalizationForm::apply` stream; no Unicode table is reimplemented, so agreement
there checks the oracle's use of normalization and not the Unicode data.

The operands must share a profile. If the two operands' `text_bounds` name
different profiles, then CG SHALL refuse the item with the owner's ill-typed cause
`DistinctTextProfiles`, source association and no harness. At run time
`compare_text` returns `Err(IllTyped)` with that cause and consumes nothing, which
the oracle surfaces as `OracleStop::IllTyped`. The two operands' `min` and `max`
may differ.

The charges are those of the owner's `prepare` and are expected in this order.
`text.input-bytes` (the sum of the two payload byte lengths, and two value
occurrences), then `text.decode-scalars` (the sum of the two payload scalar counts).
For `nfc`, `nfd`, `nfkc` and `nfkd` then `text.normalize-input`, then one
`text.normalize-output` before each emitted scalar across both operands, left then
right, with a running emitted count. Last `text.result-retain`. The non-normalizing
profiles charge no `text.normalize-*` point. A comparison applies no length bound,
so it never refuses a length.

The typed outcomes the owner produces are these. `Completed` with the Boolean.
`Incomplete` at a charge point, `text.input-bytes`, `text.decode-scalars`,
`text.normalize-input`, `text.normalize-output` or `text.result-retain`, with its
limit kind. A comparison produces no `Undefined` and no `Refused`, so neither is a
drawn control. None is assumed away, and a stopped outcome never becomes a
completed value.

When asserting, the harness SHALL emit exactly one final completion cover after all
assertions. No assumption depends on the operation's outcome.

When the carrier, a finite `text_bounds` operand bound, a payload-byte bound within
C, the resolved operand form or the native expression is unavailable, the item is
refused with the dispositions below and no harness. A text leaf inside a composite
remains FR-015's `ShadowFamilyNotBuilt`.

### Symbolic ABI and witness roundtrip (planned)

When expanding a logical text operand, CG SHALL persist physical `i64` bindings for
its one real parameter in this order: the byte length, then the payload bytes packed
eight to a word, little-endian, in position order, for exactly C bytes. The
bindings are grouped to the original parameter identity, role, the complete
`text_bounds` domain and C. Logical parameter order remains ascending by parameter
identifier; Boolean and integer ordering is unchanged. The carrier (P1) owns the
exact binding facts; these are required conceptual facts, not a claim that a current
Rust field carries them.

When decoding retained playback, CG SHALL validate each physical entry in persisted
call order with the existing I64 width, little-endian byte and comment checks and the
byte and value-count limits (each entry of a text parameter counts) before grouping.
It SHALL then check that the length is within `0..=C`, that every byte at or after
the length is zero, that the prefix is valid UTF-8 through the public
`TextPayload::from_utf8`, and that the retained sequence is admitted by the original
`TextType`. A missing, duplicate, foreign, role-swapped or domain-conflicting schema
refuses with a typed cause, malformed entries retain typed decode causes, and no
replay is attempted for these evidence failures.

When submitting a validated parameter to public replay, CG SHALL construct one
`qsl_replay::WitnessValue::Text` holding the payload's scalars per parameter. If the
public route is unavailable the item is unsupported with no harness. CG introduces no
transcript grammar or occurrence key.

## Acceptance Criteria

All supported-harness criteria are conditional on P1 (IR-1445).

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-052-AC-1 | Each admitted `quire.op.text.eq`, `.ne`, `.lt`, `.le`, `.gt` and `.ge` node over two parameters typed by `text_bounds` selects a harness naming the operation identity, both operand forms and their bounds from the admitted node; the oracle is FR-014's and the expected side is an independent native expression, not a second call of the oracle. `quire.op.text.size` selects no harness and keeps FR-014's `OperationNotDerivable`. A text admission claim selects no harness here and stays FR-035's. A text literal operand, and an operand that is not a parameter, is accounted `unsupported` naming `OperandNotSupported` and the operand node, with no harness. | Test |
| FR-052-AC-2 | Each operand is drawn as a byte length in `0..=C` and C payload bytes, zero from the length on, valid UTF-8, and admitted by the declared `TextType`. Every admitted payload is reachable exactly once, including normal-form variants of one text, and an independent host enumeration of every byte string up to a stated small length, filtered by `std::str::from_utf8` and the owner `TextType::admits` over the profile's retained sequence, matches the encoding. Clipping the length at `C - 1`, accepting an invalid UTF-8 prefix, accepting nonzero bytes past the length (two encodings for one text), admitting a payload the declared type refuses, rejecting a payload it admits, and swapping the scalar-count and byte-count length measures between `binary-utf8` and the other profiles each fail the correspondence check (QSpec FR-196-AC-5). | Test |
| FR-052-AC-3 | The payload-byte bound P is `max` for `binary-utf8`, 4 x `max` (checked arithmetic) for `unicode-scalars`, and the carrier's stated bound for `nfc`, `nfd`, `nfkc` and `nfkd`. An operand with P not greater than C is supported. An operand with P greater than C, a `max` or product beyond the carrier's length width or `u64`, or a normalizing profile whose bound the carrier does not state, yields named `unsupported` retaining the declared `max`, the profile, P (or its absence) and C; generation never clips, narrows or substitutes a smaller domain. | Test |
| FR-052-AC-4 | A text operand typed by no `text_bounds` (a plain `text`) yields source-associated `requires-bound` naming the operand's type node with no harness. A `text_bounds` that is unreadable under FR-014-AC-16 (a missing, duplicate or unlisted member, a non-canonical or beyond-`u64` count, `min` above `max`, or an unknown profile) yields named `unsupported` retaining that cause. Operands whose profiles differ are refused with `DistinctTextProfiles` and no harness. No result bound is required, read or reported, because a comparison has none. | Test |
| FR-052-AC-5 | For every operator the oracle and native expression agree on the typed outcome, ordered charges, the denied next charge and consumed counters, at zero, exact and short budgets at each charge point in the Behavior list that the profile reaches. The reference neither runs the generated oracle twice nor bridges through Runtime. Normalizing profiles compare the running `text.normalize-output` index across both operands. | Test |
| FR-052-AC-6 | The reachable noncompletions are asserted as the agreed typed outcome and never assumed away: `Incomplete` at each charge point the profile reaches, and at run time `Err(IllTyped)` with `DistinctTextProfiles` as a non-`Verified` outcome retaining its cause. No `Undefined` or `Refused` control is drawn, because the owner produces none. No cause is relabelled and a stopped outcome is never proved completed. The generated harness contains no assumption that depends on the outcome (inspected, as FR-046-AC-4). | Test |
| FR-052-AC-7 | When P1 (IR-1445) is unavailable the item remains settled as TC-030's Text scalar row states, `OperationNotRendered` (`NoRenderer`), carrying no prerequisite name, with no harness and no Verified claim. An unsupported operand form, an absent native expression and a text leaf of a composite each keep their own named disposition, with total per-item accounting under FR-039 and an unchanged supported sibling. | Test |
| FR-052-AC-8 | A healthy text comparison receives `Verified` only from an actual successful proof with all checks and exactly one final completion cover satisfied. Unsatisfied cover, failed unwind, timeout, memory exhaustion or missing verdict cannot yield `Verified`, and FR-017 and FR-029 classifications are retained. A capacity whose recorded run exceeds the request's memory or wall-clock ceiling settles `inconclusive` under FR-028, and no capacity is reported verified by a run at a larger ceiling than the request's. | Test |
| FR-052-AC-9 | Each mutant below fails with actual solver playback of an in-domain input, caught by the listed operators over a control domain that contains its distinguishing point. (a) `eq` for `ne` (and the converse): `eq` and `ne`. (b) `lt` for `le`, `gt` for `ge` (and converses): the matching ordering operator, equal operands. (c) Operand swap: `lt`, `le`, `gt` and `ge` over distinct operands; `eq` and `ne` swaps are equivalent and are not claimed. (d) Normalization skipped, or a wrong profile in the oracle: all six operators over `e` followed by U+0301 and U+00E9 under `nfc`, in both operand orders. (e) Shortlex order instead of lexicographic order: the four ordering operators over `b` against `aa`. (f) UTF-16 code unit order instead of scalar order: the four ordering operators over U+FF61 against U+10000. (g) A prefix compare that ignores a trailing scalar: all six operators over `a` against `ab`. (h) A dropped or reordered charge, including one `text.normalize-output` before a scalar: every operator, a normalizing profile and a nonempty operand for the normalize points. (i) A wrong charge amount, such as the byte sum taken from the left operand only or the emitted count reset for the right operand: every operator over two nonempty operands, the reset needing a normalizing profile. (j) A stopped outcome reported completed: every operator. | Test |
| FR-052-AC-10 | Counterexample decoding validates every physical I64 entry of a text parameter (width, little-endian bytes, comment, order, schema role, byte and value-count limits), then the length, the zero padding, the UTF-8 prefix through `TextPayload::from_utf8` and the retained sequence's admission by the original `TextType`, before grouping. Grouping yields one `WitnessValue::Text` per parameter through the public replay route for that exact declaration. Missing, duplicate, foreign, role-swapped, domain-conflicting and malformed schemas or entries, a length above C, nonzero padding, invalid UTF-8 and a payload the type refuses are refused with typed causes and no replay. An unavailable replay capability is reported without claiming acceptance. | Test |
| FR-052-AC-11 | Run evidence records the operation and source association, both operands' `text_bounds`, the payload-byte bound P and the capacity C per operand, correspondence, ordered physical bindings and their logical grouping, subject and full ordered backend arguments, the unwind bound, Kani/Rust/CBMC/solver versions, target, dependency cohort including the carrier, limits, the observed peak memory, raw checks, cover and playback, exit and typed terminal outcome. No-run, refusal, cover failure, resource exhaustion and semantic falsification remain distinguishable; preparation alone earns no proof result. | Test |

## Dependencies

- [FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md) owns text
  comparison oracle semantics and the derivable operation list; its oracle
  agreement is not this proof.
- The text carrier (IR-1445) is prerequisite P1. The Contract IR `text_bounds`
  shape (IR-1263) and literal membership (IR-1260) are recorded Done; the IR-1436
  design review (D4) fixes the carrier contract and gates the memory evidence.
- [FR-035](../../oracle/functional/FR-035-caller-text-admission.md) owns caller
  text admission and the TC-050 proof-cost record (IR-494, IR-837, IR-796). Its
  planned edits (open PRs 381 and 413) are not touched here.
- [FR-046](./FR-046-rational-function-predicate-harnesses.md),
  [FR-049](./FR-049-enum-scalar-harnesses.md),
  [FR-050](./FR-050-quantity-scalar-harnesses.md),
  [FR-051](./FR-051-decimal-scalar-harnesses.md) and QSpec FR-196 are the
  precedent and the finite-encoding obligation this row follows.
- TC-030's Text scalar row owns today's `OperationNotRendered` settlement; this
  row supersedes it for the six text comparisons only when delivered.
- [FR-015](./FR-015-bounded-kani-obligations.md), [FR-025](./FR-025-generated-subject-abi.md)
  and [FR-028](./FR-028-bounded-proof-ceilings.md) own the shared refusals and ABI row.
- [TC-066](../matrix/TC-066-text-scalar-harnesses.md) defines the planned
  verification.

## Status

PLANNED (IR-1441). This specification supplies no implementation or
generated-proof credit. All eleven criteria remain planned and untagged until P1 is
delivered with the IR-269 text harness. The public Text replay value exists in
`qsl_replay`; the CG decoder grouping and the carrier's physical bindings do not.
Oracle agreement under FR-014 is not Kani credit. No capacity C is known to verify
within the request's ceilings, and the four normalizing profiles have no stated
payload-byte bound; both are open obligations of P1, not waived gaps.
