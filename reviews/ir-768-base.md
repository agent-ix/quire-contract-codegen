---
id: SR-8581
title: "IR-768 base review of FR-021 and TC-031"
type: SpecReview
analysis: base
scope: "agent-ix/quire-contract-codegen@bca51d464c265b8adaa7ccdc96e37a7dcda922fa; spec/oracle/functional/FR-021-function-application-oracles.md, spec/oracle/matrix/TC-031-function-application-oracles.md, spec/oracle/matrix/tests.md"
review_set: subset
---

## Summary

AC-34 has a measurable public-reader fixture, schedule and byte-limit oracle; TC-031 step 14 and the matrix index cover it. AC-23 still covers actual Failed records and refusal priority.

## Examined scope

- FR-021 Behavior: checked `Call` body-root classification; actual failed-record mapping and call-node-first priority.
- FR-021-AC-23: Scalar/Equality body and requested call-item `Failed` records.
- FR-021-AC-34: strict-reader-admitted 129-function chain with unchanged byte ceilings.
- TC-031 steps 10 and 14: negative lowering controls and public-chain positive case.
- TC-031 matrix index: planned AC-34 registration.
- Context: FR-021-AC-7, AC-12, AC-25, AC-27 and IR-511 flat SSA contract.

## Verdict

**PASS** — No actionable defect in this method's PR-diff scope. The AC-34 implementation and execution evidence remain planned.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
