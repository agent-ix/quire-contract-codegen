---
id: SR-514007
title: "IR-364 PR #574 spec-review/base review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; PR #574 head 505c39bbba422e9469665656bc664842aabcd35d from main 1944268f; spec/evidence/functional/FR-004-vacuity-evidence.md; IR-364"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-004
    type: reviews
---

## Summary

Base checklist review of PR #574, which edits FR-004's planned V2 Inputs bullet and its Open
Questions section only. Ticket: IR-364. The change binds each selected clause's execution point
to the explicit typed point in the V2 generation request, authenticated against the accepted QSL
binding or, absent a binding point, the caller's typed invocation context under FR-011, failing
closed and with no clause-kind default. Requirement/revision identity (FR-011 OQ-2) stays open.

Checked against base 1944268f: FR-004-AC-16 to AC-24 are byte-identical and all nine remain
`PLANNED (IR-815)`; no AC, TC or status changed. OQ-2 is not closed: the new OQ text states it
"remains the open FR-011 OQ-2 decision" and supplies no identity source. No inferred point,
default or substitute is introduced ("clause kind supplies no default"); the binding-else-caller
source order is the decided two-source rule, worded as in #572's FR-011/FR-002 text. No hash,
digest, pin or tracking record is added. `quire validate` on FR-004 passes.

