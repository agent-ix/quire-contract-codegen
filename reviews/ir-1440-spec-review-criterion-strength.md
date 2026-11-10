---
id: SR-17940486709564847047585
title: "IR-1440 PR #766 FR-051 mutant-control strength analysis"
type: SpecReview
analysis: criterion-strength
date: 2026-11-10
scope: "agent-ix/quire-contract-codegen; PR #766 @ 6c9302b6; IR-1440; spec/kani/functional/FR-051-decimal-scalar-harnesses.md, spec/kani/matrix/TC-065-decimal-scalar-harnesses.md, spec/kani/functional/FR-025-generated-subject-abi.md, spec/kani/functional/FR-028-bounded-proof-ceilings.md, spec/kani/matrix/TC-036-generated-subject-abi.md, spec/routed/matrix/TC-030-capability-settlement.md, spec/kani/matrix/tests.md, spec/spec.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-051
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-065
    type: reviews
---

## Summary

Ticket: IR-1440. Spec-only PR #766 (head 6c9302b6) adding FR-051 and TC-065, the planned Kani row for decimal scalar operations, with edits to FR-025, FR-028, TC-036, TC-030, spec/kani/matrix/tests.md and spec/spec.md. Reviewed against origin/main 66e7eac3 (includes #764 FR-049 and #765 FR-050), quire-exact decimal.rs/numeric.rs/outcome.rs/accounting.rs, CG src/oracle/scalar/derive.rs and src/kani, qsl-replay WitnessValue, and Contract IR FR-038.

## Verdict

CHANGES SUGGESTED. Jev (typesafe.ai System One) is not installed on this host; this is a manual reading of each AC-8 mutant against quire-exact decimal.rs/numeric.rs. Mutants (a), (c), (e) (0.125 to scale two: nearest-even 12, nearest-away 13), (f) (results are always retained at T, so 0.5+0.5 gives (100,2) against normalized (1,0)), (g), (i), (j), (k) each have a distinguishing in-domain point under the listed operators; (h) is observable because every charge carries one work unit; the add/mul swap exclusion is correct (charges are symmetric); the clipping and outcome-assume controls are correctly moved to correspondence (AC-2) and inspection (AC-4). Two wording gaps.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | (d) 'Ordering by retained representation instead of value' is an equivalent mutant if the representations are scale-aligned before comparison (that is exactly Decimal::compare's value order); only a coefficient-only comparison is caught by the 1.0 (10,1) vs 2 (2,0) point. Name the mutant as coefficient-only (or unaligned) ordering. Similarly (b)'s 'distinct operands' must mean numerically distinct: sub over 1.0 and 1 swapped gives 0 at T both ways. | spec/kani/functional/FR-051-decimal-scalar-harnesses.md:158 |
| FND-002 | low | 'two entries per parameter, so two parameters are at-bound at four and one-over at three' drops TC-060's 'set the physical value-count bound to four (at-bound) and three (one-over)'; as written it reads as entry counts, and three entries for two parameters is under, not over. Restore 'set the value-count bound to'. | spec/kani/matrix/TC-065-decimal-scalar-harnesses.md:68 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-051-AC-8 | examined | spec/kani/functional/FR-051-decimal-scalar-harnesses.md:158 | Each mutant below fails with actual solver playback of an in-domain input, caught by the listed operators over a control domain that contains its distinguishing point. (a) `add` for `sub` (and the converse): `add` and `sub`, nonzero second operand. (b) Operand swap: `sub`, `div` (nonzero, distinct, `a` not `-b`), `lt`, `le`, `gt`, `ge`, distinct operands; `add` and `mul` swaps are equivalent and are not claimed. (c) `lt` for `le`, `gt` for `ge` (and converses): the matching ordering operator, equal-valued operands. (d) Ordering by retained representation instead of value: ordering operators... |
| TC-065-step-7 | examined | spec/kani/matrix/TC-065-decimal-scalar-harnesses.md:59 | 7. Run the AC-8 mutants one at a time, each over a control domain stating its    distinguishing point and only against the operators FR-051-AC-8 pairs with it:    (a) add/sub; (b) swap in `sub`, `div` (nonzero, distinct, `a` not `-b`) and the    four orderings, with no swap claim for `add` or `mul`; (c) `lt`/`le`, `gt`/`ge`    at equal values; (d) representation ordering over `1.0` and `2`; (e) rounding    mode over the 0.125 tie; (f) normalized-value result or wrong scale over    `0.5 + 0.5`; (g) dropped loss record; (h) dropped or reordered charge; (i)    `InexactDecimal` reported rounded... |
| TC-065-step-8 | examined | spec/kani/matrix/TC-065-decimal-scalar-harnesses.md:68 | 8. Perturb retained witnesses (missing, duplicate, foreign, role-swapped, malformed    width or comment, negative or oversize scale, out of domain, over entry-count or    byte bound; two entries per parameter, so two parameters are at-bound at four and    one-over at three) and assert typed refusal with no replay and no normalization.    For a valid witness assert one `WitnessValue::Decimal` with the unnormalized pair    per parameter reaches public replay for that declaration. Record an unavailable    route as an open dependency (AC-9). |
| FR-051-AC-3 | examined | spec/kani/functional/FR-051-decimal-scalar-harnesses.md:153 | For `add`, `sub`, `mul`, `div`, `negate` and `convert_rounding` the oracle and native expression agree on the typed outcome, the completed value's retained representation (not only its normalized value), the loss record, ordered charges, the denied next charge and consumed counters, at zero, exact and short budgets at each charge point in the Behavior list. For the four ordering operators they agree on `Outcome<bool>`, charges and counters, with ordering by value. The reference neither runs the generated oracle twice nor bridges through Runtime. |
| FR-051-AC-4 | examined | spec/kani/functional/FR-051-decimal-scalar-harnesses.md:154 | Each reachable noncompletion is asserted as the agreed typed outcome and never assumed away: `Undefined(DivisionByZero)` for a zero divisor (and `Incomplete` at `decimal.operands` instead at a zero budget, since the zero check follows that charge), `Refused(InexactDecimal)` for an inexact quotient or product under `exact` rounding, `Refused(DecimalOutOfDomain)` for an add, mul or negate result outside the result domain, and `Incomplete` at each charge point named in the Behavior list. No cause is relabelled and a stopped outcome is never proved completed. The generated harness contains no a... |

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | fixed in d47cf1cc |
| FND-002 | fixed | fixed in d47cf1cc |
