---
id: "SR-1337"
title: "CG PR 252 spec review (integrity): FR-021-AC-24 trace, matrix and ID hygiene"
type: SpecReview
analysis: integrity
review_set: subset
scope: "agent-ix/quire-contract-codegen@46488a7adeace7fe692081bba425ba99a83eb8b8; spec/oracle/functional/FR-021-function-application-oracles.md, spec/oracle/matrix/TC-031-function-application-oracles.md, spec/oracle/matrix/tests.md (diff origin/main...HEAD, base 6d54143)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/TC-031
    type: references
---

# SR-1337: CG PR 252 spec review (integrity)

## Summary

Ticket: IR-545. What I checked:

- ID hygiene. `git log --all -S'FR-021-AC-24'` finds only the PR commit 46488a7, so the id was
  never issued before (ADR-0056 never-reissue holds). AC-24 follows AC-23 with no gap.
- Matrix. `tests.md` adds the row `FR-021 | FR-021-AC-24 | TC-031 | 🚧 Planned`. The TC-031 index
  row lists FR-021-AC-24, and its status names AC-24 as Planned. TC-031 already `verifies` FR-021,
  so it needs no new relationship.
- TC-031. Main had steps 1-11, so step 12 is new, not a collision. Its four sub-cases (i)-(iv)
  match AC-24's four examples one for one. Expected Results gains a matching AC-24 clause marked
  🚧 Planned.
- Gates at the PR head. `make spec` exits 0 with existing warnings only (inline-data-schema,
  import-unresolved, DuplicateArchetype, DuplicateInverseEdge). `quire coverage --scope .
  --strict` reports 66 unbacked rows at the head and 66 at the base. FR-021 is 22/24 (AC-18 and
  AC-24 Planned).
- Consistency. AC-24 does not contradict FR-021-AC-1, AC-13 or AC-22; see SR-1336.

## Verdict

The trace and matrix are consistent and the id is fresh. Two low form findings.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The TC-031 Description names what the test verifies, AC by AC: AC-19 to AC-21, AC-22 and AC-18. It does not mention AC-24, although step 12 and Expected Results now cover it. AC-23 was already missing from it. | spec/oracle/matrix/TC-031-function-application-oracles.md:13-30 |
| FND-002 | low | AC-24's criteria cell starts with "🚧". The repo's own precedent for a criterion specified ahead of its code is the prefix "PLANNED (IR-NNN)." (FR-014-AC-40 to AC-42, FR-018-AC-20 and FR-021-AC-23 in #244). No other AC cell in `spec/` carries 🚧, which is a matrix status marker. Use the precedent form, "PLANNED (IR-545).", so the code PR knows what to strip. | spec/oracle/functional/FR-021-function-application-oracles.md:295 |

## Dispositions

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 5cbed2e7e370275ab8ec87ee719a2ad3dabb0705: the TC-031 Description adds "that a failed lowering record is refused as its own byte-ceiling or work refusal (FR-021-AC-23), and that two different unknown function names on one call node are two `UnknownFunction` entries in byte order (FR-021-AC-24, 🚧 Planned)." |
| FND-002 | fixed | 5cbed2e7e370275ab8ec87ee719a2ad3dabb0705: the AC-24 cell now opens "PLANNED (IR-545).", the exact form of the #244 precedent "PLANNED (IR-547).". No 🚧 is left in any FR-021 AC cell. |
