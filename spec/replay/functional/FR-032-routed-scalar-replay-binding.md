---
id: FR-032
title: "Bind routed scalar lowering-parity witnesses to QSL replay"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-022
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-029
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-024
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-016
    type: references
  - target: ix://agent-ix/quire-spec-language/ADR-013
    type: references
---
# FR-032: Bind routed scalar lowering-parity witnesses to QSL replay

## Description

When the driver requests replay of a routed scalar counterexample, CG shall bind the request
directly to its `ScalarObligationIdentity`, preserving the arithmetic-parity proposition the
actual generated harness checked. This requirement owns a public request builder and a public
settlement converter; the driver owns the intervening QSL replay call. QSL owns the operator-level
exact evaluation, envelope arm, evidence and terminal causes.

This is planned IR-631 work. The scalar route is gated on the upstream QSL-641 capability and
observation contract below. No scalar API, QSL envelope variant or passing test is claimed to
exist. This requirement names semantic obligations, not speculative upstream Rust types.

## Scope

The slice is one routed exact-scalar Kani item, its retained assertion playback, its proved
generated oracle and its checked/emitted package. It includes typed setup refusals and conversion
to the settlement read by [FR-029](../../kani/functional/FR-029-run-outcome-terminal-record.md).
It excludes driver orchestration and FR-331 serialization, source-predicate violation, a
function-level declared-parameter parity route, new operation-contract clause-kind generation,
moving existing replay calls, a local exact evaluator or direct exact-kernel dependency in CG,
copied QSL types, and conversion through `KaniObligationIdentity` or V1 `ReplayPackage`.

The [AD-002](../../assurance/AD-002-cg-qsl-replay-seam.md) boundary remains: CG builds QSL-owned
inputs, the driver calls the public QSL facade, and QSL evaluates. Scalar lowering parity is
never a source-property refutation.

## Inputs

- The `ScalarObligationIdentity` returned by
  [FR-022](../../routed/functional/FR-022-routed-generation.md), including the claimed node,
  catalogued operation identity, qualified harness name and ordered operand schema.
- The falsified run's retained Kani assertion playback and a bounded decode limit.
- The checked/emitted package used to derive this node's operand and result ranges, the proving
  context, and the caller-supplied source bytes/proof-content identity needed by QSL's envelope.
- The canonical proof-content identity of the **same proved generated oracle**. After building
  the plan, the driver executes that artifact natively over the plan's decoded operator operands
  with the harness's recorded limits and supplies its actual typed completed value, refusal,
  undefined/incomplete outcome or execution fault to the QSL request. Kani playback alone
  supplies no generated outcome. Executing a newly regenerated unmutated oracle is not an
  observation of the proved content.
- At conversion, the same scalar binding and falsified run, plus QSL's own scalar result or
  refusal from the driver's call, or the builder's typed setup refusal.

Who executes the native generated artifact, how its typed observation crosses the upstream
boundary, and how QSL validates its proof-content tie are prerequisites below. This document
requires the evidence and does not invent its upstream wire representation or pretend CG
links the caller's generated artifact.

## Outputs

- A typed admitted scalar binding and QSL-owned replay plan for the operator-level
  lowering-parity arm, or a distinct typed setup refusal. The driver completes the plan with
  its actual native generated observation before calling QSL; it executes no regenerated oracle.
- A checked conversion to the existing settlement where representable, or a typed binding
  refusal with no settlement when a result or run belongs to another obligation. The new
  upstream harness-defect reading requires a settlement/terminal extension; it is gated and
  cannot be fabricated from existing predicate replay causes.

## Behavior

- The public scalar builder shall accept `ScalarObligationIdentity` directly.
- The builder shall select and decode playback through the Kani output adapter against the
  scalar identity's own qualified harness and persisted operand order, with one eight-byte
  `i64` value per symbolic operand and no function-identity coercion.
- When producing an admitted request, the builder shall validate the claimed node and
  catalogued operation against the proving package and every operand against its own declared
  bounds in persisted symbolic call order.
- The builder shall carry the operator, decoded operands, operand ranges, result range and
  harness accounting limits to QSL without replacing them with a selected function's arguments.
- The builder shall bind the replay plan to the operand vector, limits, node, operation and
  canonical proved generated-content identity required of the driver's native observation.
- If an identity, schema, expected proof-content tie or required upstream capability cannot be
  validated, then the builder shall return a distinct typed setup refusal before replay.
- CG's scalar builder and converter shall leave exact evaluation and replay execution to QSL
  through the driver's public-facade call.
- When returning a settlement, the converter shall validate that its scalar binding, retained
  run and QSL evidence belong to the same node, operation and lowering-parity proposition.
- The converter shall read QSL's typed result/refusal discriminants and catalog codes rather
  than infer a verdict or refusal code from message text.
