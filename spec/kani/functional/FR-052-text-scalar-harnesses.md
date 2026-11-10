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
scalar row settles today as `OperationNotRendered`, and the specification part of
the row that IR-261 implements. It is a planned row, not a new IR schema, a new oracle or a claim of current
Rust support.

Four prerequisites are named, and none is delivered. Every criterion that
describes a supported harness is conditional on P1 and P2, and the criteria that
involve a normalizing profile are also conditional on P3 and P4.

- P1 (ticket IR-1445) is the quire-exact text carrier: a symbolic exact `Text`
  operand and a native expression over it, with the tractability contract of the
  IR-1436 design review (D4): `Copy`, no allocation, no `chars().count()`, no SWAR
  and no iterator loop, counting and ordering as byte loops of at most the carrier
  capacity C. The carrier's capacity C and the recorded peak memory per capacity,
  within the K-DR memory budget its acceptance criteria name, are within IR-1445's
  scope. It also provides the loop-free length check the proof-path draw constraint
  uses (AC-2), and a loop-free comparison that replaces the owner's `compare_text`
  under `cfg(kani)`. P1 must make that replacement callable from a host build, which
  AC-12 needs for every profile (IR-1501 requires it only for normalization);
  IR-1445's acceptance carries this requirement.
- P2 (ticket IR-1499) is a non-metered public constructor for an owner `Text`. The
  only public way to obtain an owner `Text` today is the metered `admit_text`, so a
  harness built on today's API would run an admission per operand as setup. The
  harness construction is conditional on it.
- P3 (ticket IR-1500) is the payload-byte bound P for `nfc`, `nfd`, `nfkc` and
  `nfkd`, stated by quire-exact. Every normalizing-profile criterion is conditional
  on it.
- P4 (ticket IR-1501) is a loop-free, allocation-free proof-path normalization for
  `nfc`, `nfd`, `nfkc` and `nfkd`, with a host-side differential check against the
  shipped normalization. Every normalizing-profile criterion is conditional on it,
  and absent it those profiles are `unsupported`.

The CG text ABI, the decoder and the replay grouping are IR-261's own
implementation work under this requirement, not prerequisites.

`compare_text` prepares both operands again from their original payloads (it does
not read the retained sequences), so one comparison costs at least the preparation
of two admissions. The loop-free requirement of D4 belongs to the carrier on the
proof path. The owner implementation the carrier replaces under `cfg(kani)`
allocates `String`s, orders with `chars().cmp` and streams normalization through an
iterator. The owner's public `NormalizationForm::apply`, `TextType::admits` (which
counts with a `filter().count()` D4 bans) and `TextPayload::from_utf8` are used only
host-side, for the correspondence enumeration and the decoder's admission check,
never on the proof path. Because the carrier replaces `compare_text` under
`cfg(kani)`, the proof covers that build; AC-12 ties the replacement to the shipped
`compare_text` on the host.

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
[FR-035](../../oracle/functional/FR-035-caller-text-admission.md). CG's derivation
does build a `TextAdmission` descriptor from a `quire.op.numeric.convert` node with
a text result, and capability settlement routes it to the Text family, but Contract
IR refuses such a node and its six test nodes (FR-035-AC-6). This row selects only
the six comparisons, so such a node keeps its existing disposition and never gets a
harness, and this row adds no admission claim. A text literal operand and an
operand that is not a parameter are outside this row (AC-1).

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
pinned Unicode 17.0.0 tables. This requirement asserts no multiplier. Quire-exact
(P3, IR-1500) must state P for these four profiles with its derivation; until it
does, they are `unsupported`.

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
- The text carrier (P1), its capacity C and its loop-free normalization, the
  non-metered `Text` constructor (P2) and, for the four normalizing profiles, the
  stated payload-byte bound (P3).
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
`TextType`. The proof-path constraint is the carrier's loop-free length check over
the retained sequence (P1, and P4 for a normalizing profile); the owner's
`TextType::admits` is the host-side reference it is checked against. Every
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
sequence of a normalizing profile is produced by the carrier's loop-free
normalization (P4), which no Unicode table of CG reimplements; the correspondence
check compares it host-side with the owner's `NormalizationForm::apply`, so
agreement checks the oracle's use of normalization and not the Unicode data. Until
P4 is delivered, the normalizing profiles are `unsupported`.

