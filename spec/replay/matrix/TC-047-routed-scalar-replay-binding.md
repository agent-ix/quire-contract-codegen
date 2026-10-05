---
id: TC-047
title: "Check public routed scalar replay binding and proposition preservation"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-032
    type: verifies
  - target: ix://agent-ix/quire-contract-codegen/FR-029
    type: verifies
---
# TC-047: Check public routed scalar replay binding and proposition preservation

## Description

Planned public-consumer coverage of
[FR-032](../functional/FR-032-routed-scalar-replay-binding.md), including the distinction between
an arithmetic-parity falsifier and a source violation. No test implementation or passing
positive replay is claimed. Positive scalar execution/evidence remains gated on that
requirement's unresolved QSL capability and proposition decision.

## Test Procedure

1. Obtain a real QSL-emitted checked package for `x + 1` over `Int[0,9]` and route its scalar
   node through public `generate_routed`. Retain the actual returned identity and harness;
   compile a public scalar builder call using those types directly.
2. In the named real-Kani lane, run the generated harness and its arithmetic mutation control.
   Retain the selected assertion playback, including the qualified harness name and actual
   symbolic operand bytes. An unmutated out-of-result-range value is an expected refusal;
   do not fabricate a source-domain falsifier or a function-contract harness.
3. Build the scalar request from the actual identity, playback and proving package. In today's
   unsupported proposition, require the typed setup refusal and observe no QSL replay call.
   Independently try missing/extra operands, wrong widths, malformed/over-limit playback,
   another harness name, each out-of-domain endpoint, swapped operands, another node or
   operation, and a symbolic value that would replace the source's literal `1`.
4. Once the upstream proposition and typed capability exist, use their admitted request and
   evidence, call QSL in the public consumer, and pass its actual result to the CG converter.
   Observe the request selection, node/operation identity and operand correspondence against
   the original QSL emission. Compare an admitted boundary operand with the adjacent refused
   one; change the witness and verify that the evaluated evidence changes accordingly.
5. For admitted scalar evidence, compare genuine same-proposition reproduction, disagreement,
   QSL non-fault refusal and executor fault. Bind another run/result to the first scalar binding;
   separately change its node and operation and require a typed binding refusal with no
   settlement. Error-path seam observations may use a recording/failing executor, but the
   positive integration must use QSL's actual evaluation.
6. Pass each converter settlement to public `run_terminal_value` with the corresponding
   falsified run. Repeat with no settlement to reuse
   [FR-029](../../kani/functional/FR-029-run-outcome-terminal-record.md) AC-15. Trace the actual
   executable tests to each criterion they assert; this document alone supplies no coverage.

## Expected Results

| Criterion | Planned observation | Mutation that must fail the check |
|---|---|---|
| FR-032-AC-1 | The public typed consumer uses only the scalar identity; an unavailable scalar selection refuses | Make the builder require a function identity or accept an unrelated function as the scalar selection |
| FR-032-AC-2 | Real ordered playback decodes or gives the distinct typed refusal before replay | Decode with alphabetic operand order, ignore a mismatched harness name, or silently discard extra bytes |
| FR-032-AC-3 | Bound endpoints and authoritative source operands stay tied to this node/operation; each invalid variant refuses | Skip one bound check, accept a changed operation/node, or substitute a drawn operand for literal `1` |
| FR-032-AC-4 | Arithmetic parity failure and expected result-range refusal do not become a source violation | Treat every Kani falsification or every runtime refusal as `Reproduced` |
| FR-032-AC-5 | Public consumer executes QSL between builder and converter (admitted branch gated) | Execute replay inside the builder or return a locally computed replay verdict |
| FR-032-AC-6 | Another scalar result/run cannot yield a settlement for this binding (positive evidence gated) | Remove the result/run-to-scalar binding check |
| FR-032-AC-7 | Existing terminal readings and QSL codes survive conversion (QSL-admitted branch gated); CG-only setup refusal is `Failed` | Convert disagreement to `Refuted`, fault to `ReplayRefused`, or invent a QSL code for an unsupported CG selection |
| FR-032-AC-8 | Real routed emission and playback exercise the seam; absent settlement refuses; positive `Refuted` remains gated | Replace routed generation with a hand-built function harness, invent a source-domain falsifier, or synthesize a settlement when it is absent |

For admitted same-proposition evidence, `Refuted` requires QSL's genuine typed reproduction of
that violation and this scalar binding. Disagreement is `Inconclusive(ReplayParity)`, non-fault
QSL refusal is `Inconclusive(ReplayRefused)` with its original code, and faults/CG defects are
`Failed`. Another obligation's evidence yields no settlement. No settlement yields
`TerminalPairError::MissingSettlement` and no terminal value. The positive reproduction
observation is not accepted until the missing upstream semantics are resolved.
