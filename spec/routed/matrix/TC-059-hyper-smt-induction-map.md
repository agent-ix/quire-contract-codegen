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
Also verify CG's typed refinement Base and Step run-record handoff to the
QSL FR-314 certificate adapter. The refinement procedures are planned and
do not claim coverage by the existing HP-2 pair tests.

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
4. Use a selected nonexistent executable and finite input-byte, wall-clock,
   memory and capture limits. Assert `MissingTool` before any answer, an
   input-byte refusal before spawn, and `Stopped`
   with the reached limit for each limit case. Check that one process gets
   only its own exact query on stdin, captures only its own stdout/stderr,
   and retains the selected executable, arguments, limits, completion and
   any emitted model/proof bytes. Neither refusal is `Unsat`; the reached
   resource limit is handed to QSL FR-182 V-7 as incomplete
   (FR-045-AC-2, AC-3).
5. Using a recompiled refinement with seven step rows, take row `r` from
   QSpec FR-375's canonically ordered `steps` array, not from source row
   position. Obtain `Initial` and each `Step { row: r }` query from FR-315.
   Observe the selected solver's stdin for every invocation.
   Assert one typed `Base` and seven typed `Step` records for rows 0 through
   6, with distinct attempts and matching refinement, subjects, role, row
   and exact query bytes. Permute the source step rows without changing their
   contents and recompile. Select a permutation for which at least one named
   row has a source position different from its canonical `steps` ordinal
   after recompilation; confirm those two positions before testing the record.
   Assert the canonical `steps` array, each row's FR-315 query bytes and CG's
   role/row bindings remain equal across the two compilations; each new solver
   invocation still has its own attempt identity. Assert that the correctly
   labelled record of the selected row passes association validation. In an
   otherwise valid collection, relabel only that selected record with its source
   position and assert typed refusal and no handoff. Repeat with a zero-step
   refinement and assert that only `Base` is required. Change one query byte
   and exchange a base and step role, then assert no handoff (FR-045-AC-4).
6. Return completed `Unsat` and distinct Alethe proof byte strings for the
   base and every step row from separate direct proof-producing solver
   invocations. In cvc5 proof-capture mode, use `--dump-proofs` and
   `--proof-format-mode=alethe` with each unchanged FR-315 query on stdin;
   capture real cvc5 output and assert that the proof field equals the
   exact bytes between the one outer proof list's parentheses. Check that
   removing only those delimiters leaves the original interior bytes and
   that QSL's FR-314 reader accepts the extracted Alethe commands. Supply
   records in two different orders. Decode
   the serialized handoff and assert both byte streams are identical and
   contain `Base` followed by rows 0 through 6, with the exact query and
   proof bytes in separate length-delimited fields. Repeat the serialization;
   make one proof present but empty, then change only one proof byte, and
   assert byte preservation and field isolation. For the empty case, capture
   `Unsat` followed by one empty outer list and assert a present zero-length
   proof field; distinguish status-only output as absent. Feed the handoff to the QSL
   adapter and assert it can form the FR-314 certificate in row order;
   QSL's TC-893 owns certificate checking and settlement (FR-045-AC-5).
7. From step 6's valid collection, remove `Base`, remove row 3, duplicate
   `Base`, duplicate row 3, and add row 7, one mutation at a time. Assert
   typed shape refusal and no serialized certificate handoff each time.
   Substitute a row-3 record from another item, subject or attempt; swap
   rows 3 and 4 while preserving their old labels. Assert typed association
   refusal and no handoff (FR-045-AC-6).
8. Remove the query field and then the proof field of row 3, one at a time;
   assert typed missing-field refusal and no handoff. Keep the proof field
   present but empty and assert handoff preserves it. Separately exercise an
   FR-315 query refusal and `Sat`, `Unknown`, absent response, malformed
   response, process failure and reached-limit solver results. For the
   proof-capture mode, exercise `Unsat` with no outer proof list, a second
   status, an error response, an incomplete outer list, a second outer list
   and a trailing non-whitespace token; none supplies proof bytes or a
   handoff. Assert typed
   non-proof evidence and no proof-bearing handoff (FR-045-AC-7).

## Expected Results

Only two completed, correctly associated `Unsat` records make QSL induction
eligible. Every other response, encoding refusal, missing tool or reached
limit remains typed non-proof evidence with the available identities and
depth. CG does not mint QSL outcome types, certificates or terminal records.
For refinement, only a complete set of correctly associated completed `Unsat`
records with present query and proof fields yields the deterministic QSL
adapter handoff. Refusal cases retain their distinct typed reasons. A present
empty proof reaches QSL unchanged for its checker to judge.
