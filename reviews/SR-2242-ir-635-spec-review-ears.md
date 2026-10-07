---
id: "SR-2242"
title: "IR-635 composite claim-shape spec diff: EARS analysis"
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

# SR-2242: IR-635 composite claim-shape spec diff: EARS analysis

## Summary

Ticket: IR-635. PR: not yet opened. Diff reviewed: ca5d61adbb62959097bfdd06a3374f2f890f2e3b...cc3a78b0df85c6d3fde86af0b5a13a6eccc02d08.
Method: spec-review/ears. Reviewer model claude-opus-5-5, session cd5dfa92-5d1b-442d-b3fe-30c6c25b005e, run 09c0daf0-442e-4959-9cd9-2346807cdb0d.

## Method

EARS conformance of every requirement statement added or changed by the diff.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | One shall-bullet bundles five O-09 obligations plus a non-normative ordering example | spec/replay/functional/FR-033-composite-parity-replay-binding.md:115-122 |

### Finding detail

- FND-001 (severity low, confidence high, check compound, unit FR-033 Behavior O-09 construction bullet): This single Behavior bullet requires five separate things: using the typed `parity_obligation`, typed encoder refusal, no new top-level preimage member, keeping drawn bounds under declared-domain substitution, and encoded-key ordering. It then ends with an illustrative example sentence that has no 'shall'. It is not atomic, so a trace cannot mark one obligation met and another unmet.

## Verdict

Other new statements are acceptable ubiquitous or event-driven 'shall' forms; status sentences are descriptive context, not requirements.

## Dispositions

Round 1, reviewed at e18689d13cff2f4eb85e618e4b55f263d6734de9 (original review at cc3a78b0df85c6d3fde86af0b5a13a6eccc02d08; custody commit 8a0c77e1077e9a5d2068b48fbd774215adaa3a01). Reviewer model claude-opus-5-5, session cd5dfa92-5d1b-442d-b3fe-30c6c25b005e, run 1d5892fd-67f7-47dc-ab34-806d87e1ee79.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | e18689d13cff2f4eb85e618e4b55f263d6734de9: Split into atomic shall-bullets; the ordering example is now a separately labelled non-normative line. |
