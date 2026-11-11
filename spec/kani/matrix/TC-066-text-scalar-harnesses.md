---
id: TC-066
title: "Verify bounded text scalar comparison harnesses"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-052
    type: verifies
---
# TC-066: Verify bounded text scalar comparison harnesses

## Description

Verify [FR-052](../functional/FR-052-text-scalar-harnesses.md) through admitted
text comparison packages (conditional on the quire-exact text carrier, IR-1445, the
non-metered `Text` constructor, IR-1499, and for normalizing profiles the payload-byte
bound, IR-1500, and the loop-free normalization, IR-1501, none of which is delivered), public CG generation, installed proof execution and owner
replay. This case is planned; FR-014 oracle agreement does not establish Kani
acceptance, and the caller-Text admission proof of TC-050 is a separate case.
Execute generated proofs only with the authorized runner and budget.

## Test Procedure

1. Admit packages covering `text.eq`, `.ne`, `.lt`, `.le`, `.gt` and `.ge`, each over
   two parameters typed by `text_bounds`. Request each harness and assert the
   operation, operand forms and operand bounds are the admitted node's, that no
   result bound is read, and that the expected side is not a second call of the
   generated oracle. Request `text.size` and assert no harness and FR-014's
   `OperationNotDerivable`. Request a caller text admission and assert no harness
   from this row. Request a text literal operand and assert `unsupported` naming
   `TextLiteralOperand`, and a non-parameter operand and assert `unsupported` naming
   `OperandNotSupported`, each with the node. Present a `numeric.convert` node with a
   text result and assert no harness and its existing disposition (AC-1).
2. Independently enumerate every byte string up to a stated small length with
   `std::str::from_utf8` and the owner `TextType::admits` over the profile's
   retained sequence (host-side, with the owner `NormalizationForm::apply` as the
   normalization reference), for one profile of each kind (`binary-utf8`, `unicode-scalars`,
   `nfc`), including `e` followed by U+0301 and U+00E9 under `nfc`. Compare with the
   carrier encoding for soundness, injectivity and completeness. Mutate by clipping
   the length at `C - 1`, accepting an invalid UTF-8 prefix, accepting nonzero bytes
   past the length, admitting a payload the type refuses, rejecting one it admits, and
   swapping the byte-count and scalar-count measures (over U+00E9 under `Text[0,1]`,
   admitted by the scalar measure and refused by the byte measure; a `Text[0,2]`
   enumeration cannot distinguish them); each must fail the correspondence check. Also enumerate the alphabet U+0061, U+00E9, U+FF61 and
   U+10000 and assert byte order equals scalar order, the property the native
   expression relies on. Inspect the FR-025 Text row: the length binding, then the
   packed payload words, in ascending logical order, with Boolean and integer order
   unchanged (AC-2).
3. Compute P for `binary-utf8` (`max`) and `unicode-scalars` (4 x `max`) and assert it
   against an independent host computation. Request an operand with P greater than C
   by one byte, a `max` whose product overflows `u64`, and a normalizing profile with
   no IR-1500 payload-byte bound. Assert named `unsupported` retaining `max`, profile,
   P (or its absence) and C, with no harness and no clipping. Assert P equal to C is
   supported (AC-3).
