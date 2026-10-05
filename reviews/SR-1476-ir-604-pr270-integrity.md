---
id: "SR-1476"
title: "CG PR 270 spec review (integrity): NFR-006 real-Kani lane gating and TC-045"
type: SpecReview
analysis: integrity
review_set: subset
date: "2026-10-04"
scope: "agent-ix/quire-contract-codegen@3da228d9eccae9196372574bd16e58a1ef5264f6; spec/kani/non-functional/NFR-006-real-kani-lane-gating.md, spec/kani/matrix/TC-045-real-kani-lane-gate.md, spec/kani/matrix/tests.md, spec/core/matrix/suites.md, spec/spec.md, spec/tests.md (diff origin/main...HEAD, base 250dc84)"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/NFR-006
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-045
    type: reviews
---

# SR-1476: CG PR 270 spec review (integrity)

## Summary

Ticket: IR-604. PR: agent-ix/quire-contract-codegen#270 at 3da228d, base 250dc84 (`origin/main`,
the merge base). Scope is `git diff origin/main...HEAD` only. Review set: subset (integrity,
criterion-strength, scope-boundary, ears-conformance). The base checklist is folded into this file.

Measured at the base:

- `make kani` runs one `cargo +1.98.1 test --locked -j 4 --test it --target-dir target-codex-backends`
  under `flock /tmp/agent-e-heavy-build.lock` with `--ignored --test-threads=1` and the six filters
  `kani_obligations skeleton_spine kani_witness_join bounded_kani_corpus kani_generation
  kani_batching` (Makefile:83-86). As stated.
- `ci: fmt-check spec lint msrv deny audit-unsafe rustdoc test` (Makefile:211); `make -n ci` prints
  no line containing `kani`. As stated.
- `.github/workflows/ci.yml` is `workflow_dispatch` only and installs no Kani. As stated. The PR does
  not touch `.github/`.
- 23 `#[ignore = "kani lane: ..."]` tests exist across seven `tests/it` files, all reachable by the
  six filters (`kani_obligations` also matches `kani_obligations_state_frame`).
- Cost figures: SR-1459 111 s, SR-1423 157 s / 279.8 s / 398 s, SR-647 908 s, SR-690 1056 s are in
  those files. See FND-002 for what does not match.
- `quire validate` on the branch exits 0 with no diagnostic on the new files. `quire coverage --strict`:
  67 unbacked on main, 80 on the branch. The 13 added rows are exactly NFR-006-AC-1 to AC-12 and
  the TC-045 Test Case Summary row. As claimed.
- spec.md, spec/tests.md, kani tests.md and suites.md rows agree on NFR-006 / TC-045 and on
  "planned". No research or delivery-order content, no compatibility layer.

## Verdict

