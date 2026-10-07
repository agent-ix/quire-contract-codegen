---
id: "SR-2241"
title: "IR-635 composite claim-shape spec diff: spec review (base)"
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

# SR-2241: IR-635 composite claim-shape spec diff: spec review (base)

## Summary

Ticket: IR-635. PR: not yet opened. Diff reviewed: ca5d61adbb62959097bfdd06a3374f2f890f2e3b...cc3a78b0df85c6d3fde86af0b5a13a6eccc02d08.
Method: spec-review/base. Reviewer model claude-opus-5-5, session cd5dfa92-5d1b-442d-b3fe-30c6c25b005e, run 09c0daf0-442e-4959-9cd9-2346807cdb0d.

## Method

Base spec-review of the changed FR-033/TC-048/AD-003 statements for consistency, completeness and agreement with owning upstream source (public quire-spec-language@30d7beb7483b721bcb1ec98926e8b717e9a76fb7 (QSL #645; 6f8518414a88b48a380c54e4c3171bbf9968d0a1 (#650) verified ancestor)). Sub-analyses are recorded in their own artifacts (SR-2242..SR-2246).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | 'preserve its legitimate changed echo' states no observable converter response | spec/replay/functional/FR-033-composite-parity-replay-binding.md:175-176 |

### Finding detail

- FND-001 (severity low, confidence medium, check ambiguous, unit FR-033 Behavior retained sent claim bullet): Retention is already pinned to the moment before the owning entry call (lines 157-158). So a request changed before send simply binds by its sent identity. 'The converter shall preserve its legitimate changed echo without claiming consumer corruption' names no observable output. 'Changed' relative to what?, and 'preserve' meaning what? Two implementers could read it as a no-op, or as a duty to compare against a pre-change build-time identity. The second reading would reintroduce the false refusal that the sentence is trying to rule out.

## Verdict

Approve after the code-review blockers (SR-2240) are fixed. Status language keeps the line between delivered QSL source facts and CG PLANNED/UNRUN code gates, and does not close proof or generated-family evidence because an API exists. Open Text-profile and nested set/bag performance limitations stay explicit, with no grammar fallback.

## Dispositions

Round 1, reviewed at e18689d13cff2f4eb85e618e4b55f263d6734de9 (original review at cc3a78b0df85c6d3fde86af0b5a13a6eccc02d08; custody commit 8a0c77e1077e9a5d2068b48fbd774215adaa3a01). Reviewer model claude-opus-5-5, session cd5dfa92-5d1b-442d-b3fe-30c6c25b005e, run 1d5892fd-67f7-47dc-ab34-806d87e1ee79.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | e18689d13cff2f4eb85e618e4b55f263d6734de9: Observable response now stated: retain the identity actually sent, and bind when the genuine report equals it, regardless of an earlier unsent request. |
