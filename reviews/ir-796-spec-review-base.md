---
id: SR-12002
title: "spec-review/base review of IR-796 TC-050 proof-bound disposition"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen; spec/oracle/matrix/TC-050-caller-text-admission.md, spec/oracle/functional/FR-035-caller-text-admission.md, spec/oracle/matrix/tests.md, spec/kani/matrix/tests.md @ f9164277dccc"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/TC-050
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-035
    type: references
---
# SR-12002: spec-review/base, IR-796 disposition (CG #412)

## Summary

Ticket: IR-796. PR quire-contract-codegen#412 at f9164277dcccf08c03d2a7d23adbae4aaa6b5fde, diff against codex/ir-494-caller-text-admission.

Base checklist over the four-file IR-796 disposition diff. AC wording is unchanged and AC-4/5 stay planned, unrun and uncredited, so no criterion is weakened and nothing is credited. The disposition pins tool versions in test-case prose, and FR-035 carries measurement narrative that belongs to the TC or the ticket.

## Scope

- examined: TC-050 Measured Proof-Bound Disposition (bound) (spec/oracle/matrix/TC-050-caller-text-admission.md:104-107): IR-796 measured whether a different harness shape could satisfy FR-035-AC-4 and FR-035-AC-5 under the unchanged published bound: 1 GiB per obligation, 600 seconds, `--unwind 8`, Cadical, Kani 0.68.0 with CBMC 6.11.0 and the Contract Runtime revision in this repository's lockfile.
- examined: FR-035 Dependencies Proof cost (spec/oracle/functional/FR-035-caller-text-admission.md:121-127): FR-035-AC-4 and FR-035-AC-5 depend on Contract Runtime's Text admission and metering path completing in Kani within the published 1 GiB ceiling under `--unwind 8`. IR-796 measured that it does not, for any harness shape that keeps these criteria. The disposition and its next owner are recorded in TC-050's Measured Proof-Bound Disposition section. The criteria are unchanged and stay planned and unrun until that dependency is met.
- examined: FR-035-AC-4 (spec/oracle/functional/FR-035-caller-text-admission.md): For each of the six profiles and both `Text[0,8]` and `Text[1,1]`, a real Kani harness uses one symbolic selector over a table bijective with all 21 sequences of zero to two scalars from the four-scalar alphabet, constructs each selected `TextPayload` by `from_utf8`, compares the generated admission oracle with a separate direct runtime call including outcome and metering, and ends with one satisfied cover after its assertions. The proof identity names this finite class and its type.
- examined: FR-035-AC-5 (spec/oracle/functional/FR-035-caller-text-admission.md): In a copy of the generated oracle with its admission call or declared `TextType` altered so the oracle accepts a payload the direct runtime call refuses, real Kani falsifies the admission agreement assertion with concrete playback; restoring the call verifies over the same symbolic class.
- examined: Oracle matrix FR-035 row (spec/oracle/matrix/tests.md): AC-4/5 are unrun and have no real-Kani proof credit. IR-796 measured that no criterion-preserving harness shape fits the published 1 GiB / `--unwind 8` bound, because one concrete `admit_text` call already exceeds it. TC-050 records the disposition and names Contract Runtime as next owner; twelve verified harnesses and the falsifying mutation control remain open.
- examined: Oracle matrix TC-050 row (spec/oracle/matrix/tests.md): PR #381 candidate exists, but the measured NFC `Text[1,1]` run is inconclusive, IR-796's measured proof-bound disposition places the cost floor in runtime admission and metering (next owner: Contract Runtime), and the twelve-harness proof and mutation control are unrun and uncredited
- examined: Kani matrix FR-035 row (spec/kani/matrix/tests.md): IR-796 measured that per-literal, partitioned and byte-table shapes all exhaust the published bound, so TC-050 records a proof-bound disposition with Contract Runtime as next owner.
- examined: Kani matrix TC-050 row (spec/kani/matrix/tests.md): one original NFC `Text[1,1]` real-Kani run is inconclusive after `MemoryExhausted`, and IR-796's measured disposition shows no criterion-preserving shape fits the published bound (next owner: Contract Runtime); twelve verified harnesses, falsifying control and production proof evidence remain open

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | TC-050 prose pins exact tool versions (Kani 0.68.0, CBMC 6.11.0) and a moving 'revision in this repository's lockfile' as part of the bound. The Hash/Digest/Pin checklist rule makes this a high finding; the receipt belongs in the IR-796 ticket. Failure scenario: after a lock bump (runtime main has already deleted the measured text/Meter code), the sentence silently points at a different revision and the disposition reads as covering code it never measured. | spec/oracle/matrix/TC-050-caller-text-admission.md:104-107 | wrong-requirement |
| FND-002 | low | The FR-035 Dependencies bullet records a ticket-scoped measurement ('IR-796 measured that it does not, for any harness shape') without the 'with this runtime and toolchain' scope that TC-050 states. Requirement prose should state the dependency only. Failure scenario: once the kernel cost is fixed upstream, the FR still asserts that no shape fits, and readers take that as a property of the requirement. | spec/oracle/functional/FR-035-caller-text-admission.md:121-127 | wrong-requirement |

## Verdict

Changes requested. Criteria are preserved and uncredited, and make spec passes, but FND-001 is a high checklist violation.

## Dispositions

Round 1, reviewed at b94bdd4e7f8d15e1d317391ac0fe07fd93bc9a02.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | - |
