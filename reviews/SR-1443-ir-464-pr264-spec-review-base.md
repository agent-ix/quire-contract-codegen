---
id: "SR-1443"
title: "IR-464 spec review: the PLANNED to implemented flip for FR-015-AC-53 to AC-58, and the TC-023, TC-025 and matrix edits"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@20b3dff4f65ed8d57e51d4d7d80654cce60c1064; spec/kani/functional/FR-015-bounded-kani-obligations.md, spec/kani/matrix/TC-023-bounded-kani-profile-corpus.md, spec/kani/matrix/TC-025-bounded-kani-obligations.md, spec/kani/matrix/tests.md"
relationships: []
---

# SR-1443: IR-464 spec review (PR #264)

## Summary

Ticket: IR-464. PR: agent-ix/quire-contract-codegen#264, head 20b3dff. This PR changes no
criterion's substance. It removes `PLANNED (IR-464)` from AC-53 to AC-58, adds one sentence to
AC-7 that brings the V1 bundle and corpus kinds under it, rewrites the planned paragraph in
FR-015 and TC-023 as "Implemented", drops "(planned, ...)" from the TC-025 heading, and flips the
two matrix rows to Covered.

## Method

- Diffed the four files against the merge base, and grepped `spec/`, `src/` and `tests/` for any
  stale `Planned (IR-464)` / `planned, IR-464` / `PLANNED (IR-464)` text. None is left.
  `AD-004` still says, at lines 112-125, that the bundle and corpus "emit no cover". That
  section is a dated survey of the old file layout (it cites `src/kani.rs:314`), so I do not
  count it as a finding.
- Checked that each matrix row agrees with the tests it cites. The AC-53/54/56/58 row (TC-025)
  and the AC-55/57 row (TC-023) match the tests I ran; see SR-1442.
- `quire validate --strict` at the head shows only the pre-existing FR-017 line 152 EARS warning.
  This PR adds no warnings. `quire coverage`: 65 unbacked rows before and after, the same set.
- Checked the brief's claim that "TC-025 item 17 adds Falsified and MissingCoverSummary". The
  diff does not change item 17. Only the heading of the TC-025 section changed. See FND-001.
- Checked that the test's extra outcomes (`MissingCoverSummary` and `Falsified`) are consistent
  with FR-015-AC-56. They are: AC-56 asks for `Verified`, and for `CoverUnsatisfied` that is
  "never `Verified` and never `Falsified`". The extra arms add to that and contradict nothing.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | TC-025 item 17 still describes only the `Verified` and `CoverUnsatisfied` runs. The test it backs also asserts `MissingCoverSummary` for the cover-stripped bundle and `Falsified` for a broken `ensures`, and the TC-025 matrix row in `tests.md` names `Falsified`. TC-023 was updated to name its new tests; TC-025 was not, so the test case trails both its test and its matrix row. | spec/kani/matrix/TC-025-bounded-kani-obligations.md:161-165 |

## Verdict

The flip is truthful: every AC marked implemented is backed by a test that fails without the
code, the matrix rows are accurate, and no stale planned text is left. The one finding is a
low-severity doc lag in TC-025 item 17.

## Dispositions

Round 1, checked at 86714933f2f9907cbf76dfa8efa4b58ba27110fa.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 86714933f2f9907cbf76dfa8efa4b58ba27110fa. TC-025 item 17 now names all four runs `tc_025_real_kani_classifies_the_v1_bundle_verified_vacuous_and_falsified` asserts: `Verified`, `Inconclusive` as `MissingCoverSummary` with the cover removed, `CoverUnsatisfied` (0 of 1), and `Falsified` for a broken `ensures`. |

Round 1 also re-read the fix's other spec edits:
- the AC-58 rewrite
- the FR-015 Behavior bullet
- TC-025 item 18

Each matches the code I measured (SR-1442). `quire validate --strict` shows only the old FR-017 line 152 warning. No new finding.
