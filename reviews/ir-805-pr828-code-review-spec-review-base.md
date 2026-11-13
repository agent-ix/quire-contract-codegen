---
id: SR-17940486709564847047972
title: "IR-805 PR #828 spec-review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen PR #828 at 123b2daf; spec/kani/functional/FR-015-bounded-kani-obligations.md (graph status paragraph :164, AC-106 row :951, AC-106 elaboration :1044-1146), spec/kani/matrix/TC-025-bounded-kani-obligations.md (:210, :244-256, :308), spec/kani/matrix/tests.md (:50, :136); IR-805"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-025
    type: reviews
---

## Summary

Ticket: IR-805, slice B. The PR rewrites the slice-B part of FR-015-AC-106's status and the graph
paragraphs of FR-015, TC-025 and tests.md. The new AC-106 row describes the runs accurately. It says three
mutants settle `Inconclusive` with a verdict disagreement, that the wrong-role mutant is an approximation,
and that objects carry no fields. It keeps AC-106 PARTIAL with the right gaps listed. The defects are in the
summaries around the row and in the normative text, which does not say what a mutant's replay must return.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | high | The FR-015 graph paragraph says slice B delivers "the four semantic mutants ... Falsified under installed Kani with same-clause `qsl_replay` agreement on the decoded graph", and tests.md:50 repeats "four semantic mutants Falsified ... with same-clause `qsl_replay` agreement". For three of the four (zero-length, rejected self-loop, wrong role) QSL does the opposite: the authored clause holds and the replay settles `Inconclusive` with `DisagreementCause::Verdicts`. Only the negated claim agrees. These summaries misstate the evidence that the AC-106 row reports correctly. | spec/kani/functional/FR-015-bounded-kani-obligations.md:164, spec/kani/matrix/tests.md:50 |
| FND-002 | medium | AC-106's normative text does not define the required replay outcome for each kind of mutant. The row says "native replay of that same authored clause", FR-015:1144 says "same-clause `qsl_replay` agreement", and TC-025:308 says "same-authored-clause `qsl_replay`". A harness-semantic mutant leaves the authored clause unchanged, so agreement is impossible by construction, and the detection signal is native disagreement on an original-domain graph. Read literally, "agreement" condemns a correct test. The criterion should state: claim mutant gives a reproduced violation; harness mutant gives an original-domain playback on which the authored clause holds (verdict disagreement), paired with the healthy `Verified` control on the same claim. | spec/kani/functional/FR-015-bounded-kani-obligations.md:951,1144, spec/kani/matrix/TC-025-bounded-kani-obligations.md:308 |
| FND-003 | medium | The status contradicts itself on the wrong-role mutant. The AC-106 row lists it among the delivered `Falsified` mutants and then lists "a wrong-role mutant over a second populated role" as not delivered. FR-015:164 and tests.md:50 count "four semantic mutants" as delivered. The mutant is a constant-false traversal (SR-17940486709564847047970 FND-001), which TC-025:310 excludes. The row also calls the tests "lib tests, ignored, Kani lane", but `make kani-gate` does not run lib tests (SR-17940486709564847047971 FND-002). | spec/kani/functional/FR-015-bounded-kani-obligations.md:164,951, spec/kani/matrix/tests.md:50 |
| FND-004 | low | Stale text. TC-025:244-252 still lists "positive-length semantics ... in the generated draw and the `qsl_replay` oracle" and "each semantic FR-015-AC-106 mutant `Falsified` ... and same-clause native replay" among the "Real-package steps that wait on CG integration and QSL-820 emission (unbacked)". Both are now backed for the relationship-end form. The tests.md:50 status still opens "Partial (IR-805 slice A)". tests.md:136 still says AC-100's "graph clause remains planned" (left over from slice A, SR-17940486709564847047969 FND-006). | spec/kani/matrix/TC-025-bounded-kani-obligations.md:244-252, spec/kani/matrix/tests.md:50,136 |

## Verdict

Not mergeable as is. The fix is text only. The AC-106 row is honest and AC-106 correctly stays PARTIAL,
but the FR-015 graph paragraph and tests.md misstate the evidence as "agreement" (FND-001). The criterion
never says that disagreement is the required outcome for a harness mutant (FND-002). The wrong-role mutant
is counted as delivered in one sentence and as missing in another (FND-003).

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-005 | low | The AC-106 row added in this PR names QSL-1905 as the owner of the non-tautological `{0,3}` healthy claim ("a non-tautological `{0,3}` healthy claim (multi-reachability, QSL-1905)"). QSL-1905 is cancelled; the multi-reachability/multi-clause work is now CG work inside IR-805 (a later slice E). The text should cite IR-805 slice E, and the same row's "which slice A refuses as `UnsupportedClaimShape` and which stays PLANNED" should name that slice. | spec/kani/functional/FR-015-bounded-kani-obligations.md:951 |

## Dispositions

| FND | outcome | reason |
|---|---|---|
| FND-001 | fixed | fixed at 6b3940be: FR-015:164 now reads "the negated-claim, zero-length-path and rejected-self-loop mutants are Falsified under installed Kani and replayed through `qsl_replay` over the authored clause: the claim mutant reproduces the violation, and each harness mutant is a spurious counterexample that the unchanged authored clause does not reproduce"; tests.md:50 matches. |
| FND-002 | fixed | fixed at 6b3940be: "Required replay outcome per mutant kind: a claim mutant ... reproduces the violation; a harness mutant (the generated reachability edited, the authored clause unchanged) is Falsified on an original-domain playback and QSL's replay of the unchanged authored clause holds on that graph, which settles Inconclusive with a verdict disagreement, so the spurious counterexample is the detection; each pairs with a healthy Verified control on the same claim." FR-015:1144 and TC-025:308 now say the same. |
| FND-003 | fixed | fixed at 6b3940be: the wrong-role mutant is listed only as not delivered ("the 'another field or role' clause is NOT evidenced") in FR-015:164, AC-106, TC-025:210 and tests.md:50, and the row states the lib tests are not run by `make kani-gate`. |
| FND-004 | fixed | fixed at 6b3940be: TC-025:247-254 now lists only the unbacked steps and records what slice B delivered; tests.md:50 opens "Partial (IR-805 slices A and B)"; tests.md:136 says AC-100's graph clause "is covered by `graph_run_evidence_preserves_every_ordered_identity_option` for the relationship-end form". |
| FND-005 | still-open | New this round: QSL-1905 (cancelled) is still cited at FR-015:951; text-only fix to cite IR-805 slice E. |
