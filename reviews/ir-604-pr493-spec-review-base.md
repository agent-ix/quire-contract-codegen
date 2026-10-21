---
id: SR-15224
title: "IR-604 PR493 base review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; spec/replay/functional/FR-016, spec/replay/matrix/TC-026, spec/replay/matrix/tests.md"
review_set: subset
---

## Summary

Reviewed FR-016, TC-026 and the replay matrix in PR #493 at head 415b634ab5c0b6960a1cf2c914b4415256a21349. Measured KaniExecutionEvidence and ProofCeilings against QSL WitnessArmResult charges and CG ReplayInputs.

## Verdict

**CONDITIONAL — a rationale sentence conflates CG decode input bounds with QSL replay bounds.**

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The new rationale says ReplayInputs and ReplayLimits bound QSL replay, but ReplayInputs.decode_limits bounds CG witness decoding before replay. Name the specific replay-limit fields and preserve the distinct AC-6 decode bound in this sentence. | spec/replay/functional/FR-016-witness-native-replay.md:169-171 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | fab64478 names the two QSL replay-limit fields and distinguishes the independent CG witness decode bound; none is Kani proof accounting. |
