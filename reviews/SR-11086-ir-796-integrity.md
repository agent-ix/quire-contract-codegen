---
id: SR-11086
title: "IR-796 proof boundary integrity review"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-contract-codegen; spec/oracle/matrix/TC-050-caller-text-admission.md, spec/oracle/matrix/tests.md, spec/kani/matrix/tests.md"
review_set: subset
---

## Summary

Ticket: IR-796. The added status note is consistent with FR-035-AC-4/5 and FR-017-AC-26. The matrices continue to classify TC-050 as Planned.

## Reviewed units

| Unit | Role | Excerpt |
| --- | --- | --- |
| TC-050 Proof Boundary | examined | A `MemoryExhausted` Kani result is inconclusive and gives no proof credit for FR-035-AC-4 or FR-035-AC-5. |
| TC-050 Observation | examined | Non-normative observation: IR-796 records that the original NFC `Text[1,1]` candidate from PR #381 reached CBMC and ended `MemoryExhausted` under the published 1 GiB ceiling. |
| Oracle matrix FR-035 and TC-050 | examined | Planned (IR-494, IR-796); AC-4/5 have no real-Kani proof credit; twelve verified harnesses and the falsifying mutation control remain open. |
| Kani matrix FR-035 and TC-050 | examined | Planned (IR-494, IR-796); one original NFC `Text[1,1]` real-Kani run is inconclusive and production proof evidence remains open. |
| FR-035-AC-4 | context_only | For each of the six profiles and both `Text[0,8]` and `Text[1,1]`, a real Kani harness symbolically ranges over all 21 sequences of zero to two scalars from the four-scalar alphabet, constructs each `TextPayload` by `from_utf8`, compares the generated admission oracle with a separate direct runtime call including outcome and metering, and ends with one satisfied cover after its assertions. |
| FR-035-AC-5 | context_only | In a copy of the generated oracle with its admission call or declared `TextType` altered so the oracle accepts a payload the direct runtime call refuses, real Kani falsifies the admission agreement assertion with concrete playback; restoring the call verifies over the same symbolic class. |
| FR-017-AC-26 | context_only | A generated FR-035 caller Text-admission harness enters the same production execution path in single and compatible batch runs; its evidence carries the exact typed caller identity. |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

PASS: no criterion is weakened or recast as a static enumeration, fixed literal, increased ceiling, or completed proof.

The reported 72.021-second, 1,089,802,240-byte observation and phase detail appear in the candidate and a Linear comment. The original run receipt is on another host and was unavailable to this reviewer; this review makes no independent execution claim.
