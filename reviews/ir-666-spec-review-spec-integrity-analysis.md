---
id: SR-2222
title: "spec-integrity-analysis of quire-contract-codegen PR #313 (IR-666)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen@cf3d6e4f0712fe4853095bfd51dcbd108e49bf74; FR-029, FR-030, FR-033, TC-040, TC-041, TC-048 as changed by PR #313, plus the unchanged FR-033 Description, FR-033-AC-1/AC-9/AC-11, FR-029-AC-21/AC-27 and TC-048 steps 8-9 read for consistency with the change"
review_set: subset
---

## Summary

Ticket: IR-666. Consistency and completeness of the renumbered F-1 to F-7 rows and the
"QSL-640 delivered" restatement across the six changed documents and the unchanged text they
interact with. Four contradictions or missing owner allocations remain.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Renumbering moved the exact-versus-shadow comparison to F-7, and FR-033 body line 85 was updated, but the preserved IR-635 rows FR-033-AC-11 ("F-6's runtime occurrence-pair count") and TC-048 step 9 ("step 5/F-6 compares") now point at F-6 RefinementCeiling; no owner is allocated to correct them | spec/replay/functional/FR-033-composite-parity-replay-binding.md:271; spec/replay/matrix/TC-048-composite-parity-replay-binding.md:114; spec/replay/functional/FR-033-composite-parity-replay-binding.md:85 |
| FND-002 | medium | FR-033's unchanged Description still says QSL FR-358 is unmerged, the parity APIs are absent from both sources and implementation is gated on QSL-640 delivery, contradicting the same file's new Falsified Settlement and Setup step 3 text that #645 delivered the facade | spec/replay/functional/FR-033-composite-parity-replay-binding.md:48; spec/replay/functional/FR-033-composite-parity-replay-binding.md:56; spec/replay/functional/FR-033-composite-parity-replay-binding.md:183 |
| FND-003 | medium | FR-029-AC-21 (and FR-029 line 152, TC-048 step 8) still bound the interim NonProductionProof/ShadowCounterexample refusals "until actual QSL-640 parity capability delivery"; the PR now states QSL-640 is delivered while the body says interim refusals hold until CG consumes it, so AC-21's condition has lapsed by its own text; no owner (IR-241/IR-635/IR-666) is allocated to restate it | spec/kani/functional/FR-029-run-outcome-terminal-record.md:317; spec/kani/functional/FR-029-run-outcome-terminal-record.md:152; spec/replay/matrix/TC-048-composite-parity-replay-binding.md:97 |
| FND-004 | medium | Common-step refusal reading conflicts: FR-030 Behavior/AC-17 and TC-041 step 16 say a QSL common-step refusal (e.g. edited source with Disagreed) yields Inconclusive(ReplayRefused) with its code, while FR-033-AC-1/AC-9, FR-029-AC-27 and TC-048 step 10 say another source/package/node refuses "with no settlement"/"before any settlement"; the spec does not say whether CG pre-checks (no value) or QSL refuses (value) | spec/kani/functional/FR-030-ir-outcome-terminal-map.md:175; spec/replay/functional/FR-033-composite-parity-replay-binding.md:261; spec/replay/matrix/TC-048-composite-parity-replay-binding.md:119 |

## Verdict

The F-1..F-7 table itself is consistent across FR-029, FR-030, FR-033, TC-040, TC-041 and TC-048
and with QSL #645. The defects are stale or ambiguous neighbouring text the renumbering and the
delivery restatement left behind.

## Dispositions

Round 1, reviewed at agent-ix/quire-contract-codegen@c44edbab9de0245308a5ea414e8774ce8e5356cb (fix diff cf3d6e4..c44edba).

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | c44edba: FR-033-AC-11 now reads "F-7's runtime occurrence-pair count" and TC-048 step 9 "step 5/F-7 compares"; claim shape otherwise unchanged, consistent with the narrow planner allocation |
| FND-002 | fixed | c44edba: FR-033 Description rewritten to state #645 delivered the parity arm and facade, with the CG consumer still planned for IR-666 and the typed unavailable-capability refusal kept until then |
| FND-003 | fixed | c44edba: FR-029-AC-17, AC-21, FR-029 line 152, Status, TC-048 header and step 8 now gate the interim refusals "until CG consumes the delivered QSL parity facade"; AC-21 ownership text (IR-241/IR-635) unchanged |
| FND-004 | fixed | c44edba: FR-030, FR-033 Outputs/Behavior/Setup step 2, FR-033-AC-1/AC-9, FR-029-AC-27, TC-040, TC-041 and TC-048 step 10 now separate CG prechecks/missing/mismatched reports (no terminal value) from a binding-valid QSL `Refused` report (terminal value) |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | low | FR-029-AC-27 keeps its "PLANNED/GATED (IR-635/QSL-640)" tag but the fix added IR-666's binding-valid `Refused`-report terminal rule (non-fault ReplayRefused, fault Failed) to it; the planner allocation lets this PR move only the AC-11/TC-048 step 9 F-6 references in IR-635 rows, so the new clause's owner is unstated | spec/kani/functional/FR-029-run-outcome-terminal-record.md:328 |

