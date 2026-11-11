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
  - target: ix://agent-ix/quire-spec-language/FR-357
    type: depends_on
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

The full proof-driven replay pipeline remains Planned under IR-631. CG's
original scalar body/limits retention, checked plan completion, native
observation comparison and the full-claim report check
(`CompletedScalarReplay::settle`, to be replaced by `settlement`, FR-032-AC-20)
are implemented; the typed settlement and its pairing with the Kani outcome
(FR-032-AC-20 to AC-24) are Planned. Native tests exercise the actual QSL
operator route; they do not establish execution of the same proved artifact
or supply a backend-success receipt.

## Scope

The slice is one routed exact-scalar Kani item, its retained assertion playback, its proved
generated oracle and its checked/emitted package. It includes typed setup refusals and conversion
to the settlement read by [FR-029](../../kani/functional/FR-029-run-outcome-terminal-record.md).
It excludes driver orchestration and FR-331 serialization, source-predicate violation, a
function-level declared-parameter parity route, new operation-contract clause-kind generation,
moving existing replay calls, a local exact evaluator or direct exact-kernel dependency in CG,
copied QSL types, and conversion through `KaniObligationIdentity` or V1 `ReplayPackage`.

The scalar plan is constrained by
[AD-002](../../assurance/AD-002-cg-qsl-replay-seam.md), including R-6 and R-7. Their scalar
contract fit is supplied by the merged operator contract; native input-binding
controls do not close the full proof-driven pipeline. This document grants no exception to those
rules. Scalar lowering parity is never a source-property refutation.

## Inputs

- The `ScalarObligationIdentity` returned by
  [FR-022](../../routed/functional/FR-022-routed-generation.md), including the claimed node,
  catalogued operation identity, qualified harness name and ordered operand schema.
- The falsified run's retained assertion playback, which is the witness-phase playback of FR-017's two-phase run (FR-017-AC-63); a proof-phase failure without a reproducing witness phase is not a falsified run and supplies none. The scalar builder owns its decode byte
  ceiling: the same 8 MiB per-harness ceiling owned by
  [FR-017](../../kani/functional/FR-017-kani-execution-evidence.md) AC-14 and the Kani capture
  adapter. Before adapter parsing, the builder checks the selected single-harness playback
  length against that ceiling. The grouped launcher limit remains 8 MiB times its harness
  count; that process bound does not authorize a larger selected scalar playback.
- The original scalar generation/proving context, retaining the renderer's own accounting
  limits alongside the generated source. CG's scalar generation authority records those
  original limits; the caller preserves that record through the proving run. The builder
  compares replay limits with that original context, not with a caller's new replay budget.
  The generated harness retains these limits through `original_scalar_limits()`
  alongside its original body and artifacts. The driver must preserve that
  authority through the proving run; this full pipeline remains IR-631 work.
  There is no missing-context default or inference from the current identity.
- The checked/emitted package used to derive this node's operand and result ranges, the proving
  context, and the original proving `call_site` package identity and source bytes needed by
  QSL's envelope. Locating the enclosing source function for compilation does not turn this
  scalar node into a function-contract obligation or predicate.
- The captured path and byte set of the **same proved generated oracle**. After building
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
the typed outcome against that record's retained staged bytes, operands
and limits before completing the request. Missing evidence or a mismatch returns a typed
completion refusal, with no QSL evaluation or settlement. An unchecked caller echo of outcome
or a newly regenerated oracle does not meet this contract.

This boundary trusts the driver to execute and authenticate that artifact honestly. QSL does
not receive or execute the artifact; it compares exact semantics against the supplied typed
observation. CG's checked completion contract requires the driver's authenticated observation,
not a bare outcome. The driver compares its original generated artifacts, harness and record
byte for byte with the receipt's retained staged entries under FR-017-AC-45 to AC-49. The
producer/consumer wiring remains CODE-gated.

## Outputs

- A typed admitted scalar binding and QSL-owned replay plan for the operator-level
  lowering-parity arm, or a distinct typed setup refusal. The driver completes the plan with
  its actual native generated observation before calling QSL; it executes no regenerated oracle.
- A checked conversion to `ScalarSettlement` (see Settlement Contract below), or a typed binding
  refusal with no settlement when a result or run belongs to another obligation. The scalar
  terminal readings follow the measured QSL contract below. `ReplaySettlement` gains no
  scalar variant, and existing predicate replay causes do not substitute for scalar readings.
- For setup refusal caused by a named unavailable capability below, a typed converter refusal with no
  settlement. The named missing capability is not relabelled a CG defect or assigned a
  fabricated QSL code. The caller cannot manufacture a terminal record from that refusal;
  attempted terminal mapping without settlement remains governed by
  [FR-029](../../kani/functional/FR-029-run-outcome-terminal-record.md) AC-15.

## Behavior

- The public scalar builder shall accept `ScalarObligationIdentity` directly.
- The builder shall select and decode playback through the Kani output adapter against the
  scalar identity's own qualified harness and persisted operand order, with one eight-byte
  `i64` value per symbolic operand and no function-identity coercion.
- When a scalar identity carries `guard_inputs` (FR-022), the builder shall decode one playback
  entry per guard input after the operand entries, in `guard_inputs` order: one eight-byte `i64`
  for an integer guard input and one byte for a Boolean guard input.
- If a decoded guard value lies outside its persisted `ScalarGuardDomain`, or a Boolean guard
  byte is neither `0` nor `1`, then the builder shall refuse the playback as
  `cg_scalar_witness_guard_out_of_domain` before replay.
- The builder shall not pass guard values to QSL as operands.
- When producing an admitted request, the builder shall validate the claimed node and
  catalogued operation against the proving package and every operand against its own declared
  bounds in persisted symbolic call order.
- The builder shall carry the operator, decoded operands, operand ranges, result range and
  harness accounting limits to QSL without replacing them with a selected function's arguments.
