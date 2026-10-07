---
id: "SR-2245"
title: "IR-635 composite claim-shape spec diff: scope-boundary analysis"
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

# SR-2245: IR-635 composite claim-shape spec diff: scope-boundary analysis

## Summary

Ticket: IR-635. PR: not yet opened. Diff reviewed: ca5d61adbb62959097bfdd06a3374f2f890f2e3b...cc3a78b0df85c6d3fde86af0b5a13a6eccc02d08.
Method: spec-review/scope-boundary. Reviewer model claude-opus-5-5, session cd5dfa92-5d1b-442d-b3fe-30c6c25b005e, run 09c0daf0-442e-4959-9cd9-2346807cdb0d.

## Method

Allocation of new obligations against the brief's lane: CG composite claim shape, E-1 composite, E-2, E-4; scalar/terminal text as context.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | E-2 restates FR-032 scalar consumer obligations as a second normative source | spec/assurance/AD-003-evidence-chain.md:305-318 |

### Finding detail

- FND-001 (severity low, confidence medium, check exceeds, unit AD-003 E-2): IR-635's allocation in AD-003 is the composite E-1 paragraph plus E-2. The new E-2 text nonetheless sets normative scalar-route converter and driver duties ('On every scalar or composite outcome ... shall compare'), which FR-032 (IR-631) already owns. This creates a second source for the scalar contract. If FR-032 later changes its context-retention or ReplayLimits wording, E-2 diverges silently.

## Verdict

Composite changes stay inside FR-033/TC-048. QSpec FR-322, QSL FR-070/FR-358 and QSL encoder ownership are referenced rather than copied. Nothing is vendored.
