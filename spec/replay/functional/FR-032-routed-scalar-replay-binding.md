---
id: FR-032
title: "Bind routed scalar witnesses without changing the proved proposition"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-022
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-029
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/FR-016
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-024
    type: references
  - target: ix://agent-ix/quire-spec-language/ADR-013
    type: references
---
# FR-032: Bind routed scalar witnesses without changing the proved proposition

## Description

When the driver requests replay of a routed scalar counterexample, CG shall bind the request
directly to its `ScalarObligationIdentity`, preserving the proposition the generated harness
actually checked. This requirement owns a public scalar request builder and a public settlement
converter; the driver owns the intervening QSL replay call.

This is planned work for IR-631. No scalar replay API or positive scalar replay test is claimed
to exist. The positive route is gated on the QSL capability and proposition decision below.
Function, frame and state-clause replay retain their existing contracts.

## Scope

The slice is one routed exact-scalar Kani item, its retained assertion playback, and its own
checked/emitted package. It includes typed setup refusals and the converter to the existing
`ReplaySettlement` read by [FR-029](../../kani/functional/FR-029-run-outcome-terminal-record.md).
It excludes driver orchestration and FR-331 serialization, new operation-contract clause-kind
generation, moving existing replay calls, a local scalar evaluator, copied QSL types, and a
conversion through `KaniObligationIdentity` or the V1 `ReplayPackage`.

## Inputs

- The `ScalarObligationIdentity` returned by
  [FR-022](../../routed/functional/FR-022-routed-generation.md), including the claimed node,
  catalogued operation identity, qualified harness name and ordered operand schema.
- The falsified run's retained Kani assertion playback and a bounded decode limit.
- The checked/emitted package and authoritative source/operand correspondence for that node;
  the proving context's supplied source bytes, dependency selections and replay limits.
- At conversion, the same scalar binding and falsified run, plus QSL's own result or refusal
  from the driver's replay call, or the builder's typed setup refusal.

## Outputs

- An admitted typed scalar binding and a QSL-owned request/plan expressible through QSL's
  public facade, or a typed setup refusal naming the unrepresentable selection, missing operand
  correspondence, identity mismatch, malformed playback or out-of-domain operand.
- A checked conversion to `ReplaySettlement`, or a typed binding refusal with no settlement
  when a result or run belongs to another obligation.

The admitted request's concrete upstream type and scalar proposition representation are
deliberately unresolved. Existing upstream types do not express this route. This document
does not introduce a substitute QSL type or promise an implementable positive signature.

## Behavior

- The public scalar builder shall accept `ScalarObligationIdentity` directly.
- The builder shall select and decode playback through the Kani output adapter against the
  scalar identity's own qualified harness and persisted operand order, with one eight-byte
  `i64` value per symbolic operand and no function-identity coercion.
- When producing an admitted request, the builder shall validate the node and operation
  against the supplied package, every operand's declared bounds, and the authoritative
  correspondence between symbolic operand positions and the selected QSL computation.
- If that correspondence changes a literal, permutes operands, substitutes another node or
  operation, or requires a selection QSL cannot represent, then the builder shall return a
  distinct typed setup refusal before the driver can call QSL.
- The builder shall preserve the checked scalar proposition, rather than interpret a failure
  of generated arithmetic parity as a false source predicate or a source-result-domain violation.
- CG's scalar builder and converter shall leave the replay execution to the driver through
  QSL's public facade.
- When returning a settlement, the converter shall validate that its scalar binding, retained
  run playback and QSL evidence belong to the same node, operation and checked proposition.
- The converter shall read only QSL's typed result/refusal discriminants and existing catalog
  codes, rather than infer a verdict or a refusal code from message text.
- For a representable scalar proposition, the converter shall apply the existing terminal
  readings: QSL's genuine reproduction of that same violation yields `Reproduced`, disagreement
  yields `Disagreement`, a non-fault QSL refusal yields `Refused` or `SetupRefused` with its own
  code, an executor fault yields the existing fault reading, and a CG setup defect without a
  QSL catalog code yields `CgDefect`.

`run_terminal_value` remains the terminal map. Its refusal of a falsified run with no settlement
is already [FR-029](../../kani/functional/FR-029-run-outcome-terminal-record.md) AC-15. The
converter does not manufacture an identity-bearing terminal record or change that rule.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-032-AC-1 | A public consumer passes the routed harness's `ScalarObligationIdentity` directly to the scalar builder; its typed call constructs neither `KaniObligationIdentity` nor V1 `ReplayPackage`. An unsupported scalar selection returns a typed setup refusal instead of a synthetic function request. | Test |
| FR-032-AC-2 | The builder decodes the routed harness's actual playback in persisted symbolic call order through the Kani output adapter; another harness name, missing/extra operand, wrong byte width, over-limit playback or malformed assertion block yields a distinct typed refusal before replay. | Test |
| FR-032-AC-3 | An operand at either declared bound is admitted when the source correspondence permits it; a value immediately outside either bound is refused before replay. A changed node, catalogued operation, operand ordering/domain or value substituted for a source literal is refused rather than rebound to a different computation. | Test |
| FR-032-AC-4 | A routed scalar arithmetic-parity falsifier is not converted into `Reproduced` merely because it exists or because the original source returns a value outside its result bound. Expected runtime refusal outside that bound does not become a source violation. If QSL cannot replay the actual checked proposition, the builder returns the unsupported-selection setup refusal. | Test |
| FR-032-AC-5 | The builder returns its admitted request to the public consumer, which invokes QSL itself and returns QSL's result to the converter. No scalar builder or converter executes replay or supplies a locally computed replay verdict. | Test |
| FR-032-AC-6 | The converter refuses a result/run paired with another scalar binding with no settlement; changing only the claimed node or operation cannot produce a settlement for the original binding. Genuine same-proposition evidence is required before `Reproduced` can be returned. | Test |
| FR-032-AC-7 | For QSL-admitted scalar evidence, disagreement maps through `run_terminal_value` to `Inconclusive(ReplayParity)`, non-fault QSL refusal to `Inconclusive(ReplayRefused)` preserving its catalog code, and executor fault to the existing `Failed` reading. A CG-only unsupported selection or malformed binding maps to `CgDefect` and `Failed`, not a fabricated QSL refusal. | Test |
| FR-032-AC-8 | The public routed test uses a real QSL-emitted `x + 1` over `Int[0,9]` package, `generate_routed`, its returned scalar identity and real Kani playback. It observes the unsupported-proposition refusal until QSL owns an admitted replay representation of that exact proposition; a positive `Refuted` observation is accepted only after the proposition decision below is resolved and QSL reproduces that same typed violation. Omitting settlement still yields `TerminalPairError::MissingSettlement` and no terminal value. | Test |

