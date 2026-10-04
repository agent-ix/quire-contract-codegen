---
id: "SR-1373"
title: "CG PR 256 spec review (integrity): totality of the replay-settlement readings, matrix and trace"
type: SpecReview
analysis: integrity
review_set: subset
scope: "agent-ix/quire-contract-codegen@e8d0fe8174e0cbdb446382df967cf62c0f545c93; spec/kani/functional/FR-029-run-outcome-terminal-record.md, spec/kani/functional/FR-030-ir-outcome-terminal-map.md, spec/kani/matrix/TC-040-run-outcome-terminal-record.md, spec/kani/matrix/TC-041-ir-outcome-terminal-map.md, spec/kani/matrix/tests.md, spec/assurance/AD-003-evidence-chain.md (diff origin/main...HEAD); CG error types read at src/replay/function.rs and src/replay/frame.rs; QSL qsl-replay read at 7c2cb303"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-029
    type: references
  - target: ix://agent-ix/quire-contract-codegen/TC-040
    type: references
---

# SR-1373: CG PR 256 spec review (integrity)

## Summary

Ticket: IR-465. Checked: whether the six replay-settlement readings cover every error the replay
path can return (totality over the pair), the AC to TC step trace, the matrix and index rows, and
id hygiene.

- Matrix. `tests.md` rows 31 and 32 list FR-029-AC-1..6 and 8..13, and FR-030-AC-1..12. The TC-040
  and TC-041 index rows list the same ids. `quire coverage` reads FR-029 as 0/12 and FR-030 as
  0/12. TC-040 steps 7 to 12 map one to one onto FR-029-AC-8 to AC-13. TC-041 steps 8 to 11 map
  onto FR-030-AC-9 to AC-12, and its expected result 1 now covers AC-1 ("every expressible pair").
  Consistent.
