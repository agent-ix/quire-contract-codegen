---
id: SR-1942
title: "IR-652 lifecycle spec review (integrity)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen@6b9cbd21f2db16abc6e6f777f11a372b689c9080; spec/kani/functional/FR-017-kani-execution-evidence.md, spec/kani/functional/FR-034-caller-death-ownership.md, spec/kani/matrix/TC-027-kani-execution-evidence.md, spec/kani/matrix/TC-049-caller-death-ownership.md (frozen candidate diff against main f4c37b253250671fdd990a635da963d013b5478c; PR not open)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-034
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-049
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: reviews
---

# SR-1942: IR-652 lifecycle spec review, integrity

## Summary

Ticket: IR-652. The PR is not open; this reviews frozen candidate 6b9cbd21f2db16abc6e6f777f11a372b689c9080.
Reviewer: claude-opus-5-5, session 8dfb2e4c-0a0e-4cdf-b7f0-98e42705d45a, run
5066cb1a-6ce7-4ef1-9919-36e3a3ff45a7. Structural completeness, consistency and atomicity of the
amended criteria and matrix entries. One high, one medium and two low findings.

## Method

Ran `quire validate` on each changed file (all exit 0) and `quire matrix --format tsv` at the
candidate head. Compared the criterion ids the matrix computes for FR-034 with the rows in the
FR-034 table. Checked the TC-049 Expected Results table against every changed and new criterion,
and read FR-017-AC-19 beside FR-034-AC-32 and AC-33 for duplicated ownership. Scope units
examined: FR-034-AC-30 to AC-34 (table structure), FR-034-AC-23 and AC-26 (references to
AC-31), the FR-034 Dependencies IR-652 bullet, the TC-049 Description and Expected Results, steps
15 and 16, FR-017-AC-19, and TC-027 Unnamed report storage.

## Verdict

**FAIL: one high, one medium and two low findings.** The four files validate, but the four new
criteria do not exist as criteria in the computed model.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-034-AC-31 to AC-34 come after a blank line (line 533) that ends the Acceptance Criteria table. They form a separate pipe block with no header row. `quire matrix` lists FR-034 criteria only up to AC-30, and `quire validate` passes silently. So the four mandatory UNRUN gates are not criteria. AC-23, AC-26, the Dependencies bullet and the TC-049 description all reference ids that do not exist. Remove the blank line, then confirm the matrix lists AC-31 to AC-34 as `untagged`. | spec/kani/functional/FR-034-caller-death-ownership.md:532-537 |
| FND-002 | medium | The TC-049 Expected Results table has no row naming FR-034-AC-31, 32, 33 or 34, although steps 15 and 16 exercise them. So the matrix entry states no required observation or caught regression for the four new criteria. | spec/kani/matrix/TC-049-caller-death-ownership.md:282-293, spec/kani/matrix/TC-049-caller-death-ownership.md:265-277 |
| FND-003 | low | FR-017-AC-19 restates FR-034-AC-32 and AC-33 almost word for word: the 16 MiB+1 retention, concurrent pipe and backing charge, the Completed, lease-close, teardown, EOF, seal, read ordering, and all-owner reclamation. It packs about seven obligations into one row, so the same norm has two owners that can drift apart. Keep the argv locator and "no named report" in FR-017-AC-19, and reference FR-034-AC-32 and AC-33 for the collector. | spec/kani/functional/FR-017-kani-execution-evidence.md:278, spec/kani/functional/FR-034-caller-death-ownership.md:535-536 |
| FND-004 | low | Hard line breaks leave sentence fragments on their own lines: FR-034 lines 66 to 68 ("All roles,", "startup,", "collection ..."), FR-034 lines 234 to 239 ("pins,", "captures", "nor", "production"), TC-027 line 63 ("Concurrent-accounting,") and TC-049 lines 271 and 272 ("original" / unindented "caller-exclusive"). These are wrap artifacts, not content. Reflow them to the 100-column style. | spec/kani/functional/FR-034-caller-death-ownership.md:66-68, spec/kani/functional/FR-034-caller-death-ownership.md:234-239, spec/kani/matrix/TC-027-kani-execution-evidence.md:62-63, spec/kani/matrix/TC-049-caller-death-ownership.md:271-272 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | low | Wrap fragments introduced by this branch remain in TC-049: line 20 ("claimed. Use the real"), line 47 ("INIT before spawning monitor,"), line 128 ("failure. The") and line 275 ("lease liveness immediately before"). The file also now ends without a final newline. Reflow them and restore the newline. | spec/kani/matrix/TC-049-caller-death-ownership.md:20, spec/kani/matrix/TC-049-caller-death-ownership.md:47, spec/kani/matrix/TC-049-caller-death-ownership.md:128, spec/kani/matrix/TC-049-caller-death-ownership.md:275 |

## Dispositions

Round 1 reviewed a661f2f297f5e9860c25e9995226d07332426ab2 (previous 6b9cbd21f2db16abc6e6f777f11a372b689c9080; PR not open). Reviewer: claude-opus-5-5, session 8dfb2e4c-0a0e-4cdf-b7f0-98e42705d45a, run 9a4f4275-0799-4c05-867e-b01504c3095c.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | a661f2f297f5e9860c25e9995226d07332426ab2: Blank line removed; quire matrix at the fix head lists FR-034-AC-31..34 untagged. |
| FND-002 | fixed | a661f2f297f5e9860c25e9995226d07332426ab2: Expected Results rows added for FR-034-AC-31, 32, 33 and 34. |
| FND-003 | fixed | a661f2f297f5e9860c25e9995226d07332426ab2: FR-017-AC-19 keeps argv, unnamed authority and absence semantics and references FR-034-AC-32/33 for the collector. |
| FND-004 | fixed | a661f2f297f5e9860c25e9995226d07332426ab2: The cited fragments in FR-034, TC-027 and TC-049 step 16 are reflowed; other TC-049 fragments are recorded as new FND-005. |

Round 2 reviewed 6f552cd97c8a6915d999e03d49129ebc3c198195 (previous a661f2f297f5e9860c25e9995226d07332426ab2; PR not open). Reviewer: claude-opus-5-5, session 8dfb2e4c-0a0e-4cdf-b7f0-98e42705d45a, run 04443041-0ffa-49c7-a688-2004c61268ae. Only the disposition-pass-1 findings were open; every original finding's latest row is already fixed.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-005 | fixed | 6f552cd97c8a6915d999e03d49129ebc3c198195: TC-049 is whitespace-normalized identical to a661f2f, has no short wrap fragments left in steps or description, and ends with a newline again. |
