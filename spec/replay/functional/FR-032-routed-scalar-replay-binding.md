---
id: FR-032
title: "Bind routed scalar lowering-parity witnesses to QSL replay"
type: FR
relationships:
  - target: ix://agent-ix/quire-contract-codegen/StR-001
    type: satisfies
  - target: ix://agent-ix/quire-contract-codegen/FR-015
    type: depends_on
  - target: ix://agent-ix/quire-contract-codegen/AD-002
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-017
    type: depends_on
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

The proposed scalar plan is constrained by
[AD-002](../../assurance/AD-002-cg-qsl-replay-seam.md), including R-6 and R-7. Their scalar
contract fit is an explicit upstream gate below. This document grants no exception to those
rules. Scalar lowering parity is never a source-property refutation.

## Inputs

- The `ScalarObligationIdentity` returned by
  [FR-022](../../routed/functional/FR-022-routed-generation.md), including the claimed node,
  catalogued operation identity, qualified harness name and ordered operand schema.
- The falsified run's retained assertion playback. The scalar builder owns its decode byte
  ceiling: the same 8 MiB per-harness ceiling owned by
  [FR-017](../../kani/functional/FR-017-kani-execution-evidence.md) AC-14 and the Kani capture
  adapter. Before adapter parsing, the builder checks the selected single-harness playback
  length against that ceiling. The grouped launcher limit remains 8 MiB times its harness
  count; that process bound does not authorize a larger selected scalar playback.
- The original scalar generation/proving context, retaining the renderer's own accounting
  limits alongside the generated source. CG's scalar generation authority records those
  original limits; the caller preserves that record through the proving run. The builder
  compares replay limits with that original context, not with a caller's new replay budget.
  This context producer is planned IR-631 work: `ScalarObligationIdentity` does not currently
  persist limits. The current renderer's limit fields are all `u64::MAX`; there is no missing-
  context default or inference from the current identity. The limits and generated source
  must be tied by the canonical proof-content identity.
- The checked/emitted package used to derive this node's operand and result ranges, the proving
  context, and the original proving `call_site` package identity and source bytes needed by
  QSL's envelope. Locating the enclosing source function for compilation does not turn this
  scalar node into a function-contract obligation or predicate.
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
  upstream harness-defect reading and QSL-reported lowering divergence each require an
  expressly legal settlement/terminal representation; both are gated and cannot be fabricated
  from existing predicate replay causes or by silently widening existing settlement variants.
- For the interim unsupported-capability setup refusal, a typed converter refusal with no
  settlement. The missing upstream capability is not relabelled a CG defect or assigned a
  fabricated QSL code. The caller cannot manufacture a terminal record from that refusal;
  attempted terminal mapping without settlement remains governed by
  [FR-029](../../kani/functional/FR-029-run-outcome-terminal-record.md) AC-15.

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

The scalar generation proposition is owned solely by
[FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md) AC-37; its literal-operand rule
is AC-16. The replay projection refers to those criteria rather than redefining them. This
slice adds no refusal-cause equality, admitted-charge sequence or consumed-counter equality.
The original limits remain inputs so the driver and QSL execute the proving context.

The current profile comprises only `quire.op.integer.add`, `quire.op.integer.sub`,
`quire.op.integer.mul` and `quire.op.integer.negate` over the persisted `i64` operands.
These operations are mathematically defined for every admitted operand vector. An observed
`Undefined` therefore cannot agree with this profile; like `Incomplete` or an execution fault,
it does not conform to the current harness's `_ => false` assertion and cannot become
harness-defect agreement. A generic upstream undefined-agreement rule for other operators
does not expand this profile. Adding an operator or another parity projection requires its
own generation and replay acceptance criteria.

`run_terminal_value` and absent-settlement behavior remain owned by
[FR-029](../../kani/functional/FR-029-run-outcome-terminal-record.md). CG-only malformed/decode
and assumption-domain setup failures retain its `CgDefect` reading. Unsupported-capability
refusal returns no settlement. QSL-reported divergence is not automatically an existing
`CgDefect` (documented as CG-raised) or `Fault` (documented as an executor fault); its legal
representation is an upstream/terminal gate. No predicate disagreement stands in for a missing
scalar cause.

