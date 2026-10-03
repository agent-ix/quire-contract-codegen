---
id: SR-1008
title: "CG PR 241 criterion strength: FR-021-AC-22 and its mutation row"
type: SpecReview
analysis: criterion-strength
review_set: subset
scope: "agent-ix/quire-contract-codegen@b4309b85673304ed2ba9b580847af46e8fce338e; FR-021-AC-22, FR-021-AC-22 mutation row, TC-031 step 9, src/oracle/function/mod.rs (git diff origin/main...HEAD, base 85b8114)"
---
# SR-1008: CG PR 241 criterion strength

## Summary

Ticket: IR-540. I checked whether AC-22 and TC-031 step 9 can fail, and whether its mutation row
is non-vacuous.

- The mutation row's first clause is "key classification by position but resolve an item's
  function by name or node id". That is close to today's behavior. Against the code at b4309b8,
  step 9's first fixture, with both bodies admissible, produces two `Generated` items with crossed
  function names and indices, not `DuplicateDeclaringNode`. Step 9's assertion fails.
- The second clause is "refuse only the first so the sibling survives and reports
  UnknownFunction". Step 9's second fixture catches it in two ways. The surviving sibling's item is
  generated, not refused. And the refused function's item is `UnknownFunction`. Both contradict
  the "every item ... carries `DuplicateDeclaringNode` (never `UnknownFunction`)" assertion.
- The second symptom from the ticket is covered: a Stage 1 refusal plus a same-node sibling must
  not yield `UnknownFunction`.
- The distinct-node-id clause catches an over-broad fix that refuses the whole request.

The criterion can fail, and the row is not vacuous.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | AC-22 says the declarations "are each refused", and step 9 says to "Assert every such declaration ... carries" `DuplicateDeclaringNode`. A declaration has no claim-map entry, though. The public output (`ExactFunctionOracles`: artifacts, `claim_map`, `location_map`) shows a declaration's refusal only through items that name it, and through its absence from `checked_package()` and the location map. Restate the declaration half as observable facts: absent from `checked_package()`, absent from `location-map.json`, and every item naming it refused. Name the location map explicitly, because the overwritten `Origin::Body` index shows up there | spec/oracle/matrix/TC-031-function-application-oracles.md:103 |
| FND-002 | low | "Items naming functions with distinct declaring node ids are generated unchanged" has no stated baseline. Once the duplicates are excluded, the surviving functions' `Origin::Body` indices are positions among the survivors, so they differ from a run in which the duplicates were admissible. State the oracle: each such item's claim equals the claim the same request produces with the duplicate declarations removed. That gives step 9's "generated unchanged" a comparison it can actually make | spec/oracle/functional/FR-021-function-application-oracles.md:248 |

## Verdict

Strong: AC-22 can fail, and its mutation row describes real failures that step 9 would catch,
including the ticket's second symptom. Two lows tighten the wording so the test has observable
targets and a concrete baseline.

## Dispositions

Reviewed at agent-ix/quire-contract-codegen@08a7f0366888bad18e710b6b9287b543fff0db61 (fix-round delta b4309b8..08a7f03). The expanded mutation row is still not vacuous: each new clause is caught by a step 9 fixture. Fixture (iii) catches a name check placed first. Running both request orders catches refusing only the later-sorted declaration. The removed-duplicates baseline catches refusing the whole request.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 08a7f03 |
| FND-002 | fixed | 08a7f03 |
