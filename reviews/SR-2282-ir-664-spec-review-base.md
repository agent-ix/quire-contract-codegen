---
id: "SR-2282"
title: "CG IR-664 spec-review base: i64 representation-boundary criteria"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-codegen@code/ir-664-i128-consumers (second frozen head; revision recorded in the IR-664 Linear marker only, per the no-SHA rule); spec/core/functional/interface-001-codegen-api.md (oracle_slice.integer_representation_boundary), spec/oracle/functional/FR-031-boolean-oracle-integer-arithmetic.md (FR-031-AC-28), spec/oracle/matrix/TC-044-boolean-oracle-integer-arithmetic.md (step 18), spec/oracle/matrix/tests.md, spec/strategy/functional/FR-008-bound-domain-strategy-admission.md (FR-008-AC-6), spec/strategy/matrix/TC-017-bound-domain-admission.md (step 4, expected results), spec/strategy/matrix/tests.md, spec/kani/functional/FR-015-bounded-kani-obligations.md (FR-015-AC-82), spec/kani/matrix/TC-025-bounded-kani-obligations.md, spec/kani/matrix/tests.md"
---

# SR-2282: CG IR-664 spec-review base

## Summary

Ticket: IR-664. This round adds three criteria (FR-031-AC-28, FR-008-AC-6, FR-015-AC-82), one
interface-001 line, three TC procedures and three matrix rows. All of them state that CG stays
i64 and refuses wider IR integers, with no narrowing. `quire validate` over the ten changed files
and over the full `make spec` glob exits 0 and gives no warning on a changed file. Every new AC
names its inputs (the two just-outside-i64 endpoints and the i128 extremes), its refusal code and
the absence of any artifact. Each one is falsifiable, and each TC step can be executed. IDs
continue their sequences without reuse. The new text contains no SHA, local path or conflict
marker. interface-001's `integer_representation_boundary` agrees with FR-031-AC-28's code and
span rule.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The last sentence of FR-008-AC-6 and the whole of FR-015-AC-82 set requirements on private seams that the public pipeline cannot reach: `admit_relation` is private, and the clause ABI and `subject_binding` are `pub(super)` and private ("if reached independently of oracle admission", "presented directly to clause ABI construction"). A requirement then pins an internal function boundary, which a refactor that merges or inlines the seam would invalidate even though no observable behaviour changed. Phrase them as a design constraint (CON) on fallible narrowing, or keep them as test-only obligations in the TC | spec/strategy/functional/FR-008-bound-domain-strategy-admission.md:128, spec/kani/functional/FR-015-bounded-kani-obligations.md:626 |

## Verdict

The new spec content is accurate to the code and testable. One low finding: two ACs are coupled
to implementation seams. The matrix-status problem is recorded once, in SR-2281 FND-004.

## Dispositions

Round 1, reviewed at the branch's third frozen head (fix commit e4231e1). The private-seam
obligations are now constraints with direct TC-017/TC-025 tests: FR-008-CON-3, FR-008-CON-4,
FR-015-CON-1 and FR-015-CON-2. FR-015-AC-82 is removed, and FR-008-AC-6 states only the public
`UnsupportedClause` outcome. `quire validate` exits 0 over the ten changed spec documents and over
the `make spec` glob, with no warning on a changed document.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | e4231e1 |
