---
id: TC-047
title: "Check public routed scalar lowering replay and generated-content binding"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-032
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/FR-029
    type: verifies
---
# TC-047: Check public routed scalar lowering replay and generated-content binding

## Description

Planned public-consumer coverage of
[FR-032](../functional/FR-032-routed-scalar-replay-binding.md), distinguishing lowering parity
failure from source-property refutation. No implementation or passing replay is claimed. The
completed QSL route is gated on QSL-641's typed scalar arm, native generated-oracle observation
contract and named harness-defect terminal cause.

## Test Procedure

1. Obtain a real QSL-emitted checked package for `x + 1` over `Int[0,9]`; call public
   `generate_routed` for its scalar node and retain the returned identity and generated harness.
   Compile a public scalar builder call using those types directly, without a function identity.
2. In the named real-Kani lane, run the generated harness and an arithmetic mutation control.
   Retain its actual selected assertion playback and proved generated content. An unmutated
   out-of-result-range value is an expected refusal; do not fabricate a source-domain falsifier.
3. Build the scalar plan from the actual identity, decoded playback, proving package and
   canonical identity of that same proved generated oracle. Until the upstream capability exists,
   require the unsupported-capability refusal. Try another harness name, missing/extra operands,
   wrong widths, malformed/over-limit playback, adjacent out-of-domain endpoints, reordered
   operands, another node/operation, changed result range and changed accounting limits.
4. For the admitted upstream route, record the exact QSL request's operator, operands, operand
   ranges, result range and limits. Ensure the literal operand stays in its singleton range.
   Execute the same proved oracle artifact natively over those operands and limits, following the
   upstream observation contract, and retain the typed outcome. Attempt
   replay without that observation, with another artifact's observation and with a regenerated
   correct oracle replacing the proved arithmetic mutation.
5. Call QSL's scalar arm in the public consumer, then pass its result to the CG converter.
   Compare generated/exact divergence, agreement despite retained Kani falsification, a QSL
   non-fault refusal and an executor fault. Check all four supported integer operators and
   refuse an undefined/incomplete generated outcome as agreement; those outcomes do not
   satisfy the current harness assertion. Change the actual generated arithmetic, not the
   claimed replay verdict. Error-path seam observations may use a recording/failing executor;
   the positive evaluation must use QSL and the actual generated artifact.
6. Bind another scalar result/run to the first binding and require no settlement; repeat by
   changing node, operator, operands, result range and limits individually. Pass valid converter
   settlements to `run_terminal_value`; repeat with no settlement to reuse
   [FR-029](../../kani/functional/FR-029-run-outcome-terminal-record.md) AC-15. Trace executable
   tests to each criterion they actually assert; this document alone provides no coverage.

## Expected Results

| Criterion | Planned observation | Mutation that must fail the check |
|---|---|---|
| FR-032-AC-1 | Public typed consumer uses scalar identity; unavailable scalar capability refuses | Require function identity or admit a synthetic predicate as the scalar route |
| FR-032-AC-2 | Ordered actual playback decodes or gives a distinct typed refusal | Decode in alphabetic order, ignore another harness name or discard extra bytes |
| FR-032-AC-3 | Operand endpoints and request operator/ranges/limits agree with the persisted harness/package | Skip a bound check, replace operator/node, use result bounds for a literal or swap operands |
| FR-032-AC-4 | Same-proved-artifact observation is required and tied to operands/limits/content | Treat playback as a generated result or accept fresh regeneration that removes the mutation |
| FR-032-AC-5 | Driver executes the proved artifact and calls QSL between builder and converter; QSL measures the current four-operator completed-value/refused-outside-range projection | Execute inside the CG builder, substitute a local evaluator, stub the positive verdict, accept `Undefined`/`Incomplete` as agreement or add unproved refusal-cause/accounting-counter equality |
| FR-032-AC-6 | Another scalar result/run yields no settlement for this binding | Remove the result/run-to-scalar binding check or omit result range/limits from it |
| FR-032-AC-7 | Lowering divergence is `Failed`, agreement is named harness-defect `Inconclusive`, QSL refusal retains its code and faults are `Failed`; never `Refuted`/`Verified` | Convert every Kani falsification to `Reproduced`, fabricate a predicate disagreement or turn a fault into a data refusal |
| FR-032-AC-8 | Real routed QSL emission, Kani playback and proved-oracle observation exercise the seam; absent settlement refuses; expected result-range refusal is not a violation | Use a hand-built function harness, relabel expected refusal as falsification or synthesize absent settlement |

All checks are planned; completed-route observations have the same explicit upstream gates as
[FR-032](../functional/FR-032-routed-scalar-replay-binding.md). Scalar lowering replay never
produces a source-property `Refuted` or `Verified` terminal. No settlement yields
`TerminalPairError::MissingSettlement` and no terminal value. Missing upstream types/causes
must be reported as a gate, not implemented as local QSL lookalikes.