4. Request a plain `text` operand, a `text_bounds` with a missing member, a duplicate
   member, a non-canonical count, a count beyond `u64`, `min` above `max` and an
   unknown profile, and two operands naming different profiles. Assert
   `requires-bound`, the retained unreadable-bound cause (FR-014-AC-16 for members, CG's own reader for counts, `min` above `max` and the profile) and `DistinctTextProfiles`
   respectively, each with no harness, and that no result bound is requested (AC-4).
5. Compare the generated oracle with the independent native expression from matching
   entry Meter state and limits: outcome, ordered charges, denied next charge and
   counters at zero, exact and short budgets at `text.input-bytes`,
   `text.decode-scalars`, `text.normalize-input`, `text.normalize-output` and
   `text.result-retain`. Run `unicode-scalars` or `binary-utf8` to confirm no
   `text.normalize-*` point is charged, and `nfc` with two nonempty operands to
   confirm the emitted index runs across both operands (AC-5).
6. Exercise each reachable noncompletion: `Incomplete` at each charge point the
   profile reaches. Assert no `Undefined` or `Refused` control exists, that no
   harness control for `DistinctTextProfiles` exists (it is refused at generation,
   step 4), and inspect the harness for no outcome-based assume (AC-6).
7. Remove the carrier and assert `OperationNotRendered` (`NoRenderer`, no
   prerequisite name). Exercise an unsupported operand form, an absent native
   expression and a text composite leaf beside a supported sibling and assert total
   accounting and the leaf's `ShadowFamilyNotBuilt` (AC-7).
8. Run the healthy harness through the approved bounded production path with its
   complete ordered options and an unwind above C. Verify all checks and exactly one
   final cover. Separately force an unsatisfied cover, a failed unwind (an unwind
   at or below C), a timeout, a memory ceiling below the observed peak and a missing
   verdict, and assert none is `Verified` and the memory case is `inconclusive` with
   its reason (AC-8).
9. Run the AC-9 mutants one at a time, each over a control domain stating its
   distinguishing point and only against the operators FR-052-AC-9 pairs with it:
   (a) eq/ne; (b) lt/le, gt/ge at equal operands; (c) swap in the four orderings over
   distinct operands, with no swap claim for `eq` or `ne`; (d) normalization skipped (the
   operands' `TextType` built as `unicode-scalars` for a declared `nfc`) over `e` plus
   U+0301 against U+00E9, both orders, all six operators, with no `nfc`/`nfkc` swap
   claimed; (e) shortlex over `b` against `aa`; (f) UTF-16 order over U+FF61
   against U+10000 under `unicode-scalars` or `binary-utf8` (and `nfc`/`nfd`), never
   `nfkc`/`nfkd`; (g) prefix compare over `a` against `ab`; (h) dropped or
   reordered charge; (i) wrong charge amount; (j) stopped reported completed. Each
   must fail with actual in-domain playback (AC-9).
10. Perturb retained witnesses (missing, duplicate, foreign, role-swapped, malformed
    width or comment, length above C, nonzero padding, invalid UTF-8, a payload the
    type refuses, over entry-count or byte bound; for a two-parameter harness set the
    value-count limit to the exact physical entry count, which must be accepted, and
    to one fewer, which must refuse before reading the last entry; apply the same
    at/under pair to the byte limit) and assert typed refusal with no replay. For a
    valid witness assert one `WitnessValue::Text` per parameter reaches public replay
    for that declaration. Record an unavailable route as an open dependency (AC-10).
11. Inspect the run record for operation and source association, bounds, P, the attained
    length A, the boundary lengths L and L_b, and C, correspondence, ordered physical bindings, argv, unwind bound, tool versions,
    target, dependency cohort including the carrier, limits, observed peak memory,
    checks, cover, playback, exit and terminal classification (AC-11).
12. On the host, run the carrier's `cfg(kani)` replacement of `compare_text` and the
    shipped `compare_text` over the step 2 enumeration for all six operators, at zero,
    exact and short budgets, and assert equal outcome, ordered charges, denied next
    charge and counters. Repeat over the step 14 family. For a payload of C + 1 bytes assert the carrier's encoding
    refusal, not agreement. Confirm the carrier makes its replacement callable from
    the host build (P1). Mutate the replacement's ordering, charge sequence and
    outcome; for `binary-utf8`, `unicode-scalars`, `nfd` and `nfkd` bound the ordering loop at L - 1 or earlier
    (which must fail the final-byte pair); for `nfc` and `nfkc` bound a payload-side
    decode or normalize-input loop at L - 1 or earlier (which must fail the final-byte
    pair under `eq`; an ordering loop bounded at L - 1 is not claimed there); and compute a charge from a truncated
    length; each must fail the differential (AC-12). The result covers the enumerated and boundary payloads, not every payload
    up to C.
13. Compute A on the host for `binary-utf8`, `unicode-scalars`, `nfd`, `nfkd`, `nfc`
    and `nfkc` at `max` of 0, 1, 2 and 8 by the quire-exact FR-400-AC-6 construction
    (for `nfc` and `nfkc`: the first stable scalar, in scalar order, with the most
    `nfd` or `nfkd` UTF-8 bytes, its decomposition repeated `max` times, admitted by
    `Text[max, max]`; for the other four the payload of P bytes). Assert A is at most
    P for every profile, A equals the independently computed P for the four profiles
    other than `nfc` and `nfkc`, A is below P for `nfc` and `nfkc` at `max` of 1, A
    equals `max` times the independently enumerated maximum decomposition length for
    `nfc` and `nfkc` only, and at `max` of 0 the payload is empty with A and P zero.
    Run the mutants: the family built at P for `nfc` at `max` of 1 (the unit plus one
    appended four-byte self-normal scalar, refused by `Text[1, 1]`), the stability
    filter dropped for `nfkc` at `max` of 1 (U+FDFA, refused), nfkd taken for `nfc` at
    `max` of 1 (U+FDFA, refused), the unit not repeated at `max` of 2 under `nfc`
    (one retained scalar, below `min` 2, refused), and the first multi-scalar
    decomposition (U+00C0) instead of the longest at `max` of 1 under `nfc` (admitted,
    fails the equality with the enumerated maximum). Each must fail (AC-13). Assert
    support is unchanged: an item with P above C stays `unsupported` (step 3) even
    when A is at most C.
14. Build the long-payload family on the host for each supported profile over
    `Text[0, max]` at L = A and L_b (AC-14): confirm every payload of every pair is
    admitted by `TextType::admits`, that L_b is L - 1 for `binary-utf8`,
    `unicode-scalars`, `nfd` and `nfkd`, and that for `nfc` and `nfkc` the L_b unit is
    the first stable scalar in scalar order with the longest shorter decomposition
    that has an admitted final-byte variant. Assert the final-byte pair is found at L
    and at L_b for every supported profile, the first-byte pair is found at L and at
    L_b for `binary-utf8`, `unicode-scalars`, `nfd` and `nfkd`, the length pair is
    built, and an absent first-byte pair for `nfc` or `nfkc` is recorded as absent.
    At `max` of 0 assert the family is the empty payload only. For `binary-utf8` at `max` of 1 assert L_b is 0, the empty payload is admitted by `Text[0, 1]`, only the length pair is built at L_b, and the final-byte and first-byte pairs are required at L only (a search that demands them at the empty L_b is a defect and fails the step). Run the mutants: L_b as
    L - 1 for `nfc` at `max` of 1 (11 bytes, no admitted payload), the family type
    `Text[max, max]` for `binary-utf8` at `max` of 2 (the 1-byte payload is refused),
    the L_b unit chosen without the final-byte-variant condition for `nfc` at `max` of
    1 (U+0CCB, whose final-byte search finds none), and a first-byte search that
    returns none for `unicode-scalars` at `max` of 1. Each must fail (AC-14).

## Expected Results

Healthy comparisons verify with a satisfied cover; every mutant and refusal retains
its own classification and receives no proof credit. Until the carrier and the
generated-proof path run, IR-261 remains unaccepted regardless of oracle
agreement.

## Status

PLANNED: all fourteen FR-052 criteria are unimplemented and untagged. No missing
carrier capability (IR-1445), `Text` constructor (IR-1499), stated payload-byte bound (IR-1500), loop-free normalization (IR-1501), replay route or generated-proof result
is waived. No capacity C is known to verify within a request's ceilings; TC-050
recorded a 1 GiB bound exceeded and the IR-1436 review records 2.6 GiB.
