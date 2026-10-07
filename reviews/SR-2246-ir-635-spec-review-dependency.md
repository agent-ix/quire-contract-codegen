---
id: "SR-2246"
title: "IR-635 composite claim-shape spec diff: dependency analysis"
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

# SR-2246: IR-635 composite claim-shape spec diff: dependency analysis

## Summary

Ticket: IR-635. PR: not yet opened. Diff reviewed: ca5d61adbb62959097bfdd06a3374f2f890f2e3b...cc3a78b0df85c6d3fde86af0b5a13a6eccc02d08.
Method: spec-review/dependency. Reviewer model claude-opus-5-5, session cd5dfa92-5d1b-442d-b3fe-30c6c25b005e, run 09c0daf0-442e-4959-9cd9-2346807cdb0d.

## Method

Relationship edges and enablement versus feature separation for the changed units.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-033 now requires QSL's FR-357-owned parity mint but has no FR-357 edge | spec/replay/functional/FR-033-composite-parity-replay-binding.md:19-28 |

### Finding detail

- FND-001 (severity low, confidence medium, check trace, unit FR-033 relationships): FR-033 now normatively requires QSL's shared `parity_obligation`, `ParityPreimage`, `ParityArgument`, `Domain`, `BoundEntries` and `IdentityEncodeError` (qsl-replay parity_identity.rs, which is headed FR-357, FR-358, with FR-357-AC-19 tracing the encoding). Its relationships reference QSL FR-358 and ADR-013 but not FR-357. A dependency query therefore does not surface FR-033 as a consumer of FR-357.

## Verdict

Enablement is correctly separated: QSL delivery is a source fact, and CG retention, driver authority and family proofs remain CODE gates. Open QSL text-profile and nested set/bag performance work is kept explicit as open upstream work.
