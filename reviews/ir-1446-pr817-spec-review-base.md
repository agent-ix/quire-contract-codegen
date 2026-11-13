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