- The builder shall bind the replay plan to the operand vector, limits, node, operation and
  retained staged artifact bytes required of the driver's native observation.
- If an identity, schema, expected staged-byte tie or required upstream capability cannot be
  validated, then the builder shall return a distinct typed setup refusal before replay.
- CG's scalar builder and converter shall leave exact evaluation and replay execution to QSL
  through the driver's public-facade call.
- When asked to return a settlement or derive its terminal value, the converter shall compare
  the QSL report's full claim identity with the exact retained claim sent by the driver, and
  validate the retained run-to-binding tie. This check applies to **every** outcome, including
  `Diverged`, `Agrees`, `GeneratedFault`, `RefusedInput`, exact `Incomplete` and other `Refused`.
  Equality of the structural obligation identity alone, or identity present only on agreement, is insufficient.
  An absent report or different report claim yields a typed binding refusal with no settlement.
  The report `claim()` contract is measured in merged QSL change #650; the CG full-claim and
  sent-context checks are exercised by `tests/it/scalar_replay_binding.rs` (FR-032-AC-6, PARTIAL).
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
and assumption-domain setup failures retain its `CgDefect` reading through
`ScalarSettlement::CgDefect`; every other `ScalarBuildError` returns no settlement
(FR-032-AC-24). After successful full-claim comparison, QSL `Diverged` and
`GeneratedFault` read as `Failed`; `Agrees` reads as `Inconclusive(ScalarAgrees)`; `RefusedInput` reads as
`Inconclusive(ReplayRefused(invalid_runtime_input))`; exact `Incomplete` reads as
`Incomplete(ResourceExhausted)`; other QSL `Refused` follows the existing typed QSL refusal
mapping and preserves its catalog code for non-fault refusals. Source observation, owned by QSL: after context/site/identity verification, merged
`scalar.rs::compare` handles native `Incomplete` or `ExecutionFault`
as `GeneratedFault` before its operand-admission backstop. Thus a native fault plus an
out-of-range operand reaches `GeneratedFault`, not `RefusedInput`; CG setup still admits
operands before dispatch. This internal QSL ordering is attribution only, not a CG
acceptance obligation or a change to FR-357. Native `Incomplete` is not exact resource exhaustion. CG's scalar settlement representation must
preserve these distinctions; it must never yield `Reproduced`, `Refuted` or `Verified`.

## Scalar Obligation Identity

CG obtains the scalar claim identity from the admitted IR reader's FR-322 structural `NodeKey`/`CheckedNodeId` for the original application and retains the authentic expression occurrence from that read. CG typed generation supplies the scalar kind and positional operand identities, ranges and provenance; CG retains their complete checked claim body for comparison. The identity is compared as an exact string or structural value; CG does not derive an obligation hash or encode a parity preimage. QSL-1293 owns removal of the old shared mint and digest check. A different package, path, or producer ordinal yields a different structural identity; a body-only change under the same identity is refused by the retained-body comparison.
`operator_parity.rs` recompiles the original context, checks the application node/operator and
selected enclosing-function membership, authenticates the occurrence and positional operands,
then compares the claimed structural identity and complete retained claim body before scalar comparison. A mismatched identity or body refuses with the owning typed stale-dependency/revision-mismatch result; there are no claimed or recomputed digests. The native CG/QSL controls below exercise this boundary; the complete
same-proved-artifact pipeline remains Planned under IR-631. Function/clause
carry-only contracts and function/frame identity rules remain unchanged.

The following CG-owned allocation defines the closed scalar wire vocabulary.
CG retains it as `ScalarReplayKind` in the original `ScalarReplayBody`; it is
separate from the clause-role `ObligationKind`. QSL carries the wire string
opaquely.

| Scalar family actually minted by this route | O-09 `obligation_kind` wire value | Authority/source |
|---|---|---|
| Integer arithmetic lowering parity (`add`, `subtract`, `multiply`, `negate`) | `operator_parity` | CG generation retains `ScalarReplayKind::OperatorParity` in the original scalar body and generated record. Native public-accessor and replay tests exercise it; the full proved-artifact pipeline remains Planned under IR-631. |

This is the sole current scalar-family value. CG shall not derive it from `operation_identity`,
reuse a precondition/postcondition/invariant/frame role, or accept a caller-chosen free string.
Future parity families require their own owning allocation; the composite family is specified
separately. When original typed retention is unavailable,
`scalar_replay_body()` returns its typed authority refusal and the builder
refuses without minting an identity. This refusal remains required even though
the typed generation/accessor is implemented.

**Scalar identity conformance.** For an admitted current-run scalar claim, the replacement AC-15/AC-16 checks use the authoritative IR-read application identity and expression occurrence together with the CG-generated and retained scalar kind and ordered operand identities and ranges, then require the actual QSL operator route to accept the matching structural claim. The adverse controls change one included member at a time, retain
repeated operand positions and refuse invented occurrence or operand membership. A synthetic
encoding example alone is not evidence of authoritative membership, generated metadata retention
or passing routed replay.

The scalar subject is the claimed operator's FR-322 application node id and its authentic
O-07 occurrence key: that node id with the selected `CheckedOccurrence`'s role `expression`
and ordinal. The retained checked claim contains that subject, occurrence, the scalar obligation-kind value from the allocation above, and positional `arguments`. There is **one argument per operand position**, in operand/harness draw order.
Each argument contains its position ordinal, typed `OperandIdentity` and actual operand range
used by the harness. The identity is a tagged representation: `GraphChild { node_id }` for an
actual graph reference (including a parameter or subterm), or `InlineLiteral { node_id,
occurrence_key, position }` for an inline literal, using the authentic application node id,
selected occurrence key and operand position ordinal. An inline literal's singleton range
carries its value (FR-015 AC-16); QSL emits no extra literal node for it. A reference to an
existing literal node is a graph child. These are canonical representation fields: the measured
Rust enum is `GraphChild(WireNodeId)` or `InlineLiteral`; QSL's encoder expands the inline
arm using the application, occurrence and position retained by the claim. Neither tag
fabricates a checked node id.
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

