---
id: SR-1568
title: "ears-conformance review of IR-629 process-provider BackendKind spec (quire-contract-codegen#287)"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-contract-codegen@43fbf2f6083663e680e1ebeeeeefe11b288fdef5; spec/routed/functional/FR-019-capability-settlement.md lines 100-115"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-019
    type: reviews
---
# EARS review: IR-629 process-provider BackendKind (PR #287)

## Summary

Ticket: IR-629. The seven new FR-019 statements are all ubiquitous-form with a named subject
("The generator" or "The process-provider arm"); `quire validate` raised no `ears:*` diagnostic on
the file. Four are singular. Three carry two `shall` clauses each, which the EARS lens reads as
non-singular.

## Verdict

Minor (three low findings); no grammar defect blocks the spec.

## Examined

- Bullet at line 104 (settle in the arm, nowhere else): examined, clean
- Bullet at line 106 (descriptor and extent alone): examined, clean
- Bullet at line 108 (advertised-mode rules): examined, clean
- Bullet at line 110 (never reach the plugin): examined, clean
- Bullets at lines 102, 112, 114: examined, findings below

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Non-singular: "shall give the closed backend kind one variant ... and shall list it in `BackendKind::ALL`" is two requirements in one statement | spec/routed/functional/FR-019-capability-settlement.md:102-103 |
| FND-002 | low | Non-singular: "shall return a negotiation disposition only, and shall never return a terminal value, a verification result or an artifact" is two requirements in one statement | spec/routed/functional/FR-019-capability-settlement.md:112-113 |
| FND-003 | low | Non-singular: "shall leave the mapping ... to the driver's pre-negotiation conversion, and shall do no plugin discovery of its own" is two requirements in one statement | spec/routed/functional/FR-019-capability-settlement.md:114-115 |

## Dispositions

Round 1, reviewed at 48a7a96eca9d7dbc66a3c7a808f042151da2607c. New defects found this round are recorded in SR-1570.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 48a7a96 |
| FND-002 | fixed | 48a7a96 |
| FND-003 | fixed | 48a7a96 |
