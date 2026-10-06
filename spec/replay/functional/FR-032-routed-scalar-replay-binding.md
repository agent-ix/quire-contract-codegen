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
observation contract below. The measured QSL operator API is described below; the full-report
identity contract is still a CODE gate. No CG implementation or passing scalar-replay test is claimed.

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

The driver is the native-observation authentication authority. CG supplies the retained
scalar generation/proving record and a checked plan-completion/conversion contract; the
builder makes no native execution or QSL replay call. The driver executes the **same proved
generated artifact** at the decoded operand vector and retained renderer limits, and authenticates
the typed outcome against that record's canonical proved generated-content identity, operands
and limits before completing the request. Missing evidence or a mismatch returns a typed
completion refusal, with no QSL evaluation or settlement. An unchecked caller echo of outcome
and digest, or a newly regenerated oracle, does not meet this contract.

This boundary trusts the driver to execute and authenticate that artifact honestly. QSL does
not receive or execute the artifact: it carries the supplied canonical content `DigestRecord`
and compares exact semantics against the supplied typed observation. QSL carrying that record
is not independent authentication of the observation. CG's checked completion contract must
require the driver's authenticated observation, rather than silently accepting a bare outcome.
The concrete producer/consumer representation remains CODE-gated; no signing scheme, tracking
hash or new observation API is invented here.

## Outputs

- A typed admitted scalar binding and QSL-owned replay plan for the operator-level
  lowering-parity arm, or a distinct typed setup refusal. The driver completes the plan with
  its actual native generated observation before calling QSL; it executes no regenerated oracle.
- A checked conversion to the existing settlement where representable, or a typed binding
  refusal with no settlement when a result or run belongs to another obligation. The new
  scalar terminal readings follow the measured QSL contract below; their CG settlement
  representation and checked conversion remain planned and gated on full-report identity.
  Existing predicate replay causes do not substitute for scalar readings.
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
- When asked to return a settlement or derive its terminal value, the converter shall compare
  the QSL report's full claim identity with the exact retained claim sent by the driver, and
  validate the retained run-to-binding tie. This check applies to **every** outcome, including
  `Diverged`, `Agrees`, `GeneratedFault`, `RefusedInput`, exact `Incomplete` and other `Refused`.
  Equality of the obligation digest alone, or identity present only on agreement, is insufficient.
  A missing or different report claim yields a typed binding refusal with no settlement.
  The report `claim()` contract is measured on unmerged QSL change #650; CODE remains
  gated on its merge and actual CG consumer conformance.
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
refusal returns no settlement. After successful full-claim comparison, QSL `Diverged` and
`GeneratedFault` read as `Failed`; `Agrees` reads as `Inconclusive(ScalarAgrees)`; `RefusedInput` reads as
`Inconclusive(ReplayRefused(invalid_runtime_input))`; exact `Incomplete` reads as
`Incomplete(ResourceExhausted)`; other QSL `Refused` follows the existing typed QSL refusal
mapping and preserves its catalog code for non-fault refusals. A native `Incomplete` is a
`GeneratedFault`, not exact resource exhaustion. CG's scalar settlement representation must
preserve these distinctions; it must never yield `Reproduced`, `Refuted` or `Verified`.

## Scalar Obligation Identity

