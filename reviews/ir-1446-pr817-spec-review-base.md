---
id: SR-17940486709564847047953
title: "IR-1446 PR #817 spec-review (base)"
type: SpecReview
analysis: base
date: 2026-11-13
scope: "agent-ix/quire-contract-codegen PR #817 at d23ae785, git diff origin/main...HEAD of code/ir-1446-cg-wording; IR-1446; spec/kani/functional/FR-052-text-scalar-harnesses.md, spec/kani/functional/FR-053-float-scalar-harnesses.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-053
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-052
    type: reviews
---

## Summary

Ticket: IR-1446. The PR rewrites the P1 paragraphs, the expected-side paragraph of FR-053 Evaluation, FR-052-AC-12 and FR-053-AC-13/AC-14. The goal is to drop "the carrier's `cfg(kani)` replacement" in favour of the host-callable native operations of quire-exact FR-404 (text) and FR-405 (float). This closes the CG-side half of IR-1446 PR #155 spec-review FND-004 (the `cfg(kani)` terms). Checked against FR-404 Behavior 6-7 and FR-405 Behavior 6-8, 15-16 and 19 on quire-exact main / PR #159.

The `cfg(kani)` wording is gone everywhere it carried meaning. A grep of both files finds `cfg(kani)` only in "not a `cfg(kani)`-only body". FR-052 now matches FR-404 Behavior 7: CG's generated harness calls the native comparison, a host-callable source with no conditional body. The removed sentence "Under `cfg(kani)` the oracle is the owner's function with the carrier's replacement substituted" was the only mechanism FR-053 stated for what the proof covers. Its replacement, "the generated harness evaluates the carrier's native operations", agrees with FR-404 Behavior 7 and FR-405 Behavior 8.

Sub-analyses: only the base review ran. The edits are wording alignment of existing statements and ACs. They add no requirement, relationship edge or domain object, so EARS, integrity, object and dependency analyses have nothing new in scope.

## Verdict

