---
id: SR-17940486709564847047591
title: "IR-1441 PR #767 FR-052 mutant-control strength analysis"
type: SpecReview
analysis: criterion-strength
date: 2026-11-10
scope: "agent-ix/quire-contract-codegen; PR #767 @ 2362dae1; IR-1441; spec/kani/functional/FR-052-text-scalar-harnesses.md, spec/kani/matrix/TC-066-text-scalar-harnesses.md, spec/kani/functional/FR-025-generated-subject-abi.md, spec/kani/functional/FR-028-bounded-proof-ceilings.md, spec/kani/matrix/TC-036-generated-subject-abi.md, spec/routed/matrix/TC-030-capability-settlement.md, spec/kani/matrix/tests.md, spec/spec.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-052
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-066
    type: reviews
---

## Summary

Ticket: IR-1441. Spec-only PR #767 (head 2362dae1) adding FR-052 and TC-066, the planned Kani row for the six text comparisons with a payload-byte bound P and carrier capacity C, with edits to FR-025 (Text ABI row, AC-7, new AC-16), FR-028, TC-036 step 16, TC-030 Text row, spec/kani/matrix/tests.md and spec/spec.md. Reviewed against origin/main cf10ec78 (includes FR-049/050/051) and quire-exact 5db2d5e3 (CG Cargo.lock pin; src/text.rs and accounting.rs unchanged at origin/main 4fe7df6), CG src/oracle/scalar/{derive,lower,render}.rs, src/core/ir/scalar_bounds.rs, src/routed/capability.rs, QSpec FR-196, Linear IR-1445, IR-261 and the IR-1436 design-review comment.

## Verdict

