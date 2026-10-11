---
id: SR-8563
title: "IR-768 FR-021 failure-domain review"
type: SpecReview
analysis: failure-domain
review_set: subset
scope: "agent-ix/quire-contract-codegen@6bd393f2092ba50dcdab88f918f706580c6684e6; spec/oracle/functional/FR-021-function-application-oracles.md, spec/oracle/matrix/TC-031-function-application-oracles.md"
---

# SR-8563: IR-768 FR-021 failure-domain review

## Summary

Ticket: IR-768. Examined overlapping source-binding and callee failures, duplicate and capability precedence, cycles, deep chains, and independent package outcomes. One real combined-defect input has two specified typed refusals without a priority rule.

## Examined units

- `FR-021` Behavior, examined: exact Call-root identity, callee binding, ordered operands, typed refusals, duplicate/capability precedence, recursive callee propagation, lowering failures, call-node-first item outcome, and linked admission.
- `FR-021-AC-12`, examined: root and callee failures are typed and isolated from unrelated items.
- `FR-021-AC-23`, examined: actual Failed records map without panic; call-node byte refusal precedes body byte refusal.
- `FR-021-AC-25`, examined: deep acyclic chain admits under both unchanged byte caps and refuses when runtime depth is exceeded.
- `TC-031` steps 11 and 13, examined: byte-failure and Call-root negative controls, including refused/missing callee.

## Verdict

Specify and test which refusal wins when the descriptor's callee is absent or refused and the checked source binds a different declared callee. This preserves deterministic typed dispositions for a reader-admitted request with both defects.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | A missing/refused descriptor callee plus a different source callee qualifies for both `UnknownCallee` and `BodyMismatch`; define their precedence and test the combined case | spec/oracle/functional/FR-021-function-application-oracles.md:166, spec/oracle/matrix/TC-031-function-application-oracles.md:198 |

## Dispositions

Round 1 reviewed at 88345096fd5f244a2cfea9b7d00853abb800d808. That commit edits only the matrix index; the overlapping-refusal wording and separate negative controls remain unchanged.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | still-open | FR-021 and TC-031 do not define or test which typed refusal wins when the descriptor callee is absent/refused and the source binds another declared callee |
| FND-001 | fixed | bb6ef66fd5e59330d74fef71e7277f04686fb362 |

Round 2 reviewed at bb6ef66fd5e59330d74fef71e7277f04686fb362. FR-021 now resolves a classifiable Call root's descriptor callee before comparing the checked source's reference: absent/refused descriptor gives `UnknownCallee`; a resolvable descriptor with different source binding gives `BodyMismatch`. AC-12 and TC-031 step 13 state and test both branches. The prior round-1 row remains as the historical outcome at 8834509.
