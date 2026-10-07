---
id: "SR-2244"
title: "IR-635 composite claim-shape spec diff: failure-domain analysis"
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

# SR-2244: IR-635 composite claim-shape spec diff: failure-domain analysis

## Summary

Ticket: IR-635. PR: not yet opened. Diff reviewed: ca5d61adbb62959097bfdd06a3374f2f890f2e3b...cc3a78b0df85c6d3fde86af0b5a13a6eccc02d08.
Method: spec-review/failure-domain. Reviewer model claude-opus-5-5, session cd5dfa92-5d1b-442d-b3fe-30c6c25b005e, run 09c0daf0-442e-4959-9cd9-2346807cdb0d.

## Method

Unstated failure modes and identity confusion in the new positional O-09, sent-claim and artifact-authentication text, checked against QSL source.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Population-keyed harness bounds partition out of O-09 arguments is unstated | spec/replay/functional/FR-033-composite-parity-replay-binding.md:76-85 |

### Finding detail

- FND-001 (severity low, confidence medium, check other, unit FR-033 Inputs O-09 bullet): When QSL recomputes the preimage (composite_parity::parity_preimage), each parameter's Bounds uses only the `harness_bounds` entries keyed `DomainKey::Node` with that parameter's node. `DomainKey::Population` entries stay in `CompositeIdentity.harness_bounds` but never enter O-09. FR-033 says only 'its drawn harness bounds', and AC-11/TC-048 step 9 never exercise a population key. A CG builder that puts a Population-keyed bound, or the whole bound list, into an argument mints an O-09 that QSL's identity tie refuses as an obligation mismatch. Every claim whose harness carries a population bound would then refuse.

## Verdict

Self-comparison, literal and duplicate-key identity rules match QSL. Echo equality is correctly kept separate from artifact authentication. Refusals stay typed, with no sentinel identity.

## New findings (disposition pass 1)

Reviewed at e18689d13cff2f4eb85e618e4b55f263d6734de9. Reviewer model claude-opus-5-5, session cd5dfa92-5d1b-442d-b3fe-30c6c25b005e, run 1d5892fd-67f7-47dc-ab34-806d87e1ee79.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | low | Fix presents Population-keyed bounds as retained composite claim members, but QSL refuses any such key in prepare | spec/replay/functional/FR-033-composite-parity-replay-binding.md:81-84 |

### New finding detail

- FND-002 (severity low, confidence high, check soundness, unit FR-033 Inputs O-09 bullet): In QSL 30d7beb7, the composite position derivation (execute/composite_domain.rs) emits only `DomainKey::Node` positions; a population leaf is a Node-keyed `Whole` position. `Positions::harness` therefore refuses any `DomainKey::Population` harness bound with `ParityBoundRefusal::HarnessUnknownKey` (InvalidRuntimeInput) inside `prepare`, before `identity_tie`. The fixed text in Inputs, AC-11, AD-003 E-1 and TC-048 step 9 says Population-keyed bounds 'remain in the full claim and CG record' and are only excluded from O-09. It never says that a composite parity request carrying one is a typed QSL refusal: the bound Refused report maps to ReplayRefused. So TC-048 step 9's Population case reads as a valid claim when it is not, and an implementer could draw population bounds expecting settlement. This also corrects FND-001's original scenario: such a claim refuses at the harness-key check, not at an O-09 mismatch.

## Dispositions

Round 1, reviewed at e18689d13cff2f4eb85e618e4b55f263d6734de9 (original review at cc3a78b0df85c6d3fde86af0b5a13a6eccc02d08; custody commit 8a0c77e1077e9a5d2068b48fbd774215adaa3a01). Reviewer model claude-opus-5-5, session cd5dfa92-5d1b-442d-b3fe-30c6c25b005e, run 1d5892fd-67f7-47dc-ab34-806d87e1ee79.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | e18689d13cff2f4eb85e618e4b55f263d6734de9: The Node-keyed/Population partition is now stated in Inputs, Behavior, AC-11, AD-003 E-1 and TC-048 step 9, and it matches QSL parity_preimage. See new FND-002: on re-measurement, this finding's original failure scenario overstated reachability. |

### Round 2 dispositions

Round 2, reviewed at 99d202c845ff821824d25806de5dbdbd3a34f306 (custody commit c34efd866f366c33ab2dbe7df2b5d2e3fdb6548d). Reviewer model claude-opus-5-5, session cd5dfa92-5d1b-442d-b3fe-30c6c25b005e, run 945a6196-e006-4287-b605-b96fcbf1f8ee.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-002 | fixed | 99d202c845ff821824d25806de5dbdbd3a34f306: Fixed text now states that Population-keyed metadata stays only in the full CG proving record, outside the admitted QSL composite claim and O-09. A request carrying a Population key gets a typed HarnessUnknownKey refusal in prepare, before identity_tie; the binding-checked Refused report maps to Inconclusive(ReplayRefused), and a CG precheck refusal stays distinct with no report. The same rule appears in Behavior, AC-11, AD-003 E-1 and TC-048 step 9, where the case is an adverse request, not an admitted claim. Re-measured against QSL 30d7beb7: composite_domain derive emits only DomainKey::Node positions; Positions::harness returns ParityBoundRefusal::HarnessUnknownKey (Code::InvalidRuntimeInput) for an unpositioned key; prepare calls harness before identity_tie. Fix diff c34efd8..99d202c touches only these Population hunks; AC-1/7/9/12/13, F rows, Setup Refusal, TC steps 8a/10, scalar E-1 and E-2 are byte-unchanged. No new findings. |
