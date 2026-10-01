---
id: "SR-671"
title: "CG PR 214 gap analysis: seam AD invariants and routed gaps"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@80243027bcda41f1d81e83718bd29738926b6972 (review), ae98754466eef0f9754e043157e4bcb9bf552600 (disposition pass 1); spec/assurance/AD-002-cg-qsl-replay-seam.md, spec/assurance/AD-003-evidence-chain.md"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/AD-002
    type: references
  - target: ix://agent-ix/quire-contract-codegen/AD-003
    type: references
---

# SR-671: CG PR 214 gap analysis

## Summary

Ticket: IR-324. PR: agent-ix/quire-contract-codegen#214 at 8024302. Spec-only PR, so there
is no production code to trace. This gap analysis checks three things:

- whether each candidate invariant (R-1 to R-8, E-1 to E-9) is testable and is not a minted
  requirement id;
- whether each routed gap (R-Q1 to R-Q8, R-S1 to R-S8, R-I1 to R-I3) has an owner and a stated
  need without being decided;
- whether each stated "current state" gap matches the code at origin/main.

Plan completion: not assessed.

Measured:

- **Invariants.** All 17 carry local labels and the AD says they are not requirement ids. 15
  name an observable test or inspection. E-6 is vague (FND-002). R-6 and R-8 are marked
  blocked, with their blocker named.
- **Routed gaps.** The ADs carry R-Q1, R-Q2, R-Q3, R-Q5, R-Q7, R-Q8, R-S2 and R-S3, each with
  an owner and a stated need. The PR body carries the full tables: R-Q1 to R-Q8 (R-Q4
  withdrawn), R-S1 to R-S8 and R-I1 to R-I3. Each row has an owner and a need, and none is
  decided in the AD. QSL items that come only from ticket text are labelled as QSL's review or
  as relayed.
- **Current-state gaps.** These match origin/main: no `ObligationIdentity` computation, the
  transcript digest in the slot, the `[1; 32]` test value, the empty declared-domain list, the
  hand-written contract literals, no terminal map in `src`, and no count on
  `KaniExecutionEvidence`. The omissions are recorded in SR-670 (FND-003 there).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | AD-003 names R-I1, R-I2 and R-I3 by id only ("IR-owned items R-I1 and R-I3 are in IR-347's reopened scope"). The committed AD states none of their needs under those ids. The needs appear only in the PR body, which does not last, and the Open-questions rows (:287-288) state them without the ids. Map each id to its need in the AD | spec/assurance/AD-003-evidence-chain.md:314-315 |
| FND-002 | low | E-6, "A stale report from a previous run cannot be read as this run's verdict", names no report, no reader and no observable. It is not testable as written. Name the artifact (for example the Kani JSON report path under the run's target dir) and the check (a pre-existing report is removed or refused before classification) | spec/assurance/AD-003-evidence-chain.md:129 |
| FND-003 | low | AD-002's failure table has no row for CG-side failures that occur after `call_site` and before `replay_frame`. `FrameReplayError::Envelope(WitnessRefusal)` comes from `WitnessEnvelope::reconstruct` (`frame_replay.rs:186`), and `FrameReplayError::Name` is also missing. The table's QSL-refusal row covers only `replay`/`replay_frame` | spec/assurance/AD-002-cg-qsl-replay-seam.md:87-94 |

## Verdict

The routing discipline holds. Every routed row has an owner and a stated need. None is
decided, no requirement id is minted, and relayed QSL text is labelled. The three findings are
low: an AD-local id-to-need mapping, one untestable invariant, and one missing failure-table
row. The substantive gaps are in SR-670.

## Dispositions

Round 1, reviewed at ae98754466eef0f9754e043157e4bcb9bf552600.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | f254ff6: AD-003 gains a "To IR" table mapping R-I1, R-I2 and R-I3 to their stated needs (AD-003:359-364) |
| FND-002 | fixed | f254ff6: E-6 names the observable. A stale artifact plus a launcher that prints nothing classifies `NoVerdict` (AD-003:144-148), and `NoVerdict` exists (`kani_execution.rs:291`) |
| FND-003 | fixed | f254ff6: AD-002's failure table adds rows for `FrameReplayError::Name`, `Transcript` and `Envelope`, plus wrapped `CallSite` faults (AD-002:91-93) |