CG owns and mints the scalar `ObligationIdentity`; QSL carries it as opaque. The scalar
operation-application allocation below is measured in the pushed ADR-013 O-09 and FR-357
amendment on **unmerged [QSL change #650](https://github.com/agent-ix/quire-spec-language/pull/650)**. Its spec and source still carry the obligation digest
without recomputing or checking it; no scalar digest-verification API or exact encoded-member
spelling is established by that branch. CODE waits for the merged owning contract and actual
CG conformance; the announced recomputation change remains unresolved against the pushed text.

The scalar subject is the claimed operator's FR-322 application node id and its authentic
O-07 occurrence key: that node id with the selected `CheckedOccurrence`'s role `expression`
and ordinal. The preimage contains only that subject, occurrence, the existing obligation kind,
and `arguments`. There is **one argument per operand position**, in operand/harness draw order.
Each argument contains its position ordinal, typed `OperandIdentity` and actual operand range
used by the harness. The identity is a tagged representation: `GraphChild { node_id }` for an
actual graph reference (including a parameter or subterm), or `InlineLiteral { node_id,
occurrence_key, position }` for an inline literal, using the authentic application node id,
selected occurrence key and operand position ordinal. An inline literal's singleton range
carries its value (FR-015 AC-16); QSL emits no extra literal node for it. A reference to an
existing literal node is a graph child. Neither tag fabricates a checked node id.
Repeated independent draws of the same parameter remain distinct position entries. CG shall
neither sort these scalar entries by parameter identifier nor deduplicate their node ids.
Unrelated enclosing-function parameters are absent. This positional scalar rule does not
change function/frame identity members, encoding or their existing identifier ordering.

CG shall obtain the graph-child node ids, selected occurrence and positional operand correspondence
from the authoritative checked/emitted proving package and retain them through generation and
proving. No generated symbol, operand index used as a node id, chosen occurrence ordinal or
application id mislabeled as `GraphChild` substitutes for that metadata. Position is an argument
member; the typed operand identifier holds the valid tagged representation above. The persisted
draw order and operator operand order must correspond exactly to the retained positional arguments.

CG shall encode this owning scalar shape through its existing one canonical place
(`core::canonical`, RFC 8785 via `quire-canonical`); the scalar positional member encoding must
follow the measured amended owning contract before CODE admission. No additional CG member,
transcript, native outcome, generated-content digest, operator label, solver, renderer counter,
build or tool/version tracking field enters the preimage. Source regions/spans are excluded.
The full replay claim separately carries its operator, decoded operands/ranges, result range,
retained renderer limits and canonical generated-content identity. It and same-artifact
observation authentication remain distinct from the O-09 obligation digest.

Measured IR `CheckedPackageV2::graph()` exposes nodes with authoritative `node_id` and
`occurrences`, and `source_map()` exposes their source map; `CheckedOccurrence` supplies role
and ordinal. The current CG scalar record does not retain its selected expression occurrence
or typed operand identities. IR's public node `body` is still a JSON value; no public typed accessor
for the scalar application's ordered operands, their identities and ranges was measured.
The model-field accessor and wrapped optional operation-leaf walk do not supply that scalar
accessor. IR-648 must expose the same `GraphChild`/`InlineLiteral` identity distinction;
inline-literal identity uses the application/occurrence/position and requires no literal node.
CODE remains gated on [IR-648](https://linear.app/agent-ix/issue/IR-648), the typed scalar
operand accessor on `CheckedPackageV2`, and on retaining the occurrence selected for the proved item. Missing or ambiguous occurrence, operand identity/range
or existing kind yields typed setup refusal and no minted identity. The positional rule resolves
literal and repeated-parameter semantics; no ambiguity gate for those semantics remains.
Decoder and original-limits context work can proceed independently.

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
| FR-032-AC-4 | PLANNED (IR-631) / GATED (QSL-641). The driver's actual native observation of the proved generated oracle is required and tied to this operand vector, limits and canonical proof-content identity; the checked completion requires driver authentication; a bare outcome/digest echo, missing observation, another artifact's observation or fresh regeneration that removes the arithmetic mutation refuses before QSL evaluation. Playback bytes and Kani falsification alone cannot supply a generated outcome. | Test |
| FR-032-AC-5 | PLANNED (IR-631) / GATED (QSL-641). The public consumer executes the same proved generated artifact and invokes QSL between the builder and converter. QSL compares its actual native observation with authoritative exact operator evaluation using the four-operator projection owned by [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md) AC-37, without adding refusal-cause or accounting-counter equality; changing the generated arithmetic while keeping original node/operator identity changes the measured parity result. `Undefined`, `Incomplete` and execution faults cannot be agreeing outcomes of this profile. No CG-local evaluator or replay verdict replaces QSL. | Test |
| FR-032-AC-6 | PLANNED (IR-631) / GATED (QSL-641). For every `Diverged`, `Agrees`, `GeneratedFault`, `RefusedInput`, exact `Incomplete` and other `Refused` outcome, the converter compares the report's full claim to the retained sent claim before conversion; a missing claim, another result/run, or changing only the obligation, node, canonical generated-content identity, operator, operand value/range, result range or limits yields typed binding refusal and no settlement. Obligation-digest equality alone and agreement-only identity cannot pass. | Test |
| FR-032-AC-7 | PLANNED (IR-631) / GATED (QSL-641). QSL's divergence between generated and authoritative exact outcomes becomes a CG lowering fault and `Failed`; agreement despite Kani falsification becomes `Inconclusive(ScalarAgrees)`; native `Incomplete`/execution fault becomes `GeneratedFault` and `Failed`, while exact `Incomplete` becomes `Incomplete(ResourceExhausted)`. `RefusedInput` preserves `invalid_runtime_input`; other typed QSL refusals follow the existing refusal map and non-fault refusals retain their code. This scalar route never returns `Reproduced`, `Refuted` or `Verified`. | Test |
| FR-032-AC-8 | PLANNED (IR-631) / GATED (QSL-641). The real QSL-emitted `x + 1` with input `Int[0,9]` and result `Int[0,10]` exercises routed generation, actual Kani arithmetic-mutation playback and same-proved-artifact native observation. A separate bounded-addition fixture with both operands and result in `[-1000,1000]` exercises an admitted operand pair whose exact result is outside the result range. The expected exact result is derived from the actual retained playback, and its correct refusal is never labelled a source violation. The fixture ranges and witness domain/result-range relation are asserted before that observation; no unreachable case or assumed solver choice counts as coverage. | Test |
| FR-032-AC-9 | PLANNED (IR-631) / CODE-GATED (merged owning scalar rule and authoritative metadata access). Independent canonical-preimage checks include only the claimed application node, authentic expression occurrence, existing kind and one argument per operand position in actual draw order, each with position ordinal, typed `OperandIdentity` and harness operand range. Actual graph references use `GraphChild { node_id }`; inline literals use `InlineLiteral { node_id, occurrence_key, position }` from the authentic application/occurrence and position, with their value in the singleton range. QSL emits no extra literal node. A referenced existing literal, parameter or subterm uses its graph node id. Two independent draws of one parameter retain two position entries. Changing any included member changes identity; changing source span, native outcome or artifact-content identity does not. Missing occurrence, operand metadata or kind refuses with no identity. Identifier sorting or node-id deduplication cannot pass; unrelated enclosing-function parameters remain absent. | Test |

## Intent and Existing Coverage

This table records the authoring gap analysis, not a computed test matrix.

| Intent | Existing authority and measured tests | Remaining gap |
|---|---|---|
| Real routed scalar generation | [FR-022](../../routed/functional/FR-022-routed-generation.md) AC-2/10/13/15; `tests/it/routed_generation.rs` carries those trace tags | The returned scalar identity has no public replay binding |
| Typed ordered playback and domain validation | [FR-016](./FR-016-witness-native-replay.md) AC-1/2/5/8; `tests/it/kani_witness_join.rs`, `kani_argument_order.rs` and `skeleton_spine.rs` exercise the function schema | Direct scalar schema and operator-level replay inputs |
| QSL refusal and terminal reading | [FR-016](./FR-016-witness-native-replay.md) AC-9/10/11/12 and [FR-029](../../kani/functional/FR-029-run-outcome-terminal-record.md) AC-1/10/13; `skeleton_spine.rs` and `terminal_map.rs` cover existing replay paths; AC-12 remains planned in the replay index | Planned checked scalar conversion to the measured QSL causes; predicate violation/parity readings do not establish scalar semantics |
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
operand/result bounds `[-1000,1000]`; `(600,600)` giving `1200` illustrates a reachable
out-of-result-range exact result, without prescribing Kani's chosen witness. The check derives
its expected exact result from the actual retained operand pair. Its QSL source counterpart and
original `call_site`
package/source binding must be admitted under the same upstream gates as the positive replay;
no hand-edited emitted package or fabricated source identity substitutes for that admission.

The measured QSL `FR-357` and `qsl-replay/src/scalar.rs` expose
`replay_operator_parity`, `OperatorClaim`, `OperatorParityReport`, `NativeOutcome` and the
scalar result/terminal readings above. The operator entry recompiles the original source,
checks `package_id`, and validates scalar-node/operator membership in the selected enclosing
function; it does not replay that function as a predicate. `OperatorClaim.identity` carries a
canonical generated-content `DigestRecord`, expressly never recomputed or checked by QSL.
On the measured unmerged QSL change #650, `OperatorParityReport::claim()` exposes
`OperatorIdentity` on every outcome and `ValueParityReport::claim()` exposes `ValueIdentity`;
FR-357 AC-13/14 specify full-claim retention and carry-and-bind observation identity. The
operator report retains the obligation, node, operator, operands/ranges, result range, limits
and observation digest even when request decoding or context validation refuses. The prior
merged report carried only the obligation digest outside agreement. This is source inspection,
not a passing-test claim; no QSL or CG scalar tests were run for this SPEC change.

The positive operator-level route has these explicit **CODE gates**:

1. QSL-641's amended full-claim spec and report API, measured on unmerged change #650,
   must merge before CG CODE relies on `claim()` for both parity reports; verify the actual
   merged spec/source and CG consumer conformance. For this operator slice the full claim is the retained obligation identity,
   scalar node, canonical generated-content `DigestRecord`, operator, ordered operands with
   their ranges, result range and retained limits. The function-parity report requirement is an
   upstream consistency gate and authorizes no CG function-parity implementation here.
2. CG generation must retain the authoritative scalar metadata needed by the owning positional O-09
   preimage and original renderer-limit/proving record. The selected occurrence, typed positional
   operand identity/range access (IR-648) and existing kind retention gaps remain explicit;
   QSL-641 does not supply a CG-owned preimage by naming an opaque digest.
3. The driver must implement same-proved-artifact execution and observation authentication;
   CG must supply the retained generation/proving record and checked completion/conversion seam.
   QSL never sees or authenticates the artifact; carried digest equality alone is insufficient.
4. CG's converter/settlement representation must consume the measured legal scalar terminal
   readings only after full-claim and retained-run comparison. Existing predicate disagreement
   or a fabricated scalar cause must not stand in for that conversion.
5. AD-002 R-7 stays in force: retain the **original** proving `call_site` package identity and
   source bytes; QSL recompiles them, checks the same `package_id`, and validates scalar
   node/operator membership in the selected function. Retain the proving source/package tie;
   neither fresh oracle generation nor a synthetic predicate selection substitutes for it.

AD-002's existing CG replay-call list still constrains CG: the planned builder makes no execution
call, and the driver executes the generated artifact and separately calls QSL's public scalar replay
facade. This contract neither adds a fourth CG executor nor claims a new call-site selection. If QSL-641 cannot retain R-6/R-7 or the
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
the CG check. The existing envelope and architecture requirements remain authoritative while the
scalar route is gated.

## Dependencies

- **Upstream**: [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md) owns the proved
  projection and literal bindings; [FR-017](../../kani/functional/FR-017-kani-execution-evidence.md)
  owns capture bounds; [FR-022](../../routed/functional/FR-022-routed-generation.md) supplies scalar
  identities; [FR-024](./FR-024-counterexample-envelope-intake.md) owns QSL envelope submission;
  [FR-029](../../kani/functional/FR-029-run-outcome-terminal-record.md) owns terminal readings;
  CG owns the scalar preimage and retained producer/converter contracts; the driver owns observation
  authentication; QSL-641 owns the amended full-report claim API. Measured QSL FR-357 owns exact
  evaluation, original-package validation and scalar terminal causes. IR-648 in Contract IR
  owns the typed `CheckedPackageV2` scalar operand accessor and blocks IR-631 CODE admission;
  its spec and implementation must be measured before that accessor gate is closed.
- **Related**: [FR-016](./FR-016-witness-native-replay.md) owns existing playback/function replay
  rules; [AD-002](../../assurance/AD-002-cg-qsl-replay-seam.md) retains QSL evaluation ownership.
- **Downstream**: [TC-047](../matrix/TC-047-routed-scalar-replay-binding.md), planned public
  consumer and real-backend evidence for IR-631; driver E9 calls the admitted QSL seam once
  its prerequisites are resolved.