Changes requested. One high (the merge rule contradicts the skip rule), one medium (cost sources
misattributed), two low.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The merge rule and the skip rule contradict each other. Statement 9 and AC-10 say a Kani-touching PR is not merged until its body carries a `result=passed`, `tree=clean`, head-matching line. Statements 10 and 11, AC-11 and the Rationale ("the merger decides on it; this requirement does not say who may accept it") describe a Kani-touching PR that ran no lane and may still be merged. Under statement 9 a `not run` PR can never merge, so AC-11's "merged without a lane" case cannot exist; under the Rationale it can, and AC-10 is then false for it. Two mergers would act differently. Fix in this PR: either a `not run` line keeps the PR unmergeable (the skip statement only records why it is waiting), or name the exception and who may grant it, and narrow statement 9 and AC-10 to match. Who may accept a skip is an owner question. | spec/kani/non-functional/NFR-006-real-kani-lane-gating.md:37-43, spec/kani/non-functional/NFR-006-real-kani-lane-gating.md:111-113, spec/kani/non-functional/NFR-006-real-kani-lane-gating.md:136-137 |
| FND-002 | medium | Cost and source citations do not match their sources. (a) "The IR-604 ticket reports 2277 to 2789 s for the full lane serial": the IR-604 description has no such figure and the ticket has no comments. It gives about 530 s for the quire-integration exemplar. No file in the repository has 2277 or 2789. (b) "six modules at 1056 s (SR-690)": SR-690 records 9 tests over four modules (kani_obligations, kani_obligations_state_frame, kani_witness_join, skeleton_spine). (c) "157 to 398 s (SR-1423, SR-1424)": SR-1424 records no timing, and 398 s is a mutant run in SR-1423, not the unmodified head. (d) "the repository's one workflow": there are two (`ci.yml` and `cla.yml`); only `ci.yml` is relevant. The rejection of the scheduled lane and of `make ci` rests on these figures. Cite only what the sources hold, or measure the full lane and record it. | spec/kani/non-functional/NFR-006-real-kani-lane-gating.md:78-82, spec/kani/non-functional/NFR-006-real-kani-lane-gating.md:93 |
| FND-003 | low | "The defects this repository has merged from exactly that behaviour were found by running the lane" lists four items. Only two are merged defects found with real Kani: IR-451 (#260) and the missing V1 bundle/corpus cover (#262/#264). FR-031-AC-20's overflow is a property of the code #268 added, not a defect that was merged. The IR-29 item is a weakness in the lane's oracle, not a merged defect. SR-1423 shows the default test does catch a `value + 1` decoder; only a decoder fixed at the synthetic pair (1, 5) passes the default suite. Reword as "behaviour only real Kani shows" or cite the two defects alone. | spec/kani/non-functional/NFR-006-real-kani-lane-gating.md:64-75 |
| FND-004 | low | SUITE-012 registers `make kani-gate` as a suite, but no TC-045 step runs under it. TC-045 steps 1 to 9 run in the default `cargo test` (`tests/it/kani_gate.rs`) with stand-ins, and `make kani-gate` runs SUITE-011's tests again. The row ties no evidence to TC-045 and duplicates SUITE-011. Drop it, or say it is the lane run that AC-10 and AC-12 inspect. | spec/core/matrix/suites.md:19, spec/kani/matrix/TC-045-real-kani-lane-gate.md:21-23 |

## New findings (disposition pass 1)

Found in round 1, reviewed at d4e8ed1de90cd8bd420bf87d1469a109d198ca76.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | medium | The new merge-skew statement does not say which pull requests it covers. "If `origin/main` holds a commit that touches the Kani-touching set and the pull request's head does not contain it, then the merger shall not merge the pull request until the head contains it and `make kani-gate` has passed on the new head" applies to every pull request, including a spec-only one that `make kani-scope` reports `not required`. That contradicts the Rationale ("is not charged to any other change") and is wider than AC-14, which covers only "a merged pull request that touched the Kani-touching set". Fix: begin the statement with "If a pull request touches a path in the Kani-touching set and ...". | spec/kani/non-functional/NFR-006-real-kani-lane-gating.md:48-50, spec/kani/non-functional/NFR-006-real-kani-lane-gating.md:99-100, spec/kani/non-functional/NFR-006-real-kani-lane-gating.md:165 |

## Dispositions

Round 1, reviewed at d4e8ed1de90cd8bd420bf87d1469a109d198ca76 (fix commit d4e8ed1 on top of 3da228d).

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | d4e8ed1: one rule. The merger shall not merge a set-touching PR without a `result=passed`, `tree=clean`, head-matching line (statement lines 45-47). A `not run` line keeps "the merge rule above still holds" (51-52). AC-15 requires such a PR to be unmerged. The Rationale "One rule, no merge on a skip" and the Open decision (waiver is the owner's call; until the owner rules, no merge) agree. No statement, AC, metric or TC step still implies a not-run PR may merge. |
| FND-002 | fixed | d4e8ed1: the 2277-2789 s figure is gone. Re-checked each citation against its file: SR-1459 111 s; SR-1423 157 s on the unmodified head (round 1); SR-647 908 s for three scalar tests; SR-690 nine tests over four modules, 1056 s; IR-604 about 530 s for the quire-integration exemplar. "Neither workflow" names ci.yml and cla.yml. "23 tests across seven files" matches the count of `kani lane` ignores. |
| FND-003 | fixed | d4e8ed1: "Two merged defects" (IR-451/#260, #262/#264). FR-031-AC-20 and IR-29 are now "properties shown only by real Kani and not defects that merged". |
| FND-004 | fixed | d4e8ed1: the SUITE-012 row is removed. suites.md is identical to main. |

Round 2, reviewed at 1c26606b6e97ddc8795a19e5d3cdb04ed4e2fba4 (fix commit 1c26606 on top of d4e8ed1).

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-005 | fixed | 1c26606: the merge-skew statement now opens "If a pull request touches a path in the Kani-touching set, `origin/main` holds a commit that touches the set and the pull request's head does not contain it, then the merger shall not merge ..." (NFR-006:49-51). It matches AC-14 and no longer charges a PR outside the set. |