- The converter shall preserve QSL's distinction between generated/native-exact divergence
  (CG lowering fault), agreement despite the Kani falsification (harness-defect inconclusive),
  out-of-operand-domain input (refusal), and an out-of-result-domain exact result (expected
  refusal that is not itself a falsification).
- The scalar converter shall never return `ReplaySettlement::Reproduced` for this
  lowering-parity proposition.

The parity projection is exactly the current harness assertion: a generated `Completed` value
equals the native mathematical result inside the result range, and a generated `Refused`
outcome occurs outside it; an undefined/incomplete outcome or execution fault does not satisfy
it. The assertion compares no refusal-cause equality, admitted-charge sequence or consumed
counters. Those are not new equality obligations of this replay slice. The recorded limits
remain inputs so the driver and QSL execute the same unlimited harness context.

The current profile comprises only `quire.op.integer.add`, `quire.op.integer.sub`,
`quire.op.integer.mul` and `quire.op.integer.negate` over the persisted `i64` operands.
These operations are mathematically defined for every admitted operand vector. An observed
`Undefined` therefore cannot agree with this profile; like `Incomplete` or an execution fault,
it does not conform to the current harness's `_ => false` assertion and cannot become
harness-defect agreement. A generic upstream undefined-agreement rule for other operators
does not expand this profile. Adding an operator or another parity projection requires its
own generation and replay acceptance criteria.

`run_terminal_value` remains the terminal map. Its refusal of a falsified run with no settlement
is already [FR-029](../../kani/functional/FR-029-run-outcome-terminal-record.md) AC-15. CG-only
setup defects carrying no QSL catalog code keep its `CgDefect`/`Failed` reading. QSL non-fault
refusals retain their own codes and faults remain `Failed`. No existing QSL predicate disagreement
is manufactured to stand in for the missing harness-defect cause.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-032-AC-1 | A public consumer passes the routed harness's `ScalarObligationIdentity` directly to the scalar builder; its typed call constructs neither `KaniObligationIdentity` nor V1 `ReplayPackage`. Until the upstream scalar arm exists, the builder returns a distinct unsupported-capability setup refusal rather than a synthetic function request. | Test |
| FR-032-AC-2 | The builder decodes actual routed-harness playback in persisted symbolic call order through the Kani output adapter; another harness name, missing/extra operand, wrong byte width, over-limit playback or malformed assertion block yields a distinct typed refusal before replay. | Test |
| FR-032-AC-3 | Each operand at its own lower or upper bound is admitted and the adjacent out-of-domain value is refused before replay. The admitted QSL inputs preserve the catalogued operator, operands, operand ranges, result range and harness limits. Another node/operation or altered operand schema refuses; a recorded literal remains a singleton-range operator operand. | Test |
| FR-032-AC-4 | The driver's actual native observation of the proved generated oracle is required and tied to this operand vector, limits and canonical proof-content identity; missing observation, another artifact's observation or fresh regeneration that removes the arithmetic mutation refuses before QSL evaluation. Playback bytes and Kani falsification alone cannot supply a generated outcome. | Test |
| FR-032-AC-5 | The public consumer executes the same proved generated artifact and invokes QSL between the builder and converter. QSL compares its actual native observation with authoritative exact operator evaluation using the current four-operator completed-value/refused-outside-range projection, without adding refusal-cause or accounting-counter equality; changing the generated arithmetic while keeping original node/operator identity changes the measured parity result. `Undefined`, `Incomplete` and execution faults cannot be agreeing outcomes of this profile. No CG-local evaluator or replay verdict replaces QSL. | Test |
| FR-032-AC-6 | The converter refuses another scalar binding's result/run with no settlement; changing only the claimed node, operation, operands, result range or limits cannot yield a settlement for the original binding. | Test |
| FR-032-AC-7 | QSL's divergence between generated and authoritative exact outcomes becomes a CG lowering fault and `Failed`; agreement despite Kani falsification becomes `Inconclusive` with the upstream named harness-defect cause; QSL non-fault refusal retains its catalog code and executor fault remains `Failed`. This scalar route never returns `Reproduced`, `Refuted` or `Verified`. | Test |
| FR-032-AC-8 | The real QSL-emitted `x + 1` over `Int[0,9]` routed case exercises `generate_routed`, its scalar identity, actual Kani playback and same-proved-artifact native observation. Expected exact result-range refusal is not reported as a source violation. Omitting settlement yields `TerminalPairError::MissingSettlement` and no terminal value. | Test |

## Intent and Existing Coverage

This table records the authoring gap analysis, not a computed test matrix.

