---
id: FR-016
title: "Decode and natively replay complete-V1 proof witnesses"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-197
    type: references
  - target: ix://agent-ix/quire-specification/FR-333
    type: references
  - target: ix://agent-ix/quire-specification/TC-219
    type: references
---
# FR-016: Decode and natively replay complete-V1 proof witnesses

## Description

When a FR-015 harness yields a counterexample, the code generator shall decode
the witness into typed complete-V1 values, validate each value against its
declared domain, and replay it through native runtime execution before any
failure is reported. This is issue #50.

## Inputs

- A retained Kani counterexample and the harness identity and pins that
  produced it.
- The obligation's declared input domains.
- A decode size limit bounding the witness bytes and value count read.

## Outputs

- A typed replay result: reproduced failure, malformed witness, mismatch,
  out-of-domain witness, or replay unavailable, carried as a FR-333 method
  result.

## Behavior

- When a counterexample is retained, the generator shall check that it is
  bound to the identity and pins of the harness being replayed, and decode
  each witness value into its declared complete-V1 scalar type within the
  decode size limit.
- The generator shall decode a witness against the harness's own persisted
  obligation schema: the obligation's argument bindings, in the order the
  obligation persists them, typed position for position against the
  harness's symbolic arguments, and each decoded value shall be named by the
  binding at its position. The harness emits its symbolic arguments in that
  same persisted order (FR-015).
- If an obligation binding is not an argument, then the generator shall refuse
  the witness schema with a typed schema refusal that reports no failure. The
  refusal is a fault in the schema, not in the witness, so it is not one of
  the five replay results. A binding that is not an argument has no symbolic
  position, and typing it would mistype a position that does not exist in the
  witness bytes.
- If a witness is bound to a different harness identity or pins, fails to
  decode, or exceeds the decode size limit, then the generator shall report a
  malformed witness and shall not report a failure.
- If a decoded value is outside its declared domain, then the generator shall
  report an out-of-domain witness and shall not report a failure.
- When a witness is in domain, the generator shall replay it through native
  runtime execution and compare the typed outcome with the harness result.
  The same typed outcome means an equal value or typed refusal, equal admitted
  charges, equal consumed counters, and equal limits.
- If native replay disagrees or cannot run, then the generator shall report a
  typed mismatch or unavailable result instead of a failure.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-016-AC-1 | Every retained witness decodes into typed values or is refused as malformed. | Test (TC-026) |
| FR-016-AC-2 | An out-of-domain witness is reported as such and never as a contract failure. | Test (TC-026) |
| FR-016-AC-3 | An in-domain witness is reported as a failure only when native replay reproduces the same typed outcome. | Test (TC-026) |
| FR-016-AC-4 | A disagreeing or unavailable native replay yields a typed mismatch or unavailable result. | Test (TC-026) |
| FR-016-AC-5 | A witness bound to a different harness identity or pins is reported as malformed and never replayed. | Test (TC-026) |
| FR-016-AC-6 | A witness over the decode size limit is reported as malformed without decoding past the limit. | Test (TC-026) |
| FR-016-AC-7 | A native replay that matches the harness value but differs in admitted charges, consumed counters or limits yields a typed mismatch. | Test (TC-026) |
| FR-016-AC-8 | A witness schema binds the obligation's persisted argument bindings, in the order the obligation persists them, position for position to the harness's symbolic arguments, and each decoded value is named by the binding at its position; a binding that is not an argument is refused with a typed schema refusal that reports no failure and is none of the five replay results. | Test (TC-026) |

## Dependencies

- **Upstream**: [FR-015](./FR-015-bounded-kani-obligations.md).
- **Downstream**: [TC-026](../../test/complete-v1/TC-026-witness-native-replay.md).
