---
id: SR-1642
title: "IR-635 spec review (integrity): criterion parsing, matrix indexes and stale owners"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen#298; spec/kani/functional/FR-025-generated-subject-abi.md, spec/kani/functional/FR-028-bounded-proof-ceilings.md, spec/kani/functional/FR-029-run-outcome-terminal-record.md, spec/replay/functional/FR-033-composite-parity-replay-binding.md, spec/replay/matrix/TC-048-composite-parity-replay-binding.md, spec/spec.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-025
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-029
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-033
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-048
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-040
    type: reviews
---

# SR-1642: IR-635 spec review, integrity

## Summary

Ticket: IR-635. PR: quire-contract-codegen#298 (spec only; the reviewed head is recorded in the private ticket marker). Reviewer: claude-opus-5-5, session 2f7447c0-1934-4031-8988-fab17e2f41b7. I computed the matrix with `quire matrix --scope . --format tsv` (quire 0.36.1, engine 0.50.1) on the candidate. FR-033-AC-1 to AC-10 and FR-029-AC-17 read `untagged`, and FR-029-AC-6 reads `tagged` by `tests/it/terminal_map.rs:203`. FR-025-AC-9 does not appear in the matrix at all. Five findings: one high and four medium.

## Method

I ran the computed matrix, then compared each changed criterion with the hand-maintained indexes: spec/tests.md, spec/kani/matrix/tests.md, spec/replay/matrix/tests.md, TC-040 and interface-001. I used the FR-032/TC-047 slice as the precedent for how a planned replay FR is indexed. I also checked the built criteria FR-029-AC-6 and AC-15 against the new planned route.

## Verdict

**FAIL**: one high finding (FND-001). Clean: the FR-033 and TC-048 frontmatter, ids and relationship types validate. Every FR-033 criterion has a single `Test` verification cell, and the spec/spec.md registry row resolves both new links.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-025-AC-9 is not a criterion. A blank line separates its row from the Acceptance Criteria table (lines 101-103), so it parses as a header-less fragment. The computed matrix lists FR-025-AC-1 to AC-8 and no AC-9. `quire validate` passes, so the planned composite leaf-binding obligation silently has no matrix row, and TC-048's "FR-025-AC-9" binding names nothing. Remove the blank line so the row joins the table, then confirm with `quire matrix`. | spec/kani/functional/FR-025-generated-subject-abi.md:102 |
| FND-002 | medium | FR-029-AC-6 now mixes a built clause and a planned one: "ordinary map returns no Tested" plus "an admitted composite parity route returns it only under AC-17 ... PLANNED (IR-635)". The computed matrix reports AC-6 `tagged` by `tc_040_no_outcome_maps_to_tested`, which asserts only the ordinary map, and spec/kani/matrix/tests.md:43 lists AC-6 as Covered. The planned composite `Tested` clause therefore reads as covered. Keep AC-6 as the ordinary-map criterion and move the composite `Tested` clause into AC-17 or into a new planned criterion. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:255, spec/kani/matrix/tests.md:43, tests/it/terminal_map.rs:203 |
| FND-003 | medium | The matrix indexes do not cover the new criteria. Under the FR-032/TC-047 precedent, spec/replay/matrix/tests.md and spec/tests.md should gain an FR-033 row and a TC-048 row, but neither has one. spec/kani/matrix/tests.md has no FR-025-AC-9 row, and its TC-036 row still ends at AC-8. Its FR-029-AC-17 row (line 47) still reads "Planned (IR-241) ... each shadow strength ... NonProductionProof ... pending QSL's answer (FR-029 Open Questions)". This PR removed that Open Questions section. The TC-040 row (line 75) still lists FR-029-AC-17, but AC-17's verification cell no longer names TC-040 and TC-048 now verifies it. Add the FR-033 and TC-048 index rows and update the FR-025, FR-029-AC-17 and TC-040 rows. | spec/replay/matrix/tests.md:20, spec/tests.md:20, spec/kani/matrix/tests.md:47, spec/kani/matrix/tests.md:75 |
| FND-004 | medium | Several documents outside the diff still state the AC-17 this PR replaced, so they now contradict FR-029. TC-040 steps and expected results (lines 46, 60, 85 and 102) map every shadow strength to `NonProductionProof` and say "No value is Tested". interface-001 line 140 gives the old IR-241 semantics. FR-029's own Status (line 303) says "FR-029-AC-17 (IR-241) ... planned". A reader of TC-040 or interface-001 would build the superseded map. Update these to the IR-635 map, or point them at FR-029-AC-17 and TC-048. | spec/kani/matrix/TC-040-run-outcome-terminal-record.md:85, spec/core/functional/interface-001-codegen-api.md:140, spec/kani/functional/FR-029-run-outcome-terminal-record.md:303 |
| FND-005 | medium | Built FR-029-AC-15 still says each non-falsified outcome, including `verified`, "given a settlement is refused with `TerminalPairError::UnexpectedSettlement`". The new route "consumes a same-claim QSL settlement for verified and falsified shadows" (FR-029 Description, line 61), and AC-17 maps such verified shadows to Proved, Tested or Incomplete. Read literally, AC-15 refuses the verified composite settlement that AC-17 requires. Scope AC-15 to the source-predicate `ReplaySettlement` input, or state that the composite settlement is a distinct typed input that AC-15 does not govern. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:264, spec/kani/functional/FR-029-run-outcome-terminal-record.md:60 |