Sibling interaction (measured with `git merge-tree`): #574 merges cleanly with #572 and #573,
and **conflicts textually with #571** in FR-004 (same Inputs bullet and OQ section).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Conflicts with sibling #571 in FR-004: both rewrite the same V2 Inputs bullet and the OQ entry, and each declares the other's subject still open (#574: identity open; #571: execution anchor open, label OQ-1). Whichever merges second must hand-combine both resolutions; a naive resolution leaves one decided subject marked open. Recommended order: #572, then #574, then #571 rebased (#571 also conflicts with #572 in FR-002/FR-011). | spec/evidence/functional/FR-004-vacuity-evidence.md:27-33, spec/evidence/functional/FR-004-vacuity-evidence.md:242-248 |
| FND-002 | medium | The new fail-closed rule ("A missing or mismatched point is refused before runner publication; clause kind supplies no default") sits in Inputs with no FR-004 Behavior or AC, cites no verifying criterion or TC (FR-011-AC-6/AC-7 and TC-020 step 8 exist only in #572; FR-004's TC-006 has no execution-point case), names neither refusal (`MissingExecutionPoint`, `ExecutionPointMismatch`) nor the refusing component, and uses "runner publication", a term #572 does not define (it says "no artifact"). | spec/evidence/functional/FR-004-vacuity-evidence.md:29-32 |
| FND-003 | low | "Each selected clause" is undefined in FR-004: the V2 analyzer's population is every executable clause of the admitted package (same bullet, FR-004-AC-16), so it is unclear whether unselected population clauses need a point or are out of the rule. | spec/evidence/functional/FR-004-vacuity-evidence.md:27-29 |

## Verdict

Mergeable after FND-001 is resolved by merge order (land #572 first, then #574; #571 rebases
last and combines both OQ resolutions). FND-002 should be fixed in this PR by naming the FR-011
criteria/TC and refusal codes and replacing "runner publication" with #572's "no artifact"
wording; FND-003 is a wording clarification. The core content is sound: AC-16..24 unchanged and
Planned, OQ-2 left open, no inferred point/default/fallback, wording consistent with #572.

## Coverage

```yaml
scope:
- id: FR-004
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: "Each selected clause's execution point is the explicit typed point carried in the V2 generation request and authenticated against its accepted QSL binding, or against the caller's typed invocation context when no binding supplies a point, under FR-011. A missing or mismatched point is refused before runner publication; clause kind supplies no default."
- id: FR-004-OQ-2
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: "OQ-2 (requirement and revision identity): the V2 source of these identities remains the open FR-011 OQ-2 decision. The explicit per-clause execution point above settles FR-011 OQ-1 for this analyzer; it does not supply or infer requirement or revision identity."
- id: FR-004-AC-16
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: "PLANNED (IR-815). The expected clause population and implication counts of the V2 analyzer equal those independently derived from the admitted `CheckedPackageV2` clause bodies."
- id: FR-004-AC-17
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: "PLANNED (IR-815). Source inspection shows the V2 analyzer opens a `CheckedPackageV2` body only through the `core/ir/` layer of AD-004, and names no `BoundPackage` or `ClauseRef`."
- id: FR-004-AC-18
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: "PLANNED (IR-815). A V2 clause whose generated source map declares an implication count unequal to the count derived from its body yields a structured binding non-success naming the clause and no classification."
- id: FR-004-AC-19
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: "PLANNED (IR-815). Over a V2 clause population, a clause whose oracle evaluation was observed, that has at least one implication and for which no expected consequent probe was observed is `vacuous`."
- id: FR-004-AC-20
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: "PLANNED (IR-815). Over a V2 clause population, a mapped clause whose oracle-evaluation region has no positive execution count is `unexecuted`."
- id: FR-004-AC-21
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: "PLANNED (IR-815). Over a V2 clause population, an implication-bearing clause for which some but not all expected consequent probes were observed is `partially_exercised`."
- id: FR-004-AC-22
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: "PLANNED (IR-815). Over a V2 clause population, an implication-bearing clause with every expected consequent probe observed, and an implication-free clause with its evaluation region observed, is `exercised`."
- id: FR-004-AC-23
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: "PLANNED (IR-815). Over a V2 clause population, removing one clause, one consequent probe, one evaluation probe, or every clause from the generated artifacts, each alone, prevents successful analysis."
- id: FR-004-AC-24
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: examined
  excerpt: "PLANNED (IR-815). Over a V2 clause population, each malformed input class of FR-004-AC-5 yields a non-success outcome with no invented classification."
- id: FR-011
  path: spec/strategy/functional/FR-011-numeric-harness-campaigns.md
  role: context_only
  excerpt: "(PR #572) When generation is requested, the generator shall require the request's typed execution point to equal the selected clause's accepted QSL binding point when present, or the caller's typed invocation point when the binding supplies none. A missing source or missing request point shall yield `MissingExecutionPoint`; a mismatch shall yield `ExecutionPointMismatch`."
- id: FR-004-OQ-1@571
  path: spec/evidence/functional/FR-004-vacuity-evidence.md
  role: context_only
  excerpt: "(PR #571) OQ-1 (anchor open; identity resolved): the V2 source of a clause's execution anchor remains FR-011 OQ-1."
```

## New findings (disposition pass 1)

Re-reviewed at head b4876d1fa4910916777ed5a36d82064757c7967e against main d6f09905 (#572 merged).
AC-1..24 are byte-identical to main and AC-16..24 remain `PLANNED (IR-815)`; only AC-25 is added.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | medium | Wording conflict with companion #575: FR-004's new Behavior bullets and AC-25 restate FR-011's generator refusals but omit #575's `StrategyErrorCode::MissingClaimOccurrence`, so a wrong-role, wrong-ordinal or other-node occurrence reads under FR-004-AC-25 as an "absent point for the selected tuple" (`MissingExecutionPoint`) while #575 FR-011-AC-7 requires `MissingClaimOccurrence`; FR-004 also names the "IR-1013 accessor" and `CheckedNodeId` where #575 says "Contract IR FR-058 typed accessor" and "checked clause id". The occurrence-keyed text also contradicts merged main FR-011, which is still node-keyed, until #575 lands. | spec/evidence/functional/FR-004-vacuity-evidence.md:183-195, spec/evidence/functional/FR-004-vacuity-evidence.md:225 |
| FND-005 | medium | FR-004-AC-25 is untraced: TM-007 has no row for it, TC-006's Traces To omits it, and its cited TC-020 has no `verifies` FR-004 relationship and is listed in the strategy matrix for FR-011/NFR-004 only; on main TC-020 steps 7-8 have no two-occurrence case (that procedure exists only in #575). | spec/evidence/functional/FR-004-vacuity-evidence.md:225, spec/evidence/matrix/tests.md:13-19, spec/strategy/matrix/TC-020-numeric-harness-campaigns.md:5-9 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | deferred | #571 (head 7eadf528) is still open and still conflicts with #574 in FR-004 and with main in FR-002/FR-011; resolution belongs to #571's rebase. Merge order: #575, then #574, then #571 rebased last. |
| FND-002 | fixed | Behavior now names the CG V2 strategy generator, `MissingExecutionPoint` / `ExecutionPointMismatch`, no artifact before publication, and AC-25 cites TC-020 steps 7-8. |
| FND-003 | fixed | Inputs now states the expected population is every executable clause and that strategy generation selects one clause and one authentic claim occurrence. |
| FND-004 | fixed | Round 2, head 457a9ab2 on main 1208404f (#575 merged): FR-004 Behavior and AC-25 now defer to merged FR-011-AC-7's occurrence-keyed refusals (which include `MissingClaimOccurrence`), name the Contract IR FR-058 accessor and use "checked clause id". |
| FND-005 | fixed | Round 2: TM-007 has an FR-004-AC-25 row (TC-006, TC-020), TC-006 traces AC-25 with a new planned control section, TC-020 has a `verifies` FR-004 edge, and merged TC-020 steps 7-8 carry the two-occurrence and wrong-occurrence cases. |