| Intent | Existing authority and measured tests | Remaining gap |
|---|---|---|
| Real routed scalar generation | [FR-022](../../routed/functional/FR-022-routed-generation.md) AC-2/10/13/15; `tests/it/routed_generation.rs` carries those trace tags | The returned scalar identity has no public replay binding |
| Typed ordered playback and domain validation | [FR-016](./FR-016-witness-native-replay.md) AC-1/2/5/8; `tests/it/kani_witness_join.rs`, `kani_argument_order.rs` and `skeleton_spine.rs` exercise the function schema | Direct scalar schema and operator-level replay inputs |
| QSL refusal and terminal reading | [FR-016](./FR-016-witness-native-replay.md) AC-9/10/11/12 and [FR-029](../../kani/functional/FR-029-run-outcome-terminal-record.md) AC-1/10/13; `skeleton_spine.rs` and `terminal_map.rs` cover existing replay paths; AC-12 remains planned in the replay index | Scalar lowering fault and new harness-defect cause; predicate violation/parity readings do not establish scalar semantics |
| Missing settlement produces no terminal value | [FR-029](../../kani/functional/FR-029-run-outcome-terminal-record.md) AC-15; `tests/it/terminal_map.rs` | Reuse in the routed integration without duplicating the terminal requirement |
| Driver calls QSL between public typed builder and converter | No scalar requirement or test on the inspected source | This requirement and planned [TC-047](../matrix/TC-047-routed-scalar-replay-binding.md) |

## Prerequisites and Implementation Status

The inspected scalar renderer (`src/kani/generate/scalar.rs`) checks generated oracle/native
`i128` parity: `Completed` must equal the native result inside the result range; `Refused` must
occur outside it. An out-of-result-range computation therefore satisfies the harness when it
refuses correctly. Falsification indicates arithmetic parity failure, not a false QSL predicate.
The renderer draws operator operands symbolically; a recorded literal has singleton bounds,
while an operand lacking a more specific range uses the result range. Its meter uses
`u64::MAX` for every limit. `ScalarObligationArgument` persists generated identifiers and bounds.

The inspected QSL facade has no operator-parity envelope arm. `CallSiteSelection` admits
function, operation, clause, field and population selections, without a scalar expression, and
`replay` rejects non-Boolean functions as `NotAPredicate`. The existing terminal causes express
predicate replay parity/refusal, not harness-defect inconclusive. These source boundaries
remain until new upstream contracts land; no widened function selection is assumed.

The positive operator-level route requires the following QSL-641 upstream contracts:

1. A public typed operator-level arm in the QSL replay/envelope facade, with operator, recorded
   operands, operand ranges, result range and accounting limits, evaluated by QSL's authoritative
   exact kernel. It must distinguish lowering fault, harness-defect inconclusive and refusal.
2. A concrete native generated-oracle observation contract: the driver's typed observed outcome
   and validation tying it to the same canonical proved generated-content identity, operand
   vector and limits. The driver executes the actual artifact and QSL never regenerates it.
   Input operands alone are not the generated result. The parity projection compares no
   admitted-charge sequence or consumed counters beyond the current harness assertion.
3. A named upstream harness-defect inconclusive cause and a representable settlement/terminal
   reading. CG must not invent a QSL cause/code or retrofit a predicate disagreement.

All acceptance criteria are **Planned**. AC-1's admitted call, AC-3's admitted QSL inputs and
AC-4 through AC-8's completed scalar route are additionally **Gated** on the above contracts;
no test coverage is claimed. Unsupported-capability refusal, ordered decoding and operand
validation can be implemented independently. A future function-level declared-parameter parity
route is a distinct upstream capability and is outside this operator-level contract.

The builder's operand-domain check precedes every replay, as
[FR-024](./FR-024-counterexample-envelope-intake.md) already requires. A decoded value outside
the Kani assumed domain is a CG setup defect and `Failed`. QSL's own range validation is an
envelope backstop whose non-fault refusal keeps its QSL code; it does not authorize bypassing
the CG check. No amendment to the existing envelope or architecture rule is implied.

## Dependencies

- **Upstream**: [FR-022](../../routed/functional/FR-022-routed-generation.md) supplies scalar
  identities; [FR-024](./FR-024-counterexample-envelope-intake.md) owns QSL envelope submission;
  [FR-029](../../kani/functional/FR-029-run-outcome-terminal-record.md) owns terminal readings;
  QSL-641 owns the missing scalar arm, native-observation seam and harness-defect cause.
- **Related**: [FR-016](./FR-016-witness-native-replay.md) owns existing playback/function replay
  rules; [AD-002](../../assurance/AD-002-cg-qsl-replay-seam.md) retains QSL evaluation ownership.
- **Downstream**: [TC-047](../matrix/TC-047-routed-scalar-replay-binding.md), planned public
  consumer and real-backend evidence for IR-631; driver E9 calls the admitted QSL seam once
  its prerequisites are resolved.
