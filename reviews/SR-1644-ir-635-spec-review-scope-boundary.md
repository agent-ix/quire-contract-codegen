---
id: SR-1644
title: "IR-635 spec review (scope boundary): CG and QSL ownership in the parity route"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/quire-contract-codegen#298; spec/kani/functional/FR-025-generated-subject-abi.md, spec/kani/functional/FR-028-bounded-proof-ceilings.md, spec/kani/functional/FR-029-run-outcome-terminal-record.md, spec/replay/functional/FR-033-composite-parity-replay-binding.md, spec/replay/matrix/TC-048-composite-parity-replay-binding.md, spec/spec.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-033
    type: reviews
  - target: ix://agent-ix/quire-contract-codegen/FR-029
    type: reviews
---

# SR-1644: IR-635 spec review, scope boundary

## Summary

Ticket: IR-635. PR: quire-contract-codegen#298 (spec only; the reviewed head is recorded in the private ticket marker). Reviewer: claude-opus-5-5, session 2f7447c0-1934-4031-8988-fab17e2f41b7. I checked that CG obligations stay inside CG and that QSL-owned behaviour is consumed, not specified. One low finding.

## Method

I read the FR-033 Dependencies ownership paragraph and compared it with every Behavior bullet and criterion. I checked AD-002 R-6/R-7 (CG computes the `ObligationIdentity`, and the request names QSL's `package_id`). I also checked that the following are QSL's:
- canonical value types and admission;
- node selection;
- exact evaluation;
- declared-bound derivation;
- the cause vocabulary.

FR-025 keeps the emitted binding and FR-028 keeps strength and ceiling ownership.

## Verdict

**PASS with one low finding.** Clean: the ownership split is stated once in FR-033's Dependencies section and is respected in substance. CG builds the request and converts the result, and QSL evaluates, derives bound keys and owns causes. CG supplies no local evaluator, synthetic predicate or canned verdict. The owning facade, not CG, must establish the parity-agreement cause and the `CgDefect` attachment.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Some FR-033 statements put an obligation on QSL inside a CG requirement. Line 124 says "QSL shall compare its exact equality result/count ...". FR-033-AC-1 asserts "QSL recompiles ... and validates FR-322 node membership", and AC-8 asserts "QSL derives bound keys". A CG test cannot hold QSL to a `shall`. Restate these as CG obligations on what CG submits and consumes, for example "the driver shall submit ... to QSL's parity arm and shall settle only from its result". Keep QSL's own behaviour as the observed oracle in TC-048. | spec/replay/functional/FR-033-composite-parity-replay-binding.md:124, spec/replay/functional/FR-033-composite-parity-replay-binding.md:164, spec/replay/functional/FR-033-composite-parity-replay-binding.md:171 |
