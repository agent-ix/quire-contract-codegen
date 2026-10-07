---
id: "SR-2243"
title: "IR-635 composite claim-shape spec diff: integrity analysis"
type: SpecReview
scope: "agent-ix/quire-contract-codegen@cc3a78b0df85c6d3fde86af0b5a13a6eccc02d08; spec/replay/functional/FR-033-composite-parity-replay-binding.md, spec/replay/matrix/TC-048-composite-parity-replay-binding.md, spec/assurance/AD-003-evidence-chain.md"
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-033
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/TC-048
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/AD-003
    type: reviews
---

# SR-2243: IR-635 composite claim-shape spec diff: integrity analysis

## Summary

Ticket: IR-635. PR: not yet opened. Diff reviewed: ca5d61adbb62959097bfdd06a3374f2f890f2e3b...cc3a78b0df85c6d3fde86af0b5a13a6eccc02d08.
Method: spec-review/integrity. Reviewer model claude-opus-5-5, session cd5dfa92-5d1b-442d-b3fe-30c6c25b005e, run 09c0daf0-442e-4959-9cd9-2346807cdb0d.

## Method

Completeness, consistency and atomicity of the changed units, plus cross-file agreement among FR-033, TC-048 and AD-003.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | E-2 determinism invariant now also carries several unrelated consumer-binding obligations | spec/assurance/AD-003-evidence-chain.md:295-318 |

### Finding detail

- FND-001 (severity low, confidence high, check compound, unit AD-003 E-2): E-2 used to be one checkable invariant: equal identity members give an equal ObligationIdentity, and regeneration is byte-identical. The diff appends preimage ownership, full-claim comparison on every outcome, wire/context/limit retention, driver authentication, ReplayLimits separation and gate status to that same item. E-2 can no longer be cited or verified as one invariant.

## Verdict

Cross-file claim-shape statements agree: AD-003 E-1, FR-033 Inputs, AC-11 and TC-048 step 9 and its matrix row all match. AC-12 is unchanged as allocated. The AC-1 contradiction is recorded once, in SR-2240 FND-001.

## Dispositions

Round 1, reviewed at e18689d13cff2f4eb85e618e4b55f263d6734de9 (original review at cc3a78b0df85c6d3fde86af0b5a13a6eccc02d08; custody commit 8a0c77e1077e9a5d2068b48fbd774215adaa3a01). Reviewer model claude-opus-5-5, session cd5dfa92-5d1b-442d-b3fe-30c6c25b005e, run 1d5892fd-67f7-47dc-ab34-806d87e1ee79.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | e18689d13cff2f4eb85e618e4b55f263d6734de9: E-2 is byte-identical to base ca5d61a's single determinism invariant. The binding text moved to the non-normative 'Claim binding allocation' section. |