Under FR-022's one-item-per-distinct-node generation, an authentic occurrence means the sole
`expression` occurrence of that application in the proving package. Zero or multiple such
occurrences yields typed occurrence-unavailable/ambiguous setup refusal and no identity; CG
shall not pick the first or lowest ordinal. Replay of multi-occurrence nodes is CODE-gated on
an owning generation change that selects and retains the proved item's occurrence. A node's
having several valid QSL occurrences alone supplies no selection provenance.

CG shall take the application identity and authentic occurrence from the admitted IR reader.
CG typed generation shall supply and retain the scalar kind, positional operand identities,
ranges and proving metadata for the complete checked claim body. The closed claim members are the application `node`,
authentic `occurrence_key`, typed `obligation_kind` and ordered `arguments`; each argument
retains its position, tagged operand identity and actual harness range. A graph operand uses
its checked graph-child node; an inline literal uses the application, occurrence and position,
with its value in the singleton range. Position remains numeric and draw ordered; two draws
of one parameter remain distinct. CG compares this complete retained body structurally
against the sent claim and returned report. It does not sort or deduplicate positions, compute
canonical obligation bytes, or treat equality of the node identity alone as claim admission.
Source spans and native outcome are not members of the checked claim; the full report separately
carries operator, decoded values, result range, limits and same-artifact observation. A body-only
change with the same structural node identity must refuse at the body comparison.

