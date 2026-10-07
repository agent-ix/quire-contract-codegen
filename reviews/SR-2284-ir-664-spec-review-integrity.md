---
id: "SR-2284"
title: "CG IR-664 spec-integrity-analysis: i64 representation-boundary criteria"
type: SpecReview
analysis: integrity
review_set: subset
scope: "agent-ix/quire-contract-codegen@code/ir-664-i128-consumers (second frozen head; revision recorded in the IR-664 Linear marker only, per the no-SHA rule); spec/core/functional/interface-001-codegen-api.md (oracle_slice.integer_representation_boundary), spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md (FR-031-AC-28), spec/oracle/matrix/TC-044-boolean-oracle-integer-arithmetic.md (step 18), spec/oracle/matrix/tests.md, spec/strategy/functional/FR-008-bound-domain-strategy-admission.md (FR-008-AC-6), spec/strategy/matrix/TC-017-bound-domain-admission.md (step 4, expected results), spec/strategy/matrix/tests.md, spec/kani/functional/FR-015-bounded-kani-obligations.md (FR-015-AC-82), spec/kani/matrix/TC-025-bounded-kani-obligations.md, spec/kani/matrix/tests.md"
---

# SR-2284: CG IR-664 spec-integrity analysis

## Summary

Ticket: IR-664. Consistency: FR-031-AC-28 (oracle, `UnsupportedExpression`), FR-008-AC-6
(strategy, `UnsupportedClause` preserving that code and span; direct seam `UnsupportedRelation`),
FR-015-AC-82 (Kani, `UnsupportedDependency`/`UnsupportedBinding`) and interface-001 name the same
endpoint set and the same no-narrowing rule, with no contradiction. Each new AC has one TC
procedure (TC-044 step 18, TC-017 step 4, the TC-025 paragraph) and one matrix row. The computed
matrix (`quire matrix --format tsv`) adds exactly these three criteria. FR-015-AC-82 and
FR-008-AC-6 are tagged. FR-031-AC-28 is untagged (SR-2281 FND-004, not repeated here).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-008-AC-6 is compound. It states three separately failing outcomes: the oracle's `UnsupportedExpression`, the strategy's `UnsupportedClause` preserving code and span, and direct relation admission's `UnsupportedRelation`. FR-015-AC-82 likewise covers two seams with two codes. A single verdict per AC cannot say which part failed. Split them into one AC per seam | spec/strategy/functional/FR-008-bound-domain-strategy-admission.md:128, spec/kani/functional/FR-015-bounded-kani-obligations.md:626 |
| FND-002 | low | Every other FR-031 criterion row opens with its status (`IMPLEMENTED (IR-xxx).` or `PLANNED (IR-xxx).`). FR-031-AC-28 has none, so its status can be read only from the matrix row | spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md:522 |

## Verdict

The new criteria agree with one another, with interface-001 and with the code. Two low findings:
AC atomicity, and a missing status prefix.

## Dispositions

Round 1, reviewed at the branch's third frozen head (fix commit "IR-664: make boundary criteria atomic and trace checked seams"). Each new row now holds
one obligation. Every constraint id is referenced by its TC procedure (TC-017 step 4, the TC-025
paragraph), by its TC's Traces To column and by a matrix row, and each is tagged in the computed
matrix.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | round-2 fix commit "IR-664: make boundary criteria atomic and trace checked seams" |
| FND-002 | fixed | round-2 fix commit "IR-664: make boundary criteria atomic and trace checked seams" |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | FR-008-AC-6 now opens with `IMPLEMENTED (IR-664).`, but no other FR-008 criterion (AC-1 to AC-5) carries a status prefix; FR-008 keeps status only in spec/strategy/matrix/tests.md. The prefix belongs to FR-031's convention, not FR-008's. Drop it so the document follows one convention | spec/strategy/functional/FR-008-bound-domain-strategy-admission.md:130 |

Round 2, reviewed at the branch's fourth frozen head (fix commit "IR-664: record reviewed findings
and align strategy criterion style"; revision recorded in the IR-664 Linear marker). The FR-008-AC-6
row differs from the previous head only by the removed `IMPLEMENTED (IR-664). ` prefix. The commit
also adds this review set under `reviews/` and changes nothing else. `quire validate` exits 0 over
the FR-008 document and over the `make spec` glob.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-003 | fixed | round-3 fix commit "IR-664: record reviewed findings and align strategy criterion style" |