Mergeable after FND-001 is resolved. The `cfg(kani)` alignment is correct. But the same paragraph still assigns the arithmetic expected side to a "second implementation in the carrier crate (P1)", and FR-405 does not supply one, so the expected side has no owner (FND-001). FND-002 is a missing seam reference. FND-003 and FND-004 are wording.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-053 Evaluation still says the arithmetic expected side 'is a second implementation in the carrier crate (P1)', and AC-14 inspects that it shares only 'the decode' with the native operations. FR-405 (P1) supplies no second implementation. Its Behavior 6/19 reference is internal to its own harnesses, implemented as the host `f32`/`f64` operations (PR #159 `src/native_float/kani_proofs.rs`), and its `decode` is private. After this PR the expected side of CG's arithmetic harness has no supplier: CG points at P1, and P1 does not list it. Name the owner. Either CG generates the reference (then it is not 'in the carrier crate'), or it is the host float operations, which fix only nearest-even bits. Or file the second implementation as an FR-405 deliverable. Then align AC-14's shared-decode clause to that owner. | spec/kani/functional/FR-053-float-scalar-harnesses.md:173-181, spec/kani/functional/FR-053-float-scalar-harnesses.md:361 |
| FND-002 | medium | P1 still promises native operations 'at `binary32` and `binary64` under each of the six rounding modes' with the D4 tractability contract, and says only 'Until IR-1446 lands them, AC-13 cannot run'. FR-405 Behavior 15-16 gate each operation and width on its own and route a non-VERIFIED combination to IR-1526's refusal seam. PR #159 records the six arithmetic operations as table rows at both widths. FR-053 does not mention IR-1526 or the tractability table anywhere. So it never says that a P1 combination in the table-row state is refused, rather than supported or blocking. State the consumption: a combination with a table row is refused by IR-1526's seam, and only VERIFIED combinations are supported rows. | spec/kani/functional/FR-053-float-scalar-harnesses.md:60-68 |
| FND-003 | low | FR-053-AC-13 now reads 'the carrier's host-callable native operations (quire-exact FR-405) and the owner's `evaluate_ieee` and `compare_ieee` and the shipped owner functions agree'. The owner functions are named twice, so the sentence reads as three parties. Drop 'and the shipped owner functions' (or the preceding pair). | spec/kani/functional/FR-053-float-scalar-harnesses.md:360 |
| FND-004 | low | 'Replacement' is left undefined after its defining phrase ('the carrier's `cfg(kani)` replacement') was removed. It remains in FR-053:179 ('tied to the replacement'), FR-053-AC-13 ('A replacement that differs', 'the replacement's rounding step'), FR-053-AC-14 ('the replacement's rounding step') and FR-052-AC-12 ('the replacement cannot hold it', 'a replacement whose ordering loop'). FR-405 Behavior 8 defined the old term as an alias only for 'cfg(kani) replacement'. Use 'native operations' / 'native comparison' throughout, or define 'replacement' once. | spec/kani/functional/FR-053-float-scalar-harnesses.md:179, spec/kani/functional/FR-053-float-scalar-harnesses.md:360-361, spec/kani/functional/FR-052-text-scalar-harnesses.md:298 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-053 P1 | examined | spec/kani/functional/FR-053-float-scalar-harnesses.md:60-68 | IR-1446 is the supplier of the native add, sub, mul, div, sqrt and fma and of the comparisons (quire-exact FR-405). These native operations are host-callable ordinary code, one source for the host and Kani builds and not a `cfg(kani)`-only body, which AC-13 needs. Until IR-1446 lands them, AC-13 cannot run. |
| FR-053 Evaluation (expected side) | examined | spec/kani/functional/FR-053-float-scalar-harnesses.md:172-181 | The expected side of an arithmetic operation is a second implementation in the carrier crate (P1) under the node's rounding mode. It SHALL share no function with the native operations (quire-exact FR-405), apart from the decode ... The generated harness evaluates the carrier's native operations ... |
| FR-053-AC-13 | examined | spec/kani/functional/FR-053-float-scalar-harnesses.md:360 | On the host, the carrier's host-callable native operations (quire-exact FR-405) and the owner's `evaluate_ieee` and `compare_ieee` and the shipped owner functions agree for every supported operation, width and rounding mode ... |
| FR-053-AC-14 | examined | spec/kani/functional/FR-053-float-scalar-harnesses.md:361 | The expected side of every supported operation reaches no function of the carrier's native operations other than the bit-pattern decode ... |
| FR-052 P1 | examined | spec/kani/functional/FR-052-text-scalar-harnesses.md:62-70 | ... and a loop-free native comparison (quire-exact FR-404) in place of the owner's `compare_text`. P1's native comparison is host-callable ordinary code, one source for host and Kani builds and not `cfg(kani)`-only ... |
| FR-052 D4 paragraph | examined | spec/kani/functional/FR-052-text-scalar-harnesses.md:88-96 | The owner implementation the native comparison stands in for allocates `String`s ... Because the harness evaluates the native comparison, the proof covers it; AC-12 ties it to the shipped `compare_text` on the host. |
| FR-052-AC-12 | examined | spec/kani/functional/FR-052-text-scalar-harnesses.md:298 | On the host, the carrier's host-callable native comparison (quire-exact FR-404) and the shipped `compare_text` agree for all six operators ... |
| quire-exact FR-404 Behavior 7 | context_only | quire-exact spec/functional/FR-404-kani-tractable-native-text-carrier.md:52 | The replacement and the constructor SHALL be callable from a host build and SHALL NOT have a body selected by `cfg(kani)` ... CG's generated harness calls `compare_native_text` in place of `compare_text` ... |
| quire-exact FR-405 Behavior 8 | context_only | quire-exact spec/functional/FR-405-kani-tractable-native-float-carrier.md:50 | The native operations SHALL be one source for the host and Kani builds, and no body SHALL be selected by `cfg(kani)`. |
| quire-exact FR-405 Behavior 15-16 | context_only | quire-exact spec/functional/FR-405-kani-tractable-native-float-carrier.md:60-61 | ... either an executed Kani VERIFIED verdict of its Behavior 19 harness, or a measured row ... The refusal that a combination in the second state of Behavior 15 receives ... are those of IR-1526's refusal seam ... |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | high | The fix for FND-001 makes the arithmetic expected side the host `f32`/`f64` operation and states in FR-053 Evaluation that, apart from `nearest-even` result bits and what the NaN rule fixes, 'the directed-rounding bits, the flags other than `invalid`, the ordered charges at a limit' are not claimed by the harness and are 'left to AC-13's host differential'. The criteria were not aligned. FR-053-AC-4 still requires the harness to prove agreement on bits, flags, ordered charges, denied charge and counters at zero, exact and short budgets for every supported operation. FR-053-AC-5 asserts `Incomplete` at each charge point and the `exact` refusal with would-be flags. FR-053-AC-9 (c), (d), (i) and (j) and FR-053-AC-10 require each rounding-mode, `exact`-refusal, overflow/underflow-flag and charge mutant to 'fail with actual solver playback'. With the host reference, a mutant that rounds toward-zero under a toward-positive node, drops `overflow`, or drops `ieee.round` passes the harness, so these criteria cannot be met as written and contradict the Evaluation paragraph. Either narrow AC-4, AC-5, AC-9 and AC-10 to the outputs the harness fixes and move the rest to AC-13 (stated per item), or give the expected side an independent derivation of the remaining outputs (charges are classification-determined; directed bits and flags need a reference). | spec/kani/functional/FR-053-float-scalar-harnesses.md:183-187, spec/kani/functional/FR-053-float-scalar-harnesses.md:364-370 |
| FND-006 | low | FR-053 Evaluation cites two quire-exact clauses for statements they do not make. 'FR-405 Behavior 6 keeps its decode private to the native body': Behavior 6, on quire-exact main and unchanged on PR #159 head 6986f4fd, says the opposite, allowing the bit-pattern decode as the one shared function and leaving its defects to 'CG FR-053-AC-14'. Only PR #159's FR-405-AC-9 says 'not even the bit-pattern decode'. 'The same independent reference quire-exact FR-405 Behavior 19 uses': Behavior 19 says only 'a separately written reference'; the host `f32`/`f64` choice is in PR #159's `src/native_float/kani_proofs.rs`, not in the requirement. Cite FR-405-AC-9 (and TC-953 or the harness docstrings for the host reference), not Behavior 6 and 19. | spec/kani/functional/FR-053-float-scalar-harnesses.md:178-179, spec/kani/functional/FR-053-float-scalar-harnesses.md:188-189 |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-007 | high | FR-053 Evaluation makes the arithmetic expected side the host `+`, `-`, `*`, `/`, `sqrt` and `mul_add`, 'the same independent host reference that quire-exact's own harnesses use (TC-953 and the docstring of `src/native_float/kani_proofs.rs` list it)'. That docstring (quire-exact PR #159 head 6986f4fd, lines 31-33, 409-411, 445-448) says the opposite for two of the six: 'Kani's model of the host `sqrt` and `/` is not bit-exact, so their expected sides are exact-integer brackets of the correctly rounded root and quotient', with a measured disagreement at `0x800000A9 / 0x9C873333`. A CG `div` or `sqrt` harness built as this paragraph specifies compares a correct native result against Kani's inexact model and fails spuriously. So no such harness can be Verified, and the stated mechanism contradicts the source it cites. Name the exact-integer bracket (or another bit-exact reference) as the expected side for `div` and `sqrt` under Kani. AC-13's host comparison with Rust `/` and `sqrt` is unaffected, because it runs on real hardware. | spec/kani/functional/FR-053-float-scalar-harnesses.md:178-180 |
| FND-008 | medium | FR-053-AC-13 now says 'quire-exact TC-953's mutant table and `scripts/ir1446_native_float_mutants.py` are the buildable form of' every mutant marked (host) in AC-9 and every AC-10 mutant the harness does not carry. On PR #159 head 6986f4fd several of those mutants have no counterpart in either the script or TC-953's table: AC-9 (c) every pair among the four non-nearest-even modes (the script has only `toward-zero-increments`, `directed-swapped` and `away-ties-to-even`; there is no nearest-away against a directed mode and no toward-zero against toward-positive or toward-negative); AC-9 (j) the `underflow` flag missing (only `underflow-on-exact`, the opposite defect, exists); AC-9 (l) `sqrt` reporting every result inexact (only `sqrt-inexact-lost`); AC-10 (c) the width taken as 64 at `float32` or 32 at `float64`; AC-10 (d) `ieee.result-retain` charged on an `exact` refusal. The example 'rounding toward zero under a toward-positive node' is also not a script mutant (`directed-swapped` matches it only for positive results). Either drop the 'buildable form' claim and require CG's differential to build the missing mutants, or map each (host) mutant to the QE mutant that kills it and route the gaps to quire-exact PR #159 (same ticket). | spec/kani/functional/FR-053-float-scalar-harnesses.md:391 |

## New findings (disposition pass 3)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-009 | low | Two wording slips in FR-053-AC-9 left over from the round-3 fix. (1) The lead-in still explains the harness's limits with 'because its expected side is the host float operation', but `div` and `sqrt` now use an exact-integer bracket (Evaluation, AC-14). Say 'because its expected side (host operation or integer bracket) fixes only the `nearest-even` bits'. (2) AC-9 (c) reads 'Harness: any other mode, drawn as a directed or nearest-away node, in place of `nearest-even`'. The caught mutant is a native body that rounds in another mode under a `nearest-even` node. A node 'drawn as a directed or nearest-away node' is the reverse direction, which the next parenthetical says is not caught. Reword to 'a native body that rounds in any other mode under a `nearest-even` node'. Behaviour is unaffected because the parenthetical states the intent. | spec/kani/functional/FR-053-float-scalar-harnesses.md:391 |

## Dispositions

Round 1, against PR #817 head 444ff8ad (fix commit 444ff8ad over d23ae785, after merge 7a0dbab5).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | - |
| FND-003 | fixed | - |
| FND-004 | fixed | - |

FND-001 after: the expected side is 'the host language's own float operation (`+`, `-`, `*`, `/`, `sqrt` and `mul_add` on `f32` and `f64`) ... there is no second implementation in the carrier crate', and AC-14 inspects 'no shared function, the decode included'. The owner is named, so the finding as written is fixed; the narrowed claim this choice produces is not carried into the ACs (FND-005). The owner of native sqrt/fma is also settled upstream: quire-exact FR-405 on origin/main (merged #155, adfa515) and on PR #159 states 'IR-1502 is CG's oracle derivation for square root and fused multiply-add and is an oracle, not a native operation; this requirement owns the native square root and fused multiply-add', and #817's P1 names IR-1446 as supplier of native sqrt and fma, consistent with it; P3 keeps IR-1502 as the FR-014 oracle. No ownership finding remains.
FND-002 after: P1 now reads 'Support is per operation and width, as quire-exact FR-405 Behavior 15 gates it: only a combination with an executed Verified verdict is a supported row. A combination whose quire-exact record is a measured table row ... is refused by IR-1526's tractability seam (FR-056, quire-exact FR-405 Behavior 16)'.
FND-003 after: AC-13 reads 'the carrier's host-callable native operations (quire-exact FR-405) and the owner's `evaluate_ieee` and `compare_ieee` agree'.
FND-004 after: 'replacement' no longer occurs in FR-052 or FR-053 (grep); the text uses 'native comparison' and 'native operations'.

Routed outside this PR (not a CG finding): quire-exact PR #159 (same ticket IR-1446) leaves FR-405 Behavior 6 allowing a shared decode while its AC-9 forbids it, and Behavior 8 still glosses CG FR-053's term "the carrier's cfg(kani) replacement", which #817 removes. Both are QE-side wording for the #159 author.

Round 2, against PR #817 head c6bf491f (fix commit c6bf491f over 444ff8ad).

| FND | outcome | reason |
| --- | --- | --- |
| FND-005 | still-open | Mostly fixed: AC-4 and AC-5 now split the harness and host halves; AC-9 marks (host) items; AC-10 moves charges to AC-13. Four claims still go beyond what the host-reference harness fixes. (1) AC-9 (c) 'Harness: `nearest-even` in place of any other mode': under a directed node the harness checks only NaN-ness and `invalid`, and under `exact` a completed result equal to the nearest-even bits passes (the reference has no inexactness test), so that half is not caught. Only 'any other mode in place of `nearest-even`' is. (2) AC-4 says the harness proves 'the typed outcome kind' for arithmetic, but the reference does not decide refused against completed under `exact`, or `Incomplete` at a limit. AC-5 correctly narrows this to 'only `exact` refuses'. (3) AC-10 gives the harness 'a stopped outcome reported completed under a limit the harness draws', while AC-4 and the Evaluation paragraph leave the denied charge and limits to AC-13; quire-exact's harness runs a generous meter. (4) AC-10 gives the harness the result-unit charge, but AC-4 says 'the ordered charges ... are proved by AC-13's host differential and not by the harness' with no exception. |
| FND-006 | fixed | - |

FND-005 after (the part that is fixed): AC-4 'For every supported arithmetic operation the harness proves the typed outcome kind, the completed value's bits under `nearest-even` (and the NaN result's sign, quietness and payload and the `invalid` flag under every spelling) and the provenance; the result bits under the other spellings, the flags other than `invalid`, the ordered charges, the denied next charge and the consumed counters at zero, exact and short budgets are proved by AC-13's host differential and not by the harness.'
FND-006 after: 'not even the decode of a bit pattern (quire-exact FR-405-AC-9)' and 'the same independent host reference that quire-exact's own harnesses use (TC-953 and the docstring of `src/native_float/kani_proofs.rs` list it)'. Both Behavior citations are gone. The new docstring citation is wrong for `/` and `sqrt` (FND-007).
Checked and correct: 'no arithmetic row is supported; the comparisons are supported' matches quire-exact PR #159. TC-953's evidence record and FR-405 Status show the comparisons and negation VERIFIED at both widths, and only the `nearest-even` harnesses of add and sub (both widths) and mul (`binary32`) VERIFIED among the arithmetic harnesses, with every other arithmetic harness a timeout. FR-405 Behavior 19 on #159 makes a combination with several harnesses VERIFIED only when all of them are, and Behavior 15 gates on that. The per-harness layout (three per arithmetic operation and width, one per comparison and negation) matches the TC-953 record.

Round 3, against PR #817 head 75f74aeb (fix commit 75f74aeb over c6bf491f).

| FND | outcome | reason |
| --- | --- | --- |
| FND-005 | fixed | - |
| FND-007 | fixed | - |
| FND-008 | fixed | - |

FND-005 after: residual (1) AC-9 (c) harness now covers only another mode in place of `nearest-even`, with 'the reverse direction is not caught: under a directed node or `exact` the harness checks only NaN-ness and `invalid`, or equality with the `nearest-even` bits'. (2) AC-4: 'the provenance of a completed result, and that only `exact` refuses (it cannot tell a refusal from a completed result under `exact`, and a limit is not drawn)'. (3) and (4) AC-10: 'the Kani harness carries none of them: its expected side fixes no charge and draws no limit (quire-exact's harnesses run a meter that never denies), so the ordered charges, the result-unit charge among them, and a stopped outcome are all AC-13's'; AC-4 lists 'the ordered charges (including the result-unit charge)' under AC-13. No arithmetic criterion now claims a harness output beyond `nearest-even` bits, NaN results, `invalid`, provenance and 'only `exact` refuses', which is what quire-exact PR #159's harnesses check (kani_proofs.rs:24-38).
FND-007 after: 'The expected side of `div` and `sqrt` is not the host `/` or `sqrt`: Kani's model of those is not bit-exact (it reports `0x800000A9 / 0x9C873333` as other than the hardware's `0x1BA00000`), so quire-exact's harnesses bracket the correctly rounded quotient and root in exact integers ... and a CG `div` or `sqrt` harness SHALL do the same.' AC-14 is aligned. The hardware value 0x1BA00000 was independently checked as the correctly rounded binary32 quotient by exact rational arithmetic. The `mul_add` sentence ('Whether Kani's model of `mul_add` is exact is not established, because no quire-exact `fma` harness has returned a verdict') is accurate: all six `fma` harnesses in TC-953's evidence record are timeouts with no verdict. If the model is inexact, the harness fails spuriously and cannot report a false Verified, so the open question is safe.
FND-008 after: 'The differential builds every mutant marked (host) in AC-9 and every mutant of AC-10 itself ... Quire-exact's TC-953 mutant table and `scripts/ir1446_native_float_mutants.py` are a starting set, not the full list (it has, for example, the dropped `overflow` flag, the dropped `ieee.round` charge and `ieee.exact-intermediate` charged on a classification-decided input, and lacks several of the mutants listed here).' The three examples exist in the script as `overflow-flag-missing`, `no-round-charge` and `exact-intermediate-on-decided`.