## Dispositions

Round 1 re-check of the fix-round candidate of PR #298, covering every original finding and regressions in the fix. The fixing commit is recorded in the private ticket marker.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | The blank line is gone and FR-025-AC-9 now joins the Acceptance Criteria table. The computed `quire matrix` at the fix head lists FR-025-AC-9 as `untagged`. The kani matrix has an FR-025-AC-9 row (TC-036, TC-048) marked Planned with "no executable trace coverage claimed", and TC-036 gains a step 9 that delegates to TC-048. |
| FND-002 | fixed | FR-029-AC-6 now reads only "The ordinary production/source-predicate map returns no `Tested`." The composite `Tested` clause moved to planned FR-029-AC-19. The computed matrix still shows AC-6 tagged by `tc_040_no_outcome_maps_to_tested`, which asserts exactly that ordinary map, so no planned composite coverage is claimed. |
| FND-003 | fixed | spec/replay/matrix/tests.md gains an FR-033 row (AC-1 to AC-10 to TC-048, Planned, UNVERIFIED SOURCE, Gated) and a TC-048 summary row. spec/tests.md lists FR-033 and TC-048 in the Replay row. The kani matrix adds FR-025-AC-9 and extends the TC-036 row to AC-9. Its FR-029-AC-17 row now covers AC-17 and AC-19 to AC-27 under TC-048, without the removed Open Questions reference, and the TC-040 row drops AC-17. Every new row is marked planned, and none claims executable coverage. |
| FND-004 | fixed | TC-040 now scopes its description and steps 6 and 14 to the ordinary source-predicate map. Its step 16 and expected result 16 delegate the planned parity map to TC-048, with the interim refusals held only until QSL-640. interface-001 run_terminal_value semantics now describe the distinct typed composite input, AC-17 and AC-19 to AC-27, UNVERIFIED SOURCE and the QSL-640 gate. FR-029 Status names "FR-029-AC-17 and AC-19 to AC-27 (IR-635, actual QSL-640 gate)". |
| FND-005 | fixed | FR-029-AC-15 is now scoped "For the ordinary source-predicate `ReplaySettlement` input" and ends "The distinct typed composite-parity settlement input is governed by AC-17 and AC-19 to AC-27, not this criterion." New FR-029-AC-27 states that AC-15 does not reject a valid verified-parity settlement. The Behavior and Outputs sections say the same. The computed matrix still tags AC-15 with `tc_040_a_settlement_accompanies_a_falsified_outcome_only`, which exercises only the ordinary input. |
