---
id: SR-12005
title: "spec-review/failure-domain review of IR-796 TC-050 proof-bound disposition"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-contract-codegen; spec/oracle/matrix/TC-050-caller-text-admission.md, spec/oracle/functional/FR-035-caller-text-admission.md, spec/oracle/matrix/tests.md, spec/kani/matrix/tests.md @ f9164277dccc"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/TC-050
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-035
    type: references
---
# SR-12005: spec-review/failure-domain, IR-796 disposition (CG #412)

## Summary

Ticket: IR-796. PR quire-contract-codegen#412 at f9164277dcccf08c03d2a7d23adbae4aaa6b5fde, diff against codex/ir-494-caller-text-admission.

Checks the disposition's classification of what counts as weakening. It wrongly classes a raised unwind bound as criterion weakening.

## Scope

- examined: TC-050 Measured Proof-Bound Disposition (no shape fits) (spec/oracle/matrix/TC-050-caller-text-admission.md:140-146): No harness shape that keeps the public constructor, the direct runtime admission call and full outcome and metering agreement fits the published ceiling with this runtime and toolchain. A narrower assertion, stubbing of runtime admission or metering, a raised ceiling or unwind bound, or a per-harness unwind override would weaken FR-035-AC-4 or FR-035-AC-5, and none is accepted as discharge. FR-035-AC-4 and FR-035-AC-5 remain planned and unrun, and they have no proof credit.
- examined: TC-050 Measured Proof-Bound Disposition (memcmp) (spec/oracle/matrix/TC-050-caller-text-admission.md:136-138): Separately, `assert_eq!` on two `Meter` values compares the ten-counter array through CBMC's `memcmp` model, whose loop fails its unwinding assertion at `--unwind 8` even with no admission call.
- examined: FR-035-AC-4 (spec/oracle/functional/FR-035-caller-text-admission.md): For each of the six profiles and both `Text[0,8]` and `Text[1,1]`, a real Kani harness uses one symbolic selector over a table bijective with all 21 sequences of zero to two scalars from the four-scalar alphabet, constructs each selected `TextPayload` by `from_utf8`, compares the generated admission oracle with a separate direct runtime call including outcome and metering, and ends with one satisfied cover after its assertions. The proof identity names this finite class and its type.
- examined: FR-035-AC-5 (spec/oracle/functional/FR-035-caller-text-admission.md): In a copy of the generated oracle with its admission call or declared `TextType` altered so the oracle accepts a payload the direct runtime call refuses, real Kani falsifies the admission agreement assertion with concrete playback; restoring the call verifies over the same symbolic class.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | TC-050:142-144 says a raised unwind bound or a per-harness unwind override 'would weaken FR-035-AC-4 or FR-035-AC-5'. With unwinding assertions on, a higher unwind bound checks more iterations: it is a published-resource-bound change, not a weaker criterion, and neither AC names --unwind 8. Failure scenario: a sound route, such as an override covering the 80-byte memcmp loop, is rejected as 'weakening' rather than taken to a bound amendment, and the wrong boundary gets enforced. | spec/oracle/matrix/TC-050-caller-text-admission.md:140-146 | wrong-requirement |

## Verdict

Changes requested: separate 'weakens a criterion' (narrower assertion, stubbing) from 'changes the published bound' (ceiling, unwind).