## Dispositions (round 2)

Round 2, reviewed at agent-ix/quire-contract-codegen@9003f83f52a7cb7476915c20a75e843b295c1a04 (fix diff c44edba..9003f83). Changed lines re-checked for regressions of FND-001 to FND-004: none.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-005 | fixed | 9003f83: FR-029-AC-27 is byte-identical to base 7439865 (IR-635 text restored); the IR-666 Refused-report rule moved to new FR-029-AC-28 tagged "PLANNED CG CONSUMER (IR-666)"; FR-029 Status names AC-28 (IR-666); TC-040, TC-041, TC-048 and FR-030-AC-17 reference AC-28 |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-006 | low | New FR-029-AC-28 is not registered consistently: the matrix index rows (spec/kani/matrix/tests.md:50 "FR-029-AC-17, FR-029-AC-19 through FR-029-AC-27", Planned IR-635; spec/replay/matrix/tests.md:35 TC-048) omit it, as spec/kani/matrix/tests.md:51 omits FR-030-AC-15..17 since the first head; the FR-029 "Planned (IR-635)" Purpose bullet now spans "AC-19 to AC-28"; and AC-28 declares Inspection although it requires an executable public-facade invocation (`quire matrix` reports method-without-symbol) | spec/kani/functional/FR-029-run-outcome-terminal-record.md:332; spec/kani/functional/FR-029-run-outcome-terminal-record.md:100; spec/kani/matrix/tests.md:50; spec/kani/matrix/tests.md:51; spec/replay/matrix/tests.md:35 |

## Dispositions (round 3)

Round 3, reviewed at agent-ix/quire-contract-codegen@9844bdcef77a6ef58e725f60359ef18233d9efdd (fix diff 9003f83..9844bdc). Changed lines re-checked for regressions of FND-001 to FND-005: none; FR-029-AC-27 is unchanged and IR-635-owned.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-006 | fixed | 9844bdc: the FR-029 IR-635 Purpose bullet is back to "AC-19 to AC-27" and a separate IR-666 bullet names AC-28; FR-029-AC-28 is IR-666-tagged with verification Test; spec/kani/matrix/tests.md adds an FR-029-AC-28 row (TC-040, TC-041, TC-048, Planned IR-666), an FR-030-AC-15..17 row (TC-041, Planned IR-666) and AC-28/AC-15..17 in the TC-040/TC-041 summaries; spec/replay/matrix/tests.md adds an FR-029-AC-28 row and lists it under TC-048; TC-041 now `verifies` FR-029; `quire matrix` reports AC-28 and FR-030-AC-15..17 as untagged planned criteria |

## New findings (disposition pass 3)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-007 | low | The index now lists TC-041 as verifying FR-029-AC-28 and TC-041 gained a `verifies FR-029` edge, but no TC-041 step or expected result cites FR-029-AC-28; the Refused-report pass-through cases in step/expected 16 are attributed only to FR-030-AC-17, so the TC-041 trace for AC-28 is unstated | spec/kani/matrix/TC-041-ir-outcome-terminal-map.md:8; spec/kani/matrix/tests.md:51; spec/kani/matrix/tests.md:84 |

## Dispositions (round 4)

Round 4, reviewed at agent-ix/quire-contract-codegen@4033aefb1996951b4c9c361500e3d64d936c7e89 (fix diff 9844bdc..4033aef, two lines in TC-041). No regression of FND-001 to FND-006; no new finding.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-007 | fixed | 4033aef: TC-041 step 16 now cites FR-029-AC-28 on the reachable `prepare` non-fault `Refused` report pass-through case, and expected result 16, covering that report, the `Fault`/`Admission(Fault)` Failed reading and the no-terminal-value precheck/wrong-claim cases, cites "(FR-029-AC-28; FR-030-AC-17)" |

## Rebase confirmation

Rebased head agent-ix/quire-contract-codegen@ec8d2806ff8d1a9cd9929ea949c2c30072728259 on main 194229bf16b6eaca79ae74bed7e7daed572ad164 (IR-631 #314), replacing review-clean head e89eb68ac0c90f4b199cc07b7b0a8fab78ec376c on old main 743986589149474da3da78b40f9395d21e936090. `git range-diff` shows all seven commits equivalent; the only change is the conflict context in old 9844bdc -> new 13f36eb, spec/replay/matrix/tests.md. In that file at ec8d280, the `| FR-032 | FR-032-AC-1 through FR-032-AC-14 | TC-047 |` and `| TC-047 |` rows are byte-identical to 194229bf; the `| FR-033 | FR-033-AC-1 through FR-033-AC-13 | TC-048 |` (QSL #645 delivered) row, the `| FR-029 | FR-029-AC-28 | TC-048 |` row and the `| TC-048 |` row (FR-029-AC-28, IR-635 claim / IR-666 consumer) are byte-identical to e89eb68; every other line equals 194229bf. The other 11 PR files equal e89eb68 and the other 11 main files equal 194229bf; `git diff 194229bf ec8d280` and `git diff 7439865 e89eb68` both report 12 files, +464/-117. No drift and no new finding.
