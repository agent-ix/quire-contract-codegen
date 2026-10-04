---
id: "SR-1431"
title: "CG PR 262 spec review (criterion strength): FR-015-AC-53 to AC-58"
type: SpecReview
analysis: criterion-strength
review_set: subset
scope: "agent-ix/quire-contract-codegen@a5cd4bd54fece9ad071a399691d4dd106d9ade2d; FR-015-AC-53..58 in spec/kani/functional/FR-015-bounded-kani-obligations.md:323-328; emitted templates in src/kani/generate/{precondition,contract,scalar,frame,v1_bundle}.rs and corpus/bounded_kani_corpus.rs, and classify_report in src/kani/classify.rs, read as context"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: references
---

# SR-1431: CG PR 262 spec review (criterion strength)

## Summary

Ticket: IR-464. I judged by hand whether each new AC can fail. Jev (the calibrated
System One judge) was not used, so these judgements are mine. The AC-54, AC-56 and AC-57 claims
were also checked against real Kani 0.68 (see SR-1429).

- FR-015-AC-53: can fail. A missing cover, a second cover, or a cover before an assertion each
  violate it, and on this sha the V1 bundle and corpus templates do. Adverse cases are named.
- FR-015-AC-54: can fail (today's bundle template has no cover). Its witness claim is accurate:
  a failing `ensures` blocks the path, as measured.
- FR-015-AC-55: can fail (today's corpus template has no cover). The last clause ("and nothing
  stronger") cannot be tested and is a remark.
- FR-015-AC-56: can fail. Healthy runs give `Verified`; an unsatisfiable requires gives
  `CoverUnsatisfied {0, 1}`, as measured. "Healthy" is undefined.
- FR-015-AC-57: the first sentence can fail (healthy runs with no cover give
  `MissingCoverSummary` today). The second sentence is a meta-statement that nothing can
  falsify. The falsified-corpus case is missing (SR-1429 FND-004).
- FR-015-AC-58: can fail, in two parts. The exactly-one wording is ambiguous (SR-1429 FND-003).
  The design rationale is folded into the AC.

## Verdict

AC-53, AC-54 and AC-55 are strong. AC-56 and AC-57 need "healthy" defined. AC-57 and AC-58 carry
non-normative prose that should move to the FR body. No AC is vacuous.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-015-AC-56 and AC-57 rest on "a healthy V1 bundle harness" or "a healthy bounded-corpus harness" without defining it. The parenthesis in AC-56 defines `Verified`, not "healthy". Two test authors could choose different fixtures: one whose requires is satisfiable but whose subject breaks the ensures, or a corpus case whose oracle is false. State it directly: a bundle whose requires is satisfiable within the argument bounds and whose subject satisfies the ensures, and a corpus case whose oracle evaluates true. | spec/kani/functional/FR-015-bounded-kani-obligations.md:326-327 |
| FND-002 | low | Non-falsifiable sentences inside ACs: AC-57's "A vacuous corpus run is not constructible ..., so no corpus criterion states a `CoverUnsatisfied` run" and AC-55's "its cover is reachability past the assertion and nothing stronger". Move them to the FR body or the TC-023 description. | spec/kani/functional/FR-015-bounded-kani-obligations.md:325,327 |
| FND-003 | low | AC-58 embeds design rationale that no test can fail: "A harness-spec constructor ... is not required by this criterion: the generator has no single emission seam today (AD-004 plans one ...)". The rationale is correct (`src/kani/generate/spec.rs` does not exist), but it belongs in the preamble or AD-004, not the criterion. | spec/kani/functional/FR-015-bounded-kani-obligations.md:328 |

## New findings (disposition pass 1)

Re-checked at 5a9fe3e040dd83251a17540d0fe623f69d4fadba.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | low | The new AC-57 reads as requiring a false case for each family ("a bounded-corpus harness of each of the arithmetic, graph and collection families whose oracle is true classifies `Verified`, and one whose oracle is false classifies `Falsified`"). The arithmetic family cannot supply one: `generate_bounded_kani_corpus_case` hard-codes the arithmetic case's value to `true` ("Admission establishes the checked arithmetic/definedness property"), and only graph and collection carry a lowered Boolean value. Scope the false case to the families that can produce one, as TC-023 and the matrix row already do ("a case whose oracle is false"). | spec/kani/functional/FR-015-bounded-kani-obligations.md:339, src/kani/generate/corpus/bounded_kani_corpus.rs:462-468 |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 1216b32: AC-56 now reads "whose requires clause some bounded argument satisfies and whose `ensures` holds for every such argument", and AC-57 reads "whose oracle is true" / "whose oracle is false". The leftover "healthy" in the TC-023 and tests.md notes is SR-1429 FND-007. |
| FND-002 | fixed | 1216b32: AC-55's "nothing stronger" and AC-57's "not constructible" sentence are gone from the ACs; the corpus rationale is now in the FR-015 preamble. |
| FND-003 | fixed | 1216b32: AC-58 no longer carries the `HarnessSpec` rationale; it moved to the preamble ("seven templates in six files ..."), which matches my round-0 count. |
| FND-004 | fixed | 9bebed4 (round 2): AC-57, the If-bullet (FR-015:273) and TC-023 scope the false case to graph and collection. I verified "an arithmetic case's oracle is always true": CG hard-codes the arithmetic value `true` (`bounded_kani_corpus.rs:468`), and IR's `lower_checked_arithmetic` (quire-contract-ir cbcd790, the Cargo.lock commit) refuses an overflowing i128 result or one outside `[minimum, maximum]` as `kani_definedness_checked_range`. So the rendered `checked_op(...).is_some_and(range)` oracle of any admitted case is true. |
