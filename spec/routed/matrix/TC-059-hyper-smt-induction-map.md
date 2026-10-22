---
id: TC-059
title: "Bind QSL-encoded HP-2 induction queries to solver-run evidence"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-045
    type: verifies
---
# TC-059: Bind QSL-encoded HP-2 induction queries to solver-run evidence

## Description

Verify CG's solver invocation, exact query association, bounded capture and
typed non-success over QSL-owned canonical HP-2 queries. QSL owns the query
encoder, method outcome and terminal settlement; this test does not build a
second encoder or claim CG execution alone proves a hyperproperty.

## Test Procedure

1. Supply a QSL-admitted HP-2 safety clause, ordered subjects, one bound
   instance and a Meter. At `k = 1`, obtain base and step bytes using
   FR-315's `encode_hyper_query` call shape. Capture the bytes at CG's
   selected solver stdin boundary and assert byte equality. Check the two
   run records retain the product, item, instance, depth, query kind, exact
   bytes, solver selection and distinct attempt identities (FR-045-AC-1,
   AC-3). This step waits for QSL-1010's production encoder.
2. Return one `Unsat` response from each of two bounded solver invocations.
   Assert CG retains two matching completed query records and offers only
   this pair for QSL-owned `Inductive { depth: 1 }` settlement; QSL's
   FR-182-AC-7 test owns the `proved`, `decisive-witness`, `Uncertified`
   assertion. Swap one record's product, item, instance, depth, kind or query
   bytes and assert that the pair is ineligible (FR-045-AC-1, AC-2).
3. Drive base `Sat` with raw emitted model bytes, base `Unsat` with step
   `Sat`, solver `Unknown`, an original QSL encoding refusal, missing and
   malformed responses, contradictory statuses, nonzero process exit and
   stale response association. Assert distinct typed run evidence and no
   CG terminal proof or refutation. QSL reconstruction/replay alone can
   turn a base candidate into a decisive product path (FR-045-AC-2).
4. Use a selected nonexistent executable and finite wall-clock, memory and
   capture limits. Assert `MissingTool` before any answer and `Stopped`
   with the reached limit for each limit case. Check that one process gets
   only its own exact query on stdin, captures only its own stdout/stderr,
   and retains the selected executable, arguments, limits, completion and
   any emitted model/proof bytes. Neither refusal is `Unsat`; the reached
   resource limit is handed to QSL FR-182 V-7 as incomplete
   (FR-045-AC-2, AC-3).

## Expected Results

Only two completed, correctly associated `Unsat` records make QSL induction
eligible. Every other response, encoding refusal, missing tool or reached
limit remains typed non-proof evidence with the available identities and
depth. CG does not mint QSL outcome types, certificates or terminal records.