- Ids are fresh (see SR-1372). The FR-029-AC-7 gap is a deletion recorded in #186, not a reissue.
- Totality. CG's errors at the base were enumerated: `SpineReplayError::{UnboundArgument,
  FieldDelimiter, Transcript, Refused, WrongArm, Identity}`, `ReplayPackageError::{InvalidFunction,
  Dependencies, CallSite}`, `FrameReplayError::{Dependencies, CallSite, Name, Transcript, Envelope,
  Refused}` and `DependencyLockError::{Duplicate, Input}`. Each was checked against the readings.

## Verdict

The trace is consistent. Totality has two holes (FND-001, FND-002), and both must be closed so the
code ticket has a closed input type to build. FND-003 and FND-004 are low.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Totality hole for setup refusals that carry no code. AC-13 and FR-030-AC-12 cover only a setup refusal on data "that carries a QSL catalog code". Three of the setup refusals that AD-003's link 6-7 row and FR-030 name are CG's own errors and have no QSL code: `ReplayPackageError::InvalidFunction`, `FrameReplayError::Name` and `DependencyLockError::Duplicate`. FR-030:89 classifies the whole `DependencyLockError` as a setup refusal. Under the closed-set rule they cannot be `ReplayRefused`, and no row gives them a value after the interim. Fix: say permanently that a setup refusal with no QSL code is a CG defect and maps to `Failed`, or name the QSL code each one takes. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:43-44,142; spec/kani/functional/FR-030-ir-outcome-terminal-map.md:88-90 |
| FND-002 | medium | FR-029-AC-11's closed list of CG-origin failures leaves out variants that exist at the base: `SpineReplayError::Identity` (raised before any replay), and `FrameReplayError::Transcript` (the AC names only `SpineReplayError::Transcript`). Whether these are CG defects or setup refusals is not stated. A test written to the AC passes while either one maps elsewhere. Fix: list each variant, or state the rule ("every CG error that is not a setup refusal is `Failed`") with the list as examples. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:113-119,140 |
| FND-003 | low | FR-029-AC-8 says the "disagreement case carries the replay's `DisagreementCause`", which implies the "completed no value" case carries none. In QSL a replay that completes no value settles `WitnessSettlement::Inconclusive` with `DisagreementCause::NoValue` (result.rs:69), and the only public constructor never gives `Inconclusive` without a cause. Both cases carry one. Fix: say both carry their `DisagreementCause`, or merge the two readings. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:37-38,137 |
| FND-004 | low | The check column of AD-003 link 7 still says "one total `match` each over the outcome" and then "written over the pair". The rest of the PR (E-3, FR-030-AC-7) says the match is over the pair. | spec/assurance/AD-003-evidence-chain.md:59 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | medium | The fix maps `ReplayPackageError::InvalidFunction`, `FrameReplayError::Name` and `DependencyLockError::Duplicate` to `Failed` as CG defects. FR-029 states it as a decision ("The Kani adapter shall map to `Failed` ..."), and nothing marks it as awaiting owner confirmation. The CG spec still classifies the same three the other way. AD-003's link 6-7 failure row (line 77, unchanged) puts `InvalidFunction`, `Name` and `Dependencies` in the "replay setup refused on data" group, mapped to `Inconclusive(ReplayRefused)`, and says "`CallSiteRefusal` exposes no code at this base", which is now false. AD-003 R-Q1 (b) still lists "`InvalidFunction`; `Name`; a dependency-lock refusal" as relayed-QSL case (b) `ReplayRefused`, and its appended HELD note covers only `CallSiteRefusal`. AD-004 step 5 lists `InvalidFunction`, `Name` and `DependencyLockError` the same way. Also, `Duplicate` is the condition QSL itself codes `invalid_package` (`DependencyInputRefusal::DuplicateIdentity`, spine.rs:677), so "carries no QSL code" is CG's choice of where to catch it, not a fact about the data. Reversing a relayed QSL classification is not something this spec should settle unilaterally. Fix: either hold the three with the HELD class, or mark the `Failed` mapping as owner-to-confirm. In both cases, make AD-003 line 77, R-Q1 (b) and AD-004 step 5 agree with FR-029. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:42-45,121-129,150; spec/kani/functional/FR-030-ir-outcome-terminal-map.md:89-92; spec/assurance/AD-003-evidence-chain.md:77,378; spec/assurance/AD-004-cg-crate-layout.md:803-815 |
| FND-006 | low | The totality claim is not qualified for the HELD class. FR-029's Description says "Every pair maps to exactly one terminal value". FR-030-AC-1 says every expressible pair maps to exactly one `TerminalValue`. AD-003 E-3 says "so no falsified run is left without a value". Meanwhile the held setup-refusal row states no value, and FR-029 Status says "until the ruling the code cannot make the map total over this class". As written, FR-030-AC-1 cannot pass while AC-12 is held. Fix: qualify the three statements, for example "every pair outside the class FR-029-AC-13 holds", or say that AC-1 is held together with AC-13 and AC-12. | spec/kani/functional/FR-029-run-outcome-terminal-record.md:28-31,189-190; spec/kani/functional/FR-030-ir-outcome-terminal-map.md:110; spec/assurance/AD-003-evidence-chain.md:164-166 |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-007 | low | The round-2 sweep found AD-004 step 5 left behind. Its totality statements are still unqualified: line 474, "total over the pair (step 5)", and line 786, "The map is total over (Kani outcome, QSL replay result)". FR-029, FR-030, AD-003 E-3 and TC-041 now exclude the held setup-refusal class. Lines 779-781 and 786 still name QSL-351 as the carrier of `Inconclusive(cause)` ("Neither merges before QSL-351", "QSL-351 adds the two causes"), but QSL-351 is Done and its held part is not merged. FR-029 and FR-030 Status already say so honestly. Not blocking. Fix with one clause each: "outside the held class (AD-003 R-Q1)" and "the unmerged QSL `Inconclusive` work". | spec/assurance/AD-004-cg-crate-layout.md:474,779-781,786 |

## Dispositions

Round 1, reviewed at a619ee7e4b34ac632474e93f7a4606fa85989e79. CG error enums re-read at that head (src unchanged by the PR). QSL re-measured at 7c2cb3034839410dae6707e269256d0485e85ec6.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | a619ee7e4b34ac632474e93f7a4606fa85989e79: the codeless CG setup errors now have a row, CG defect to `Failed`, in the FR-029 Description, Behavior and AC-11, and in the FR-030 table and Behavior. `DependencyLockError` is split: `Input` is HELD and `Duplicate` is `Failed`. The totality hole is closed. Whether that classification is the spec's decision to make is new finding FND-005. |
| FND-002 | fixed | a619ee7e4b34ac632474e93f7a4606fa85989e79: AC-11 now lists `SpineReplayError::{UnboundArgument, FieldDelimiter, Transcript, WrongArm, Identity}`, `FrameReplayError::{Transcript, Envelope, Name}`, `ReplayPackageError::InvalidFunction`, `DependencyLockError::Duplicate`, the out-of-bound playback and the decode failure. Checked against the enums: every remaining variant has another reading. `Refused` goes by its walked content, `CallSite` is a fault or HELD, `Dependencies` is `Duplicate` or `Input`, and `EvidenceFailureCause::Verdict` is the disagreement reading. Complete. |
| FND-003 | fixed | a619ee7e4b34ac632474e93f7a4606fa85989e79: FR-029-AC-8 now reads "a replay disagreement of each `DisagreementCause` (`Verdicts`, `Witness` and `NoValue`) maps to `Inconclusive(ReplayParity)` carrying that `DisagreementCause`". These are exactly QSL's three variants (result.rs:59-90). TC-040 step 7 matches. |
| FND-004 | fixed | a619ee7e4b34ac632474e93f7a4606fa85989e79: AD-003 link 7's check column now reads "one total `match` each over the pair". |

Round 2, reviewed at 657af41a983cd05381ec5495d3c0117397a53070 (fix commit 657af41 on top of a619ee7). QSL re-measured at 7c2cb3034839410dae6707e269256d0485e85ec6 (unchanged).

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-005 | fixed | 657af41a983cd05381ec5495d3c0117397a53070: `InvalidFunction`, `FrameReplayError::Name` and `DependencyLockError::Duplicate` (and `Input`) are now HELD with the setup-refusal class under one ruling. This holds in the FR-029 Description, Behavior, AC-11, AC-13 and Status, the FR-030 table, Behavior and AC-12, TC-040 step 12, TC-041 step 11 and the tests.md rows. FR-029-AC-11 no longer maps them to `Failed`. AD-003 line 77 marks the second group HELD, and its "exposes no code" sentence is replaced by the measured fact. AD-003 R-Q1 names the whole class. AD-004 step 5 marks the `Failed` interim superseded. The spec no longer settles the reversal. |
| FND-006 | fixed | 657af41a983cd05381ec5495d3c0117397a53070: the FR-029 Description says "the totality and exactly-one claims hold for every pair outside that class". FR-030's Description and AC-1 say "other than a `Counterexample` with a setup refusal on data (held with FR-030-AC-12)". AD-003 E-3 says "outside the held setup-refusal class". TC-041 expected result 1 says "outside the held setup-refusal class (step 11)". tests.md says "AC-1 is held with it for that class". FR-030-AC-1 and TC-041 are consistent. The AD-004 leftovers are FND-007. |

Round 3, reviewed at 14d9d7f994a695e930d92f3d43b2f8eaaf827fd9 (a docs-only fix commit on top of 657af41 that touches only AD-003 and AD-004). QSL re-measured at 7c2cb3034839410dae6707e269256d0485e85ec6 (unchanged).

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-007 | fixed | 14d9d7f994a695e930d92f3d43b2f8eaaf827fd9: AD-004:474 now reads "total over the pair, outside the held setup-refusal class (step 5)". AD-004:786-787 now reads "the two causes are pending in QSL. The map is total over (Kani outcome, QSL replay result), outside the held setup-refusal class below". AD-004:779-781 and the risk row at 1284 now read "depend on QSL types that are pending in QSL (`Inconclusive(cause)`, a `NonZero` `Proved`; the tool pin is already gone, #551) ... Neither merges before those types are in QSL". The AD-003 edits are true against QSL main. Line 76 says "outside the held setup-refusal class". Section (a) and line 337 say QSL-351 merged only #551 and option B is pending. R-Q1 says "pending in QSL" and that the codes "are already in QSL `main`". Verified: no `ToolPin`/`tool_pin`/`toolchain_pin` in qsl-replay (6360c035, #551); no `Inconclusive` variant or `NonZero` in proof_result.rs. The leftover sweep of AD-001..AD-004, FR-029, FR-030, TC-040, TC-041 and the kani matrix found no unqualified totality or "exactly one" claim, no "Neither merges before QSL-351", no "exposes no code", and no setup-refusal classification that disagrees with the HELD class. Pre-existing tool-pin tense outside this PR's diff (AD-003 R-Q9 "has a public `tool_pin`", AD-004:541 and 777 "the tool pin QSL-351 removes/drops") is wording only and is not recorded as a finding here. |