The operands must share a profile. If the two operands' `text_bounds` name
different profiles, then CG SHALL refuse the item with the owner's ill-typed cause
`DistinctTextProfiles`, source association and no harness. The owner's
`compare_text` returns `Err(IllTyped)` with that cause and consumes nothing, but a
generated harness cannot reach it, because the refusal precedes generation, so it
is not a harness control. The two operands' `min` and `max` may differ.

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

When the carrier, the `Text` constructor, a finite `text_bounds` operand bound, a
payload-byte bound within C, the resolved operand form or the native expression is
unavailable, the item is refused with the dispositions of AC-1, AC-3, AC-4 and AC-7 and no
harness. A text leaf inside a composite
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

All supported-harness criteria are conditional on P1 (IR-1445) and P2 (IR-1499). Every
normalizing-profile criterion is conditional on P3 (IR-1500): the payload-byte bound
of AC-3 and the normalizing-profile parts of AC-2, AC-5, AC-6, AC-9 (d), (h) and (i)
and AC-10. The same normalizing-profile parts, and AC-12 for a normalizing profile,
are also conditional on P4 (IR-1501).

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-052-AC-1 | Each admitted `quire.op.text.eq`, `.ne`, `.lt`, `.le`, `.gt` and `.ge` node over two parameters typed by `text_bounds` selects a harness naming the operation identity, both operand forms and their bounds from the admitted node; the oracle is FR-014's and the expected side is an independent native expression, not a second call of the oracle. `quire.op.text.size` selects no harness and keeps FR-014's `OperationNotDerivable`. A text admission claim selects no harness here and stays FR-035's. A text literal operand is accounted `unsupported` naming `TextLiteralOperand` and the operand node, and an operand that is neither a parameter nor a literal is accounted `unsupported` naming FR-015's `OperandNotSupported` and the node, each with no harness. A `numeric.convert` node with a text result selects no harness and keeps its existing disposition. | Test |
| FR-052-AC-2 | Each operand is drawn as a byte length in `0..=C` and C payload bytes, zero from the length on, valid UTF-8, and admitted by the declared `TextType` through the carrier's loop-free length check. Every admitted payload is reachable exactly once, including normal-form variants of one text, and an independent host enumeration of every byte string up to a stated small length, filtered by `std::str::from_utf8` and the host-side owner `TextType::admits` over the profile's retained sequence, matches the encoding. Clipping the length at `C - 1`, accepting an invalid UTF-8 prefix, accepting nonzero bytes past the length (two encodings for one text), admitting a payload the declared type refuses, rejecting a payload it admits, and swapping the scalar-count and byte-count length measures between `binary-utf8` and the other profiles (which needs a bound where the measures split, such as U+00E9, 2 bytes and 1 scalar, under `Text[0,1]`, admitted by the scalar measure and refused by the byte measure; under `Text[0,2]` with strings up to two bytes both measures admit every string and the mutant cannot fail) each fail the correspondence check (QSpec FR-196-AC-5). | Test |
| FR-052-AC-3 | The payload-byte bound P is `max` for `binary-utf8`, 4 x `max` (checked arithmetic) for `unicode-scalars`, and the bound IR-1500 states for `nfc`, `nfd`, `nfkc` and `nfkd`. An operand with P not greater than C is supported. An operand with P greater than C, a `max` or product beyond the carrier's length width or `u64`, or a normalizing profile whose bound IR-1500 does not state, yields named `unsupported` retaining the declared `max`, the profile, P (or its absence) and C; generation never clips, narrows or substitutes a smaller domain. | Test |
| FR-052-AC-4 | A text operand typed by no `text_bounds` (a plain `text`) yields source-associated `requires-bound` naming the operand's type node with no harness. A `text_bounds` that is unreadable yields named `unsupported` retaining its cause: a missing, duplicate or unlisted member (FR-014-AC-16), or a non-canonical or beyond-`u64` count, `min` above `max` or an unknown profile (each yields no bound from CG's own reader `read_text_bounds`, which returns no `TextType` for them). Operands whose profiles differ are refused with `DistinctTextProfiles` and no harness. No result bound is required, read or reported, because a comparison has none. | Test |
| FR-052-AC-5 | For every operator the oracle and native expression agree on the typed outcome, ordered charges, the denied next charge and consumed counters, at zero, exact and short budgets at each charge point in the Behavior list that the profile reaches. The reference neither runs the generated oracle twice nor bridges through Runtime. Normalizing profiles compare the running `text.normalize-output` index across both operands. | Test |
| FR-052-AC-6 | The reachable noncompletions are asserted as the agreed typed outcome and never assumed away: `Incomplete` at each charge point the profile reaches. The `DistinctTextProfiles` ill-typed result is refused at generation (AC-4) and is not a harness control. No `Undefined` or `Refused` control is drawn, because the owner produces none. No cause is relabelled and a stopped outcome is never proved completed. The generated harness contains no assumption that depends on the outcome (inspected, as FR-046-AC-4). | Test |
| FR-052-AC-7 | When P1 (IR-1445) or P2 (IR-1499) is unavailable the item remains settled as TC-030's Text scalar row states, `OperationNotRendered` (`NoRenderer`), carrying no prerequisite name, with no harness and no Verified claim. An unsupported operand form, an absent native expression and a text leaf of a composite each keep their own named disposition, with total per-item accounting under FR-039 and an unchanged supported sibling. | Test |
| FR-052-AC-8 | A healthy text comparison receives `Verified` only from an actual successful proof with all checks and exactly one final completion cover satisfied. Unsatisfied cover, failed unwind, timeout, memory exhaustion or missing verdict cannot yield `Verified`, and FR-017 and FR-029 classifications are retained. A capacity whose recorded run exceeds the request's memory or wall-clock ceiling settles `inconclusive` under FR-028, and no capacity is reported verified by a run at a larger ceiling than the request's. | Test |
| FR-052-AC-9 | Each mutant below fails with actual solver playback of an in-domain input, caught by the listed operators over a control domain that contains its distinguishing point. (a) `eq` for `ne` (and the converse): `eq` and `ne`. (b) `lt` for `le`, `gt` for `ge` (and converses): the matching ordering operator, equal operands. (c) Operand swap: `lt`, `le`, `gt` and `ge` over distinct operands; `eq` and `ne` swaps are equivalent and are not claimed. (d) Normalization skipped: in a copy of the generated harness the operands' `TextType` is built with `unicode-scalars` instead of the declared `nfc`; all six operators over `e` followed by U+0301 and U+00E9, in both operand orders (normal forms equal, raw payloads unequal). A swap between `nfc` and `nfkc` is not claimed, since that pair gives the same result and charges. (e) Shortlex order instead of lexicographic order: the four ordering operators over `b` against `aa`. (f) UTF-16 code unit order instead of scalar order: the four ordering operators over U+FF61 against U+10000 under `unicode-scalars` or `binary-utf8` (and `nfc` or `nfd`, which leave both unchanged); not under `nfkc` or `nfkd`, where U+FF61 becomes U+3002 and the two orders agree. (g) A prefix compare that ignores a trailing scalar: all six operators over `a` against `ab`. (h) A dropped or reordered charge, including one `text.normalize-output` before a scalar: every operator, a normalizing profile and a nonempty operand for the normalize points. (i) A wrong charge amount, such as the byte sum taken from the left operand only or the emitted count reset for the right operand: every operator over two nonempty operands, the reset needing a normalizing profile. (j) A stopped outcome reported completed: every operator. | Test |
| FR-052-AC-10 | Counterexample decoding validates every physical I64 entry of a text parameter (width, little-endian bytes, comment, order, schema role, byte and value-count limits), then the length, the zero padding, the UTF-8 prefix through `TextPayload::from_utf8` and the retained sequence's admission by the original `TextType`, before grouping. Grouping yields one `WitnessValue::Text` per parameter through the public replay route for that exact declaration. Missing, duplicate, foreign, role-swapped, domain-conflicting and malformed schemas or entries, a length above C, nonzero padding, invalid UTF-8 and a payload the type refuses are refused with typed causes and no replay. An unavailable replay capability is reported without claiming acceptance. | Test |
| FR-052-AC-11 | Run evidence records the operation and source association, both operands' `text_bounds`, the payload-byte bound P and the capacity C per operand, correspondence, ordered physical bindings and their logical grouping, subject and full ordered backend arguments, the unwind bound, Kani/Rust/CBMC/solver versions, target, dependency cohort including the carrier, limits, the observed peak memory, raw checks, cover and playback, exit and typed terminal outcome. No-run, refusal, cover failure, resource exhaustion and semantic falsification remain distinguishable; preparation alone earns no proof result. | Test |
| FR-052-AC-12 | On the host, the carrier's `cfg(kani)` replacement of `compare_text` and the shipped `compare_text` agree for all six operators on the typed outcome, ordered charges, the denied next charge and consumed counters at zero, exact and short budgets, over the AC-2 enumeration and over a fixed long-payload family at the capacity boundaries. For each supported profile the family uses a stated host `TextType` and that profile's largest admitted payload length L = min(P, C) and L - 1 (for `unicode-scalars`, which is supported only when 4 x `max` is at most C, L is 4 x `max` and every scalar is four bytes). Each length has pairs that differ only in the final byte position L (a continuation byte of a multibyte last scalar, or a one-byte final scalar, never a differing lead byte), only in the first byte, and only in length. The case of a payload of C + 1 bytes asserts the carrier's encoding refusal, not agreement, since the replacement cannot hold it. A replacement whose ordering loop stops at L - 1 fails the final-byte pair, one whose charge is computed from a truncated length fails every length, and one whose ordering, charge sequence or outcome otherwise differs on any enumerated or boundary pair fails the check. The check ties the proof to the shipped oracle for the enumerated and boundary payloads, not for every payload up to C. | Test |

## Dependencies

- [FR-014](../../oracle/functional/FR-014-exact-scalar-oracles.md) owns text
  comparison oracle semantics and the derivable operation list; its oracle
  agreement is not this proof.
- The text carrier (IR-1445) is prerequisite P1, the non-metered `Text`
  constructor (IR-1499) is P2, the normalizing-profile payload-byte bound
  (IR-1500) is P3, and the loop-free normalization (IR-1501) is P4. The Contract IR `text_bounds` shape (IR-1263) and literal
  membership (IR-1260) are recorded Done; the IR-1436 design review (D4) fixes the
  carrier contract and gates the memory evidence.
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
generated-proof credit. All twelve criteria remain planned and untagged until P1 is
delivered with the IR-269 text harness. P2 (IR-1499), P3 (IR-1500) and P4 (IR-1501) are likewise
undelivered. The public Text replay value exists in
`qsl_replay`; the CG decoder grouping and the carrier's physical bindings do not.
Oracle agreement under FR-014 is not Kani credit. No capacity C is known to verify
within the request's ceilings (an open IR-1445 obligation), and the four
normalizing profiles have no stated payload-byte bound (IR-1500) or loop-free normalization (IR-1501); these are open
obligations, not waived gaps.
