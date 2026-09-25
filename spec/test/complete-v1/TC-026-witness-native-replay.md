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
order the harness uses, and refuses the non-argument binding with a typed
schema refusal (`InvalidInput`, code `cg_witness_schema_non_argument_binding`) that
reports no failure and is none of the five replay results.
The real transcript decodes to values named by their bindings, and the dropped
and retyped schemas refuse by arity and width. Only the reproducing witness is
reported as a failure; the others yield malformed (three cases), out-of-domain,
mismatch and unavailable results respectively.

## Status

Partial. FR-016-AC-8 is implemented and tested in the default suite: schema
order and naming by position (`src/kani_witness_join.rs` unit tests) and the
harness emission order (`tests/it/kani_argument_order.rs`); the real-backend
decode runs in the ignored Kani lane (`tests/it/kani_witness_join.rs`).

FR-016-AC-1 and FR-016-AC-5 are planned. The witness join reports every one of
its refusals as a `KaniOutcome` refusal code rather than as FR-016's
malformed-witness replay result, which does not exist yet. That covers the
harness-identity refusal (`cg_witness_harness_identity_mismatch`, kind
`Refused`), the arity and width refusals, the schema refusal, and the
Boolean-byte and comment refusals (kind `InvalidInput`); the arity and width
refusals are tested in the default suite and against a real falsification in
the ignored lane, and the others are tested only by quire-contract-ir's own
`Witness::decode` tests. Binding to the harness pins (AC-5), the decode size
limit (AC-6), domain validation (AC-2) and native replay (AC-3, AC-4, AC-7)
are also planned.
