---
id: SR-8561
title: "IR-768 FR-021 requirement-language review"
type: SpecReview
analysis: ears-conformance
review_set: subset
scope: "agent-ix/quire-contract-codegen@6bd393f2092ba50dcdab88f918f706580c6684e6; spec/oracle/functional/FR-021-function-application-oracles.md"
---

# SR-8561: IR-768 FR-021 requirement-language review

## Summary

Ticket: IR-768. Examined the changed FR-021 Behavior statements and AC-12, AC-23, and AC-25 for named actor, concrete response, trigger, and verifiability. Targeted Quire grammar check found zero grammar findings, and the edited statements state observable validation and refusal outcomes.

## Examined units

- `FR-021` Behavior, examined: the generator inspects each distinct checked body root in declaring-node-id order, classifies checked Call roots with source binding, and lowers the other two forms through Contract IR.
- `FR-021-AC-12`, examined: invalid checked Call roots and callee dependencies receive typed, isolated refusals.
- `FR-021-AC-23`, examined: actual Contract IR Failed records retain their mapping and call-node-first precedence.
- `FR-021-AC-25`, examined: the authentic 129-function chain generates within unchanged caps and retains the runtime depth refusal.

## Verdict

No requirement-language defect found in the edited statements.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