CHANGES REQUIRED. Jev (typesafe.ai System One) is not installed on this host; this is a manual reading of each mutant against quire-exact text.rs, checked with Python unicodedata. Distinguishing points hold for (a), (b) at equal operands, (c) over distinct operands with the eq/ne swap correctly excluded as equivalent, (d) 'normalization skipped' (e+U+0301 vs U+00E9 differ bytewise), (e) b vs aa (lex b>aa, shortlex b<aa), (g) a vs ab, (h) and (i) via the compared charge sequence and running NormalizedScalars amount, and (j). Two mutants have an admissible control domain where they cannot fail, and one AC-2 mutant depends on an unstated length.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Mutant (d) 'a wrong profile in the oracle' is equivalent on its stated point for nfc->nfkc: NFKC(e+U+0301) = NFKC(U+00E9) = U+00E9, one emitted scalar per operand, so outcome and every charge match NFC and the mutant cannot fail. (nfc->nfd is caught only by the normalize-output count, not by any operator.) Also the generated oracle has no profile of its own: compare_text reads left.text_type.profile, so the mutation's locus is the carrier's TextType, not the oracle. Add a compatibility distinguishing point (e.g. U+FB01 vs 'fi' under nfc) and name where the profile is mutated. | spec/kani/functional/FR-052-text-scalar-harnesses.md:237 |
| FND-002 | medium | Mutant (f) (UTF-16 code-unit order) states no profile. Under nfkc or nfkd U+FF61 normalizes to U+3002, and U+3002 vs U+10000 has the same order in UTF-16 (0x3002 < 0xD800) and in scalars, so the mutant cannot fail there. Pin (f)'s control to unicode-scalars, binary-utf8, nfc or nfd, or choose a pair stable under compatibility normalization (e.g. U+E000 vs U+10000). | spec/kani/functional/FR-052-text-scalar-harnesses.md:237 |
| FND-003 | low | The 'swapping the scalar-count and byte-count length measures' mutant fails the correspondence check only if the host enumeration's 'stated small length' reaches a multi-byte scalar with a declared max below its byte length (e.g. U+00E9 under Text[0,1]). At a stated length of 1 byte both measures agree on every admitted string. Require the enumeration length to be at least 2 and to include a max where the measures differ. | spec/kani/functional/FR-052-text-scalar-harnesses.md:230 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-052-AC-9 | examined | spec/kani/functional/FR-052-text-scalar-harnesses.md:237 | Each mutant below fails with actual solver playback of an in-domain input, caught by the listed operators over a control domain that contains its distinguishing point. (a) `eq` for `ne` (and the converse): `eq` and `ne`. (b) `lt` for `le`, `gt` for `ge` (and converses): the matching ordering operator, equal operands. (c) Operand swap: `lt`, `le`, `gt` and `ge` over distinct operands; `eq` and `ne` swaps are equivalent and are not claimed. (d) Normalization skipped, or a wrong profile in the oracle: all six operators over `e` followed by U+0301 and U+00E9 under `nfc`, in both operand orders.... |
| TC-066-step-9 | examined | spec/kani/matrix/TC-066-text-scalar-harnesses.md:76 | 9. Run the AC-9 mutants one at a time, each over a control domain stating its distinguishing point and only against the operators FR-052-AC-9 pairs with it: (a) eq/ne; (b) lt/le, gt/ge at equal operands; (c) swap in the four orderings over distinct operands, with no swap claim for `eq` or `ne`; (d) normalization skipped or wrong profile over `e` plus U+0301 against U+00E9 under `nfc`, both orders, all six operators; (e) shortlex over `b` against `aa`; (f) UTF-16 order over U+FF61 against U+10000; (g) prefix compare over `a` against `ab`; (h) dropped or reordered charge; (i) wrong charge amo... |
| FR-052-AC-2 | examined | spec/kani/functional/FR-052-text-scalar-harnesses.md:230 | Each operand is drawn as a byte length in `0..=C` and C payload bytes, zero from the length on, valid UTF-8, and admitted by the declared `TextType`. Every admitted payload is reachable exactly once, including normal-form variants of one text, and an independent host enumeration of every byte string up to a stated small length, filtered by `std::str::from_utf8` and the owner `TextType::admits` over the profile's retained sequence, matches the encoding. Clipping the length at `C - 1`, accepting an invalid UTF-8 prefix, accepting nonzero bytes past the length (two encodings for one text), adm... |
| TC-066-step-2 | examined | spec/kani/matrix/TC-066-text-scalar-harnesses.md:30 | 2. Independently enumerate every byte string up to a stated small length with `std::str::from_utf8` and the owner `TextType::admits` over the profile's retained sequence, for one profile of each kind (`binary-utf8`, `unicode-scalars`, `nfc`), including `e` followed by U+0301 and U+00E9 under `nfc`. Compare with the carrier encoding for soundness, injectivity and completeness. Mutate by clipping the length at `C - 1`, accepting an invalid UTF-8 prefix, accepting nonzero bytes past the length, admitting a payload the type refuses, rejecting one it admits, and swapping the byte-count and scala... |
| FR-052-AC-5 | examined | spec/kani/functional/FR-052-text-scalar-harnesses.md:233 | For every operator the oracle and native expression agree on the typed outcome, ordered charges, the denied next charge and consumed counters, at zero, exact and short budgets at each charge point in the Behavior list that the profile reaches. The reference neither runs the generated oracle twice nor bridges through Runtime. Normalizing profiles compare the running `text.normalize-output` index across both operands. |
| FR-052-AC-6 | examined | spec/kani/functional/FR-052-text-scalar-harnesses.md:234 | The reachable noncompletions are asserted as the agreed typed outcome and never assumed away: `Incomplete` at each charge point the profile reaches, and at run time `Err(IllTyped)` with `DistinctTextProfiles` as a non-`Verified` outcome retaining its cause. No `Undefined` or `Refused` control is drawn, because the owner produces none. No cause is relabelled and a stopped outcome is never proved completed. The generated harness contains no assumption that depends on the outcome (inspected, as FR-046-AC-4). |
