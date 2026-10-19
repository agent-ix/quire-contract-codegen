---
id: "SR-13191"
title: "CG PR 451 re-review gap analysis: FR-019-AC-45 and FR-019-AC-46"
type: SpecReview
analysis: gap-analysis
review_set: subset
scope: "agent-ix/quire-contract-codegen@5c07c891b630056da636f5925de496def23b7f75; spec/routed/functional/FR-019-capability-settlement.md (AC-10, AC-45, AC-46, AC-47), spec/routed/matrix/tests.md, src/routed/capability.rs, tests/it/capability_settlement.rs (diff origin/main...HEAD, merge base 35f37aaf, origin/main 661ef492)"
relationships:
  - type: references
    target: "ix://agent-ix/quire-contract-codegen/FR-019"
---

# SR-13191: CG PR 451 re-review gap analysis

## Summary

Ticket: IR-887 (parent IR-878). PR: agent-ix/quire-contract-codegen#451 at 5c07c891.
Plan completion: not assessed. The run is planless and scoped to the PR diff. Semantic review was
run for FR-019-AC-45 and FR-019-AC-46.

Trace tags. The four new tests carry `TC-030` plus only `FR-019-AC-45` (two tests) or
`FR-019-AC-46` (two tests). None is tagged AC-10 or AC-47; both of those are Analysis criteria in
the matrix.

AC-45, case by case:

- byte-equal adjacent [A,A]: covered.
- non-adjacent [A,B,A]: covered.
- differing [A1,A2] in both orders: covered.
- [A1,B,A2] in all six permutations: covered.
- origin-only difference: covered (`ProviderOrigin::Process` against `Linked`).
- other-field difference: covered for `advertised` and `bounds` only.
- "returns no item settlements": covered, since every case asserts `Err` of a `Result` that has no
  partial Ok.
- typed variant with no cause or payload: covered.

AC-46, case by case:

- bytewise-first among the repeated identities A and B, under four orders: covered.
- case-distinct `Kani`/`kani` accepted: covered.
- a bad version, a foreign vocabulary and an empty vocabulary each refused as `InvalidCapability`
  ahead of repeat detection, with an unknown item kind showing no kind is read: covered.

Strict coverage. The reviewer ran `quire matrix --scope <worktree> --format tsv` (quire 0.36.2) the
same way on both trees:

| | criteria | tagged | method-without-symbol | backed |
| --- | --- | --- | --- | --- |
| origin/main 661ef492 | 857 | 482 | 51 | 533 |
| PR head | 857 | 484 | 50 | 534 |

AC-45 and AC-46 go from untagged to tagged. FR-039-AC-5 reads method-without-symbol on main and
untagged on the head only because the branch predates main's #454 (it is BEHIND by that one
commit). The author's 532 to 534 is measured from the merge base 35f37aaf (482+50), and the +2 is
confirmed. After a squash onto current main the count is 535/857. `quoin matrix` shows "no run
evidence" for all 857 criteria, so zero contradicted holds only vacuously.

Reverse gap: none. The new code is owned by FR-019. No stubs or coverage inflation were found.

## Verdict

**CONDITIONAL.** AC-45 and AC-46 are backed case by case except one listed AC-45 case, a repeat
that differs only in `domains`. A mutant that keys detection on (identity, domains) survives
(FND-001, low).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-019-AC-45 requires refusal when the descriptors "differ in any other CG descriptor field". The tests vary `advertised`, `origin` and `bounds`, but never `domains`: every fixture has `domains: None`. A reviewer mutant that keys repeat detection on (identity, domains) passes all 48 tests. Fix: add a manifest pair that differs only in `domains`, for example `Some(BTreeSet::new())` or `Some({Integer})` against `None` | tests/it/capability_settlement.rs:1186-1232, spec/routed/functional/FR-019-capability-settlement.md:437 |

## Coverage

- Criteria examined: FR-019-AC-45 (tagged; one listed case unexercised, FND-001), FR-019-AC-46
  (tagged, fully backed), FR-019-AC-10 and FR-019-AC-47 (Analysis rows, not tagged by this PR;
  correct).
- Strict delta: +2 tagged (AC-45, AC-46). Contradicted: 0, vacuously, because no run evidence
  exists.
- Semantic review: run for AC-45 and AC-46.
- Plan completion: not assessed

## Dispositions

Round 1, against head 29e8f650b36869d4b3769b637247ed0d6e9a0bc1.

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | `tc_030_domains_only_change_refuses_without_settlements` (tagged TC-030 and FR-019-AC-45 only) varies only `domains` (`None` against `Some(BTreeSet::new())`) in both orders and asserts the typed refusal. The reviewer-run mutant keying detection on (identity, domains) now fails that test |