## Intent and Existing Coverage

This table records the authoring gap analysis, not a computed test matrix.

| Intent | Existing authority and measured tests | Remaining gap |
|---|---|---|
| Real routed scalar generation | [FR-022](../../routed/functional/FR-022-routed-generation.md) AC-2/10/13/15; `tests/it/routed_generation.rs` carries those trace tags | The returned scalar identity has no public replay binding |
| Typed ordered playback and domain validation | [FR-016](./FR-016-witness-native-replay.md) AC-1/2/5/8; `tests/it/kani_witness_join.rs`, `kani_argument_order.rs` and `skeleton_spine.rs` exercise the function schema | Direct scalar schema and source-operand correspondence |
| QSL parity/refusal and terminal reading | [FR-016](./FR-016-witness-native-replay.md) AC-9/10/11/12 and [FR-029](../../kani/functional/FR-029-run-outcome-terminal-record.md) AC-1/10/13; `skeleton_spine.rs` and `terminal_map.rs` cover existing replay paths; AC-12 remains planned in the replay index | A scalar result bound to the actual scalar proposition |
| Missing settlement produces no terminal value | [FR-029](../../kani/functional/FR-029-run-outcome-terminal-record.md) AC-15; `tests/it/terminal_map.rs` | Reuse the rule in the public routed integration, without a duplicate terminal requirement |
| Driver calls replay between public typed builder and converter | No positive scalar requirement or test on the inspected source | This requirement and planned [TC-047](../matrix/TC-047-routed-scalar-replay-binding.md) |

## Prerequisites and Unresolved Proposition

The inspected scalar renderer (`src/kani/generate/scalar.rs`) checks generated oracle/native
`i128` parity: `Completed` must equal the native result inside the result range; `Refused` must
occur outside it. Therefore an out-of-result-range computation is an expected refusal that
satisfies the harness. A falsification of this harness exposes arithmetic parity failure; it
does not establish that a QSL predicate is false. The renderer draws every operation operand
symbolically; a recorded literal has singleton bounds, while an operand lacking a more specific
range uses the result range. `ScalarObligationArgument`
persists a generated identifier and bounds, not a QSL parameter/node correspondence.

The inspected QSL main facade's `CallSiteSelection` admits function, operation, clause, field and
population selections, with no scalar-expression selection. Its `replay` selection rejects a
non-Boolean function with `ReplayRefusal::NotAPredicate`. The present `x + 1` scalar item therefore
cannot be replayed honestly by inventing a Boolean function identity or renaming its operands.

Before the positive route is implementable, the owning lanes must settle:

1. Whether the driver intends to prove this existing oracle-parity proposition or a distinct
   source obligation. The latter requires its own authoritative generation contract; it must
   not silently replace the current harness proposition in this adapter.
2. QSL's public typed selection, witness and evidence for the chosen scalar proposition,
   including what is a violation, an expected domain refusal, and a generated-code defect.
3. Authoritative operand provenance, including literals and repeated/derived operands, and
   the evidence identity required to join QSL's result with this scalar node and operation.

Until resolved, AC-5's admitted execution branch, AC-6's positive evidence branch, AC-7's
QSL-admitted result branch and AC-8's positive `Refuted` branch are **Planned/Gated**.
The unsupported-selection refusal and proposition-preservation checks remain implementable
without claiming the missing upstream capability. No normative positive scalar proposition or
new QSL type is invented here.

## Dependencies

- **Upstream**: [FR-022](../../routed/functional/FR-022-routed-generation.md) supplies routed scalar
  identities; [FR-029](../../kani/functional/FR-029-run-outcome-terminal-record.md) owns terminal
  readings; QSL owns the missing public scalar replay capability and evidence semantics.
- **Related**: [FR-016](./FR-016-witness-native-replay.md) owns function replay and playback
  rules; [FR-024](./FR-024-counterexample-envelope-intake.md) owns existing envelope paths.
- **Downstream**: [TC-047](../matrix/TC-047-routed-scalar-replay-binding.md), planned public
  consumer and real-backend evidence for IR-631; driver E9 uses the admitted seam once its
  prerequisites are resolved.