## Setup Refusal Precedence

The builder uses the first applicable refusal in this order, even while the upstream gate holds:

1. Original proof-context, node/operator/schema, package/source and generated-content identity
   coherence, including retention of the original scalar limits.
2. Selected playback byte ceiling, checked before parsing through the Kani output adapter.
3. Adapter selection and decoding: assertion block, qualified harness name, arity, byte width
   and persisted operand ordering.
4. Decoded operand-domain validation against the original schema.
5. Availability of all upstream capabilities needed for an admitted plan.

An otherwise valid request reaches the distinct unsupported-capability refusal at step 5.
A malformed request reaches its earlier typed refusal; the absent capability never hides it.
After plan completion, the native-observation/proof-content tie is checked before QSL evaluation.
Cross-binding converter failures return no settlement rather than settling another obligation.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-032-AC-1 | PLANNED (IR-631) / GATED (QSL-641). A public consumer passes the routed harness's `ScalarObligationIdentity` directly to the scalar builder; its typed call constructs neither `KaniObligationIdentity` nor V1 `ReplayPackage`. After the ordered setup checks, an otherwise valid request returns the unsupported-capability refusal while the gate holds; conversion returns no settlement rather than a synthetic function request or invented terminal cause. | Test |
| FR-032-AC-2 | PLANNED (IR-631). The builder decodes actual routed-harness playback in persisted symbolic call order through the Kani output adapter; another harness name, missing/extra operand, wrong byte width, playback above the builder-owned 8 MiB per-harness ceiling or malformed assertion block yields the distinct typed refusal under the stated precedence before replay. | Test |
| FR-032-AC-3 | PLANNED (IR-631) / GATED (QSL-641). Each operand at its own lower or upper bound is admitted and the adjacent out-of-domain value is refused before replay. The admitted QSL inputs preserve the catalogued operator, operands, operand ranges, result range and harness limits. Another node/operation, altered operand schema or replay limit differing from the retained original generation context refuses; operand correspondence uses [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md) AC-16. | Test |
| FR-032-AC-4 | PLANNED (IR-631) / GATED (QSL-641). The driver's actual native observation of the proved generated oracle is required and tied to this operand vector, limits and canonical proof-content identity; missing observation, another artifact's observation or fresh regeneration that removes the arithmetic mutation refuses before QSL evaluation. Playback bytes and Kani falsification alone cannot supply a generated outcome. | Test |
| FR-032-AC-5 | PLANNED (IR-631) / GATED (QSL-641). The public consumer executes the same proved generated artifact and invokes QSL between the builder and converter. QSL compares its actual native observation with authoritative exact operator evaluation using the four-operator projection owned by [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md) AC-37, without adding refusal-cause or accounting-counter equality; changing the generated arithmetic while keeping original node/operator identity changes the measured parity result. `Undefined`, `Incomplete` and execution faults cannot be agreeing outcomes of this profile. No CG-local evaluator or replay verdict replaces QSL. | Test |
| FR-032-AC-6 | PLANNED (IR-631) / GATED (QSL-641). The converter refuses another scalar binding's result/run with no settlement; changing only the claimed node, operation, operands, result range or limits cannot yield a settlement for the original binding. | Test |
| FR-032-AC-7 | PLANNED (IR-631) / GATED (QSL-641). QSL's divergence between generated and authoritative exact outcomes becomes a CG lowering fault and `Failed`; agreement despite Kani falsification becomes `Inconclusive` with the upstream named harness-defect cause; QSL non-fault refusal retains its catalog code and executor fault remains `Failed`. This scalar route never returns `Reproduced`, `Refuted` or `Verified`. | Test |
| FR-032-AC-8 | PLANNED (IR-631) / GATED (QSL-641). The real QSL-emitted `x + 1` with input `Int[0,9]` and result `Int[0,10]` exercises routed generation, actual Kani arithmetic-mutation playback and same-proved-artifact native observation. A separate bounded-addition fixture with both operands and result in `[-1000,1000]` reaches `(600,600)`, so its expected exact result-range refusal at `1200` is observed and never labelled a source violation. The fixture ranges are checked before that observation; no unreachable case counts as coverage. | Test |

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

