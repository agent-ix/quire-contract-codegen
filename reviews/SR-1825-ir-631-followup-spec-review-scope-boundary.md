---
id: SR-1825
title: "IR-631 follow-up scope boundary: CG criteria versus QSL-owned evaluation order"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/quire-contract-codegen@f84c4fc93fe4ba5f8460e60bb8e75998d5cbcfd8; spec/replay/functional/FR-032-routed-scalar-replay-binding.md, spec/replay/matrix/TC-047-routed-scalar-replay-binding.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-032
    type: reviews
---

# SR-1825: IR-631 follow-up scope boundary

## Summary

Ticket: IR-631. PR: quire-contract-codegen, not yet opened. It is spec only. Reviewer: claude-opus-5-5, session 8950150c-5937-457d-9408-c19aa15729b8, run f92b6bc1-4101-4d05-affe-ea238bac36af. One low finding.

## Method

I checked how responsibility is split among CG (preimage, retention, setup, converter), the driver (native execution and authentication), QSL (recompile, identity verify, exact evaluation, terminal causes) and IR-648 (typed operand accessor). I also checked each changed criterion for an obligation CG cannot discharge or that belongs to another owner. Merged FR-357 was read for the QSL-owned contract.

## Verdict

**PASS with one low finding.** The ownership is stated cleanly:

- CG mints through QSL's shared function and copies no encoder.
- The driver authenticates the observation.
- QSL never sees the artifact.
- IR-648 owns the accessor.
- Function-level value parity is out of scope.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | AC-7 makes QSL's internal order a CG criterion: native `Incomplete`/execution fault becomes `GeneratedFault` "before QSL operand admission, even with an out-of-range operand". Merged FR-357 does not specify that order. It states operand admission "before any evaluation" and native-fault `Failed` "without running the comparison", and the order exists only in `scalar.rs::compare` source. CG's own setup refuses out-of-range operands before dispatch (AC-3, FR-024), so CG's pipeline cannot reach this case. A CG test of it calls QSL directly and tests QSL. Keep AC-7 to CG's conversion of each outcome, and leave the order to FR-357 or a QSL ticket. | spec/replay/functional/FR-032-routed-scalar-replay-binding.md:298 |

## Dispositions

Round 1 re-check of fix commit `832633d7afa778e8a3688601595beb5d38917cb5` (run 733cf463-44a3-4ff9-83f7-a1a0a32f9463, model claude-opus-5-5). Each finding was verified against the spec text at that commit, not against the author's receipt. The reviewed content of `f84c4fc93fe4ba5f8460e60bb8e75998d5cbcfd8` was confirmed unchanged after the rebase onto main. The planned criteria remain unrun; no implementation, mutation coverage or settlement proof is claimed.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 832633d7afa778e8a3688601595beb5d38917cb5: AC-7 no longer asserts the native-fault-before-admission order; FR-032 labels it a QSL source observation and attribution only, and TC-047 step 8 drops the combined out-of-range test. |
