---
id: TC-026
title: "Verify witness decoding and native replay"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-016
    type: verifies
  - target: ix://agent-ix/quire-specification/TC-219
    type: references
---
# TC-026: Verify witness decoding and native replay

## Description

Verify that retained counterexamples are decoded, domain-checked and replayed
natively before any failure is reported.

## Test Procedure

Build a harness's witness schema from its persisted obligation arguments,
including one non-argument binding, and compare the schema's order with the
harness's emitted symbolic arguments. Decode a real falsifying transcript
against the persisted schema, then against that schema with a binding dropped
and with a binding retyped. Then replay a reproducing witness, a malformed
witness, a witness bound to another harness identity, a witness over the decode
size limit, an out-of-domain witness, a witness whose native value or charges
disagree with the harness, and a witness whose native replay is unavailable.

## Expected Results

The schema follows the persisted argument order, which is the emission
order, and refuses the non-argument binding. The real transcript decodes to
values named by their bindings, and the dropped and retyped schemas refuse by
arity and width. Only the reproducing witness is reported as a failure; the
others yield malformed (three cases), out-of-domain, mismatch and unavailable
results respectively.

## Status

Partial. The schema, decode and harness-identity cases are implemented in
`src/kani_witness_join.rs`'s unit tests, `tests/it/kani_argument_order.rs` and
the real-backend Kani lane in `tests/it/kani_witness_join.rs`. Pin binding, the
decode size limit, domain validation and native replay are planned.
