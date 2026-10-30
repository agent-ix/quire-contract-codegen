---
id: SR-616003
title: IR-1091 CG PR 616 FR-015 collection status spec review
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; PR 616 (stacked on PR 613); diff 731029ed..9f6a3b08, round 2 9f6a3b08..bc3d50c9, round 3 bc3d50c9..c9c6bde4; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md, spec/kani/matrix/tests.md"
review_set: subset
---

## Summary

Ticket: IR-1091. Spec review (base, with a targeted integrity/consistency pass) of the FR-015 status edits in fix head 9f6a3b087e64c39147946ae59f3a7b4ef138e2de. The only spec change in the stacked diff is the status portion of two rows, FR-015-AC-96 and FR-015-AC-102. Draft PR 615 (over-capacity wording) is held for the owner and was not used as authority. The other sub-analyses (EARS, failure-domain, evidence, dependency) were skipped because no requirement statement, AC normative text, verification method or relationship changed.

## Verdict

CONDITIONAL. The two edited rows are correct:
- The normative text of AC-96 and AC-102 is byte-identical.
- AC-96 stays PARTIAL, scoped to feasible OrderedSet domains, with the over-capacity branch pending the owner ruling.
- AC-102 drops only the dangling citation and remains backed by two tests that are still tagged.
- Every newly cited test resolves through `quire trace --id FR-015-AC-96`, and `quire validate` passes on the three files.

However, three other status sites in the same spec still say that AC-96's uniqueness clause has no AC-96-tagged test. That now contradicts the edited AC-96 row and the trace.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-015's "Partly implemented" status paragraph still lists "-96 (uniqueness clause)" among the clauses with "no test carrying that AC's tag". That contradicts the edited AC-96 row and the four AC-96-tagged ordered-set tests. | spec/kani/functional/FR-015-bounded-kani-obligations.md:126 |
| FND-002 | medium | The TC-025 status text still says "FR-015-AC-96 (uniqueness) ... backed only by tests tagged for other ACs, so they are partial". The feasible uniqueness clause now has AC-96-tagged tests; only the over-capacity branch is pending. | spec/kani/matrix/TC-025-bounded-kani-obligations.md:79 |
| FND-003 | low | The FR-015 row of the matrix index still says "AC-96's uniqueness clause has no AC-96-tagged test (:1461 is tagged AC-97/AC-102)". That cites the renamed, untagged test, so the citation dangles, as the one this PR fixed in AC-102 did. | spec/kani/matrix/tests.md:39 |

## Coverage

Examined:
- the FR-015-AC-96 row (status and backing list);
- the FR-015-AC-102 row (backing list);
- the FR-015 implementation-status paragraph (lines 124 to 130);
- TC-025's coverage status (lines 60 and 76 to 81);
- the FR-015 and TC-025 rows of spec/kani/matrix/tests.md.

Read for context only: FR-015-AC-97, -99, -101 and IR-1091's held over-capacity decision. Nothing in the edits narrows an AC or claims the over-capacity path. The three findings are consistency drift only, and none needs normative wording or the owner's #615 ruling. Each is a status-text correction in the same style as the AC-96 row edit.

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | Head bc3d50c93cfe9ad44974b762606477bc8069799a: the FR-015 status paragraph (lines 124 to 130) now says -96 has AC-96-tagged feasible OrderedSet uniqueness controls, with the over-capacity branch awaiting the owner's IR-1091 / CG #615 ruling. AC-96 is kept partial, and the edit is confined to that status paragraph, with no AC table row touched. |
| FND-002 | fixed | TC-025:79 now says AC-96 has tagged feasible OrderedSet uniqueness controls with over-capacity pending, that AC-99 still awaits the final reviewed-head Kani gate and graph clauses, and that "Both criteria stay partial". |
| FND-003 | fixed | The FR-015 row at spec/kani/matrix/tests.md:39 no longer cites :1461. It names `feasible_ordered_set_emits_symbolic_ordered_unique_slots`, `boolean_ordered_set_keeps_both_values_and_distinct_active_slots` and the Kani-lane uniqueness verdict control, all AC-96-tagged per `quire trace`. The TC-025 row at :113 was corrected the same way. None of the four sites claims AC-96 or AC-99 is complete, and `quire validate` passes on all three files. |
| FND-004 | fixed | Round 3, head c9c6bde4f303792ce7fa346f4901ab82afa0cb7e (spec-only diff bc3d50c9..c9c6bde4, four status hunks: FR-015:129-131, TC-025:79, tests.md:39 and :113). Each now states that AC-101's representational-ceiling clause has AC-101-tagged unit controls (`domain_representability_failures_keep_the_typed_reason`, `rendering_refuses_an_unrepresentable_element_domain_without_a_harness`) and that the missing-or-malformed-declaration clause lacks a generator-boundary control (`a_declaration_without_listable_fields_or_the_selected_field_is_field_not_admitted` is helper-internal). This matches the AC-101 row and `quire trace --id FR-015-AC-101`. No AC row is edited, AC-101 stays PARTIAL, and `quire validate` passes on all three files. |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | low | The rewritten FR-015 status paragraph says "-101's representational-ceiling and malformed-declaration clauses lack tagged generator-level controls". But the representational-ceiling clause is backed by `domain_representability_failures_keep_the_typed_reason` and `rendering_refuses_an_unrepresentable_element_domain_without_a_harness`, both tagged FR-015-AC-101 in src/kani/generate/collection.rs, as the AC-101 row itself states. This drift predates the PR (tests.md:39/:113 carry the same claim) and is outside IR-1091's ordered-set scope, but this round restated it in edited text. | spec/kani/functional/FR-015-bounded-kani-obligations.md:129 |

## Capacity follow-up review

Reviewed exact head `03f5d74afe349ea8a7c95f0c673118a75ce746de`, commits `efd0bd8d..03f5d74a`, against merged CG #615 AC-96 wording. CLEAN: no new findings. The normative AC-96 row includes the full-i64 inclusive capacity rule, the `u64` refusal invariant, refusal precedence, typed reason/no harness, and unchanged feasible endpoint obligations. The added status text correctly names tagged Boolean/IntRange controls while keeping installed-backend proof pending and AC-96 PARTIAL. TC-025 repeats that status without claiming complete proof. Scoped `quire validate` passed. No additional analysis lens applies to these status changes beyond base/integrity consistency already reviewed for this PR.