Measured IR `CheckedPackageV2::graph()` exposes nodes with authoritative `node_id` and
`occurrences`, and `source_map()` exposes their source map; `CheckedOccurrence` supplies role
and ordinal. The current CG scalar record does not retain its selected expression occurrence
or typed operand identities. IR's public node `body` is still a JSON value; no public typed accessor
for the scalar application's ordered operands, their identities and ranges was measured.
The model-field accessor and wrapped optional operation-leaf walk do not supply that scalar
accessor. IR-648 (Done) exposes the same `GraphChild`/`InlineLiteral` identity distinction;
inline-literal identity uses the application/occurrence/position and requires no literal node.
[IR-648](https://linear.app/agent-ix/issue/IR-648), the typed scalar
operand accessor on `CheckedPackageV2`, is Done (the merged tests call `scalar_application_operands`), and CG retains the occurrence selected for the proved item. Missing or ambiguous occurrence, operand identity/range
or authoritative scalar kind yields typed setup refusal and no minted identity. The positional rule resolves
literal and repeated-parameter semantics; no ambiguity gate for those semantics remains.
The merged QSL verifier authenticates the occurrence against the application, operand count,
and each graph-child reference at its position. An inline literal must be the actual inline
integer at that position, with its observed value and singleton range; typed mismatches refuse
before structural claim comparison. A graph-child identity check does not itself validate that child's
claimed integer range against its declared proving domain. CG shall retain and validate the
actual harness ranges against the checked/emitted proving schema, retaining their positional
provenance. A parameter or subterm typed by its own `bounded_domain` uses that bound's
checked derived integer interval; its bound must belong to the lowered claim's checked bounds.
An inline literal or referenced literal-value node uses its actual singleton integer value.
For a subterm with no own bound and no literal value, the current generation authority uses
the enclosing scalar claim's first checked result bound, rather than infer a new subterm range.
The replay record must retain that exact result-bound provenance and interval. The same rule
applies to any other operand lowered through the result-bound case. No wider interval, caller
hint or freshly inferred child domain substitutes for the range the harness actually assumed.
If typed access cannot recover that provenance, the route refuses with a typed setup refusal;
this specification neither broadens generation nor changes its result-bound rule. IR-648's authentic metadata
access and CG retention are delivered (#481, #702). Missing or malformed authoritative identity or retained body returns typed setup refusal before replay; no invented identity or sentinel is admitted.

Decoder and original-limits context work can proceed independently.

## Converter Boundary and Original Context

The converter, `CompletedScalarReplay::settlement`, accepts the actual immutable public `OperatorParityReport` together
with CG's retained binding and checked driver-completion context; absent report means no
settlement under FR-029 AC-15. It reads `report.claim()`;
it does not construct, deserialize or alter QSL's private report fields. The retained binding
holds the expected sent `OperatorIdentity` separately, so mismatches are observable at this
public boundary without a fabricated report.

`OperatorIdentity` and the structural node identity alone do not bind the replay wire's package, selected function, source
bytes or dependencies. In addition to full-claim equality, the converter shall compare the
actual wire used for that run, recorded by checked driver completion, with the original
proving `call_site` context retained by CG: package identity/reference, selected enclosing
function, dependencies, source byte provision and original limits including `ReplayLimits`.
An equal claim replayed against another internally consistent package or enclosing function
must refuse with no settlement. The driver must record the actual sent context, not echo an
expected claim/context supplied afterward. This checked completion/retention representation
is a CODE gate; no new QSL identity field, tracking hash or public constructor is claimed.

## Settlement Contract

The driver calls QSL between `CompletedScalarReplay::request` and the converter, then pairs the
result with the Kani outcome. Every name below is public and owned by CG; no `qsl_replay` call is
made in the routed path.

| Step | Public item | Contract |
|---|---|---|
| Request | `CompletedScalarReplay::request()` | Returns the wire, `OperatorClaim` and `ReplayLimits` for `replay_operator_parity`; unchanged. |
| Convert | `CompletedScalarReplay::settlement(&ScalarSentRequest, Option<&OperatorParityReport>) -> Result<ScalarSettlement<'r>, ScalarBuildError>` | Full-claim and sent-context checks of FR-032-AC-6, then `ScalarSettlement::Parity(report.result())`. Replaces `settle`. |
| Setup defect | `ScalarBuildError::setup_settlement(&self) -> Option<ScalarSettlement<'static>>` | The FR-032-AC-24 classification, with its reason. |
| Pair | `scalar_run_terminal_value` and `scalar_ir_outcome_terminal_value` | Same shape as `run_terminal_value` and `ir_outcome_terminal_value` with `Option<ScalarSettlement<'_>>`; same `TerminalPairError`. |

`ScalarSettlement<'a>` has exactly two variants: `Parity(&'a OperatorParityResult)` and
`CgDefect`. It lives beside `ReplaySettlement` in `kani/terminal.rs` and is built from `qsl_replay` types;
`kani/` imports nothing from `replay/`, and `replay/` code constructs the value (AD-004). A distinct input, as FR-029 requires for the composite route, keeps the
ordinary six readings and FR-029-AC-15 unchanged and makes `Reproduced` and `Inconclusive(ReplayParity)`
unrepresentable on this route. The terminal value of each variant:

| Settlement | Source (QSL-owned name) | `TerminalValue` |
|---|---|---|
| `Parity` | `OperatorParityResult::Diverged` | `Failed` |
| `Parity` | `OperatorParityResult::GeneratedFault` | `Failed` |
| `Parity` | `OperatorParityResult::Agrees` | `Inconclusive(ScalarAgrees)` |
| `Parity` | `OperatorParityResult::RefusedInput` | `Inconclusive(ReplayRefused)` with `invalid_runtime_input` |
| `Parity` | `OperatorParityResult::Incomplete` | `Incomplete(ResourceExhausted)` |
| `Parity` | `OperatorParityResult::Refused`, non-fault | `Inconclusive(ReplayRefused)` with the refusal's catalog code |
| `Parity` | `OperatorParityResult::Refused`, fault | `Failed` |
| `CgDefect` | CG-raised `ScalarBuildError` per FR-032-AC-24 (`Playback`, `Staging`, `Identity`, `Operation`, non-tamper `Authority`) | `Failed` |
| none | converter refusal and binding-mismatch `ScalarBuildError` per FR-032-AC-24 | `TerminalPairError::MissingSettlement` for a falsified run; no terminal value |

**Trust boundary.** Neither pair function checks that the `ScalarSettlement` belongs to the obligation of the `KaniExecutionEvidence` or `KaniOutcome` it is paired with: the driver is the trusted pairing authority, as it is for `ScalarDriverObservation`. CG ties the report to the sent request and the completed plan inside `settlement` (FR-032-AC-6, FR-032-AC-20), and the evidence carries no scalar identity to compare. A tie checked by CG is a separate requirement with its own criterion, not claimed here.

Scalar lowering parity is never a refutation: this route yields no `Refuted`, `Proved`, `Tested`
or `Inconclusive(ReplayParity)`. Where the Kani run is falsified and QSL's replay agrees with the
generated oracle, the harness, not the source, is suspect, which is `ScalarAgrees`. The fault row
of `Refused` is verified by Inspection of the pass-through, because no public input induces it
(the approach of FR-029-AC-28).

## Setup Refusal Precedence

The builder uses the first applicable refusal in this order, even while the upstream gate holds:

1. Original proof-context, node/operator/schema, package/source and retained staged-byte
   coherence, including retention of the original scalar limits.
2. Selected playback byte ceiling, checked before parsing through the Kani output adapter.
3. Adapter selection and decoding: assertion block, qualified harness name, arity, byte width
   and persisted operand ordering.
4. Decoded operand-domain validation against the original schema.
5. Availability of the typed operand/provenance accessor (IR-648, Done), authoritative scalar-kind
   value/source, retained unique expression occurrence and proving-context/limits record,
   and checked driver observation/context-completion seam.

An otherwise valid request reaches unsupported-capability at step 5 only when at least one
named capability is unavailable; all are available today (IR-648, IR-940 Done), so `ScalarBuildError`
has no such variant and step 5 is vacuous. Once those capabilities are available and the prior setup
checks succeed, the builder shall return an admitted replay plan; completion with authenticated
observation and actual original context shall enable replay and conversion. A permanently
refusing builder after these conditions hold cannot satisfy this requirement.
A malformed request reaches its earlier typed refusal; the absent capability never hides it.
After plan completion, the native-observation/staged-byte tie is checked before QSL evaluation.
Cross-binding converter failures return no settlement rather than settling another obligation.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-032-AC-1 | PARTIAL (IR-1184, #741): every plan() call in `tests/it/scalar_replay_binding.rs` passes the routed `ScalarObligationIdentity` directly and is admitted, so the builder admits plans and no unsupported-capability refusal exists in `ScalarBuildError` (IR-648 and IR-940 are Done); no test carries an AC-1 tag, so `quire matrix` reports it untagged until the IR-631 code slice tags it. A public consumer passes the routed harness's `ScalarObligationIdentity` directly to the scalar builder (the no-`KaniObligationIdentity`, no-V1-`ReplayPackage` clause is owned by FR-032-AC-25 alone). After the ordered setup checks, an otherwise valid request returns unsupported-capability only while a named step-5 capability is unavailable; when all are available it returns an admitted plan, and authenticated completion enables replay/conversion. Conversion of a capability refusal returns no settlement rather than a synthetic function request or invented terminal cause. | Test |
| FR-032-AC-2 | PARTIAL (#757): decode order, harness name, arity, byte width, malformed assertion block and the 8 MiB ceiling are asserted by tagged `tests/it/scalar_playback.rs` tests; the stated refusal precedence across the five setup steps has no ordering test (PLANNED). The builder decodes actual routed-harness playback in persisted symbolic call order through the Kani output adapter; another harness name, missing/extra operand, wrong byte width, playback above the builder-owned 8 MiB per-harness ceiling or malformed assertion block yields the distinct typed refusal under the stated precedence before replay. | Test |
| FR-032-AC-3 | PARTIAL (#757, #481): inclusive endpoints with adjacent refusals, operator/literal operand decode and retained original limits are asserted by tagged `scalar_playback.rs` and `scalar_literal_operands.rs` tests; the full QSL-admitted request from a real Kani witness is PLANNED. Each operand at its own lower or upper bound is admitted and the adjacent out-of-domain value is refused before replay. The admitted QSL inputs preserve the catalogued operator, operands, operand ranges, result range and harness limits. Another node/operation, altered operand schema or replay limit differing from the retained original generation context refuses; operand correspondence uses [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md) AC-16. | Test |
| FR-032-AC-4 | PARTIAL (#702, #741): `ScalarReplayPlan::complete` refuses a changed staged byte, identity, harness, record, operand order, limit, source or package, and refuses the observation of a compiled arithmetic mutant, in `scalar_replay_binding.rs`, whose tags do not name AC-4 (untagged in `quire matrix` until the IR-631 code slice tags it); observation of the artifact a real Kani run proved is PLANNED. The driver's actual native observation of the proved generated oracle is required and tied to this operand vector, limits and retained staged artifact bytes; checked completion requires driver authentication. A bare outcome echo, missing observation, another artifact's observation or fresh regeneration that removes the arithmetic mutation refuses before QSL evaluation. Playback bytes and Kani falsification alone cannot supply a generated outcome. | Test |
| FR-032-AC-5 | PARTIAL (#741): the genuine QSL facade returns `Agrees` over the retained observation for each of the four operators in `scalar_replay_residuals.rs` (tag TC-047 only, so `quire matrix` reports AC-5 untagged until the IR-631 code slice tags it); a QSL `Diverged` result from a changed generated artifact is not asserted (PLANNED, real-Kani lane). The public consumer executes the same proved generated artifact and invokes QSL between the builder and converter. QSL compares its actual native observation with authoritative exact operator evaluation using the four-operator projection owned by [FR-015](../../kani/functional/FR-015-bounded-kani-obligations.md) AC-37, without adding refusal-cause or accounting-counter equality; changing the generated arithmetic while keeping original node/operator identity changes the measured parity result. `Undefined`, `Incomplete` and execution faults cannot be agreeing outcomes of this profile. No CG-local evaluator or replay verdict replaces QSL. | Test |
| FR-032-AC-6 | PARTIAL (#702, #741): tagged `scalar_replay_binding.rs` asserts that an absent report, an equal-claim report for another function, byte provision or replay budget, and a single changed native outcome, operand value, limit, occurrence or result range each refuse; the obligation, node, kind (other than via the first control), operator and operand-identity members are not individually mutated (PLANNED). For every `Diverged`, `Agrees`, `GeneratedFault`, `RefusedInput`, exact `Incomplete` and other `Refused` outcome, the converter compares the report's full claim to the retained sent claim and actual sent wire against the original proving context before conversion; a changed package/function/source/dependency or original limit refuses even with an equal claim. At the immutable-report/retained-binding boundary, an absent report, another result/run, or changing only the obligation, node, occurrence, kind, operator, operand identity/value/range, result range, limits or native outcome yields typed binding refusal and no settlement. Obligation identity equality or agreement alone cannot pass. | Test |
| FR-032-AC-7 | PLANNED (IR-631); the conversion is specified by FR-032-AC-20 to FR-032-AC-24, which own the mapping table below. QSL's divergence between generated and authoritative exact outcomes becomes a CG lowering fault and `Failed`; agreement despite Kani falsification becomes `Inconclusive(ScalarAgrees)`; native `Incomplete`/execution fault becomes `GeneratedFault` and `Failed` while exact `Incomplete` becomes `Incomplete(ResourceExhausted)`. `RefusedInput` preserves `invalid_runtime_input`; other typed QSL refusals follow the existing refusal map and non-fault refusals retain their code. This scalar route never returns `Reproduced`, `Refuted` or `Verified`. | Test |
| FR-032-AC-8 | PLANNED (IR-631); no test exercises either fixture through the scalar builder (verified by grep: AC-8 has no tag), and real-Kani playback is unavailable to the default lane. The real QSL-emitted `x + 1` with input `Int[0,9]` and result `Int[0,10]` exercises routed generation, actual Kani arithmetic-mutation playback and same-proved-artifact native observation. A separate bounded-addition fixture with both operands and result in `[-1000,1000]` exercises an admitted operand pair whose exact result is outside the result range. The expected exact result is derived from the actual retained playback, and its correct refusal is never labelled a source violation. The fixture ranges and witness domain/result-range relation are asserted before that observation; no unreachable case or assumed solver choice counts as coverage. | Test |
| FR-032-AC-9 | ~~Retired hash-preimage criterion.~~ Its authoritative member-selection, positional-order, distinct-draw, missing-provenance and changed-member checks are carried by FR-032-AC-15. The retired identity-digest oracle is not reused. | Inspection |
| FR-032-AC-10 | ~~Retired shared-mint criterion.~~ IR-reader structural identity and actual QSL route conformance are carried by FR-032-AC-16. The retired parity encoder is not called. | Inspection |
| FR-032-AC-11 | ~~Retired claimed/recomputed-digest criterion.~~ Exact identity and retained-body mismatch refusal before comparison is carried by FR-032-AC-17. | Inspection |
| FR-032-AC-12 | PLANNED (IR-631) / CODE-GATED (typed scalar-family/metadata retention and actual CG/QSL route). When a structurally self-consistent claim invents an application occurrence or operand child, or misstates an inline literal's value/singleton range, the consumer observes the corresponding merged typed membership refusal before structural claim comparison; a self-consistent string cannot admit invented metadata. | Test |
| FR-032-AC-13 | ~~Retired preimage-encoder failure criterion.~~ Missing or malformed authoritative identity/body refusal with no fabricated identity is carried by FR-032-AC-18. | Inspection |
| FR-032-AC-14 | PLANNED (IR-631) / CODE-GATED (actual CG scalar builder/converter). Adding the scalar builder/converter preserves existing function/clause carry-only behavior and function/frame identity encoding; none is redirected through scalar parity or changed by its positional argument rule. | Test |
| FR-032-AC-15 | PARTIAL (#702, #741, #757; IR-648 and IR-940 Done): tagged `scalar_replay_binding.rs` asserts the body equals the IR reader's `scalar_application_operands`, two draws of one parameter stay distinct, and a changed package, path or producer ordinal refuses; source-span invariance and missing or changed occurrence are asserted only by `scalar_replay_residuals.rs`, which is untagged (PLANNED: tag it). The admitted IR reader supplies the original application structural identity and authentic expression occurrence. CG typed generation supplies and retains the scalar kind and one argument for each operand position in actual draw order, with typed graph-child or inline-literal provenance and the harness range. Two draws of one parameter remain two entries. Changing a package, path or producer ordinal changes the structural identity; changing an included body member while the identity stays equal is caught by complete retained-body comparison and refuses. A source-span or native-outcome change alone does not alter the claim identity. Missing or ambiguous occurrence, operand provenance or scalar-kind retention refuses before replay; no identifier sort, node deduplication, invented ordinal or unrelated enclosing parameter passes. | Test |
| FR-032-AC-16 | PARTIAL (#702, #741): tagged `scalar_replay_binding.rs` asserts that the genuine QSL operator facade returns `Agrees` for the matching claim over the `x + x` fixture, and refuses a claim with a changed range or kind; acceptance of the matching claim for the other three operators and the inline literal is in untagged `scalar_replay_residuals.rs` (PLANNED: tag it). An independent fixture obtains the same application `NodeKey`/`CheckedNodeId` from IR admission and CG retention, checks the exact structural identity and complete positional body, and the actual QSL operator route accepts the matching claim. No CG-local identity derivation, copied encoder, stand-in verifier or synthetic acceptance passes. | Test |
| FR-032-AC-17 | PARTIAL (#702, #741): tagged `scalar_replay_binding.rs` asserts that a changed package, path or producer ordinal, or a changed body member under an equal identity, returns `ScalarBuildError::BodyMismatch`; QSL's own typed stale-dependency/revision-mismatch result at the facade is not asserted (PLANNED). With a valid original claim retained, altering only the claimed package, path or producer ordinal, or changing one otherwise membership-valid retained body member while reusing the old structural identity, produces a typed stale-dependency/revision-mismatch refusal before scalar comparison. A body-only edit cannot pass because the node identity is equal. No claimed or recomputed digest is required. | Test |
| FR-032-AC-18 | PARTIAL (#757, #481): `core/scalar_authority.rs` asserts that zero or several expression occurrences refuse without inventing one, and tagged `scalar_replay_binding.rs` asserts the plan refusals for a changed body; the refusal for an absent retained kind or operand identity has no tagged test (PLANNED). If the admitted IR read lacks a valid application identity or authentic occurrence, or CG typed generation lacks a required retained body member, the CG builder returns typed setup refusal without replay or fabricated identity. A caller string or usable sentinel cannot replace the absent authority. | Test |
| FR-032-AC-19 | PLANNED (IR-631 code slice; IR-304 is Done, but `src` has no `guard_inputs` symbol, so no decoding exists). For a scalar identity with `guard_inputs`, the expected playback entry count is the operand count plus the guard-input count, and the guard entries follow the operands in `guard_inputs` order: an integer guard as one eight-byte `i64` and a Boolean guard as one byte. A short playback refuses as `kani_witness_arity_mismatch`; a four-byte integer guard entry as `kani_witness_width_mismatch`; a Boolean byte `2` or an integer guard value one outside its narrowed domain as `cg_scalar_witness_guard_out_of_domain`. The decoded operand vector sent to QSL excludes the guard values. An identity without `guard_inputs` decodes exactly as before. | Test (TC-047) |
| FR-032-AC-20 | PLANNED (IR-631). When the driver passes the genuine `OperatorParityReport` that `replay_operator_parity` returned, with the `ScalarSentRequest` it actually sent, to `CompletedScalarReplay::settlement`, the converter, after the AC-6 checks, returns `ScalarSettlement::Parity` borrowing that report's `OperatorParityResult`. A missing report, another run's report, or a wire or replay budget different from the retained original context returns `ScalarBuildError::Report` or `ScalarBuildError::SentContext` and no `ScalarSettlement`. `ScalarSettlement` is a distinct typed input of the Kani terminal map (it is not a variant of `ReplaySettlement`) and is built from `qsl_replay` types only. `CompletedScalarReplay::settlement` replaces `CompletedScalarReplay::settle`, which returned a bare `qsl_replay::TerminalValue` and is removed with no retained alias or shim. Neither the converter nor any public scalar type names `KaniObligationIdentity`, builds a synthetic one or takes CG's V1 `ReplayPackage`. | Test |
| FR-032-AC-21 | PLANNED (IR-631). The scalar terminal map reads `OperatorParityResult` through one exhaustive `match` with no wildcard or catch-all arm (`#[deny(clippy::wildcard_enum_match_arm)]`), each arm returning `result.terminal_value()` so that QSL owns every value and a new QSL result class fails the build. QSL's values are: `Diverged` and `GeneratedFault` give `Failed`; `Agrees` gives `Inconclusive(ScalarAgrees)` carrying that report's agreement; `RefusedInput` gives `Inconclusive(ReplayRefused)` carrying the refusal's own code (`invalid_runtime_input`); `Incomplete` gives `Incomplete(ResourceExhausted)`; `Refused` gives what `TerminalValue::from_replay_refusal` gives, which is `Failed` for a fault and `Inconclusive(ReplayRefused)` with the refusal's catalog code otherwise. For every genuine report, the value equals `OperatorParityReport::terminal_value()`. Native `Incomplete` or an execution fault is `GeneratedFault`, never exact `Incomplete`. The map never yields `Refuted`, `Proved`, `Tested` or `Inconclusive(ReplayParity)`, which are predicate-route readings. | Test, Inspection |
| FR-032-AC-22 | PLANNED (IR-631). `scalar_run_terminal_value(&KaniExecutionEvidence, Option<ScalarSettlement>)` and `scalar_ir_outcome_terminal_value(&KaniOutcome, Option<ScalarSettlement>)` pair the settlement with the Kani outcome. A `Falsified` run or a `Counterexample` outcome with a settlement gives the AC-21 value for `Parity` and `Failed` for `CgDefect`; an outcome of any other kind with a settlement returns `TerminalPairError::UnexpectedSettlement` and no value; an outcome with no settlement gives the same value as the ordinary entry point. `ReplaySettlement` and the ordinary map are unchanged. | Test |
| FR-032-AC-23 | PLANNED (IR-631). For a falsified run (`KaniRunOutcome::Falsified` evidence and a `Counterexample` IR outcome), omitting the settlement returns `TerminalPairError::MissingSettlement` and no terminal value from both `scalar_run_terminal_value` and `scalar_ir_outcome_terminal_value`. A converter refusal (AC-20) leaves no `ScalarSettlement`: the result of `settlement(...).ok()` for a missing report or another run's report, passed to either pair function, yields `MissingSettlement`; a refusal cannot be turned into a terminal record. | Test |
| FR-032-AC-24 | PLANNED (IR-631). `ScalarBuildError::setup_settlement` is an exhaustive `match` over every variant and, for `Authority`, over every `ScalarAuthorityError` cause. It returns `Some(ScalarSettlement::CgDefect)`, read as `Failed` by both pair functions, for a failure this repository raised on the right obligation: `Playback` (decode, byte-width, arity and assumed-domain failure of the falsified run's own playback), `Staging`, `Identity`, `Operation`, and `Authority` caused by `Unavailable`, `Occurrence`, `Operation`, `Operands`, `Ranges` or `Record`. It returns `None` for a binding mismatch against independently retained or driver-asserted context, where a `Failed` record could be written for the wrong obligation: `Authority` caused by `GeneratedArtifactChanged`, `BodyMismatch`, `Context`, `Artifacts`, `Observation`, `Report` and `SentContext`. A `None` leaves a falsified run with `MissingSettlement`. `BodyMismatch`, `Context`, `Artifacts` and `Observation` arise from `plan` or `complete` before the driver calls QSL; `Report` and `SentContext` arise from `settlement` after it. | Test |
| FR-032-AC-25 | PLANNED (IR-631): the merged code satisfies the typed-call half (verified by grep: `src/replay/scalar_builder.rs` names neither `KaniObligationIdentity` nor CG's `ReplayPackage`; its `raw_package` parameter is `qsl_replay::ReplayPackage`), and no test asserts it. A compiled end-to-end call of `generate_routed`, `OriginalScalarContext::plan`, `ScalarReplayPlan::complete`, `CompletedScalarReplay::request`, `CompletedScalarReplay::settlement` and `scalar_run_terminal_value` uses only `ScalarObligationIdentity`, and a source scan finds no `KaniObligationIdentity` or CG `ReplayPackage` token, and no `settle` method, in the scalar builder module or its public re-exports. | Test |

## Intent and Existing Coverage

This table separates delivered native input-binding evidence from complete
proof-driven pipeline qualification; it is not a computed test matrix.

| Intent | Existing authority and measured tests | Remaining gap |
|---|---|---|
| Real routed scalar generation | [FR-022](../../routed/functional/FR-022-routed-generation.md) AC-2/10/13/15; `tests/it/routed_generation.rs` carries those trace tags | `OriginalScalarContext` binds the retained body; the same-proved-artifact pipeline remains Planned under IR-631 |
| Typed ordered playback and domain validation | [FR-016](./FR-016-witness-native-replay.md) AC-1/2/5/8; surviving `skeleton_spine.rs` tests exercise native replay with a direct identity fixture. The removed `kani_witness_join.rs` and `kani_argument_order.rs` were V1-only historical controls. | Native scalar schema/operator inputs are exercised in `scalar_replay_binding.rs` and `scalar_replay_residuals.rs`; full proof-driven qualification remains Planned under IR-631 |
| QSL refusal and terminal reading | [FR-016](./FR-016-witness-native-replay.md) AC-9/10/11/12 and [FR-029](../../kani/functional/FR-029-run-outcome-terminal-record.md) AC-1/10/13; `skeleton_spine.rs` and `terminal_map.rs` cover existing replay paths; AC-12 remains planned in the replay index | Native actual-sent/report ties are exercised in `scalar_replay_binding.rs`; complete scalar vector coverage remains Planned under IR-631, and predicate violation/parity does not establish it |
| Missing settlement produces no terminal value | [FR-029](../../kani/functional/FR-029-run-outcome-terminal-record.md) AC-15; `tests/it/terminal_map.rs` | Scalar route: `scalar_run_terminal_value` and `scalar_ir_outcome_terminal_value` with no settlement (FR-032-AC-23, Planned); FR-029-AC-15 itself is unchanged |
| Driver calls QSL between public typed builder and converter | `scalar_replay_binding.rs` and `scalar_replay_residuals.rs` execute the native observation and real QSL operator entry | The complete same-proved-artifact pipeline in [TC-047](../matrix/TC-047-routed-scalar-replay-binding.md) remains Planned under IR-631 |

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

The merged QSL `FR-357` and `qsl-replay/src/scalar.rs` expose
`replay_operator_parity`, `OperatorClaim`, `OperatorParityReport`, `NativeOutcome` and the
scalar result/terminal readings above. The public facade call is
`replay_operator_parity(wire, claim, replay_limits)`, with `ReplayLimits` last as amended
by merged QSL change #645. That input-byte guard is separate from the claim's `ScalarLimits`
for exact evaluation; `ReplayLimits` is not an `OperatorIdentity` member. Retain both original
contexts rather than invent a claim field or use an unretained default. The operator entry recompiles the original source,
checks `package_id`, and validates scalar-node/operator membership in the selected enclosing
function; it does not replay that function as a predicate. `OperatorClaim.identity` carries the obligation identity; the driver separately retains
the staged artifact bytes used to authenticate its native observation.
On measured merged QSL change #650, `OperatorParityReport::claim()` exposes
`OperatorIdentity` on every outcome and `ValueParityReport::claim()` exposes `ValueIdentity`;
FR-357 AC-13 specifies full-claim retention. The
operator report retains the obligation, node, operator, operands/ranges, result range, limits,
occurrence and kind even when request decoding or context validation refuses. The current full claim also retains the native outcome; an obligation-only
comparison cannot pass. The native binding tests below exercise these public
interfaces; they do not supply proof-driven pipeline qualification.

The complete proved-artifact route has these explicit **CODE gates**. Native
constructor/replay controls exercise the CG seams without closing that pipeline:

1. CG's consumer must conform to the merged full-claim report API. For this operator slice
   the full claim is the retained obligation identity, scalar node/occurrence, kind,
   operator, ordered operands with identities/ranges, result range, retained limits and native
   outcome. Compare it before settlement on every outcome. The driver separately compares its
   original artifact bytes with the receipt before checked completion; QSL never receives them.
   The merged function-parity report does not authorize CG function-parity implementation here.
2. CG generation must retain the authoritative scalar metadata needed by the positional
   checked claim body and original renderer-limit/proving record. Unique authentic occurrence, typed positional operand identity/range/provenance access
   (IR-648, Done) and the typed scalar-family implementation/retention are delivered (#481, #702); only their full-pipeline use remains a gate.
   CG must take the structural application identity and authentic occurrence from the IR reader,
   retain the typed scalar kind and positional body from CG generation, and compare the complete body even when the node identity is unchanged, and prove matching-claim
   acceptance and mismatch refusal with actual QSL. CG must validate retained graph-child ranges
   against the proving schema.

3. The driver must implement same-proved-artifact execution and observation authentication;
   CG must supply the retained generation/proving record and checked completion/conversion seam,
   including the actual sent original replay wire/context, independently of full-claim equality.
   QSL never sees or authenticates the artifact; carried structural identity equality alone is insufficient.
4. CG's converter/settlement representation must consume the measured legal scalar terminal
   readings only after full-claim and retained-run comparison. Existing predicate disagreement
   or a fabricated scalar cause must not stand in for that conversion.
5. AD-002 R-7 stays in force: retain the **original** proving `call_site` package identity and
   source bytes; QSL recompiles them, checks the same `package_id`, and validates scalar
   node/operator membership in the selected function. Retain the proving source/package tie;
   neither fresh oracle generation nor a synthetic predicate selection substitutes for it.

AD-002's existing CG replay-call list still constrains CG: the planned builder makes no execution
call, and the driver executes the generated artifact and separately calls QSL's public scalar replay
facade. This contract neither adds a fourth CG executor nor claims a new call-site selection. The merged operator facade retains R-6/R-7; actual CG and driver conformance remains gated,
and this document grants no architecture exception.

Status is stated per criterion. AC-1 to AC-6 and AC-15 to AC-18 are **Partial**: the merged
native tests (#702, #741, #757, #481; IR-1184) assert the CG-side checks each row names, and the
rest of each row stays Planned. AC-7, AC-8, AC-12, AC-14, AC-19 and AC-20 to AC-25 are
**Planned**; AC-9 to AC-11 and AC-13 are retired. The IR-648 and IR-940 gates are closed (both
Done); the remaining gates for the whole route are the driver's same-proved-artifact execution
(the driver's E9 run, IR-610) and a real falsified Kani run through the route (the IR-631 code slice). No unsupported-capability refusal exists in `ScalarBuildError`
today. The merged function-level declared-parameter parity route is outside this operator-level CG contract.

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
  CG owns the scalar structural claim retention and producer/converter contracts; the driver owns observation
  authentication; QSL-641 owns the amended full-report claim API. Measured QSL FR-357 owns exact
  evaluation, original-package validation and scalar terminal causes. IR-648 in Contract IR
  owned the typed `CheckedPackageV2` scalar operand accessor and is Done.
- **Related**: [FR-016](./FR-016-witness-native-replay.md) owns existing playback/function replay
  rules; [AD-002](../../assurance/AD-002-cg-qsl-replay-seam.md) retains QSL evaluation ownership.
- **Downstream**: [TC-047](../matrix/TC-047-routed-scalar-replay-binding.md), planned public
  consumer and real-backend evidence for IR-631; driver E9 calls the admitted QSL seam once
  its prerequisites are resolved.

## Native Input-Binding Evidence

`tests/it/scalar_replay_binding.rs` exercises repeated parameter draws and
actual driver/QSL ties. `tests/it/scalar_replay_residuals.rs` adds every current
integer operator, distinct parameters, an occurrence-qualified inline literal,
source-span invariance, independently observed result changes and missing-body
refusals. The subtract and multiply paths consume the catalogue identities
`quire.op.integer.sub` and `quire.op.integer.mul` stated above.

These are real native executions and owner replay calls, not proof receipts.
The complete same-proved-artifact falsification pipeline remains Planned under
IR-631. The AC-15 through AC-18 requirement text remains unchanged; native
input-binding evidence alone does not upgrade whole-pipeline status.
