---
id: SR-8560
title: "IR-768 FR-021 and TC-031 base spec review"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-contract-codegen@6bd393f2092ba50dcdab88f918f706580c6684e; spec/oracle/functional/FR-021-function-application-oracles.md, spec/oracle/matrix/TC-031-function-application-oracles.md"
---

# SR-8560: IR-768 FR-021 and TC-031 base spec review

## Summary

Ticket: IR-768. Reviewed the changed FR-021 Behavior, AC-12, AC-23, AC-25, and TC-031 steps 11 and 13 against the existing function generator and its test matrix. The checked-source Call-root contract states the source binding and typed refusal cases, retains Scalar/CompositeEquality lowering and call-item lowering, and keeps actual Failed-record mapping and call-node-first precedence. One traceability defect remains.

## Examined units

- `FR-021` Behavior, examined: distinct checked Call roots are classified in node-id order with exact operation, callee, ordered operands, arity and types; Scalar and CompositeEquality retain Contract IR lowering; Call roots have no synthetic lowering record.
- `FR-021-AC-12`, examined: wrong source binding or shape and absent/refused callee produce typed, isolated refusals; duplicate-declaration and capability precedence remains.
- `FR-021-AC-23`, examined: actual Failed records retain per-kind mapping and byte `limit`/`consumed`; a failed call-item lowering precedes a failed body lowering.
- `FR-021-AC-25`, examined: reader-admitted 129-function/128-link chain under unchanged 1 MiB caps, root checks, generation and depth refusal.
- `TC-031` steps 11 and 13, examined: matching byte-failure and chain cases, controls and expected refusals.
- `spec/oracle/matrix/tests.md`, context only: existing TC-031 coverage registry.

## Verdict

The amended FR and TC agree on the core behavior. The new AC-25 is absent from the matrix index, so that index incorrectly presents the old acceptance-criterion set and cannot track its coverage state. This is a medium finding for the author to fix before handoff.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Register new FR-021-AC-25 in the TC-031 matrix index; the current index ends at AC-24 and gives no coverage status for the new chain criterion | spec/oracle/functional/FR-021-function-application-oracles.md:329, spec/oracle/matrix/TC-031-function-application-oracles.md:185, spec/oracle/matrix/tests.md:86 |

## Dispositions

Round 1 reviewed at 88345096fd5f244a2cfea9b7d00853abb800d808. The diff adds a planned AC-25 row to the FR-021 coverage table and AC-25 to the TC-031 criterion list in `spec/oracle/matrix/tests.md`.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 88345096fd5f244a2cfea9b7d00853abb800d808 |
