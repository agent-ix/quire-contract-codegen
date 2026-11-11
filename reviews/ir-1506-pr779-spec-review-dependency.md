---
id: SR-17940486709564847047643
title: "IR-1506 PR #779 FR-052 attained-length spec dependency review"
type: SpecReview
analysis: dependency
date: 2026-11-11
scope: "agent-ix/quire-contract-codegen; PR #779 @ a0d477a0; IR-1506; spec/kani/functional/FR-052-text-scalar-harnesses.md, spec/kani/matrix/TC-066-text-scalar-harnesses.md, spec/kani/matrix/tests.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-052
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-066
    type: reviews
---

## Summary

Ticket: IR-1506. Spec-only PR #779 (head a0d477a0, one commit on origin/main d7e0505f), reviewed in a separate detached worktree of that head, scoped to git diff origin/main...HEAD (3 files under spec/kani/). Checked against quire-exact origin/main 7200065: FR-399, FR-400 (Statement, Derivation, Looseness, the 'Consequence for CG FR-052-AC-12' paragraph, FR-400-AC-1..AC-6), FR-401 (the 'table that does not fit' rule) and TC-947..TC-949, and against FR-052 at origin/main (git show). Host-side data check with Python unicodedata (Unicode 16.0.0, no cargo, no Kani): the stable scalars (nfc(c)=c, resp. nfkc(c)=c) with the most nfd (resp. nfkd) UTF-8 bytes are U+16126, U+16127, U+16128, U+16D6A at 12 bytes, so U+16126 is first, matching FR-400-AC-6; stable decomposition byte lengths are {1,2,3,4,5,6,8,9,12}; without the stability filter the nfkd maximum is U+FDFA (33 bytes, 18 scalars), and U+FDFA is also the nfc-stable scalar with the most nfkd bytes; the first nfc-stable scalar with a multi-scalar nfd is U+00C0 (3 bytes). An exhaustive canonical-equivalence search found admitted Text[1,1] nfc payload lengths {1..9,12} (none at 10 or 11); under nfkc a 10-byte admitted payload exists (for example U+1D6C2 U+0313 U+0300 U+0345, retained U+1F82), so L_b = 9 is a valid constructed length but not the nearest admitted length below A for nfkc. Single-byte variant search under Text[max,max]: at L for nfc/nfkc the final-byte pair exists (U+1611F to U+16120, that is U+16126 to U+16128) and no first-byte pair exists at max 1 or 2; at L_b no first-byte pair exists, and the final-byte pair exists for a Hangul LVT unit (23 variants) but not for U+0CCB (0 variants). make spec passes (exit 0, no warnings in the changed files); quire validate on FR-052 reports no warnings at base or head; quire matrix lists FR-052-AC-1..AC-13 untagged. No Unicode version literal, hash or pin is added by the diff (the pre-existing 'pinned Unicode 17.0.0 tables' sentence at FR-052:137 is unchanged). No ID collision: no other open PR touches FR-052, TC-066 or the FR-052 row of spec/kani/matrix/tests.md (#523 edits only FR-015 rows of that file); FR-052-AC-13 is referenced nowhere else. No reviews/ir-1506-* file exists in CG or any worktree.

## Verdict

PASS. The new `references` edge to ix://agent-ix/quire-exact/FR-400 resolves: FR-400 is on quire-exact origin/main, Planned (IR-1500), and FR-400 already references CG FR-052 back. The Dependencies bullet names FR-400-AC-2 (P) and FR-400-AC-6 (A) and restates neither as a constant. AC-13 computes A on the host through NormalizationForm::apply and TextType::admits rather than reading quire-exact's recorded value, so there is no pin or table hash and no hidden dependency on TC-948's run. The AC preamble now makes A for nfc/nfkc conditional on P3 (IR-1500), and AC-12 for normalizing profiles stays conditional on P4 (IR-1501), matching FR-401's ruling that a table that does not fit stays unsupported. The citation style (`quire-exact [FR-400](ix://agent-ix/quire-exact/FR-400)`) matches existing CG uses such as `[quire-exact FR-369](ix://...)`. No open PR collides (#523 edits only FR-015 rows of spec/kani/matrix/tests.md; #381 and #413 do not touch FR-052).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-052 Dependencies (FR-400) | examined | spec/kani/functional/FR-052-text-scalar-harnesses.md:319-321 | - quire-exact [FR-400](ix://agent-ix/quire-exact/FR-400) (criteria FR-400-AC-2 and FR-400-AC-6) owns the payload-byte bound P and the attained length A that AC-3, AC-12 and AC-13 use; this row restates neither as a constant. |
| FR-052 P3 prerequisite | examined | spec/kani/functional/FR-052-text-scalar-harnesses.md:75-78 | - P3 (ticket IR-1500) is the payload-byte bound P for `nfc`, `nfd`, `nfkc` and `nfkd`, stated by quire-exact in [FR-400](ix://agent-ix/quire-exact/FR-400), whose FR-400-AC-6 also records the attained payload length A that AC-12 and AC-13 use. Every normalizing-profile criterion is conditional on it. |
| FR-052 AC preamble | examined | spec/kani/functional/FR-052-text-scalar-harnesses.md:279-283 | All supported-harness criteria are conditional on P1 (IR-1445) and P2 (IR-1499). Every normalizing-profile criterion is conditional on P3 (IR-1500): the payload-byte bound of AC-3 and the normalizing-profile parts of AC-2, AC-5, AC-6, AC-9 (d), (h) and (i) and AC-10; A of AC-12 and AC-13 for `nfc` and `nfkc` is conditional on P3 as well. The same normalizing-profile parts, and AC-12 for a normalizing profile, are also conditional on P4 (IR-1501). |
| FR-052-AC-3 | examined | spec/kani/functional/FR-052-text-scalar-harnesses.md:289 | The payload-byte bound P is `max` for `binary-utf8`, 4 x `max` (checked arithmetic) for `unicode-scalars`, and the bound IR-1500 states (quire-exact FR-400, `payload_byte_bound`) for `nfc`, `nfd`, `nfkc` and `nfkd`. P is an upper bound, which may exceed every admitted payload length (AC-13). An operand with P not greater than C is supported. An operand with P greater than C, a `max` or product beyond the carrier's length width or `u64`, or a normalizing profile whose bound IR-1500 does not state, yields named `unsupported` retaining the declared `max`, the profile, P (or its absence) and C;... |
| FR-052-AC-13 | examined | spec/kani/functional/FR-052-text-scalar-harnesses.md:299 | The attained length A(profile, `max`) is computed on the host at the operand's declared `max` by the construction of quire-exact FR-400-AC-6, not read from a constant and not replaced by P. For `nfc` (respectively `nfkc`), c is the first, in scalar order, of the stable scalars (`nfc(c) = c`, respectively `nfkc(c) = c`) whose `NormalizationForm::apply` nfd (respectively nfkd) output has the most UTF-8 bytes; the payload is that output repeated `max` times, is admitted by the host `TextType::admits` under `Text[max, max]` of the profile, and A is its byte length. For `binary-utf8` the payload... |
| TM FR-052 row | examined | spec/kani/matrix/tests.md:14 | / FR-052 / FR-052-AC-1 through FR-052-AC-13 / TC-066 / 🚧 PLANNED (IR-1441): no text scalar Kani harness; needs the quire-exact text carrier (IR-1445), a non-metered `Text` constructor (IR-1499) and, for normalizing profiles, a payload-byte bound (IR-1500) and a loop-free normalization (IR-1501); TC-066 controls have not run. / |
