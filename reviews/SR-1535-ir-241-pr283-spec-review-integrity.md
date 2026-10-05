---
id: SR-1535
title: "Integrity review of quire-contract-codegen PR #283 (composite-equality shadow contract)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen@1f3025ea99c3fe1fa804ed43fce116e339f9c96d; spec/kani/functional/FR-028-bounded-proof-ceilings.md, spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/oracle/functional/FR-018-composite-equality-oracles.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md, spec/kani/matrix/TC-039-bounded-proof-ceilings.md, spec/oracle/matrix/TC-029-composite-equality-oracles.md, spec/kani/matrix/tests.md, spec/oracle/matrix/tests.md, spec/assurance/AD-001-codegen-architecture.md"
review_set: subset
---
# Integrity review of PR #283

## Summary

Ticket: IR-241 (also IR-264). This review checks the PR diff against `main` e3d3ab4 for
completeness, consistency and atomicity. It covers the 21 new criteria (FR-028-AC-13 to AC-23,
FR-015-AC-69 to AC-76, FR-018-AC-21 and AC-22), their Behavior statements, the 13 mutation rows,
and TC-025 steps 29 to 36, TC-039 steps 11 to 21 and TC-029 steps 13 and 14.

The numbering does not collide with anything. FR-028-AC-10 and AC-11 were retired from an earlier
draft of the tractability-record design: the SR-1384 and SR-041 dispositions on `main` reference
them, and no live document reserves them, so starting at AC-13 is safe. FR-015 ends at AC-68 on
`main` and FR-018 ends at AC-20. No open PR (#282, #272, #222, #209) and no local in-flight branch
adds FR-015, FR-018 or FR-028 criteria at these numbers. `make spec` gives 6 warnings at base and
the same 6 at head, all in FR-024, with no errors. `quire coverage` moves from 320/459 to 320/480
rows backed: 21 new planned rows and none newly backed, so the strict coverage delta is clean.

Four findings. Two medium: FR-028-AC-21 contradicts its own Open Question, and FR-028-AC-13 to AC-23
are called family-agnostic although they are written for equality. Two low: FR-028-AC-21 overlaps
AC-3, and FR-028-AC-13's crate exclusion is wider than the shadow can satisfy.

## Scope examined

- FR-028-AC-13 to FR-028-AC-23 (examined), with the matching Behavior bullets and mutation rows.
- FR-015-AC-69 to FR-015-AC-76 (examined), with the IR-264 Behavior bullets and the unsupported-shape table.
- FR-018-AC-21, FR-018-AC-22 (examined), with their Behavior bullets and mutation rows.
- FR-028-AC-3, FR-028-AC-7 (context_only: overlap check).
- TC-025 steps 29 to 36, TC-039 steps 11 to 21 and Expected Results 11 to 21, TC-029 steps 13 and 14 (examined).
- spec/kani/matrix/tests.md and spec/oracle/matrix/tests.md rows and Traces To (examined).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-028-AC-21 fixes the mechanism ("by observing the tree's peak resident memory"), but FR-028's Open Question "How a memory ceiling is observed" leaves the choice between RSS polling, an address-space limit and a control group open. The AC also refuses a platform only where the generator "can neither observe nor limit" memory, yet it requires "every execution evidence records that peak". A platform that can limit memory (RLIMIT_AS) but cannot observe peak RSS passes the refusal clause and fails the evidence clause. The author's own probe used a 12 GB address-space cap while CBMC's RSS was about 4 GB, so the two quantities differ. | spec/kani/functional/FR-028-bounded-proof-ceilings.md FR-028-AC-21; Open Questions "How a memory ceiling is observed" |
| FND-002 | medium | FR-028-AC-13 to AC-23 are called "family-agnostic" in the Description and the Planned paragraph, but AC-15 compares the `Completed` verdict and the admitted `equality.pair` count, and AC-16's boundary set is defined over integer, Boolean and option leaves. AC-13 names `quire_contract_runtime` and `quire_exact`. The next shadow families on the Checklist (function application, kernel types) cannot meet AC-15 as written. The criteria should either be scoped to equality or state the comparison as a per-family parameter. | FR-028-AC-15; FR-028-AC-16; FR-028 Description Planned (IR-241) paragraph |
| FND-003 | low | FR-028-AC-21 restates FR-028-AC-3 (a memory-exceeding run settles inconclusive memory-exhausted naming the ceiling) with different terms. AC-3 says "stopped", AC-21 says "killed whole". AC-3 names the ceiling, AC-21 names the ceiling and the peak. Two criteria can now be backed by different tests that disagree. AC-14's last sentence also restates AC-7. AC-21 could replace AC-3's text by reference. | FR-028-AC-21; FR-028-AC-3; FR-028-AC-14; FR-028-AC-7 |
| FND-004 | low | FR-028-AC-13 bans any path into "any crate that oracle's crate depends on". Read literally, that includes `core`, `alloc` and `std`, and the shadow needs them (`Option` is `core::option::Option`). An implementer has to guess which exemptions were intended. The excluded crates should be named, or `core`, `alloc`, `std` and `kani` exempted explicitly. | FR-028-AC-13 |

## Verdict

The PR is structurally sound. Ids are collision-free, the matrices and Traces To lists match the new
criteria one for one, the TC steps are numbered without gaps (TC-025 runs to 36, TC-039 Procedure
and Expected Results both run to 21, TC-029 runs to 14), and nothing is marked covered that is not.
The two medium findings are internal consistency defects to fix before merge.

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed 3e0bc7c | FR-028-AC-21 is now mechanism-neutral ('observes or limits'). The evidence records the mechanism, and the peak only where memory is observed. A platform with no mechanism is refused. The Open Question is rewritten to match and notes that an address-space limit is not an RSS limit. |
| FND-002 | fixed 3e0bc7c | FR-028-AC-14 and AC-15 compare 'the observables the family names', with composite equality's given as the first, and the Description says the criteria name each family's observables. |
| FND-003 | fixed 3e0bc7c | FR-028-AC-21 now 'settles as FR-028-AC-3 states', and AC-14 defers a missing obligation to FR-028-AC-7. |
| FND-004 | fixed 3e0bc7c | FR-028-AC-13: 'names no crate but core, alloc, std and kani'. |
