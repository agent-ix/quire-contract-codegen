---
id: TC-058
title: "Load CG generated-closure cases into QSL evaluator comparison"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-044
    type: verifies
---
# TC-058: Load CG generated-closure cases into QSL evaluator comparison

## Description

Verify that CG publishes QSL-admitted generated-closure test inputs and their
QSL std reference evaluations in a loadable generated bundle. The QSL-owned
differential harness is responsible for running both evaluators and deciding
agreement; this case verifies CG's export boundary.

## Test Procedure

1. Admit one scalar, one equality and one nested generated-to-generated-to-
   interpreted application through the QSL checked-closure interface. Export
   with a caller-selected `EVALUATOR_CORPUS_DIR` and load `cases.jsonl` using
   QSL's public corpus loader. Assert its format, case count, distinct stable
   ids, checked application and registered-body identities, limits, complete
   expected evaluations and ordered charges (FR-044-AC-1).
2. Export twice from fresh admitted instances and compare bytes. Independently
   perturb one identity, location, limit, result distinction, refusal, loss,
   consumed work or charge-order field and assert that the loader reports the
   changed case or refuses invalid checked input. Remove one case and assert
   the expected census fails (FR-044-AC-2).
3. Make QSL reference evaluation unavailable or make one checked input
   unencodable; assert the producer fails and leaves no partial published
   bundle. Inspect the producer for computation of expected outcomes from
   emitted source or Contract Runtime `exact` (FR-044-AC-2).

## Expected Results

The QSL loader admits all three intact records as the same checked
applications and expected evaluations; byte identity, adverse changes and
whole-bundle failure behave as specified. The QSL-owned IR-591 harness then
uses this bundle for std/no_std comparison in its own CI.
