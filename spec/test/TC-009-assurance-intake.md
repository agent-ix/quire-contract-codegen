---
id: TC-009
title: "Verify Quoin intake without Quoin or Quire executing a producer"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-006
    type: verifies
---
# TC-009: Verify Quoin intake without Quoin or Quire executing a producer

## Description

Verify that each proof obligation's attestation states the result read out of the producer's own
bytes, that Quoin retains exactly those bytes, that every declared command is the command the
producer target runs, and that neither Quoin nor Quire is asked to execute a producer.

## Test Procedure

Run the assurance chain over already-produced results and require every scenario, control and adapter
probe to match. Ask Make what `assurance-inputs` would run and require every declared argv to appear
in that plan. Then run the chain three more times: once with every producer replaced by a logging
stub, requiring the log to stay empty; once with `quoin` stubbed, requiring the chain to fail and the
log to be non-empty; and once with `quire` stubbed, requiring every request made of Quire to be a
static read.

## Expected Results

Every scenario, control and probe matches. Every proof is attested `passed` because its bytes say so.
The producer log is empty, the tool log is not, and the control run fails — an empty log and an
unconsulted `PATH` are otherwise the same observation.