The generation authorities are [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md)
AC-37 and AC-16. Existing source-inspection and real-backend tests of those criteria are not
new scalar-replay coverage. The input limits-context record and scalar decode ceiling are new
adapter obligations; no such context is claimed present in `ScalarObligationIdentity` today.

The emitted increment fixture's result range is `Int[0,10]`, as
[FR-022](../../routed/functional/FR-022-routed-generation.md) AC-15 and
`tests/it/routed_generation.rs` record. It supplies no result-range-refusal witness. The separate
addition descriptor already accepted by `tests/it/kani_obligations.rs::scalar_package` has
operand/result bounds `[-1000,1000]`; `(600,600)` is a concrete in-domain operand vector with
an out-of-result-range exact result. Its QSL source counterpart and original `call_site`
package/source binding must be admitted under the same upstream gates as the positive replay;
no hand-edited emitted package or fabricated source identity substitutes for that admission.

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
3. A named upstream harness-defect inconclusive cause and an expressly legal settlement/terminal
   representation for both that cause and QSL-reported generated/exact divergence. The existing
   `CgDefect` and `Fault` documentation cannot be widened silently. CG must not invent a QSL
   cause/code or retrofit a predicate disagreement.
4. AD-002 R-6: a QSL-owned scalar `ObligationIdentity` and authoritative scalar preimage that
   CG can mint from the obligation members and carry unchanged into request, envelope and
   settlement. That recipe is absent today; neither a function-contract preimage nor a
   transcript digest substitutes for it.
5. AD-002 R-7: the scalar arm retains the **original** proving `call_site` package identity and
   source bytes; QSL recompiles those bytes, checks the same package identity, and validates
   scalar node/operator membership. The existing facade can locate an enclosing non-Boolean
   function for compilation, but its predicate replay selection is still refused; the planned
   operator arm must evaluate this scalar node without a synthetic predicate selection.

AD-002's existing CG replay-call list still constrains CG: the planned builder makes no execution
call, and the driver executes through QSL's public replay facade. This contract neither adds a
fourth CG executor nor claims a new call-site selection. If QSL-641 cannot retain R-6/R-7 or the
facade allocation, the scalar route remains gated pending an explicit owning-contract decision;
this document grants no architecture exception.

All acceptance criteria are **Planned**. AC-1's admitted call, AC-3's admitted QSL inputs and
AC-4 through AC-8's completed scalar route are additionally **Gated** on the above contracts;
no test coverage is claimed. Unsupported-capability refusal, ordered decoding and operand
validation can be implemented independently. A future function-level declared-parameter parity
route is a distinct upstream capability and is outside this operator-level contract.

The builder's operand-domain check precedes every replay, as
[FR-024](./FR-024-counterexample-envelope-intake.md) already requires. A decoded value outside
the Kani assumed domain is a CG setup defect and `Failed`. QSL's own range validation is an
envelope backstop whose non-fault refusal keeps its QSL code; it does not authorize bypassing
the CG check. The existing envelope and architecture requirements remain authoritative while the scalar route is gated.

## Dependencies

- **Upstream**: [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md) owns the proved projection and literal bindings; [FR-017](../../kani/functional/FR-017-kani-execution-evidence.md) owns capture bounds; [FR-022](../../routed/functional/FR-022-routed-generation.md) supplies scalar
  identities; [FR-024](./FR-024-counterexample-envelope-intake.md) owns QSL envelope submission;
  [FR-029](../../kani/functional/FR-029-run-outcome-terminal-record.md) owns terminal readings;
  QSL-641 owns the missing scalar arm, native-observation seam, scalar identity recipe, original-package validation and legal scalar settlement causes.
- **Related**: [FR-016](./FR-016-witness-native-replay.md) owns existing playback/function replay
  rules; [AD-002](../../assurance/AD-002-cg-qsl-replay-seam.md) retains QSL evaluation ownership.
- **Downstream**: [TC-047](../matrix/TC-047-routed-scalar-replay-binding.md), planned public
  consumer and real-backend evidence for IR-631; driver E9 calls the admitted QSL seam once
  its prerequisites are resolved.
